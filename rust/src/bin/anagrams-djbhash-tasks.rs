extern crate combinations;
extern crate bisect;
extern crate djbhashmap;

use std::env;
use std::io::{BufRead,BufReader};
use std::fs::File;
use std::sync::mpsc::{channel,Sender,Receiver};
use std::thread;
use std::sync::Arc;

use djbhashmap::{HashMap,HashSet};

pub fn split_words(s : &str) -> Vec<String> { s.split_whitespace().map(|w| w.to_string()).collect() }

fn load_dictionary() -> HashMap<Vec<u8>,Vec<String>> {
    let file = File::open("anadict.txt").expect("cannot open anadict.txt");
    let buffered_file = BufReader::new(file);
    let mut map          = HashMap::new();
    for line in buffered_file.lines() {
        let line                  = line.unwrap();
        let words   : Vec<String> = split_words(&line);
        let key     : Vec<u8>     = words[0].chars().map( |ch| ch as u8 ).collect();
        let entries : Vec<String> = words.iter().skip(1).map( |w| w.to_string() ).collect();
        map.insert(key, entries);
    }
    return map;
}

fn get_letters(s : &str) -> Vec<u8> {
    let mut t : Vec<u8> = s.chars().map( |ch| ch as u8 ).collect();
    t.sort();
    return t;
}

fn search(arc_dictionary : Arc<HashMap<Vec<u8>,Vec<String>>>,
          request_receiver   : &Receiver<Vec<Vec<u8>>>)
          -> HashSet<String> {
    let dictionary = arc_dictionary.clone();
    let mut set    = HashSet::new();
    loop {
        let key_set = request_receiver.recv().unwrap();
        if key_set.len() == 0 { break; }
        for key in key_set.iter() {
            match dictionary.get(key) {
                Some(ref val) => {
                    for word in val.iter() { set.insert(word.clone()); }
                }
                None => { }
            }
        }
    }
    return set;
}

fn spawn_workers(n_workers : usize) -> (Receiver<HashSet<String>>,Vec<Sender<Vec<Vec<u8>>>>) {
    let shared_dictionary                             = Arc::new( load_dictionary() );
    let (response_sender, response_receiver)          = channel();
    let mut request_senders : Vec<Sender<Vec<Vec<u8>>>> = Vec::new();
    for _ in 0..n_workers {
        let dictionary                         = shared_dictionary.clone();
        let response_sender                    = response_sender.clone();
        let (request_sender, request_receiver) = channel();
        request_senders.push(request_sender);
        // Set up and start worker task
        thread::spawn(move || {
            response_sender.send( search(dictionary, &request_receiver) ).unwrap();
        });
    }
    (response_receiver, request_senders)
}

static WIDTH : usize = 6;        // number of worker tasks
static DEPTH : usize = 200000;   // keys / request sent to worker task

fn main() {
    let mut args = env::args();
    if args.len() < 2 { panic!("Usage: anagrams letters"); }
    let letters = get_letters(&args.nth(1).unwrap());

    let (response_receiver, request_senders) = spawn_workers(WIDTH);

    // Iterate through the combinations, collecting groups of them
    // (key_set) to be sent to individual workers.
    let mut worker  = 0;
    let mut key_set = Vec::new();
    for i in 2..letters.len() + 1 {
        combinations::each_combination(&letters, i, |combo| {
            key_set.push( combo.to_owned() );
            if key_set.len() >= DEPTH {
                let mut ks = Vec::new();
                std::mem::swap(&mut ks, &mut key_set);
                request_senders.get(worker).unwrap().send(ks).unwrap();
                worker = (worker + 1) % WIDTH;
            }
        });
    }
    // Send remaining combinations (key_set) to the next worker.
    if !key_set.is_empty() { request_senders.get(worker).unwrap().send(key_set).unwrap(); }
    // Send an empty key set to tell each worker to terminate.
    for chan in request_senders.iter() { chan.send(Vec::new()).unwrap(); };

    // Collect responses from workers.
    let mut set : HashSet<String> = HashSet::new();
    for _ in 0..WIDTH {
        let response_set = response_receiver.recv().unwrap();
        for word in response_set.iter() { set.insert(word.clone()); }
    }
    println!("{}", set.len());
}

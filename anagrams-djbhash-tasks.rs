extern crate combinations;
extern crate bisect;
extern crate djbhash;
extern crate alloc;

use std::{iter,os};
use std::io::{BufferedReader,File};
use alloc::arc::Arc;

use djbhash::{HashMap,HashSet};

pub fn split_words(s : &str) -> Vec<String> { s.words().map(|w| w.to_string()).collect() }

fn load_dictionary() -> HashMap<Vec<u8>,Vec<String>> {
    let mut bufferedFile = BufferedReader::new( File::open( &Path::new("anadict.txt") ) );
    let mut map          = HashMap::new();
    for line in bufferedFile.lines() {
        let line                  = line.unwrap();
        let words   : Vec<String> = split_words(line.as_slice());
        let key     : Vec<u8>     = words.get(0).as_slice().chars().map( |ch| ch as u8 ).collect();
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
    let dictionary = arc_dictionary.deref();
    let mut set    = HashSet::new();
    loop {
        let key_set = request_receiver.recv();
        if key_set.len() == 0 { break; }
        for key in key_set.iter() {
            match dictionary.find(key) {
                Some(ref val) => {
                    for word in val.iter() { set.insert(word.clone()); }
                }
                None => { }
            }
        }
    }
    return set;
}

fn spawn_workers(n_workers : uint) -> (Receiver<HashSet<String>>,Vec<Sender<Vec<Vec<u8>>>>) {
    let shared_dictionary                             = Arc::new( load_dictionary() );
    let (response_sender, response_receiver)          = std::comm::channel();
    let mut request_senders : Vec<Sender<Vec<Vec<u8>>>> = Vec::new();
    for _ in range(0, n_workers) {
        let dictionary                         = shared_dictionary.clone();
        let response_sender                    = response_sender.clone();
        let (request_sender, request_receiver) = std::comm::channel();
        request_senders.push(request_sender);
        // Set up and start worker task
        spawn( proc() {
            response_sender.send( search(dictionary, &request_receiver) );
        });
    }
    (response_receiver, request_senders)
}

static width : uint = 6;        // number of worker tasks
static depth : uint = 200000;   // keys / request sent to worker task

fn main() {
    let args = os::args();
    if args.len() < 2 { fail!("Usage: anagrams letters"); }
    let letters = get_letters(args.get(1).as_slice());

    let (response_receiver, request_senders) = spawn_workers(width);

    // Iterate through the combinations, collecting groups of them
    // (key_set) to be sent to individual workers.
    let mut worker  = 0;
    let mut key_set = Vec::new();
    for i in iter::range(2u,letters.len() + 1) {
        combinations::each_combination(letters.as_slice(), i, |combo| {
            key_set.push( combo.to_owned() );
            if key_set.len() >= depth {
                let mut ks = Vec::new();
                std::mem::swap(&mut ks, &mut key_set);
                request_senders.get(worker).send(ks);
                worker = (worker + 1) % width;
            }
        });
    }
    // Send remaining combinations (key_set) to the next worker.
    if !key_set.is_empty() { request_senders.get(worker).send(key_set); }
    // Send an empty key set to tell each worker to terminate.
    for chan in request_senders.iter() { chan.send(Vec::new()) };

    // Collect responses from workers.
    let mut set : HashSet<String> = HashSet::new();
    for _ in range(0, width) {
        let response_set = response_receiver.recv();
        for word in response_set.iter() { set.insert(word.clone()); }
    }
    println!("{:u}", set.len());
}

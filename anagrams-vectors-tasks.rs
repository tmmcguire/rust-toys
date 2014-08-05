extern crate combinations;
extern crate bisect;

use std::io::{BufferedReader,File};
use std::collections::hashmap::HashSet;

pub fn split_words(s : &str) -> Vec<String> { s.words().map(|w| w.to_string()).collect() }

fn load_dictionary() -> (Vec<Vec<char>>,Vec<Vec<String>>) {
    let path = Path::new("anadict.txt");
    let file = File::open(&path);
    let mut bufferedFile = BufferedReader::new(file);
    let mut keys = Vec::new();
    let mut values = Vec::new();
    for line in bufferedFile.lines() {
        let line = line.unwrap();
        let words = split_words(line.as_slice());
        keys.push( words.get(0).as_slice().chars().collect() );
        values.push( Vec::from_fn(words.len() - 1, |i| words.get(i+1).clone()) );
    }
    return (keys,values);
}

fn get_letters(s : &str) -> Vec<char> {
    let mut t : Vec<char> = s.chars().collect();
    t.sort();
    t
}

fn search(keys         : &[Vec<char>],
          values       : &[Vec<String>],
          request_port : &Receiver<Vec<Vec<char>>>) -> HashSet<String> {
    let klen = keys.len();
    let mut set = HashSet::new();
    loop {
        let key_set = request_port.recv();
        if key_set.len() == 0 { break; }
        for key in key_set.iter() {
            let j = bisect::bisect_left_ref(keys, key, 0, klen);
            if j < klen && keys[j] == *key {
                for word in values[j].iter() { set.insert(word.clone()); }
            }
        }
    }
    return set;
}

fn spawn_workers(width : uint) -> (Receiver<HashSet<String>>, Vec<Sender<Vec<Vec<char>>>>) {
    let (response_sender, response_receiver) = std::comm::channel();
    let mut request_senders : Vec<Sender<Vec<Vec<char>>>> = Vec::new();
    for _ in range(0, width) {
        let (request_chan,request_port) = std::comm::channel();
        request_senders.push(request_chan);
        // Set up and start worker task
        let response_sender = response_sender.clone();
        std::task::spawn (proc() {
            let (keys,values) = load_dictionary();
            response_sender.send( search(keys.as_slice(), values.as_slice(), &request_port) );
        });
    }
    (response_receiver, request_senders)
}

fn main() {
    let width = 6;              // number of worker tasks
    let depth = 500000;         // keys / request sent to worker task

    let args = std::os::args();
    if args.len() < 2 {
        fail!("Usage: anagrams letters");
    }
    let letters = get_letters(args.get(1).as_slice());

    let (response_receiver, request_senders) = spawn_workers(width);

    let mut t = 0;
    let mut key_set = Vec::new();
    for i in range(2u,letters.len() + 1) {
        combinations::each_combination(letters.as_slice(), i, |combo| {
            key_set.push( combo.to_owned() );
            if key_set.len() >= depth {
                let mut ks = Vec::new();
                // ks <-> key_set;
                std::mem::swap(&mut ks, &mut key_set);
                request_senders.get(t).send(ks);
                t = (t + 1) % width;
            }
        });
    }
    if !key_set.is_empty() { request_senders.get(t).send(key_set); }
    for sender in request_senders.iter() { sender.send(Vec::new()) };

    let mut set : HashSet<String> = HashSet::new();
    for _ in range(0, width) {
        let response_set = response_receiver.recv();
        for word in response_set.iter() { set.insert(word.clone()); }
    }
    println!("{:u}", set.len());
}

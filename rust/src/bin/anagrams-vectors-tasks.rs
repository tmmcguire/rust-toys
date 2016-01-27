extern crate combinations;
extern crate bisect;

use std::io::{BufReader,BufRead};
use std::fs::File;
use std::collections::HashSet;
use std::sync::mpsc::{channel,Sender,Receiver};
use std::thread;

pub fn split_words(s : &str) -> Vec<String> { s.split_whitespace().map(|w| w.to_string()).collect() }

fn load_dictionary() -> (Vec<Vec<char>>,Vec<Vec<String>>) {
    let file = File::open("anadict.txt").expect("cannot read anadict.txt");
    let buffered_file = BufReader::new(file);
    let mut keys = Vec::new();
    let mut values = Vec::new();
    for line in buffered_file.lines() {
        let line = line.unwrap();
        let words = split_words(&line);
        keys.push( words[0].chars().collect() );
        values.push( (0..words.len() - 1).map(|i| words[i+1].clone() ).collect() );
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
        let key_set = request_port.recv().unwrap();
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

fn spawn_workers(width : usize) -> (Receiver<HashSet<String>>, Vec<Sender<Vec<Vec<char>>>>) {
    let (response_sender, response_receiver) = channel();
    let mut request_senders : Vec<Sender<Vec<Vec<char>>>> = Vec::new();
    for _ in 0..width {
        let (request_chan,request_port) = channel();
        request_senders.push(request_chan);
        // Set up and start worker task
        let response_sender = response_sender.clone();
        thread::spawn (move || {
            let (keys,values) = load_dictionary();
            response_sender.send( search(&keys, &values, &request_port) ).unwrap();
        });
    }
    (response_receiver, request_senders)
}

fn main() {
    let width = 6;              // number of worker tasks
    let depth = 500000;         // keys / request sent to worker task

    let mut args = std::env::args();
    if args.len() < 2 {
        panic!("Usage: anagrams letters");
    }
    let letters = get_letters(&args.nth(1).unwrap());

    let (response_receiver, request_senders) = spawn_workers(width);

    let mut t = 0;
    let mut key_set = Vec::new();
    for i in 2..letters.len() + 1 {
        combinations::each_combination(&letters, i, |combo| {
            key_set.push( combo.to_owned() );
            if key_set.len() >= depth {
                let mut ks = Vec::new();
                // ks <-> key_set;
                std::mem::swap(&mut ks, &mut key_set);
                request_senders.get(t).unwrap().send(ks).unwrap();
                t = (t + 1) % width;
            }
        });
    }
    if !key_set.is_empty() { request_senders.get(t).unwrap().send(key_set).unwrap(); }
    for sender in request_senders.iter() { sender.send(Vec::new()).unwrap(); };

    let mut set : HashSet<String> = HashSet::new();
    for _ in 0..width {
        let response_set = response_receiver.recv().unwrap();
        for word in response_set.iter() { set.insert(word.clone()); }
    }
    println!("{}", set.len());
}

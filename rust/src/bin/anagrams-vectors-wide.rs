extern crate combinations;
extern crate bisect;

use std::env;
use std::io::{BufReader,BufRead};
use std::fs::File;
use std::sync::mpsc::{channel,Sender,Receiver};
use std::collections::HashSet;
use std::thread;

pub fn split_words(s : &str) -> Vec<String> { s.split_whitespace().map(|w| w.to_string()).collect() }

struct KVPair {
    keys   : Vec<Vec<char>>,
    values : Vec<Vec<String>>
}

fn load_dictionary(width : usize) -> Vec<KVPair> {
    let file = File::open("anadict.txt").expect("cannot read anadict.txt");
    let buffered_file = BufReader::new(file);
    let mut pairs: Vec<KVPair> = (0..width).map(|_| KVPair{keys: Vec::new(), values: Vec::new()}).collect();
    let mut t = 0;

    for line in buffered_file.lines() {
        let line = line.unwrap();
        let words = split_words(&line);
        pairs[t].keys.push( words[0].chars().collect() );
        pairs[t].values.push( words.iter().skip(1).map(|w| w.clone()).collect() );
        t = (t+1) % width;
    }
    return pairs;
}

fn get_letters(s : &str) -> Vec<char> {
    let mut t : Vec<char> = s.chars().collect();
    t.sort();
    t
}

fn search(argument : &[char], keys : &[Vec<char>], values : &[Vec<String>]) -> HashSet<String> {
    let klen = keys.len();
    let mut set = HashSet::new();
    for i in 2..argument.len() + 1 {
        let mut key = vec!['\0'; i];
        combinations::each_combination(argument, i, |combo| {
            for j in 0..combo.len() { key[j] = combo[j]; }
            let j = bisect::bisect_left_ref(keys, &key, 0, klen);
            if j < klen && keys[j] == key {
                for word in values[j].iter() { set.insert(word.clone()); }
            }
        });
    }
    return set;
}

fn main() {
    let width = 3;

    let mut args = env::args();
    if args.len() < 2 { panic!("Usage: anagrams letters"); }
    let letters = get_letters(&args.nth(1).unwrap());

    let (response_sender,response_receiver)
        : (Sender<HashSet<String>>,Receiver<HashSet<String>>) = channel();

    let dictionary = load_dictionary(width);
    for pair in dictionary.iter() {
        let response_sender = response_sender.clone();
        let letters = letters.clone();
        let keys = pair.keys.clone();
        let values = pair.values.clone();
        thread::spawn(move || {
            let set = search(&letters, &keys, &values);
            response_sender.send(set).unwrap();
        });
    }
    let mut set: HashSet<String> = HashSet::new();
    for _ in 0..width {
        let response_set = response_receiver.recv().unwrap();
        for word in response_set.iter() { set.insert(word.clone()); }
    }
    println!("{}", set.len());
}

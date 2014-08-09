extern crate combinations;
extern crate bisect;

use std::{iter,os};
use std::io::{BufferedReader,File};
use std::collections::hashmap::HashSet;

pub fn split_words(s : &str) -> Vec<String> { s.words().map(|w| w.to_string()).collect() }

struct KVPair {
    keys   : Vec<Vec<char>>,
    values : Vec<Vec<String>>
}

fn load_dictionary(width : uint) -> Vec<KVPair> {
    let mut bufferedFile = BufferedReader::new( File::open( &Path::new("anadict.txt") ) );
    let mut pairs = Vec::from_fn(width, |_| KVPair{keys : Vec::new(), values : Vec::new()});
    let mut t = 0;

    for line in bufferedFile.lines() {
        let line = line.unwrap();
        let words = split_words(line.as_slice());
        let pair = pairs.get_mut(t);
        pair.keys.push( words.get(0).as_slice().chars().collect() );
        pair.values.push( words.iter().skip(1).map(|w| w.clone()).collect() );
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
    for i in iter::range(2, argument.len() + 1) {
        let mut key = Vec::from_elem(i, '\0');
        combinations::each_combination(argument, i, |combo| {
            for j in iter::range(0, combo.len()) { *key.get_mut(j) = combo[j]; }
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

    let args = os::args();
    if args.len() < 2 { fail!("Usage: anagrams letters"); }
    let letters = get_letters(args.get(1).as_slice());

    let (response_sender,response_receiver)
        : (Sender<HashSet<String>>,Receiver<HashSet<String>>) = std::comm::channel();

    let dictionary = load_dictionary(width);
    for pair in dictionary.iter() {
        let response_sender = response_sender.clone();
        let letters = letters.clone();
        let keys = pair.keys.clone();
        let values = pair.values.clone();
        spawn( proc() {
            let set = search(letters.as_slice(), keys.as_slice(), values.as_slice());
            response_sender.send(set);
        });
    }
    let mut set = HashSet::new();
    for _ in range(0, width) {
        let response_set = response_receiver.recv();
        for word in response_set.iter() { set.insert(word.clone()); }
    }
    println!("{:u}", set.len());
}

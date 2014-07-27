extern crate combinations;

use std::os;
use std::io::{BufferedReader,File};
use std::collections::hashmap::{HashMap, HashSet};

pub fn split_words(s : &str) -> Vec<String> { s.words().map(|w| w.to_string()).collect() }

fn load_dictionaries(width : uint) -> Vec<HashMap<Vec<char>,Vec<String>>> {
    let mut bufferedFile = BufferedReader::new( File::open( &Path::new("anadict.txt") ) );
    let mut maps = Vec::from_fn(width, |_| HashMap::new());
    let mut t = 0;
    for line in bufferedFile.lines() {
        let line = line.unwrap();
        let words = split_words(line.as_slice());
        let key : Vec<char> = words.get(0).as_slice().chars().collect();
        maps.get_mut(t).insert(Vec::from_fn(key.len(), |i| *key.get(i)),
                               Vec::from_fn(words.len() - 1, |i| words.get(i+1).clone()));
        t = (t+1) % width;
    }
    return maps;
}

fn get_letters(s : &str) -> Vec<char> {
    let mut t : Vec<char> = s.chars().collect();
    t.sort();
    t
}

fn search(letters : &[char], dictionary : &HashMap<Vec<char>,Vec<String>>) -> HashSet<String> {
    let mut set = HashSet::new();
    for i in range(2, letters.len() + 1) {
        let mut key = Vec::from_elem(i, '\x00');
        combinations::each_combination(letters, i, |combo| {
            for j in range(0, combo.len()) { *key.get_mut(j) = combo[j]; }
            match dictionary.find(&key) {
                Some(ref val) => {
                    for word in val.iter() { set.insert(word.clone()); }
                }
                None => { }
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

    let dictionaries = load_dictionaries(width);

    let (sender, receiver) = std::comm::channel();

    for dictionary in dictionaries.iter() {
        let sender = sender.clone();
        let letters = letters.clone();
        let dictionary = dictionary.clone();
        spawn(proc () { sender.send( search(letters.as_slice(), &dictionary) ); });
    }

    let mut set : HashSet<String> = HashSet::new();
    for _ in range(0, width) {
        let response_set = receiver.recv();
        for word in response_set.iter() { set.insert(word.clone()); }
    }
    println!("{:u}", set.len());
}

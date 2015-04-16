extern crate combinations;
extern crate bisect;

use std::os;
use std::io::{File,BufferedReader};
use std::collections::hashmap::HashSet;

fn split_words(s : &str) -> Vec<String> { s.words().map(|w| w.to_string()).collect() }

fn load_dictionary() -> (Vec<Vec<char>>,Vec<Vec<String>>) {
    let mut bufferedFile = BufferedReader::new( File::open( &Path::new("anadict.txt") ) );
    let mut keys = Vec::new();
    let mut values = Vec::new();
    for line in bufferedFile.lines() {
        let line = line.unwrap();
        let words = split_words(line.as_slice());
        keys.push( words.get(0).as_slice().chars().collect() );
        values.push( words.iter().skip(1).map(|w| w.clone()).collect() );
    }
    return (keys,values);
}

fn get_letters(s : &str) -> Vec<char> {
    let mut t : Vec<char> = s.chars().collect();
    t.sort();
    return t;
}

pub fn main() {
    let args = os::args();
    if args.len() < 2 { fail!("Usage: anagrams letters"); }
    let letters = get_letters(args.get(1).as_slice());
    let (keys,values) = load_dictionary();
    let klen = keys.len();
    let mut set = HashSet::new();
    for i in range(2u, letters.len() + 1) {
        let mut key = Vec::from_elem(i, '\x00');
        combinations::each_combination(letters.as_slice(), i, |combo| {
            for j in range(0, combo.len()) { *key.get_mut(j) = combo[j]; }
            let j = bisect::bisect_left_ref(keys.as_slice(), &key, 0, klen);
            if j < klen && *keys.get(j) == key {
                for word in values.get(j).iter() { set.insert(word.clone()); }
            }
        });
    }
    println!("{:u}", set.len());
}

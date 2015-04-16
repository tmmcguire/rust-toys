extern crate combinations;
extern crate bisect;

use std::default::Default;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead,BufReader,Read};
use std::path::Path;

fn split_words(s : &str) -> Vec<String> { s.split(" ").map(|w| w.to_string()).collect() }

fn load_dictionary() -> (Vec<Vec<char>>,Vec<Vec<String>>) {
    let buffered_file = BufReader::new( File::open( &Path::new("anadict.txt") ).unwrap() );
    let mut keys = Vec::new();
    let mut values = Vec::new();
    for line in buffered_file.lines() {
        let line = line.unwrap();
        let words = split_words(&line);
        keys.push( words[0].chars().collect() );
        values.push( words.iter().skip(1).map(|w| w.clone()).collect() );
    }
    (keys,values)
}

fn get_letters(s : &str) -> Vec<char> {
    let mut t : Vec<char> = s.chars().collect();
    t.sort();
    t
}

pub fn main() {
    let args : Vec<String> = std::env::args().collect();
    if args.len() < 2 { panic!("Usage: anagrams letters"); }
    let letters = get_letters(&args[1]);
    let (keys,values) = load_dictionary();
    let klen = keys.len();
    let mut set = HashSet::new();
    for i in 2..letters.len() + 1 {
        let mut key : Vec<char> = vec![Default::default(); i];
        combinations::each_combination(&letters, i, |combo| {
            for j in 0..combo.len() { key[j] = combo[j]; }
            let j = bisect::bisect_left_ref(&keys, &key, 0, klen);
            if j < klen && keys[j] == key {
                for word in values[j].iter() { set.insert(word.clone()); }
            }
        });
    }
    println!("{}", set.len());
}

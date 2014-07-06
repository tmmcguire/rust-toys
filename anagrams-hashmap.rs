extern crate combinations;

use std::{iter,os};
use std::io::{File,BufferedReader};
use std::path::Path;
use std::collections::hashmap::{HashMap, HashSet};
// use extra::time;

// fn duration(tag : &str, start : time::Timespec, end : time::Timespec) {
//     let d_sec = end.sec - start.sec;
//     let d_nsec = end.nsec - start.nsec;
//     if d_nsec >= 0 {
//         println!("{:s}: {:?}", tag, time::Timespec { sec : d_sec, nsec : d_nsec });
//     } else {
//         println!("{:s}: {:?}", tag, time::Timespec { sec : d_sec - 1, nsec : d_nsec + 1000000000 });
//     }
// }

pub fn split_words(s : &str) -> Vec<String> { s.words().map(|w| w.to_string()).collect() }

fn load_dictionary() -> HashMap<Vec<i8>,Vec<String>> {
    let path = Path::new("anadict.txt");
    let file = File::open(&path);
    let mut bufferedFile = BufferedReader::new(file);
    let mut map = HashMap::new();
    for line in bufferedFile.lines() {
        let line = line.unwrap();
        let words = split_words(line.as_slice());
        let key : Vec<char> = words.get(0).as_slice().chars().collect();
        map.insert(Vec::from_fn(key.len(), |i| *key.get(i) as i8),
                   Vec::from_fn(words.len() - 1, |i| words.get(i+1).clone()));
    }
    return map;
}

fn get_letters(s : &str) -> Vec<i8> {
    let mut t : Vec<char> = s.chars().collect();
    t.sort();
    return Vec::from_fn(t.len(), |i| *t.get(i) as i8);
}

fn search(letters : &[i8], dictionary : &HashMap<Vec<i8>,Vec<String>>) -> HashSet<String>
{
    let mut set = HashSet::new();
    for i in iter::range(0, letters.len() + 1) {
        // let start = time::get_time();
        let mut key : Vec<i8> = Vec::from_elem(i, 0i8);
        combinations::each_combination(letters, i, |combo| {
            let combo : &[i8] = combo;
            for j in iter::range(0, combo.len()) { *key.get_mut(j) = combo[j]; }
            match dictionary.find(&key) {
                Some(ref val) => {
                    for word in val.iter() { set.insert(word.clone()); }
                }
                None => { }
            }
        });
        // duration("iteration", start, time::get_time());
    }
    return set;
}

fn main() {
    // let start = time::get_time();
    let args = os::args();
    if args.len() < 2 { fail!("Usage: anagrams letters"); }
    let letters = get_letters(args.get(1).as_slice());
    // duration("get_letters", start, time::get_time());
    let dictionary = load_dictionary();
    // duration("load_dictionary", start, time::get_time());
    let set = search(letters.as_slice(),&dictionary);
    // duration("search", start, time::get_time());
    println!("{:u}", set.len());
}

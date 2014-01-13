extern mod combinations;

use std::{vec,os};
use std::io::File;
use std::io::buffered::BufferedReader;
use std::hashmap::{HashMap, HashSet};

pub fn split_words(s : &str) -> ~[~str] { s.words().map(|w| w.to_owned()).collect() }

fn load_dictionaries(width : uint) -> ~[~HashMap<~[u8],~[~str]>] {
    let mut bufferedFile = BufferedReader::new( File::open( &Path::new("anadict.txt") ) );
    let mut maps = vec::from_fn(width, |_| ~HashMap::new());
    let mut t = 0;
    for line in bufferedFile.lines() {
        let words = split_words(line);
        let key : ~[char] = words[0].chars().collect();
        maps[t].insert(vec::from_fn(key.len(), |i| key[i] as u8),
                       vec::from_fn(words.len() - 1, |i| words[i+1].clone()));
        t = (t+1) % width;
    }
    return maps;
}

fn get_letters(s : &str) -> ~[u8] {
    let mut t : ~[char] = s.chars().collect();
    t.sort();
    return vec::from_fn(t.len(), |i| t[i] as u8);
}

fn search(letters : &[u8], dictionary : &HashMap<~[u8],~[~str]>) -> ~HashSet<~str> {
    let mut set = ~HashSet::new();
    for i in range(2, letters.len() + 1) {
        let mut key = vec::from_elem(i, 0u8);
        combinations::each_combination(letters, i, |combo| {
            for j in range(0, combo.len()) { key[j] = combo[j]; }
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
    if args.len() < 2 { fail!(~"Usage: anagrams letters"); }
    let letters = get_letters(args[1]);

    let dictionaries = load_dictionaries(width);

    let (response_port, response_chan) = SharedChan::new();

    for dictionary in dictionaries.iter() {
        let response_chan = response_chan.clone();
        let letters = letters.clone();
        let dictionary = dictionary.clone();
        do spawn {
            response_chan.send( search(letters, dictionary) );
        }
    }

    let mut set : ~HashSet<~str> = ~HashSet::new();
    for _ in range(0, width) {
        let response_set = response_port.recv();
        for word in response_set.iter() { set.insert(word.clone()); }
        // 0.7, 0.8, 0.9:
        // for response_port.recv().iter().advance |&word| { set.insert(word); }
        // produces:
        // anagrams-hashmap-wide.rs:81:12: 81:33 error: borrowed value does not live long enough
        // anagrams-hashmap-wide.rs:81         for response_port.recv().iter().advance |&word| { set.insert(word); }
        //                                         ^~~~~~~~~~~~~~~~~~~~~
        // anagrams-hashmap-wide.rs:81:76: 81:77 note: borrowed pointer must be valid for the method call at 81:76...
        // anagrams-hashmap-wide.rs:81         for response_port.recv().iter().advance |&word| { set.insert(word); }
        //                                                                                                         ^
        // anagrams-hashmap-wide.rs:81:12: 81:40 note: ...but borrowed value is only valid for the method call at 81:12
        // anagrams-hashmap-wide.rs:81         for response_port.recv().iter().advance |&word| { set.insert(word); }
        //                                         ^~~~~~~~~~~~~~~~~~~~~~~~~~~~
    }
    println!("{:u}", set.len());
}

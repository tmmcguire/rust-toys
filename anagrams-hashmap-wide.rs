extern mod extra;

use std::{vec,os};
use std::comm::*;
use std::io::*;
use std::hashmap::*;

mod combinations;
mod bisect;

pub fn split_words(s : &str) -> ~[~str] { s.word_iter().map(|w| w.to_owned()).collect() }

fn load_dictionaries(width : uint) -> ~[~HashMap<~[u8],~[~str]>] {
    match file_reader(&Path("anadict.txt")) {
        Ok(reader) => {
            let mut maps = vec::from_fn(width, |_| ~HashMap::new());
            let mut t = 0;
            reader.each_line(|line| {
                    let words = split_words(line);
                    let key : ~[char] = words[0].iter().collect();
                    maps[t].insert(vec::from_fn(key.len(), |i| key[i] as u8),
                                   vec::from_fn(words.len() - 1, |i| words[i+1].clone()));
                    t = (t + 1) % width;
                    true
                });
            return maps;
        }
        Err(msg) => { fail!(msg); }
    }
}

fn get_letters(s : &str) -> ~[u8] {
    let mut t : ~[char] = s.iter().collect();
    extra::sort::quick_sort(t, |a,b| *a <= *b);
    return vec::from_fn(t.len(), |i| t[i] as u8);
}

fn search(letters : &[u8], dictionary : &HashMap<~[u8],~[~str]>) -> ~HashSet<~str> {
    let mut set = ~HashSet::new();
    for i in range(2, letters.len() + 1) {
        let mut key = vec::from_elem(i, 0u8);
        do combinations::each_combination(letters,i) |combo| {
            for j in range(0, combo.len()) { key[j] = combo[j]; }
            match dictionary.find(&key) {
                Some(ref val) => {
                    for word in val.iter() { set.insert(word.clone()); }
                }
                None => { }
            }
        }
    }
    return set;
}

fn main() {
    let width = 6;

    let args = os::args();
    if args.len() < 2 { fail!(~"Usage: anagrams letters"); }
    let letters = get_letters(args[1]);

    let dictionaries = load_dictionaries(width);

    let (response_port,response_chan) = stream();
    let response_chan = SharedChan::new(response_chan);

    for dictionary in dictionaries.iter() {
        let response_chan = response_chan.clone();
        let letters = letters.clone();
        let dictionary = dictionary.clone();
        do spawn {
            response_chan.send( search(letters, dictionary) );
        }
    }

    let mut set : ~HashSet<~str> = ~HashSet::new();
    do width.times() {
        let response_set = response_port.recv();
        for word in response_set.iter() { set.insert(word.clone()); }
        // 0.7:
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

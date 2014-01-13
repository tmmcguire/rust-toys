extern mod combinations;
extern mod bisect;

use std::{vec,iter,os};
use std::io::File;
use std::io::buffered::BufferedReader;
use std::hashmap::HashSet;

pub fn split_words(s : &str) -> ~[~str] { s.words().map(|w| w.to_owned()).collect() }

struct kv_pair {
    keys   : ~[~[u8]],
    values : ~[~[~str]]
}

fn load_dictionary(width : uint) -> ~[kv_pair] {
    let mut bufferedFile = BufferedReader::new( File::open( &Path::new("anadict.txt") ) );
    let mut pairs = vec::from_fn(width, |_| kv_pair{keys : ~[], values : ~[]});
    let mut t = 0;

    for line in bufferedFile.lines() {
        let words = split_words(line);
        pairs[t].keys.push( vec::from_fn(words[0].len(), |i| words[0][i] as u8) );
        pairs[t].values.push( vec::from_fn(words.len() - 1, |i| words[i+1].clone()) );
        t = (t+1) % width;
    }
    return pairs;
}

fn get_letters(s : &str) -> ~[u8] {
    let mut t : ~[char] = s.chars().collect();
    t.sort();
    return vec::from_fn(t.len(), |i| t[i] as u8);
}

fn search(argument : &[u8], keys : &[~[u8]], values : &[~[~str]]) -> ~HashSet<~str> {
    let klen = keys.len();
    let mut set = ~HashSet::new();
    for i in iter::range(2, argument.len() + 1) {
        let mut key = vec::from_elem(i, 0u8);
        combinations::each_combination(argument,i, |combo| {
            for j in iter::range(0, combo.len()) { key[j] = combo[j]; }
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
    if args.len() < 2 { fail!(~"Usage: anagrams letters"); }
    let letters = get_letters(args[1]);

    let (response_port,response_chan)
        : (Port<~HashSet<~str>>,SharedChan<~HashSet<~str>>) = SharedChan::new();

    let dictionary = load_dictionary(width);
    for kv_pair in dictionary.iter() {
        let response_chan = response_chan.clone();
        let letters = letters.clone();
        let keys = kv_pair.keys.clone();
        let values = kv_pair.values.clone();
        do spawn {
            let set = search(letters, keys, values);
            response_chan.send(set);
        }
    }
    let mut set = ~HashSet::new();
    for _ in range(0, width) {
        let response_set = response_port.recv();
        for word in response_set.iter() { set.insert(word.clone()); }
    }
    println!("{:u}", set.len());
}

extern mod combinations;
extern mod bisect;

use std::{vec,os};
use std::io::File;
use std::io::buffered::BufferedReader;
use std::hashmap::HashSet;

pub fn split_words(s : &str) -> ~[~str] { s.words().map(|w| w.to_owned()).collect() }

fn load_dictionary() -> (~[~[u8]],~[~[~str]]) {
    let mut bufferedFile = BufferedReader::new( File::open( &Path::new("anadict.txt") ) );
    let mut keys = ~[];
    let mut values = ~[];
    for line in bufferedFile.lines() {
        let words = split_words(line);
        keys.push( vec::from_fn(words[0].len(), |i| words[0][i] as u8) );
        values.push( vec::from_fn(words.len() - 1, |i| words[i+1].clone()) );
    }
    return (keys,values);
}

fn get_letters(s : &str) -> ~[u8] {
    let mut t : ~[char] = s.chars().collect();
    t.sort();
    return vec::from_fn(t.len(), |i| t[i] as u8);
}

fn main() {
    let args = os::args();
    if args.len() < 2 { fail!(~"Usage: anagrams letters"); }
    let letters = get_letters(args[1]);
    let (keys,values) = load_dictionary();
    let klen = keys.len();
    let mut set = HashSet::new();
    for i in range(2, letters.len() + 1) {
        let mut key = vec::from_elem(i, 0u8);
        combinations::each_combination(letters, i, |combo| {
            for j in range(0, combo.len()) { key[j] = combo[j]; }
            let j = bisect::bisect_left_ref(keys, &key, 0, klen);
            if j < klen && keys[j] == key {
                for word in values[j].iter() { set.insert(word.clone()); }
            }
        });
    }
    println!("{:u}", set.len());
}

extern mod extra;

extern mod combinations;
extern mod bisect;

use std::{vec,uint,os};
use std::io::*;
use std::hashmap::*;

pub fn split_words(s : &str) -> ~[~str] { s.word_iter().transform(|w| w.to_owned()).collect() }

fn load_dictionary() -> (~[~[u8]],~[~[~str]]) {
    match file_reader(&Path("anadict.txt")) {
        Ok(reader) => {
            let mut keys = ~[];
            let mut values = ~[];
            for reader.each_line() |line| {
                let words = split_words(line);
                keys.push( vec::from_fn(words[0].len(), |i| words[0][i] as u8) );
                values.push( vec::from_fn(words.len() - 1, |i| copy words[i+1]) );
            }
            return (keys,values);
        }
        Err(msg) => { fail!(msg); }
    }
}

fn get_letters(s : &str) -> ~[u8] {
    let mut t : ~[char] = s.iter().collect();
    extra::sort::quick_sort(t, |a,b| *a <= *b);
    return vec::from_fn(t.len(), |i| t[i] as u8);
}

fn main() {
    let args = os::args();
    if args.len() < 2 { fail!(~"Usage: anagrams letters"); }
    let letters = get_letters(args[1]);
    let (keys,values) = load_dictionary();
    let klen = keys.len();
    let mut set = HashSet::new();
    for uint::iterate(2, letters.len() + 1) |i| {
        let mut key = vec::from_elem(i, 0);
        for combinations::each_combination(letters,i) |combo| {
            for uint::iterate(0, combo.len()) |j| { key[j] = combo[j]; }
            let j = bisect::bisect_left_ref(keys, &key, 0, klen);
            if j < klen && keys[j] == key {
                for values[j].iter().advance |&word| { set.insert(word); }
            }
        }
    }
    println(fmt!("%u", set.len()));
}

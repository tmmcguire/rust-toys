extern mod extra;

extern mod combinations;
extern mod bisect;

use std::{vec,uint,os};
use std::comm::*;
use std::io::*;
use std::hashmap::*;

pub fn split_words(s : &str) -> ~[~str] { s.word_iter().transform(|w| w.to_owned()).collect() }

struct kv_pair {
    keys   : ~[~[u8]],
    values : ~[~[~str]]
}

fn load_dictionary(width : uint) -> ~[kv_pair] {
    match file_reader(&Path("anadict.txt")) {
        Ok(reader) => {
            let mut t = 0;
            let mut pairs = vec::from_fn(width, 
                                         |_| kv_pair{keys   : ~[],
                                                     values : ~[]});
            for reader.each_line() |line| {
                let words = split_words(line);
                pairs[t].keys.push(
                    vec::from_fn(words[0].len(), |i| words[0][i] as u8)
                );
                pairs[t].values.push(
                    vec::from_fn(words.len() - 1, |i| copy words[i+1])
                );
                t = (t + 1) % width;
            }
            return pairs;
        }
        Err(msg) => { fail!(msg); }
    }
}

fn get_letters(s : &str) -> ~[u8] {
    let mut t : ~[char] = s.iter().collect();
    extra::sort::quick_sort(t, |a,b| *a <= *b);
    return vec::from_fn(t.len(), |i| t[i] as u8);
}

fn search(argument : &[u8], keys : &[~[u8]], values : &[~[~str]]) -> ~HashSet<~str> {
    let klen = keys.len();
    let mut set = ~HashSet::new();
    for uint::range(2, argument.len() + 1) |i| {
        let mut key = vec::from_elem(i, 0);
        for combinations::each_combination(argument,i) |combo| {
            for uint::iterate(0, combo.len()) |j| { key[j] = combo[j]; }
            let j = bisect::bisect_left_ref(keys, &key, 0, klen);
            if j < klen && keys[j] == key {
                for values[j].iter().advance |&word| { set.insert(word); }
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

    let (response_port,response_chan)
        : (Port<~HashSet<~str>>,Chan<~HashSet<~str>>) = stream();
    let response_chan = SharedChan::new(response_chan);

    let dictionary = load_dictionary(width);
    for dictionary.iter().advance |&kv_pair{keys   : keys,
                                            values : values}| {
        let response_chan = response_chan.clone();
        let letters = copy letters;
        do spawn {
            let set = search(letters, keys, values);
            response_chan.send(set);
        }
    }
    let mut set = ~HashSet::new();
    for width.times {
        let response_set = response_port.recv();
        for response_set.iter().advance |&word| { set.insert(word); }
    }
    println(fmt!("%u", set.len()));
}

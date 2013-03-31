extern mod std;

mod combinations;
mod bisect;

use core::comm::*;
use core::io::*;
use core::hashmap::linear::*;
use core::cmp::{Eq, Ord};
use core::task::*;

struct kv_pair {
    keys   : ~[~[int]],
    values : ~[~[~str]]
}

fn load_dictionary(width : uint) -> ~[kv_pair] {
    match file_reader(&Path("anadict-rust.txt")) {
        Ok(reader) => {
            let mut t = 0;
            let mut pairs = vec::from_fn(width, 
                                         |_| kv_pair{keys   : ~[],
                                                     values : ~[]});
            for reader.each_line() |line| {
                let words = line.split_str(" ");
                pairs[t].keys.push(
                    vec::from_fn(words[0].len(), |i| words[0][i] as int)
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

fn get_letters(s : &str) -> ~[int] {
    let mut t = str::chars(s);
    std::sort::quick_sort(t, |a,b| *a <= *b);
    return vec::from_fn(t.len(), |i| t[i] as int);
}

fn search(argument : &[int], keys : &[~[int]], values : &[~[~str]]) -> ~LinearSet<~str> {
    let klen = keys.len();
    let mut set = ~LinearSet::new();
    for uint::range(2, argument.len() + 1) |i| {
        let mut key = vec::from_elem(i, 0);
        for combinations::each_combination(argument,i) |combo| {
            for uint::range(0,i) |j| { key[j] = combo[j]; }
            let j = bisect::bisect_left_ref(keys, &key, 0, klen);
            if j < klen && keys[j] == key {
                for values[j].each |&word| { set.insert(word); }
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
        : (Port<~LinearSet<~str>>,Chan<~LinearSet<~str>>) = stream();
    let response_chan = SharedChan(response_chan);

    for load_dictionary(width).each |&kv_pair{keys   : keys,
                                              values : values}| {
        let response_chan = response_chan.clone();
        let letters = copy letters;
        do spawn {
            let set = search(letters, keys, values);
            response_chan.send(set);
        }
    }
    let mut set = ~LinearSet::new();
    for uint::range(0,width) |_| {
        let res = response_port.recv();
        for res.each |&word| { set.insert(word); }
    }
    println(fmt!("%u", set.len()));
}

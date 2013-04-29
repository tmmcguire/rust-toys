extern mod std;

extern mod combinations;
extern mod bisect;
extern mod misc;

use core::comm::*;
use core::io::*;
use core::hashmap::linear::*;

struct kv_pair {
    keys   : ~[~[u8]],
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
                let words = misc::split_words(line);
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
    let mut t = str::to_chars(s);
    std::sort::quick_sort(t, |a,b| *a <= *b);
    return vec::from_fn(t.len(), |i| t[i] as u8);
}

fn search(argument : &[u8], keys : &[~[u8]], values : &[~[~str]]) -> ~LinearSet<~str> {
    let klen = keys.len();
    let mut set = ~LinearSet::new();
    for uint::range(2, argument.len() + 1) |i| {
        let mut key = vec::from_elem(i, 0);
        for combinations::each_combination(argument,i) |combo| {
            for combo.eachi |j,&ch| { key[j] = ch; }
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
    for width.times {
        for response_port.recv().each |&word| { set.insert(word); }
    }
    println(fmt!("%u", set.len()));
}

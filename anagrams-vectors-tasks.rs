extern mod std;

use core::cmp::{Eq, Ord};
use core::comm::*;
use core::io::*;
use core::hashmap::linear::*;
use core::task::spawn;

mod combinations;
mod bisect;

fn load_dictionary() -> (~[~[int]],~[~[~str]]) {
    match file_reader(&Path("anadict-rust.txt")) {
        Ok(reader) => {
            let mut keys = ~[];
            let mut values = ~[];
            for reader.each_line() |line| {
                let words = line.split_str(" ");
                keys.push( vec::from_fn(words[0].len(), |i| words[0][i] as int) );
                values.push( vec::from_fn(words.len() - 1, |i| copy words[i+1]) );
            }
            return (keys,values);
        }
        Err(msg) => { fail!(msg); }
    }
}

fn get_letters(s : &str) -> ~[int] {
    let mut t = str::chars(s);
    std::sort::quick_sort(t, |a,b| *a <= *b);
    return vec::from_fn(t.len(), |i| t[i] as int);
}

fn main() {
    let width = 6;
    let depth = 10000;

    let args = os::args();
    if args.len() < 2 {
        fail!(~"Usage: anagrams letters");
    }
    let letters = get_letters(args[1]);

    let (keys,values) = load_dictionary();

    let (response_port,response_chan)
        : (Port<~LinearSet<~str>>,Chan<~LinearSet<~str>>) = stream();
    let response_chan = SharedChan(response_chan);

    let request_streams   : ~[(Port<~[~[int]]>,Chan<~[~[int]]>)]
        = vec::from_fn(width, |_| stream());
    let mut request_chans : ~[Chan<~[~[int]]>] = ~[];
    for request_streams.each |&(request_port, request_chan)| {
        request_chans.push(request_chan);
        let response_chan = response_chan.clone();
        let keys = copy keys;
        let values = copy values;
        do spawn {
            let klen = keys.len();
            let mut set = ~LinearSet::new();
            loop {
                let key_set = request_port.recv();
                if key_set.len() == 0 { break; }
                for key_set.each |key| {
                    let j = bisect::bisect_left_ref(keys, key, 0, klen);
                    if j < klen && keys[j] == *key {
                        for values[j].each |&word| {
                            set.insert(copy word);
                        }
                    }
                }
            }
            response_chan.send(set);
        }
    }

    let mut t = 0;
    let mut key_set = ~[];
    for uint::range(2,letters.len() + 1) |i| {
        for combinations::each_combination(letters,i) |combo| {
            let key = vec::from_fn(i, |i| combo[i]);
            key_set.push(key);
            if key_set.len() >= depth {
                let mut ks = ~[];
                ks <-> key_set;
                request_chans[t].send(ks);
                t = (t + 1) % width;
            }
        }
    }
    if !key_set.is_empty() { request_chans[t].send(key_set); }
    for request_chans.each |chan| { chan.send(~[]) };

    let mut set : ~LinearSet<~str> = ~LinearSet::new();
    for uint::range(0,width) |_| {
        let res = response_port.recv();
        for res.each |&word| { set.insert(word); }
    }
    println(fmt!("%u", set.len()));
}

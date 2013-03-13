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

    let args = os::args();
    if args.len() < 2 {
        fail!(~"Usage: anagrams letters");
    }
    let letters = get_letters(args[1]);

    let (response_port,response_chan) : (Port<~LinearSet<~str>>,Chan<~LinearSet<~str>>) = stream();
    let response_chan = SharedChan(response_chan);

    let request_streams   : ~[(Port<~[int]>,Chan<~[int]>)] = vec::from_fn(6, |_| stream());
    let mut request_chans : ~[Chan<~[int]>]                = ~[];
    for request_streams.each |&(request_port, request_chan)| {

        request_chans.push(request_chan);
        let response_chan = response_chan.clone();

        do spawn {

            let (keys,values) = load_dictionary();
            let klen = keys.len();
            let mut set : ~LinearSet<~str> = ~LinearSet::new();

            loop {
                let key = request_port.recv();
                if key.len() == 0 { break; }
                let j = bisect::bisect_left_ref(keys, &key, 0, klen);
                if j < klen && keys[j] == key {
                    for values[j].each |&word| {
                        set.insert(copy word);
                    }
                }
            }
            println(fmt!("    %u", set.len()));
            response_chan.send(set);
        }
    }

    let mut t = 0;
    for uint::range(2,letters.len() + 1) |i| {
        // let mut key = vec::from_elem(i, 0);
        for combinations::each_combination(letters,i) |combo| {
            // for uint::range(0,i) |j| { key[j] = combo[j]; }
            let key = vec::from_fn(i, |i| combo[i]);
            request_chans[t].send(key);
            t = (t + 1) % 6;
        }
    }
    for request_chans.each |chan| { chan.send(~[]) };
    let mut set : ~LinearSet<~str> = ~LinearSet::new();
    for uint::range(0,6) |_| {
        let res = response_port.recv();
        for res.each |&word| { set.insert(word); }
    }
    println(fmt!("%u", set.len()));

}

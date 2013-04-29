extern mod std;

extern mod combinations;
extern mod bisect;
extern mod misc;

use core::comm::*;
use core::io::*;
use core::hashmap::linear::*;
use core::task::spawn;

fn load_dictionary() -> (~[~[u8]],~[~[~str]]) {
    match file_reader(&Path("anadict-rust.txt")) {
        Ok(reader) => {
            let mut keys = ~[];
            let mut values = ~[];
            for reader.each_line() |line| {
                let words = misc::split_words(line);
                keys.push( vec::from_fn(words[0].len(), |i| words[0][i] as u8) );
                values.push( vec::from_fn(words.len() - 1, |i| copy words[i+1]) );
            }
            return (keys,values);
        }
        Err(msg) => { fail!(msg); }
    }
}

fn get_letters(s : &str) -> ~[u8] {
    let mut t = str::to_chars(s);
    std::sort::quick_sort(t, |a,b| *a <= *b);
    return vec::from_fn(t.len(), |i| t[i] as u8);
}

fn search(keys         : &[~[u8]],
          values       : &[~[~str]],
          request_port : &Port<~[~[u8]]>) -> ~LinearSet<~str> {
    let klen = keys.len();
    let mut set = ~LinearSet::new();
    loop {
        let key_set = request_port.recv();
        if key_set.len() == 0 { break; }
        for key_set.each |key| {
            let j = bisect::bisect_left_ref(keys, key, 0, klen);
            if j < klen && keys[j] == *key {
                for values[j].each |&word| { set.insert(copy word); }
            }
        }
    }
    return set;
}

fn main() {
    let width = 6;              // number of worker tasks
    let depth = 10000;          // keys / request sent to worker task

    let args = os::args();
    if args.len() < 2 {
        fail!(~"Usage: anagrams letters");
    }
    let letters = get_letters(args[1]);

    let (response_port,response_chan) = stream();
    let response_chan = SharedChan(response_chan);

    let mut request_chans : ~[Chan<~[~[u8]]>] = ~[];
    for width.times {
        let (request_port,request_chan) = stream();
        request_chans.push(request_chan);
        // Set up and start worker task
        let response_chan = response_chan.clone();
        do spawn {
            let (keys,values) = load_dictionary();
            response_chan.send( search(keys, values, &request_port) );
        }
    }

    let mut t = 0;
    let mut key_set = ~[];
    for uint::range(2,letters.len() + 1) |i| {
        for combinations::each_combination(letters,i) |combo| {
            key_set.push( vec::from_slice(combo) );
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
    for width.times {
        for response_port.recv().each |&word| { set.insert(word); }
    }
    println(fmt!("%u", set.len()));
}

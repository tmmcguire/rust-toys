extern mod combinations;
extern mod bisect;

use std::{os,vec,util};
use std::io::File;
use std::io::buffered::BufferedReader;
use std::hashmap::HashSet;
use std::task::spawn;

pub fn split_words(s : &str) -> ~[~str] { s.words().map(|w| w.to_owned()).collect() }

fn load_dictionary() -> (~[~[u8]],~[~[~str]]) {
    let path = Path::new("anadict.txt");
    let file = File::open(&path);
    let mut bufferedFile = BufferedReader::new(file);
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

fn search(keys         : &[~[u8]],
          values       : &[~[~str]],
          request_port : &Port<~[~[u8]]>) -> ~HashSet<~str> {
    let klen = keys.len();
    let mut set = ~HashSet::new();
    loop {
        let key_set = request_port.recv();
        if key_set.len() == 0 { break; }
        for key in key_set.iter() {
            let j = bisect::bisect_left_ref(keys, key, 0, klen);
            if j < klen && keys[j] == *key {
                for word in values[j].iter() { set.insert(word.clone()); }
            }
        }
    }
    return set;
}

fn spawn_workers(width : uint) -> (Port<~HashSet<~str>>, ~[Chan<~[~[u8]]>]) {
    let (response_port, response_chan) = SharedChan::new();
    let mut request_chans : ~[Chan<~[~[u8]]>] = ~[];
    for _ in range(0, width) {
        let (request_port,request_chan) = Chan::new();
        request_chans.push(request_chan);
        // Set up and start worker task
        let response_chan = response_chan.clone();
        do spawn {
            let (keys,values) = load_dictionary();
            response_chan.send( search(keys, values, &request_port) );
        }
    }
    (response_port, request_chans)
}

fn main() {
    let width = 6;              // number of worker tasks
    let depth = 500000;         // keys / request sent to worker task

    let args = os::args();
    if args.len() < 2 {
        fail!(~"Usage: anagrams letters");
    }
    let letters = get_letters(args[1]);

    let (response_port, request_chans) = spawn_workers(width);

    let mut t = 0;
    let mut key_set = ~[];
    for i in range(2,letters.len() + 1) {
        combinations::each_combination(letters, i, |combo| {
            key_set.push( combo.to_owned() );
            if key_set.len() >= depth {
                let mut ks = ~[];
                // ks <-> key_set;
                util::swap(&mut ks, &mut key_set);
                request_chans[t].send(ks);
                t = (t + 1) % width;
            }
        });
    }
    if !key_set.is_empty() { request_chans[t].send(key_set); }
    for chan in request_chans.iter() { chan.send(~[]) };

    let mut set : ~HashSet<~str> = ~HashSet::new();
    for _ in range(0, width) {
        let response_set = response_port.recv();
        for word in response_set.iter() { set.insert(word.clone()); }
    }
    println!("{:u}", set.len());
}

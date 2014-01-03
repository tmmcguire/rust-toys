extern mod extra;

extern mod combinations;
extern mod bisect;

use std::{vec,iter,os,util};
use std::comm;
use std::comm::SharedChan;
use std::io::*;
use std::hashmap::*;
use std::task::spawn;
use extra::time;

fn duration(tag : &str, start : time::Timespec, end : time::Timespec) {
    let d_sec = end.sec - start.sec;
    let d_nsec = end.nsec - start.nsec;
    if d_nsec >= 0 {
        println!("{:s}: {:?}", tag, time::Timespec { sec : d_sec, nsec : d_nsec });
    } else {
        println!("{:s}: {:?}", tag, time::Timespec { sec : d_sec - 1, nsec : d_nsec + 1000000000 });
    }
}

pub fn split_words(s : &str) -> ~[~str] { s.word_iter().map(|w| w.to_owned()).collect() }

fn load_dictionary() -> ~HashMap<~[u8],~[~str]> {
    match file_reader(&Path("anadict.txt")) {
        Ok(reader) => {
            let mut map = ~HashMap::new();
            reader.each_line(|line| {
                    let words = split_words(line);
                    let key : ~[char] = words[0].iter().collect();
                    map.insert(vec::from_fn(key.len(),       |i| key[i] as u8),
                               vec::from_fn(words.len() - 1, |i| words[i+1].clone()));
                    true
                });
            return map;
        }
        Err(msg) => { fail!(msg); }
    }
}

fn get_letters(s : &str) -> ~[u8] {
    let mut t : ~[char] = s.iter().collect();
    extra::sort::quick_sort(t, |a,b| *a <= *b);
    return vec::from_fn(t.len(), |i| t[i] as u8);
}

fn search(dictionary : &HashMap<~[u8],~[~str]>, request_port : &Port<~[~[u8]]>) -> ~HashSet<~str> {
    let mut set = ~HashSet::new();
    loop {
        let key_set = request_port.recv();
        if key_set.len() == 0 { break; }
        for key in key_set.iter() {
            match dictionary.find(key) {
                Some(ref val) => {
                    for word in val.iter() { set.insert(word.clone()); }
                }
                None => { }
            }
        }
    }
    return set;
}

fn spawn_workers(n_workers : uint) -> (Port<~HashSet<~str>>,~[Chan<~[~[u8]]>]) {
    let (response_port, response_chan) = comm::stream();
    let response_chan = SharedChan::new(response_chan);
    let mut request_chans : ~[Chan<~[~[u8]]>] = ~[];
    do n_workers.times() {
        let (request_port,request_chan) = comm::stream();
        request_chans.push(request_chan);
        // Set up and start worker task
        let response_chan = response_chan.clone();
        do spawn {
            let dictionary = load_dictionary();
            response_chan.send( search(dictionary, &request_port) );
        }
    }
    (response_port, request_chans)
}

static width : uint = 6;        // number of worker tasks
static depth : uint = 10000;    // keys / request sent to worker task

fn main() {
    let args = os::args();
    if args.len() < 2 {
        fail!(~"Usage: anagrams letters");
    }
    let letters = get_letters(args[1]);

    let (response_port, request_chans) = spawn_workers(width);

    // Iterate through the combinations, collecting groups of them
    // (key_set) to be sent to individual workers.
    let mut worker = 0;
    let mut key_set = ~[];
    for i in iter::range(2,letters.len() + 1) {
        do combinations::each_combination(letters,i) |combo| {
            key_set.push( combo.to_owned() );
            if key_set.len() >= depth {
                let mut ks = ~[];
                util::swap(&mut ks, &mut key_set);
                request_chans[worker].send(ks);
                worker = (worker + 1) % width;
            }
        }
    }
    // Send remaining combinations (key_set) to the next worker.
    if !key_set.is_empty() { request_chans[worker].send(key_set); }
    // Send an empty key set to tell each worker to terminate.
    for chan in request_chans.iter() { chan.send(~[]) };

    // Collect responses from workers.
    let mut set : ~HashSet<~str> = ~HashSet::new();
    do width.times() {
        let response_set = response_port.recv();
        for word in response_set.iter() { set.insert(word.clone()); }
    }
    println!("{:u}", set.len());
}
extern mod extra;

extern mod combinations;
extern mod bisect;
extern mod djbhash;

use std::{vec,iter,os,util};
use std::io::File;
use std::io::buffered::BufferedReader;
// use extra::time;

use extra::arc::Arc;

use djbhash::HashMap;
use std::hashmap::HashSet;

// fn duration(tag : &str, start : time::Timespec, end : time::Timespec) {
//     let d_sec = end.sec - start.sec;
//     let d_nsec = end.nsec - start.nsec;
//     if d_nsec >= 0 {
//         println!("{:s}: {:?}", tag, time::Timespec { sec : d_sec, nsec : d_nsec });
//     } else {
//         println!("{:s}: {:?}", tag, time::Timespec { sec : d_sec - 1, nsec : d_nsec + 1000000000 });
//     }
// }

pub fn split_words(s : &str) -> ~[~str] { s.words().map(|w| w.to_owned()).collect() }

fn load_dictionary() -> ~HashMap<~[u8],~[~str]> {
    let path = Path::new("anadict.txt");
    let file = File::open(&path);
    let mut bufferedFile = BufferedReader::new(file);
    let mut map = ~HashMap::new();
    for line in bufferedFile.lines() {
        let words = split_words(line);
        let key : ~[char] = words[0].chars().collect();
        map.insert(vec::from_fn(key.len(), |i| key[i] as u8),
                   vec::from_fn(words.len() - 1, |i| words[i+1].clone()));
    }
    return map;
}

fn get_letters(s : &str) -> ~[u8] {
    let mut t : ~[char] = s.chars().collect();
    t.sort();
    return vec::from_fn(t.len(), |i| t[i] as u8);
}

fn search(arc_dictionary : Arc<~HashMap<~[u8],~[~str]>>, request_port : &Port<~[~[u8]]>) -> ~HashSet<~str> {
    let dictionary = arc_dictionary.get();
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
    let shared_dictionary = Arc::new( load_dictionary() );
    let (response_port, response_chan) = SharedChan::new();
    let mut request_chans : ~[Chan<~[~[u8]]>] = ~[];
    for _ in range(0, n_workers) {
        let dictionary = shared_dictionary.clone();
        let (request_port,request_chan) = Chan::new();
        request_chans.push(request_chan);
        // Set up and start worker task
        let response_chan = response_chan.clone();
        do spawn {
            response_chan.send( search(dictionary, &request_port) );
        }
    }
    (response_port, request_chans)
}

static width : uint = 6;        // number of worker tasks
static depth : uint = 200000;   // keys / request sent to worker task

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
        combinations::each_combination(letters, i, |combo| {
            key_set.push( combo.to_owned() );
            if key_set.len() >= depth {
                let mut ks = ~[];
                util::swap(&mut ks, &mut key_set);
                request_chans[worker].send(ks);
                worker = (worker + 1) % width;
            }
        });
    }
    // Send remaining combinations (key_set) to the next worker.
    if !key_set.is_empty() { request_chans[worker].send(key_set); }
    // Send an empty key set to tell each worker to terminate.
    for chan in request_chans.iter() { chan.send(~[]) };

    // Collect responses from workers.
    let mut set : ~HashSet<~str> = ~HashSet::new();
    for _ in range(0, width) {
        let response_set = response_port.recv();
        for word in response_set.iter() { set.insert(word.clone()); }
    }
    println!("{:u}", set.len());
}

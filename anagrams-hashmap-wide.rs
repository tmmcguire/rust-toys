extern mod std;

use core::comm::*;
use core::io::*;
use core::hashmap::linear::*;

mod combinations;
mod bisect;
mod misc;

fn load_dictionaries(width : uint) -> ~[~LinearMap<~[u8],~[~str]>] {
    match file_reader(&Path("anadict-rust.txt")) {
        Ok(reader) => {
            let mut maps = vec::from_fn(width, |_| ~LinearMap::new());
            let mut t = 0;
            for reader.each_line |line| {
                let words = misc::split_words(line);
                let key = str::to_chars(words[0]);
                maps[t].insert(vec::from_fn(key.len(), |i| key[i] as u8),
                               vec::from_fn(words.len() - 1, |i| copy words[i+1]));
                t = (t + 1) % width;
            }
            return maps;
        }
        Err(msg) => { fail!(msg); }
    }
}

fn get_letters(s : &str) -> ~[u8] {
    let mut t = str::to_chars(s);
    std::sort::quick_sort(t, |a,b| *a <= *b);
    return vec::from_fn(t.len(), |i| t[i] as u8);
}

fn search(letters : &[u8],
          dictionary : &LinearMap<~[u8],~[~str]>) -> ~LinearSet<~str> {
    let mut set = ~LinearSet::new();
    for uint::range(2, letters.len() + 1) |i| {
        let mut key = vec::from_elem(i, 0);
        for combinations::each_combination(letters,i) |combo| {
            for combo.eachi |j,&ch| { key[j] = ch; }
            match dictionary.find(&key) {
                Some(ref val) => {
                    for val.each |word| { set.insert(copy *word); }
                }
                None => { }
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

    let dictionaries = load_dictionaries(width);

    let (response_port,response_chan)
        = stream();
    let response_chan = SharedChan(response_chan);

    for dictionaries.each |&dictionary| {
        let response_chan = response_chan.clone();
        let letters = copy letters;
        do spawn {
            response_chan.send( search(letters, dictionary) );
        }
    }

    let mut set = ~LinearSet::new();
    for width.times {
        for response_port.recv().each |&word| { set.insert(word); }
    }
    println(fmt!("%u", set.len()));
}

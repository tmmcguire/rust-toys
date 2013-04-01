extern mod std;

mod combinations;
mod bisect;

use core::comm::*;
use core::io::*;
use core::hashmap::linear::*;
use core::task::*;

fn load_dictionaries(width : uint) -> ~[~LinearMap<~[int],~[~str]>] {
    match file_reader(&Path("anadict-rust.txt")) {
        Ok(reader) => {
            let mut maps = vec::from_fn(width, |_| ~LinearMap::new());
            let mut t = 0;
            for reader.each_line |line| {
                let words = line.split_str(" ");
                let key = str::chars(words[0]);
                maps[t].insert(vec::from_fn(key.len(), |i| key[i] as int),
                               vec::from_fn(words.len() - 1, |i| copy words[i+1]));
                t = (t + 1) % width;
            }
            return maps;
        }
        Err(msg) => { fail!(msg); }
    }
}

fn get_letters(s : &str) -> ~[int] {
    let mut t = str::chars(s);
    std::sort::quick_sort(t, |a,b| *a <= *b);
    return vec::from_fn(t.len(), |i| t[i] as int);
}

fn search(letters : &[int],
          dictionary : &LinearMap<~[int],~[~str]>) -> ~LinearSet<~str> {
    let mut set = ~LinearSet::new();
    for uint::range(2, letters.len() + 1) |i| {
        let mut key = vec::from_elem(i, 0);
        for combinations::each_combination(letters,i) |combo| {
            // mapi seems to be significantly slower
            for uint::range(0,i) |j| { key[j] = combo[j]; }
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
    for uint::range(0,width) |_| {
        for response_port.recv().each |&word| { set.insert(word); }
    }
    println(fmt!("%u", set.len()));
}

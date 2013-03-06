extern mod std;

mod combinations;
mod bisect;

use core::io::*;
use core::hashmap::linear::*;

fn load_dictionary() -> ~LinearMap<~[int],~[~str]> {
    match file_reader(&Path("anadict-rust.txt")) {
        Ok(reader) => {
            let mut map = ~LinearMap::new();
            for reader.each_line |line| {
                let words = line.split_str(" ");
                let key = str::chars(words[0]);
                map.insert(vec::from_fn(key.len(), |i| key[i] as int),
                           vec::from_fn(words.len() - 1, |i| copy words[i+1]));
            }
            return map;
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
    let dictionary = load_dictionary();
    let mut set = LinearSet::new();
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
    println(fmt!("%u", set.len()));
}

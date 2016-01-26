extern crate djbhashmap;
extern crate combinations;
extern crate mmap;

use std::env;
use djbhashmap::{HashMap,HashSet};

fn get_letters(s : &str) -> Vec<u8> {
    let mut t : Vec<u8> = s.chars().map(|ch| ch as u8).collect();
    t.sort();
    t
}

fn line_map<'b>(buffer : &'b [u8]) -> HashMap<&'b [u8],&'b [u8]> {
    let length = buffer.len();
    let mut map = HashMap::new();
    let mut i = 0;
    while i < length {
        let mut j = i;
        while j < length && buffer[j] != ' ' as u8 { j += 1; }
        let mut k = j+1;
        while k < length && buffer[k] != '\n' as u8 { k += 1; }
        map.insert(&buffer[i..j], &buffer[j+1..k]);
        i = k + 1;
    }
    return map;
}

fn search<'b>(letters : &[u8], dictionary : &'b HashMap<&'b [u8],&'b [u8]>) -> HashSet<&'b [u8]>
{
    let mut set = HashSet::new();
    for i in 2..letters.len() + 1 {
        let mut key = vec![0u8; i];
        combinations::each_combination(letters, i, |combo| {
            for j in 0..combo.len() { key[j] = combo[j]; }
            let k: &[u8] = &key;
            dictionary.get(&k).map(|&v| set.insert(v));
        });
    }
    return set;
}

fn main() {
    let mut args = env::args();
    if args.len() < 2 {
        panic!("Usage: anagrams letters");
    }
    let letters = get_letters(&args.nth(1).unwrap());
    let region = mmap::MappedRegion::mmap("anadict.txt").expect("cannot read anadicttxt");
    let buf = region.get_slice();
    let map = line_map(buf);
    let set = search(&letters, &map);
    let mut count = 0;
    for ln in set.iter() {
        count += 1 + ln.iter().filter(|&ch| *ch == ' ' as u8).count();
    }
    println!("{}", count);
}

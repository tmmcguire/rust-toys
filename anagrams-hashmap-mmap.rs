extern mod extra;

extern mod combinations;
extern mod mmap;

use std::{os,iter,vec};
use std::hashmap::*;

fn get_letters(s : &str) -> ~[u8] {
    let mut t : ~[char] = s.iter().collect();
    extra::sort::quick_sort(t, |a,b| *a <= *b);
    return vec::from_fn(t.len(), |i| t[i] as u8);
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
        map.insert(buffer.slice(i, j), buffer.slice(j+1, k));
        i = k + 1;
    }
    return map;
}

#[deriving(IterBytes)]
struct MapKey(~[u8]);

impl<'self> Equiv<&'self [u8]> for MapKey {
    fn equiv(&self, other: & &'self [u8]) -> bool {
        let slice1: &[u8] = **self;
        let slice2: &[u8] = *other;
        slice1 == slice2
    }
}

fn search<'b>(letters : &[u8], dictionary : &'b HashMap<&'b [u8],&'b [u8]>) -> HashSet<&'b [u8]>
{
    let mut set = HashSet::new();
    for i in iter::range(2, letters.len() + 1) {
        let mut key = MapKey(vec::from_elem(i, 0u8));
        do combinations::each_combination(letters,i) |combo| {
            for j in iter::range(0, combo.len()) { key[j] = combo[j]; }
            dictionary.find_equiv(&key).map(|&v| set.insert(*v));
        }
    }
    return set;
}

fn main() {
    let args = os::args();
    if args.len() < 2 {
        fail!(~"Usage: anagrams letters");
    }
    let letters = get_letters(args[1]);
    do mmap::with_mmap_file_contents("anadict.txt") |buf| {
        let map = line_map(buf);
        let set = search(letters, &map);
        let mut count = 0;
        for ln in set.iter() {
            count += 1 + ln.iter().count(|&ch| { ch == ' ' as u8 });
        }
        println(fmt!("%?", count));
    }
}

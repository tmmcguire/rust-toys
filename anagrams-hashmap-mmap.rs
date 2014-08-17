extern crate djbhash;
extern crate combinations;
extern crate mmap;

use std::os;
use djbhash::{AsBytes,HashMap,HashSet};

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
        map.insert(buffer.slice(i, j), buffer.slice(j+1, k));
        i = k + 1;
    }
    return map;
}

struct MapKey(Vec<u8>);

impl MapKey {
    fn set(&mut self, i : uint, v : u8) -> () {
        match self {
            &MapKey(ref mut ary) => { *ary.get_mut(i) = v; }
        }
    }
}

impl<'l> Equiv<&'l [u8]> for MapKey {
    fn equiv(&self, other: & &'l [u8]) -> bool {
        match self {
            &MapKey(ref slice1) => {
                let s1 : &[u8] = slice1.as_slice();
                let s2 : &[u8] = other.as_slice();
                s1 == s2
            }
        }
    }
}

impl AsBytes for MapKey {
    fn as_byte_vec<'a>(&'a self) -> &'a [u8] {
        match self { &MapKey(ref slice) => slice.as_byte_vec() }
    }
}

fn search<'b>(letters : &[u8], dictionary : &'b HashMap<&'b [u8],&'b [u8]>) -> HashSet<&'b [u8]>
{
    let mut set = HashSet::new();
    for i in range(2, letters.len() + 1) {
        let mut key = MapKey(Vec::from_elem(i, 0u8));
        combinations::each_combination(letters, i, |combo| {
          for j in range(0, combo.len()) { key.set(j, combo[j]); }
                dictionary.find_equiv(&key).map(|&v| set.insert(v));
        });
    }
    return set;
}

fn main() {
    let args = os::args();
    if args.len() < 2 {
        fail!("Usage: anagrams letters");
    }
    let letters = get_letters(args.get(1).as_slice());
    mmap::with_mmap_file_contents("anadict.txt", |buf| {
        let map = line_map(buf);
        let set = search(letters.as_slice(), &map);
        let mut count = 0u;
        for ln in set.iter() {
            count += 1 + ln.iter().filter(|&ch| { *ch == ' ' as u8 }).count();
        }
        println!("{:u}", count);
    });
}

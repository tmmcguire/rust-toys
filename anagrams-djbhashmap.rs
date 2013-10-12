extern mod extra;
extern mod combinations;
extern mod djbhash;

use std::{vec,iter,os};
use std::io::*;
use extra::time;

use djbhash::{HashMap,HashSet};

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

fn load_dictionary() -> ~HashMap<~[i8],~[~str]> {
    match file_reader(&Path("anadict.txt")) {
        Ok(reader) => {
            let mut map = ~HashMap::new();
            reader.each_line(|line| {
                    let words = split_words(line);
                    let key : ~[char] = words[0].iter().collect();
                    map.insert(vec::from_fn(key.len(),       |i| key[i] as i8),
                               vec::from_fn(words.len() - 1, |i| words[i+1].clone()));
                    true
                });
            return map;
        }
        Err(msg) => { fail!(msg); }
    }
}

fn get_letters(s : &str) -> ~[i8] {
    let mut t : ~[char] = s.iter().collect();
    extra::sort::quick_sort(t, |a,b| *a <= *b);
    return vec::from_fn(t.len(), |i| t[i] as i8);
}

fn search(letters : &[i8], dictionary : &HashMap<~[i8],~[~str]>) -> ~HashSet<~str>
{
    let mut set = ~HashSet::new();
    for i in iter::range(0, letters.len() + 1) {
        // let start = time::get_time();
        let mut key : ~[i8] = vec::from_elem(i, 0i8);
        do combinations::each_combination(letters,i) |combo| {
            for j in iter::range(0, combo.len()) { key[j] = combo[j]; }
            match dictionary.find(&key) {
                Some(ref val) => {
                    for word in val.iter() { set.insert(word.clone()); }
                }
                None => { }
            }
        }
        // duration("iteration", start, time::get_time());
    }
    return set;
}

fn main() {
    // let start = time::get_time();
    let args = os::args();
    if args.len() < 2 { fail!(~"Usage: anagrams letters"); }
    let letters = get_letters(args[1]);
    // duration("get_letters", start, time::get_time());
    let dictionary = load_dictionary();
    // duration("load_dictionary", start, time::get_time());
    let set = search(letters,dictionary);
    // duration("search", start, time::get_time());
    println!("collisions {:u} capacity {:u} len {:u}", dictionary.collisions(), dictionary.capacity(), dictionary.len());
    println!("{:u}", set.len());
}

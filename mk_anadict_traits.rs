extern mod extra;

use std::{vec,str};
use std::cmp::{Eq,Ord};
use std::hash::Hash;
use std::io::*;
use std::result::*;
use std::to_bytes::IterBytes;
use std::hashmap::*;

trait SortedKeys<K : Ord> {
    fn each_key_sorted(&self, blk : &fn(key : &K) -> bool) -> bool;
}

impl<'self, K : Hash + IterBytes + Eq + Ord, V> SortedKeys<K> for HashMap<K,V> {

    fn each_key_sorted(&self, blk : &fn(key : &'self K) -> bool) -> bool {
        // let mut keys : ~[&'self K] = vec::with_capacity(self.len());
        let mut keys = vec::with_capacity(self.len());
        for self.iter().advance |(k,_)| { keys.push(k); }
        extra::sort::quick_sort(keys, |a,b| *a <= *b);
        for keys.iter().advance |&k| { if !blk(k) { return false; } }
        return true;
    }

}

trait DictReader {
    fn read_dict(&self) -> ~HashMap<~str,~[~str]>;
}

impl DictReader for @Reader {

    fn read_dict(&self) -> ~HashMap<~str,~[~str]> {
        let mut map = ~HashMap::new();
        for self.each_line |line| {
            let line = line.trim();
            let length = line.len();
            // Original is using pre-strip() line for comparisons
            if length >= 2 && length < 19
                && line.iter().all(|ch| (ch.is_ascii() && ch.is_lowercase())) {
                let mut chars : ~[char] = line.iter().collect();
                extra::sort::quick_sort(chars, |a,b| *a <= *b);
                let key = str::from_chars(chars);

                // 0.7:
                // This previously didn't work.
                map.find_or_insert(key, ~[]).push(line.to_owned());
                // find_or_insert's result wasn't mutable. Had to use:
                // let mut value = match map.pop(&key) {
                //     None => { ~[] }
                //     Some(old) => { old }
                // };
                // value.push(line.to_owned());
                // map.insert(key,value);
            }
        }
        return map;
    }

}

trait DictWriter {
    fn write_dict(&self, dict : &HashMap<~str,~[~str]>);
}

impl DictWriter for @Writer {

    fn write_dict(&self, dict : &HashMap<~str,~[~str]>) {
        for dict.each_key_sorted() |key| {
            let line : ~str = dict.get(key).connect(" ");
            self.write_str(fmt!("%s %s\n", *key, line));
        }
    }

}

fn main() {
    match (file_reader(&Path("/usr/share/dict/words")),
           file_writer(&Path("anadict-rust.txt"), [Create,Truncate])) {
        (Ok(r),    Ok(w))     => {
            // It's like magic, but not as exciting.
            w.write_dict(r.read_dict());
        }
        (Err(msg), _)         => { fail!(msg); }
        (_,        Err(msg))  => { fail!(msg); }
    }
}

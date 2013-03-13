extern mod std;

use core::cmp::{Eq,Ord};
use core::hash::Hash;
use core::io::*;
use core::result::*;
use core::to_bytes::IterBytes;
use core::hashmap::linear::*;

trait SortedKeys<K : Ord> {
    fn each_key_sorted(&self, blk : &fn(key : &K) -> bool);
}

impl<K : Hash + IterBytes + Eq + Ord, V> SortedKeys<K> for LinearMap<K,V> {

    fn each_key_sorted(&self, blk : &fn(key : &self/K) -> bool) {
        let mut keys : ~[&self/K] = vec::with_capacity(self.len());
        for self.each |&(k,_)| { keys.push(k); }
        std::sort::quick_sort(keys, |a,b| *a <= *b);
        for keys.each |&k| { if !blk(k) { break; } }
    }

}

trait DictReader {
    fn read_dict(&self) -> ~LinearMap<~str,~[~str]>;
}

impl DictReader for Reader {

    fn read_dict(&self) -> ~LinearMap<~str,~[~str]> {
        let mut map = ~LinearMap::new();
        for self.each_line |line| {
            let line = line.trim();
            let length = line.len();
            // Original is using pre-strip() line for comparisons
            if length >= 2 && length < 19
                && line.all(|ch| (char::is_ascii(ch)
                                  && char::is_lowercase(ch))) {
                let mut chars = str::chars(line);
                std::sort::quick_sort(chars, |a,b| *a <= *b);
                let key = str::from_chars(chars);

                // What I'd like to do:
                // map.find_or_insert(key, ~[]).push(line);
                // But find_or_insert's result isn't mutable.

                let mut value = match map.pop(&key) {
                    None => { ~[] }
                    Some(old) => { old }
                };
                value.push(line);
                map.insert(key,value);
            }
        }
        return map;
    }

}

trait DictWriter {
    fn write_dict(&self, dict : &LinearMap<~str,~[~str]>);
}

impl DictWriter for Writer {

    fn write_dict(&self, dict : &LinearMap<~str,~[~str]>) {
        for dict.each_key_sorted() |key| {
            let line = str::connect(dict.get(key).map(|v| copy *v), " ");
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

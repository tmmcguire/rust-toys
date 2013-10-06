extern mod extra;

use std::str;
use std::cmp::{Eq,Ord};
use std::hash::Hash;
use std::io::*;
use std::result::*;
use std::to_bytes::IterBytes;
use std::hashmap::*;

trait SortedKeys<K : Ord> {
    fn sorted_keys(&self) -> ~[K];
}

impl<K : Hash + IterBytes + Eq + Ord + Clone, V> SortedKeys<K> for HashMap<K,V> {

    fn sorted_keys(&self) -> ~[K] {
        let mut keys : ~[K] = self.iter().map(|(k,_)| k.clone()).collect();
        extra::sort::quick_sort(keys, |a,b| *a <= *b);
        keys
    }

}

trait DictReader {
    fn read_dict(&self) -> ~HashMap<~str,~[~str]>;
}

impl DictReader for @Reader {

    fn read_dict(&self) -> ~HashMap<~str,~[~str]> {
        let mut map = ~HashMap::new();
        self.each_line(|line| {
                let line = line.trim();
                let length = line.len();
                // Original is using pre-strip() line for comparisons
                if length >= 2 && length < 19
                    && line.iter().all(|ch| (ch.is_ascii() && ch.is_lowercase())) {
                    let mut chars : ~[char] = line.iter().collect();
                    extra::sort::quick_sort(chars, |a,b| *a <= *b);
                    let key = str::from_chars(chars);
                    map.find_or_insert(key, ~[]).push(line.to_owned());
                }
                true
            });
        return map;
    }

}

trait DictWriter {
    fn write_dict(&self, dict : &HashMap<~str,~[~str]>);
}

impl DictWriter for @Writer {

    fn write_dict(&self, dict : &HashMap<~str,~[~str]>) {
        let keys = dict.sorted_keys();  // needed for lifetime
        for key in keys.iter() {
            let line : ~str = dict.get(key).connect(" ");
            self.write_str(fmt!("%s %s\n", *key, line));
        }
    }

}

fn main() {
    match (file_reader(&Path("/usr/share/dict/words")),
           file_writer(&Path("anadict.txt"), [Create,Truncate])) {
        (Ok(r),    Ok(w))     => {
            // It's like magic, but not as exciting.
            w.write_dict(r.read_dict());
        }
        (Err(msg), _)         => { fail!(msg); }
        (_,        Err(msg))  => { fail!(msg); }
    }
}

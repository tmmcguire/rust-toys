// mk_anadict_traits.rs
//
// Updated to Rust 0.11.0

use std::io::{BufferedReader,File,IoResult,Truncate,Write};
use std::collections::hashmap::HashMap;
use std::hash::Hash;

trait SortedKeys<K : Ord> {
    fn sorted_keys(&self) -> Vec<K>;
}

impl<K : Hash + Eq + Ord + Clone, V> SortedKeys<K> for HashMap<K,V> {

    fn sorted_keys(&self) -> Vec<K> {
        let mut keys : Vec<K> = self.iter().map(|(k,_)| k.clone()).collect();
        keys.sort();
        keys
    }

}

trait DictReader<T> {
    fn read_dict(&mut self) -> HashMap<String,Vec<String>>;
}

impl<T:Reader> DictReader<T> for BufferedReader<T> {

    fn read_dict(&mut self) -> HashMap<String,Vec<String>> {
        let mut map = HashMap::new();
        for line in self.lines() {
            let line = line.unwrap();
            let line = line.as_slice().trim();
            let length = line.len();
            // Original is using pre-strip() line for comparisons
            if length >= 2 && length < 19
                && line.chars().all( |ch| (ch.is_ascii() && ch.is_lowercase()) ) {
                    let mut chars : Vec<char> = line.chars().collect();
                    chars.sort();
                    let key = std::str::from_chars(chars.as_slice());
                    map.find_or_insert(key, Vec::new()).push( line.to_string() )
                }
        }
        map
    }

}

trait DictWriter : Writer {

    fn write_dict(&mut self, dict : &HashMap<String,Vec<String>>) {
        let keys = dict.sorted_keys(); // needed for lifetime
        for key in keys.iter() {
            let line : String = dict.get(key).connect(" ");
            match write!( self, "{:s} {:s}\n", *key, line ) {
                Ok(_)  => { }
                Err(e) => { fail!(e) }
            }
        }
    }

}

impl DictWriter for File { }
impl DictWriter for IoResult<File> { }

fn main() {
    let words_path = Path::new("/usr/share/dict/words");
    let words_file = File::open(&words_path);
    let mut words  = BufferedReader::new(words_file);
    let dictionary = words.read_dict();

    let dict_path     = Path::new("anadict.txt");
    let mut dict_file = File::open_mode(&dict_path, Truncate, Write);

    dict_file.write_dict( &dictionary );
}

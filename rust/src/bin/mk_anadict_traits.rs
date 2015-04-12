// mk_anadict_traits.rs
//
// Updated to Rust 1.0.0-beta

use std::collections::HashMap;
use std::fs::{File,OpenOptions};
use std::hash::Hash;
use std::io::{BufReader,Read,Write};
use std::path::Path;
use std::error::Error;

// Needed implicitly
use std::ascii::AsciiExt;               // is_ascii()
use std::io::BufRead;                   // lines()

trait SortedKeys<K : Ord> {
    fn sorted_keys(&self) -> Vec<K>;
}

impl<K : Hash + Eq + Ord + Clone,V> SortedKeys<K> for HashMap<K,V> {

    fn sorted_keys(&self) -> Vec<K> {
        let mut keys : Vec<K> = self.keys().cloned().collect();
        keys.sort();
        keys
    }

}

trait DictReader {
    fn read_dict(&mut self) -> HashMap<String,Vec<String>>;
}

impl<T:Read> DictReader for BufReader<T> {

    fn read_dict(&mut self) -> HashMap<String,Vec<String>> {
        let mut map = HashMap::new();
        for line in self.lines() {
            let line = line.unwrap();
            let line = line.trim();    // use let binding to increase lifetime
            let length = line.len();
            // Original is using pre-strip() line for comparisons
            if length >= 2 && length < 19
                && line.chars().all( |ch| (ch.is_ascii() && ch.is_lowercase()) ) {
                    let mut chars : Vec<char> = line.chars().collect();
                    chars.sort();
                    let key = chars.iter().cloned().collect();
                    map.entry(key).or_insert_with(|| Vec::new()).push( line.to_string() );
                 }
        }
        map
    }

}

trait DictWriter : Write {

    fn write_dict(&mut self, dict : &HashMap<String,Vec<String>>) {
        let keys = dict.sorted_keys(); // needed for lifetime
        for key in keys.iter() {
            let line : String = dict.get(key).unwrap().connect(" ");
            match write!( self, "{} {}\n", *key, line ) {
                Ok(_)  => { }
                Err(e) => { panic!(e) }
            }
        }
    }

}

impl DictWriter for File { }

fn main() {
    let words_path = Path::new("/usr/share/dict/words");
    let words_file = File::open(&words_path).unwrap();
    let mut words  = BufReader::new(words_file);
    let dictionary = words.read_dict();

    let dict_path  = Path::new("anadict.txt");
    let dict_file  = OpenOptions::new().create(true).write(true).truncate(true).open( &dict_path );

    match dict_file {
        Err(e) => panic!("error opening file: {} ({})", Error::description(&e), e),
        Ok(mut f) => {
            f.write_dict( &dictionary );
        }
    }
}

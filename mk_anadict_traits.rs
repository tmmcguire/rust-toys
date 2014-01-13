use std::str;
use std::io::{File,Truncate,Write};
use std::io::buffered::BufferedReader;
use std::hashmap::HashMap;

trait SortedKeys<K : Ord> {
    fn sorted_keys(&self) -> ~[K];
}

impl<K : Hash + IterBytes + Eq + Ord + Clone + TotalOrd, V> SortedKeys<K> for HashMap<K,V> {

    fn sorted_keys(&self) -> ~[K] {
        let mut keys : ~[K] = self.iter().map(|(k,_)| k.clone()).collect();
        keys.sort();
        keys
    }

}

trait DictReader<T> {
    fn read_dict(&mut self) -> ~HashMap<~str,~[~str]>;
}

impl<T:Reader> DictReader<T> for BufferedReader<T> {

    fn read_dict(&mut self) -> ~HashMap<~str,~[~str]> {
        let mut map = ~HashMap::new();
        for line in self.lines() {
            let line = line.trim();
            let length = line.len();
            // Original is using pre-strip() line for comparisons
            if length >= 2 && length < 19
                && line.chars().all( |ch| (ch.is_ascii() && ch.is_lowercase()) ) {
                let mut chars : ~[char] = line.chars().collect();
                chars.sort();
                let key = str::from_chars(chars);
                map.find_or_insert(key, ~[]).push( line.to_owned() )
            }
        }
        return map;
    }

}

trait DictWriter : Writer {

    fn write_dict(&mut self, dict : &HashMap<~str,~[~str]>) {
        let keys = dict.sorted_keys(); // needed for lifetime
        for key in keys.iter() {
            let line : ~str = dict.get(key).connect(" ");
            self.write_line( format!("{:s} {:s}", *key, line) );
        }
    }

}

impl DictWriter for File { }
impl<W:Writer> DictWriter for Option<W> { }

fn main() {
    let words_path = Path::new("/usr/share/dict/words");
    let words_file = File::open(&words_path);
    let mut words = BufferedReader::new(words_file);

    let dictionary = words.read_dict();

    let dict_path = Path::new("anadict.txt");
    let mut dict_file = File::open_mode(&dict_path, Truncate, Write);

    dict_file.write_dict( dictionary );
}

extern mod extra;

use std::str;
use std::hashmap::HashMap;
use std::io::{File,Truncate,Write};
use std::io::buffered::BufferedReader;

fn read_dict<R:Reader>(reader : &mut BufferedReader<R>) -> ~HashMap<~str,~[~str]> {
    let mut map = ~HashMap::new();
    for line in reader.lines() {
        let line   = line.trim();
        let length = line.len();
        // Original is using pre-strip() line for comparisons
        if length >= 2 && length < 19 && line.chars().all(|ch| (ch.is_ascii() && ch.is_lowercase())) {
            let mut chars : ~[char] = line.chars().collect();
            chars.sort();
            let key = str::from_chars(chars);
            map.find_or_insert(key, ~[]).push( line.to_owned() )
        }
    }
    return map;
}

fn sorted_keys<V:Clone>(dict : &HashMap<~str,V>) -> ~[~str] {
    let mut keys : ~[~str] = dict.keys().map( |k| k.clone() ).collect();
    keys.sort();
    return keys;
}

fn print_dict(writer : &mut Option<File>, dict : &HashMap<~str,~[~str]>) {
    let skeys = sorted_keys(dict);
    for key in skeys.iter() {
        let line : ~str = dict.get(key).connect(" ");
        writer.write_str( format!("{:s} {:s}\n", *key, line) );
    }
}

fn main() {
    let mut words_file = BufferedReader::new( File::open( &Path::new("/usr/share/dict/words") ) );
    let mut dict_file = File::open_mode( &Path::new("anadict.txt"), Truncate, Write );
    print_dict( &mut dict_file, read_dict( &mut words_file ) );
}

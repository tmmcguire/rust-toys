// mk_anadict.rs
//
// Updated to Rust 0.11.0

use std::collections::hashmap::HashMap;
use std::io::{BufferedReader,BufferedWriter,File,Truncate,Write};

fn read_dict<R:Reader>(reader : &mut BufferedReader<R>) -> HashMap<String,Vec<String>> {
    let mut map = HashMap::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let line : &str = line.as_slice().trim();
        let length = line.len();
        // Original is using pre-strip() line for comparisons
        if length >= 2 && length < 19 && line.chars().all(|ch| (ch.is_ascii() && ch.is_lowercase())) {
            let mut chars : Vec<char> = line.chars().collect();
            chars.sort();
            let key = std::str::from_chars(chars.as_slice());
            map.find_or_insert(key, Vec::new()).push( line.to_string() );
        }
    }
    return map;
}

fn sorted_keys<V:Clone>(dict : &HashMap<String,V>) -> Vec<String> {
    let mut keys : Vec<String> = dict.keys().map( |k| k.clone() ).collect();
    keys.sort();
    return keys;
}

fn print_dict<W:Writer>(writer : &mut BufferedWriter<W>, dict : HashMap<String,Vec<String>>) {
    for key in sorted_keys(&dict).iter() {
        let line : String = dict.get(key).connect(" ");
        match write!( writer, "{:s} {:s}\n", *key, line ) {
            Err(e) => { fail!(e) },
            _ => { }
        }
    }
}

fn main() {
    let words_path       = Path::new("/usr/share/dict/words");
    let words_file       = File::open( &words_path );
    let mut words_reader = BufferedReader::new( words_file );
    let dictionary       = read_dict( &mut words_reader );
    let anadict_path     = Path::new("anadict.txt");
    let dict_file        = File::open_mode( &anadict_path, Truncate, Write );
    let mut dict_writer  = BufferedWriter::new( dict_file );
    print_dict( &mut dict_writer, dictionary );
}

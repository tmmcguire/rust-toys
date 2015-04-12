// mk_anadict.rs
//
// Updated to Rust 1.0.0-beta

use std::collections::HashMap;
use std::fs::{File,OpenOptions};
use std::io::{BufReader,BufWriter,Read,Write};
use std::path::Path;
use std::error::Error;

// Needed implicitly
use std::ascii::AsciiExt;               // is_ascii()
use std::io::BufRead;                   // lines()

fn read_dict<R:Read>(reader : &mut BufReader<R>) -> HashMap<String,Vec<String>> {
    let mut map = HashMap::new();
    for line in reader.lines() {
        let line : String = line.unwrap();
        let line : &str = line.trim();
        let length = line.len();
        // Original is using pre-strip() line for comparisons
        if length >= 2 && length < 19 && line.chars().all(|ch| (ch.is_ascii() && ch.is_lowercase())) {
            let mut chars : Vec<char> = line.chars().collect();
            chars.sort();
            let key : String = chars.iter().cloned().collect();
            map.entry(key).or_insert_with(|| { Vec::new() }).push( line.to_string() );
        }
    }
    return map;
}

fn sorted_keys<V:Clone>(dict : &HashMap<String,V>) -> Vec<String> {
    let mut keys : Vec<String> = dict.keys().map( |k| k.clone() ).collect();
    keys.sort();
    return keys;
}

fn print_dict<W:Write>(writer : &mut BufWriter<W>, dict : HashMap<String,Vec<String>>) {
    for key in sorted_keys(&dict).iter() {
        let line : String = dict.get(key).unwrap().connect(" ");
        match write!( writer, "{} {}\n", *key, line ) {
            Err(e) => { panic!(e) },
            _ => { }
        }
    }
}

fn main() {
    let words_path       = Path::new("/usr/share/dict/words");
    let words_file       = File::open( &words_path ).unwrap();
    let mut words_reader = BufReader::new( words_file );
    let dictionary       = read_dict( &mut words_reader );
    let anadict_path     = Path::new("anadict.txt");
    let dict_file        = OpenOptions::new().create(true).write(true).truncate(true).open( &anadict_path );
    match dict_file {
        Err(e) => panic!("error opening file: {} ({})", Error::description(&e), e),
        Ok(f) => {
            let mut dict_writer  = BufWriter::new(f);
            print_dict( &mut dict_writer, dictionary );
        }
    }
}

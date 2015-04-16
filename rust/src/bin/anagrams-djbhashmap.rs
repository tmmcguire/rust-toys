extern crate combinations;
extern crate djbhashmap;
// extern crate time;

use djbhashmap::{HashMap,HashSet};

use std::fs::File;
use std::io::{BufRead,BufReader,Read};
use std::path::Path;

fn split_words(s : &str) -> Vec<String> {
    s.split(" ").map(|w| w.to_string()).collect()
}

fn load_dictionary() -> HashMap<Vec<i8>,Vec<String>> {
    let file = match File::open( &Path::new("anadict.txt") ) {
        Ok(f)  => f,
        Err(e) => panic!(e)
    };
    let buffered_file = BufReader::new(file);
    let mut map = HashMap::new();
    for line in buffered_file.lines() {
        let line = line.unwrap();
        let mut words = split_words(&line);
        let key : Vec<i8> = words.remove(0).chars().map(|ch| ch as i8).collect();
        map.insert(key, words);
    }
    map
}

fn get_letters(s : &str) -> Vec<i8> {
    let mut t : Vec<char> = s.chars().collect();
    t.sort();
    t.iter().map(|&ch| ch as i8).collect()
}

fn search(letters : &[i8], dictionary : &HashMap<Vec<i8>,Vec<String>>) -> HashSet<String> {
    let mut set = HashSet::new();
    for i in 0..letters.len() + 1 {
        // let start = time::get_time();
        let mut key : Vec<i8> = vec![0; i];
        combinations::each_combination(letters, i, |combo| {
            for j in 0..combo.len() { key[j] = combo[j]; }
            match dictionary.get(&key) {
                Some(val) => {
                    for word in val.iter() { set.insert(word.clone()); }
                }
                None => { }
            }
        });
        // println!("iteration: {}", time::get_time() - start);
    }
    set
}

fn main() {
    // let start = time::get_time();
    let args : Vec<String> = std::env::args().collect();
    if args.len() < 2 { panic!("Usage: anagrams letters"); }
    let letters = get_letters(&args[1]);
    // println!("get_letters: {}", time::get_time() - start);
    let dictionary = load_dictionary();
    // println!("load_dictionary: {}", time::get_time() - start);
    let set = search(&letters,&dictionary);
    // println!("search: {}", time::get_time() - start);
    println!("{}", set.len());
}

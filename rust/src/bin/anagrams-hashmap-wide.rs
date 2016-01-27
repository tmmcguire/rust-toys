extern crate combinations;

use std::io::{BufRead,BufReader};
use std::fs::File;
use std::sync::mpsc::channel;
use std::thread;
use std::collections::{HashMap, HashSet};

pub fn split_words(s : &str) -> Vec<String> { s.split_whitespace().map(|w| w.to_string()).collect() }

fn load_dictionaries(width : usize) -> Vec<HashMap<Vec<char>,Vec<String>>> {
    let file = File::open("anadict.txt").unwrap();
    let buffered_file = BufReader::new(file);
    let mut maps: Vec<HashMap<Vec<char>,Vec<String>>> = (0..width).map(|_| HashMap::new()).collect();
    let mut t = 0;
    for line in buffered_file.lines() {
        let line = line.unwrap();
        let words = split_words(&line);
        let key : Vec<char> = words[0].chars().collect();
        maps.get_mut(t).unwrap().insert(key, words[1..].iter().map(|s| s.clone()).collect());
        t = (t+1) % width;
    }
    return maps;
}

fn get_letters(s : &str) -> Vec<char> {
    let mut t : Vec<char> = s.chars().collect();
    t.sort();
    t
}

fn search(letters : &[char], dictionary : &HashMap<Vec<char>,Vec<String>>) -> HashSet<String> {
    let mut set = HashSet::new();
    for i in 2..letters.len() + 1 {
        let mut key = vec!('\x00'; i);
        combinations::each_combination(letters, i, |combo| {
            for j in 0..combo.len() { *(key.get_mut(j).unwrap()) = combo[j]; }
            match dictionary.get(&key) {
                Some(ref val) => {
                    for word in val.iter() { set.insert(word.clone()); }
                }
                None => { }
            }
        });
    }
    return set;
}

fn main() {
    let width = 3;

    let mut args = std::env::args();
    if args.len() < 2 { panic!("Usage: anagrams letters"); }
    let letters = get_letters(&args.nth(1).unwrap());

    let dictionaries = load_dictionaries(width);

    let (sender, receiver) = channel();

    for dictionary in dictionaries.iter() {
        let sender = sender.clone();
        let letters = letters.clone();
        let dictionary = dictionary.clone();
        thread::spawn(move || { sender.send( search(&letters, &dictionary) ).unwrap(); });
    }

    let mut set : HashSet<String> = HashSet::new();
    for _ in 0..width {
        let response_set = receiver.recv().unwrap();
        for word in response_set.iter() { set.insert(word.clone()); }
    }
    println!("{}", set.len());
}

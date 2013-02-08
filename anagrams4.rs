extern mod std;

mod combinations;
mod bisect;

use io::*;
use std::map::*;

fn load_dictionary() -> ~HashMap<@~[char],@~[@~str]> {
    match file_reader(&Path("anadict-rust.txt")) {
        Ok(reader) => {
            let mut map = ~HashMap();
            for reader.each_line() |line| {
                let words = str::split_str(line, " ");
                map.insert(@str::chars(words[0]), @vec::from_fn(words.len() - 1, |i| @copy words[i+1]));
            }
            return map;
        }
        Err(msg) => { fail msg; }
    }
}

fn main() {
    let args = os::args();
    if args.len() < 2 {
        fail ~"Usage: anagrams letters";
    }
    let mut letters = str::chars(args[1]);
    std::sort::quick_sort(letters, |a,b| *a <= *b);
    let dictionary = load_dictionary();
    let mut set : Set<@~str> = HashMap();
    for uint::range(2,letters.len()) |i| {
        for combinations::each_combination_ref(letters,i) |combo| {
            let ana = @vec::from_fn(combo.len(), |i| *combo[i]);
            match dictionary.find(ana) {
                Some(ref val) => {
                    for val.each() |word| { set_add(set, *word); }
                }
                None => { }
            }
        }
    }
    let result = vec_from_set(set);
    println(fmt!("%u", result.len()));
}

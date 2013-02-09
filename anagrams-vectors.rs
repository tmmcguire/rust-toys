extern mod std;

mod combinations;
mod bisect;

use io::*;
use std::map::*;
use cmp::{Eq, Ord};

impl char : Ord {
    #[inline(always)] pure fn lt(&self, other: &char) -> bool { *self <  *other }
    #[inline(always)] pure fn le(&self, other: &char) -> bool { *self <= *other }
    #[inline(always)] pure fn gt(&self, other: &char) -> bool { *self >  *other }
    #[inline(always)] pure fn ge(&self, other: &char) -> bool { *self >= *other }
}

fn chars_eq(l : &[char], r : &[char]) -> bool { l == r }

fn load_dictionary() -> (~[~[char]],~[~[~str]]) {
    match file_reader(&Path("anadict-rust.txt")) {
        Ok(reader) => {
            let mut keys = ~[];
            let mut values = ~[];
            for reader.each_line() |line| {
                let words = str::split_str(line, " ");
                vec::push(&mut keys, str::chars(words[0]));
                vec::push(&mut values, vec::from_fn(words.len() - 1, |i| copy words[i+1]));
            }
            return (keys,values);
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
    let (keys,values) = load_dictionary();
    let klen = keys.len();
    let mut set : Set<@~str> = HashMap();
    for uint::range(2,letters.len()) |i| {
        for combinations::each_combination(letters,i) |combo| {
            let j = bisect::bisect_left(keys, vec::from_slice(combo), 0, klen);
            if j < klen && chars_eq(keys[j], combo) {
                for values[j].each |word| {
                    set_add(set, @copy *word);
                }
            }
        }
    }
    let result = vec_from_set(set);
//    println(fmt!("%?", result));
    println(fmt!("%u", result.len()));
}

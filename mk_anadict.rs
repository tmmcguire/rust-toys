extern mod extra;

use std::{vec,str};
use std::io::*;
use std::result::*;
use std::hashmap::*;

fn read_dict(reader : @Reader) -> ~HashMap<~str,~[~str]> {
    let mut map = ~HashMap::new();
    reader.each_line(|line| {
            let line   = line.trim();
            let length = line.len();
            // Original is using pre-strip() line for comparisons
            if length >= 2 && length < 19 && line.iter().all(|ch| (ch.is_ascii() && ch.is_lowercase())) {
                let mut chars : ~[char] = line.iter().collect();
                extra::sort::quick_sort(chars, |a,b| *a <= *b);
                let key = str::from_chars(chars);

                let mut value = match map.pop(&key) {
                    None => { ~[] }
                    Some(old) => { old }
                };
                value.push(line.to_owned());
                map.insert(key,value);
            }
            true
        });
    return map;
}

fn sorted_keys<V:Clone>(dict : &HashMap<~str,V>) -> ~[~str] {
    let mut keys = vec::with_capacity( dict.len() );
    dict.each_key(|key| { keys.push(key.clone()); true });
    extra::sort::quick_sort(keys, |a,b| *a <= *b);
    return keys;
}

fn print_dict(writer : @Writer, dict : &HashMap<~str,~[~str]>) {
    let skeys = sorted_keys(dict);
    for key in skeys.iter() {
        let line : ~str = dict.get(key).connect(" ");
        writer.write_str( fmt!("%s %s\n", *key, line) );
    }
}

fn main() {
    match (file_reader(&Path("/usr/share/dict/words")),
           file_writer(&Path("anadict.txt"), [Create,Truncate])) {
        (Ok(r),    Ok(w))    => { print_dict(w, read_dict(r)); }
        (Err(msg), _)        => { fail!(msg); }
        (_,        Err(msg)) => {fail!(msg); }
    }
}

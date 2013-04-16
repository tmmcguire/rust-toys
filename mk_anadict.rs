extern mod std;

use core::io::*;
use core::result::*;
use core::hashmap::linear::*;

fn read_dict(reader : @Reader) -> ~LinearMap<~str,~[~str]> {
    let mut map = ~LinearMap::new();
    for reader.each_line |line| {
        let line   = line.trim();
        let length = line.len();
        // Original is using pre-strip() line for comparisons
        if length >= 2 && length < 19 && line.all(|ch| (char::is_ascii(ch) && char::is_lowercase(ch))) {
            let mut chars = str::to_chars(line);
            std::sort::quick_sort(chars, |a,b| *a <= *b);
            let key = str::from_chars(chars);
            
            let mut value = match map.pop(&key) {
                None => { ~[] }
                Some(old) => { old }
            };
            value.push(line.to_owned());
            map.insert(key,value);

        }
    }
    return map;
}

fn sorted_keys<V:Copy>(dict : &LinearMap<~str,V>) -> ~[~str] {
    let mut keys = vec::with_capacity( dict.len() );
    for dict.each_key |&key| { keys.push(key); }
    std::sort::quick_sort(keys, |a,b| *a <= *b);
    return keys;
}

fn print_dict(writer : @Writer, dict : &LinearMap<~str,~[~str]>) {
    for sorted_keys(dict).each |key| {
        let line = str::connect( dict.get(key).map(|&v| v), " " );
        writer.write_str( fmt!("%s %s\n", *key, line) );
    }
}

fn main() {
    match (file_reader(&Path("/usr/share/dict/words")),
           file_writer(&Path("anadict-rust.txt"), [Create,Truncate])) {
        (Ok(r),    Ok(w))    => { print_dict(w, read_dict(r)); }
        (Err(msg), _)        => { fail!(msg); }
        (_,        Err(msg)) => {fail!(msg); }
    }
}

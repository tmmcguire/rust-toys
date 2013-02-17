extern mod std;

use io::*;
use result::*;
use std::map::*;

fn read_dict(reader : Reader) -> ~HashMap<@~str,@[@~str]> {
    let map = ~HashMap();
    for reader.each_line |line| {
        let line   = line.trim();
        let length = line.len();
        // Original is using pre-strip() line for comparisons
        if length >= 2 && length < 19 && line.all(|ch| (char::is_ascii(ch) && char::is_lowercase(ch))) {
            let mut chars = str::chars(line);
            std::sort::quick_sort(chars, |a,b| *a <= *b);
            let key = str::from_chars(chars);
            map.update(@key, @[@line], |old,nw| at_vec::append(old,nw));
        }
    }
    return map;
}

fn sorted_keys<V:Copy>(dict : &HashMap<@~str,V>) -> ~[@~str] {
    let mut keys = vec::with_capacity( dict.size() );
    for dict.each_key |key| { keys.push(key); }
    std::sort::quick_sort(keys, |a,b| *a <= *b);
    return keys;
}

fn print_dict(writer : Writer, dict : &HashMap<@~str,@[@~str]>) {
    for sorted_keys(dict).each |key| {
        let line = str::connect( dict.get(*key).map(|v| **v), " " );
        writer.write_str( fmt!("%s %s\n", **key, line) );
    }
}

fn main() {
    match (file_reader(&Path("/usr/share/dict/words")),
           file_writer(&Path("anadict-rust.txt"), [Create,Truncate])) {
        (Ok(r),    Ok(w))    => { print_dict(w, read_dict(r)); }
        (Err(msg), _)        => { fail msg; }
        (_,        Err(msg)) => {fail msg; }
    }
}

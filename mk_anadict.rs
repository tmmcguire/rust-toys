extern mod std;

use io::*;
use result::*;
use std::map::*;

fn read_dict(reader : Reader) -> ~HashMap<@~str,@[@~str]> {
    let map = ~HashMap();
    for reader.each_line |line| {
        let line = line.trim();
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
    let mut keys = vec::with_capacity(dict.size());
    for dict.each_key |key| { vec::push(&mut keys, key); }
    std::sort::quick_sort(keys, |a,b| *a <= *b);
    return keys;
}

fn print_dict(writer : Writer, dict : &HashMap<@~str,@[@~str]>) {
    for vec::each(sorted_keys(dict)) |key| {
        writer.write_str(**key);
        for vec::each(dict.get(*key)) |value| {
            writer.write_char(' ');
            writer.write_str(**value);
        }
        writer.write_char('\n');
    }
}

fn main() {
    match file_reader(&Path("/usr/share/dict/words")) {
        Ok(reader) => {
            let dict = read_dict(reader);
            match file_writer(&Path("anadict-rust.txt"), [Create,Truncate]) {
                Ok(writer) => { print_dict(writer, dict); }
                Err(msg)   => { fail msg; }
            }
        }
        Err(msg) => { fail msg; }
    }
}

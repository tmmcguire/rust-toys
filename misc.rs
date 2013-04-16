#[ link(name = "misc", vers="1.0") ];
#[ crate_type = "lib" ];

#[warn(deprecated_mode)];
#[warn(deprecated_pattern)];
#[warn(vecs_implicitly_copyable)];
#[deny(non_camel_case_types)];

extern mod combinations;

pub fn split_words(s : &str) -> ~[~str] {
    let mut words = ~[];
    for str::each_word(s) |word| {
        words.push(word.to_owned());
    }
    return words;
}

pub mod linearmap {

    use core::hashmap::linear::*;

    pub fn search(letters : &[i8],
                  dictionary : &LinearMap<~[i8],~[~str]>)
        -> ~LinearSet<~str>
    {
        let mut set = ~LinearSet::new();
        for uint::range(2, letters.len() + 1) |i| {
            let mut key = vec::from_elem(i, 0);
            for ::combinations::each_combination(letters,i) |combo| {
                // mapi seems to be significantly slower
                for uint::range(0,i) |j| { key[j] = combo[j]; }
                match dictionary.find(&key) {
                    Some(ref val) => {
                        for val.each |word| { set.insert(copy *word); }
                    }
                    None => { }
                }
            }
        }
        return set;
    }

}
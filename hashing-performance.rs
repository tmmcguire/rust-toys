extern mod extra;

use extra::time;

fn duration(f : &fn () -> ()) -> time::Timespec {
    let start = time::get_time();
    f();
    let end = time::get_time();
    let d_sec = end.sec - start.sec;
    let d_nsec = end.nsec - start.nsec;
    if d_nsec >= 0 {
        time::Timespec { sec : d_sec, nsec : d_nsec }
    } else {
        time::Timespec { sec : d_sec - 1, nsec : d_nsec + 1000000000 }
    }
}
    
fn thirtythree_million_siphashes() {
    use std::hash::*;

    let s = ~"abcdefghijklmnopqrstuvwxyz";
    let mut potato = 0u64;
    do 33000000.times {
        potato ^= s.hash();
    }
    println(fmt!("%?", potato));
}

fn djbhash(bytes : &[u8]) -> u64 {
    let mut hash = 5381u64;
    for byte in bytes.iter() {
        hash = (33u64 * hash) ^ *byte as u64;
    }
    return hash;
}
    
fn thirtythree_million_djbhashes() {
    let s = ~"abcdefghijklmnopqrstuvwxyz";
    let mut potato = 0u64;
    do 33000000.times {
        potato ^= djbhash(s.as_bytes());
    }
    println(fmt!("%?", potato));
}

fn main() {
    println(fmt!("sip: %?", duration(thirtythree_million_siphashes)));
    println(fmt!("djb: %?", duration(thirtythree_million_djbhashes)));
}

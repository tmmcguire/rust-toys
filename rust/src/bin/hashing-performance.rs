extern crate time;
extern crate djbhashmap;

use std::hash::{Hash,Hasher,SipHasher};
use djbhashmap::djbhasher::DJBHasher;
    
fn thirtythree_million_siphashes() {
    let s = "abcdefghijklmnopqrstuvwxyz";
    let mut potato = 0u64;
    for _ in 0..33000000 {
        let mut hasher = SipHasher::new();
        s.hash(&mut hasher);
        potato += hasher.finish();
    }
    println!("{}", potato);
}

fn siphash_duration() -> time::Duration {
    let start = time::get_time();
    thirtythree_million_siphashes();
    let duration = time::get_time() - start;
    println!("sip: {}", duration);
    duration
}

    
fn thirtythree_million_djbhashes() {
    let s = "abcdefghijklmnopqrstuvwxyz";
    let mut potato = 0u64;
    for _ in 0..33000000 {
        let mut hasher = DJBHasher::new();
        s.hash(&mut hasher);
        potato += hasher.finish();
    }
    println!("{}", potato);
}

fn djbhash_duration() -> time::Duration {
    let start = time::get_time();
    thirtythree_million_djbhashes();
    let duration = time::get_time() - start;
    println!("djb: {}", duration);
    duration
}

fn main() {
    let _ = siphash_duration();
    let _ = djbhash_duration();
}

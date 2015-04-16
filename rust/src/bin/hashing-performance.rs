extern mod extra;

use extra::time;

fn duration(f : || -> ()) -> time::Timespec {
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

fn to_float(t : &time::Timespec) -> f64 {
    (t.sec as f64) + ((t.nsec as f64) / 1000000000.0f64)
}
    
fn thirtythree_million_siphashes() {
    let s = ~"abcdefghijklmnopqrstuvwxyz";
    let mut potato = 0u64;
    for _ in range(0, 33000000) {
        potato += s.hash();
    }
    println!("{:?}", potato);
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
    for _ in range(0, 33000000) {
        potato += djbhash(s.as_bytes());
    }
    println!("{:?}", potato);
}

fn main() {
    println!("sip: {:?}", to_float(&duration(thirtythree_million_siphashes)));
    println!("djb: {:?}", to_float(&duration(thirtythree_million_djbhashes)));
}

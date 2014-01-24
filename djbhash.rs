// Copyright 2013 Tommy M. McGuire
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[ crate_id = "djbhash#1.0" ];
#[ crate_type = "lib" ];

use std::io::Writer;
use std::to_bytes::IterBytes;
use std::vec;
use std::uint;
use std::util;

// Simple Writer/IterBytes based implementation of the DJB hash
// (See http://cr.yp.to/cdb/cdb.txt and http://www.cse.yorku.ca/~oz/hash.html)
struct DJBState {
    hash : u64
}

impl DJBState {
    #[inline]
    fn new() -> DJBState { DJBState { hash : 5381u64 } }

    #[inline]
    fn djbhash<T:IterBytes>(t : &T) -> u64 {
        let mut state = DJBState::new();
        t.iter_bytes(true, |b| { state.write(b); true });
        state.flush();
        return state.hash();
    }

    #[inline]
    fn hash(&self) -> u64 { self.hash }
}

impl Writer for DJBState {
    #[inline]
    fn write(&mut self, buf : &[u8]) {
        let len = buf.len();
        let mut i = 0;
        while i < len { self.hash = (33u64 * self.hash) ^ buf[i] as u64; i += 1; }           /* 3.1s */
        // for i in range(0, len) { self.hash = (33u64 * self.hash) ^ buf[i] as u64 }        /* 3.6s */
        // for i in range(0, buf.len()) { self.hash = (33u64 * self.hash) ^ buf[i] as u64 }  /* 3.6s */
        // for byte in buf.iter() { self.hash = (33u64 * self.hash) ^ *byte as u64 }         /* 3.8s */
    }

    #[inline]
    fn flush(&mut self) { }
}

/* Original hash function */
// fn djbhash(bytes : &[u8]) -> u64 {
//     let mut hash = 5381u64;
//     for byte in bytes.iter() {
//             hash = (33u64 * hash) ^ *byte as u64;
//     }
//     return hash;
// }

/* ----------------------------------------------- */

// This is an implementation of the Container, Mutable, Map, and
// MutableMap traits based on the DJB hash and Python's dictionaries.
// See:
//
// * http://stackoverflow.com/questions/327311/how-are-pythons-built-in-dictionaries-implemented
//   (especially the links in the first answer),
//
// * http://www.laurentluce.com/posts/python-dictionary-implementation/
//
// * http://pybites.blogspot.com/2008/10/pure-python-dictionary-implementation.html

static PERTURB_SHIFT : uint = 5;

enum Entry<K,V> {
    Empty,                      // This slot is empty
    Full(K,V,u64),              // This slot is holding a key and value
    Ghost(K,u64),               // This slot once held key k
}

impl<K, V> Entry<K,V> {
    // fn is_empty(&self) -> bool { match *self { Empty => true, _ => false } }
    #[inline]
    fn is_full(&self)  -> bool { match *self { Full(..) => true, _ => false } }

    #[inline]
    fn is_ghost(&self) -> bool { match *self { Ghost(..) => true, _ => false } }

    #[inline]
    fn matches_equiv<Q : Equiv<K>>(&self, key : &Q, hash : u64) -> bool {
        match *self {
            Empty                               => true,
            Full(ref k, _, h) | Ghost(ref k, h) => hash == h && key.equiv(k),
        }
    }
}

impl <K : Eq, V> Entry<K,V> {
    #[inline]
    fn matches(&self, key : &K, hash : u64) -> bool {
        match *self {
            Empty                               => true,
            Full(ref k, _, h) | Ghost(ref k, h) => hash == h && k == key,
        }
    }
}

pub struct HashMap<K,V> {
    table      : ~[Entry<K,V>],
    capacity   : uint,
    mask       : u64,
    length     : uint,
    ghosts     : uint,
}

impl<K : Eq + IterBytes,V> HashMap<K,V> {
    #[inline]
    pub fn new() -> HashMap<K,V> { HashMap::with_capacity(8) }

    #[inline]
    pub fn with_capacity(sz : uint) -> HashMap<K,V> {
        let capacity = uint::next_power_of_two(sz);
        HashMap {
            table : vec::from_fn(capacity, |_| Empty),
            capacity : capacity,
            mask : (capacity as u64) - 1,
            length : 0,
            ghosts : 0,
        }
    }

    #[inline]
    pub fn capacity(&self) -> uint { self.capacity }

    // This algorithm gleefully stolen from Python
    #[inline]
    fn probe(&self, key : &K, hash : u64) -> uint {
        let mut shifted_hash = hash;
        let mut free         = None;
        let mut i            = shifted_hash & self.mask;
        while !self.table[i].matches(key,hash) {
            if free.is_none() && self.table[i].is_ghost() { free = Some(i); }
            i = ((5 * i) + 1 + shifted_hash) & self.mask;
            shifted_hash = shifted_hash >> PERTURB_SHIFT;
        }
        if self.table[i].is_full() || free.is_none() {
            i as uint
        } else {
            free.unwrap() as uint
        }
    }

    #[inline]
    fn probe_equiv<Q:IterBytes + Equiv<K>>(&self, key : &Q, hash : u64) -> uint {
        let mut shifted_hash = hash;
        let mut free         = None;
        let mut i            = shifted_hash & self.mask;
        while !self.table[i].matches_equiv(key,hash) {
            if free.is_none() && self.table[i].is_ghost() { free = Some(i); }
            i = ((5 * i) + 1 + shifted_hash) & self.mask;
            shifted_hash = shifted_hash >> PERTURB_SHIFT;
        }
        if self.table[i].is_full() || free.is_none() {
            i as uint
        } else {
            free.unwrap() as uint
        }
    }

    // Precondition: this is used by expand, so there must be enough space in the table.
    #[inline]
    fn swap_with_hash(&mut self, key: K, hash : u64, value: V) -> Option<V> {
        let i = self.probe(&key, hash);
        match self.table[i] {
            Empty => {
                self.table[i] = Full(key,value,hash);
                self.length += 1;
                None
            },
            Ghost(..) => {
                self.table[i] = Full(key,value,hash);
                self.length += 1;
                self.ghosts -= 1;
                None
            },
            Full(_,ref mut v, _) => {
                Some( util::replace(v, value) )
            },
        }
    }

    #[inline]
    fn do_expand(&mut self, new_capacity : uint) {
        let mut new_tbl = HashMap::with_capacity( new_capacity );
        for elt in self.table.mut_iter() {
            match util::replace(elt, Empty) {
                Full(k,v,h)        => { new_tbl.swap_with_hash(k,h,v); },
                Empty | Ghost(..)  => { },
            }
        }
        // Copy new table's elements into self.  Note: attempting
        // to do this more directly causes: "use of partially moved
        // value"
        let cap    = new_tbl.capacity;
        let mask   = new_tbl.mask;
        let len    = new_tbl.length;
        let ghosts = new_tbl.ghosts;
        self.table    = new_tbl.table;
        self.capacity = cap;
        self.mask     = mask;
        self.length   = len;
        self.ghosts   = ghosts;
    }

    #[inline]
    fn expand(&mut self) {
        if self.length * 3 > self.capacity * 2 {
            // Expand table if live entries nearing capacity
            self.do_expand( self.capacity * 2 );
        } else if (self.length + self.ghosts) * 3 >= self.capacity * 2 {
            // Rehash to flush out excess ghosts
            self.do_expand( self.capacity );
        }
    }

    #[inline]
    pub fn find_equiv<'a, Q:IterBytes + Equiv<K>>(&'a self, k: &Q) -> Option<&'a V> {
        let i = self.probe_equiv(k, DJBState::djbhash(k));
        match self.table[i] {
            Empty | Ghost(..)   => None,
            Full(_, ref val, _) => Some(val),
        }
    }

    #[inline]
    pub fn iter<'a>(&'a self) -> HashMapIterator<'a, K, V> {
        HashMapIterator { iterator : self.table.iter() }
    }
}

impl<K,V> Container for HashMap<K,V> {
    #[inline]
    fn len(&self) -> uint { self.length }
}

impl<K,V> Mutable for HashMap<K,V> {
    #[inline]
    fn clear(&mut self) { 
        for elt in self.table.mut_iter() { *elt = Empty; }
        self.length = 0;
        self.ghosts = 0;
    }
}

impl<K : Eq + IterBytes,V> Map<K,V> for HashMap<K,V> {
    #[inline]
    fn find<'a>(&'a self, key: &K) -> Option<&'a V> {
        let i = self.probe(key, DJBState::djbhash(key));
        match self.table[i] {
            Empty | Ghost(..)   => None,
            Full(_, ref val, _) => Some(val),
        }
    }
}

impl<K : Eq + IterBytes,V> MutableMap<K,V> for HashMap<K,V> {

    #[inline]
    fn swap(&mut self, key: K, value: V) -> Option<V> {
        self.expand();
        let hash = DJBState::djbhash(&key);
        self.swap_with_hash(key, hash, value)
    }

    #[inline]
    fn pop(&mut self, key: &K) -> Option<V> {
        self.expand();
        let i = self.probe(key, DJBState::djbhash(key));
        let (result,replacement) = match util::replace(&mut self.table[i], Empty) {
            Empty       => (None,Empty),
            Ghost(k,h)  => (None,Ghost(k,h)),
            Full(k,v,h) => {
                self.length -= 1;
                self.ghosts += 1;
                (Some(v),Ghost(k,h))
            },
        };
        self.table[i] = replacement;
        result
    }

    #[inline]
    fn find_mut<'a>(&'a mut self, key: &K) -> Option<&'a mut V> {
        let i = self.probe(key, DJBState::djbhash(key));
        match self.table[i] {
            Empty | Ghost(..)     => None,
            Full(_,ref mut val,_) => Some(val),
        }
    }
}


pub struct HashMapIterator<'a,K,V> {
    priv iterator : vec::VecIterator<'a,Entry<K,V>>,
}

impl<'a,K,V> Iterator<(&'a K, &'a V)> for HashMapIterator<'a,K,V> {
    #[inline]
    fn next(&mut self) -> Option<(&'a K, &'a V)> {
        for elt in self.iterator {
            match *elt {
                Empty | Ghost(..)        => { },
                Full(ref key, ref val,_) => { return Some((key, val)) },
            }
        }
        return None;
    }
}


/* ----------------------------------------------- */

pub struct HashSet<T> {
    priv map : HashMap<T,()>
}

impl<T : Eq + IterBytes> HashSet<T> {
    #[inline]
    pub fn new() -> HashSet<T> { HashSet { map : HashMap::new() } }

    #[inline]
    pub fn iter<'a>(&'a self) -> HashSetIterator<'a, T> {
        HashSetIterator { iterator: self.map.iter() }
    }
}    

impl<T> Container for HashSet<T> {
    #[inline]
    fn len(&self) -> uint { self.map.len() }
}

impl<T> Mutable for HashSet<T> {
    #[inline]
    fn clear(&mut self) { self.map.clear() }
}

impl<T : Eq + IterBytes> Set<T> for HashSet<T> {
    #[inline]
    fn contains(&self, elt : &T) -> bool { self.map.contains_key(elt) }

    #[inline]
    fn is_disjoint(&self, other : &HashSet<T>) -> bool {
        for elt in self.map.table.iter() {
            match *elt {
                Full(ref k,_,_) => {
                    if other.contains(k) { return false; }
                },
                _ => { },
            }
        }
        return true;
    }

    #[inline]
    fn is_subset(&self, other : &HashSet<T>) -> bool {
        for elt in self.map.table.iter() {
            match *elt {
                Full(ref k,_,_) => {
                    if !other.contains(k) { return false; }
                },
                _ => { },
            }
        }
        return true;
    }

    #[inline]
    fn is_superset(&self, other : &HashSet<T>) -> bool {
        other.is_subset(self)
    }
}

impl<T : Eq + IterBytes> MutableSet<T> for HashSet<T> {
    #[inline]
    fn insert(&mut self, value: T) -> bool {
        self.map.insert(value,())
    }

    #[inline]
    fn remove(&mut self, value: &T) -> bool {
        self.map.remove(value)
    }
}

pub struct HashSetIterator<'a,T> {
    priv iterator : HashMapIterator<'a,T,()>,
}

impl<'a,T> Iterator<&'a T> for HashSetIterator<'a,T> {
    #[inline]
    fn next(&mut self) -> Option<&'a T> {
        for (k,_) in self.iterator {
            return Some(k);
        }
        return None;
    }
}


/* ----------------------------------------------- */

#[cfg(test)]
mod tests {
    extern mod extra;

    use super::{DJBState,HashMap,HashSet};

    #[test]
    fn test_empty() {
        let m : HashMap<uint,uint> = HashMap::new();
        assert_eq!(m.len(), 0);
        assert_eq!(m.capacity(), 8);
        assert_eq!(m.find(&1), None);

        let mut count = 0;
        for (_,_) in m.iter() { count += 1; }
        assert_eq!(count, 0);
    }

    #[test]
    fn test_one() {
        let mut m : HashMap<uint,uint> = HashMap::new();
        assert_eq!(m.len(), 0);
        assert!(m.insert(1,400));
        assert_eq!(m.len(), 1);
        assert_eq!(m.capacity(), 8);
        match m.find(&1) {
            Some(y) => assert_eq!(*y,400),
            None => fail!("failure!")
        }
        match m.find_mut(&1) {
            Some(y) => *y = 500,
            _ => fail!("failed!")
        }
        match m.find(&1) {
            Some(y) => assert_eq!(*y,500),
            None => fail!("failure again!")
        }

        let mut count = 0;
        for (_,_) in m.iter() { count += 1; }
        assert_eq!(count, 1);

        match m.pop(&1) {
            Some(y) => assert_eq!(y,500),
            None => fail!("oh, noes!")
        }
        assert_eq!(m.len(), 0);
    }

    #[test]
    fn test_eight() {
        let mut m : HashMap<uint,uint> = HashMap::new();
        let v = [1,3,5,7,9,11,13,15];
        for i in v.iter() {
            assert!(m.insert(*i,100 * *i));
        }
        assert_eq!(m.len(),8);
        assert_eq!(m.capacity(),16);
        assert!( !m.insert(3, 12000) );
 
        let mut count = 0;
        for (_,_) in m.iter() { count += 1; }
        assert_eq!(count, 8);
    }

    #[test]
    fn test_set_empty() {
        let s : HashSet<uint> = HashSet::new();
        assert_eq!(s.len(), 0);
        assert!(!s.contains(&3));
        let mut count = 0;
        for _ in s.iter() { count += 1; }
        assert_eq!(count, 0);
    }

    #[test]
    fn test_set_nonempty() {
        let mut s : HashSet<uint> = HashSet::new();
        let v = [1,3,5,7,9,11,13,15];
        for i in v.iter() {
            assert!(s.insert(*i));
        }
        assert_eq!(s.len(), 8);
        assert!(s.contains(&3));
        let mut count = 0;
        for _ in s.iter() { count += 1; }
        assert_eq!(count, 8);
        let empty : HashSet<uint> = HashSet::new();
        assert!( s.is_disjoint(&empty) );
        assert!( empty.is_subset(&s) );
        assert!( s.is_superset(&empty) );
    }

    #[bench]
    fn hash_bench_siphash(b: &mut extra::test::BenchHarness) {
        let s = "abcdefghijklmnopqrstuvwxyz";
        b.iter(|| { s.hash(); });
    }

    #[bench]
    fn hash_bench_djbhash(b: &mut extra::test::BenchHarness) {
        let s = "abcdefghijklmnopqrstuvwxyz";
        b.iter(|| { DJBState::djbhash(&s); });
    }
}

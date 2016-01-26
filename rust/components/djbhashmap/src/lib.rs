pub mod djbhasher;
pub mod entry;

use std::borrow::Borrow;
use std::cmp::PartialEq;
use std::hash::{Hash,Hasher};
use std::slice::Iter;

use djbhasher::DJBHasher;
use entry::Entry;

const PERTURB_SHIFT : u64 = 5;

pub struct HashMap<K,V> {
    table      : Vec<Entry<K,V>>,
    capacity   : usize,
    mask       : u64,
    length     : usize,
    ghosts     : usize,
}

impl<K,V> HashMap<K,V> where K : Hash + Eq {

    #[inline]
    pub fn new() -> HashMap<K,V> {
        HashMap::with_capacity(8)
    }

    #[inline]
    pub fn with_capacity(sz : usize) -> HashMap<K,V> {
        let capacity = usize::next_power_of_two(sz);
        let mut table = Vec::with_capacity(capacity);
        for _ in 0..capacity { table.push(Entry::Empty); }
        HashMap {
            table    : table,
            capacity : capacity,
            mask     : (capacity as u64) - 1,
            length   : 0,
            ghosts   : 0,
        }
    }

    #[inline]
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    // This algorithm gleefully stolen from Python
    #[inline]
    fn probe(&self, key : &K, hash : u64) -> usize {
        let mut shifted_hash = hash;
        let mut free         = None;
        let mut i            = (shifted_hash & self.mask) as usize;
        while !self.table[i].matches(key,hash) {
            if free.is_none() && self.table[i].is_ghost() { free = Some(i); }
            i = (((5 * i as u64) + 1 + shifted_hash) & self.mask) as usize;
            shifted_hash = shifted_hash >> PERTURB_SHIFT;
        }
        if self.table[i].is_full() || free.is_none() {
            i
        } else {
            free.unwrap()
        }
    }

    #[inline]
    fn probe_equiv<Q:PartialEq<K>>(&self, key : &Q, hash : u64) -> usize {
        let mut shifted_hash = hash;
        let mut free         = None;
        let mut i            = (shifted_hash & self.mask) as usize;
        while !self.table[i].matches(key,hash) {
            if free.is_none() && self.table[i].is_ghost() { free = Some(i); }
            i = (((5 * i as u64) + 1 + shifted_hash) & self.mask) as usize;
            shifted_hash = shifted_hash >> PERTURB_SHIFT;
        }
        if self.table[i].is_full() || free.is_none() {
            i
        } else {
            free.unwrap()
        }
    }

    // Precondition: this is used by expand, so there must be enough space in the table.
    #[inline]
    fn swap_with_hash(&mut self, key : K, hash : u64, value : V) -> Option<V> {
        let i = self.probe(&key, hash);
        let mut elt = &mut self.table[i];
        match elt {
            &mut Entry::Empty => {
                let f = Entry::Full(key,value,hash);
                std::mem::replace(elt, f);
                self.length += 1;
                None
            },
            &mut Entry::Ghost(..) => {
                let f = Entry::Full(key,value,hash);
                std::mem::replace(elt, f);
                self.length += 1;
                self.ghosts -= 1;
                None
            },
            &mut Entry::Full(_,ref mut v, _) => {
                Some( std::mem::replace(v, value) )
            },
        }
    }

    #[inline]
    fn do_expand(&mut self, new_capacity : usize) {
        let mut new_tbl = HashMap::with_capacity( new_capacity );
        for i in 0..self.table.len() {
            match std::mem::replace(&mut self.table[i], Entry::Empty) {
                Entry::Full(k,v,h)               => { new_tbl.swap_with_hash(k,h,v); }
                Entry::Empty | Entry::Ghost(..)  => { }
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
        let capacity = self.capacity;
        if self.length * 3 > capacity * 2 {
            // Expand table if live entries nearing capacity
            self.do_expand( capacity * 2 );
        } else if (self.length + self.ghosts) * 3 >= capacity * 2 {
            // Rehash to flush out excess ghosts
            self.do_expand( capacity );
        }
    }

    #[inline]
    pub fn find_equiv<'a, Q>(&'a self, k : &Q) -> Option<&'a V> where Q : Hash + PartialEq<K> {
        let mut hasher = DJBHasher::new();
        k.hash(&mut hasher);
        let hash = hasher.finish();
        let i = self.probe_equiv(k, hash);
        match &self.table[i] {
            &Entry::Empty | &Entry::Ghost(..) => None,
            &Entry::Full(_, ref val, _)       => Some(val),
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.length
    }

    #[inline]
    pub fn clear(&mut self) { 
        for i in 0..self.table.len() {
            self.table[i] = Entry::Empty;
        }
        self.length = 0;
        self.ghosts = 0;
    }

    pub fn insert(&mut self, k : K, v : V) -> Option<V> {
        self.expand();
        let mut hasher = DJBHasher::new();
        k.hash(&mut hasher);
        let hash = hasher.finish();
        self.swap_with_hash(k, hash, v)
    }

    pub fn get<Q>(&self, k : &Q) -> Option<&V> where K : Borrow<Q>, Q : Hash + PartialEq<K> {
        self.find_equiv(k)
    }

    pub fn iter(&self) -> HashMapIter<K,V> {
        HashMapIter { inner: self.table.iter() }
    }

    pub fn keys(&self) -> HashMapKeys<K,V> {
        HashMapKeys { inner: self.iter() }
    }
}

#[test]
fn test_map_1() {
    let mut map = HashMap::new();
    map.insert("hello", 1);
    assert_eq!(map.get(&"hello"), Some(&1));
    assert_eq!(map.get(&"foo"), None);
}

// ----------------------------------------

pub struct HashMapIter<'l,K: 'l,V: 'l> {
    inner: Iter<'l,Entry<K,V>>,
}

impl<'l,K,V> Iterator for HashMapIter<'l,K,V> {
    type Item = (&'l K, &'l V);
    fn next(&mut self) -> Option<(&'l K, &'l V)> {
        let mut n = self.inner.next();
        loop {
            match n {
                Some(entry) if entry.is_full() => {
                    return Some((entry.key().unwrap(), entry.value().unwrap()))
                }
                Some(..) => {
                    n = self.inner.next();
                }
                None => {
                    return None;
                }
            }
        }
    }
}

pub struct HashMapKeys<'l,K: 'l,V: 'l> {
    inner: HashMapIter<'l,K,V>,
}

impl<'l,K,V> Iterator for HashMapKeys<'l,K,V> {
    type Item = &'l K;
    fn next(&mut self) -> Option<&'l K> {
        match self.inner.next() {
            Some((ref k, _)) => Some(k),
            None => None
        }
    }
}

// ----------------------------------------

pub struct HashSet<T> {
    map : HashMap<T,()>
}

impl<T> HashSet<T> where T : Hash + Eq {

    pub fn new() -> HashSet<T> {
        HashSet { map : HashMap::new() }
    }

    pub fn insert(&mut self, v : T) -> bool {
        match self.map.insert(v, ()) {
            Some(_) => false,
            None    => true,
        }
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn iter(&self) -> HashMapKeys<T,()> {
        self.map.keys()
    }
}

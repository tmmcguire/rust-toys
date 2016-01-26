
#[derive(Clone)]
pub enum Entry<K,V> {
    Empty,                      // This slot is empty
    Full(K,V,u64),              // This slot is holding a key and value
    Ghost(K,u64),               // This slot once held key k
}

impl<K, V> Entry<K,V> {
    #[inline]
    #[allow(dead_code)]         // For completeness; this fn isn't used here
    pub fn is_empty(&self) -> bool {
        match *self {
            Entry::Empty => true,
            _ => false
        }
    }

    #[inline]
    pub fn is_full(&self)  -> bool {
        match *self {
            Entry::Full(..) => true,
            _ => false
        }
    }

    #[inline]
    pub fn is_ghost(&self) -> bool {
        match *self {
            Entry::Ghost(..) => true,
            _ => false
        }
    }

    #[inline]
    pub fn matches<Q : PartialEq<K>>(&self, key : &Q, hash : u64) -> bool {
        match *self {
            Entry::Empty                                      => true,
            Entry::Full(ref k, _, h) | Entry::Ghost(ref k, h) => hash == h && key == k,
        }
    }

    pub fn key(&self) -> Option<&K> {
        match *self {
            Entry::Full(ref k,_,_) => Some(k),
            _ => None
        }
    }

    pub fn value<'l>(&'l self) -> Option<&'l V> {
        match *self {
            Entry::Full(_,ref v,_) => Some(v),
            _ => None
        }
    }
}

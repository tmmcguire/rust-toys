use std::hash::Hasher;

pub struct DJBHasher {
    hash : u64
}

impl DJBHasher {
    #[inline]
    pub fn new() -> DJBHasher {
        DJBHasher { hash : 5381u64 }
    }
}

impl Hasher for DJBHasher {
    fn finish(&self) -> u64 {
        self.hash
    }

    fn write(&mut self, bytes: &[u8]) {
        for i in 0..bytes.len() {
            self.hash = (33u64 * self.hash) ^ bytes[i] as u64
        }
    }
}

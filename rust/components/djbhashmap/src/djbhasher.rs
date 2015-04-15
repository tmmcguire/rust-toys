use std::hash::Hasher;

pub struct DJBState {
    hash : u64
}

impl DJBState {
    #[inline]
    pub fn new() -> DJBState {
        DJBState { hash : 5381u64 }
    }
}

impl Hasher for DJBState {
    fn finish(&self) -> u64 {
        self.hash
    }

    fn write(&mut self, bytes: &[u8]) {
        for i in 0..bytes.len() {
            self.hash = (33u64 * self.hash) ^ bytes[i] as u64
        }
    }
}

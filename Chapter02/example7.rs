use std::collections::HashMap;
use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hasher};

#[derive(Clone)]
struct MyHasher {
    count: u64,
}

impl Hasher for MyHasher {
    fn finish(&self) -> u64 {
        self.count
    }

    fn write(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.count = self.count.wrapping_add(*byte as u64);
        }
    }
}

impl BuildHasher for MyHasher {
    type Hasher = Self;
    fn build_hasher(&self) -> Self::Hasher {
        self.clone()
    }
}

fn main() {
    // <u8, u8> as an example, just to make the type inference
    // happy.
    let map: HashMap<u8, u8> = HashMap::with_hasher(RandomState::new());

    // ------------
    let mut map = HashMap::with_hasher(MyHasher { count: 12345 });
    map.insert("Hello", "World");
}

use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Serialize, Deserialize)]
pub struct Record {
    hash_name: u64,
}

impl Record {
    pub fn from_key<K: Hash>(key: &K) -> Self {
        let mut s = DefaultHasher::new();
        key.hash(&mut s);
        Record {
            hash_name: s.finish(),
        }
    }
}

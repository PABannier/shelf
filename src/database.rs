use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Debug)]
pub struct Database<K, V> {
    pub map: Mutex<HashMap<K, V>>,
}

impl<K, V> Database<K, V> {
    pub fn new() -> Self {
        Database {
            map: Mutex::new(HashMap::<K, V>::new()),
        }
    }
}

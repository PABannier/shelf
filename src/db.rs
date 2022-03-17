use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

pub type Db<K, V> = Arc<Mutex<BTreeMap<K, V>>>;

pub fn new_db<K, V>() -> Db<K, V> {
    Arc::new(Mutex::new(BTreeMap::<K, V>::new()))
}

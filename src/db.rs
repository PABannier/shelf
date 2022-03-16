use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub type Db<K, V> = Arc<Mutex<HashMap<K, V>>>;

pub fn new_db<K, V>() -> Db<K, V> {
    Arc::new(Mutex::new(HashMap::<K, V>::new()))
}

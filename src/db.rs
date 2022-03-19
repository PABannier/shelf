use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

pub type Db<K> = Arc<Mutex<BTreeMap<K, String>>>;

pub fn new_db<K>() -> Db<K> {
    Arc::new(Mutex::new(BTreeMap::<K, String>::new()))
}

use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

pub type Db<K, V> = Arc<Mutex<BTreeMap<K, V>>>;

pub fn new_db<K, V>() -> Db<K, V> {
    Arc::new(Mutex::new(BTreeMap::<K, V>::new()))
}

pub fn mock_db() -> Db<String, String> {
    let mut db = BTreeMap::new();
    db.insert("hello".to_string(), "world".to_string());
    db.insert("hell".to_string(), "hades".to_string());
    db.insert("foo".to_string(), "bar".to_string());
    db.insert("baz".to_string(), "foobar".to_string());
    db.insert("azerty".to_string(), "uiop".to_string());
    Arc::new(Mutex::new(db))
}

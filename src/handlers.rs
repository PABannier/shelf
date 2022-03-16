use std::convert::Infallible;
use warp::{http::StatusCode, reply::json};

use crate::db::Db;
use crate::response::Response;
use crate::{Keyable, Storable};

pub async fn get_key<K: Keyable, V: Storable>(
    key: K,
    db: Db<K, V>,
) -> Result<impl warp::Reply, Infallible> {
    let db = db.lock().unwrap();
    match db.get(&key) {
        Some(value) => Ok(json(&Response::Value {
            key,
            value: value.clone(),
        })),
        None => {
            let msg = format!("key '{}' not found", key);
            Ok(json(&Response::Error::<K, V> { msg }))
        }
    }
}

// pub async fn insert_key<K: Eq + Hash, V: Serialize>(
//     key: K,
//     db: Db<K, String>,
// ) -> Result<impl warp::Reply, Infallible> {
//     let mut db = db.lock().unwrap();
//     db.insert(key, "hello".to_string());
//     Ok(StatusCode::CREATED)
// }

pub async fn delete_key<K: Keyable, V: Storable>(
    key: K,
    db: Db<K, V>,
) -> Result<impl warp::Reply, Infallible> {
    let mut db = db.lock().unwrap();
    match db.remove(&key) {
        Some(_) => Ok(StatusCode::OK),
        None => Ok(StatusCode::NO_CONTENT),
    }
}

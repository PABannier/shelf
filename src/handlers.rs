use std::convert::Infallible;
use warp::http::Response;

use crate::db::Db;
use crate::{Keyable, Storable};

pub async fn get_key<K: Keyable, V: Storable>(
    key: K,
    db: Db<K, V>,
) -> Result<impl warp::Reply, Infallible> {
    let db = db.lock().unwrap();
    match db.get(&key) {
        Some(value) => Ok(Response::builder().body(value.to_string())),
        None => Ok(Response::builder().body("204 NO CONTENT".to_string())),
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
        Some(_) => Ok(Response::builder().body("200 OK".to_string())),
        None => Ok(Response::builder().body("204 NO CONTENT".to_string())),
    }
}

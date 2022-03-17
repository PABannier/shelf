use std::convert::Infallible;
use warp::{http::Response, reply};

use crate::db::Db;
use crate::{Keyable, Storable};

pub async fn get_key_list<'a, K: Keyable, V: Storable>(
    key_prefix: K,
    db: Db<K, V>,
) -> Result<impl warp::Reply, Infallible> {
    let db = db.lock().unwrap();
    let res: Vec<(&K, &V)> = db.range(key_prefix..).collect();
    Ok(reply::json(&res))
}

pub async fn get_key<'a, K: Keyable, V: Storable>(
    key: K,
    db: Db<K, V>,
) -> Result<impl warp::Reply, Infallible> {
    let db = db.lock().unwrap();
    match db.get(&key) {
        Some(value) => Ok(Response::builder().body(value.to_string())),
        None => Ok(Response::builder().body("204 NO CONTENT".to_string())),
    }
}

pub async fn insert_key<'a, K: Keyable, V: Storable>(
    key: K,
    value: V,
    db: Db<K, V>,
) -> Result<impl warp::Reply, Infallible> {
    let mut db = db.lock().unwrap();
    db.insert(key, value);
    Ok(Response::builder().body("200 OK".to_string()))
}

pub async fn delete_key<'a, K: Keyable, V: Storable>(
    key: K,
    db: Db<K, V>,
) -> Result<impl warp::Reply, Infallible> {
    let mut db = db.lock().unwrap();
    match db.remove(&key) {
        Some(_) => Ok(Response::builder().body("200 OK".to_string())),
        None => Ok(Response::builder().body("204 NO CONTENT".to_string())),
    }
}

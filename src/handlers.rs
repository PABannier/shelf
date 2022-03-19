use bytes::BufMut;
use futures::TryStreamExt;
use std::convert::Infallible;
use warp::{
    http::{Response, StatusCode},
    multipart::{FormData, Part},
    reply, Buf, Rejection,
};

use crate::db::Db;
use crate::{Keyable, Storable};

pub async fn get_key_list<K: Keyable, V: Storable>(
    key_prefix: K,
    db: Db<K, V>,
) -> Result<impl warp::Reply, Infallible> {
    let db = db.lock().unwrap();
    let res: Vec<(&K, &V)> = db.range(key_prefix..).collect();
    Ok(reply::json(&res))
}

pub async fn get_key<K: Keyable, V: Storable>(
    key: K,
    db: Db<K, V>,
) -> Result<impl warp::Reply, Infallible> {
    let db = db.lock().unwrap();
    match db.get(&key) {
        Some(value) => Ok(Response::builder()
            .status(StatusCode::OK)
            .body(value.to_string())),
        None => Ok(Response::builder()
            .status(StatusCode::NO_CONTENT)
            .body("204 NO CONTENT".to_string())),
    }
}

pub async fn insert_key<K: Keyable, V: Storable>(
    key: K,
    value: V,
    db: Db<K, V>,
) -> Result<impl warp::Reply, Infallible> {
    let mut db = db.lock().unwrap();
    db.insert(key, value);
    Ok(Response::builder()
        .status(StatusCode::OK)
        .body("200 OK".to_string()))
}

pub async fn delete_key<K: Keyable, V: Storable>(
    key: K,
    db: Db<K, V>,
) -> Result<impl warp::Reply, Infallible> {
    let mut db = db.lock().unwrap();
    match db.remove(&key) {
        Some(_) => Ok(Response::builder()
            .status(StatusCode::OK)
            .body("200 OK".to_string())),
        None => Ok(Response::builder()
            .status(StatusCode::NO_CONTENT)
            .body("204 NO CONTENT".to_string())),
    }
}

pub async fn upload_file<K: Keyable>(
    key: K,
    mut buf: impl Buf,
) -> Result<impl warp::Reply, Rejection> {
    // Draining the buffer (possibly non-contiguous blocks of memory)
    let mut value = Vec::new();
    while buf.has_remaining() {
        value.put(buf.chunk());
        buf.advance(buf.chunk().len());
    }

    // Writing on-disk
    let file_path = format!("./files/{}", key);
    tokio::fs::write(&file_path, value).await.map_err(|e| {
        eprintln!("error writing file: {}", e);
        warp::reject::reject()
    })?;

    Ok(Response::builder()
        .status(StatusCode::OK)
        .body("200 OK".to_string()))
}

pub async fn download_file<K: Keyable>(key: K) -> Result<impl warp::Reply, Rejection> {
    let file_path = format!("./files/{}", key);

    let file = tokio::fs::read(file_path).await.map_err(|e| {
        eprintln!("error reading file: {}", e);
        warp::reject::reject()
    })?;

    Ok(Response::builder().status(StatusCode::OK).body(file))
}

pub async fn rejection(err: Rejection) -> Result<impl warp::Reply, Infallible> {
    let (code, message) = if err.is_not_found() {
        (StatusCode::NOT_FOUND, "Not found".to_string())
    } else if err.find::<warp::reject::PayloadTooLarge>().is_some() {
        (StatusCode::BAD_REQUEST, "Payload too large".to_string())
    } else {
        eprintln!("An unhandled error occured...");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal server error".to_string(),
        )
    };

    Ok(warp::reply::with_status(message, code))
}

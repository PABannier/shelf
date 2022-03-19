use bytes::BufMut;
use futures::TryStreamExt;
use std::convert::Infallible;
use warp::{
    http::{Response, StatusCode},
    multipart::{FormData, Part},
    reply, Rejection,
};

use crate::db::Db;
use crate::hash::Record;
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
    data: FormData,
    db: Db<K, Record>,
) -> Result<impl warp::Reply, Rejection> {
    let mut db = db.lock().unwrap();

    let parts: Vec<Part> = data.try_collect().await.map_err(|e| {
        eprintln!("form error {}", e);
        warp::reject::reject()
    })?;

    for part in parts {
        if part.name() == "file" {
            let content_type = part.content_type();
            let value = part
                .stream()
                .try_fold(Vec::new(), |mut vec, data| {
                    vec.put(data);
                    async move { Ok(vec) }
                })
                .await
                .map_err(|e| {
                    eprintln!("reading file error {}", e);
                    warp::reject::reject()
                })?;

            let file_name = format!("./files/{}", key);

            tokio::fs::write(file_name, value).await.map_err(|e| {
                eprintln!("error writing file {}", e);
                warp::reject::reject()
            })?;
        }
    }

    let record = Record::from_key(&key);
    db.insert(key, record);

    Ok(Response::builder()
        .status(StatusCode::OK)
        .body("200 OK".to_string()))
}

pub async fn rejection(err: Rejection) -> std::result::Result<impl warp::Reply, Infallible> {
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

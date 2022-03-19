use std::convert::Infallible;

use serde::{Deserialize, Serialize};
use warp::Filter;

use crate::db::Db;
use crate::{handlers, Keyable};

#[derive(Serialize, Deserialize)]
struct Value {
    value: String,
}

pub fn commands<K: Keyable>(
    db: Db<K>,
) -> impl Filter<Extract = impl warp::Reply, Error = Infallible> + Clone {
    // get_key_list(db.clone())
    //     .or(get_key(db.clone()))
    //     .or(delete_key(db.clone()))
    //     .or(insert_key(db.clone()))
    //     .or(upload_file::<K>())
    //     .recover(handlers::rejection)
    upload_file::<K>(db.clone())
        .or(download_file::<K>(db.clone()))
        .recover(handlers::rejection)
}

pub fn upload_file<K: Keyable>(
    db: Db<K>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // TODO: restrict body size to MAX_SIZE
    warp::path("upload")
        .and(warp::path::param::<K>())
        .and(warp::put())
        .and(warp::body::aggregate())
        .and(with_db(db))
        .and_then(handlers::upload_file)
}

pub fn download_file<K: Keyable>(
    db: Db<K>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("download")
        .and(warp::path::param::<K>())
        .and(warp::get())
        .and(with_db(db))
        .and_then(handlers::download_file)
}

pub fn get_key_list<K: Keyable>(
    db: Db<K>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("search")
        .and(warp::path::param::<K>())
        .and(warp::get())
        .and(with_db::<K>(db))
        .and_then(handlers::get_key_list::<K>)
}

pub fn get_key<K: Keyable>(
    db: Db<K>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path::param::<K>()
        .and(warp::get())
        .and(with_db::<K>(db))
        .and_then(handlers::get_key::<K>)
}

pub fn insert_key<K: Keyable>(
    db: Db<K>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path::param::<K>()
        .and(warp::put())
        .and(json_body())
        .and(with_db::<K>(db))
        .and_then(handlers::insert_key::<K>)
}

pub fn delete_key<K: Keyable>(
    db: Db<K>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path::param::<K>()
        .and(warp::delete())
        .and(with_db::<K>(db))
        .and_then(handlers::delete_key::<K>)
}

fn with_db<K: Send + Sync>(
    db: Db<K>,
) -> impl Filter<Extract = (Db<K>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}

fn json_body<V: for<'a> serde::Deserialize<'a> + Send>(
) -> impl Filter<Extract = (V,), Error = warp::Rejection> + Clone {
    // Limit to 16Kb the size of the request
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

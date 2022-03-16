use serde::{Deserialize, Serialize};
use warp::Filter;

use crate::db::Db;
use crate::{handlers, Keyable, Storable};

#[derive(Serialize, Deserialize)]
struct Value {
    value: String,
}

pub fn commands<K: Keyable, V: Storable>(
    db: Db<K, V>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    get_key(db.clone()).or(delete_key(db.clone()))
}

pub fn get_key<K: Keyable, V: Storable>(
    db: Db<K, V>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path::param::<K>()
        .and(warp::get())
        .and(with_db::<K, V>(db))
        .and_then(handlers::get_key::<K, V>)
}

// pub fn insert_key<
//     K: 'static + Send + Sync + Eq + Hash + Display + FromStr,
//     V: 'static + Send + Sync + Serialize,
// >(
//     db: Db<K, V>,
// ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
//     warp::path!("keys" / String)
//         .and(warp::put())
//         .and(json_body())
//         .and(with_db::<K, V>(db))
//         .and_then(handlers::insert_key::<K, V>)
// }

pub fn delete_key<K: Keyable, V: Storable>(
    db: Db<K, V>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path::param::<K>()
        .and(warp::delete())
        .and(with_db::<K, V>(db))
        .and_then(handlers::delete_key::<K, V>)
}

fn with_db<K: Send + Sync, V: Send + Sync>(
    db: Db<K, V>,
) -> impl Filter<Extract = (Db<K, V>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}

// fn json_body() -> impl Filter<Extract = (Value,), Error = warp::Rejection> + Clone {
//     warp::body::content_length_limit(1024 * 16).and(warp::body::json())
// }

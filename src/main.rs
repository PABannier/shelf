mod db;
mod filters;
mod handlers;
mod hash;
mod response;

#[cfg(test)]
mod tests;

use crate::db::{new_db, Db};
use serde::Serialize;
use std::fmt::Display;
use std::hash::Hash;
use std::str::FromStr;
use warp::Filter;

pub trait Keyable:
    'static
    + Send
    + Sync
    + Eq
    + Hash
    + Display
    + Clone
    + Serialize
    + FromStr
    + Ord
    + std::convert::AsRef<[u8]>
{
}
impl Keyable for String {}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    // Instantiate a new database
    let db: Db<String> = new_db();
    db.lock()
        .unwrap()
        .insert("hello".to_string(), "world".to_string());

    db.lock()
        .unwrap()
        .insert("hell".to_string(), "hades".to_string());

    let api = filters::commands(db);
    let routes = api.with(warp::log("shelf"));

    // Start server...
    println!("Listening at 127.0.0.1:3000...");
    warp::serve(routes).run(([127, 0, 0, 1], 3000)).await;
}

use crate::db::Db;
use crate::filters::*;
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

fn mock_db() -> Db<String> {
    let mut db = BTreeMap::new();
    db.insert("hello".to_string(), "world".to_string());
    db.insert("hell".to_string(), "hades".to_string());
    db.insert("foo".to_string(), "bar".to_string());
    db.insert("baz".to_string(), "foobar".to_string());
    db.insert("azerty".to_string(), "uiop".to_string());
    Arc::new(Mutex::new(db))
}

#[tokio::test]
async fn test_get() {
    let db = mock_db();
    let filter = get_key(db);

    let res = warp::test::request().path("/hello").reply(&filter).await;
    assert_eq!(res.status(), 200);
    assert_eq!(res.body(), "world");

    let res = warp::test::request()
        .path("/a_non_existing_key")
        .reply(&filter)
        .await;
    assert_eq!(res.status(), 204);
    assert_eq!(res.body(), "204 NO CONTENT");
}

#[tokio::test]
async fn test_list_key() {
    let db = mock_db();
    let filter = get_key_list(db);

    let res = warp::test::request()
        .path("/search/hel")
        .reply(&filter)
        .await;
    assert_eq!(res.status(), 200);
    assert_eq!(res.body(), "[[\"hell\",\"hades\"],[\"hello\",\"world\"]]");

    let res = warp::test::request()
        .path("/search/non_existing_prefix")
        .reply(&filter)
        .await;
    assert_eq!(res.status(), 200);
    assert_eq!(res.body(), "[]");
}

#[tokio::test]
async fn test_delete() {
    let db = mock_db();
    let filter = delete_key(db);

    let res = warp::test::request()
        .method("DELETE")
        .path("/foo")
        .reply(&filter)
        .await;
    assert_eq!(res.status(), 200);
    assert_eq!(res.body(), "200 OK");

    let res = warp::test::request()
        .method("DELETE")
        .path("/a_non_existing_key")
        .reply(&filter)
        .await;
    assert_eq!(res.status(), 204);
    assert_eq!(res.body(), "204 NO CONTENT");
}

// #[tokio::test]
// async fn test_put() {
//     let db = mock_db();
//     let filter = insert_key(db);

//     let res = warp::test::request()
//         .method("PUT")
//         .path("/thisisatest")
//         .reply(&filter)
//         .await;
//     assert_eq!(res.status(), 200);
//     assert_eq!(res.body(), "");
// }

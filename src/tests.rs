use duct::{self, cmd};
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

const TEST_URI: &str = "http://localhost:3000";

lazy_static! {
    static ref SERVER: Arc<Mutex<duct::ReaderHandle>> = {
        println!("Starting mock server");
        Arc::new(Mutex::new(
            cmd!("cargo", "run", "3000").reader().expect("Valid server"),
        ))
    };
}

async fn initialize_server() {
    lazy_static::initialize(&SERVER);
}

#[tokio::test]
async fn insertion_get_works() {
    // Setup server
    initialize_server().await;

    let client = reqwest::Client::new();
    let res = client.put(TEST_URI).body("foo bar").send().await.unwrap();

    println!("{:#?}", res);

    let resp = reqwest::get(TEST_URI)
        .await
        .unwrap()
        .json::<HashMap<String, String>>()
        .await
        .unwrap();

    println!("{:#?}", resp);
}

#[test]
fn deletion_works() {}

#[test]
fn unknown_command() {}

#[test]
fn non_existing_key() {}

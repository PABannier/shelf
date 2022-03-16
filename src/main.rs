extern crate futures;
extern crate tokio;
extern crate tokio_stream;
extern crate tokio_util;

mod database;
mod request;
mod response;
mod server;

#[cfg(test)]
mod tests;

use crate::server::start_server;
use std::env;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let port = env::args().nth(1).unwrap_or_else(|| "3000".to_string());
    let port = port.parse::<i16>().unwrap();

    println!("Listening at 127.0.0.1:{}", port);

    let res = start_server(port).await;
    res
}

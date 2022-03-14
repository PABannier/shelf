extern crate atoi;
extern crate redis;
extern crate thiserror;
extern crate tokio;

mod command;
mod connection;
mod frame;
mod server;

#[cfg(test)]
mod tests;

use crate::server::start_server;
use clap::Parser;

/// Arguments for setting up the master server
#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
struct SetupArgs {
    /// URI of the Filer store (Redis URI)
    #[clap(short, long, default_value = "redis://127.0.0.1/")]
    db_uri: String,

    /// Port to which master server listens
    #[clap(short, long, default_value_t = 3000)]
    port: u16,
}

#[tokio::main]
async fn main() {
    let args = SetupArgs::parse();

    let port = args.port;
    let db_uri = &args.db_uri;

    println!("Starting server on port {}", port);

    start_server(port, db_uri).await;
}

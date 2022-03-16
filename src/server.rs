use tokio::net::TcpListener;
use tokio_stream::StreamExt;
use tokio_util::codec::{Framed, LinesCodec};

use futures::SinkExt;
use std::error::Error;
use std::sync::Arc;

use crate::database::Database;
use crate::request::Request;
use crate::response::Response;

pub async fn start_server(port: i16) -> Result<(), Box<dyn Error>> {
    let port = format!("127.0.0.1:{}", port);
    let listener = TcpListener::bind(port).await?;
    let db: Arc<Database<String, String>> = Arc::new(Database::new());

    loop {
        match listener.accept().await {
            Ok((socket, _)) => {
                let db = db.clone();

                tokio::spawn(async move {
                    let mut lines = Framed::new(socket, LinesCodec::new());

                    while let Some(result) = lines.next().await {
                        match result {
                            Ok(line) => {
                                let response = handle_request(&line, &db);
                                let response = response.serialize();

                                if let Err(e) = lines.send(response.as_str()).await {
                                    println!("error on sending response; error = {:?}", e);
                                }
                            }
                            Err(e) => {
                                println!("error on decoding from socket; error = {:?}", e);
                            }
                        }
                    }
                });
            }
            Err(e) => println!("error accepting socket; error = {:?}", e),
        }
    }
}

pub fn handle_request(line: &str, db: &Arc<Database<String, String>>) -> Response {
    let request = match Request::parse(line) {
        Ok(req) => req,
        Err(e) => return Response::Error { msg: e },
    };

    let mut db = db.map.lock().unwrap();
    match request {
        Request::Get { key } => match db.get(&key) {
            Some(value) => Response::Value {
                key,
                value: value.to_string(),
            },
            None => Response::Error {
                msg: format!("no key {}", key),
            },
        },
        Request::Put { key, value } => {
            let previous = db.insert(key.clone(), value.clone());
            Response::Set {
                key,
                value,
                previous,
            }
        }
        Request::Delete { key } => match db.remove(&key) {
            Some(value) => Response::Value {
                key,
                value: value.to_string(),
            },
            None => Response::Error {
                msg: format!("no key {}", key),
            },
        },
    }
}

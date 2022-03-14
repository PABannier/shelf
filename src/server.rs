use crate::Command::{self, Delete, Get, Put};
use tokio::net::{TcpListener, TcpStream};

pub async fn start_server(port: u16, db_uri: &str) {
    let client = redis::Client::open(db_uri).unwrap();

    let address = format!("127.0.0.1:{}", port);
    let listener = TcpListener::bind(&address).await.unwrap();
    println!(
        "Master server started at {}. Listening to requests...",
        address
    );

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let connection = client.get_async_connection().await.unwrap();

        tokio::spawn(async move {
            process_requests(socket, connection).await;
        });
    }
}

async fn process_requests(socket: TcpStream, redis_conn: redis::aio::Connection) {
    let mut conn = Connection::new(socket);

    while let Some(frame) = conn.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Put(cmd) => {
                // TODO: insert to Redis database
                Frame::Simple("200. OK.".to_string())
            }
            Get(cmd) => {
                // TODO: get value from Redis database
                Frame::Null
            }
            Delete(cmd) => {
                // TODO: Delete value from Redis database
                Frame::Simple("OK".to_string())
            }
            cmd => Frame::Simple("Unknown command".to_string()),
        };

        conn.write_frame(&response).await.unwrap();
    }
}

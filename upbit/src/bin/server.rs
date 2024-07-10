use futures_util::{StreamExt, SinkExt};
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::protocol::Message;

async fn handle_connection(stream: tokio::net::TcpStream) {
    let ws_stream = accept_async(stream).await.expect("Error during the websocket handshake");

    println!("New WebSocket connection: {}", ws_stream.get_ref().peer_addr().unwrap());

    let (mut write, mut read) = ws_stream.split();

    // Echo received messages
    while let Some(message) = read.next().await {
        match message {
            Ok(msg) => {
                println!("Received: {:?}", msg);
                if write.send(msg).await.is_err() {
                    println!("Error sending message");
                    return;
                }
            },
            Err(e) => {
                println!("Error: {:?}", e);
                return;
            }
        }
    }
}

async fn run_server() {
    let listener = TcpListener::bind("127.0.0.1:6851").await.expect("Can't bind to address");

    println!("WebSocket server listening on ws://127.0.0.1:6851");

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(handle_connection(stream));
    }
}

#[tokio::main]
async fn main() {
    run_server().await;
}

use futures_util::{StreamExt, SinkExt};
use tokio_tungstenite::tungstenite::protocol::Message;
use url::Url;

async fn run_client() {
    let url = Url::parse("ws://127.0.0.1:6851").unwrap();
    let (ws_stream, _) = tokio_tungstenite::connect_async(url).await.expect("Failed to connect");

    println!("WebSocket client connected");

    let (mut write, mut read) = ws_stream.split();

    // Send a test message to the server
    let msg = Message::Text("Hello WebSocket Server!".into());
    write.send(msg).await.expect("Failed to send message");

    // Receive the echo message from the server
    while let Some(message) = read.next().await {
        match message {
            Ok(msg) => println!("Received: {:?}", msg),
            Err(e) => {
                println!("Error: {:?}", e);
                return;
            }
        }
    }
}

#[tokio::main]
async fn main() {
    run_client().await;
}

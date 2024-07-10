// 통신 테스트 용도
use futures_util::{StreamExt, SinkExt};
use tokio_tungstenite::tungstenite::protocol::Message;
use url::Url;
use serde_json::json;


async fn run_client() -> Result<(), Box<dyn std::error::Error>> {
    let url = Url::parse("wss://api.upbit.com/websocket/v1").unwrap();
    let (ws_stream, _) = tokio_tungstenite::connect_async(url).await.expect("Failed to connect");

    println!("WebSocket client connected");

    let (mut write, mut read) = ws_stream.split();

    // WebSocket 구독 메시지 작성
    let subscribe_message = json!([
        {
            "ticket": "stockispsyduck"
        },
        {
            "type": "ticker",
            "codes": ["KRW-BTC"],
            "isOnlyRealtime": true,
        },
    ]);

    // 구독 메시지 전송
    write.send(Message::Text(subscribe_message.to_string())).await?;
    println!("Subscription message sent");

    // 메시지 수신 및 처리
    while let Some(message) = read.next().await {
        match message {
            Ok(msg) => match msg {
                Message::Text(text) => println!("Received: {}", text),
                Message::Binary(bin) => match std::str::from_utf8(&bin) {
                    Ok(text) => println!("Received binary data (decoded as text): {}", text),
                    Err(e) => println!("Failed to decode binary data: {}", e),
                },    
                    _ => (),
            },
            Err(e) => {
                println!("Error receiving message: {}", e);
                break;
            }
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    run_client().await;
}
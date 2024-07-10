// // 통신 테스트 용도
// use futures_util::{StreamExt, SinkExt};
// use tokio_tungstenite::tungstenite::protocol::Message;
// use url::Url;
// use serde_json::json;
// use std::time::{Instant, Duration};



// async fn run_client() -> Result<(), Box<dyn std::error::Error>> {
//     let url = Url::parse("wss://api.upbit.com/websocket/v1").unwrap();
//     let (ws_stream, _) = tokio_tungstenite::connect_async(url).await.expect("Failed to connect");

//     println!("WebSocket client connected");

//     let (mut write, mut read) = ws_stream.split();

//     // WebSocket 구독 메시지 작성
//     let subscribe_message = json!([
//         {
//             "ticket": "stockispsyduck"
//             // "ticket": &ticket_id[..6],
//         },
//         {
//             "type": "ticker",
//             "codes": ["KRW-BTC"],
//             // "isOnlyRealtime": true,
//         },
//     ]);

//     // 구독 메시지 전송
//     write.send(Message::Text(subscribe_message.to_string())).await?;

//     // 시작 시간 기록
//     let mut start = Instant::now();

//     // 메시지 수신 및 처리
//     while let Some(message) = read.next().await {
//         let mut end = Instant::now();
//         let elapsed = end.duration_since(start).as_secs_f64();
//         start = Instant::now();
//         match message {
//             Ok(msg) => println!("Received binary data (decoded as text): {}, Time elapsed: {:10}", msg, elapsed),
//             Err(e) => {
//                 println!("Error receiving message: {}", e);
//                 break;
//             }
//         }
//     }

//     Ok(())
// }

// #[tokio::main]
// async fn main() {
//     run_client().await;
// }


use futures_util::{StreamExt, SinkExt};
use tokio_tungstenite::tungstenite::protocol::Message;
use url::Url;
use serde_json::json;
use std::time::{Instant, Duration};
use tokio::time::{sleep, timeout};
use std::fs::File;
use serde::Serialize;

#[derive(Serialize)]
struct TradeData {
    trade_timestamp: f64,
    elapsed_time: f64,
}

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
            "codes": ["KRW-BTC"]
        },
    ]);

    // 구독 메시지 전송
    write.send(Message::Text(subscribe_message.to_string())).await?;

    // 데이터 저장을 위한 벡터
    let mut data: Vec<TradeData> = Vec::new();

    // 시작 시간 기록
    let mut start = Instant::now();

    // 10분 동안 메시지 수신 및 처리
    let duration = Duration::from_secs(60*60*2); // 2시간
    let mut interval = tokio::time::interval(duration);
    interval.tick().await; // 첫 번째 틱을 기다림

    loop {
        let msg_fut = read.next();
        let dur_fut = interval.tick();

        tokio::select! {
            Some(message) = msg_fut => {
                let end = Instant::now();
                let elapsed = end.duration_since(start).as_secs_f64();
                start = Instant::now();
                match message {
                    Ok(msg) => {
                        println!("Received binary data (decoded as text): {}, Time elapsed: {:10}", msg, elapsed);
                        
                        // msg를 json으로 파싱하여 trade_timestamp 추출
                        let json_msg: serde_json::Value = serde_json::from_str(&msg.to_string())?;
                        let trade_timestamp = json_msg.get("trade_timestamp").and_then(|ts| ts.as_f64()).unwrap_or_else(|| end.elapsed().as_secs_f64());

                        data.push(TradeData { trade_timestamp, elapsed_time: elapsed });
                    },
                    Err(e) => {
                        println!("Error receiving message: {}", e);
                        break;
                    }
                }
            }
            _ = dur_fut => {
                println!("10 minutes elapsed, stopping the client.");
                break;
            }
        }
    }

    // 데이터를 CSV 파일로 저장
    let file = File::create("trade_data_rust.csv")?;
    let mut wtr = csv::Writer::from_writer(file);
    for record in data {
        wtr.serialize(record)?;
    }
    wtr.flush()?;
    println!("Data saved to trade_data.csv");

    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(e) = run_client().await {
        println!("Error: {}", e);
    }
}

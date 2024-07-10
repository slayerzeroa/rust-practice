use reqwest;
use tokio;
use std::collections::HashMap;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://api.upbit.com/v1/trades/ticks";

    // 필수 파라미터 추가
    let mut params = HashMap::new();
    params.insert("market", "KRW-BTC");
    params.insert("count", "1");

    let client = reqwest::Client::new();

    loop {
        let res = client.get(url)
            .query(&params)
            .send()
            .await?
            .text()
            .await?;

        println!("body = {res:?}");

        // 5초 동안 대기
        sleep(Duration::from_secs(5)).await;
    }
}
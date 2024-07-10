use reqwest;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://api.upbit.com/v1/market/all";

    let body = reqwest::get(url)
        .await?
        .text()
        .await?;

    println!("body = {body:?}");
    Ok(())
}

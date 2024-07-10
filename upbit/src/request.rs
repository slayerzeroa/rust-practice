use reqwest::blocking::{Client, Response};
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;

// Error handling
#[derive(Debug)]
struct ApiError(String);

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "API Error: {}", self.0)
    }
}

impl Error for ApiError {}

pub fn call_get(url: &str, params: &HashMap<&str, &str>) -> Result<Response, Box<dyn Error>> {
    let client = Client::new();
    let resp = client.get(url).query(&params).send()?;
    
    if resp.status().is_success() {
        Ok(resp)
    } else {
        Err(Box::new(ApiError(format!(
            "Request failed with status: {}",
            resp.status()
        ))))
    }
}

// Function to call public API
pub fn call_public_api(url: &str, params: HashMap<&str, &str>) -> Result<(Vec<Market>, HashMap<String, String>), Box<dyn Error>> {
    let client = Client::new();
    let resp = client.get(url).query(&params).send()?;

    if resp.status().is_success() {
        let data: Vec<Market> = resp.json()?;
        let headers = resp.headers();

        let remaining_req = headers.get("Remaining-Req").unwrap_or(&"".parse().unwrap()).to_str()?;
        let limit = parse_remaining_req(remaining_req);

        Ok((data, limit))
    } else {
        Err(Box::new(ApiError(format!(
            "Request failed with status: {}",
            resp.status()
        ))))
    }
}

pub fn parse_remaining_req(remaining_req: &str) -> HashMap<String, String> {
    let mut limit = HashMap::new();
    // 예제 구문 분석 로직 (필요에 따라 수정)
    for part in remaining_req.split(';') {
        let mut iter = part.split('=');
        if let (Some(key), Some(value)) = (iter.next(), iter.next()) {
            limit.insert(key.to_string(), value.to_string());
        }
    }
    limit
}

// The get_tickers function
pub fn get_tickers(fiat: &str, is_details: bool, limit_info: bool, verbose: bool) -> Result<Value, Box<dyn Error>> {
    let url = "https://api.upbit.com/v1/market/all";
    let mut params = HashMap::new();
    params.insert("isDetails", if is_details { "true" } else { "false" });

    let (markets, req_limit_info) = call_public_api(url, params)?;

    let tickers: Vec<Value> = if verbose || is_details {
        markets.into_iter()
            .filter(|x| x.market.starts_with(fiat))
            .map(|x| serde_json::to_value(x).unwrap())
            .collect()
    } else {
        markets.into_iter()
            .filter(|x| x.market.starts_with(fiat))
            .map(|x| serde_json::json!(x.market))
            .collect()
    };

    if limit_info {
        Ok(serde_json::json!({ "tickers": tickers, "limit_info": req_limit_info }))
    } else {
        Ok(serde_json::json!(tickers))
    }
}
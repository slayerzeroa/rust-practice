use mysql::*;
use mysql::prelude::*;
use barter_data::exchange::gateio::spot::GateioSpot;
use barter_data::{
    exchange::{
        binance::{futures::BinanceFuturesUsd, spot::BinanceSpot},
        coinbase::Coinbase,
        okx::Okx,
        bybit::{futures::BybitPerpetualsUsd, spot::BybitSpot},
    },
    streams::Streams,
    subscription::trade::PublicTrades,
};
use barter_integration::model::instrument::kind::InstrumentKind;
use futures::StreamExt;
// use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 데이터베이스 연결 설정
    let url = "";
    let pool = Pool::new(url)?;

    // 공개 거래 스트림을 위한 초기화
    let streams = Streams::<PublicTrades>::builder()
        .subscribe([
            (BinanceSpot::default(), "btc", "usdt", InstrumentKind::Spot, PublicTrades),
            (BinanceSpot::default(), "eth", "usdt", InstrumentKind::Spot, PublicTrades),
            (BinanceSpot::default(), "sol", "usdt", InstrumentKind::Spot, PublicTrades),
        ])
        .subscribe([
            (BinanceFuturesUsd::default(), "btc", "usdt", InstrumentKind::Perpetual, PublicTrades),
            (BinanceFuturesUsd::default(), "eth", "usdt", InstrumentKind::Perpetual, PublicTrades),
            (BinanceFuturesUsd::default(), "sol", "usdt", InstrumentKind::Perpetual, PublicTrades),
        ])
        .subscribe([
            (Coinbase, "btc", "usd", InstrumentKind::Spot, PublicTrades),
            (Coinbase, "eth", "usd", InstrumentKind::Spot, PublicTrades),
            (Coinbase, "sol", "usd", InstrumentKind::Spot, PublicTrades),
        ])
        .subscribe([
            (GateioSpot::default(), "btc", "usdt", InstrumentKind::Spot, PublicTrades),
            (GateioSpot::default(), "eth", "usdt", InstrumentKind::Spot, PublicTrades),
            (GateioSpot::default(), "sol", "usdt", InstrumentKind::Spot, PublicTrades),
        ])
        .subscribe([
            (Okx, "btc", "usdt", InstrumentKind::Spot, PublicTrades),
            (Okx, "eth", "usdt", InstrumentKind::Spot, PublicTrades),
            (Okx, "sol", "usdt", InstrumentKind::Spot, PublicTrades),
            (Okx, "btc", "usdt", InstrumentKind::Perpetual, PublicTrades),
            (Okx, "eth", "usdt", InstrumentKind::Perpetual, PublicTrades),
            (Okx, "sol", "usdt", InstrumentKind::Perpetual, PublicTrades),
        ])
        .subscribe([
            (BybitSpot::default(), "btc", "usdt", InstrumentKind::Spot, PublicTrades),
            (BybitSpot::default(), "eth", "usdt", InstrumentKind::Spot, PublicTrades),
            (BybitSpot::default(), "sol", "usdt", InstrumentKind::Spot, PublicTrades),
        ])
        .subscribe([
            (BybitPerpetualsUsd::default(), "btc", "usdt", InstrumentKind::Perpetual, PublicTrades),
            (BybitPerpetualsUsd::default(), "eth", "usdt", InstrumentKind::Perpetual, PublicTrades),
            (BybitPerpetualsUsd::default(), "sol", "usdt", InstrumentKind::Perpetual, PublicTrades),
        ])
        .init()
        .await?;

    // 모든 거래소의 공개 거래 스트림을 하나의 스트림으로 결합
    let mut joined_stream = streams.join_map().await;

    // 스트림에서 데이터 받기 및 데이터베이스에 삽입
    while let Some((exchange, trade)) = joined_stream.next().await {
        println!("Exchange: {exchange}, Market<PublicTrade>: {trade:?}");
        let mut conn = pool.get_conn()?;
        
        let stmt = format!(
            r"INSERT INTO PublicTrades (id, exchange_time, received_time, exchange_name, base_currency, quote_currency, instrument_kind, price, amount, side)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        );

        conn.exec_drop(
            stmt,
            (
                trade.kind.id,
                trade.exchange_time.naive_utc().to_string(), // DateTime<Utc>를 NaiveDateTime으로 변환
                trade.received_time.naive_utc().to_string(),
                exchange.to_string(),
                trade.instrument.base.to_string(),
                trade.instrument.quote.to_string(),
                trade.instrument.kind.to_string(),
                trade.kind.price,
                trade.kind.amount,
                trade.kind.side.to_string(),
            ),
        )?;
    }

    Ok(())
}

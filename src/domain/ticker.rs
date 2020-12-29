use serde::{Deserialize, Serialize};
// use chrono::{DateTime, Utc};
#[derive(Debug, Deserialize, Serialize)]
pub struct GetTickerResponse {
    pub ask: String,
    pub bid: String,
    pub last_trade: String,
    pub pair: String,
    pub rolling_24_hour_volume: String,
    pub status: String,
    pub timestamp: i64,
}

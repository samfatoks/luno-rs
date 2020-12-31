use super::convert_is_buy_to_order_type;
use crate::OrderType;
use chrono::serde::ts_milliseconds;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Trade {
    pub price: String,
    pub sequence: i64,
    #[serde(with = "ts_milliseconds")]
    pub timestamp: DateTime<Utc>,
    #[serde(rename = "is_buy", deserialize_with = "convert_is_buy_to_order_type")]
    pub order_type: OrderType,
    pub volume: String,
}

#[derive(Deserialize, Serialize)]
pub struct ListTradesResponse {
    pub trades: Vec<Trade>,
}

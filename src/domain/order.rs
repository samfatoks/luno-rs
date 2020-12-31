use crate::Error;
use chrono::serde::ts_milliseconds;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{fmt, str};

#[derive(Debug, Deserialize, Serialize)]
pub enum OrderType {
    BID,
    ASK,
}
impl fmt::Display for OrderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrderType::BID => write!(f, "BID"),
            OrderType::ASK => write!(f, "ASK"),
        }
    }
}

impl str::FromStr for OrderType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "BID" => Ok(OrderType::BID),
            "ASK" => Ok(OrderType::ASK),
            _ => Err(Error::InvalidCurrencyPair(s.to_string())),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Order {
    pub order_id: String,
    #[serde(with = "ts_milliseconds")]
    pub creation_timestamp: DateTime<Utc>,
    #[serde(with = "ts_milliseconds")]
    pub expiration_timestamp: DateTime<Utc>,
    #[serde(with = "ts_milliseconds")]
    pub completed_timestamp: DateTime<Utc>,
    #[serde(rename = "type")]
    pub order_type: OrderType,
    pub state: String,
    pub limit_price: String,
    pub limit_volume: String,
    pub base: String,
    pub counter: String,
    pub fee_base: String,
    pub fee_counter: String,
    pub pair: String,
}

#[derive(Deserialize)]
pub struct ListOrdersResponse {
    pub orders: Vec<Order>,
}

/// OrderBookEntry contains the limit price and available volume.
#[derive(Debug, Deserialize, Serialize)]
pub struct OrderBookEntry {
    /// Limit price
    pub price: String,
    /// Volume available
    pub volume: String,
}

/// Contains a list of all bids and asks for the currency pair specified in the Order Book
#[derive(Debug, Deserialize, Serialize)]
pub struct OrderBook {
    pub asks: Vec<OrderBookEntry>,
    pub bids: Vec<OrderBookEntry>,
    #[serde(with = "ts_milliseconds")]
    pub timestamp: DateTime<Utc>,
}

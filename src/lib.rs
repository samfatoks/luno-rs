//! This library is a Rust wrapper for [Luno API](https://www.luno.com/api)
//!
//! ## Authentication
//!
//! Please visit the [Settings](https://www.luno.com/wallet/settings/api_keys) page
//! to generate an API key.
//!
//! ## Usage
//!
//! Put this in your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! luno = "0.1"
//! ```
//!
//! ### Example usage
//!
//! __LunoClient__
//!
//! ```rust
//! #[macro_use]
//! extern crate log;
//!
//! use luno_rs::LunoClient;
//! use std::env;
//!
//! #[async_std::main]
//! async fn main() {
//!     env_logger::builder().format_timestamp_millis().init();
//!
//!     let api_id = env::var("LUNO_API_ID").unwrap();
//!     let api_secret = env::var("LUNO_API_SECRET").unwrap();
//!
//!     let client = LunoClient::new(api_id, api_secret).unwrap();
//!     let balances = client.get_balances().await.unwrap();
//!     for balance in balances {
//!         info!("{} -> Balance: {}, Reserved: {}", balance.asset, balance.balance, balance.reserved);
//!     }
//! }
//! ```
//!
//! __LunoClientBuilder__
//!
//! ```rust
//! #[macro_use]
//! extern crate log;
//!
//! use luno_rs::{LunoClientBuilder, CurrencyPair};
//! use std::env;
//!
//! #[async_std::main]
//! async fn main() {
//!     env_logger::builder().format_timestamp_millis().init();
//!
//!     let api_id = env::var("LUNO_API_ID").unwrap();
//!     let api_secret = env::var("LUNO_API_SECRET").unwrap();
//!
//!     let client = LunoClientBuilder::new(api_id, api_secret)
//!         .with_timeout(30000)
//!         .with_request_logger()
//!         .build()
//!         .unwrap();
//!     let ticker = client.get_ticker(CurrencyPair::XRPNGN).await.unwrap();
//!     info!("{:#?}", ticker);
//! }
//! ```
#![allow(missing_docs)]

#[macro_use]
extern crate log;

mod credential;
mod domain;
mod error;
mod http;
mod luno_client;
mod middleware;

pub use domain::{
    AccountBalance, CurrencyPair, ListBalancesResponse, ListOrdersResponse, ListTickersResponse,
    ListTradesResponse, Order, OrderBook, OrderBookEntry, OrderType, Ticker, Trade,
};
pub use error::{Error, LunoError};
pub use luno_client::{LunoClient, LunoClientBuilder};

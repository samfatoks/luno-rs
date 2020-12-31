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
//! __Using Client__
//!
//! ```no_run
//! use luno::Client;
//! use std::env;
//!
//! #[async_std::main]
//! async fn main() {
//!     let api_id = env::var("LUNO_API_ID").unwrap();
//!     let api_secret = env::var("LUNO_API_SECRET").unwrap();
//!
//!     let client = Client::new(api_id, api_secret).unwrap();
//!     let balances = client.list_balances().await.unwrap();
//!     for balance in balances {
//!         println!("{} -> Balance: {}, Reserved: {}", balance.asset, balance.balance, balance.reserved);
//!     }
//! }
//! ```
//!
//! __Using ClientBuilder__
//!
//! ```no_run
//! use luno::{ClientBuilder, CurrencyPair};
//! use std::env;
//!
//! #[async_std::main]
//! async fn main() {
//!     let api_id = env::var("LUNO_API_ID").unwrap();
//!     let api_secret = env::var("LUNO_API_SECRET").unwrap();
//!
//!     let client = ClientBuilder::new(api_id, api_secret)
//!         .with_timeout(30000)
//!         .with_request_logger()
//!         .build()
//!         .unwrap();
//!     let ticker = client.get_ticker(CurrencyPair::XRPNGN).await.unwrap();
//!     println!("{:#?}", ticker);
//! }
//! ```
#![allow(missing_docs)]

#[macro_use]
extern crate log;

mod client;
mod credential;
mod domain;
mod error;
mod http;
mod middleware;

pub use client::{Client, ClientBuilder};
pub use domain::{
    AccountBalance, CurrencyPair, Order, OrderBook, OrderBookEntry, OrderType, Ticker, Trade,
};
pub use error::{Error, LunoError};

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
//! luno-rs = "0.1"
//! ```
//!
//! ### Example usage
//!
//! __Using LunoClient__
//!
//! ```no_run
//! use luno_rs::LunoClient;
//! use std::env;
//!
//! #[async_std::main]
//! async fn main() {
//!     let key_id = env::var("LUNO_KEY_ID").unwrap();
//!     let key_secret = env::var("LUNO_KEY_SECRET").unwrap();
//!
//!     let client = LunoClient::new(key_id, key_secret);
//!     let balances = client.list_balances().await.unwrap();
//!     for balance in balances {
//!         println!("{} -> Balance: {}, Reserved: {}", balance.asset, balance.balance, balance.reserved);
//!     }
//! }
//! ```
//!
//! __Using LunoClientBuilder__
//!
//! ```no_run
//! use luno_rs::{LunoClientBuilder, CurrencyPair};
//! use std::env;
//!
//! #[async_std::main]
//! async fn main() {
//!     let key_id = env::var("LUNO_KEY_ID").unwrap();
//!     let key_secret = env::var("LUNO_KEY_SECRET").unwrap();
//!
//!     let client = LunoClientBuilder::new(key_id, key_secret)
//!         .with_timeout(30000)
//!         .with_request_logger()
//!         .build();
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

pub use client::{LunoClient, LunoClientBuilder};
pub use domain::{
    AccountBalance, CurrencyPair, Order, OrderBook, OrderBookEntry, OrderType, Ticker, Trade,
};
pub use error::{Error, LunoError};

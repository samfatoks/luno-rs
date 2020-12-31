use crate::{
    credential::Credential,
    domain::{
        AccountBalance, CurrencyPair, ListBalancesResponse, ListOrdersResponse,
        ListTickersResponse, ListTradesResponse, Order, OrderBook, Ticker, Trade,
    },
    error::Error,
    http::Http,
};
use async_std::task;
use chrono::{DateTime, Utc};
use futures_util::{stream, StreamExt};
use std::sync::Arc;
use std::time::Duration;
pub struct ClientBuilder {
    credential: Credential,
    timeout: Duration,
    enable_logger_middleware: bool,
}

impl ClientBuilder {
    /// Create a new ClientBuilder
    pub fn new(api_id: String, api_secret: String) -> Self {
        let credential = Credential::new(api_id, api_secret);
        ClientBuilder {
            credential,
            timeout: Duration::from_millis(60000),
            enable_logger_middleware: false,
        }
    }

    /// Add timeout in milliseconds
    pub fn with_timeout(mut self, timeout_ms: u64) -> Self {
        self.timeout = Duration::from_millis(timeout_ms);
        self
    }

    /// Add request/response logger middleware
    pub fn with_request_logger(mut self) -> Self {
        self.enable_logger_middleware = true;
        self
    }

    /// Build ClientBuilder into a Client
    pub fn build(self) -> Result<Client, Error> {
        Client::new_with_features(self.credential, self.timeout, self.enable_logger_middleware)
    }
}

pub struct Client {
    http: Arc<Http>,
}

impl Client {
    /// Create a new Client
    pub fn new(api_id: String, api_secret: String) -> Result<Self, Error> {
        let http = Http::new(api_id, api_secret)?;
        let client = Client {
            http: Arc::new(http),
        };
        Ok(client)
    }

    fn new_with_features(
        credential: Credential,
        timeout: Duration,
        enable_logger_middleware: bool,
    ) -> Result<Self, Error> {
        let http = Http::new_with_features(credential, timeout, enable_logger_middleware)?;
        let client = Client {
            http: Arc::new(http),
        };
        Ok(client)
    }

    /// List the balances on all assets linked to Luno profile
    pub async fn list_balances(&self) -> Result<Vec<AccountBalance>, Error> {
        let response: ListBalancesResponse = self.http.process_request("/api/1/balance").await?;
        Ok(response.balances)
    }

    /// List all orders on Luno profile
    pub async fn list_orders(&self) -> Result<Vec<Order>, Error> {
        let response: ListOrdersResponse = self
            .http
            .process_request("/api/1/listorders?state=PENDING")
            .await?;
        Ok(response.orders)
    }

    /// Get ticker for currency pair
    pub async fn get_ticker(&self, currency_pair: CurrencyPair) -> Result<Ticker, Error> {
        let path = format!("/api/1/ticker?pair={}", currency_pair);
        let response: Ticker = self.http.process_request(path).await?;
        Ok(response)
    }

    /// List tickers for all currency pairs
    pub async fn list_tickers(&self) -> Result<Vec<Ticker>, Error> {
        let response: ListTickersResponse = self.http.process_request("/api/1/tickers").await?;
        Ok(response.tickers)
    }

    /// List tickers for specific currency pairs
    pub async fn list_tickers_for_currency_pairs(
        &self,
        currency_pairs: Vec<CurrencyPair>,
    ) -> Result<Vec<Ticker>, Error> {
        let responses = stream::iter(currency_pairs)
            .map(|cp| {
                let http = self.http.clone();
                let path = format!("/api/1/ticker?pair={}", cp);
                task::spawn(async move { http.process_request(path).await.unwrap() })
            })
            .buffer_unordered(10);
        Ok(responses.collect().await)
    }

    /// Get order book
    pub async fn get_order_book(&self, currency_pair: CurrencyPair) -> Result<OrderBook, Error> {
        let path = format!("/api/1/orderbook?pair={}", currency_pair);
        let response: OrderBook = self.http.process_request(path).await?;
        Ok(response)
    }

    /// Get top 100 bids and asks in order book
    pub async fn get_order_book_top_100(
        &self,
        currency_pair: CurrencyPair,
    ) -> Result<OrderBook, Error> {
        let path = format!("/api/1/orderbook_top?pair={}", currency_pair);
        let response: OrderBook = self.http.process_request(path).await?;
        Ok(response)
    }

    /// List the most recent Trades for the specified currency pair in the last 24 hours. At most 100 results are returned per call.
    ///
    /// # Example
    /// ```no_run
    /// use luno::{Client, CurrencyPair};
    /// use std::env;
    ///
    /// # #[async_std::main]
    /// # async fn main() {
    /// #    let api_id = env::var("LUNO_API_ID").unwrap();
    /// #    let api_secret = env::var("LUNO_API_SECRET").unwrap();
    /// let client = Client::new(api_id, api_secret).unwrap();
    /// let trades = client
    ///     .list_trades(CurrencyPair::XBTNGN)
    ///     .await
    ///     .unwrap();
    /// for (i, trade) in trades.iter().enumerate() {
    ///     println!(
    ///         "{} :: {} -> Type: {}, Price: {} Volume: {}",
    ///         i, trade.timestamp, trade.order_type, trade.price, trade.volume
    ///     );
    /// }
    /// # }
    /// ```
    pub async fn list_trades(&self, currency_pair: CurrencyPair) -> Result<Vec<Trade>, Error> {
        let path = format!("/api/1/trades?pair={}", currency_pair);
        let response: ListTradesResponse = self.http.process_request(path).await?;
        Ok(response.trades)
    }

    /// List trades since duration ago
    ///
    /// # Example
    /// ```no_run
    /// use luno::{Client, CurrencyPair};
    /// use std::{env, time::Duration};
    ///
    /// # #[async_std::main]
    /// # async fn main() {
    /// #    let api_id = env::var("LUNO_API_ID").unwrap();
    /// #    let api_secret = env::var("LUNO_API_SECRET").unwrap();
    /// let client = Client::new(api_id, api_secret).unwrap();
    /// let trades = client
    ///     .list_trades_since(CurrencyPair::XBTNGN, Duration::from_secs(20))
    ///     .await
    ///     .unwrap();
    /// for (i, trade) in trades.iter().enumerate() {
    ///     println!(
    ///         "{} :: {} -> Type: {}, Price: {} Volume: {}",
    ///         i, trade.timestamp, trade.order_type, trade.price, trade.volume
    ///     );
    /// }
    /// # }
    /// ```
    pub async fn list_trades_since(
        &self,
        currency_pair: CurrencyPair,
        duration: Duration,
    ) -> Result<Vec<Trade>, Error> {
        let now: DateTime<Utc> = Utc::now();
        let since = now.timestamp_millis() - duration.as_millis() as i64;
        let path = format!("/api/1/trades?pair={}&since={}", currency_pair, since);
        let response: ListTradesResponse = self.http.process_request(path).await?;
        Ok(response.trades)
    }
}
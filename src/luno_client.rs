use crate::{
    credential::Credential, http::Http, AccountBalance, CurrencyPair, Error, GetBalancesResponse,
    GetTickersResponse, ListOrdersResponse, Order, OrderBook, Ticker, Trade, Trades,
};
use async_std::task;
use chrono::{DateTime, Utc};
use futures_util::{stream, StreamExt}; //future::join_all,
use std::sync::Arc;
use std::time::Duration;
pub struct LunoClientBuilder {
    credential: Credential,
    timeout: Duration,
    enable_debug_middleware: bool,
}

impl LunoClientBuilder {
    pub fn new(api_id: String, api_secret: String) -> Self {
        let credential = Credential::new(api_id, api_secret);
        LunoClientBuilder {
            credential,
            timeout: Duration::from_millis(60000),
            enable_debug_middleware: false,
        }
    }

    pub fn with_timeout(mut self, timeout_ms: u64) -> Self {
        self.timeout = Duration::from_millis(timeout_ms);
        self
    }

    pub fn with_debug_middleware(mut self) -> Self {
        self.enable_debug_middleware = true;
        self
    }

    pub fn build(self) -> Result<LunoClient, Error> {
        LunoClient::new_with_features(self.credential, self.timeout, self.enable_debug_middleware)
    }
}

pub struct LunoClient {
    http: Arc<Http>,
}

impl LunoClient {
    pub fn new(api_id: String, api_secret: String) -> Result<Self, Error> {
        let http = Http::new(api_id, api_secret)?;
        let client = LunoClient {
            http: Arc::new(http),
        };
        Ok(client)
    }

    fn new_with_features(
        credential: Credential,
        timeout: Duration,
        enable_debug_middleware: bool,
    ) -> Result<Self, Error> {
        let http = Http::new_with_features(credential, timeout, enable_debug_middleware)?;
        let client = LunoClient {
            http: Arc::new(http),
        };
        Ok(client)
    }

    pub async fn get_balances(&self) -> Result<Vec<AccountBalance>, Error> {
        let response: GetBalancesResponse = self.http.process_request("/api/1/balance").await?;
        Ok(response.balances)
    }

    pub async fn list_orders(&self) -> Result<Vec<Order>, Error> {
        let response: ListOrdersResponse = self
            .http
            .process_request("/api/1/listorders?state=PENDING")
            .await?;
        Ok(response.orders)
    }

    // pub async fn get_markets_info(&self) -> Result<Vec<MarketsInfo>, Error> {
    //     let response: GetMarketsInfoResponse = self.process_request("/api/exchange/1/markets").await?;
    //     Ok(response.markets)
    // }

    /// Get ticker for currency pair
    pub async fn get_ticker(&self, currency_pair: CurrencyPair) -> Result<Ticker, Error> {
        let path = format!("/api/1/ticker?pair={}", currency_pair);
        let response: Ticker = self.http.process_request(path).await?;
        Ok(response)
    }

    /// Get tickers for all currency pairs
    pub async fn get_tickers(&self) -> Result<Vec<Ticker>, Error> {
        let response: GetTickersResponse = self.http.process_request("/api/1/tickers").await?;
        Ok(response.tickers)
    }

    /// Get tickers for specific currency pairs
    pub async fn get_tickers_for_currency_pairs(
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

    /// Returns a list of the most recent Trades for the specified currency pair in the last 24 hours. At most 100 results are returned per call.
    pub async fn list_trades(&self, currency_pair: CurrencyPair) -> Result<Vec<Trade>, Error> {
        let path = format!("/api/1/trades?pair={}", currency_pair);
        let response: Trades = self.http.process_request(path).await?;
        Ok(response.trades)
    }

    /// List trades since duration ago
    pub async fn list_trades_since(
        &self,
        currency_pair: CurrencyPair,
        duration: Duration,
    ) -> Result<Vec<Trade>, Error> {
        let now: DateTime<Utc> = Utc::now();
        let since = now.timestamp_millis() - duration.as_millis() as i64;
        let path = format!("/api/1/trades?pair={}&since={}", currency_pair, since);
        let response: Trades = self.http.process_request(path).await?;
        Ok(response.trades)
    }

    // pub async fn get_tickers2(
    //     &self,
    //     currency_pairs: Vec<CurrencyPair>,
    // ) -> Result<Vec<GetTickerResponse>, Error> {
    //     let responses = join_all(currency_pairs.into_iter().map(|cp| async move {
    //         self.http
    //             .process_request(format!("/api/1/ticker?pair={}", cp))
    //             .await
    //     }))
    //     .await;
    //     Ok(responses.into_iter().filter_map(Result::ok).collect())
    // }
}


use std::time::Duration;

use crate::{credential::Credential, error::Error, http::Http};
use crate::domain::{GetBalanceResponse, AccountBalance, ListOrdersResponse, Order, GetTickerResponse};


pub struct LunoClientBuilder {
    credential: Credential,
    timeout: Duration,
    enable_debug_middleware: bool
}

impl LunoClientBuilder {
    pub fn new(key_id: String, secret: String) -> Self {
        let credential = Credential::new(key_id, secret);
        LunoClientBuilder {
            credential,
            timeout: Duration::from_millis(60000),
            enable_debug_middleware: false
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
    pub http: Http,
}

impl LunoClient {
    pub fn new(key_id: String, secret: String) -> Result<Self, Error> {
        let http = Http::new(key_id, secret)?;
        let client = LunoClient {
            http
        };
        Ok(client)
    }

    fn new_with_features(credential: Credential, timeout: Duration, enable_debug_middleware: bool) -> Result<Self, Error> {
        let http = Http::new_with_features(credential, timeout, enable_debug_middleware)?;
        let client = LunoClient {
            http
        };
        Ok(client)
    }

    pub async fn get_balances(&self) -> Result<Vec<AccountBalance>, Error> {
        let response: GetBalanceResponse = self.http.process_request("/api/1/balance").await?;
        Ok(response.balances)
    }

    pub async fn list_orders(&self) -> Result<Vec<Order>, Error> {
        let response: ListOrdersResponse = self.http.process_request("/api/1/listorders?state=PENDING").await?;
        Ok(response.orders)
    }

    // pub async fn get_markets_info(&self) -> Result<Vec<MarketsInfo>, Error> {
    //     let response: GetMarketsInfoResponse = self.process_request("/api/exchange/1/markets").await?;
    //     Ok(response.markets)
    // }

    pub async fn get_ticker(&self) -> Result<GetTickerResponse, Error> {
        let response: GetTickerResponse = self.http.process_request("/api/1/ticker?pair=XBTNGN").await?;
        Ok(response)
    }
    

}
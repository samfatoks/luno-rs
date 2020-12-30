use chrono::serde::ts_milliseconds;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize)]
pub struct Ticker {
    pub ask: String,
    pub bid: String,
    pub last_trade: String,
    pub pair: String,
    pub rolling_24_hour_volume: String,
    pub status: String,
    #[serde(with = "ts_milliseconds")]
    pub timestamp: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct GetTickersResponse {
    pub tickers: Vec<Ticker>,
}

#[cfg(test)]
mod tests {
    use super::Ticker;

    #[test]
    fn parses_json_correctly() {
        //let client = LunoClient::new("demo".to_string(), "password".to_string()).unwrap();
        let data = r#"
        {
            "pair": "XBTNGN",
            "timestamp": 1609241817077,
            "bid": "12579999.00000000",
            "ask": "12580000.00000000",
            "last_trade": "12560699.00000000",
            "rolling_24_hour_volume": "205.51107600",
            "status": "ACTIVE"
        }
        "#;
        let response: Ticker = serde_json::from_str(data).unwrap();
        assert_eq!(
            response.timestamp.to_string(),
            "2020-12-29 11:36:57.077 UTC"
        );
    }
}

use serde::Deserialize;

#[derive(Deserialize)]
pub struct MarketsInfo {
    pub base_currency: String,
    pub counter_currency: String,
    pub fee_scale: String,
}

#[derive(Deserialize)]
pub struct GetMarketsInfoResponse {
    pub markets: Vec<MarketsInfo>
}
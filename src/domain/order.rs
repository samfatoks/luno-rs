use serde::Deserialize;

#[derive(Deserialize)]
pub struct Order {
    pub order_id: String,
    pub creation_timestamp: i64,
    pub expiration_timestamp: i64,
    pub completed_timestamp: i64,
    #[serde(rename = "type")]
    pub order_type: String,
    pub state: String,
    pub limit_price: String,
    pub limit_volume: String,
    pub base: String,
    pub counter: String,
    pub fee_base: String,
    pub fee_counter: String,
    pub pair: String
}

#[derive(Deserialize)]
pub struct ListOrdersResponse {
    pub orders: Vec<Order>
}
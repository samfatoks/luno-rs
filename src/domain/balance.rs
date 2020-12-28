use serde::Deserialize;

#[derive(Deserialize)]
pub struct AccountBalance {
    pub account_id: String,
    pub asset: String,
    pub balance: String,
    pub reserved: String,
    pub unconfirmed: String,
}

#[derive(Deserialize)]
pub struct GetBalanceResponse {
    #[serde(rename = "balance")]
    pub balances: Vec<AccountBalance>
}
use luno_rs::{CurrencyPair, LunoClient};

#[async_std::main]
async fn main() {
    let client = LunoClient::new("LUNO_API_ID", "LUNO_API_SECRET");
    let trades = client.list_trades(CurrencyPair::XBTNGN).await.unwrap();
    println!("{:?}", trades);
}

use luno_rs::LunoClient;

#[async_std::main]
async fn main() {
    let client = LunoClient::new("LUNO_API_ID", "LUNO_API_SECRET");
    let orders = client.list_orders().await.unwrap();
    println!("{:?}", orders);
}

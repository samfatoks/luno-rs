use std::time;
use surf::middleware::{Middleware, Next};
use surf::{Client, Request, Response, Result};

#[derive(Debug)]
pub struct Logger;

#[surf::utils::async_trait]
impl Middleware for Logger {
    async fn handle(&self, req: Request, client: Client, next: Next<'_>) -> Result<Response> {
        let url = req.url().clone();
        info!("sending request to {}", url);
        let now = time::Instant::now();
        let res = next.run(req, client).await?;
        info!("request [{}] took ({:?})", url, now.elapsed());
        Ok(res)
    }
}

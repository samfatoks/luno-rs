use surf::middleware::{Next, Middleware};
use surf::{Client, Request, Response, Result};
use std::time;

#[derive(Debug)]
pub struct Logger;

#[surf::utils::async_trait]
impl Middleware for Logger {
    async fn handle(
        &self,
        req: Request,
        client: Client,
        next: Next<'_>,
    ) -> Result<Response> {
        info!("sending request to {}", req.url());
        let now = time::Instant::now();
        let res = next.run(req, client).await?;
        info!("request completed ({:?})", now.elapsed());
        Ok(res)
    }
}
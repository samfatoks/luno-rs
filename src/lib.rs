#[macro_use]
extern crate log;

mod error;
mod domain;
mod middleware;
mod luno_client;
mod credential;
mod http;

pub use luno_client::{LunoClient, LunoClientBuilder};
pub use error::{Error, LunoError};
pub use domain::{GetBalanceResponse, AccountBalance, ListOrdersResponse, Order, GetTickerResponse};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

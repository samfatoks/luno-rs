#[macro_use]
extern crate log;

mod credential;
mod domain;
mod error;
mod http;
mod luno_client;
mod middleware;

pub use domain::{
    AccountBalance, GetBalancesResponse, GetTickerResponse, ListOrdersResponse, Order,
};
pub use error::{Error, LunoError};
pub use luno_client::{LunoClient, LunoClientBuilder};

#[cfg(test)]
mod tests {
    //use crate::LunoClient;

    #[test]
    fn it_works() {
        //let client = LunoClient::new("demo".to_string(), "password".to_string()).unwrap();
        assert_eq!(2 + 2, 4);
    }
}

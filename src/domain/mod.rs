mod balance;
mod currency;
mod market;
mod order;
mod ticker;
mod trade;
pub use balance::{AccountBalance, ListBalancesResponse};
pub use currency::CurrencyPair;
pub use market::{GetMarketsInfoResponse, MarketsInfo};
pub use order::{ListOrdersResponse, Order, OrderBook, OrderBookEntry, OrderType};
use serde::{self, de};
use std::fmt;
pub use ticker::{ListTickersResponse, Ticker};
pub use trade::{ListTradesResponse, Trade};

struct OrderTypeVisitor;
pub fn convert_is_buy_to_order_type<'de, D>(d: D) -> Result<OrderType, D::Error>
where
    D: de::Deserializer<'de>,
{
    d.deserialize_bool(OrderTypeVisitor)
}

impl<'de> de::Visitor<'de> for OrderTypeVisitor {
    type Value = OrderType;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "is_buy should be a boolean type")
    }
    fn visit_bool<E>(self, is_buy: bool) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(match is_buy {
            true => OrderType::BID,
            false => OrderType::ASK,
        })
    }
}

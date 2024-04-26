use rust_decimal::prelude::*;
use rust_decimal_macros::dec;

use super::orderbook::{Order, OrderBook};
use std::collections::HashMap;

// BTC/USD
// BTC -> base
// USD -> quote (amount you sell in return for the base)
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct TradingPair {
    base: String,
    quote: String,
}

impl TradingPair {
    pub fn new(base: String, quote: String) -> TradingPair {
        TradingPair { base, quote }
    }

    pub fn to_string(&self) -> String {
        format!("{}/{}", self.base, self.quote)
    }
}

pub struct MatchingEngine {
    orderbooks: HashMap<TradingPair, OrderBook>,
}

impl MatchingEngine {
    pub fn new() -> MatchingEngine {
        MatchingEngine {
            orderbooks: HashMap::new(),
        }
    }

    pub fn add_new_market(&mut self, pair: TradingPair) {
        self.orderbooks.insert(pair.clone(), OrderBook::new());
        println!("Added new market: {:?}", pair.to_string());
    }

    pub fn place_limit_order(
        &mut self,
        pair: TradingPair,
        price: Decimal,
        order: Order,
    ) -> Result<(), String> {
        let orderbook = self.orderbooks.get_mut(&pair);
        match orderbook {
            Some(orderbook) => {
                orderbook.add_order(price, order);
                println!("Placed limit order at price level: {:?}", price);
                Ok(())
            }
            None => Err(format!(
                "Orderbook for Market {:?} does not exist",
                pair.to_string()
            )),
        }
    }
}

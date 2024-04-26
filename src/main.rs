mod matching_engine;
use matching_engine::engine::{MatchingEngine, TradingPair};
use matching_engine::orderbook::{BidOrAsk, Order, OrderBook};
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;

fn main() {
    let buy_order = Order::new(BidOrAsk::Bid, 5.5);
    let buy_order2 = Order::new(BidOrAsk::Bid, 2.5);

    let sell_order = Order::new(BidOrAsk::Ask, 3.5);

    let mut order_book = OrderBook::new();
    order_book.add_order(dec!(4.4), buy_order);
    order_book.add_order(dec!(4.4), buy_order2);

    let sell_order = Order::new(BidOrAsk::Ask, 3.5);
    order_book.add_order(dec!(20.0), sell_order);

    // println!("{:?}", order_book);

    let mut engine = MatchingEngine::new();
    let pair = TradingPair::new("BTC".to_string(), "USD".to_string());
    engine.add_new_market(pair.clone());

    let buy_order3 = Order::new(BidOrAsk::Bid, 6.5);
    engine
        .place_limit_order(pair.clone(), dec!(10.000), buy_order3)
        .unwrap();
}

//! # HFT Trading Bot - Demo

use hft_trading_bot::solution::{
    can_place_order, make_market_making_quotes, RiskState, StrategyType, TradingConfig,
};

fn main() {
    println!("=== HFT Trading Bot Demo ===");

    let cfg = TradingConfig {
        symbol: "BTC/USD".to_string(),
        strategy: StrategyType::MarketMaking,
        max_position: 10.0,
        max_loss: 10_000.0,
        max_order_size: 1.0,
    };

    cfg.validate().expect("valid config");

    let risk = RiskState {
        position: 2.0,
        unrealized_pnl: 250.0,
    };

    let (bid, ask) = make_market_making_quotes(50_000.0, 2.5, 0.5);
    println!("Bid quote:  side={:?} price={} size={}", bid.side, bid.price, bid.size);
    println!("Ask quote:  side={:?} price={} size={}", ask.side, ask.price, ask.size);

    println!("Can place bid: {}", can_place_order(&cfg, risk, bid));
    println!("Can place ask: {}", can_place_order(&cfg, risk, ask));
}

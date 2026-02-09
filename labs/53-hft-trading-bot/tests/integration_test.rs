use hft_trading_bot::solution::{
    can_place_order, make_market_making_quotes, CandidateOrder, RiskState, Side, StrategyType,
    TradingConfig,
};

fn sample_config() -> TradingConfig {
    TradingConfig {
        symbol: "BTC/USD".to_string(),
        strategy: StrategyType::MarketMaking,
        max_position: 10.0,
        max_loss: 1_000.0,
        max_order_size: 2.0,
    }
}

#[test]
fn test_config_validation() {
    let cfg = sample_config();
    assert!(cfg.validate().is_ok());
}

#[test]
fn test_config_invalid_symbol() {
    let mut cfg = sample_config();
    cfg.symbol = " ".to_string();
    assert!(cfg.validate().is_err());
}

#[test]
fn test_can_place_within_limits() {
    let cfg = sample_config();
    let risk = RiskState {
        position: 0.0,
        unrealized_pnl: 100.0,
    };
    let order = CandidateOrder {
        side: Side::Buy,
        price: 50_000.0,
        size: 1.0,
    };
    assert!(can_place_order(&cfg, risk, order));
}

#[test]
fn test_reject_size_over_limit() {
    let cfg = sample_config();
    let risk = RiskState {
        position: 0.0,
        unrealized_pnl: 0.0,
    };
    let order = CandidateOrder {
        side: Side::Buy,
        price: 50_000.0,
        size: 3.0,
    };
    assert!(!can_place_order(&cfg, risk, order));
}

#[test]
fn test_reject_position_limit_breach() {
    let cfg = sample_config();
    let risk = RiskState {
        position: 9.5,
        unrealized_pnl: 0.0,
    };
    let order = CandidateOrder {
        side: Side::Buy,
        price: 50_000.0,
        size: 1.0,
    };
    assert!(!can_place_order(&cfg, risk, order));
}

#[test]
fn test_reject_loss_limit_breach() {
    let cfg = sample_config();
    let risk = RiskState {
        position: 0.0,
        unrealized_pnl: -1_500.0,
    };
    let order = CandidateOrder {
        side: Side::Sell,
        price: 50_000.0,
        size: 1.0,
    };
    assert!(!can_place_order(&cfg, risk, order));
}

#[test]
fn test_market_making_quotes_are_symmetric() {
    let (bid, ask) = make_market_making_quotes(100.0, 0.5, 1.25);
    assert_eq!(bid.side, Side::Buy);
    assert_eq!(ask.side, Side::Sell);
    assert!((bid.price - 99.5).abs() < 1e-9);
    assert!((ask.price - 100.5).abs() < 1e-9);
    assert!((bid.size - 1.25).abs() < 1e-9);
    assert!((ask.size - 1.25).abs() < 1e-9);
}

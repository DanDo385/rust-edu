//! # Lab 53: HFT Trading Bot - Your Implementation

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StrategyType {
    MarketMaking,
    Momentum,
    Arbitrage,
}

#[derive(Debug, Clone)]
pub struct TradingConfig {
    pub symbol: String,
    pub strategy: StrategyType,
    pub max_position: f64,
    pub max_loss: f64,
    pub max_order_size: f64,
}

impl TradingConfig {
    pub fn validate(&self) -> Result<(), String> {
        todo!("Validate config invariants")
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RiskState {
    pub position: f64,
    pub unrealized_pnl: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Copy)]
pub struct CandidateOrder {
    pub side: Side,
    pub price: f64,
    pub size: f64,
}

pub fn can_place_order(_cfg: &TradingConfig, _risk: RiskState, _order: CandidateOrder) -> bool {
    todo!("Enforce position, size, and drawdown limits")
}

pub fn make_market_making_quotes(_mid_price: f64, _half_spread: f64, _size: f64) -> (CandidateOrder, CandidateOrder) {
    todo!("Generate symmetric bid/ask quotes")
}

#[doc(hidden)]
pub mod solution;

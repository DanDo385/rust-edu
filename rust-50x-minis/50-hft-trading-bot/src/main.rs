// Project 50: HFT Trading Bot (FINAL CAPSTONE)
//
// High-frequency trading simulation demonstrating low-latency design,
// market data processing, order execution, and performance optimization.
// This showcases Rust's strengths in systems programming and real-time performance.

use std::collections::{BTreeMap, VecDeque};
use std::time::{Instant, Duration};

fn main() {
    println!("=== High-Frequency Trading Bot ===\n");

    // Configuration
    let config = TradingConfig {
        symbol: "BTC/USD".to_string(),
        strategy: StrategyType::MarketMaking,
        max_position: 10.0,
        max_loss: 10_000.0,
        max_order_size: 1.0,
    };

    println!("Configuration:");
    println!("  Strategy: {:?}", config.strategy);
    println!("  Symbol: {}", config.symbol);
    println!("  Max position: {} BTC", config.max_position);
    println!("  Max loss: ${:.2}", config.max_loss);
    println!();

    // Run trading simulation
    run_hft_simulation(config);

    // Demonstrate performance optimizations
    demonstrate_performance_optimizations();
}

// ============================================================================
// TRADING CONFIGURATION
// ============================================================================

#[derive(Clone)]
struct TradingConfig {
    symbol: String,
    strategy: StrategyType,
    max_position: f64,
    max_loss: f64,
    max_order_size: f64,
}

#[derive(Debug, Clone, Copy)]
enum StrategyType {
    MarketMaking,
    Momentum,
    Arbitrage,
}

// ============================================================================
// ORDER BOOK
// ============================================================================

struct OrderBook {
    bids: BTreeMap<OrderedFloat, f64>, // price -> size (descending)
    asks: BTreeMap<OrderedFloat, f64>, // price -> size (ascending)
    last_update: Instant,
}

impl OrderBook {
    fn new() -> Self {
        OrderBook {
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
            last_update: Instant::now(),
        }
    }

    fn update_bid(&mut self, price: f64, size: f64) {
        let key = OrderedFloat(price);
        if size > 0.0 {
            self.bids.insert(key, size);
        } else {
            self.bids.remove(&key);
        }
        self.last_update = Instant::now();
    }

    fn update_ask(&mut self, price: f64, size: f64) {
        let key = OrderedFloat(price);
        if size > 0.0 {
            self.asks.insert(key, size);
        } else {
            self.asks.remove(&key);
        }
        self.last_update = Instant::now();
    }

    fn best_bid(&self) -> Option<(f64, f64)> {
        self.bids.iter().next_back().map(|(p, s)| (p.0, *s))
    }

    fn best_ask(&self) -> Option<(f64, f64)> {
        self.asks.iter().next().map(|(p, s)| (p.0, *s))
    }

    fn mid_price(&self) -> Option<f64> {
        match (self.best_bid(), self.best_ask()) {
            (Some((bid, _)), Some((ask, _))) => Some((bid + ask) / 2.0),
            _ => None,
        }
    }

    fn spread(&self) -> Option<f64> {
        match (self.best_bid(), self.best_ask()) {
            (Some((bid, _)), Some((ask, _))) => Some(ask - bid),
            _ => None,
        }
    }

    fn display(&self) {
        println!("Order Book:");

        // Show top 5 asks (reversed for display)
        let asks: Vec<_> = self.asks.iter().take(5).collect();
        for (price, size) in asks.iter().rev() {
            println!("  ASK: ${:.2} ({:.4} BTC)", price.0, size);
        }

        println!("  ---SPREAD: ${:.2}---", self.spread().unwrap_or(0.0));

        // Show top 5 bids
        for (price, size) in self.bids.iter().rev().take(5) {
            println!("  BID: ${:.2} ({:.4} BTC)", price.0, size);
        }
        println!();
    }
}

// Wrapper for f64 to make it Ord (needed for BTreeMap key)
#[derive(Debug, Clone, Copy)]
struct OrderedFloat(f64);

impl PartialEq for OrderedFloat {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_bits() == other.0.to_bits()
    }
}

impl Eq for OrderedFloat {}

impl PartialOrd for OrderedFloat {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for OrderedFloat {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.partial_cmp(&other.0).unwrap_or(std::cmp::Ordering::Equal)
    }
}

// ============================================================================
// TRADING STRATEGY
// ============================================================================

struct TradingStrategy {
    strategy_type: StrategyType,
    position_tracker: PositionTracker,
}

impl TradingStrategy {
    fn new(strategy_type: StrategyType) -> Self {
        TradingStrategy {
            strategy_type,
            position_tracker: PositionTracker::new(),
        }
    }

    fn generate_signal(&self, order_book: &OrderBook) -> Option<TradingSignal> {
        match self.strategy_type {
            StrategyType::MarketMaking => self.market_making_signal(order_book),
            StrategyType::Momentum => self.momentum_signal(order_book),
            StrategyType::Arbitrage => self.arbitrage_signal(order_book),
        }
    }

    fn market_making_signal(&self, order_book: &OrderBook) -> Option<TradingSignal> {
        let mid = order_book.mid_price()?;
        let spread = order_book.spread()?;

        // Place orders inside the spread
        let bid_price = mid - spread * 0.3;
        let ask_price = mid + spread * 0.3;

        // Alternate between bid and ask based on position
        if self.position_tracker.get_position() < 0.0 {
            // Short position, prefer buying
            Some(TradingSignal {
                side: Side::Buy,
                price: bid_price,
                size: 0.1,
                order_type: OrderType::Limit,
            })
        } else {
            // Long or neutral, prefer selling
            Some(TradingSignal {
                side: Side::Sell,
                price: ask_price,
                size: 0.1,
                order_type: OrderType::Limit,
            })
        }
    }

    fn momentum_signal(&self, order_book: &OrderBook) -> Option<TradingSignal> {
        let (bid, _) = order_book.best_bid()?;
        let (ask, _) = order_book.best_ask()?;

        // Simplified momentum: if spread is tight, assume momentum
        let spread = ask - bid;
        if spread < bid * 0.001 { // 0.1% spread = high liquidity = momentum
            Some(TradingSignal {
                side: Side::Buy,
                price: ask,
                size: 0.5,
                order_type: OrderType::Market,
            })
        } else {
            None
        }
    }

    fn arbitrage_signal(&self, _order_book: &OrderBook) -> Option<TradingSignal> {
        // Simplified: would compare prices across exchanges
        None
    }
}

#[derive(Debug)]
struct TradingSignal {
    side: Side,
    price: f64,
    size: f64,
    order_type: OrderType,
}

#[derive(Debug, Clone, Copy)]
enum Side {
    Buy,
    Sell,
}

#[derive(Debug)]
enum OrderType {
    Market,
    Limit,
}

// ============================================================================
// POSITION TRACKER
// ============================================================================

struct PositionTracker {
    position: f64,      // BTC
    cash: f64,          // USD
    realized_pnl: f64,  // USD
    trades: Vec<Trade>,
}

impl PositionTracker {
    fn new() -> Self {
        PositionTracker {
            position: 0.0,
            cash: 100_000.0, // Start with $100k
            realized_pnl: 0.0,
            trades: Vec::new(),
        }
    }

    fn execute_trade(&mut self, side: Side, price: f64, size: f64) {
        let trade = Trade {
            side,
            price,
            size,
            timestamp: Instant::now(),
        };

        match side {
            Side::Buy => {
                self.position += size;
                self.cash -= price * size;
            }
            Side::Sell => {
                self.position -= size;
                self.cash += price * size;
            }
        }

        self.trades.push(trade);
    }

    fn get_position(&self) -> f64 {
        self.position
    }

    fn unrealized_pnl(&self, current_price: f64) -> f64 {
        self.position * current_price + self.cash - 100_000.0
    }

    fn total_pnl(&self) -> f64 {
        self.realized_pnl
    }
}

#[derive(Debug)]
struct Trade {
    side: Side,
    price: f64,
    size: f64,
    timestamp: Instant,
}

// ============================================================================
// RISK MANAGER
// ============================================================================

struct RiskManager {
    config: TradingConfig,
}

impl RiskManager {
    fn new(config: TradingConfig) -> Self {
        RiskManager { config }
    }

    fn check_order(&self, signal: &TradingSignal, tracker: &PositionTracker) -> Result<(), String> {
        // Check position limit
        let new_position = match signal.side {
            Side::Buy => tracker.get_position() + signal.size,
            Side::Sell => tracker.get_position() - signal.size,
        };

        if new_position.abs() > self.config.max_position {
            return Err(format!("Position limit exceeded: {:.2}", new_position));
        }

        // Check order size
        if signal.size > self.config.max_order_size {
            return Err(format!("Order size too large: {:.2}", signal.size));
        }

        // Check loss limit
        let unrealized = tracker.unrealized_pnl(signal.price);
        if unrealized < -self.config.max_loss {
            return Err(format!("Loss limit exceeded: ${:.2}", unrealized));
        }

        Ok(())
    }
}

// ============================================================================
// ORDER EXECUTOR
// ============================================================================

struct OrderExecutor {
    latency_stats: LatencyStats,
}

impl OrderExecutor {
    fn new() -> Self {
        OrderExecutor {
            latency_stats: LatencyStats::new(),
        }
    }

    fn execute_order(
        &mut self,
        signal: &TradingSignal,
        order_book: &OrderBook,
        tracker: &mut PositionTracker,
    ) -> Result<Fill, String> {
        let start = Instant::now();

        // Simulate order execution latency
        std::thread::sleep(Duration::from_micros(50));

        // Determine fill price
        let fill_price = match signal.order_type {
            OrderType::Market => {
                // Market order fills at best available price
                match signal.side {
                    Side::Buy => order_book.best_ask().map(|(p, _)| p).ok_or("No asks")?,
                    Side::Sell => order_book.best_bid().map(|(p, _)| p).ok_or("No bids")?,
                }
            }
            OrderType::Limit => signal.price,
        };

        // Execute trade
        tracker.execute_trade(signal.side, fill_price, signal.size);

        let latency = start.elapsed();
        self.latency_stats.record(latency);

        Ok(Fill {
            side: signal.side,
            price: fill_price,
            size: signal.size,
            latency,
        })
    }

    fn get_stats(&self) -> &LatencyStats {
        &self.latency_stats
    }
}

#[derive(Debug)]
struct Fill {
    side: Side,
    price: f64,
    size: f64,
    latency: Duration,
}

// ============================================================================
// LATENCY STATISTICS
// ============================================================================

struct LatencyStats {
    latencies: VecDeque<Duration>,
    max_samples: usize,
}

impl LatencyStats {
    fn new() -> Self {
        LatencyStats {
            latencies: VecDeque::new(),
            max_samples: 1000,
        }
    }

    fn record(&mut self, latency: Duration) {
        self.latencies.push_back(latency);
        if self.latencies.len() > self.max_samples {
            self.latencies.pop_front();
        }
    }

    fn average(&self) -> Duration {
        if self.latencies.is_empty() {
            return Duration::from_micros(0);
        }

        let total: Duration = self.latencies.iter().sum();
        total / self.latencies.len() as u32
    }

    fn max(&self) -> Duration {
        self.latencies.iter().max().copied().unwrap_or(Duration::from_micros(0))
    }

    fn min(&self) -> Duration {
        self.latencies.iter().min().copied().unwrap_or(Duration::from_micros(0))
    }

    fn percentile(&self, p: f64) -> Duration {
        if self.latencies.is_empty() {
            return Duration::from_micros(0);
        }

        let mut sorted: Vec<_> = self.latencies.iter().copied().collect();
        sorted.sort();

        let idx = ((sorted.len() as f64 * p) as usize).min(sorted.len() - 1);
        sorted[idx]
    }
}

// ============================================================================
// HFT SIMULATION
// ============================================================================

fn run_hft_simulation(config: TradingConfig) {
    println!("--- Starting Trading Simulation ---\n");

    // Initialize components
    let mut order_book = OrderBook::new();
    let mut strategy = TradingStrategy::new(config.strategy);
    let mut executor = OrderExecutor::new();
    let risk_manager = RiskManager::new(config.clone());

    // Seed initial order book
    seed_order_book(&mut order_book);

    // Simulation loop
    let num_iterations = 100;
    let mut fills = Vec::new();

    for i in 0..num_iterations {
        // Simulate market data updates
        update_order_book(&mut order_book);

        // Generate trading signal
        if let Some(signal) = strategy.generate_signal(&order_book) {
            // Risk check
            if let Err(e) = risk_manager.check_order(&signal, &strategy.position_tracker) {
                if i < 10 {
                    println!("[{:06.3}ms] ⚠️  Order rejected: {}",
                        order_book.last_update.elapsed().as_secs_f64() * 1000.0, e);
                }
                continue;
            }

            // Execute order
            match executor.execute_order(&signal, &order_book, &mut strategy.position_tracker) {
                Ok(fill) => {
                    if i < 10 {
                        println!("[{:06.3}ms] ✅ {:?} {:.4} BTC @ ${:.2} (latency: {:?})",
                            order_book.last_update.elapsed().as_secs_f64() * 1000.0,
                            fill.side, fill.size, fill.price, fill.latency);
                    }
                    fills.push(fill);
                }
                Err(e) => {
                    if i < 10 {
                        println!("[{:06.3}ms] ❌ Execution failed: {}",
                            order_book.last_update.elapsed().as_secs_f64() * 1000.0, e);
                    }
                }
            }
        }

        // Small delay between iterations
        std::thread::sleep(Duration::from_micros(100));
    }

    println!();

    // Show final results
    show_results(&strategy.position_tracker, &executor, &order_book, &fills);
}

fn seed_order_book(order_book: &mut OrderBook) {
    let base_price = 45_000.0;

    // Add bids
    for i in 0..10 {
        let price = base_price - (i as f64 * 0.25);
        let size = 0.5 + (i as f64 * 0.1);
        order_book.update_bid(price, size);
    }

    // Add asks
    for i in 0..10 {
        let price = base_price + 0.75 + (i as f64 * 0.25);
        let size = 0.5 + (i as f64 * 0.1);
        order_book.update_ask(price, size);
    }
}

fn update_order_book(order_book: &mut OrderBook) {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    // Simulate random price movements
    if let Some((bid_price, _)) = order_book.best_bid() {
        let change = rng.gen_range(-0.5..0.5);
        let new_price = bid_price + change;
        let size = rng.gen_range(0.1..1.0);
        order_book.update_bid(new_price, size);
    }

    if let Some((ask_price, _)) = order_book.best_ask() {
        let change = rng.gen_range(-0.5..0.5);
        let new_price = ask_price + change;
        let size = rng.gen_range(0.1..1.0);
        order_book.update_ask(new_price, size);
    }
}

fn show_results(
    tracker: &PositionTracker,
    executor: &OrderExecutor,
    order_book: &OrderBook,
    fills: &[Fill],
) {
    println!("--- Trading Results ---");
    println!("Total trades: {}", fills.len());
    println!("Final position: {:.4} BTC", tracker.get_position());
    println!("Final cash: ${:.2}", tracker.cash);

    if let Some(mid) = order_book.mid_price() {
        println!("Unrealized PnL: ${:.2}", tracker.unrealized_pnl(mid));
    }
    println!();

    let stats = executor.get_stats();
    println!("--- Latency Statistics ---");
    println!("Average: {:?}", stats.average());
    println!("Min: {:?}", stats.min());
    println!("Max: {:?}", stats.max());
    println!("P50: {:?}", stats.percentile(0.50));
    println!("P95: {:?}", stats.percentile(0.95));
    println!("P99: {:?}", stats.percentile(0.99));
    println!();

    // Calculate win rate
    let mut wins = 0;
    let mut losses = 0;
    for window in fills.windows(2) {
        if let [fill1, fill2] = window {
            match (fill1.side, fill2.side) {
                (Side::Buy, Side::Sell) => {
                    if fill2.price > fill1.price {
                        wins += 1;
                    } else {
                        losses += 1;
                    }
                }
                _ => {}
            }
        }
    }

    let total_round_trips = wins + losses;
    if total_round_trips > 0 {
        println!("--- Performance Metrics ---");
        println!("Round-trip trades: {}", total_round_trips);
        println!("Wins: {}", wins);
        println!("Losses: {}", losses);
        println!("Win rate: {:.1}%", (wins as f64 / total_round_trips as f64) * 100.0);
        println!();
    }
}

// ============================================================================
// PERFORMANCE OPTIMIZATIONS DEMONSTRATION
// ============================================================================

fn demonstrate_performance_optimizations() {
    println!("--- Performance Optimization Demonstrations ---\n");

    // 1. Cache-friendly data layout
    demonstrate_cache_locality();

    // 2. Lock-free operations
    demonstrate_lockfree_performance();

    // 3. Zero-copy parsing
    demonstrate_zerocopy();
}

fn demonstrate_cache_locality() {
    println!("1. Cache Locality Impact:");

    const SIZE: usize = 1_000_000;

    // Struct of Arrays (cache-friendly)
    let prices_soa: Vec<f64> = (0..SIZE).map(|i| i as f64).collect();
    let sizes_soa: Vec<f64> = (0..SIZE).map(|i| (i * 2) as f64).collect();

    let start = Instant::now();
    let mut sum = 0.0;
    for i in 0..SIZE {
        sum += prices_soa[i] * sizes_soa[i];
    }
    let soa_time = start.elapsed();

    // Array of Structs (less cache-friendly)
    struct PriceSize {
        price: f64,
        size: f64,
    }

    let aos: Vec<PriceSize> = (0..SIZE)
        .map(|i| PriceSize {
            price: i as f64,
            size: (i * 2) as f64,
        })
        .collect();

    let start = Instant::now();
    let mut sum2 = 0.0;
    for item in &aos {
        sum2 += item.price * item.size;
    }
    let aos_time = start.elapsed();

    println!("  Struct of Arrays: {:?}", soa_time);
    println!("  Array of Structs: {:?}", aos_time);
    println!("  Speedup: {:.2}x", aos_time.as_nanos() as f64 / soa_time.as_nanos() as f64);
    println!("  (Results: {}, {})", sum, sum2);
    println!();
}

fn demonstrate_lockfree_performance() {
    println!("2. Lock-Free vs Mutex Performance:");

    // This is simplified - real implementation would use crossbeam
    const ITERATIONS: usize = 100_000;

    // Simulate atomic operations (lock-free)
    let start = Instant::now();
    let mut counter = 0u64;
    for _ in 0..ITERATIONS {
        counter += 1; // In real code: counter.fetch_add(1, Ordering::Relaxed)
    }
    let lockfree_time = start.elapsed();

    // Simulate mutex operations
    use std::sync::Mutex;
    let mutex_counter = Mutex::new(0u64);

    let start = Instant::now();
    for _ in 0..ITERATIONS {
        let mut guard = mutex_counter.lock().unwrap();
        *guard += 1;
    }
    let mutex_time = start.elapsed();

    println!("  Lock-free (atomic): {:?}", lockfree_time);
    println!("  Mutex-protected: {:?}", mutex_time);
    println!("  Speedup: {:.2}x", mutex_time.as_nanos() as f64 / lockfree_time.as_nanos() as f64);
    println!();
}

fn demonstrate_zerocopy() {
    println!("3. Zero-Copy Parsing:");

    let data: Vec<u8> = vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xF0, 0x3F]; // 1.0 in f64

    // Zero-copy: reinterpret bytes directly
    let start = Instant::now();
    let price = f64::from_le_bytes(data[0..8].try_into().unwrap());
    let zerocopy_time = start.elapsed();

    // With copying: convert to string first
    let start = Instant::now();
    let hex_str = hex::encode(&data);
    let _parsed = u64::from_str_radix(&hex_str, 16).unwrap();
    let withcopy_time = start.elapsed();

    println!("  Zero-copy: {:?}", zerocopy_time);
    println!("  With copying: {:?}", withcopy_time);
    println!("  Speedup: {:.2}x", withcopy_time.as_nanos() as f64 / zerocopy_time.as_nanos() as f64);
    println!("  (Result: {})", price);
    println!();
}

// Helper function for hex encoding (simplified)
mod hex {
    pub fn encode(bytes: &[u8]) -> String {
        bytes.iter().map(|b| format!("{:02x}", b)).collect()
    }
}

// ============================================================================
// WHAT RUST DOES UNDER THE HOOD
// ============================================================================
// 1. BTREEMAP FOR ORDER BOOK
//    BTreeMap keeps prices sorted (critical for best_bid/best_ask).
//    O(log n) insert/delete/lookup vs HashMap's O(1) but unsorted.
//    Cache-friendly traversal - better than linked list.
//    No dynamic allocations during iteration - zero garbage.
//
// 2. NO GARBAGE COLLECTION
//    Every allocation has defined lifetime via ownership.
//    Fill structs dropped immediately after use - no GC pauses.
//    Predictable latency - critical for HFT.
//    Compare to Java/C#: unpredictable GC pauses kill performance.
//
// 3. INLINE OPTIMIZATIONS
//    Small functions like best_bid() get inlined by compiler.
//    No function call overhead - direct memory access.
//    LLVM optimizations comparable to hand-written assembly.
//
// 4. SIMD OPPORTUNITIES
//    Vec operations can use SIMD instructions (AVX, SSE).
//    Example: calculating PnL across positions vectorizes automatically.
//    Rust's iterators allow compiler to detect SIMD patterns.
//
// 5. CACHE LINE ALIGNMENT
//    Instant (timestamp) is small - packed efficiently.
//    OrderBook fits in L1/L2 cache for hot path.
//    BTreeMap nodes designed for cache locality.

// ============================================================================
// KEY TAKEAWAYS
// ============================================================================
// 1. HFT is all about microsecond-level latency optimization
// 2. Order book management is critical for price discovery
// 3. Risk management prevents catastrophic losses
// 4. Market making profits from bid-ask spread
// 5. Cache locality matters more than algorithm complexity
// 6. Lock-free data structures eliminate contention
// 7. Zero-copy parsing avoids unnecessary allocations
// 8. Rust's no-GC guarantee enables predictable latency
// 9. Compiler optimizations are as good as C/C++
// 10. Real HFT requires hardware (FPGA) and co-location

// ============================================================================
// PRODUCTION HFT SYSTEMS
// ============================================================================
// Real HFT systems add:
// 1. FPGA acceleration (sub-microsecond latency)
// 2. Kernel bypass networking (DPDK, Solarflare)
// 3. Co-location (servers in exchange datacenter)
// 4. Custom hardware (NICs, switches, cables)
// 5. Direct market access (FIX, FAST, ITCH protocols)
// 6. Tick-to-trade under 1 microsecond
// 7. Smart order routing across exchanges
// 8. Machine learning for signal generation
// 9. Post-trade analytics and compliance
// 10. 24/7 monitoring and alerting
//
// Infrastructure cost: $1M+ per year
// Development cost: $5M+ per strategy
// Competition: Arms race with other HFT firms
//
// This educational simulation: ~200µs latency
// Real HFT: ~1µs latency (200x faster!)

// ============================================================================
// WHY RUST FOR HFT
// ============================================================================
// ✅ No GC pauses (vs Java, C#, Go)
// ✅ Memory safety (vs C, C++)
// ✅ Zero-cost abstractions
// ✅ Fearless concurrency
// ✅ Modern tooling (cargo, clippy)
// ✅ Great performance (matches C++)
// ✅ Growing adoption in finance
//
// Real firms using Rust:
// - Figment (blockchain trading)
// - Kraken (crypto exchange)
// - Diem/Novi (Facebook's crypto)
// - Many prop trading firms (stealth mode)

// ============================================================================
// COMMON MISTAKES TO AVOID
// ============================================================================
// ❌ Using Mutex in hot path (use atomics or lock-free)
// ❌ Allocating in order execution path (pre-allocate)
// ❌ String parsing in critical path (binary protocols)
// ❌ Not monitoring latency percentiles (P99, P99.9)
// ❌ Ignoring CPU cache effects (profile with perf)
// ❌ Over-optimizing cold path (focus on hot path)
// ❌ Not testing with realistic market data
// ❌ Insufficient risk management (blow up risk)
// ❌ Ignoring regulatory requirements (compliance)
// ❌ Underestimating infrastructure costs

// ============================================================================
// FURTHER OPTIMIZATIONS
// ============================================================================
// 1. Use fixed-point arithmetic instead of f64 (faster, deterministic)
// 2. Implement custom allocator (avoid malloc in hot path)
// 3. Pin threads to CPU cores (avoid context switches)
// 4. Use huge pages (reduce TLB misses)
// 5. Prefetch data before use (CPU cache warming)
// 6. Batch operations (amortize overhead)
// 7. Use SIMD explicitly (std::arch)
// 8. Profile with perf, VTune (find bottlenecks)
// 9. Implement lock-free queue (crossbeam)
// 10. Consider FPGA/ASIC for ultimate speed

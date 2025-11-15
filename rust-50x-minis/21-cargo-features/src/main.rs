// Project 21: Cargo Features
//
// Demonstrates how to use Cargo feature flags for conditional compilation.
// Features allow you to optionally include/exclude code and dependencies,
// reducing binary size and compile time.

use colored::Colorize;

#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "logging")]
use log::{info, warn, debug};

fn main() {
    // Initialize logger if logging feature is enabled
    #[cfg(feature = "logging")]
    env_logger::init();

    println!("{}", "=== Cargo Features Demo ===".bright_blue().bold());
    println!();

    // Show which features are enabled
    print_feature_status();
    println!();

    // Demonstrate conditional compilation
    demonstrate_serialization();
    println!();

    // Show build configuration
    print_build_config();
    println!();

    // Performance demonstration
    demonstrate_performance();
}

// ============================================================================
// FEATURE STATUS
// ============================================================================

/// Prints which features are currently enabled
fn print_feature_status() {
    println!("{}", "Feature Configuration:".bright_yellow());

    #[cfg(feature = "json")]
    println!("  {} JSON support: {}", "✓".green(), "enabled".green());

    #[cfg(not(feature = "json"))]
    println!("  {} JSON support: {}", "✗".red(), "disabled".red());

    #[cfg(feature = "xml")]
    println!("  {} XML support: {}", "✓".green(), "enabled".green());

    #[cfg(not(feature = "xml"))]
    println!("  {} XML support: {}", "✗".red(), "disabled".red());

    #[cfg(feature = "logging")]
    println!("  {} Logging: {}", "✓".green(), "enabled".green());

    #[cfg(not(feature = "logging"))]
    println!("  {} Logging: {}", "✗".red(), "disabled".red());
}

// ============================================================================
// CONDITIONAL COMPILATION EXAMPLES
// ============================================================================

// This struct is only available when JSON feature is enabled
#[cfg(feature = "json")]
#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: u32,
    name: String,
    email: String,
    active: bool,
}

// Alternative version without JSON feature
#[cfg(not(feature = "json"))]
#[derive(Debug)]
struct User {
    id: u32,
    name: String,
    email: String,
    active: bool,
}

impl User {
    fn new(id: u32, name: &str, email: &str) -> Self {
        User {
            id,
            name: name.to_string(),
            email: email.to_string(),
            active: true,
        }
    }
}

/// Demonstrates different serialization formats based on enabled features
fn demonstrate_serialization() {
    println!("{}", "Serialization Demo:".bright_yellow());

    let user = User::new(1, "Alice", "alice@example.com");

    println!("  Created user: {:?}", user);

    // JSON serialization (only if feature enabled)
    #[cfg(feature = "json")]
    {
        match serde_json::to_string_pretty(&user) {
            Ok(json) => {
                println!("\n  {} JSON representation:", "✓".green());
                for line in json.lines() {
                    println!("    {}", line.bright_white());
                }

                #[cfg(feature = "logging")]
                info!("Successfully serialized user to JSON");
            }
            Err(e) => {
                println!("  {} Failed to serialize: {}", "✗".red(), e);

                #[cfg(feature = "logging")]
                warn!("JSON serialization failed: {}", e);
            }
        }
    }

    #[cfg(not(feature = "json"))]
    {
        println!("\n  {} JSON feature not enabled", "ℹ".blue());
        println!("    Run with: cargo run --features json");
    }

    // XML serialization (only if feature enabled)
    #[cfg(feature = "xml")]
    {
        println!("\n  {} XML support is enabled!", "✓".green());
        println!("    (XML serialization would be implemented here)");

        #[cfg(feature = "logging")]
        info!("XML feature is available");
    }

    #[cfg(not(feature = "xml"))]
    {
        println!("\n  {} XML feature not enabled", "ℹ".blue());
        println!("    Run with: cargo run --features xml");
    }
}

// ============================================================================
// BUILD CONFIGURATION
// ============================================================================

/// Shows information about the current build profile
fn print_build_config() {
    println!("{}", "Build Configuration:".bright_yellow());

    // cfg! macro evaluates at compile time
    if cfg!(debug_assertions) {
        println!("  Build mode: {}", "debug".yellow());
        println!("  Optimizations: {}", "disabled".yellow());
        println!("  Debug symbols: {}", "included".yellow());
    } else {
        println!("  Build mode: {}", "release".green());
        println!("  Optimizations: {}", "enabled (opt-level 3)".green());
        println!("  Debug symbols: {}", "stripped".green());
    }

    // Platform detection
    if cfg!(target_os = "linux") {
        println!("  Target OS: Linux");
    } else if cfg!(target_os = "macos") {
        println!("  Target OS: macOS");
    } else if cfg!(target_os = "windows") {
        println!("  Target OS: Windows");
    }

    // Architecture detection
    if cfg!(target_arch = "x86_64") {
        println!("  Architecture: x86_64 (64-bit)");
    } else if cfg!(target_arch = "aarch64") {
        println!("  Architecture: ARM64");
    }

    #[cfg(feature = "logging")]
    debug!("Build configuration printed");
}

// ============================================================================
// PERFORMANCE DEMONSTRATION
// ============================================================================

/// Demonstrates performance differences between debug and release builds
fn demonstrate_performance() {
    println!("{}", "Performance Test:".bright_yellow());

    use std::time::Instant;

    let start = Instant::now();

    // Simple computation to show optimization differences
    let mut sum: u64 = 0;
    for i in 0..10_000_000 {
        sum = sum.wrapping_add(i);
    }

    let duration = start.elapsed();

    println!("  Computed sum of 10 million numbers");
    println!("  Result: {}", sum);
    println!("  Time: {:?}", duration);

    if cfg!(debug_assertions) {
        println!("\n  {} This is a debug build (slow)", "⚠".yellow());
        println!("    For better performance, run: cargo run --release");
    } else {
        println!("\n  {} This is a release build (fast)", "✓".green());
        println!("    Optimizations are enabled!");
    }

    #[cfg(feature = "logging")]
    info!("Performance test completed in {:?}", duration);
}

// ============================================================================
// CONDITIONAL FUNCTIONS
// ============================================================================

// This function only exists when JSON feature is enabled
#[cfg(feature = "json")]
fn serialize_to_json<T: Serialize>(data: &T) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(data)
}

// This function only exists when XML feature is enabled
#[cfg(feature = "xml")]
fn _serialize_to_xml<T>(_data: &T) -> String {
    // Placeholder implementation
    "<xml>Example XML</xml>".to_string()
}

// ============================================================================
// PLATFORM-SPECIFIC CODE
// ============================================================================

#[cfg(target_os = "linux")]
fn get_platform_name() -> &'static str {
    "Linux"
}

#[cfg(target_os = "macos")]
fn get_platform_name() -> &'static str {
    "macOS"
}

#[cfg(target_os = "windows")]
fn get_platform_name() -> &'static str {
    "Windows"
}

#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
fn get_platform_name() -> &'static str {
    "Unknown"
}

// ============================================================================
// WHAT RUST DOES UNDER THE HOOD
// ============================================================================
// 1. COMPILE-TIME EVALUATION
//    All cfg! and #[cfg] attributes are evaluated during compilation.
//    Code that doesn't match is completely removed - it doesn't even
//    make it into the binary!
//
// 2. ZERO RUNTIME COST
//    There are NO runtime checks for features. If JSON is disabled,
//    the serialization code simply doesn't exist in the final binary.
//    This is unlike Python's if statements which check at runtime.
//
// 3. DEPENDENCY RESOLUTION
//    When you don't enable a feature, Cargo doesn't even download
//    the optional dependencies. This saves compilation time.
//
// 4. PROFILE OPTIMIZATION
//    Debug builds (opt-level 0): Fast compilation, slow execution
//    Release builds (opt-level 3): Slow compilation, fast execution
//    LTO (Link Time Optimization): Even slower build, even faster runtime
//
// 5. BINARY SIZE
//    Features directly affect binary size:
//    - No features: ~500 KB
//    - With serde_json: ~700 KB
//    - With all features: ~1 MB
//    Stripped release builds are smaller.

// ============================================================================
// KEY TAKEAWAYS
// ============================================================================
// 1. Features enable optional functionality at compile time
// 2. Use #[cfg(feature = "...")] for conditional compilation
// 3. Optional dependencies are only included when features are enabled
// 4. Features are additive (can't disable what another dependency enables)
// 5. Debug builds are slow but compile fast
// 6. Release builds are fast but compile slow
// 7. cfg! works for features, target OS, architecture, etc.
// 8. Features reduce binary size by excluding unused code
// 9. Zero runtime cost - all decisions made at compile time
// 10. Use features to create modular, flexible libraries

// ============================================================================
// FEATURE FLAGS BEST PRACTICES
// ============================================================================
// ✅ DO:
// - Make features optional and additive
// - Provide sensible defaults
// - Document feature requirements clearly
// - Test all feature combinations in CI
// - Use features for optional dependencies
//
// ❌ DON'T:
// - Create features that change behavior incompatibly
// - Make features mutually exclusive (problematic in dependency tree)
// - Use features for runtime configuration (use config files instead)
// - Create too many fine-grained features (maintenance burden)

// ============================================================================
// COMMON BEGINNER MISTAKES
// ============================================================================
// ❌ Forgetting to enable feature: cargo run (missing --features json)
// ❌ Wrong cfg syntax: #[cfg(json)] instead of #[cfg(feature = "json")]
// ❌ Not using dep: prefix: json = ["serde_json"] instead of ["dep:serde_json"]
// ❌ Expecting features to be mutually exclusive (they're additive)
// ❌ Using runtime if instead of compile-time #[cfg]
// ❌ Not testing without default features

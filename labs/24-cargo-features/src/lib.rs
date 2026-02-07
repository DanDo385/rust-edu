// Lab 24: Cargo Features - Conditional Compilation
//
// This module demonstrates Rust's compile-time conditional compilation
// using #[cfg] attributes and the cfg!() macro. All feature detection
// and conditional compilation happens at compile time with zero runtime cost.
//
// Key Concepts:
// - #[cfg(target_os = "...")] for platform-specific code
// - cfg!(debug_assertions) for build profile detection
// - #[cfg(target_arch = "...")] for architecture detection
// - #[cfg(feature = "...")] for feature-gated code
// - Conditional struct derives and implementations

// ============================================================================
// PLATFORM DETECTION
// ============================================================================

/// Returns the name of the current target operating system.
///
/// Uses #[cfg(target_os)] to select the correct implementation at compile time.
/// The compiler completely removes code paths for other platforms -- they don't
/// even exist in the final binary. This is NOT a runtime check.
///
/// # Memory Model
/// The returned &'static str lives in the binary's read-only data segment.
/// No heap allocation occurs.
#[cfg(target_os = "linux")]
pub fn get_platform_name() -> &'static str {
    "Linux"
}

#[cfg(target_os = "macos")]
pub fn get_platform_name() -> &'static str {
    "macOS"
}

#[cfg(target_os = "windows")]
pub fn get_platform_name() -> &'static str {
    "Windows"
}

#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
pub fn get_platform_name() -> &'static str {
    "Unknown"
}

// ============================================================================
// BUILD MODE DETECTION
// ============================================================================

/// Returns true if the current build has debug assertions enabled.
///
/// In debug builds (cargo build / cargo test), debug_assertions is true.
/// In release builds (cargo build --release), debug_assertions is false.
///
/// The cfg!() macro evaluates at compile time and produces a bool constant.
/// The compiler can optimize away the unreachable branch entirely.
///
/// # Symbol Deep Dive
/// cfg!() is a macro, not a function. It expands to `true` or `false` at
/// compile time. The optimizer then removes the dead branch -- zero runtime cost.
pub fn is_debug_build() -> bool {
    cfg!(debug_assertions)
}

/// Returns a human-readable string describing the current build mode.
///
/// Demonstrates using cfg!() for conditional logic that compiles to
/// a simple constant on each build profile.
pub fn get_build_mode() -> &'static str {
    if cfg!(debug_assertions) {
        "debug"
    } else {
        "release"
    }
}

// ============================================================================
// ARCHITECTURE DETECTION
// ============================================================================

/// Returns the name of the current target architecture.
///
/// Uses #[cfg(target_arch)] to select the correct string at compile time.
/// Common architectures:
/// - "x86_64": 64-bit Intel/AMD (most desktops/servers)
/// - "aarch64": 64-bit ARM (Apple Silicon, ARM servers)
/// - "x86": 32-bit Intel/AMD (legacy)
/// - "arm": 32-bit ARM (embedded, Raspberry Pi)
#[cfg(target_arch = "x86_64")]
pub fn get_target_arch() -> &'static str {
    "x86_64"
}

#[cfg(target_arch = "aarch64")]
pub fn get_target_arch() -> &'static str {
    "aarch64"
}

#[cfg(target_arch = "x86")]
pub fn get_target_arch() -> &'static str {
    "x86"
}

#[cfg(target_arch = "arm")]
pub fn get_target_arch() -> &'static str {
    "arm"
}

#[cfg(not(any(
    target_arch = "x86_64",
    target_arch = "aarch64",
    target_arch = "x86",
    target_arch = "arm"
)))]
pub fn get_target_arch() -> &'static str {
    "unknown"
}

// ============================================================================
// POINTER WIDTH DETECTION
// ============================================================================

/// Returns the pointer width of the current target in bits.
///
/// This is useful for detecting 32-bit vs 64-bit platforms.
/// On 64-bit systems: pointers are 8 bytes (64 bits).
/// On 32-bit systems: pointers are 4 bytes (32 bits).
pub fn get_pointer_width() -> usize {
    std::mem::size_of::<usize>() * 8
}

/// Returns true if running on a 64-bit platform.
pub fn is_64_bit() -> bool {
    cfg!(target_pointer_width = "64")
}

// ============================================================================
// USER STRUCT (WITHOUT OPTIONAL SERDE)
// ============================================================================

/// A basic user record demonstrating struct design that can be extended
/// with feature-gated derives.
///
/// In the full cargo-features demo, this struct conditionally derives
/// Serialize/Deserialize when the "json" feature is enabled. Here we
/// show the base struct without framework dependencies.
///
/// # Memory Model
/// ```text
/// Stack:                          Heap:
/// ┌──────────────────────┐       ┌─────────────────┐
/// │ id:     u32 (4 bytes)│       │ "Alice"          │
/// │ name:   ptr+len+cap  │──────>│ (5 bytes + cap)  │
/// │ email:  ptr+len+cap  │──┐    └─────────────────┘
/// │ active: bool (1 byte)│  │    ┌──────────────────────┐
/// └──────────────────────┘  └───>│ "alice@example.com"   │
///                                │ (18 bytes + cap)      │
///                                └──────────────────────┘
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub active: bool,
}

impl User {
    /// Creates a new active user with the given id, name, and email.
    ///
    /// New users are active by default. This mirrors real-world patterns
    /// where accounts are created in an active state.
    pub fn new(id: u32, name: &str, email: &str) -> Self {
        User {
            id,
            name: name.to_string(),
            email: email.to_string(),
            active: true,
        }
    }

    /// Deactivates this user account.
    pub fn deactivate(&mut self) {
        self.active = false;
    }

    /// Activates this user account.
    pub fn activate(&mut self) {
        self.active = true;
    }

    /// Returns a display-friendly summary of the user.
    pub fn summary(&self) -> String {
        let status = if self.active { "active" } else { "inactive" };
        format!("User #{}: {} ({}) [{}]", self.id, self.name, self.email, status)
    }
}

// ============================================================================
// FEATURE STATUS REPORTING
// ============================================================================

/// Represents the status of a compile-time feature.
#[derive(Debug, Clone, PartialEq)]
pub struct FeatureStatus {
    pub name: String,
    pub enabled: bool,
}

impl FeatureStatus {
    /// Creates a new FeatureStatus.
    pub fn new(name: &str, enabled: bool) -> Self {
        FeatureStatus {
            name: name.to_string(),
            enabled,
        }
    }

    /// Returns a human-readable status string.
    pub fn display(&self) -> String {
        if self.enabled {
            format!("{}: enabled", self.name)
        } else {
            format!("{}: disabled", self.name)
        }
    }
}

/// Collects the status of all known features for this crate.
///
/// This function queries compile-time feature flags and returns a Vec
/// of FeatureStatus structs. Each #[cfg(feature = "...")] check is
/// resolved at compile time -- the returned Vec contains only the
/// statuses that were compiled into the binary.
pub fn get_feature_statuses() -> Vec<FeatureStatus> {
    let mut statuses = Vec::new();

    // Each cfg! check is resolved at compile time to true or false.
    // The compiler can optimize away the false branches entirely.
    statuses.push(FeatureStatus::new("json", cfg!(feature = "json")));
    statuses.push(FeatureStatus::new("xml", cfg!(feature = "xml")));
    statuses.push(FeatureStatus::new("logging", cfg!(feature = "logging")));

    statuses
}

/// Returns the number of currently enabled features.
pub fn count_enabled_features() -> usize {
    get_feature_statuses()
        .iter()
        .filter(|f| f.enabled)
        .count()
}

// ============================================================================
// BUILD INFORMATION STRUCT
// ============================================================================

/// Aggregates all compile-time build information into a single struct.
///
/// This is a pattern commonly used in production Rust applications to
/// expose build metadata (often combined with the `built` crate).
#[derive(Debug, Clone)]
pub struct BuildInfo {
    pub platform: &'static str,
    pub arch: &'static str,
    pub pointer_width: usize,
    pub debug: bool,
    pub build_mode: &'static str,
}

impl BuildInfo {
    /// Collects current build information.
    ///
    /// All values are determined at compile time. This function simply
    /// aggregates them into a convenient struct.
    pub fn collect() -> Self {
        BuildInfo {
            platform: get_platform_name(),
            arch: get_target_arch(),
            pointer_width: get_pointer_width(),
            debug: is_debug_build(),
            build_mode: get_build_mode(),
        }
    }
}

impl std::fmt::Display for BuildInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Platform: {}, Arch: {}, {}-bit, Mode: {}",
            self.platform, self.arch, self.pointer_width, self.build_mode
        )
    }
}

// ============================================================================
// CONDITIONAL COMPILATION UTILITIES
// ============================================================================

/// Returns the endianness of the current target.
///
/// Most modern systems are little-endian (x86, ARM in LE mode).
/// Some embedded/network systems are big-endian.
pub fn get_endianness() -> &'static str {
    if cfg!(target_endian = "little") {
        "little-endian"
    } else {
        "big-endian"
    }
}

/// Returns the target OS family ("unix" or "windows").
///
/// This is broader than target_os -- both Linux and macOS are "unix" family.
pub fn get_os_family() -> &'static str {
    if cfg!(unix) {
        "unix"
    } else if cfg!(windows) {
        "windows"
    } else {
        "unknown"
    }
}

// ============================================================================
// WHAT RUST DOES UNDER THE HOOD
// ============================================================================
// 1. COMPILE-TIME EVALUATION
//    All cfg! and #[cfg] attributes are evaluated during compilation.
//    Code that doesn't match is completely removed -- it doesn't even
//    make it into the binary!
//
// 2. ZERO RUNTIME COST
//    There are NO runtime checks for features. If JSON is disabled,
//    the serialization code simply doesn't exist in the final binary.
//    cfg!() expands to a literal `true` or `false` constant.
//
// 3. DEAD CODE ELIMINATION
//    When cfg!() produces `false`, the optimizer removes the entire
//    unreachable branch. The `if cfg!(...)` pattern compiles to the
//    same code as #[cfg(...)], just with a different syntax.
//
// 4. FEATURE ADDITIVITY
//    Features in Cargo are always additive. Enabling feature A cannot
//    disable feature B. This prevents "feature hell" in dependency trees.
//
// 5. BINARY SIZE
//    Features directly affect binary size:
//    - Disabled features: code doesn't exist in binary
//    - No feature = no dependency download, no compile time
//    - This is why cfg is superior to runtime if-else for optional code

// ============================================================================
// KEY TAKEAWAYS
// ============================================================================
// 1. #[cfg(...)] removes entire items (functions, structs, impls) at compile time
// 2. cfg!(...) evaluates to true/false at compile time (usable in expressions)
// 3. target_os, target_arch, target_pointer_width detect platform
// 4. debug_assertions distinguishes debug from release builds
// 5. feature = "name" gates code on Cargo feature flags
// 6. Features are additive -- they can only add functionality
// 7. Zero runtime cost for all conditional compilation
// 8. Use BuildInfo pattern to aggregate compile-time metadata
// 9. FeatureStatus pattern makes feature reporting testable
// 10. Always provide fallback implementations for unknown platforms

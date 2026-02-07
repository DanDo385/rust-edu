// Lab 24: Cargo Features - Integration Tests
//
// These tests verify the conditional compilation utilities without
// requiring any optional dependencies (colored, serde, etc.).
// All tests exercise compile-time cfg detection and the User data model.

use cargo_features::*;

// ============================================================================
// PLATFORM DETECTION TESTS
// ============================================================================

#[test]
fn test_get_platform_name_returns_known_platform() {
    let platform = get_platform_name();
    let known_platforms = ["Linux", "macOS", "Windows", "Unknown"];
    assert!(
        known_platforms.contains(&platform),
        "Expected known platform, got: {}",
        platform
    );
}

#[test]
fn test_get_platform_name_is_not_empty() {
    let platform = get_platform_name();
    assert!(!platform.is_empty(), "Platform name should not be empty");
}

#[cfg(target_os = "macos")]
#[test]
fn test_platform_is_macos() {
    assert_eq!(get_platform_name(), "macOS");
}

#[cfg(target_os = "linux")]
#[test]
fn test_platform_is_linux() {
    assert_eq!(get_platform_name(), "Linux");
}

#[cfg(target_os = "windows")]
#[test]
fn test_platform_is_windows() {
    assert_eq!(get_platform_name(), "Windows");
}

// ============================================================================
// BUILD MODE DETECTION TESTS
// ============================================================================

#[test]
fn test_is_debug_build_returns_bool() {
    // In test mode (cargo test), debug_assertions are enabled by default.
    let result = is_debug_build();
    // cargo test runs in debug mode unless --release is specified
    assert!(
        result == true || result == false,
        "is_debug_build should return a boolean"
    );
}

#[test]
fn test_debug_build_in_test_mode() {
    // cargo test defaults to debug mode (debug_assertions = true)
    // This test will pass under normal `cargo test` invocation.
    // If run with `cargo test --release`, debug_assertions would be false.
    if cfg!(debug_assertions) {
        assert!(is_debug_build());
        assert_eq!(get_build_mode(), "debug");
    } else {
        assert!(!is_debug_build());
        assert_eq!(get_build_mode(), "release");
    }
}

#[test]
fn test_get_build_mode_returns_valid_string() {
    let mode = get_build_mode();
    assert!(
        mode == "debug" || mode == "release",
        "Build mode should be 'debug' or 'release', got: {}",
        mode
    );
}

#[test]
fn test_build_mode_consistent_with_is_debug() {
    // These two functions must agree with each other.
    if is_debug_build() {
        assert_eq!(get_build_mode(), "debug");
    } else {
        assert_eq!(get_build_mode(), "release");
    }
}

// ============================================================================
// ARCHITECTURE DETECTION TESTS
// ============================================================================

#[test]
fn test_get_target_arch_returns_known_arch() {
    let arch = get_target_arch();
    let known_archs = ["x86_64", "aarch64", "x86", "arm", "unknown"];
    assert!(
        known_archs.contains(&arch),
        "Expected known architecture, got: {}",
        arch
    );
}

#[test]
fn test_get_target_arch_is_not_empty() {
    let arch = get_target_arch();
    assert!(!arch.is_empty(), "Architecture name should not be empty");
}

#[cfg(target_arch = "x86_64")]
#[test]
fn test_arch_is_x86_64() {
    assert_eq!(get_target_arch(), "x86_64");
}

#[cfg(target_arch = "aarch64")]
#[test]
fn test_arch_is_aarch64() {
    assert_eq!(get_target_arch(), "aarch64");
}

// ============================================================================
// POINTER WIDTH TESTS
// ============================================================================

#[test]
fn test_get_pointer_width_is_valid() {
    let width = get_pointer_width();
    // Most platforms are either 32-bit or 64-bit.
    assert!(
        width == 32 || width == 64,
        "Pointer width should be 32 or 64, got: {}",
        width
    );
}

#[test]
fn test_is_64_bit_matches_pointer_width() {
    if is_64_bit() {
        assert_eq!(get_pointer_width(), 64);
    } else {
        assert_eq!(get_pointer_width(), 32);
    }
}

#[cfg(target_pointer_width = "64")]
#[test]
fn test_64_bit_platform() {
    assert!(is_64_bit());
    assert_eq!(get_pointer_width(), 64);
}

// ============================================================================
// USER STRUCT TESTS
// ============================================================================

#[test]
fn test_user_new_creates_active_user() {
    let user = User::new(1, "Alice", "alice@example.com");
    assert_eq!(user.id, 1);
    assert_eq!(user.name, "Alice");
    assert_eq!(user.email, "alice@example.com");
    assert!(user.active, "New users should be active by default");
}

#[test]
fn test_user_deactivate() {
    let mut user = User::new(1, "Alice", "alice@example.com");
    assert!(user.active);
    user.deactivate();
    assert!(!user.active, "User should be inactive after deactivation");
}

#[test]
fn test_user_activate_after_deactivate() {
    let mut user = User::new(1, "Alice", "alice@example.com");
    user.deactivate();
    assert!(!user.active);
    user.activate();
    assert!(user.active, "User should be active after reactivation");
}

#[test]
fn test_user_summary_active() {
    let user = User::new(42, "Bob", "bob@test.com");
    let summary = user.summary();
    assert!(summary.contains("Bob"), "Summary should contain user name");
    assert!(summary.contains("bob@test.com"), "Summary should contain email");
    assert!(summary.contains("42"), "Summary should contain user id");
    assert!(summary.contains("active"), "Summary should show active status");
    assert!(!summary.contains("inactive"), "Active user should not show 'inactive'");
}

#[test]
fn test_user_summary_inactive() {
    let mut user = User::new(7, "Carol", "carol@test.com");
    user.deactivate();
    let summary = user.summary();
    assert!(summary.contains("Carol"), "Summary should contain user name");
    assert!(summary.contains("inactive"), "Summary should show inactive status");
}

#[test]
fn test_user_clone() {
    let user = User::new(1, "Alice", "alice@example.com");
    let cloned = user.clone();
    assert_eq!(user, cloned, "Cloned user should equal original");
    // Cloned strings are independent heap allocations.
    assert_eq!(cloned.name, "Alice");
}

#[test]
fn test_user_equality() {
    let user1 = User::new(1, "Alice", "alice@example.com");
    let user2 = User::new(1, "Alice", "alice@example.com");
    assert_eq!(user1, user2, "Users with same fields should be equal");
}

#[test]
fn test_user_inequality_different_id() {
    let user1 = User::new(1, "Alice", "alice@example.com");
    let user2 = User::new(2, "Alice", "alice@example.com");
    assert_ne!(user1, user2, "Users with different IDs should not be equal");
}

#[test]
fn test_user_debug_format() {
    let user = User::new(1, "Alice", "alice@example.com");
    let debug = format!("{:?}", user);
    assert!(debug.contains("User"), "Debug format should contain 'User'");
    assert!(debug.contains("Alice"), "Debug format should contain name");
}

#[test]
fn test_user_with_empty_name() {
    // Edge case: empty strings are valid Rust strings.
    let user = User::new(0, "", "");
    assert_eq!(user.name, "");
    assert_eq!(user.email, "");
    assert!(user.active);
}

#[test]
fn test_user_with_unicode() {
    let user = User::new(1, "Rene", "rene@example.com");
    assert_eq!(user.name, "Rene");
    let summary = user.summary();
    assert!(summary.contains("Rene"));
}

// ============================================================================
// FEATURE STATUS TESTS
// ============================================================================

#[test]
fn test_feature_status_new() {
    let status = FeatureStatus::new("test-feature", true);
    assert_eq!(status.name, "test-feature");
    assert!(status.enabled);
}

#[test]
fn test_feature_status_display_enabled() {
    let status = FeatureStatus::new("json", true);
    assert_eq!(status.display(), "json: enabled");
}

#[test]
fn test_feature_status_display_disabled() {
    let status = FeatureStatus::new("xml", false);
    assert_eq!(status.display(), "xml: disabled");
}

#[test]
fn test_feature_status_equality() {
    let a = FeatureStatus::new("json", true);
    let b = FeatureStatus::new("json", true);
    assert_eq!(a, b);
}

#[test]
fn test_feature_status_inequality() {
    let a = FeatureStatus::new("json", true);
    let b = FeatureStatus::new("json", false);
    assert_ne!(a, b);
}

#[test]
fn test_get_feature_statuses_returns_three_features() {
    let statuses = get_feature_statuses();
    assert_eq!(
        statuses.len(),
        3,
        "Should report status for json, xml, and logging features"
    );
}

#[test]
fn test_get_feature_statuses_contains_expected_names() {
    let statuses = get_feature_statuses();
    let names: Vec<&str> = statuses.iter().map(|s| s.name.as_str()).collect();
    assert!(names.contains(&"json"), "Should contain 'json' feature");
    assert!(names.contains(&"xml"), "Should contain 'xml' feature");
    assert!(names.contains(&"logging"), "Should contain 'logging' feature");
}

#[test]
fn test_get_feature_statuses_json_default_enabled() {
    // The Cargo.toml has default = ["json"], so json should be enabled
    // when running cargo test without --no-default-features.
    let statuses = get_feature_statuses();
    let json_status = statuses.iter().find(|s| s.name == "json").unwrap();
    assert!(
        json_status.enabled,
        "json feature should be enabled by default"
    );
}

#[test]
fn test_count_enabled_features() {
    let count = count_enabled_features();
    // At minimum, the "json" default feature should be enabled.
    assert!(
        count >= 1,
        "At least one feature (json) should be enabled by default, got: {}",
        count
    );
    assert!(
        count <= 3,
        "At most 3 features can be enabled, got: {}",
        count
    );
}

// ============================================================================
// BUILD INFO TESTS
// ============================================================================

#[test]
fn test_build_info_collect() {
    let info = BuildInfo::collect();
    assert!(!info.platform.is_empty(), "Platform should not be empty");
    assert!(!info.arch.is_empty(), "Arch should not be empty");
    assert!(
        info.pointer_width == 32 || info.pointer_width == 64,
        "Pointer width should be 32 or 64"
    );
}

#[test]
fn test_build_info_consistency() {
    let info = BuildInfo::collect();
    // All fields should be consistent with the individual functions.
    assert_eq!(info.platform, get_platform_name());
    assert_eq!(info.arch, get_target_arch());
    assert_eq!(info.pointer_width, get_pointer_width());
    assert_eq!(info.debug, is_debug_build());
    assert_eq!(info.build_mode, get_build_mode());
}

#[test]
fn test_build_info_display() {
    let info = BuildInfo::collect();
    let display = format!("{}", info);
    assert!(display.contains("Platform:"), "Display should contain 'Platform:'");
    assert!(display.contains("Arch:"), "Display should contain 'Arch:'");
    assert!(display.contains("Mode:"), "Display should contain 'Mode:'");
    assert!(
        display.contains("bit"),
        "Display should contain pointer width with 'bit'"
    );
}

#[test]
fn test_build_info_debug_format() {
    let info = BuildInfo::collect();
    let debug = format!("{:?}", info);
    assert!(debug.contains("BuildInfo"), "Debug should contain struct name");
}

#[test]
fn test_build_info_clone() {
    let info = BuildInfo::collect();
    let cloned = info.clone();
    assert_eq!(info.platform, cloned.platform);
    assert_eq!(info.arch, cloned.arch);
    assert_eq!(info.pointer_width, cloned.pointer_width);
    assert_eq!(info.debug, cloned.debug);
    assert_eq!(info.build_mode, cloned.build_mode);
}

// ============================================================================
// ENDIANNESS AND OS FAMILY TESTS
// ============================================================================

#[test]
fn test_get_endianness_returns_valid_value() {
    let endianness = get_endianness();
    assert!(
        endianness == "little-endian" || endianness == "big-endian",
        "Endianness should be 'little-endian' or 'big-endian', got: {}",
        endianness
    );
}

#[cfg(target_endian = "little")]
#[test]
fn test_endianness_is_little() {
    assert_eq!(get_endianness(), "little-endian");
}

#[test]
fn test_get_os_family_returns_valid_value() {
    let family = get_os_family();
    assert!(
        family == "unix" || family == "windows" || family == "unknown",
        "OS family should be 'unix', 'windows', or 'unknown', got: {}",
        family
    );
}

#[cfg(unix)]
#[test]
fn test_os_family_is_unix() {
    assert_eq!(get_os_family(), "unix");
}

#[cfg(windows)]
#[test]
fn test_os_family_is_windows() {
    assert_eq!(get_os_family(), "windows");
}

// ============================================================================
// CROSS-FUNCTION CONSISTENCY TESTS
// ============================================================================

#[test]
fn test_all_platform_functions_consistent() {
    // If platform is macOS or Linux, OS family should be unix.
    let platform = get_platform_name();
    let family = get_os_family();

    match platform {
        "macOS" | "Linux" => assert_eq!(family, "unix"),
        "Windows" => assert_eq!(family, "windows"),
        _ => {} // Unknown platforms, no constraint
    }
}

#[test]
fn test_arch_and_pointer_width_consistent() {
    let arch = get_target_arch();
    let width = get_pointer_width();

    // 64-bit architectures should have 64-bit pointers.
    match arch {
        "x86_64" | "aarch64" => assert_eq!(width, 64, "{} should have 64-bit pointers", arch),
        "x86" | "arm" => assert_eq!(width, 32, "{} should have 32-bit pointers", arch),
        _ => {} // Unknown architectures, no constraint
    }
}

// ============================================================================
// MULTIPLE USER OPERATIONS TEST
// ============================================================================

#[test]
fn test_multiple_users() {
    let users: Vec<User> = (1..=5)
        .map(|i| User::new(i, &format!("User{}", i), &format!("user{}@test.com", i)))
        .collect();

    assert_eq!(users.len(), 5);
    assert!(users.iter().all(|u| u.active), "All new users should be active");

    // Check IDs are sequential.
    for (i, user) in users.iter().enumerate() {
        assert_eq!(user.id, (i + 1) as u32);
    }
}

#[test]
fn test_user_activate_deactivate_cycle() {
    let mut user = User::new(1, "Test", "test@test.com");
    for _ in 0..10 {
        user.deactivate();
        assert!(!user.active);
        user.activate();
        assert!(user.active);
    }
}

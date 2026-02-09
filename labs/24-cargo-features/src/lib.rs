//! # Lab 24: Cargo Features
//!
//! Student-facing API for compile-time configuration exercises.

pub fn get_platform_name() -> &'static str {
    // TODO: Return a platform label using cfg attributes/macros.
    todo!("Implement platform detection")
}

pub fn is_debug_build() -> bool {
    // TODO: Return cfg!(debug_assertions).
    todo!("Implement debug build detection")
}

pub fn get_build_mode() -> &'static str {
    // TODO: Return "debug" or "release" based on cfg.
    todo!("Implement build mode detection")
}

pub fn get_target_arch() -> &'static str {
    // TODO: Return target architecture name.
    todo!("Implement target arch detection")
}

pub fn get_pointer_width() -> usize {
    // TODO: Return pointer width in bits.
    todo!("Implement pointer width detection")
}

pub fn is_64_bit() -> bool {
    // TODO: Return true for 64-bit targets.
    todo!("Implement 64-bit detection")
}

#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub active: bool,
}

impl User {
    pub fn new(id: u32, name: &str, email: &str) -> Self {
        let _ = (id, name, email);
        todo!("Create User")
    }

    pub fn deactivate(&mut self) {
        todo!("Deactivate user")
    }

    pub fn activate(&mut self) {
        todo!("Activate user")
    }

    pub fn summary(&self) -> String {
        todo!("Build user summary")
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FeatureStatus {
    pub name: String,
    pub enabled: bool,
}

impl FeatureStatus {
    pub fn new(name: &str, enabled: bool) -> Self {
        let _ = (name, enabled);
        todo!("Create FeatureStatus")
    }

    pub fn display(&self) -> String {
        todo!("Display feature status")
    }
}

pub fn get_feature_statuses() -> Vec<FeatureStatus> {
    // TODO: Return feature status list.
    todo!("Collect feature statuses")
}

pub fn count_enabled_features() -> usize {
    // TODO: Count enabled feature statuses.
    todo!("Count enabled features")
}

#[derive(Debug, Clone)]
pub struct BuildInfo {
    pub platform: &'static str,
    pub arch: &'static str,
    pub pointer_width: usize,
    pub debug: bool,
    pub build_mode: &'static str,
}

impl BuildInfo {
    pub fn collect() -> Self {
        // TODO: Aggregate compile-time build info.
        todo!("Collect build info")
    }
}

impl std::fmt::Display for BuildInfo {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!("Format BuildInfo")
    }
}

pub fn get_endianness() -> &'static str {
    // TODO: Return little-endian or big-endian.
    todo!("Detect endianness")
}

pub fn get_os_family() -> &'static str {
    // TODO: Return unix/windows/unknown family.
    todo!("Detect OS family")
}

#[doc(hidden)]
pub mod solution;

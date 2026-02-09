//! # Cargo Features Demo

use cargo_features::solution;

fn main() {
    println!("=== Cargo Features Demo ===\n");
    println!("platform: {}", solution::get_platform_name());
    println!("arch: {}", solution::get_target_arch());
    println!("pointer width: {}", solution::get_pointer_width());
    println!("build mode: {}", solution::get_build_mode());
    println!("debug: {}", solution::is_debug_build());
    println!("endianness: {}", solution::get_endianness());
    println!("os family: {}", solution::get_os_family());

    let info = solution::BuildInfo::collect();
    println!("build info: {}", info);
}

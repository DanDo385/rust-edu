//! # Modules and Crates Demo

use modules_crates::solution::{self, prelude::*};

fn main() {
    println!("=== Modules and Crates Demo ===\n");

    let user = User::new("alice".to_string(), "alice@example.com".to_string());
    let token = authenticate(&user);

    println!("user: {}", user.display_name());
    println!("token valid: {}", token.is_valid());

    let svc = solution::services::auth::AuthService::default();
    println!("service verify: {}", svc.verify(&token, &user));

    solution::init("modules-crates demo");
    println!("version: {}", solution::version());
}

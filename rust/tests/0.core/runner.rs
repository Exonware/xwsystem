// #exonware/xwsystem/rust/tests/0.core/runner.rs
//! Core test runner.
//!
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//!
//! This runner executes all core tests following GUIDE_TEST.md structure.

fn main() {
    // Run all core tests
    // In Rust, we use cargo test, but this runner can be used for organization
    println!("Running core tests...");
    println!("Use: cargo test --test test_core_defs");
    println!("Use: cargo test --test test_core_errors");
    println!("Use: cargo test --test test_core_base");
    println!("Use: cargo test --test test_core_contracts");
    println!("Or: cargo test --test core_tests (if we create a unified test file)");
}


// #exonware/xwsystem/rust/tests/lib.rs
//! Test library for xwsystem_rust.
//!
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//!
//! This module organizes tests following GUIDE_TEST.md structure.

#[cfg(test)]
mod core_tests {
    // Core tests (0.core)
    #[path = "0.core/test_core_defs.rs"]
    mod test_core_defs;
    
    #[path = "0.core/test_core_errors.rs"]
    mod test_core_errors;
    
    #[path = "0.core/test_core_base.rs"]
    mod test_core_base;
    
    #[path = "0.core/test_core_contracts.rs"]
    mod test_core_contracts;
}

#[cfg(test)]
mod unit_tests {
    // Unit tests (1.unit)
    #[path = "1.unit/defs_tests/test_defs.rs"]
    mod test_defs;
}


# XWSystem Rust Implementation

Rust implementation of XWSystem core modules.

## Overview

This directory contains Rust implementations of XWSystem modules, starting with the simplest modules and gradually expanding.

## Current Modules

### Version Module (`version`)

Centralized version management for XWSystem.

**Features:**
- Version constants and metadata
- Version information utilities
- Development/release version detection
- JSON serialization support

## Building

```bash
cd xwsystem/rust
cargo build
```

For release build:
```bash
cargo build --release
```

## Running Tests

```bash
cargo test
```

## Running Examples

```bash
cargo run --example version_example
```

## Project Structure

```
rust/
├── Cargo.toml          # Rust project configuration
├── src/
│   ├── lib.rs          # Library root
│   └── version.rs      # Version module
├── tests/              # Integration tests
│   └── version_tests.rs
├── examples/           # Example programs
│   └── version_example.rs
└── README.md          # This file
```

## Development

This is a work in progress. We're starting with the simplest modules and gradually porting more complex functionality.

## License

MIT License - see LICENSE file in parent directory.


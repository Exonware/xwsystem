# Python vs Rust Caching Benchmark

This directory contains benchmarks comparing Python and Rust implementations of xwsystem caching modules.

## Setup

### Prerequisites

1. **Python xwsystem** (already installed)
2. **Rust toolchain** (install from https://rustup.rs/)
3. **PyO3 bindings** (built with Python feature)

### Building Rust Bindings

```bash
cd xwsystem/rust
cargo build --release --features python

# Or install as Python package
pip install -e . --features python
```

### Installing Rust Module

```bash
# From xwsystem root
cd rust
maturin develop --features python

# Or using pip
pip install -e . --features python
```

## Running Benchmarks

```bash
python benchmark_comparison.py
```

This will:
1. Benchmark Python implementations (LRU, LFU, TTL)
2. Benchmark Rust implementations (LRU, LFU, TTL)
3. Generate comparison report
4. Save results to `results/` directory

## Results

Results are saved in:
- `results/benchmark_results.json` - Raw JSON data
- `results/benchmark_report.md` - Markdown report

## Benchmark Operations

The benchmark tests:
- **Get operations**: Retrieving values from cache
- **Put operations**: Storing values in cache
- **Size operations**: Getting cache size

Each operation is benchmarked with:
- 100 warmup iterations
- 1000 actual iterations
- Statistical analysis (mean, median, stdev, min, max)

## Expected Results

Rust implementations typically show:
- **2-10x speedup** for get operations
- **3-15x speedup** for put operations
- **Lower memory usage**
- **Better scalability** for large caches

## Troubleshooting

### Rust module not found

```bash
# Ensure Rust bindings are built
cd xwsystem/rust
cargo build --release --features python

# Install Python bindings
maturin develop --features python
```

### Import errors

```bash
# Verify Python can find the module
python -c "from exonware.rust.xwsystem.caching import LRUCache; print('OK')"
```

### Performance issues

- Ensure release build: `cargo build --release`
- Check CPU frequency scaling
- Close other applications during benchmarking

## Custom Benchmarks

You can create custom benchmarks by modifying `benchmark_comparison.py`:

```python
runner = BenchmarkRunner(iterations=5000, warmup=500)
runner.run_all_benchmarks()
```

## License

MIT License - see LICENSE file in parent directory.

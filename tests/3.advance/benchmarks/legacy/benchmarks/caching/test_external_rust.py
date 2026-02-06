#!/usr/bin/env python3
#exonware/xwsystem/tests/3.advance/benchmarks/legacy/benchmarks/caching/test_external_rust.py
"""Test script to verify external Rust caches are working."""

import sys

print("="*60)
print("Testing External Rust Cache Dependencies")
print("="*60)

# Test 1: Check Python dependencies
print("\n1. Checking Python dependencies...")
python_deps = {
    'diskcache': 'diskcache',
    'cacheout': 'cacheout',
    'pylru': 'pylru',
    'cachetools': 'cachetools',
    'cachebox': 'cachebox',
}

missing_python = []
for name, module in python_deps.items():
    try:
        __import__(module)
        print(f"  [OK] {name} installed")
    except ImportError:
        print(f"  [FAIL] {name} NOT installed")
        missing_python.append(name)

if missing_python:
    print(f"\n  Install missing packages: pip install {' '.join(missing_python)}")
else:
    print("  [OK] All Python dependencies installed")

# Test 2: Check Rust dependencies in Cargo.toml
print("\n2. Checking Rust dependencies in Cargo.toml...")
try:
    import tomli
    with open('../../rust/Cargo.toml', 'rb') as f:
        cargo = tomli.load(f)
    
    rust_deps = ['mini-moka', 'quick_cache', 'dashmap']
    for dep in rust_deps:
        if dep in cargo.get('dependencies', {}):
            print(f"  [OK] {dep} in Cargo.toml")
        else:
            print(f"  [FAIL] {dep} NOT in Cargo.toml")
except Exception as e:
    print(f"  ⚠ Could not check Cargo.toml: {e}")

# Test 3: Try importing external Rust caches
print("\n3. Testing external Rust cache imports...")
try:
    from exonware.rust.xwsystem.caching import (
        MokaCache,
        MokaTTLCache,
        MokaWeightedCache,
        QuickCache,
        QuickCacheTTL,
        DashMapCache,
        DashMapTTLCache,
    )
    print("  [OK] All external Rust caches imported successfully")
    HAS_EXTERNAL_RUST = True
except ImportError as e:
    print(f"  [FAIL] External Rust caches not available: {e}")
    print("  -> Build with: cd ../../rust && maturin develop --release --features external-caches,python")
    HAS_EXTERNAL_RUST = False

# Test 4: Test cache instantiation and basic operations
if HAS_EXTERNAL_RUST:
    print("\n4. Testing cache instantiation and operations...")
    test_caches = [
        ("MokaCache", lambda: MokaCache(100)),
        ("MokaTTLCache", lambda: MokaTTLCache(100, 60.0)),
        ("MokaWeightedCache", lambda: MokaWeightedCache(100)),
        ("QuickCache", lambda: QuickCache(100)),
        ("QuickCacheTTL", lambda: QuickCacheTTL(100, 60.0)),
        ("DashMapCache", lambda: DashMapCache(100)),
        ("DashMapTTLCache", lambda: DashMapTTLCache(100, 60.0)),
    ]
    
    for name, factory in test_caches:
        try:
            cache = factory()
            # Test put
            cache.put("test_key", {"value": 42})
            # Test get
            result = cache.get("test_key")
            if result and result.get("value") == 42:
                print(f"  [OK] {name} - operations working")
            else:
                print(f"  [FAIL] {name} - get returned wrong value")
            # Test delete
            cache.delete("test_key")
            # Test clear
            cache.clear()
        except Exception as e:
            print(f"  [FAIL] {name} - error: {e}")

# Test 5: Check benchmark script imports
print("\n5. Testing benchmark script imports...")
try:
    sys.path.insert(0, '.')
    from benchmark_rust import (
        HAS_RUST,
        HAS_EXTERNAL_RUST as BENCH_HAS_EXTERNAL_RUST,
        benchmark_cache_class,
    )
    print(f"  [OK] Benchmark script imports successfully")
    print(f"    HAS_RUST: {HAS_RUST}")
    print(f"    HAS_EXTERNAL_RUST: {BENCH_HAS_EXTERNAL_RUST}")
except Exception as e:
    print(f"  [FAIL] Benchmark script import error: {e}")
    import traceback
    traceback.print_exc()

print("\n" + "="*60)
print("Summary")
print("="*60)
if HAS_EXTERNAL_RUST and not missing_python:
    print("[OK] All dependencies installed and working!")
    print("[OK] Ready to run benchmark: python benchmark_rust.py")
else:
    if missing_python:
        print(f"[FAIL] Missing Python packages: {', '.join(missing_python)}")
    if not HAS_EXTERNAL_RUST:
        print("[FAIL] External Rust caches not available - need to build")
print("="*60)

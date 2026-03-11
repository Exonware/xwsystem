# XWSystem Caching Tests

## Overview

Comprehensive test suite for the xwsystem caching module following **GUIDELINES_TEST.md**.

## Test Structure

```
tests/
├── pytest.ini                    # Pytest configuration
├── conftest.py                   # Shared fixtures
├── verify_installation.py        # Installation verification
│
├── 0.core/caching/              # Core Tests (20% for 80% value)
│   ├── test_core_caching.py     # High-value integration tests
│   ├── conftest.py
│   └── runner.py
│
├── 1.unit/caching_unit/          # Unit Tests (Component-level)
│   ├── lru_cache_tests/
│   │   └── test_lru_cache.py
│   ├── security_tests/
│   │   └── test_secure_cache.py
│   ├── conftest.py
│   └── runner.py
│
├── 2.integration/caching/       # Integration Tests (End-to-end)
│   ├── test_end_to_end.py
│   ├── conftest.py
│   └── runner.py
│
└── 3.advance/caching/           # Advance Tests (Excellence - v1.0.0+)
    ├── test_security.py         # Priority #1: Security
    ├── test_performance.py      # Priority #4: Performance
    └── runner.py
```

## Quick Start

### Run All Tests

```bash
# Run core tests (fast, high-value)
python tests/0.core/caching/runner.py

# Run unit tests
python tests/1.unit/caching_unit/runner.py

# Run integration tests
python tests/2.integration/caching/runner.py

# Run advance tests (v1.0.0+)
python tests/3.advance/caching/runner.py
```

### Using Pytest Directly

```bash
# Run all caching tests
pytest tests/ -m xwsystem_caching -v

# Run core tests only
pytest tests/0.core/caching/ -m xwsystem_core -v

# Run security tests
pytest -m "xwsystem_security and xwsystem_caching" -v

# Run performance tests
pytest -m "xwsystem_performance and xwsystem_caching" -v
```

## Test Categories

### 📦 Core Tests (`0.core/caching/`)
- **Purpose:** 20% tests for 80% value
- **Speed:** < 30 seconds total
- **Focus:** Critical functionality, common use cases
- **Markers:** `xwsystem_core`, `xwsystem_caching`

### 🧩 Unit Tests (`1.unit/caching/`)
- **Purpose:** Component-level validation
- **Speed:** < 5 minutes total
- **Focus:** Individual classes and methods
- **Markers:** `xwsystem_unit`, `xwsystem_caching`

### 🔗 Integration Tests (`2.integration/caching/`)
- **Purpose:** End-to-end scenarios
- **Speed:** < 15 minutes total
- **Focus:** Real-world workflows
- **Markers:** `xwsystem_integration`, `xwsystem_caching`

### 🎓 Advance Tests (`3.advance/caching/`)
- **Purpose:** Excellence validation (v1.0.0+)
- **Speed:** < 30 minutes total
- **Focus:** Security, Performance, Extensibility
- **Markers:** `xwsystem_advance`, `xwsystem_security`, `xwsystem_performance`

## Test Coverage

| Component | Core | Unit | Integration | Advance |
|-----------|------|------|-------------|---------|
| LRU Cache | ✅ | ✅ | ✅ | ✅ |
| LFU Cache | ✅ | ⚠️ | ✅ | ✅ |
| TTL Cache | ✅ | ⚠️ | ⚠️ | ⚠️ |
| Secure Cache | ✅ | ✅ | ✅ | ✅ |
| Decorators | ✅ | ⚠️ | ⚠️ | ⚠️ |
| Validation | ⚠️ | ✅ | ⚠️ | ✅ |

**Legend:** ✅ Covered | ⚠️ Partial | ❌ Missing

## Installation Verification

```bash
python tests/verify_installation.py
```

Expected output:
```
================================================================================
🔍 Verifying xwsystem installation...
================================================================================

Testing Import...
✅ Import successful

Testing Caching Functionality...
✅ Caching functionality works

Testing Dependencies...
✅ pytest available

================================================================================
🎉 SUCCESS! xwsystem is ready to use!
================================================================================
```

## Markers

All tests use pytest markers for categorization:

```ini
xwsystem_core           # Core functionality tests
xwsystem_unit           # Unit tests
xwsystem_integration    # Integration tests
xwsystem_advance        # Advance tests (v1.0.0+)
xwsystem_security       # Security excellence (Priority #1)
xwsystem_performance    # Performance excellence (Priority #4)
xwsystem_caching        # Caching module specific
slow                   # Slow tests (> 1 second)
```

## Common Commands

```bash
# Run specific markers
pytest -m xwsystem_core
pytest -m xwsystem_security
pytest -m "xwsystem_unit and not slow"

# Stop on first failure
pytest -x

# Show local variables on failure
pytest -l

# Very verbose
pytest -vv

# With coverage
pytest --cov=exonware.xwsystem.caching --cov-report=html
```

## Test Quality Standards

Following **GUIDELINES_TEST.md**:

✅ **Root Cause Fixing** - Never rig tests, always fix problems  
✅ **Stop on First Failure** (`-x`) - Fast feedback  
✅ **No Warning Suppression** - Warnings indicate real issues  
✅ **Comprehensive Coverage** - 80%+ for critical modules  
✅ **Security First** - Priority #1 testing  
✅ **Performance Benchmarks** - O(1) LFU validation  

## Performance Targets

- **Core Tests:** < 30 seconds
- **Unit Tests:** < 5 minutes
- **Integration Tests:** < 15 minutes
- **Advance Tests:** < 30 minutes

## Contributing

When adding new tests:

1. Place in correct layer (core/unit/integration/advance)
2. Use appropriate markers
3. Follow naming convention: `test_<action>_<expected>`
4. Add docstrings explaining purpose
5. Include Given/When/Then for integration tests
6. Update this README if adding new test categories

## Notes

- **Advance tests** are OPTIONAL until v1.0.0
- **All tests** must pass before releases
- **No rigged tests** - fix problems, don't hide them
- **Security tests** have highest priority (#1)

---

**Company:** eXonware.com  
**Author:** eXonware Backend Team  
**Email:** connect@exonware.com  
**Version:** 0.0.1.388  
**Generated:** 01-Nov-2025


# xwsystem Critical Issues Fix Checklist

**Company:** eXonware.com
**Author:** Eng. Muhammad AlShehri
**Email:** connect@exonware.com
**Date:** 2026-01-24

## Phase 1: Critical UTF-8 & Exception Handling

### UTF-8 Configuration
- [ ] Fix `tests/runner.py` - Use `ensure_utf8_console()`
- [ ] Fix `tests/0.core/runner.py` - Use `ensure_utf8_console()`
- [ ] Fix `tests/1.unit/codec_tests/runner.py` - Use `ensure_utf8_console()`
- [ ] Fix `src/exonware/xwsystem/utils/test_runner.py` - Use `ensure_utf8_console()` instead of `_configure_windows_utf8()`

### Bare `except:` Statements
- [ ] Fix `src/exonware/xwsystem/io/codec/base.py` line 698
- [ ] Fix `src/exonware/xwsystem/console/writer.py` lines 49, 56
- [ ] Fix `src/exonware/xwsystem/console/cli/console.py` line 95
- [ ] Fix `src/exonware/xwsystem/ipc/message_queue.py` lines 211, 401
- [ ] Fix `tests/0.core/runner.py` line 32

### `except Exception: pass` Patterns
- [ ] Fix `tests/runner.py` UTF-8 block
- [ ] Fix `tests/0.core/runner.py` apply_emojis
- [ ] Fix `tests/1.unit/codec_tests/runner.py`
- [ ] Fix `src/exonware/xwsystem/utils/test_runner.py` _configure_windows_utf8
- [ ] Fix `src/exonware/xwsystem/console/cli/encoding.py` ensure_utf8_console fallback
- [ ] Review and fix `src/exonware/xwsystem/io/base.py` (multiple instances)
- [ ] Review and fix `src/exonware/xwsystem/io/indexing/facade.py`
- [ ] Review and fix `src/exonware/xwsystem/security/validator.py`

---

## Phase 2: Test Infrastructure Compliance

### Main Test Runner
- [ ] Refactor `tests/runner.py` to orchestrate layer runners
- [ ] Add `DualOutput` usage to main runner
- [ ] Add `format_path` usage to main runner
- [ ] Add `print_header` usage to main runner
- [ ] Add `print_status` usage to main runner
- [ ] Implement Markdown output to `docs/tests/`
- [ ] Remove direct pytest calls, use subprocess to layer runners

### Missing Layer Runners
- [ ] Create `tests/1.unit/runner.py`
- [ ] Create `tests/2.integration/runner.py`

### Layer Runners Writing Files
- [ ] Fix `tests/1.unit/codec_tests/runner.py` - Remove `runner_out.md` writing
- [ ] Fix `src/exonware/xwsystem/utils/test_runner.py` - TestRunner should not write for layer runners

### Forbidden pytest Flags
- [ ] Remove `--disable-warnings` from `tests/1.unit/serialization_tests/runner.py`

### Warning Suppression
- [ ] Remove `warnings.simplefilter("ignore")` from `test_all_serializers.py`
- [ ] Remove `warnings.simplefilter("ignore")` from `test_final_optimization.py`
- [ ] Remove `warnings.simplefilter("ignore")` from `test_complete_optimization.py`

---

## Phase 3: Critical Bugs

### Path Bugs
- [ ] Fix `tests/1.unit/config_tests/conftest.py` - Change 6× `.parent` to 4× `.parent`
- [ ] Fix `tests/0.core/runner.py` - Update path comment (add `0.`)
- [ ] Fix `tests/0.core/config/runner.py` - Update path comment (add `0.`)

---

## Phase 4: Structure & Consistency

### Missing Unit Module Runners
- [ ] Create `tests/1.unit/config_tests/runner.py`
- [ ] Create `tests/1.unit/patterns_tests/runner.py`
- [ ] Create `tests/1.unit/performance_tests/runner.py`
- [ ] Create `tests/1.unit/security_tests/runner.py`
- [ ] Create `tests/1.unit/structures_tests/runner.py`
- [ ] Create `tests/1.unit/threading_tests/runner.py`
- [ ] Create `tests/1.unit/monitoring_tests/runner.py`

### pytest.ini Consolidation
- [ ] Merge `xwsystem/pytest.ini` and `xwsystem/tests/pytest.ini` into single file
- [ ] Ensure all required settings present (asyncio_mode, markers, addopts)

### `from __future__ import annotations`
- [ ] Add to all test files with type hints (prioritize forward references)

---

## Phase 5: Error Handling Improvements

### io/base.py
- [ ] Replace generic `except Exception` with specific exceptions
- [ ] Consider custom exception types for better error messages

### io/codec/base.py
- [ ] Fix `load()` method - Replace bare `except:` with specific exceptions

---

## Phase 6: Documentation & Review

### pytest.skip Review
- [ ] Audit all `pytest.skip` usage
- [ ] Document legitimate skips vs. broken tests

### Enterprise Dependency Stubs
- [ ] Review `tests/conftest.py` stub strategy
- [ ] Document or refactor stub approach

---

## Verification Steps

After each phase:
- [ ] Run `python tests/runner.py` to verify no regressions
- [ ] Check that UTF-8 encoding works on Windows
- [ ] Verify no bare `except:` remain (grep check)
- [ ] Verify no `--disable-warnings` flags remain
- [ ] Verify main runner writes to `docs/tests/`
- [ ] Verify layer runners don't write files
- [ ] Run core tests: `python tests/0.core/runner.py`
- [ ] Run unit tests: `python tests/1.unit/runner.py` (after creation)
- [ ] Run integration tests: `python tests/2.integration/runner.py` (after creation)

---

## Final Verification

- [ ] All checklist items completed
- [ ] All tests pass
- [ ] No linter errors
- [ ] Documentation updated
- [ ] Code review completed

# Test Status Report — xwsystem

**Library:** exonware-xwsystem  
**Last Updated:** 29-Jan-2026  

---

## Purpose

Provide the current **test status** for `xwsystem` and point to the canonical evidence:
- Detailed run reports: `docs/logs/tests/TEST_*.md`
- Tests index: `docs/logs/tests/INDEX.md`
- Cumulative log: `docs/logs/SUMMARY_TEST.md`

---

## Test suite structure (canonical)

`xwsystem` follows the layered model:

1. `tests/0.core/` — fast, high-value tests (target: <30s)
2. `tests/1.unit/` — component tests
3. `tests/2.integration/` — cross-module scenarios
4. `tests/3.advance/` — excellence tests (Security/Usability/Maintainability/Performance/Extensibility)

---

## How to run

```bash
# All layers (recommended entry point)
python tests/runner.py

# Individual layers
python tests/runner.py --core
python tests/runner.py --unit
python tests/runner.py --integration
python tests/runner.py --advance
```

---

## Latest evidence

- Latest known run report (by filename timestamp): `docs/logs/tests/TEST_20260128_0614_SUMMARY.md`
- Tests index: `docs/logs/tests/INDEX.md`

---

## Coverage status

Current coverage targets (per QA/testing guides):
- Overall ≥ 90%
- Core critical paths: 100%

Supporting internal summary documents:
- `tests/TEST_COVERAGE_SUMMARY.md`

---

## Known gaps / follow-ups

This report is intentionally minimal. The authoritative “why” and history belong in:
- `docs/logs/SUMMARY_TEST.md`
- `docs/changes/CHANGE_*.md`


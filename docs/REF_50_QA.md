# QA Reference — xwsystem

**Library:** exonware-xwsystem  
**Version:** See [`src/exonware/xwsystem/version.py`](../src/exonware/xwsystem/version.py) (`__version__`; single source of truth).  
**Last Updated:** Synchronized with `version.get_date()` from that module at doc review time.  
**Requirements source:** [REF_01_REQ.md](REF_01_REQ.md) sec. 8 (Five Priorities)

---

## Purpose

This is the **single source of truth** for `xwsystem` quality gates and release readiness state.

---

## Readiness state (Go / No-Go)

| Gate | Requirement | Status | Evidence |
|------|-------------|--------|----------|
| Tests | All layers pass (0.core, 1.unit, 2.integration, 3.advance) | ⏳ | `docs/logs/tests/INDEX.md` |
| Coverage | Overall ≥ 90% and core critical paths 100% | ⏳ | `REF_51_TEST.md` |
| Lint/Types | Formatting + type checks pass | ⏳ | CI checks |
| Security | Security suites pass; no known critical vulnerabilities | ⏳ | `docs/logs/tests/INDEX.md` + compliance evidence |
| Performance | Benchmarks meet SLA; no regressions (>5% investigate; >10% block) | ⏳ | `benchmarks/` (project root per GUIDE_54_BENCH) + `REF_54_BENCH.md` |
| Docs | Required docs exist and are current (REF_*, logs indices, usage) | ⏳ | `docs/INDEX.md` + compliance checks |

**Decision:** ⏳ Pending (blocked until gates are green)

---

## Quality gates (canonical)

### Gate 1 — Test execution
- No skipped/rigged tests
- No warning suppression to “make green”
- Evidence recorded under `docs/logs/tests/`

### Gate 2 — Coverage
- Target: ≥ 90% overall
- Core critical paths: 100%

### Gate 3 — Code quality
- Lint errors: 0
- Type-checking errors: 0
- No forbidden pytest opts (`-q`, `--disable-warnings`, `--tb=no`, etc.)

### Gate 4 — Performance
- Benchmarks run and compared to baseline
- Meets SLAs defined in `REF_54_BENCH.md`

### Gate 5 — Security
- Security tests pass
- Input validation tested
- No known critical CVEs in dependencies (per compliance program)

### Gate 6 — Regression prevention
- No breaking changes without migration plan
- Existing tests remain meaningful (no over-mocking / timeouts inflated to hide issues)

---

## Required evidence locations

- Tests: `docs/logs/tests/TEST_*.md` + `docs/logs/tests/INDEX.md`
- Benchmarks: `benchmarks/` at project root (campaign folders) + `REF_54_BENCH.md`; legacy index may exist under `docs/logs/benchmarks/`
- Releases: `docs/logs/releases/` + index
- Compliance: `docs/compliance/**` + traceability matrix

**Release checklist:** The canonical gates above define go/no-go. Historical checklist content was absorbed; use this section as the single source of truth.


# CHANGE_20260129_0429_XWSYSTEM_GUIDE_COMPLIANCE_BASELINE

**Library:** xwsystem (`exonware-xwsystem`)  
**Company:** eXonware.com  
**Author:** eXonware Backend Team  
**Email:** connect@exonware.com  
**Version:** 0.0.1  
**Last Updated:** 29-Jan-2026  

---

## Purpose

Baseline audit of `xwsystem/` against the eXonware guide suite, so the follow-up work can be executed and validated with **hard gates** (CI + local checks) and reach **100% adherence**.

This document is an output of the broader lifecycle work around:
- Phase orchestration and planning
- Documentation placement and mandatory documents
- Root structure, development, and testing standards
- Release and deployment discipline

---

## Snapshot (what exists today)

### Root structure (current)

The current `xwsystem/` root includes (non-exhaustive):
- `.github/`, `.gitignore`, `LICENSE`, `pyproject.toml`, `pytest.ini`, `README.md`, `requirements.txt`, `docs/`, `examples/`, `src/`, `tests/`
- **Also present** (not part of the canonical root whitelist): `BACKUP_BEFORE_OPTIMIZATION/`, `benchmarks/`, `rust/`

### Docs structure (current)

`xwsystem/docs/` currently contains:
- Present REF docs: `REF_12_IDEA.md`, `REF_22_PROJECT.md`, `REF_21_PLAN.md`, `REF_13_ARCH.md`, `REF_15_API.md`, `REF_54_BENCH.md`, `REF_USAGE.md`
- Compliance docs subtree: `docs/compliance/**` (substantial content already exists)
- Test run evidence currently stored under: `docs/tests/TEST_*.md`
- A project-local guide folder: `docs/guides/GUIDE_USAGE_XWSYSTEM.md`
- Many status/fix/victory/ad-hoc reports directly under `docs/` (e.g. `FINAL_VICTORY.md`, `FIX_SUMMARY.md`, etc.)

### Test runner output (current)

`xwsystem/tests/runner.py` explicitly writes Markdown reports to `docs/tests/TEST_<timestamp>_SUMMARY.md` (see its module docstring and `reports_dir = ... / "docs" / "tests"`).

### Packaging config (current)

`xwsystem/pyproject.toml` includes:
- `requires-python = ">=3.12"` (aligned with the Python 3.12 mandate)
- `tool.mypy.python_version = "0.1.0.1"` (**invalid** Python version string; must be corrected)
- pytest settings exist in both `pytest.ini` and `[tool.pytest.ini_options]` (must be harmonized so the project uses one canonical config and follows the project’s testing standards).

---

## Compliance gaps (must be fixed)

### A) Root structure violations

**Observed violations:**
- `xwsystem/BACKUP_BEFORE_OPTIMIZATION/` exists at root (forbidden: “no backup folders”).
- `xwsystem/benchmarks/` exists at root (not part of the allowed root set).
- `xwsystem/rust/` exists at root, but the documented root whitelist did not include `rust/` even though the Rust development guide defines `library-name/rust/` as canonical.

**Required fix direction (per plan):**
- Move backup materials to `xwsystem/docs/_archive/`.
- Relocate benchmarking assets to canonical benchmark locations (prefer `xwsystem/tests/3.advance/benchmarks/` for pytest-driven benchmarks + evidence in `xwsystem/docs/logs/benchmarks/`).
- Harmonize the guides so `rust/` is explicitly permitted for Rust-enabled libraries (then enforce it).

### B) Missing mandatory docs

The following mandatory documents were missing under `xwsystem/docs/` when this baseline was captured:
- `REF_14_DX.md` (DX contract for this library)
- `REF_50_QA.md` (QA gates for this library)
- `GUIDE_USAGE.md` (canonical usage guide; previously content lived in `docs/USAGE_GUIDE.md` and `docs/guides/GUIDE_USAGE_XWSYSTEM.md`)
- `REF_51_TEST.md` (test status artifact)

### C) Missing canonical logs tree

The following required log folders / indices do not exist in `xwsystem/docs/logs/`:
- `docs/logs/tests/INDEX.md`
- `docs/logs/benchmarks/INDEX.md`
- `docs/logs/releases/INDEX.md`
- `docs/logs/feedback/INDEX.md`
- `docs/logs/project/INDEX.md`
- `docs/logs/SUMMARY_CHANGE.md`
- `docs/logs/SUMMARY_PROJECT.md`

Additionally, test run evidence currently lives in `docs/tests/` and must be migrated into `docs/logs/tests/` (with an index).

### D) Documentation placement policy violation

`xwsystem/docs/guides/` existed and contained `GUIDE_USAGE_XWSYSTEM.md`.
Per the documentation architecture, methodology guides are centralized; project outputs should be `REF_*` and `docs/logs/**` artifacts, plus `docs/GUIDE_USAGE.md` (usage guide) — not project-local copies of guides.

### E) Naming & consolidation drift inside `xwsystem/docs/`

Many documents under `xwsystem/docs/` appear to be:
- One-off session summaries / victory reports / status updates
- Not in canonical locations (`docs/logs/changes/`, `docs/_archive/`, `docs/logs/**`)

These must be consolidated and/or archived using the standard mapping rules (status into `REF_22_PROJECT.md#project-status-overview`, change details into `docs/logs/changes/`, historic artifacts into `docs/_archive/`).

### F) Test evidence location mismatch

Current behavior:
- `xwsystem/tests/runner.py` writes Markdown to `docs/tests/`.

Required behavior (per plan / canonical logs model):
- Only orchestrator runner writes Markdown evidence under `xwsystem/docs/logs/tests/TEST_*.md` and updates `xwsystem/docs/logs/tests/INDEX.md`.

### G) Tooling configuration mismatch (hard correctness issue)

`xwsystem/pyproject.toml` contains:
- `tool.mypy.python_version = "0.1.0.1"` which is not a Python version string and must be corrected to align with Python 3.12+ requirements.

---

## Immediate next actions (execution order)

1. Harmonize contradictions in the global standards that block enforcement for this library (root whitelist vs Rust, and test examples vs forbidden flags).\n+2. Normalize `xwsystem/` root: remove forbidden folders from root by migrating into canonical locations.\n+3. Create missing mandatory docs (`REF_14_DX.md`, `REF_50_QA.md`, `GUIDE_USAGE.md`, `REF_51_TEST.md`).\n+4. Create canonical `docs/logs/**` structure and migrate existing evidence.\n+5. Fix hard config errors (mypy python_version) and standardize pytest configuration.\n+6. Add enforcement tooling so the above becomes non-regressible (CI + local compliance command).\n+
---

## Notes

- This baseline is intentionally strict and enumerates violations; it does not attempt to “grandfather” legacy structure. The end-state must be **100% guide-compliant with hard gates**.


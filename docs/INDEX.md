# xwsystem Documentation Index

**Version:** 0.0.1.387  
**Last Updated:** 07-Feb-2026  
**Standard:** GUIDE_00_MASTER, GUIDE_41_DOCS — canonical REF_*, REF_51_TEST, GUIDE_01_*, logs/ (including changes/), compliance/, _archive only.

---

## Start here

- [Main README](../README.md) — Project overview and quick start
- [Documentation README](README.md) — Docs overview

---

## Requirements (source of truth)

| Document | Purpose | Producing guide |
|----------|---------|------------------|
| [REF_01_REQ.md](REF_01_REQ.md) | **Requirements source** (feeds REF_11, REF_12, REF_13, REF_14, REF_15, REF_21, REF_22, etc.); single source of raw and refined requirements | GUIDE_01_REQ |

---

## Reference documentation (REF_*)

| Document | Purpose | Producing guide |
|----------|---------|------------------|
| [REF_11_COMP.md](REF_11_COMP.md) | Compliance stance and standards (from REF_01_REQ sec. 4) | GUIDE_11_COMP |
| [REF_12_IDEA.md](REF_12_IDEA.md) | Idea capture and decisions (from REF_01_REQ) | GUIDE_12_IDEA |
| [REF_22_PROJECT.md](REF_22_PROJECT.md) | Requirements and goals (from REF_01_REQ) | GUIDE_22_PROJECT |
| [REF_21_PLAN.md](REF_21_PLAN.md) | Lifecycle and planning templates | GUIDE_21_PLAN |
| [REF_13_ARCH.md](REF_13_ARCH.md) | Architecture and design (from REF_01_REQ) | GUIDE_13_ARCH |
| [REF_14_DX.md](REF_14_DX.md) | DX contract | GUIDE_14_DX |
| [REF_15_API.md](REF_15_API.md) | API reference (from REF_01_REQ sec. 6) | GUIDE_15_API |
| [REF_50_QA.md](REF_50_QA.md) | Quality gates and readiness (from REF_01_REQ sec. 8) | GUIDE_50_QA |
| [REF_54_BENCH.md](REF_54_BENCH.md) | Performance SLAs (from REF_01_REQ sec. 8) | GUIDE_54_BENCH |
| [REF_35_REVIEW.md](REF_35_REVIEW.md) | Review summary (optional) | GUIDE_35_REVIEW |

---

## Reports and usage

| Document | Purpose | Producing guide |
|----------|---------|------------------|
| [REF_51_TEST.md](REF_51_TEST.md) | Test status and coverage (from REF_01_REQ sec. 8) | GUIDE_51_TEST |
| [GUIDE_01_USAGE.md](GUIDE_01_USAGE.md) | How to use this library | Project (GUIDE_41_DOCS) |

**Project status:** [REF_22_PROJECT.md#project-status-overview](REF_22_PROJECT.md#project-status-overview)

---

## Change documentation

- **[logs/changes/](logs/changes/INDEX.md)** — Implementation change logs (`CHANGE_YYYYMMDD_HHMM_DESCRIPTION.md`). Per GUIDE_00_MASTER, change evidence lives under `docs/logs/`.

---

## Logs (evidence)

Per GUIDE_00_MASTER, evidence lives under `docs/logs/`:

- `logs/changes/` — CHANGE_*.md + INDEX (implementation change logs)
- `logs/plans/` — PLAN_*.md (GUIDE_21_PLAN)
- `logs/tests/` — TEST_*.md + INDEX (GUIDE_51_TEST)
- `logs/benchmarks/` — BENCH_*.md + INDEX (GUIDE_54_BENCH)
- `logs/releases/` — RELEASE_*.md + INDEX (GUIDE_61_DEP)
- `logs/feedback/` — feedback + INDEX (GUIDE_62_FEED)
- `logs/reviews/` — REVIEW_*.md + optional INDEX (GUIDE_35_REVIEW); [REVIEW_20260207_PROJECT_STATUS.md](logs/reviews/REVIEW_20260207_PROJECT_STATUS.md) — project/requirements review

Create these folders and indexes as evidence is produced.

---

## Compliance

- **[compliance/](compliance/)** — Standards alignment, traceability, risk, verification (GUIDE_11_COMP)

---

## Archive

- **[_archive/](_archive/README.md)** — Cleared; value absorbed into REF_*, GUIDE_01_USAGE, logs/

---

## Quick links

| Goal | Document |
|------|----------|
| Use the library | [GUIDE_01_USAGE.md](GUIDE_01_USAGE.md) |
| Requirements and status | [REF_01_REQ.md](REF_01_REQ.md), [REF_22_PROJECT.md](REF_22_PROJECT.md), [REF_22_PROJECT.md#project-status-overview](REF_22_PROJECT.md#project-status-overview) |
| API and architecture | [REF_15_API.md](REF_15_API.md), [REF_13_ARCH.md](REF_13_ARCH.md) |
| Planning and ideas | [REF_21_PLAN.md](REF_21_PLAN.md), [REF_12_IDEA.md](REF_12_IDEA.md) |
| Quality and performance | [REF_50_QA.md](REF_50_QA.md), [REF_54_BENCH.md](REF_54_BENCH.md), [REF_51_TEST.md](REF_51_TEST.md) |

---

*Layout follows GUIDE_00_MASTER and GUIDE_41_DOCS. No other doc types at docs root.*

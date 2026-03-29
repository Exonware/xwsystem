# Verification & Validation (V&V)

**Purpose:** Provide documented evidence that requirements are verified and validated to NASA-STD-8739.8, ECSS-Q-ST-80C, DO-178C, and related standards.  
**Applicability:** All core libraries and safety-critical services; mandatory from v2 onwards.  
**Last status review:** 08-Nov-2025 00:00 UTC - ✅ Process documented, component scope defined.

---

## Directory Layout

```
docs/compliance/verification/
+-- vv_plan.md           # Master V&V plan
+-- vv_procedures.md     # Procedures, methods, roles
+-- vv_evidence.md       # Evidence catalogue and storage links
+-- vv_metrics.md        # Coverage, quality, cost metrics
+-- vv_reports.md        # Periodic V&V reports and sign-off forms
+-- vv_tools.md          # Tooling inventory and automation configs
```

Populate each file with current information; link to automated outputs (pytest reports, coverage, benchmarks, formal verification results).

---

## V&V Lifecycle

1. **Planning** - Define scope, resources, schedule, risk considerations.  
2. **Execution** - Run verification (requirements/design/code/integration/system) and validation (functional, performance, security, safety).  
3. **Reporting** - Collect evidence, analyse results, produce sign-off packages, recommend improvements.

Methods include static analysis, dynamic testing, formal verification, and compliance-specific evaluations.

---

## Integration Points

- **GUIDE_TEST.md:** Drives testing hierarchy and tooling.  
- **GUIDE_DEV.md:** Ensures coding and documentation practices support verification.  
- **GUIDE_ARCH.md:** Aligns architecture decisions with verification strategy.  
- **GUIDE_COMP.md:** Maps V&V evidence to compliance controls.

---

## Evidence Management

- Requirement coverage matrices (link to traceability automation).  
- Test reports (unit/integration/advance/compliance).  
- Benchmark outputs and performance validations.  
- Formal verification artefacts and safety analyses.  
- Audit logs and review minutes.

Store raw artefacts beneath `docs/compliance/evidence/` and reference them from `vv_evidence.md`.

---

## Formal Verification Scope (ECSS-E-ST-70-41C / DO-178C sec. 6.4.4)

| Component | Classification | Property Focus | Verification Path |
|-----------|----------------|----------------|-------------------|
| `xwsystem.memory.MemoryMonitor` | Safety-critical | Bounded allocation, leak prevention | Property-based tests (`tests/formal_verification/`), static analysis |
| `xwsystem.crypto.CryptoManager` | Safety-critical | Deterministic output, key lifecycle | Model checking, Hypothesis |
| `xwsystem.fs.AtomicFile` | Safety-critical | Atomic writes, rollback safety | Theorem proving (Z3), concurrency stress |
| `xwsystem.validation.DataValidator` | Safety-critical | Input invariants, sanitisation | Property-based generators, fuzz campaigns |
| `xwsystem.async.TypeSafety` | Safety-critical | Thread-safety, lifecycle invariants | Liveness checks, race detectors |

- Components align with the experiment reference (`experiments/250902-mars-standard/tests/formal_verification/README.md`).  
- Formal verification tasks must be tracked via PLAN documents with metadata (see GUIDE_PLAN).  
- Proof artefacts and scripts reside in `tests/formal_verification/` alongside automated Hypothesis suites.

## Test Categories (derived from experiment playbooks)

- **Property-Based** - Hypothesis suites for data structures and serialization (Level C).  
- **Model Checking** - Scenario exploration using custom state machines (Level B).  
- **Static Analysis** - Extended `mypy`/lint rules plus safety-specific Semgrep checks (Level B).  
- **Theorem Proving** - Z3 scripts validating invariants for safety modules (Level A).  
- **Dynamic & Benchmarks** - Reuse benchmarking and compliance suites to validate performance and safety assumptions.

## Verification Schedule

| Phase | Activity | Artefact |
|-------|----------|----------|
| Planning | PLAN document annotated with requirement IDs and verification strategy | `docs/logs/plans/PLAN_*.md` |
| Implementation | CHANGE doc summarising code and test updates | `docs/logs/changes/CHANGE_*.md` |
| Execution | Test runners, benchmark reports, formal proofs | `tests/` hierarchy, `docs/benchmarks/` |
| Reporting | Traceability matrix row, metrics update, release checklist | `docs/compliance/traceability/TRACE_MATRIX.md`, `vv_metrics.md`, `REF_21_PLAN.md` |

## Evidence Links

- Traceability workflow (`.github/workflows/traceability.yml`) enforces up-to-date matrices.  
- Risk register (`docs/compliance/risk-assessment/README.md`) captures verification-related risks.  
- Store formal proof outputs in `docs/compliance/evidence/verification/` (e.g., solver logs, Hypothesis statistics).

## Historical Context

Earlier V&V documentation (December 2023) is available through repository history and informs the component list above.


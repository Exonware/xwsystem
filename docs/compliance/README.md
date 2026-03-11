# Compliance Overview

**Applies from:** Version 0.x (structure) � **Mandatory from:** Version 2.x (Mars Standard Draft)  
**Owner:** Architecture & Compliance Group

---

## Purpose

Establish the central location for compliance artifacts that support the eXonware Mars Standard roadmap. This directory aggregates requirements, gap analyses, risk assessments, evidence, and traceability across all target standards.

---

## Structure

```
docs/compliance/
+-- mars_standard/        # Roadmap and milestone status
+-- standards/            # Per-standard packages (NASA, ECSS, DO-178C, ISO, NIST, AI, ...)
+-- safety/               # Safety case, analysis, and monitoring artefacts
+-- gap-analysis/         # Cross-standard gap matrices
+-- risk-assessment/      # Harmonised risk models
+-- verification/         # V&V plans and execution evidence
+-- evidence/             # Stored reports, audits, certifications
```

Repositories derived from xwsystem must either mirror this structure or link back to these canonical packages.

---

## Current Compliance Status (08-Nov-2025 00:00 UTC)

| Standard Category | Status | Evidence Available | Notes |
|-------------------|--------|--------------------|-------|
| NASA (8719.13/8739.7/8739.8, NPR 7150.2D/8715.3) | ❌ Not compliant | Requirements register, GUIDE_DEV references | Safety case, IV&V evidence, and traceability to be produced. |
| ECSS (E/Q/ST series) | ✅ Compliant | `docs/guides/GUIDE_PLAN.md`, `docs/REF_21_PLAN.md`, `docs/compliance/traceability/TRACE_MATRIX.md`, `docs/compliance/risk-assessment/README.md`, `docs/compliance/verification/README.md` | Lifecycle, traceability, dependability, and formal verification scopes documented. Workflow enforces trace matrix freshness. |
| DO-178C Level A | ❌ Not compliant | None | DAL assignments, requirements-based testing, QA audits, and CM evidence not produced. |
| ISO/IEC 27k / 9001 / lifecycle | ❌ Not started | None | Packages scaffolded but no controls, policies, or assessments documented. |
| NIST SP 800 series | ❌ Not started | None | Control mapping and risk framework not yet authored. |
| AI Governance (NIST RMF, ISO 23894/42001, EU AI Act) | ❌ Not started | None | AI governance programme pending. |

This table reflects the latest review completed on 08-Nov-2025.

> **Current focus:** During v0/v1 we emphasise guides, references, and logs (GUIDE_*/REF_*/CHANGE_/PLAN_) as the primary evidence. Heavy automation (trace exports, ISO/NIST tooling) is scheduled for the v2 compliance sprint.

---

## Standards Coverage

| Domain            | Standards (examples)                                                                 | v0.x | v1.x | v2.x | v3.x | v4.x |
|-------------------|---------------------------------------------------------------------------------------|------|------|------|------|------|
| Space & Safety    | NASA-STD-8719.13/8739.7/8739.8, NASA NPR 7150.2D & 8715.3, ECSS-E/Q/ST, DO-178C, CCSDS | ⚠️ | ⚠️ | ✅ | ✅ | ✅ |
| Cybersecurity     | ISO/IEC 27001, 27002, 27017, 27018, ISO/IEC 9001, ISO/IEC 12207/15288/15504            | ❌ | ⚠️ | ✅ | ✅ | ✅ |
| NIST Controls     | NIST SP 800-53, 800-63, 800-160, 800-37                                               | ❌ | ❌ | ✅ | ✅ | ✅ |
| AI Governance     | NIST AI RMF 1.0, ISO/IEC 23894, ISO/IEC 42001, EU AI Act, IEEE 7000-series            | ❌ | ⚠️ | ✅ | ✅ | ✅ |

Legend: ✅ implemented • ⚠️ partial/in progress • ❌ not started  
Current status (v0.x): ECSS documentation complete; remaining standards will advance during the v2 compliance sprint.

---

## Immediate Tasks (v2 readiness)

1. Populate NASA/ECSS/DO-178C packages with requirement registers, gap analyses, and evidence.  
2. Author ISO/IEC and NIST requirement summaries, gap analyses, and action plans.  
3. Build AI compliance packages (policy templates, risk forms, evidence requirements).  
4. Implement traceability tooling (automated requirement ? code ? test mapping).  
5. Populate `mars_standard/README.md` with milestone tracking across all standards.

---

## References

- [GUIDE_COMP.md](../guides/GUIDE_COMP.md) � Compliance playbook  
- [GUIDE_ARCH.md](../guides/GUIDE_ARCH.md) � Architecture roadmap and compliance integration  
- [GUIDE_DEV.md](../guides/GUIDE_DEV.md) � Developer execution standards  
- Historical assessments (December 2023) are available via repository history for reference.

---

*When standards evolve or new certifications are targeted, update this directory and notify Architecture & Compliance stakeholders.*


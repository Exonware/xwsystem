# ECSS Standards Compliance

**Scope:** European Cooperation for Space Standardization (software engineering, product assurance, formal methods, dependability, data exchange).  
**Applicability:** All core libraries and mission-critical services; mandatory from v2 onwards.  
**Priority:** High.  
**Last status review:** 08-Nov-2025 00:00 UTC — ✅ Fully documented (pending operational evidence tracked via PLAN metadata).  
> Compliance is satisfied with the current guides, references, and logs. Additional automation (expanded trace exports, tooling) will arrive during the v2 programme.

---

## Requirements Register (initial)

| ID | Requirement | Owner | Implementation Artifact | Evidence | Status |
|----|-------------|-------|--------------------------|---------|--------|
| ECSS-E-40C-RQ-001 | Align development lifecycle with ECSS processes | Architecture | GUIDE_PLAN / REF_PLAN / GUIDE_ARCH | `../../guides/GUIDE_PLAN.md`, `../../REF_21_PLAN.md` | ✅ |
| ECSS-E-40C-RQ-002 | Maintain requirements/design/code traceability | Compliance | Trace matrix generator | `../traceability/TRACE_MATRIX.md`, `.github/workflows/traceability.yml` | ✅ |
| ECSS-E-70-41C-RQ-001 | Apply formal methods to critical modules | Architecture | verification/vv_plan.md | `../verification/README.md#formal-verification-scope` | ✅ |
| ECSS-Q-80C-RQ-001 | Establish product assurance plan & metrics | QA | verification/vv_metrics.md | `../verification/README.md`, `../evidence/verification/` | ✅ |
| ECSS-Q-80-02C-RQ-001 | Document dependability assessment | Compliance | risk-assessment/README.md | `../risk-assessment/README.md` | ✅ |
| ECSS-M-20C-RQ-001 | Ensure data exchange compliance (incl. CCSDS) | Data Team | serialization design docs | `../ccsds/README.md` | ✅ |

_Status legend:_ ✅ complete • ⚠️ in progress • ❌ not started.

---

## Gap Analysis

The 08-Nov-2025 review closed prior ECSS gaps. Outstanding work is limited to keeping artefacts current:

- **Lifecycle alignment:** Controlled via GUIDE_PLAN + REF_PLAN; update when phases change.  
- **Traceability:** Workflow fails builds if TRACE_MATRIX.md diverges.  
- **Formal verification:** Component scope stored in `docs/compliance/verification/README.md`; progress tracked via PLAN documents.  
- **Dependability:** Risk register captures acceptance decisions (see risk checklist).  
- **Data exchange:** Map new codecs/exporters in CCSDS README as they land.

---

## Integration Points

- Architecture: Development flow (GUIDE_PLAN + REF_PLAN + GUIDE_ARCH) must reference ECSS lifecycle expectations.  
- Development: Coding standards and testing structure already align; ensure documentation and assurance evidence are linked.  
- Testing: Expand `tests/formal_verification/` and `tests/safety_critical/` to output ECSS-compliant reports.

---

## Evidence

- Store ECSS-related artefacts under `docs/compliance/evidence/ecss/`.

---

## Notes

- Track ECSS revisions and note version numbers here.  
- Coordinate effort with NASA and DO-178C compliance teams to avoid duplicate tasks.


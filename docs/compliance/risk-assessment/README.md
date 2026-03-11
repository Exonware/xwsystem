# Risk Assessment & Management

**Purpose:** Maintain a harmonised risk management framework covering NASA NPR 8715.3, ECSS risk requirements, ISO/IEC 27001, and NIST RMF obligations.  
**Applicability:** All eXonware products starting v2 (Mars Standard draft) with optional early adoption in v1.  
**Last status review:** 08-Nov-2025 00:00 UTC — ✅ Framework documented, dependability checklist in place.

---

## Directory Layout

The canonical risk register lives in this README to minimise document sprawl.  
If richer exports are required, create CSV snapshots in `docs/compliance/evidence/risk/`.

---

## Risk Management Lifecycle

1. **Identify** – Workshops, checklists, historical data, and expert reviews.  
2. **Analyse** – Score probability (1–5) and impact (1–5), derive risk score, assign owner.  
3. **Plan** – Define avoidance, reduction, transfer, or acceptance strategies plus contingency plans.  
4. **Mitigate** – Execute actions, allocate resources, integrate with delivery plans.  
5. **Monitor** – Track indicators, trend risks, run weekly/monthly/quarterly reviews.  
6. **Communicate** – Share status with stakeholders, log updates in CHANGE/PROJECT docs.

Risk levels follow a four-colour scale (Critical, High, Medium, Low). Accepted risks require explicit approval and review dates.

---

## Integration Points

- **Architecture (GUIDE_ARCH):** Document risk considerations in ADRs and module blueprints.  
- **Development (GUIDE_DEV):** Enforce mitigations via coding standards, testing, and tooling.  
- **Compliance (GUIDE_COMP):** Link each risk to standards and evidence.  
- **Operations:** Feed incidents and monitoring data into risk register updates.

---

## ECSS-Q-ST-80-02C Dependability Checklist

| Area | Questions (aligned with `experiments/250902-mars-standard/tests/compliance/README.md`) | Status | Evidence |
|------|-----------------------------------------------------------------------------------------|--------|----------|
| Fault avoidance | Are safety-critical modules identified and reviewed? | ✅ | `docs/compliance/standards/ecss/README.md` |
| Fault tolerance | Do recovery procedures exist for each critical component? | ⚠️ | Link relevant plan once created |
| Fault detection | Is automated monitoring in place for anomalies? | ⚠️ | Add monitoring task to next PLAN_* document |
| Risk acceptance | Are accepted risks documented with review dates? | ⚠️ | Track in table below |

Use this checklist whenever a new PLAN document is written. Populate the “Status” and “Evidence” columns by referencing CHANGE/PLAN artefacts.

## Consolidated Risk Register

| ID | Risk Description | Probability | Impact | Risk Score | Owner | Mitigation / Contingency | Status | Review Date |
|----|------------------|-------------|--------|------------|-------|--------------------------|--------|-------------|
| RISK-001 | Trace matrix automation not deployed before ECSS audit | 3 | 4 | 12 (High) | Compliance Lead | Implement `tools/ci/generate_trace_matrix.py` workflow; include in release checklist | In progress | 2025-12-15 |
| RISK-002 | Dependability assessment lacks operational data | 2 | 4 | 8 (Medium) | Architecture | Capture post-mortem summaries in CHANGE docs; add monitoring KPIs to next PLAN | Open | 2026-01-15 |
| RISK-003 | Formal verification scope undefined for safety components | 3 | 5 | 15 (Critical) | Verification | Add component list to `docs/compliance/verification/README.md` and track proofs in PLAN | Mitigated | 2025-11-30 |

### Risk Matrix

| Impact \\ Probability | 1 | 2 | 3 | 4 | 5 |
|-----------------------|---|---|---|---|---|
| **5 – Critical**      | 5 | 10 | 15 | 20 | 25 |
| **4 – Major**         | 4 | 8  | 12 | 16 | 20 |
| **3 – Moderate**      | 3 | 6  | 9  | 12 | 15 |
| **2 – Minor**         | 2 | 4  | 6  | 8  | 10 |
| **1 – Negligible**    | 1 | 2  | 3  | 4  | 5 |

Scores ≥15 are **Critical**, 10–14 **High**, 6–9 **Medium**, and ≤5 **Low**. Critical and High risks require dedicated tasks in the next PLAN document and explicit approval for any acceptance decisions.

## Monitoring & Reporting

- **Weekly:** Update the risk register table when new items are discovered.  
- **Monthly:** Review mitigation progress and adjust probability/impact scores.  
- **Quarterly:** Export the table to CSV (store in `docs/compliance/evidence/risk/`) and attach to CHANGE or PROJECT summaries.  
- **On Release:** Verify that all Critical/High risks have mitigation plans recorded in the release checklist (see `REF_21_PLAN.md`).

Historical risk assessments from December 2023 remain in repository history for reference. Use them as input when seeding future risk entries.


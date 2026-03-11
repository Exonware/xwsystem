# Compliance Reference — xwsystem (REF_11_COMP)

**Library:** exonware-xwsystem  
**Last Updated:** 07-Feb-2026  
**Requirements source:** [REF_01_REQ.md](REF_01_REQ.md) sec. 4  
**Producing guide:** [GUIDE_11_COMP.md](../../docs/guides/GUIDE_11_COMP.md)

---

## Purpose

Compliance stance and standards for xwsystem. Filled from REF_01_REQ sec. 4. Per GUIDE_11_COMP, compliance packages and evidence live under `docs/compliance/` when applicable.

---

## Current stance (from REF_01_REQ sec. 4)

| Area | Stance |
|------|--------|
| **Regulatory/standards** | OWASP Top 10 alignment; resource limits and secure codecs; no code execution from untrusted data; path traversal protection; input validation. Mars Standard / NASA-style traceability and compliance evidence in docs/compliance where applicable. Industry norms for serialization (e.g. safe XML with defusedxml, forbid_dtd). |
| **Security & privacy** | Auth and secrets handled by callers (xwauth) or app; xwsystem provides crypto helpers, path validation, and secure defaults. No PII stored in xwsystem; audit logging and classification are app-level. |
| **Certifications/evidence** | SOC2 or similar only if required by production use or government contracts; for now no formal cert. Compliance gap-analysis and risk-assessment under docs/compliance. |

---

## Mars Standard alignment

- **Current:** OWASP and secure-defaults in place; MARS/NASA-style traceability and evidence in docs/compliance where applicable.
- **Later:** Formal MARS implementation when required; gap-analysis and risk-assessment per GUIDE_11_COMP.

---

## Traceability

- **Requirements:** [REF_01_REQ.md](REF_01_REQ.md) sec. 4  
- **Architecture:** [REF_13_ARCH.md](REF_13_ARCH.md)  
- **Planning:** [REF_21_PLAN.md](REF_21_PLAN.md)

---

*Per GUIDE_11_COMP. Compliance packages and evidence under docs/compliance/ per the guide.*

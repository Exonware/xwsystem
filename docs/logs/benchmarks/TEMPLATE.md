# Benchmark Report Template

> Replace bracketed sections with project-specific details. Remove any guidance comments once completed.

---

## Metadata

- **Benchmark ID:** BENCH_YYYYMMDD_HHMM_NAME
- **Author:** [Name]
- **Date:** YYYY-MM-DD
- **Related Change/Plan:** [Link to change or plan]
- **Summary:** 1–2 sentence overview of the measurement objective.

---

## Purpose

Describe the motivation for this benchmark. Reference the relevant SLA or NFR in [../../REF_BENCH.md](../../REF_BENCH.md).

---

## Methodology

| Component | Details |
|-----------|---------|
| Scenario | What is being measured |
| Dataset / Baseline | Location inside aseline/ or external source |
| Environment | Hardware, OS, configuration |
| Tooling | pytest-benchmark, custom script, etc. |
| Repeat Count | Number of iterations / runs |

---

## Results

Present raw measurements (tables, charts, bullet points). Highlight deviations from expected baselines.

---

## Analysis

Interpret the data. Discuss variance, anomalies, and potential risks. Compare with prior runs, referencing [../INDEX.md](../INDEX.md).

---

## Actions

- [ ] Update [../INDEX.md](../INDEX.md)
- [ ] Open follow-up plan/change (if required)
- [ ] Notify stakeholders (if SLA breached)

---

## Attachments

List any supplementary files, datasets, or notebooks.

---

*Once complete, remove unused sections and ensure all checkboxes are updated.*


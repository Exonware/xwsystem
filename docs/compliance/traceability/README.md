# Traceability Artefacts

This folder stores generated traceability outputs that link planning documents to
implementation and verification evidence. Artefacts are produced by
`tools/ci/generate_trace_matrix.py` and kept under version control so auditors
can review historical records without rerunning the tooling.

## Workflow

- Plans live under `docs/logs/plans/`. Add metadata lines such as:
  - `Requirement ID: ECSS-E-40C-RQ-001`
  - `Components: xwsystem.xwdata`
  - `Verification: tests/2.integration/test_data_flow.py`
- The traceability generator parses those lines and writes:
  - `TRACE_MATRIX.md` - a human-readable Markdown table.
  - `TRACE_MATRIX.json` - machine-readable JSON for integrations.
- A GitHub Actions workflow (`.github/workflows/traceability.yml`) runs the generator on every
  push and pull request, publishing the Markdown and JSON as build artefacts.

> **Note:** Traceability automation becomes mandatory in v2. Until then, it is acceptable to keep plans, guides, and logs linked manually-run the generator only when you have metadata available.

## Updating the Matrix Locally

```bash
python tools/ci/generate_trace_matrix.py
```

Pass `--package` to target additional repositories when needed:

```bash
python tools/ci/generate_trace_matrix.py --package xwsystem --package xwnode
```

The command re-creates both the Markdown and JSON outputs inside this
directory. Commit the regenerated files alongside any planning updates so the
documentation remains consistent with the codebase.


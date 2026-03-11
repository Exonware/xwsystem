# xwsystem Test Skip Report

Generated from: `pytest tests -v --tb=no -rs`

**Summary:** 1820 collected, 1303 passed, 42 skipped, 1 failed (fixed).

---

## 1. Intentional / by design (format or API capability)

| Count | File | Reason |
|------:|------|--------|
| 6 | `tests/0.core/io/test_core_serialization_fixed_features.py` | **XML/TOML/YAML do not support path-based operations or path-based updates.** Only JSON is tested for `test_partial_access_operations` and `test_patch_application` for those formats. This is a format limitation, not a missing test. |
| 10 | `tests/1.unit/serialization_tests/test_serialization_basic_features.py` | **Enterprise-only features.** Tests skip when run against individual format serializers (JsonSerializer, etc.) because the test expects a unified/facade API: sniff_format, get_at/set_at, apply_patch, validate_schema, canonicalize, batch streaming, loads_typed, checksums, type adapters, versioning, context manager. Those are documented as "enterprise feature" – coverage lives in enterprise/unified tests. |
| 1 | `tests/1.unit/security_tests/test_path_validator.py` | **Platform:** "Unix system path test skipped on non-Unix system". Correct on Windows. |

**Action:** None required unless you want path-based or enterprise behavior on per-format serializers.

---

## 2. Environment / optional dependencies

| Count | File | Reason |
|------:|------|--------|
| 3 | `tests/1.unit/io_tests/archive_tests/test_all_archive_formats_roundtrip.py` | **Optional archive tools:** ZPAQ, wimlib, squashfs not installed. Install pyzpaq, wimlib-imagex, or squashfs-tools to run these. |
| 9 | `tests/1.unit/security_tests/test_crypto_comprehensive.py` | **"cryptography library not available".** Symmetric/asymmetric encryption tests skip if the `cryptography` import fails. Ensure `pip install cryptography` and that the same interpreter sees it. |
| 2 | `tests/1.unit/io_tests/file_tests/test_conversion.py` | **"Codecs not available" / "File conversion not available".** Depends on codec or conversion setup. |
| 2 | `tests/1.unit/serialization_tests/test_all_serializers.py` | **DBM:** "DBM requires file-based operations - use save_file()". **Shelve:** assertion on file_extensions (`.shelf` vs `.shelve`/`.db`). Platform/serializer-specific. |

**Action:** Install optional deps (e.g. `cryptography`, pyzpaq, codecs) if you want these tests to run. Crypto tests are important for security; others are optional formats.

---

## 3. Stale or outdated tests (removed/renamed features)

| Count | File | Reason |
|------:|------|--------|
| 2 | `tests/0.core/utils/test_auto_install.py` | **"xwlazy has been removed from the codebase"** and **"AvroSerializer not available: No module named 'exonware.xwsystem.serialization'".** Tests reference removed or moved code. |
| 6 | `tests/0.core/utils/test_core_lazy_loading.py` | **"xwlazy has been removed from the codebase".** All six tests skip. |

**Action:** Update or remove these tests. If lazy loading or Avro live elsewhere, point the tests at the new module/API; otherwise remove or mark as expected-fail/skip with a clear reason.

---

## 4. Summary table

| Category | Skip count | Action |
|----------|------------|--------|
| By design (format/API) | 17 | Optional: add path-based/enterprise tests if you want that coverage on format serializers. |
| Environment / optional deps | 16 | Install cryptography (and optional tools) to run crypto and archive/conversion tests. |
| Stale tests (xwlazy / Avro) | 8 | Update or remove tests; fix Avro import path if still supported. |
| **Total** | **42** | |

---

## 5. Important tests that are currently skipped

- **Crypto (9 skips):** `test_crypto_comprehensive.py` – key generation, encrypt/decrypt, sign/verify. **Important for security.** Fix by ensuring `cryptography` is installed and importable.
- **Lazy loading (6 + 2 skips):** `test_core_lazy_loading.py` and part of `test_auto_install.py` – **important if you still support lazy loading.** If the feature was removed, delete or relocate tests; if it moved, update imports and skip reasons.
- **Enterprise-style serialization (10 skips):** `test_serialization_basic_features.py` – **important if you care about unified/enterprise API.** Run equivalent tests against the facade/unified serializer so these features are covered.

---

## 6. Single failure (fixed)

- **test_serialization_worst_case_scenarios.py::TestSerializationWorstCaseScenarios::test_malformed_json_handling**  
  **Cause:** Test called `loads_text()`, but format-specific `JsonSerializer` only exposes `loads()`.  
  **Fix:** Use `loads(malformed)` instead of `loads_text(malformed)` (and same for the valid_complex branch).  
  After this change, a full run can complete without that failure.

---

## 7. Re-run and re-check skips

```powershell
cd "d:\OneDrive\DEV\exonware\xwsystem"
$env:PYTHONPATH = "d:\OneDrive\DEV\exonware\xwsystem\src"
py -m pytest tests -v --tb=no -rs
```

To run without stopping at first failure (and get full skip list):

```powershell
py -m pytest tests -v --tb=no -rs --no-header -q 2>&1 | Select-String "SKIPPED|skipped|failed|passed"
```

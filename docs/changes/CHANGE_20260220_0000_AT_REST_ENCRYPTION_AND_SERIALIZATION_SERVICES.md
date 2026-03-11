# Feature — At-Rest Encryption and Serialization Pipeline Services

**Library:** exonware-xwsystem  
**Version:** 0.0.1  
**Date:** 20-Feb-2026 00:00  
**Author:** eXonware Backend Team

---

## Overview

Added unified at-rest encryption (IAtRestEncryption) with multiple implementations (AES-256-GCM, XChaCha20-Poly1305, Fernet), a common XWJE envelope format, and optional Argon2id KDF. Introduced serialization pipeline services (Encryption, Archive, Binary) so any format—including XWJSON—can use encryption, compression, and binary framing via save_file/load_file options without implementing them per-format. Aligns with GUIDE_00_MASTER (Five Priorities: Security first), GUIDE_41_DOCS (REF_15_API, REF_54_BENCH), and the XW Encryption and XWJSON Protection Plan.

**Change Type:** Feature  
**Impact Level:** High

---

## Motivation

### Problem Statement

- Need to protect XWJSON (and other serialized) data at rest so that a copy of the file is unreadable without the key.
- Encryption, compression, and binary handling should be reusable across all serialization formats, not duplicated in each.

### Goals

1. Single security interface and envelope format for all at-rest encryption strategies (swappable, benchmarkable).
2. Composable pipeline services (encryption, archive, binary) in xwsystem serialization; XWJSON and other formats consume them via options.
3. Benchmark encryption strategies (throughput, latency, safety ranking) and document results.

### Non-Goals

- Dual-file XWJSON encryption (deferred to later).
- Rust-backed encryption backend (optional future).

---

## Changes Made

### Code Changes (xwsystem)

**Files Added:**
- `src/exonware/xwsystem/security/at_rest.py` — IAtRestEncryption implementations, envelope build/parse, AES256GCMAtRest, XChaCha20Poly1305AtRest, FernetAtRest, get_at_rest_encryption.
- `src/exonware/xwsystem/security/kdf.py` — derive_key_pbkdf2, derive_key_argon2id, derive_key_from_password (optional Argon2id).
- `src/exonware/xwsystem/io/serialization/services/__init__.py` — exports for pipeline services.
- `src/exonware/xwsystem/io/serialization/services/encryption.py` — EncryptionService (bytes ↔ envelope).
- `src/exonware/xwsystem/io/serialization/services/archive.py` — ArchiveService (gzip/zst/lz4).
- `src/exonware/xwsystem/io/serialization/services/binary.py` — BinaryService (length-prefix framing).
- `src/exonware/xwsystem/io/serialization/services/pipeline.py` — apply_pipeline_save, apply_pipeline_load.
- `benchmarks/encryption_benchmark.py` — throughput and latency benchmark for at-rest implementations.
- `tests/1.unit/security_tests/test_at_rest.py` — unit tests for at-rest encryption.

**Files Modified:**
- `src/exonware/xwsystem/security/contracts.py` — added IAtRestEncryption protocol.
- `src/exonware/xwsystem/security/__init__.py` — exports for at_rest, kdf.
- `src/exonware/xwsystem/io/serialization/base.py` — save_file/load_file call pipeline when options include encryption/archive/binary_framing.
- `src/exonware/xwsystem/io/serialization/__init__.py` — exports for services and pipeline.

### Code Changes (xwjson)

**Files Modified:**
- `src/exonware/xwjson/formats/binary/xwjson/serializer.py` — save_file, async_save_file, load_file apply pipeline when key/password/encryption/archive/binary_framing present; single-file when pipeline used.
- `src/exonware/xwjson/__init__.py` — added is_encrypted(bytes_or_path).

### Documentation

- `docs/REF_15_API.md` — new sections: Security — At-Rest Encryption; Serialization — Pipeline Services (per GUIDE_15_API).
- `docs/REF_54_BENCH.md` — new subsection: At-Rest Encryption (script, results link, safety table) per GUIDE_54_BENCH.
- `docs/encryption_benchmark_results.md` — benchmark output (throughput, latency, safety ranking).

---

## References

- Plan: XW Encryption and XWJSON Protection Plan (cursor plans).
- Guides: GUIDE_00_MASTER, GUIDE_15_API, GUIDE_41_DOCS, GUIDE_51_TEST, GUIDE_54_BENCH, GUIDE_64_SECURITY.

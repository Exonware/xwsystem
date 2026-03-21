#!/usr/bin/env python3
"""
Encryption benchmark: throughput (MB/s) and latency for IAtRestEncryption implementations.
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Run after all at-rest strategies are implemented. Outputs fastest→slowest and safety tier table.
Usage: PYTHONPATH=src python "benchmarks/20260321-benchmark xwsystem encryption/scripts/encryption_benchmark.py" [--output docs/encryption_benchmark_results.md]
"""

from __future__ import annotations
import argparse
import os
import sys
import time
from pathlib import Path
# Ensure src is on path when run from repo root
_repo_root = Path(__file__).resolve().parents[3]
if str(_repo_root / "src") not in sys.path:
    sys.path.insert(0, str(_repo_root / "src"))
from exonware.xwsystem.security.at_rest import (
    AES256GCMAtRest,
    XChaCha20Poly1305AtRest,
    FernetAtRest,
    get_at_rest_encryption,
)
# Payload sizes: 1 KiB, 100 KiB, 1 MiB, 10 MiB
SIZES_BYTES = [1024, 1024 * 100, 1024 * 1024, 10 * 1024 * 1024]
# Small payloads for latency: 64 B, 1 KiB
LATENCY_SIZES = [64, 1024]
# Iterations per size for throughput; warmup then timed
ITERATIONS = 5
WARMUP = 2


def run_throughput(impl_name: str, enc, key: bytes, payload: bytes) -> tuple[float, float]:
    """Return (encrypt_mb_s, decrypt_mb_s)."""
    # Encrypt
    for _ in range(WARMUP):
        enc.encrypt(payload, key=key)
    start = time.perf_counter()
    for _ in range(ITERATIONS):
        ct = enc.encrypt(payload, key=key)
    enc_elapsed = time.perf_counter() - start
    total_mb = (len(payload) * ITERATIONS) / (1024 * 1024)
    enc_mb_s = total_mb / enc_elapsed if enc_elapsed > 0 else 0
    # Decrypt (reuse last ct)
    for _ in range(WARMUP):
        enc.decrypt(ct, key=key)
    start = time.perf_counter()
    for _ in range(ITERATIONS):
        enc.decrypt(ct, key=key)
    dec_elapsed = time.perf_counter() - start
    dec_total_mb = (len(ct) * ITERATIONS) / (1024 * 1024)
    dec_mb_s = dec_total_mb / dec_elapsed if dec_elapsed > 0 else 0
    return enc_mb_s, dec_mb_s


def run_latency(enc, key: bytes, payload: bytes) -> tuple[float, float]:
    """Return (encrypt_ms, decrypt_ms) per call."""
    ct = enc.encrypt(payload, key=key)
    start = time.perf_counter()
    for _ in range(ITERATIONS):
        enc.encrypt(payload, key=key)
    enc_ms = (time.perf_counter() - start) / ITERATIONS * 1000
    start = time.perf_counter()
    for _ in range(ITERATIONS):
        enc.decrypt(ct, key=key)
    dec_ms = (time.perf_counter() - start) / ITERATIONS * 1000
    return enc_ms, dec_ms
# Safety tier table (documented, not measured)
SAFETY_TABLE = [
    ("aes256-gcm", "256", "AEAD", "PBKDF2/Argon2id", "1 (highest)"),
    ("xchacha20-poly1305", "256", "AEAD", "PBKDF2/Argon2id", "1 (highest)"),
    ("fernet", "128 (effective)", "CBC+HMAC", "PBKDF2", "2 (high)"),
]


def main() -> None:
    ap = argparse.ArgumentParser(description="Encryption benchmark: throughput and latency")
    ap.add_argument("--output", "-o", type=str, help="Write markdown summary to this file")
    ap.add_argument("--quiet", "-q", action="store_true", help="Less stdout")
    args = ap.parse_args()
    key = os.urandom(32)
    implementations = [
        ("aes256-gcm", AES256GCMAtRest(key=key)),
        ("xchacha20-poly1305", XChaCha20Poly1305AtRest(key=key)),
        ("fernet", FernetAtRest(key=key)),
    ]
    lines = []
    lines.append("# Encryption Benchmark Results")
    lines.append("")
    lines.append("## Throughput (MB/s)")
    lines.append("")
    lines.append("| Algorithm | 1 KiB | 100 KiB | 1 MiB | 10 MiB |")
    lines.append("|-----------|-------|---------|-------|--------|")
    results_throughput = []
    for name, enc in implementations:
        row = [name]
        for size in SIZES_BYTES:
            payload = os.urandom(size)
            enc_mb, dec_mb = run_throughput(name, enc, key, payload)
            row.append(f"{enc_mb:.1f} / {dec_mb:.1f}")
            results_throughput.append((name, size, enc_mb, dec_mb))
        lines.append("| " + " | ".join(row) + " |")
        if not args.quiet:
            print(f"{name}: 1KiB enc/dec MB/s done")
    lines.append("")
    lines.append("(Encrypt / Decrypt MB/s per cell)")
    lines.append("")
    lines.append("## Latency (ms per encrypt/decrypt)")
    lines.append("")
    lines.append("| Algorithm | 64 B | 1 KiB |")
    lines.append("|-----------|------|-------|")
    for name, enc in implementations:
        row = [name]
        for size in LATENCY_SIZES:
            payload = os.urandom(size)
            enc_ms, dec_ms = run_latency(enc, key, payload)
            row.append(f"{enc_ms:.3f} / {dec_ms:.3f}")
        lines.append("| " + " | ".join(row) + " |")
    lines.append("")
    lines.append("## Safety ranking (documented)")
    lines.append("")
    lines.append("| Algorithm | Key size | Mode | KDF | Tier |")
    lines.append("|-----------|----------|------|-----|------|")
    for row in SAFETY_TABLE:
        lines.append("| " + " | ".join(row) + " |")
    lines.append("")
    lines.append("Tier 1 = highest (AEAD, 256-bit). Tier 2 = high (secure but older construction).")
    lines.append("")
    summary = "\n".join(lines)
    if not args.quiet:
        print(summary)
    if args.output:
        out_path = Path(args.output)
        out_path.parent.mkdir(parents=True, exist_ok=True)
        out_path.write_text(summary, encoding="utf-8")
        if not args.quiet:
            print(f"Wrote {out_path}")
if __name__ == "__main__":
    main()

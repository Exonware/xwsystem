#!/usr/bin/env python3
"""
Atomic file I/O benchmark: AtomicFileWriter (write+commit) vs plain open/write/close.
Saves results to ../benchmarks/. Run from xwsystem root or from this script's directory.
"""

from __future__ import annotations
import json
import sys
import time
from pathlib import Path
from datetime import datetime, timezone
_SCRIPT_DIR = Path(__file__).resolve().parent
_CAMPAIGN_ROOT = _SCRIPT_DIR.parent
_XWSYSTEM_ROOT = _CAMPAIGN_ROOT.parent.parent
_src = _XWSYSTEM_ROOT / "src"
if _src.exists() and str(_src) not in sys.path:
    sys.path.insert(0, str(_src))
if str(_XWSYSTEM_ROOT) not in sys.path:
    sys.path.insert(0, str(_XWSYSTEM_ROOT))
BENCHMARKS_DIR = _CAMPAIGN_ROOT / "benchmarks"
DATA_DIR = _CAMPAIGN_ROOT / "data"
PAYLOAD_SIZES = [(1024, "1KB"), (100 * 1024, "100KB"), (1024 * 1024, "1MB")]
NUM_WRITES = 200  # per size per method


def run_timed(func, n: int):
    start = time.perf_counter()
    for _ in range(n):
        func()
    elapsed = time.perf_counter() - start
    return n / elapsed if elapsed > 0 else 0.0


def main():
    BENCHMARKS_DIR.mkdir(parents=True, exist_ok=True)
    DATA_DIR.mkdir(parents=True, exist_ok=True)
    from exonware.xwsystem.io.common.atomic import AtomicFileWriter
    results_all: list[dict] = []
    print("=" * 70, flush=True)
    print("ATOMIC I/O BENCHMARK — AtomicFileWriter vs plain write", flush=True)
    print("=" * 70, flush=True)
    print(f"  Writes per run: {NUM_WRITES}", flush=True)
    print(f"  Output: {BENCHMARKS_DIR}", flush=True)
    print("-" * 70, flush=True)
    for size_bytes, size_name in PAYLOAD_SIZES:
        payload = b"x" * size_bytes
        target_dir = DATA_DIR / "atomic_io_run"
        target_dir.mkdir(parents=True, exist_ok=True)
        # Atomic: AtomicFileWriter with unique file per iteration
        def atomic_write():
            p = target_dir / f"atomic_{size_name}_{time.perf_counter_ns()}.bin"
            with AtomicFileWriter(p, mode="wb") as f:
                f.write(payload)
            if p.exists():
                p.unlink()
        try:
            ops = run_timed(atomic_write, NUM_WRITES)
            results_all.append({
                "name": f"AtomicFileWriter ({size_name})",
                "size_name": size_name,
                "size_bytes": size_bytes,
                "writes_per_sec": round(ops, 2),
            })
            print(f"  AtomicFileWriter ({size_name}): {ops:.1f} writes/s", flush=True)
        except Exception as e:
            results_all.append({
                "name": f"AtomicFileWriter ({size_name})",
                "size_name": size_name,
                "error": str(e),
            })
            print(f"  AtomicFileWriter ({size_name}): ERROR {e}", flush=True)
        # Plain: open, write, close
        def plain_write():
            p = target_dir / f"plain_{size_name}_{time.perf_counter_ns()}.bin"
            with open(p, "wb") as f:
                f.write(payload)
            if p.exists():
                p.unlink()
        ops_plain = run_timed(plain_write, NUM_WRITES)
        results_all.append({
            "name": f"plain write ({size_name})",
            "size_name": size_name,
            "size_bytes": size_bytes,
            "writes_per_sec": round(ops_plain, 2),
        })
        print(f"  plain write ({size_name}): {ops_plain:.1f} writes/s", flush=True)
    # Cleanup
    import shutil
    if target_dir.exists():
        shutil.rmtree(target_dir, ignore_errors=True)
    ts = datetime.now(timezone.utc).strftime("%Y%m%d_%H%M%S")
    results_json_path = BENCHMARKS_DIR / f"results_{ts}.json"
    bench_md_path = BENCHMARKS_DIR / f"BENCH_{ts}_atomic_io.md"
    with open(results_json_path, "w", encoding="utf-8") as f:
        json.dump({
            "timestamp": ts,
            "campaign": "20260210-benchmark xwsystem atomic io",
            "num_writes": NUM_WRITES,
            "payload_sizes": [p[1] for p in PAYLOAD_SIZES],
            "results": results_all,
        }, f, indent=2)
    print("-" * 70, flush=True)
    lines = [
        f"# BENCH {ts} — Atomic file I/O",
        "",
        "## Summary",
        "",
        "| Method | Size | Writes/sec |",
        "|--------|------|------------|",
    ]
    for r in results_all:
        if "error" in r:
            lines.append(f"| {r['name']} | {r.get('size_name', '')} | {r['error']} |")
        else:
            lines.append(f"| {r['name']} | {r.get('size_name', '')} | {r.get('writes_per_sec', '')} |")
    lines.extend(["", "## Raw data", "", f"- [results_{ts}.json](results_{ts}.json)", ""])
    with open(bench_md_path, "w", encoding="utf-8") as f:
        f.write("\n".join(lines))
    index_path = BENCHMARKS_DIR / "INDEX.md"
    index_content = index_path.read_text(encoding="utf-8") if index_path.exists() else ""
    new_entry = (
        f"| {datetime.now(timezone.utc).strftime('%d-%b-%Y')} | "
        f"{datetime.now(timezone.utc).strftime('%H:%M:%S')} | See BENCH_* | Atomic I/O | "
        f"[BENCH_{ts}_atomic_io.md](BENCH_{ts}_atomic_io.md) |\n"
    )
    if "| (No BENCH_*" in index_content:
        index_content = index_content.replace(
            "| (No BENCH_* reports recorded yet) | | | |",
            new_entry.strip(),
        )
    else:
        idx = index_content.find("|------|")
        if idx >= 0:
            insert = index_content.find("\n", idx + 1) + 1
            index_content = index_content[:insert] + new_entry + index_content[insert:]
    index_path.write_text(index_content, encoding="utf-8")
    print(f"  Results: {results_json_path}", flush=True)
    print(f"  Report:  {bench_md_path}", flush=True)
    print("  Done.", flush=True)
if __name__ == "__main__":
    main()

#!/usr/bin/env python3
"""
Object pool benchmark: ObjectPool get+release vs direct instantiation.
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
MAX_SIZE = 1000
NUM_OPS = 10_000


class SimpleObj:
    """Simple object for pool benchmark."""

    def __init__(self):
        self.x = 0


def run_timed(func, n: int):
    start = time.perf_counter()
    for _ in range(n):
        func()
    elapsed = time.perf_counter() - start
    return n / elapsed if elapsed > 0 else 0.0


def main():
    BENCHMARKS_DIR.mkdir(parents=True, exist_ok=True)
    DATA_DIR.mkdir(parents=True, exist_ok=True)
    from exonware.xwsystem.patterns.object_pool import ObjectPool
    results_all: list[dict] = []
    print("=" * 70, flush=True)
    print("OBJECT POOL BENCHMARK — ObjectPool vs direct instantiation", flush=True)
    print("=" * 70, flush=True)
    print(f"  max_size={MAX_SIZE}, num_ops={NUM_OPS}", flush=True)
    print("-" * 70, flush=True)
    # Pool, thread-safe: get then release
    pool_safe = ObjectPool(max_size=MAX_SIZE, enable_thread_safety=True)
    def do_pool_safe():
        obj = pool_safe.get(SimpleObj)
        pool_safe.release(obj)
    ops = run_timed(do_pool_safe, NUM_OPS)
    results_all.append({"name": "ObjectPool get+release (thread_safe=True)", "ops_per_sec": round(ops, 2)})
    print(f"  ObjectPool (thread_safe=True): {ops:.1f} ops/s", flush=True)
    # Pool, non–thread-safe
    pool_unsafe = ObjectPool(max_size=MAX_SIZE, enable_thread_safety=False)
    def do_pool_unsafe():
        obj = pool_unsafe.get(SimpleObj)
        pool_unsafe.release(obj)
    ops = run_timed(do_pool_unsafe, NUM_OPS)
    results_all.append({"name": "ObjectPool get+release (thread_safe=False)", "ops_per_sec": round(ops, 2)})
    print(f"  ObjectPool (thread_safe=False): {ops:.1f} ops/s", flush=True)
    # Direct instantiation
    def do_direct():
        SimpleObj()
    ops = run_timed(do_direct, NUM_OPS)
    results_all.append({"name": "direct SimpleObj()", "ops_per_sec": round(ops, 2)})
    print(f"  direct SimpleObj(): {ops:.1f} ops/s", flush=True)
    ts = datetime.now(timezone.utc).strftime("%Y%m%d_%H%M%S")
    results_json_path = BENCHMARKS_DIR / f"results_{ts}.json"
    bench_md_path = BENCHMARKS_DIR / f"BENCH_{ts}_object_pool.md"
    with open(results_json_path, "w", encoding="utf-8") as f:
        json.dump({
            "timestamp": ts,
            "campaign": "20260210-benchmark xwsystem object pool",
            "max_size": MAX_SIZE,
            "num_ops": NUM_OPS,
            "results": results_all,
        }, f, indent=2)
    lines = [
        f"# BENCH {ts} — Object pool",
        "",
        "## Summary",
        "",
        "| Method | ops/sec |",
        "|--------|--------|",
    ]
    for r in results_all:
        lines.append(f"| {r['name']} | {r.get('ops_per_sec', '')} |")
    lines.extend(["", "## Raw data", "", f"- [results_{ts}.json](results_{ts}.json)", ""])
    with open(bench_md_path, "w", encoding="utf-8") as f:
        f.write("\n".join(lines))
    index_path = BENCHMARKS_DIR / "INDEX.md"
    index_content = index_path.read_text(encoding="utf-8") if index_path.exists() else ""
    new_entry = (
        f"| {datetime.now(timezone.utc).strftime('%d-%b-%Y')} | "
        f"{datetime.now(timezone.utc).strftime('%H:%M:%S')} | See BENCH_* | Object pool | "
        f"[BENCH_{ts}_object_pool.md](BENCH_{ts}_object_pool.md) |\n"
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
    print("-" * 70, flush=True)
    print(f"  Results: {results_json_path}", flush=True)
    print(f"  Report:  {bench_md_path}", flush=True)
    print("  Done.", flush=True)
if __name__ == "__main__":
    main()

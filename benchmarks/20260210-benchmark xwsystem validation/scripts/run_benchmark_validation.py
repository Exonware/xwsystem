#!/usr/bin/env python3
"""
Validation benchmark: check_data_depth and validate_path_input throughput.
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
NUM_OPS = 10_000


def run_timed(func, n: int):
    start = time.perf_counter()
    for _ in range(n):
        func()
    elapsed = time.perf_counter() - start
    return n / elapsed if elapsed > 0 else 0.0


def main():
    BENCHMARKS_DIR.mkdir(parents=True, exist_ok=True)
    DATA_DIR.mkdir(parents=True, exist_ok=True)
    from exonware.xwsystem.validation.data_validator import check_data_depth, validate_path_input
    results_all: list[dict] = []
    print("=" * 70, flush=True)
    print("VALIDATION BENCHMARK — check_data_depth, validate_path_input", flush=True)
    print("=" * 70, flush=True)
    print(f"  Ops per run: {NUM_OPS}", flush=True)
    print("-" * 70, flush=True)
    # Shallow structure
    shallow = {"a": 1, "b": [2, 3], "c": "x"}
    def do_depth_shallow():
        check_data_depth(shallow)
    ops = run_timed(do_depth_shallow, NUM_OPS)
    results_all.append({"name": "check_data_depth (shallow)", "ops_per_sec": round(ops, 2)})
    print(f"  check_data_depth (shallow): {ops:.1f} ops/s", flush=True)
    # Deep structure (e.g. depth 15)
    deep = {"x": 1}
    node = deep
    for _ in range(14):
        node["n"] = {"y": 1}
        node = node["n"]
    def do_depth_deep():
        check_data_depth(deep, max_depth=20)
    ops = run_timed(do_depth_deep, NUM_OPS)
    results_all.append({"name": "check_data_depth (deep)", "ops_per_sec": round(ops, 2)})
    print(f"  check_data_depth (deep): {ops:.1f} ops/s", flush=True)
    # Path validation — safe path
    safe_path = "config/app/settings.json"
    def do_path():
        validate_path_input(safe_path)
    ops = run_timed(do_path, NUM_OPS)
    results_all.append({"name": "validate_path_input (safe)", "ops_per_sec": round(ops, 2)})
    print(f"  validate_path_input (safe): {ops:.1f} ops/s", flush=True)
    ts = datetime.now(timezone.utc).strftime("%Y%m%d_%H%M%S")
    results_json_path = BENCHMARKS_DIR / f"results_{ts}.json"
    bench_md_path = BENCHMARKS_DIR / f"BENCH_{ts}_validation.md"
    with open(results_json_path, "w", encoding="utf-8") as f:
        json.dump({
            "timestamp": ts,
            "campaign": "20260210-benchmark xwsystem validation",
            "num_ops": NUM_OPS,
            "results": results_all,
        }, f, indent=2)
    lines = [
        f"# BENCH {ts} — Validation",
        "",
        "## Summary",
        "",
        "| Check | ops/sec |",
        "|-------|--------|",
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
        f"{datetime.now(timezone.utc).strftime('%H:%M:%S')} | See BENCH_* | Validation | "
        f"[BENCH_{ts}_validation.md](BENCH_{ts}_validation.md) |\n"
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

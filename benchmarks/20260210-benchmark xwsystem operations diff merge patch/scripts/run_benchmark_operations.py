#!/usr/bin/env python3
"""
Operations benchmark: DiffOperation, MergeOperation, PatchOperationImpl throughput.
Saves results to ../benchmarks/. Run from xwsystem root or from this script's directory.
"""

from __future__ import annotations
import copy
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
NUM_OPS = 500


def make_small():
    a = {"a": 1, "b": [2, 3], "c": "x"}
    b = {"a": 2, "b": [2, 3, 4], "c": "x"}
    return a, b


def make_medium():
    a = {"users": [{"id": i, "name": f"u{i}"} for i in range(100)], "meta": {"p": 1}}
    b = {"users": [{"id": i, "name": f"user_{i}"} for i in range(100)], "meta": {"p": 2}}
    return a, b


def make_large():
    a = {"items": [{"id": i, "data": list(range(20))} for i in range(500)], "meta": {}}
    b = {"items": [{"id": i, "data": list(range(21))} for i in range(500)], "meta": {"v": 1}}
    return a, b


def run_timed(func, n: int):
    start = time.perf_counter()
    for _ in range(n):
        func()
    elapsed = time.perf_counter() - start
    return n / elapsed if elapsed > 0 else 0.0


def main():
    BENCHMARKS_DIR.mkdir(parents=True, exist_ok=True)
    DATA_DIR.mkdir(parents=True, exist_ok=True)
    from exonware.xwsystem.operations.diff import DiffOperation
    from exonware.xwsystem.operations.merge import MergeOperation
    from exonware.xwsystem.operations.patch import PatchOperationImpl
    from exonware.xwsystem.operations.defs import DiffMode, MergeStrategy
    diff_op = DiffOperation()
    merge_op = MergeOperation()
    patch_op = PatchOperationImpl()
    results_all: list[dict] = []
    fixtures = [("small", make_small), ("medium", make_medium), ("large", make_large)]
    print("=" * 70, flush=True)
    print("OPERATIONS BENCHMARK — Diff, Merge, Patch", flush=True)
    print("=" * 70, flush=True)
    print(f"  Ops per run: {NUM_OPS}", flush=True)
    print("-" * 70, flush=True)
    for size_name, make_fn in fixtures:
        orig, mod = make_fn()
        # Diff
        for mode in (DiffMode.STRUCTURAL, DiffMode.CONTENT, DiffMode.FULL):
            def do_diff():
                diff_op.diff(copy.deepcopy(orig), copy.deepcopy(mod), mode)
            ops = run_timed(do_diff, NUM_OPS)
            results_all.append({
                "name": f"diff_{mode.value} ({size_name})",
                "ops_per_sec": round(ops, 2),
            })
            print(f"  diff {mode.value} ({size_name}): {ops:.1f} ops/s", flush=True)
        # Merge: need fresh target/source each time (merge may mutate)
        for strategy in (MergeStrategy.DEEP, MergeStrategy.SHALLOW, MergeStrategy.OVERWRITE):
            def do_merge():
                t = copy.deepcopy(orig)
                s = copy.deepcopy(mod)
                merge_op.merge(t, s, strategy)
            ops = run_timed(do_merge, NUM_OPS)
            results_all.append({
                "name": f"merge_{strategy.value} ({size_name})",
                "ops_per_sec": round(ops, 2),
            })
            print(f"  merge {strategy.value} ({size_name}): {ops:.1f} ops/s", flush=True)
    # Patch: one data + list of operations
    patch_ops = [{"op": "replace", "path": "/a", "value": 99}, {"op": "add", "path": "/z", "value": 1}]
    data_small = make_small()[0]
    def do_patch():
        patch_op.apply_patch(copy.deepcopy(data_small), patch_ops)
    ops = run_timed(do_patch, NUM_OPS)
    results_all.append({"name": "patch_apply (small)", "ops_per_sec": round(ops, 2)})
    print(f"  patch apply (small): {ops:.1f} ops/s", flush=True)
    ts = datetime.now(timezone.utc).strftime("%Y%m%d_%H%M%S")
    results_json_path = BENCHMARKS_DIR / f"results_{ts}.json"
    bench_md_path = BENCHMARKS_DIR / f"BENCH_{ts}_operations.md"
    with open(results_json_path, "w", encoding="utf-8") as f:
        json.dump({
            "timestamp": ts,
            "campaign": "20260210-benchmark xwsystem operations diff merge patch",
            "num_ops": NUM_OPS,
            "results": results_all,
        }, f, indent=2)
    lines = [
        f"# BENCH {ts} — Operations (Diff, Merge, Patch)",
        "",
        "## Summary",
        "",
        "| Operation | ops/sec |",
        "|-----------|--------|",
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
        f"{datetime.now(timezone.utc).strftime('%H:%M:%S')} | See BENCH_* | Operations | "
        f"[BENCH_{ts}_operations.md](BENCH_{ts}_operations.md) |\n"
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

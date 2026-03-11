#!/usr/bin/env python3
"""
Caching benchmark: PylruCache (default) vs all xwsystem caches and top Python caches.
Default cache is PylruCache (pylru wrapper); when pylru is not installed, factory
falls back to FunctoolsLRUCache. Saves results to ../benchmarks/ (BENCH_*.md, results_*.json).
Uses ../data/ for config/temp. Legacy baseline: ../data/baseline/_legacy/.
Run from xwsystem root or from this script's directory.
"""

from __future__ import annotations
import json
import os
import sys
import time
import tempfile
from pathlib import Path
from datetime import datetime, timezone
# Ensure xwsystem package is importable
_SCRIPT_DIR = Path(__file__).resolve().parent
_CAMPAIGN_ROOT = _SCRIPT_DIR.parent
_XWSYSTEM_ROOT = _CAMPAIGN_ROOT.parent.parent
if str(_XWSYSTEM_ROOT) not in sys.path:
    sys.path.insert(0, str(_XWSYSTEM_ROOT))
BENCHMARKS_DIR = _CAMPAIGN_ROOT / "benchmarks"
DATA_DIR = _CAMPAIGN_ROOT / "data"
CAPACITY = 2000
PREFILL_KEYS = 1000
NUM_OPS = 10_000
MIXED_GET_RATIO = 0.8


def run_ops(get_fn, put_fn, mode: str, cache, key_prefix: str = "k"):
    if mode == "get":
        start = time.perf_counter()
        for i in range(NUM_OPS):
            get_fn(f"{key_prefix}_{i % PREFILL_KEYS}")
        elapsed = time.perf_counter() - start
        return NUM_OPS / elapsed if elapsed > 0 else 0.0
    if mode == "put":
        start = time.perf_counter()
        for i in range(NUM_OPS):
            put_fn(f"{key_prefix}_{i % CAPACITY}", i)
        elapsed = time.perf_counter() - start
        return NUM_OPS / elapsed if elapsed > 0 else 0.0
    if mode == "mixed":
        start = time.perf_counter()
        for i in range(NUM_OPS):
            if (i % 10) < 8:
                get_fn(f"{key_prefix}_{i % PREFILL_KEYS}")
            else:
                put_fn(f"{key_prefix}_{(i + PREFILL_KEYS) % CAPACITY}", i)
        elapsed = time.perf_counter() - start
        return NUM_OPS / elapsed if elapsed > 0 else 0.0
    return 0.0


def _fmt_ops(v):
    if v >= 1_000_000:
        return f"{v/1e6:.2f}M"
    if v >= 1_000:
        return f"{v/1e3:.1f}k"
    return f"{v:.0f}"


def bench_cache(name: str, cache, results: dict) -> None:
    def get_fn(k):
        return cache.get(k, None)
    def put_fn(k, v):
        cache.put(k, v)
    # Prefill for get/mixed
    for i in range(PREFILL_KEYS):
        put_fn(f"k_{i}", i)
    get_ops = run_ops(get_fn, put_fn, "get", cache)
    put_ops = run_ops(get_fn, put_fn, "put", cache)
    mixed_ops = run_ops(get_fn, put_fn, "mixed", cache)
    results["get_ops_per_sec"] = round(get_ops, 2)
    results["put_ops_per_sec"] = round(put_ops, 2)
    results["mixed_ops_per_sec"] = round(mixed_ops, 2)


def main():
    BENCHMARKS_DIR.mkdir(parents=True, exist_ok=True)
    DATA_DIR.mkdir(parents=True, exist_ok=True)
    legacy_baseline = DATA_DIR / "baseline" / "_legacy" / "baseline_v0.0.1.387.json"
    results_all: list[dict] = []
    step = [0]
    total_caches = 16 + 3 + 1  # xwsystem (incl. PylruCache default) + external + rust
    def progress(name: str, r: dict) -> None:
        step[0] += 1
        if "error" in r:
            print(f"  [{step[0]}/{total_caches}] {name}  SKIP: {r['error']}", flush=True)
        else:
            g = r.get("get_ops_per_sec", 0)
            p = r.get("put_ops_per_sec", 0)
            m = r.get("mixed_ops_per_sec", 0)
            print(f"  [{step[0]}/{total_caches}] {name}  get={_fmt_ops(g)}/s  put={_fmt_ops(p)}/s  mixed={_fmt_ops(m)}/s", flush=True)
    print("=" * 70, flush=True)
    print("CACHING BENCHMARK — PylruCache (default) vs xwsystem + top Python caches", flush=True)
    print("=" * 70, flush=True)
    print(f"  Capacity: {CAPACITY}, prefill: {PREFILL_KEYS}, ops per workload: {NUM_OPS}", flush=True)
    print(f"  Output: {BENCHMARKS_DIR}", flush=True)
    print("-" * 70, flush=True)
    # --- xwsystem caches (from xwsystem/src/exonware/xwsystem/caching) ---
    try:
        from exonware.xwsystem.caching import (
            PylruCache,
            FunctoolsLRUCache,
            CacheboxCache,
            CachetoolsLRUCache,
            CachetoolsLFUCache,
            CachetoolsTTLCache,
            CachetoolsRRCache,
            LRUCache,
            LFUCache,
            OptimizedLFUCache,
            TTLCache,
            MemoryBoundedLRUCache,
            MemoryBoundedLFUCache,
            FluentLRUCache,
            ObservableLRUCache,
        )
    except ImportError as e:
        results_all.append({"name": "xwsystem (import)", "error": str(e)})
    else:
        xw_caches = [
            ("PylruCache (default)", lambda: PylruCache(capacity=CAPACITY)),
            ("FunctoolsLRUCache", lambda: FunctoolsLRUCache(capacity=CAPACITY)),
            ("CacheboxCache", lambda: CacheboxCache(capacity=CAPACITY)),
            ("CachetoolsLRUCache", lambda: CachetoolsLRUCache(capacity=CAPACITY)),
            ("CachetoolsLFUCache", lambda: CachetoolsLFUCache(capacity=CAPACITY)),
            ("CachetoolsTTLCache", lambda: CachetoolsTTLCache(capacity=CAPACITY, ttl=3600)),
            ("CachetoolsRRCache", lambda: CachetoolsRRCache(capacity=CAPACITY)),
            ("LRUCache", lambda: LRUCache(capacity=CAPACITY)),
            ("LFUCache", lambda: LFUCache(capacity=CAPACITY)),
            ("OptimizedLFUCache", lambda: OptimizedLFUCache(capacity=CAPACITY)),
            ("TTLCache", lambda: TTLCache(capacity=CAPACITY, ttl=3600)),
            ("MemoryBoundedLRUCache", lambda: MemoryBoundedLRUCache(capacity=CAPACITY)),
            ("MemoryBoundedLFUCache", lambda: MemoryBoundedLFUCache(capacity=CAPACITY)),
            ("FluentLRUCache", lambda: FluentLRUCache(capacity=CAPACITY)),
            ("ObservableLRUCache", lambda: ObservableLRUCache(capacity=CAPACITY)),
        ]
        for label, factory in xw_caches:
            try:
                cache = factory()
                r = {"name": label, "group": "xwsystem"}
                bench_cache(label, cache, r)
                results_all.append(r)
                progress(label, r)
            except Exception as e:
                r = {"name": label, "group": "xwsystem", "error": str(e)}
                results_all.append(r)
                progress(label, r)
        # TwoTierCache (needs disk dir)
        try:
            with tempfile.TemporaryDirectory(prefix="two_tier_", dir=str(DATA_DIR)) as tmp:
                from exonware.xwsystem.caching.two_tier_cache import TwoTierCache
                cache = TwoTierCache(memory_size=CAPACITY, disk_size=CAPACITY * 2, disk_cache_dir=tmp)
                r = {"name": "TwoTierCache", "group": "xwsystem"}
                bench_cache("TwoTierCache", cache, r)
                results_all.append(r)
                progress("TwoTierCache", r)
        except Exception as e:
            r = {"name": "TwoTierCache", "group": "xwsystem", "error": str(e)}
            results_all.append(r)
            progress("TwoTierCache", r)
    # --- External Python caches (top 10 style) ---
    # diskcache
    try:
        import diskcache
        with tempfile.TemporaryDirectory(prefix="diskcache_", dir=str(DATA_DIR)) as tmp:
            cache = diskcache.Cache(tmp, size_limit=CAPACITY * 100)
            for i in range(PREFILL_KEYS):
                cache.set(f"k_{i}", i)
            r = {"name": "diskcache", "group": "external"}
            start = time.perf_counter()
            for i in range(NUM_OPS):
                cache.get(f"k_{i % PREFILL_KEYS}")
            r["get_ops_per_sec"] = round(NUM_OPS / (time.perf_counter() - start), 2)
            start = time.perf_counter()
            for i in range(NUM_OPS):
                cache.set(f"k_{i % CAPACITY}", i)
            r["put_ops_per_sec"] = round(NUM_OPS / (time.perf_counter() - start), 2)
            start = time.perf_counter()
            for i in range(NUM_OPS):
                if (i % 10) < 8:
                    cache.get(f"k_{i % PREFILL_KEYS}")
                else:
                    cache.set(f"k_{(i + PREFILL_KEYS) % CAPACITY}", i)
            r["mixed_ops_per_sec"] = round(NUM_OPS / (time.perf_counter() - start), 2)
            results_all.append(r)
            cache.close()
            progress("diskcache", r)
    except ImportError:
        r = {"name": "diskcache", "group": "external", "error": "not installed"}
        results_all.append(r)
        progress("diskcache", r)
    except Exception as e:
        r = {"name": "diskcache", "group": "external", "error": str(e)}
        results_all.append(r)
        progress("diskcache", r)
    # cacheout
    try:
        import cacheout
        cache = cacheout.LRUCache(maxsize=CAPACITY)
        r = {"name": "cacheout.LRUCache", "group": "external"}
        for i in range(PREFILL_KEYS):
            cache.set(f"k_{i}", i)
        start = time.perf_counter()
        for i in range(NUM_OPS):
            cache.get(f"k_{i % PREFILL_KEYS}")
        r["get_ops_per_sec"] = round(NUM_OPS / (time.perf_counter() - start), 2)
        start = time.perf_counter()
        for i in range(NUM_OPS):
            cache.set(f"k_{i % CAPACITY}", i)
        r["put_ops_per_sec"] = round(NUM_OPS / (time.perf_counter() - start), 2)
        start = time.perf_counter()
        for i in range(NUM_OPS):
            if (i % 10) < 8:
                cache.get(f"k_{i % PREFILL_KEYS}")
            else:
                cache.set(f"k_{(i + PREFILL_KEYS) % CAPACITY}", i)
        r["mixed_ops_per_sec"] = round(NUM_OPS / (time.perf_counter() - start), 2)
        results_all.append(r)
        progress("cacheout.LRUCache", r)
    except ImportError:
        r = {"name": "cacheout", "group": "external", "error": "not installed"}
        results_all.append(r)
        progress("cacheout", r)
    # pylru
    try:
        import pylru
        cache = pylru.lrucache(CAPACITY)
        r = {"name": "pylru", "group": "external"}
        for i in range(PREFILL_KEYS):
            cache[f"k_{i}"] = i
        start = time.perf_counter()
        for i in range(NUM_OPS):
            _ = cache.get(f"k_{i % PREFILL_KEYS}")
        r["get_ops_per_sec"] = round(NUM_OPS / (time.perf_counter() - start), 2)
        start = time.perf_counter()
        for i in range(NUM_OPS):
            cache[f"k_{i % CAPACITY}"] = i
        r["put_ops_per_sec"] = round(NUM_OPS / (time.perf_counter() - start), 2)
        start = time.perf_counter()
        for i in range(NUM_OPS):
            if (i % 10) < 8:
                _ = cache.get(f"k_{i % PREFILL_KEYS}")
            else:
                cache[f"k_{(i + PREFILL_KEYS) % CAPACITY}"] = i
        r["mixed_ops_per_sec"] = round(NUM_OPS / (time.perf_counter() - start), 2)
        results_all.append(r)
        progress("pylru", r)
    except ImportError:
        r = {"name": "pylru", "group": "external", "error": "not installed"}
        results_all.append(r)
        progress("pylru", r)
    # --- Rust attempt (xwsystem rust bindings) ---
    try:
        from exonware.rust.xwsystem.caching import MokaCache
        cache = MokaCache(CAPACITY)
        r = {"name": "Rust MokaCache (external_caching_rust)", "group": "rust"}
        for i in range(PREFILL_KEYS):
            cache.put(f"k_{i}", i)
        start = time.perf_counter()
        for i in range(NUM_OPS):
            cache.get(f"k_{i % PREFILL_KEYS}")
        r["get_ops_per_sec"] = round(NUM_OPS / (time.perf_counter() - start), 2)
        start = time.perf_counter()
        for i in range(NUM_OPS):
            cache.put(f"k_{i % CAPACITY}", i)
        r["put_ops_per_sec"] = round(NUM_OPS / (time.perf_counter() - start), 2)
        start = time.perf_counter()
        for i in range(NUM_OPS):
            if (i % 10) < 8:
                cache.get(f"k_{i % PREFILL_KEYS}")
            else:
                cache.put(f"k_{(i + PREFILL_KEYS) % CAPACITY}", i)
        r["mixed_ops_per_sec"] = round(NUM_OPS / (time.perf_counter() - start), 2)
        results_all.append(r)
        progress("Rust MokaCache", r)
    except ImportError:
        r = {
            "name": "Rust (Moka/QuickCache/DashMap)",
            "group": "rust",
            "error": "rust bindings not built (maturin develop --features external-caches,python)",
        }
        results_all.append(r)
        progress(r["name"], r)
    # --- Write results ---
    ts = datetime.now(timezone.utc).strftime("%Y%m%d_%H%M%S")
    results_json_path = BENCHMARKS_DIR / f"results_{ts}.json"
    bench_md_path = BENCHMARKS_DIR / f"BENCH_{ts}_caching_comparison.md"
    out_payload = {
        "timestamp": ts,
        "campaign": "20260209-benchmark xwsystem caching vs others",
        "capacity": CAPACITY,
        "prefill_keys": PREFILL_KEYS,
        "num_ops": NUM_OPS,
        "mixed_get_ratio": MIXED_GET_RATIO,
        "results": results_all,
    }
    if legacy_baseline.exists():
        out_payload["legacy_baseline_path"] = str(legacy_baseline)
    with open(results_json_path, "w", encoding="utf-8") as f:
        json.dump(out_payload, f, indent=2)
    print("-" * 70, flush=True)
    print("SUMMARY (get / put / mixed ops/s)", flush=True)
    print("-" * 70, flush=True)
    for r in results_all:
        if "error" in r:
            print(f"  {r['name']}: {r['error']}", flush=True)
        else:
            g = r.get("get_ops_per_sec", 0)
            p = r.get("put_ops_per_sec", 0)
            m = r.get("mixed_ops_per_sec", 0)
            print(f"  {r['name']}:  get {_fmt_ops(g)}/s   put {_fmt_ops(p)}/s   mixed {_fmt_ops(m)}/s", flush=True)
    print("-" * 70, flush=True)
    print(f"  Results JSON: {results_json_path}", flush=True)
    lines = [
        f"# BENCH {ts} — Caching comparison",
        "",
        "## Summary",
        "",
        "| Cache | Group | Get (ops/s) | Put (ops/s) | Mixed (ops/s) |",
        "|-------|-------|-------------|-------------|---------------|",
    ]
    for r in results_all:
        if "error" in r:
            lines.append(f"| {r['name']} | {r.get('group', '')} | — | — | {r['error']} |")
        else:
            lines.append(
                f"| {r['name']} | {r.get('group', '')} | "
                f"{r.get('get_ops_per_sec', '')} | {r.get('put_ops_per_sec', '')} | "
                f"{r.get('mixed_ops_per_sec', '')} |"
            )
    lines.extend([
        "",
        "## Raw data",
        "",
        f"- Full results: [results_{ts}.json](results_{ts}.json)",
        "",
    ])
    if legacy_baseline.exists():
        lines.append(f"- Legacy baseline: [data/baseline/_legacy/baseline_v0.0.1.387.json](../data/baseline/_legacy/baseline_v0.0.1.387.json)")
        lines.append("")
    with open(bench_md_path, "w", encoding="utf-8") as f:
        f.write("\n".join(lines))
    # Update INDEX.md
    index_path = BENCHMARKS_DIR / "INDEX.md"
    index_content = index_path.read_text(encoding="utf-8") if index_path.exists() else ""
    new_entry = (
        f"| {datetime.now(timezone.utc).strftime('%d-%b-%Y')} | "
        f"{datetime.now(timezone.utc).strftime('%H:%M:%S')} | See BENCH_* | Caching comparison | "
        f"[BENCH_{ts}_caching_comparison.md](BENCH_{ts}_caching_comparison.md) |\n"
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
        else:
            index_content = index_content.replace(
                "| (No BENCH_* reports recorded yet) | | | |",
                new_entry.strip(),
            )
    index_path.parent.mkdir(parents=True, exist_ok=True)
    index_path.write_text(index_content, encoding="utf-8")
    print(f"  Report:      {bench_md_path}", flush=True)
    print("  Done.", flush=True)
if __name__ == "__main__":
    main()

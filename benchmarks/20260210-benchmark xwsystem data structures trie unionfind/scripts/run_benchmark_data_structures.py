#!/usr/bin/env python3
"""
Data structures benchmark: TrieNode-based trie (insert/lookup) and UnionFind (union/find) throughput.
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
NUM_OPS = 5000
TRIE_KEYS = 500
UF_SIZE = 1000
UF_OPS = 2000


def run_timed(func, n: int):
    start = time.perf_counter()
    for _ in range(n):
        func()
    elapsed = time.perf_counter() - start
    return n / elapsed if elapsed > 0 else 0.0


def main():
    BENCHMARKS_DIR.mkdir(parents=True, exist_ok=True)
    DATA_DIR.mkdir(parents=True, exist_ok=True)
    from exonware.xwsystem.data_structures.trie import TrieNode
    from exonware.xwsystem.data_structures.union_find import UnionFind
    results_all: list[dict] = []
    print("=" * 70, flush=True)
    print("DATA STRUCTURES BENCHMARK — Trie (TrieNode), UnionFind", flush=True)
    print("=" * 70, flush=True)
    print("-" * 70, flush=True)
    # Trie: insert then lookup using TrieNode
    def trie_insert(root: TrieNode, key: str) -> None:
        node = root
        for ch in key:
            if ch not in node.children:
                node.children[ch] = TrieNode()
            node = node.children[ch]
        node.is_end_word = True
    def trie_lookup(root: TrieNode, key: str) -> bool:
        node = root
        for ch in key:
            if ch not in node.children:
                return False
            node = node.children[ch]
        return node.is_end_word
    keys = [f"key_{i}_suffix" for i in range(TRIE_KEYS)]
    root = TrieNode()
    for k in keys:
        trie_insert(root, k)
    def do_trie_lookup():
        for k in keys:
            trie_lookup(root, k)
    ops = run_timed(do_trie_lookup, NUM_OPS // TRIE_KEYS)
    results_all.append({"name": "trie_lookup (TrieNode)", "ops_per_sec": round(ops * TRIE_KEYS, 2)})
    print(f"  trie lookup (TrieNode): {ops * TRIE_KEYS:.1f} ops/s", flush=True)
    # Dict lookup comparison
    d = {k: True for k in keys}
    def do_dict_lookup():
        for k in keys:
            _ = k in d
    ops = run_timed(do_dict_lookup, NUM_OPS // TRIE_KEYS)
    results_all.append({"name": "dict_lookup", "ops_per_sec": round(ops * TRIE_KEYS, 2)})
    print(f"  dict lookup: {ops * TRIE_KEYS:.1f} ops/s", flush=True)
    # Trie insert (new trie per run to avoid reusing same nodes)
    def do_trie_insert_batch():
        r = TrieNode()
        for k in keys:
            trie_insert(r, k)
    ops = run_timed(do_trie_insert_batch, max(1, NUM_OPS // (TRIE_KEYS * 2)))
    results_all.append({"name": "trie_insert (TrieNode batch)", "ops_per_sec": round(ops * TRIE_KEYS, 2)})
    print(f"  trie insert batch: {ops * TRIE_KEYS:.1f} ops/s", flush=True)
    # UnionFind: make_set then union+find mix
    def do_uf_work():
        uf = UnionFind()
        for i in range(UF_SIZE):
            uf.make_set(i)
        for i in range(0, UF_SIZE - 1, 2):
            uf.union(i, i + 1)
        for _ in range(UF_OPS):
            uf.find(i % UF_SIZE)
    ops = run_timed(do_uf_work, max(1, NUM_OPS // 100))
    results_all.append({"name": "UnionFind (make_set+union+find)", "ops_per_sec": round(ops, 2)})
    print(f"  UnionFind (make_set+union+find): {ops:.1f} runs/s", flush=True)
    ts = datetime.now(timezone.utc).strftime("%Y%m%d_%H%M%S")
    results_json_path = BENCHMARKS_DIR / f"results_{ts}.json"
    bench_md_path = BENCHMARKS_DIR / f"BENCH_{ts}_data_structures.md"
    with open(results_json_path, "w", encoding="utf-8") as f:
        json.dump({
            "timestamp": ts,
            "campaign": "20260210-benchmark xwsystem data structures trie unionfind",
            "num_ops": NUM_OPS,
            "trie_keys": TRIE_KEYS,
            "uf_size": UF_SIZE,
            "uf_ops": UF_OPS,
            "results": results_all,
        }, f, indent=2)
    lines = [
        f"# BENCH {ts} — Data structures (Trie, UnionFind)",
        "",
        "## Summary",
        "",
        "| Structure | ops/sec |",
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
        f"{datetime.now(timezone.utc).strftime('%H:%M:%S')} | See BENCH_* | Data structures | "
        f"[BENCH_{ts}_data_structures.md](BENCH_{ts}_data_structures.md) |\n"
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

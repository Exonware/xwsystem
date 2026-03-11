#!/usr/bin/env python3
"""
JSON benchmark: JsonSerializer vs top Python JSON libraries (C++/Rust backends).
Saves results to ../benchmarks/ (BENCH_*.md, results_*.json) and uses ../data/ for payloads.
Run from xwsystem root or from this script's directory.
"""

from __future__ import annotations
import json
import sys
import time
from pathlib import Path
from datetime import datetime, timezone
# Ensure xwsystem package is importable (run from scripts/ or from xwsystem root)
_SCRIPT_DIR = Path(__file__).resolve().parent
_CAMPAIGN_ROOT = _SCRIPT_DIR.parent
_XWSYSTEM_ROOT = _CAMPAIGN_ROOT.parent.parent
if str(_XWSYSTEM_ROOT) not in sys.path:
    sys.path.insert(0, str(_XWSYSTEM_ROOT))
# Let xwlazy handle installation of any missing JSON libs (orjson, ujson, etc.)
_XWLAZY_SRC = _XWSYSTEM_ROOT.parent / "xwlazy" / "src"
if _XWLAZY_SRC.exists() and str(_XWLAZY_SRC) not in sys.path:
    sys.path.insert(0, str(_XWLAZY_SRC))
try:
    from exonware.xwlazy import auto_enable_lazy
    auto_enable_lazy(mode="smart", root=str(_XWSYSTEM_ROOT))
except Exception:
    pass  # Run without xwlazy if unavailable
BENCHMARKS_DIR = _CAMPAIGN_ROOT / "benchmarks"
DATA_DIR = _CAMPAIGN_ROOT / "data"
ROUNDS = 5
ITERATIONS_PER_ROUND = 500
WARMUP = 50


def make_payload_small():
    return {"a": 1, "b": [2, 3], "c": "hello", "d": {"x": 1.0, "y": 2.0}}


def make_payload_medium():
    return {
        "users": [{"id": i, "name": f"user_{i}", "active": i % 2 == 0} for i in range(200)],
        "meta": {"page": 1, "total": 200},
    }


def make_payload_large():
    return {
        "items": [{"id": i, "data": list(range(20)), "label": f"item_{i}"} for i in range(1000)],
        "meta": {"count": 1000},
    }


def run_timed(func, n: int):
    start = time.perf_counter()
    for _ in range(n):
        func()
    elapsed = time.perf_counter() - start
    return n / elapsed if elapsed > 0 else 0.0


def bench_lib(name: str, payload: dict, load_fn, dump_fn, results: list[dict]) -> None:
    raw = dump_fn(payload)
    if isinstance(raw, bytes):
        raw_str = raw.decode("utf-8")
    else:
        raw_str = raw
    # Cold decode
    def decode_cold():
        return load_fn(raw_str if isinstance(raw_str, str) else raw_str.encode("utf-8"))
    cold_decode_ops = run_timed(decode_cold, ITERATIONS_PER_ROUND)
    # Warm decode
    def decode_warm():
        return load_fn(raw_str if isinstance(raw_str, str) else raw_str.encode("utf-8"))
    for _ in range(WARMUP):
        decode_warm()
    warm_decode_ops = run_timed(decode_warm, ITERATIONS_PER_ROUND)
    # Cold encode (re-decode once so we have same object)
    obj = load_fn(raw_str if isinstance(raw_str, str) else raw_str.encode("utf-8"))
    def encode_cold():
        return dump_fn(obj)
    cold_encode_ops = run_timed(encode_cold, ITERATIONS_PER_ROUND)
    # Warm encode
    for _ in range(WARMUP):
        dump_fn(obj)
    warm_encode_ops = run_timed(lambda: dump_fn(obj), ITERATIONS_PER_ROUND)
    results.append({
        "name": name,
        "cold_decode_ops_per_sec": round(cold_decode_ops, 2),
        "warm_decode_ops_per_sec": round(warm_decode_ops, 2),
        "cold_encode_ops_per_sec": round(cold_encode_ops, 2),
        "warm_encode_ops_per_sec": round(warm_encode_ops, 2),
    })


def _fmt_ops(v):
    if v >= 1_000_000:
        return f"{v/1e6:.2f}M"
    if v >= 1_000:
        return f"{v/1e3:.1f}k"
    return f"{v:.0f}"


def main():
    BENCHMARKS_DIR.mkdir(parents=True, exist_ok=True)
    DATA_DIR.mkdir(parents=True, exist_ok=True)
    payload_small = make_payload_small()
    payload_medium = make_payload_medium()
    payload_large = make_payload_large()
    # Save sample payload to data/
    sample_path = DATA_DIR / "payload_sample.json"
    with open(sample_path, "w", encoding="utf-8") as f:
        json.dump(payload_medium, f, indent=2)
    results_all: list[dict] = []
    payloads = [("small", payload_small), ("medium", payload_medium), ("large", payload_large)]
    # Count total steps for progress (libs x 3 sizes; some libs may be skipped)
    libs_planned = ["JsonSerializer", "orjson", "ujson", "stdlib json", "rapidjson", "msgspec", "simplejson", "pysimdjson", "hyperjson"]
    total_steps = len(libs_planned) * 3
    step = [0]
    def progress(name: str, results: list[dict]) -> None:
        step[0] += 1
        if results:
            r = results[-1]
            if "error" not in r:
                cdec = r.get("cold_decode_ops_per_sec", 0)
                wdec = r.get("warm_decode_ops_per_sec", 0)
                cenc = r.get("cold_encode_ops_per_sec", 0)
                wenc = r.get("warm_encode_ops_per_sec", 0)
                print(f"  [{step[0]}/{total_steps}] {name}  decode: cold={_fmt_ops(cdec)}/s warm={_fmt_ops(wdec)}/s  encode: cold={_fmt_ops(cenc)}/s warm={_fmt_ops(wenc)}/s", flush=True)
            else:
                print(f"  [{step[0]}/{total_steps}] {name}  SKIP: {r['error']}", flush=True)
        else:
            print(f"  [{step[0]}/{total_steps}] {name}  (no result)", flush=True)
    print("=" * 70, flush=True)
    print("JSON BENCHMARK — JsonSerializer vs top Python JSON libs", flush=True)
    print("=" * 70, flush=True)
    print(f"  Iterations: {ITERATIONS_PER_ROUND} per round, warmup: {WARMUP}", flush=True)
    print(f"  Payloads: small, medium, large", flush=True)
    print(f"  Output: {BENCHMARKS_DIR}", flush=True)
    print("-" * 70, flush=True)
    # --- JsonSerializer (xwsystem) ---
    try:
        from exonware.xwsystem.io.serialization.formats.text.json import JsonSerializer
        ser = JsonSerializer()
        for size_name, payload in payloads:
            results = []
            bench_lib(
                f"JsonSerializer ({size_name})",
                payload,
                lambda r, s=ser: s.decode(r),
                lambda o, s=ser: s.encode(o),
                results,
            )
            results_all.extend(results)
            progress(f"JsonSerializer ({size_name})", results)
    except Exception as e:
        results_all.append({"name": "JsonSerializer", "error": str(e)})
        step[0] += 3
        print(f"  JsonSerializer  ERROR: {e}", flush=True)
    # --- orjson ---
    try:
        import orjson
        for size_name, payload in payloads:
            results = []
            bench_lib(
                f"orjson ({size_name})",
                payload,
                orjson.loads,
                lambda o: orjson.dumps(o),
                results,
            )
            results_all.extend(results)
            progress(f"orjson ({size_name})", results)
    except ImportError:
        results_all.append({"name": "orjson", "error": "not installed"})
        step[0] += 3
        print(f"  orjson  SKIP: not installed", flush=True)
    # --- ujson ---
    try:
        import ujson
        for size_name, payload in payloads:
            results = []
            bench_lib(
                f"ujson ({size_name})",
                payload,
                ujson.loads,
                ujson.dumps,
                results,
            )
            results_all.extend(results)
            progress(f"ujson ({size_name})", results)
    except ImportError:
        results_all.append({"name": "ujson", "error": "not installed"})
        step[0] += 3
        print(f"  ujson  SKIP: not installed", flush=True)
    # --- stdlib json ---
    for size_name, payload in payloads:
        results = []
        bench_lib(
            f"stdlib json ({size_name})",
            payload,
            json.loads,
            json.dumps,
            results,
        )
        results_all.extend(results)
        progress(f"stdlib json ({size_name})", results)
    # --- rapidjson ---
    try:
        import rapidjson
        for size_name, payload in payloads:
            results = []
            bench_lib(
                f"rapidjson ({size_name})",
                payload,
                rapidjson.loads,
                rapidjson.dumps,
                results,
            )
            results_all.extend(results)
            progress(f"rapidjson ({size_name})", results)
    except ImportError:
        results_all.append({"name": "rapidjson", "error": "not installed"})
        step[0] += 3
        print(f"  rapidjson  SKIP: not installed", flush=True)
    # --- msgspec ---
    try:
        import msgspec
        for size_name, payload in payloads:
            results = []
            bench_lib(
                f"msgspec ({size_name})",
                payload,
                lambda r: msgspec.json.decode(r.encode("utf-8") if isinstance(r, str) else r),
                lambda o: msgspec.json.encode(o),
                results,
            )
            results_all.extend(results)
            progress(f"msgspec ({size_name})", results)
    except ImportError:
        results_all.append({"name": "msgspec", "error": "not installed"})
        step[0] += 3
        print(f"  msgspec  SKIP: not installed", flush=True)
    # --- simplejson ---
    try:
        import simplejson
        for size_name, payload in payloads:
            results = []
            bench_lib(
                f"simplejson ({size_name})",
                payload,
                simplejson.loads,
                simplejson.dumps,
                results,
            )
            results_all.extend(results)
            progress(f"simplejson ({size_name})", results)
    except ImportError:
        results_all.append({"name": "simplejson", "error": "not installed"})
        step[0] += 3
        print(f"  simplejson  SKIP: not installed", flush=True)
    # --- pysimdjson ---
    try:
        import simdjson
        parser = simdjson.Parser()
        for size_name, payload in payloads:
            raw = json.dumps(payload)
            results = []
            def load_simd(r):
                doc = parser.parse(r.encode("utf-8") if isinstance(r, str) else r)
                return doc.as_dict()
            bench_lib(
                f"pysimdjson ({size_name})",
                payload,
                load_simd,
                lambda o: json.dumps(o),
                results,
            )
            results_all.extend(results)
            progress(f"pysimdjson ({size_name})", results)
    except ImportError:
        results_all.append({"name": "pysimdjson", "error": "not installed"})
        step[0] += 3
        print(f"  pysimdjson  SKIP: not installed", flush=True)
    # --- hyperjson / hyperlight-hyperjson (fork with same API) ---
    try:
        try:
            import hyperjson
        except ImportError:
            import hyperlight_hyperjson as hyperjson
        for size_name, payload in payloads:
            results = []
            bench_lib(
                f"hyperjson ({size_name})",
                payload,
                hyperjson.loads,
                hyperjson.dumps,
                results,
            )
            results_all.extend(results)
            progress(f"hyperjson ({size_name})", results)
    except ImportError:
        results_all.append({"name": "hyperjson", "error": "not installed"})
        step[0] += 3
        print(f"  hyperjson  SKIP: not installed", flush=True)
    # --- Write results ---
    ts = datetime.now(timezone.utc).strftime("%Y%m%d_%H%M%S")
    results_json_path = BENCHMARKS_DIR / f"results_{ts}.json"
    bench_md_path = BENCHMARKS_DIR / f"BENCH_{ts}_json_comparison.md"
    with open(results_json_path, "w", encoding="utf-8") as f:
        json.dump({
            "timestamp": ts,
            "campaign": "20260209-benchmark xwsystem json vs others",
            "rounds": ROUNDS,
            "iterations_per_round": ITERATIONS_PER_ROUND,
            "warmup": WARMUP,
            "results": results_all,
        }, f, indent=2)
    print("-" * 70, flush=True)
    print("SUMMARY (warm decode / warm encode ops/s)", flush=True)
    print("-" * 70, flush=True)
    for r in results_all:
        if "error" in r:
            print(f"  {r['name']}: {r['error']}", flush=True)
        else:
            wdec = r.get("warm_decode_ops_per_sec", 0)
            wenc = r.get("warm_encode_ops_per_sec", 0)
            print(f"  {r['name']}:  decode {_fmt_ops(wdec)}/s   encode {_fmt_ops(wenc)}/s", flush=True)
    print("-" * 70, flush=True)
    print(f"  Results JSON: {results_json_path}", flush=True)
    # Markdown report
    lines = [
        f"# BENCH {ts} — JSON comparison",
        "",
        "## Summary",
        "",
        "| Library | Payload | Cold decode (ops/s) | Warm decode (ops/s) | Cold encode (ops/s) | Warm encode (ops/s) |",
        "|---------|---------|---------------------|---------------------|---------------------|---------------------|",
    ]
    for r in results_all:
        if "error" in r:
            lines.append(f"| {r['name']} | — | — | — | — | {r['error']} |")
        else:
            cdec = r.get("cold_decode_ops_per_sec", "")
            wdec = r.get("warm_decode_ops_per_sec", "")
            cenc = r.get("cold_encode_ops_per_sec", "")
            wenc = r.get("warm_encode_ops_per_sec", "")
            lines.append(f"| {r['name']} | — | {cdec} | {wdec} | {cenc} | {wenc} |")
    lines.extend([
        "",
        "## Raw data",
        "",
        f"- Full results: [results_{ts}.json](results_{ts}.json)",
        "",
    ])
    with open(bench_md_path, "w", encoding="utf-8") as f:
        f.write("\n".join(lines))
    # Update INDEX.md
    index_path = BENCHMARKS_DIR / "INDEX.md"
    index_path.parent.mkdir(parents=True, exist_ok=True)
    index_content = index_path.read_text(encoding="utf-8") if index_path.exists() else ""
    new_entry = (
        f"| {datetime.now(timezone.utc).strftime('%d-%b-%Y')} | "
        f"{datetime.now(timezone.utc).strftime('%H:%M:%S')} | See BENCH_* | JSON comparison | "
        f"[BENCH_{ts}_json_comparison.md](BENCH_{ts}_json_comparison.md) |\n"
    )
    if "| (No BENCH_*" in index_content:
        index_content = index_content.replace(
            "| (No BENCH_* reports recorded yet) | | | |",
            new_entry.strip(),
        )
    else:
        idx = index_content.find("| (No BENCH_*")
        if idx < 0:
            idx = index_content.find("|------|")
            if idx >= 0:
                insert = index_content.find("\n", idx + 1) + 1
                index_content = index_content[:insert] + new_entry + index_content[insert:]
        else:
            index_content = index_content.replace(
                "| (No BENCH_* reports recorded yet) | | | |",
                new_entry.strip(),
            )
    index_path.write_text(index_content, encoding="utf-8")
    print(f"  Report:      {bench_md_path}", flush=True)
    print("  Done.", flush=True)
if __name__ == "__main__":
    main()

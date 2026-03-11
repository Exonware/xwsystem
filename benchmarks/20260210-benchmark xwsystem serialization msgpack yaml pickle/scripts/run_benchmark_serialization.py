#!/usr/bin/env python3
"""
Serialization benchmark: MsgPackSerializer, YamlSerializer, PickleSerializer vs msgpack, PyYAML/ruamel.yaml, stdlib pickle.
Saves results to ../benchmarks/ (BENCH_*.md, results_*.json). Run from xwsystem root or from this script's directory.
"""

from __future__ import annotations
import json
import pickle
import sys
import time
from pathlib import Path
from datetime import datetime, timezone
# Ensure xwsystem package is importable (run from scripts/ or from xwsystem root)
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
ITERATIONS_PER_ROUND = 50
WARMUP = 10


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


def bench_serializer(name: str, payload: dict, load_fn, dump_fn, results: list[dict]) -> None:
    raw = dump_fn(payload)
    is_bytes = isinstance(raw, bytes)
    raw_for_decode = raw if is_bytes else raw.encode("utf-8") if isinstance(raw, str) else raw
    def decode_fn():
        return load_fn(raw_for_decode)
    cold_decode_ops = run_timed(decode_fn, ITERATIONS_PER_ROUND)
    for _ in range(WARMUP):
        decode_fn()
    warm_decode_ops = run_timed(decode_fn, ITERATIONS_PER_ROUND)
    obj = load_fn(raw_for_decode)
    def encode_fn():
        return dump_fn(obj)
    cold_encode_ops = run_timed(encode_fn, ITERATIONS_PER_ROUND)
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
    payloads = [
        ("small", make_payload_small()),
        ("medium", make_payload_medium()),
        ("large", make_payload_large()),
    ]
    # YAML/Pickle are slow on large payloads; use small+medium only for them
    payloads_small_medium = [payloads[0], payloads[1]]
    results_all: list[dict] = []
    step = [0]
    total_steps = 0  # count as we go
    def progress(name: str, results: list[dict]) -> None:
        step[0] += 1
        if results:
            r = results[-1]
            if "error" not in r:
                cdec = r.get("cold_decode_ops_per_sec", 0)
                wdec = r.get("warm_decode_ops_per_sec", 0)
                cenc = r.get("cold_encode_ops_per_sec", 0)
                wenc = r.get("warm_encode_ops_per_sec", 0)
                print(
                    f"  [{step[0]}] {name}  decode: cold={_fmt_ops(cdec)}/s warm={_fmt_ops(wdec)}/s  encode: cold={_fmt_ops(cenc)}/s warm={_fmt_ops(wenc)}/s",
                    flush=True,
                )
            else:
                print(f"  [{step[0]}] {name}  SKIP: {r['error']}", flush=True)
        else:
            print(f"  [{step[0]}] {name}  (no result)", flush=True)
    print("=" * 70, flush=True)
    print("SERIALIZATION BENCHMARK — MsgPack, YAML, Pickle (xwsystem vs others)", flush=True)
    print("=" * 70, flush=True)
    print(f"  Iterations: {ITERATIONS_PER_ROUND} per round, warmup: {WARMUP}", flush=True)
    print(f"  Output: {BENCHMARKS_DIR}", flush=True)
    print("-" * 70, flush=True)
    # --- MsgPack: xwsystem MsgPackSerializer ---
    try:
        from exonware.xwsystem.io.serialization.formats.binary.msgpack import MsgPackSerializer
        ser = MsgPackSerializer()
        for size_name, payload in payloads:
            results = []
            bench_serializer(
                f"MsgPackSerializer ({size_name})",
                payload,
                lambda r, s=ser: s.decode(r),
                lambda o, s=ser: s.encode(o),
                results,
            )
            results_all.extend(results)
            progress(f"MsgPackSerializer ({size_name})", results)
        total_steps += 3
    except Exception as e:
        results_all.append({"name": "MsgPackSerializer", "error": str(e)})
        step[0] += 3
        print(f"  MsgPackSerializer  ERROR: {e}", flush=True)
    # --- MsgPack: msgpack lib ---
    try:
        import msgpack
        for size_name, payload in payloads:
            results = []
            bench_serializer(
                f"msgpack ({size_name})",
                payload,
                msgpack.unpackb,
                msgpack.packb,
                results,
            )
            results_all.extend(results)
            progress(f"msgpack ({size_name})", results)
        total_steps += 3
    except ImportError:
        results_all.append({"name": "msgpack", "error": "not installed"})
        step[0] += 3
        print("  msgpack  SKIP: not installed", flush=True)
    # --- YAML: xwsystem YamlSerializer (small+medium only; large is very slow) ---
    try:
        from exonware.xwsystem.io.serialization.formats.text.yaml import YamlSerializer
        ser = YamlSerializer()
        for size_name, payload in payloads_small_medium:
            results = []
            bench_serializer(
                f"YamlSerializer ({size_name})",
                payload,
                lambda r, s=ser: s.decode(r.decode("utf-8") if isinstance(r, bytes) else r),
                lambda o, s=ser: s.encode(o),
                results,
            )
            results_all.extend(results)
            progress(f"YamlSerializer ({size_name})", results)
        total_steps += 2
    except Exception as e:
        results_all.append({"name": "YamlSerializer", "error": str(e)})
        step[0] += 2
        print(f"  YamlSerializer  ERROR: {e}", flush=True)
    # --- YAML: PyYAML (small+medium only) ---
    try:
        import yaml
        def yaml_load(r):
            if isinstance(r, bytes):
                r = r.decode("utf-8")
            return yaml.safe_load(r)
        def yaml_dump(o):
            return yaml.dump(o, default_flow_style=False)
        for size_name, payload in payloads_small_medium:
            results = []
            bench_serializer(
                f"PyYAML ({size_name})",
                payload,
                yaml_load,
                yaml_dump,
                results,
            )
            results_all.extend(results)
            progress(f"PyYAML ({size_name})", results)
        total_steps += 2
    except ImportError:
        results_all.append({"name": "PyYAML", "error": "not installed"})
        step[0] += 2
        print("  PyYAML  SKIP: not installed", flush=True)
    # --- YAML: ruamel.yaml (small+medium only) ---
    try:
        from ruamel.yaml import YAML
        yaml_ruamel = YAML()
        def ruamel_load(r):
            from io import BytesIO
            if isinstance(r, bytes):
                r = BytesIO(r)
            return yaml_ruamel.load(r)
        def ruamel_dump(o):
            from io import StringIO
            s = StringIO()
            yaml_ruamel.dump(o, s)
            return s.getvalue()
        for size_name, payload in payloads_small_medium:
            results = []
            bench_serializer(
                f"ruamel.yaml ({size_name})",
                payload,
                ruamel_load,
                ruamel_dump,
                results,
            )
            results_all.extend(results)
            progress(f"ruamel.yaml ({size_name})", results)
        total_steps += 2
    except ImportError:
        results_all.append({"name": "ruamel.yaml", "error": "not installed"})
        step[0] += 2
        print("  ruamel.yaml  SKIP: not installed", flush=True)
    # --- Pickle: xwsystem PickleSerializer (small+medium only) ---
    try:
        from exonware.xwsystem.io.serialization.formats.binary.pickle import PickleSerializer
        ser = PickleSerializer()
        for size_name, payload in payloads_small_medium:
            results = []
            bench_serializer(
                f"PickleSerializer ({size_name})",
                payload,
                lambda r, s=ser: s.decode(r),
                lambda o, s=ser: s.encode(o),
                results,
            )
            results_all.extend(results)
            progress(f"PickleSerializer ({size_name})", results)
        total_steps += 2
    except Exception as e:
        results_all.append({"name": "PickleSerializer", "error": str(e)})
        step[0] += 2
        print(f"  PickleSerializer  ERROR: {e}", flush=True)
    # --- Pickle: stdlib (small+medium only) ---
    for size_name, payload in payloads_small_medium:
        results = []
        bench_serializer(
            f"stdlib pickle ({size_name})",
            payload,
            pickle.loads,
            pickle.dumps,
            results,
        )
        results_all.extend(results)
        progress(f"stdlib pickle ({size_name})", results)
    total_steps += 2
    # --- Write results ---
    ts = datetime.now(timezone.utc).strftime("%Y%m%d_%H%M%S")
    results_json_path = BENCHMARKS_DIR / f"results_{ts}.json"
    bench_md_path = BENCHMARKS_DIR / f"BENCH_{ts}_serialization_comparison.md"
    with open(results_json_path, "w", encoding="utf-8") as f:
        json.dump({
            "timestamp": ts,
            "campaign": "20260210-benchmark xwsystem serialization msgpack yaml pickle",
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
    lines = [
        f"# BENCH {ts} — Serialization comparison (MsgPack, YAML, Pickle)",
        "",
        "## Summary",
        "",
        "| Library | Payload | Cold decode | Warm decode | Cold encode | Warm encode |",
        "|---------|---------|--------------|-------------|-------------|-------------|",
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
    index_path = BENCHMARKS_DIR / "INDEX.md"
    index_content = index_path.read_text(encoding="utf-8") if index_path.exists() else ""
    new_entry = (
        f"| {datetime.now(timezone.utc).strftime('%d-%b-%Y')} | "
        f"{datetime.now(timezone.utc).strftime('%H:%M:%S')} | See BENCH_* | Serialization comparison | "
        f"[BENCH_{ts}_serialization_comparison.md](BENCH_{ts}_serialization_comparison.md) |\n"
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
    print(f"  Report:      {bench_md_path}", flush=True)
    print("  Done.", flush=True)
if __name__ == "__main__":
    main()

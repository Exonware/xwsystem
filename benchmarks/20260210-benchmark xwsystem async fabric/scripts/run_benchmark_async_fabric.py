#!/usr/bin/env python3
"""
Async Process Fabric benchmark scaffold (REF_54_BENCH).
Measures: task submission throughput, publish→consume latency, shared memory
create+write / reuse. Writes results to ../benchmarks/. Run from xwsystem root.
"""

from __future__ import annotations
import asyncio
import json
import sys
import time
from pathlib import Path
from datetime import datetime, timezone
_SCRIPT_DIR = Path(__file__).resolve().parent
_CAMPAIGN_ROOT = _SCRIPT_DIR.parent
_XWSYSTEM_ROOT = _CAMPAIGN_ROOT.parent.parent
_src = _XWSYSTEM_ROOT / "src"
if _src.exists() and str(_XWSYSTEM_ROOT) not in sys.path:
    sys.path.insert(0, str(_XWSYSTEM_ROOT))
if str(_XWSYSTEM_ROOT) not in sys.path:
    sys.path.insert(0, str(_XWSYSTEM_ROOT))
BENCHMARKS_DIR = _CAMPAIGN_ROOT / "benchmarks"
DATA_DIR = _CAMPAIGN_ROOT / "data"
NUM_LIGHTWEIGHT_TASKS = 1000
SHARED_MEMORY_SIZE = 256 * 1024  # 256 KB


def _noop_task() -> int:
    """Lightweight task for submission throughput."""
    return 1
async def _bench_submit_throughput():
    """Submit NUM_LIGHTWEIGHT_TASKS and return ops/s (target ≥ 5,000)."""
    try:
        from exonware.xwsystem.ipc.async_fabric import AsyncProcessFabric
    except ImportError as e:
        return {"error": str(e), "ops_per_sec": None}
    fabric = AsyncProcessFabric()
    start = time.perf_counter()
    async with fabric.session() as session:
        for i in range(NUM_LIGHTWEIGHT_TASKS):
            await session.submit(_noop_task, task_id=f"t_{i}")
        # Allow pool to process; get_results would block until done
        await asyncio.sleep(0)
    elapsed = time.perf_counter() - start
    return {"ops_per_sec": round(NUM_LIGHTWEIGHT_TASKS / elapsed, 2), "num_tasks": NUM_LIGHTWEIGHT_TASKS}
async def _bench_queue_latency():
    """Publish→Consume round-trip latency in ms (target < 5 ms)."""
    try:
        from exonware.xwsystem.ipc.async_fabric import AsyncProcessFabric
    except ImportError as e:
        return {"error": str(e), "latency_ms": None}
    fabric = AsyncProcessFabric()
    latencies_ms = []
    async with fabric.session() as session:
        for _ in range(50):
            t0 = time.perf_counter()
            await session.publish("bench", {"ts": t0}, timeout=1.0)
            msg = await session.consume("bench", timeout=1.0)
            t1 = time.perf_counter()
            if msg is not None:
                latencies_ms.append((t1 - t0) * 1000)
    if not latencies_ms:
        return {"error": "no round-trips", "latency_ms": None}
    return {"latency_ms": round(sum(latencies_ms) / len(latencies_ms), 3), "samples": len(latencies_ms)}
async def _bench_shared_memory():
    """Segment create+write (256 KB) and reuse read (targets < 2 ms / < 1 ms)."""
    try:
        from exonware.xwsystem.ipc.async_fabric import AsyncProcessFabric
    except ImportError as e:
        return {"error": str(e), "create_write_ms": None, "reuse_read_ms": None}
    fabric = AsyncProcessFabric()
    payload = b"x" * SHARED_MEMORY_SIZE
    create_write_ms = None
    reuse_read_ms = None
    async with fabric.session() as session:
        try:
            t0 = time.perf_counter()
            seg = session.share("bench_seg", size=SHARED_MEMORY_SIZE, create_if_missing=True)
            seg.set(payload)
            create_write_ms = (time.perf_counter() - t0) * 1000
            t0 = time.perf_counter()
            _ = seg.get()
            reuse_read_ms = (time.perf_counter() - t0) * 1000
        finally:
            session.release_shared("bench_seg")
    return {
        "create_write_ms": round(create_write_ms, 3) if create_write_ms is not None else None,
        "reuse_read_ms": round(reuse_read_ms, 3) if reuse_read_ms is not None else None,
    }


def main():
    BENCHMARKS_DIR.mkdir(parents=True, exist_ok=True)
    DATA_DIR.mkdir(parents=True, exist_ok=True)
    print("=" * 70, flush=True)
    print("ASYNC PROCESS FABRIC BENCHMARK (REF_54_BENCH scaffold)", flush=True)
    print("=" * 70, flush=True)
    results = {}
    # Task submission
    r = asyncio.run(_bench_submit_throughput())
    results["submit_throughput"] = r
    if "error" in r:
        print(f"  Submit throughput: SKIP ({r['error']})", flush=True)
    else:
        print(f"  Submit throughput: {r.get('ops_per_sec')} ops/s ({r.get('num_tasks')} tasks)", flush=True)
    # Queue latency
    r = asyncio.run(_bench_queue_latency())
    results["queue_latency"] = r
    if "error" in r:
        print(f"  Queue latency: SKIP ({r['error']})", flush=True)
    else:
        print(f"  Queue latency: {r.get('latency_ms')} ms (P50-style)", flush=True)
    # Shared memory
    r = asyncio.run(_bench_shared_memory())
    results["shared_memory"] = r
    if "error" in r:
        print(f"  Shared memory: SKIP ({r['error']})", flush=True)
    else:
        print(f"  Shared memory create+write: {r.get('create_write_ms')} ms", flush=True)
        print(f"  Shared memory reuse read: {r.get('reuse_read_ms')} ms", flush=True)
    ts = datetime.now(timezone.utc).strftime("%Y%m%d_%H%M%S")
    results_json_path = BENCHMARKS_DIR / f"results_{ts}.json"
    bench_md_path = BENCHMARKS_DIR / f"BENCH_{ts}_async_fabric.md"
    with open(results_json_path, "w", encoding="utf-8") as f:
        json.dump({"timestamp": ts, "campaign": "20260210-benchmark xwsystem async fabric", "results": results}, f, indent=2)
    with open(bench_md_path, "w", encoding="utf-8") as f:
        f.write(f"# BENCH {ts} — Async Process Fabric\n\n## Summary\n\n")
        f.write("| Scenario | Result |\n|----------|--------|\n")
        for name, data in results.items():
            if isinstance(data, dict) and "error" in data:
                f.write(f"| {name} | SKIP: {data['error']} |\n")
            else:
                f.write(f"| {name} | {data} |\n")
        f.write("\n## Raw data\n\n")
        f.write(f"- [results_{ts}.json](results_{ts}.json)\n")
    print("-" * 70, flush=True)
    print(f"  Results: {results_json_path}", flush=True)
    print("  Done.", flush=True)
if __name__ == "__main__":
    main()

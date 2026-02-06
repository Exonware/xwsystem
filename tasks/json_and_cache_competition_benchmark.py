#!/usr/bin/env python3
#xwsystem/tasks/json_and_cache_competition_benchmark.py
"""
JSON & Caching Competition Benchmark

Purpose:
- Provide a FAIR, REPEATABLE benchmark for freelancers.
- Compare:
  1) Existing `JsonSerializer` vs new `CandidateJsonSerializer`
  2) Existing `OptimizedLFUCache` vs new `CandidateCache`

Freelancer goals:
- Implement `CandidateJsonSerializer` so that it:
  * Is functionally correct on the test payloads
  * Outperforms `JsonSerializer` in encode+decode throughput
- Implement `CandidateCache` so that it:
  * Is functionally correct on cache workloads
  * Outperforms `OptimizedLFUCache` on hot read/write scenarios

IMPORTANT:
- The candidate implementations live in this folder:
  * `xwsystem/tasks/candidate_json.py`
  * `xwsystem/tasks/candidate_cache.py`
- They are intentionally empty skeletons. The freelancer's task is to
  implement them WITHOUT reusing the existing implementations.

Company: eXonware.com
Author: ENG-AI (freelancer evaluation scaffold)
Version: 0.0.0
Generation Date: 06-Feb-2026
"""

from __future__ import annotations

import sys
import time
from dataclasses import dataclass, asdict
from typing import Any, Callable

from exonware.xwsystem.io.serialization.formats.text.json import JsonSerializer
from exonware.xwsystem.caching.lfu_optimized import OptimizedLFUCache

from candidate_json import CandidateJsonSerializer
from candidate_cache import CandidateCache


# ======================================================================
# DATA MODELS
# ======================================================================


@dataclass
class PerfResult:
    """Simple performance result for one implementation."""

    name: str
    kind: str  # "serializer" or "cache"
    operations_per_second: float
    duration_seconds: float
    iterations: int
    success: bool
    error: str | None = None


# ======================================================================
# HELPER: BENCHMARK RUNNER
# ======================================================================


def _time_loop(iterations: int, fn: Callable[[], Any]) -> tuple[float, bool, str | None]:
    """
    Run `fn` `iterations` times and return (ops_per_sec, success, error).

    Handles `NotImplementedError` and other exceptions gracefully so that
    the benchmark can still produce comparable output for the baseline
    implementation even if the candidate is incomplete.
    """
    start = time.perf_counter()
    try:
        for _ in range(iterations):
            fn()
    except NotImplementedError as e:
        duration = time.perf_counter() - start
        return 0.0, False, f"NotImplementedError: {e}"
    except Exception as e:  # pragma: no cover - diagnostic path
        duration = time.perf_counter() - start
        return 0.0, False, f"{type(e).__name__}: {e}"

    duration = time.perf_counter() - start
    if duration <= 0:
        return 0.0, True, None
    return float(iterations) / duration, True, None


# ======================================================================
# JSON SERIALIZER BENCHMARKS
# ======================================================================


def _build_json_payloads() -> list[dict[str, Any]]:
    """Representative JSON payloads for XSystem scenarios."""
    small = {
        "name": "small",
        "id": 1,
        "tags": ["xwsystem", "test"],
        "active": True,
    }
    medium = {
        "name": "medium",
        "id": 2,
        "items": [{"index": i, "value": f"value_{i}"} for i in range(128)],
        "config": {
            "debug": True,
            "timeout": 30,
            "retries": 5,
        },
    }
    large = {
        "name": "large",
        "id": 3,
        "matrix": [[float(i * j) for j in range(32)] for i in range(32)],
        "metadata": {
            "project": "XSystem",
            "author": "eXonware",
            "version": "1.0.0",
            "features": ["serialization", "caching", "monitoring", "security"],
        },
    }
    return [small, medium, large]


def benchmark_serializer(
    label: str,
    serializer_factory: Callable[[], Any],
    iterations: int = 10_000,
) -> PerfResult:
    """
    Benchmark encode+decode cycles for a given serializer factory.

    We intentionally combine encode+decode in a single loop to reflect
    typical end‑to‑end usage.
    """
    payloads = _build_json_payloads()
    serializer = serializer_factory()

    # Pre‑encode once to ensure any internal warmup happens outside timing
    for p in payloads:
        _ = serializer.encode(p)

    def op() -> None:
        for p in payloads:
            encoded = serializer.encode(p)
            decoded = serializer.decode(encoded)
            # Sanity check: basic invariant on round‑trip
            if not isinstance(decoded, dict) or "id" not in decoded:
                raise AssertionError("Invalid decode result in serializer benchmark")

    ops_per_sec, success, error = _time_loop(iterations, op)
    duration = iterations * len(payloads) / ops_per_sec if ops_per_sec > 0 else 0.0

    return PerfResult(
        name=label,
        kind="serializer",
        operations_per_second=ops_per_sec,
        duration_seconds=duration,
        iterations=iterations * len(payloads),
        success=success,
        error=error,
    )


def run_json_benchmarks(iterations: int = 5_000) -> list[PerfResult]:
    """Run benchmarks for built‑in and candidate JSON serializers."""
    results: list[PerfResult] = []

    # Baseline: built‑in JsonSerializer
    results.append(
        benchmark_serializer("JsonSerializer (baseline)", JsonSerializer, iterations),
    )

    # Candidate: freelancer implementation
    results.append(
        benchmark_serializer("CandidateJsonSerializer", CandidateJsonSerializer, iterations),
    )

    return results


# ======================================================================
# CACHE BENCHMARKS
# ======================================================================


def benchmark_cache(
    label: str,
    cache_factory: Callable[[], Any],
    *,
    key_space: int = 10_000,
    iterations: int = 50_000,
) -> PerfResult:
    """
    Benchmark put/get workload for a cache implementation.

    Workload:
    - Mixture of puts and gets over a fixed key space
    - Hot keys are revisited frequently to stress hit‑rate behaviour
    """
    cache = cache_factory()

    def op() -> None:
        # Simple mixed read/write pattern
        for i in range(128):
            key = f"key_{i % key_space}"
            cache.put(key, f"value_{i}")
            _ = cache.get(key)

    ops_per_sec, success, error = _time_loop(iterations, op)
    duration = iterations * 128 / ops_per_sec if ops_per_sec > 0 else 0.0

    return PerfResult(
        name=label,
        kind="cache",
        operations_per_second=ops_per_sec,
        duration_seconds=duration,
        iterations=iterations * 128,
        success=success,
        error=error,
    )


def run_cache_benchmarks(iterations: int = 10_000) -> list[PerfResult]:
    """Run benchmarks for baseline vs candidate cache implementations."""
    results: list[PerfResult] = []

    # Baseline: OptimizedLFUCache (current fastest implementation)
    results.append(
        benchmark_cache(
            "OptimizedLFUCache (baseline)",
            lambda: OptimizedLFUCache(capacity=4_096),
            iterations=iterations,
        ),
    )

    # Candidate: freelancer implementation
    results.append(
        benchmark_cache(
            "CandidateCache",
            lambda: CandidateCache(capacity=4_096),
            iterations=iterations,
        ),
    )

    return results


# ======================================================================
# ENTRYPOINT
# ======================================================================


def main() -> None:
    """Run all competition benchmarks and print a human‑readable report."""
    print("=" * 80)
    print("XWSYSTEM JSON & CACHE COMPETITION BENCHMARK")
    print("=" * 80)
    print(f"Python: {sys.version}")
    print("-" * 80)

    serializer_results = run_json_benchmarks()
    cache_results = run_cache_benchmarks()

    all_results: list[PerfResult] = serializer_results + cache_results

    # Pretty print results
    for result in all_results:
        status = "OK" if result.success else "FAIL"
        print(
            f"[{status}] {result.kind.upper():10s} | {result.name:30s} "
            f"| {result.operations_per_second:12,.2f} ops/sec"
        )
        if result.error:
            print(f"       ERROR: {result.error}")

    print("-" * 80)
    print("Raw results (for CI / further analysis):")
    for result in all_results:
        print(asdict(result))


if __name__ == "__main__":  # pragma: no cover - manual execution path
    main()


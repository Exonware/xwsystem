# 20260210 — Benchmark: xwsystem Async Process Fabric

Scaffold harness for REF_54_BENCH Async Process Fabric SLAs (task submission, queue latency, shared memory).

**Scenarios (REF_54_BENCH):**
- Task submission throughput: submit 1k lightweight tasks (target ≥ 5,000 ops/s)
- Queue latency: Publish→Consume round-trip (target < 5 ms)
- Shared memory: segment create+write 256 KB, reuse read (targets < 2 ms / < 1 ms)

**Run:**
```bash
# From xwsystem root
python "benchmarks/20260210-benchmark xwsystem async fabric/scripts/run_benchmark_async_fabric.py"
```

Results go to `benchmarks/` (BENCH_*.md, results_*.json). See [REF_54_BENCH](../../docs/REF_54_BENCH.md) for SLA status.

---

## Results

**Last run:** 10-Feb-2026 (BENCH_20260210_203221)

| Scenario | Result |
|----------|--------|
| submit_throughput | 3274 ops/s (1000 tasks) |
| queue_latency | 0.019 ms (50 samples) |
| shared_memory | create+write 160.09 ms, reuse read 0.006 ms |

**Report:** [benchmarks/BENCH_20260210_203221_async_fabric.md](benchmarks/BENCH_20260210_203221_async_fabric.md) · **Data:** [benchmarks/results_20260210_203221.json](benchmarks/results_20260210_203221.json)

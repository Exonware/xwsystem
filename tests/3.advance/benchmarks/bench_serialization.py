"""
#exonware/xwsystem/tests/3.advance/benchmarks/bench_serialization.py

Benchmarks for xwsystem serialization.

WHY: Verify serialization performance meets SLAs defined in docs/REF_BENCH.md.
"""

from __future__ import annotations

from exonware.xwsystem import JsonSerializer


def test_benchmark_json_roundtrip_small(benchmark):
    """
    SLA (placeholder): keep mean under agreed baseline for small payloads.
    Evidence: docs/logs/benchmarks/
    """
    s = JsonSerializer()
    payload = {"ok": True, "v": [1, 2, 3], "name": "alice"}

    def _roundtrip() -> dict:
        return s.loads(s.dumps(payload))

    result = benchmark(_roundtrip)
    assert result["ok"] is True


def test_benchmark_json_roundtrip_medium(benchmark):
    """
    SLA (placeholder): no regression >10% vs baseline.
    """
    s = JsonSerializer()
    payload = {"items": [{"id": i, "name": f"user-{i}", "active": (i % 2 == 0)} for i in range(2_000)]}

    def _roundtrip() -> dict:
        return s.loads(s.dumps(payload))

    result = benchmark(_roundtrip)
    assert len(result["items"]) == 2_000


#!/usr/bin/env python3
"""
#exonware/xwsystem/src/exonware/xwsystem/monitoring/metrics_reader.py

Read-only host metrics (CPU / memory / disk / network) + per-process
snapshots. Wraps :mod:`psutil` when available and gracefully degrades
to stdlib-only readings when it isn't.

Exposed as a single :func:`read_snapshot` call that returns a JSON-safe
dict — suitable for dashboards and API responses.
"""

from __future__ import annotations

import os
import shutil
import time
from dataclasses import asdict, dataclass, field
from pathlib import Path
from typing import Any


@dataclass(slots=True)
class HostMetrics:
    ts: float
    cpu_percent: float = 0.0
    cpu_count: int = 0
    load_avg_1m: float = 0.0
    load_avg_5m: float = 0.0
    load_avg_15m: float = 0.0
    mem_total_bytes: int = 0
    mem_used_bytes: int = 0
    mem_percent: float = 0.0
    disk_total_bytes: int = 0
    disk_used_bytes: int = 0
    disk_percent: float = 0.0
    net_bytes_sent: int = 0
    net_bytes_recv: int = 0
    proc_rss_bytes: int = 0
    proc_open_files: int = 0
    proc_threads: int = 0
    backend: str = "stdlib"

    def to_dict(self) -> dict[str, Any]:
        return asdict(self)


def _stdlib_fallback(disk_root: str) -> HostMetrics:
    try:
        load = os.getloadavg() if hasattr(os, "getloadavg") else (0.0, 0.0, 0.0)
    except Exception:
        load = (0.0, 0.0, 0.0)
    try:
        disk = shutil.disk_usage(disk_root)
        disk_tuple = (disk.total, disk.used, (disk.used / disk.total) * 100.0 if disk.total else 0.0)
    except Exception:
        disk_tuple = (0, 0, 0.0)
    return HostMetrics(
        ts=time.time(),
        cpu_count=os.cpu_count() or 0,
        load_avg_1m=load[0], load_avg_5m=load[1], load_avg_15m=load[2],
        disk_total_bytes=disk_tuple[0],
        disk_used_bytes=disk_tuple[1],
        disk_percent=round(disk_tuple[2], 2),
        backend="stdlib",
    )


def read_snapshot(*, disk_root: str | Path = "/") -> HostMetrics:
    """Return a fresh host metrics snapshot.

    ``disk_root`` picks which mount we report usage for. Default "/" works
    on POSIX; on Windows we fall back to the drive containing cwd.
    """
    root = str(disk_root)
    if os.name == "nt":
        root = os.path.splitdrive(os.getcwd())[0] + "\\"

    try:
        import psutil  # type: ignore[import-not-found]
    except ImportError:
        return _stdlib_fallback(root)

    proc = psutil.Process(os.getpid())
    with proc.oneshot():
        try:
            num_files = len(proc.open_files())
        except Exception:
            num_files = 0
        proc_mem = proc.memory_info().rss
        proc_threads = proc.num_threads()

    mem = psutil.virtual_memory()
    disk = psutil.disk_usage(root)
    net = psutil.net_io_counters()
    try:
        load = os.getloadavg()
    except Exception:
        load = (0.0, 0.0, 0.0)

    return HostMetrics(
        ts=time.time(),
        cpu_percent=float(psutil.cpu_percent(interval=None)),
        cpu_count=psutil.cpu_count(logical=True) or 0,
        load_avg_1m=load[0], load_avg_5m=load[1], load_avg_15m=load[2],
        mem_total_bytes=int(mem.total),
        mem_used_bytes=int(mem.used),
        mem_percent=round(float(mem.percent), 2),
        disk_total_bytes=int(disk.total),
        disk_used_bytes=int(disk.used),
        disk_percent=round(float(disk.percent), 2),
        net_bytes_sent=int(net.bytes_sent),
        net_bytes_recv=int(net.bytes_recv),
        proc_rss_bytes=int(proc_mem),
        proc_open_files=int(num_files),
        proc_threads=int(proc_threads),
        backend="psutil",
    )


__all__ = ["HostMetrics", "read_snapshot"]

#!/usr/bin/env python3
"""
#exonware/xwsystem/benchmarks/20260321-benchmark xwsystem advance legacy/scripts/runner.py
Benchmark runner for xwsystem (pytest-benchmark).
Outputs:
- benchmarks/<campaign>/benchmarks/BENCH_YYYYMMDD_HHMM_DESCRIPTION.md (GUIDE_54_BENCH)
This is the canonical benchmark entry point used before release.
"""

from __future__ import annotations
import sys
import subprocess
from datetime import datetime
from pathlib import Path
from exonware.xwsystem.console.cli import ensure_utf8_console
from exonware.xwsystem.utils.test_runner import DualOutput, format_path


def _update_benchmarks_index(*, logs_dir: Path, report_file: Path, ok: bool) -> None:
    index_path = logs_dir / "INDEX.md"
    if not index_path.exists():
        return
    stamp = report_file.name.removeprefix("BENCH_").split("_", 2)
    date_part = stamp[0] if stamp else ""
    time_part = stamp[1] if len(stamp) > 1 else ""
    date_fmt = f"{date_part[0:4]}-{date_part[4:6]}-{date_part[6:8]}" if len(date_part) == 8 else date_part
    time_fmt = f"{time_part[0:2]}:{time_part[2:4]}" if len(time_part) >= 4 else time_part
    result = "✅ PASSED" if ok else "❌ FAILED"
    row = f"| {date_fmt} | {time_fmt} | {result} | Advance benchmarks | [{report_file.name}]({report_file.name}) |"
    lines = index_path.read_text(encoding="utf-8", errors="replace").splitlines()
    out: list[str] = []
    inserted = False
    for i, ln in enumerate(lines):
        out.append(ln)
        if not inserted and ln.strip().startswith("|------|") and i >= 1 and lines[i - 1].strip().startswith("| Date | Time |"):
            out.append(row)
            inserted = True
    if inserted:
        index_path.write_text("\n".join(out) + "\n", encoding="utf-8")


def main() -> int:
    ensure_utf8_console()
    repo_root = Path(__file__).resolve().parents[3]
    bench_dir = Path(__file__).parent
    campaign_root = bench_dir.parent
    logs_dir = campaign_root / "benchmarks"
    logs_dir.mkdir(parents=True, exist_ok=True)
    storage_dir = campaign_root / "data" / "baseline"
    storage_dir.mkdir(parents=True, exist_ok=True)
    timestamp = datetime.now().strftime("%Y%m%d_%H%M")
    output_file = logs_dir / f"BENCH_{timestamp}_ADVANCE_BENCHMARKS.md"
    output = DualOutput(output_file)
    output.print("# Benchmark Execution Report", "# Benchmark Execution Report")
    output.print(f"**Library:** xwsystem", f"**Library:** xwsystem")
    output.print(f"**Generated:** {datetime.now().strftime('%d-%b-%Y %H:%M:%S')}", "")
    output.print(f"**Bench Directory:** {format_path(bench_dir)}", f"**Bench Directory:** `{format_path(bench_dir)}`")
    output.print("---", "---")
    baseline_name = "baseline"
    # Decide baseline behavior:
    # - If no baseline snapshot exists yet, save one.
    # - If it exists, compare against it and fail on >10% mean regression.
    #
    # NOTE: `storage_dir` may also contain legacy baseline artifacts under `_legacy/`;
    # those must not be treated as pytest-benchmark snapshots.
    has_any_baseline = (storage_dir / baseline_name).exists()
    cmd = [
        sys.executable,
        "-m",
        "pytest",
        str(bench_dir / "bench_serialization.py"),
        "--benchmark-only",
        f"--benchmark-storage={storage_dir}",
    ]
    if has_any_baseline:
        cmd.extend(
            [
                f"--benchmark-compare={baseline_name}",
                "--benchmark-compare-fail=mean:10%",
            ]
        )
    else:
        cmd.append(f"--benchmark-save={baseline_name}")
    output.print(f"Command: {' '.join(cmd)}", f"**Command:** `{ ' '.join(cmd) }`")
    result = subprocess.run(
        cmd,
        cwd=str(repo_root),
        capture_output=True,
        text=True,
        encoding="utf-8",
        errors="replace",
    )
    if result.stdout:
        output.print(result.stdout, f"```\\n{result.stdout}\\n```")
    if result.stderr:
        output.print(result.stderr, f"**Stderr:**\\n```\\n{result.stderr}\\n```")
    ok = result.returncode == 0
    output.print("✅ PASSED" if ok else "❌ FAILED", f"**Result:** {'✅ PASSED' if ok else '❌ FAILED'}")
    output.save({"library": "xwsystem", "layer": "benchmarks", "runner": "BenchmarkRunner"})
    print(f"💾 Benchmark results saved to: {format_path(output_file)}")
    _update_benchmarks_index(logs_dir=logs_dir, report_file=output_file, ok=ok)
    return result.returncode
if __name__ == "__main__":
    raise SystemExit(main())

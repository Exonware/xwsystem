#!/usr/bin/env python3
"""
#exonware/xwsystem/tests/runner.py

Main test runner for xwsystem
Coordinates all test layers and records a single Markdown summary under docs/tests.

Company: eXonware.com
Author: Eng. Muhammad AlShehri
Email: connect@exonware.com
Version: 0.0.2
Generation Date: 2026-01-24

Usage:
 python tests/runner.py # Run all tests
 python tests/runner.py --core # Run only core tests
 python tests/runner.py --unit # Run only unit tests
 python tests/runner.py --integration # Run only integration tests
 python tests/runner.py --advance # Run only advance tests (v1.0.0+)
 python tests/runner.py --security # Run only security tests
 python tests/runner.py --performance # Run only performance tests

Output:
 - Terminal: Colored, formatted output
 - File: docs/logs/tests/TEST_<timestamp>_SUMMARY.md (Markdown-friendly format)
"""

import sys
import subprocess
from pathlib import Path
from datetime import datetime

from exonware.xwsystem.console.cli import ensure_utf8_console
from exonware.xwsystem.utils.test_runner import (
    DualOutput,
    format_path,
    print_header,
    print_section,
    print_status,
)

ensure_utf8_console()

# Lib installed editable; no manual src path setup
test_dir = Path(__file__).parent


def _update_summary_test(*, repo_root: Path, report_file: Path, ok: bool) -> None:
    """
    Update `docs/logs/SUMMARY_TEST.md` with the latest run pointer.

    This is a lightweight summary; the detailed evidence remains in `docs/logs/tests/`.
    """
    summary_path = repo_root / "docs" / "logs" / "SUMMARY_TEST.md"
    summary_path.parent.mkdir(parents=True, exist_ok=True)

    now = datetime.now()
    status = "✅ PASSED" if ok else "❌ FAILED"
    report_rel = f"tests/{report_file.name}"

    content = (
        "# Test Execution Log - exonware-xwsystem\n\n"
        "**Library:** exonware-xwsystem  \n"
        "**Auto-generated:** Updated by `tests/runner.py`  \n"
        f"**Last Updated:** {now.strftime('%d-%b-%Y %H:%M:%S')}  \n\n"
        "---\n\n"
        "## 📊 Latest Test Run\n\n"
        f"**Date:** {now.strftime('%d-%b-%Y %H:%M:%S')}  \n"
        f"**Status:** {status}  \n"
        f"**Report:** [{report_file.name}]({report_rel})  \n\n"
        "---\n\n"
        "## 🔗 Links\n\n"
        f"- Test reports index: [docs/logs/tests/INDEX.md](tests/INDEX.md)\n"
        f"- Test status: [docs/REPORT_TEST.md](../REPORT_TEST.md)\n"
    )

    summary_path.write_text(content, encoding="utf-8")


def _update_tests_index(*, repo_root: Path, report_file: Path, run_type: str) -> None:
    """Insert a new row into `docs/logs/tests/INDEX.md`."""
    index_path = repo_root / "docs" / "logs" / "tests" / "INDEX.md"
    if not index_path.exists():
        return

    stamp = report_file.name.removeprefix("TEST_").removesuffix("_SUMMARY.md")
    # Expected: YYYYMMDD_HHMM
    date_part, time_part = (stamp.split("_", 1) + [""])[:2]
    date_fmt = f"{date_part[0:4]}-{date_part[4:6]}-{date_part[6:8]}" if len(date_part) == 8 else date_part
    time_fmt = f"{time_part[0:2]}:{time_part[2:4]}" if len(time_part) >= 4 else time_part

    row = f"| {date_fmt} | {time_fmt} |  | {run_type} | Main runner summary | [{report_file.name}]({report_file.name}) |"

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


def run_sub_runner(
    runner_path: Path,
    description: str,
    output: DualOutput,
    *,
    extra_args: list[str] | None = None,
) -> int:
    """Run a sub-runner and return exit code. Optional extra_args passed to the runner."""
    separator = "=" * 80
    output.print(f"\n{separator}", f"\n## {description}\n")
    output.print(f"🚀 {description}", f"**Status:** Running...")
    output.print(f"{separator}\n", "")

    cmd = [sys.executable, str(runner_path)]
    if extra_args:
        cmd.extend(extra_args)
    result = subprocess.run(
        cmd,
        cwd=runner_path.parent,
        capture_output=True,
        text=True,
        encoding="utf-8",
        errors="replace",
    )

    # Print sub-runner output
    if result.stdout:
        output.print(result.stdout, f"```\n{result.stdout}\n```")
    if result.stderr:
        output.print(result.stderr, f"**Errors:**\n```\n{result.stderr}\n```")

    # Status
    status = "✅ PASSED" if result.returncode == 0 else "❌ FAILED"
    output.print(f"\n{status}", f"\n**Result:** {status}")

    return result.returncode


def main():
    """Main test runner function following GUIDE_DEV.md."""
    # Setup output logger
    reports_dir = test_dir.parent / "docs" / "logs" / "tests"
    reports_dir.mkdir(parents=True, exist_ok=True)
    timestamp = datetime.now().strftime("%Y%m%d_%H%M")
    output_file = reports_dir / f"TEST_{timestamp}_SUMMARY.md"
    output = DualOutput(output_file)

    # Header
    header = "=" * 80
    output.print(header, "# Test Execution Report")
    output.print("xwsystem Test Runner",
           f"**Library:** xwsystem \n**Type:** Main Orchestrator - Hierarchical Test Execution")
    output.print("Main Orchestrator - Hierarchical Test Execution", "")
    output.print(header, "---")

    # Parse arguments
    args = sys.argv[1:]

    # Define sub-runners
    core_runner = test_dir / "0.core" / "runner.py"
    unit_runner = test_dir / "1.unit" / "runner.py"
    integration_runner = test_dir / "2.integration" / "runner.py"
    advance_runner = test_dir / "3.advance" / "runner.py"

    exit_codes = []

    # Determine which tests to run
    if "--core" in args:
        if core_runner.exists():
            exit_codes.append(run_sub_runner(core_runner, "Core Tests", output))
        else:
            output.print("⚠️ Core runner not found", "> ⚠️ Core runner not found")

    elif "--unit" in args:
        if unit_runner.exists():
            exit_codes.append(run_sub_runner(unit_runner, "Unit Tests", output))
        else:
            output.print("⚠️ Unit runner not found", "> ⚠️ Unit runner not found")

    elif "--integration" in args:
        if integration_runner.exists():
            exit_codes.append(run_sub_runner(integration_runner, "Integration Tests", output))
        else:
            output.print("⚠️ Integration runner not found", "> ⚠️ Integration runner not found")

    elif "--advance" in args:
        if advance_runner.exists():
            exit_codes.append(run_sub_runner(advance_runner, "Advance Tests", output))
        else:
            msg = "\n⚠️ Advance tests not available (requires v1.0.0)"
            output.print(msg, f"\n> {msg}")

    elif "--security" in args or "--performance" in args or "--usability" in args or "--maintainability" in args or "--extensibility" in args:
        # Forward to advance runner; use run_sub_runner so output is captured in report
        if advance_runner.exists():
            kind = "Security" if "--security" in args else "Performance" if "--performance" in args else "Usability" if "--usability" in args else "Maintainability" if "--maintainability" in args else "Extensibility"
            exit_codes.append(
                run_sub_runner(advance_runner, f"Advance Tests ({kind})", output, extra_args=args)
            )
        else:
            msg = "\n⚠️ Advance tests not available (requires v1.0.0)"
            output.print(msg, f"\n> {msg}")

    else:
        # Run all tests in sequence
        msg_header = "\n🚀 Running: ALL Tests"
        msg_layers = " Layers: 0.core ✅ 1.unit ✅ 2.integration ✅ 3.advance"
        output.print(msg_header, "\n## Running All Test Layers")
        output.print(msg_layers, f"\n**Execution Order:** 0.core ✅ 1.unit ✅ 2.integration ✅ 3.advance\n")
        output.print("", "")

        # Core tests
        if core_runner.exists():
            exit_codes.append(run_sub_runner(core_runner, "Layer 0: Core Tests", output))

        # Unit tests
        if unit_runner.exists():
            exit_codes.append(run_sub_runner(unit_runner, "Layer 1: Unit Tests", output))

        # Integration tests
        if integration_runner.exists():
            exit_codes.append(run_sub_runner(integration_runner, "Layer 2: Integration Tests", output))

        # Advance tests (if available)
        if advance_runner.exists():
            exit_codes.append(run_sub_runner(advance_runner, "Layer 3: Advance Tests", output))

    # Print summary
    summary_header = f"\n{'='*80}"
    output.print(summary_header, f"\n---\n\n## 📈 Test Execution Summary")
    output.print("📈 TEST EXECUTION SUMMARY", "")
    output.print(f"{'='*80}", "")

    total_runs = len(exit_codes)
    passed = sum(1 for code in exit_codes if code == 0)
    failed = total_runs - passed

    output.print(f"Total Layers: {total_runs}", f"- **Total Layers:** {total_runs}")
    output.print(f"Passed: {passed}", f"- **Passed:** {passed}")
    output.print(f"Failed: {failed}", f"- **Failed:** {failed}")

    # Final status
    if all(code == 0 for code in exit_codes):
        final_msg = "\n✅ ALL TESTS PASSED!"
        output.print(final_msg, f"\n### {final_msg}")

        # Save output
        output.save({'library': 'xwsystem', 'layer': 'main', 'description': 'Main orchestrator run'})
        print(f"\n💾 Test results saved to: {format_path(output_file)}")
        _update_summary_test(repo_root=test_dir.parent, report_file=output_file, ok=True)
        _update_tests_index(repo_root=test_dir.parent, report_file=output_file, run_type="Full suite" if not args else "Partial")

        sys.exit(0)
    else:
        final_msg = "\n❌ SOME TESTS FAILED!"
        output.print(final_msg, f"\n### {final_msg}")

        # Save output
        output.save({'library': 'xwsystem', 'layer': 'main', 'description': 'Main orchestrator run'})
        print(f"\n💾 Test results saved to: {format_path(output_file)}")
        _update_summary_test(repo_root=test_dir.parent, report_file=output_file, ok=False)
        _update_tests_index(repo_root=test_dir.parent, report_file=output_file, run_type="Full suite" if not args else "Partial")

        sys.exit(1)


if __name__ == "__main__":
    main()

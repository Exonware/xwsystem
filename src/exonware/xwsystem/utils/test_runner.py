#!/usr/bin/env python3
"""
#exonware/xwsystem/src/exonware/xwsystem/utils/test_runner.py
Reusable pytest runner utilities for all eXonware libraries.
Implements the hierarchical runner utilities described in:
- docs/guides/GUIDE_DEV.md
- docs/guides/GUIDE_TEST.md
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.9.0.34
Generation Date: 28-Dec-2025
"""

from __future__ import annotations
import sys
import subprocess
from dataclasses import dataclass
from datetime import datetime
from pathlib import Path
from typing import Any
from exonware.xwsystem.console.cli import ensure_utf8_console


def timestamp_for_filename() -> str:
    """
    Return timestamp for time-sensitive log/test/review file names.
    Format: YYYYMMDD_HHMMSS_mmm (hour, minute, second, millisecond).
    Per GUIDE_00_MASTER to avoid duplicate names when multiple runs occur the same day.
    """
    now = datetime.now()
    return now.strftime("%Y%m%d_%H%M%S") + "_" + f"{now.microsecond // 1000:03d}"


def format_path(path: Path) -> str:
    """Format a path as a full absolute path string."""
    return str(path.resolve())


class DualOutput:
    """
    Capture output for terminal and Markdown file simultaneously.
    """

    def __init__(self, output_file: Path):
        self.output_file = output_file
        self._markdown_lines: list[str] = []

    def print(self, text: str, markdown_format: str | None = None, *, emoji: str | None = None) -> None:
        prefix = f"{emoji} " if emoji else ""
        line = f"{prefix}{text}"
        print(line)
        self._markdown_lines.append(markdown_format if markdown_format is not None else line)

    def save(self, header_info: dict[str, Any] | None = None) -> None:
        header_info = header_info or {}
        now = datetime.now()
        header = (
            "# Test Runner Output\n\n"
            f"**Library:** {header_info.get('library', '')}  \n"
            f"**Layer:** {header_info.get('layer', '')}  \n"
            f"**Generated:** {now.strftime('%d-%b-%Y %H:%M:%S')}.{now.microsecond // 1000:03d}  \n"
            f"**Runner:** {header_info.get('runner', 'TestRunner')}  \n\n"
            "---\n\n"
        )
        self.output_file.write_text(header + "\n".join(self._markdown_lines) + "\n", encoding="utf-8")


def print_header(title: str, output: DualOutput | None = None) -> None:
    sep = "=" * 80
    if output is None:
        print(sep)
        print(title)
        print(sep)
        return
    output.print(sep)
    output.print(title)
    output.print(sep)


def print_section(title: str, output: DualOutput | None = None) -> None:
    if output is None:
        print(f"\n{'=' * 80}\n{title}\n{'=' * 80}\n")
        return
    output.print(f"\n{title}", f"\n## {title}\n")


def print_status(success: bool, message: str, output: DualOutput | None = None) -> None:
    emoji = "✅" if success else "❌"
    if output is None:
        print(f"{emoji} {message}")
        return
    output.print(message, f"**Result:** {emoji} {message}", emoji=emoji)
@dataclass(frozen=True)

class PytestRunResult:
    exit_code: int


def run_pytest(
    *,
    test_dir: Path,
    markers: list[str] | None = None,
    extra_args: list[str] | None = None,
    cwd: Path | None = None,
) -> PytestRunResult:
    """
    Run pytest as a subprocess with standard eXonware flags.
    """
    markers = markers or []
    extra_args = extra_args or []
    cwd = cwd or test_dir
    args: list[str] = [
        sys.executable,
        "-m",
        "pytest",
        str(test_dir),
        "-v",
        "--tb=short",
        "-x",
        "--strict-markers",
    ]
    if markers:
        args.extend(["-m", " and ".join(markers)])
    args.extend(extra_args)
    completed = subprocess.run(args, cwd=str(cwd))
    return PytestRunResult(exit_code=int(completed.returncode))


class TestRunner:
    """
    Simple reusable test runner for a single directory/layer.
    """

    def __init__(
        self,
        *,
        library_name: str,
        layer_name: str,
        description: str,
        test_dir: Path,
        markers: list[str] | None = None,
        output_file: Path | None = None,
        pytest_cwd: Path | None = None,
    ):
        self.library_name = library_name
        self.layer_name = layer_name
        self.description = description
        self.test_dir = test_dir
        self.markers = markers or []
        # When set, pytest subprocess uses this cwd (e.g. project root for pytest.ini / src layout).
        self.pytest_cwd = pytest_cwd
        # Layer runners should pass output_file=None to not write files
        # Only main runner writes to docs/logs/tests/
        self.output_file = output_file
        if self.output_file is None:
            # Layer runner - create dummy output that won't save
            # Use a temporary path that we'll ignore
            import tempfile
            dummy_file = Path(tempfile.gettempdir()) / "xwsystem_test_runner_dummy.md"
            self.output = DualOutput(dummy_file)
            self._is_layer_runner = True
        else:
            self.output = DualOutput(self.output_file)
            self._is_layer_runner = False

    def run(self) -> int:
        ensure_utf8_console()
        print_header(self.description, self.output)
        self.output.print(
            f"Directory: {format_path(self.test_dir)}",
            f"**Directory:** `{format_path(self.test_dir)}`",
            emoji="📂",
        )
        cwd = self.pytest_cwd if self.pytest_cwd is not None else self.test_dir
        result = run_pytest(test_dir=self.test_dir, markers=self.markers, cwd=cwd)
        ok = result.exit_code == 0
        print_status(ok, "PASSED" if ok else "FAILED", self.output)
        # Layer runners don't write files - only main runner writes to docs/logs/tests/
        if not self._is_layer_runner:
            self.output.save(
                {
                    "library": self.library_name,
                    "layer": self.layer_name,
                    "runner": "TestRunner",
                }
            )
        return result.exit_code

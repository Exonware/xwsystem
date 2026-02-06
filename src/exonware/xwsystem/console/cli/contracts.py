#exonware/xwsystem/src/exonware/xwsystem/console/cli/contracts.py
"""
Company: eXonware.com
Author: Eng. Muhammad AlShehri
Email: connect@exonware.com
Version: 0.1.0.4
Generation Date: September 04, 2025

CLI module contracts - interfaces and enums for command-line interface functionality.
"""

from typing import Protocol, runtime_checkable
from typing import Any, Optional

# Import general console enums from console level (priority)
from ..defs import Alignment, BorderStyle, Colors, Style
# Import CLI-specific enums from defs
from .defs import (
    ColorType,
    ProgressStyle,
    TableStyle,
    PromptType,
    ArgumentType
)


@runtime_checkable
class IConsole(Protocol):
    """Interface for console operations."""
    
    def print(self, text: str, color: Optional[ColorType] = None, **kwargs) -> None:
        """Print text to console."""
        ...
    
    def input(self, prompt: str, **kwargs) -> str:
        """Get input from user."""
        ...
    
    def clear(self) -> None:
        """Clear console screen."""
        ...


@runtime_checkable
class IProgressBar(Protocol):
    """Interface for progress bar operations."""
    
    def start(self, total: int, description: str = "") -> None:
        """Start progress bar."""
        ...
    
    def update(self, increment: int = 1) -> None:
        """Update progress."""
        ...
    
    def finish(self) -> None:
        """Finish progress bar."""
        ...


@runtime_checkable
class ITable(Protocol):
    """Interface for table operations."""
    
    def add_row(self, *values: Any) -> None:
        """Add row to table."""
        ...
    
    def render(self) -> str:
        """Render table as string."""
        ...


@runtime_checkable
class IPrompt(Protocol):
    """Interface for user prompts."""
    
    def ask(self, question: str, **kwargs) -> Any:
        """Ask user a question."""
        ...


@runtime_checkable
class IArgumentParser(Protocol):
    """Interface for argument parsing."""
    
    def add_argument(self, *args, **kwargs) -> None:
        """Add argument to parser."""
        ...
    
    def parse_args(self, args: Optional[list[str]] = None) -> Any:
        """Parse command line arguments."""
        ...


@runtime_checkable
class ICLI(Protocol):
    """Interface for CLI operations."""
    
    @property
    def name(self) -> str:
        """Get CLI name."""
        ...
    
    @property
    def version(self) -> str:
        """Get CLI version."""
        ...
    
    def add_command(self, name: str, command: Any) -> None:
        """Add a command to the CLI."""
        ...
    
    def add_option(self, name: str, option: Any) -> None:
        """Add an option to the CLI."""
        ...
    
    def run(self, args: Optional[list[str]] = None) -> int:
        """Run the CLI."""
        ...
    
    def get_help(self) -> str:
        """Get help text."""
        ...


@runtime_checkable
class IProgress(Protocol):
    """Interface for progress operations."""
    
    def start(self, total: int, description: str = "") -> None:
        """Start progress tracking."""
        ...
    
    def update(self, increment: int = 1) -> None:
        """Update progress."""
        ...
    
    def finish(self) -> None:
        """Finish progress tracking."""
        ...


@runtime_checkable
class IPrompts(Protocol):
    """Interface for user prompts."""
    
    def ask(self, question: str, **kwargs) -> Any:
        """Ask user a question."""
        ...
    
    def confirm(self, message: str, default: bool = False) -> bool:
        """Ask for confirmation."""
        ...
    
    def select(self, message: str, choices: list[str], default: Optional[str] = None) -> str:
        """Ask user to select from choices."""
        ...


@runtime_checkable
class ITableFormatter(Protocol):
    """Interface for table formatting."""
    
    def add_row(self, *values: Any) -> None:
        """Add row to table."""
        ...
    
    def render(self) -> str:
        """Render table as string."""
        ...
    
    def clear(self) -> None:
        """Clear all rows."""
        ...
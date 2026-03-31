#exonware/xwsystem/src/exonware/xwsystem/console/cli/__init__.py
"""
Command Line Interface (CLI) Utilities
======================================
Production-grade CLI utilities for xwsystem.
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.9.0.31
Generated: 2025-01-27
"""
# Import general console enums from console level (priority)

from ..defs import Colors, Style, Alignment, BorderStyle
from .colors import colorize, CliColoredOutput
from .args import CliArgumentParser, CliArgument, CliCommand, ArgumentType
from .progress import CliProgressBar, CliSpinnerProgress, CliMultiProgress, CliProgressConfig
from .tables import CliTable, CliTableFormatter, CliColumn
from .console import CliConsole
from .event_logger import CliEventLogger
from .encoding import ensure_utf8_console
__all__ = [
    # General console enums (from console level)
    'Colors', 
    'Style',
    'Alignment',
    'BorderStyle',
    # Colors
    'colorize',
    'CliColoredOutput',
    # Arguments
    'CliArgumentParser',
    'CliArgument',
    'CliCommand',
    'ArgumentType',
    # Progress
    'CliProgressBar',
    'CliSpinnerProgress', 
    'CliMultiProgress',
    'CliProgressConfig',
    # Tables
    'CliTable',
    'CliTableFormatter',
    'CliColumn',
    # Console (extends ConsoleWriter from console level)
    'CliConsole',
    # Event Logger (extends logging.Logger)
    'CliEventLogger',
    # Encoding
    'ensure_utf8_console',
]

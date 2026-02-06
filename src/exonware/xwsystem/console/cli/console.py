#exonware/xwsystem/src/exonware/xwsystem/console/cli/console.py
"""
Company: eXonware.com
Author: Eng. Muhammad AlShehri
Email: connect@exonware.com
Version: 0.1.0.5
Generation Date: September 04, 2025

Console utilities for CLI operations.
"""

import os
from typing import Optional
from .contracts import IConsole, ColorType
from .errors import ConsoleError
# Import ConsoleWriter from console level (priority)
from ..writer import ConsoleWriter


class CliConsole(ConsoleWriter, IConsole):
    """
    Console implementation for CLI operations.
    
    Extends ConsoleWriter from console level and adds CLI-specific features
    like color support and IConsole interface compliance.
    """
    
    def print(self, text: str, color: Optional[ColorType] = None, **kwargs) -> None:
        """
        Print text to console with optional color.
        
        Args:
            text: Text to print
            color: Optional color type to apply
            **kwargs: Additional arguments passed to print()
        """
        try:
            if color and self.supports_color():
                text = self._apply_color(text, color)
            
            # Use parent's write_line method
            self.write_line(text, **kwargs)
        except Exception as e:
            raise ConsoleError(f"Failed to print to console: {e}")
    
    def input(self, prompt: str, **kwargs) -> str:
        """
        Get input from user.
        
        Args:
            prompt: Prompt text
            **kwargs: Additional arguments (for compatibility)
            
        Returns:
            User input string
        """
        try:
            # Use parent's read method
            return self.read(prompt)
        except Exception as e:
            raise ConsoleError(f"Failed to get input: {e}")
    
    def _apply_color(self, text: str, color: ColorType) -> str:
        """Apply color to text."""
        color_codes = {
            ColorType.RESET: '\033[0m',
            ColorType.BOLD: '\033[1m',
            ColorType.DIM: '\033[2m',
            ColorType.UNDERLINE: '\033[4m',
            ColorType.RED: '\033[31m',
            ColorType.GREEN: '\033[32m',
            ColorType.YELLOW: '\033[33m',
            ColorType.BLUE: '\033[34m',
            ColorType.MAGENTA: '\033[35m',
            ColorType.CYAN: '\033[36m',
            ColorType.WHITE: '\033[37m',
            ColorType.GRAY: '\033[90m',
        }
        
        code = color_codes.get(color, '')
        return f"{code}{text}\033[0m" if code else text
    
    def get_size(self) -> tuple[int, int]:
        """
        Get console size.
        
        Returns:
            Tuple of (columns, lines)
        """
        try:
            if hasattr(os, 'get_terminal_size'):
                size = os.get_terminal_size()
                return (size.columns, size.lines)
            else:
                return (80, 24)  # Default size
        except (OSError, AttributeError, RuntimeError):
            # Catch specific exceptions - terminal size may not be available
            return (80, 24)
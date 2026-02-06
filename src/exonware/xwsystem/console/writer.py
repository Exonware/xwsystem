#!/usr/bin/env python3
#exonware/xwsystem/src/exonware/xwsystem/console/writer.py
#exonware/xwsystem/console/writer.py
"""
Company: eXonware.com
Author: Eng. Muhammad AlShehri
Email: connect@exonware.com
Version: 0.1.0.3
Generation Date: 2025-01-27

Console Writer implementation for user interaction (not logging).
Can be used as a base class for bots and other user interaction systems.
"""

import os
import platform
import sys
from typing import Optional
from .base import AConsoleWriter
from .errors import ConsoleWriterError


class ConsoleWriter(AConsoleWriter):
    """
    Console writer for user interaction (not logging).
    
    This class provides methods for writing to and reading from the console.
    Can be used as a base class for bots and other user interaction systems.
    
    Example:
        writer = ConsoleWriter()
        writer.write_line("Hello, user!")
        name = writer.read("Enter your name: ")
    """
    
    def __init__(self):
        """Initialize console writer."""
        self._supports_color = self._check_color_support()
        self._is_interactive = self._check_interactive()
    
    def _check_color_support(self) -> bool:
        """Check if terminal supports color."""
        try:
            return (
                hasattr(sys.stdout, 'isatty') and 
                sys.stdout.isatty() and 
                os.getenv('TERM') != 'dumb' and
                os.getenv('NO_COLOR') is None
            )
        except (AttributeError, OSError, RuntimeError):
            # Catch specific exceptions - stdout may not support isatty
            return False
    
    def _check_interactive(self) -> bool:
        """Check if console is interactive."""
        try:
            return hasattr(sys.stdin, 'isatty') and sys.stdin.isatty()
        except (AttributeError, OSError, RuntimeError):
            # Catch specific exceptions - stdin may not support isatty
            return False
    
    def write(self, text: str, **kwargs) -> None:
        """
        Write text to console.
        
        Args:
            text: Text to write
            **kwargs: Additional arguments passed to print()
        """
        try:
            print(text, end='', **kwargs)
            sys.stdout.flush()
        except Exception as e:
            raise ConsoleWriterError(f"Failed to write to console: {e}")
    
    def write_line(self, text: str = "", **kwargs) -> None:
        """
        Write a line to console.
        
        Args:
            text: Text to write (default: empty string)
            **kwargs: Additional arguments passed to print()
        """
        try:
            print(text, **kwargs)
            sys.stdout.flush()
        except Exception as e:
            raise ConsoleWriterError(f"Failed to write line to console: {e}")
    
    def read(self, prompt: str = "") -> str:
        """
        Read input from console.
        
        Args:
            prompt: Prompt text to display before reading
            
        Returns:
            User input string
            
        Raises:
            ConsoleWriterError: If console is not interactive
        """
        try:
            if not self._is_interactive:
                raise ConsoleWriterError("Console is not interactive")
            
            return input(prompt)
        except Exception as e:
            raise ConsoleWriterError(f"Failed to read from console: {e}")
    
    def clear(self) -> None:
        """Clear console screen using Python's native platform detection."""
        try:
            if platform.system() == 'Windows':
                os.system('cls')
            else:  # Unix/Linux/macOS
                os.system('clear')
        except Exception as e:
            raise ConsoleWriterError(f"Failed to clear console: {e}")
    
    def is_interactive(self) -> bool:
        """Check if console is interactive."""
        return self._is_interactive
    
    def supports_color(self) -> bool:
        """Check if console supports color."""
        return self._supports_color


__all__ = [
    'ConsoleWriter',
]

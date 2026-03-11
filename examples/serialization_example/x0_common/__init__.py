#exonware/xwsystem/examples/serialization_example/x0_common/__init__.py
"""
Common Utilities for Serialization Examples
Shared code for all serialization examples.
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.0.1
Generation Date: October 12, 2025
"""

from .data_generator import generate_sample_data, generate_large_dataset
from .test_helpers import print_result, compare_performance, save_to_file
__all__ = [
    'generate_sample_data',
    'generate_large_dataset',
    'print_result',
    'compare_performance',
    'save_to_file',
]

#exonware/xwsystem/src/exonware/xwsystem/io/serialization/formats/__init__.py
"""
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.9.0.17
Generation Date: November 2, 2025
Serialization formats - Format subpackages.
Subpackages: .text, .binary, .database, .tabular (pandas). Tabular is not
imported here so that .text / .binary / .database load without pandas.
Import tabular explicitly: from ...formats.tabular import ExcelSerializer, ...
"""

__all__: list[str] = []

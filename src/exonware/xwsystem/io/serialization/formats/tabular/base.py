#exonware/xwsystem/src/exonware/xwsystem/io/serialization/formats/tabular/base.py
"""
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.9.0.18
Generation Date: January 2025
Tabular serialization base class - ATabularSerialization.
Extends ASerialization to provide DataFrame conversion capabilities.
All tabular formats (Excel, CSV, Google Sheets, DataFrame) extend this.
"""

from abc import ABC, abstractmethod
from typing import Any
import pandas as pd
from ...base import ASerialization
from ....contracts import EncodeOptions, DecodeOptions


class ATabularSerialization(ASerialization, ABC):
    """
    Abstract base class for tabular data formats.
    Extends ASerialization to provide DataFrame conversion capabilities.
    All tabular formats (Excel, CSV, Google Sheets, DataFrame) extend this.
    Key features:
    - Convert to/from pandas DataFrame(s)
    - Support multiple sheets (returns dict of sheet_name: DataFrame)
    - Integrate with xwsystem serialization framework
    Examples:
        >>> # Excel serializer
        >>> excel_serializer = ExcelSerializer()
        >>> df = excel_serializer.to_df(excel_bytes)
        >>> excel_bytes = excel_serializer.from_df(df)
        >>> # CSV serializer
        >>> csv_serializer = CsvSerializer()
        >>> df = csv_serializer.to_df(csv_str)
        >>> csv_str = csv_serializer.from_df(df)
    """
    @abstractmethod

    def to_df(
        self, 
        data: bytes | str | dict, 
        sheet_name: str | list[str] | None = None,
        **options
    ) -> pd.DataFrame | dict[str, pd.DataFrame]:
        """
        Convert tabular data to DataFrame(s).
        Args:
            data: Input data (bytes, str, or dict depending on format)
            sheet_name: Specific sheet name(s) to load, or None for all sheets
            **options: Format-specific options
        Returns:
            Single DataFrame if one sheet, or dict of {sheet_name: DataFrame} if multiple
        Raises:
            SerializationError: If conversion fails
        """
        pass
    @abstractmethod

    def from_df(
        self, 
        df: pd.DataFrame | dict[str, pd.DataFrame], 
        **options
    ) -> bytes | str:
        """
        Convert DataFrame(s) to tabular format.
        Args:
            df: Single DataFrame or dict of {sheet_name: DataFrame}
            **options: Format-specific options
        Returns:
            Serialized data (bytes for binary formats, str for text formats)
        Raises:
            SerializationError: If conversion fails
        """
        pass

#exonware/xwsystem/src/exonware/xwsystem/io/serialization/formats/tabular/excel.py
"""
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.9.0.34
Generation Date: January 2025
Excel serialization - .xlsx, .xls format.
Following I→A→XW pattern:
- I: ISerialization (interface)
- A: ASerialization (abstract base)
- ATabular: ATabularSerialization (tabular base)
- Concrete: ExcelSerializer
"""

import io
import os
import tempfile
from typing import Any
from pathlib import Path
import pandas as pd
from .base import ATabularSerialization
from ....contracts import EncodeOptions, DecodeOptions
from ....defs import CodecCapability
from ....errors import SerializationError


class ExcelSerializer(ATabularSerialization):
    """
    Excel serializer - follows the I→A→ATabular pattern.
    I: ISerialization (interface)
    A: ASerialization (abstract base)
    ATabular: ATabularSerialization (tabular base)
    Concrete: ExcelSerializer
    Supports .xlsx and .xls formats via pandas/openpyxl/xlrd.
    Examples:
        >>> serializer = ExcelSerializer()
        >>> 
        >>> # Convert Excel bytes to DataFrame
        >>> df = serializer.to_df(excel_bytes)
        >>> 
        >>> # Convert DataFrame to Excel bytes
        >>> excel_bytes = serializer.from_df(df)
        >>> 
        >>> # Save to file
        >>> serializer.save_file(df, "data.xlsx")
        >>> 
        >>> # Load from file
        >>> df = serializer.load_file("data.xlsx")
    """
    # ========================================================================
    # CODEC METADATA
    # ========================================================================
    @property

    def codec_id(self) -> str:
        return "excel"
    @property

    def media_types(self) -> list[str]:
        return [
            "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",  # .xlsx
            "application/vnd.ms-excel",  # .xls
        ]
    @property

    def file_extensions(self) -> list[str]:
        return [".xlsx", ".xls"]
    @property

    def format_name(self) -> str:
        return "Excel"
    @property

    def mime_type(self) -> str:
        return "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"
    @property

    def is_binary_format(self) -> bool:
        return True  # Excel is binary
    @property

    def supports_streaming(self) -> bool:
        return False  # Excel doesn't naturally support streaming
    @property

    def capabilities(self) -> CodecCapability:
        return CodecCapability.BIDIRECTIONAL
    @property

    def aliases(self) -> list[str]:
        return ["excel", "Excel", "xlsx", "XLSX", "xls", "XLS"]
    @property

    def codec_types(self) -> list[str]:
        """Excel is a data exchange format."""
        return ["data", "tabular"]
    # ========================================================================
    # CORE ENCODE/DECODE (Using pandas)
    # ========================================================================

    def encode(self, value: Any, *, options: EncodeOptions | None = None) -> bytes:
        """
        Encode data to Excel bytes.
        Args:
            value: DataFrame or dict of {sheet_name: DataFrame}
            options: Excel options (engine, sheet_name, etc.)
        Returns:
            Excel file as bytes
        Raises:
            SerializationError: If encoding fails
        """
        try:
            # If value is already a DataFrame or dict, use from_df
            if isinstance(value, (pd.DataFrame, dict)):
                return self.from_df(value, **(options or {}))
            # Otherwise, try to serialize as-is (fallback)
            raise SerializationError(
                f"Excel encoding requires DataFrame or dict of DataFrames, got {type(value)}",
                format_name=self.format_name
            )
        except Exception as e:
            raise SerializationError(
                f"Failed to encode Excel: {e}",
                format_name=self.format_name,
                original_error=e
            )

    def decode(self, repr: bytes | str, *, options: DecodeOptions | None = None) -> Any:
        """
        Decode Excel bytes to DataFrame(s).
        Args:
            repr: Excel file as bytes
            options: Excel options (sheet_name, engine, etc.)
        Returns:
            DataFrame or dict of {sheet_name: DataFrame}
        Raises:
            SerializationError: If decoding fails
        """
        try:
            if isinstance(repr, str):
                # If string, treat as file path
                return self.load_file(repr, **(options or {}))
            # Use to_df for bytes
            return self.to_df(repr, **(options or {}))
        except Exception as e:
            raise SerializationError(
                f"Failed to decode Excel: {e}",
                format_name=self.format_name,
                original_error=e
            )
    # ========================================================================
    # TABULAR METHODS (to_df/from_df)
    # ========================================================================

    def to_df(
        self, 
        excel_content: bytes | str | Path, 
        sheet_name: str | list[str] | None = None,
        **options
    ) -> pd.DataFrame | dict[str, pd.DataFrame]:
        """
        Convert Excel content to DataFrame(s).
        Args:
            excel_content: Excel file as bytes, file path, or Path object
            sheet_name: Specific sheet name(s) to load, or None for all sheets
            **options: Additional pandas read_excel options (engine, header, etc.)
        Returns:
            Single DataFrame if one sheet, or dict of {sheet_name: DataFrame} if multiple
        Examples:
            >>> serializer = ExcelSerializer()
            >>> df = serializer.to_df(excel_bytes)  # Single sheet
            >>> sheets = serializer.to_df(excel_bytes)  # All sheets (dict)
            >>> df = serializer.to_df(excel_bytes, sheet_name="Sheet1")  # Specific sheet
        """
        try:
            # Handle file path
            if isinstance(excel_content, (str, Path)):
                file_path = str(excel_content)
                if sheet_name is None:
                    # Read all sheets
                    return pd.read_excel(file_path, sheet_name=None, **options)
                else:
                    # Read specific sheet(s)
                    return pd.read_excel(file_path, sheet_name=sheet_name, **options)
            # Handle bytes
            if not excel_content:
                raise ValueError("Excel content cannot be empty")
            excel_io = io.BytesIO(excel_content)
            excel_io.seek(0)  # Ensure position at start (helps avoid seek-related issues)
            # Determine engine based on options or file extension
            engine = options.get('engine', 'openpyxl')  # Default to openpyxl for .xlsx
            filter_opts = {k: v for k, v in options.items() if k != 'engine'}
            try:
                if sheet_name is None:
                    return pd.read_excel(excel_io, sheet_name=None, engine=engine, **filter_opts)
                result = pd.read_excel(excel_io, sheet_name=sheet_name, engine=engine, **filter_opts)
                return result if isinstance(sheet_name, str) else result
            except OSError as e:
                # Windows Errno 22 (Invalid argument) can occur when openpyxl uses temp files
                # e.g. under OneDrive or with long paths. Fallback to temp file.
                if e.errno == 22 or (hasattr(e, 'winerror') and e.winerror == 22):
                    return self._to_df_via_tempfile(excel_content, sheet_name, engine, filter_opts)
                raise
        except SerializationError:
            raise
        except Exception as e:
            raise SerializationError(
                f"Failed to convert Excel to DataFrame: {e}",
                format_name=self.format_name,
                original_error=e
            )

    def _to_df_via_tempfile(
        self,
        excel_content: bytes,
        sheet_name: str | list[str] | None,
        engine: str,
        options: dict,
    ) -> pd.DataFrame | dict[str, pd.DataFrame]:
        """
        Fallback: write bytes to temp file and read. Avoids Errno 22 on Windows
        when BytesIO/openpyxl temp extraction hits OneDrive or path length limits.
        """
        fd, path = tempfile.mkstemp(suffix='.xlsx')
        try:
            try:
                os.write(fd, excel_content)
            finally:
                os.close(fd)
            # Use short path: mkstemp uses system temp dir
            if sheet_name is None:
                return pd.read_excel(path, sheet_name=None, engine=engine, **options)
            result = pd.read_excel(path, sheet_name=sheet_name, engine=engine, **options)
            return result if isinstance(sheet_name, str) else result
        finally:
            try:
                os.unlink(path)
            except OSError:
                pass

    def from_df(
        self, 
        df: pd.DataFrame | dict[str, pd.DataFrame], 
        **options
    ) -> bytes:
        """
        Convert DataFrame(s) to Excel bytes.
        Args:
            df: Single DataFrame or dict of {sheet_name: DataFrame}
            **options: Additional pandas to_excel options (engine, index, etc.)
        Returns:
            Excel file as bytes
        Examples:
            >>> serializer = ExcelSerializer()
            >>> excel_bytes = serializer.from_df(df)  # Single sheet
            >>> excel_bytes = serializer.from_df({"Sheet1": df1, "Sheet2": df2})  # Multiple sheets
        """
        try:
            excel_io = io.BytesIO()
            # Determine engine
            engine = options.get('engine', 'openpyxl')  # Default to openpyxl
            if isinstance(df, dict):
                # Multiple sheets
                with pd.ExcelWriter(excel_io, engine=engine) as writer:
                    for sheet_name, sheet_df in df.items():
                        sheet_df.to_excel(
                            writer, 
                            sheet_name=sheet_name,
                            index=options.get('index', False),
                            **{k: v for k, v in options.items() if k not in ('engine', 'index')}
                        )
            else:
                # Single sheet
                df.to_excel(
                    excel_io,
                    engine=engine,
                    index=options.get('index', False),
                    sheet_name=options.get('sheet_name', 'Sheet1'),
                    **{k: v for k, v in options.items() if k not in ('engine', 'index', 'sheet_name')}
                )
            excel_io.seek(0)
            return excel_io.read()
        except Exception as e:
            raise SerializationError(
                f"Failed to convert DataFrame to Excel: {e}",
                format_name=self.format_name,
                original_error=e
            )

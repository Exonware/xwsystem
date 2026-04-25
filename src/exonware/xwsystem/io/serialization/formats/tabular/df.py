#exonware/xwsystem/src/exonware/xwsystem/io/serialization/formats/tabular/df.py
"""
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.9.0.43
Generation Date: January 2025
DataFrame serialization - Direct pandas DataFrame operations.
Following I→A→ATabular pattern:
- I: ISerialization (interface)
- A: ASerialization (abstract base)
- ATabular: ATabularSerialization (tabular base)
- Concrete: DataFrameSerializer
Useful for intermediate DataFrame processing and format conversion.
"""

from typing import Any
from pathlib import Path
import pandas as pd
from .base import ATabularSerialization
from ....contracts import EncodeOptions, DecodeOptions
from ....defs import CodecCapability
from ....errors import SerializationError


class DataFrameSerializer(ATabularSerialization):
    """
    DataFrame serializer - follows the I→A→ATabular pattern.
    I: ISerialization (interface)
    A: ASerialization (abstract base)
    ATabular: ATabularSerialization (tabular base)
    Concrete: DataFrameSerializer
    Direct DataFrame operations - useful for intermediate processing.
    Examples:
        >>> serializer = DataFrameSerializer()
        >>> 
        >>> # Convert DataFrame to bytes (pickle)
        >>> df_bytes = serializer.from_df(df)
        >>> 
        >>> # Convert bytes to DataFrame
        >>> df = serializer.to_df(df_bytes)
        >>> 
        >>> # Save DataFrame
        >>> serializer.save_file(df, "data.pkl")
        >>> 
        >>> # Load DataFrame
        >>> df = serializer.load_file("data.pkl")
    """
    # ========================================================================
    # CODEC METADATA
    # ========================================================================
    @property

    def codec_id(self) -> str:
        return "dataframe"
    @property

    def media_types(self) -> list[str]:
        return [
            "application/x-pandas-dataframe",
            "application/x-pickle",
        ]
    @property

    def file_extensions(self) -> list[str]:
        return [".pkl", ".pickle", ".df"]
    @property

    def format_name(self) -> str:
        return "DataFrame"
    @property

    def mime_type(self) -> str:
        return "application/x-pandas-dataframe"
    @property

    def is_binary_format(self) -> bool:
        return True  # DataFrame pickle is binary
    @property

    def supports_streaming(self) -> bool:
        return False  # DataFrames don't naturally support streaming
    @property

    def capabilities(self) -> CodecCapability:
        return CodecCapability.BIDIRECTIONAL
    @property

    def aliases(self) -> list[str]:
        return ["dataframe", "DataFrame", "df", "pandas", "pkl"]
    @property

    def codec_types(self) -> list[str]:
        """DataFrame is an intermediate data format."""
        return ["data", "tabular", "intermediate"]
    # ========================================================================
    # CORE ENCODE/DECODE (Using pickle)
    # ========================================================================

    def encode(self, value: Any, *, options: EncodeOptions | None = None) -> bytes:
        """
        Encode DataFrame to pickle bytes.
        Args:
            value: DataFrame or dict of {sheet_name: DataFrame}
            options: Pickle options (protocol, etc.)
        Returns:
            Pickled DataFrame as bytes
        Raises:
            SerializationError: If encoding fails
        """
        try:
            import pickle
            opts = options or {}
            protocol = opts.get('protocol', pickle.HIGHEST_PROTOCOL)
            return pickle.dumps(value, protocol=protocol)
        except Exception as e:
            raise SerializationError(
                f"Failed to encode DataFrame: {e}",
                format_name=self.format_name,
                original_error=e
            )

    def decode(self, repr: bytes | str, *, options: DecodeOptions | None = None) -> Any:
        """
        Decode pickle bytes to DataFrame(s).
        Args:
            repr: Pickled DataFrame as bytes, or file path
            options: Decode options
        Returns:
            DataFrame or dict of {sheet_name: DataFrame}
        Raises:
            SerializationError: If decoding fails
        """
        try:
            import pickle
            # Handle file path
            if isinstance(repr, (str, Path)):
                with open(repr, 'rb') as f:
                    return pickle.load(f)
            # Handle bytes
            if isinstance(repr, str):
                repr = repr.encode('utf-8')
            return pickle.loads(repr)
        except Exception as e:
            raise SerializationError(
                f"Failed to decode DataFrame: {e}",
                format_name=self.format_name,
                original_error=e
            )
    # ========================================================================
    # TABULAR METHODS (to_df/from_df)
    # ========================================================================

    def to_df(
        self, 
        data: bytes | str | Path, 
        sheet_name: str | list[str] | None = None,
        **options
    ) -> pd.DataFrame | dict[str, pd.DataFrame]:
        """
        Convert pickled data to DataFrame(s).
        Args:
            data: Pickled DataFrame as bytes, or file path
            sheet_name: Ignored (pickle preserves structure)
            **options: Decode options
        Returns:
            DataFrame or dict of {sheet_name: DataFrame} (as stored)
        Examples:
            >>> serializer = DataFrameSerializer()
            >>> df = serializer.to_df(pickled_bytes)
            >>> df = serializer.to_df("data.pkl")
        """
        try:
            # Use decode method
            result = self.decode(data, **(options or {}))
            # If sheet_name specified and result is dict, filter
            if sheet_name is not None and isinstance(result, dict):
                if isinstance(sheet_name, str):
                    return result.get(sheet_name)
                else:
                    return {name: result.get(name) for name in sheet_name if name in result}
            return result
        except Exception as e:
            raise SerializationError(
                f"Failed to convert to DataFrame: {e}",
                format_name=self.format_name,
                original_error=e
            )

    def from_df(
        self, 
        df: pd.DataFrame | dict[str, pd.DataFrame], 
        **options
    ) -> bytes:
        """
        Convert DataFrame(s) to pickle bytes.
        Args:
            df: Single DataFrame or dict of {sheet_name: DataFrame}
            **options: Pickle options (protocol, etc.)
        Returns:
            Pickled DataFrame as bytes
        Examples:
            >>> serializer = DataFrameSerializer()
            >>> bytes = serializer.from_df(df)
            >>> bytes = serializer.from_df({"Sheet1": df1, "Sheet2": df2})
        """
        try:
            # Use encode method
            return self.encode(df, **(options or {}))
        except Exception as e:
            raise SerializationError(
                f"Failed to convert DataFrame: {e}",
                format_name=self.format_name,
                original_error=e
            )

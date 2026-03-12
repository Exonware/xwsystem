#exonware/xwsystem/src/exonware/xwsystem/io/serialization/formats/tabular/googlesheets.py
"""
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.9.0.4
Generation Date: January 2025
Google Sheets serialization.
Following I→A→ATabular pattern:
- I: ISerialization (interface)
- A: ASerialization (abstract base)
- ATabular: ATabularSerialization (tabular base)
- Concrete: GoogleSheetsSerializer
Note: For local saving, convert to Excel or CSV first.
For cloud saving, use xwstorage (avoids circular dependencies).
"""

from typing import Any, Optional
from pathlib import Path
import pandas as pd
from .base import ATabularSerialization
from ....contracts import EncodeOptions, DecodeOptions
from ....defs import CodecCapability
from ....errors import SerializationError


class GoogleSheetsSerializer(ATabularSerialization):
    """
    Google Sheets serializer - follows the I→A→ATabular pattern.
    I: ISerialization (interface)
    A: ASerialization (abstract base)
    ATabular: ATabularSerialization (tabular base)
    Concrete: GoogleSheetsSerializer
    Uses gspread library for Google Sheets API access.
    Local saving: Convert to Excel or CSV first, then save.
    Cloud saving: Use xwstorage (avoids circular dependencies).
    Examples:
        >>> serializer = GoogleSheetsSerializer(credentials_path="path/to/credentials.json")
        >>> 
        >>> # Read from Google Sheets
        >>> df = serializer.to_df(spreadsheet_url, sheet_name="Sheet1")
        >>> sheets = serializer.to_df(spreadsheet_url)  # All sheets
        >>> 
        >>> # Write to Google Sheets
        >>> serializer.from_df(df, spreadsheet_url, sheet_name="Sheet1")
        >>> serializer.from_df({"Sheet1": df1, "Sheet2": df2}, spreadsheet_url)  # Multiple sheets
    """

    def __init__(
        self, 
        credentials_path: Optional[str | Path] = None,
        max_depth: Optional[int] = None, 
        max_size_mb: Optional[float] = None
    ):
        """
        Initialize Google Sheets serializer.
        Args:
            credentials_path: Path to Google service account credentials JSON file
            max_depth: Maximum nesting depth (inherited from ASerialization)
            max_size_mb: Maximum size in MB (inherited from ASerialization)
        """
        super().__init__(max_depth=max_depth, max_size_mb=max_size_mb)
        self._credentials_path = str(credentials_path) if credentials_path else None
        self._client = None

    def _get_client(self):
        """Get or create gspread client (lazy initialization)."""
        if self._client is None:
            import importlib.util
            _gspread_spec = importlib.util.find_spec('gspread')
            _oauth2client_spec = importlib.util.find_spec('oauth2client')
            if _gspread_spec is None or _oauth2client_spec is None:
                raise ImportError(
                    "gspread and oauth2client are required for Google Sheets support. "
                    "Install with: pip install gspread oauth2client"
                )
            import gspread
            from oauth2client.service_account import ServiceAccountCredentials
            if not self._credentials_path:
                raise ValueError("credentials_path is required for Google Sheets access")
            scope = 'https://www.googleapis.com/auth/spreadsheets'
            creds = ServiceAccountCredentials.from_json_keyfile_name(
                self._credentials_path, scope
            )
            self._client = gspread.authorize(creds)
        return self._client
    # ========================================================================
    # CODEC METADATA
    # ========================================================================
    @property

    def codec_id(self) -> str:
        return "googlesheets"
    @property

    def media_types(self) -> list[str]:
        return ["application/vnd.google-apps.spreadsheet"]
    @property

    def file_extensions(self) -> list[str]:
        return []  # Google Sheets don't have local file extensions
    @property

    def format_name(self) -> str:
        return "Google Sheets"
    @property

    def mime_type(self) -> str:
        return "application/vnd.google-apps.spreadsheet"
    @property

    def is_binary_format(self) -> bool:
        return False  # Google Sheets API uses JSON
    @property

    def supports_streaming(self) -> bool:
        return False  # Google Sheets API doesn't support streaming
    @property

    def capabilities(self) -> CodecCapability:
        return CodecCapability.BIDIRECTIONAL
    @property

    def aliases(self) -> list[str]:
        return ["googlesheets", "GoogleSheets", "gsheet", "gsheets"]
    @property

    def codec_types(self) -> list[str]:
        """Google Sheets is a cloud data format."""
        return ["data", "tabular", "cloud"]
    # ========================================================================
    # CORE ENCODE/DECODE
    # ========================================================================

    def encode(self, value: Any, *, options: Optional[EncodeOptions] = None) -> str:
        """
        Encode data to Google Sheets format.
        Note: This is a placeholder. Use from_df() with spreadsheet_url instead.
        Args:
            value: DataFrame or dict of DataFrames
            options: Options including spreadsheet_url and sheet_name
        Returns:
            Spreadsheet URL (as string representation)
        Raises:
            SerializationError: If encoding fails
        """
        try:
            opts = options or {}
            spreadsheet_url = opts.get('spreadsheet_url')
            if not spreadsheet_url:
                raise ValueError("spreadsheet_url is required in options")
            # Use from_df for actual encoding
            if isinstance(value, (pd.DataFrame, dict)):
                self.from_df(value, spreadsheet_url=spreadsheet_url, **(opts or {}))
                return spreadsheet_url
            raise ValueError(f"Google Sheets encoding requires DataFrame or dict, got {type(value)}")
        except Exception as e:
            raise SerializationError(
                f"Failed to encode Google Sheets: {e}",
                format_name=self.format_name,
                original_error=e
            )

    def decode(self, repr: bytes | str, *, options: Optional[DecodeOptions] = None) -> Any:
        """
        Decode Google Sheets to DataFrame(s).
        Args:
            repr: Spreadsheet URL (as string)
            options: Options including sheet_name
        Returns:
            DataFrame or dict of {sheet_name: DataFrame}
        Raises:
            SerializationError: If decoding fails
        """
        try:
            if isinstance(repr, bytes):
                repr = repr.decode('utf-8')
            opts = options or {}
            sheet_name = opts.get('sheet_name')
            return self.to_df(repr, sheet_name=sheet_name, **(opts or {}))
        except Exception as e:
            raise SerializationError(
                f"Failed to decode Google Sheets: {e}",
                format_name=self.format_name,
                original_error=e
            )
    # ========================================================================
    # TABULAR METHODS (to_df/from_df)
    # ========================================================================

    def to_df(
        self, 
        spreadsheet_url: str, 
        sheet_name: Optional[str | list[str]] = None,
        header: int = 0,
        **options
    ) -> pd.DataFrame | dict[str, pd.DataFrame]:
        """
        Convert Google Sheet to DataFrame(s).
        Args:
            spreadsheet_url: Google Sheets URL
            sheet_name: Specific sheet name(s) to load, or None for all sheets
            header: Row index to use as header (default: 0)
            **options: Additional options
        Returns:
            Single DataFrame if one sheet, or dict of {sheet_name: DataFrame} if multiple
        Examples:
            >>> serializer = GoogleSheetsSerializer(credentials_path="creds.json")
            >>> df = serializer.to_df(spreadsheet_url, sheet_name="Sheet1")
            >>> sheets = serializer.to_df(spreadsheet_url)  # All sheets
        """
        try:
            from gspread_dataframe import get_as_dataframe
            client = self._get_client()
            spreadsheet = client.open_by_url(spreadsheet_url)
            if sheet_name is None:
                # Read all sheets
                result = {}
                for worksheet in spreadsheet.worksheets():
                    try:
                        df = get_as_dataframe(worksheet, header=header, **options)
                        result[worksheet.title] = df
                    except Exception as e:
                        # Skip sheets that can't be read
                        continue
                return result
            elif isinstance(sheet_name, str):
                # Read single sheet
                try:
                    worksheet = spreadsheet.worksheet(sheet_name)
                    return get_as_dataframe(worksheet, header=header, **options)
                except Exception as e:
                    raise SerializationError(
                        f"Failed to read sheet '{sheet_name}': {e}",
                        format_name=self.format_name,
                        original_error=e
                    )
            else:
                # Read multiple specific sheets
                result = {}
                for name in sheet_name:
                    try:
                        worksheet = spreadsheet.worksheet(name)
                        df = get_as_dataframe(worksheet, header=header, **options)
                        result[name] = df
                    except Exception as e:
                        # Skip sheets that can't be read
                        continue
                return result
        except Exception as e:
            raise SerializationError(
                f"Failed to convert Google Sheet to DataFrame: {e}",
                format_name=self.format_name,
                original_error=e
            )

    def from_df(
        self, 
        df: pd.DataFrame | dict[str, pd.DataFrame], 
        spreadsheet_url: str,
        sheet_name: Optional[str] = None,
        **options
    ) -> None:
        """
        Convert DataFrame(s) to Google Sheet.
        Note: This writes directly to Google Sheets (cloud).
        For local saving, convert to Excel or CSV first.
        Args:
            df: Single DataFrame or dict of {sheet_name: DataFrame}
            spreadsheet_url: Google Sheets URL
            sheet_name: Sheet name (required if df is single DataFrame)
            **options: Additional options
        Examples:
            >>> serializer = GoogleSheetsSerializer(credentials_path="creds.json")
            >>> serializer.from_df(df, spreadsheet_url, sheet_name="Sheet1")
            >>> serializer.from_df({"Sheet1": df1, "Sheet2": df2}, spreadsheet_url)
        """
        try:
            from gspread_dataframe import set_with_dataframe
            client = self._get_client()
            spreadsheet = client.open_by_url(spreadsheet_url)
            if isinstance(df, dict):
                # Multiple sheets
                for name, sheet_df in df.items():
                    try:
                        try:
                            worksheet = spreadsheet.worksheet(name)
                            worksheet.clear()
                        except Exception:
                            # Create worksheet if it doesn't exist
                            worksheet = spreadsheet.add_worksheet(
                                title=name, rows="2000", cols="30"
                            )
                        set_with_dataframe(worksheet, sheet_df, **options)
                    except Exception as e:
                        raise SerializationError(
                            f"Failed to write sheet '{name}': {e}",
                            format_name=self.format_name,
                            original_error=e
                        )
            else:
                # Single sheet
                if not sheet_name:
                    raise ValueError("sheet_name is required when writing a single DataFrame")
                try:
                    try:
                        worksheet = spreadsheet.worksheet(sheet_name)
                        worksheet.clear()
                    except Exception:
                        # Create worksheet if it doesn't exist
                        worksheet = spreadsheet.add_worksheet(
                            title=sheet_name, rows="2000", cols="30"
                        )
                    set_with_dataframe(worksheet, df, **options)
                except Exception as e:
                    raise SerializationError(
                        f"Failed to write sheet '{sheet_name}': {e}",
                        format_name=self.format_name,
                        original_error=e
                    )
        except Exception as e:
            raise SerializationError(
                f"Failed to convert DataFrame to Google Sheet: {e}",
                format_name=self.format_name,
                original_error=e
            )

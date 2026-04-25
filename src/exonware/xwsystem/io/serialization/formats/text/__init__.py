#exonware/xwsystem/src/exonware/xwsystem/io/serialization/formats/text/__init__.py
"""Text-based serialization formats."""

from .json import JsonSerializer
from .jsonlines import JsonLinesSerializer
from .csv import CsvSerializer
from .configparser import ConfigParserSerializer
from .formdata import FormDataSerializer
from .multipart import MultipartSerializer

try:
    from .json5 import Json5Serializer
except (ImportError, ModuleNotFoundError):
    Json5Serializer = None

try:
    from .yaml import YamlSerializer
except (ImportError, ModuleNotFoundError):
    YamlSerializer = None

try:
    from .toml import TomlSerializer
except (ImportError, ModuleNotFoundError):
    TomlSerializer = None

try:
    from .xml import XmlSerializer
except (ImportError, ModuleNotFoundError):
    XmlSerializer = None

__all__ = [
    # Primary serializers
    "JsonSerializer",
    "Json5Serializer",
    "JsonLinesSerializer",
    "YamlSerializer",
    "TomlSerializer",
    "XmlSerializer",
    "CsvSerializer",
    "ConfigParserSerializer",
    "FormDataSerializer",
    "MultipartSerializer",
]

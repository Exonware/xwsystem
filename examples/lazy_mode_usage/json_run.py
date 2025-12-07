"""
Example: Lazy Mode Usage with BSON Serialization

Usage:
    python json_run.py

Behavior:
    - Uninstalls serialization dependencies first to test lazy installation
    - Enables lazy mode for xwsystem via exonware.conf with "smart" mode
    - Demonstrates that missing dependencies are handled lazily
"""

import subprocess
import sys
from pathlib import Path

def clean_libs():
    # Uninstall serialization libraries to test lazy installation
    print("🧹 Cleaning up serialization dependencies...")
    serialization_packages = [
        "pymongo",           # BSON
        "msgpack",           # MessagePack
        "lxml",              # XML
        "cbor2",             # CBOR
        "fastavro",          # Avro
        "protobuf",          # Protocol Buffers
        "thrift",            # Apache Thrift
        "pyarrow",           # Parquet/Feather
        "orc",               # Apache ORC
        "plyvel",            # LevelDB
        "lmdb",              # LMDB
        "zarr",              # Zarr
        "h5py",              # HDF5
        "tables",            # PyTables
    ]

    for package in serialization_packages:
        try:
            subprocess.run(
                [sys.executable, "-m", "pip", "uninstall", package, "-y"],
                capture_output=True,
                check=False
            )
            print(f"✅ Uninstalled {package}")
        except Exception:
            print(f"❌ Failed to uninstall {package}")
            pass

    print("✅ Cleanup complete\n")

clean_libs()

from exonware.xwsystem.io.serialization.formats.binary import BsonSerializer as serializer

data = {"name": "John", "age": 30}
serializer_obj = serializer()
result = serializer_obj.encode(data)
print(f"\n📄 Serialized: {result}")
#print("✅ Success! Lazy mode auto-installed dependencies if missing.")

#!/usr/bin/env python3
#exonware/xwsystem/examples/format_conversion/example_format_conversion.py
"""
Example: Format Conversion with IArchiver ↔ ICodec Integration

Demonstrates the seamless integration between archivers and codecs.
"""

import sys
sys.path.insert(0, 'src')

from pathlib import Path
from exonware.xwsystem.io.file import XWFile
from exonware.xwsystem.io.codec.base import get_global_registry

print("=" * 70)
print("Format Conversion Integration Demo")
print("=" * 70)

# Example 1: Create a test file and zip it
print("\n[Example 1] Create ZIP archive")
print("-" * 70)

test_file = Path("example_data.txt")
test_file.write_text("This is test data for compression demo!")

from exonware.xwsystem.io.archive.archivers import ZipArchiver
zip_archiver = ZipArchiver()
zip_data = zip_archiver.compress({"example_data.txt": test_file.read_bytes()})
Path("example.zip").write_bytes(zip_data)
print(f"[OK] Created example.zip ({len(zip_data)} bytes)")

# Example 2: Convert ZIP → TAR using XWFile.convert()
print("\n[Example 2] Convert ZIP -> TAR")
print("-" * 70)

XWFile.convert("example.zip", "example.tar")
print(f"[OK] XWFile.convert('example.zip', 'example.tar')")
print(f"     Converted: {Path('example.tar').stat().st_size} bytes")

# Example 3: Use file.save_as() instance method
print("\n[Example 3] Use file.save_as() method")
print("-" * 70)

file = XWFile("example.zip")
file.save_as("example2.tar")
print(f"[OK] file.save_as('example2.tar')")
print(f"     Saved: {Path('example2.tar').stat().st_size} bytes")

# Example 4: Verify archivers are in CodecRegistry
print("\n[Example 4] CodecRegistry Integration")
print("-" * 70)

registry = get_global_registry()

zip_codec = registry.get_by_id("zip")
tar_codec = registry.get_by_id("tar")

print(f"[OK] get_codec_by_id('zip'): {zip_codec.__class__.__name__}")
print(f"     Category: {zip_codec.category.value}")
print(f"     Extensions: {zip_codec.file_extensions}")

print(f"\n[OK] get_codec_by_id('tar'): {tar_codec.__class__.__name__}")
print(f"     Category: {tar_codec.category.value}")
print(f"     Extensions: {tar_codec.file_extensions}")

# Example 5: Auto-detection from extension
print("\n[Example 5] Auto-Detection from Extension")
print("-" * 70)

codec = registry.get_by_extension(".zip")
print(f"[OK] get_by_extension('.zip'): {codec.__class__.__name__}")

# Example 6: Demonstrate category validation
print("\n[Example 6] Category Validation")
print("-" * 70)

print(f"[OK] ZIP and TAR have same category: {zip_codec.category == tar_codec.category}")
print(f"     Category: {zip_codec.category.value} == {tar_codec.category.value}")
print("\n[INFO] This ensures only compatible formats can be converted:")
print("       [OK] ZIP <-> TAR (both ARCHIVE)")
print("       [OK] JSON <-> YAML (both SERIALIZATION - future)")
print("       [ERROR] ZIP <-> JSON (different categories - prevented)")

# Cleanup
print("\n[Cleanup]")
print("-" * 70)
for f in ["example_data.txt", "example.zip", "example.tar", "example2.tar"]:
    p = Path(f)
    if p.exists():
        p.unlink()
        print(f"[OK] Removed {f}")

print("\n" + "=" * 70)
print("Demo Complete!")
print("=" * 70)
print("\nKey Features Demonstrated:")
print("  [OK] IArchiver extends ICodec")
print("  [OK] Archivers registered in CodecRegistry")
print("  [OK] XWFile.convert(source, target)")
print("  [OK] file.save_as(path, format)")
print("  [OK] Category-based validation")
print("  [OK] Auto-detection from extensions")

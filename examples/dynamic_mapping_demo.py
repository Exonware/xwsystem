#!/usr/bin/env python3
#exonware/xwsystem/examples/dynamic_mapping_demo.py
"""
Dynamic Package-Import Mapping Demo

This demo shows how the enhanced lazy discovery system automatically creates
mappings between package names (what you install with pip) and import names
(what you use in code) by reading from project configuration files.

Example mappings:
- "fastavro": ["fastavro", "fastavro"] (same name)
- "opencv-python": ["opencv-python", "cv2"] (different import name)
- "Pillow": ["Pillow", "PIL"] (different import name)
"""

import sys
import os
from pathlib import Path

# Add the src directory to the path
sys.path.insert(0, str(Path(__file__).parent.parent / 'src'))

from xwlazy.lazy import get_lazy_discovery, DependencyMapper


def main():
    """Demonstrate the dynamic mapping system."""
    print("🚀 Dynamic Package-Import Mapping Demo")
    print("=" * 50)
    
    # Get the lazy discovery instance
    discovery = get_lazy_discovery()
    
    print("\n📦 Package to Import Names Mapping:")
    print("-" * 40)
    
    # Get the package to import names mapping
    package_mapping = discovery.get_package_import_mapping()
    
    # Show some examples
    examples = [
        "fastavro",
        "opencv-python", 
        "Pillow",
        "PyYAML",
        "scikit-learn",
        "beautifulsoup4"
    ]
    
    for package in examples:
        if package in package_mapping:
            imports = package_mapping[package]
            print(f"  {package}: {imports}")
        else:
            print(f"  {package}: Not found in project dependencies")
    
    print("\n🔍 Import to Package Names Mapping:")
    print("-" * 40)
    
    # Get the import to package names mapping
    import_mapping = discovery.get_import_package_mapping()
    
    # Show some examples
    import_examples = [
        "fastavro",
        "cv2",
        "PIL", 
        "yaml",
        "sklearn",
        "bs4"
    ]
    
    for import_name in import_examples:
        if import_name in import_mapping:
            package = import_mapping[import_name]
            print(f"  {import_name}: {package}")
        else:
            print(f"  {import_name}: Not found in project dependencies")
    
    print("\n🔧 DependencyMapper Usage:")
    print("-" * 40)
    
    # Demonstrate DependencyMapper
    mapper = DependencyMapper()
    
    # Test some mappings
    test_imports = ["fastavro", "cv2", "PIL", "yaml", "sklearn"]
    
    for import_name in test_imports:
        package_name = mapper.get_package_name(import_name)
        print(f"  Import '{import_name}' -> Package '{package_name}'")
    
    print("\n📊 Discovery Sources:")
    print("-" * 40)
    
    sources = discovery.get_discovery_sources()
    if sources:
        for source in sources:
            print(f"  ✓ {source}")
    else:
        print("  No sources discovered")
    
    print("\n📈 Total Dependencies Discovered:")
    print("-" * 40)
    
    all_deps = discovery.discover_all_dependencies()
    print(f"  Total: {len(all_deps)} dependencies")
    
    # Show first 10 dependencies
    if all_deps:
        print("\n  First 10 dependencies:")
        for i, (import_name, package_name) in enumerate(list(all_deps.items())[:10]):
            print(f"    {i+1:2d}. {import_name} -> {package_name}")
        
        if len(all_deps) > 10:
            print(f"    ... and {len(all_deps) - 10} more")
    
    print("\n✨ Dynamic Mapping Benefits:")
    print("-" * 40)
    print("  • Automatically discovers package mappings from project config")
    print("  • No hardcoded mappings needed")
    print("  • Works across all exonware libraries")
    print("  • Caches mappings for performance")
    print("  • Supports multiple import names per package")
    print("  • Falls back to common mappings if not found in config")


if __name__ == "__main__":
    main()

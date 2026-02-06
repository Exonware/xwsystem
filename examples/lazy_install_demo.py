#exonware/xwsystem/examples/lazy_install_demo.py
"""
Lazy Install Demo for xwsystem

This demo shows how the universal lazy install system works.
It demonstrates automatic installation of missing packages when
xwsystem modules try to import external dependencies.

Author: Eng. Muhammad AlShehri
Company: eXonware.com
Email: connect@exonware.com
Version: 0.0.1
Generated: 2025-01-27
"""

import sys
import os

# Add src to path for development
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', 'src'))

def demo_lazy_install_configuration():
    """Demo 1: Lazy Install Configuration"""
    print("=" * 60)
    print("🚀 DEMO 1: Lazy Install Configuration")
    print("=" * 60)
    
    from xwlazy.lazy.config import (
        LazyConfig,
        DEFAULT_LAZY_CONFIG,
    )
    
    print(f"📊 Current lazy install status: {LazyConfig.get_config()}")
    print(f"📊 Is enabled: {LazyConfig.is_enabled()}")
    
    # Show some package mappings
    print(f"\n📦 Package mappings (showing first 10):")
    packages = LazyConfig.get_all_packages()
    for i, (import_name, package_name) in enumerate(list(packages.items())[:10]):
        print(f"   {import_name} → {package_name}")
    
    print(f"\n📦 Total packages in mapping: {len(packages)}")
    
    # Test package name resolution
    test_imports = ['msgpack', 'cv2', 'yaml', 'sklearn', 'requests']
    print(f"\n🔍 Package name resolution:")
    for import_name in test_imports:
        package_name = LazyConfig.get_package_name(import_name)
        print(f"   {import_name} → {package_name}")
    
    # Toggle lazy install
    print(f"\n🔄 Toggling lazy install...")
    if LazyConfig.is_enabled():
        LazyConfig.disable()
        print("   ✅ Lazy install disabled")
    else:
        LazyConfig.enable()
        print("   ✅ Lazy install enabled")
    
    print(f"📊 New status: {LazyConfig.is_enabled()}")

def demo_lazy_import_with_install():
    """Demo 2: Lazy Import with Install"""
    print("\n" + "=" * 60)
    print("🚀 DEMO 2: Lazy Import with Install")
    print("=" * 60)
    
    from xwlazy.lazy import lazy_import_with_install
    
    # Test with a common package that might not be installed
    test_modules = [
        ('msgpack', 'msgpack'),
        ('requests', 'requests'),
        ('yaml', 'PyYAML'),
        ('nonexistent_module', 'nonexistent-package')
    ]
    
    for module_name, expected_package in test_modules:
        print(f"\n🔍 Testing: {module_name}")
        print(f"   Expected package: {expected_package}")
        
        try:
            module, available = lazy_import_with_install(module_name)
            if available:
                print(f"   ✅ Module available: {module}")
                print(f"   📦 Module file: {getattr(module, '__file__', 'Built-in')}")
            else:
                print(f"   ❌ Module not available")
        except Exception as e:
            print(f"   ⚠️  Error: {e}")

def demo_lazy_install_stats():
    """Demo 3: Lazy Install Statistics"""
    print("\n" + "=" * 60)
    print("🚀 DEMO 3: Lazy Install Statistics")
    print("=" * 60)
    
    from xwlazy.lazy import get_lazy_install_stats
    
    stats = get_lazy_install_stats()
    print(f"📊 Lazy Install Statistics:")
    print(f"   Enabled: {stats['enabled']}")
    print(f"   Total installed: {stats['total_installed']}")
    print(f"   Total failed: {stats['total_failed']}")
    
    if stats['installed_packages']:
        print(f"   📦 Installed packages: {stats['installed_packages']}")
    
    if stats['failed_packages']:
        print(f"   ❌ Failed packages: {stats['failed_packages']}")

def demo_environment_control():
    """Demo 4: Environment Variable Control"""
    print("\n" + "=" * 60)
    print("🚀 DEMO 4: Environment Variable Control")
    print("=" * 60)
    
    from xwlazy.lazy.config import load_from_environment
    
    print("🌍 Environment variable control:")
    print("   Set XWSYSTEM_LAZY_INSTALL=true to enable")
    print("   Set XWSYSTEM_LAZY_INSTALL=false to disable")
    
    current_env = os.getenv('XWSYSTEM_LAZY_INSTALL', 'Not set')
    print(f"   Current value: {current_env}")
    
    # Reload configuration from environment
    load_from_environment()
    print("   ✅ Configuration reloaded from environment")

def demo_real_world_usage():
    """Demo 5: Real-World Usage Pattern"""
    print("\n" + "=" * 60)
    print("🚀 DEMO 5: Real-World Usage Pattern")
    print("=" * 60)
    
    print("💡 This is how xwsystem modules will use lazy install:")
    print()
    
    # Simulate what happens in a serialization module
    print("📝 In xwsystem/serialization/msgpack.py:")
    print("   from ..utils.lazy_package import lazy_import_with_install")
    print("   ")
    print("   msgpack, MSGPACK_AVAILABLE = lazy_import_with_install('msgpack')")
    print("   ")
    print("   class MsgPackSerializer:")
    print("       def __init__(self):")
    print("           if not MSGPACK_AVAILABLE:")
    print("               raise ImportError('msgpack not available')")
    print("       ")
    print("       def serialize(self, data):")
    print("           return msgpack.packb(data)")
    print()
    
    print("🎯 When user calls:")
    print("   from xwsystem import MsgPackSerializer")
    print("   serializer = MsgPackSerializer()  # ← This triggers lazy install!")
    print()
    
    print("🔄 What happens:")
    print("   1. MsgPackSerializer.__init__() is called")
    print("   2. msgpack.py loads and calls lazy_import_with_install('msgpack')")
    print("   3. Import fails → pip install msgpack")
    print("   4. Import succeeds → User gets working serializer ✅")

def demo_universal_coverage():
    """Demo 6: Universal Coverage"""
    print("\n" + "=" * 60)
    print("🚀 DEMO 6: Universal Coverage")
    print("=" * 60)
    
    from xwlazy.lazy.config import UNIVERSAL_PACKAGES
    
    # Categorize packages
    categories = {
        'Serialization': [],
        'HTTP & Networking': [],
        'Security & Crypto': [],
        'Database': [],
        'Data Science': [],
        'Web Frameworks': [],
        'Development Tools': [],
        'System & OS': [],
        'File & I/O': [],
        'Other': []
    }
    
    # Categorize packages (simplified)
    for import_name, package_name in UNIVERSAL_PACKAGES.items():
        if any(x in import_name.lower() for x in ['msgpack', 'cbor', 'avro', 'protobuf', 'thrift', 'yaml', 'json']):
            categories['Serialization'].append(import_name)
        elif any(x in import_name.lower() for x in ['requests', 'httpx', 'aiohttp', 'urllib']):
            categories['HTTP & Networking'].append(import_name)
        elif any(x in import_name.lower() for x in ['crypto', 'jwt', 'bcrypt', 'cryptography']):
            categories['Security & Crypto'].append(import_name)
        elif any(x in import_name.lower() for x in ['psycopg', 'mysql', 'sqlite', 'mongo', 'neo4j']):
            categories['Database'].append(import_name)
        elif any(x in import_name.lower() for x in ['numpy', 'pandas', 'matplotlib', 'sklearn', 'scipy']):
            categories['Data Science'].append(import_name)
        elif any(x in import_name.lower() for x in ['django', 'flask', 'fastapi', 'uvicorn']):
            categories['Web Frameworks'].append(import_name)
        elif any(x in import_name.lower() for x in ['pytest', 'black', 'isort', 'mypy']):
            categories['Development Tools'].append(import_name)
        elif any(x in import_name.lower() for x in ['psutil', 'colorama', 'pytz', 'dateutil']):
            categories['System & OS'].append(import_name)
        elif any(x in import_name.lower() for x in ['aiofiles', 'watchdog']):
            categories['File & I/O'].append(import_name)
        else:
            categories['Other'].append(import_name)
    
    print("📊 Universal Package Coverage:")
    total_packages = len(UNIVERSAL_PACKAGES)
    for category, packages in categories.items():
        if packages:
            print(f"   {category}: {len(packages)} packages")
            if len(packages) <= 5:
                print(f"      {', '.join(packages)}")
            else:
                print(f"      {', '.join(packages[:3])}... (+{len(packages)-3} more)")
    
    print(f"\n🎯 Total packages covered: {total_packages}")
    print("   This covers ALL external dependencies used by xwsystem!")

def main():
    """Run all demos"""
    print("🚀 xwsystem Lazy Install System Demo")
    print("=" * 60)
    print("This demo shows how the universal lazy install system works.")
    print("It automatically installs missing packages when xwsystem")
    print("modules try to import external dependencies.")
    print()
    
    try:
        demo_lazy_install_configuration()
        demo_lazy_import_with_install()
        demo_lazy_install_stats()
        demo_environment_control()
        demo_real_world_usage()
        demo_universal_coverage()
        
        print("\n" + "=" * 60)
        print("✅ Demo completed successfully!")
        print("=" * 60)
        print()
        print("🎯 Key Benefits:")
        print("   • Automatic dependency resolution")
        print("   • Zero-configuration for users")
        print("   • Universal coverage of all xwsystem dependencies")
        print("   • Environment variable control")
        print("   • Comprehensive logging and statistics")
        print("   • Thread-safe operation")
        print()
        print("💡 Next Steps:")
        print("   1. Apply lazy_import_with_install to all xwsystem modules")
        print("   2. Test with missing packages")
        print("   3. Add [lazy] extra to pyproject.toml")
        print("   4. Update documentation")
        
    except Exception as e:
        print(f"\n❌ Demo failed: {e}")
        import traceback
        traceback.print_exc()

if __name__ == "__main__":
    main()

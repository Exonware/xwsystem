#exonware/xwsystem/rust/python/exonware/rust/xwsystem/caching/__init__.py
"""
eXonware Rust XWSystem Caching Python bindings.
This package provides Python bindings for the Rust implementation of XWSystem caching.
"""

import sys
import os
from pathlib import Path
# Try to import the module, with fallback paths if needed
try:
    import _xwsystem_rust as rust_module
except ImportError:
    # Try to find the module in various locations
    current_file = Path(__file__).resolve()
    # Navigate up from: python/exonware/rust/xwsystem/caching/__init__.py
    # to: rust/
    rust_dir = current_file.parent.parent.parent.parent.parent.parent
    possible_paths = [
        rust_dir / "target" / "release",
        rust_dir / "target" / "release" / "deps",
        rust_dir.parent / "target" / "release",
    ]
    for path in possible_paths:
        if path.exists() and str(path) not in sys.path:
            sys.path.insert(0, str(path))
    # Also check site-packages
    import site
    site_paths = site.getsitepackages() + [site.getusersitepackages()]
    for site_path in site_paths:
        if os.path.exists(site_path) and site_path not in sys.path:
            sys.path.insert(0, site_path)
    # Try importing again - if still fails, raise with helpful message
    try:
        import _xwsystem_rust as rust_module
    except ImportError:
        raise ImportError(
            f"Failed to import _xwsystem_rust module. "
            f"Checked paths: {[str(p) for p in possible_paths] + site_paths}. "
            f"Please ensure the module is built and installed. "
            f"Run: cd {rust_dir} && python -m maturin develop --release --features python,external-caches"
        )
# Core cache implementations (always available)
LRUCache = rust_module.caching.LRUCache
LFUCache = rust_module.caching.LFUCache
TTLCache = rust_module.caching.TTLCache
OptimizedLFUCache = rust_module.caching.OptimizedLFUCache
MemoryBoundedLRUCache = rust_module.caching.MemoryBoundedLRUCache
SecureLRUCache = rust_module.caching.SecureLRUCache
__all__ = [
    "LRUCache",
    "LFUCache",
    "TTLCache",
    "OptimizedLFUCache",
    "MemoryBoundedLRUCache",
    "SecureLRUCache",
]
# External Rust caches (available when built with --features external-caches)
MokaCache = rust_module.caching.MokaCache
MokaTTLCache = rust_module.caching.MokaTTLCache
MokaWeightedCache = rust_module.caching.MokaWeightedCache
QuickCache = rust_module.caching.QuickCache
QuickCacheTTL = rust_module.caching.QuickCacheTTL
DashMapCache = rust_module.caching.DashMapCache
DashMapTTLCache = rust_module.caching.DashMapTTLCache
__all__.extend([
    "MokaCache",
    "MokaTTLCache",
    "MokaWeightedCache",
    "QuickCache",
    "QuickCacheTTL",
    "DashMapCache",
    "DashMapTTLCache",
])

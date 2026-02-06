#exonware/xwsystem/rust/python/exonware/rust/xwsystem/__init__.py
"""
eXonware Rust XWSystem Python bindings.

This package provides Python bindings for the Rust implementation of XWSystem.
"""

try:
    # Import the Python wrappers
    from exonware.rust.xwsystem import version
    
    # Import caching module
    try:
        from exonware.rust.xwsystem import caching
    except ImportError:
        caching = None
    
    __all__ = ["version", "caching"]
    if caching:
        __all__.append("caching")
    __version__ = version.get_version()
except ImportError as e:
    import warnings
    warnings.warn(
        f"Failed to import Rust bindings: {e}. "
        "Make sure the Rust extension is built. Run: cd xwsystem/rust && maturin develop",
        ImportWarning
    )
    __all__ = []
    __version__ = "0.1.0.1"

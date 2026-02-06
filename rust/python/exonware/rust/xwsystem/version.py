#exonware/xwsystem/rust/python/exonware/rust/xwsystem/version.py
"""
Python wrapper for Rust version module.

This provides a Pythonic interface to the Rust version module.
"""

# Version info not available from Rust module yet
# Provide fallback version info
VERSION = "0.1.0"
VERSION_MAJOR = 0
VERSION_MINOR = 1
VERSION_PATCH = 0
VERSION_BUILD = 0
VERSION_SUFFIX = ""
VERSION_STRING = "0.1.0"


# Re-export functions
def get_version() -> str:
    """Get the current version string."""
    return VERSION_STRING


def get_version_info() -> tuple:
    """Get version as a tuple (major, minor, patch, build)."""
    return (VERSION_MAJOR, VERSION_MINOR, VERSION_PATCH, VERSION_BUILD)


def get_version_dict() -> dict:
    """Get version information as a dictionary."""
    return {
        "major": VERSION_MAJOR,
        "minor": VERSION_MINOR,
        "patch": VERSION_PATCH,
        "build": VERSION_BUILD,
        "suffix": VERSION_SUFFIX,
        "string": VERSION_STRING,
    }


def is_dev_version() -> bool:
    """Check if this is a development version."""
    return VERSION_MAJOR == 0


def is_release_version() -> bool:
    """Check if this is a release version."""
    return VERSION_MAJOR > 0


__all__ = [
    "VERSION",
    "VERSION_MAJOR",
    "VERSION_MINOR",
    "VERSION_PATCH",
    "VERSION_BUILD",
    "VERSION_SUFFIX",
    "VERSION_STRING",
    "get_version",
    "get_version_info",
    "get_version_dict",
    "is_dev_version",
    "is_release_version",
]

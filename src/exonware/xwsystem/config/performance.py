#exonware/xwsystem/src/exonware/xwsystem/config/performance.py
"""Performance optimization configuration for xwsystem I/O operations.

This module provides configuration options to enable/disable performance
optimizations (parallel index building, append-only logs) with automatic
fallback to original implementations.
"""

from __future__ import annotations

from dataclasses import dataclass
import os


@dataclass
class PerformanceConfig:
    """Configuration for performance optimizations."""
    
    # Parallel index building
    enable_parallel_index: bool = True  # Auto-enabled for large files
    parallel_index_workers: int | None = None  # None = auto (CPU count)
    parallel_index_chunk_size_mb: int = 100  # 100MB chunks
    parallel_index_threshold_mb: int = 200  # Use parallel for files >200MB
    
    # Append-only log for atomic updates
    enable_append_log: bool = True  # Auto-enabled for large files
    append_log_threshold_mb: int = 100  # Use append-only log for files >100MB
    append_log_compaction_threshold_mb: int = 100  # Compact when log > 100MB
    
    # Fallback behavior
    fallback_on_error: bool = True  # Fall back to original if optimization fails
    
    @classmethod
    def from_env(cls) -> PerformanceConfig:
        """Create config from environment variables."""
        return cls(
            enable_parallel_index=os.getenv("XWSYSTEM_PARALLEL_INDEX", "true").lower() == "true",
            parallel_index_workers=int(os.getenv("XWSYSTEM_PARALLEL_WORKERS", "0")) or None,
            parallel_index_chunk_size_mb=int(os.getenv("XWSYSTEM_CHUNK_SIZE_MB", "100")),
            parallel_index_threshold_mb=int(os.getenv("XWSYSTEM_PARALLEL_THRESHOLD_MB", "200")),
            enable_append_log=os.getenv("XWSYSTEM_APPEND_LOG", "true").lower() == "true",
            append_log_threshold_mb=int(os.getenv("XWSYSTEM_APPEND_LOG_THRESHOLD_MB", "100")),
            append_log_compaction_threshold_mb=int(os.getenv("XWSYSTEM_LOG_THRESHOLD_MB", "100")),
            fallback_on_error=os.getenv("XWSYSTEM_FALLBACK", "true").lower() == "true",
        )
    
    @classmethod
    def conservative(cls) -> PerformanceConfig:
        """Conservative config (disable optimizations, use originals)."""
        return cls(
            enable_parallel_index=False,
            enable_append_log=False,
            fallback_on_error=True,
        )
    
    @classmethod
    def aggressive(cls) -> PerformanceConfig:
        """Aggressive config (enable all optimizations, no fallback)."""
        return cls(
            enable_parallel_index=True,
            enable_append_log=True,
            fallback_on_error=False,
        )


# Global config instance
_config: PerformanceConfig | None = None


def get_performance_config() -> PerformanceConfig:
    """Get global performance config."""
    global _config
    if _config is None:
        _config = PerformanceConfig.from_env()
    return _config


def set_performance_config(config: PerformanceConfig) -> None:
    """Set global performance config."""
    global _config
    _config = config


__all__ = [
    "PerformanceConfig",
    "get_performance_config",
    "set_performance_config",
]

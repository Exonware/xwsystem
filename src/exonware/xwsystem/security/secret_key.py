#!/usr/bin/env python3
"""
Simple secret key helper for xwsystem.

Usage:
    from xwsystem import security
    token = security.secret_key["MY_TOKEN"]
    security.secret_key["MY_TOKEN"] = "new-value"  # Stored in env for this process

Lookup rules (read):
    1. Environment variable named exactly as the key (e.g. "MY_TOKEN")
    2. If not found, a file inside project root ".secrets" directory:
       - First tries a file named "<key>"
       - Then any file matching "<key>.*" (e.g. "MY_TOKEN.txt", "MY_TOKEN.json")

Notes:
    - Environment variables take precedence over files.
    - Values are treated as opaque secrets (plain text), no parsing is done.
    - Assignments only affect the current process environment; they do not
      write back to disk.
"""

from __future__ import annotations

import os
from pathlib import Path
from typing import Dict, Iterable, Iterator, MutableMapping, Optional

from ..config.logging_setup import get_logger

logger = get_logger(__name__)


def _discover_project_root() -> Path:
    """
    Best-effort detection of the xwsystem project root.

    This assumes the standard layout used in the mono-repo:
        <repo_root>/xwsystem/src/exonware/xwsystem/security/secret_key.py

    In that case the project root is four levels above this file.
    If that layout changes or we are running from an installed wheel,
    we simply fall back to the current working directory.
    """
    here = Path(__file__).resolve()
    try:
        # .../xwsystem/src/exonware/xwsystem/security/secret_key.py
        project_root = here.parents[4]
        if (project_root / ".secrets").exists():
            return project_root
    except IndexError:  # pragma: no cover - defensive
        pass
    # Fallback: current working directory
    return Path.cwd()


def _get_secrets_dir() -> Path:
    """
    Resolve the directory where secret files are stored.

    Order:
        1. XWSYSTEM_SECRETS_DIR environment variable (if set)
        2. "<project_root>/.secrets"
    """
    env_dir = os.getenv("XWSYSTEM_SECRETS_DIR")
    if env_dir:
        p = Path(env_dir).expanduser()
        return p
    project_root = _discover_project_root()
    return project_root / ".secrets"


class SecretKeyStore(MutableMapping[str, str]):
    """
    Mapping-like interface for secret keys.

    Reading:
        value = secret_key["ID"]

    Writing:
        secret_key["ID"] = "something"
        # The value is stored in os.environ for this process.

    This helper is intentionally minimal and focused on DX:
        - Environment variables win over files.
        - Files live in a single ".secrets" directory at project root.
        - File extensions are ignored; the first matching file is used.
    """

    def __init__(self) -> None:
        self._cache: Dict[str, str] = {}

    # ------------------------------------------------------------------
    # Core helpers
    # ------------------------------------------------------------------
    @staticmethod
    def _env_key(name: str) -> str:
        """
        Compute the environment variable name for a given key.

        We keep this simple and use the key as-is, so examples and tests
        can choose descriptive names:
            secret_key["XWAUTH_JWT_SECRET"]
            secret_key["TELEGRAM_PARROT_BOT_TOKEN"]
        """
        return name

    def _load_from_env(self, name: str) -> Optional[str]:
        env_name = self._env_key(name)
        value = os.getenv(env_name)
        if value is not None:
            return value
        return None

    def _load_from_files(self, name: str) -> Optional[str]:
        secrets_dir = _get_secrets_dir()
        # Ensure directory exists for developer convenience
        try:
            secrets_dir.mkdir(parents=True, exist_ok=True)
        except Exception:  # pragma: no cover - non-fatal
            pass

        # 1) Exact file name "<name>"
        exact_path = secrets_dir / name
        candidates: Iterable[Path]
        if exact_path.is_file():
            candidates = [exact_path]
        else:
            # 2) Any file matching "<name>.*"
            candidates = sorted(secrets_dir.glob(f"{name}.*"))

        for path in candidates:
            try:
                # Treat secret as opaque text, strip common trailing whitespace
                value = path.read_text(encoding="utf-8").strip()
                return value
            except OSError as exc:  # pragma: no cover - unlikely
                logger.warning(
                    "Failed to read secret file %s: %s", path, exc, extra={"secret_key": name}
                )
        return None

    # ------------------------------------------------------------------
    # MutableMapping API
    # ------------------------------------------------------------------
    def __getitem__(self, key: str) -> str:
        if key in self._cache:
            return self._cache[key]

        # 1) Environment variables (highest priority)
        value = self._load_from_env(key)
        if value is not None:
            self._cache[key] = value
            return value

        # 2) .secrets/<key> or .secrets/<key>.*
        value = self._load_from_files(key)
        if value is not None:
            self._cache[key] = value
            return value

        raise KeyError(
            f"Secret '{key}' not found. "
            "Set an environment variable with this name, or create a file "
            f"named '{key}' or '{key}.<ext>' inside the project '.secrets' folder. "
            "Replace the secret with yours. This secret is a token or secret code "
            "for the developer and you need to make your own."
        )

    def __setitem__(self, key: str, value: str) -> None:
        """
        Store secret in the current process environment and cache.

        This does NOT write to disk; it is intentionally ephemeral so that
        examples and tests can be configured programmatically without
        touching the filesystem.
        """
        env_name = self._env_key(key)
        os.environ[env_name] = value
        self._cache[key] = value

    def __delitem__(self, key: str) -> None:
        self._cache.pop(key, None)
        env_name = self._env_key(key)
        os.environ.pop(env_name, None)

    def __iter__(self) -> Iterator[str]:
        # Expose cached keys only; we don't enumerate env or filesystem.
        return iter(self._cache)

    def __len__(self) -> int:  # pragma: no cover - trivial
        return len(self._cache)


# Global instance used by xwsystem.security
secret_key: SecretKeyStore = SecretKeyStore()


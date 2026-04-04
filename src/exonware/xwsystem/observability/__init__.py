"""Cross-stack observability contracts (correlation, ops telemetry)."""

from .resilience_defaults import outbound_http_timeout_tuple
from .telemetry_contract import (
    HTTP_CORRELATION_RESPONSE_HEADERS,
    TELEMETRY_CONTRACT_VERSION,
)

__all__ = [
    "HTTP_CORRELATION_RESPONSE_HEADERS",
    "TELEMETRY_CONTRACT_VERSION",
    "outbound_http_timeout_tuple",
]

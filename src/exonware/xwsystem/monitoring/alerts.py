#!/usr/bin/env python3
"""
#exonware/xwsystem/src/exonware/xwsystem/monitoring/alerts.py

Declarative alert rule engine.

An :class:`AlertRule` binds a metric path (``cpu_percent``,
``queries_per_min``, ...) to a threshold comparison. The engine
evaluates a dict of current readings and returns :class:`AlertFire`
records for each rule that tripped. Hosts persist / notify / page on
those.

Tiny, pure, no I/O — the same building block the threat-rule engine
uses but expressed over numeric thresholds instead of event predicates.
"""

from __future__ import annotations

from dataclasses import dataclass, field
from typing import Any, Literal


Operator = Literal[">", ">=", "<", "<=", "==", "!="]


@dataclass(slots=True)
class AlertRule:
    id: str
    metric: str
    op: Operator
    threshold: float
    severity: str = "warning"  # info | warning | critical
    name: str = ""
    description: str = ""
    enabled: bool = True


@dataclass(slots=True)
class AlertFire:
    rule_id: str
    rule_name: str
    severity: str
    metric: str
    value: float
    threshold: float
    op: Operator
    message: str
    meta: dict[str, Any] = field(default_factory=dict)


_OPS = {
    ">":  lambda a, b: a > b,
    ">=": lambda a, b: a >= b,
    "<":  lambda a, b: a < b,
    "<=": lambda a, b: a <= b,
    "==": lambda a, b: a == b,
    "!=": lambda a, b: a != b,
}


def evaluate(rules: list[AlertRule], readings: dict[str, Any]) -> list[AlertFire]:
    fires: list[AlertFire] = []
    for rule in rules:
        if not rule.enabled:
            continue
        if rule.metric not in readings:
            continue
        value = readings[rule.metric]
        try:
            numeric = float(value)
        except (TypeError, ValueError):
            continue
        op_fn = _OPS.get(rule.op)
        if op_fn is None:
            continue
        if not op_fn(numeric, rule.threshold):
            continue
        fires.append(AlertFire(
            rule_id=rule.id,
            rule_name=rule.name or rule.id,
            severity=rule.severity,
            metric=rule.metric,
            value=numeric,
            threshold=rule.threshold,
            op=rule.op,
            message=f"{rule.metric} {rule.op} {rule.threshold} (got {numeric})",
        ))
    return fires


def default_rules() -> list[AlertRule]:
    """Starter set covering the headline cluster-health signals."""
    return [
        AlertRule(id="cpu-hot", metric="cpu_percent", op=">", threshold=85.0, severity="warning", name="CPU above 85%"),
        AlertRule(id="mem-hot", metric="mem_percent", op=">", threshold=90.0, severity="critical", name="Memory above 90%"),
        AlertRule(id="disk-hot", metric="disk_percent", op=">", threshold=90.0, severity="critical", name="Disk above 90%"),
        AlertRule(id="fail-rate", metric="fail_rate", op=">", threshold=0.05, severity="warning", name="Fail rate above 5%"),
        AlertRule(id="p95-latency", metric="p95_ms", op=">", threshold=1500.0, severity="warning", name="p95 latency above 1500ms"),
    ]


__all__ = ["AlertRule", "AlertFire", "Operator", "evaluate", "default_rules"]

#!/usr/bin/env python3
#exonware/xwsystem/src/exonware/xwsystem/security/monitor.py
"""
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.9.0.40
Generation Date: 07-Jan-2025
Security monitor implementation for XWSystem.
Implements ISecurityMonitor protocol for security monitoring and threat detection.
"""

import time
from collections import defaultdict, deque
from typing import Any
from datetime import datetime, timedelta
from .base import ASecurityMonitorBase
from .contracts import ISecurityMonitor
from .defs import SecurityLevel
from ..config.logging_setup import get_logger
logger = get_logger("xwsystem.security.monitor")


class SecurityMonitor(ASecurityMonitorBase, ISecurityMonitor):
    """
    Security monitor implementation.
    Provides comprehensive security monitoring including:
    - Intrusion detection
    - Failed login monitoring
    - Anomaly detection
    - Security alerts
    - Threat level management
    """

    def __init__(self, security_level: SecurityLevel = SecurityLevel.MEDIUM):
        """
        Initialize security monitor.
        Args:
            security_level: Security level for monitoring
        """
        super().__init__()
        self.security_level = security_level
        self._threat_level = SecurityLevel.MEDIUM
        self._security_alerts: list[dict[str, Any]] = []
        self._failed_logins: dict[str, deque] = defaultdict(lambda: deque(maxlen=100))
        self._event_history: deque = deque(maxlen=1000)
        self._anomaly_threshold = 5.0  # Standard deviations for anomaly detection
        # Intrusion detection patterns
        self._intrusion_patterns = [
            "multiple_failed_logins",
            "unusual_access_pattern",
            "privilege_escalation_attempt",
            "data_exfiltration_pattern",
            "command_injection_attempt",
        ]

    def detect_intrusion(self, event_data: dict[str, Any]) -> bool:
        """
        Detect intrusion attempts.
        Args:
            event_data: Event data to analyze
        Returns:
            True if intrusion detected
        """
        event_type = event_data.get("type", "")
        user = event_data.get("user", "")
        resource = event_data.get("resource", "")
        timestamp = event_data.get("timestamp", time.time())
        # Check for multiple failed logins
        if event_type == "failed_login":
            if self.monitor_failed_logins(user, max_attempts=5):
                self._add_alert("intrusion", f"Multiple failed logins detected for user: {user}")
                return True
        # Check for unusual access patterns
        if event_type == "access":
            if self._check_unusual_access(user, resource, timestamp):
                self._add_alert("intrusion", f"Unusual access pattern detected for user: {user}")
                return True
        # Check for privilege escalation attempts
        if event_type == "privilege_escalation":
            self._add_alert("privilege_escalation", f"Privilege escalation attempt detected for user: {user}")
            return True
        # Check for command injection patterns
        if "command" in event_data:
            command = str(event_data["command"])
            suspicious_patterns = [";", "|", "&&", "||", "`", "$(", "<", ">"]
            if any(pattern in command for pattern in suspicious_patterns):
                self._add_alert("intrusion", f"Command injection pattern detected for user: {user}")
                return True
        # Check for data exfiltration patterns
        if event_type == "data_access":
            data_size = event_data.get("data_size", 0)
            if data_size > 100 * 1024 * 1024:  # 100MB threshold
                self._add_alert("intrusion", f"Large data access detected for user: {user}")
                return True
        return False

    def monitor_failed_logins(self, user: str, max_attempts: int = 5) -> bool:
        """
        Monitor failed login attempts.
        Args:
            user: User identifier
            max_attempts: Maximum allowed attempts
        Returns:
            True if threshold exceeded
        """
        now = time.time()
        user_logins = self._failed_logins[user]
        # Add current failed login
        user_logins.append(now)
        # Check attempts in last 15 minutes
        cutoff_time = now - 900  # 15 minutes
        recent_attempts = [t for t in user_logins if t > cutoff_time]
        if len(recent_attempts) > max_attempts:
            self._add_alert("failed_login", f"User {user} exceeded {max_attempts} failed login attempts")
            return True
        return False

    def detect_anomaly(self, behavior_data: dict[str, Any]) -> bool:
        """
        Detect anomalous behavior.
        Args:
            behavior_data: Behavior data to analyze
        Returns:
            True if anomaly detected
        """
        behavior_type = behavior_data.get("type", "")
        user = behavior_data.get("user", "")
        value = behavior_data.get("value", 0)
        # Store event for pattern analysis
        self._event_history.append({
            "type": behavior_type,
            "user": user,
            "value": value,
            "timestamp": time.time()
        })
        # Simple statistical anomaly detection
        if len(self._event_history) < 10:
            return False  # Need more data
        # Get recent values for this behavior type
        recent_values = [
            e["value"] for e in self._event_history
            if e["type"] == behavior_type and isinstance(e["value"], (int, float))
        ]
        if len(recent_values) < 5:
            return False
        # Calculate mean and standard deviation
        mean = sum(recent_values) / len(recent_values)
        variance = sum((x - mean) ** 2 for x in recent_values) / len(recent_values)
        std_dev = variance ** 0.5
        if std_dev == 0:
            return False
        # Check if current value is anomaly (more than threshold standard deviations)
        z_score = abs((value - mean) / std_dev)
        if z_score > self._anomaly_threshold:
            self._add_alert("anomaly", f"Anomalous behavior detected: {behavior_type} for user: {user}")
            return True
        return False

    def get_security_alerts(self) -> list[dict[str, Any]]:
        """
        Get security alerts.
        Returns:
            List of security alerts
        """
        return self._security_alerts.copy()

    def clear_security_alerts(self) -> None:
        """Clear security alerts."""
        self._security_alerts.clear()

    def get_threat_level(self) -> SecurityLevel:
        """
        Get current threat level.
        Returns:
            Current threat level
        """
        return self._threat_level

    def set_threat_level(self, level: SecurityLevel) -> None:
        """
        Set threat level.
        Args:
            level: Threat level to set
        """
        self._threat_level = level
        logger.info(f"Threat level set to: {level.value}")

    def get_security_metrics(self) -> dict[str, Any]:
        """
        Get security metrics.
        Returns:
            Security metrics dictionary
        """
        now = time.time()
        one_hour_ago = now - 3600
        one_day_ago = now - 86400
        # Count alerts by severity
        alert_counts = defaultdict(int)
        for alert in self._security_alerts:
            alert_type = alert.get("type", "unknown")
            alert_counts[alert_type] += 1
        # Count failed logins
        recent_failed_logins = sum(
            len([t for t in logins if t > one_hour_ago])
            for logins in self._failed_logins.values()
        )
        # Count events
        recent_events = len([e for e in self._event_history if e.get("timestamp", 0) > one_hour_ago])
        return {
            "threat_level": self._threat_level.value,
            "security_level": self.security_level.value,
            "total_alerts": len(self._security_alerts),
            "alert_counts": dict(alert_counts),
            "failed_logins_last_hour": recent_failed_logins,
            "events_last_hour": recent_events,
            "total_failed_login_users": len(self._failed_logins),
            "monitoring_active": True,
        }

    def _check_unusual_access(self, user: str, resource: str, timestamp: float) -> bool:
        """
        Check for unusual access patterns.
        Args:
            user: User identifier
            resource: Resource identifier
            timestamp: Access timestamp
        Returns:
            True if unusual access detected
        """
        # Get user's access history
        user_accesses = [
            e for e in self._event_history
            if e.get("user") == user and e.get("type") == "access"
        ]
        if len(user_accesses) < 3:
            return False  # Not enough history
        # Check for access outside normal hours (simple heuristic)
        access_time = datetime.fromtimestamp(timestamp)
        hour = access_time.hour
        # Normal hours: 8 AM to 8 PM
        if hour < 8 or hour > 20:
            # Check if this is unusual for this user
            normal_hour_accesses = [
                e for e in user_accesses
                if 8 <= datetime.fromtimestamp(e.get("timestamp", 0)).hour <= 20
            ]
            if len(normal_hour_accesses) > len(user_accesses) * 0.8:
                # User usually accesses during normal hours
                return True
        return False

    def _add_alert(self, alert_type: str, message: str) -> None:
        """
        Add security alert.
        Args:
            alert_type: Alert type
            message: Alert message
        """
        alert = {
            "type": alert_type,
            "message": message,
            "timestamp": time.time(),
            "datetime": datetime.now().isoformat(),
            "severity": self._get_alert_severity(alert_type),
        }
        self._security_alerts.append(alert)
        # Keep only last 1000 alerts
        if len(self._security_alerts) > 1000:
            self._security_alerts = self._security_alerts[-1000:]
        logger.warning(f"Security alert [{alert_type}]: {message}")
        # Update threat level based on alerts
        self._update_threat_level()

    def _get_alert_severity(self, alert_type: str) -> str:
        """
        Get alert severity.
        Args:
            alert_type: Alert type
        Returns:
            Severity level
        """
        severity_map = {
            "intrusion": "high",
            "failed_login": "medium",
            "anomaly": "medium",
            "privilege_escalation": "critical",
            "data_exfiltration": "high",
        }
        return severity_map.get(alert_type, "low")

    def _update_threat_level(self) -> None:
        """Update threat level based on recent alerts."""
        if not self._security_alerts:
            self._threat_level = SecurityLevel.MEDIUM
            return
        # Get alerts from last hour
        now = time.time()
        one_hour_ago = now - 3600
        recent_alerts = [
            a for a in self._security_alerts
            if a.get("timestamp", 0) > one_hour_ago
        ]
        # Count critical/high severity alerts
        critical_count = sum(
            1 for a in recent_alerts
            if a.get("severity") in ["critical", "high"]
        )
        if critical_count >= 5:
            self._threat_level = SecurityLevel.CRITICAL
        elif critical_count >= 2:
            self._threat_level = SecurityLevel.HIGH
        elif critical_count >= 1:
            self._threat_level = SecurityLevel.MEDIUM
        else:
            self._threat_level = SecurityLevel.LOW

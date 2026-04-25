#!/usr/bin/env python3
"""
#exonware/xwsystem/src/exonware/xwsystem/notifications/__init__.py

Unified notification dispatcher.

A :class:`Notifier` multiplexes messages to one or more channels. The
channels themselves are pluggable — ship real Slack / email / SMS /
webhook implementations by registering a channel object that conforms
to :class:`INotificationChannel`.

This package provides working defaults: :class:`WebhookChannel` posts
HTTP JSON; :class:`InMemoryChannel` captures messages for tests and
eventual UI replay.
"""

from .facade import (
    INotificationChannel,
    InMemoryChannel,
    Notifier,
    Notification,
    Severity,
    WebhookChannel,
)

__all__ = [
    "INotificationChannel",
    "InMemoryChannel",
    "Notifier",
    "Notification",
    "Severity",
    "WebhookChannel",
]

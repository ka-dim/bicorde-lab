"""Diagnostic client for bicorde-lab."""

from .diagnostic import health_is_ok, version_is_valid

__all__ = ["health_is_ok", "version_is_valid"]

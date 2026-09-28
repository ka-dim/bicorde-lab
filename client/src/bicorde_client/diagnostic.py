"""Validation helpers for server responses."""


def health_is_ok(payload: dict[str, object]) -> bool:
    """Return whether a health payload reports an OK status."""
    return payload.get("status") == "ok"


def version_is_valid(payload: dict[str, object]) -> bool:
    """Return whether a version payload contains a non-empty string."""
    version = payload.get("version")
    return isinstance(version, str) and bool(version.strip())

"""HTTP client for the Rust diagnostic server."""

import json
from urllib.request import urlopen

from .diagnostic import health_is_ok, version_is_valid


def fetch_json(url: str) -> dict[str, object]:
    """Fetch a JSON object from an HTTP endpoint."""
    with urlopen(url, timeout=5) as response:
        payload = json.load(response)

    if not isinstance(payload, dict):
        raise ValueError("the server response is not a JSON object")

    return payload


def server_is_healthy(base_url: str) -> bool:
    """Check the health and version endpoints of a server."""
    normalized_url = base_url.rstrip("/")

    health_payload = fetch_json(f"{normalized_url}/health")
    version_payload = fetch_json(f"{normalized_url}/version")

    return health_is_ok(health_payload) and version_is_valid(version_payload)

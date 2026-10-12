"""Command-line entry point for the diagnostic client."""

import sys
from urllib.error import URLError

from .http_client import server_is_healthy


def main() -> int:
    base_url = sys.argv[1] if len(sys.argv) > 1 else "http://127.0.0.1:3000"

    try:
        healthy = server_is_healthy(base_url)
    except (URLError, ValueError) as error:
        print(f"Server diagnostic failed: {error}")
        return 1

    if not healthy:
        print("Server returned invalid diagnostic data")
        return 1

    print(f"Server diagnostic succeeded: {base_url}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

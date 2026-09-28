from bicorde_client import health_is_ok, version_is_valid


def test_health_accepts_ok_status() -> None:
    assert health_is_ok({"status": "ok"})


def test_health_rejects_other_status() -> None:
    assert not health_is_ok({"status": "error"})


def test_version_accepts_non_empty_string() -> None:
    assert version_is_valid({"version": "0.1.0"})


def test_version_rejects_empty_string() -> None:
    assert not version_is_valid({"version": "  "})

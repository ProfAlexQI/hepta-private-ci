#!/usr/bin/env python3
"""Verify the version-controlled caller set for Hepta authority constructors."""

from __future__ import annotations

import json
import pathlib
import sys

ROOT = pathlib.Path(__file__).resolve().parents[1]
MANIFEST = ROOT / "docs/architecture/HEPTA_AUTHORITY_CALLERS_V1.json"
SEARCH_ROOT = ROOT / "codex-rs"
CONSTRUCTORS = (
    "AuthorityGrant::snapshot_read_only(",
    "AuthorityGrant::agent_local(",
    "AuthorityGrant::qualification_cognitive_write(",
)


def fail(message: str) -> "NoReturn":
    raise SystemExit(f"FAIL_HEPTA_AUTHORITY_CALLERS_P0_1: {message}")


def main() -> int:
    try:
        manifest = json.loads(MANIFEST.read_text(encoding="utf-8"))
    except (OSError, UnicodeError, json.JSONDecodeError) as error:
        fail(f"cannot read authority caller manifest: {error}")
    callers = manifest.get("grantConstructorCallers")
    if not isinstance(callers, dict) or not callers:
        fail("grantConstructorCallers must be a non-empty object")
    allowed_paths = set(callers)
    for relative in allowed_paths:
        if not (ROOT / relative).is_file():
            fail(f"allowed caller path is missing: {relative}")

    observed: dict[str, list[str]] = {}
    forbidden_requests = manifest.get("forbiddenProductRequests")
    if not isinstance(forbidden_requests, list) or not all(
        isinstance(value, str) for value in forbidden_requests
    ):
        fail("forbiddenProductRequests must be a string list")
    allowed_forbidden_path = manifest.get("allowedForbiddenRequestPath")
    if not isinstance(allowed_forbidden_path, str):
        fail("allowedForbiddenRequestPath must be a string")

    for path in sorted(SEARCH_ROOT.rglob("*.rs")):
        relative = path.relative_to(ROOT).as_posix()
        try:
            source = path.read_text(encoding="utf-8")
        except (OSError, UnicodeError) as error:
            fail(f"cannot read {relative}: {error}")
        found = [constructor for constructor in CONSTRUCTORS if constructor in source]
        if found:
            observed[relative] = found
            if relative not in allowed_paths:
                fail(f"unregistered authority constructor caller: {relative}: {found}")
        if relative != allowed_forbidden_path:
            for request in forbidden_requests:
                if request in source:
                    fail(f"product source {relative} requests forbidden capability {request}")

    missing_observed = sorted(allowed_paths - set(observed))
    if missing_observed:
        fail(f"caller manifest contains paths with no constructor call: {missing_observed}")

    legacy = manifest.get("legacyProductionAdapter")
    if not isinstance(legacy, dict):
        fail("legacyProductionAdapter must be an object")
    legacy_path = legacy.get("path")
    if not isinstance(legacy_path, str) or not (ROOT / legacy_path).is_file():
        fail("legacy production adapter path is missing")
    legacy_source = (ROOT / legacy_path).read_text(encoding="utf-8")
    if "adopt_verified_legacy_cognitive_write" not in legacy_source:
        fail("legacy adapter adoption function is missing")
    if "LegacyProductionAuthorityVerifier" not in legacy_source:
        fail("legacy adapter external verifier seam is missing")
    for forbidden in forbidden_requests:
        if forbidden in legacy_source:
            fail(f"legacy adapter requests forbidden capability {forbidden}")
    if legacy.get("mayIssueGrant") is not False:
        fail("legacy adapter mayIssueGrant must remain false")
    if legacy.get("requiresExternalVerifier") is not True:
        fail("legacy adapter must require an external verifier")
    if legacy.get("existingProductionWriterCallerMigrated") is not False:
        fail("source manifest cannot claim the existing writer caller is migrated")

    authority = manifest.get("authority")
    if not isinstance(authority, dict) or any(authority.values()):
        fail("authority caller manifest must retain a fully closed authority boundary")

    print("PASS_HEPTA_AUTHORITY_CALLERS_P0_1_SOURCE_ONLY")
    return 0


if __name__ == "__main__":
    sys.exit(main())

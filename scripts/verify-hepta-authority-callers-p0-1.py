#!/usr/bin/env python3
"""Verify the version-controlled caller set for Hepta authority constructors."""

from __future__ import annotations

import json
import pathlib
import sys
from typing import NoReturn

ROOT = pathlib.Path(__file__).resolve().parents[1]
MANIFEST = ROOT / "docs/architecture/HEPTA_AUTHORITY_CALLERS_V1.json"
SEARCH_ROOT = ROOT / "codex-rs"
CONSTRUCTORS = (
    "AuthorityGrant::snapshot_read_only(",
    "AuthorityGrant::agent_local(",
    "AuthorityGrant::qualification_cognitive_write(",
)
PRODUCTION_WRITER_OPEN = "ProductionDurableWriter::open("


def fail(message: str) -> NoReturn:
    raise SystemExit(f"FAIL_HEPTA_AUTHORITY_CALLERS_P0_2: {message}")


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
    cross_owner_writer_open_callers: list[str] = []
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
        if PRODUCTION_WRITER_OPEN in source and not relative.startswith(
            "codex-rs/hepta-memory/"
        ):
            cross_owner_writer_open_callers.append(relative)

    missing_observed = sorted(allowed_paths - set(observed))
    if missing_observed:
        fail(f"caller manifest contains paths with no constructor call: {missing_observed}")

    legacy = manifest.get("legacyProductionAdapter")
    if not isinstance(legacy, dict):
        fail("legacyProductionAdapter must be an object")
    legacy_path = legacy.get("path")
    if not isinstance(legacy_path, str) or not (ROOT / legacy_path).is_file():
        fail("legacy production adapter path is missing")
    candidate_path = legacy.get("candidateFacadePath")
    if not isinstance(candidate_path, str) or not (ROOT / candidate_path).is_file():
        fail("legacy facade candidate path is missing")
    writer_host_path = legacy.get("productionWriterHostPath")
    if not isinstance(writer_host_path, str) or not (ROOT / writer_host_path).is_file():
        fail("production writer host path is missing")

    legacy_source = (ROOT / legacy_path).read_text(encoding="utf-8")
    required_adapter_markers = (
        "ProductionCognitiveWriteAuthorization",
        "authorize_verified_capability::<CognitiveWriteCapability",
        "ProductionAuthorityVerifier",
        "AuthorityLeaseBinding::new(",
    )
    for marker in required_adapter_markers:
        if marker not in legacy_source:
            fail(f"legacy adapter marker is missing: {marker}")
    for forbidden in forbidden_requests:
        if forbidden in legacy_source:
            fail(f"legacy adapter requests forbidden capability {forbidden}")
    if legacy.get("mayIssueGrant") is not False:
        fail("legacy adapter mayIssueGrant must remain false")
    if legacy.get("requiresExternalVerifier") is not True:
        fail("legacy adapter must require an external verifier")
    if legacy.get("requiresTypedExternalEffectCapability") is not True:
        fail("production dispatch must require a separate typed effect capability")
    if legacy.get("existingProductionWriterCallerMigrated") is not True:
        fail("existing production writer caller must be marked migrated")

    writer_host_source = (ROOT / writer_host_path).read_text(encoding="utf-8")
    if writer_host_source.count("ProductionCognitiveWriteAuthorization::verify(") < 2:
        fail("all production writer open paths must pass through the typed adapter")
    required_host_markers = (
        "cognitive_write: Authorized<CognitiveWriteCapability>",
        "external_effect: Option<Authorized<ExternalEffectCapability>>",
        "external_effect: Authorized<ExternalEffectCapability>",
        "validate_external_effect_capability(",
        "production external-effect capability is not explicitly attached",
    )
    for marker in required_host_markers:
        if marker not in writer_host_source:
            fail(f"production writer host marker is missing: {marker}")
    if "pub fn attach_target(mut self, target: Arc<dyn ProductionOutboxTarget>) -> Self" in writer_host_source:
        fail("production target attachment still bypasses typed effect authority")
    if sorted(set(cross_owner_writer_open_callers)) != [writer_host_path]:
        fail(
            "cross-owner production writer open callers are not fully migrated: "
            f"{sorted(set(cross_owner_writer_open_callers))}"
        )

    authority = manifest.get("authority")
    if not isinstance(authority, dict) or any(authority.values()):
        fail("authority caller manifest must retain a fully closed authority boundary")

    print("PASS_HEPTA_AUTHORITY_CALLERS_P0_2_MIGRATED_SOURCE_ONLY")
    return 0


if __name__ == "__main__":
    sys.exit(main())

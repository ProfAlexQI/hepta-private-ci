#!/usr/bin/env python3
"""Verify the repository contract for C1 Servo worker build-input packet orchestration."""
from __future__ import annotations

import json
import pathlib
import sys
from typing import Any

ROOT = pathlib.Path(__file__).resolve().parents[1]
TOOL = ROOT / "scripts/hepta-servo-worker-build-manifest.py"
TEST = ROOT / "scripts/tests/test_hepta_servo_worker_build_manifest.py"
PACKET_SCHEMA = ROOT / "docs/hepta-vnext/browser/hepta.servo.worker_build_input_packet.v1.schema.json"
MANIFEST_SCHEMA = ROOT / "docs/hepta-vnext/browser/hepta.servo.worker_build_manifest.v1.schema.json"


class VerificationError(RuntimeError):
    pass


def fail(message: str) -> None:
    raise VerificationError(message)


def load(path: pathlib.Path) -> dict[str, Any]:
    if not path.is_file():
        fail(f"missing contract file: {path.relative_to(ROOT)}")
    try:
        value = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, UnicodeError, json.JSONDecodeError) as error:
        fail(f"cannot parse {path.relative_to(ROOT)}: {error}")
    if not isinstance(value, dict):
        fail(f"{path.relative_to(ROOT)} must contain one object")
    return value


def main() -> int:
    try:
        packet = load(PACKET_SCHEMA)
        manifest = load(MANIFEST_SCHEMA)
        for path in (TOOL, TEST):
            if not path.is_file():
                fail(f"missing implementation file: {path.relative_to(ROOT)}")

        if packet.get("$schema") != "https://json-schema.org/draft/2020-12/schema":
            fail("build input packet schema must use JSON Schema draft 2020-12")
        properties = packet.get("properties")
        if not isinstance(properties, dict):
            fail("build input packet schema lacks properties")
        if properties.get("schema", {}).get("const") != "hepta.servo.worker_build_input_packet.v1":
            fail("build input packet schema identity drifted")
        if properties.get("claim_level", {}).get("const") != "WORKER_BUILD_INPUTS_FROZEN_ONLY":
            fail("build input packet claim level drifted")
        artifact = properties.get("artifact", {}).get("properties", {})
        if artifact.get("worker_artifact_built", {}).get("const") is not False:
            fail("build input packet may not claim a worker artifact")
        if artifact.get("runtime_qualified", {}).get("const") is not False:
            fail("build input packet may not claim runtime qualification")
        authority = packet.get("$defs", {}).get("authority", {}).get("properties", {})
        if not authority or any(definition.get("const") is not False for definition in authority.values()):
            fail("build input packet authority schema is not all false")

        manifest_properties = manifest.get("properties")
        if not isinstance(manifest_properties, dict):
            fail("worker build manifest schema lacks properties")
        for key in (
            "network_access_during_build",
            "worker_tcp_listener",
            "worker_http_surface",
            "worker_external_network",
            "worker_credential_export",
            "worker_production_authority",
            "worker_effect_authority",
        ):
            if manifest_properties.get(key, {}).get("const") is not False:
                fail(f"worker build manifest schema widened {key}")

        source = TOOL.read_text(encoding="utf-8")
        for forbidden in (
            "import socket",
            "import urllib",
            "import requests",
            "http.client",
            "urlopen(",
            "os.environ[",
            "subprocess",
        ):
            if forbidden in source:
                fail(f"build manifest tool contains forbidden ambient/network surface: {forbidden}")
        required_tokens = (
            "BUILD_INPUTS_FROZEN_ARTIFACT_AND_RUNTIME_NOT_QUALIFIED",
            "BUILD_INPUTS_RECOMPUTED_ARTIFACT_AND_RUNTIME_NOT_QUALIFIED",
            "CARGO_NET_OFFLINE",
            "environment_values",
            "worker_artifact_built\": False",
            "runtime_qualified\": False",
            "os.O_EXCL",
            "reject_duplicate_keys",
            "validate_source_bundle",
            "validate_patch_inventory",
            "validate_license_packet",
            "validate_sbom",
        )
        for token in required_tokens:
            if token not in source:
                fail(f"build manifest tool is missing {token}")

        tests = TEST.read_text(encoding="utf-8")
        expected_tests = (
            "test_create_and_verify_recompute_exact_inputs",
            "test_output_is_create_only",
            "test_tampered_environment_fails_recompute",
            "test_unknown_environment_key_fails_closed",
            "test_positive_build_network_fails_closed",
            "test_noncanonical_supporting_json_fails_closed",
            "test_absolute_path_in_build_command_fails_closed",
        )
        for test in expected_tests:
            if f"def {test}" not in tests:
                fail(f"build manifest fixture test is missing: {test}")
    except VerificationError as error:
        print(json.dumps({"status": "FAIL_CLOSED", "error": str(error)}, sort_keys=True))
        return 1

    print(
        json.dumps(
            {
                "schema": "hepta.servo.worker_build_inputs_contract_verification.v1",
                "status": "PASS_FIXTURE_CONTRACT_ONLY",
                "canonical_servo_source_bundle_present": False,
                "worker_artifact_built": False,
                "runtime_qualified": False,
                "authority": "all_false",
            },
            sort_keys=True,
            separators=(",", ":"),
        )
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())

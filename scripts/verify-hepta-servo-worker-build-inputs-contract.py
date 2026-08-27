#!/usr/bin/env python3
"""Verify the repository contract for C1 Servo worker build-input orchestration."""
from __future__ import annotations

import json
import pathlib
import sys
from typing import Any

ROOT = pathlib.Path(__file__).resolve().parents[1]
ENGINE = ROOT / "scripts/hepta-servo-worker-build-manifest.py"
ENTRYPOINT = ROOT / "scripts/hepta-servo-worker-build-inputs.py"
ENGINE_TEST = ROOT / "scripts/tests/test_hepta_servo_worker_build_manifest.py"
POLICY_TEST = ROOT / "scripts/tests/test_hepta_servo_worker_build_policy.py"
WORKFLOW = ROOT / ".github/workflows/hepta-servo-worker-build-inputs-contract.yml"
BINDING = ROOT / "docs/hepta-vnext/browser/C1_BUILD_INPUT_BINDING.md"
STATUS = ROOT / "docs/hepta-vnext/browser/C1_BUILD_STATUS.json"
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
        status = load(STATUS)
        for path in (ENGINE, ENTRYPOINT, ENGINE_TEST, POLICY_TEST, WORKFLOW, BINDING):
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
        if not authority or any(
            definition.get("const") is not False for definition in authority.values()
        ):
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

        engine = ENGINE.read_text(encoding="utf-8")
        entrypoint = ENTRYPOINT.read_text(encoding="utf-8")
        for source, label in ((engine, "engine"), (entrypoint, "entrypoint")):
            for forbidden in (
                "import socket",
                "import urllib",
                "import requests",
                "http.client",
                "urlopen(",
                "subprocess",
            ):
                if forbidden in source:
                    fail(f"build-input {label} contains forbidden ambient/network surface: {forbidden}")
        for token in (
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
        ):
            if token not in engine:
                fail(f"build manifest engine is missing {token}")
        for token in (
            "build command must invoke Cargo directly",
            "build command must include --locked",
            "build command must include --offline",
            "duplicate Cargo features are forbidden",
            "SECRET_KEY_MARKERS",
            "engine.main(arguments)",
        ):
            if token not in entrypoint:
                fail(f"strict build-input entrypoint is missing {token}")

        engine_tests = ENGINE_TEST.read_text(encoding="utf-8")
        for test in (
            "test_create_and_verify_recompute_exact_inputs",
            "test_output_is_create_only",
            "test_tampered_environment_fails_recompute",
            "test_unknown_environment_key_fails_closed",
            "test_positive_build_network_fails_closed",
            "test_noncanonical_supporting_json_fails_closed",
            "test_absolute_path_in_build_command_fails_closed",
        ):
            if f"def {test}" not in engine_tests:
                fail(f"build manifest fixture test is missing: {test}")
        policy_tests = POLICY_TEST.read_text(encoding="utf-8")
        for test in (
            "test_valid_locked_offline_cargo_build_is_accepted",
            "test_non_cargo_executable_is_rejected",
            "test_registry_or_acquisition_operation_is_rejected",
            "test_missing_locked_is_rejected",
            "test_missing_offline_is_rejected",
            "test_duplicate_feature_is_rejected",
            "test_newline_in_command_is_rejected",
            "test_secret_or_multiline_environment_is_rejected",
        ):
            if f"def {test}" not in policy_tests:
                fail(f"strict build policy test is missing: {test}")

        workflow = WORKFLOW.read_text(encoding="utf-8")
        for token in (
            "workflow_call:",
            "hepta-servo-worker-build-inputs.py",
            "test_hepta_servo_worker_build_manifest.py",
            "test_hepta_servo_worker_build_policy.py",
            "verify-hepta-focused-gates.py",
            "canonical_servo_source_bundle_present=false",
            "runtime_qualified=false",
        ):
            if token not in workflow:
                fail(f"build-input workflow is missing {token}")
        binding = BINDING.read_text(encoding="utf-8")
        if "scripts/hepta-servo-worker-build-inputs.py" not in binding:
            fail("build-input binding does not identify the strict canonical entrypoint")
        if status.get("status") != "BUILD_INPUT_FREEZER_IMPLEMENTED_EXACT_SERVO_EVIDENCE_PENDING":
            fail("build status overclaims or drifted")
        if status.get("merge_authorized") is not False:
            fail("build status may not authorize merge")
        if any(value is not False for value in status.get("authority", {}).values()):
            fail("build status contains positive authority")
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

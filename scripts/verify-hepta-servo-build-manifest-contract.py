#!/usr/bin/env python3
"""Static merge-gate verifier for the C1 build-manifest contract."""
from __future__ import annotations

import json
import pathlib
import sys

ROOT = pathlib.Path(__file__).resolve().parents[1]
TOOL = ROOT / "scripts/hepta-servo-build-manifest.py"
CORE = ROOT / "scripts/hepta_servo_build_manifest_core.py"
IO = ROOT / "scripts/hepta_servo_build_manifest_io.py"
TEST = ROOT / "scripts/tests/test_hepta_servo_build_manifest.py"
WORKFLOW = ROOT / ".github/workflows/hepta-servo-build-manifest-contract.yml"
SCHEMA = ROOT / "docs/hepta-vnext/browser/hepta.servo.worker_build_manifest.v1.schema.json"
STATUS = ROOT / "docs/hepta-vnext/browser/C1_BUILD_MANIFEST_STATUS.json"
FALSE_FIELDS = {
    "network_access_during_build", "worker_tcp_listener", "worker_http_surface",
    "worker_external_network", "worker_credential_export", "worker_production_authority",
    "worker_effect_authority",
}


def fail(message: str) -> None:
    raise RuntimeError(message)


def main() -> int:
    try:
        for path in (TOOL, CORE, IO, TEST, WORKFLOW, SCHEMA, STATUS):
            if not path.is_file(): fail(f"missing {path.relative_to(ROOT)}")
        source = TOOL.read_text() + CORE.read_text() + IO.read_text()
        for token in (
            "SOURCE_PIN_VERIFIED_BUILD_NOT_QUALIFIED", "SPDX-2.3-json",
            "PATH_DIGEST_SHA256", "cargo build or cargo rustc", "0o600",
            "output is create-only", "worker_production_authority\": False",
            "worker_effect_authority\": False",
        ):
            if token not in source: fail(f"tool is missing {token}")
        for forbidden in ("requests", "urllib", "socket.", "subprocess.run", "os.environ.items"):
            if forbidden in source: fail(f"tool contains forbidden dynamic surface {forbidden}")
        tests = TEST.read_text()
        for name in (
            "test_snapshot_verify_is_canonical_sorted_closed_and_create_only",
            "test_duplicate_feature_fails", "test_secret_or_unknown_environment_fails",
            "test_positive_source_authority_fails", "test_input_tamper_breaks_verification",
            "test_noncanonical_json_fails", "test_registry_mutating_command_fails",
        ):
            if f"def {name}" not in tests: fail(f"missing test {name}")
        workflow = WORKFLOW.read_text()
        for token in ("workflow_call:", "py_compile", "test_hepta_servo_build_manifest.py",
                      "verify-hepta-servo-build-manifest-contract.py", "servo_build_performed=false",
                      "runtime_authority=false"):
            if token not in workflow: fail(f"workflow is missing {token}")
        schema = json.loads(SCHEMA.read_text()); properties = schema["properties"]
        if set(schema["required"]) != set(properties): fail("build manifest schema required fields drifted")
        for field in FALSE_FIELDS:
            if properties.get(field) != {"const": False}: fail(f"schema widens {field}")
        status = json.loads(STATUS.read_text())
        if status.get("state") != "IMPLEMENTED_CONTRACT_EVIDENCE_PENDING": fail("status overclaims evidence")
        if any(value is not False for value in status.get("authority", {}).values()): fail("status enables authority")
    except (OSError, KeyError, ValueError, RuntimeError) as error:
        print(f"HEPTA_SERVO_BUILD_MANIFEST_CONTRACT=FAIL: {error}", file=sys.stderr); return 1
    print('{"authority":"all_false","claim":"build_inputs_only","network":false,"status":"HEPTA_SERVO_BUILD_MANIFEST_CONTRACT_PASS"}')
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

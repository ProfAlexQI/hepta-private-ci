#!/usr/bin/env python3
"""Verify the repository-native Hepta browser plan bundle.

The .yaml files intentionally contain JSON, which is valid YAML and keeps this
merge gate independent of third-party Python packages.
"""

from __future__ import annotations

import json
import re
import sys
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[1]
BUNDLE = ROOT / "docs/hepta-vnext/browser"
CURRENT_PATH = BUNDLE / "CURRENT.yaml"
STAGE_PATH = BUNDLE / "STAGE_MATRIX.yaml"
TRACE_PATH = BUNDLE / "TRACEABILITY_MATRIX.yaml"
SERVO_PATH = BUNDLE / "SERVO_UPSTREAM_PIN.json"
SCHEMA_PATH = BUNDLE / "hepta.browser.qualification_receipt.v1.schema.json"
PLAN_PATH = BUNDLE / "EXECUTION_PLAN.md"
THREAT_PATH = BUNDLE / "THREAT_MODEL.md"
README_PATH = BUNDLE / "README.md"

EXPECTED_STAGES = [f"WEB_C{i}" for i in range(8)]
HEX40 = re.compile(r"^[0-9a-f]{40}$")
LOCAL_PATH_MARKERS = ("/Users/", "/Volumes/T5", "/home/qian", "C:\\Users\\")
AUTHORITY_KEYS = {
    "runtime_authority",
    "effect_authority",
    "production_caller",
    "production_writer",
    "external_network",
    "raw_cookie_export",
    "credential_export",
    "operator_acceptance",
    "promotion",
    "release_qualified",
}


class VerificationError(RuntimeError):
    pass


def fail(message: str) -> None:
    raise VerificationError(message)


def load_json(path: Path) -> dict[str, Any]:
    if not path.is_file():
        fail(f"missing required file: {path.relative_to(ROOT)}")
    try:
        value = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, UnicodeError, json.JSONDecodeError) as error:
        fail(f"cannot parse {path.relative_to(ROOT)} as canonical JSON/YAML: {error}")
    if not isinstance(value, dict):
        fail(f"top-level value must be an object: {path.relative_to(ROOT)}")
    return value


def require_repo_path(value: Any, label: str) -> Path:
    if not isinstance(value, str) or not value or value.startswith(("/", "~")):
        fail(f"{label} must be a non-empty repository-relative path")
    path = ROOT / value
    if not path.is_file():
        fail(f"{label} points to a missing file: {value}")
    return path


def verify_no_local_paths(paths: list[Path]) -> None:
    for path in paths:
        text = path.read_text(encoding="utf-8")
        for marker in LOCAL_PATH_MARKERS:
            if marker in text:
                fail(f"machine-local path marker {marker!r} found in {path.relative_to(ROOT)}")


def verify_current(current: dict[str, Any]) -> list[Path]:
    if current.get("schema") != "hepta.browser.current.v1" or current.get("schema_version") != 1:
        fail("CURRENT.yaml schema/version mismatch")
    if current.get("phase") != "DEVELOPMENT":
        fail("browser bundle must remain in DEVELOPMENT")
    if current.get("claim_level") != "L1_QUALIFICATION_ONLY":
        fail("browser bundle must remain qualification-only")
    if current.get("fail_closed") is not True:
        fail("CURRENT.yaml must declare fail_closed=true")

    authority = current.get("authority")
    if not isinstance(authority, dict):
        fail("CURRENT.yaml authority must be an object")
    if set(authority) != AUTHORITY_KEYS:
        fail(f"CURRENT.yaml authority keys differ: {sorted(set(authority) ^ AUTHORITY_KEYS)}")
    enabled = sorted(key for key, value in authority.items() if value is not False)
    if enabled:
        fail(f"qualification bundle attempted to enable authority: {enabled}")

    pointers = [
        "canonical_plan",
        "stage_matrix",
        "traceability_matrix",
        "threat_model",
        "receipt_schema",
        "servo_pin",
    ]
    return [require_repo_path(current.get(key), f"CURRENT.yaml#{key}") for key in pointers]


def verify_stages(matrix: dict[str, Any]) -> set[str]:
    if matrix.get("schema") != "hepta.browser.stage_matrix.v1" or matrix.get("schema_version") != 1:
        fail("STAGE_MATRIX schema/version mismatch")
    if matrix.get("execution_order") != EXPECTED_STAGES:
        fail("STAGE_MATRIX execution_order must be WEB_C0 through WEB_C7")
    stages = matrix.get("stages")
    if not isinstance(stages, list) or len(stages) != len(EXPECTED_STAGES):
        fail("STAGE_MATRIX must define exactly eight stages")
    by_id: dict[str, dict[str, Any]] = {}
    for stage in stages:
        if not isinstance(stage, dict) or not isinstance(stage.get("id"), str):
            fail("every stage must be an object with an id")
        stage_id = stage["id"]
        if stage_id in by_id:
            fail(f"duplicate stage: {stage_id}")
        by_id[stage_id] = stage
        for code_path in stage.get("code_paths", []):
            require_repo_path(code_path, f"{stage_id}.code_paths")
    if list(by_id) != EXPECTED_STAGES:
        fail(f"stage objects are not ordered C0-C7: {list(by_id)}")

    expected_status = {
        "WEB_C0": "IMPLEMENTED_QUALIFICATION_ONLY",
        "WEB_C1": "NOT_IMPLEMENTED_SOURCE_PINNED",
        "WEB_C2": "IMPLEMENTED_FIXTURE_ONLY",
        "WEB_C3": "IMPLEMENTED_FIXTURE_ONLY",
        "WEB_C4": "NOT_IMPLEMENTED",
        "WEB_C5": "NOT_IMPLEMENTED",
        "WEB_C6": "NOT_IMPLEMENTED",
        "WEB_C7": "NOT_IMPLEMENTED",
    }
    actual_status = {stage_id: stage.get("status") for stage_id, stage in by_id.items()}
    if actual_status != expected_status:
        fail(f"stage status drift: {actual_status}")

    for stage_id, stage in by_id.items():
        for dependency in stage.get("depends_on", []):
            if dependency not in by_id:
                fail(f"{stage_id} has unknown dependency {dependency}")
            if EXPECTED_STAGES.index(dependency) >= EXPECTED_STAGES.index(stage_id):
                fail(f"{stage_id} depends on non-prior stage {dependency}")
        if not isinstance(stage.get("receipt_kind"), str) or not stage["receipt_kind"]:
            fail(f"{stage_id} is missing receipt_kind")
    return set(by_id)


def verify_traceability(trace: dict[str, Any], stage_ids: set[str]) -> int:
    if trace.get("schema") != "hepta.browser.traceability_matrix.v1" or trace.get("schema_version") != 1:
        fail("TRACEABILITY_MATRIX schema/version mismatch")
    requirements = trace.get("requirements")
    if not isinstance(requirements, list) or not requirements:
        fail("TRACEABILITY_MATRIX must contain requirements")
    seen: set[str] = set()
    for requirement in requirements:
        if not isinstance(requirement, dict):
            fail("traceability requirement must be an object")
        requirement_id = requirement.get("id")
        if not isinstance(requirement_id, str) or not requirement_id:
            fail("traceability requirement is missing id")
        if requirement_id in seen:
            fail(f"duplicate requirement id: {requirement_id}")
        seen.add(requirement_id)
        if requirement.get("stage") not in stage_ids:
            fail(f"{requirement_id} references unknown stage")
        if requirement.get("release_blocking") is not True:
            fail(f"{requirement_id} must remain release_blocking")
        for code_path in requirement.get("code", []):
            require_repo_path(code_path, f"{requirement_id}.code")
        for document_path in requirement.get("documents", []):
            require_repo_path(document_path, f"{requirement_id}.documents")
    return len(seen)


def verify_servo(pin: dict[str, Any]) -> None:
    if pin.get("schema") != "hepta.browser.servo_upstream_pin.v1" or pin.get("schema_version") != 1:
        fail("SERVO_UPSTREAM_PIN schema/version mismatch")
    if pin.get("repository") != "servo/servo":
        fail("Servo pin must target servo/servo")
    if not HEX40.fullmatch(str(pin.get("commit", ""))) or not HEX40.fullmatch(str(pin.get("tree", ""))):
        fail("Servo commit/tree must be exact lowercase 40-hex IDs")
    if pin.get("license") != "MPL-2.0":
        fail("Servo license binding must be MPL-2.0")
    if pin.get("integration_status") != "SOURCE_PIN_ONLY_NOT_IMPORTED":
        fail("Servo must remain source-pin-only in this slice")
    authority = pin.get("authority")
    if not isinstance(authority, dict) or any(value is not False for value in authority.values()):
        fail("Servo pin cannot enable authority")


def verify_receipt_schema(schema: dict[str, Any]) -> None:
    if schema.get("$schema") != "https://json-schema.org/draft/2020-12/schema":
        fail("qualification receipt must use JSON Schema draft 2020-12")
    properties = schema.get("properties")
    if not isinstance(properties, dict):
        fail("qualification receipt schema has no properties")
    stage_enum = properties.get("stage", {}).get("enum")
    if stage_enum != EXPECTED_STAGES:
        fail("qualification receipt stage enum drift")
    authority = properties.get("authority", {})
    authority_properties = authority.get("properties", {})
    if set(authority_properties) != AUTHORITY_KEYS:
        fail("qualification receipt authority keys drift")
    for key, definition in authority_properties.items():
        if not isinstance(definition, dict) or definition.get("const") is not False:
            fail(f"qualification receipt may not enable authority field {key}")


def main() -> int:
    try:
        current = load_json(CURRENT_PATH)
        referenced_paths = verify_current(current)
        matrix = load_json(STAGE_PATH)
        stage_ids = verify_stages(matrix)
        requirement_count = verify_traceability(load_json(TRACE_PATH), stage_ids)
        verify_servo(load_json(SERVO_PATH))
        verify_receipt_schema(load_json(SCHEMA_PATH))
        verify_no_local_paths(
            [CURRENT_PATH, STAGE_PATH, TRACE_PATH, SERVO_PATH, SCHEMA_PATH, PLAN_PATH, THREAT_PATH, README_PATH]
        )
    except VerificationError as error:
        print(f"hepta browser plan verification failed: {error}", file=sys.stderr)
        return 1

    summary = {
        "schema": "hepta.browser.plan_verification.v1",
        "status": "PASS_QUALIFICATION_PLAN",
        "stages": EXPECTED_STAGES,
        "requirements": requirement_count,
        "servo_commit": load_json(SERVO_PATH)["commit"],
        "authority": "all_false",
        "referenced_files": sorted(str(path.relative_to(ROOT)) for path in referenced_paths),
    }
    print(json.dumps(summary, sort_keys=True, separators=(",", ":")))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

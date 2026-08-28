#!/usr/bin/env python3
"""Source-only gate for the stacked INF-0C evidence-v2 tranche."""

from __future__ import annotations

import json
import sys
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[1]
MINIMAL = ROOT / "scripts/hepta-inference-inf0c-real-e2e.py"
EVIDENCE = ROOT / "scripts/hepta-inference-inf0c-evidence-v2.py"
SELF_TEST = ROOT / "scripts/hepta-inference-inf0c-evidence-v2-loopback-selftest.py"
CONTRACT = ROOT / "docs/hepta-vnext/inference/HEPTA_INFERENCE_INF0C_EVIDENCE_HARNESS_V2.md"
STATUS = ROOT / "docs/hepta-vnext/inference/HEPTA_INFERENCE_IMPLEMENTATION_STATUS_V1.json"
MATRIX = ROOT / "docs/hepta-vnext/inference/HEPTA_INFERENCE_STAGE_MATRIX_V2.json"
WORKFLOW = ROOT / ".github/workflows/hepta-inference-inf0c-evidence-v2.yml"
PASS = "PASS_HEPTA_INFERENCE_INF0C_EVIDENCE_V2_SOURCE_ONLY"

FALSE_AUTHORITY = (
    "production_listener",
    "production_writer",
    "provider_effect",
    "external_effect",
    "shared_kg_write",
    "memory_write",
    "route_write",
    "fleet_write",
    "model_npu",
    "remote_inference",
    "automatic_model_install",
    "operator_acceptance",
    "promotion",
    "release",
)


class GateError(RuntimeError):
    pass


def require(condition: bool, message: str) -> None:
    if not condition:
        raise GateError(message)


def text(path: Path) -> str:
    require(path.is_file(), f"missing {path.relative_to(ROOT)}")
    return path.read_text(encoding="utf-8")


def object_json(path: Path) -> dict[str, Any]:
    try:
        value = json.loads(text(path))
    except json.JSONDecodeError as error:
        raise GateError(f"invalid JSON in {path.relative_to(ROOT)}: {error}") from error
    require(isinstance(value, dict), f"{path.relative_to(ROOT)} must contain an object")
    return value


def require_markers(source: str, markers: tuple[str, ...], label: str) -> None:
    for marker in markers:
        require(marker in source, f"{label} missing marker: {marker}")


def require_closed_authority(value: dict[str, Any], label: str) -> None:
    authority = value.get("authority")
    require(isinstance(authority, dict), f"{label}.authority is missing")
    require(authority.get("qualification_only") is True, f"{label} is not qualification-only")
    for field in FALSE_AUTHORITY:
        require(authority.get(field) is False, f"{label}.authority.{field} must be false")


def main() -> int:
    minimal = text(MINIMAL)
    evidence = text(EVIDENCE)
    self_test = text(SELF_TEST)
    contract = text(CONTRACT)
    workflow = text(WORKFLOW)
    status = object_json(STATUS)
    matrix = object_json(MATRIX)

    require_markers(
        minimal,
        (
            'EXPECTED_OUTPUT = "HEPTA_INF0C_OK"',
            "semantic_output_receipt",
            "semantic output mismatch",
            "extract_response_text",
            "_pin_loopback_literal",
            "endpoint resolved outside loopback",
            'media_type == "application/json"',
            '"semantic_output": semantic',
            '"raw_persisted": False',
            '"raw_model_output_persisted": False',
            '"schema": "hepta.inference.inf0c.real_software_e2e.v2"',
        ),
        "minimal real-software harness",
    )

    require_markers(
        evidence,
        (
            "http.client.HTTPConnection",
            "transport_disconnect_executed",
            "backend_cancellation_acknowledged",
            "MAX_DISCONNECT_PREFIX",
            "ALLOWED_STREAM_MEDIA_TYPES",
            "raw_prefix_persisted",
            "ControlHelper",
            "HEPTA_INF0C_SERVICE_CONTROL_HELPER",
            "HEPTA_INF0C_SERVICE_CONTROL_HELPER_SHA256",
            "parse_sha256_binding",
            "resolve_control_helper",
            "verify_control_helper",
            "_secure_helper_metadata",
            "_same_canonical_path",
            "service-control helper is not executable",
            "service-control helper must not be group/other writable",
            "service-control helper parent must not be group/other writable",
            "service-control helper path must be absolute",
            "helper_revalidated_before_and_after",
            "subprocess.run",
            "stdout=subprocess.DEVNULL",
            "stderr=subprocess.DEVNULL",
            "shell=False",
            "sanitized_helper_environment",
            "controlled_restart_executed",
            "unavailable_observed",
            "raw_helper_output_persisted",
            "HEPTA_INF0C_TEST_SECRET",
            "semantic mismatch fixture",
            "helper replacement revalidation",
            "loopback_addresses_pinned_to_literals",
            "semantic_output_verified",
        ),
        "evidence harness",
    )
    for banned in (
        "shell=True",
        "capture_output=True",
        "stdout=subprocess.PIPE",
        "stderr=subprocess.PIPE",
        '"raw_model_output_persisted": True',
        '"backend_cancellation_acknowledged": True',
        '"raw_prefix_persisted": True',
        '"raw_helper_output_persisted": True',
    ):
        require(banned not in evidence, f"evidence harness contains banned path: {banned}")

    require_markers(
        self_test,
        (
            "ThreadingHTTPServer",
            "disconnect_stream",
            "prefix_byte_length",
            "media_type",
            "application/octet-stream",
            "backend_cancellation_acknowledged",
            "raw_prefix_persisted",
            "localhost was not pinned to a loopback IP literal",
            "PASS_HEPTA_INFERENCE_INF0C_EVIDENCE_V2_LOOPBACK_SELF_TEST",
        ),
        "loopback self-test",
    )
    require_markers(
        contract,
        (
            "backend_cancellation_acknowledged=false",
            "<helper> stop  ollama",
            "<helper> start lmstudio",
            "SOURCE_PRESENT_NOT_RUN",
            "INF-1=NOT_STARTED",
            "HEPTA_INF0C_OK",
            "revalidated immediately before and after",
            "application/json",
            "text/event-stream",
        ),
        "evidence contract",
    )
    require_markers(
        workflow,
        (
            "ref: ${{ github.event.pull_request.head.sha || github.sha }}",
            "fetch-depth: 2",
            "python3 scripts/hepta-inference-inf0c-evidence-v2.py --self-test",
            "python3 scripts/hepta-inference-inf0c-evidence-v2-loopback-selftest.py",
            "python3 scripts/hepta-inference-inf0c-evidence-v2-source-gate.py",
            "--run-controlled-restart",
            "HEPTA_INF0C_SERVICE_CONTROL_HELPER",
            "HEPTA_INF0C_SERVICE_CONTROL_HELPER_SHA256",
        ),
        "evidence workflow",
    )

    require_closed_authority(status, "status")
    require_closed_authority(matrix, "matrix")
    implemented = status.get("implemented")
    require(isinstance(implemented, dict), "status.implemented is missing")
    for flag in (
        "real_software_e2e_harness",
        "real_e2e_loopback_literal_pinning",
        "real_e2e_semantic_output_verification",
        "real_e2e_response_media_type_fence",
        "real_e2e_transport_disconnect_harness",
        "real_e2e_disconnect_media_type_fence",
        "real_e2e_controlled_restart_harness",
        "real_e2e_trusted_control_helper",
        "real_e2e_control_helper_secure_mode",
        "real_e2e_control_helper_executable_fence",
        "real_e2e_control_helper_per_invocation_revalidation",
        "real_e2e_control_helper_fixed_argv_matrix_self_test",
        "real_e2e_control_helper_secret_environment_fence",
    ):
        require(implemented.get(flag) is True, f"implemented flag false: {flag}")
    for flag in ("hepta_inferd", "native_worker", "real_model_e2e", "hardware_receipt"):
        require(implemented.get(flag) is False, f"status claims {flag}")

    stages = {
        item.get("id"): item
        for item in matrix.get("stages", [])
        if isinstance(item, dict)
    }
    inf0c = stages.get("INF-0C", {})
    for flag in (
        "transport_disconnect_harness_source_complete",
        "controlled_restart_harness_source_complete",
        "trusted_control_helper_source_complete",
        "loopback_literal_pinning_source_complete",
        "semantic_output_verification_source_complete",
        "response_media_type_fence_source_complete",
        "control_helper_per_invocation_revalidation_source_complete",
        "control_helper_secure_mode_source_complete",
        "control_helper_executable_fence_source_complete",
    ):
        require(inf0c.get(flag) is True, f"INF-0C source flag false: {flag}")
    for flag in (
        "real_software_e2e_executed",
        "semantic_output_verified_on_real_models",
        "transport_disconnect_executed",
        "controlled_restart_executed",
        "backend_cancellation_acknowledged",
    ):
        require(inf0c.get(flag) is False, f"INF-0C execution claimed early: {flag}")
    require(stages.get("INF-1", {}).get("status") == "NOT_STARTED", "INF-1 activated early")
    require(status.get("qualified") is False, "status qualified early")
    require(matrix.get("overall_status") == "SOURCE_PRESENT_NOT_RUN", "matrix status drift")

    print(PASS)
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except GateError as error:
        print(
            f"FAIL_HEPTA_INFERENCE_INF0C_EVIDENCE_V2_SOURCE_GATE: {error}",
            file=sys.stderr,
        )
        raise SystemExit(1) from error

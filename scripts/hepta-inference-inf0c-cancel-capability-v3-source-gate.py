#!/usr/bin/env python3
"""Source-only gate for the stacked INF-0C cancellation capability tranche."""

from __future__ import annotations

import json
import subprocess
import sys
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[1]
DOCS = ROOT / "docs/hepta-vnext/inference"
STATUS = DOCS / "HEPTA_INFERENCE_IMPLEMENTATION_STATUS_V1.json"
MATRIX = DOCS / "HEPTA_INFERENCE_STAGE_MATRIX_V2.json"
RECEIPT = DOCS / "HEPTA_INFERENCE_INF0C_SOURCE_RECEIPT_2026-08-28.json"
CONTRACT = DOCS / "HEPTA_INFERENCE_INF0C_CANCEL_CAPABILITY_V3.md"
PROBE = ROOT / "scripts/hepta-inference-inf0c-cancel-capability-v3.py"
SELF_TEST = ROOT / "scripts/hepta-inference-inf0c-cancel-capability-v3-selftest.py"
WORKFLOW = ROOT / ".github/workflows/hepta-inference-inf0c-cancel-capability-v3.yml"
PLAN_BLOB = "4381207acce1bf6371c248dc3280fff1f2ae59ce"
PARENT_RECEIPT = "9d5a592d50e333fd5db3bf73f5ab3fc9fe4d8988"
STACKED_BRANCH = "codex/hepta-inference-inf0c-cancel-capability-v3-20260828"
PASS = "PASS_HEPTA_INFERENCE_INF0C_CANCEL_CAPABILITY_V3_SOURCE_ONLY"
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


def git(*args: str, check: bool = True) -> str:
    completed = subprocess.run(
        ["git", *args], cwd=ROOT, capture_output=True, text=True, check=False
    )
    if check and completed.returncode:
        raise GateError(f"git {' '.join(args)} failed: {completed.stderr.strip()}")
    return completed.stdout.strip() if completed.returncode == 0 else ""


def candidate_head() -> str:
    parents = git("rev-list", "--parents", "-n", "1", "HEAD").split()
    require(len(parents) in (2, 3), "unexpected checkout parent shape")
    return git("rev-parse", "HEAD^2") if len(parents) == 3 else git("rev-parse", "HEAD")


def markers(source: str, expected: tuple[str, ...], label: str) -> None:
    for marker in expected:
        require(marker in source, f"{label} missing marker: {marker}")


def closed_authority(value: dict[str, Any], label: str) -> None:
    authority = value.get("authority")
    require(isinstance(authority, dict), f"{label}.authority is missing")
    require(authority.get("qualification_only") is True, f"{label} is not qualification-only")
    for field in FALSE_AUTHORITY:
        require(authority.get(field) is False, f"{label}.{field} must be false")


def main() -> int:
    candidate = candidate_head()
    source = git("rev-parse", f"{candidate}^")
    source_tree = git("show", "-s", "--format=%T", source)
    source_parent = git("rev-parse", f"{source}^")
    require(source_parent == PARENT_RECEIPT, "source is not stacked on the frozen evidence-v2 receipt")

    status = object_json(STATUS)
    matrix = object_json(MATRIX)
    receipt = object_json(RECEIPT)
    for label, value in (("status", status), ("matrix", matrix), ("receipt", receipt)):
        require(value.get("plan_git_blob_sha1") == PLAN_BLOB, f"{label} plan drift")
        closed_authority(value, label)
    require(status.get("status") == "SOURCE_PRESENT_NOT_RUN", "status drift")
    require(status.get("qualified") is False, "status qualified early")
    require(status.get("stacked_cancel_branch") == STACKED_BRANCH, "cancel branch drift")
    require(matrix.get("current_stage") == "INF-0C", "stage drift")
    require(matrix.get("overall_status") == "SOURCE_PRESENT_NOT_RUN", "matrix status drift")
    require(matrix.get("stacked_cancel_branch") == STACKED_BRANCH, "matrix cancel branch drift")

    implemented = status.get("implemented", {})
    for flag in (
        "real_e2e_explicit_cancel_capability_probe",
        "real_e2e_background_cancel_endpoint_probe",
        "real_e2e_cancel_response_id_fence",
        "real_e2e_cancel_terminal_poll",
        "real_e2e_cancel_digest_only_receipt",
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
    require(inf0c.get("cancel_capability_probe_source_complete") is True, "cancel probe source incomplete")
    require(inf0c.get("provider_cancel_capability_probe_executed") is False, "cancel probe execution claimed early")
    require(inf0c.get("provider_cancel_capability_classified") is False, "cancel capability classified early")
    require(inf0c.get("provider_explicit_cancel_acknowledged") is False, "provider cancel acknowledged early")
    require(inf0c.get("backend_cancellation_acknowledged") is False, "backend cancellation acknowledged early")
    require(stages.get("INF-1", {}).get("status") == "NOT_STARTED", "INF-1 activated early")

    probe = text(PROBE)
    markers(
        probe,
        (
            '"background": True',
            '"store": True',
            'f"{responses_url}/{response_id}/cancel"',
            "CANCELLED_STATUSES",
            "validate_response_id",
            "terminal_retrieve_attempts",
            "provider_cancel_acknowledged",
            "transport_disconnect_used",
            "raw_response_id_persisted",
            "BASE.LOOPBACK_OPENER",
            "MAX_HTTP_BODY",
        ),
        "cancel probe",
    )
    for banned in (
        "requests.",
        "shell=True",
        '"provider_cancel_acknowledged": True,\n                "transport_disconnect_used": True',
        '"raw_response_id_persisted": True',
        '"raw_model_output_persisted": True',
    ):
        require(banned not in probe, f"cancel probe contains banned path: {banned}")

    self_test = text(SELF_TEST)
    markers(
        self_test,
        (
            "ThreadingHTTPServer",
            "unsupported_create",
            "unsupported_cancel",
            "wrong_cancel_id",
            "terminal_completed",
            "PASS_HEPTA_INFERENCE_INF0C_CANCEL_CAPABILITY_V3_SELF_TEST",
        ),
        "cancel self-test",
    )
    contract = text(CONTRACT)
    markers(
        contract,
        (
            "explicit_background_cancel_acknowledged",
            "explicit_cancel_unsupported",
            "transport_disconnect_used=false",
            "backend_cancellation_acknowledged=false",
            "INF-1=NOT_STARTED",
        ),
        "cancel contract",
    )
    workflow = text(WORKFLOW)
    markers(
        workflow,
        (
            "github.event.pull_request.head.sha || github.sha",
            "python3 scripts/hepta-inference-inf0c-cancel-capability-v3-selftest.py",
            "python3 scripts/hepta-inference-inf0c-cancel-capability-v3-source-gate.py",
            "python3 scripts/hepta-inference-inf0c-evidence-v2-source-gate.py",
            "python3 scripts/hepta-inference-inf0-source-gate.py",
            "--execute",
            "if-no-files-found: error",
        ),
        "cancel workflow",
    )

    require(receipt.get("source_candidate_commit") == source, "receipt parent mismatch")
    require(receipt.get("source_candidate_tree") == source_tree, "receipt tree mismatch")
    require(receipt.get("parent_receipt_head") == PARENT_RECEIPT, "receipt parent binding drift")
    require(receipt.get("claim") == "SOURCE_PRESENT_NOT_RUN", "receipt claim drift")
    require(receipt.get("qualified") is False, "receipt qualified early")
    changed = {
        line
        for line in git("diff", "--name-only", source, candidate).splitlines()
        if line
    }
    require(changed == {str(RECEIPT.relative_to(ROOT))}, "receipt commit must modify only receipt")
    print(PASS)
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except GateError as error:
        print(f"FAIL_HEPTA_INFERENCE_INF0C_CANCEL_CAPABILITY_V3_SOURCE_GATE: {error}", file=sys.stderr)
        raise SystemExit(1) from error

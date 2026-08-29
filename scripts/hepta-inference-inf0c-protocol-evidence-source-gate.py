#!/usr/bin/env python3
"""Fail-closed source gate for INF-0C protocol evidence and all descendants."""
from __future__ import annotations

import json
import subprocess
import sys
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[1]
DOCS = ROOT / "docs/hepta-vnext/inference"
PROTOCOL = ROOT / "scripts/hepta-inference-inf0c-protocol-evidence.py"
SELF_TEST = ROOT / "scripts/hepta-inference-inf0c-protocol-evidence-selftest.py"
CONTRACT = DOCS / "HEPTA_INFERENCE_INF0C_PROTOCOL_EVIDENCE.md"
STATUS = DOCS / "HEPTA_INFERENCE_IMPLEMENTATION_STATUS_V1.json"
MATRIX = DOCS / "HEPTA_INFERENCE_STAGE_MATRIX_V2.json"
RECEIPT = DOCS / "HEPTA_INFERENCE_INF0C_SOURCE_RECEIPT_2026-08-28.json"
WORKFLOW = ROOT / ".github/workflows/hepta-inference-inf0c-protocol-evidence.yml"
STACK_BASE = "0550d2936373d310ecd1ec140910e19cac83526d"
RECEIPT_COMMIT = "93bdd3245c2f3d0685ceae8e2ce1267c40a63685"
PROTOCOL_BRANCH = "codex/hepta-inference-inf0c-protocol-evidence-20260828"
PLAN_BLOB = "4381207acce1bf6371c248dc3280fff1f2ae59ce"
PASS = "PASS_HEPTA_INFERENCE_INF0C_PROTOCOL_SOURCE_ONLY"
SOURCE_FILES = {
    ".github/workflows/hepta-inference-inf0c-protocol-evidence.yml",
    "docs/hepta-vnext/inference/HEPTA_INFERENCE_IMPLEMENTATION_STATUS_V1.json",
    "docs/hepta-vnext/inference/HEPTA_INFERENCE_INF0C_PROTOCOL_EVIDENCE.md",
    "docs/hepta-vnext/inference/HEPTA_INFERENCE_STAGE_MATRIX_V2.json",
    "scripts/hepta-inference-inf0c-protocol-evidence-selftest.py",
    "scripts/hepta-inference-inf0c-protocol-evidence-source-gate.py",
    "scripts/hepta-inference-inf0c-protocol-evidence.py",
}
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


def run_git(*args: str) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        ["git", *args],
        cwd=ROOT,
        check=False,
        capture_output=True,
        text=True,
    )


def git(*args: str) -> str:
    run = run_git(*args)
    if run.returncode:
        raise GateError(f"git {' '.join(args)} failed: {run.stderr.strip()}")
    return run.stdout.strip()


def git_ok(*args: str) -> bool:
    return run_git(*args).returncode == 0


def markers(source: str, values: tuple[str, ...], label: str) -> None:
    for value in values:
        require(value in source, f"{label} missing marker: {value}")


def closed(value: dict[str, Any], label: str) -> None:
    authority = value.get("authority")
    require(isinstance(authority, dict), f"{label}.authority missing")
    require(authority.get("qualification_only") is True, f"{label} not qualification-only")
    for field in FALSE_AUTHORITY:
        require(authority.get(field) is False, f"{label}.{field} must be false")


def verify_immutable_historical_receipt(receipt: dict[str, Any]) -> None:
    require(
        git_ok("cat-file", "-e", f"{RECEIPT_COMMIT}^{{commit}}"),
        "protocol receipt commit unavailable; workflow must fetch full history",
    )
    require(
        git_ok("merge-base", "--is-ancestor", RECEIPT_COMMIT, "HEAD"),
        "current head is not a descendant of the qualified protocol receipt",
    )

    source = git("rev-parse", f"{RECEIPT_COMMIT}^")
    require(source == receipt.get("source_candidate_commit"), "receipt source mismatch")
    source_tree = git("show", "-s", "--format=%T", source)
    require(source_tree == receipt.get("source_candidate_tree"), "receipt tree mismatch")
    stack_base = git("rev-parse", f"{source}^")
    require(stack_base == STACK_BASE, "protocol source parent drift")
    require(
        receipt.get("parent_stack_receipt_commit") == STACK_BASE,
        "receipt stack-base drift",
    )

    changed_source = {
        line for line in git("diff", "--name-only", STACK_BASE, source).splitlines() if line
    }
    require(changed_source == SOURCE_FILES, "unexpected protocol source file set")
    changed_receipt = {
        line
        for line in git("diff", "--name-only", source, RECEIPT_COMMIT).splitlines()
        if line
    }
    require(
        changed_receipt == {str(RECEIPT.relative_to(ROOT))},
        "historical protocol receipt commit must be receipt-only",
    )

    historical_receipt = git(
        "show", f"{RECEIPT_COMMIT}:{RECEIPT.relative_to(ROOT).as_posix()}"
    )
    require(
        historical_receipt == text(RECEIPT).rstrip("\n"),
        "historical protocol receipt was mutated on a descendant",
    )


def main() -> int:
    protocol, self_test, contract, workflow = map(
        text, (PROTOCOL, SELF_TEST, CONTRACT, WORKFLOW)
    )
    status, matrix, receipt = map(object_json, (STATUS, MATRIX, RECEIPT))
    for label, value in (("status", status), ("matrix", matrix), ("receipt", receipt)):
        require(value.get("plan_git_blob_sha1") == PLAN_BLOB, f"{label} plan drift")
        closed(value, label)
    require(
        status.get("protocol_development_branch") == PROTOCOL_BRANCH,
        "status branch drift",
    )
    require(matrix.get("protocol_branch") == PROTOCOL_BRANCH, "matrix branch drift")
    require(
        status.get("status") == matrix.get("overall_status") == "SOURCE_PRESENT_NOT_RUN",
        "historical status drift",
    )
    require(
        status.get("qualified") is False and receipt.get("qualified") is False,
        "historical receipt claims qualification early",
    )

    markers(
        protocol,
        (
            'TOOL_NAME = "hepta_probe"',
            'TOOL_NONCE = "HEPTA_INF0C_TOOL"',
            "TOOL_VALUE = 7",
            "tool_choice",
            "parallel_tool_calls",
            "additionalProperties",
            "extract_exact_tool_call",
            "must return exactly one function call",
            "raw_arguments_persisted",
            "canonical_inventory",
            "model inventory changed during qualification",
            "raw_model_ids_persisted",
            "implicit_download",
            "MAX_SSE_EVENT_BYTES",
            "MAX_SSE_TOTAL_BYTES",
            "MAX_SSE_EVENTS",
            "ALLOWED_SSE_EVENT_TYPES",
            "read_strict_sse",
            "SSE sequence number is not monotonic",
            "SSE stream ended without response.completed",
            "SSE event appeared after response.completed",
            "legacy [DONE] sentinel is not accepted",
            "raw_events_persisted",
            '"qualified": False',
        ),
        "protocol harness",
    )
    for banned in (
        '"qualified": True',
        '"implicit_download": True',
        '"raw_events_persisted": True',
        "shell=True",
    ):
        require(banned not in protocol, f"banned protocol path: {banned}")
    markers(
        self_test,
        (
            "ThreadingHTTPServer",
            "/malformed-json",
            "/unknown-event",
            "/truncated",
            "/duplicate-completion",
            "/event-after-completion",
            "/bad-sequence",
            "/legacy-done",
            "/oversized",
            "/bad-media",
            "/slow",
            "PASS_HEPTA_INFERENCE_INF0C_PROTOCOL_SSE_SELF_TEST",
        ),
        "protocol self-test",
    )
    markers(
        contract,
        (
            "hepta_probe",
            "HEPTA_INF0C_TOOL",
            "model inventory",
            "text/event-stream",
            "raw_arguments_persisted=false",
            "SOURCE_PRESENT_NOT_RUN",
            "INF-1=NOT_STARTED",
        ),
        "protocol contract",
    )
    markers(
        workflow,
        (
            "ref: ${{ github.event.pull_request.head.sha || github.sha }}",
            "fetch-depth: 0",
            "python3 scripts/hepta-inference-inf0c-protocol-evidence.py --self-test",
            "python3 scripts/hepta-inference-inf0c-protocol-evidence-selftest.py",
            "python3 scripts/hepta-inference-inf0c-protocol-evidence-source-gate.py",
            "python3 scripts/hepta-inference-inf0c-evidence-v2-source-gate.py",
            "python3 scripts/hepta-inference-inf0-source-gate.py",
            "--execute",
            "--ollama-model",
            "--lmstudio-model",
        ),
        "protocol workflow",
    )

    implemented = status.get("implemented", {})
    for flag in (
        "real_e2e_tool_call_harness",
        "real_e2e_exact_tool_choice",
        "real_e2e_tool_arguments_digest_only",
        "real_e2e_model_inventory_stability_fence",
        "real_e2e_no_implicit_download_inventory_fence",
        "real_e2e_strict_sse_parser",
        "real_e2e_sse_sequence_fence",
        "real_e2e_sse_terminal_fence",
        "real_e2e_sse_total_and_event_bounds",
        "real_e2e_malformed_event_fixture",
        "real_e2e_timeout_fixture",
    ):
        require(implemented.get(flag) is True, f"implemented flag false: {flag}")
    for flag in ("hepta_inferd", "native_worker", "real_model_e2e", "hardware_receipt"):
        require(implemented.get(flag) is False, f"historical status claims {flag}")
    stages = {
        item.get("id"): item
        for item in matrix.get("stages", [])
        if isinstance(item, dict)
    }
    inf0c = stages.get("INF-0C", {})
    for flag in (
        "tool_call_harness_source_complete",
        "model_inventory_stability_source_complete",
        "strict_sse_parser_source_complete",
        "malformed_event_fixture_source_complete",
        "timeout_fixture_source_complete",
        "no_implicit_download_inventory_fence_source_complete",
    ):
        require(inf0c.get(flag) is True, f"INF-0C source flag false: {flag}")
    for flag in (
        "real_tool_call_executed",
        "real_model_inventory_stability_verified",
        "real_strict_sse_executed",
        "backend_cancellation_acknowledged",
        "qualified",
    ):
        require(inf0c.get(flag) is False, f"historical execution claimed early: {flag}")
    require(
        stages.get("INF-1", {}).get("status") == "NOT_STARTED",
        "historical INF-1 status was mutated",
    )

    verify_immutable_historical_receipt(receipt)
    require(receipt.get("claim") == "SOURCE_PRESENT_NOT_RUN", "receipt claim drift")
    print(PASS)
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except GateError as error:
        print(
            f"FAIL_HEPTA_INFERENCE_INF0C_PROTOCOL_SOURCE_GATE: {error}",
            file=sys.stderr,
        )
        raise SystemExit(1) from error

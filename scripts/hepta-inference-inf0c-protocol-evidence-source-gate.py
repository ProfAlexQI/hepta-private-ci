#!/usr/bin/env python3
"""Descendant-safe source gate for the historical INF-0C protocol harness."""

from __future__ import annotations

import pathlib
import subprocess
import sys

ROOT = pathlib.Path(__file__).resolve().parents[1]
PROTOCOL = ROOT / "scripts/hepta-inference-inf0c-protocol-evidence.py"
SELF_TEST = ROOT / "scripts/hepta-inference-inf0c-protocol-evidence-selftest.py"
CONTRACT = ROOT / "docs/hepta-vnext/inference/HEPTA_INFERENCE_INF0C_PROTOCOL_EVIDENCE.md"
WORKFLOW = ROOT / ".github/workflows/hepta-inference-inf0c-protocol-evidence.yml"
HISTORICAL_GATE = ROOT / "scripts/hepta-inference-inf0c-historical-receipt-gate.py"
PASS = "PASS_HEPTA_INFERENCE_INF0C_PROTOCOL_SOURCE_ONLY"


class GateError(RuntimeError):
    pass


def require(condition: bool, message: str) -> None:
    if not condition:
        raise GateError(message)


def text(path: pathlib.Path) -> str:
    require(path.is_file(), f"missing {path.relative_to(ROOT)}")
    return path.read_text(encoding="utf-8")


def require_markers(source: str, values: tuple[str, ...], label: str) -> None:
    for value in values:
        require(value in source, f"{label} missing marker: {value}")


def require_historical_receipt() -> None:
    completed = subprocess.run(
        [sys.executable, str(HISTORICAL_GATE)],
        cwd=ROOT,
        check=False,
        capture_output=True,
        text=True,
    )
    if completed.returncode != 0:
        detail = completed.stderr.strip() or completed.stdout.strip()
        raise GateError(f"immutable historical receipt gate failed: {detail}")


def main() -> int:
    protocol = text(PROTOCOL)
    self_test = text(SELF_TEST)
    contract = text(CONTRACT)
    workflow = text(WORKFLOW)
    text(HISTORICAL_GATE)

    require_historical_receipt()

    require_markers(
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

    require_markers(
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
    require_markers(
        contract,
        (
            "hepta_probe",
            "HEPTA_INF0C_TOOL",
            "model inventory",
            "text/event-stream",
            "raw_arguments_persisted=false",
            "SOURCE_PRESENT_NOT_RUN",
        ),
        "protocol contract",
    )
    require_markers(
        workflow,
        (
            "ref: ${{ github.event.pull_request.head.sha || github.sha }}",
            "fetch-depth: 0",
            "python3 scripts/hepta-inference-inf0c-protocol-evidence.py --self-test",
            "python3 scripts/hepta-inference-inf0c-protocol-evidence-selftest.py",
            "python3 scripts/hepta-inference-inf0c-protocol-evidence-source-gate.py",
            "python3 scripts/hepta-inference-inf0c-historical-receipt-gate.py",
            "runs-on: [self-hosted, hepta-inference-e2e]",
            "--execute",
            "--ollama-model",
            "--lmstudio-model",
            "real_provider_executed=false",
            "backend_cancellation_acknowledged=false",
            "operator_accepted=false",
            "released=false",
        ),
        "protocol workflow",
    )
    for stale in (
        "python3 scripts/hepta-inference-inf0c-evidence-v2-source-gate.py",
        "python3 scripts/hepta-inference-inf0-source-gate.py",
    ):
        require(stale not in workflow, f"mutable historical gate replayed: {stale}")

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

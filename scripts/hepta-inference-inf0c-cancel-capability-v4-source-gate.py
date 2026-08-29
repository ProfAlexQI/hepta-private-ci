#!/usr/bin/env python3
from __future__ import annotations

import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
PROBE = ROOT / "scripts/hepta-inference-inf0c-cancel-capability-v4.py"
SELFTEST = ROOT / "scripts/hepta-inference-inf0c-cancel-capability-v4-selftest.py"
STATUS = ROOT / "docs/hepta-vnext/inference/HEPTA_INFERENCE_INF0C_CANCEL_CAPABILITY_V4_STATUS_2026-08-29.json"


def require(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(f"FAIL_HEPTA_INFERENCE_INF0C_CANCEL_CAPABILITY_V4_SOURCE_GATE: {message}")


def main() -> None:
    probe = PROBE.read_text(encoding="utf-8")
    selftest = SELFTEST.read_text(encoding="utf-8")
    status = json.loads(STATUS.read_text(encoding="utf-8"))
    for marker in (
        "UNSUPPORTED_MEDIA_TYPES",
        "status in V3.UNSUPPORTED_HTTP_STATUSES",
        "unexpected unsupported-response media type",
        "content_type == \"application/json\"",
        "V3.request_bounded_json = request_bounded_json",
        "transport_disconnect_used",
    ):
        require(marker in probe, f"missing probe marker: {marker}")
    for marker in (
        "plain404",
        "html405",
        "empty501",
        "problem422",
        "binary404",
        "plain200",
        "PASS_HEPTA_INFERENCE_INF0C_CANCEL_CAPABILITY_V4_HTTP_CLASSIFICATION_SELF_TEST",
    ):
        require(marker in selftest, f"missing self-test marker: {marker}")
    for forbidden in (
        "shell=True",
        "requests.",
        "transport_disconnect_used = True",
        "provider_cancel_acknowledged = True",
        "application/octet-stream\"}",
    ):
        require(forbidden not in probe, f"forbidden probe marker: {forbidden}")
    require(status["status"] == "SOURCE_PRESENT_NOT_RUN", "status drift")
    require(status["qualified"] is False, "qualified early")
    require(status["provider_cancel_capability_classified"] is False, "classified early")
    require(status["backend_cancellation_acknowledged"] is False, "acknowledged early")
    require(status["controlled_worker_termination_required"] is True, "compensating policy missing")
    require(all(value is False for value in status["authority"].values()), "authority opened")
    print("PASS_HEPTA_INFERENCE_INF0C_CANCEL_CAPABILITY_V4_SOURCE_ONLY")


if __name__ == "__main__":
    main()

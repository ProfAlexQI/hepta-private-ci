#!/usr/bin/env python3
"""Fail-closed source gate for the INF-2A owner-local shadow client."""
from __future__ import annotations
import json
import os
import pathlib
import sys

ROOT = pathlib.Path(__file__).resolve().parents[1]
CLIENT = ROOT / "codex-rs/hepta-infer-client/src/lib.rs"
TESTS = ROOT / "codex-rs/hepta-infer-client/src/tests.rs"
CARGO = ROOT / "codex-rs/hepta-infer-client/Cargo.toml"
WORKSPACE = ROOT / "codex-rs/Cargo.toml"
POLICY = ROOT / "docs/hepta-vnext/inference/HEPTA_INFERENCE_INF0C_RUNTIME_CLOSURE_POLICY_V1.json"
RECEIPT = ROOT / "docs/hepta-vnext/inference/HEPTA_INFERENCE_INF0C_RUNTIME_CLOSURE_RECEIPT_2026-08-29.json"
PERF = ROOT / "docs/hepta-vnext/inference/HEPTA_INFERENCE_INF4_PERFORMANCE_CLASSIFICATION_2026-08-29.json"
PASS = "PASS_HEPTA_INFERENCE_INF2A_SHADOW_CLIENT_SOURCE_ONLY"

class GateError(RuntimeError):
    pass

def require(value: bool, message: str) -> None:
    if not value:
        raise GateError(message)

def text(path: pathlib.Path) -> str:
    require(path.is_file(), f"missing {path.relative_to(ROOT)}")
    return path.read_text(encoding="utf-8")

def obj(path: pathlib.Path) -> dict:
    value = json.loads(text(path))
    require(isinstance(value, dict), f"{path.name} must contain an object")
    return value

def main() -> int:
    client = text(CLIENT)
    tests = text(TESTS)
    cargo = text(CARGO)
    workspace = text(WORKSPACE)
    policy = obj(POLICY)
    receipt = obj(RECEIPT)
    perf = obj(PERF)

    for marker in (
        "ShadowCompareOnly",
        "CapabilityUnsupported",
        "CapabilityKnownGap",
        "ModelTupleNotRouted",
        "REJECT_BEFORE_BACKEND_DISPATCH",
    ):
        # The dispatch policy literal lives in the immutable evidence documents;
        # all executable client markers must still be present in source/tests.
        source = client + tests + json.dumps(policy, sort_keys=True) + json.dumps(receipt, sort_keys=True)
        require(marker in source, f"missing fail-closed marker: {marker}")

    for marker in (
        "stream.ensure_current_user_peer()?",
        "timeout(self.config.connect_timeout",
        "timeout(self.config.exchange_timeout",
        "MAX_FRAME_BYTES",
        "INF_CLIENT_CAPABILITY_UNSUPPORTED",
        "INF_CLIENT_CAPABILITY_KNOWN_GAP_NOT_ROUTED",
        "INF_CLIENT_MODEL_TUPLE_NOT_ROUTED",
    ):
        require(marker in client, f"client missing marker: {marker}")

    for banned in (
        "reqwest",
        "http://",
        "https://",
        "TcpStream",
        "raw_prompt",
        "automatic_model_install",
        "RemoteFallback",
        "Production",
        "shell=True",
    ):
        require(banned not in client, f"client contains banned route or authority: {banned}")

    require('name = "codex-hepta-infer-client"' in cargo, "client package name drift")
    require('codex-hepta-infer-core = { workspace = true }' in cargo, "core dependency missing")
    require('codex-hepta-inferd = { workspace = true }' in cargo, "daemon test dependency missing")
    require('"hepta-infer-client",' in workspace, "client is not a workspace member")
    require('codex-hepta-infer-client = { path = "hepta-infer-client" }' in workspace, "client workspace dependency missing")

    require(policy.get("terminal_class") == "UNSUPPORTED_FAIL_CLOSED", "runtime policy class drift")
    require(policy.get("dispatch_policy") == "REJECT_BEFORE_BACKEND_DISPATCH", "runtime dispatch policy drift")
    require(receipt.get("transport_disconnect") == "QUALIFIED", "disconnect evidence missing")
    require(receipt.get("controlled_restart") == "QUALIFIED", "restart evidence missing")
    require(receipt.get("backend_cancellation_acknowledged") is False, "provider cancel acknowledgement overclaimed")
    require(receipt.get("technical_gap_closed_by_capability_classification") is True, "cancel capability closure missing")
    require(perf.get("classification") == "KNOWN_GAP_NOT_ROUTED", "performance class drift")
    router = perf.get("router_disposition", {})
    require(router.get("default_router_eligible") is False, "unmeasured tuple entered default router")
    require(router.get("experimental_router_eligible") is False, "unmeasured tuple entered experimental router")

    for source in (client, tests, cargo, json.dumps(policy), json.dumps(receipt), json.dumps(perf)):
        require("operator_accepted\": true" not in source, "operator acceptance claimed early")
        require("promoted\": true" not in source, "promotion claimed early")

    print(PASS)
    return 0

if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except (GateError, json.JSONDecodeError) as error:
        print(f"FAIL_HEPTA_INFERENCE_INF2A_SHADOW_CLIENT_SOURCE_GATE: {error}", file=sys.stderr)
        raise SystemExit(1) from error

#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
PROFILE = ROOT / "docs/hepta-vnext/inference/HEPTA_INFERENCE_CAPABILITY_PROFILE_V1.json"
LEDGER = ROOT / "docs/hepta-vnext/inference/HEPTA_INFERENCE_GAP_CLOSURE_LEDGER_V1.json"
ALLOWED_CAPABILITY = {"QUALIFIED", "UNSUPPORTED_FAIL_CLOSED", "PENDING"}
ALLOWED_GAP = {"CLOSED_PASS", "CLOSED_UNSUPPORTED_FAIL_CLOSED", "PENDING", "NOT_STARTED"}


def require(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(message)


def validate(profile: dict, ledger: dict) -> None:
    require(profile.get("schema") == "hepta.inference.capability-profile.v1", "capability schema mismatch")
    require(ledger.get("schema") == "hepta.inference.gap-closure-ledger.v1", "ledger schema mismatch")
    require(profile.get("repository") == "ProfHepta/hepta-private-ci", "non-canonical repository binding")
    require(ledger.get("repository") == "ProfHepta/hepta-private-ci", "non-canonical ledger binding")
    require(profile.get("stage") == "INF-0C" and ledger.get("stage") == "INF-0C", "stage mismatch")

    policy = profile.get("routing_policy", {})
    require(policy.get("default") == "FAIL_CLOSED", "routing must default fail closed")
    require(policy.get("implicit_model_install") is False, "implicit model install must remain disabled")
    require(policy.get("remote_fallback") is False, "remote fallback must remain disabled")

    tuples = profile.get("tuples")
    require(isinstance(tuples, list) and len(tuples) == 2, "exact two local qualification tuples required")
    for item in tuples:
        require(item.get("production_eligible") is False, "source-only tuple cannot be production eligible")
        for name in ("semantic_text", "native_tool_call", "strict_sse", "transport_disconnect", "backend_cancel_ack", "controlled_restart"):
            capability = item.get(name, {})
            status = capability.get("status")
            require(status in ALLOWED_CAPABILITY, f"invalid {name} status for {item.get('id')}")
            if status == "QUALIFIED":
                require(isinstance(capability.get("evidence_run"), int), f"qualified {name} lacks executable evidence")
            if status == "UNSUPPORTED_FAIL_CLOSED":
                require(bool(capability.get("fail_closed_action")), f"unsupported {name} lacks fail-closed action")
        if item["native_tool_call"]["status"] != "QUALIFIED":
            require(item["native_tool_call"].get("fail_closed_action") == "REJECT_TOOL_REQUEST", "unqualified tool calls must be rejected")
        if item["backend_cancel_ack"]["status"] != "QUALIFIED":
            require("GENERATION" in item["backend_cancel_ack"].get("fail_closed_action", ""), "cancel fallback must advance generation")

    authority = profile.get("authority", {})
    require(authority.get("qualification_only") is True, "qualification-only posture missing")
    for key, value in authority.items():
        if key != "qualification_only":
            require(value is False, f"authority opened: {key}")

    gaps = ledger.get("gaps")
    require(isinstance(gaps, list) and gaps, "gap ledger empty")
    ids = set()
    for gap in gaps:
        gap_id = gap.get("id")
        require(isinstance(gap_id, str) and gap_id and gap_id not in ids, "duplicate or empty gap id")
        ids.add(gap_id)
        status = gap.get("status")
        require(status in ALLOWED_GAP, f"invalid gap status: {gap_id}")
        if status == "CLOSED_PASS":
            require(isinstance(gap.get("evidence_run"), int), f"closed PASS gap lacks run: {gap_id}")
        if status == "CLOSED_UNSUPPORTED_FAIL_CLOSED":
            require(isinstance(gap.get("evidence_run"), int) and bool(gap.get("containment")), f"unsupported closure lacks evidence/containment: {gap_id}")

    overall = ledger.get("overall", {})
    all_closed = all(gap["status"].startswith("CLOSED_") for gap in gaps)
    require(overall.get("all_gaps_closed") is all_closed, "all_gaps_closed does not match ledger")
    if not all_closed:
        require(overall.get("qualified") is False, "incomplete ledger cannot be qualified")
        require(overall.get("inf1_active") is False, "incomplete ledger cannot activate INF-1")
        require(overall.get("operator_accepted") is False, "incomplete ledger cannot be operator accepted")
        require(overall.get("promoted") is False, "incomplete ledger cannot be promoted")


def self_test() -> None:
    profile = json.loads(PROFILE.read_text(encoding="utf-8"))
    ledger = json.loads(LEDGER.read_text(encoding="utf-8"))
    validate(profile, ledger)
    bad = json.loads(json.dumps(profile))
    bad["tuples"][0]["native_tool_call"] = {"status": "UNSUPPORTED_FAIL_CLOSED", "evidence_run": 1}
    try:
        validate(bad, ledger)
    except SystemExit:
        pass
    else:
        raise SystemExit("self-test failed: unsupported capability without containment accepted")
    print("PASS_HEPTA_INFERENCE_CAPABILITY_PROFILE_SELF_TEST")


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--self-test", action="store_true")
    args = parser.parse_args()
    if args.self_test:
        self_test()
        return
    validate(json.loads(PROFILE.read_text(encoding="utf-8")), json.loads(LEDGER.read_text(encoding="utf-8")))
    print("PASS_HEPTA_INFERENCE_CAPABILITY_PROFILE_GATE")


if __name__ == "__main__":
    main()

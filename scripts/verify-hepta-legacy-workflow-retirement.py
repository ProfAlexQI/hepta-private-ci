#!/usr/bin/env python3
"""Verify that superseded Hepta workflows cannot auto-trigger or be reused."""

from __future__ import annotations

import json
import pathlib
import sys
from typing import Any

ROOT = pathlib.Path(__file__).resolve().parents[1]
POLICY = ROOT / "docs/hepta-vnext/browser/CI_LEGACY_WORKFLOW_RETIREMENT_V1.json"
BLOCKING = ROOT / ".github/workflows/blocking-ci.yml"
BROWSER = ROOT / ".github/workflows/hepta-browser-next-required-v9.yml"
VNEXT = ROOT / ".github/workflows/hepta-vnext-qualification.yml"

EXPECTED_RETIRED = [
    ".github/workflows/hepta-browser-c0-c3.yml",
    ".github/workflows/hepta-runner-allocation-canary.yml",
    ".github/workflows/hepta-servo-independent-source-contract.yml",
    ".github/workflows/hepta-servo-independent-source-contract-v2.yml",
    ".github/workflows/hepta-servo-source-bundle-verify-contract.yml",
    ".github/workflows/hepta-servo-source-bundle-contract.yml",
    ".github/workflows/hepta-servo-source-contract.yml",
    ".github/workflows/hepta-servo-build-input-contract.yml",
    ".github/workflows/hepta-servo-build-input-contract-v2.yml",
    ".github/workflows/hepta-servo-worker-build-inputs-contract.yml",
    ".github/workflows/hepta-servo-toolchain-contract.yml",
    ".github/workflows/hepta-servo-worker-reproducibility-contract.yml",
    ".github/workflows/hepta-servo-worker-launch-plan-contract.yml",
    ".github/workflows/hepta-servo-worker-startup-descriptor-contract.yml",
]

EXPECTED_REUSABLE = [
    {
        "workflow": ".github/workflows/hepta-browser-c1-artifact-gate.yml",
        "caller": ".github/workflows/blocking-ci.yml",
        "allowed_triggers": ["workflow_call", "workflow_dispatch"],
        "purpose": "qualification-only artifact-bound child process gate",
    },
    {
        "workflow": ".github/workflows/hepta-servo-artifact-contract.yml",
        "caller": ".github/workflows/blocking-ci.yml",
        "allowed_triggers": ["workflow_call", "workflow_dispatch"],
        "purpose": "synthetic executable-header and artifact-binding contract",
    },
]

NEGATIVE_MARKERS = (
    "automatic_trigger=false",
    "workflow_call=false",
    "manual_provenance_only=true",
    "exact_servo_source_accepted=false",
    "worker_source_topology_accepted=false",
    "build_authorized=false",
    "servo_build_run=false",
    "servo_runtime_qualified=false",
    "runtime_authority=false",
    "production_caller=false",
    "production_writer=false",
    "effect_authority=false",
    "external_effect=false",
    "external_network_allowed=false",
    "credential_export_allowed=false",
    "operator_acceptance=false",
    "promotion=false",
    "release_qualified=false",
)


def fail(message: str) -> None:
    raise RuntimeError(message)


def canonical(value: object) -> bytes:
    return json.dumps(
        value,
        sort_keys=True,
        separators=(",", ":"),
        ensure_ascii=False,
    ).encode("utf-8")


def load_policy() -> dict[str, Any]:
    raw = POLICY.read_bytes()
    value = json.loads(raw.decode("utf-8"))
    if not isinstance(value, dict):
        fail("retirement policy must contain one object")
    if raw != canonical(value):
        fail("retirement policy is not compact canonical JSON")
    return value


def trigger_block(text: str) -> str:
    start = text.find("\non:\n")
    if start == -1:
        if text.startswith("on:\n"):
            start = -1
        else:
            fail("workflow is missing an on block")
    start += 1
    boundary = text.find("\npermissions:", start)
    if boundary == -1:
        boundary = text.find("\njobs:", start)
    if boundary == -1:
        fail("workflow is missing permissions or jobs after the on block")
    return text[start:boundary]


def verify_retired(path: pathlib.Path) -> None:
    text = path.read_text(encoding="utf-8")
    block = trigger_block(text)
    if block != "on:\n  workflow_dispatch:\n":
        fail(f"{path.relative_to(ROOT)} is not workflow_dispatch-only: {block!r}")
    for forbidden in ("pull_request:", "push:", "workflow_call:", "schedule:", "repository_dispatch:"):
        if forbidden in block:
            fail(f"{path.relative_to(ROOT)} retains forbidden trigger {forbidden}")
    if "\n    uses:" in text or "\n      - uses:" in text:
        fail(f"{path.relative_to(ROOT)} retired stub invokes an action or workflow")
    if "permissions:\n  contents: read\n" not in text:
        fail(f"{path.relative_to(ROOT)} does not keep read-only contents permission")
    for marker in NEGATIVE_MARKERS:
        if marker not in text:
            fail(f"{path.relative_to(ROOT)} is missing negative marker {marker!r}")


def verify_reusable(item: dict[str, Any]) -> None:
    path = ROOT / item["workflow"]
    text = path.read_text(encoding="utf-8")
    block = trigger_block(text)
    for required in ("workflow_call:", "workflow_dispatch:"):
        if required not in block:
            fail(f"{item['workflow']} is missing reusable trigger {required}")
    for forbidden in ("pull_request:", "push:", "schedule:", "repository_dispatch:"):
        if forbidden in block:
            fail(f"{item['workflow']} retains duplicate trigger {forbidden}")
    blocking = BLOCKING.read_text(encoding="utf-8")
    expected_use = f"uses: ./{item['workflow']}"
    if expected_use not in blocking:
        fail(f"{item['workflow']} is not called by blocking-ci")


def main() -> int:
    try:
        for path in (POLICY, BLOCKING, BROWSER, VNEXT):
            if not path.is_file():
                fail(f"missing {path.relative_to(ROOT)}")
        policy = load_policy()
        if policy.get("schema") != "hepta.ci.legacy_workflow_retirement.v1":
            fail("retirement policy schema drifted")
        if policy.get("schema_version") != 1 or policy.get("phase") != "DEVELOPMENT":
            fail("retirement policy version or phase drifted")
        if policy.get("status") != "RETIRED_AUTOMATIC_TRIGGER_SURFACE":
            fail("retirement policy status drifted")

        retired = policy.get("retired_workflows")
        if not isinstance(retired, list):
            fail("retired_workflows must be an array")
        paths = [item.get("workflow") for item in retired if isinstance(item, dict)]
        if paths != EXPECTED_RETIRED:
            fail("retired workflow set or ordering drifted")
        retired_bytes: bytes | None = None
        for item in retired:
            if not isinstance(item, dict) or item.get("mode") != "MANUAL_PROVENANCE_ONLY":
                fail("retired workflow mode drifted")
            workflow = item.get("workflow")
            if not isinstance(workflow, str):
                fail("retired workflow path is invalid")
            path = ROOT / workflow
            if not path.is_file():
                fail(f"missing retired workflow {workflow}")
            verify_retired(path)
            current_bytes = path.read_bytes()
            if retired_bytes is None:
                retired_bytes = current_bytes
            elif current_bytes != retired_bytes:
                fail("retired workflows must use one byte-identical provenance stub")

        reusable = policy.get("reusable_workflows")
        if reusable != EXPECTED_REUSABLE:
            fail("reusable workflow set or contract drifted")
        for item in EXPECTED_REUSABLE:
            verify_reusable(item)

        invariants = policy.get("invariants")
        expected_false = (
            "retired_pull_request_trigger",
            "retired_push_trigger",
            "retired_workflow_call",
            "reusable_pull_request_trigger",
            "reusable_push_trigger",
            "exact_servo_source_accepted",
            "worker_source_topology_accepted",
            "build_authorized",
            "servo_build_run",
            "servo_runtime_qualified",
        )
        if not isinstance(invariants, dict):
            fail("retirement invariants must be an object")
        for key in expected_false:
            if invariants.get(key) is not False:
                fail(f"retirement invariant {key} is not false")
        if invariants.get("canonical_required_workflows_unchanged") is not True:
            fail("retirement policy does not preserve canonical required workflows")

        authority = policy.get("authority")
        if not isinstance(authority, dict) or any(value is not False for value in authority.values()):
            fail("retirement policy attempted to enable authority")

        blocking = BLOCKING.read_text(encoding="utf-8")
        browser = BROWSER.read_text(encoding="utf-8")
        vnext = VNEXT.read_text(encoding="utf-8")
        if "python3 scripts/verify-hepta-legacy-workflow-retirement.py" not in blocking:
            fail("blocking-ci does not execute the retirement verifier")
        for label, text, required_name in (
            ("blocking-ci", blocking, "name: CI required"),
            ("Browser v9", browser, "name: Hepta Browser next required v9"),
            ("Hepta vNext", vnext, "name: Hepta vNext required"),
        ):
            block = trigger_block(text)
            if "pull_request:" not in block:
                fail(f"{label} lost its independent pull_request trigger")
            if required_name not in text:
                fail(f"{label} lost its stable required check name")
    except (OSError, UnicodeError, json.JSONDecodeError, RuntimeError) as error:
        print(f"HEPTA_LEGACY_WORKFLOW_RETIREMENT=FAIL: {error}", file=sys.stderr)
        return 1

    print(
        json.dumps(
            {
                "schema": "hepta.ci.legacy_workflow_retirement.verification.v1",
                "status": "PASS_RETIRED_AUTOMATIC_TRIGGER_SURFACE",
                "retired_workflows": len(EXPECTED_RETIRED),
                "reusable_call_only_workflows": len(EXPECTED_REUSABLE),
                "canonical_required_contexts": [
                    "CI required",
                    "Hepta Browser next required v9",
                    "Hepta vNext required",
                ],
                "exact_servo_source_accepted": False,
                "worker_source_topology_accepted": False,
                "build_authorized": False,
                "servo_runtime_qualified": False,
                "authority": "all_false",
            },
            sort_keys=True,
            separators=(",", ":"),
        )
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

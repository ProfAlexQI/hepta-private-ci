#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import subprocess
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[1]
EXPECTED_PARENT = "537394a0067d204b215db8bee3de533494535481"
EXPECTED_PARENT_TREE = "fd0f84d73504507078cecfcc1043490ca0856187"
EXPECTED_BRANCH = "codex/hepta-architecture-v5-b1b-model-invocation-20260831"

OWNED_PATHS = {
    ".github/workflows/hepta-architecture-v5-b1b-model-boundary.yml",
    "codex-rs/hepta-contracts/src/checked_model_invocation.rs",
    "codex-rs/hepta-contracts/src/lib.rs",
    "codex-rs/hepta-contracts/tests/model_verified_use_boundary.rs",
    "docs/architecture/HEPTA_CURRENT_PLAN.json",
    "docs/architecture/HEPTA_P0_7B_B1B_MODEL_INVOCATION_CONTRACT_V1.md",
    "docs/architecture/HEPTA_P0_7B_B1B_STATUS.json",
    "docs/architecture/HEPTA_V5_B1B_MODEL_CALLSITE_INVENTORY.json",
    "scripts/verify-hepta-v5-b1b-model-boundary.py",
}

REQUIRED_SOURCE_SNIPPETS = (
    "PhysicalCapabilityKind::ModelInvocation",
    "verify_physical_capability_use(",
    ".consume_at_boundary(",
    "persist_witness(&witness)",
    "self.adapter.invoke(dispatch).await",
    "wire_payload_sha256",
    "endpoint_identity_sha256",
    "routing_policy_sha256",
    "response_contract_sha256",
    "tool_contract_sha256",
    "ModelInvocationOutcome::indeterminate",
    "B1B_MODEL_BOUNDARY_RUNTIME_REGISTERED: bool = false",
    "B1B_MODEL_BOUNDARY_PRODUCTION_CALLER: bool = false",
    "B1B_MODEL_BOUNDARY_MODEL_INVOCATION: bool = false",
    "B1B_MODEL_BOUNDARY_EXTERNAL_EFFECT: bool = false",
)

FORBIDDEN_SOURCE_SNIPPETS = (
    "pub fn adapter(",
    "pub fn adapter_mut(",
    "pub fn into_parts(",
    "pub fn restore_raw",
    "pub fn from_raw",
    "impl Clone for CheckedModelInvocation",
    "derive(Clone, Debug, Eq, PartialEq)]\npub struct CheckedModelInvocation",
)

AUTHORITY_KEYS = (
    "runtime",
    "productionCaller",
    "productionWriter",
    "modelInvocation",
    "providerDispatch",
    "toolExecution",
    "networkConnect",
    "externalFilesystemMutation",
    "secretOperation",
    "matrixSend",
    "externalEffect",
    "fleetMutation",
    "operatorAcceptance",
    "promotion",
    "release",
)


def fail(message: str) -> None:
    raise SystemExit(f"FAIL_HEPTA_V5_B1B_MODEL_BOUNDARY: {message}")


def read(path: str) -> str:
    target = ROOT / path
    if not target.is_file():
        fail(f"missing required file: {path}")
    return target.read_text(encoding="utf-8")


def strict_object(pairs: list[tuple[str, Any]]) -> dict[str, Any]:
    result: dict[str, Any] = {}
    for key, value in pairs:
        if key in result:
            fail(f"duplicate JSON key: {key}")
        result[key] = value
    return result


def load_json(path: str) -> dict[str, Any]:
    try:
        value = json.loads(read(path), object_pairs_hook=strict_object)
    except json.JSONDecodeError as error:
        fail(f"invalid JSON in {path}: {error}")
    if not isinstance(value, dict):
        fail(f"top-level JSON object required: {path}")
    return value


def git(*args: str) -> str:
    completed = subprocess.run(
        ["git", *args],
        cwd=ROOT,
        check=False,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
    )
    if completed.returncode != 0:
        fail(f"git {' '.join(args)} failed: {completed.stderr.strip()}")
    return completed.stdout.strip()


def require_direct_parent() -> None:
    head = git("rev-parse", "HEAD")
    parent = git("rev-parse", "HEAD^")
    parent_tree = git("rev-parse", "HEAD^^{tree}")
    parent_count = len(git("rev-list", "--parents", "-n", "1", "HEAD").split()) - 1
    if head == EXPECTED_PARENT:
        fail("HEAD is still the B1a parent")
    if parent != EXPECTED_PARENT:
        fail(f"unexpected direct parent: {parent}")
    if parent_tree != EXPECTED_PARENT_TREE:
        fail(f"unexpected parent tree: {parent_tree}")
    if parent_count != 1:
        fail(f"B1b source commit must have exactly one parent, got {parent_count}")
    changed = set(filter(None, git("diff", "--name-only", EXPECTED_PARENT, "HEAD").splitlines()))
    if changed != OWNED_PATHS:
        missing = sorted(OWNED_PATHS - changed)
        extra = sorted(changed - OWNED_PATHS)
        fail(f"owned path drift; missing={missing}, extra={extra}")


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--require-direct-parent", action="store_true")
    args = parser.parse_args()

    source = read("codex-rs/hepta-contracts/src/checked_model_invocation.rs")
    for snippet in REQUIRED_SOURCE_SNIPPETS:
        if snippet not in source:
            fail(f"missing source invariant: {snippet}")
    for snippet in FORBIDDEN_SOURCE_SNIPPETS:
        if snippet in source:
            fail(f"raw adapter escape detected: {snippet}")

    lib_rs = read("codex-rs/hepta-contracts/src/lib.rs")
    for snippet in (
        "mod checked_model_invocation;",
        "pub use checked_model_invocation::CheckedModelInvocation;",
        "pub use checked_model_invocation::ModelInvocationAdapter;",
        "pub use checked_model_invocation::ModelInvocationIntent;",
    ):
        if snippet not in lib_rs:
            fail(f"crate-root wiring missing: {snippet}")

    tests = read("codex-rs/hepta-contracts/tests/model_verified_use_boundary.rs")
    for snippet in (
        "witness_is_committed_before_exact_adapter_call",
        "payload_drift_fails_before_claim_and_adapter",
        "witness_persistence_failure_blocks_adapter_and_consumes_claim",
        "transport_failure_is_indeterminate_without_retry",
        "claim_rejection_blocks_the_physical_adapter",
    ):
        if snippet not in tests:
            fail(f"required negative test missing: {snippet}")

    current = load_json("docs/architecture/HEPTA_CURRENT_PLAN.json")
    if current.get("repository") != "TrillionniumFoundation/hepta-private-ci":
        fail("current-plan repository identity is stale")
    if current.get("currentPackage") != "P0.7b/B1b_model_invocation_verified_use_boundary":
        fail("currentPackage does not select B1b")
    candidate = current.get("candidateBinding")
    if not isinstance(candidate, dict) or candidate.get("stackParentCommit") != EXPECTED_PARENT:
        fail("current plan is not bound to the exact B1a parent")
    authority = current.get("authority")
    if not isinstance(authority, dict):
        fail("current plan authority object is missing")
    for key in AUTHORITY_KEYS:
        if authority.get(key) is not False:
            fail(f"authority must remain false: {key}")

    status = load_json("docs/architecture/HEPTA_P0_7B_B1B_STATUS.json")
    if status.get("state") != "source_implemented_executable_qualification_pending":
        fail("B1b status must remain source implemented / qualification pending")
    if status.get("parentCommit") != EXPECTED_PARENT:
        fail("B1b status parent drift")
    status_authority = status.get("authority")
    if not isinstance(status_authority, dict) or any(value is not False for value in status_authority.values()):
        fail("B1b status contains positive authority")

    inventory = load_json("docs/architecture/HEPTA_V5_B1B_MODEL_CALLSITE_INVENTORY.json")
    if inventory.get("productCallsitesRegistered") is not False:
        fail("B1b must not register a product model callsite")
    if inventory.get("b4GlobalInventoryRequired") is not True:
        fail("B1b must retain the B4 global callsite proof")

    workflow = read(".github/workflows/hepta-architecture-v5-b1b-model-boundary.yml")
    if "contents: read" not in workflow or "contents: write" in workflow:
        fail("B1b qualification workflow must be read-only")
    for forbidden in ("git push", "update-ref", "gh pr merge", "curl -X POST"):
        if forbidden in workflow:
            fail(f"workflow mutation detected: {forbidden}")

    if args.require_direct_parent:
        require_direct_parent()

    print("PASS_HEPTA_V5_B1B_MODEL_BOUNDARY_SOURCE_ONLY")


if __name__ == "__main__":
    main()

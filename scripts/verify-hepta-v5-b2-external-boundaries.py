#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import subprocess
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[1]
EXPECTED_PARENT = "cd6823c94b3fbd1c3845a398206f526b8e4bc85e"
EXPECTED_PARENT_TREE = "fb9af24eaea3283ede58611d639ea8ab1176f2c4"
EXPECTED_BRANCH = "codex/hepta-architecture-v5-b2-external-boundaries-gpt56-20260831"

OWNED_PATHS = {
    ".github/workflows/hepta-architecture-v5-b2-external-boundaries.yml",
    "codex-rs/hepta-contracts/src/lib.rs",
    "codex-rs/hepta-contracts/src/physical_boundaries.rs",
    "codex-rs/hepta-contracts/src/physical_boundaries/external.rs",
    "codex-rs/hepta-contracts/tests/external_verified_use_boundaries.rs",
    "docs/architecture/HEPTA_CURRENT_PLAN.json",
    "docs/architecture/HEPTA_P0_7B_B2_EXTERNAL_BOUNDARIES_CONTRACT_V1.md",
    "docs/architecture/HEPTA_P0_7B_B2_STATUS.json",
    "docs/architecture/HEPTA_V5_B2_EXTERNAL_CALLSITE_INVENTORY.json",
    "scripts/verify-hepta-v5-b2-external-boundaries.py",
}

REQUIRED_SOURCE_SNIPPETS = (
    "ToolProcessIntent",
    "OutboundNetworkIntent",
    "ExternalFilesystemMutationIntent",
    "PhysicalCapabilityKind::ToolProcessSpawn",
    "PhysicalCapabilityKind::OutboundNetworkConnect",
    "PhysicalCapabilityKind::ExternalFilesystemMutation",
    "Authorized<ExternalEffectCapability>",
    "verify_physical_capability_use(",
    ".consume_at_boundary(",
    "persist_witness(&witness)",
    "self.adapter.cross(dispatch).await",
    "executable_file_sha256",
    "argv_sha256",
    "cwd_identity_sha256",
    "environment_policy_sha256",
    "sandbox_policy_sha256",
    "approval_sha256",
    "canonical_destination",
    "resolved_ip_set_sha256",
    "proxy_policy_sha256",
    "tls_policy_sha256",
    "canonical_target_sha256",
    "device_mount_identity_sha256",
    "expected_prior_state_sha256",
    "NoFollowRequired",
    "ExternalBoundaryOutcome::indeterminate",
    "B2_EXTERNAL_BOUNDARIES_TOOL_EXECUTION: bool = false",
    "B2_EXTERNAL_BOUNDARIES_NETWORK_CONNECT: bool = false",
    "B2_EXTERNAL_BOUNDARIES_FILESYSTEM_MUTATION: bool = false",
    "B2_EXTERNAL_BOUNDARIES_EXTERNAL_EFFECT: bool = false",
)

FORBIDDEN_SOURCE_SNIPPETS = (
    "pub fn adapter(",
    "pub fn adapter_mut(",
    "pub fn into_parts(",
    "pub fn restore_raw",
    "pub fn from_raw",
    "impl Clone for CheckedExternalBoundary",
    "derive(Clone, Debug, Eq, PartialEq)]\npub struct CheckedExternalBoundary",
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
    raise SystemExit(f"FAIL_HEPTA_V5_B2_EXTERNAL_BOUNDARIES: {message}")


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
        fail("HEAD is still the B1b parent")
    if parent != EXPECTED_PARENT:
        fail(f"unexpected direct parent: {parent}")
    if parent_tree != EXPECTED_PARENT_TREE:
        fail(f"unexpected parent tree: {parent_tree}")
    if parent_count != 1:
        fail(f"B2 source commit must have exactly one parent, got {parent_count}")
    changed = set(filter(None, git("diff", "--name-only", EXPECTED_PARENT, "HEAD").splitlines()))
    if changed != OWNED_PATHS:
        missing = sorted(OWNED_PATHS - changed)
        extra = sorted(changed - OWNED_PATHS)
        fail(f"owned path drift; missing={missing}, extra={extra}")


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--require-direct-parent", action="store_true")
    args = parser.parse_args()

    source = read("codex-rs/hepta-contracts/src/physical_boundaries/external.rs")
    for snippet in REQUIRED_SOURCE_SNIPPETS:
        if snippet not in source:
            fail(f"missing source invariant: {snippet}")
    for snippet in FORBIDDEN_SOURCE_SNIPPETS:
        if snippet in source:
            fail(f"raw adapter escape detected: {snippet}")

    root = read("codex-rs/hepta-contracts/src/physical_boundaries.rs")
    if "pub mod external;" not in root:
        fail("physical boundary root does not export B2")
    lib_rs = read("codex-rs/hepta-contracts/src/lib.rs")
    if "pub mod physical_boundaries;" not in lib_rs:
        fail("crate root does not expose physical boundary module")

    tests = read("codex-rs/hepta-contracts/tests/external_verified_use_boundaries.rs")
    for snippet in (
        "all_three_boundaries_claim_and_persist_before_adapter",
        "boundary_specific_facts_change_physical_payload_digests",
        "external_filesystem_requires_no_follow",
        "final_payload_drift_fails_before_claim_and_adapter",
        "claim_and_witness_failures_block_adapter",
        "post_crossing_transport_failure_is_indeterminate_without_retry",
    ):
        if snippet not in tests:
            fail(f"required B2 test missing: {snippet}")

    current = load_json("docs/architecture/HEPTA_CURRENT_PLAN.json")
    if current.get("repository") != "TrillionniumFoundation/hepta-private-ci":
        fail("current-plan repository identity is stale")
    if current.get("currentPackage") != "P0.7b/B2_tool_network_filesystem_boundaries":
        fail("currentPackage does not select B2")
    candidate = current.get("candidateBinding")
    if not isinstance(candidate, dict):
        fail("current plan candidate binding is missing")
    if candidate.get("stackParentCommit") != EXPECTED_PARENT:
        fail("current plan is not bound to the exact B1b parent")
    if candidate.get("sourceBranch") != EXPECTED_BRANCH:
        fail("current plan source branch drift")
    authority = current.get("authority")
    if not isinstance(authority, dict):
        fail("current plan authority object is missing")
    for key in AUTHORITY_KEYS:
        if authority.get(key) is not False:
            fail(f"authority must remain false: {key}")

    status = load_json("docs/architecture/HEPTA_P0_7B_B2_STATUS.json")
    if status.get("state") != "source_implemented_executable_qualification_pending":
        fail("B2 status must remain source implemented / qualification pending")
    if status.get("parentCommit") != EXPECTED_PARENT:
        fail("B2 status parent drift")
    status_authority = status.get("authority")
    if not isinstance(status_authority, dict) or any(value is not False for value in status_authority.values()):
        fail("B2 status contains positive authority")

    inventory = load_json("docs/architecture/HEPTA_V5_B2_EXTERNAL_CALLSITE_INVENTORY.json")
    if inventory.get("productCallsitesRegistered") is not False:
        fail("B2 must not register a product callsite")
    if inventory.get("b4GlobalInventoryRequired") is not True:
        fail("B2 must retain the B4 global callsite proof")
    boundary = inventory.get("checkedBoundary")
    if not isinstance(boundary, dict) or boundary.get("physicalKinds") != [
        "tool_process_spawn",
        "outbound_network_connect",
        "external_filesystem_mutation",
    ]:
        fail("B2 physical-kind inventory drift")

    workflow = read(".github/workflows/hepta-architecture-v5-b2-external-boundaries.yml")
    if "contents: read" not in workflow or "contents: write" in workflow:
        fail("B2 qualification workflow must be read-only")
    for forbidden in ("git push", "update-ref", "gh pr merge", "curl -X POST"):
        if forbidden in workflow:
            fail(f"workflow mutation detected: {forbidden}")

    if args.require_direct_parent:
        require_direct_parent()

    print("PASS_HEPTA_V5_B2_EXTERNAL_BOUNDARIES_SOURCE_ONLY")


if __name__ == "__main__":
    main()

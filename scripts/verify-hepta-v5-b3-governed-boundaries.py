#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import subprocess
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[1]
EXPECTED_PARENT = "24a2c1b733cc1d0f1288b39ffd42057dc6ade8ba"
EXPECTED_PARENT_TREE = "7ed08fb76eb8f0a30f3be926b66ff0d81fa46336"
EXPECTED_BRANCH = "codex/hepta-architecture-v5-b3-governed-boundaries-gpt56-20260831"

OWNED_PATHS = {
    ".github/workflows/hepta-architecture-v5-b3-governed-boundaries.yml",
    "codex-rs/hepta-contracts/src/physical_boundaries.rs",
    "codex-rs/hepta-contracts/src/physical_boundaries/governed.rs",
    "codex-rs/hepta-contracts/src/physical_boundaries/governed/core.rs",
    "codex-rs/hepta-contracts/src/physical_boundaries/governed/fleet.rs",
    "codex-rs/hepta-contracts/src/physical_boundaries/governed/matrix.rs",
    "codex-rs/hepta-contracts/src/physical_boundaries/governed/operator.rs",
    "codex-rs/hepta-contracts/src/physical_boundaries/governed/release.rs",
    "codex-rs/hepta-contracts/src/physical_boundaries/governed/secret.rs",
    "codex-rs/hepta-contracts/tests/governed_verified_use_boundaries.rs",
    "docs/architecture/HEPTA_CURRENT_PLAN.json",
    "docs/architecture/HEPTA_P0_7B_B3_GOVERNED_BOUNDARIES_CONTRACT_V1.md",
    "docs/architecture/HEPTA_P0_7B_B3_STATUS.json",
    "docs/architecture/HEPTA_V5_B3_GOVERNED_CALLSITE_INVENTORY.json",
    "scripts/verify-hepta-v5-b3-governed-boundaries.py",
}

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
    raise SystemExit(f"FAIL_HEPTA_V5_B3_GOVERNED_BOUNDARIES: {message}")


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
        fail("HEAD is still the B2 parent")
    if parent != EXPECTED_PARENT:
        fail(f"unexpected direct parent: {parent}")
    if parent_tree != EXPECTED_PARENT_TREE:
        fail(f"unexpected parent tree: {parent_tree}")
    if parent_count != 1:
        fail(f"B3 source commit must have exactly one parent, got {parent_count}")
    changed = set(filter(None, git("diff", "--name-only", EXPECTED_PARENT, "HEAD").splitlines()))
    if changed != OWNED_PATHS:
        missing = sorted(OWNED_PATHS - changed)
        extra = sorted(changed - OWNED_PATHS)
        fail(f"owned path drift; missing={missing}, extra={extra}")


def require_snippets(path: str, snippets: tuple[str, ...]) -> None:
    source = read(path)
    for snippet in snippets:
        if snippet not in source:
            fail(f"missing invariant in {path}: {snippet}")


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--require-direct-parent", action="store_true")
    args = parser.parse_args()

    require_snippets(
        "codex-rs/hepta-contracts/src/physical_boundaries.rs",
        ("pub mod external;", "pub mod governed;"),
    )
    require_snippets(
        "codex-rs/hepta-contracts/src/physical_boundaries/governed.rs",
        (
            "mod core;",
            "pub mod secret;",
            "pub mod matrix;",
            "pub mod fleet;",
            "pub mod operator;",
            "pub mod release;",
            "PhysicalCapabilityKind::SecretOperation",
            "PhysicalCapabilityKind::MatrixSend",
            "PhysicalCapabilityKind::FleetMutation",
            "PhysicalCapabilityKind::OperatorAcceptance",
            "PhysicalCapabilityKind::ReleasePromotion",
            "B3_GOVERNED_BOUNDARIES_OPERATOR_ACCEPTANCE: bool = false",
            "B3_GOVERNED_BOUNDARIES_PROMOTION: bool = false",
            "B3_GOVERNED_BOUNDARIES_RELEASE: bool = false",
        ),
    )

    core = read("codex-rs/hepta-contracts/src/physical_boundaries/governed/core.rs")
    for snippet in (
        "verify_physical_capability_use(",
        ".consume_at_boundary(",
        "persist_witness(&witness)",
        "self.adapter.cross(dispatch).await",
        "GovernedBoundaryOutcome::indeterminate",
        "adapter: A",
    ):
        if snippet not in core:
            fail(f"missing shared-core invariant: {snippet}")
    for forbidden in (
        "pub fn adapter(",
        "pub fn adapter_mut(",
        "pub fn into_parts(",
        "pub fn restore_raw",
        "pub fn from_raw",
        "impl Clone for GovernedBoundaryCore",
    ):
        if forbidden in core:
            fail(f"raw governed adapter escape detected: {forbidden}")

    require_snippets(
        "codex-rs/hepta-contracts/src/physical_boundaries/governed/secret.rs",
        (
            "opaque_secret_ref_sha256",
            "token_family_id",
            "purpose",
            "audience",
            "expected_secret_revision",
            "operation_deadline_unix_seconds",
            "GovernedBoundaryIntent::SecretOperation",
            "GovernedBoundaryCore<ExternalEffectCapability",
        ),
    )
    require_snippets(
        "codex-rs/hepta-contracts/src/physical_boundaries/governed/matrix.rs",
        (
            "room_id",
            "event_id",
            "matrix_identity_generation",
            "outbox_envelope_sha256",
            "homeserver_route_sha256",
            "GovernedBoundaryIntent::MatrixSend",
            "GovernedBoundaryCore<ExternalEffectCapability",
        ),
    )
    require_snippets(
        "codex-rs/hepta-contracts/src/physical_boundaries/governed/fleet.rs",
        (
            "registry_revision",
            "release_id",
            "owner_epoch",
            "process_generation",
            "immutable_release_identity_sha256",
            "expected_prior_registry_sha256",
            "GovernedBoundaryCore<FleetMutationCapability",
        ),
    )
    require_snippets(
        "codex-rs/hepta-contracts/src/physical_boundaries/governed/operator.rs",
        (
            "complete_evidence_manifest_sha256",
            "implementer_identity_sha256",
            "independent_reviewer_identity_sha256",
            "review_challenge_sha256",
            "expires_at_unix_seconds",
            "self.implementer_identity_sha256 == self.independent_reviewer_identity_sha256",
            "GovernedBoundaryCore<OperatorAcceptanceCapability",
        ),
    )
    require_snippets(
        "codex-rs/hepta-contracts/src/physical_boundaries/governed/release.rs",
        (
            "release_manifest_sha256",
            "artifact_set_sha256",
            "sbom_sha256",
            "migration_compatibility_sha256",
            "rollback_evidence_sha256",
            "independent_review_receipt_sha256",
            "operator_acceptance_receipt_sha256",
            "GovernedBoundaryCore<ReleasePromotionCapability",
        ),
    )

    tests = read("codex-rs/hepta-contracts/tests/governed_verified_use_boundaries.rs")
    for snippet in (
        "all_five_governed_boundaries_persist_witness_before_adapter",
        "governed_boundary_specific_facts_change_physical_digests",
        "operator_reviewer_identity_must_be_independent",
        "secret_intent_serialization_contains_no_raw_secret_material",
        "claim_witness_and_transport_fail_closed",
        "repository_source_grants_no_governed_authority",
    ):
        if snippet not in tests:
            fail(f"required B3 test missing: {snippet}")

    current = load_json("docs/architecture/HEPTA_CURRENT_PLAN.json")
    if current.get("repository") != "TrillionniumFoundation/hepta-private-ci":
        fail("current-plan repository identity is stale")
    if current.get("currentPackage") != "P0.7b/B3_secret_matrix_fleet_operator_release_boundaries":
        fail("currentPackage does not select B3")
    candidate = current.get("candidateBinding")
    if not isinstance(candidate, dict):
        fail("current plan candidate binding is missing")
    if candidate.get("stackParentCommit") != EXPECTED_PARENT:
        fail("current plan is not bound to the exact B2 parent")
    if candidate.get("sourceBranch") != EXPECTED_BRANCH:
        fail("current plan source branch drift")
    authority = current.get("authority")
    if not isinstance(authority, dict):
        fail("current plan authority object is missing")
    for key in AUTHORITY_KEYS:
        if authority.get(key) is not False:
            fail(f"authority must remain false: {key}")

    status = load_json("docs/architecture/HEPTA_P0_7B_B3_STATUS.json")
    if status.get("state") != "source_implemented_executable_qualification_pending":
        fail("B3 status must remain source implemented / qualification pending")
    if status.get("parentCommit") != EXPECTED_PARENT:
        fail("B3 status parent drift")
    status_authority = status.get("authority")
    if not isinstance(status_authority, dict) or any(value is not False for value in status_authority.values()):
        fail("B3 status contains positive authority")

    inventory = load_json("docs/architecture/HEPTA_V5_B3_GOVERNED_CALLSITE_INVENTORY.json")
    if inventory.get("productCallsitesRegistered") is not False:
        fail("B3 must not register a product callsite")
    for key in (
        "sourceMayIssueIndependentReview",
        "sourceMayIssueOperatorAcceptance",
        "sourceMayIssuePromotion",
        "sourceMayIssueRelease",
    ):
        if inventory.get(key) is not False:
            fail(f"source-issued governance escape: {key}")
    if inventory.get("b4GlobalInventoryRequired") is not True:
        fail("B3 must retain the B4 global callsite proof")
    boundaries = inventory.get("boundaries")
    if not isinstance(boundaries, list) or len(boundaries) != 5:
        fail("B3 inventory must contain exactly five physical boundaries")

    workflow = read(".github/workflows/hepta-architecture-v5-b3-governed-boundaries.yml")
    if "contents: read" not in workflow or "contents: write" in workflow:
        fail("B3 qualification workflow must be read-only")
    for forbidden in ("git push", "update-ref", "gh pr merge", "curl -X POST"):
        if forbidden in workflow:
            fail(f"workflow mutation detected: {forbidden}")

    if args.require_direct_parent:
        require_direct_parent()

    print("PASS_HEPTA_V5_B3_GOVERNED_BOUNDARIES_SOURCE_ONLY")


if __name__ == "__main__":
    main()

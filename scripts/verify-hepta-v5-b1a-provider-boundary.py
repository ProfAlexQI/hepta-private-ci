#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import os
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[1]
EXPECTED_PARENT = "aff643de331ef1ce54940e3925aa64077a019ff7"
EXPECTED_BRANCH = "codex/hepta-architecture-v5-b1a-restack-eb8adb-20260831"
SOURCE_MAIN = Path("codex-rs/hepta-contracts/src/checked_provider_operation.rs")
SOURCE_PARTS = [
    Path("codex-rs/hepta-contracts/src/checked_provider_operation_parts/impl_construction.rs"),
    Path("codex-rs/hepta-contracts/src/checked_provider_operation_parts/impl_dispatch.rs"),
    Path("codex-rs/hepta-contracts/src/checked_provider_operation_parts/impl_reconcile.rs"),
    Path("codex-rs/hepta-contracts/src/checked_provider_operation_parts/helpers.rs"),
    Path("codex-rs/hepta-contracts/src/checked_provider_operation_parts/tests.rs"),
]
TEST_MAIN = Path("codex-rs/hepta-contracts/tests/provider_verified_use_boundary.rs")
TEST_PARTS = [
    Path("codex-rs/hepta-contracts/tests/provider_verified_use_boundary_parts/support1.rs"),
    Path("codex-rs/hepta-contracts/tests/provider_verified_use_boundary_parts/support2.rs"),
    Path("codex-rs/hepta-contracts/tests/provider_verified_use_boundary_parts/tests1.rs"),
    Path("codex-rs/hepta-contracts/tests/provider_verified_use_boundary_parts/tests2.rs"),
]
STATUS = Path("docs/architecture/HEPTA_P0_7B_B1A_STATUS.json")
INVENTORY = Path("docs/architecture/HEPTA_V5_B1A_PROVIDER_CALLSITE_INVENTORY.json")
CONTRACT = Path("docs/architecture/HEPTA_P0_7B_B1A_PROVIDER_BOUNDARY_CONTRACT_V1.md")
WORKFLOW = Path(".github/workflows/hepta-architecture-v5-b1a-provider-boundary.yml")


def reject_duplicates(pairs: list[tuple[str, object]]) -> dict[str, object]:
    result: dict[str, object] = {}
    for key, value in pairs:
        if key in result:
            raise ValueError(f"duplicate JSON key: {key}")
        result[key] = value
    return result


def load_json(path: Path) -> dict[str, object]:
    return json.loads((ROOT / path).read_text(encoding="utf-8"), object_pairs_hook=reject_duplicates)


def read(path: Path) -> str:
    return (ROOT / path).read_text(encoding="utf-8")


def require(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(f"FAIL_HEPTA_V5_B1A: {message}")


def git(*args: str) -> str:
    return subprocess.check_output(["git", *args], cwd=ROOT, text=True).strip()


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--require-direct-parent", action="store_true")
    args = parser.parse_args()

    paths = [SOURCE_MAIN, *SOURCE_PARTS, TEST_MAIN, *TEST_PARTS, STATUS, INVENTORY, CONTRACT, WORKFLOW]
    for path in paths:
        require((ROOT / path).is_file(), f"missing {path}")

    if args.require_direct_parent:
        require(git("rev-parse", "HEAD^") == EXPECTED_PARENT, "source head is not a direct child of current B0")
        branch = os.environ.get("GITHUB_HEAD_REF") or os.environ.get("GITHUB_REF_NAME") or git("branch", "--show-current")
        require(branch == EXPECTED_BRANCH, f"unexpected source branch {branch!r}")

    main_source = read(SOURCE_MAIN)
    source = main_source + "\n" + "\n".join(read(path) for path in SOURCE_PARTS)
    test_main = read(TEST_MAIN)
    tests = test_main + "\n" + "\n".join(read(path) for path in TEST_PARTS)
    contract = read(CONTRACT)
    workflow = read(WORKFLOW)

    for path in SOURCE_PARTS:
        require(f'include!("checked_provider_operation_parts/{path.name}");' in main_source, f"missing include {path.name}")
    for path in TEST_PARTS:
        require(f'include!("provider_verified_use_boundary_parts/{path.name}");' in test_main, f"missing test include {path.name}")

    for needle in [
        "Authorized<ProviderDispatchCapability>",
        "Authorized<ExternalEffectCapability>",
        "PhysicalCapabilityKind::ProviderDispatch",
        "PhysicalCapabilityKind::ExternalEffect",
        "dispatch_once_with_payload",
        "TrustedPhysicalClock",
        "PhysicalUseClaimStore",
        "persist_witness_pair",
        "let final_payload_sha256 = Sha256Digest::for_bytes(wire_payload);",
        ".dispatch_once_with_payload(intent, wire_payload, crossed_at_unix_seconds)",
    ]:
        require(needle in source, f"missing boundary token: {needle}")
    require("pub async fn dispatch_once(" not in source, "public no-payload dispatch exposed")
    require(
        source.index("persist_witness_pair(")
        < source.index(".dispatch_once_with_payload(intent, wire_payload, crossed_at_unix_seconds)"),
        "witness pair is not persisted before raw dispatch",
    )

    for needle in [
        "dual_claims_and_witness_pair_are_durable_before_provider_dispatch",
        "witness_pair_persistence_failure_prevents_physical_send_and_replay",
        "second_capability_claim_failure_never_crosses_provider_boundary",
        "final_payload_drift_is_rejected_before_any_claim_or_send",
    ]:
        require(needle in tests, f"missing focused test: {needle}")

    status = load_json(STATUS)
    require(status["stackParent"]["commit"] == EXPECTED_PARENT, "status parent commit drift")
    require(
        status["stackParent"]["branch"] == "codex/hepta-architecture-v5-b0-exact-restack-20260831",
        "status parent branch drift",
    )
    expected_files = [str(path) for path in [SOURCE_MAIN, *SOURCE_PARTS, TEST_MAIN, *TEST_PARTS]]
    require(status["source"]["files"] == expected_files, "status source inventory drift")
    require(status["source"]["implemented"] is True, "source not marked implemented")
    require(status["allGapsClosed"] is False, "B1a cannot claim all architecture gaps closed")
    require(all(value is False for value in status["authority"].values()), "positive authority in status")

    inventory = load_json(INVENTORY)
    require(inventory["productCallsites"] == [], "product callsite registered")
    require(inventory["runtimeRegistered"] is False and inventory["qualified"] is False, "runtime claim crossed")
    require(all(value is False for value in inventory["authority"].values()), "positive authority in inventory")
    require(set(inventory["allowedRustReferences"]) == set(expected_files), "Rust reference inventory drift")

    require("current broad-capability verification" in contract, "contract missing current-use revalidation")
    require(EXPECTED_BRANCH in workflow, "workflow branch drift")
    require(
        "cargo test --locked -p codex-hepta-contracts --test provider_verified_use_boundary" in workflow,
        "focused executable gate missing",
    )
    require(
        "cargo clippy --locked -p codex-hepta-contracts --all-targets -- -D warnings" in workflow,
        "strict Clippy gate missing",
    )
    print("PASS_HEPTA_V5_B1A_PROVIDER_BOUNDARY_SOURCE")
    return 0


if __name__ == "__main__":
    sys.exit(main())

#!/usr/bin/env python3
"""Fail-closed source verifier for AuthBus P1.3 semantic closure V12."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
import subprocess
import sys
from typing import Any

ROOT = Path(__file__).resolve().parents[1]
PARENT = "8572f3d2182541b14e0719b229ccd8754494f134"
PARENT_TREE = "aac769f278dad18b97b3c63c97f9b43dd325aa24"
SOURCE = ROOT / "codex-rs/hepta-authbus-p1-3-qualification/src/semantic_quota.rs"
LIB = ROOT / "codex-rs/hepta-authbus-p1-3-qualification/src/lib.rs"
TESTS = ROOT / "codex-rs/hepta-authbus-p1-3-qualification/tests/p1_3_semantic.rs"
MANIFEST = ROOT / "codex-rs/hepta-authbus-p1-3-qualification/Cargo.toml"
STATUS = ROOT / "docs/hepta-vnext/authbus/AUTHBUS_P1_3_SEMANTIC_CLOSURE_V12_2026-08-31.json"
DESIGN = ROOT / "docs/hepta-vnext/authbus/AUTHBUS_P1_3_SEMANTIC_CLOSURE_V12_2026-08-31.md"
WORKFLOW = ROOT / ".github/workflows/hepta-authbus-p1-3-semantic-v12.yml"

EXPECTED_DELTA = {
    ".github/workflows/hepta-authbus-p1-3-semantic-v12.yml",
    "codex-rs/hepta-authbus-p1-3-qualification/src/lib.rs",
    "codex-rs/hepta-authbus-p1-3-qualification/src/semantic_quota.rs",
    "codex-rs/hepta-authbus-p1-3-qualification/tests/p1_3_semantic.rs",
    "docs/hepta-vnext/authbus/AUTHBUS_P1_3_SEMANTIC_CLOSURE_V12_2026-08-31.json",
    "docs/hepta-vnext/authbus/AUTHBUS_P1_3_SEMANTIC_CLOSURE_V12_2026-08-31.md",
    "scripts/verify-authbus-p1-3-semantic-v12.py",
}

REQUIRED_TESTS = {
    "rpm_and_tpm_are_isolated_by_exact_minute_window",
    "daily_budget_rolls_only_at_exact_utc_day_boundary",
    "max_context_is_per_request_and_never_aggregate_spend",
    "context_above_per_request_limit_fails_before_any_counter_changes",
    "stale_or_wrong_window_binding_fails_closed",
    "stale_revision_and_changed_idempotency_binding_fail_closed",
    "completed_transition_conserves_hold_consumed_and_remaining",
    "invalid_state_transitions_and_post_dispatch_expiry_are_rejected",
    "transition_chain_detects_digest_tampering",
    "request_count_is_cumulative_across_window_rollover",
    "unknown_limit_and_authority_escape_are_rejected",
}


class DuplicateKeyError(ValueError):
    pass


def reject_duplicate_keys(pairs: list[tuple[str, Any]]) -> dict[str, Any]:
    value: dict[str, Any] = {}
    for key, item in pairs:
        if key in value:
            raise DuplicateKeyError(f"duplicate JSON key: {key}")
        value[key] = item
    return value


def fail(message: str) -> None:
    raise SystemExit(f"FAIL_AUTHBUS_P1_3_SEMANTIC_V12: {message}")


def require(condition: bool, message: str) -> None:
    if not condition:
        fail(message)


def read(path: Path) -> str:
    try:
        return path.read_text(encoding="utf-8")
    except OSError as exc:
        fail(f"cannot read {path.relative_to(ROOT)}: {exc}")


def false_tree(value: Any, path: str) -> None:
    if isinstance(value, dict):
        for key, child in value.items():
            false_tree(child, f"{path}.{key}")
    elif isinstance(value, bool):
        require(not value, f"positive authority at {path}")


def verify_git_delta() -> None:
    try:
        parent_tree = subprocess.check_output(
            ["git", "rev-parse", f"{PARENT}^{{tree}}"],
            cwd=ROOT,
            text=True,
        ).strip()
        changed = set(
            subprocess.check_output(
                ["git", "diff", "--name-only", PARENT, "HEAD"],
                cwd=ROOT,
                text=True,
            ).splitlines()
        )
    except subprocess.CalledProcessError as exc:
        fail(f"cannot resolve exact parent or diff: {exc}")
    require(parent_tree == PARENT_TREE, "exact parent tree drift")
    require(
        changed == EXPECTED_DELTA,
        f"exact delta mismatch; missing={sorted(EXPECTED_DELTA - changed)} extra={sorted(changed - EXPECTED_DELTA)}",
    )


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--check-git-delta", action="store_true")
    args = parser.parse_args()

    source = read(SOURCE)
    lib = read(LIB)
    tests = read(TESTS)
    manifest = read(MANIFEST)
    design = read(DESIGN)
    workflow = read(WORKFLOW)

    try:
        status = json.loads(read(STATUS), object_pairs_hook=reject_duplicate_keys)
    except (json.JSONDecodeError, DuplicateKeyError) as exc:
        fail(f"invalid status JSON: {exc}")

    require(status.get("schema") == "hepta.authbus.p1.3.semantic-closure.v12", "schema drift")
    require(status.get("schemaVersion") == 12, "schema version drift")
    require(
        status.get("status") == "source_implemented_executable_qualification_pending",
        "status overclaim or drift",
    )
    require(status.get("parent", {}).get("commit") == PARENT, "parent commit drift")
    require(status.get("parent", {}).get("tree") == PARENT_TREE, "parent tree drift")
    require(
        status.get("candidate", {}).get("commit") == "resolved_by_exact_head_workflow",
        "committed source self-asserts an exact candidate",
    )
    require(
        status.get("executionQualification", {}).get("semanticQualificationAccepted") is False,
        "semantic qualification self-issued",
    )
    false_tree(status.get("authority"), "authority")

    expected_closed = {
        "window_keyed_revision_bound_rpm_accounting",
        "window_keyed_revision_bound_tpm_accounting",
        "window_keyed_revision_bound_daily_budget_accounting",
        "exact_utc_minute_and_day_interval_validation",
        "per_request_context_non_accumulation",
        "active_only_concurrency_holds",
        "reservation_held_consumed_remaining_conservation",
        "state_specific_vector_rules",
        "stale_revision_rejection",
        "exact_idempotency_replay_and_binding_conflict",
        "append_only_recomputed_transition_digest_chain",
        "invalid_transition_and_post_dispatch_expiry_rejection",
        "multi_window_multi_request_and_tamper_tests",
    }
    require(set(status.get("closedInSource", [])) == expected_closed, "source-closure ledger drift")
    require(set(status.get("requiredTests", [])) == REQUIRED_TESTS, "required-test ledger drift")

    for token in [
        "pub struct QuotaWindowKey",
        "pub struct QuotaWindowBindings",
        "pub struct SemanticReservationRequest",
        "pub struct SemanticReservationRecord",
        "pub struct SemanticTransitionReceipt",
        "pub struct WindowedQuotaLedger",
        "pub fn verify_transition_chain",
        "QuotaDimension::Rpm | QuotaDimension::Tpm => QuotaWindowKind::MinuteUtc",
        "QuotaDimension::DayBudget => QuotaWindowKind::DayUtc",
        "held = consumed + remaining",
        "actual.context",
        "intentionally absent from aggregate counters",
        "fn transactional<T>",
        "pub fn verify_invariants",
        "SemanticReservationState::Indeterminate",
    ]:
        require(token in source, f"semantic source missing required token: {token}")

    for forbidden in [
        "unsafe {",
        "std::net",
        "reqwest",
        "hyper::",
        "sqlx::",
        "rusqlite",
        "Command::new",
        "std::fs::write",
        "production_authority = true",
        "AUTHBUS_P1_3_AUTHORITY: bool = true",
    ]:
        require(forbidden not in source, f"forbidden semantic-kernel surface: {forbidden}")

    require(
        '#[cfg(feature = "p1-3-qualification")]\nmod semantic_quota;' in lib,
        "semantic module is not feature-gated",
    )
    require("pub use semantic_quota::WindowedQuotaLedger;" in lib, "ledger not exported")
    require("pub use semantic_quota::verify_transition_chain;" in lib, "chain verifier not exported")
    for authority_constant in [
        "AUTHBUS_P1_3_AUTHORITY: bool = false",
        "AUTHBUS_P1_3_EFFECT_AUTHORITY: bool = false",
        "AUTHBUS_P1_3_PRODUCTION_CALLER: bool = false",
        "AUTHBUS_P1_3_PRODUCTION_WRITER: bool = false",
        "AUTHBUS_P1_3_OPERATOR_ACCEPTANCE: bool = false",
        "AUTHBUS_P1_3_PROMOTION: bool = false",
        "AUTHBUS_P1_3_G5_ALLOWED: bool = false",
        "AUTHBUS_P1_3_EXECUTE_ALLOWED: bool = false",
    ]:
        require(authority_constant in lib, f"negative authority constant drift: {authority_constant}")

    require("default = []" in manifest, "qualification crate default feature set is not empty")
    require("p1-3-qualification = [" in manifest, "explicit qualification feature missing")
    for test_name in REQUIRED_TESTS:
        require(f"fn {test_name}()" in tests, f"missing adversarial test: {test_name}")

    for token in [
        "window-keyed accounting",
        "per-request context enforcement",
        "held = consumed + remaining",
        "Post-dispatch expiry is forbidden",
        "All such authority remains false",
    ]:
        require(token in design, f"design note missing requirement: {token}")

    require("permissions:\n  contents: read" in workflow, "workflow is not read-only")
    for forbidden in [
        "contents: write",
        "git push",
        "git commit",
        "update-ref",
        "create-pull-request",
        "gh pr merge",
        "workflow_run:",
    ]:
        require(forbidden not in workflow, f"workflow mutation/escalation surface: {forbidden}")
    for token in [
        "cargo fmt",
        "p1_3_semantic",
        "cargo test",
        "cargo check",
        "cargo clippy",
        "-D warnings",
        "verify-authbus-p1-3-semantic-v12.py",
    ]:
        require(token in workflow, f"workflow missing gate: {token}")

    if args.check_git_delta:
        verify_git_delta()

    subprocess.run(["git", "diff", "--check"], cwd=ROOT, check=True)
    print("PASS_AUTHBUS_P1_3_SEMANTIC_V12_SOURCE_ONLY")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except BrokenPipeError:
        sys.exit(1)

#!/usr/bin/env python3
"""Fail-closed source gate for AuthBus B4 P0.3 qualification."""

from __future__ import annotations

import json
from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[1]
CRATE = ROOT / "codex-rs" / "hepta-authbus-p0-3-qualification"


def fail(message: str) -> None:
    print(f"FAIL_AUTHBUS_P0_3_SOURCE: {message}", file=sys.stderr)
    raise SystemExit(1)


def require(condition: bool, message: str) -> None:
    if not condition:
        fail(message)


def read(relative: str) -> str:
    path = ROOT / relative
    require(path.is_file(), f"missing {relative}")
    return path.read_text(encoding="utf-8")


def main() -> None:
    require(CRATE.is_dir(), "P0.3 qualification crate is absent")
    require(
        not (ROOT / ".github/workflows/authbus-p0-3-lock-bootstrap.yml").exists(),
        "one-shot bootstrap workflow must be removed from the final candidate",
    )
    require(
        not (ROOT / "scripts/bootstrap-authbus-p0-3.py").exists(),
        "source transform script must be removed from the final candidate",
    )

    cargo = read("codex-rs/hepta-authbus-p0-3-qualification/Cargo.toml")
    lock = read("codex-rs/hepta-authbus-p0-3-qualification/Cargo.lock")
    lib = read("codex-rs/hepta-authbus-p0-3-qualification/src/lib.rs")
    scheduler = read("codex-rs/hepta-authbus-p0-3-qualification/src/scheduler.rs")
    p0_3_tests = read("codex-rs/hepta-authbus-p0-3-qualification/tests/p0_3.rs")
    reconcile_binding_tests = read(
        "codex-rs/hepta-authbus-p0-3-qualification/tests/reconcile_binding.rs"
    )
    reconcile_history_tests = read(
        "codex-rs/hepta-authbus-p0-3-qualification/tests/reconcile_history.rs"
    )
    replay_fence_tests = read(
        "codex-rs/hepta-authbus-p0-3-qualification/tests/replay_fence.rs"
    )
    parent_workspace = read("codex-rs/Cargo.toml")

    require("[workspace]" in cargo, "crate must remain an isolated nested workspace")
    require('rust-version = "1.95"' in cargo, "Rust MSRV must remain pinned to 1.95")
    require('resolver = "3"' in cargo, "nested workspace must use resolver 3")
    require("default = []" in cargo, "default features must remain empty")
    require(
        'p0-3-qualification = ["dep:codex-hepta-contracts"]' in cargo,
        "P0.3 implementation must remain feature-gated",
    )
    require("version = 4" in lock, "committed Cargo.lock v4 is required")
    require(
        "name = \"codex-hepta-authbus-p0-3-qualification\"" in lock,
        "Cargo.lock must bind the P0.3 package",
    )
    require(
        '"hepta-authbus-p0-3-qualification"' not in parent_workspace,
        "P0.3 crate must not be added to the product workspace",
    )

    required_false = [
        "AUTHBUS_B4_P0_3_AUTHORITY",
        "AUTHBUS_B4_P0_3_EFFECT_AUTHORITY",
        "AUTHBUS_B4_P0_3_PRODUCTION_CALLER",
        "AUTHBUS_B4_P0_3_PRODUCTION_WRITER",
        "AUTHBUS_B4_P0_3_OPERATOR_ACCEPTANCE",
        "AUTHBUS_B4_P0_3_PROMOTION",
        "AUTHBUS_B4_P0_3_G5_ALLOWED",
        "AUTHBUS_B4_P0_3_EXECUTE_ALLOWED",
    ]
    for constant in required_false:
        require(
            f"pub const {constant}: bool = false;" in lib,
            f"{constant} must remain false",
        )
    require(
        "pub const AUTHBUS_B4_P0_3_QUALIFICATION_ONLY: bool = true;" in lib,
        "qualification-only marker must remain true",
    )

    required_markers = [
        "request_count",
        "CanonicalQuotaVector",
        "P03AdmissionDisposition::AlreadyPresent",
        "IdempotencyConflict",
        "BindingConflict",
        "reconcile_old_permit",
        "VerifiedConsumed",
        "VerifiedNoEffect",
        "HeldUnknown",
        "terminal_reconcile_by_permit",
        "expire_active_permits",
        "ExpiredPreDispatch",
        "OutcomeUnknown",
        "verify_invariants",
        "dispatch_marker_sha256",
        "unknown_marker_sha256",
        "dispatch_marker_digest",
        "unknown_marker_digest",
        "validate_replay_fence",
        ".can_hold(self.used, self.held, CanonicalQuotaVector::default())",
    ]
    for marker in required_markers:
        require(marker in scheduler, f"missing P0.3 implementation marker {marker}")

    combined_tests = "\n".join(
        [
            p0_3_tests,
            reconcile_binding_tests,
            reconcile_history_tests,
            replay_fence_tests,
        ]
    )
    required_tests = [
        "exact_idempotency_replays_original_permit_and_changed_payload_conflicts",
        "old_fence_consumed_reconcile_completes_once_and_replays_receipt",
        "verified_no_effect_releases_old_permit_without_usage",
        "unknown_reconcile_keeps_hold_and_later_terminal_evidence_can_settle",
        "stale_or_forged_reconcile_evidence_is_non_mutating",
        "expiry_releases_only_pre_dispatch_and_never_unknown_effects",
        "dispatch_and_unknown_markers_are_exactly_idempotent",
        "reconcile_digest_binds_expected_revision",
        "exact_unknown_reconcile_remains_replayable_after_terminal_settlement",
        "marker_replay_is_bound_to_exact_fence_evidence_and_observed_time",
    ]
    for marker in required_tests:
        require(marker in combined_tests, f"missing P0.3 regression {marker}")

    forbidden = [
        "access_token",
        "refresh_token",
        "client_secret",
        "authorization_header",
        "provider_body",
        "openbao_client",
        "listen(",
    ]
    combined_source = (lib + scheduler).lower()
    for marker in forbidden:
        require(marker not in combined_source, f"forbidden authority/secret marker {marker}")

    receipt = {
        "schema": "hepta.authbus.b4.p0.3.source-gate.v2",
        "source_present": True,
        "executed": True,
        "rust_qualified": False,
        "committed_lock": True,
        "rust_version": "1.95",
        "resolver": "3",
        "canonical_idempotency": True,
        "dispatch_replay_evidence_bound": True,
        "unknown_replay_evidence_bound": True,
        "historical_reconcile_replay": True,
        "combined_used_held_invariant": True,
        "p0_2_executable_gate_inherited": False,
        "qualification_only": True,
        "authority": False,
        "effect_authority": False,
        "production_caller": False,
        "production_writer": False,
        "operator_acceptance": False,
        "promotion": False,
        "g5_allowed": False,
        "execute_allowed": False,
        "decision": "PASS_AUTHBUS_P0_3_SOURCE_ONLY",
    }
    print(json.dumps(receipt, sort_keys=True, separators=(",", ":")))


if __name__ == "__main__":
    main()

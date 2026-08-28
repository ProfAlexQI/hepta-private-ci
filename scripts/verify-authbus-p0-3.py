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
    cargo = read("codex-rs/hepta-authbus-p0-3-qualification/Cargo.toml")
    lib = read("codex-rs/hepta-authbus-p0-3-qualification/src/lib.rs")
    scheduler = read("codex-rs/hepta-authbus-p0-3-qualification/src/scheduler.rs")
    tests = read("codex-rs/hepta-authbus-p0-3-qualification/tests/p0_3.rs")
    parent_workspace = read("codex-rs/Cargo.toml")

    require("[workspace]" in cargo, "crate must remain an isolated nested workspace")
    require("default = []" in cargo, "default features must remain empty")
    require(
        'p0-3-qualification = ["dep:codex-hepta-contracts"]' in cargo,
        "P0.3 implementation must remain feature-gated",
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
    ]
    for marker in required_markers:
        require(marker in scheduler, f"missing P0.3 implementation marker {marker}")

    required_tests = [
        "exact_idempotency_replays_original_permit_and_changed_payload_conflicts",
        "old_fence_consumed_reconcile_completes_once_and_replays_receipt",
        "verified_no_effect_releases_old_permit_without_usage",
        "unknown_reconcile_keeps_hold_and_later_terminal_evidence_can_settle",
        "stale_or_forged_reconcile_evidence_is_non_mutating",
        "expiry_releases_only_pre_dispatch_and_never_unknown_effects",
        "dispatch_and_unknown_markers_are_exactly_idempotent",
    ]
    for marker in required_tests:
        require(marker in tests, f"missing P0.3 regression {marker}")

    forbidden = [
        "access_token",
        "refresh_token",
        "client_secret",
        "authorization_header",
        "provider_body",
        "openbao_client",
        "listen(",
    ]
    combined = (lib + scheduler).lower()
    for marker in forbidden:
        require(marker not in combined, f"forbidden authority/secret marker {marker}")

    receipt = {
        "schema": "hepta.authbus.b4.p0.3.source-gate.v1",
        "source_present": True,
        "executed": True,
        "rust_qualified": False,
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

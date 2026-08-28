#!/usr/bin/env python3
"""Fail-closed source gate for the AuthBus P0.2 qualification crate."""

from __future__ import annotations

import json
from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[1]
CRATE = ROOT / "codex-rs" / "hepta-authbus-qualification"


def fail(message: str) -> None:
    print(f"FAIL_AUTHBUS_P0_2_SOURCE: {message}", file=sys.stderr)
    raise SystemExit(1)


def require(condition: bool, message: str) -> None:
    if not condition:
        fail(message)


def read(relative: str) -> str:
    path = ROOT / relative
    require(path.is_file(), f"missing {relative}")
    return path.read_text(encoding="utf-8")


def main() -> None:
    require(CRATE.is_dir(), "qualification crate is absent")
    require(not (CRATE / "src" / "_probe.rs").exists(), "_probe.rs must be removed")
    require(not (CRATE / "src" / "_probe2.rs").exists(), "_probe2.rs must be removed")

    cargo = read("codex-rs/hepta-authbus-qualification/Cargo.toml")
    lib = read("codex-rs/hepta-authbus-qualification/src/lib.rs")
    model = read("codex-rs/hepta-authbus-qualification/src/model.rs")
    store = read("codex-rs/hepta-authbus-qualification/src/store.rs")
    migration = read(
        "codex-rs/hepta-authbus-qualification/migrations/0001_authbus_p0_2.sql"
    )
    tests = read("codex-rs/hepta-authbus-qualification/tests/sqlite_wal.rs")

    require("[workspace]" in cargo, "crate must remain an isolated nested workspace")
    require("default = []" in cargo, "default features must remain empty")
    require(
        "sqlite-qualification" in cargo,
        "SQLite implementation must remain explicit-feature-only",
    )

    required_false = [
        "AUTHBUS_P0_2_AUTHORITY",
        "AUTHBUS_P0_2_EFFECT_AUTHORITY",
        "AUTHBUS_P0_2_PRODUCTION_CALLER",
        "AUTHBUS_P0_2_PRODUCTION_WRITER",
        "AUTHBUS_P0_2_OPERATOR_ACCEPTANCE",
        "AUTHBUS_P0_2_PROMOTION",
        "AUTHBUS_P0_2_G5_ALLOWED",
        "AUTHBUS_P0_2_EXECUTE_ALLOWED",
    ]
    for constant in required_false:
        require(
            f"pub const {constant}: bool = false;" in lib,
            f"{constant} must be statically false",
        )
    require(
        "pub const AUTHBUS_P0_2_QUALIFICATION_ONLY: bool = true;" in lib,
        "qualification-only marker must remain true",
    )

    required_tables = [
        "authbus_p0_2_meta",
        "operations",
        "token_family_claims",
        "quota_reservations",
        "dispatch_attempts",
        "status_observations",
        "outbox",
        "outbox_cursor",
        "fsync_receipts",
    ]
    for table in required_tables:
        require(f"CREATE TABLE {table}" in migration, f"missing table {table}")

    forbidden_storage_names = [
        "access_token",
        "refresh_token",
        "client_secret",
        "authorization_header",
        "secret_bytes",
        "provider_body",
    ]
    migration_lower = migration.lower()
    for name in forbidden_storage_names:
        require(name not in migration_lower, f"raw-secret column/name forbidden: {name}")

    required_store_markers = [
        "open_durable_evidence_pool",
        "begin_dispatch",
        "record_dispatch_observation",
        "record_status_observation",
        "recover_operation",
        "ack_outbox",
        "verify_integrity",
        "StaleWriter",
        "StaleFence",
    ]
    joined = model + store
    for marker in required_store_markers:
        require(marker in joined, f"missing coordinator marker {marker}")

    required_tests = [
        "durable_attempt_reopens_lookup_only_and_never_auto_dispatches",
        "unknown_marker_then_lookup_completion_settles_quota_and_claim_atomically",
        "stale_status_revision_changed_digest_and_time_rollback_fail_closed",
        "writer_generation_rebind_fences_old_store_and_ticket",
        "commit_failpoints_rollback_entire_transaction",
        "outbox_ack_uses_cursor_cas_and_exact_replay",
        "corrupt_row_digest_is_detected_on_read_and_integrity_scan",
    ]
    for marker in required_tests:
        require(marker in tests, f"missing regression {marker}")

    receipt = {
        "schema": "hepta.authbus.p0.2.source-gate.v1",
        "source_present": True,
        "executed": True,
        "rust_qualified": False,
        "qualification_only": True,
        "authority": False,
        "effect_authority": False,
        "production_caller": False,
        "production_writer": False,
        "operator_acceptance": False,
        "promotion": False,
        "g5_allowed": False,
        "execute_allowed": False,
        "decision": "PASS_AUTHBUS_P0_2_SOURCE_ONLY",
    }
    print(json.dumps(receipt, sort_keys=True, separators=(",", ":")))


if __name__ == "__main__":
    main()

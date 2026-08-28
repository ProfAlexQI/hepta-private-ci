#!/usr/bin/env python3
"""Fail-closed source verifier for AuthBus P1.2.

This verifier proves only source presence, schema shape, workspace isolation and
negative authority. It never claims Rust execution or production readiness.
"""

from __future__ import annotations

import json
import re
import sys
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
CRATE = ROOT / "codex-rs/hepta-authbus-p1-2-qualification"
STATUS = ROOT / "docs/hepta-vnext/authbus/AUTHBUS_P1_2_IMPLEMENTATION_STATUS_2026-08-28.json"

REQUIRED = [
    CRATE / "Cargo.toml",
    CRATE / "README.md",
    CRATE / "migrations/0001_authbus_p1_2.sql",
    CRATE / "src/lib.rs",
    CRATE / "src/model.rs",
    CRATE / "src/store.rs",
    CRATE / "tests/p1_2.rs",
    ROOT / "docs/hepta-vnext/authbus/AUTHBUS_P1_2_DEVELOPMENT_PLAN_2026-08-28.md",
    STATUS,
    ROOT / "docs/hepta-vnext/authbus/AUTHBUS_P1_2_IMPLEMENTATION_STATUS_2026-08-28.md",
    ROOT / ".github/workflows/authbus-p1-2-qualification.yml",
]

FALSE_AUTHORITY_FIELDS = [
    "authority",
    "effect_authority",
    "production_caller",
    "production_writer",
    "operator_acceptance",
    "promotion",
    "g5_allowed",
    "execute_allowed",
    "listener_enabled",
    "provider_call_enabled",
    "openbao_enabled",
    "parent_workspace_wired",
    "private_key_storage",
    "raw_signature_storage",
    "secret_storage",
]

CONSTANTS_FALSE = [
    "AUTHBUS_P1_2_AUTHORITY",
    "AUTHBUS_P1_2_EFFECT_AUTHORITY",
    "AUTHBUS_P1_2_PRODUCTION_CALLER",
    "AUTHBUS_P1_2_PRODUCTION_WRITER",
    "AUTHBUS_P1_2_OPERATOR_ACCEPTANCE",
    "AUTHBUS_P1_2_PROMOTION",
    "AUTHBUS_P1_2_G5_ALLOWED",
    "AUTHBUS_P1_2_EXECUTE_ALLOWED",
    "AUTHBUS_P1_2_LISTENER_ENABLED",
    "AUTHBUS_P1_2_PROVIDER_CALL_ENABLED",
    "AUTHBUS_P1_2_OPENBAO_ENABLED",
    "AUTHBUS_P1_2_PRIVATE_KEY_STORAGE",
    "AUTHBUS_P1_2_RAW_SIGNATURE_STORAGE",
    "AUTHBUS_P1_2_SECRET_STORAGE",
    "AUTHBUS_P1_2_PARENT_WORKSPACE_WIRED",
]

TABLES = [
    "authbus_p1_2_meta",
    "p12_key_registrations",
    "p12_key_heads",
    "p12_nonce_claims",
    "p12_operations",
    "p12_status_evidence",
    "p12_status_heads",
    "p12_manual_evidence",
    "p12_manual_heads",
    "p12_terminal_tombstones",
    "p12_durable_receipts",
    "p12_gc_cursor",
]

TESTS = [
    "default_off_authority_and_private_file_posture_are_enforced",
    "key_rotation_revocation_and_reopen_are_durable_and_monotonic",
    "nonce_replay_survives_reopen_and_capacity_fails_closed",
    "provider_status_replay_conflict_and_terminal_tombstone_survive_reopen",
    "manual_evidence_uses_an_independent_revision_ledger_and_lookup_only_resume",
    "writer_generation_rebind_fences_every_stale_store_instance",
    "every_precommit_failpoint_rolls_back_without_partial_replay_state",
    "bounded_gc_preserves_live_heads_and_terminal_tombstones_until_deadline",
    "row_digest_corruption_fails_integrity_and_reopen_closed",
]


def fail(message: str) -> "NoReturn":
    print(json.dumps({"claim": "FAIL_AUTHBUS_P1_2_SOURCE", "error": message}, sort_keys=True))
    raise SystemExit(1)


def require(condition: bool, message: str) -> None:
    if not condition:
        fail(message)


def text(path: Path) -> str:
    try:
        return path.read_text(encoding="utf-8")
    except (OSError, UnicodeError) as exc:
        fail(f"cannot read {path.relative_to(ROOT)}: {exc}")


def main() -> int:
    for path in REQUIRED:
        require(path.is_file(), f"missing required file: {path.relative_to(ROOT)}")

    cargo = tomllib.loads(text(CRATE / "Cargo.toml"))
    package = cargo.get("package", {})
    require(package.get("name") == "codex-hepta-authbus-p1-2-qualification", "unexpected package name")
    require(package.get("edition") == "2024", "P1.2 must use Rust edition 2024")
    require(package.get("rust-version") == "1.95", "P1.2 must bind Rust 1.95")
    require(cargo.get("workspace", {}).get("resolver") == "3", "P1.2 must be a nested resolver-3 workspace")
    features = cargo.get("features", {})
    require(features.get("default") == [], "P1.2 default features must remain empty")
    require("p1-2-qualification" in features, "missing explicit P1.2 qualification feature")
    deps = cargo.get("dependencies", {})
    require(deps.get("codex-hepta-authbus-p1-qualification", {}).get("optional") is True, "P1.1 must remain optional/default-off")
    require(deps.get("sqlx", {}).get("version") == "=0.9.0", "P1.2 must pin the reviewed SQLx 0.9 line")
    require("sqlite-bundled" in deps.get("sqlx", {}).get("features", []), "P1.2 must use bundled SQLite qualification")

    lib = text(CRATE / "src/lib.rs")
    require("pub const AUTHBUS_P1_2_QUALIFICATION_ONLY: bool = true;" in lib, "qualification-only constant drift")
    for constant in CONSTANTS_FALSE:
        require(f"pub const {constant}: bool = false;" in lib, f"negative-authority constant drift: {constant}")
    require("const _: ()" in lib, "compile-time negative-authority assertions missing")
    require('#[cfg(feature = "p1-2-qualification")]' in lib, "feature isolation missing")

    migration = text(CRATE / "migrations/0001_authbus_p1_2.sql")
    for table in TABLES:
        require(re.search(rf"CREATE TABLE\s+{re.escape(table)}\b", migration, re.IGNORECASE) is not None, f"missing durable table {table}")
    require(migration.upper().count(" STRICT;") >= len(TABLES), "every P1.2 table must be STRICT")
    forbidden_column = re.compile(
        r"\b(private_key|raw_signature|access_token|refresh_token|authorization_header|provider_body|secret_value|secret_bytes)\b\s+(?:TEXT|BLOB)",
        re.IGNORECASE,
    )
    require(forbidden_column.search(migration) is None, "durable schema can represent forbidden secret/signature bytes")
    require("CHECK (authority = 0)" in migration, "schema authority check missing")
    require("CHECK (effect_authority = 0)" in migration, "schema effect-authority check missing")

    store = text(CRATE / "src/store.rs")
    for marker in [
        "use codex_state::SqliteConfig;",
        "use codex_utils_absolute_path::AbsolutePathBuf;",
        "open_durable_evidence_pool",
        "PRAGMA secure_delete = ON",
        "PRAGMA trusted_schema = OFF",
        "ensure_current_writer",
        "p12_terminal_tombstones",
        "collect_garbage",
        "verify_integrity",
        "PRAGMA quick_check",
        "StorageUnavailableBeforeCommit",
    ]:
        require(marker in store, f"missing durable store invariant: {marker}")
    for forbidden in [
        "connect_with",
        "SqlitePoolOptions",
        "SqliteConnectOptions",
        "SqliteJournalMode",
        "SqliteSynchronous",
    ]:
        require(forbidden not in store, f"direct SQLite construction survived: {forbidden}")
    require("require_durable_key" in store, "durable key-purpose/epoch enforcement missing")
    require("expected_revision" in store, "GC CAS binding missing")

    model = text(CRATE / "src/model.rs")
    for marker in [
        "P12NonceClaim",
        "P12ProviderObservation",
        "P12ManualObservation",
        "P12WriterIdentity",
        "P12GcRequest",
        "P12IntegrityReport",
        "TerminalImmutable",
        "NonceReplay",
        "StaleWriter",
    ]:
        require(marker in model, f"missing typed P1.2 model: {marker}")

    tests = text(CRATE / "tests/p1_2.rs")
    for test_name in TESTS:
        require(f"async fn {test_name}" in tests, f"missing executable regression {test_name}")
    for failpoint in [
        "KeyBeforeCommit",
        "NonceBeforeCommit",
        "OperationBeforeCommit",
        "StatusBeforeCommit",
        "ManualBeforeCommit",
        "GcBeforeCommit",
        "StorageUnavailableBeforeCommit",
    ]:
        require(failpoint in tests, f"missing failpoint coverage: {failpoint}")

    status = json.loads(text(STATUS))
    require(status.get("schema") == "hepta.authbus.p1.2.implementation-status.v1", "status schema drift")
    require(status.get("parent_commit") == "0be01b7b5063066794731e545cf304e4c07c1fc5", "status parent binding drift")
    require(status.get("implemented") is True and status.get("wired") is False, "status implementation/wiring boundary drift")
    require(status.get("qualified") is False, "source status cannot claim executable qualification")
    for field in FALSE_AUTHORITY_FIELDS:
        require(status.get(field) is False, f"status authority field escaped: {field}")

    output = {
        "claim": "PASS_AUTHBUS_P1_2_SOURCE_ONLY",
        "parent_commit": status["parent_commit"],
        "qualification_only": True,
        "implemented": True,
        "wired": False,
        "qualified": False,
        "tables": len(TABLES),
        "regressions": len(TESTS),
        "authority": False,
        "effect_authority": False,
        "production_caller": False,
        "production_writer": False,
        "listener_enabled": False,
        "provider_call_enabled": False,
        "openbao_enabled": False,
    }
    print(json.dumps(output, sort_keys=True, separators=(",", ":")))
    return 0


if __name__ == "__main__":
    sys.exit(main())

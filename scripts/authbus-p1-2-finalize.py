#!/usr/bin/env python3
from __future__ import annotations

import base64
import hashlib
import io
import re
import shutil
import tarfile
from pathlib import Path, PurePosixPath

ARCHIVE_SHA256 = "1a5570e79a842c389b9af929926030f5ce9d98b3f46e2f35d7f639d8a1aaa8fc"
CRATE_ROOT = Path("codex-rs/hepta-authbus-p1-2-qualification")
CHUNK_PATHS = [
    Path(f"scripts/authbus-p1-2-source-seed.part-{index:02d}.b64")
    for index in range(8)
]
EXPECTED_ARCHIVE_PATHS = {
    ".github/workflows/authbus-p1-2-qualification.yml",
    "codex-rs/hepta-authbus-p1-2-qualification/Cargo.toml",
    "codex-rs/hepta-authbus-p1-2-qualification/README.md",
    "codex-rs/hepta-authbus-p1-2-qualification/migrations/0001_authbus_p1_2.sql",
    "codex-rs/hepta-authbus-p1-2-qualification/src/lib.rs",
    "codex-rs/hepta-authbus-p1-2-qualification/src/model.rs",
    "codex-rs/hepta-authbus-p1-2-qualification/src/store.rs",
    "codex-rs/hepta-authbus-p1-2-qualification/tests/p1_2.rs",
    "docs/hepta-vnext/authbus/AUTHBUS_P1_2_DEVELOPMENT_PLAN_2026-08-28.md",
    "docs/hepta-vnext/authbus/AUTHBUS_P1_2_IMPLEMENTATION_STATUS_2026-08-28.json",
    "docs/hepta-vnext/authbus/AUTHBUS_P1_2_IMPLEMENTATION_STATUS_2026-08-28.md",
    "scripts/verify-authbus-p1-2.py",
}


def replace_once(source: str, before: str, after: str, label: str) -> str:
    count = source.count(before)
    if count != 1:
        raise SystemExit(f"expected exactly one {label}, found {count}")
    return source.replace(before, after)


def materialize_reviewed_archive() -> None:
    present_chunks = sorted(
        Path("scripts").glob("authbus-p1-2-source-seed.part-*.b64")
    )
    if present_chunks != CHUNK_PATHS:
        raise SystemExit(f"unexpected source archive chunk set: {present_chunks}")

    encoded = "".join(
        "".join(path.read_text(encoding="ascii").split())
        for path in CHUNK_PATHS
    )
    archive = base64.b64decode(encoded, validate=True)
    actual_sha256 = hashlib.sha256(archive).hexdigest()
    if actual_sha256 != ARCHIVE_SHA256:
        raise SystemExit(f"source archive digest mismatch: {actual_sha256}")

    with tarfile.open(fileobj=io.BytesIO(archive), mode="r:gz") as bundle:
        members = bundle.getmembers()
        names = {member.name for member in members}
        if names != EXPECTED_ARCHIVE_PATHS or len(members) != len(
            EXPECTED_ARCHIVE_PATHS
        ):
            raise SystemExit(
                f"unexpected archive paths: {sorted(names ^ EXPECTED_ARCHIVE_PATHS)}"
            )

        for member in members:
            path = PurePosixPath(member.name)
            if (
                path.is_absolute()
                or ".." in path.parts
                or not member.isfile()
                or member.size > 512_000
            ):
                raise SystemExit(f"unsafe archive member: {member.name}")

            destination = Path(member.name)
            if destination.exists():
                raise SystemExit(f"archive destination already exists: {member.name}")
            source = bundle.extractfile(member)
            if source is None:
                raise SystemExit(f"missing archive bytes: {member.name}")
            destination.parent.mkdir(parents=True, exist_ok=True)
            with destination.open("wb") as output:
                shutil.copyfileobj(source, output)
            destination.chmod(member.mode & 0o777)


def remediate_source() -> None:
    cargo_path = CRATE_ROOT / "Cargo.toml"
    store_path = CRATE_ROOT / "src/store.rs"
    test_path = CRATE_ROOT / "tests/p1_2.rs"
    verifier_path = Path("scripts/verify-authbus-p1-2.py")

    cargo = cargo_path.read_text(encoding="utf-8")
    cargo = replace_once(
        cargo,
        '    "dep:codex-hepta-contracts",\n    "dep:serde",\n',
        '    "dep:codex-hepta-contracts",\n'
        '    "dep:codex-state",\n'
        '    "dep:codex-utils-absolute-path",\n'
        '    "dep:serde",\n',
        "P1.2 feature dependency anchor",
    )
    cargo = replace_once(
        cargo,
        'codex-hepta-contracts = { path = "../hepta-contracts", optional = true }\n'
        'serde = { version = "1.0.228", features = ["derive"], optional = true }\n',
        'codex-hepta-contracts = { path = "../hepta-contracts", optional = true }\n'
        'codex-state = { path = "../state", optional = true }\n'
        'codex-utils-absolute-path = { path = "../utils/absolute-path", optional = true }\n'
        'serde = { version = "1.0.228", features = ["derive"], optional = true }\n',
        "P1.2 dependency table anchor",
    )

    store = store_path.read_text(encoding="utf-8")
    for removal in (
        "use std::time::Duration;\n",
        "use serde::Deserialize;\n",
        "use sqlx::sqlite::SqliteConnectOptions;\n",
        "use sqlx::sqlite::SqliteJournalMode;\n",
        "use sqlx::sqlite::SqlitePoolOptions;\n",
        "use sqlx::sqlite::SqliteSynchronous;\n",
    ):
        store = replace_once(store, removal, "", f"import {removal.strip()}")

    store = replace_once(
        store,
        "use codex_hepta_contracts::Sha256Digest;\n",
        "use codex_hepta_contracts::Sha256Digest;\n"
        "use codex_state::SqliteConfig;\n"
        "use codex_utils_absolute_path::AbsolutePathBuf;\n",
        "canonical SQLite import anchor",
    )

    digest_patterns = (
        re.compile(
            r"(?P<expr>(?:(?:[A-Za-z_][A-Za-z0-9_]*)\.)*"
            r"[A-Za-z_][A-Za-z0-9_]*_sha256)\.to_string\(\)"
        ),
        re.compile(
            r"(?P<expr>(?:(?:[A-Za-z_][A-Za-z0-9_]*)\.)*"
            r"(?:[A-Za-z_][A-Za-z0-9_]*_digest|digest)\(\)\?)"
            r"\.to_string\(\)"
        ),
    )
    digest_repairs = 0
    for pattern in digest_patterns:
        store, count = pattern.subn(
            lambda match: f"{match.group('expr')}.as_str().to_owned()",
            store,
        )
        digest_repairs += count
    if digest_repairs != 29:
        raise SystemExit(
            f"expected exactly 29 Sha256Digest text repairs, got {digest_repairs}"
        )

    start_marker = "        let database_path = root.join(DATABASE_FILE);\n"
    post_marker = '        sqlx::query("PRAGMA secure_delete = ON")\n'
    if store.count(start_marker) != 1:
        raise SystemExit("expected exactly one SQLite pool start anchor")
    if store.count(post_marker) != 1:
        raise SystemExit("expected exactly one secure-delete post anchor")
    start = store.index(start_marker)
    end = store.index(post_marker, start)
    pool_block = store[start:end]
    if len(pool_block.encode("utf-8")) > 2_048:
        raise SystemExit("direct SQLite pool block exceeds bounded size")
    required_fragments = (
        "let database_path = root.join(DATABASE_FILE);",
        "let options = SqliteConnectOptions::new()",
        ".filename(&database_path)",
        ".create_if_missing(true)",
        ".journal_mode(SqliteJournalMode::Wal)",
        ".synchronous(SqliteSynchronous::Full)",
        ".foreign_keys(true)",
        ".busy_timeout(Duration::from_secs(5));",
        "let pool = SqlitePoolOptions::new()",
        ".max_connections(1)",
        ".connect_with(options)",
        ".map_err(map_sqlx)?;",
    )
    for fragment in required_fragments:
        count = pool_block.count(fragment)
        if count != 1:
            raise SystemExit(
                f"direct SQLite pool block fragment {fragment!r} count is {count}"
            )
    if pool_block.count(".await") != 1 or len(pool_block.splitlines()) not in range(
        12, 16
    ):
        raise SystemExit("direct SQLite pool block shape drifted")

    shim_block = (
        "        let absolute_root = AbsolutePathBuf::try_from(root.clone())\n"
        "            .map_err(|_| P12Error::StorageUnavailable)?;\n"
        "        let database_path = root.join(DATABASE_FILE);\n"
        "        let pool = SqliteConfig::from_sqlite_home(absolute_root)\n"
        "            .open_durable_evidence_pool(&database_path)\n"
        "            .await\n"
        "            .map_err(map_sqlx)?;\n\n"
    )
    store = store[:start] + shim_block + store[end:]

    suspicious = [
        (line_number, line)
        for line_number, line in enumerate(store.splitlines(), start=1)
        if ".to_string()" in line
        and ("_sha256" in line or "digest()?" in line)
    ]
    if suspicious:
        raise SystemExit(f"unrepaired digest text conversions: {suspicious}")
    for forbidden in (
        "connect_with",
        "SqlitePoolOptions",
        "SqliteConnectOptions",
        "SqliteJournalMode",
        "SqliteSynchronous",
        "Duration::from_secs(5)",
    ):
        if forbidden in store:
            raise SystemExit(f"direct SQLite construction survived: {forbidden}")
    if store.count("open_durable_evidence_pool") != 1:
        raise SystemExit("canonical durable SQLite shim count drifted")

    tests = test_path.read_text(encoding="utf-8")
    tests = replace_once(
        tests,
        '        "nonce-two",\n        NOW + 300,\n',
        '        "nonce-two",\n'
        "        // `issued_at` is `NOW - 2`; this exercises the exact 300-second TTL limit.\n"
        "        NOW + 298,\n",
        "nonce-two TTL fixture",
    )

    verifier = verifier_path.read_text(encoding="utf-8")
    verifier = replace_once(
        verifier,
        '''    store = text(CRATE / "src/store.rs")
    for marker in [
        "SqliteJournalMode::Wal",
        "SqliteSynchronous::Full",
        "PRAGMA secure_delete = ON",
        "PRAGMA trusted_schema = OFF",
        ".max_connections(1)",
        "ensure_current_writer",
        "p12_terminal_tombstones",
        "collect_garbage",
        "verify_integrity",
        "PRAGMA quick_check",
        "StorageUnavailableBeforeCommit",
    ]:
        require(marker in store, f"missing durable store invariant: {marker}")
    require("require_durable_key" in store, "durable key-purpose/epoch enforcement missing")
    require("expected_revision" in store, "GC CAS binding missing")
''',
        '''    store = text(CRATE / "src/store.rs")
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
''',
        "canonical SQLite source-verifier block",
    )

    cargo_path.write_text(cargo, encoding="utf-8")
    store_path.write_text(store, encoding="utf-8")
    test_path.write_text(tests, encoding="utf-8")
    verifier_path.write_text(verifier, encoding="utf-8")
    print(f"applied_sha256_digest_text_repairs={digest_repairs}")
    print("applied_exact_ttl_boundary_repairs=1")
    print("applied_canonical_sqlite_shim_repairs=1")
    print("applied_source_verifier_shim_repairs=1")


def main() -> None:
    materialize_reviewed_archive()
    remediate_source()


if __name__ == "__main__":
    main()

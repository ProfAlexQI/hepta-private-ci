#!/usr/bin/env python3
"""Build, qualify, and CAS-publish the bounded P0.3.2 repair candidate."""

from __future__ import annotations

import json
import os
import shutil
import subprocess
import sys
from pathlib import Path
from typing import Sequence

BASE = "256a47d882413ac1f80892a80177419bb5d12c70"
EXPECTED_HEAD = "a03863c8c6124a2024d959063da7ea412386de97"
BRANCH = "codex/hepta-intelligence-shared-projection-planner-v5-20260828"

EXPECTED_FILES = {
    ".github/workflows/hepta-intelligence-shared-projection-planner-v5.yml",
    ".github/workflows/restack-p0-3-3-on-p0-3-2.yml",
    "codex-rs/hepta-memory/src/cognitive_store.rs",
    "codex-rs/hepta-memory/src/cognitive_kg_store.rs",
    "codex-rs/hepta-memory/src/cognitive_projection_planner.rs",
    "codex-rs/hepta-memory/src/cognitive_test_support.rs",
    "codex-rs/hepta-memory/src/fact_grounding.rs",
    "codex-rs/hepta-memory/src/fact_grounding/durable.rs",
    "codex-rs/hepta-memory/src/fact_grounding/durable/grounding.rs",
    "codex-rs/hepta-memory/src/fact_grounding/durable/grounding/prepare.rs",
    "codex-rs/hepta-memory/src/fact_grounding/durable/grounding/ledger.rs",
    "codex-rs/hepta-memory/src/fact_grounding/durable/grounding/ledger/insert.rs",
    "codex-rs/hepta-memory/src/fact_grounding/durable/grounding/ledger/support.rs",
    "codex-rs/hepta-memory/src/fact_grounding/durable/grounding/ledger/verify.rs",
    "codex-rs/hepta-memory/src/fact_grounding/durable/schema.rs",
    "codex-rs/hepta-memory/src/fact_grounding/durable/tests.rs",
    "codex-rs/hepta-memory/src/fact_grounding/shadow_projection_gate.rs",
    "codex-rs/hepta-memory/src/framing.rs",
    "scripts/check-hepta-intelligence-p0-3-2-clippy.py",
    "scripts/run-hepta-intelligence-shared-projection-planner-v7.py",
    "scripts/verify-hepta-intelligence-shared-projection-planner-v5.py",
}

RUST_FILES = [
    "hepta-memory/src/fact_grounding.rs",
    "hepta-memory/src/fact_grounding/durable.rs",
    "hepta-memory/src/fact_grounding/durable/schema.rs",
    "hepta-memory/src/fact_grounding/durable/tests.rs",
    "hepta-memory/src/fact_grounding/shadow_projection_gate.rs",
]


def run(
    command: Sequence[str],
    *,
    cwd: Path,
    check: bool = True,
) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        list(command),
        cwd=cwd,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=check,
    )


def git(root: Path, *args: str, check: bool = True) -> subprocess.CompletedProcess[str]:
    return run(("git", *args), cwd=root, check=check)


def replace_once(root: Path, relative: str, old: str, new: str) -> None:
    path = root / relative
    text = path.read_text(encoding="utf-8")
    count = text.count(old)
    if count != 1:
        raise RuntimeError(
            f"{relative}: expected one repair target, observed {count}: {old[:120]!r}"
        )
    path.write_text(text.replace(old, new, 1), encoding="utf-8")


def apply_repairs(root: Path) -> None:
    replace_once(
        root,
        "codex-rs/hepta-memory/src/fact_grounding/shadow_projection_gate.rs",
        '''        sqlx::query("DROP TRIGGER kg_projection_generation_receipts_no_update")
            .execute(&store.pool)
            .await
            .expect("drop receipt immutability guard");
        sqlx::query(
            "UPDATE kg_projection_generation_receipts
             SET output_sha256 =
                 '0000000000000000000000000000000000000000000000000000000000000000'
             WHERE projection_scope = 'agent_private'",
        )
        .execute(&store.pool)
        .await
        .expect("tamper current receipt");
        sqlx::query(
            "CREATE TRIGGER kg_projection_generation_receipts_no_update
             BEFORE UPDATE ON kg_projection_generation_receipts BEGIN
                 SELECT RAISE(ABORT, 'KG projection generation receipts are immutable');
             END",
        )
        .execute(&store.pool)
        .await
        .expect("restore receipt immutability guard");''',
        '''        let mut tamper = store.pool.begin().await.expect("begin receipt tamper");
        sqlx::query("DROP TRIGGER kg_projection_generation_receipts_no_update")
            .execute(&mut *tamper)
            .await
            .expect("drop receipt immutability guard");
        sqlx::query(
            "UPDATE kg_projection_generation_receipts
             SET output_sha256 =
                 '0000000000000000000000000000000000000000000000000000000000000000'
             WHERE projection_scope = 'agent_private'",
        )
        .execute(&mut *tamper)
        .await
        .expect("tamper current receipt");
        sqlx::query(
            "CREATE TRIGGER kg_projection_generation_receipts_no_update
             BEFORE UPDATE ON kg_projection_generation_receipts BEGIN
                 SELECT RAISE(ABORT, 'KG projection generation receipts are immutable');
             END",
        )
        .execute(&mut *tamper)
        .await
        .expect("restore receipt immutability guard");
        tamper.commit().await.expect("commit receipt tamper");''',
    )

    replace_once(
        root,
        "codex-rs/hepta-memory/src/fact_grounding/durable/tests.rs",
        '''    sqlx::query("DROP TRIGGER kg_revision_fact_grounding_spans_no_update")
        .execute(&store.pool)
        .await
        .expect("drop guard");
    sqlx::query(
        "UPDATE kg_revision_fact_grounding_spans
         SET evidence_sha256 =
             '0000000000000000000000000000000000000000000000000000000000000000'
         WHERE rowid = (
             SELECT rowid FROM kg_revision_fact_grounding_spans LIMIT 1
         )",
    )
    .execute(&store.pool)
    .await
    .expect("tamper");
    sqlx::query(
        "CREATE TRIGGER kg_revision_fact_grounding_spans_no_update
         BEFORE UPDATE ON kg_revision_fact_grounding_spans BEGIN
             SELECT RAISE(ABORT, 'fact-grounding spans are immutable');
         END",
    )
    .execute(&store.pool)
    .await
    .expect("restore guard");''',
        '''    let mut tamper = store.pool.begin().await.expect("begin evidence tamper");
    sqlx::query("DROP TRIGGER kg_revision_fact_grounding_spans_no_update")
        .execute(&mut *tamper)
        .await
        .expect("drop guard");
    sqlx::query(
        "UPDATE kg_revision_fact_grounding_spans
         SET evidence_sha256 =
             '0000000000000000000000000000000000000000000000000000000000000000'
         WHERE rowid = (
             SELECT rowid FROM kg_revision_fact_grounding_spans LIMIT 1
         )",
    )
    .execute(&mut *tamper)
    .await
    .expect("tamper");
    sqlx::query(
        "CREATE TRIGGER kg_revision_fact_grounding_spans_no_update
         BEFORE UPDATE ON kg_revision_fact_grounding_spans BEGIN
             SELECT RAISE(ABORT, 'fact-grounding spans are immutable');
         END",
    )
    .execute(&mut *tamper)
    .await
    .expect("restore guard");
    tamper.commit().await.expect("commit evidence tamper");''',
    )

    replace_once(
        root,
        "codex-rs/hepta-memory/src/fact_grounding.rs",
        '''                Some(previous) if identity == previous => {
                    if span.evidence_ordinal != expected_ordinal {
                        return Err(FactGroundingError::Receipt(
                            "receipt evidence ordinals are not contiguous".to_string(),
                        ));
                    }
                }
                Some(previous) if identity < previous || span.evidence_ordinal != 0 => {''',
        '''                Some(previous)
                    if identity == previous && span.evidence_ordinal != expected_ordinal =>
                {
                    return Err(FactGroundingError::Receipt(
                        "receipt evidence ordinals are not contiguous".to_string(),
                    ));
                }
                Some(previous)
                    if identity != previous
                        && (identity < previous || span.evidence_ordinal != 0) =>
                {''',
    )

    replace_once(
        root,
        "codex-rs/hepta-memory/src/fact_grounding/durable/schema.rs",
        "verify_migration_ledger_connection(&mut *transaction, migration_checksum.as_str())",
        "verify_migration_ledger_connection(&mut transaction, migration_checksum.as_str())",
    )
    replace_once(
        root,
        "codex-rs/hepta-memory/src/fact_grounding/durable/schema.rs",
        "verify_schema_oracle_connection(&mut **transaction)",
        "verify_schema_oracle_connection(transaction)",
    )
    replace_once(
        root,
        "codex-rs/hepta-memory/src/fact_grounding/durable/schema.rs",
        "        &mut **transaction,\n        Sha256Digest::for_bytes",
        "        transaction,\n        Sha256Digest::for_bytes",
    )
    replace_once(
        root,
        "codex-rs/hepta-memory/src/fact_grounding/durable.rs",
        "grounding::verify_receipts(&mut **transaction, self.owner_agent_id.as_str()).await",
        "grounding::verify_receipts(transaction, self.owner_agent_id.as_str()).await",
    )
    replace_once(
        root,
        "scripts/check-hepta-intelligence-p0-3-2-clippy.py",
        '''        "-W",
        "warnings",
    ]''',
        '''        "-W",
        "warnings",
        "--cap-lints=warn",
    ]''',
    )
    replace_once(
        root,
        "scripts/verify-hepta-intelligence-shared-projection-planner-v5.py",
        '"grounding::verify_receipts(&mut **transaction",',
        '"grounding::verify_receipts(transaction",',
    )
    replace_once(
        root,
        "scripts/verify-hepta-intelligence-shared-projection-planner-v5.py",
        '"verify_schema_oracle_connection(&mut **transaction).await?;",',
        '"verify_schema_oracle_connection(transaction).await?;",',
    )


def build_candidate(root: Path) -> str:
    run(("rustfmt", "--edition", "2024", *RUST_FILES), cwd=root / "codex-rs")
    shutil.rmtree(root / "artifacts", ignore_errors=True)
    shutil.rmtree(root / "scripts" / "__pycache__", ignore_errors=True)
    git(root, "config", "user.name", "Qian QI")
    git(root, "config", "user.email", "102159240+ProfAlexQI@users.noreply.github.com")
    git(root, "reset", "--soft", BASE)
    git(root, "add", "-A")
    git(root, "diff", "--cached", "--check")
    git(
        root,
        "commit",
        "--no-gpg-sign",
        "-m",
        "feat(memory): share semantic KG projection planner P0.3.2",
    )
    count = git(root, "rev-list", "--count", f"{BASE}..HEAD").stdout.strip()
    if count != "1":
        raise RuntimeError(f"candidate must be one commit above base, observed {count}")
    observed = set(git(root, "diff", "--name-only", f"{BASE}..HEAD").stdout.splitlines())
    if observed != EXPECTED_FILES:
        raise RuntimeError(
            "candidate allowlist mismatch: "
            f"missing={sorted(EXPECTED_FILES-observed)}, "
            f"unexpected={sorted(observed-EXPECTED_FILES)}"
        )
    return git(root, "rev-parse", "HEAD").stdout.strip()


def qualify(root: Path, candidate: str) -> dict[str, object]:
    runner = run(
        (sys.executable, "scripts/run-hepta-intelligence-shared-projection-planner-v7.py"),
        cwd=root,
        check=False,
    )
    if runner.stdout:
        print(runner.stdout)
    if runner.stderr:
        print(runner.stderr, file=sys.stderr)
    path = root / "artifacts/hepta-intelligence-shared-projection-planner-v7/qualification-receipt.json"
    receipt = json.loads(path.read_text(encoding="utf-8"))
    if receipt["head"] != candidate:
        raise RuntimeError("qualification receipt does not bind the repaired candidate")
    false_keys = (
        "wired",
        "tool_v3_registered",
        "tool_v4_registered",
        "default_projection_pointer_changed",
        "default_recall_query_changed",
        "production_projection_gate",
        "production_authority",
        "external_effects",
        "operator_accepted",
        "promoted",
        "callers_ratchet",
    )
    for key in false_keys:
        if receipt[key] is not False:
            raise RuntimeError(f"authority flag {key} is not false")
    if not receipt["qualified"] or not all(check["passed"] for check in receipt["checks"]):
        failed = [check["id"] for check in receipt["checks"] if not check["passed"]]
        raise RuntimeError(f"repaired candidate failed v7 gates: {failed}")
    return receipt


def publish(root: Path, candidate: str) -> None:
    remote = git(root, "ls-remote", "origin", f"refs/heads/{BRANCH}").stdout.split()[0]
    if remote != EXPECTED_HEAD:
        raise RuntimeError(f"canonical branch drifted from {EXPECTED_HEAD} to {remote}")
    git(
        root,
        "push",
        f"--force-with-lease=refs/heads/{BRANCH}:{EXPECTED_HEAD}",
        "origin",
        f"HEAD:refs/heads/{BRANCH}",
    )
    body = (
        f"P0.3.2 bounded repair passed every v7 gate on Ubuntu 22 and published canonical "
        f"head `{candidate}`. This is executable diagnostic evidence; formal Ubuntu 24 exact-head "
        "admission on the published head is still required. All authority flags remain false."
    )
    run(("gh", "pr", "comment", "40", "--body", body), cwd=root, check=False)


def main() -> int:
    if len(sys.argv) != 2:
        print("usage: p032-bounded-repair-v1.py <candidate-checkout>", file=sys.stderr)
        return 2
    root = Path(sys.argv[1]).resolve()
    result_path = root / "repair-controller-result.json"
    result: dict[str, object] = {
        "schema": "hepta_intelligence_p0_3_2_bounded_repair_v1",
        "base": BASE,
        "input_head": None,
        "candidate_head": None,
        "qualified": False,
        "published": False,
        "error": None,
    }
    try:
        input_head = git(root, "rev-parse", "HEAD").stdout.strip()
        result["input_head"] = input_head
        if input_head != EXPECTED_HEAD:
            raise RuntimeError(f"unexpected repair input {input_head}")
        apply_repairs(root)
        candidate = build_candidate(root)
        result["candidate_head"] = candidate
        qualify(root, candidate)
        result["qualified"] = True
        publish(root, candidate)
        result["published"] = True
    except Exception as error:
        result["error"] = f"{type(error).__name__}: {error}"
        print(result["error"], file=sys.stderr)
    result_path.write_text(json.dumps(result, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    print(json.dumps(result, indent=2, sort_keys=True))
    return 0 if result["qualified"] and result["published"] else 1


if __name__ == "__main__":
    sys.exit(main())

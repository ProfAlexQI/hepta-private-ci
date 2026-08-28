#!/usr/bin/env python3
from __future__ import annotations

from pathlib import Path

STORE_PATH = Path("codex-rs/hepta-authbus-p1-2-qualification/src/store.rs")


def replace_exact(
    source: str,
    before: str,
    after: str,
    expected_count: int,
    label: str,
) -> str:
    count = source.count(before)
    if count != expected_count:
        raise SystemExit(f"expected {expected_count} {label} anchors, found {count}")
    return source.replace(before, after)


def main() -> None:
    source = STORE_PATH.read_text(encoding="utf-8")

    helper_anchor = """    fn maybe_fail(&self, failpoint: P12Failpoint) -> P12Result<()> {
        if self.failpoints.load(Ordering::SeqCst) & failpoint.bit() == 0 {
            return Ok(());
        }
        if failpoint == P12Failpoint::StorageUnavailableBeforeCommit {
            Err(P12Error::StorageUnavailable)
        } else {
            Err(P12Error::InjectedFailure)
        }
    }

    async fn ensure_current_writer(
"""
    helper_replacement = """    fn maybe_fail(&self, failpoint: P12Failpoint) -> P12Result<()> {
        if self.failpoints.load(Ordering::SeqCst) & failpoint.bit() == 0 {
            return Ok(());
        }
        if failpoint == P12Failpoint::StorageUnavailableBeforeCommit {
            Err(P12Error::StorageUnavailable)
        } else {
            Err(P12Error::InjectedFailure)
        }
    }

    async fn commit_or_rollback(
        &self,
        transaction: Transaction<'_, Sqlite>,
        failpoint: P12Failpoint,
    ) -> P12Result<()> {
        let failure = self
            .maybe_fail(failpoint)
            .and_then(|_| self.maybe_fail(P12Failpoint::StorageUnavailableBeforeCommit))
            .err();
        if let Some(error) = failure {
            transaction.rollback().await.map_err(map_sqlx)?;
            return Err(error);
        }
        transaction.commit().await.map_err(map_sqlx)
    }

    async fn ensure_current_writer(
"""
    source = replace_exact(
        source,
        helper_anchor,
        helper_replacement,
        1,
        "commit-or-rollback helper",
    )

    failpoint_counts = {
        "KeyBeforeCommit": 2,
        "NonceBeforeCommit": 1,
        "OperationBeforeCommit": 1,
        "StatusBeforeCommit": 1,
        "ManualBeforeCommit": 1,
        "GcBeforeCommit": 1,
    }
    repaired = 0
    for variant, expected_count in failpoint_counts.items():
        before = f"""        self.maybe_fail(P12Failpoint::{variant})?;
        self.maybe_fail(P12Failpoint::StorageUnavailableBeforeCommit)?;
        transaction.commit().await.map_err(map_sqlx)?;
"""
        after = f"""        self.commit_or_rollback(transaction, P12Failpoint::{variant})
            .await?;
"""
        source = replace_exact(
            source,
            before,
            after,
            expected_count,
            f"{variant} explicit rollback",
        )
        repaired += expected_count

    if source.count("async fn commit_or_rollback(") != 1:
        raise SystemExit("commit-or-rollback helper count drifted")
    if source.count(".commit_or_rollback(transaction, P12Failpoint::") != repaired:
        raise SystemExit("explicit rollback call count drifted")
    if "self.maybe_fail(P12Failpoint::StorageUnavailableBeforeCommit)?;" in source:
        raise SystemExit("implicit storage-failpoint rollback survived")

    STORE_PATH.write_text(source, encoding="utf-8")
    print(f"applied_explicit_precommit_rollbacks={repaired}")
    print("applied_commit_or_rollback_helper=1")


if __name__ == "__main__":
    main()

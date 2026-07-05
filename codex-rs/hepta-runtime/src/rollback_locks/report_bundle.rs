use crate::RollbackGroupAttempt;
use crate::WriteGroupLock;
use crate::WriteTargetLock;

use super::WriteGroupLockReport;
use super::WriteLockPruneReport;
use super::WriteLockReport;
use super::WriteLockSummaryReport;
use super::WriteTargetLockReport;

pub(crate) fn build_write_target_lock_report(
    lock: WriteTargetLock,
    attempt: Option<&RollbackGroupAttempt>,
) -> WriteTargetLockReport {
    WriteTargetLockReport {
        session_id: lock.session_id,
        target_path: lock.target_path,
        owner_kind: lock.owner_kind,
        owner_id: lock.owner_id,
        rollback_group_id: lock.rollback_group_id,
        rollback_attempt_id: lock.rollback_attempt_id,
        rollback_status: attempt.as_ref().map(|attempt| attempt.status.clone()),
        pending_transaction_ids: attempt
            .as_ref()
            .map(|attempt| attempt.pending_transaction_ids.clone())
            .unwrap_or_default(),
        failed_transaction_id: attempt.and_then(|attempt| attempt.failed_transaction_id.clone()),
        locked_at_unix_ms: lock.locked_at_unix_ms,
        lease_expires_at_unix_ms: lock.lease_expires_at_unix_ms,
    }
}

pub(crate) fn build_write_group_lock_report(
    lock: WriteGroupLock,
    attempt: Option<&RollbackGroupAttempt>,
) -> WriteGroupLockReport {
    WriteGroupLockReport {
        session_id: lock.session_id,
        group_id: lock.group_id,
        owner_kind: lock.owner_kind,
        owner_id: lock.owner_id,
        rollback_attempt_id: lock.rollback_attempt_id,
        rollback_status: attempt.as_ref().map(|attempt| attempt.status.clone()),
        pending_transaction_ids: attempt
            .as_ref()
            .map(|attempt| attempt.pending_transaction_ids.clone())
            .unwrap_or_default(),
        failed_transaction_id: attempt.and_then(|attempt| attempt.failed_transaction_id.clone()),
        locked_at_unix_ms: lock.locked_at_unix_ms,
        lease_expires_at_unix_ms: lock.lease_expires_at_unix_ms,
    }
}

pub(crate) fn build_write_lock_report(
    schema_version: u32,
    mut target_locks: Vec<WriteTargetLockReport>,
    mut group_locks: Vec<WriteGroupLockReport>,
) -> WriteLockReport {
    target_locks.sort_by(|left, right| right.locked_at_unix_ms.cmp(&left.locked_at_unix_ms));
    group_locks.sort_by(|left, right| right.locked_at_unix_ms.cmp(&left.locked_at_unix_ms));
    let summary = WriteLockSummaryReport {
        total_target_locks: target_locks.len(),
        total_group_locks: group_locks.len(),
        rollback_bound_target_locks: target_locks
            .iter()
            .filter(|lock| target_lock_is_rollback_bound(lock))
            .count(),
        rollback_bound_group_locks: group_locks
            .iter()
            .filter(|lock| group_lock_is_rollback_bound(lock))
            .count(),
        orphaned_target_locks: target_locks
            .iter()
            .filter(|lock| target_lock_is_orphaned(lock))
            .count(),
        orphaned_group_locks: group_locks
            .iter()
            .filter(|lock| group_lock_is_orphaned(lock))
            .count(),
    };
    WriteLockReport {
        schema_version,
        summary,
        target_locks,
        group_locks,
    }
}

pub(crate) fn build_write_lock_prune_report(
    now_unix_ms: u64,
    before_target_locks: usize,
    before_group_locks: usize,
    remaining_target_locks: usize,
    remaining_group_locks: usize,
) -> WriteLockPruneReport {
    WriteLockPruneReport {
        now_unix_ms,
        pruned_target_locks: before_target_locks.saturating_sub(remaining_target_locks),
        pruned_group_locks: before_group_locks.saturating_sub(remaining_group_locks),
        remaining_target_locks,
        remaining_group_locks,
    }
}

pub(super) fn target_lock_is_orphaned(lock: &WriteTargetLockReport) -> bool {
    target_lock_is_rollback_bound(lock)
        && (lock.rollback_attempt_id.is_none() || lock.rollback_status.is_none())
}

pub(super) fn group_lock_is_orphaned(lock: &WriteGroupLockReport) -> bool {
    group_lock_is_rollback_bound(lock)
        && (lock.rollback_attempt_id.is_none() || lock.rollback_status.is_none())
}

fn target_lock_is_rollback_bound(lock: &WriteTargetLockReport) -> bool {
    lock.owner_kind == "rollback_group"
        || lock.rollback_group_id.is_some()
        || lock.rollback_attempt_id.is_some()
}

fn group_lock_is_rollback_bound(lock: &WriteGroupLockReport) -> bool {
    lock.owner_kind == "rollback_group" || lock.rollback_attempt_id.is_some()
}

#[cfg(test)]
mod tests {
    use crate::RollbackGroupAttemptStatus;

    use super::*;

    #[test]
    fn build_write_lock_report_summarizes_rollback_bound_and_orphaned_entries() {
        let report = build_write_lock_report(
            7,
            vec![
                WriteTargetLockReport {
                    session_id: "session-main".into(),
                    target_path: "/tmp/one".into(),
                    owner_kind: "rollback_group".into(),
                    owner_id: "grp-1".into(),
                    rollback_group_id: Some("grp-1".into()),
                    rollback_attempt_id: Some("rbk-1".into()),
                    rollback_status: Some(RollbackGroupAttemptStatus::PartialFailed),
                    pending_transaction_ids: vec!["txn-1".into()],
                    failed_transaction_id: Some("txn-1".into()),
                    locked_at_unix_ms: 5,
                    lease_expires_at_unix_ms: 50,
                },
                WriteTargetLockReport {
                    session_id: "session-main".into(),
                    target_path: "/tmp/two".into(),
                    owner_kind: "rollback_group".into(),
                    owner_id: "grp-1".into(),
                    rollback_group_id: Some("grp-1".into()),
                    rollback_attempt_id: None,
                    rollback_status: None,
                    pending_transaction_ids: vec![],
                    failed_transaction_id: None,
                    locked_at_unix_ms: 10,
                    lease_expires_at_unix_ms: 50,
                },
            ],
            vec![WriteGroupLockReport {
                session_id: "session-main".into(),
                group_id: "grp-1".into(),
                owner_kind: "rollback_group".into(),
                owner_id: "rbk-1".into(),
                rollback_attempt_id: None,
                rollback_status: None,
                pending_transaction_ids: vec![],
                failed_transaction_id: None,
                locked_at_unix_ms: 1,
                lease_expires_at_unix_ms: 50,
            }],
        );

        assert_eq!(report.schema_version, 7);
        assert_eq!(report.summary.total_target_locks, 2);
        assert_eq!(report.summary.total_group_locks, 1);
        assert_eq!(report.summary.rollback_bound_target_locks, 2);
        assert_eq!(report.summary.rollback_bound_group_locks, 1);
        assert_eq!(report.summary.orphaned_target_locks, 1);
        assert_eq!(report.summary.orphaned_group_locks, 1);
        assert_eq!(report.target_locks[0].target_path, "/tmp/two");
    }
}

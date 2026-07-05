use super::RollbackGroupLockDiagnostics;
use super::WriteGroupLockReport;
use super::WriteLockReport;
use super::WriteTargetLockReport;
use super::report_bundle;

pub(crate) fn collect_rollback_group_lock_diagnostics(
    session_id: &str,
    group_id: &str,
    latest_attempt_id: Option<&str>,
    locks: &WriteLockReport,
) -> RollbackGroupLockDiagnostics {
    RollbackGroupLockScope::from_report(session_id, group_id, locks)
        .into_diagnostics(latest_attempt_id)
}

struct RollbackGroupLockScope<'a> {
    group_lock: Option<&'a WriteGroupLockReport>,
    target_locks: Vec<&'a WriteTargetLockReport>,
}

impl<'a> RollbackGroupLockScope<'a> {
    fn from_report(session_id: &str, group_id: &str, locks: &'a WriteLockReport) -> Self {
        let group_lock = locks
            .group_locks
            .iter()
            .find(|lock| lock.session_id == session_id && lock.group_id == group_id);
        let target_locks = locks
            .target_locks
            .iter()
            .filter(|lock| {
                lock.session_id == session_id
                    && lock
                        .rollback_group_id
                        .as_deref()
                        .map(|rollback_group_id| rollback_group_id == group_id)
                        .unwrap_or(false)
            })
            .collect();

        Self {
            group_lock,
            target_locks,
        }
    }

    fn into_diagnostics(self, latest_attempt_id: Option<&str>) -> RollbackGroupLockDiagnostics {
        RollbackGroupLockDiagnostics {
            group_locked: self.group_lock.is_some(),
            group_lock_attempt_id: self.group_lock_attempt_id(),
            target_lock_count: self.target_locks.len(),
            orphaned_lock_count: self.orphaned_lock_count(),
            latest_attempt_owns_lock_set: self.latest_attempt_owns_lock_set(latest_attempt_id),
        }
    }

    fn group_lock_attempt_id(&self) -> Option<String> {
        self.group_lock
            .and_then(|lock| lock.rollback_attempt_id.clone())
    }

    fn orphaned_lock_count(&self) -> usize {
        usize::from(
            self.group_lock
                .map(report_bundle::group_lock_is_orphaned)
                .unwrap_or(false),
        ) + self
            .target_locks
            .iter()
            .filter(|lock| report_bundle::target_lock_is_orphaned(lock))
            .count()
    }

    fn latest_attempt_owns_lock_set(&self, latest_attempt_id: Option<&str>) -> bool {
        self.latest_attempt_owns_group_lock(latest_attempt_id)
            && (self.target_locks.is_empty()
                || self.latest_attempt_owns_target_locks(latest_attempt_id))
    }

    fn latest_attempt_owns_group_lock(&self, latest_attempt_id: Option<&str>) -> bool {
        latest_attempt_id
            .map(|latest_attempt_id| {
                self.group_lock
                    .and_then(|lock| lock.rollback_attempt_id.as_deref())
                    .map(|attempt_id| attempt_id == latest_attempt_id)
                    .unwrap_or(false)
            })
            .unwrap_or(false)
    }

    fn latest_attempt_owns_target_locks(&self, latest_attempt_id: Option<&str>) -> bool {
        latest_attempt_id
            .map(|latest_attempt_id| {
                !self.target_locks.is_empty()
                    && self.target_locks.iter().all(|lock| {
                        lock.rollback_attempt_id
                            .as_deref()
                            .map(|attempt_id| attempt_id == latest_attempt_id)
                            .unwrap_or(false)
                    })
            })
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use crate::RollbackGroupAttemptStatus;

    use super::*;

    fn sample_locks() -> WriteLockReport {
        WriteLockReport {
            schema_version: 1,
            summary: super::super::WriteLockSummaryReport {
                total_target_locks: 2,
                total_group_locks: 1,
                rollback_bound_target_locks: 2,
                rollback_bound_group_locks: 1,
                orphaned_target_locks: 1,
                orphaned_group_locks: 0,
            },
            target_locks: vec![
                WriteTargetLockReport {
                    session_id: "session-main".into(),
                    target_path: "/tmp/a".into(),
                    owner_kind: "rollback_group".into(),
                    owner_id: "grp-1".into(),
                    rollback_group_id: Some("grp-1".into()),
                    rollback_attempt_id: Some("rbk-current".into()),
                    rollback_status: Some(RollbackGroupAttemptStatus::PartialFailed),
                    pending_transaction_ids: vec!["txn-a".into()],
                    failed_transaction_id: Some("txn-a".into()),
                    locked_at_unix_ms: 2,
                    lease_expires_at_unix_ms: 20,
                },
                WriteTargetLockReport {
                    session_id: "session-main".into(),
                    target_path: "/tmp/b".into(),
                    owner_kind: "rollback_group".into(),
                    owner_id: "grp-1".into(),
                    rollback_group_id: Some("grp-1".into()),
                    rollback_attempt_id: None,
                    rollback_status: None,
                    pending_transaction_ids: vec![],
                    failed_transaction_id: None,
                    locked_at_unix_ms: 1,
                    lease_expires_at_unix_ms: 20,
                },
            ],
            group_locks: vec![WriteGroupLockReport {
                session_id: "session-main".into(),
                group_id: "grp-1".into(),
                owner_kind: "rollback_group".into(),
                owner_id: "rbk-current".into(),
                rollback_attempt_id: Some("rbk-current".into()),
                rollback_status: Some(RollbackGroupAttemptStatus::PartialFailed),
                pending_transaction_ids: vec!["txn-a".into()],
                failed_transaction_id: Some("txn-a".into()),
                locked_at_unix_ms: 3,
                lease_expires_at_unix_ms: 20,
            }],
        }
    }

    #[test]
    fn collect_rollback_group_lock_diagnostics_requires_latest_attempt_to_own_lock_set() {
        let diagnostics = collect_rollback_group_lock_diagnostics(
            "session-main",
            "grp-1",
            Some("rbk-current"),
            &sample_locks(),
        );

        assert!(diagnostics.group_locked);
        assert_eq!(
            diagnostics.group_lock_attempt_id.as_deref(),
            Some("rbk-current")
        );
        assert_eq!(diagnostics.target_lock_count, 2);
        assert_eq!(diagnostics.orphaned_lock_count, 1);
        assert!(!diagnostics.latest_attempt_owns_lock_set);
    }

    #[test]
    fn collect_rollback_group_lock_diagnostics_allows_group_only_ownership() {
        let diagnostics = collect_rollback_group_lock_diagnostics(
            "session-main",
            "grp-2",
            Some("rbk-group-only"),
            &WriteLockReport {
                schema_version: 1,
                summary: super::super::WriteLockSummaryReport {
                    total_target_locks: 0,
                    total_group_locks: 1,
                    rollback_bound_target_locks: 0,
                    rollback_bound_group_locks: 1,
                    orphaned_target_locks: 0,
                    orphaned_group_locks: 0,
                },
                target_locks: vec![],
                group_locks: vec![WriteGroupLockReport {
                    session_id: "session-main".into(),
                    group_id: "grp-2".into(),
                    owner_kind: "rollback_group".into(),
                    owner_id: "rbk-group-only".into(),
                    rollback_attempt_id: Some("rbk-group-only".into()),
                    rollback_status: Some(RollbackGroupAttemptStatus::Completed),
                    pending_transaction_ids: vec![],
                    failed_transaction_id: None,
                    locked_at_unix_ms: 4,
                    lease_expires_at_unix_ms: 40,
                }],
            },
        );

        assert!(diagnostics.group_locked);
        assert_eq!(diagnostics.target_lock_count, 0);
        assert_eq!(diagnostics.orphaned_lock_count, 0);
        assert!(diagnostics.latest_attempt_owns_lock_set);
    }
}

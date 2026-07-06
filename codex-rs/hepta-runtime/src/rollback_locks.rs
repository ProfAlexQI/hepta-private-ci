use serde::Serialize;

use crate::RollbackGroupAttemptStatus;

mod diagnostics;
mod report_bundle;

pub(crate) use diagnostics::collect_rollback_group_lock_diagnostics;
pub(crate) use report_bundle::build_write_group_lock_report;
pub(crate) use report_bundle::build_write_lock_prune_report;
pub(crate) use report_bundle::build_write_lock_report;
pub(crate) use report_bundle::build_write_target_lock_report;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RollbackGroupLockDiagnosticsReport {
    pub group_lock_attempt_id: Option<String>,
    pub target_lock_count: usize,
    pub orphaned_lock_count: usize,
    pub latest_attempt_owns_lock_set: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RollbackGroupLockDiagnostics {
    pub(crate) group_locked: bool,
    pub(crate) group_lock_attempt_id: Option<String>,
    pub(crate) target_lock_count: usize,
    pub(crate) orphaned_lock_count: usize,
    pub(crate) latest_attempt_owns_lock_set: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WriteTargetLockReport {
    pub session_id: String,
    pub target_path: String,
    pub owner_kind: String,
    pub owner_id: String,
    pub rollback_group_id: Option<String>,
    pub rollback_attempt_id: Option<String>,
    pub rollback_status: Option<RollbackGroupAttemptStatus>,
    pub pending_transaction_ids: Vec<String>,
    pub failed_transaction_id: Option<String>,
    pub locked_at_unix_ms: u64,
    pub lease_expires_at_unix_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WriteGroupLockReport {
    pub session_id: String,
    pub group_id: String,
    pub owner_kind: String,
    pub owner_id: String,
    pub rollback_attempt_id: Option<String>,
    pub rollback_status: Option<RollbackGroupAttemptStatus>,
    pub pending_transaction_ids: Vec<String>,
    pub failed_transaction_id: Option<String>,
    pub locked_at_unix_ms: u64,
    pub lease_expires_at_unix_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WriteLockReport {
    pub schema_version: u32,
    pub summary: WriteLockSummaryReport,
    pub target_locks: Vec<WriteTargetLockReport>,
    pub group_locks: Vec<WriteGroupLockReport>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WriteLockSummaryReport {
    pub total_target_locks: usize,
    pub total_group_locks: usize,
    pub rollback_bound_target_locks: usize,
    pub rollback_bound_group_locks: usize,
    pub orphaned_target_locks: usize,
    pub orphaned_group_locks: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WriteLockPruneReport {
    pub now_unix_ms: u64,
    pub pruned_target_locks: usize,
    pub pruned_group_locks: usize,
    pub remaining_target_locks: usize,
    pub remaining_group_locks: usize,
}

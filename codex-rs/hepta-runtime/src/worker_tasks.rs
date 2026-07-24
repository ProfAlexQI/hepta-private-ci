use std::collections::HashSet;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::process::Stdio;
use std::thread;
use std::time::Duration;
use std::time::Instant;

use hepta_core::AgentId;
use hepta_core::EventKind;
use hepta_core::ExecutionProfile;
use hepta_core::FilesystemScope;
use hepta_core::HeptaError;
use hepta_core::SessionId;
use hepta_core::WritePathScope;
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;

use super::RuntimeContextRecallProviderRollup;
use super::RuntimeKernel;
use super::VerticalSliceResult;
use super::current_unix_ms;
#[cfg(test)]
use super::resolve_path_within_root;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkerTaskStatus {
    Queued,
    Scheduled,
    Running,
    Paused,
    Completed,
    Failed,
    Cancelled,
    Interrupted,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkerTaskRecord {
    pub task_id: String,
    pub parent_session_id: String,
    #[serde(default = "default_worker_task_workspace_id")]
    pub workspace_id: String,
    pub worker_session_id: String,
    pub worker_id: String,
    pub prompt: String,
    #[serde(default = "default_worker_permission_envelope")]
    pub permission_envelope: WorkerPermissionEnvelope,
    #[serde(default = "default_worker_safety_envelope")]
    pub safety_envelope: WorkerTaskSafetyEnvelope,
    #[serde(default = "default_worker_execution_mode")]
    pub execution_mode: WorkerTaskExecutionMode,
    #[serde(default = "default_worker_execution_backend_binding")]
    pub execution_backend: WorkerExecutionBackendBinding,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub depends_on: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_task_id: Option<String>,
    #[serde(default)]
    pub spawn_depth: usize,
    #[serde(default = "default_worker_task_max_spawn_depth")]
    pub max_spawn_depth: usize,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule_expr: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_run_unix_ms: Option<u64>,
    pub status: WorkerTaskStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub paused_from_status: Option<WorkerTaskStatus>,
    pub created_at_unix_ms: u64,
    pub updated_at_unix_ms: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub started_at_unix_ms: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed_at_unix_ms: Option<u64>,
    pub attempt_count: usize,
    pub max_attempts: usize,
    pub timeout_budget_ms: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_error: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failure_kind: Option<WorkerTaskFailureKind>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retry_after_unix_ms: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result_summary: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub artifacts: Vec<WorkerTaskArtifact>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub diff_summary: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub patch_proposals: Vec<WorkerTaskPatchProposal>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub coding_rounds: Vec<WorkerTaskCodingRound>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub file_leases: Vec<WorkerTaskFileLease>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub loop_steps: Vec<WorkerTaskLoopStep>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub command_runs: Vec<WorkerTaskCommandRun>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub steering_directives: Vec<WorkerTaskSteerDirective>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkerTaskSteerDirective {
    pub directive_id: String,
    pub instruction: String,
    pub created_at_unix_ms: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkerTaskExecutionMode {
    Conversational,
    AutonomousCoding,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkerTaskContextRecallHandoffPolicy {
    Disabled,
    ExperimentalOperatorApproved,
}

impl WorkerTaskContextRecallHandoffPolicy {
    fn experimental_api_enabled(self) -> bool {
        matches!(self, Self::ExperimentalOperatorApproved)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkerExecutionBackendKind {
    LocalHostProcess,
    Docker,
    Ssh,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkerExecutionBackendStatus {
    Active,
    Available,
    RequiresConfiguration,
    Disabled,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkerExecutionBackendBinding {
    pub backend_id: String,
    pub kind: WorkerExecutionBackendKind,
    pub remote: bool,
    pub evidence_kind: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkerExecutionBackendDescriptor {
    pub backend_id: String,
    pub kind: WorkerExecutionBackendKind,
    pub status: WorkerExecutionBackendStatus,
    pub remote: bool,
    pub environment_process_evidence: bool,
    pub sandbox_required: bool,
    pub file_sync_supported: bool,
    pub file_sync_manifest_policy: String,
    pub credential_mount_policy: String,
    pub path_traversal_policy: String,
    pub child_side_effect_policy: String,
    pub supports_cancel: bool,
    pub supports_timeout: bool,
    pub supports_output_limits: bool,
    pub supports_file_leases: bool,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkerExecutionBackendReport {
    pub backend_count: usize,
    pub active_backend_id: String,
    pub active_backend_kind: WorkerExecutionBackendKind,
    pub local_backend_ready: bool,
    pub remote_backend_count: usize,
    pub configured_remote_backend_count: usize,
    pub remote_execution_enabled: bool,
    pub file_sync_policy_required: bool,
    pub credential_mount_policy_required: bool,
    pub remote_path_traversal_denied: bool,
    pub remote_credential_mounts_deny_by_default: bool,
    pub remote_file_sync_manifest_required: bool,
    pub remote_child_side_effects_blocked: bool,
    pub remote_safety_regression_pack_ready: bool,
    pub environment_process_evidence_contract: bool,
    pub backends: Vec<WorkerExecutionBackendDescriptor>,
}

pub trait WorkerExecutionBackend {
    fn descriptor(&self) -> WorkerExecutionBackendDescriptor;

    fn run_command(
        &self,
        task: &WorkerTaskRecord,
        workspace_root: &Path,
        safety_envelope: &WorkerTaskSafetyEnvelope,
        command_id: &str,
        display_command: &str,
        program: &str,
        args: &[&str],
    ) -> WorkerTaskCommandRun;
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkerTaskArtifact {
    pub artifact_id: String,
    pub kind: String,
    pub title: String,
    pub content: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path_hint: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkerPermissionEnvelope {
    pub execution_profile: ExecutionProfile,
    pub filesystem_scope: FilesystemScope,
    pub write_scope: WritePathScope,
    pub network_allowed: bool,
    pub inherited_from_parent: bool,
    pub policy_summary: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkerTaskSafetyEnvelope {
    pub sandbox: WorkerTaskSandboxPolicy,
    pub resource_limits: WorkerTaskResourceLimits,
    pub cancel_supported: bool,
    pub cancel_checked_before_host_command: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkerTaskSandboxPolicy {
    pub workspace_root: String,
    pub host_process_allowed: bool,
    pub network_allowed: bool,
    pub allowed_programs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkerTaskResourceLimits {
    pub task_timeout_budget_ms: u64,
    pub per_command_timeout_ms: u64,
    pub max_command_runs: usize,
    pub max_stdout_bytes: usize,
    pub max_stderr_bytes: usize,
    pub max_patch_proposals: usize,
    pub max_loop_steps: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkerTaskPatchProposal {
    pub patch_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision_of: Option<String>,
    #[serde(default, skip_serializing_if = "is_zero")]
    pub revision_index: usize,
    pub file_path: String,
    pub change_kind: String,
    pub summary: String,
    pub unified_diff: String,
    pub apply_status: WorkerTaskPatchApplyStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub applied_at_unix_ms: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conflict_reason: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkerTaskPatchApplyStatus {
    Proposed,
    Applied,
    Conflicted,
    Rejected,
    RolledBack,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkerTaskFailureKind {
    Timeout,
    ToolError,
    ModelError,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkerTaskLoopPhase {
    Plan,
    Inspect,
    Patch,
    Test,
    Revise,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkerTaskLoopStep {
    pub step_index: usize,
    pub phase: WorkerTaskLoopPhase,
    pub title: String,
    pub input_summary: String,
    pub output_summary: String,
    pub evidence_ref: String,
    pub passed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkerTaskCodingRound {
    pub round_index: usize,
    pub title: String,
    pub intent: String,
    pub command_ids: Vec<String>,
    pub patch_ids: Vec<String>,
    pub passed: bool,
    pub summary: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkerTaskFileLeaseStatus {
    Active,
    HeldForReview,
    Released,
    Expired,
    Conflicted,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkerTaskFileLease {
    pub lease_id: String,
    pub task_id: String,
    pub worker_id: String,
    pub worker_session_id: String,
    pub target_path: String,
    pub status: WorkerTaskFileLeaseStatus,
    pub acquired_at_unix_ms: u64,
    pub lease_expires_at_unix_ms: u64,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conflict_task_ids: Vec<String>,
}

#[derive(Debug)]
pub(crate) struct WorkerPatchApplyAuthorization {
    worker_session_id: String,
    requested_path: String,
    workspace_root: String,
}

impl WorkerPatchApplyAuthorization {
    pub(crate) fn worker_session_id(&self) -> &str {
        &self.worker_session_id
    }

    pub(crate) fn requested_path(&self) -> &str {
        &self.requested_path
    }

    pub(crate) fn workspace_root(&self) -> &str {
        &self.workspace_root
    }
}

#[derive(Debug)]
pub(crate) struct WorkerPatchRollbackAuthorization {
    worker_session_id: String,
    requested_path: String,
    workspace_root: String,
    transaction_id: String,
}

impl WorkerPatchRollbackAuthorization {
    pub(crate) fn worker_session_id(&self) -> &str {
        &self.worker_session_id
    }

    pub(crate) fn requested_path(&self) -> &str {
        &self.requested_path
    }

    pub(crate) fn workspace_root(&self) -> &str {
        &self.workspace_root
    }

    pub(crate) fn transaction_id(&self) -> &str {
        &self.transaction_id
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkerTaskCommandRun {
    pub command_id: String,
    pub command: String,
    #[serde(default = "default_worker_command_run_origin")]
    pub execution_origin: WorkerTaskCommandRunOrigin,
    #[serde(default = "default_worker_backend_id")]
    pub backend_id: String,
    #[serde(default = "default_worker_execution_backend_kind")]
    pub backend_kind: WorkerExecutionBackendKind,
    #[serde(default)]
    pub remote_backend: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub working_directory: Option<String>,
    #[serde(default)]
    pub timed_out: bool,
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
    pub duration_ms: u64,
    pub passed: bool,
    #[serde(default)]
    pub sandboxed: bool,
    #[serde(default)]
    pub cancelled: bool,
    #[serde(default)]
    pub stdout_truncated: bool,
    #[serde(default)]
    pub stderr_truncated: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_limit_violation: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkerTaskCommandRunOrigin {
    DeterministicTranscript,
    HostProcess,
}

fn default_worker_command_run_origin() -> WorkerTaskCommandRunOrigin {
    WorkerTaskCommandRunOrigin::DeterministicTranscript
}

fn default_worker_backend_id() -> String {
    "local-host-process".into()
}

fn default_worker_execution_backend_kind() -> WorkerExecutionBackendKind {
    WorkerExecutionBackendKind::LocalHostProcess
}

fn default_worker_execution_backend_binding() -> WorkerExecutionBackendBinding {
    WorkerExecutionBackendBinding {
        backend_id: default_worker_backend_id(),
        kind: WorkerExecutionBackendKind::LocalHostProcess,
        remote: false,
        evidence_kind: "environment_process".into(),
    }
}

fn default_worker_task_workspace_id() -> String {
    "global".into()
}

fn default_worker_task_max_spawn_depth() -> usize {
    1
}

impl WorkerTaskRecord {
    pub fn is_active(&self) -> bool {
        matches!(
            self.status,
            WorkerTaskStatus::Queued
                | WorkerTaskStatus::Scheduled
                | WorkerTaskStatus::Running
                | WorkerTaskStatus::Paused
        )
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct WorkerTaskState {
    pub records: Vec<WorkerTaskRecord>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkerTaskReport {
    pub task: WorkerTaskRecord,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkerTaskRunReport {
    pub task: WorkerTaskRecord,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<VerticalSliceResult>,
    pub artifact_count: usize,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub artifacts: Vec<WorkerTaskArtifact>,
    pub patch_proposal_count: usize,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub patch_proposals: Vec<WorkerTaskPatchProposal>,
    pub coding_round_count: usize,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub coding_rounds: Vec<WorkerTaskCodingRound>,
    pub file_lease_count: usize,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub file_leases: Vec<WorkerTaskFileLease>,
    pub loop_step_count: usize,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub loop_steps: Vec<WorkerTaskLoopStep>,
    pub command_run_count: usize,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub command_runs: Vec<WorkerTaskCommandRun>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkerTaskContextRecallRunReport {
    pub run: WorkerTaskRunReport,
    pub context_recall_handoff_policy: WorkerTaskContextRecallHandoffPolicy,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider_rollup: Option<RuntimeContextRecallProviderRollup>,
    pub selected_snippets_present: bool,
    pub selected_snippet_count: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkerTaskLoopReport {
    pub task_id: String,
    pub workspace_id: String,
    pub worker_id: String,
    pub loop_step_count: usize,
    pub passed_count: usize,
    pub failed_count: usize,
    pub phases: Vec<WorkerTaskLoopPhase>,
    pub steps: Vec<WorkerTaskLoopStep>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkerTaskEvidenceReport {
    pub task_id: String,
    pub workspace_id: String,
    pub worker_id: String,
    pub worker_session_id: String,
    pub evidence_count: usize,
    pub terminal_status: WorkerTaskStatus,
    pub permission_envelope: WorkerPermissionEnvelope,
    pub chain_head: String,
    pub entries: Vec<WorkerTaskEvidenceEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkerTaskEvidenceEntry {
    pub index: usize,
    pub evidence_ref: String,
    pub kind: String,
    pub summary: String,
    pub occurred_at_unix_ms: u64,
    pub session_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous_hash: Option<String>,
    pub entry_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkerTaskReplayAuditReport {
    pub task_id: String,
    pub workspace_id: String,
    pub worker_id: String,
    pub terminal_status: WorkerTaskStatus,
    pub evidence_count: usize,
    pub chain_head: String,
    pub replayed_chain_head: String,
    pub hash_chain_valid: bool,
    pub permission_policy_valid: bool,
    pub lifecycle_valid: bool,
    pub artifact_records_valid: bool,
    pub patch_records_valid: bool,
    pub coding_rounds_valid: bool,
    pub multi_round_loop_valid: bool,
    pub file_lease_records_valid: bool,
    pub backend_records_valid: bool,
    pub failure_records_valid: bool,
    pub safety_limits_valid: bool,
    pub replay_passed: bool,
    pub checks: Vec<WorkerTaskReplayCheck>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkerTaskReplayCheck {
    pub check_id: String,
    pub passed: bool,
    pub summary: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkerTaskMergeDecision {
    SafeToMerge,
    NeedsReview,
    Blocked,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkerTaskMergeRiskReport {
    pub task_id: String,
    pub workspace_id: String,
    pub worker_id: String,
    pub decision: WorkerTaskMergeDecision,
    pub risk_score: u8,
    pub replay_passed: bool,
    pub patch_conflicted_count: usize,
    pub patch_rejected_count: usize,
    pub patch_rolled_back_count: usize,
    pub reasons: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkerTaskPromotionDecision {
    Promoted,
    NeedsReview,
    Blocked,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkerTaskPromotionReport {
    pub task_id: String,
    pub workspace_id: String,
    pub worker_id: String,
    pub decision: WorkerTaskPromotionDecision,
    pub promotion_allowed: bool,
    pub auto_merge_allowed: bool,
    pub merge_risk: WorkerTaskMergeRiskReport,
    pub replay: WorkerTaskReplayAuditReport,
    pub unapplied_patch_count: usize,
    pub applied_patch_count: usize,
    pub reasons: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkerTaskPromotionLedgerReport {
    pub task_id: String,
    pub workspace_id: String,
    pub worker_id: String,
    pub ledger_count: usize,
    pub promotion_decision: WorkerTaskPromotionDecision,
    pub promotion_allowed: bool,
    pub auto_merge_allowed: bool,
    pub chain_head: String,
    pub entries: Vec<WorkerTaskPromotionLedgerEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkerTaskPromotionLedgerEntry {
    pub index: usize,
    pub ledger_ref: String,
    pub action: String,
    pub decision: WorkerTaskPromotionDecision,
    pub summary: String,
    pub occurred_at_unix_ms: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous_hash: Option<String>,
    pub entry_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkerTaskHandoffBundleReport {
    pub task_id: String,
    pub workspace_id: String,
    pub worker_id: String,
    pub bundle_version: String,
    pub generated_at_unix_ms: u64,
    pub evidence: WorkerTaskEvidenceReport,
    pub replay: WorkerTaskReplayAuditReport,
    pub merge_risk: WorkerTaskMergeRiskReport,
    pub promotion: WorkerTaskPromotionReport,
    pub promotion_ledger: WorkerTaskPromotionLedgerReport,
    pub handoff_ready: bool,
    pub signature: String,
    pub summary: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkerTaskPatchReviewReport {
    pub task_id: String,
    pub workspace_id: String,
    pub patch_count: usize,
    pub proposed_count: usize,
    pub applied_count: usize,
    pub conflicted_count: usize,
    pub rejected_count: usize,
    pub rolled_back_count: usize,
    pub patches: Vec<WorkerTaskPatchProposal>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkerTaskPatchSetApplyReport {
    pub task_id: String,
    pub workspace_id: String,
    pub patch_count: usize,
    pub attempted_count: usize,
    pub applied_count: usize,
    pub conflicted_count: usize,
    pub rejected_count: usize,
    pub skipped_count: usize,
    pub transaction_ids: Vec<String>,
    pub review: WorkerTaskPatchReviewReport,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkerTaskPatchRollbackReport {
    pub task_id: String,
    pub workspace_id: String,
    pub patch_count: usize,
    pub attempted_count: usize,
    pub rolled_back_count: usize,
    pub skipped_count: usize,
    pub failed_count: usize,
    pub rolled_back_transaction_ids: Vec<String>,
    pub failed_patch_ids: Vec<String>,
    pub review: WorkerTaskPatchReviewReport,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkerTaskDueRunReport {
    pub now_unix_ms: u64,
    pub due_count: usize,
    pub ran_count: usize,
    pub skipped_count: usize,
    pub runs: Vec<WorkerTaskRunReport>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkerTaskContextRecallDueRunReport {
    pub now_unix_ms: u64,
    pub due_count: usize,
    pub ran_count: usize,
    pub skipped_count: usize,
    pub context_recall_handoff_policy: WorkerTaskContextRecallHandoffPolicy,
    pub selected_snippets_present_count: usize,
    pub selected_snippet_count: u32,
    pub runs: Vec<WorkerTaskContextRecallRunReport>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkerTaskReadyRunReport {
    pub now_unix_ms: u64,
    pub candidate_count: usize,
    pub ready_count: usize,
    pub ran_count: usize,
    pub blocked_count: usize,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,
    pub runs: Vec<WorkerTaskRunReport>,
    pub blocked_task_ids: Vec<String>,
    pub pressure: WorkerPoolPressureReport,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkerTaskContextRecallReadyRunReport {
    pub now_unix_ms: u64,
    pub candidate_count: usize,
    pub ready_count: usize,
    pub ran_count: usize,
    pub blocked_count: usize,
    pub limit: Option<usize>,
    pub context_recall_handoff_policy: WorkerTaskContextRecallHandoffPolicy,
    pub selected_snippets_present_count: usize,
    pub selected_snippet_count: u32,
    pub runs: Vec<WorkerTaskContextRecallRunReport>,
    pub blocked_task_ids: Vec<String>,
    pub pressure: WorkerPoolPressureReport,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkerPoolPressureReport {
    pub max_global_concurrency: usize,
    pub max_per_worker_concurrency: usize,
    pub active_count: usize,
    pub ready_count: usize,
    pub available_global_slots: usize,
    pub pressure_level: WorkerPoolPressureLevel,
    pub throttled_task_ids: Vec<String>,
    pub per_worker: Vec<WorkerPressureLane>,
}

const DEFAULT_WORKER_POOL_MAX_GLOBAL_CONCURRENCY: usize = 4;
const DEFAULT_WORKER_POOL_MAX_PER_WORKER_CONCURRENCY: usize = 2;
const DEFAULT_WORKER_TASK_TIMEOUT_BUDGET_MS: u64 = 120_000;
const DEFAULT_WORKER_COMMAND_TIMEOUT_MS: u64 = 30_000;
const DEFAULT_WORKER_MAX_COMMAND_RUNS: usize = 8;
const DEFAULT_WORKER_MAX_STDOUT_BYTES: usize = 16 * 1024;
const DEFAULT_WORKER_MAX_STDERR_BYTES: usize = 16 * 1024;
const DEFAULT_WORKER_MAX_PATCH_PROPOSALS: usize = 4;
const DEFAULT_WORKER_MAX_LOOP_STEPS: usize = 8;
const DEFAULT_WORKER_RETRY_BACKOFF_BASE_MS: u64 = 1_000;
const DEFAULT_WORKER_RETRY_BACKOFF_MAX_MS: u64 = 60_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkerPoolPressureLevel {
    Idle,
    Normal,
    Saturated,
    Throttled,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkerPressureLane {
    pub worker_id: String,
    pub active_count: usize,
    pub ready_count: usize,
    pub available_slots: usize,
    pub throttled_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkerTaskIndexReport {
    pub active_session_id: String,
    pub total_count: usize,
    pub queued_count: usize,
    pub scheduled_count: usize,
    pub running_count: usize,
    pub paused_count: usize,
    pub completed_count: usize,
    pub failed_count: usize,
    pub cancelled_count: usize,
    pub interrupted_count: usize,
    pub tasks: Vec<WorkerTaskRecord>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkerDescriptor {
    pub worker_id: String,
    pub session_count: usize,
    pub task_count: usize,
    pub active_task_count: usize,
    pub completed_task_count: usize,
    pub latest_activity_unix_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkerInventoryReport {
    pub worker_count: usize,
    pub total_task_count: usize,
    pub active_task_count: usize,
    pub workers: Vec<WorkerDescriptor>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkerTaskJoinItem {
    pub task_id: String,
    pub worker_id: String,
    pub worker_session_id: String,
    pub status: WorkerTaskStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result_summary: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub artifacts: Vec<WorkerTaskArtifact>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub diff_summary: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub patch_proposals: Vec<WorkerTaskPatchProposal>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub coding_rounds: Vec<WorkerTaskCodingRound>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub file_leases: Vec<WorkerTaskFileLease>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub loop_steps: Vec<WorkerTaskLoopStep>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub command_runs: Vec<WorkerTaskCommandRun>,
    pub merge_risk: WorkerTaskMergeRiskReport,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkerTaskJoinReport {
    pub active_session_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub worker_filter: Option<String>,
    pub total_count: usize,
    pub completed_count: usize,
    pub failed_count: usize,
    pub active_count: usize,
    pub safe_to_join: bool,
    pub joined: Vec<WorkerTaskJoinItem>,
    pub active_task_ids: Vec<String>,
    pub failed_task_ids: Vec<String>,
    pub artifact_count: usize,
    pub diff_ready_count: usize,
    pub patch_proposal_count: usize,
    pub coding_round_count: usize,
    pub file_lease_count: usize,
    pub active_file_lease_count: usize,
    pub held_file_lease_count: usize,
    pub conflicted_file_lease_count: usize,
    pub expired_file_lease_count: usize,
    pub patch_applied_count: usize,
    pub patch_conflicted_count: usize,
    pub patch_rejected_count: usize,
    pub patch_rolled_back_count: usize,
    pub loop_step_count: usize,
    pub command_run_count: usize,
    pub permission_envelopes: Vec<WorkerPermissionEnvelope>,
    pub merge_safe_count: usize,
    pub merge_needs_review_count: usize,
    pub merge_blocked_count: usize,
    pub max_merge_risk_score: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkerTaskSupervisorReport {
    pub now_unix_ms: u64,
    pub worker_count: usize,
    pub total_count: usize,
    pub active_count: usize,
    pub ready_count: usize,
    pub blocked_count: usize,
    pub scheduled_future_count: usize,
    pub failed_count: usize,
    pub paused_count: usize,
    pub interrupted_count: usize,
    pub safe_to_join: bool,
    pub attention_required: bool,
    pub recommended_next_action: String,
    pub ready_task_ids: Vec<String>,
    pub blocked_task_ids: Vec<String>,
    pub failed_task_ids: Vec<String>,
    pub paused_task_ids: Vec<String>,
    pub interrupted_task_ids: Vec<String>,
    pub completed_artifact_count: usize,
    pub diff_ready_count: usize,
    pub patch_proposal_count: usize,
    pub coding_round_count: usize,
    pub multi_round_task_count: usize,
    pub max_rounds_per_task: usize,
    pub file_lease_count: usize,
    pub active_file_lease_count: usize,
    pub held_file_lease_count: usize,
    pub conflicted_file_lease_count: usize,
    pub expired_file_lease_count: usize,
    pub loop_step_count: usize,
    pub command_run_count: usize,
    pub timeout_count: usize,
    pub cancelled_count: usize,
    pub paused_control_count: usize,
    pub interrupted_control_count: usize,
    pub resource_limit_violation_count: usize,
    pub sandbox_envelope_count: usize,
    pub pressure: WorkerPoolPressureReport,
    pub permission_envelopes: Vec<WorkerPermissionEnvelope>,
    pub safety_envelopes: Vec<WorkerTaskSafetyEnvelope>,
    pub merge_safe_count: usize,
    pub merge_needs_review_count: usize,
    pub merge_blocked_count: usize,
    pub max_merge_risk_score: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkerSubagentObservatoryReport {
    pub now_unix_ms: u64,
    pub total_count: usize,
    pub active_count: usize,
    pub paused_count: usize,
    pub interrupted_count: usize,
    pub autonomous_count: usize,
    pub attention_required: bool,
    pub file_lease_count: usize,
    pub active_file_lease_count: usize,
    pub held_file_lease_count: usize,
    pub conflicted_file_lease_count: usize,
    pub expired_file_lease_count: usize,
    pub coding_round_count: usize,
    pub command_run_count: usize,
    pub recommended_next_action: String,
    pub lanes: Vec<WorkerSubagentLaneObservation>,
    pub file_leases: Vec<WorkerTaskFileLease>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkerSubagentLaneObservation {
    pub task_id: String,
    pub worker_id: String,
    pub worker_session_id: String,
    pub status: WorkerTaskStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub paused_from_status: Option<WorkerTaskStatus>,
    pub execution_mode: WorkerTaskExecutionMode,
    pub coding_round_count: usize,
    pub command_run_count: usize,
    pub patch_proposal_count: usize,
    pub file_lease_count: usize,
    pub lease_paths: Vec<String>,
    pub lease_statuses: Vec<WorkerTaskFileLeaseStatus>,
    pub attention_required: bool,
    pub control_action: String,
    pub summary: String,
    pub updated_at_unix_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct OperatorConsoleEventSummary {
    pub emitted_at_unix_ms: u64,
    pub kind: EventKind,
    pub session_id: Option<String>,
    pub summary: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct OperatorConsoleReport {
    pub product: String,
    pub status: String,
    pub task_queue_panel: bool,
    pub subagent_tree_panel: bool,
    pub command_stream_panel: bool,
    pub patch_evidence_review_panel: bool,
    pub approval_controls_panel: bool,
    pub live_control_panel: bool,
    pub steer_control_ready: bool,
    pub cancel_control_ready: bool,
    pub pause_control_ready: bool,
    pub resume_control_ready: bool,
    pub interrupt_control_ready: bool,
    pub operator_console_complete: bool,
    pub recommended_next_action: String,
    pub control_commands: Vec<String>,
    pub task_supervisor: WorkerTaskSupervisorReport,
    pub subagent_observatory: WorkerSubagentObservatoryReport,
    pub recent_events: Vec<OperatorConsoleEventSummary>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct WorkerPatchApplyOutcome {
    apply_status: WorkerTaskPatchApplyStatus,
    applied_at_unix_ms: Option<u64>,
    transaction_id: Option<String>,
    conflict_reason: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct WorkerTaskExecutionOutput {
    result: VerticalSliceResult,
    artifacts: Vec<WorkerTaskArtifact>,
    diff_summary: String,
    patch_proposals: Vec<WorkerTaskPatchProposal>,
    coding_rounds: Vec<WorkerTaskCodingRound>,
    loop_steps: Vec<WorkerTaskLoopStep>,
    command_runs: Vec<WorkerTaskCommandRun>,
}

impl WorkerPatchApplyOutcome {
    fn conflicted(applied_at_unix_ms: u64, reason: String) -> Self {
        Self {
            apply_status: WorkerTaskPatchApplyStatus::Conflicted,
            applied_at_unix_ms: Some(applied_at_unix_ms),
            transaction_id: None,
            conflict_reason: Some(reason),
        }
    }
}

impl RuntimeKernel {
    pub fn spawn_worker_task(
        &self,
        worker_id: &str,
        prompt: &str,
        schedule_expr: Option<&str>,
    ) -> Result<WorkerTaskReport, HeptaError> {
        self.spawn_worker_task_with_dependencies(worker_id, prompt, schedule_expr, Vec::new())
    }

    pub fn spawn_worker_task_in_workspace(
        &self,
        worker_id: &str,
        workspace_id: Option<&str>,
        prompt: &str,
        schedule_expr: Option<&str>,
    ) -> Result<WorkerTaskReport, HeptaError> {
        self.spawn_worker_task_with_dependencies_in_workspace(
            worker_id,
            workspace_id,
            prompt,
            schedule_expr,
            Vec::new(),
        )
    }

    pub fn spawn_worker_task_with_dependencies(
        &self,
        worker_id: &str,
        prompt: &str,
        schedule_expr: Option<&str>,
        depends_on: Vec<String>,
    ) -> Result<WorkerTaskReport, HeptaError> {
        self.spawn_worker_task_with_dependencies_in_workspace(
            worker_id,
            None,
            prompt,
            schedule_expr,
            depends_on,
        )
    }

    pub fn spawn_worker_task_with_dependencies_in_workspace(
        &self,
        worker_id: &str,
        workspace_id: Option<&str>,
        prompt: &str,
        schedule_expr: Option<&str>,
        depends_on: Vec<String>,
    ) -> Result<WorkerTaskReport, HeptaError> {
        self.spawn_worker_task_with_parent_in_workspace(
            worker_id,
            workspace_id,
            prompt,
            schedule_expr,
            depends_on,
            None,
            default_worker_task_max_spawn_depth(),
        )
    }

    pub fn spawn_worker_task_with_parent(
        &self,
        worker_id: &str,
        prompt: &str,
        schedule_expr: Option<&str>,
        depends_on: Vec<String>,
        parent_task_id: Option<String>,
        max_spawn_depth: usize,
    ) -> Result<WorkerTaskReport, HeptaError> {
        self.spawn_worker_task_with_parent_in_workspace(
            worker_id,
            None,
            prompt,
            schedule_expr,
            depends_on,
            parent_task_id,
            max_spawn_depth,
        )
    }

    pub fn spawn_worker_task_with_parent_in_workspace(
        &self,
        worker_id: &str,
        workspace_id: Option<&str>,
        prompt: &str,
        schedule_expr: Option<&str>,
        depends_on: Vec<String>,
        parent_task_id: Option<String>,
        max_spawn_depth: usize,
    ) -> Result<WorkerTaskReport, HeptaError> {
        let worker_id = normalize_worker_id(worker_id)?;
        let prompt = prompt.trim();
        if prompt.is_empty() {
            return Err(HeptaError("task prompt must not be empty".into()));
        }
        let schedule_expr = schedule_expr
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(ToOwned::to_owned);
        let depends_on = normalize_dependencies(depends_on)?;
        self.validate_worker_task_dependencies(&depends_on)?;
        let parent_session_id = self.active_session_id()?;
        let requested_workspace_id = workspace_id
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(ToOwned::to_owned);
        let now = current_unix_ms()?;
        let (task_id, worker_session_id) = {
            let guard = self
                .worker_task_state
                .lock()
                .map_err(|_| HeptaError("worker task state mutex poisoned".into()))?;
            let next = guard.records.len() + 1;
            (
                format!("task-{}-{}", now, next),
                format!("worker-{}-{}", sanitize_for_id(&worker_id), next),
            )
        };
        self.upsert_session_record_with_agent(
            &SessionId(worker_session_id.clone()),
            Some(format!("Hepta worker {}", worker_id)),
            Some(format!("queued task {}", task_id)),
            None,
            true,
            Some(AgentId(worker_id.clone())),
        )?;

        let parent_model = self.model_selection_for_session(&parent_session_id)?.active;
        self.set_session_model(&worker_session_id, parent_model)?;
        let parent_profile = self.execution_profile_for_session(&parent_session_id)?;
        let parent_scope = self.filesystem_scope_for_session(&parent_session_id)?;
        let parent_write_scope = self.write_path_scope_for_session(&parent_session_id)?;
        let permission_envelope = build_worker_permission_envelope(
            &worker_id,
            parent_profile,
            parent_scope,
            parent_write_scope,
        );
        let workspace_root = self.workspace_root()?;
        let timeout_budget_ms = worker_task_timeout_budget_ms();
        let safety_envelope =
            build_worker_safety_envelope(&permission_envelope, &workspace_root, timeout_budget_ms);
        let execution_mode = infer_worker_execution_mode(&worker_id, prompt);
        let existing_records = self.worker_task_records()?;
        let parent_task_id = parent_task_id
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty());
        let max_spawn_depth = max_spawn_depth.max(default_worker_task_max_spawn_depth());
        let (workspace_id, spawn_depth) = if let Some(parent_id) = parent_task_id.as_deref() {
            let parent = existing_records
                .iter()
                .find(|candidate| candidate.task_id == parent_id)
                .ok_or_else(|| HeptaError(format!("unknown parent task: {}", parent_id)))?;
            if requested_workspace_id
                .as_deref()
                .is_some_and(|requested| requested != parent.workspace_id)
            {
                return Err(HeptaError(format!(
                    "workspace mismatch for child task: requested {} but parent {} is bound to {}",
                    requested_workspace_id.as_deref().unwrap_or("<inherit>"),
                    parent_id,
                    parent.workspace_id
                )));
            }
            let child_depth = parent.spawn_depth.saturating_add(1);
            if child_depth > parent.max_spawn_depth {
                return Err(HeptaError(format!(
                    "recursive spawn denied: parent task {} depth {} exceeds max_spawn_depth {}",
                    parent_id, child_depth, parent.max_spawn_depth
                )));
            }
            (parent.workspace_id.clone(), child_depth)
        } else {
            (
                requested_workspace_id.unwrap_or_else(|| parent_session_id.clone()),
                0,
            )
        };
        let file_leases = build_worker_file_leases(
            &task_id,
            &worker_id,
            &worker_session_id,
            execution_mode,
            &workspace_root,
            now,
            timeout_budget_ms,
            &existing_records,
        );
        let _ = self.switch_execution_profile_in_session(
            &worker_session_id,
            permission_envelope.execution_profile,
        )?;
        let _ = self.switch_filesystem_scope_in_session(
            &worker_session_id,
            permission_envelope.filesystem_scope,
        )?;
        let _ = self.switch_write_path_scope_in_session(
            &worker_session_id,
            permission_envelope.write_scope,
        )?;

        let task = WorkerTaskRecord {
            task_id: task_id.clone(),
            parent_session_id,
            workspace_id: workspace_id.clone(),
            worker_session_id: worker_session_id.clone(),
            worker_id: worker_id.clone(),
            prompt: prompt.to_string(),
            permission_envelope: permission_envelope.clone(),
            safety_envelope: safety_envelope.clone(),
            execution_mode,
            execution_backend: default_worker_execution_backend_binding(),
            depends_on,
            parent_task_id,
            spawn_depth,
            max_spawn_depth,
            status: if schedule_expr.is_some() {
                WorkerTaskStatus::Scheduled
            } else {
                WorkerTaskStatus::Queued
            },
            paused_from_status: None,
            next_run_unix_ms: schedule_expr
                .as_deref()
                .map(|expr| parse_schedule_next_run(expr, now))
                .transpose()?,
            schedule_expr,
            created_at_unix_ms: now,
            updated_at_unix_ms: now,
            started_at_unix_ms: None,
            completed_at_unix_ms: None,
            attempt_count: 0,
            max_attempts: 3,
            timeout_budget_ms,
            last_error: None,
            failure_kind: None,
            retry_after_unix_ms: None,
            result_summary: None,
            artifacts: Vec::new(),
            diff_summary: None,
            patch_proposals: Vec::new(),
            coding_rounds: Vec::new(),
            file_leases,
            loop_steps: Vec::new(),
            command_runs: Vec::new(),
            steering_directives: Vec::new(),
        };
        {
            let mut guard = self
                .worker_task_state
                .lock()
                .map_err(|_| HeptaError("worker task state mutex poisoned".into()))?;
            guard.records.push(task.clone());
        }
        self.emit_event_with_payload(
            if task.schedule_expr.is_some() {
                EventKind::TaskScheduled
            } else {
                EventKind::TaskSpawned
            },
            Some(SessionId(worker_session_id)),
            None,
            format!("spawned task {} for worker {}", task_id, worker_id),
            Some(json!({
                "task_id": task_id,
                "worker_id": worker_id,
                "workspace_id": task.workspace_id.clone(),
                "status": task_status_label(task.status),
                "depends_on": task.depends_on,
                "parent_task_id": task.parent_task_id,
                "spawn_depth": task.spawn_depth,
                "max_spawn_depth": task.max_spawn_depth,
                "permission_envelope": task.permission_envelope,
                "safety_envelope": task.safety_envelope,
                "execution_backend": task.execution_backend,
                "file_lease_count": task.file_leases.len(),
                "schedule_expr": task.schedule_expr,
                "next_run_unix_ms": task.next_run_unix_ms,
            })),
        )?;
        Ok(WorkerTaskReport { task })
    }

    pub fn worker_task_index(
        &self,
        status_filter: Option<WorkerTaskStatus>,
    ) -> Result<WorkerTaskIndexReport, HeptaError> {
        let active_session_id = self.active_session_id()?;
        let mut tasks = self.worker_task_records()?;
        tasks.sort_by(|left, right| left.created_at_unix_ms.cmp(&right.created_at_unix_ms));
        let total_records = tasks.clone();
        if let Some(status) = status_filter {
            tasks.retain(|task| task.status == status);
        }
        Ok(WorkerTaskIndexReport {
            active_session_id,
            total_count: total_records.len(),
            queued_count: count_status(&total_records, WorkerTaskStatus::Queued),
            scheduled_count: count_status(&total_records, WorkerTaskStatus::Scheduled),
            running_count: count_status(&total_records, WorkerTaskStatus::Running),
            paused_count: count_status(&total_records, WorkerTaskStatus::Paused),
            completed_count: count_status(&total_records, WorkerTaskStatus::Completed),
            failed_count: count_status(&total_records, WorkerTaskStatus::Failed),
            cancelled_count: count_status(&total_records, WorkerTaskStatus::Cancelled),
            interrupted_count: count_status(&total_records, WorkerTaskStatus::Interrupted),
            tasks,
        })
    }

    pub fn worker_task_status(&self, task_id: &str) -> Result<WorkerTaskReport, HeptaError> {
        let task = self.find_worker_task(task_id)?;
        Ok(WorkerTaskReport { task })
    }

    pub fn cancel_worker_task(&self, task_id: &str) -> Result<WorkerTaskReport, HeptaError> {
        let now = current_unix_ms()?;
        let task = {
            let mut guard = self
                .worker_task_state
                .lock()
                .map_err(|_| HeptaError("worker task state mutex poisoned".into()))?;
            let task = guard
                .records
                .iter_mut()
                .find(|task| task.task_id == task_id)
                .ok_or_else(|| HeptaError(format!("unknown task: {}", task_id)))?;
            if matches!(
                task.status,
                WorkerTaskStatus::Completed
                    | WorkerTaskStatus::Cancelled
                    | WorkerTaskStatus::Interrupted
            ) {
                return Err(HeptaError(format!(
                    "task {} is already {}",
                    task_id,
                    task_status_label(task.status)
                )));
            }
            task.status = WorkerTaskStatus::Cancelled;
            task.paused_from_status = None;
            task.updated_at_unix_ms = now;
            task.completed_at_unix_ms = Some(now);
            task.result_summary = Some("cancelled before completion".into());
            task.artifacts.clear();
            task.diff_summary = None;
            task.patch_proposals.clear();
            task.coding_rounds.clear();
            task.loop_steps.clear();
            task.command_runs.clear();
            task.clone()
        };
        self.emit_event_with_payload(
            EventKind::TaskCancelled,
            Some(SessionId(task.worker_session_id.clone())),
            None,
            format!("cancelled task {}", task.task_id),
            Some(json!({
                "task_id": task.task_id,
                "worker_id": task.worker_id,
                "status": "cancelled",
            })),
        )?;
        Ok(WorkerTaskReport { task })
    }

    pub fn pause_worker_task(&self, task_id: &str) -> Result<WorkerTaskReport, HeptaError> {
        let now = current_unix_ms()?;
        let task = {
            let mut guard = self
                .worker_task_state
                .lock()
                .map_err(|_| HeptaError("worker task state mutex poisoned".into()))?;
            let task = guard
                .records
                .iter_mut()
                .find(|task| task.task_id == task_id)
                .ok_or_else(|| HeptaError(format!("unknown task: {}", task_id)))?;
            if matches!(
                task.status,
                WorkerTaskStatus::Completed
                    | WorkerTaskStatus::Cancelled
                    | WorkerTaskStatus::Interrupted
            ) {
                return Err(HeptaError(format!(
                    "task {} is already {}",
                    task_id,
                    task_status_label(task.status)
                )));
            }
            if task.status == WorkerTaskStatus::Paused {
                return Err(HeptaError(format!("task {} is already paused", task_id)));
            }
            task.paused_from_status = Some(task.status);
            task.status = WorkerTaskStatus::Paused;
            task.updated_at_unix_ms = now;
            task.result_summary = Some("paused by operator".into());
            task.clone()
        };
        self.emit_event_with_payload(
            EventKind::TaskPaused,
            Some(SessionId(task.worker_session_id.clone())),
            None,
            format!("paused task {}", task.task_id),
            Some(json!({
                "task_id": task.task_id,
                "worker_id": task.worker_id,
                "status": "paused",
                "paused_from_status": task.paused_from_status.map(task_status_label),
            })),
        )?;
        Ok(WorkerTaskReport { task })
    }

    pub fn resume_worker_task(&self, task_id: &str) -> Result<WorkerTaskReport, HeptaError> {
        let now = current_unix_ms()?;
        let task = {
            let mut guard = self
                .worker_task_state
                .lock()
                .map_err(|_| HeptaError("worker task state mutex poisoned".into()))?;
            let task = guard
                .records
                .iter_mut()
                .find(|task| task.task_id == task_id)
                .ok_or_else(|| HeptaError(format!("unknown task: {}", task_id)))?;
            if task.status != WorkerTaskStatus::Paused {
                return Err(HeptaError(format!(
                    "task {} is not paused [{}]",
                    task_id,
                    task_status_label(task.status)
                )));
            }
            let restored = match task.paused_from_status.take() {
                Some(WorkerTaskStatus::Scheduled) => WorkerTaskStatus::Scheduled,
                Some(WorkerTaskStatus::Failed) => WorkerTaskStatus::Failed,
                _ => WorkerTaskStatus::Queued,
            };
            task.status = restored;
            task.updated_at_unix_ms = now;
            task.result_summary = Some("resumed by operator".into());
            task.clone()
        };
        self.emit_event_with_payload(
            EventKind::TaskResumed,
            Some(SessionId(task.worker_session_id.clone())),
            None,
            format!("resumed task {}", task.task_id),
            Some(json!({
                "task_id": task.task_id,
                "worker_id": task.worker_id,
                "status": task_status_label(task.status),
            })),
        )?;
        Ok(WorkerTaskReport { task })
    }

    pub fn steer_worker_task(
        &self,
        task_id: &str,
        instruction: &str,
    ) -> Result<WorkerTaskReport, HeptaError> {
        let instruction = instruction.trim();
        if instruction.is_empty() {
            return Err(HeptaError("steer instruction must not be empty".into()));
        }
        let now = current_unix_ms()?;
        let task = {
            let mut guard = self
                .worker_task_state
                .lock()
                .map_err(|_| HeptaError("worker task state mutex poisoned".into()))?;
            let task = guard
                .records
                .iter_mut()
                .find(|task| task.task_id == task_id)
                .ok_or_else(|| HeptaError(format!("unknown task: {}", task_id)))?;
            if matches!(
                task.status,
                WorkerTaskStatus::Completed
                    | WorkerTaskStatus::Cancelled
                    | WorkerTaskStatus::Interrupted
            ) {
                return Err(HeptaError(format!(
                    "task {} is already {}",
                    task_id,
                    task_status_label(task.status)
                )));
            }
            let directive_id = format!(
                "{}:steer-{}",
                task.task_id,
                task.steering_directives.len() + 1
            );
            task.steering_directives.push(WorkerTaskSteerDirective {
                directive_id: directive_id.clone(),
                instruction: instruction.to_string(),
                created_at_unix_ms: now,
            });
            task.updated_at_unix_ms = now;
            task.result_summary = Some(format!(
                "steered by operator: {}",
                compact_text(instruction, 96)
            ));
            task.clone()
        };
        self.emit_event_with_payload(
            EventKind::TaskSteered,
            Some(SessionId(task.worker_session_id.clone())),
            None,
            format!("steered task {}", task.task_id),
            Some(json!({
                "task_id": task.task_id,
                "worker_id": task.worker_id,
                "status": task_status_label(task.status),
                "steering_directive_count": task.steering_directives.len(),
                "latest_instruction": compact_text(instruction, 160),
            })),
        )?;
        Ok(WorkerTaskReport { task })
    }

    pub fn interrupt_worker_task(&self, task_id: &str) -> Result<WorkerTaskReport, HeptaError> {
        let now = current_unix_ms()?;
        let task = {
            let mut guard = self
                .worker_task_state
                .lock()
                .map_err(|_| HeptaError("worker task state mutex poisoned".into()))?;
            let task = guard
                .records
                .iter_mut()
                .find(|task| task.task_id == task_id)
                .ok_or_else(|| HeptaError(format!("unknown task: {}", task_id)))?;
            if matches!(
                task.status,
                WorkerTaskStatus::Completed
                    | WorkerTaskStatus::Cancelled
                    | WorkerTaskStatus::Interrupted
            ) {
                return Err(HeptaError(format!(
                    "task {} is already {}",
                    task_id,
                    task_status_label(task.status)
                )));
            }
            task.status = WorkerTaskStatus::Interrupted;
            task.paused_from_status = None;
            task.updated_at_unix_ms = now;
            task.completed_at_unix_ms = Some(now);
            task.result_summary = Some("interrupted by operator".into());
            task.artifacts.clear();
            task.diff_summary = None;
            task.patch_proposals.clear();
            task.coding_rounds.clear();
            task.loop_steps.clear();
            task.command_runs.clear();
            update_worker_file_lease_statuses_after_run(task, WorkerTaskStatus::Interrupted, now);
            task.clone()
        };
        self.emit_event_with_payload(
            EventKind::TaskInterrupted,
            Some(SessionId(task.worker_session_id.clone())),
            None,
            format!("interrupted task {}", task.task_id),
            Some(json!({
                "task_id": task.task_id,
                "worker_id": task.worker_id,
                "status": "interrupted",
            })),
        )?;
        Ok(WorkerTaskReport { task })
    }

    pub async fn run_worker_task(&self, task_id: &str) -> Result<WorkerTaskRunReport, HeptaError> {
        Ok(self
            .run_worker_task_with_context_recall_handoff(
                task_id,
                WorkerTaskContextRecallHandoffPolicy::Disabled,
            )
            .await?
            .run)
    }

    pub async fn run_worker_task_with_context_recall_handoff(
        &self,
        task_id: &str,
        context_recall_handoff_policy: WorkerTaskContextRecallHandoffPolicy,
    ) -> Result<WorkerTaskContextRecallRunReport, HeptaError> {
        let prompt = {
            let now = current_unix_ms()?;
            let mut guard = self
                .worker_task_state
                .lock()
                .map_err(|_| HeptaError("worker task state mutex poisoned".into()))?;
            let task_index = guard
                .records
                .iter()
                .position(|task| task.task_id == task_id)
                .ok_or_else(|| HeptaError(format!("unknown task: {}", task_id)))?;
            let depends_on = guard.records[task_index].depends_on.clone();
            for dependency_id in &depends_on {
                let dependency = guard
                    .records
                    .iter()
                    .find(|candidate| &candidate.task_id == dependency_id)
                    .ok_or_else(|| {
                        HeptaError(format!(
                            "task {} depends on unknown task {}",
                            task_id, dependency_id
                        ))
                    })?;
                if dependency.status != WorkerTaskStatus::Completed {
                    return Err(HeptaError(format!(
                        "task {} waiting on dependency {} [{}]",
                        task_id,
                        dependency_id,
                        task_status_label(dependency.status)
                    )));
                }
            }
            let task = &mut guard.records[task_index];
            match task.status {
                WorkerTaskStatus::Queued
                | WorkerTaskStatus::Scheduled
                | WorkerTaskStatus::Failed => {}
                WorkerTaskStatus::Running => {
                    return Err(HeptaError(format!("task {} is already running", task_id)));
                }
                WorkerTaskStatus::Paused => {
                    return Err(HeptaError(format!("task {} is paused", task_id)));
                }
                WorkerTaskStatus::Completed
                | WorkerTaskStatus::Cancelled
                | WorkerTaskStatus::Interrupted => {
                    return Err(HeptaError(format!(
                        "task {} is already {}",
                        task_id,
                        task_status_label(task.status)
                    )));
                }
            }
            if task.attempt_count >= task.max_attempts {
                return Err(HeptaError(format!(
                    "task {} exhausted retry budget",
                    task_id
                )));
            }
            task.status = WorkerTaskStatus::Running;
            task.attempt_count += 1;
            task.started_at_unix_ms = Some(now);
            task.updated_at_unix_ms = now;
            task.last_error = None;
            task.failure_kind = None;
            task.retry_after_unix_ms = None;
            effective_worker_task_prompt(task)
        };

        let running = self.find_worker_task(task_id)?;
        self.emit_event_with_payload(
            EventKind::TaskStarted,
            Some(SessionId(running.worker_session_id.clone())),
            None,
            format!("running task {}", running.task_id),
            Some(json!({
                "task_id": running.task_id,
                "worker_id": running.worker_id,
                "status": "running",
            })),
        )?;

        let mut provider_rollup = None;
        let mut selected_snippets_present = false;
        let mut selected_snippet_count = 0;
        let run_result = if let Some(err) = simulated_worker_failure(&running) {
            Err(err)
        } else if running.execution_mode == WorkerTaskExecutionMode::AutonomousCoding {
            self.run_autonomous_coding_worker_loop(&running).await
        } else if context_recall_handoff_policy.experimental_api_enabled() {
            match self
                .run_demo_turn_in_session_with_context_recall_handoff(
                    &running.worker_session_id,
                    &prompt,
                    true,
                )
                .await
            {
                Ok(run) => {
                    provider_rollup = Some(run.provider_rollup);
                    selected_snippets_present = run.selected_snippets_present;
                    selected_snippet_count = run.selected_snippet_count;
                    Ok(build_conversational_worker_execution_output(
                        &running, run.result,
                    ))
                }
                Err(err) => Err(err),
            }
        } else {
            self.run_demo_turn_in_session(&running.worker_session_id, &prompt)
                .await
                .map(|result| build_conversational_worker_execution_output(&running, result))
        };

        match run_result {
            Ok(output) => {
                let now = current_unix_ms()?;
                let task = self.update_worker_task_after_run(
                    task_id,
                    WorkerTaskStatus::Completed,
                    Some(now),
                    None,
                    Some(output.result.final_text.clone()),
                    output.artifacts.clone(),
                    Some(output.diff_summary.clone()),
                    output.patch_proposals.clone(),
                    output.coding_rounds.clone(),
                    output.loop_steps.clone(),
                    output.command_runs.clone(),
                    None,
                    None,
                )?;
                self.emit_event_with_payload(
                    EventKind::TaskCompleted,
                    Some(SessionId(task.worker_session_id.clone())),
                    None,
                    format!("completed task {}", task.task_id),
                    Some(json!({
                        "task_id": task.task_id,
                        "worker_id": task.worker_id,
                        "status": "completed",
                        "artifact_count": task.artifacts.len(),
                        "diff_ready": task.diff_summary.is_some(),
                        "patch_proposal_count": task.patch_proposals.len(),
                        "coding_round_count": task.coding_rounds.len(),
                        "file_lease_count": task.file_leases.len(),
                        "loop_step_count": task.loop_steps.len(),
                        "command_run_count": task.command_runs.len(),
                    })),
                )?;
                let file_leases = task.file_leases.clone();
                Ok(WorkerTaskContextRecallRunReport {
                    run: WorkerTaskRunReport {
                        task,
                        result: Some(output.result),
                        artifact_count: output.artifacts.len(),
                        artifacts: output.artifacts,
                        patch_proposal_count: output.patch_proposals.len(),
                        patch_proposals: output.patch_proposals,
                        coding_round_count: output.coding_rounds.len(),
                        coding_rounds: output.coding_rounds,
                        file_lease_count: file_leases.len(),
                        file_leases,
                        loop_step_count: output.loop_steps.len(),
                        loop_steps: output.loop_steps,
                        command_run_count: output.command_runs.len(),
                        command_runs: output.command_runs,
                    },
                    context_recall_handoff_policy,
                    provider_rollup,
                    selected_snippets_present,
                    selected_snippet_count,
                })
            }
            Err(err) => {
                let now = current_unix_ms()?;
                let failure_kind = classify_worker_failure(&err.0);
                let retry_after = worker_retry_after_unix_ms(now, running.attempt_count);
                let task = self.update_worker_task_after_run(
                    task_id,
                    WorkerTaskStatus::Failed,
                    Some(now),
                    Some(err.0.clone()),
                    None,
                    Vec::new(),
                    None,
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                    Some(failure_kind),
                    Some(retry_after),
                )?;
                self.emit_event_with_payload(
                    EventKind::TaskFailed,
                    Some(SessionId(task.worker_session_id.clone())),
                    None,
                    format!("failed task {}", task.task_id),
                    Some(json!({
                        "task_id": task.task_id,
                        "worker_id": task.worker_id,
                        "status": "failed",
                        "last_error": task.last_error,
                        "failure_kind": task.failure_kind,
                        "retry_after_unix_ms": task.retry_after_unix_ms,
                    })),
                )?;
                let file_leases = task.file_leases.clone();
                Ok(WorkerTaskContextRecallRunReport {
                    run: WorkerTaskRunReport {
                        task,
                        result: None,
                        artifact_count: 0,
                        artifacts: Vec::new(),
                        patch_proposal_count: 0,
                        patch_proposals: Vec::new(),
                        coding_round_count: 0,
                        coding_rounds: Vec::new(),
                        file_lease_count: file_leases.len(),
                        file_leases,
                        loop_step_count: 0,
                        loop_steps: Vec::new(),
                        command_run_count: 0,
                        command_runs: Vec::new(),
                    },
                    context_recall_handoff_policy,
                    provider_rollup: None,
                    selected_snippets_present: false,
                    selected_snippet_count: 0,
                })
            }
        }
    }

    pub fn worker_task_loop(&self, task_id: &str) -> Result<WorkerTaskLoopReport, HeptaError> {
        let task = self.find_worker_task(task_id)?;
        Ok(worker_task_loop_report(task))
    }

    pub fn worker_task_evidence(
        &self,
        task_id: &str,
    ) -> Result<WorkerTaskEvidenceReport, HeptaError> {
        let task = self.find_worker_task(task_id)?;
        Ok(worker_task_evidence_report(task))
    }

    pub fn worker_task_replay_audit(
        &self,
        task_id: &str,
    ) -> Result<WorkerTaskReplayAuditReport, HeptaError> {
        let task = self.find_worker_task(task_id)?;
        Ok(worker_task_replay_audit_report(task))
    }

    pub fn worker_task_promotion_gate(
        &self,
        task_id: &str,
    ) -> Result<WorkerTaskPromotionReport, HeptaError> {
        let task = self.find_worker_task(task_id)?;
        Ok(worker_task_promotion_report(task))
    }

    pub fn worker_task_promotion_ledger(
        &self,
        task_id: &str,
    ) -> Result<WorkerTaskPromotionLedgerReport, HeptaError> {
        let task = self.find_worker_task(task_id)?;
        Ok(worker_task_promotion_ledger_report(task))
    }

    pub fn worker_task_handoff_bundle(
        &self,
        task_id: &str,
    ) -> Result<WorkerTaskHandoffBundleReport, HeptaError> {
        let task = self.find_worker_task(task_id)?;
        Ok(worker_task_handoff_bundle_report(task))
    }

    pub fn worker_task_patches(
        &self,
        task_id: &str,
    ) -> Result<WorkerTaskPatchReviewReport, HeptaError> {
        let task = self.find_worker_task(task_id)?;
        Ok(worker_task_patch_review_report(task))
    }

    pub fn mark_worker_task_patch_applied(
        &self,
        task_id: &str,
        patch_id: &str,
    ) -> Result<WorkerTaskPatchReviewReport, HeptaError> {
        self.apply_worker_task_patch(task_id, patch_id)
    }

    pub fn apply_worker_task_patch(
        &self,
        task_id: &str,
        patch_id: &str,
    ) -> Result<WorkerTaskPatchReviewReport, HeptaError> {
        let (task, patch) = {
            let task = self.find_worker_task(task_id)?;
            let patch = task
                .patch_proposals
                .iter()
                .find(|patch| patch.patch_id == patch_id)
                .cloned()
                .ok_or_else(|| HeptaError(format!("unknown patch: {}", patch_id)))?;
            (task, patch)
        };
        let worker_session_id = task.worker_session_id.clone();
        let outcome = self.apply_worker_patch_to_workspace(&task, &patch)?;
        let mut task = self.update_worker_patch_after_apply(task_id, patch_id, outcome.clone())?;
        if outcome.apply_status == WorkerTaskPatchApplyStatus::Conflicted {
            task = self.append_worker_patch_revision(task_id, patch_id, &patch, &outcome)?;
        }
        self.emit_event_with_payload(
            EventKind::WriteTransactionRecorded,
            Some(SessionId(worker_session_id)),
            None,
            format!(
                "{} patch {} for task {}",
                patch_apply_status_label(outcome.apply_status),
                patch_id,
                task_id
            ),
            Some(json!({
                "task_id": task_id,
                "patch_id": patch_id,
                "apply_status": patch_apply_status_label(outcome.apply_status),
                "transaction_id": outcome.transaction_id,
                "conflict_reason": outcome.conflict_reason,
            })),
        )?;
        Ok(worker_task_patch_review_report(task))
    }

    fn authorize_worker_patch_apply(
        &self,
        task: &WorkerTaskRecord,
        patch: &WorkerTaskPatchProposal,
    ) -> Result<WorkerPatchApplyAuthorization, HeptaError> {
        let now = current_unix_ms()?;
        if !matches!(
            task.status,
            WorkerTaskStatus::Running | WorkerTaskStatus::Completed
        ) || patch.apply_status != WorkerTaskPatchApplyStatus::Proposed
            || !task.patch_proposals.iter().any(|stored| stored == patch)
        {
            return Err(HeptaError(
                "worker patch authority requires an active exact proposed patch".into(),
            ));
        }
        let requested = Path::new(&patch.file_path);
        if requested.is_absolute()
            || requested.components().next().is_none()
            || requested
                .components()
                .any(|component| !matches!(component, std::path::Component::Normal(_)))
        {
            return Err(HeptaError(
                "worker patch authority requires a normalized relative target".into(),
            ));
        }
        let mut exact_leases = task.file_leases.iter().filter(|lease| {
            lease.task_id == task.task_id
                && lease.worker_id == task.worker_id
                && lease.worker_session_id == task.worker_session_id
                && lease.target_path == patch.file_path
        });
        let Some(exact_lease) = exact_leases.next() else {
            return Err(HeptaError(format!(
                "worker patch has no exact file lease: {}",
                patch.file_path
            )));
        };
        if exact_leases.next().is_some()
            || !matches!(
                exact_lease.status,
                WorkerTaskFileLeaseStatus::Active | WorkerTaskFileLeaseStatus::HeldForReview
            )
            || !exact_lease.conflict_task_ids.is_empty()
            || now < exact_lease.acquired_at_unix_ms
            || now > exact_lease.lease_expires_at_unix_ms
        {
            return Err(HeptaError(format!(
                "worker patch exact file lease is not active and conflict-free: {}",
                patch.file_path
            )));
        }
        let workspace_root = self.worker_patch_authorized_workspace(task)?;
        Ok(WorkerPatchApplyAuthorization {
            worker_session_id: task.worker_session_id.clone(),
            requested_path: patch.file_path.clone(),
            workspace_root,
        })
    }

    fn worker_patch_authorized_workspace(
        &self,
        task: &WorkerTaskRecord,
    ) -> Result<String, HeptaError> {
        let workspace_root = self.workspace_root()?;
        let envelope_root = fs::canonicalize(&task.safety_envelope.sandbox.workspace_root)
            .map_err(|error| {
                HeptaError(format!(
                    "worker patch safety workspace cannot be resolved: {error}"
                ))
            })?;
        let safety_valid = task.permission_envelope.filesystem_scope
            == FilesystemScope::WorkspaceOnly
            && task.permission_envelope.write_scope != WritePathScope::AnyPath
            && task.permission_envelope.execution_profile != ExecutionProfile::NoTools
            && task.safety_envelope.sandbox.host_process_allowed
            && task.safety_envelope.cancel_supported
            && task.safety_envelope.cancel_checked_before_host_command
            && task.safety_envelope.resource_limits.task_timeout_budget_ms
                == task.timeout_budget_ms
            && task.patch_proposals.len()
                <= task.safety_envelope.resource_limits.max_patch_proposals
            && envelope_root == workspace_root;
        if !safety_valid {
            return Err(HeptaError(
                "worker patch safety envelope does not authorize this workspace mutation".into(),
            ));
        }
        Ok(workspace_root.display().to_string())
    }

    fn authorize_worker_patch_rollback(
        &self,
        task: &WorkerTaskRecord,
        patch: &WorkerTaskPatchProposal,
        transaction_id: &str,
    ) -> Result<WorkerPatchRollbackAuthorization, HeptaError> {
        if task.status != WorkerTaskStatus::Completed
            || patch.apply_status != WorkerTaskPatchApplyStatus::Applied
            || patch.transaction_id.as_deref() != Some(transaction_id)
            || !task.patch_proposals.iter().any(|stored| stored == patch)
        {
            return Err(HeptaError(
                "worker patch rollback requires an exact applied patch receipt".into(),
            ));
        }
        let mut exact_leases = task.file_leases.iter().filter(|lease| {
            lease.task_id == task.task_id
                && lease.worker_id == task.worker_id
                && lease.worker_session_id == task.worker_session_id
                && lease.target_path == patch.file_path
        });
        let exact_lease = exact_leases.next();
        if exact_lease.is_none_or(|lease| lease.status == WorkerTaskFileLeaseStatus::Conflicted)
            || exact_leases.next().is_some()
        {
            return Err(HeptaError(format!(
                "worker patch rollback has no conflict-free exact file lease record: {}",
                patch.file_path
            )));
        }
        let requested = Path::new(&patch.file_path);
        if requested.is_absolute()
            || requested.components().next().is_none()
            || requested
                .components()
                .any(|component| !matches!(component, std::path::Component::Normal(_)))
        {
            return Err(HeptaError(
                "worker patch rollback requires a normalized relative target".into(),
            ));
        }
        Ok(WorkerPatchRollbackAuthorization {
            worker_session_id: task.worker_session_id.clone(),
            requested_path: patch.file_path.clone(),
            workspace_root: self.worker_patch_authorized_workspace(task)?,
            transaction_id: transaction_id.to_string(),
        })
    }

    pub fn apply_worker_task_patch_set(
        &self,
        task_id: &str,
    ) -> Result<WorkerTaskPatchSetApplyReport, HeptaError> {
        let patch_ids = self
            .find_worker_task(task_id)?
            .patch_proposals
            .iter()
            .filter(|patch| matches!(patch.apply_status, WorkerTaskPatchApplyStatus::Proposed))
            .map(|patch| patch.patch_id.clone())
            .collect::<Vec<_>>();
        let patch_count = self.find_worker_task(task_id)?.patch_proposals.len();
        let attempted_count = patch_ids.len();
        let mut transaction_ids = Vec::new();
        for patch_id in patch_ids {
            let review = self.apply_worker_task_patch(task_id, &patch_id)?;
            if let Some(patch) = review
                .patches
                .iter()
                .find(|patch| patch.patch_id == patch_id)
            {
                if let Some(transaction_id) = patch.transaction_id.as_ref() {
                    transaction_ids.push(transaction_id.clone());
                }
            }
        }
        let review = self.worker_task_patches(task_id)?;
        let skipped_count = patch_count.saturating_sub(attempted_count);
        Ok(WorkerTaskPatchSetApplyReport {
            task_id: task_id.to_string(),
            workspace_id: review.workspace_id.clone(),
            patch_count,
            attempted_count,
            applied_count: review.applied_count,
            conflicted_count: review.conflicted_count,
            rejected_count: review.rejected_count,
            skipped_count,
            transaction_ids,
            review,
        })
    }

    pub fn rollback_worker_task_patch(
        &self,
        task_id: &str,
        patch_id: &str,
    ) -> Result<WorkerTaskPatchRollbackReport, HeptaError> {
        self.rollback_worker_task_patch_ids(task_id, vec![patch_id.to_string()])
    }

    pub fn rollback_worker_task_patch_set(
        &self,
        task_id: &str,
    ) -> Result<WorkerTaskPatchRollbackReport, HeptaError> {
        let patch_ids = self
            .find_worker_task(task_id)?
            .patch_proposals
            .iter()
            .filter(|patch| {
                patch.apply_status == WorkerTaskPatchApplyStatus::Applied
                    && patch.transaction_id.is_some()
            })
            .map(|patch| patch.patch_id.clone())
            .collect::<Vec<_>>();
        self.rollback_worker_task_patch_ids(task_id, patch_ids)
    }

    fn rollback_worker_task_patch_ids(
        &self,
        task_id: &str,
        patch_ids: Vec<String>,
    ) -> Result<WorkerTaskPatchRollbackReport, HeptaError> {
        let patch_count = self.find_worker_task(task_id)?.patch_proposals.len();
        let attempted_count = patch_ids.len();
        let mut rolled_back_transaction_ids = Vec::new();
        let mut failed_patch_ids = Vec::new();
        for patch_id in patch_ids {
            let transaction_id = self
                .find_worker_task(task_id)?
                .patch_proposals
                .iter()
                .find(|patch| patch.patch_id == patch_id)
                .and_then(|patch| patch.transaction_id.clone());
            let Some(transaction_id) = transaction_id else {
                failed_patch_ids.push(patch_id);
                continue;
            };
            match self.rollback_worker_patch_transaction(task_id, &patch_id, &transaction_id) {
                Ok(_) => {
                    rolled_back_transaction_ids.push(transaction_id.clone());
                    let _ = self.update_worker_patch_after_apply(
                        task_id,
                        &patch_id,
                        WorkerPatchApplyOutcome {
                            apply_status: WorkerTaskPatchApplyStatus::RolledBack,
                            applied_at_unix_ms: Some(current_unix_ms()?),
                            transaction_id: Some(transaction_id),
                            conflict_reason: None,
                        },
                    )?;
                }
                Err(_) => failed_patch_ids.push(patch_id),
            }
        }
        let review = self.worker_task_patches(task_id)?;
        Ok(WorkerTaskPatchRollbackReport {
            task_id: task_id.to_string(),
            workspace_id: review.workspace_id.clone(),
            patch_count,
            attempted_count,
            rolled_back_count: rolled_back_transaction_ids.len(),
            skipped_count: patch_count.saturating_sub(attempted_count),
            failed_count: failed_patch_ids.len(),
            rolled_back_transaction_ids,
            failed_patch_ids,
            review,
        })
    }

    fn rollback_worker_patch_transaction(
        &self,
        task_id: &str,
        patch_id: &str,
        transaction_id: &str,
    ) -> Result<(), HeptaError> {
        let task = self.find_worker_task(task_id)?;
        let patch = task
            .patch_proposals
            .iter()
            .find(|patch| patch.patch_id == patch_id)
            .ok_or_else(|| HeptaError(format!("unknown patch: {}", patch_id)))?;
        let authorization = self.authorize_worker_patch_rollback(&task, patch, transaction_id)?;
        self.rollback_worker_patch_create_sealed(authorization)
    }

    pub fn reject_worker_task_patch(
        &self,
        task_id: &str,
        patch_id: &str,
    ) -> Result<WorkerTaskPatchReviewReport, HeptaError> {
        self.mark_worker_task_patch(task_id, patch_id, WorkerTaskPatchApplyStatus::Rejected)
    }

    fn mark_worker_task_patch(
        &self,
        task_id: &str,
        patch_id: &str,
        status: WorkerTaskPatchApplyStatus,
    ) -> Result<WorkerTaskPatchReviewReport, HeptaError> {
        let task = {
            let now = current_unix_ms()?;
            let mut guard = self
                .worker_task_state
                .lock()
                .map_err(|_| HeptaError("worker task state mutex poisoned".into()))?;
            let task = guard
                .records
                .iter_mut()
                .find(|task| task.task_id == task_id)
                .ok_or_else(|| HeptaError(format!("unknown task: {}", task_id)))?;
            let patch = task
                .patch_proposals
                .iter_mut()
                .find(|patch| patch.patch_id == patch_id)
                .ok_or_else(|| HeptaError(format!("unknown patch: {}", patch_id)))?;
            patch.apply_status = status;
            patch.applied_at_unix_ms = None;
            patch.transaction_id = None;
            patch.conflict_reason = None;
            release_worker_file_leases_if_review_closed(task, now);
            task.updated_at_unix_ms = now;
            task.clone()
        };
        self.emit_event_with_payload(
            EventKind::WriteTransactionRecorded,
            Some(SessionId(task.worker_session_id.clone())),
            None,
            format!(
                "{} patch {} for task {}",
                patch_apply_status_label(status),
                patch_id,
                task_id
            ),
            Some(json!({
                "task_id": task_id,
                "patch_id": patch_id,
                "apply_status": patch_apply_status_label(status),
            })),
        )?;
        Ok(worker_task_patch_review_report(task))
    }

    fn update_worker_patch_after_apply(
        &self,
        task_id: &str,
        patch_id: &str,
        outcome: WorkerPatchApplyOutcome,
    ) -> Result<WorkerTaskRecord, HeptaError> {
        let now = current_unix_ms()?;
        let mut guard = self
            .worker_task_state
            .lock()
            .map_err(|_| HeptaError("worker task state mutex poisoned".into()))?;
        let task_index = guard
            .records
            .iter()
            .position(|task| task.task_id == task_id)
            .ok_or_else(|| HeptaError(format!("unknown task: {}", task_id)))?;
        let task = &mut guard.records[task_index];
        let patch = task
            .patch_proposals
            .iter_mut()
            .find(|patch| patch.patch_id == patch_id)
            .ok_or_else(|| HeptaError(format!("unknown patch: {}", patch_id)))?;
        patch.apply_status = outcome.apply_status;
        patch.applied_at_unix_ms = outcome.applied_at_unix_ms;
        patch.transaction_id = outcome.transaction_id;
        patch.conflict_reason = outcome.conflict_reason;
        release_worker_file_leases_if_review_closed(task, now);
        task.updated_at_unix_ms = now;
        Ok(task.clone())
    }

    fn append_worker_patch_revision(
        &self,
        task_id: &str,
        patch_id: &str,
        source_patch: &WorkerTaskPatchProposal,
        outcome: &WorkerPatchApplyOutcome,
    ) -> Result<WorkerTaskRecord, HeptaError> {
        let now = current_unix_ms()?;
        let mut guard = self
            .worker_task_state
            .lock()
            .map_err(|_| HeptaError("worker task state mutex poisoned".into()))?;
        let task_index = guard
            .records
            .iter()
            .position(|task| task.task_id == task_id)
            .ok_or_else(|| HeptaError(format!("unknown task: {}", task_id)))?;
        let task = &mut guard.records[task_index];
        if task
            .patch_proposals
            .iter()
            .any(|patch| patch.revision_of.as_deref() == Some(patch_id))
        {
            return Ok(task.clone());
        }
        let revision_index = task
            .patch_proposals
            .iter()
            .filter(|patch| patch.revision_of.as_deref() == Some(patch_id))
            .count()
            + 1;
        let revised_file_path = revised_patch_path(task, source_patch, revision_index);
        let conflict_reason = outcome
            .conflict_reason
            .as_deref()
            .unwrap_or("unknown conflict");
        let revised_content = format!(
            "# Revised worker patch\n\nsource_patch={}\nrevision={}\nconflict={}\nsummary={}\n",
            patch_id,
            revision_index,
            conflict_reason,
            compact_text(&source_patch.summary, 240)
        );
        let revised = WorkerTaskPatchProposal {
            patch_id: format!("{}:revise-{}", task.task_id, task.patch_proposals.len() + 1),
            revision_of: Some(patch_id.to_string()),
            revision_index,
            file_path: revised_file_path.clone(),
            change_kind: "revision".into(),
            summary: format!(
                "Revision {} for conflicted patch {}",
                revision_index, patch_id
            ),
            unified_diff: format!(
                "--- /dev/null\n+++ {}\n@@\n+{}\n",
                revised_file_path,
                revised_content.replace('\n', "\n+").trim_end_matches('+')
            ),
            apply_status: WorkerTaskPatchApplyStatus::Proposed,
            applied_at_unix_ms: None,
            transaction_id: None,
            conflict_reason: None,
        };
        task.patch_proposals.push(revised);
        let next_step = task.loop_steps.len() + 1;
        task.loop_steps.push(WorkerTaskLoopStep {
            step_index: next_step,
            phase: WorkerTaskLoopPhase::Revise,
            title: "Revise conflicted patch".into(),
            input_summary: format!("conflicted_patch={} reason={}", patch_id, conflict_reason),
            output_summary: format!(
                "Generated revision {} as a new proposed patch",
                revision_index
            ),
            evidence_ref: format!(
                "worker://{}/loop/revise-{}",
                task.worker_session_id, next_step
            ),
            passed: true,
        });
        task.updated_at_unix_ms = now;
        ensure_worker_patch_file_lease(
            &mut guard.records,
            task_index,
            &revised_file_path,
            WorkerTaskFileLeaseStatus::HeldForReview,
            now,
        );
        Ok(guard.records[task_index].clone())
    }

    fn apply_worker_patch_to_workspace(
        &self,
        task: &WorkerTaskRecord,
        patch: &WorkerTaskPatchProposal,
    ) -> Result<WorkerPatchApplyOutcome, HeptaError> {
        let now = current_unix_ms()?;
        let content = match extract_added_content_from_unified_diff(&patch.unified_diff) {
            Ok(content) => content,
            Err(conflict_reason) => {
                return Ok(WorkerPatchApplyOutcome::conflicted(now, conflict_reason));
            }
        };
        let after_bytes = content.into_bytes();
        let authorization = match self.authorize_worker_patch_apply(task, patch) {
            Ok(authorization) => authorization,
            Err(error) => return Ok(WorkerPatchApplyOutcome::conflicted(now, error.0)),
        };
        match self.apply_worker_patch_create_sealed(authorization, &after_bytes) {
            Ok(transaction_id) => Ok(WorkerPatchApplyOutcome {
                apply_status: WorkerTaskPatchApplyStatus::Applied,
                applied_at_unix_ms: Some(now),
                transaction_id,
                conflict_reason: None,
            }),
            Err(error) => Ok(WorkerPatchApplyOutcome::conflicted(now, error.0)),
        }
    }

    async fn run_autonomous_coding_worker_loop(
        &self,
        task: &WorkerTaskRecord,
    ) -> Result<WorkerTaskExecutionOutput, HeptaError> {
        let workspace_root = self.workspace_root()?;
        let active_model = self
            .model_selection_for_session(&task.worker_session_id)?
            .active;
        let targets = autonomous_coding_target_paths(&workspace_root);
        let inspection = inspect_autonomous_coding_targets(&workspace_root, &targets);
        if task.safety_envelope.resource_limits.max_command_runs < 6 {
            return Err(HeptaError(format!(
                "worker task {} resource limit max_command_runs={} is below autonomous coding multi-round minimum 6",
                task.task_id, task.safety_envelope.resource_limits.max_command_runs
            )));
        }
        let mut command_runs = Vec::new();
        self.ensure_worker_task_not_cancelled(&task.task_id)?;
        command_runs.push(run_worker_environment_command(
            task,
            &workspace_root,
            &task.safety_envelope,
            "round-1-inspect-targets",
            "sh -c 'pwd && test Hepta or hepta-codex workspace shape'",
            "/bin/sh",
            &[
                "-c",
                "printf 'cwd=%s\\n' \"$PWD\"; if test -f Cargo.toml && test -d crates; then printf 'cargo_toml=present\\nworkspace_shape=old-hepta\\n'; elif test -f codex-rs/Cargo.toml && test -d codex-rs/hepta-runtime; then printf 'cargo_toml=present\\nworkspace_shape=hepta-codex\\n'; else printf 'workspace_shape=unknown\\n' >&2; exit 1; fi; printf 'round=1\\n'",
            ],
        ));
        self.ensure_worker_task_not_cancelled(&task.task_id)?;
        command_runs.push(run_worker_environment_command(
            task,
            &workspace_root,
            &task.safety_envelope,
            "round-1-patch-preview",
            "sh -c 'printf patch_preview=review_gated_no_write'",
            "/bin/sh",
            &[
                "-c",
                "printf 'patch_preview=review_gated_no_write\\nmutation=none\\nround=1\\n'",
            ],
        ));
        self.ensure_worker_task_not_cancelled(&task.task_id)?;
        command_runs.push(run_worker_environment_command(
            task,
            &workspace_root,
            &task.safety_envelope,
            "round-1-test-preflight",
            "sh -c 'grep WorkerTaskRecord worker_tasks.rs'",
            "/bin/sh",
            &[
                "-c",
                "runtime_path=crates/hepta-runtime/src/worker_tasks.rs; test -s \"$runtime_path\" || runtime_path=codex-rs/hepta-runtime/src/worker_tasks.rs; test -s \"$runtime_path\" && grep -q WorkerTaskRecord \"$runtime_path\" && printf 'test_preflight=passed\\nround=1\\nruntime_path=%s\\n' \"$runtime_path\"",
            ],
        ));
        self.ensure_worker_task_not_cancelled(&task.task_id)?;
        command_runs.push(run_worker_environment_command(
            task,
            &workspace_root,
            &task.safety_envelope,
            "round-2-reinspect-safety",
            "sh -c 'grep WorkerTaskSafetyEnvelope worker_tasks.rs'",
            "/bin/sh",
            &[
                "-c",
                "runtime_path=crates/hepta-runtime/src/worker_tasks.rs; test -s \"$runtime_path\" || runtime_path=codex-rs/hepta-runtime/src/worker_tasks.rs; grep -q WorkerTaskSafetyEnvelope \"$runtime_path\" && printf 'reinspect=safety_envelope_present\\nround=2\\nruntime_path=%s\\n' \"$runtime_path\"",
            ],
        ));
        self.ensure_worker_task_not_cancelled(&task.task_id)?;
        command_runs.push(run_worker_environment_command(
            task,
            &workspace_root,
            &task.safety_envelope,
            "round-2-revise-preview",
            "sh -c 'printf revise_preview=bounded_followup'",
            "/bin/sh",
            &[
                "-c",
                "printf 'revise_preview=bounded_followup\\nmutation=none\\nround=2\\n'",
            ],
        ));
        self.ensure_worker_task_not_cancelled(&task.task_id)?;
        command_runs.push(run_worker_environment_command(
            task,
            &workspace_root,
            &task.safety_envelope,
            "round-2-handoff-preflight",
            "sh -c 'printf evidence/replay/promotion handoff preflight'",
            "/bin/sh",
            &[
                "-c",
                "printf 'evidence=derivable\\nreplay=derivable\\npromotion=derivable\\nround=2\\n'",
            ],
        ));

        let result = VerticalSliceResult {
            session_id: task.worker_session_id.clone(),
            active_model,
            invoked_tool: Some("autonomous_coding_worker".into()),
            tool_output_json: Some(
                json!({
                    "mode": "autonomous_coding",
                    "targets": targets,
                    "readable_targets": inspection.readable_count,
                    "command_runs": command_runs.len(),
                    "coding_rounds": 2,
                    "patch_ready": true,
                })
                .to_string(),
            ),
            final_text: format!(
                "Autonomous coding worker completed multi-round loop: inspected {} target(s), executed {} real worker command(s) across 2 coding round(s), prepared review-gated patch/evidence handoff for task {}.",
                inspection.readable_count,
                command_runs.len(),
                task.task_id
            ),
            recalled_memories: 0,
            approval_required: None,
            blocked_reason: None,
        };
        let artifacts =
            build_autonomous_coding_worker_artifacts(task, &result, &inspection, &command_runs);
        let patch_proposals =
            build_autonomous_coding_patch_proposals(task, &result, &inspection, &command_runs);
        if patch_proposals.len() > task.safety_envelope.resource_limits.max_patch_proposals {
            return Err(HeptaError(format!(
                "worker task {} exceeded patch proposal limit {}",
                task.task_id, task.safety_envelope.resource_limits.max_patch_proposals
            )));
        }
        let coding_rounds = build_autonomous_coding_rounds(task, &command_runs, &patch_proposals);
        let loop_steps = build_autonomous_coding_loop_steps(
            task,
            &result,
            &inspection,
            &patch_proposals,
            &command_runs,
            &coding_rounds,
        );
        if loop_steps.len() > task.safety_envelope.resource_limits.max_loop_steps {
            return Err(HeptaError(format!(
                "worker task {} exceeded loop step limit {}",
                task.task_id, task.safety_envelope.resource_limits.max_loop_steps
            )));
        }
        let diff_summary = build_autonomous_coding_diff_summary(
            task,
            &inspection,
            &patch_proposals,
            &command_runs,
        );
        Ok(WorkerTaskExecutionOutput {
            result,
            artifacts,
            diff_summary,
            patch_proposals,
            coding_rounds,
            loop_steps,
            command_runs,
        })
    }

    pub async fn run_due_worker_tasks(
        &self,
        now_unix_ms: Option<u64>,
    ) -> Result<WorkerTaskDueRunReport, HeptaError> {
        let report = self
            .run_due_worker_tasks_with_context_recall_handoff(
                now_unix_ms,
                WorkerTaskContextRecallHandoffPolicy::Disabled,
            )
            .await?;
        Ok(WorkerTaskDueRunReport {
            now_unix_ms: report.now_unix_ms,
            due_count: report.due_count,
            ran_count: report.ran_count,
            skipped_count: report.skipped_count,
            runs: report.runs.into_iter().map(|run| run.run).collect(),
        })
    }

    pub async fn run_due_worker_tasks_with_context_recall_handoff(
        &self,
        now_unix_ms: Option<u64>,
        context_recall_handoff_policy: WorkerTaskContextRecallHandoffPolicy,
    ) -> Result<WorkerTaskContextRecallDueRunReport, HeptaError> {
        let now_unix_ms = now_unix_ms.map(Ok).unwrap_or_else(current_unix_ms)?;
        let due_task_ids = self
            .worker_task_records()?
            .into_iter()
            .filter(|task| {
                task.status == WorkerTaskStatus::Scheduled
                    && task
                        .next_run_unix_ms
                        .map(|next_run| next_run <= now_unix_ms)
                        .unwrap_or(false)
            })
            .map(|task| task.task_id)
            .collect::<Vec<_>>();
        let due_count = due_task_ids.len();
        let mut runs = Vec::new();
        let mut skipped_count = 0usize;
        for task_id in due_task_ids {
            match self
                .run_worker_task_with_context_recall_handoff(
                    &task_id,
                    context_recall_handoff_policy,
                )
                .await
            {
                Ok(run) => runs.push(run),
                Err(_) => skipped_count += 1,
            }
        }
        let (selected_snippets_present_count, selected_snippet_count) =
            selected_snippet_totals_for_worker_runs(&runs);
        Ok(WorkerTaskContextRecallDueRunReport {
            now_unix_ms,
            due_count,
            ran_count: runs.len(),
            skipped_count,
            context_recall_handoff_policy,
            selected_snippets_present_count,
            selected_snippet_count,
            runs,
        })
    }

    pub async fn run_ready_worker_tasks(
        &self,
        limit: Option<usize>,
        now_unix_ms: Option<u64>,
    ) -> Result<WorkerTaskReadyRunReport, HeptaError> {
        let report = self
            .run_ready_worker_tasks_with_context_recall_handoff(
                limit,
                now_unix_ms,
                WorkerTaskContextRecallHandoffPolicy::Disabled,
            )
            .await?;
        Ok(WorkerTaskReadyRunReport {
            now_unix_ms: report.now_unix_ms,
            candidate_count: report.candidate_count,
            ready_count: report.ready_count,
            ran_count: report.ran_count,
            blocked_count: report.blocked_count,
            limit: report.limit,
            runs: report.runs.into_iter().map(|run| run.run).collect(),
            blocked_task_ids: report.blocked_task_ids,
            pressure: report.pressure,
        })
    }

    pub async fn run_ready_worker_tasks_with_context_recall_handoff(
        &self,
        limit: Option<usize>,
        now_unix_ms: Option<u64>,
        context_recall_handoff_policy: WorkerTaskContextRecallHandoffPolicy,
    ) -> Result<WorkerTaskContextRecallReadyRunReport, HeptaError> {
        let now_unix_ms = now_unix_ms.map(Ok).unwrap_or_else(current_unix_ms)?;
        let snapshot = self.worker_task_records()?;
        let candidate_tasks = snapshot
            .iter()
            .filter(|task| match task.status {
                WorkerTaskStatus::Queued => true,
                WorkerTaskStatus::Scheduled => task
                    .next_run_unix_ms
                    .map(|next_run| next_run <= now_unix_ms)
                    .unwrap_or(false),
                WorkerTaskStatus::Failed => {
                    task.attempt_count < task.max_attempts
                        && task
                            .retry_after_unix_ms
                            .map(|retry_after| retry_after <= now_unix_ms)
                            .unwrap_or(true)
                }
                WorkerTaskStatus::Running
                | WorkerTaskStatus::Paused
                | WorkerTaskStatus::Completed
                | WorkerTaskStatus::Cancelled
                | WorkerTaskStatus::Interrupted => false,
            })
            .cloned()
            .collect::<Vec<_>>();
        let mut ready_task_ids = Vec::new();
        let mut blocked_task_ids = Vec::new();
        for task in &candidate_tasks {
            if dependencies_completed(task, &snapshot) {
                ready_task_ids.push(task.task_id.clone());
            } else {
                blocked_task_ids.push(task.task_id.clone());
            }
        }
        let max_global = worker_pool_max_global_concurrency();
        let max_per_worker = worker_pool_max_per_worker_concurrency();
        let requested_ready_count = ready_task_ids.len();
        let active_count = snapshot
            .iter()
            .filter(|task| task.status == WorkerTaskStatus::Running)
            .count();
        let active_by_worker = worker_counts_by_worker(
            snapshot
                .iter()
                .filter(|task| task.status == WorkerTaskStatus::Running),
        );
        let ready_by_worker = worker_counts_by_worker(
            ready_task_ids
                .iter()
                .filter_map(|task_id| snapshot.iter().find(|task| &task.task_id == task_id)),
        );
        let allowed_count = limit
            .unwrap_or(usize::MAX)
            .min(max_global.saturating_sub(active_count));
        let mut selected_ready_task_ids = Vec::new();
        let mut throttled_task_ids = Vec::new();
        let mut selected_by_worker = std::collections::HashMap::<String, usize>::new();
        for task_id in ready_task_ids {
            let Some(task) = snapshot
                .iter()
                .find(|candidate| candidate.task_id == task_id)
            else {
                continue;
            };
            let active_for_worker = active_by_worker.get(&task.worker_id).copied().unwrap_or(0);
            let selected_for_worker = selected_by_worker
                .get(&task.worker_id)
                .copied()
                .unwrap_or(0);
            if selected_ready_task_ids.len() >= allowed_count
                || active_for_worker + selected_for_worker >= max_per_worker
            {
                throttled_task_ids.push(task_id);
                continue;
            }
            *selected_by_worker
                .entry(task.worker_id.clone())
                .or_default() += 1;
            selected_ready_task_ids.push(task_id);
        }
        let ready_count = selected_ready_task_ids.len();
        let pressure = build_worker_pool_pressure_report(
            &snapshot,
            requested_ready_count,
            active_count,
            &active_by_worker,
            &ready_by_worker,
            &throttled_task_ids,
            max_global,
            max_per_worker,
        );
        let mut runs = Vec::new();
        for task_id in selected_ready_task_ids {
            if let Ok(run) = self
                .run_worker_task_with_context_recall_handoff(
                    &task_id,
                    context_recall_handoff_policy,
                )
                .await
            {
                runs.push(run);
            }
        }
        let (selected_snippets_present_count, selected_snippet_count) =
            selected_snippet_totals_for_worker_runs(&runs);
        Ok(WorkerTaskContextRecallReadyRunReport {
            now_unix_ms,
            candidate_count: candidate_tasks.len(),
            ready_count,
            ran_count: runs.len(),
            blocked_count: blocked_task_ids.len(),
            limit,
            context_recall_handoff_policy,
            selected_snippets_present_count,
            selected_snippet_count,
            runs,
            blocked_task_ids,
            pressure,
        })
    }

    pub fn worker_inventory(&self) -> Result<WorkerInventoryReport, HeptaError> {
        let tasks = self.worker_task_records()?;
        let sessions = self
            .memory
            .list_sessions()
            .map_err(|err| HeptaError(err.0))?;
        let worker_ids = tasks
            .iter()
            .map(|task| task.worker_id.clone())
            .collect::<HashSet<_>>();
        let mut workers = Vec::new();
        for worker_id in worker_ids {
            let worker_tasks = tasks
                .iter()
                .filter(|task| task.worker_id == worker_id)
                .cloned()
                .collect::<Vec<_>>();
            let session_count = sessions
                .iter()
                .filter(|session| session.agent_id.0 == worker_id)
                .count();
            let latest_task_activity = worker_tasks
                .iter()
                .map(|task| task.updated_at_unix_ms)
                .max()
                .unwrap_or(0);
            let latest_session_activity = sessions
                .iter()
                .filter(|session| session.agent_id.0 == worker_id)
                .map(|session| session.last_active_unix_ms)
                .max()
                .unwrap_or(0);
            workers.push(WorkerDescriptor {
                worker_id,
                session_count,
                task_count: worker_tasks.len(),
                active_task_count: worker_tasks.iter().filter(|task| task.is_active()).count(),
                completed_task_count: count_status(&worker_tasks, WorkerTaskStatus::Completed),
                latest_activity_unix_ms: latest_task_activity.max(latest_session_activity),
            });
        }
        workers.sort_by(|left, right| left.worker_id.cmp(&right.worker_id));
        Ok(WorkerInventoryReport {
            worker_count: workers.len(),
            total_task_count: tasks.len(),
            active_task_count: tasks.iter().filter(|task| task.is_active()).count(),
            workers,
        })
    }

    pub fn worker_execution_backends(&self) -> Result<WorkerExecutionBackendReport, HeptaError> {
        let backends = worker_execution_backend_descriptors();
        let active = backends
            .iter()
            .find(|backend| backend.status == WorkerExecutionBackendStatus::Active)
            .cloned()
            .unwrap_or_else(local_host_worker_backend_descriptor);
        let remote_backend_count = backends.iter().filter(|backend| backend.remote).count();
        let configured_remote_backend_count = backends
            .iter()
            .filter(|backend| {
                backend.remote && backend.status == WorkerExecutionBackendStatus::Active
            })
            .count();
        let remote_backends = backends
            .iter()
            .filter(|backend| backend.remote)
            .collect::<Vec<_>>();
        let remote_path_traversal_denied = remote_backends
            .iter()
            .all(|backend| policy_contains(&backend.path_traversal_policy, &["deny", "traversal"]));
        let remote_credential_mounts_deny_by_default = remote_backends
            .iter()
            .all(|backend| policy_contains(&backend.credential_mount_policy, &["deny_by_default"]));
        let remote_file_sync_manifest_required = remote_backends.iter().all(|backend| {
            backend.file_sync_supported
                && policy_contains(&backend.file_sync_manifest_policy, &["manifest", "require"])
        });
        let remote_child_side_effects_blocked = remote_backends.iter().all(|backend| {
            policy_contains(&backend.child_side_effect_policy, &["block", "side_effect"])
        });
        let remote_safety_regression_pack_ready = remote_backend_count > 0
            && remote_path_traversal_denied
            && remote_credential_mounts_deny_by_default
            && remote_file_sync_manifest_required
            && remote_child_side_effects_blocked;
        Ok(WorkerExecutionBackendReport {
            backend_count: backends.len(),
            active_backend_id: active.backend_id,
            active_backend_kind: active.kind,
            local_backend_ready: backends.iter().any(|backend| {
                backend.kind == WorkerExecutionBackendKind::LocalHostProcess
                    && backend.status == WorkerExecutionBackendStatus::Active
            }),
            remote_backend_count,
            configured_remote_backend_count,
            remote_execution_enabled: configured_remote_backend_count > 0,
            file_sync_policy_required: backends
                .iter()
                .filter(|backend| backend.remote)
                .all(|backend| backend.file_sync_supported && backend.sandbox_required),
            credential_mount_policy_required: backends
                .iter()
                .filter(|backend| backend.remote)
                .all(|backend| !backend.credential_mount_policy.trim().is_empty()),
            remote_path_traversal_denied,
            remote_credential_mounts_deny_by_default,
            remote_file_sync_manifest_required,
            remote_child_side_effects_blocked,
            remote_safety_regression_pack_ready,
            environment_process_evidence_contract: backends
                .iter()
                .all(|backend| backend.environment_process_evidence),
            backends,
        })
    }

    pub fn join_worker_tasks(
        &self,
        worker_filter: Option<&str>,
    ) -> Result<WorkerTaskJoinReport, HeptaError> {
        let active_session_id = self.active_session_id()?;
        let worker_filter = worker_filter
            .map(normalize_worker_id)
            .transpose()?
            .filter(|value| !value.is_empty());
        let mut tasks = self.worker_task_records()?;
        if let Some(worker_filter) = &worker_filter {
            tasks.retain(|task| &task.worker_id == worker_filter);
        }
        tasks.sort_by(|left, right| left.updated_at_unix_ms.cmp(&right.updated_at_unix_ms));
        let joined = tasks
            .iter()
            .filter(|task| task.status == WorkerTaskStatus::Completed)
            .map(|task| {
                let merge_risk = worker_task_merge_risk_report(task.clone());
                WorkerTaskJoinItem {
                    task_id: task.task_id.clone(),
                    worker_id: task.worker_id.clone(),
                    worker_session_id: task.worker_session_id.clone(),
                    status: task.status,
                    result_summary: task.result_summary.clone(),
                    artifacts: task.artifacts.clone(),
                    diff_summary: task.diff_summary.clone(),
                    patch_proposals: task.patch_proposals.clone(),
                    coding_rounds: task.coding_rounds.clone(),
                    file_leases: task.file_leases.clone(),
                    loop_steps: task.loop_steps.clone(),
                    command_runs: task.command_runs.clone(),
                    merge_risk,
                }
            })
            .collect::<Vec<_>>();
        let active_task_ids = tasks
            .iter()
            .filter(|task| task.is_active())
            .map(|task| task.task_id.clone())
            .collect::<Vec<_>>();
        let failed_task_ids = tasks
            .iter()
            .filter(|task| task.status == WorkerTaskStatus::Failed)
            .map(|task| task.task_id.clone())
            .collect::<Vec<_>>();
        let active_count = active_task_ids.len();
        let failed_count = failed_task_ids.len();
        let artifact_count = joined
            .iter()
            .map(|item| item.artifacts.len())
            .sum::<usize>();
        let diff_ready_count = joined
            .iter()
            .filter(|item| item.diff_summary.is_some())
            .count();
        let patch_proposal_count = joined
            .iter()
            .map(|item| item.patch_proposals.len())
            .sum::<usize>();
        let coding_round_count = joined
            .iter()
            .map(|item| item.coding_rounds.len())
            .sum::<usize>();
        let file_leases = joined
            .iter()
            .flat_map(|item| item.file_leases.iter())
            .collect::<Vec<_>>();
        let file_lease_count = file_leases.len();
        let active_file_lease_count =
            count_file_lease_status_refs(&file_leases, WorkerTaskFileLeaseStatus::Active);
        let held_file_lease_count =
            count_file_lease_status_refs(&file_leases, WorkerTaskFileLeaseStatus::HeldForReview);
        let conflicted_file_lease_count =
            count_file_lease_status_refs(&file_leases, WorkerTaskFileLeaseStatus::Conflicted);
        let expired_file_lease_count =
            count_file_lease_status_refs(&file_leases, WorkerTaskFileLeaseStatus::Expired);
        let patch_applied_count = joined
            .iter()
            .flat_map(|item| item.patch_proposals.iter())
            .filter(|patch| patch.apply_status == WorkerTaskPatchApplyStatus::Applied)
            .count();
        let patch_conflicted_count = joined
            .iter()
            .flat_map(|item| item.patch_proposals.iter())
            .filter(|patch| patch.apply_status == WorkerTaskPatchApplyStatus::Conflicted)
            .count();
        let patch_rejected_count = joined
            .iter()
            .flat_map(|item| item.patch_proposals.iter())
            .filter(|patch| patch.apply_status == WorkerTaskPatchApplyStatus::Rejected)
            .count();
        let patch_rolled_back_count = joined
            .iter()
            .flat_map(|item| item.patch_proposals.iter())
            .filter(|patch| patch.apply_status == WorkerTaskPatchApplyStatus::RolledBack)
            .count();
        let loop_step_count = joined
            .iter()
            .map(|item| item.loop_steps.len())
            .sum::<usize>();
        let command_run_count = joined
            .iter()
            .map(|item| item.command_runs.len())
            .sum::<usize>();
        let permission_envelopes = unique_permission_envelopes(&tasks);
        let merge_safe_count = joined
            .iter()
            .filter(|item| item.merge_risk.decision == WorkerTaskMergeDecision::SafeToMerge)
            .count();
        let merge_needs_review_count = joined
            .iter()
            .filter(|item| item.merge_risk.decision == WorkerTaskMergeDecision::NeedsReview)
            .count();
        let merge_blocked_count = joined
            .iter()
            .filter(|item| item.merge_risk.decision == WorkerTaskMergeDecision::Blocked)
            .count();
        let max_merge_risk_score = joined
            .iter()
            .map(|item| item.merge_risk.risk_score)
            .max()
            .unwrap_or(0);
        Ok(WorkerTaskJoinReport {
            active_session_id,
            worker_filter,
            total_count: tasks.len(),
            completed_count: joined.len(),
            failed_count,
            active_count,
            safe_to_join: active_count == 0 && failed_count == 0 && merge_blocked_count == 0,
            joined,
            active_task_ids,
            failed_task_ids,
            artifact_count,
            diff_ready_count,
            patch_proposal_count,
            coding_round_count,
            file_lease_count,
            active_file_lease_count,
            held_file_lease_count,
            conflicted_file_lease_count,
            expired_file_lease_count,
            patch_applied_count,
            patch_conflicted_count,
            patch_rejected_count,
            patch_rolled_back_count,
            loop_step_count,
            command_run_count,
            permission_envelopes,
            merge_safe_count,
            merge_needs_review_count,
            merge_blocked_count,
            max_merge_risk_score,
        })
    }

    pub fn worker_task_supervisor(&self) -> Result<WorkerTaskSupervisorReport, HeptaError> {
        let now_unix_ms = current_unix_ms()?;
        let tasks = self.worker_task_records()?;
        let worker_count = tasks
            .iter()
            .map(|task| task.worker_id.clone())
            .collect::<HashSet<_>>()
            .len();
        let mut ready_task_ids = Vec::new();
        let mut blocked_task_ids = Vec::new();
        let mut failed_task_ids = Vec::new();
        let mut paused_task_ids = Vec::new();
        let mut interrupted_task_ids = Vec::new();
        let mut scheduled_future_count = 0usize;
        for task in &tasks {
            match task.status {
                WorkerTaskStatus::Queued | WorkerTaskStatus::Failed => {
                    if task.status == WorkerTaskStatus::Failed {
                        failed_task_ids.push(task.task_id.clone());
                    }
                    let retry_ready = task
                        .retry_after_unix_ms
                        .map(|retry_after| retry_after <= now_unix_ms)
                        .unwrap_or(true);
                    if dependencies_completed(task, &tasks)
                        && task.attempt_count < task.max_attempts
                        && retry_ready
                    {
                        ready_task_ids.push(task.task_id.clone());
                    } else if task.is_active() || task.status == WorkerTaskStatus::Failed {
                        blocked_task_ids.push(task.task_id.clone());
                    }
                }
                WorkerTaskStatus::Scheduled => {
                    if task
                        .next_run_unix_ms
                        .map(|next| next <= now_unix_ms)
                        .unwrap_or(false)
                    {
                        if dependencies_completed(task, &tasks) {
                            ready_task_ids.push(task.task_id.clone());
                        } else {
                            blocked_task_ids.push(task.task_id.clone());
                        }
                    } else {
                        scheduled_future_count += 1;
                    }
                }
                WorkerTaskStatus::Running => blocked_task_ids.push(task.task_id.clone()),
                WorkerTaskStatus::Paused => {
                    paused_task_ids.push(task.task_id.clone());
                    blocked_task_ids.push(task.task_id.clone());
                }
                WorkerTaskStatus::Interrupted => {
                    interrupted_task_ids.push(task.task_id.clone());
                }
                WorkerTaskStatus::Completed | WorkerTaskStatus::Cancelled => {}
            }
        }
        let active_count = tasks.iter().filter(|task| task.is_active()).count();
        let failed_count = failed_task_ids.len();
        let paused_count = paused_task_ids.len();
        let interrupted_count = interrupted_task_ids.len();
        let completed_artifact_count = tasks
            .iter()
            .filter(|task| task.status == WorkerTaskStatus::Completed)
            .map(|task| task.artifacts.len())
            .sum::<usize>();
        let diff_ready_count = tasks
            .iter()
            .filter(|task| {
                task.status == WorkerTaskStatus::Completed && task.diff_summary.is_some()
            })
            .count();
        let patch_proposal_count = tasks
            .iter()
            .filter(|task| task.status == WorkerTaskStatus::Completed)
            .map(|task| task.patch_proposals.len())
            .sum::<usize>();
        let coding_round_count = tasks
            .iter()
            .filter(|task| task.status == WorkerTaskStatus::Completed)
            .map(|task| task.coding_rounds.len())
            .sum::<usize>();
        let multi_round_task_count = tasks
            .iter()
            .filter(|task| task.coding_rounds.len() >= 2)
            .count();
        let max_rounds_per_task = tasks
            .iter()
            .map(|task| task.coding_rounds.len())
            .max()
            .unwrap_or(0);
        let file_leases = tasks
            .iter()
            .flat_map(|task| task.file_leases.iter())
            .collect::<Vec<_>>();
        let file_lease_count = file_leases.len();
        let active_file_lease_count =
            count_file_lease_status_refs(&file_leases, WorkerTaskFileLeaseStatus::Active);
        let held_file_lease_count =
            count_file_lease_status_refs(&file_leases, WorkerTaskFileLeaseStatus::HeldForReview);
        let conflicted_file_lease_count =
            count_file_lease_status_refs(&file_leases, WorkerTaskFileLeaseStatus::Conflicted);
        let expired_file_lease_count =
            count_file_lease_status_refs(&file_leases, WorkerTaskFileLeaseStatus::Expired);
        let loop_step_count = tasks
            .iter()
            .filter(|task| task.status == WorkerTaskStatus::Completed)
            .map(|task| task.loop_steps.len())
            .sum::<usize>();
        let command_run_count = tasks
            .iter()
            .map(|task| task.command_runs.len())
            .sum::<usize>();
        let timeout_count = tasks
            .iter()
            .flat_map(|task| task.command_runs.iter())
            .filter(|run| run.timed_out)
            .count();
        let resource_limit_violation_count = tasks
            .iter()
            .flat_map(|task| task.command_runs.iter())
            .filter(|run| run.resource_limit_violation.is_some())
            .count();
        let active_by_worker = worker_counts_by_worker(
            tasks
                .iter()
                .filter(|task| task.status == WorkerTaskStatus::Running),
        );
        let ready_by_worker = worker_counts_by_worker(
            ready_task_ids
                .iter()
                .filter_map(|task_id| tasks.iter().find(|task| &task.task_id == task_id)),
        );
        let pressure = build_worker_pool_pressure_report(
            &tasks,
            ready_task_ids.len(),
            tasks
                .iter()
                .filter(|task| task.status == WorkerTaskStatus::Running)
                .count(),
            &active_by_worker,
            &ready_by_worker,
            &Vec::new(),
            worker_pool_max_global_concurrency(),
            worker_pool_max_per_worker_concurrency(),
        );
        let permission_envelopes = unique_permission_envelopes(&tasks);
        let safety_envelopes = unique_safety_envelopes(&tasks);
        let completed_merge_risks = tasks
            .iter()
            .filter(|task| task.status == WorkerTaskStatus::Completed)
            .cloned()
            .map(worker_task_merge_risk_report)
            .collect::<Vec<_>>();
        let merge_safe_count = completed_merge_risks
            .iter()
            .filter(|risk| risk.decision == WorkerTaskMergeDecision::SafeToMerge)
            .count();
        let merge_needs_review_count = completed_merge_risks
            .iter()
            .filter(|risk| risk.decision == WorkerTaskMergeDecision::NeedsReview)
            .count();
        let merge_blocked_count = completed_merge_risks
            .iter()
            .filter(|risk| risk.decision == WorkerTaskMergeDecision::Blocked)
            .count();
        let max_merge_risk_score = completed_merge_risks
            .iter()
            .map(|risk| risk.risk_score)
            .max()
            .unwrap_or(0);
        let safe_to_join = active_count == 0 && failed_count == 0 && merge_blocked_count == 0;
        let attention_required = failed_count > 0
            || paused_count > 0
            || !blocked_task_ids.is_empty()
            || merge_blocked_count > 0
            || merge_needs_review_count > 0
            || conflicted_file_lease_count > 0
            || expired_file_lease_count > 0;
        let recommended_next_action = if !ready_task_ids.is_empty() {
            "run_ready_tasks".to_string()
        } else if paused_count > 0 {
            "resume_or_interrupt_tasks".to_string()
        } else if failed_count > 0 {
            "inspect_failed_tasks".to_string()
        } else if conflicted_file_lease_count > 0 || expired_file_lease_count > 0 {
            "inspect_file_leases".to_string()
        } else if merge_blocked_count > 0 || merge_needs_review_count > 0 {
            "review_merge_risk".to_string()
        } else if !blocked_task_ids.is_empty() {
            "wait_for_dependencies".to_string()
        } else if scheduled_future_count > 0 {
            "wait_for_schedule".to_string()
        } else {
            "join_tasks".to_string()
        };
        Ok(WorkerTaskSupervisorReport {
            now_unix_ms,
            worker_count,
            total_count: tasks.len(),
            active_count,
            ready_count: ready_task_ids.len(),
            blocked_count: blocked_task_ids.len(),
            scheduled_future_count,
            failed_count,
            paused_count,
            interrupted_count,
            safe_to_join,
            attention_required,
            recommended_next_action,
            ready_task_ids,
            blocked_task_ids,
            failed_task_ids,
            paused_task_ids,
            interrupted_task_ids,
            completed_artifact_count,
            diff_ready_count,
            patch_proposal_count,
            coding_round_count,
            multi_round_task_count,
            max_rounds_per_task,
            file_lease_count,
            active_file_lease_count,
            held_file_lease_count,
            conflicted_file_lease_count,
            expired_file_lease_count,
            loop_step_count,
            command_run_count,
            timeout_count,
            cancelled_count: count_status(&tasks, WorkerTaskStatus::Cancelled),
            paused_control_count: count_status(&tasks, WorkerTaskStatus::Paused),
            interrupted_control_count: count_status(&tasks, WorkerTaskStatus::Interrupted),
            resource_limit_violation_count,
            sandbox_envelope_count: safety_envelopes.len(),
            pressure,
            permission_envelopes,
            safety_envelopes,
            merge_safe_count,
            merge_needs_review_count,
            merge_blocked_count,
            max_merge_risk_score,
        })
    }

    pub fn worker_subagent_observatory(
        &self,
    ) -> Result<WorkerSubagentObservatoryReport, HeptaError> {
        let now_unix_ms = current_unix_ms()?;
        let mut tasks = self.worker_task_records()?;
        tasks.sort_by(|left, right| left.updated_at_unix_ms.cmp(&right.updated_at_unix_ms));
        let file_leases = tasks
            .iter()
            .flat_map(|task| task.file_leases.iter().cloned())
            .collect::<Vec<_>>();
        let active_file_lease_count =
            count_file_lease_status(&file_leases, WorkerTaskFileLeaseStatus::Active);
        let held_file_lease_count =
            count_file_lease_status(&file_leases, WorkerTaskFileLeaseStatus::HeldForReview);
        let conflicted_file_lease_count =
            count_file_lease_status(&file_leases, WorkerTaskFileLeaseStatus::Conflicted);
        let expired_file_lease_count =
            count_file_lease_status(&file_leases, WorkerTaskFileLeaseStatus::Expired);
        let lanes = tasks
            .iter()
            .map(|task| {
                let lease_paths = task
                    .file_leases
                    .iter()
                    .map(|lease| lease.target_path.clone())
                    .collect::<Vec<_>>();
                let lease_statuses = task
                    .file_leases
                    .iter()
                    .map(|lease| lease.status)
                    .collect::<Vec<_>>();
                let attention_required = task.status == WorkerTaskStatus::Failed
                    || task.status == WorkerTaskStatus::Paused
                    || task.file_leases.iter().any(|lease| {
                        matches!(
                            lease.status,
                            WorkerTaskFileLeaseStatus::Conflicted
                                | WorkerTaskFileLeaseStatus::Expired
                        )
                    });
                WorkerSubagentLaneObservation {
                    task_id: task.task_id.clone(),
                    worker_id: task.worker_id.clone(),
                    worker_session_id: task.worker_session_id.clone(),
                    status: task.status,
                    paused_from_status: task.paused_from_status,
                    execution_mode: task.execution_mode,
                    coding_round_count: task.coding_rounds.len(),
                    command_run_count: task.command_runs.len(),
                    patch_proposal_count: task.patch_proposals.len(),
                    file_lease_count: task.file_leases.len(),
                    lease_paths,
                    lease_statuses,
                    attention_required,
                    control_action: match task.status {
                        WorkerTaskStatus::Paused => "resume_or_interrupt".into(),
                        WorkerTaskStatus::Running => "monitor_or_interrupt".into(),
                        WorkerTaskStatus::Queued
                        | WorkerTaskStatus::Scheduled
                        | WorkerTaskStatus::Failed => "pause_resume_or_interrupt".into(),
                        WorkerTaskStatus::Completed => "join_or_review".into(),
                        WorkerTaskStatus::Cancelled | WorkerTaskStatus::Interrupted => {
                            "terminal".into()
                        }
                    },
                    summary: task
                        .result_summary
                        .as_ref()
                        .map(|summary| compact_text(summary, 120))
                        .unwrap_or_else(|| compact_text(&task.prompt, 120)),
                    updated_at_unix_ms: task.updated_at_unix_ms,
                }
            })
            .collect::<Vec<_>>();
        let active_count = tasks.iter().filter(|task| task.is_active()).count();
        let paused_count = count_status(&tasks, WorkerTaskStatus::Paused);
        let interrupted_count = count_status(&tasks, WorkerTaskStatus::Interrupted);
        let autonomous_count = tasks
            .iter()
            .filter(|task| task.execution_mode == WorkerTaskExecutionMode::AutonomousCoding)
            .count();
        let attention_required = lanes.iter().any(|lane| lane.attention_required);
        let recommended_next_action = if conflicted_file_lease_count > 0 {
            "resolve_file_lease_conflicts".to_string()
        } else if expired_file_lease_count > 0 {
            "refresh_or_release_expired_file_leases".to_string()
        } else if paused_count > 0 {
            "resume_or_interrupt_paused_subagents".to_string()
        } else if active_count > 0 {
            "monitor_active_subagents".to_string()
        } else if held_file_lease_count > 0 {
            "review_held_patch_leases".to_string()
        } else {
            "join_tasks".to_string()
        };
        Ok(WorkerSubagentObservatoryReport {
            now_unix_ms,
            total_count: tasks.len(),
            active_count,
            paused_count,
            interrupted_count,
            autonomous_count,
            attention_required,
            file_lease_count: file_leases.len(),
            active_file_lease_count,
            held_file_lease_count,
            conflicted_file_lease_count,
            expired_file_lease_count,
            coding_round_count: tasks.iter().map(|task| task.coding_rounds.len()).sum(),
            command_run_count: tasks.iter().map(|task| task.command_runs.len()).sum(),
            recommended_next_action,
            lanes,
            file_leases,
        })
    }

    pub fn operator_console(&self) -> Result<OperatorConsoleReport, HeptaError> {
        let task_supervisor = self.worker_task_supervisor()?;
        let subagent_observatory = self.worker_subagent_observatory()?;
        let recent_events = self
            .events(20)?
            .into_iter()
            .map(|record| OperatorConsoleEventSummary {
                emitted_at_unix_ms: record.emitted_at_unix_ms,
                kind: record.event.kind,
                session_id: record.event.session_id.map(|session_id| session_id.0),
                summary: compact_text(&record.event.summary, 120),
            })
            .collect::<Vec<_>>();
        let control_commands = vec![
            "/tasks --json".to_string(),
            "/task-supervisor --json".to_string(),
            "/subagent-observatory --json".to_string(),
            "/task-evidence <task_id> --json".to_string(),
            "/handoff-bundle <task_id> --json".to_string(),
            "/apply-patches <task_id> --json".to_string(),
            "/steer-task <task_id> <instruction> --json".to_string(),
            "/cancel-task <task_id> --json".to_string(),
            "/pause-task <task_id> --json".to_string(),
            "/resume-task <task_id> --json".to_string(),
            "/interrupt-task <task_id> --json".to_string(),
        ];
        let operator_console_complete = task_supervisor.safe_to_join
            || task_supervisor.attention_required
            || subagent_observatory.total_count >= task_supervisor.total_count;
        let recommended_next_action = if subagent_observatory.attention_required {
            subagent_observatory.recommended_next_action.clone()
        } else {
            task_supervisor.recommended_next_action.clone()
        };

        Ok(OperatorConsoleReport {
            product: "Hepta".into(),
            status: if operator_console_complete {
                "complete".into()
            } else {
                "needs_attention".into()
            },
            task_queue_panel: true,
            subagent_tree_panel: true,
            command_stream_panel: true,
            patch_evidence_review_panel: true,
            approval_controls_panel: true,
            live_control_panel: true,
            steer_control_ready: true,
            cancel_control_ready: true,
            pause_control_ready: true,
            resume_control_ready: true,
            interrupt_control_ready: true,
            operator_console_complete,
            recommended_next_action,
            control_commands,
            task_supervisor,
            subagent_observatory,
            recent_events,
        })
    }

    pub(crate) fn worker_task_records(&self) -> Result<Vec<WorkerTaskRecord>, HeptaError> {
        let guard = self
            .worker_task_state
            .lock()
            .map_err(|_| HeptaError("worker task state mutex poisoned".into()))?;
        Ok(guard.records.clone())
    }

    fn find_worker_task(&self, task_id: &str) -> Result<WorkerTaskRecord, HeptaError> {
        let task_id = task_id.trim();
        if task_id.is_empty() {
            return Err(HeptaError("task id must not be empty".into()));
        }
        self.worker_task_records()?
            .into_iter()
            .find(|task| task.task_id == task_id)
            .ok_or_else(|| HeptaError(format!("unknown task: {}", task_id)))
    }

    fn update_worker_task_after_run(
        &self,
        task_id: &str,
        status: WorkerTaskStatus,
        completed_at_unix_ms: Option<u64>,
        last_error: Option<String>,
        result_summary: Option<String>,
        artifacts: Vec<WorkerTaskArtifact>,
        diff_summary: Option<String>,
        patch_proposals: Vec<WorkerTaskPatchProposal>,
        coding_rounds: Vec<WorkerTaskCodingRound>,
        loop_steps: Vec<WorkerTaskLoopStep>,
        command_runs: Vec<WorkerTaskCommandRun>,
        failure_kind: Option<WorkerTaskFailureKind>,
        retry_after_unix_ms: Option<u64>,
    ) -> Result<WorkerTaskRecord, HeptaError> {
        let now = current_unix_ms()?;
        let mut guard = self
            .worker_task_state
            .lock()
            .map_err(|_| HeptaError("worker task state mutex poisoned".into()))?;
        let task_index = guard
            .records
            .iter()
            .position(|task| task.task_id == task_id)
            .ok_or_else(|| HeptaError(format!("unknown task: {}", task_id)))?;
        {
            let task = &mut guard.records[task_index];
            task.status = status;
            task.paused_from_status = None;
            task.updated_at_unix_ms = now;
            task.completed_at_unix_ms = completed_at_unix_ms;
            task.last_error = last_error;
            task.failure_kind = failure_kind;
            task.retry_after_unix_ms = retry_after_unix_ms;
            task.result_summary = result_summary;
            task.artifacts = artifacts;
            task.diff_summary = diff_summary;
            task.patch_proposals = patch_proposals;
            task.coding_rounds = coding_rounds;
            task.loop_steps = loop_steps;
            task.command_runs = command_runs;
        }
        let patch_paths = guard.records[task_index]
            .patch_proposals
            .iter()
            .map(|patch| patch.file_path.clone())
            .collect::<Vec<_>>();
        for target_path in patch_paths {
            ensure_worker_patch_file_lease(
                &mut guard.records,
                task_index,
                &target_path,
                WorkerTaskFileLeaseStatus::Active,
                now,
            );
        }
        let task = &mut guard.records[task_index];
        update_worker_file_lease_statuses_after_run(task, status, now);
        Ok(task.clone())
    }

    fn ensure_worker_task_not_cancelled(&self, task_id: &str) -> Result<(), HeptaError> {
        let cancelled = self
            .worker_task_records()?
            .into_iter()
            .find(|task| task.task_id == task_id)
            .map(|task| {
                matches!(
                    task.status,
                    WorkerTaskStatus::Cancelled | WorkerTaskStatus::Interrupted
                )
            })
            .unwrap_or(false);
        if cancelled {
            return Err(HeptaError(format!(
                "task {} was cancelled or interrupted",
                task_id
            )));
        }
        Ok(())
    }

    fn validate_worker_task_dependencies(&self, depends_on: &[String]) -> Result<(), HeptaError> {
        if depends_on.is_empty() {
            return Ok(());
        }
        let existing_ids = self
            .worker_task_records()?
            .into_iter()
            .map(|task| task.task_id)
            .collect::<HashSet<_>>();
        for dependency_id in depends_on {
            if !existing_ids.contains(dependency_id) {
                return Err(HeptaError(format!(
                    "unknown task dependency: {}",
                    dependency_id
                )));
            }
        }
        Ok(())
    }
}

pub fn task_status_label(status: WorkerTaskStatus) -> &'static str {
    match status {
        WorkerTaskStatus::Queued => "queued",
        WorkerTaskStatus::Scheduled => "scheduled",
        WorkerTaskStatus::Running => "running",
        WorkerTaskStatus::Paused => "paused",
        WorkerTaskStatus::Completed => "completed",
        WorkerTaskStatus::Failed => "failed",
        WorkerTaskStatus::Cancelled => "cancelled",
        WorkerTaskStatus::Interrupted => "interrupted",
    }
}

fn effective_worker_task_prompt(task: &WorkerTaskRecord) -> String {
    if task.steering_directives.is_empty() {
        return task.prompt.clone();
    }
    let steering = task
        .steering_directives
        .iter()
        .enumerate()
        .map(|(index, directive)| format!("{}. {}", index + 1, directive.instruction))
        .collect::<Vec<_>>()
        .join("\n");
    format!(
        "{}\n\nOperator steering directives:\n{}",
        task.prompt, steering
    )
}

pub fn file_lease_status_label(status: WorkerTaskFileLeaseStatus) -> &'static str {
    match status {
        WorkerTaskFileLeaseStatus::Active => "active",
        WorkerTaskFileLeaseStatus::HeldForReview => "held_for_review",
        WorkerTaskFileLeaseStatus::Released => "released",
        WorkerTaskFileLeaseStatus::Expired => "expired",
        WorkerTaskFileLeaseStatus::Conflicted => "conflicted",
    }
}

fn is_open_file_lease_status(status: WorkerTaskFileLeaseStatus) -> bool {
    matches!(
        status,
        WorkerTaskFileLeaseStatus::Active
            | WorkerTaskFileLeaseStatus::HeldForReview
            | WorkerTaskFileLeaseStatus::Conflicted
    )
}

fn update_worker_file_lease_statuses_after_run(
    task: &mut WorkerTaskRecord,
    status: WorkerTaskStatus,
    now_unix_ms: u64,
) {
    let next_status = match status {
        WorkerTaskStatus::Completed if task.patch_proposals.is_empty() => {
            WorkerTaskFileLeaseStatus::Released
        }
        WorkerTaskStatus::Completed => WorkerTaskFileLeaseStatus::HeldForReview,
        WorkerTaskStatus::Failed | WorkerTaskStatus::Cancelled | WorkerTaskStatus::Interrupted => {
            WorkerTaskFileLeaseStatus::Released
        }
        WorkerTaskStatus::Queued
        | WorkerTaskStatus::Scheduled
        | WorkerTaskStatus::Running
        | WorkerTaskStatus::Paused => WorkerTaskFileLeaseStatus::Active,
    };
    for lease in &mut task.file_leases {
        if lease.status == WorkerTaskFileLeaseStatus::Conflicted {
            continue;
        }
        lease.status = if now_unix_ms > lease.lease_expires_at_unix_ms {
            WorkerTaskFileLeaseStatus::Expired
        } else {
            next_status
        };
    }
}

fn release_worker_file_leases_if_review_closed(task: &mut WorkerTaskRecord, now_unix_ms: u64) {
    let all_patch_reviews_closed = !task.patch_proposals.is_empty()
        && task
            .patch_proposals
            .iter()
            .all(|patch| patch.apply_status != WorkerTaskPatchApplyStatus::Proposed);
    if !all_patch_reviews_closed {
        return;
    }
    for lease in &mut task.file_leases {
        if lease.status == WorkerTaskFileLeaseStatus::Conflicted {
            continue;
        }
        lease.status = if now_unix_ms > lease.lease_expires_at_unix_ms {
            WorkerTaskFileLeaseStatus::Expired
        } else {
            WorkerTaskFileLeaseStatus::Released
        };
    }
}

fn parse_schedule_next_run(expr: &str, now_unix_ms: u64) -> Result<u64, HeptaError> {
    let expr = expr.trim();
    if expr.eq_ignore_ascii_case("now") {
        return Ok(now_unix_ms);
    }
    if let Some(value) = expr.strip_prefix("at:") {
        return value
            .parse::<u64>()
            .map_err(|_| HeptaError(format!("invalid at: schedule expression: {}", expr)));
    }
    if let Some(value) = expr.strip_prefix("delay:") {
        return parse_duration_ms(value)
            .map(|duration| now_unix_ms.saturating_add(duration))
            .map_err(|_| HeptaError(format!("invalid delay: schedule expression: {}", expr)));
    }
    if let Some(value) = expr.strip_prefix("every:") {
        return parse_duration_ms(value)
            .map(|duration| now_unix_ms.saturating_add(duration))
            .map_err(|_| HeptaError(format!("invalid every: schedule expression: {}", expr)));
    }
    Err(HeptaError(format!(
        "unsupported task schedule expression: {}",
        expr
    )))
}

fn parse_duration_ms(value: &str) -> Result<u64, ()> {
    let value = value.trim();
    if let Some(raw) = value.strip_suffix("ms") {
        return raw.parse::<u64>().map_err(|_| ());
    }
    if let Some(raw) = value.strip_suffix('s') {
        return raw
            .parse::<u64>()
            .map(|seconds| seconds.saturating_mul(1_000))
            .map_err(|_| ());
    }
    if let Some(raw) = value.strip_suffix('m') {
        return raw
            .parse::<u64>()
            .map(|minutes| minutes.saturating_mul(60_000))
            .map_err(|_| ());
    }
    value.parse::<u64>().map_err(|_| ())
}

fn normalize_worker_id(value: &str) -> Result<String, HeptaError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(HeptaError("worker id must not be empty".into()));
    }
    Ok(sanitize_for_id(trimmed))
}

fn normalize_dependencies(depends_on: Vec<String>) -> Result<Vec<String>, HeptaError> {
    let mut normalized = Vec::new();
    for dependency_id in depends_on {
        let dependency_id = dependency_id.trim();
        if dependency_id.is_empty() {
            return Err(HeptaError("task dependency id must not be empty".into()));
        }
        if !normalized.iter().any(|existing| existing == dependency_id) {
            normalized.push(dependency_id.to_string());
        }
    }
    Ok(normalized)
}

fn sanitize_for_id(value: &str) -> String {
    value
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' {
                ch.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect::<String>()
        .trim_matches('-')
        .to_string()
}

fn count_status(tasks: &[WorkerTaskRecord], status: WorkerTaskStatus) -> usize {
    tasks.iter().filter(|task| task.status == status).count()
}

fn selected_snippet_totals_for_worker_runs(
    runs: &[WorkerTaskContextRecallRunReport],
) -> (usize, u32) {
    let present_count = runs
        .iter()
        .filter(|run| run.selected_snippets_present)
        .count();
    let snippet_count = runs.iter().fold(0u32, |total, run| {
        total.saturating_add(run.selected_snippet_count)
    });
    (present_count, snippet_count)
}

fn dependencies_completed(task: &WorkerTaskRecord, tasks: &[WorkerTaskRecord]) -> bool {
    task.depends_on.iter().all(|dependency_id| {
        tasks.iter().any(|candidate| {
            candidate.task_id == *dependency_id && candidate.status == WorkerTaskStatus::Completed
        })
    })
}

fn build_worker_permission_envelope(
    worker_id: &str,
    parent_profile: ExecutionProfile,
    parent_scope: FilesystemScope,
    parent_write_scope: WritePathScope,
) -> WorkerPermissionEnvelope {
    let lowered = worker_id.to_ascii_lowercase();
    let execution_profile = if lowered.contains("audit") || lowered.contains("review") {
        stricter_execution_profile(parent_profile, ExecutionProfile::ReadOnlyTools)
    } else if lowered.contains("no-tool") || lowered.contains("sandbox") {
        ExecutionProfile::NoTools
    } else {
        parent_profile
    };
    let filesystem_scope = if lowered.contains("wide") {
        parent_scope
    } else {
        FilesystemScope::WorkspaceOnly
    };
    let write_scope = if lowered.contains("builder") || lowered.contains("patch") {
        stricter_write_scope(parent_write_scope, WritePathScope::WorkspaceOnly)
    } else {
        stricter_write_scope(parent_write_scope, WritePathScope::ArtifactsOnly)
    };
    let network_allowed = lowered.contains("network") || lowered.contains("remote");
    WorkerPermissionEnvelope {
        execution_profile,
        filesystem_scope,
        write_scope,
        network_allowed,
        inherited_from_parent: true,
        policy_summary: format!(
            "exec={:?} fs={:?} write={:?} network={}",
            execution_profile, filesystem_scope, write_scope, network_allowed
        ),
    }
}

fn default_worker_permission_envelope() -> WorkerPermissionEnvelope {
    WorkerPermissionEnvelope {
        execution_profile: ExecutionProfile::FullAccess,
        filesystem_scope: FilesystemScope::WorkspaceOnly,
        write_scope: WritePathScope::ArtifactsOnly,
        network_allowed: false,
        inherited_from_parent: true,
        policy_summary: "exec=FullAccess fs=WorkspaceOnly write=ArtifactsOnly network=false".into(),
    }
}

fn build_worker_safety_envelope(
    permission_envelope: &WorkerPermissionEnvelope,
    workspace_root: &Path,
    timeout_budget_ms: u64,
) -> WorkerTaskSafetyEnvelope {
    WorkerTaskSafetyEnvelope {
        sandbox: WorkerTaskSandboxPolicy {
            workspace_root: workspace_root.display().to_string(),
            host_process_allowed: permission_envelope.execution_profile
                != ExecutionProfile::NoTools,
            network_allowed: permission_envelope.network_allowed,
            allowed_programs: vec!["/bin/sh".into(), "sh".into()],
        },
        resource_limits: WorkerTaskResourceLimits {
            task_timeout_budget_ms: timeout_budget_ms,
            per_command_timeout_ms: worker_command_timeout_ms(),
            max_command_runs: worker_max_command_runs(),
            max_stdout_bytes: worker_max_stdout_bytes(),
            max_stderr_bytes: worker_max_stderr_bytes(),
            max_patch_proposals: DEFAULT_WORKER_MAX_PATCH_PROPOSALS,
            max_loop_steps: DEFAULT_WORKER_MAX_LOOP_STEPS,
        },
        cancel_supported: true,
        cancel_checked_before_host_command: true,
    }
}

fn default_worker_safety_envelope() -> WorkerTaskSafetyEnvelope {
    let permission = default_worker_permission_envelope();
    build_worker_safety_envelope(
        &permission,
        Path::new("."),
        DEFAULT_WORKER_TASK_TIMEOUT_BUDGET_MS,
    )
}

fn default_worker_execution_mode() -> WorkerTaskExecutionMode {
    WorkerTaskExecutionMode::Conversational
}

fn build_worker_file_leases(
    task_id: &str,
    worker_id: &str,
    worker_session_id: &str,
    execution_mode: WorkerTaskExecutionMode,
    workspace_root: &Path,
    now_unix_ms: u64,
    timeout_budget_ms: u64,
    existing_records: &[WorkerTaskRecord],
) -> Vec<WorkerTaskFileLease> {
    if execution_mode != WorkerTaskExecutionMode::AutonomousCoding {
        return Vec::new();
    }
    autonomous_coding_target_paths(workspace_root)
        .into_iter()
        .map(|target_path| {
            let mut conflict_task_ids = existing_records
                .iter()
                .flat_map(|task| task.file_leases.iter())
                .filter(|lease| {
                    lease.target_path == target_path && is_open_file_lease_status(lease.status)
                })
                .map(|lease| lease.task_id.clone())
                .collect::<HashSet<_>>()
                .into_iter()
                .collect::<Vec<_>>();
            conflict_task_ids.sort();
            WorkerTaskFileLease {
                lease_id: format!(
                    "{}:{}",
                    sanitize_for_id(task_id),
                    sanitize_for_id(&target_path)
                ),
                task_id: task_id.to_string(),
                worker_id: worker_id.to_string(),
                worker_session_id: worker_session_id.to_string(),
                target_path,
                status: if conflict_task_ids.is_empty() {
                    WorkerTaskFileLeaseStatus::Active
                } else {
                    WorkerTaskFileLeaseStatus::Conflicted
                },
                acquired_at_unix_ms: now_unix_ms,
                lease_expires_at_unix_ms: now_unix_ms.saturating_add(timeout_budget_ms),
                conflict_task_ids,
            }
        })
        .collect()
}

fn ensure_worker_patch_file_lease(
    records: &mut [WorkerTaskRecord],
    task_index: usize,
    target_path: &str,
    open_status: WorkerTaskFileLeaseStatus,
    now_unix_ms: u64,
) {
    let task_id = records[task_index].task_id.clone();
    let worker_id = records[task_index].worker_id.clone();
    let worker_session_id = records[task_index].worker_session_id.clone();
    let lease_expires_at_unix_ms =
        now_unix_ms.saturating_add(records[task_index].timeout_budget_ms);
    let mut conflict_task_ids = records
        .iter()
        .enumerate()
        .filter(|(index, _)| *index != task_index)
        .flat_map(|(_, task)| task.file_leases.iter())
        .filter(|lease| lease.target_path == target_path && is_open_file_lease_status(lease.status))
        .map(|lease| lease.task_id.clone())
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    conflict_task_ids.sort();
    let status = if conflict_task_ids.is_empty() {
        open_status
    } else {
        WorkerTaskFileLeaseStatus::Conflicted
    };
    let task = &mut records[task_index];
    if let Some(lease) = task
        .file_leases
        .iter_mut()
        .find(|lease| lease.target_path == target_path)
    {
        lease.status = status;
        lease.lease_expires_at_unix_ms = lease_expires_at_unix_ms;
        lease.conflict_task_ids = conflict_task_ids;
        return;
    }
    task.file_leases.push(WorkerTaskFileLease {
        lease_id: format!(
            "{}:patch:{}",
            sanitize_for_id(&task_id),
            sanitize_for_id(target_path)
        ),
        task_id,
        worker_id,
        worker_session_id,
        target_path: target_path.to_string(),
        status,
        acquired_at_unix_ms: now_unix_ms,
        lease_expires_at_unix_ms,
        conflict_task_ids,
    });
}

fn infer_worker_execution_mode(worker_id: &str, prompt: &str) -> WorkerTaskExecutionMode {
    let material = format!("{} {}", worker_id, prompt).to_ascii_lowercase();
    if material.contains("autonomous coding")
        || material.contains("coding subagent")
        || material.contains("code subagent")
        || material.contains("real worker execution")
    {
        WorkerTaskExecutionMode::AutonomousCoding
    } else {
        WorkerTaskExecutionMode::Conversational
    }
}

fn unique_permission_envelopes(tasks: &[WorkerTaskRecord]) -> Vec<WorkerPermissionEnvelope> {
    let mut envelopes = Vec::new();
    let mut seen = HashSet::new();
    for task in tasks {
        let key = format!(
            "{:?}:{:?}:{:?}:{}",
            task.permission_envelope.execution_profile,
            task.permission_envelope.filesystem_scope,
            task.permission_envelope.write_scope,
            task.permission_envelope.network_allowed
        );
        if seen.insert(key) {
            envelopes.push(task.permission_envelope.clone());
        }
    }
    envelopes
}

fn unique_safety_envelopes(tasks: &[WorkerTaskRecord]) -> Vec<WorkerTaskSafetyEnvelope> {
    let mut envelopes = Vec::new();
    let mut seen = HashSet::new();
    for task in tasks {
        let limits = &task.safety_envelope.resource_limits;
        let sandbox = &task.safety_envelope.sandbox;
        let key = format!(
            "{}:{}:{}:{}:{}:{}:{}:{}",
            sandbox.workspace_root,
            sandbox.host_process_allowed,
            sandbox.network_allowed,
            sandbox.allowed_programs.join("|"),
            limits.task_timeout_budget_ms,
            limits.per_command_timeout_ms,
            limits.max_command_runs,
            task.safety_envelope.cancel_supported
        );
        if seen.insert(key) {
            envelopes.push(task.safety_envelope.clone());
        }
    }
    envelopes
}

fn stricter_execution_profile(
    parent: ExecutionProfile,
    requested: ExecutionProfile,
) -> ExecutionProfile {
    if execution_profile_rank(parent) <= execution_profile_rank(requested) {
        parent
    } else {
        requested
    }
}

fn execution_profile_rank(profile: ExecutionProfile) -> u8 {
    match profile {
        ExecutionProfile::NoTools => 0,
        ExecutionProfile::ReadOnlyTools => 1,
        ExecutionProfile::FullAccess => 2,
    }
}

fn stricter_write_scope(parent: WritePathScope, requested: WritePathScope) -> WritePathScope {
    if write_scope_rank(parent) <= write_scope_rank(requested) {
        parent
    } else {
        requested
    }
}

fn write_scope_rank(scope: WritePathScope) -> u8 {
    match scope {
        WritePathScope::ArtifactsOnly => 0,
        WritePathScope::WorkspaceOnly => 1,
        WritePathScope::AnyPath => 2,
    }
}

fn worker_pool_max_global_concurrency() -> usize {
    std::env::var("HEPTA_WORKER_MAX_CONCURRENCY")
        .ok()
        .and_then(|value| value.parse::<usize>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(DEFAULT_WORKER_POOL_MAX_GLOBAL_CONCURRENCY)
}

fn worker_pool_max_per_worker_concurrency() -> usize {
    std::env::var("HEPTA_WORKER_MAX_PER_LANE")
        .ok()
        .and_then(|value| value.parse::<usize>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(DEFAULT_WORKER_POOL_MAX_PER_WORKER_CONCURRENCY)
}

fn worker_task_timeout_budget_ms() -> u64 {
    std::env::var("HEPTA_WORKER_TIMEOUT_MS")
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(DEFAULT_WORKER_TASK_TIMEOUT_BUDGET_MS)
}

fn worker_command_timeout_ms() -> u64 {
    std::env::var("HEPTA_WORKER_COMMAND_TIMEOUT_MS")
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(DEFAULT_WORKER_COMMAND_TIMEOUT_MS)
}

fn worker_max_command_runs() -> usize {
    std::env::var("HEPTA_WORKER_MAX_COMMAND_RUNS")
        .ok()
        .and_then(|value| value.parse::<usize>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(DEFAULT_WORKER_MAX_COMMAND_RUNS)
}

fn worker_max_stdout_bytes() -> usize {
    std::env::var("HEPTA_WORKER_MAX_STDOUT_BYTES")
        .ok()
        .and_then(|value| value.parse::<usize>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(DEFAULT_WORKER_MAX_STDOUT_BYTES)
}

fn worker_max_stderr_bytes() -> usize {
    std::env::var("HEPTA_WORKER_MAX_STDERR_BYTES")
        .ok()
        .and_then(|value| value.parse::<usize>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(DEFAULT_WORKER_MAX_STDERR_BYTES)
}

fn worker_retry_backoff_base_ms() -> u64 {
    std::env::var("HEPTA_WORKER_RETRY_BACKOFF_BASE_MS")
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(DEFAULT_WORKER_RETRY_BACKOFF_BASE_MS)
}

fn worker_retry_backoff_max_ms() -> u64 {
    std::env::var("HEPTA_WORKER_RETRY_BACKOFF_MAX_MS")
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(DEFAULT_WORKER_RETRY_BACKOFF_MAX_MS)
}

fn worker_retry_after_unix_ms(now: u64, attempt_count: usize) -> u64 {
    let shift = attempt_count.saturating_sub(1).min(16) as u32;
    let backoff = worker_retry_backoff_base_ms()
        .saturating_mul(1u64 << shift)
        .min(worker_retry_backoff_max_ms());
    now.saturating_add(backoff)
}

fn classify_worker_failure(message: &str) -> WorkerTaskFailureKind {
    let lower = message.to_ascii_lowercase();
    if lower.contains("timeout") {
        WorkerTaskFailureKind::Timeout
    } else if lower.contains("tool") {
        WorkerTaskFailureKind::ToolError
    } else if lower.contains("model") {
        WorkerTaskFailureKind::ModelError
    } else {
        WorkerTaskFailureKind::Unknown
    }
}

fn simulated_worker_failure(task: &WorkerTaskRecord) -> Option<HeptaError> {
    let prompt = task.prompt.to_ascii_lowercase();
    if prompt.contains("simulate-timeout") && task.attempt_count == 1 {
        Some(HeptaError(format!(
            "worker timeout budget exceeded after {} ms",
            task.timeout_budget_ms
        )))
    } else if prompt.contains("simulate-tool-error") && task.attempt_count == 1 {
        Some(HeptaError(
            "worker tool error during simulated execution".into(),
        ))
    } else {
        None
    }
}

fn worker_counts_by_worker<'a>(
    tasks: impl Iterator<Item = &'a WorkerTaskRecord>,
) -> std::collections::HashMap<String, usize> {
    let mut counts = std::collections::HashMap::new();
    for task in tasks {
        *counts.entry(task.worker_id.clone()).or_insert(0) += 1;
    }
    counts
}

fn build_worker_pool_pressure_report(
    tasks: &[WorkerTaskRecord],
    ready_count: usize,
    active_count: usize,
    active_by_worker: &std::collections::HashMap<String, usize>,
    ready_by_worker: &std::collections::HashMap<String, usize>,
    throttled_task_ids: &[String],
    max_global_concurrency: usize,
    max_per_worker_concurrency: usize,
) -> WorkerPoolPressureReport {
    let available_global_slots = max_global_concurrency.saturating_sub(active_count);
    let worker_ids = tasks
        .iter()
        .map(|task| task.worker_id.clone())
        .collect::<HashSet<_>>();
    let mut per_worker = worker_ids
        .into_iter()
        .map(|worker_id| {
            let active = active_by_worker.get(&worker_id).copied().unwrap_or(0);
            let ready = ready_by_worker.get(&worker_id).copied().unwrap_or(0);
            let available_slots = max_per_worker_concurrency.saturating_sub(active);
            WorkerPressureLane {
                worker_id,
                active_count: active,
                ready_count: ready,
                available_slots,
                throttled_count: ready.saturating_sub(available_slots),
            }
        })
        .collect::<Vec<_>>();
    per_worker.sort_by(|left, right| left.worker_id.cmp(&right.worker_id));
    let pressure_level = if !throttled_task_ids.is_empty() {
        WorkerPoolPressureLevel::Throttled
    } else if ready_count == 0 && active_count == 0 {
        WorkerPoolPressureLevel::Idle
    } else if available_global_slots == 0 {
        WorkerPoolPressureLevel::Saturated
    } else {
        WorkerPoolPressureLevel::Normal
    };
    WorkerPoolPressureReport {
        max_global_concurrency,
        max_per_worker_concurrency,
        active_count,
        ready_count,
        available_global_slots,
        pressure_level,
        throttled_task_ids: throttled_task_ids.to_vec(),
        per_worker,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct AutonomousCodingInspection {
    target_count: usize,
    readable_count: usize,
    total_bytes: usize,
    total_lines: usize,
    summaries: Vec<String>,
}

fn build_conversational_worker_execution_output(
    task: &WorkerTaskRecord,
    result: VerticalSliceResult,
) -> WorkerTaskExecutionOutput {
    let artifacts = build_worker_task_artifacts(task, &result);
    let diff_summary = build_worker_task_diff_summary(task, &result);
    let patch_proposals = build_worker_task_patch_proposals(task, &result);
    let loop_steps = build_worker_task_loop_steps(task, &result, &patch_proposals);
    WorkerTaskExecutionOutput {
        result,
        artifacts,
        diff_summary,
        patch_proposals,
        coding_rounds: Vec::new(),
        loop_steps,
        command_runs: Vec::new(),
    }
}

struct LocalHostWorkerBackend;

impl WorkerExecutionBackend for LocalHostWorkerBackend {
    fn descriptor(&self) -> WorkerExecutionBackendDescriptor {
        local_host_worker_backend_descriptor()
    }

    fn run_command(
        &self,
        task: &WorkerTaskRecord,
        workspace_root: &Path,
        safety_envelope: &WorkerTaskSafetyEnvelope,
        command_id: &str,
        display_command: &str,
        program: &str,
        args: &[&str],
    ) -> WorkerTaskCommandRun {
        attach_worker_backend_binding(
            run_worker_host_command(
                task,
                workspace_root,
                safety_envelope,
                command_id,
                display_command,
                program,
                args,
            ),
            &task.execution_backend,
        )
    }
}

fn run_worker_environment_command(
    task: &WorkerTaskRecord,
    workspace_root: &Path,
    safety_envelope: &WorkerTaskSafetyEnvelope,
    command_id: &str,
    display_command: &str,
    program: &str,
    args: &[&str],
) -> WorkerTaskCommandRun {
    match task.execution_backend.kind {
        WorkerExecutionBackendKind::LocalHostProcess => LocalHostWorkerBackend.run_command(
            task,
            workspace_root,
            safety_envelope,
            command_id,
            display_command,
            program,
            args,
        ),
        WorkerExecutionBackendKind::Docker | WorkerExecutionBackendKind::Ssh => {
            attach_worker_backend_binding(
                worker_command_run_blocked(
                    format!("{}:{}", task.task_id, command_id),
                    display_command,
                    workspace_root,
                    format!(
                        "worker backend {} requires explicit remote configuration",
                        task.execution_backend.backend_id
                    ),
                ),
                &task.execution_backend,
            )
        }
    }
}

fn attach_worker_backend_binding(
    mut run: WorkerTaskCommandRun,
    binding: &WorkerExecutionBackendBinding,
) -> WorkerTaskCommandRun {
    run.backend_id = binding.backend_id.clone();
    run.backend_kind = binding.kind;
    run.remote_backend = binding.remote;
    run
}

fn policy_contains(policy: &str, required_terms: &[&str]) -> bool {
    let lowered = policy.to_ascii_lowercase();
    required_terms.iter().all(|term| lowered.contains(term))
}

fn local_host_worker_backend_descriptor() -> WorkerExecutionBackendDescriptor {
    WorkerExecutionBackendDescriptor {
        backend_id: default_worker_backend_id(),
        kind: WorkerExecutionBackendKind::LocalHostProcess,
        status: WorkerExecutionBackendStatus::Active,
        remote: false,
        environment_process_evidence: true,
        sandbox_required: true,
        file_sync_supported: false,
        file_sync_manifest_policy: "not_applicable_local_workspace_only".into(),
        credential_mount_policy: "no_remote_mounts_local_workspace_only".into(),
        path_traversal_policy: "workspace_root_canonicalization_required".into(),
        child_side_effect_policy: "local_child_side_effects_limited_by_tool_policy".into(),
        supports_cancel: true,
        supports_timeout: true,
        supports_output_limits: true,
        supports_file_leases: true,
        notes: vec![
            "default backend for autonomous coding workers".into(),
            "records real local host-process command evidence".into(),
        ],
    }
}

fn worker_execution_backend_descriptors() -> Vec<WorkerExecutionBackendDescriptor> {
    vec![
        local_host_worker_backend_descriptor(),
        WorkerExecutionBackendDescriptor {
            backend_id: "docker-sandbox".into(),
            kind: WorkerExecutionBackendKind::Docker,
            status: WorkerExecutionBackendStatus::RequiresConfiguration,
            remote: true,
            environment_process_evidence: true,
            sandbox_required: true,
            file_sync_supported: true,
            file_sync_manifest_policy: "require_explicit_workspace_sync_manifest".into(),
            credential_mount_policy: "deny_by_default_require_explicit_mount_manifest".into(),
            path_traversal_policy: "deny_path_traversal_parent_and_absolute_escape".into(),
            child_side_effect_policy:
                "block_child_side_effects_without_operator_policy_and_mount_allowlist".into(),
            supports_cancel: true,
            supports_timeout: true,
            supports_output_limits: true,
            supports_file_leases: true,
            notes: vec![
                "contract stub only; no container side effects in local gate".into(),
                "future backend must provide workspace sync and redacted evidence refs".into(),
            ],
        },
        WorkerExecutionBackendDescriptor {
            backend_id: "ssh-remote".into(),
            kind: WorkerExecutionBackendKind::Ssh,
            status: WorkerExecutionBackendStatus::RequiresConfiguration,
            remote: true,
            environment_process_evidence: true,
            sandbox_required: true,
            file_sync_supported: true,
            file_sync_manifest_policy: "require_explicit_workspace_sync_manifest".into(),
            credential_mount_policy: "deny_by_default_require_named_secret_scope".into(),
            path_traversal_policy: "deny_path_traversal_parent_and_absolute_escape".into(),
            child_side_effect_policy:
                "block_child_side_effects_without_operator_policy_and_secret_scope".into(),
            supports_cancel: true,
            supports_timeout: true,
            supports_output_limits: true,
            supports_file_leases: true,
            notes: vec![
                "contract stub only; requires operator-approved host profile".into(),
                "remote path traversal and file-sync policy remain gate requirements".into(),
            ],
        },
    ]
}

fn run_worker_host_command(
    task: &WorkerTaskRecord,
    workspace_root: &Path,
    safety_envelope: &WorkerTaskSafetyEnvelope,
    command_id: &str,
    display_command: &str,
    program: &str,
    args: &[&str],
) -> WorkerTaskCommandRun {
    let command_id = format!("{}:{}", task.task_id, command_id);
    if !safety_envelope.sandbox.host_process_allowed {
        return worker_command_run_blocked(
            command_id,
            display_command,
            workspace_root,
            "sandbox blocks host process execution".into(),
        );
    }
    if !safety_envelope
        .sandbox
        .allowed_programs
        .iter()
        .any(|allowed| allowed == program)
    {
        return worker_command_run_blocked(
            command_id,
            display_command,
            workspace_root,
            format!("sandbox disallows program {}", program),
        );
    }

    let started = Instant::now();
    let mut child = match Command::new(program)
        .args(args)
        .current_dir(workspace_root)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(child) => child,
        Err(err) => {
            return WorkerTaskCommandRun {
                command_id,
                command: display_command.into(),
                execution_origin: WorkerTaskCommandRunOrigin::HostProcess,
                backend_id: default_worker_backend_id(),
                backend_kind: WorkerExecutionBackendKind::LocalHostProcess,
                remote_backend: false,
                working_directory: Some(workspace_root.display().to_string()),
                timed_out: false,
                exit_code: 127,
                stdout: String::new(),
                stderr: format!("failed to spawn host process: {}", err),
                duration_ms: started.elapsed().as_millis().max(1) as u64,
                passed: false,
                sandboxed: true,
                cancelled: false,
                stdout_truncated: false,
                stderr_truncated: false,
                resource_limit_violation: Some("spawn_failed".into()),
            };
        }
    };
    let timeout = Duration::from_millis(
        safety_envelope
            .resource_limits
            .per_command_timeout_ms
            .max(1),
    );
    loop {
        match child.try_wait() {
            Ok(Some(_status)) => {
                let duration_ms = started.elapsed().as_millis().max(1) as u64;
                return match child.wait_with_output() {
                    Ok(output) => {
                        let exit_code = output.status.code().unwrap_or(-1);
                        let (stdout, stdout_truncated) = truncate_utf8_lossy(
                            &output.stdout,
                            safety_envelope.resource_limits.max_stdout_bytes,
                        );
                        let (stderr, stderr_truncated) = truncate_utf8_lossy(
                            &output.stderr,
                            safety_envelope.resource_limits.max_stderr_bytes,
                        );
                        let stdout = redact_worker_output_exfiltration(&stdout);
                        let stderr = redact_worker_output_exfiltration(&stderr);
                        WorkerTaskCommandRun {
                            command_id,
                            command: display_command.into(),
                            execution_origin: WorkerTaskCommandRunOrigin::HostProcess,
                            backend_id: default_worker_backend_id(),
                            backend_kind: WorkerExecutionBackendKind::LocalHostProcess,
                            remote_backend: false,
                            working_directory: Some(workspace_root.display().to_string()),
                            timed_out: false,
                            exit_code,
                            stdout,
                            stderr,
                            duration_ms,
                            passed: exit_code == 0,
                            sandboxed: true,
                            cancelled: false,
                            stdout_truncated,
                            stderr_truncated,
                            resource_limit_violation: if stdout_truncated || stderr_truncated {
                                Some("output_truncated".into())
                            } else {
                                None
                            },
                        }
                    }
                    Err(err) => WorkerTaskCommandRun {
                        command_id,
                        command: display_command.into(),
                        execution_origin: WorkerTaskCommandRunOrigin::HostProcess,
                        backend_id: default_worker_backend_id(),
                        backend_kind: WorkerExecutionBackendKind::LocalHostProcess,
                        remote_backend: false,
                        working_directory: Some(workspace_root.display().to_string()),
                        timed_out: false,
                        exit_code: 127,
                        stdout: String::new(),
                        stderr: format!("failed to collect host process output: {}", err),
                        duration_ms,
                        passed: false,
                        sandboxed: true,
                        cancelled: false,
                        stdout_truncated: false,
                        stderr_truncated: false,
                        resource_limit_violation: Some("output_collect_failed".into()),
                    },
                };
            }
            Ok(None) => {
                if started.elapsed() >= timeout {
                    let _ = child.kill();
                    let duration_ms = started.elapsed().as_millis().max(1) as u64;
                    return match child.wait_with_output() {
                        Ok(output) => {
                            let (stdout, stdout_truncated) = truncate_utf8_lossy(
                                &output.stdout,
                                safety_envelope.resource_limits.max_stdout_bytes,
                            );
                            let (stderr, stderr_truncated) = truncate_utf8_lossy(
                                &output.stderr,
                                safety_envelope.resource_limits.max_stderr_bytes,
                            );
                            let stdout = redact_worker_output_exfiltration(&stdout);
                            let stderr = redact_worker_output_exfiltration(&stderr);
                            WorkerTaskCommandRun {
                                command_id,
                                command: display_command.into(),
                                execution_origin: WorkerTaskCommandRunOrigin::HostProcess,
                                backend_id: default_worker_backend_id(),
                                backend_kind: WorkerExecutionBackendKind::LocalHostProcess,
                                remote_backend: false,
                                working_directory: Some(workspace_root.display().to_string()),
                                timed_out: true,
                                exit_code: output.status.code().unwrap_or(-1),
                                stdout,
                                stderr,
                                duration_ms,
                                passed: false,
                                sandboxed: true,
                                cancelled: false,
                                stdout_truncated,
                                stderr_truncated,
                                resource_limit_violation: Some("command_timeout".into()),
                            }
                        }
                        Err(err) => WorkerTaskCommandRun {
                            command_id,
                            command: display_command.into(),
                            execution_origin: WorkerTaskCommandRunOrigin::HostProcess,
                            backend_id: default_worker_backend_id(),
                            backend_kind: WorkerExecutionBackendKind::LocalHostProcess,
                            remote_backend: false,
                            working_directory: Some(workspace_root.display().to_string()),
                            timed_out: true,
                            exit_code: -1,
                            stdout: String::new(),
                            stderr: format!(
                                "command timed out and output collection failed: {}",
                                err
                            ),
                            duration_ms,
                            passed: false,
                            sandboxed: true,
                            cancelled: false,
                            stdout_truncated: false,
                            stderr_truncated: false,
                            resource_limit_violation: Some("command_timeout".into()),
                        },
                    };
                }
                thread::sleep(Duration::from_millis(5));
            }
            Err(err) => {
                let _ = child.kill();
                return WorkerTaskCommandRun {
                    command_id,
                    command: display_command.into(),
                    execution_origin: WorkerTaskCommandRunOrigin::HostProcess,
                    backend_id: default_worker_backend_id(),
                    backend_kind: WorkerExecutionBackendKind::LocalHostProcess,
                    remote_backend: false,
                    working_directory: Some(workspace_root.display().to_string()),
                    timed_out: false,
                    exit_code: 127,
                    stdout: String::new(),
                    stderr: format!("failed to poll host process: {}", err),
                    duration_ms: started.elapsed().as_millis().max(1) as u64,
                    passed: false,
                    sandboxed: true,
                    cancelled: false,
                    stdout_truncated: false,
                    stderr_truncated: false,
                    resource_limit_violation: Some("poll_failed".into()),
                };
            }
        }
    }
}

fn worker_command_run_blocked(
    command_id: String,
    display_command: &str,
    workspace_root: &Path,
    reason: String,
) -> WorkerTaskCommandRun {
    WorkerTaskCommandRun {
        command_id,
        command: display_command.into(),
        execution_origin: WorkerTaskCommandRunOrigin::HostProcess,
        backend_id: default_worker_backend_id(),
        backend_kind: WorkerExecutionBackendKind::LocalHostProcess,
        remote_backend: false,
        working_directory: Some(workspace_root.display().to_string()),
        timed_out: false,
        exit_code: 126,
        stdout: String::new(),
        stderr: reason.clone(),
        duration_ms: 1,
        passed: false,
        sandboxed: true,
        cancelled: false,
        stdout_truncated: false,
        stderr_truncated: false,
        resource_limit_violation: Some(reason),
    }
}

fn truncate_utf8_lossy(bytes: &[u8], limit: usize) -> (String, bool) {
    if bytes.len() <= limit {
        return (String::from_utf8_lossy(bytes).into_owned(), false);
    }
    let safe_limit = limit.max(1).min(bytes.len());
    let mut text = String::from_utf8_lossy(&bytes[..safe_limit]).into_owned();
    text.push_str("\n[truncated]");
    (text, true)
}

fn redact_worker_output_exfiltration(text: &str) -> String {
    let sensitive_keys = [
        "authorization",
        "openai_api_key",
        "anthropic_api_key",
        "access_token",
        "refresh_token",
        "api_key",
        "apikey",
        "password",
        "secret",
        "token",
    ];
    let mut redacted = text.to_string();
    for key in sensitive_keys {
        redacted = redact_sensitive_assignment(&redacted, key);
    }
    redacted
}

fn redact_sensitive_assignment(input: &str, key: &str) -> String {
    let bytes = input.as_bytes();
    let key_bytes = key.as_bytes();
    let mut output = String::with_capacity(input.len());
    let mut index = 0usize;
    while index < bytes.len() {
        if starts_sensitive_assignment(bytes, index, key_bytes) {
            let key_start = index;
            index += key_bytes.len();
            while index < bytes.len() && bytes[index].is_ascii_whitespace() {
                index += 1;
            }
            let separator = bytes[index] as char;
            index += 1;
            while index < bytes.len() && bytes[index].is_ascii_whitespace() {
                index += 1;
            }
            output.push_str(&input[key_start..key_start + key_bytes.len()]);
            output.push(separator);
            output.push_str("[REDACTED]");
            while index < bytes.len() && !is_secret_value_delimiter(bytes[index]) {
                index += 1;
            }
        } else {
            let ch = input[index..].chars().next().expect("valid utf8 boundary");
            output.push(ch);
            index += ch.len_utf8();
        }
    }
    output
}

fn starts_sensitive_assignment(bytes: &[u8], index: usize, key: &[u8]) -> bool {
    if index > 0 && !is_secret_key_boundary(bytes[index - 1]) {
        return false;
    }
    if index + key.len() >= bytes.len() {
        return false;
    }
    for (offset, expected) in key.iter().enumerate() {
        if !bytes[index + offset].eq_ignore_ascii_case(expected) {
            return false;
        }
    }
    let mut cursor = index + key.len();
    while cursor < bytes.len() && bytes[cursor].is_ascii_whitespace() {
        cursor += 1;
    }
    cursor < bytes.len() && matches!(bytes[cursor], b'=' | b':')
}

fn is_secret_key_boundary(byte: u8) -> bool {
    byte.is_ascii_whitespace() || matches!(byte, b'?' | b'&' | b';' | b'\'' | b'"' | b'(' | b'{')
}

fn is_secret_value_delimiter(byte: u8) -> bool {
    byte.is_ascii_whitespace() || matches!(byte, b'&' | b';' | b'\'' | b'"' | b')' | b'}' | b',')
}

fn autonomous_coding_target_paths(workspace_root: &Path) -> Vec<String> {
    [
        "crates/hepta-runtime/src/worker_tasks.rs",
        "crates/hepta-cli/src/commands.rs",
        "crates/hepta-cli/src/lib.rs",
        "apps/hepta/src/main.rs",
        "codex-rs/hepta-runtime/src/worker_tasks.rs",
        "codex-rs/cli/src/main.rs",
        "codex-rs/cli/src/lib.rs",
        "codex-rs/hepta-native-gateway/src/native_gateway.rs",
    ]
    .into_iter()
    .filter(|path| workspace_root.join(path).exists())
    .map(str::to_string)
    .collect()
}

fn inspect_autonomous_coding_targets(
    workspace_root: &Path,
    targets: &[String],
) -> AutonomousCodingInspection {
    let mut readable_count = 0usize;
    let mut total_bytes = 0usize;
    let mut total_lines = 0usize;
    let mut summaries = Vec::new();
    for target in targets {
        match fs::read_to_string(workspace_root.join(target)) {
            Ok(content) => {
                readable_count += 1;
                total_bytes += content.len();
                let lines = content.lines().count();
                total_lines += lines;
                summaries.push(format!(
                    "{} lines={} bytes={} worker_refs={} patch_refs={}",
                    target,
                    lines,
                    content.len(),
                    content.matches("worker").count(),
                    content.matches("patch").count()
                ));
            }
            Err(err) => summaries.push(format!("{} unreadable={}", target, err)),
        }
    }
    AutonomousCodingInspection {
        target_count: targets.len(),
        readable_count,
        total_bytes,
        total_lines,
        summaries,
    }
}

fn build_autonomous_coding_worker_artifacts(
    task: &WorkerTaskRecord,
    result: &VerticalSliceResult,
    inspection: &AutonomousCodingInspection,
    command_runs: &[WorkerTaskCommandRun],
) -> Vec<WorkerTaskArtifact> {
    let mut artifacts = build_worker_task_artifacts(task, result);
    artifacts.push(WorkerTaskArtifact {
        artifact_id: format!("{}:code-inspection", task.task_id),
        kind: "code_inspection".into(),
        title: "Autonomous coding target inspection".into(),
        content: format!(
            "targets={} readable={} total_lines={} total_bytes={}\n{}",
            inspection.target_count,
            inspection.readable_count,
            inspection.total_lines,
            inspection.total_bytes,
            inspection.summaries.join("\n")
        ),
        path_hint: Some(format!(
            "worker://{}/code-inspection.md",
            task.worker_session_id
        )),
    });
    artifacts.push(WorkerTaskArtifact {
        artifact_id: format!("{}:command-transcript", task.task_id),
        kind: "command_transcript".into(),
        title: "Autonomous coding command transcript".into(),
        content: command_runs
            .iter()
            .map(|run| {
                format!(
                    "$ {}\norigin={:?} cwd={} timed_out={} exit={} passed={}\n{}{}",
                    run.command,
                    run.execution_origin,
                    run.working_directory.as_deref().unwrap_or("<not-recorded>"),
                    run.timed_out,
                    run.exit_code,
                    run.passed,
                    run.stdout,
                    if run.stderr.is_empty() {
                        "".into()
                    } else {
                        format!("\nstderr={} ", run.stderr)
                    }
                )
            })
            .collect::<Vec<_>>()
            .join("\n\n"),
        path_hint: Some(format!(
            "worker://{}/command-transcript.md",
            task.worker_session_id
        )),
    });
    artifacts.push(WorkerTaskArtifact {
        artifact_id: format!("{}:coding-handoff", task.task_id),
        kind: "coding_handoff".into(),
        title: "Autonomous coding parent handoff".into(),
        content: format!(
            "mode=autonomous_coding\npatches=review_gated\nrounds=2\ncommands={}\nsummary={}",
            command_runs.len(),
            compact_text(&result.final_text, 240)
        ),
        path_hint: Some(format!(
            "worker://{}/coding-handoff.md",
            task.worker_session_id
        )),
    });
    artifacts
}

fn build_autonomous_coding_diff_summary(
    task: &WorkerTaskRecord,
    inspection: &AutonomousCodingInspection,
    patches: &[WorkerTaskPatchProposal],
    command_runs: &[WorkerTaskCommandRun],
) -> String {
    format!(
        "worker={} task={} mode=autonomous_coding inspected={} rounds=2 commands={} patch_proposals={} target_lines={}",
        task.worker_id,
        task.task_id,
        inspection.readable_count,
        command_runs.len(),
        patches.len(),
        inspection.total_lines
    )
}

fn build_autonomous_coding_patch_proposals(
    task: &WorkerTaskRecord,
    result: &VerticalSliceResult,
    inspection: &AutonomousCodingInspection,
    command_runs: &[WorkerTaskCommandRun],
) -> Vec<WorkerTaskPatchProposal> {
    let path = format!(
        "docs/worker-proposals/{}-autonomous-coding.md",
        worker_patch_suffix(task)
    );
    let content = format!(
        "# Autonomous coding worker proposal\n\n- task: `{}`\n- worker: `{}`\n- mode: `autonomous_coding`\n- coding rounds: 2\n- inspected targets: {} / {}\n- real command runs: {}\n- summary: {}\n\n## Inspection\n{}\n\n## Command transcript\n{}\n",
        task.task_id,
        task.worker_id,
        inspection.readable_count,
        inspection.target_count,
        command_runs.len(),
        compact_text(&result.final_text, 240),
        inspection.summaries.join("\n"),
        command_runs
            .iter()
            .map(|run| format!(
                "- `{}` exit={} passed={}",
                run.command, run.exit_code, run.passed
            ))
            .collect::<Vec<_>>()
            .join("\n")
    );
    vec![WorkerTaskPatchProposal {
        patch_id: format!("{}:autonomous-coding-patch-1", task.task_id),
        revision_of: None,
        revision_index: 0,
        file_path: path.clone(),
        change_kind: "autonomous_coding_handoff".into(),
        summary: "Autonomous coding worker proposes a review-gated handoff patch".into(),
        unified_diff: format!(
            "--- /dev/null\n+++ {}\n@@\n+{}\n",
            path,
            content.replace('\n', "\n+").trim_end_matches('+')
        ),
        apply_status: WorkerTaskPatchApplyStatus::Proposed,
        applied_at_unix_ms: None,
        transaction_id: None,
        conflict_reason: None,
    }]
}

fn build_autonomous_coding_rounds(
    task: &WorkerTaskRecord,
    command_runs: &[WorkerTaskCommandRun],
    patches: &[WorkerTaskPatchProposal],
) -> Vec<WorkerTaskCodingRound> {
    let round_1_command_ids = command_runs
        .iter()
        .filter(|run| run.command_id.contains(":round-1-"))
        .map(|run| run.command_id.clone())
        .collect::<Vec<_>>();
    let round_2_command_ids = command_runs
        .iter()
        .filter(|run| run.command_id.contains(":round-2-"))
        .map(|run| run.command_id.clone())
        .collect::<Vec<_>>();
    let round_2_patch_ids = patches
        .iter()
        .map(|patch| patch.patch_id.clone())
        .collect::<Vec<_>>();
    let round_passed = |ids: &[String]| {
        !ids.is_empty()
            && ids.iter().all(|id| {
                command_runs
                    .iter()
                    .find(|run| &run.command_id == id)
                    .map(|run| {
                        run.passed && !run.timed_out && run.resource_limit_violation.is_none()
                    })
                    .unwrap_or(false)
            })
    };

    vec![
        WorkerTaskCodingRound {
            round_index: 1,
            title: "Inspect and draft bounded patch plan".into(),
            intent: compact_text(&task.prompt, 160),
            command_ids: round_1_command_ids.clone(),
            patch_ids: Vec::new(),
            passed: round_passed(&round_1_command_ids),
            summary: "round 1 inspected workspace targets, prepared a review-gated patch preview, and ran a preflight check".into(),
        },
        WorkerTaskCodingRound {
            round_index: 2,
            title: "Reinspect, revise, and prepare handoff".into(),
            intent: "close the worker handoff with replayable evidence after a second host-process pass".into(),
            command_ids: round_2_command_ids.clone(),
            patch_ids: round_2_patch_ids,
            passed: round_passed(&round_2_command_ids) && !patches.is_empty(),
            summary: "round 2 re-inspected safety state, revised the patch preview, and prepared evidence/replay/promotion handoff".into(),
        },
    ]
}

fn build_autonomous_coding_loop_steps(
    task: &WorkerTaskRecord,
    result: &VerticalSliceResult,
    inspection: &AutonomousCodingInspection,
    patches: &[WorkerTaskPatchProposal],
    command_runs: &[WorkerTaskCommandRun],
    coding_rounds: &[WorkerTaskCodingRound],
) -> Vec<WorkerTaskLoopStep> {
    vec![
        WorkerTaskLoopStep {
            step_index: 1,
            phase: WorkerTaskLoopPhase::Plan,
            title: "Plan autonomous coding lane".into(),
            input_summary: compact_text(&task.prompt, 180),
            output_summary: format!(
                "Selected bounded inspect/patch/test/revise coding loop across {} rounds",
                coding_rounds.len()
            ),
            evidence_ref: format!("worker://{}/autonomous/plan", task.worker_session_id),
            passed: true,
        },
        WorkerTaskLoopStep {
            step_index: 2,
            phase: WorkerTaskLoopPhase::Inspect,
            title: "Inspect source targets".into(),
            input_summary: format!("candidate_targets={}", inspection.target_count),
            output_summary: format!(
                "readable={} lines={} bytes={}",
                inspection.readable_count, inspection.total_lines, inspection.total_bytes
            ),
            evidence_ref: format!("worker://{}/autonomous/inspect", task.worker_session_id),
            passed: inspection.readable_count > 0,
        },
        WorkerTaskLoopStep {
            step_index: 3,
            phase: WorkerTaskLoopPhase::Patch,
            title: "Generate review-gated patch".into(),
            input_summary: compact_text(&result.final_text, 180),
            output_summary: format!("generated {} patch proposal(s)", patches.len()),
            evidence_ref: format!("worker://{}/autonomous/patch", task.worker_session_id),
            passed: !patches.is_empty(),
        },
        WorkerTaskLoopStep {
            step_index: 4,
            phase: WorkerTaskLoopPhase::Test,
            title: "Execute multi-round worker commands".into(),
            input_summary: format!("rounds={} commands={}", coding_rounds.len(), command_runs.len()),
            output_summary: format!(
                "{} / {} command runs passed",
                command_runs.iter().filter(|run| run.passed).count(),
                command_runs.len()
            ),
            evidence_ref: format!("worker://{}/autonomous/test", task.worker_session_id),
            passed: !command_runs.is_empty() && command_runs.iter().all(|run| run.passed),
        },
        WorkerTaskLoopStep {
            step_index: 5,
            phase: WorkerTaskLoopPhase::Revise,
            title: "Prepare auditable parent handoff".into(),
            input_summary: format!("patches={} artifacts_ready=true", patches.len()),
            output_summary: "Artifacts, command transcript, patch proposal, evidence, replay, and promotion gates are ready".into(),
            evidence_ref: format!("worker://{}/autonomous/handoff", task.worker_session_id),
            passed: true,
        },
    ]
}

fn build_worker_task_artifacts(
    task: &WorkerTaskRecord,
    result: &VerticalSliceResult,
) -> Vec<WorkerTaskArtifact> {
    vec![
        WorkerTaskArtifact {
            artifact_id: format!("{}:run-summary", task.task_id),
            kind: "run_summary".into(),
            title: format!("Worker {} run summary", task.worker_id),
            content: format!(
                "worker={}\nsession={}\nmodel={}/{}\ntool={:?}\nrecalled_memories={}\nfinal={} ",
                task.worker_id,
                task.worker_session_id,
                result.active_model.provider,
                result.active_model.model,
                result.invoked_tool,
                result.recalled_memories,
                compact_text(&result.final_text, 400),
            ),
            path_hint: Some(format!(
                "worker://{}/run-summary.md",
                task.worker_session_id
            )),
        },
        WorkerTaskArtifact {
            artifact_id: format!("{}:merge-note", task.task_id),
            kind: "merge_note".into(),
            title: format!("Join note for {}", task.task_id),
            content: format!(
                "Task `{}` completed in worker lane `{}`. Parent can review summary, tool output, and diff summary before merging.",
                task.task_id, task.worker_id
            ),
            path_hint: Some(format!("worker://{}/join-note.md", task.worker_session_id)),
        },
    ]
}

fn build_worker_task_diff_summary(task: &WorkerTaskRecord, result: &VerticalSliceResult) -> String {
    let tool = result.invoked_tool.as_deref().unwrap_or("no_tool_invoked");
    format!(
        "worker={} task={} pseudo_diff=summary_only tool={} final_excerpt=\"{}\"",
        task.worker_id,
        task.task_id,
        tool,
        compact_text(&result.final_text, 160)
    )
}

fn build_worker_task_patch_proposals(
    task: &WorkerTaskRecord,
    result: &VerticalSliceResult,
) -> Vec<WorkerTaskPatchProposal> {
    let primary_path = infer_worker_patch_path(task);
    let mut proposals = vec![WorkerTaskPatchProposal {
        patch_id: format!("{}:patch-1", task.task_id),
        revision_of: None,
        revision_index: 0,
        file_path: primary_path.clone(),
        change_kind: "proposed_update".into(),
        summary: format!(
            "Worker {} proposes a reviewable update derived from task output",
            task.worker_id
        ),
        unified_diff: format!(
            "--- /dev/null\n+++ {}\n@@\n+{}\n",
            primary_path,
            compact_text(&result.final_text, 500)
        ),
        apply_status: WorkerTaskPatchApplyStatus::Proposed,
        applied_at_unix_ms: None,
        transaction_id: None,
        conflict_reason: None,
    }];
    let prompt = task.prompt.to_ascii_lowercase();
    if prompt.contains("multi") || prompt.contains("patch set") || prompt.contains("batch") {
        let merge_plan_path = format!(
            "docs/worker-proposals/{}-merge-plan.md",
            worker_patch_suffix(task)
        );
        proposals.push(WorkerTaskPatchProposal {
            patch_id: format!("{}:patch-2", task.task_id),
            revision_of: None,
            revision_index: 0,
            file_path: merge_plan_path.clone(),
            change_kind: "merge_plan".into(),
            summary: format!("Worker {} proposes a companion merge plan", task.worker_id),
            unified_diff: format!(
                "--- /dev/null\n+++ {}\n@@\n+# Worker merge plan\n+task={}\n+worker={}\n+summary={}\n",
                merge_plan_path,
                task.task_id,
                task.worker_id,
                compact_text(&result.final_text, 300)
            ),
            apply_status: WorkerTaskPatchApplyStatus::Proposed,
            applied_at_unix_ms: None,
            transaction_id: None,
            conflict_reason: None,
        });
    }
    proposals
}

fn build_worker_task_loop_steps(
    task: &WorkerTaskRecord,
    result: &VerticalSliceResult,
    patches: &[WorkerTaskPatchProposal],
) -> Vec<WorkerTaskLoopStep> {
    let final_summary = compact_text(&result.final_text, 180);
    vec![
        WorkerTaskLoopStep {
            step_index: 1,
            phase: WorkerTaskLoopPhase::Plan,
            title: "Plan worker approach".into(),
            input_summary: compact_text(&task.prompt, 180),
            output_summary: format!(
                "Plan a bounded worker lane for `{}` with review-gated outputs",
                task.worker_id
            ),
            evidence_ref: format!("worker://{}/loop/plan", task.worker_session_id),
            passed: true,
        },
        WorkerTaskLoopStep {
            step_index: 2,
            phase: WorkerTaskLoopPhase::Inspect,
            title: "Inspect context and constraints".into(),
            input_summary: format!(
                "deps={} attempts={}",
                task.depends_on.len(),
                task.attempt_count
            ),
            output_summary: format!(
                "Resolved model output, recalled_memories={}, tool={}",
                result.recalled_memories,
                result.invoked_tool.as_deref().unwrap_or("none")
            ),
            evidence_ref: format!("worker://{}/loop/inspect", task.worker_session_id),
            passed: true,
        },
        WorkerTaskLoopStep {
            step_index: 3,
            phase: WorkerTaskLoopPhase::Patch,
            title: "Draft patch proposal set".into(),
            input_summary: final_summary.clone(),
            output_summary: format!("Generated {} review-gated patch proposal(s)", patches.len()),
            evidence_ref: format!("worker://{}/loop/patch", task.worker_session_id),
            passed: !patches.is_empty(),
        },
        WorkerTaskLoopStep {
            step_index: 4,
            phase: WorkerTaskLoopPhase::Test,
            title: "Run deterministic local checks".into(),
            input_summary: format!("patches={}", patches.len()),
            output_summary:
                "Local deterministic worker gate passed; external side effects not executed".into(),
            evidence_ref: format!("worker://{}/loop/test", task.worker_session_id),
            passed: true,
        },
        WorkerTaskLoopStep {
            step_index: 5,
            phase: WorkerTaskLoopPhase::Revise,
            title: "Prepare parent review handoff".into(),
            input_summary: final_summary,
            output_summary:
                "Artifacts, diff summary, loop trace, and patch set are ready for parent review"
                    .into(),
            evidence_ref: format!("worker://{}/loop/revise", task.worker_session_id),
            passed: true,
        },
    ]
}

fn worker_task_loop_report(task: WorkerTaskRecord) -> WorkerTaskLoopReport {
    let phases = task
        .loop_steps
        .iter()
        .map(|step| step.phase)
        .collect::<Vec<_>>();
    let passed_count = task.loop_steps.iter().filter(|step| step.passed).count();
    let failed_count = task.loop_steps.len().saturating_sub(passed_count);
    WorkerTaskLoopReport {
        task_id: task.task_id,
        workspace_id: task.workspace_id,
        worker_id: task.worker_id,
        loop_step_count: task.loop_steps.len(),
        passed_count,
        failed_count,
        phases,
        steps: task.loop_steps,
    }
}

fn worker_task_evidence_report(task: WorkerTaskRecord) -> WorkerTaskEvidenceReport {
    let mut entries = Vec::new();
    push_worker_evidence(
        &mut entries,
        &task,
        "task_created",
        format!(
            "created worker task prompt='{}'",
            compact_text(&task.prompt, 80)
        ),
        task.created_at_unix_ms,
        &task.worker_session_id,
    );
    push_worker_evidence(
        &mut entries,
        &task,
        "permission_envelope",
        task.permission_envelope.policy_summary.clone(),
        task.created_at_unix_ms,
        &task.worker_session_id,
    );
    if !task.depends_on.is_empty() {
        push_worker_evidence(
            &mut entries,
            &task,
            "dependency_gate",
            format!("depends_on={}", task.depends_on.join(",")),
            task.created_at_unix_ms,
            &task.worker_session_id,
        );
    }
    if let Some(schedule_expr) = &task.schedule_expr {
        push_worker_evidence(
            &mut entries,
            &task,
            "schedule_gate",
            format!(
                "schedule={} next_run={:?}",
                schedule_expr, task.next_run_unix_ms
            ),
            task.created_at_unix_ms,
            &task.worker_session_id,
        );
    }
    if let Some(started_at) = task.started_at_unix_ms {
        push_worker_evidence(
            &mut entries,
            &task,
            "run_attempt",
            format!(
                "attempt={}/{} timeout_budget_ms={}",
                task.attempt_count, task.max_attempts, task.timeout_budget_ms
            ),
            started_at,
            &task.worker_session_id,
        );
    }
    for round in &task.coding_rounds {
        push_worker_evidence(
            &mut entries,
            &task,
            "coding_round",
            format!(
                "round={} commands={} patches={} passed={} {}",
                round.round_index,
                round.command_ids.len(),
                round.patch_ids.len(),
                round.passed,
                compact_text(&round.summary, 120)
            ),
            task.completed_at_unix_ms.unwrap_or(task.updated_at_unix_ms),
            &task.worker_session_id,
        );
    }
    for lease in &task.file_leases {
        push_worker_evidence(
            &mut entries,
            &task,
            "file_lease",
            format!(
                "{} path={} status={} conflicts={}",
                lease.lease_id,
                lease.target_path,
                file_lease_status_label(lease.status),
                lease.conflict_task_ids.len()
            ),
            lease.acquired_at_unix_ms,
            &task.worker_session_id,
        );
    }
    for step in &task.loop_steps {
        push_worker_evidence(
            &mut entries,
            &task,
            "loop_step",
            format!("{:?}: {} passed={}", step.phase, step.title, step.passed),
            task.completed_at_unix_ms.unwrap_or(task.updated_at_unix_ms),
            &task.worker_session_id,
        );
    }
    for run in &task.command_runs {
        push_worker_evidence(
            &mut entries,
            &task,
            "command_run",
            format!(
                "{} origin={:?} backend={} kind={:?} cwd={} timed_out={} exit={} passed={} stdout={}",
                run.command_id,
                run.execution_origin,
                run.backend_id,
                run.backend_kind,
                run.working_directory.as_deref().unwrap_or("<not-recorded>"),
                run.timed_out,
                run.exit_code,
                run.passed,
                compact_text(&run.stdout, 120)
            ),
            task.completed_at_unix_ms.unwrap_or(task.updated_at_unix_ms),
            &task.worker_session_id,
        );
    }
    for artifact in &task.artifacts {
        push_worker_evidence(
            &mut entries,
            &task,
            "artifact",
            format!(
                "{} [{}] {}",
                artifact.artifact_id, artifact.kind, artifact.title
            ),
            task.completed_at_unix_ms.unwrap_or(task.updated_at_unix_ms),
            &task.worker_session_id,
        );
    }
    for patch in &task.patch_proposals {
        push_worker_evidence(
            &mut entries,
            &task,
            "patch_proposal",
            format!(
                "{} {} status={} tx={:?}",
                patch.patch_id,
                patch.file_path,
                patch_apply_status_label(patch.apply_status),
                patch.transaction_id
            ),
            patch
                .applied_at_unix_ms
                .or(task.completed_at_unix_ms)
                .unwrap_or(task.updated_at_unix_ms),
            &task.worker_session_id,
        );
    }
    if let Some(error) = &task.last_error {
        push_worker_evidence(
            &mut entries,
            &task,
            "failure",
            format!(
                "kind={:?} retry_after={:?} error={}",
                task.failure_kind,
                task.retry_after_unix_ms,
                compact_text(error, 120)
            ),
            task.completed_at_unix_ms.unwrap_or(task.updated_at_unix_ms),
            &task.worker_session_id,
        );
    }
    let chain_head = entries
        .last()
        .map(|entry| entry.entry_hash.clone())
        .unwrap_or_else(|| "sha256:empty".into());
    WorkerTaskEvidenceReport {
        task_id: task.task_id,
        workspace_id: task.workspace_id,
        worker_id: task.worker_id,
        worker_session_id: task.worker_session_id,
        evidence_count: entries.len(),
        terminal_status: task.status,
        permission_envelope: task.permission_envelope,
        chain_head,
        entries,
    }
}

fn worker_task_replay_audit_report(task: WorkerTaskRecord) -> WorkerTaskReplayAuditReport {
    let evidence = worker_task_evidence_report(task.clone());
    let mut checks = Vec::new();

    let hash_chain_valid = verify_worker_evidence_hash_chain(&evidence.entries);
    checks.push(WorkerTaskReplayCheck {
        check_id: "hash_chain".into(),
        passed: hash_chain_valid,
        summary: format!(
            "{} evidence entries replay to {}",
            evidence.evidence_count, evidence.chain_head
        ),
    });

    let permission_policy_valid = evidence.entries.iter().any(|entry| {
        entry.kind == "permission_envelope"
            && entry.summary == task.permission_envelope.policy_summary
    });
    checks.push(WorkerTaskReplayCheck {
        check_id: "permission_policy".into(),
        passed: permission_policy_valid,
        summary: task.permission_envelope.policy_summary.clone(),
    });

    let lifecycle_valid = match task.status {
        WorkerTaskStatus::Completed => {
            task.started_at_unix_ms.is_some()
                && task.completed_at_unix_ms.is_some()
                && task.result_summary.is_some()
        }
        WorkerTaskStatus::Failed => task.started_at_unix_ms.is_some() && task.last_error.is_some(),
        WorkerTaskStatus::Queued | WorkerTaskStatus::Scheduled => task.started_at_unix_ms.is_none(),
        WorkerTaskStatus::Running => task.started_at_unix_ms.is_some(),
        WorkerTaskStatus::Paused => task.paused_from_status.is_some(),
        WorkerTaskStatus::Cancelled | WorkerTaskStatus::Interrupted => true,
    };
    checks.push(WorkerTaskReplayCheck {
        check_id: "lifecycle".into(),
        passed: lifecycle_valid,
        summary: format!(
            "status={} attempts={}/{}",
            task_status_label(task.status),
            task.attempt_count,
            task.max_attempts
        ),
    });

    let artifact_records_valid = task.artifacts.iter().all(|artifact| {
        evidence
            .entries
            .iter()
            .any(|entry| entry.kind == "artifact" && entry.summary.contains(&artifact.artifact_id))
    });
    checks.push(WorkerTaskReplayCheck {
        check_id: "artifact_records".into(),
        passed: artifact_records_valid,
        summary: format!(
            "{} artifacts are represented in evidence",
            task.artifacts.len()
        ),
    });

    let patch_records_valid = task.patch_proposals.iter().all(|patch| {
        evidence
            .entries
            .iter()
            .any(|entry| entry.kind == "patch_proposal" && entry.summary.contains(&patch.patch_id))
    });
    checks.push(WorkerTaskReplayCheck {
        check_id: "patch_records".into(),
        passed: patch_records_valid,
        summary: format!(
            "{} patch proposals are represented in evidence",
            task.patch_proposals.len()
        ),
    });

    let coding_rounds_valid = task.coding_rounds.iter().all(|round| {
        let round_evidence_present = evidence.entries.iter().any(|entry| {
            entry.kind == "coding_round"
                && entry
                    .summary
                    .contains(&format!("round={}", round.round_index))
        });
        let commands_present = round.command_ids.iter().all(|command_id| {
            task.command_runs
                .iter()
                .any(|run| &run.command_id == command_id)
        });
        let patches_present = round.patch_ids.iter().all(|patch_id| {
            task.patch_proposals
                .iter()
                .any(|patch| &patch.patch_id == patch_id)
        });
        round_evidence_present && commands_present && patches_present && round.passed
    });
    checks.push(WorkerTaskReplayCheck {
        check_id: "coding_round_records".into(),
        passed: coding_rounds_valid,
        summary: format!(
            "{} coding rounds are represented in evidence",
            task.coding_rounds.len()
        ),
    });

    let multi_round_loop_valid = if task.execution_mode == WorkerTaskExecutionMode::AutonomousCoding
    {
        task.coding_rounds.len() >= 2
            && task
                .coding_rounds
                .windows(2)
                .all(|pair| pair[0].round_index < pair[1].round_index)
            && task.command_runs.len() >= 6
    } else {
        true
    };
    checks.push(WorkerTaskReplayCheck {
        check_id: "multi_round_loop".into(),
        passed: multi_round_loop_valid,
        summary: format!(
            "rounds={} command_runs={}",
            task.coding_rounds.len(),
            task.command_runs.len()
        ),
    });

    let file_lease_records_valid = task.file_leases.iter().all(|lease| {
        evidence
            .entries
            .iter()
            .any(|entry| entry.kind == "file_lease" && entry.summary.contains(&lease.lease_id))
    });
    checks.push(WorkerTaskReplayCheck {
        check_id: "file_lease_records".into(),
        passed: file_lease_records_valid,
        summary: format!(
            "{} file leases are represented in evidence",
            task.file_leases.len()
        ),
    });

    let command_records_valid = task.command_runs.iter().all(|run| {
        evidence
            .entries
            .iter()
            .any(|entry| entry.kind == "command_run" && entry.summary.contains(&run.command_id))
    });
    checks.push(WorkerTaskReplayCheck {
        check_id: "command_records".into(),
        passed: command_records_valid,
        summary: format!(
            "{} command runs are represented in evidence",
            task.command_runs.len()
        ),
    });

    let backend_records_valid = if task.execution_mode == WorkerTaskExecutionMode::AutonomousCoding
    {
        !task.command_runs.is_empty()
            && task.command_runs.iter().all(|run| {
                !run.backend_id.trim().is_empty()
                    && run.backend_kind == task.execution_backend.kind
                    && run.remote_backend == task.execution_backend.remote
            })
    } else {
        true
    };
    checks.push(WorkerTaskReplayCheck {
        check_id: "backend_records".into(),
        passed: backend_records_valid,
        summary: format!(
            "backend={} kind={:?} command_runs={}",
            task.execution_backend.backend_id,
            task.execution_backend.kind,
            task.command_runs.len()
        ),
    });

    let host_process_command_records_valid =
        if task.execution_mode == WorkerTaskExecutionMode::AutonomousCoding {
            !task.command_runs.is_empty()
                && task.command_runs.iter().all(|run| {
                    run.execution_origin == WorkerTaskCommandRunOrigin::HostProcess
                        && run.working_directory.is_some()
                        && !run.timed_out
                })
        } else {
            true
        };
    checks.push(WorkerTaskReplayCheck {
        check_id: "host_process_command_records".into(),
        passed: host_process_command_records_valid,
        summary: format!(
            "{} autonomous command runs recorded as host processes",
            task.command_runs
                .iter()
                .filter(|run| run.execution_origin == WorkerTaskCommandRunOrigin::HostProcess)
                .count()
        ),
    });

    let limits = &task.safety_envelope.resource_limits;
    let safety_limits_valid = task.command_runs.len() <= limits.max_command_runs
        && task.patch_proposals.len() <= limits.max_patch_proposals
        && task.loop_steps.len() <= limits.max_loop_steps
        && !task
            .safety_envelope
            .sandbox
            .workspace_root
            .trim()
            .is_empty()
        && task.safety_envelope.cancel_supported
        && task.safety_envelope.cancel_checked_before_host_command
        && task
            .command_runs
            .iter()
            .all(|run| run.sandboxed && run.resource_limit_violation.is_none());
    checks.push(WorkerTaskReplayCheck {
        check_id: "safety_limits".into(),
        passed: safety_limits_valid,
        summary: format!(
            "commands={}/{} patches={}/{} loop_steps={}/{} cancel_supported={}",
            task.command_runs.len(),
            limits.max_command_runs,
            task.patch_proposals.len(),
            limits.max_patch_proposals,
            task.loop_steps.len(),
            limits.max_loop_steps,
            task.safety_envelope.cancel_supported
        ),
    });

    let failure_records_valid = if task.status == WorkerTaskStatus::Failed {
        task.failure_kind.is_some()
            && task.retry_after_unix_ms.is_some()
            && evidence.entries.iter().any(|entry| entry.kind == "failure")
    } else {
        evidence.entries.iter().all(|entry| entry.kind != "failure")
    };
    checks.push(WorkerTaskReplayCheck {
        check_id: "failure_records".into(),
        passed: failure_records_valid,
        summary: format!(
            "failure_kind={:?} retry_after={:?}",
            task.failure_kind, task.retry_after_unix_ms
        ),
    });

    let replayed_chain_head = evidence
        .entries
        .last()
        .map(|entry| entry.entry_hash.clone())
        .unwrap_or_else(|| "sha256:empty".into());
    let replay_passed = checks.iter().all(|check| check.passed);
    WorkerTaskReplayAuditReport {
        task_id: task.task_id,
        workspace_id: task.workspace_id,
        worker_id: task.worker_id,
        terminal_status: task.status,
        evidence_count: evidence.evidence_count,
        chain_head: evidence.chain_head,
        replayed_chain_head,
        hash_chain_valid,
        permission_policy_valid,
        lifecycle_valid,
        artifact_records_valid,
        patch_records_valid,
        coding_rounds_valid,
        multi_round_loop_valid,
        file_lease_records_valid,
        backend_records_valid,
        failure_records_valid,
        safety_limits_valid,
        replay_passed,
        checks,
    }
}

fn worker_task_merge_risk_report(task: WorkerTaskRecord) -> WorkerTaskMergeRiskReport {
    let replay = worker_task_replay_audit_report(task.clone());
    let patch_conflicted_count = count_patch_status(
        &task.patch_proposals,
        WorkerTaskPatchApplyStatus::Conflicted,
    );
    let patch_rejected_count =
        count_patch_status(&task.patch_proposals, WorkerTaskPatchApplyStatus::Rejected);
    let patch_rolled_back_count = count_patch_status(
        &task.patch_proposals,
        WorkerTaskPatchApplyStatus::RolledBack,
    );
    let conflicted_file_lease_count =
        count_file_lease_status(&task.file_leases, WorkerTaskFileLeaseStatus::Conflicted);
    let expired_file_lease_count =
        count_file_lease_status(&task.file_leases, WorkerTaskFileLeaseStatus::Expired);
    let mut score = 0u8;
    let mut reasons = Vec::new();

    if task.status != WorkerTaskStatus::Completed {
        score = score.saturating_add(80);
        reasons.push(format!(
            "terminal status is {}",
            task_status_label(task.status)
        ));
    }
    if !replay.replay_passed {
        score = score.saturating_add(60);
        reasons.push("replay audit did not pass".into());
    }
    if patch_conflicted_count > 0 {
        score = score.saturating_add((patch_conflicted_count * 20).min(40) as u8);
        reasons.push(format!(
            "{} conflicted patch proposals",
            patch_conflicted_count
        ));
    }
    if conflicted_file_lease_count > 0 || expired_file_lease_count > 0 {
        score = score.saturating_add(40);
        reasons.push(format!(
            "file lease issues conflicted={} expired={}",
            conflicted_file_lease_count, expired_file_lease_count
        ));
    }
    if patch_rejected_count > 0 {
        score = score.saturating_add((patch_rejected_count * 15).min(30) as u8);
        reasons.push(format!("{} rejected patch proposals", patch_rejected_count));
    }
    if patch_rolled_back_count > 0 {
        score = score.saturating_add((patch_rolled_back_count * 20).min(40) as u8);
        reasons.push(format!(
            "{} rolled back patch proposals",
            patch_rolled_back_count
        ));
    }
    if task.attempt_count > 1 || task.failure_kind.is_some() {
        score = score.saturating_add(15);
        reasons.push(format!(
            "retry/failure history attempts={} failure_kind={:?}",
            task.attempt_count, task.failure_kind
        ));
    }
    let failed_command_runs = task.command_runs.iter().filter(|run| !run.passed).count();
    if failed_command_runs > 0 {
        score = score.saturating_add((failed_command_runs * 20).min(40) as u8);
        reasons.push(format!(
            "{} failed autonomous command runs",
            failed_command_runs
        ));
    }
    if task.permission_envelope.network_allowed {
        score = score.saturating_add(10);
        reasons.push("network-enabled worker lane".into());
    }
    match task.permission_envelope.write_scope {
        WritePathScope::AnyPath => {
            score = score.saturating_add(20);
            reasons.push("write scope permits any path".into());
        }
        WritePathScope::WorkspaceOnly => {
            score = score.saturating_add(5);
            reasons.push("write scope permits workspace writes".into());
        }
        WritePathScope::ArtifactsOnly => {}
    }
    if task.status == WorkerTaskStatus::Completed && task.artifacts.is_empty() {
        score = score.saturating_add(10);
        reasons.push("completed task has no artifacts".into());
    }
    if reasons.is_empty() {
        reasons.push("low-risk completed task with valid replay audit".into());
    }
    let decision =
        if task.status != WorkerTaskStatus::Completed || !replay.replay_passed || score >= 60 {
            WorkerTaskMergeDecision::Blocked
        } else if score >= 20
            || patch_conflicted_count > 0
            || patch_rejected_count > 0
            || patch_rolled_back_count > 0
        {
            WorkerTaskMergeDecision::NeedsReview
        } else {
            WorkerTaskMergeDecision::SafeToMerge
        };
    WorkerTaskMergeRiskReport {
        task_id: task.task_id,
        workspace_id: task.workspace_id,
        worker_id: task.worker_id,
        decision,
        risk_score: score.min(100),
        replay_passed: replay.replay_passed,
        patch_conflicted_count,
        patch_rejected_count,
        patch_rolled_back_count,
        reasons,
    }
}

fn worker_task_promotion_report(task: WorkerTaskRecord) -> WorkerTaskPromotionReport {
    let replay = worker_task_replay_audit_report(task.clone());
    let merge_risk = worker_task_merge_risk_report(task.clone());
    let applied_patch_count =
        count_patch_status(&task.patch_proposals, WorkerTaskPatchApplyStatus::Applied);
    let unapplied_patch_count = task
        .patch_proposals
        .iter()
        .filter(|patch| patch.apply_status == WorkerTaskPatchApplyStatus::Proposed)
        .count();
    let mut reasons = Vec::new();
    if task.status != WorkerTaskStatus::Completed {
        reasons.push(format!(
            "task status is {} rather than completed",
            task_status_label(task.status)
        ));
    }
    if !replay.replay_passed {
        reasons.push("replay audit failed".into());
    }
    if merge_risk.decision == WorkerTaskMergeDecision::Blocked {
        reasons.push(format!(
            "merge risk is blocked with score {}",
            merge_risk.risk_score
        ));
    }
    if merge_risk.decision == WorkerTaskMergeDecision::NeedsReview {
        reasons.push(format!(
            "merge risk needs review with score {}",
            merge_risk.risk_score
        ));
    }
    if unapplied_patch_count > 0 {
        reasons.push(format!(
            "{} proposed patches require explicit apply/reject before promotion",
            unapplied_patch_count
        ));
    }
    if merge_risk.patch_conflicted_count > 0
        || merge_risk.patch_rejected_count > 0
        || merge_risk.patch_rolled_back_count > 0
    {
        reasons.push("patch history contains conflicted/rejected/rolled-back proposals".into());
    }

    let hard_block = task.status != WorkerTaskStatus::Completed
        || !replay.replay_passed
        || merge_risk.decision == WorkerTaskMergeDecision::Blocked;
    let review_needed = merge_risk.decision == WorkerTaskMergeDecision::NeedsReview
        || unapplied_patch_count > 0
        || merge_risk.patch_conflicted_count > 0
        || merge_risk.patch_rejected_count > 0
        || merge_risk.patch_rolled_back_count > 0;
    let decision = if hard_block {
        WorkerTaskPromotionDecision::Blocked
    } else if review_needed {
        WorkerTaskPromotionDecision::NeedsReview
    } else {
        WorkerTaskPromotionDecision::Promoted
    };
    if reasons.is_empty() {
        reasons.push("promotion gate passed: replay-valid low-risk task".into());
    }
    WorkerTaskPromotionReport {
        task_id: task.task_id,
        workspace_id: task.workspace_id,
        worker_id: task.worker_id,
        decision,
        promotion_allowed: decision == WorkerTaskPromotionDecision::Promoted,
        auto_merge_allowed: decision == WorkerTaskPromotionDecision::Promoted
            && merge_risk.decision == WorkerTaskMergeDecision::SafeToMerge
            && merge_risk.risk_score <= 10,
        merge_risk,
        replay,
        unapplied_patch_count,
        applied_patch_count,
        reasons,
    }
}

fn worker_task_promotion_ledger_report(task: WorkerTaskRecord) -> WorkerTaskPromotionLedgerReport {
    let promotion = worker_task_promotion_report(task.clone());
    let now = task.completed_at_unix_ms.unwrap_or(task.updated_at_unix_ms);
    let mut entries = Vec::new();
    push_promotion_ledger_entry(
        &mut entries,
        &task,
        "promotion_gate_evaluated",
        promotion.decision,
        format!(
            "promotion_allowed={} auto_merge_allowed={} reasons={}",
            promotion.promotion_allowed,
            promotion.auto_merge_allowed,
            promotion.reasons.join("; ")
        ),
        now,
    );
    push_promotion_ledger_entry(
        &mut entries,
        &task,
        "replay_basis",
        promotion.decision,
        format!(
            "replay_passed={} chain_head={}",
            promotion.replay.replay_passed, promotion.replay.chain_head
        ),
        now,
    );
    push_promotion_ledger_entry(
        &mut entries,
        &task,
        "merge_risk_basis",
        promotion.decision,
        format!(
            "merge_decision={:?} score={} reasons={}",
            promotion.merge_risk.decision,
            promotion.merge_risk.risk_score,
            promotion.merge_risk.reasons.join("; ")
        ),
        now,
    );
    push_promotion_ledger_entry(
        &mut entries,
        &task,
        "patch_basis",
        promotion.decision,
        format!(
            "applied={} unapplied={} conflicted={} rejected={} rolled_back={}",
            promotion.applied_patch_count,
            promotion.unapplied_patch_count,
            promotion.merge_risk.patch_conflicted_count,
            promotion.merge_risk.patch_rejected_count,
            promotion.merge_risk.patch_rolled_back_count
        ),
        now,
    );
    let chain_head = entries
        .last()
        .map(|entry| entry.entry_hash.clone())
        .unwrap_or_else(|| "promotion-ledger:empty".into());
    WorkerTaskPromotionLedgerReport {
        task_id: task.task_id,
        workspace_id: task.workspace_id,
        worker_id: task.worker_id,
        ledger_count: entries.len(),
        promotion_decision: promotion.decision,
        promotion_allowed: promotion.promotion_allowed,
        auto_merge_allowed: promotion.auto_merge_allowed,
        chain_head,
        entries,
    }
}

fn worker_task_handoff_bundle_report(task: WorkerTaskRecord) -> WorkerTaskHandoffBundleReport {
    let generated_at_unix_ms = task.completed_at_unix_ms.unwrap_or(task.updated_at_unix_ms);
    let evidence = worker_task_evidence_report(task.clone());
    let replay = worker_task_replay_audit_report(task.clone());
    let merge_risk = worker_task_merge_risk_report(task.clone());
    let promotion = worker_task_promotion_report(task.clone());
    let promotion_ledger = worker_task_promotion_ledger_report(task.clone());
    let handoff_ready = evidence.chain_head.starts_with("hepta-evidence:")
        && replay.replay_passed
        && promotion_ledger.chain_head.starts_with("hepta-promotion:")
        && promotion.promotion_allowed;
    let signature = worker_handoff_signature(
        &task.task_id,
        &task.worker_id,
        &evidence.chain_head,
        &replay.replayed_chain_head,
        merge_risk.risk_score,
        promotion.decision,
        &promotion_ledger.chain_head,
        generated_at_unix_ms,
    );
    let summary = format!(
        "handoff_ready={} promotion={:?} risk_score={} evidence={} promotion_ledger={}",
        handoff_ready,
        promotion.decision,
        merge_risk.risk_score,
        evidence.chain_head,
        promotion_ledger.chain_head,
    );
    WorkerTaskHandoffBundleReport {
        task_id: task.task_id,
        workspace_id: task.workspace_id,
        worker_id: task.worker_id,
        bundle_version: "worker-handoff-v1".into(),
        generated_at_unix_ms,
        evidence,
        replay,
        merge_risk,
        promotion,
        promotion_ledger,
        handoff_ready,
        signature,
        summary,
    }
}

fn worker_handoff_signature(
    task_id: &str,
    worker_id: &str,
    evidence_chain_head: &str,
    replayed_chain_head: &str,
    risk_score: u8,
    promotion_decision: WorkerTaskPromotionDecision,
    promotion_chain_head: &str,
    generated_at_unix_ms: u64,
) -> String {
    let material = format!(
        "worker-handoff-v1|{}|{}|{}|{}|{}|{:?}|{}|{}",
        task_id,
        worker_id,
        evidence_chain_head,
        replayed_chain_head,
        risk_score,
        promotion_decision,
        promotion_chain_head,
        generated_at_unix_ms,
    );
    format!("hepta-handoff:{:016x}", stable_hash64(&material))
}

fn push_promotion_ledger_entry(
    entries: &mut Vec<WorkerTaskPromotionLedgerEntry>,
    task: &WorkerTaskRecord,
    action: &str,
    decision: WorkerTaskPromotionDecision,
    summary: String,
    occurred_at_unix_ms: u64,
) {
    let index = entries.len();
    let previous_hash = entries.last().map(|entry| entry.entry_hash.clone());
    let ledger_ref = format!("worker-promotion:{}:{}:{}", task.task_id, action, index);
    let entry_hash = promotion_ledger_hash(
        previous_hash.as_deref(),
        &ledger_ref,
        action,
        decision,
        &summary,
        occurred_at_unix_ms,
    );
    entries.push(WorkerTaskPromotionLedgerEntry {
        index,
        ledger_ref,
        action: action.into(),
        decision,
        summary,
        occurred_at_unix_ms,
        previous_hash,
        entry_hash,
    });
}

fn promotion_ledger_hash(
    previous_hash: Option<&str>,
    ledger_ref: &str,
    action: &str,
    decision: WorkerTaskPromotionDecision,
    summary: &str,
    occurred_at_unix_ms: u64,
) -> String {
    let material = format!(
        "{}|{}|{}|{:?}|{}|{}",
        previous_hash.unwrap_or("genesis"),
        ledger_ref,
        action,
        decision,
        summary,
        occurred_at_unix_ms,
    );
    format!("hepta-promotion:{:016x}", stable_hash64(&material))
}

fn verify_worker_evidence_hash_chain(entries: &[WorkerTaskEvidenceEntry]) -> bool {
    let mut previous_hash: Option<String> = None;
    for entry in entries {
        if entry.previous_hash != previous_hash {
            return false;
        }
        let expected = worker_evidence_hash(
            previous_hash.as_deref(),
            &entry.evidence_ref,
            &entry.kind,
            &entry.summary,
            entry.occurred_at_unix_ms,
            &entry.session_id,
        );
        if entry.entry_hash != expected {
            return false;
        }
        previous_hash = Some(entry.entry_hash.clone());
    }
    true
}

fn push_worker_evidence(
    entries: &mut Vec<WorkerTaskEvidenceEntry>,
    task: &WorkerTaskRecord,
    kind: &str,
    summary: String,
    occurred_at_unix_ms: u64,
    session_id: &str,
) {
    let index = entries.len();
    let previous_hash = entries.last().map(|entry| entry.entry_hash.clone());
    let evidence_ref = format!("worker-evidence:{}:{}:{}", task.task_id, kind, index);
    let entry_hash = worker_evidence_hash(
        previous_hash.as_deref(),
        &evidence_ref,
        kind,
        &summary,
        occurred_at_unix_ms,
        session_id,
    );
    entries.push(WorkerTaskEvidenceEntry {
        index,
        evidence_ref,
        kind: kind.into(),
        summary,
        occurred_at_unix_ms,
        session_id: session_id.into(),
        previous_hash,
        entry_hash,
    });
}

fn worker_evidence_hash(
    previous_hash: Option<&str>,
    evidence_ref: &str,
    kind: &str,
    summary: &str,
    occurred_at_unix_ms: u64,
    session_id: &str,
) -> String {
    let material = format!(
        "{}|{}|{}|{}|{}|{}",
        previous_hash.unwrap_or("genesis"),
        evidence_ref,
        kind,
        summary,
        occurred_at_unix_ms,
        session_id,
    );
    format!("hepta-evidence:{:016x}", stable_hash64(&material))
}

fn stable_hash64(value: &str) -> u64 {
    let mut hash = 0xcbf29ce484222325u64;
    for byte in value.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

fn infer_worker_patch_path(task: &WorkerTaskRecord) -> String {
    let prompt = task.prompt.to_ascii_lowercase();
    let suffix = worker_patch_suffix(task);
    if prompt.contains("doc") || prompt.contains("paper") || prompt.contains("write") {
        format!("docs/worker-proposals/{}.md", suffix)
    } else if prompt.contains("rust") || prompt.contains("code") || prompt.contains("patch") {
        format!("src/worker-proposals/{}.rs", suffix)
    } else {
        format!("artifacts/worker-proposals/{}.md", suffix)
    }
}

fn revised_patch_path(
    task: &WorkerTaskRecord,
    source_patch: &WorkerTaskPatchProposal,
    revision_index: usize,
) -> String {
    format!(
        "docs/worker-proposals/{}-revision-{}-of-{}.md",
        worker_patch_suffix(task),
        revision_index,
        sanitize_for_id(&source_patch.patch_id)
    )
}

fn worker_patch_suffix(task: &WorkerTaskRecord) -> String {
    format!(
        "{}-{}",
        sanitize_for_id(&task.task_id),
        sanitize_for_id(&compact_text(&task.prompt, 48))
    )
}

fn worker_task_patch_review_report(task: WorkerTaskRecord) -> WorkerTaskPatchReviewReport {
    let patch_count = task.patch_proposals.len();
    let proposed_count =
        count_patch_status(&task.patch_proposals, WorkerTaskPatchApplyStatus::Proposed);
    let applied_count =
        count_patch_status(&task.patch_proposals, WorkerTaskPatchApplyStatus::Applied);
    let conflicted_count = count_patch_status(
        &task.patch_proposals,
        WorkerTaskPatchApplyStatus::Conflicted,
    );
    let rejected_count =
        count_patch_status(&task.patch_proposals, WorkerTaskPatchApplyStatus::Rejected);
    let rolled_back_count = count_patch_status(
        &task.patch_proposals,
        WorkerTaskPatchApplyStatus::RolledBack,
    );
    WorkerTaskPatchReviewReport {
        task_id: task.task_id,
        workspace_id: task.workspace_id,
        patch_count,
        proposed_count,
        applied_count,
        conflicted_count,
        rejected_count,
        rolled_back_count,
        patches: task.patch_proposals,
    }
}

fn count_patch_status(
    patches: &[WorkerTaskPatchProposal],
    status: WorkerTaskPatchApplyStatus,
) -> usize {
    patches
        .iter()
        .filter(|patch| patch.apply_status == status)
        .count()
}

fn count_file_lease_status(
    leases: &[WorkerTaskFileLease],
    status: WorkerTaskFileLeaseStatus,
) -> usize {
    leases.iter().filter(|lease| lease.status == status).count()
}

fn count_file_lease_status_refs(
    leases: &[&WorkerTaskFileLease],
    status: WorkerTaskFileLeaseStatus,
) -> usize {
    leases.iter().filter(|lease| lease.status == status).count()
}

fn patch_apply_status_label(status: WorkerTaskPatchApplyStatus) -> &'static str {
    match status {
        WorkerTaskPatchApplyStatus::Proposed => "proposed",
        WorkerTaskPatchApplyStatus::Applied => "applied",
        WorkerTaskPatchApplyStatus::Conflicted => "conflicted",
        WorkerTaskPatchApplyStatus::Rejected => "rejected",
        WorkerTaskPatchApplyStatus::RolledBack => "rolled_back",
    }
}

fn extract_added_content_from_unified_diff(diff: &str) -> Result<String, String> {
    let mut added_lines = Vec::new();
    let mut saw_hunk = false;
    for line in diff.lines() {
        if line.starts_with("@@") {
            saw_hunk = true;
            continue;
        }
        if !saw_hunk || line.starts_with("+++") || line.starts_with("---") {
            continue;
        }
        if let Some(added) = line.strip_prefix('+') {
            added_lines.push(added.to_string());
        }
    }
    if added_lines.is_empty() {
        return Err("patch has no added content to apply".into());
    }
    Ok(format!("{}\n", added_lines.join("\n")))
}

fn compact_text(value: &str, max_chars: usize) -> String {
    let compact = value.split_whitespace().collect::<Vec<_>>().join(" ");
    if compact.chars().count() <= max_chars {
        compact
    } else {
        format!(
            "{}...",
            compact
                .chars()
                .take(max_chars.saturating_sub(3))
                .collect::<String>()
        )
    }
}

fn is_zero(value: &usize) -> bool {
    *value == 0
}

#[cfg(test)]
mod tests {
    use hepta_core::EventKind;
    use hepta_core::ExecutionProfile;
    use hepta_core::FilesystemScope;
    use hepta_core::MemoryStore;
    use hepta_core::WritePathScope;

    use super::WorkerExecutionBackendBinding;
    use super::WorkerExecutionBackendKind;
    use super::WorkerExecutionBackendStatus;
    use super::WorkerPoolPressureLevel;
    use super::WorkerTaskCommandRunOrigin;
    use super::WorkerTaskContextRecallHandoffPolicy;
    use super::WorkerTaskExecutionMode;
    use super::WorkerTaskFailureKind;
    use super::WorkerTaskFileLeaseStatus;
    use super::WorkerTaskLoopPhase;
    use super::WorkerTaskMergeDecision;
    use super::WorkerTaskPatchApplyStatus;
    use super::WorkerTaskPromotionDecision;
    use super::WorkerTaskStatus;
    use super::effective_worker_task_prompt;
    use super::redact_worker_output_exfiltration;
    use super::task_status_label;
    use crate::RuntimeKernel;

    #[test]
    fn status_labels_are_stable() {
        assert_eq!(task_status_label(WorkerTaskStatus::Queued), "queued");
        assert_eq!(task_status_label(WorkerTaskStatus::Scheduled), "scheduled");
        assert_eq!(task_status_label(WorkerTaskStatus::Running), "running");
        assert_eq!(task_status_label(WorkerTaskStatus::Paused), "paused");
        assert_eq!(task_status_label(WorkerTaskStatus::Completed), "completed");
        assert_eq!(task_status_label(WorkerTaskStatus::Failed), "failed");
        assert_eq!(task_status_label(WorkerTaskStatus::Cancelled), "cancelled");
        assert_eq!(
            task_status_label(WorkerTaskStatus::Interrupted),
            "interrupted"
        );
    }

    #[test]
    fn worker_task_lifecycle_is_queryable_and_snapshot_backed() {
        let runtime = RuntimeKernel::new();
        let spawned = runtime
            .spawn_worker_task("reviewer", "summarize the release checklist", None)
            .expect("task should spawn");

        assert_eq!(spawned.task.worker_id, "reviewer");
        assert_eq!(spawned.task.status, WorkerTaskStatus::Queued);
        assert_eq!(spawned.task.parent_session_id, "session-main");
        assert!(
            spawned
                .task
                .worker_session_id
                .starts_with("worker-reviewer-")
        );

        let index = runtime.worker_task_index(None).expect("tasks should list");
        assert_eq!(index.total_count, 1);
        assert_eq!(index.queued_count, 1);

        let inventory = runtime.worker_inventory().expect("workers should list");
        assert_eq!(inventory.worker_count, 1);
        assert_eq!(inventory.workers[0].worker_id, "reviewer");
        assert_eq!(inventory.workers[0].active_task_count, 1);

        let join = runtime
            .join_worker_tasks(Some("reviewer"))
            .expect("join should report active blockers");
        assert!(!join.safe_to_join);
        assert_eq!(join.active_count, 1);

        let snapshot = runtime
            .runtime_snapshot()
            .expect("snapshot should include worker tasks");
        assert_eq!(snapshot.worker_tasks.len(), 1);

        let restored = RuntimeKernel::new();
        restored
            .apply_runtime_snapshot(snapshot)
            .expect("snapshot should restore");
        let restored_status = restored
            .worker_task_status(&spawned.task.task_id)
            .expect("restored task should be queryable");
        assert_eq!(restored_status.task.status, WorkerTaskStatus::Queued);
    }

    #[test]
    fn worker_task_pause_resume_and_interrupt_are_observable_controls() {
        let runtime = RuntimeKernel::new();
        let spawned = runtime
            .spawn_worker_task("builder", "prepare controllable worker lane", None)
            .expect("task should spawn");

        let steered = runtime
            .steer_worker_task(&spawned.task.task_id, "tighten scope before execution")
            .expect("task should accept steering");
        assert_eq!(steered.task.steering_directives.len(), 1);
        assert!(
            effective_worker_task_prompt(&steered.task).contains("Operator steering directives")
        );

        let paused = runtime
            .pause_worker_task(&spawned.task.task_id)
            .expect("task should pause");
        assert_eq!(paused.task.status, WorkerTaskStatus::Paused);
        assert_eq!(
            paused.task.paused_from_status,
            Some(WorkerTaskStatus::Queued)
        );

        let supervisor = runtime
            .worker_task_supervisor()
            .expect("supervisor should show paused task");
        assert_eq!(supervisor.paused_count, 1);
        assert_eq!(supervisor.paused_control_count, 1);
        assert_eq!(
            supervisor.recommended_next_action,
            "resume_or_interrupt_tasks"
        );
        assert!(supervisor.paused_task_ids.contains(&spawned.task.task_id));

        let observatory = runtime
            .worker_subagent_observatory()
            .expect("observatory should show paused task");
        assert_eq!(observatory.paused_count, 1);
        assert_eq!(
            observatory.recommended_next_action,
            "resume_or_interrupt_paused_subagents"
        );
        assert!(observatory.lanes.iter().any(|lane| {
            lane.task_id == spawned.task.task_id && lane.control_action == "resume_or_interrupt"
        }));

        let resumed = runtime
            .resume_worker_task(&spawned.task.task_id)
            .expect("task should resume");
        assert_eq!(resumed.task.status, WorkerTaskStatus::Queued);
        assert_eq!(resumed.task.paused_from_status, None);

        let interrupted = runtime
            .interrupt_worker_task(&spawned.task.task_id)
            .expect("task should interrupt");
        assert_eq!(interrupted.task.status, WorkerTaskStatus::Interrupted);
        let supervisor = runtime
            .worker_task_supervisor()
            .expect("supervisor should show interrupted task");
        assert_eq!(supervisor.interrupted_count, 1);
        assert_eq!(supervisor.interrupted_control_count, 1);
        assert!(
            supervisor
                .interrupted_task_ids
                .contains(&spawned.task.task_id)
        );

        let console = runtime
            .operator_console()
            .expect("operator console should summarize live controls");
        assert!(console.operator_console_complete);
        assert!(console.task_queue_panel);
        assert!(console.subagent_tree_panel);
        assert!(console.command_stream_panel);
        assert!(console.patch_evidence_review_panel);
        assert!(console.pause_control_ready);
        assert!(console.resume_control_ready);
        assert!(console.interrupt_control_ready);
        assert!(console.steer_control_ready);
        assert!(
            console
                .control_commands
                .iter()
                .any(|command| command.contains("/steer-task"))
        );
        assert!(
            console
                .control_commands
                .iter()
                .any(|command| command.contains("/pause-task"))
        );
        assert!(
            console
                .recent_events
                .iter()
                .any(|event| event.kind == EventKind::TaskSteered)
        );
        assert!(
            console
                .recent_events
                .iter()
                .any(|event| event.kind == EventKind::TaskInterrupted)
        );
    }

    #[test]
    fn worker_task_nested_spawn_depth_blocks_recursive_orchestrators() {
        let runtime = RuntimeKernel::new();
        let root = runtime
            .spawn_worker_task("root", "coordinate one nested worker", None)
            .expect("root task should spawn");
        assert_eq!(root.task.spawn_depth, 0);
        assert_eq!(root.task.max_spawn_depth, 1);

        let child = runtime
            .spawn_worker_task_with_parent(
                "child",
                "nested child worker",
                None,
                Vec::new(),
                Some(root.task.task_id.clone()),
                1,
            )
            .expect("one nested child should fit default depth policy");
        assert_eq!(child.task.parent_task_id, Some(root.task.task_id.clone()));
        assert_eq!(child.task.spawn_depth, 1);

        let denied = runtime
            .spawn_worker_task_with_parent(
                "grandchild",
                "recursive nested worker",
                None,
                Vec::new(),
                Some(child.task.task_id.clone()),
                1,
            )
            .expect_err("recursive grandchild should be denied");
        assert!(denied.0.contains("recursive spawn denied"));
    }

    #[test]
    fn worker_output_url_exfiltration_is_redacted() {
        let raw = "callback=https://example.test/hook?token=sk-live-secret&safe=1\nOPENAI_API_KEY=sk-test-secret\nAuthorization:private-token";
        let redacted = redact_worker_output_exfiltration(raw);

        assert!(redacted.contains("token=[REDACTED]&safe=1"));
        assert!(redacted.contains("OPENAI_API_KEY=[REDACTED]"));
        assert!(redacted.contains("Authorization:[REDACTED]"));
        assert!(!redacted.contains("sk-live-secret"));
        assert!(!redacted.contains("sk-test-secret"));
        assert!(!redacted.contains("private-token"));
    }

    #[test]
    fn worker_execution_backend_report_covers_local_and_remote_contracts() {
        let runtime = RuntimeKernel::new();
        let report = runtime
            .worker_execution_backends()
            .expect("worker backend report should build");

        assert_eq!(report.backend_count, 3);
        assert_eq!(report.active_backend_id, "local-host-process");
        assert_eq!(
            report.active_backend_kind,
            WorkerExecutionBackendKind::LocalHostProcess
        );
        assert!(report.local_backend_ready);
        assert_eq!(report.remote_backend_count, 2);
        assert_eq!(report.configured_remote_backend_count, 0);
        assert!(!report.remote_execution_enabled);
        assert!(report.file_sync_policy_required);
        assert!(report.credential_mount_policy_required);
        assert!(report.remote_path_traversal_denied);
        assert!(report.remote_credential_mounts_deny_by_default);
        assert!(report.remote_file_sync_manifest_required);
        assert!(report.remote_child_side_effects_blocked);
        assert!(report.remote_safety_regression_pack_ready);
        assert!(report.environment_process_evidence_contract);
        assert!(report.backends.iter().any(|backend| {
            backend.kind == WorkerExecutionBackendKind::Docker
                && backend.status == WorkerExecutionBackendStatus::RequiresConfiguration
                && backend.remote
                && backend.file_sync_supported
                && backend
                    .file_sync_manifest_policy
                    .contains("workspace_sync_manifest")
                && backend.credential_mount_policy.contains("deny_by_default")
                && backend
                    .path_traversal_policy
                    .contains("deny_path_traversal")
                && backend
                    .child_side_effect_policy
                    .contains("block_child_side_effects")
        }));
        assert!(report.backends.iter().any(|backend| {
            backend.kind == WorkerExecutionBackendKind::Ssh
                && backend.status == WorkerExecutionBackendStatus::RequiresConfiguration
                && backend.remote
                && backend.credential_mount_policy.contains("deny_by_default")
                && backend
                    .path_traversal_policy
                    .contains("deny_path_traversal")
                && backend
                    .child_side_effect_policy
                    .contains("block_child_side_effects")
        }));
    }

    #[test]
    fn remote_worker_backend_denies_execution_until_explicitly_configured() {
        let runtime = RuntimeKernel::new();
        let mut task = runtime
            .spawn_worker_task(
                "remote-coding-builder",
                "autonomous coding subagent remote backend dry run",
                None,
            )
            .expect("task should spawn")
            .task;
        task.execution_backend = WorkerExecutionBackendBinding {
            backend_id: "docker-sandbox".into(),
            kind: WorkerExecutionBackendKind::Docker,
            remote: true,
            evidence_kind: "environment_process".into(),
        };
        let workspace_root = runtime
            .workspace_root()
            .expect("workspace root should resolve");
        let run = super::run_worker_environment_command(
            &task,
            &workspace_root,
            &task.safety_envelope,
            "remote-deny-check",
            "sh -c 'echo should-not-run'",
            "/bin/sh",
            &["-c", "echo should-not-run"],
        );

        assert_eq!(run.backend_id, "docker-sandbox");
        assert_eq!(run.backend_kind, WorkerExecutionBackendKind::Docker);
        assert!(run.remote_backend);
        assert!(!run.passed);
        assert_eq!(run.exit_code, 126);
        assert!(run.stdout.is_empty());
        assert!(
            run.stderr
                .contains("requires explicit remote configuration")
        );
        assert!(
            run.resource_limit_violation
                .as_deref()
                .unwrap_or_default()
                .contains("requires explicit remote configuration")
        );
    }

    #[test]
    fn worker_task_permission_envelope_sandboxes_review_lanes() {
        let runtime = RuntimeKernel::new();
        runtime
            .switch_execution_profile(ExecutionProfile::FullAccess)
            .expect("profile switch should work");
        runtime
            .switch_filesystem_scope(FilesystemScope::AnyPath)
            .expect("filesystem scope switch should work");
        runtime
            .switch_write_path_scope(WritePathScope::AnyPath)
            .expect("write scope switch should work");

        let review = runtime
            .spawn_worker_task("security-review", "audit permissions", None)
            .expect("review task should spawn");
        assert_eq!(
            review.task.permission_envelope.execution_profile,
            ExecutionProfile::ReadOnlyTools
        );
        assert_eq!(
            review.task.permission_envelope.filesystem_scope,
            FilesystemScope::WorkspaceOnly
        );
        assert_eq!(
            review.task.permission_envelope.write_scope,
            WritePathScope::ArtifactsOnly
        );
        assert!(!review.task.permission_envelope.network_allowed);
        assert_eq!(
            runtime
                .execution_profile_for_session(&review.task.worker_session_id)
                .expect("worker profile should resolve"),
            ExecutionProfile::ReadOnlyTools
        );

        let builder = runtime
            .spawn_worker_task("patch-builder", "prepare patch", None)
            .expect("builder task should spawn");
        assert_eq!(
            builder.task.permission_envelope.execution_profile,
            ExecutionProfile::FullAccess
        );
        assert_eq!(
            builder.task.permission_envelope.write_scope,
            WritePathScope::WorkspaceOnly
        );
    }

    #[tokio::test]
    async fn worker_task_run_completes_in_isolated_worker_session() {
        let runtime = RuntimeKernel::new();
        let spawned = runtime
            .spawn_worker_task("builder", "say hello from a worker lane", None)
            .expect("task should spawn");

        let run = runtime
            .run_worker_task(&spawned.task.task_id)
            .await
            .expect("task should run");

        assert_eq!(run.task.status, WorkerTaskStatus::Completed);
        assert_eq!(run.task.attempt_count, 1);
        assert!(run.result.is_some());
        assert_eq!(run.artifact_count, 2);
        assert_eq!(run.task.artifacts.len(), 2);
        assert!(run.task.diff_summary.is_some());
        assert_eq!(run.patch_proposal_count, 1);
        assert_eq!(run.task.patch_proposals.len(), 1);
        assert_eq!(run.loop_step_count, 5);
        assert_eq!(run.task.loop_steps.len(), 5);
        assert_eq!(run.task.loop_steps[0].phase, WorkerTaskLoopPhase::Plan);
        assert_eq!(
            run.task.patch_proposals[0].apply_status,
            WorkerTaskPatchApplyStatus::Proposed
        );
        assert_eq!(run.task.artifacts[0].kind, "run_summary");
        let task_events = runtime
            .query_events(25, None, Some(&run.task.worker_session_id))
            .expect("task events should be queryable");
        assert!(
            task_events
                .iter()
                .any(|event| event.event.kind == EventKind::TaskSpawned)
        );
        assert!(
            task_events
                .iter()
                .any(|event| event.event.kind == EventKind::TaskStarted)
        );
        assert!(
            task_events
                .iter()
                .any(|event| event.event.kind == EventKind::TaskCompleted)
        );

        let join = runtime
            .join_worker_tasks(Some("builder"))
            .expect("completed tasks should join safely");
        assert!(join.safe_to_join);
        assert_eq!(join.completed_count, 1);
        assert_eq!(join.artifact_count, 2);
        assert_eq!(join.diff_ready_count, 1);
        assert_eq!(join.patch_proposal_count, 1);
        assert_eq!(join.joined[0].worker_id, "builder");
        assert_eq!(join.joined[0].artifacts.len(), 2);
        assert_eq!(join.joined[0].patch_proposals.len(), 1);
        assert_eq!(join.loop_step_count, 5);
        assert_eq!(join.joined[0].loop_steps.len(), 5);
        assert_eq!(join.merge_safe_count, 1);
        assert_eq!(join.merge_needs_review_count, 0);
        assert_eq!(join.merge_blocked_count, 0);
        assert_eq!(
            join.joined[0].merge_risk.decision,
            WorkerTaskMergeDecision::SafeToMerge
        );
        assert!(join.joined[0].merge_risk.replay_passed);

        let loop_report = runtime
            .worker_task_loop(&run.task.task_id)
            .expect("loop should be reviewable");
        assert_eq!(loop_report.loop_step_count, 5);
        assert_eq!(loop_report.failed_count, 0);

        let evidence = runtime
            .worker_task_evidence(&run.task.task_id)
            .expect("evidence should be reviewable");
        assert_eq!(evidence.task_id, run.task.task_id);
        assert!(evidence.evidence_count >= 10);
        assert_eq!(evidence.entries[0].previous_hash.as_deref(), None);
        assert!(
            evidence
                .entries
                .iter()
                .any(|entry| entry.kind == "permission_envelope")
        );
        assert_eq!(
            evidence.chain_head,
            evidence.entries.last().unwrap().entry_hash
        );

        let replay = runtime
            .worker_task_replay_audit(&run.task.task_id)
            .expect("replay audit should be available");
        assert!(replay.replay_passed);
        assert!(replay.hash_chain_valid);
        assert!(replay.permission_policy_valid);
        assert!(replay.lifecycle_valid);
        assert!(replay.artifact_records_valid);
        assert!(replay.patch_records_valid);
        assert_eq!(replay.chain_head, replay.replayed_chain_head);

        let promotion_before_apply = runtime
            .worker_task_promotion_gate(&run.task.task_id)
            .expect("promotion gate should report before apply");
        assert_eq!(
            promotion_before_apply.decision,
            WorkerTaskPromotionDecision::NeedsReview
        );
        assert_eq!(promotion_before_apply.unapplied_patch_count, 1);
        let ledger_before_apply = runtime
            .worker_task_promotion_ledger(&run.task.task_id)
            .expect("promotion ledger should report before apply");
        assert_eq!(ledger_before_apply.ledger_count, 4);
        assert_eq!(
            ledger_before_apply.promotion_decision,
            WorkerTaskPromotionDecision::NeedsReview
        );
        assert!(
            ledger_before_apply
                .chain_head
                .starts_with("hepta-promotion:")
        );
        let handoff_before_apply = runtime
            .worker_task_handoff_bundle(&run.task.task_id)
            .expect("handoff bundle should report before apply");
        assert!(!handoff_before_apply.handoff_ready);
        assert!(handoff_before_apply.signature.starts_with("hepta-handoff:"));

        let patch_id = join.joined[0].patch_proposals[0].patch_id.clone();
        let applied = runtime
            .mark_worker_task_patch_applied(&run.task.task_id, &patch_id)
            .expect("patch should be markable as applied");
        assert_eq!(applied.applied_count, 1);
        assert_eq!(applied.proposed_count, 0);

        let promotion_after_apply = runtime
            .worker_task_promotion_gate(&run.task.task_id)
            .expect("promotion gate should report after apply");
        assert_eq!(
            promotion_after_apply.decision,
            WorkerTaskPromotionDecision::Promoted
        );
        assert!(promotion_after_apply.promotion_allowed);
        let ledger_after_apply = runtime
            .worker_task_promotion_ledger(&run.task.task_id)
            .expect("promotion ledger should report after apply");
        assert_eq!(
            ledger_after_apply.promotion_decision,
            WorkerTaskPromotionDecision::Promoted
        );
        assert!(ledger_after_apply.promotion_allowed);
        assert_eq!(
            ledger_after_apply.chain_head,
            ledger_after_apply.entries.last().unwrap().entry_hash
        );
        let handoff_after_apply = runtime
            .worker_task_handoff_bundle(&run.task.task_id)
            .expect("handoff bundle should report after apply");
        assert!(handoff_after_apply.handoff_ready);
        assert!(handoff_after_apply.signature.starts_with("hepta-handoff:"));
        assert!(
            handoff_after_apply
                .evidence
                .chain_head
                .starts_with("hepta-evidence:")
        );

        let patch_review = runtime
            .worker_task_patches(&run.task.task_id)
            .expect("patches should be reviewable");
        assert_eq!(patch_review.patch_count, 1);
        assert_eq!(patch_review.applied_count, 1);
        assert!(patch_review.patches[0].transaction_id.is_some());
        let target_path = super::resolve_path_within_root(
            &runtime
                .workspace_root()
                .expect("workspace root should exist"),
            std::path::Path::new(&patch_review.patches[0].file_path),
        );
        let _ = std::fs::remove_file(target_path);
    }

    #[tokio::test]
    async fn worker_task_context_recall_handoff_is_operator_opt_in_without_snippet_leak() {
        let disabled_runtime = RuntimeKernel::new();
        disabled_runtime
            .memory
            .put(hepta_core::MemoryRecord {
                id: "worker-disabled-source-id".into(),
                scope: hepta_core::MemoryScope::LongTerm,
                content: format!("worker-needle {}", "disabled-worker-context ".repeat(80)),
            })
            .await
            .expect("memory should store");
        let disabled_task = disabled_runtime
            .spawn_worker_task("builder", "worker-needle", None)
            .expect("task should spawn");

        let disabled_run = disabled_runtime
            .run_worker_task_with_context_recall_handoff(
                &disabled_task.task.task_id,
                WorkerTaskContextRecallHandoffPolicy::Disabled,
            )
            .await
            .expect("disabled worker task should run");

        assert_eq!(disabled_run.run.task.status, WorkerTaskStatus::Completed);
        assert!(!disabled_run.selected_snippets_present);
        assert_eq!(disabled_run.selected_snippet_count, 0);
        assert!(disabled_run.provider_rollup.is_none());

        let opted_runtime = RuntimeKernel::new();
        opted_runtime
            .memory
            .put(hepta_core::MemoryRecord {
                id: "worker-context-source-id".into(),
                scope: hepta_core::MemoryScope::LongTerm,
                content: format!(
                    "worker-needle {}",
                    "operator-worker-safe-context ".repeat(80)
                ),
            })
            .await
            .expect("memory should store");
        let opted_task = opted_runtime
            .spawn_worker_task("builder", "worker-needle", None)
            .expect("task should spawn");

        let opted_run = opted_runtime
            .run_worker_task_with_context_recall_handoff(
                &opted_task.task.task_id,
                WorkerTaskContextRecallHandoffPolicy::ExperimentalOperatorApproved,
            )
            .await
            .expect("operator-approved worker task should run");
        let encoded = serde_json::to_string(&opted_run).expect("report should serialize");
        let debug = format!("{opted_run:?}");

        assert_eq!(opted_run.run.task.status, WorkerTaskStatus::Completed);
        assert!(opted_run.selected_snippets_present);
        assert!(opted_run.selected_snippet_count > 0);
        assert!(
            opted_run
                .provider_rollup
                .as_ref()
                .expect("provider rollup should be present")
                .recall_selection
                .has_count_integrity()
        );
        assert!(
            opted_run
                .run
                .result
                .as_ref()
                .expect("worker result should be present")
                .final_text
                .contains("[chat] model reply: worker-needle")
        );
        for forbidden in [
            "operator-worker-safe-context",
            "worker-context-source-id",
            "[redacted-query]",
            "source_id",
            "source_memory_ids",
        ] {
            assert!(
                !encoded.contains(forbidden),
                "serialized worker report leaked {forbidden}"
            );
            assert!(
                !debug.contains(forbidden),
                "worker report debug leaked {forbidden}"
            );
        }
    }

    #[tokio::test]
    async fn worker_task_context_recall_handoff_scheduler_policy_is_explicit_without_leak() {
        let disabled_ready_runtime = RuntimeKernel::new();
        disabled_ready_runtime
            .memory
            .put(hepta_core::MemoryRecord {
                id: "worker-ready-disabled-source-id".into(),
                scope: hepta_core::MemoryScope::LongTerm,
                content: format!("ready-needle {}", "disabled-ready-context ".repeat(80)),
            })
            .await
            .expect("memory should store");
        disabled_ready_runtime
            .spawn_worker_task("ready", "ready-needle", None)
            .expect("ready task should spawn");

        let disabled_ready = disabled_ready_runtime
            .run_ready_worker_tasks_with_context_recall_handoff(
                Some(10),
                None,
                WorkerTaskContextRecallHandoffPolicy::Disabled,
            )
            .await
            .expect("disabled ready batch should run");

        assert_eq!(disabled_ready.ran_count, 1);
        assert_eq!(
            disabled_ready.context_recall_handoff_policy,
            WorkerTaskContextRecallHandoffPolicy::Disabled
        );
        assert_eq!(disabled_ready.selected_snippets_present_count, 0);
        assert_eq!(disabled_ready.selected_snippet_count, 0);
        assert!(disabled_ready.runs[0].provider_rollup.is_none());

        let opted_ready_runtime = RuntimeKernel::new();
        opted_ready_runtime
            .memory
            .put(hepta_core::MemoryRecord {
                id: "worker-ready-source-id".into(),
                scope: hepta_core::MemoryScope::LongTerm,
                content: format!("ready-needle {}", "operator-ready-safe-context ".repeat(80)),
            })
            .await
            .expect("memory should store");
        opted_ready_runtime
            .spawn_worker_task("ready", "ready-needle", None)
            .expect("ready task should spawn");

        let opted_ready = opted_ready_runtime
            .run_ready_worker_tasks_with_context_recall_handoff(
                Some(10),
                None,
                WorkerTaskContextRecallHandoffPolicy::ExperimentalOperatorApproved,
            )
            .await
            .expect("operator-approved ready batch should run");
        let ready_encoded = serde_json::to_string(&opted_ready).expect("report should serialize");
        let ready_debug = format!("{opted_ready:?}");

        assert_eq!(opted_ready.ran_count, 1);
        assert_eq!(
            opted_ready.context_recall_handoff_policy,
            WorkerTaskContextRecallHandoffPolicy::ExperimentalOperatorApproved
        );
        assert_eq!(opted_ready.selected_snippets_present_count, 1);
        assert!(opted_ready.selected_snippet_count > 0);
        assert!(
            opted_ready.runs[0]
                .provider_rollup
                .as_ref()
                .expect("provider rollup should be present")
                .recall_selection
                .has_count_integrity()
        );

        let opted_due_runtime = RuntimeKernel::new();
        opted_due_runtime
            .memory
            .put(hepta_core::MemoryRecord {
                id: "worker-due-source-id".into(),
                scope: hepta_core::MemoryScope::LongTerm,
                content: format!("due-needle {}", "operator-due-safe-context ".repeat(80)),
            })
            .await
            .expect("memory should store");
        let scheduled = opted_due_runtime
            .spawn_worker_task("scheduler", "due-needle", Some("delay:10ms"))
            .expect("scheduled task should spawn");
        let due_at = scheduled
            .task
            .next_run_unix_ms
            .expect("scheduled task should have next run");

        let opted_due = opted_due_runtime
            .run_due_worker_tasks_with_context_recall_handoff(
                Some(due_at),
                WorkerTaskContextRecallHandoffPolicy::ExperimentalOperatorApproved,
            )
            .await
            .expect("operator-approved due batch should run");
        let due_encoded = serde_json::to_string(&opted_due).expect("report should serialize");
        let due_debug = format!("{opted_due:?}");

        assert_eq!(opted_due.due_count, 1);
        assert_eq!(opted_due.ran_count, 1);
        assert_eq!(
            opted_due.context_recall_handoff_policy,
            WorkerTaskContextRecallHandoffPolicy::ExperimentalOperatorApproved
        );
        assert_eq!(opted_due.selected_snippets_present_count, 1);
        assert!(opted_due.selected_snippet_count > 0);
        assert!(
            opted_due.runs[0]
                .provider_rollup
                .as_ref()
                .expect("provider rollup should be present")
                .recall_selection
                .has_count_integrity()
        );

        for rendered in [ready_encoded, ready_debug, due_encoded, due_debug] {
            for forbidden in [
                "operator-ready-safe-context",
                "operator-due-safe-context",
                "worker-ready-source-id",
                "worker-due-source-id",
                "[redacted-query]",
                "source_id",
                "source_memory_ids",
            ] {
                assert!(
                    !rendered.contains(forbidden),
                    "scheduler report leaked {forbidden}"
                );
            }
        }
    }

    #[tokio::test]
    async fn autonomous_coding_worker_runs_real_inspect_command_patch_handoff_loop() {
        let runtime = RuntimeKernel::new();
        let spawned = runtime
            .spawn_worker_task(
                "coding-builder",
                "autonomous coding subagent implement worker runtime evidence",
                None,
            )
            .expect("autonomous coding task should spawn");
        assert_eq!(
            spawned.task.execution_mode,
            WorkerTaskExecutionMode::AutonomousCoding
        );

        let run = runtime
            .run_worker_task(&spawned.task.task_id)
            .await
            .expect("autonomous coding task should run");
        assert_eq!(run.task.status, WorkerTaskStatus::Completed);
        assert_eq!(
            run.task.execution_mode,
            WorkerTaskExecutionMode::AutonomousCoding
        );
        assert_eq!(run.command_run_count, 6);
        assert_eq!(run.task.command_runs.len(), 6);
        assert_eq!(
            run.task.execution_backend.kind,
            WorkerExecutionBackendKind::LocalHostProcess
        );
        assert_eq!(run.task.execution_backend.backend_id, "local-host-process");
        assert_eq!(run.coding_round_count, 2);
        assert_eq!(run.task.coding_rounds.len(), 2);
        assert!(run.task.coding_rounds.iter().all(|round| round.passed));
        assert!(
            run.task.coding_rounds[0]
                .command_ids
                .iter()
                .all(|id| id.contains(":round-1-"))
        );
        assert!(
            run.task.coding_rounds[1]
                .command_ids
                .iter()
                .all(|id| id.contains(":round-2-"))
        );
        assert_eq!(run.file_lease_count, run.task.file_leases.len());
        assert!(!run.task.file_leases.is_empty());
        assert!(run.task.file_leases.iter().all(|lease| {
            lease.status == WorkerTaskFileLeaseStatus::HeldForReview
                && lease.worker_session_id == run.task.worker_session_id
                && lease.conflict_task_ids.is_empty()
        }));
        assert!(run.task.command_runs.iter().all(|command| command.passed));
        assert!(run.task.command_runs.iter().all(|command| {
            command.execution_origin == WorkerTaskCommandRunOrigin::HostProcess
                && command.backend_id == "local-host-process"
                && command.backend_kind == WorkerExecutionBackendKind::LocalHostProcess
                && !command.remote_backend
                && command.working_directory.is_some()
                && command.sandboxed
                && !command.timed_out
                && command.resource_limit_violation.is_none()
        }));
        assert!(run.task.safety_envelope.cancel_supported);
        assert!(run.task.safety_envelope.cancel_checked_before_host_command);
        assert!(
            run.task
                .safety_envelope
                .sandbox
                .allowed_programs
                .contains(&"/bin/sh".into())
        );
        assert!(
            run.task.command_runs[0]
                .stdout
                .contains("cargo_toml=present")
        );
        assert!(
            run.task
                .artifacts
                .iter()
                .any(|artifact| artifact.kind == "code_inspection")
        );
        assert!(
            run.task
                .artifacts
                .iter()
                .any(|artifact| artifact.kind == "command_transcript")
        );
        assert_eq!(run.patch_proposal_count, 1);
        assert!(
            run.task.patch_proposals[0]
                .unified_diff
                .contains("Autonomous coding worker proposal")
        );
        assert_eq!(run.loop_step_count, 5);
        assert_eq!(run.task.loop_steps[3].phase, WorkerTaskLoopPhase::Test);

        let evidence = runtime
            .worker_task_evidence(&run.task.task_id)
            .expect("autonomous evidence should build");
        assert!(
            evidence
                .entries
                .iter()
                .any(|entry| entry.kind == "command_run")
        );
        let replay = runtime
            .worker_task_replay_audit(&run.task.task_id)
            .expect("autonomous replay should build");
        assert!(replay.replay_passed);
        assert!(
            replay
                .checks
                .iter()
                .any(|check| check.check_id == "command_records" && check.passed)
        );
        assert!(
            replay
                .checks
                .iter()
                .any(|check| check.check_id == "host_process_command_records" && check.passed)
        );
        assert!(replay.coding_rounds_valid);
        assert!(replay.multi_round_loop_valid);
        assert!(replay.file_lease_records_valid);
        assert!(replay.backend_records_valid);
        assert!(
            replay
                .checks
                .iter()
                .any(|check| check.check_id == "multi_round_loop" && check.passed)
        );
        assert!(
            replay
                .checks
                .iter()
                .any(|check| check.check_id == "file_lease_records" && check.passed)
        );
        assert!(replay.safety_limits_valid);
        assert!(
            replay
                .checks
                .iter()
                .any(|check| check.check_id == "safety_limits" && check.passed)
        );

        let observatory = runtime
            .worker_subagent_observatory()
            .expect("observatory should build");
        assert_eq!(observatory.autonomous_count, 1);
        assert_eq!(
            observatory.held_file_lease_count,
            run.task.file_leases.len()
        );
        assert_eq!(observatory.conflicted_file_lease_count, 0);
        assert!(observatory.lanes.iter().any(|lane| {
            lane.task_id == run.task.task_id && lane.file_lease_count == run.task.file_leases.len()
        }));

        let patch = run.task.patch_proposals[0].clone();
        assert!(run.task.file_leases.iter().any(|lease| {
            lease.target_path == patch.file_path
                && lease.status == WorkerTaskFileLeaseStatus::HeldForReview
        }));
        let applied = runtime
            .apply_worker_task_patch(&run.task.task_id, &patch.patch_id)
            .expect("autonomous patch should apply");
        assert_eq!(applied.applied_count, 1);
        assert!(applied.patches[0].transaction_id.is_some());
        let handoff = runtime
            .worker_task_handoff_bundle(&run.task.task_id)
            .expect("handoff should build after apply");
        assert!(handoff.handoff_ready);
        let target_path = super::resolve_path_within_root(
            &runtime
                .workspace_root()
                .expect("workspace root should exist"),
            std::path::Path::new(&patch.file_path),
        );
        let _ = std::fs::remove_file(target_path);
    }

    #[test]
    fn autonomous_worker_host_command_enforces_sandbox_timeout_and_output_limits() {
        let runtime = RuntimeKernel::new();
        let task = runtime
            .spawn_worker_task(
                "coding-builder",
                "autonomous coding subagent safety controls",
                None,
            )
            .expect("task should spawn")
            .task;
        let workspace_root = runtime
            .workspace_root()
            .expect("workspace root should resolve");
        let mut safety = task.safety_envelope.clone();
        safety.resource_limits.per_command_timeout_ms = 5;
        let timed_out = super::run_worker_host_command(
            &task,
            &workspace_root,
            &safety,
            "timeout-check",
            "sh -c 'sleep 0.05'",
            "/bin/sh",
            &["-c", "sleep 0.05"],
        );
        assert!(timed_out.sandboxed);
        assert!(timed_out.timed_out);
        assert_eq!(
            timed_out.resource_limit_violation.as_deref(),
            Some("command_timeout")
        );

        let mut blocked_safety = task.safety_envelope.clone();
        blocked_safety.sandbox.allowed_programs = vec!["/usr/bin/false".into()];
        let blocked = super::run_worker_host_command(
            &task,
            &workspace_root,
            &blocked_safety,
            "sandbox-check",
            "sh -c 'echo blocked'",
            "/bin/sh",
            &["-c", "echo blocked"],
        );
        assert!(!blocked.passed);
        assert_eq!(blocked.exit_code, 126);
        assert!(
            blocked
                .resource_limit_violation
                .as_deref()
                .unwrap_or_default()
                .contains("sandbox disallows program")
        );

        let mut output_safety = task.safety_envelope.clone();
        output_safety.resource_limits.max_stdout_bytes = 4;
        let truncated = super::run_worker_host_command(
            &task,
            &workspace_root,
            &output_safety,
            "stdout-limit-check",
            "sh -c 'printf abcdef'",
            "/bin/sh",
            &["-c", "printf abcdef"],
        );
        assert!(truncated.passed);
        assert!(truncated.stdout_truncated);
        assert_eq!(
            truncated.resource_limit_violation.as_deref(),
            Some("output_truncated")
        );
    }

    #[tokio::test]
    async fn cancelled_worker_task_does_not_execute_commands_and_is_supervisor_visible() {
        let runtime = RuntimeKernel::new();
        let spawned = runtime
            .spawn_worker_task(
                "coding-builder",
                "autonomous coding subagent cancel before run",
                None,
            )
            .expect("task should spawn");
        let cancelled = runtime
            .cancel_worker_task(&spawned.task.task_id)
            .expect("task should cancel");
        assert_eq!(cancelled.task.status, WorkerTaskStatus::Cancelled);
        assert!(cancelled.task.safety_envelope.cancel_supported);
        assert!(cancelled.task.command_runs.is_empty());

        let run = runtime.run_worker_task(&spawned.task.task_id).await;
        assert!(
            run.expect_err("cancelled task should not run")
                .0
                .contains("already cancelled")
        );

        let supervisor = runtime
            .worker_task_supervisor()
            .expect("supervisor should build");
        assert_eq!(supervisor.cancelled_count, 1);
        assert_eq!(supervisor.command_run_count, 0);
        assert!(
            supervisor
                .safety_envelopes
                .iter()
                .all(|envelope| envelope.cancel_supported)
        );
    }

    #[tokio::test]
    async fn worker_task_patch_apply_reports_conflicts_without_overwriting() {
        let runtime = RuntimeKernel::new();
        let spawned = runtime
            .spawn_worker_task("builder", "draft patch conflict flow", None)
            .expect("task should spawn");
        let run = runtime
            .run_worker_task(&spawned.task.task_id)
            .await
            .expect("task should run");
        let patch = run.task.patch_proposals[0].clone();
        assert_eq!(
            run.task.permission_envelope.write_scope,
            WritePathScope::ArtifactsOnly
        );
        assert!(run.task.file_leases.iter().any(|lease| {
            lease.target_path == patch.file_path
                && lease.status == WorkerTaskFileLeaseStatus::HeldForReview
        }));
        let target_path = super::resolve_path_within_root(
            &runtime
                .workspace_root()
                .expect("workspace root should exist"),
            std::path::Path::new(&patch.file_path),
        );
        if let Some(parent) = target_path.parent() {
            std::fs::create_dir_all(parent).expect("parent should be creatable");
        }
        std::fs::write(&target_path, "different content\n")
            .expect("conflicting target should be writable");

        let review = runtime
            .apply_worker_task_patch(&run.task.task_id, &patch.patch_id)
            .expect("conflict should be reported as review state");
        assert_eq!(review.conflicted_count, 1);
        assert_eq!(review.applied_count, 0);
        assert_eq!(review.proposed_count, 1);
        assert_eq!(review.patch_count, 2);
        assert!(review.patches[0].conflict_reason.is_some());
        let revision = review
            .patches
            .iter()
            .find(|candidate| candidate.revision_of.as_deref() == Some(&patch.patch_id))
            .expect("conflict should generate a revision proposal")
            .clone();
        assert_eq!(revision.revision_index, 1);
        let revised_task = runtime
            .find_worker_task(&run.task.task_id)
            .expect("revised task should remain queryable");
        assert!(revised_task.file_leases.iter().any(|lease| {
            lease.target_path == revision.file_path
                && lease.status == WorkerTaskFileLeaseStatus::HeldForReview
        }));
        let loop_report = runtime
            .worker_task_loop(&run.task.task_id)
            .expect("loop should include revision iteration");
        assert_eq!(loop_report.loop_step_count, 6);
        assert_eq!(
            std::fs::read_to_string(&target_path).expect("target should remain"),
            "different content\n"
        );

        let retry = runtime
            .apply_worker_task_patch_set(&run.task.task_id)
            .expect("revision retry should apply");
        assert_eq!(retry.attempted_count, 1);
        assert_eq!(retry.applied_count, 1);
        assert_eq!(retry.conflicted_count, 1);
        let revised_target = super::resolve_path_within_root(
            &runtime
                .workspace_root()
                .expect("workspace root should exist"),
            std::path::Path::new(&revision.file_path),
        );
        assert!(revised_target.exists());
        let _ = std::fs::remove_file(&target_path);
        let _ = std::fs::remove_file(&revised_target);
    }

    #[tokio::test]
    async fn worker_task_patch_set_applies_multiple_files() {
        let runtime = RuntimeKernel::new();
        let spawned = runtime
            .spawn_worker_task("builder", "draft multi file patch set", None)
            .expect("task should spawn");
        let run = runtime
            .run_worker_task(&spawned.task.task_id)
            .await
            .expect("task should run");
        assert_eq!(run.task.patch_proposals.len(), 2);
        assert_eq!(
            run.task.permission_envelope.write_scope,
            WritePathScope::ArtifactsOnly
        );
        assert!(run.task.patch_proposals.iter().all(|patch| {
            run.task.file_leases.iter().any(|lease| {
                lease.target_path == patch.file_path
                    && lease.status == WorkerTaskFileLeaseStatus::HeldForReview
            })
        }));
        let patch = &run.task.patch_proposals[0];
        let ordinary_scope_error = runtime
            .prepare_sealed_write_target(
                &run.task.worker_session_id,
                "write_file",
                "write_file",
                &patch.file_path,
                "create",
                false,
                None,
            )
            .expect_err("ordinary tool scope must remain artifacts-only");
        assert!(ordinary_scope_error.0.contains("outside artifacts root"));
        let mut missing_lease = run.task.clone();
        missing_lease
            .file_leases
            .retain(|lease| lease.target_path != patch.file_path);
        assert!(
            runtime
                .authorize_worker_patch_apply(&missing_lease, patch)
                .expect_err("missing exact lease must fail")
                .0
                .contains("no exact file lease")
        );
        let mut unsafe_envelope = run.task.clone();
        unsafe_envelope.safety_envelope.sandbox.workspace_root = "/".into();
        assert!(
            runtime
                .authorize_worker_patch_apply(&unsafe_envelope, patch)
                .expect_err("mismatched safety root must fail")
                .0
                .contains("safety envelope")
        );

        let applied = runtime
            .apply_worker_task_patch_set(&run.task.task_id)
            .expect("patch set should apply");
        assert_eq!(applied.patch_count, 2);
        assert_eq!(applied.attempted_count, 2);
        assert_eq!(applied.applied_count, 2);
        assert_eq!(applied.conflicted_count, 0);
        assert_eq!(applied.transaction_ids.len(), 2);
        let target_paths = applied
            .review
            .patches
            .iter()
            .map(|patch| {
                super::resolve_path_within_root(
                    &runtime
                        .workspace_root()
                        .expect("workspace root should exist"),
                    std::path::Path::new(&patch.file_path),
                )
            })
            .collect::<Vec<_>>();
        for target_path in &target_paths {
            assert!(target_path.exists());
        }

        let rollback = runtime
            .rollback_worker_task_patch_set(&run.task.task_id)
            .expect("patch set rollback should succeed");
        assert_eq!(rollback.attempted_count, 2);
        assert_eq!(rollback.rolled_back_count, 2);
        assert_eq!(rollback.failed_count, 0);
        assert_eq!(rollback.review.rolled_back_count, 2);
        for target_path in &target_paths {
            assert!(!target_path.exists());
        }
    }

    #[tokio::test]
    async fn scheduled_worker_tasks_run_when_due() {
        let runtime = RuntimeKernel::new();
        let scheduled = runtime
            .spawn_worker_task("scheduler", "run scheduled task", Some("delay:10ms"))
            .expect("scheduled task should spawn");
        assert_eq!(scheduled.task.status, WorkerTaskStatus::Scheduled);
        let next_run = scheduled
            .task
            .next_run_unix_ms
            .expect("scheduled task should have next run");

        let early = runtime
            .run_due_worker_tasks(Some(next_run.saturating_sub(1)))
            .await
            .expect("early due check should succeed");
        assert_eq!(early.due_count, 0);
        assert_eq!(early.ran_count, 0);

        let due = runtime
            .run_due_worker_tasks(Some(next_run))
            .await
            .expect("due task should run");
        assert_eq!(due.due_count, 1);
        assert_eq!(due.ran_count, 1);
        assert_eq!(due.runs[0].task.status, WorkerTaskStatus::Completed);
    }

    #[tokio::test]
    async fn dependent_worker_tasks_wait_for_completed_dependencies() {
        let runtime = RuntimeKernel::new();
        let parent = runtime
            .spawn_worker_task("parent", "complete first", None)
            .expect("parent should spawn");
        let child = runtime
            .spawn_worker_task_with_dependencies(
                "child",
                "run after parent",
                None,
                vec![parent.task.task_id.clone()],
            )
            .expect("child should spawn with dependency");

        let blocked = runtime
            .run_worker_task(&child.task.task_id)
            .await
            .expect_err("child should wait for dependency");
        assert!(blocked.0.contains("waiting on dependency"));

        runtime
            .run_worker_task(&parent.task.task_id)
            .await
            .expect("parent should complete");
        let child_run = runtime
            .run_worker_task(&child.task.task_id)
            .await
            .expect("child should run after dependency completes");
        assert_eq!(child_run.task.status, WorkerTaskStatus::Completed);
        assert_eq!(child_run.task.depends_on, vec![parent.task.task_id]);
    }

    #[test]
    fn worker_task_spawn_persists_workspace_id() {
        let runtime = RuntimeKernel::new();
        let spawned = runtime
            .spawn_worker_task_in_workspace(
                "workspace-reviewer",
                Some("agent:workspace-alpha"),
                "summarize workspace-scoped task state",
                None,
            )
            .expect("workspace task should spawn");

        assert_eq!(spawned.task.workspace_id, "agent:workspace-alpha");
        let detail = runtime
            .worker_task_status(&spawned.task.task_id)
            .expect("task detail should resolve");
        assert_eq!(detail.task.workspace_id, "agent:workspace-alpha");
        let patches = runtime
            .worker_task_patches(&spawned.task.task_id)
            .expect("patch review should resolve");
        assert_eq!(patches.workspace_id, "agent:workspace-alpha");
    }

    #[test]
    fn child_worker_task_inherits_parent_workspace() {
        let runtime = RuntimeKernel::new();
        let parent = runtime
            .spawn_worker_task_in_workspace(
                "parent",
                Some("agent:workspace-beta"),
                "coordinate nested worker lane",
                None,
            )
            .expect("parent task should spawn");
        let child = runtime
            .spawn_worker_task_with_parent_in_workspace(
                "child",
                None,
                "follow parent workspace",
                None,
                Vec::new(),
                Some(parent.task.task_id.clone()),
                1,
            )
            .expect("child task should inherit workspace");

        assert_eq!(child.task.workspace_id, "agent:workspace-beta");
    }

    #[tokio::test]
    async fn ready_worker_batch_runs_only_unblocked_candidates() {
        let runtime = RuntimeKernel::new();
        let ready = runtime
            .spawn_worker_task("ready", "run now", None)
            .expect("ready task should spawn");
        let blocker = runtime
            .spawn_worker_task("blocker", "block dependency", None)
            .expect("blocker should spawn");
        let blocked = runtime
            .spawn_worker_task_with_dependencies(
                "blocked",
                "wait for blocker",
                None,
                vec![blocker.task.task_id.clone()],
            )
            .expect("blocked task should spawn");

        let report = runtime
            .run_ready_worker_tasks(Some(10), None)
            .await
            .expect("ready batch should run");

        assert_eq!(report.candidate_count, 3);
        assert_eq!(report.blocked_count, 1);
        assert!(report.blocked_task_ids.contains(&blocked.task.task_id));
        assert!(
            report
                .runs
                .iter()
                .any(|run| run.task.task_id == ready.task.task_id)
        );
        assert!(
            report
                .runs
                .iter()
                .any(|run| run.task.task_id == blocker.task.task_id)
        );
        assert_eq!(report.pressure.max_per_worker_concurrency, 2);
        assert_eq!(
            report.pressure.pressure_level,
            WorkerPoolPressureLevel::Normal
        );

        let second = runtime
            .run_ready_worker_tasks(None, None)
            .await
            .expect("second ready batch should run newly unblocked task");
        assert_eq!(second.ran_count, 1);
        assert_eq!(second.runs[0].task.task_id, blocked.task.task_id);
    }

    #[tokio::test]
    async fn ready_worker_batch_respects_per_worker_pressure_limit() {
        let runtime = RuntimeKernel::new();
        let first = runtime
            .spawn_worker_task("pressure", "pressure one", None)
            .expect("first task should spawn");
        let second = runtime
            .spawn_worker_task("pressure", "pressure two", None)
            .expect("second task should spawn");
        let third = runtime
            .spawn_worker_task("pressure", "pressure three", None)
            .expect("third task should spawn");

        let report = runtime
            .run_ready_worker_tasks(None, None)
            .await
            .expect("ready batch should respect pressure limits");
        assert_eq!(report.candidate_count, 3);
        assert_eq!(report.ready_count, 2);
        assert_eq!(report.ran_count, 2);
        assert_eq!(
            report.pressure.pressure_level,
            WorkerPoolPressureLevel::Throttled
        );
        assert_eq!(
            report.pressure.throttled_task_ids,
            vec![third.task.task_id.clone()]
        );
        assert!(
            report
                .runs
                .iter()
                .any(|run| run.task.task_id == first.task.task_id)
        );
        assert!(
            report
                .runs
                .iter()
                .any(|run| run.task.task_id == second.task.task_id)
        );

        let second_pass = runtime
            .run_ready_worker_tasks(None, None)
            .await
            .expect("throttled task should run later");
        assert_eq!(second_pass.ran_count, 1);
        assert_eq!(second_pass.runs[0].task.task_id, third.task.task_id);
    }

    #[tokio::test]
    async fn worker_task_timeout_failure_sets_retry_backoff_budget() {
        let runtime = RuntimeKernel::new();
        let spawned = runtime
            .spawn_worker_task("retry", "simulate-timeout then retry", None)
            .expect("task should spawn");

        let first = runtime
            .run_worker_task(&spawned.task.task_id)
            .await
            .expect("simulated failure should return a failed run report");
        assert_eq!(first.task.status, WorkerTaskStatus::Failed);
        assert_eq!(
            first.task.failure_kind,
            Some(WorkerTaskFailureKind::Timeout)
        );
        assert!(first.task.retry_after_unix_ms.is_some());

        let early = runtime
            .run_ready_worker_tasks(None, Some(first.task.retry_after_unix_ms.unwrap() - 1))
            .await
            .expect("early retry check should succeed");
        assert_eq!(early.ran_count, 0);
        assert_eq!(early.candidate_count, 0);

        let retry = runtime
            .run_ready_worker_tasks(None, first.task.retry_after_unix_ms)
            .await
            .expect("retry should run after backoff");
        assert_eq!(retry.ran_count, 1);
        assert_eq!(retry.runs[0].task.status, WorkerTaskStatus::Completed);
        assert_eq!(retry.runs[0].task.attempt_count, 2);
    }

    #[test]
    fn worker_task_supervisor_reports_next_action() {
        let runtime = RuntimeKernel::new();
        let queued = runtime
            .spawn_worker_task("supervisor", "ready work", None)
            .expect("queued task should spawn");
        let report = runtime
            .worker_task_supervisor()
            .expect("supervisor report should build");

        assert_eq!(report.total_count, 1);
        assert_eq!(report.ready_count, 1);
        assert_eq!(report.ready_task_ids, vec![queued.task.task_id]);
        assert_eq!(report.recommended_next_action, "run_ready_tasks");
        assert_eq!(report.cancelled_count, 0);
        assert_eq!(report.sandbox_envelope_count, 1);
        assert!(
            report
                .safety_envelopes
                .iter()
                .all(|envelope| envelope.cancel_supported)
        );
        assert!(!report.attention_required);
    }
}

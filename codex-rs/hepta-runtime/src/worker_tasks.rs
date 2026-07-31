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
        tasks.sort_by_key(|task| task.created_at_unix_ms);
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
            execution_receipt: None,
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
        tasks.sort_by_key(|task| task.updated_at_unix_ms);
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
        tasks.sort_by_key(|task| task.updated_at_unix_ms);
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

include!("worker_tasks/builders.rs");

#[cfg(test)]
#[path = "worker_tasks/tests.rs"]
mod tests;

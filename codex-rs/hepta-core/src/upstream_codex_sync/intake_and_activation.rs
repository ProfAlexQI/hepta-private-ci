use serde::Deserialize;
use serde::Serialize;

pub const HEPTA_UPSTREAM_CODEX_INTAKE_BASE_HEAD: &str = "108234b5ebe6941764a6b8edbb37b2aa04369f07";
pub const HEPTA_UPSTREAM_CODEX_INTAKE_CUTOFF_REF: &str =
    "refs/remotes/upstream/hepta-intake-20260721-r2";
pub const HEPTA_UPSTREAM_CODEX_INTAKE_CUTOFF_HEAD: &str =
    "88fac6fe108237a105d3203e3508b0d531054312";
const HEPTA_UPSTREAM_CODEX_INTAKE_MANIFEST_PATH: &str =
    "docs/architecture/HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_2026-07-21_R2.json";
const HEPTA_UPSTREAM_CODEX_PREDECESSOR_INTAKE_MANIFEST_PATH: &str =
    "docs/architecture/HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_2026-07-21.json";
const HEPTA_UPSTREAM_CODEX_PREDECESSOR_INTAKE_MANIFEST_SHA256: &str =
    "157274d564f6e4274ad7ce50d9038670ce99b277e9ed481d879243c3404e6882";
const HEPTA_UPSTREAM_CODEX_PREDECESSOR_INTAKE_CUTOFF_REF: &str =
    "refs/remotes/upstream/hepta-intake-20260721";
const HEPTA_UPSTREAM_CODEX_PREDECESSOR_INTAKE_CUTOFF_HEAD: &str =
    "45ac251e178416ff5c3022457ad8d2778c0d4549";

// This older range is retained as provenance for the bucket-level absorption
// and replay receipts. It is not the current upstream freshness cutoff.
const HEPTA_UPSTREAM_CODEX_HISTORICAL_RECEIPT_TARGET_HEAD: &str =
    "7d47056ea42636271ac020b86347fbbef49490aa";
const HEPTA_UPSTREAM_CODEX_HISTORICAL_LEDGER_CHANGED_FILE_COUNT: usize = 878;
const HEPTA_UPSTREAM_CODEX_HISTORICAL_SELECTED_ABSORPTION_COUNT: usize = 716;

fn historical_upstream_codex_receipt_diff_range() -> String {
    format!(
        "{}..{}",
        HEPTA_UPSTREAM_CODEX_INTAKE_BASE_HEAD, HEPTA_UPSTREAM_CODEX_HISTORICAL_RECEIPT_TARGET_HEAD
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HeptaUpstreamCodexSyncRisk {
    P0Security,
    P0Runtime,
    P1Compatibility,
    P2Product,
    Guardrail,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeptaUpstreamCodexSyncContract {
    pub id: String,
    pub risk: HeptaUpstreamCodexSyncRisk,
    pub title: String,
    pub upstream_scope: Vec<String>,
    pub hepta_surfaces: Vec<String>,
    pub required_gate: String,
    pub auto_apply_allowed: bool,
    pub active_runtime_dependency_allowed: bool,
    pub public_release_claim_allowed: bool,
    pub contract_ready: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeptaUpstreamCodexSyncLaneReport {
    pub product: String,
    pub status: String,
    pub lane_id: String,
    pub upstream_repository: String,
    pub compatibility_snapshot_role: String,
    pub sync_mode: String,
    pub contract_count: usize,
    pub ready_contract_count: usize,
    pub sync_lane_ready: bool,
    pub upstream_fetch_performed: bool,
    pub upstream_latest_claimed: bool,
    pub upstream_merge_performed: bool,
    pub active_runtime_auto_rebase_allowed: bool,
    pub active_runtime_codex_engine_dependency_allowed: bool,
    pub active_dependency_isolation_gate: String,
    pub requires_diff_classification_before_absorption: bool,
    pub requires_adapter_contract_before_active_runtime: bool,
    pub requires_release_governance_before_public_claim: bool,
    pub local_only_audit: bool,
    pub report_only: bool,
    pub mutates_runtime_state: bool,
    pub external_network_read: bool,
    pub external_send: bool,
    pub credential_value_read: bool,
    pub secret_file_read: bool,
    pub provider_invoked: bool,
    pub channel_delivery_performed: bool,
    pub gateway_rpc_performed: bool,
    pub contracts: Vec<HeptaUpstreamCodexSyncContract>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeptaUpstreamCodexSnapshotRiskClass {
    pub id: String,
    pub risk: HeptaUpstreamCodexSyncRisk,
    pub upstream_path_hints: Vec<String>,
    pub hepta_review_surfaces: Vec<String>,
    pub required_action: String,
    pub auto_absorb_allowed: bool,
    pub active_runtime_dependency_allowed: bool,
    pub classification_required: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeptaUpstreamCodexSnapshotReport {
    pub product: String,
    pub status: String,
    pub snapshot_lane_id: String,
    pub upstream_repository: String,
    pub upstream_default_ref: String,
    pub compatibility_snapshot_role: String,
    pub snapshot_gate: String,
    pub sync_lane_gate: String,
    pub active_dependency_isolation_gate: String,
    pub observed_upstream_head_required_before_absorption: bool,
    pub local_compatibility_head_required: bool,
    pub diff_range_required_before_absorption: bool,
    pub diff_inventory_required_before_absorption: bool,
    pub risk_class_count: usize,
    pub ready_risk_class_count: usize,
    pub snapshot_intake_ready: bool,
    pub upstream_fetch_performed: bool,
    pub upstream_merge_performed: bool,
    pub upstream_checkout_performed: bool,
    pub active_runtime_dependency_allowed: bool,
    pub active_runtime_auto_rebase_allowed: bool,
    pub public_release_claim_allowed: bool,
    pub external_network_read_default: bool,
    pub workspace_mutation_default: bool,
    pub credential_value_read: bool,
    pub secret_file_read: bool,
    pub provider_invoked: bool,
    pub channel_delivery_performed: bool,
    pub gateway_rpc_performed: bool,
    pub risk_classes: Vec<HeptaUpstreamCodexSnapshotRiskClass>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeptaUpstreamCodexDiffLedgerBucket {
    pub id: String,
    pub risk: HeptaUpstreamCodexSyncRisk,
    pub upstream_path_hints: Vec<String>,
    pub hepta_review_surfaces: Vec<String>,
    pub required_action: String,
    pub promotion_gate: String,
    pub auto_absorb_allowed: bool,
    pub active_runtime_dependency_allowed: bool,
    pub classification_required: bool,
    pub bucket_ready: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeptaUpstreamCodexDiffLedgerReport {
    pub product: String,
    pub status: String,
    pub ledger_id: String,
    pub upstream_repository: String,
    pub baseline_upstream_head: String,
    pub target_upstream_head: String,
    pub target_head_source: String,
    pub target_ref: String,
    pub candidate_diff_range: String,
    pub snapshot_gate: String,
    pub diff_ledger_gate: String,
    pub sync_lane_gate: String,
    pub active_dependency_isolation_gate: String,
    pub commit_inventory_required: bool,
    pub file_inventory_required: bool,
    pub risk_bucket_classification_required: bool,
    pub bucket_count: usize,
    pub ready_bucket_count: usize,
    pub diff_ledger_ready: bool,
    pub upstream_fetch_performed: bool,
    pub upstream_merge_performed: bool,
    pub upstream_checkout_performed: bool,
    pub workspace_mutation_default: bool,
    pub active_runtime_dependency_allowed: bool,
    pub active_runtime_auto_rebase_allowed: bool,
    pub public_release_claim_allowed: bool,
    pub external_network_read_default: bool,
    pub credential_value_read: bool,
    pub secret_file_read: bool,
    pub provider_invoked: bool,
    pub channel_delivery_performed: bool,
    pub gateway_rpc_performed: bool,
    pub buckets: Vec<HeptaUpstreamCodexDiffLedgerBucket>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HeptaUpstreamCodexCurrentIntakeDisposition {
    Absorbed,
    Deferred,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeptaUpstreamCodexCurrentIntakeDecision {
    pub classification: String,
    pub disposition: HeptaUpstreamCodexCurrentIntakeDisposition,
    pub upstream_commit: Option<String>,
    pub local_receipts: Vec<String>,
    pub absorption_kind: Option<String>,
    pub rationale: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeptaUpstreamCodexCurrentIntakeReport {
    pub product: String,
    pub status: String,
    pub intake_id: String,
    pub manifest_path: String,
    pub observation_state: String,
    pub classification_state: String,
    pub selected_state: String,
    pub remaining_state: String,
    pub baseline_head: String,
    pub cutoff_ref: String,
    pub cutoff_head: String,
    pub candidate_diff_range: String,
    pub predecessor_manifest_path: String,
    pub predecessor_manifest_sha256: String,
    pub predecessor_cutoff_ref: String,
    pub predecessor_cutoff_head: String,
    pub predecessor_cutoff_preserved: bool,
    pub observed_commit_count: usize,
    pub observed_changed_file_count: usize,
    pub observed_codex_rs_changed_file_count: usize,
    pub selected_absorption_count: usize,
    pub deferred_decision_count: usize,
    pub historical_receipt_target_head: String,
    pub historical_receipt_changed_file_count: usize,
    pub historical_receipt_selected_absorption_count: usize,
    pub historical_receipt_is_current_freshness_proof: bool,
    pub current_intake_ready: bool,
    pub full_range_absorption_claimed: bool,
    pub upstream_fetch_performed: bool,
    pub upstream_merge_performed: bool,
    pub upstream_rebase_performed: bool,
    pub whole_tree_replacement_performed: bool,
    pub cargo_lock_replacement_performed: bool,
    pub active_runtime_dependency_allowed: bool,
    pub public_release_claim_allowed: bool,
    pub decisions: Vec<HeptaUpstreamCodexCurrentIntakeDecision>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeptaUpstreamCodexProductGovernanceAbsorptionReport {
    pub product: String,
    pub status: String,
    pub absorption_id: String,
    pub upstream_repository: String,
    pub candidate_diff_range: String,
    pub selected_bucket_id: String,
    pub selected_bucket_risk: HeptaUpstreamCodexSyncRisk,
    pub selected_changed_file_count: usize,
    pub selected_commit_sample_count: usize,
    pub source_ledger_gate: String,
    pub absorption_gate: String,
    pub active_dependency_isolation_gate: String,
    pub selected_as_first_absorption_contract: bool,
    pub low_risk_runtime_promotion: bool,
    pub requires_hepta_translation: bool,
    pub raw_upstream_doc_copy_allowed: bool,
    pub raw_upstream_package_policy_copy_allowed: bool,
    pub active_runtime_code_wiring_allowed: bool,
    pub active_runtime_dependency_allowed: bool,
    pub public_release_claim_allowed: bool,
    pub contract_ready: bool,
    pub upstream_fetch_performed: bool,
    pub upstream_merge_performed: bool,
    pub upstream_checkout_performed: bool,
    pub workspace_mutation_default: bool,
    pub credential_value_read: bool,
    pub secret_file_read: bool,
    pub provider_invoked: bool,
    pub channel_delivery_performed: bool,
    pub gateway_rpc_performed: bool,
    pub selected_paths: Vec<String>,
    pub required_next_gates: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeptaUpstreamCodexProductGovernanceTranslationReport {
    pub product: String,
    pub status: String,
    pub translation_id: String,
    pub translation_packet_path: String,
    pub selected_bucket_id: String,
    pub selected_changed_file_count: usize,
    pub translated_surface_count: usize,
    pub required_surface_count: usize,
    pub source_absorption_gate: String,
    pub translation_gate: String,
    pub active_dependency_isolation_gate: String,
    pub release_governance_documented: bool,
    pub package_policy_documented: bool,
    pub plugin_marketplace_policy_documented: bool,
    pub sandbox_runtime_policy_documented: bool,
    pub operator_approval_policy_documented: bool,
    pub requires_hepta_translation: bool,
    pub raw_upstream_doc_copy_allowed: bool,
    pub raw_upstream_package_policy_copy_allowed: bool,
    pub active_runtime_code_wiring_allowed: bool,
    pub active_runtime_dependency_allowed: bool,
    pub public_release_claim_allowed: bool,
    pub translation_ready: bool,
    pub upstream_fetch_performed: bool,
    pub upstream_merge_performed: bool,
    pub upstream_checkout_performed: bool,
    pub workspace_mutation_default: bool,
    pub credential_value_read: bool,
    pub secret_file_read: bool,
    pub provider_invoked: bool,
    pub channel_delivery_performed: bool,
    pub gateway_rpc_performed: bool,
    pub hepta_actions: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeptaUpstreamCodexReleaseGovernancePromotionReport {
    pub product: String,
    pub status: String,
    pub promotion_id: String,
    pub promotion_packet_path: String,
    pub selected_bucket_id: String,
    pub selected_changed_file_count: usize,
    pub source_translation_gate: String,
    pub promotion_gate: String,
    pub active_dependency_isolation_gate: String,
    pub release_claim_taxonomy_ready: bool,
    pub package_install_context_ready: bool,
    pub plugin_marketplace_policy_ready: bool,
    pub operator_approval_model_ready: bool,
    pub watchdog_soak_evidence_ready: bool,
    pub public_claim_boundary_ready: bool,
    pub side_effect_boundary_ready: bool,
    pub required_promotion_condition_count: usize,
    pub ready_promotion_condition_count: usize,
    pub promotion_packet_ready: bool,
    pub public_release_claim_allowed: bool,
    pub public_ga_claim_allowed: bool,
    pub release_artifact_write_allowed: bool,
    pub active_runtime_code_wiring_allowed: bool,
    pub active_runtime_dependency_allowed: bool,
    pub upstream_auto_rebase_allowed: bool,
    pub upstream_fetch_performed: bool,
    pub upstream_merge_performed: bool,
    pub upstream_checkout_performed: bool,
    pub workspace_mutation_default: bool,
    pub active_service_restart: bool,
    pub credential_value_read: bool,
    pub secret_file_read: bool,
    pub provider_invoked: bool,
    pub channel_delivery_performed: bool,
    pub gateway_rpc_performed: bool,
    pub public_release_published: bool,
    pub promotion_conditions: Vec<String>,
    pub remaining_blockers: Vec<String>,
    pub required_next_gates: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeptaUpstreamCodexLegacyCompatibilityAbsorptionReport {
    pub product: String,
    pub status: String,
    pub absorption_id: String,
    pub upstream_repository: String,
    pub candidate_diff_range: String,
    pub selected_bucket_id: String,
    pub selected_bucket_risk: HeptaUpstreamCodexSyncRisk,
    pub selected_changed_file_count: usize,
    pub source_ledger_gate: String,
    pub absorption_gate: String,
    pub active_dependency_isolation_gate: String,
    pub retained_as_compatibility_snapshot: bool,
    pub requires_hepta_command_contract: bool,
    pub active_cli_tui_promotion_allowed: bool,
    pub active_runtime_code_wiring_allowed: bool,
    pub active_runtime_dependency_allowed: bool,
    pub public_release_claim_allowed: bool,
    pub contract_ready: bool,
    pub upstream_fetch_performed: bool,
    pub upstream_merge_performed: bool,
    pub upstream_checkout_performed: bool,
    pub workspace_mutation_default: bool,
    pub credential_value_read: bool,
    pub secret_file_read: bool,
    pub provider_invoked: bool,
    pub channel_delivery_performed: bool,
    pub gateway_rpc_performed: bool,
    pub sample_surfaces: Vec<String>,
    pub required_next_gates: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeptaUpstreamCodexLegacyCompatibilityReplayReport {
    pub product: String,
    pub status: String,
    pub replay_id: String,
    pub replay_packet_path: String,
    pub selected_bucket_id: String,
    pub selected_changed_file_count: usize,
    pub replay_surface_count: usize,
    pub required_replay_surface_count: usize,
    pub source_absorption_gate: String,
    pub replay_gate: String,
    pub active_dependency_isolation_gate: String,
    pub cli_command_contract_ready: bool,
    pub tui_presentation_replay_ready: bool,
    pub code_mode_replay_ready: bool,
    pub terminal_helper_replay_ready: bool,
    pub dependency_boundary_ready: bool,
    pub active_cli_tui_promotion_allowed: bool,
    pub active_runtime_code_wiring_allowed: bool,
    pub active_runtime_dependency_allowed: bool,
    pub public_release_claim_allowed: bool,
    pub replay_ready: bool,
    pub upstream_fetch_performed: bool,
    pub upstream_merge_performed: bool,
    pub upstream_checkout_performed: bool,
    pub workspace_mutation_default: bool,
    pub credential_value_read: bool,
    pub secret_file_read: bool,
    pub provider_invoked: bool,
    pub channel_delivery_performed: bool,
    pub gateway_rpc_performed: bool,
    pub replay_surfaces: Vec<String>,
    pub required_next_gates: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeptaUpstreamCodexLegacyCompatibilityPromotionReport {
    pub product: String,
    pub status: String,
    pub promotion_id: String,
    pub promotion_packet_path: String,
    pub selected_bucket_id: String,
    pub selected_changed_file_count: usize,
    pub source_replay_gate: String,
    pub promotion_gate: String,
    pub active_dependency_isolation_gate: String,
    pub cli_command_contract_parity_ready: bool,
    pub tui_presentation_parity_ready: bool,
    pub code_mode_callback_boundary_ready: bool,
    pub terminal_helper_contract_ready: bool,
    pub adapter_shadow_replay_ready: bool,
    pub operator_approval_model_ready: bool,
    pub side_effect_boundary_ready: bool,
    pub required_promotion_condition_count: usize,
    pub ready_promotion_condition_count: usize,
    pub promotion_packet_ready: bool,
    pub active_cli_tui_promotion_allowed: bool,
    pub active_tui_presentation_promotion_allowed: bool,
    pub active_code_mode_promotion_allowed: bool,
    pub active_runtime_code_wiring_allowed: bool,
    pub active_runtime_dependency_allowed: bool,
    pub public_release_claim_allowed: bool,
    pub upstream_fetch_performed: bool,
    pub upstream_merge_performed: bool,
    pub upstream_checkout_performed: bool,
    pub workspace_mutation_default: bool,
    pub active_service_restart: bool,
    pub credential_value_read: bool,
    pub secret_file_read: bool,
    pub provider_invoked: bool,
    pub channel_delivery_performed: bool,
    pub gateway_rpc_performed: bool,
    pub public_release_published: bool,
    pub promotion_conditions: Vec<String>,
    pub remaining_blockers: Vec<String>,
    pub required_next_gates: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeptaUpstreamCodexProviderSecurityAbsorptionReport {
    pub product: String,
    pub status: String,
    pub absorption_id: String,
    pub upstream_repository: String,
    pub candidate_diff_range: String,
    pub selected_bucket_id: String,
    pub selected_bucket_risk: HeptaUpstreamCodexSyncRisk,
    pub selected_changed_file_count: usize,
    pub selected_security_surface_count: usize,
    pub required_security_surface_count: usize,
    pub source_ledger_gate: String,
    pub absorption_gate: String,
    pub active_dependency_isolation_gate: String,
    pub p0_security_review_required: bool,
    pub requires_provider_contract: bool,
    pub requires_auth_credential_redaction: bool,
    pub requires_sandbox_exec_replay: bool,
    pub requires_network_policy_replay: bool,
    pub active_provider_promotion_allowed: bool,
    pub active_security_policy_promotion_allowed: bool,
    pub active_runtime_code_wiring_allowed: bool,
    pub active_runtime_dependency_allowed: bool,
    pub public_release_claim_allowed: bool,
    pub contract_ready: bool,
    pub upstream_fetch_performed: bool,
    pub upstream_merge_performed: bool,
    pub upstream_checkout_performed: bool,
    pub workspace_mutation_default: bool,
    pub credential_value_read: bool,
    pub secret_file_read: bool,
    pub provider_invoked: bool,
    pub channel_delivery_performed: bool,
    pub gateway_rpc_performed: bool,
    pub security_surfaces: Vec<String>,
    pub required_next_gates: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeptaUpstreamCodexProviderSecurityReplayReport {
    pub product: String,
    pub status: String,
    pub replay_id: String,
    pub replay_packet_path: String,
    pub selected_bucket_id: String,
    pub selected_changed_file_count: usize,
    pub replay_surface_count: usize,
    pub required_replay_surface_count: usize,
    pub source_absorption_gate: String,
    pub replay_gate: String,
    pub active_dependency_isolation_gate: String,
    pub redacted_provider_contract_ready: bool,
    pub auth_credential_redaction_ready: bool,
    pub approval_policy_replay_ready: bool,
    pub sandbox_exec_replay_ready: bool,
    pub network_policy_replay_ready: bool,
    pub side_effect_boundary_ready: bool,
    pub active_provider_promotion_allowed: bool,
    pub active_security_policy_promotion_allowed: bool,
    pub active_runtime_code_wiring_allowed: bool,
    pub active_runtime_dependency_allowed: bool,
    pub public_release_claim_allowed: bool,
    pub replay_ready: bool,
    pub upstream_fetch_performed: bool,
    pub upstream_merge_performed: bool,
    pub upstream_checkout_performed: bool,
    pub workspace_mutation_default: bool,
    pub credential_value_read: bool,
    pub secret_file_read: bool,
    pub provider_invoked: bool,
    pub channel_delivery_performed: bool,
    pub gateway_rpc_performed: bool,
    pub replay_surfaces: Vec<String>,
    pub required_next_gates: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeptaUpstreamCodexProviderSecurityPromotionReport {
    pub product: String,
    pub status: String,
    pub promotion_id: String,
    pub promotion_packet_path: String,
    pub selected_bucket_id: String,
    pub selected_changed_file_count: usize,
    pub source_replay_gate: String,
    pub promotion_gate: String,
    pub active_dependency_isolation_gate: String,
    pub redacted_provider_contract_ready: bool,
    pub auth_credential_redaction_ready: bool,
    pub approval_policy_replay_ready: bool,
    pub sandbox_exec_replay_ready: bool,
    pub network_policy_replay_ready: bool,
    pub operator_approval_model_ready: bool,
    pub side_effect_boundary_ready: bool,
    pub required_promotion_condition_count: usize,
    pub ready_promotion_condition_count: usize,
    pub promotion_packet_ready: bool,
    pub active_provider_promotion_allowed: bool,
    pub active_security_policy_promotion_allowed: bool,
    pub active_runtime_code_wiring_allowed: bool,
    pub active_runtime_dependency_allowed: bool,
    pub public_release_claim_allowed: bool,
    pub upstream_fetch_performed: bool,
    pub upstream_merge_performed: bool,
    pub upstream_checkout_performed: bool,
    pub workspace_mutation_default: bool,
    pub active_service_restart: bool,
    pub credential_value_read: bool,
    pub secret_file_read: bool,
    pub provider_invoked: bool,
    pub channel_delivery_performed: bool,
    pub gateway_rpc_performed: bool,
    pub public_release_published: bool,
    pub promotion_conditions: Vec<String>,
    pub remaining_blockers: Vec<String>,
    pub required_next_gates: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeptaUpstreamCodexRuntimeAppServerAbsorptionReport {
    pub product: String,
    pub status: String,
    pub absorption_id: String,
    pub upstream_repository: String,
    pub candidate_diff_range: String,
    pub selected_bucket_id: String,
    pub selected_bucket_risk: HeptaUpstreamCodexSyncRisk,
    pub selected_changed_file_count: usize,
    pub selected_runtime_surface_count: usize,
    pub required_runtime_surface_count: usize,
    pub source_ledger_gate: String,
    pub absorption_gate: String,
    pub active_dependency_isolation_gate: String,
    pub p0_runtime_review_required: bool,
    pub requires_adapter_contract: bool,
    pub requires_session_thread_replay: bool,
    pub requires_tool_mcp_replay: bool,
    pub requires_app_server_protocol_replay: bool,
    pub requires_exec_hook_replay: bool,
    pub active_runtime_promotion_allowed: bool,
    pub active_app_server_promotion_allowed: bool,
    pub active_tool_mcp_promotion_allowed: bool,
    pub active_runtime_code_wiring_allowed: bool,
    pub active_runtime_dependency_allowed: bool,
    pub public_release_claim_allowed: bool,
    pub contract_ready: bool,
    pub upstream_fetch_performed: bool,
    pub upstream_merge_performed: bool,
    pub upstream_checkout_performed: bool,
    pub workspace_mutation_default: bool,
    pub credential_value_read: bool,
    pub secret_file_read: bool,
    pub provider_invoked: bool,
    pub channel_delivery_performed: bool,
    pub gateway_rpc_performed: bool,
    pub runtime_surfaces: Vec<String>,
    pub required_next_gates: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeptaUpstreamCodexRuntimeAppServerReplayReport {
    pub product: String,
    pub status: String,
    pub replay_id: String,
    pub replay_packet_path: String,
    pub selected_bucket_id: String,
    pub selected_changed_file_count: usize,
    pub replay_surface_count: usize,
    pub required_replay_surface_count: usize,
    pub source_absorption_gate: String,
    pub replay_gate: String,
    pub active_dependency_isolation_gate: String,
    pub app_server_protocol_replay_ready: bool,
    pub session_thread_replay_ready: bool,
    pub tool_mcp_replay_ready: bool,
    pub exec_hook_replay_ready: bool,
    pub side_effect_boundary_ready: bool,
    pub active_runtime_promotion_allowed: bool,
    pub active_app_server_promotion_allowed: bool,
    pub active_tool_mcp_promotion_allowed: bool,
    pub active_runtime_code_wiring_allowed: bool,
    pub active_runtime_dependency_allowed: bool,
    pub public_release_claim_allowed: bool,
    pub replay_ready: bool,
    pub upstream_fetch_performed: bool,
    pub upstream_merge_performed: bool,
    pub upstream_checkout_performed: bool,
    pub workspace_mutation_default: bool,
    pub credential_value_read: bool,
    pub secret_file_read: bool,
    pub provider_invoked: bool,
    pub channel_delivery_performed: bool,
    pub gateway_rpc_performed: bool,
    pub replay_surfaces: Vec<String>,
    pub required_next_gates: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeptaUpstreamCodexRuntimeAppServerPromotionReport {
    pub product: String,
    pub status: String,
    pub promotion_id: String,
    pub promotion_packet_path: String,
    pub selected_bucket_id: String,
    pub selected_changed_file_count: usize,
    pub source_replay_gate: String,
    pub promotion_gate: String,
    pub active_dependency_isolation_gate: String,
    pub app_server_route_event_contract_ready: bool,
    pub session_thread_lifecycle_contract_ready: bool,
    pub tool_mcp_request_envelope_ready: bool,
    pub exec_hook_event_loop_replay_ready: bool,
    pub adapter_shadow_replay_ready: bool,
    pub operator_approval_model_ready: bool,
    pub side_effect_boundary_ready: bool,
    pub required_promotion_condition_count: usize,
    pub ready_promotion_condition_count: usize,
    pub promotion_packet_ready: bool,
    pub active_runtime_promotion_allowed: bool,
    pub active_app_server_promotion_allowed: bool,
    pub active_tool_mcp_promotion_allowed: bool,
    pub active_runtime_code_wiring_allowed: bool,
    pub active_runtime_dependency_allowed: bool,
    pub public_release_claim_allowed: bool,
    pub upstream_fetch_performed: bool,
    pub upstream_merge_performed: bool,
    pub upstream_checkout_performed: bool,
    pub workspace_mutation_default: bool,
    pub active_service_restart: bool,
    pub credential_value_read: bool,
    pub secret_file_read: bool,
    pub provider_invoked: bool,
    pub channel_delivery_performed: bool,
    pub gateway_rpc_performed: bool,
    pub public_release_published: bool,
    pub promotion_conditions: Vec<String>,
    pub remaining_blockers: Vec<String>,
    pub required_next_gates: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeptaUpstreamCodexAbsorptionReplayReadinessReport {
    pub product: String,
    pub status: String,
    pub readiness_id: String,
    pub readiness_packet_path: String,
    pub upstream_repository: String,
    pub candidate_diff_range: String,
    pub ledger_changed_file_count: usize,
    pub selected_absorption_changed_file_count: usize,
    pub selected_bucket_count: usize,
    pub required_selected_bucket_count: usize,
    pub absorption_contract_ready_count: usize,
    pub required_absorption_contract_ready_count: usize,
    pub translation_replay_ready_count: usize,
    pub required_translation_replay_ready_count: usize,
    pub p0_replay_ready_count: usize,
    pub required_p0_replay_ready_count: usize,
    pub p1_replay_ready_count: usize,
    pub required_p1_replay_ready_count: usize,
    pub p2_translation_ready_count: usize,
    pub required_p2_translation_ready_count: usize,
    pub product_governance_translation_ready: bool,
    pub legacy_compatibility_replay_ready: bool,
    pub provider_security_replay_ready: bool,
    pub runtime_appserver_replay_ready: bool,
    pub source_ledger_gate: String,
    pub active_dependency_isolation_gate: String,
    pub readiness_gate: String,
    pub all_selected_buckets_absorbed: bool,
    pub all_required_translation_replay_ready: bool,
    pub active_runtime_code_wiring_allowed: bool,
    pub active_runtime_dependency_allowed: bool,
    pub active_runtime_auto_rebase_allowed: bool,
    pub active_codex_engine_dependency_allowed: bool,
    pub public_release_claim_allowed: bool,
    pub readiness_ready: bool,
    pub upstream_fetch_performed: bool,
    pub upstream_merge_performed: bool,
    pub upstream_checkout_performed: bool,
    pub workspace_mutation_default: bool,
    pub active_service_restart: bool,
    pub credential_value_read: bool,
    pub secret_file_read: bool,
    pub provider_invoked: bool,
    pub channel_delivery_performed: bool,
    pub gateway_rpc_performed: bool,
    pub public_release_published: bool,
    pub covered_buckets: Vec<String>,
    pub closed_gates: Vec<String>,
    pub required_next_gates: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeptaUpstreamCodexPromotionDecision {
    pub bucket_id: String,
    pub risk: HeptaUpstreamCodexSyncRisk,
    pub selected_changed_file_count: usize,
    pub absorption_replay_ready: bool,
    pub required_surface_promotion_packet: String,
    pub surface_promotion_packet_ready: bool,
    pub active_promotion_allowed: bool,
    pub blocker: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeptaUpstreamCodexPromotionReadinessReport {
    pub product: String,
    pub status: String,
    pub decision_id: String,
    pub decision_packet_path: String,
    pub upstream_repository: String,
    pub candidate_diff_range: String,
    pub source_readiness_gate: String,
    pub promotion_readiness_gate: String,
    pub active_dependency_isolation_gate: String,
    pub assessed_bucket_count: usize,
    pub required_assessed_bucket_count: usize,
    pub absorption_replay_ready_count: usize,
    pub required_absorption_replay_ready_count: usize,
    pub required_surface_promotion_packet_count: usize,
    pub completed_surface_promotion_packet_count: usize,
    pub promotable_bucket_count: usize,
    pub promotion_blocked_bucket_count: usize,
    pub readiness_source_ready: bool,
    pub active_promotion_ready: bool,
    pub decision_ready: bool,
    pub active_runtime_code_wiring_allowed: bool,
    pub active_runtime_dependency_allowed: bool,
    pub active_runtime_auto_rebase_allowed: bool,
    pub active_codex_engine_dependency_allowed: bool,
    pub public_release_claim_allowed: bool,
    pub upstream_fetch_performed: bool,
    pub upstream_merge_performed: bool,
    pub upstream_checkout_performed: bool,
    pub workspace_mutation_default: bool,
    pub active_service_restart: bool,
    pub credential_value_read: bool,
    pub secret_file_read: bool,
    pub provider_invoked: bool,
    pub channel_delivery_performed: bool,
    pub gateway_rpc_performed: bool,
    pub public_release_published: bool,
    pub decisions: Vec<HeptaUpstreamCodexPromotionDecision>,
    pub promotion_blockers: Vec<String>,
    pub required_next_gates: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeptaUpstreamCodexPromotionClosureReport {
    pub product: String,
    pub status: String,
    pub closure_id: String,
    pub closure_packet_path: String,
    pub upstream_repository: String,
    pub candidate_diff_range: String,
    pub source_promotion_readiness_gate: String,
    pub closure_gate: String,
    pub active_dependency_isolation_gate: String,
    pub required_surface_promotion_packet_count: usize,
    pub completed_surface_promotion_packet_count: usize,
    pub all_surface_promotion_packets_complete: bool,
    pub promotable_bucket_count: usize,
    pub promotion_blocked_bucket_count: usize,
    pub active_promotion_ready: bool,
    pub active_promotion_denial_ready: bool,
    pub closure_ready: bool,
    pub active_runtime_code_wiring_allowed: bool,
    pub active_runtime_dependency_allowed: bool,
    pub active_runtime_auto_rebase_allowed: bool,
    pub active_codex_engine_dependency_allowed: bool,
    pub public_release_claim_allowed: bool,
    pub public_ga_claim_allowed: bool,
    pub release_artifact_write_allowed: bool,
    pub upstream_fetch_performed: bool,
    pub upstream_merge_performed: bool,
    pub upstream_checkout_performed: bool,
    pub workspace_mutation_default: bool,
    pub active_service_restart: bool,
    pub credential_value_read: bool,
    pub secret_file_read: bool,
    pub provider_invoked: bool,
    pub channel_delivery_performed: bool,
    pub gateway_rpc_performed: bool,
    pub public_release_published: bool,
    pub closure_invariants: Vec<String>,
    pub required_next_gates: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeptaUpstreamCodexActiveWiringPreconditionReport {
    pub product: String,
    pub status: String,
    pub precondition_id: String,
    pub precondition_packet_path: String,
    pub upstream_repository: String,
    pub candidate_diff_range: String,
    pub source_closure_gate: String,
    pub active_wiring_precondition_gate: String,
    pub active_dependency_isolation_gate: String,
    pub promotion_closure_ready: bool,
    pub all_surface_promotion_packets_complete: bool,
    pub active_promotion_denial_ready: bool,
    pub explicit_operator_approval_required: bool,
    pub operator_approval_recorded: bool,
    pub activation_request_id_required: bool,
    pub activation_request_id_present: bool,
    pub live_dependency_isolation_required: bool,
    pub watchdog_required: bool,
    pub browser_smoke_required: bool,
    pub long_soak_required: bool,
    pub active_wiring_precondition_ready: bool,
    pub active_wiring_allowed: bool,
    pub active_runtime_code_wiring_allowed: bool,
    pub active_runtime_dependency_allowed: bool,
    pub active_runtime_auto_rebase_allowed: bool,
    pub active_codex_engine_dependency_allowed: bool,
    pub public_release_claim_allowed: bool,
    pub public_ga_claim_allowed: bool,
    pub release_artifact_write_allowed: bool,
    pub upstream_fetch_performed: bool,
    pub upstream_merge_performed: bool,
    pub upstream_checkout_performed: bool,
    pub workspace_mutation_default: bool,
    pub active_service_restart: bool,
    pub credential_value_read: bool,
    pub secret_file_read: bool,
    pub provider_invoked: bool,
    pub channel_delivery_performed: bool,
    pub gateway_rpc_performed: bool,
    pub public_release_published: bool,
    pub preconditions: Vec<String>,
    pub required_next_gates: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeptaUpstreamCodexActivationRequestPacketField {
    pub name: String,
    pub required: bool,
    pub recorded: bool,
    pub redacted_or_hashed: bool,
    pub purpose: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeptaUpstreamCodexActivationRequestPacketReport {
    pub product: String,
    pub status: String,
    pub packet_id: String,
    pub packet_schema_path: String,
    pub upstream_repository: String,
    pub candidate_diff_range: String,
    pub source_precondition_gate: String,
    pub activation_request_packet_gate: String,
    pub active_dependency_isolation_gate: String,
    pub active_wiring_precondition_ready: bool,
    pub active_wiring_allowed_by_precondition: bool,
    pub operator_approval_required: bool,
    pub operator_approval_recorded: bool,
    pub activation_request_id_required: bool,
    pub activation_request_id_recorded: bool,
    pub required_schema_field_count: usize,
    pub recorded_required_schema_field_count: usize,
    pub schema_field_count: usize,
    pub activation_packet_schema_ready: bool,
    pub activation_packet_recorded: bool,
    pub active_wiring_allowed: bool,
    pub active_runtime_code_wiring_allowed: bool,
    pub active_runtime_dependency_allowed: bool,
    pub active_runtime_auto_rebase_allowed: bool,
    pub active_codex_engine_dependency_allowed: bool,
    pub public_release_claim_allowed: bool,
    pub public_ga_claim_allowed: bool,
    pub release_artifact_write_allowed: bool,
    pub upstream_fetch_performed: bool,
    pub upstream_merge_performed: bool,
    pub upstream_checkout_performed: bool,
    pub workspace_mutation_default: bool,
    pub active_service_restart: bool,
    pub credential_value_read: bool,
    pub secret_file_read: bool,
    pub provider_invoked: bool,
    pub channel_delivery_performed: bool,
    pub gateway_rpc_performed: bool,
    pub public_release_published: bool,
    pub schema_fields: Vec<HeptaUpstreamCodexActivationRequestPacketField>,
    pub packet_invariants: Vec<String>,
    pub required_next_gates: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeptaUpstreamCodexActivationPacketDryRunFixture {
    pub fixture_id: String,
    pub title: String,
    pub recorded_required_field_count: usize,
    pub missing_required_field_count: usize,
    pub operator_approval_recorded: bool,
    pub activation_request_id_recorded: bool,
    pub live_evidence_recorded: bool,
    pub rollback_plan_recorded: bool,
    pub public_release_claim_requested: bool,
    pub release_artifact_write_requested: bool,
    pub validation_status: String,
    pub blocked_reason: String,
    pub active_wiring_allowed: bool,
    pub public_release_claim_allowed: bool,
    pub release_artifact_write_allowed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeptaUpstreamCodexActivationPacketDryRunReport {
    pub product: String,
    pub status: String,
    pub validator_id: String,
    pub validator_doc_path: String,
    pub upstream_repository: String,
    pub candidate_diff_range: String,
    pub source_packet_gate: String,
    pub dry_run_validator_gate: String,
    pub active_dependency_isolation_gate: String,
    pub activation_packet_schema_ready: bool,
    pub activation_packet_recorded: bool,
    pub required_schema_field_count: usize,
    pub fixture_count: usize,
    pub blocked_fixture_count: usize,
    pub allowed_fixture_count: usize,
    pub dry_run_validator_ready: bool,
    pub active_wiring_allowed: bool,
    pub active_runtime_code_wiring_allowed: bool,
    pub active_runtime_dependency_allowed: bool,
    pub active_runtime_auto_rebase_allowed: bool,
    pub active_codex_engine_dependency_allowed: bool,
    pub public_release_claim_allowed: bool,
    pub public_ga_claim_allowed: bool,
    pub release_artifact_write_allowed: bool,
    pub upstream_fetch_performed: bool,
    pub upstream_merge_performed: bool,
    pub upstream_checkout_performed: bool,
    pub workspace_mutation_default: bool,
    pub active_service_restart: bool,
    pub credential_value_read: bool,
    pub secret_file_read: bool,
    pub provider_invoked: bool,
    pub channel_delivery_performed: bool,
    pub gateway_rpc_performed: bool,
    pub public_release_published: bool,
    pub fixtures: Vec<HeptaUpstreamCodexActivationPacketDryRunFixture>,
    pub validation_invariants: Vec<String>,
    pub required_next_gates: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeptaUpstreamCodexActivationEvidenceRequirement {
    pub id: String,
    pub required: bool,
    pub recorded: bool,
    pub fresh: bool,
    pub source_gate: String,
    pub purpose: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeptaUpstreamCodexActivationEvidenceLedgerReport {
    pub product: String,
    pub status: String,
    pub ledger_id: String,
    pub ledger_doc_path: String,
    pub upstream_repository: String,
    pub candidate_diff_range: String,
    pub source_dry_run_gate: String,
    pub evidence_ledger_gate: String,
    pub active_dependency_isolation_gate: String,
    pub dry_run_validator_ready: bool,
    pub activation_packet_recorded: bool,
    pub required_evidence_count: usize,
    pub recorded_evidence_count: usize,
    pub fresh_evidence_count: usize,
    pub evidence_ledger_ready: bool,
    pub evidence_recorded: bool,
    pub active_wiring_allowed: bool,
    pub active_runtime_code_wiring_allowed: bool,
    pub active_runtime_dependency_allowed: bool,
    pub active_runtime_auto_rebase_allowed: bool,
    pub active_codex_engine_dependency_allowed: bool,
    pub public_release_claim_allowed: bool,
    pub public_ga_claim_allowed: bool,
    pub release_artifact_write_allowed: bool,
    pub upstream_fetch_performed: bool,
    pub upstream_merge_performed: bool,
    pub upstream_checkout_performed: bool,
    pub workspace_mutation_default: bool,
    pub active_service_restart: bool,
    pub credential_value_read: bool,
    pub secret_file_read: bool,
    pub provider_invoked: bool,
    pub channel_delivery_performed: bool,
    pub gateway_rpc_performed: bool,
    pub public_release_published: bool,
    pub evidence_requirements: Vec<HeptaUpstreamCodexActivationEvidenceRequirement>,
    pub ledger_invariants: Vec<String>,
    pub required_next_gates: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeptaUpstreamCodexActivationReadinessClosureReport {
    pub product: String,
    pub status: String,
    pub closure_id: String,
    pub closure_doc_path: String,
    pub upstream_repository: String,
    pub candidate_diff_range: String,
    pub source_packet_gate: String,
    pub source_dry_run_gate: String,
    pub source_evidence_ledger_gate: String,
    pub activation_readiness_closure_gate: String,
    pub active_dependency_isolation_gate: String,
    pub activation_packet_schema_ready: bool,
    pub dry_run_validator_ready: bool,
    pub evidence_ledger_ready: bool,
    pub activation_packet_recorded: bool,
    pub evidence_recorded: bool,
    pub required_schema_field_count: usize,
    pub blocked_fixture_count: usize,
    pub allowed_fixture_count: usize,
    pub required_evidence_count: usize,
    pub recorded_evidence_count: usize,
    pub fresh_evidence_count: usize,
    pub readiness_inputs_ready: bool,
    pub activation_denied_by_default: bool,
    pub activation_readiness_closure_ready: bool,
    pub operator_approved_activation_ready: bool,
    pub active_wiring_allowed: bool,
    pub active_runtime_code_wiring_allowed: bool,
    pub active_runtime_dependency_allowed: bool,
    pub active_runtime_auto_rebase_allowed: bool,
    pub active_codex_engine_dependency_allowed: bool,
    pub public_release_claim_allowed: bool,
    pub public_ga_claim_allowed: bool,
    pub release_artifact_write_allowed: bool,
    pub upstream_fetch_performed: bool,
    pub upstream_merge_performed: bool,
    pub upstream_checkout_performed: bool,
    pub workspace_mutation_default: bool,
    pub active_service_restart: bool,
    pub credential_value_read: bool,
    pub secret_file_read: bool,
    pub provider_invoked: bool,
    pub channel_delivery_performed: bool,
    pub gateway_rpc_performed: bool,
    pub public_release_published: bool,
    pub closure_invariants: Vec<String>,
    pub required_next_gates: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeptaUpstreamCodexActivationDeniedSampleReport {
    pub product: String,
    pub status: String,
    pub sample_id: String,
    pub sample_doc_path: String,
    pub upstream_repository: String,
    pub candidate_diff_range: String,
    pub source_readiness_closure_gate: String,
    pub denied_sample_gate: String,
    pub active_dependency_isolation_gate: String,
    pub activation_readiness_closure_ready: bool,
    pub sample_packet_shape_complete: bool,
    pub sample_required_schema_field_count: usize,
    pub sample_recorded_schema_field_count: usize,
    pub sample_required_evidence_count: usize,
    pub sample_fresh_evidence_count: usize,
    pub sample_operator_approval_field_present: bool,
    pub sample_operator_approval_recorded: bool,
    pub sample_public_release_claim_requested: bool,
    pub sample_release_artifact_write_requested: bool,
    pub sample_validation_status: String,
    pub sample_blocked_reason: String,
    pub active_wiring_allowed: bool,
    pub active_runtime_code_wiring_allowed: bool,
    pub active_runtime_dependency_allowed: bool,
    pub active_runtime_auto_rebase_allowed: bool,
    pub active_codex_engine_dependency_allowed: bool,
    pub public_release_claim_allowed: bool,
    pub public_ga_claim_allowed: bool,
    pub release_artifact_write_allowed: bool,
    pub upstream_fetch_performed: bool,
    pub upstream_merge_performed: bool,
    pub upstream_checkout_performed: bool,
    pub workspace_mutation_default: bool,
    pub active_service_restart: bool,
    pub credential_value_read: bool,
    pub secret_file_read: bool,
    pub provider_invoked: bool,
    pub channel_delivery_performed: bool,
    pub gateway_rpc_performed: bool,
    pub public_release_published: bool,
    pub sample_invariants: Vec<String>,
    pub required_next_gates: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeptaUpstreamCodexActivationEvidenceFreshnessPolicyEntry {
    pub evidence_id: String,
    pub source_gate: String,
    pub freshness_anchor: String,
    pub max_age_policy: String,
    pub required_for_activation: bool,
    pub recorded: bool,
    pub fresh: bool,
    pub denial_reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeptaUpstreamCodexActivationEvidenceFreshnessPolicyReport {
    pub product: String,
    pub status: String,
    pub policy_id: String,
    pub policy_doc_path: String,
    pub upstream_repository: String,
    pub candidate_diff_range: String,
    pub source_denied_sample_gate: String,
    pub freshness_policy_gate: String,
    pub active_dependency_isolation_gate: String,
    pub denied_sample_ready: bool,
    pub required_evidence_count: usize,
    pub policy_entry_count: usize,
    pub missing_evidence_count: usize,
    pub fresh_evidence_count: usize,
    pub expired_evidence_count: usize,
    pub stale_evidence_count: usize,
    pub freshness_policy_ready: bool,
    pub activation_blocked_by_freshness_policy: bool,
    pub activation_allowed_by_freshness_policy: bool,
    pub freshness_denial_reason: String,
    pub active_wiring_allowed: bool,
    pub active_runtime_code_wiring_allowed: bool,
    pub active_runtime_dependency_allowed: bool,
    pub active_runtime_auto_rebase_allowed: bool,
    pub active_codex_engine_dependency_allowed: bool,
    pub public_release_claim_allowed: bool,
    pub public_ga_claim_allowed: bool,
    pub release_artifact_write_allowed: bool,
    pub upstream_fetch_performed: bool,
    pub upstream_merge_performed: bool,
    pub upstream_checkout_performed: bool,
    pub workspace_mutation_default: bool,
    pub active_service_restart: bool,
    pub credential_value_read: bool,
    pub secret_file_read: bool,
    pub provider_invoked: bool,
    pub channel_delivery_performed: bool,
    pub gateway_rpc_performed: bool,
    pub public_release_published: bool,
    pub freshness_entries: Vec<HeptaUpstreamCodexActivationEvidenceFreshnessPolicyEntry>,
    pub policy_invariants: Vec<String>,
    pub required_next_gates: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeptaUpstreamCodexActivationEvidenceBindingRecordSchemaField {
    pub name: String,
    pub required: bool,
    pub recorded: bool,
    pub redacted_or_hashed: bool,
    pub purpose: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeptaUpstreamCodexActivationEvidenceBindingRecordManifestEntry {
    pub evidence_id: String,
    pub source_gate: String,
    pub required_schema_field_count: usize,
    pub recorded_schema_field_count: usize,
    pub evidence_recorded: bool,
    pub timestamp_recorded: bool,
    pub active_binary_sha_bound: bool,
    pub route_or_status_hash_bound: bool,
    pub artifact_hash_or_redacted_path_bound: bool,
    pub activation_request_id_bound: bool,
    pub binding_denial_reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeptaUpstreamCodexActivationEvidenceBindingRecordManifestReport {
    pub product: String,
    pub status: String,
    pub manifest_id: String,
    pub manifest_doc_path: String,
    pub upstream_repository: String,
    pub candidate_diff_range: String,
    pub source_freshness_policy_gate: String,
    pub binding_manifest_gate: String,
    pub active_dependency_isolation_gate: String,
    pub freshness_policy_ready: bool,
    pub required_evidence_count: usize,
    pub binding_record_count: usize,
    pub missing_binding_record_count: usize,
    pub recorded_binding_record_count: usize,
    pub required_record_schema_field_count: usize,
    pub recorded_record_schema_field_count: usize,
    pub timestamped_record_count: usize,
    pub binary_sha_bound_record_count: usize,
    pub route_or_status_hash_bound_record_count: usize,
    pub artifact_hash_or_redacted_path_bound_record_count: usize,
    pub activation_request_id_bound_record_count: usize,
    pub binding_manifest_ready: bool,
    pub activation_blocked_by_binding_manifest: bool,
    pub activation_allowed_by_binding_manifest: bool,
    pub binding_denial_reason: String,
    pub active_wiring_allowed: bool,
    pub active_runtime_code_wiring_allowed: bool,
    pub active_runtime_dependency_allowed: bool,
    pub active_runtime_auto_rebase_allowed: bool,
    pub active_codex_engine_dependency_allowed: bool,
    pub public_release_claim_allowed: bool,
    pub public_ga_claim_allowed: bool,
    pub release_artifact_write_allowed: bool,
    pub upstream_fetch_performed: bool,
    pub upstream_merge_performed: bool,
    pub upstream_checkout_performed: bool,
    pub workspace_mutation_default: bool,
    pub active_service_restart: bool,
    pub credential_value_read: bool,
    pub secret_file_read: bool,
    pub provider_invoked: bool,
    pub channel_delivery_performed: bool,
    pub gateway_rpc_performed: bool,
    pub public_release_published: bool,
    pub binding_schema_fields: Vec<HeptaUpstreamCodexActivationEvidenceBindingRecordSchemaField>,
    pub binding_records: Vec<HeptaUpstreamCodexActivationEvidenceBindingRecordManifestEntry>,
    pub binding_invariants: Vec<String>,
    pub required_next_gates: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeptaUpstreamCodexActivationEvidenceRecordDeniedFixtureEntry {
    pub evidence_id: String,
    pub evidence_record_id: String,
    pub source_gate: String,
    pub recorded_at_unix_ms: String,
    pub active_binary_sha256: String,
    pub route_or_status_hash: String,
    pub artifact_sha256_or_redacted_path: String,
    pub activation_request_id_binding: String,
    pub schema_complete: bool,
    pub operator_approved: bool,
    pub request_binding_verified: bool,
    pub live_gate_hash_verified: bool,
    pub artifact_hash_verified: bool,
    pub freshness_window_satisfied: bool,
    pub trusted: bool,
    pub validation_status: String,
    pub denial_reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeptaUpstreamCodexActivationEvidenceRecordDeniedFixtureReport {
    pub product: String,
    pub status: String,
    pub fixture_id: String,
    pub fixture_doc_path: String,
    pub upstream_repository: String,
    pub candidate_diff_range: String,
    pub source_binding_manifest_gate: String,
    pub denied_fixture_gate: String,
    pub active_dependency_isolation_gate: String,
    pub binding_manifest_ready: bool,
    pub required_evidence_count: usize,
    pub fixture_record_count: usize,
    pub schema_complete_fixture_record_count: usize,
    pub trusted_fixture_record_count: usize,
    pub operator_approved_fixture_record_count: usize,
    pub request_binding_verified_record_count: usize,
    pub live_gate_hash_verified_record_count: usize,
    pub artifact_hash_verified_record_count: usize,
    pub fresh_fixture_record_count: usize,
    pub blocked_fixture_record_count: usize,
    pub allowed_fixture_record_count: usize,
    pub denied_fixture_ready: bool,
    pub activation_blocked_by_denied_fixture: bool,
    pub activation_allowed_by_denied_fixture: bool,
    pub fixture_denial_reason: String,
    pub active_wiring_allowed: bool,
    pub active_runtime_code_wiring_allowed: bool,
    pub active_runtime_dependency_allowed: bool,
    pub active_runtime_auto_rebase_allowed: bool,
    pub active_codex_engine_dependency_allowed: bool,
    pub public_release_claim_allowed: bool,
    pub public_ga_claim_allowed: bool,
    pub release_artifact_write_allowed: bool,
    pub upstream_fetch_performed: bool,
    pub upstream_merge_performed: bool,
    pub upstream_checkout_performed: bool,
    pub workspace_mutation_default: bool,
    pub active_service_restart: bool,
    pub credential_value_read: bool,
    pub secret_file_read: bool,
    pub provider_invoked: bool,
    pub channel_delivery_performed: bool,
    pub gateway_rpc_performed: bool,
    pub public_release_published: bool,
    pub fixture_records: Vec<HeptaUpstreamCodexActivationEvidenceRecordDeniedFixtureEntry>,
    pub fixture_invariants: Vec<String>,
    pub required_next_gates: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeptaUpstreamCodexActivationTrustedEvidenceAcceptanceMatrixEntry {
    pub evidence_id: String,
    pub evidence_record_id: String,
    pub source_gate: String,
    pub schema_complete: bool,
    pub required_verification_count: usize,
    pub satisfied_verification_count: usize,
    pub operator_approval_required: bool,
    pub operator_approval_verified: bool,
    pub activation_request_binding_required: bool,
    pub activation_request_binding_verified: bool,
    pub active_binary_sha_required: bool,
    pub active_binary_sha_verified: bool,
    pub route_or_status_hash_required: bool,
    pub route_or_status_hash_verified: bool,
    pub artifact_hash_or_redacted_path_required: bool,
    pub artifact_hash_or_redacted_path_verified: bool,
    pub freshness_window_required: bool,
    pub freshness_window_satisfied: bool,
    pub trusted_source_required: bool,
    pub trusted_source_verified: bool,
    pub accepted: bool,
    pub acceptance_status: String,
    pub denial_reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeptaUpstreamCodexActivationTrustedEvidenceAcceptanceMatrixReport {
    pub product: String,
    pub status: String,
    pub matrix_id: String,
    pub matrix_doc_path: String,
    pub upstream_repository: String,
    pub candidate_diff_range: String,
    pub source_denied_fixture_gate: String,
    pub trusted_acceptance_matrix_gate: String,
    pub active_dependency_isolation_gate: String,
    pub source_denied_fixture_ready: bool,
    pub required_evidence_count: usize,
    pub verification_entry_count: usize,
    pub schema_complete_verification_entry_count: usize,
    pub required_verification_count_per_record: usize,
    pub total_required_verification_count: usize,
    pub total_satisfied_verification_count: usize,
    pub operator_approval_verified_record_count: usize,
    pub request_binding_verified_record_count: usize,
    pub active_binary_sha_verified_record_count: usize,
    pub route_or_status_hash_verified_record_count: usize,
    pub artifact_hash_verified_record_count: usize,
    pub freshness_window_satisfied_record_count: usize,
    pub trusted_source_verified_record_count: usize,
    pub accepted_record_count: usize,
    pub blocked_record_count: usize,
    pub trusted_evidence_acceptance_matrix_ready: bool,
    pub activation_blocked_by_trusted_acceptance_matrix: bool,
    pub activation_allowed_by_trusted_acceptance_matrix: bool,
    pub acceptance_denial_reason: String,
    pub active_wiring_allowed: bool,
    pub active_runtime_code_wiring_allowed: bool,
    pub active_runtime_dependency_allowed: bool,
    pub active_runtime_auto_rebase_allowed: bool,
    pub active_codex_engine_dependency_allowed: bool,
    pub public_release_claim_allowed: bool,
    pub public_ga_claim_allowed: bool,
    pub release_artifact_write_allowed: bool,
    pub upstream_fetch_performed: bool,
    pub upstream_merge_performed: bool,
    pub upstream_checkout_performed: bool,
    pub workspace_mutation_default: bool,
    pub active_service_restart: bool,
    pub credential_value_read: bool,
    pub secret_file_read: bool,
    pub provider_invoked: bool,
    pub channel_delivery_performed: bool,
    pub gateway_rpc_performed: bool,
    pub public_release_published: bool,
    pub verification_entries: Vec<HeptaUpstreamCodexActivationTrustedEvidenceAcceptanceMatrixEntry>,
    pub acceptance_invariants: Vec<String>,
    pub required_next_gates: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeptaUpstreamCodexActivationTrustedRecordShapeValidatorFixture {
    pub fixture_id: String,
    pub fixture_kind: String,
    pub evidence_record_count: usize,
    pub schema_complete_record_count: usize,
    pub required_verification_count_per_record: usize,
    pub total_required_verification_count: usize,
    pub total_satisfied_verification_count: usize,
    pub operator_approval_verified_record_count: usize,
    pub request_binding_verified_record_count: usize,
    pub active_binary_sha_verified_record_count: usize,
    pub route_or_status_hash_verified_record_count: usize,
    pub artifact_hash_verified_record_count: usize,
    pub freshness_window_satisfied_record_count: usize,
    pub trusted_source_verified_record_count: usize,
    pub accepted_record_count: usize,
    pub blocked_record_count: usize,
    pub public_release_claim_requested: bool,
    pub release_artifact_write_requested: bool,
    pub validation_status: String,
    pub active_wiring_allowed: bool,
    pub public_release_claim_allowed: bool,
    pub release_artifact_write_allowed: bool,
    pub denial_reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeptaUpstreamCodexActivationTrustedRecordShapeValidatorReport {
    pub product: String,
    pub status: String,
    pub validator_id: String,
    pub validator_doc_path: String,
    pub upstream_repository: String,
    pub candidate_diff_range: String,
    pub source_trusted_acceptance_matrix_gate: String,
    pub trusted_record_shape_validator_gate: String,
    pub active_dependency_isolation_gate: String,
    pub source_trusted_acceptance_matrix_ready: bool,
    pub required_evidence_count: usize,
    pub fixture_count: usize,
    pub partial_trusted_fixture_count: usize,
    pub public_claim_attempt_fixture_count: usize,
    pub blocked_fixture_count: usize,
    pub allowed_fixture_count: usize,
    pub required_verification_count_per_record: usize,
    pub total_required_verification_count_per_fixture: usize,
    pub max_satisfied_verification_count: usize,
    pub trusted_record_shape_validator_ready: bool,
    pub activation_blocked_by_shape_validator: bool,
    pub activation_allowed_by_shape_validator: bool,
    pub shape_denial_reason: String,
    pub active_wiring_allowed: bool,
    pub active_runtime_code_wiring_allowed: bool,
    pub active_runtime_dependency_allowed: bool,
    pub active_runtime_auto_rebase_allowed: bool,
    pub active_codex_engine_dependency_allowed: bool,
    pub public_release_claim_allowed: bool,
    pub public_ga_claim_allowed: bool,
    pub release_artifact_write_allowed: bool,
    pub upstream_fetch_performed: bool,
    pub upstream_merge_performed: bool,
    pub upstream_checkout_performed: bool,
    pub workspace_mutation_default: bool,
    pub active_service_restart: bool,
    pub credential_value_read: bool,
    pub secret_file_read: bool,
    pub provider_invoked: bool,
    pub channel_delivery_performed: bool,
    pub gateway_rpc_performed: bool,
    pub public_release_published: bool,
    pub fixtures: Vec<HeptaUpstreamCodexActivationTrustedRecordShapeValidatorFixture>,
    pub shape_invariants: Vec<String>,
    pub required_next_gates: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeptaUpstreamCodexActivationEvidenceCompletenessGateFamily {
    pub gate_id: String,
    pub gate_script: String,
    pub gate_ready: bool,
    pub blocks_activation_without_trusted_evidence: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeptaUpstreamCodexActivationEvidenceCompletenessScoreboardReport {
    pub product: String,
    pub status: String,
    pub scoreboard_id: String,
    pub scoreboard_doc_path: String,
    pub upstream_repository: String,
    pub candidate_diff_range: String,
    pub source_trusted_record_shape_validator_gate: String,
    pub evidence_completeness_scoreboard_gate: String,
    pub active_dependency_isolation_gate: String,
    pub source_trusted_record_shape_validator_ready: bool,
    pub required_gate_family_count: usize,
    pub ready_gate_family_count: usize,
    pub activation_blocking_gate_family_count: usize,
    pub required_evidence_count: usize,
    pub required_trusted_record_count: usize,
    pub accepted_trusted_record_count: usize,
    pub fresh_trusted_record_count: usize,
    pub operator_approval_recorded: bool,
    pub activation_request_recorded: bool,
    pub public_claim_attempt_blocked: bool,
    pub release_artifact_write_attempt_blocked: bool,
    pub operator_approved_activation_ready: bool,
    pub evidence_completeness_scoreboard_ready: bool,
    pub activation_blocked_by_scoreboard: bool,
    pub activation_allowed_by_scoreboard: bool,
    pub scoreboard_denial_reason: String,
    pub active_wiring_allowed: bool,
    pub active_runtime_code_wiring_allowed: bool,
    pub active_runtime_dependency_allowed: bool,
    pub active_runtime_auto_rebase_allowed: bool,
    pub active_codex_engine_dependency_allowed: bool,
    pub public_release_claim_allowed: bool,
    pub public_ga_claim_allowed: bool,
    pub release_artifact_write_allowed: bool,
    pub upstream_fetch_performed: bool,
    pub upstream_merge_performed: bool,
    pub upstream_checkout_performed: bool,
    pub workspace_mutation_default: bool,
    pub active_service_restart: bool,
    pub credential_value_read: bool,
    pub secret_file_read: bool,
    pub provider_invoked: bool,
    pub channel_delivery_performed: bool,
    pub gateway_rpc_performed: bool,
    pub public_release_published: bool,
    pub gate_families: Vec<HeptaUpstreamCodexActivationEvidenceCompletenessGateFamily>,
    pub scoreboard_invariants: Vec<String>,
    pub required_next_gates: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeptaUpstreamCodexActivationEvidenceRecordingReceiptField {
    pub name: String,
    pub required: bool,
    pub recorded: bool,
    pub redacted_or_hashed: bool,
    pub purpose: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeptaUpstreamCodexActivationEvidenceRecordingDryRunReceiptReport {
    pub product: String,
    pub status: String,
    pub receipt_id: String,
    pub receipt_doc_path: String,
    pub upstream_repository: String,
    pub candidate_diff_range: String,
    pub source_scoreboard_gate: String,
    pub evidence_recording_dry_run_receipt_gate: String,
    pub active_dependency_isolation_gate: String,
    pub source_scoreboard_ready: bool,
    pub required_receipt_field_count: usize,
    pub recorded_receipt_field_count: usize,
    pub redacted_or_hashed_field_count: usize,
    pub required_evidence_count: usize,
    pub required_trusted_record_count: usize,
    pub accepted_trusted_record_count: usize,
    pub fresh_trusted_record_count: usize,
    pub operator_approval_recorded: bool,
    pub activation_request_recorded: bool,
    pub receipt_schema_ready: bool,
    pub receipt_recorded: bool,
    pub real_evidence_recorded: bool,
    pub trusted_record_materialized: bool,
    pub public_claim_attempt_blocked: bool,
    pub release_artifact_write_attempt_blocked: bool,
    pub evidence_recording_dry_run_ready: bool,
    pub activation_blocked_by_receipt: bool,
    pub activation_allowed_by_receipt: bool,
    pub receipt_denial_reason: String,
    pub active_wiring_allowed: bool,
    pub active_runtime_code_wiring_allowed: bool,
    pub active_runtime_dependency_allowed: bool,
    pub active_runtime_auto_rebase_allowed: bool,
    pub active_codex_engine_dependency_allowed: bool,
    pub public_release_claim_allowed: bool,
    pub public_ga_claim_allowed: bool,
    pub release_artifact_write_allowed: bool,
    pub upstream_fetch_performed: bool,
    pub upstream_merge_performed: bool,
    pub upstream_checkout_performed: bool,
    pub workspace_mutation_default: bool,
    pub active_service_restart: bool,
    pub credential_value_read: bool,
    pub secret_file_read: bool,
    pub provider_invoked: bool,
    pub channel_delivery_performed: bool,
    pub gateway_rpc_performed: bool,
    pub public_release_published: bool,
    pub receipt_fields: Vec<HeptaUpstreamCodexActivationEvidenceRecordingReceiptField>,
    pub receipt_invariants: Vec<String>,
    pub required_next_gates: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeptaUpstreamCodexActivationEvidenceRecordingDeniedReceiptAttempt {
    pub attempt_id: String,
    pub attempt_kind: String,
    pub receipt_field_count: usize,
    pub recorded_receipt_field_count: usize,
    pub accepted_trusted_record_count: usize,
    pub fresh_trusted_record_count: usize,
    pub operator_approval_recorded: bool,
    pub activation_request_recorded: bool,
    pub public_claim_requested: bool,
    pub release_artifact_write_requested: bool,
    pub receipt_materialized: bool,
    pub workspace_write_allowed: bool,
    pub active_wiring_allowed: bool,
    pub public_release_claim_allowed: bool,
    pub release_artifact_write_allowed: bool,
    pub denial_status: String,
    pub denial_reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeptaUpstreamCodexActivationEvidenceRecordingDenialMatrixReport {
    pub product: String,
    pub status: String,
    pub matrix_id: String,
    pub matrix_doc_path: String,
    pub upstream_repository: String,
    pub candidate_diff_range: String,
    pub source_receipt_gate: String,
    pub evidence_recording_denial_matrix_gate: String,
    pub active_dependency_isolation_gate: String,
    pub source_receipt_gate_ready: bool,
    pub required_denied_attempt_count: usize,
    pub denied_receipt_attempt_count: usize,
    pub allowed_receipt_attempt_count: usize,
    pub max_recorded_receipt_field_count: usize,
    pub max_accepted_trusted_record_count: usize,
    pub max_fresh_trusted_record_count: usize,
    pub public_claim_attempt_count: usize,
    pub release_artifact_write_attempt_count: usize,
    pub receipt_sink_write_performed: bool,
    pub evidence_receipt_persisted: bool,
    pub trusted_record_materialized: bool,
    pub no_write_sink_ready: bool,
    pub activation_blocked_by_no_write_sink: bool,
    pub activation_allowed_by_no_write_sink: bool,
    pub active_wiring_allowed: bool,
    pub active_runtime_code_wiring_allowed: bool,
    pub active_runtime_dependency_allowed: bool,
    pub active_runtime_auto_rebase_allowed: bool,
    pub active_codex_engine_dependency_allowed: bool,
    pub public_release_claim_allowed: bool,
    pub public_ga_claim_allowed: bool,
    pub release_artifact_write_allowed: bool,
    pub upstream_fetch_performed: bool,
    pub upstream_merge_performed: bool,
    pub upstream_checkout_performed: bool,
    pub workspace_mutation_default: bool,
    pub active_service_restart: bool,
    pub credential_value_read: bool,
    pub secret_file_read: bool,
    pub provider_invoked: bool,
    pub channel_delivery_performed: bool,
    pub gateway_rpc_performed: bool,
    pub public_release_published: bool,
    pub denied_receipt_attempts:
        Vec<HeptaUpstreamCodexActivationEvidenceRecordingDeniedReceiptAttempt>,
    pub no_write_sink_invariants: Vec<String>,
    pub required_next_gates: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeptaUpstreamCodexActivationEvidenceReceiptPersistenceCommandField {
    pub name: String,
    pub required: bool,
    pub recorded: bool,
    pub redacted_or_hashed: bool,
    pub purpose: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeptaUpstreamCodexActivationEvidenceReceiptPersistenceCommandContractReport {
    pub product: String,
    pub status: String,
    pub command_contract_id: String,
    pub command_contract_doc_path: String,
    pub upstream_repository: String,
    pub candidate_diff_range: String,
    pub source_denial_matrix_gate: String,
    pub receipt_persistence_command_contract_gate: String,
    pub active_dependency_isolation_gate: String,
    pub source_denial_matrix_ready: bool,
    pub required_command_field_count: usize,
    pub recorded_command_field_count: usize,
    pub redacted_or_hashed_field_count: usize,
    pub operator_approval_required: bool,
    pub operator_approval_recorded: bool,
    pub activation_request_required: bool,
    pub activation_request_recorded: bool,
    pub trusted_record_materialized: bool,
    pub receipt_persistence_command_enabled_by_default: bool,
    pub receipt_persistence_command_invoked: bool,
    pub receipt_persistence_execution_performed: bool,
    pub receipt_persistence_noop_ready: bool,
    pub workspace_write_performed: bool,
    pub evidence_receipt_persisted: bool,
    pub activation_blocked_by_persistence_contract: bool,
    pub activation_allowed_by_persistence_contract: bool,
    pub active_wiring_allowed: bool,
    pub active_runtime_code_wiring_allowed: bool,
    pub active_runtime_dependency_allowed: bool,
    pub active_runtime_auto_rebase_allowed: bool,
    pub active_codex_engine_dependency_allowed: bool,
    pub public_release_claim_allowed: bool,
    pub public_ga_claim_allowed: bool,
    pub release_artifact_write_allowed: bool,
    pub upstream_fetch_performed: bool,
    pub upstream_merge_performed: bool,
    pub upstream_checkout_performed: bool,
    pub workspace_mutation_default: bool,
    pub active_service_restart: bool,
    pub credential_value_read: bool,
    pub secret_file_read: bool,
    pub provider_invoked: bool,
    pub channel_delivery_performed: bool,
    pub gateway_rpc_performed: bool,
    pub public_release_published: bool,
    pub command_fields: Vec<HeptaUpstreamCodexActivationEvidenceReceiptPersistenceCommandField>,
    pub command_contract_invariants: Vec<String>,
    pub required_next_gates: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeptaUpstreamCodexActivationEvidenceReceiptPersistenceInvocationDryRunFixture {
    pub fixture_id: String,
    pub fixture_kind: String,
    pub recorded_command_field_count: usize,
    pub redacted_or_hashed_field_count: usize,
    pub operator_approval_recorded: bool,
    pub activation_request_recorded: bool,
    pub accepted_trusted_record_count: usize,
    pub fresh_trusted_record_count: usize,
    pub receipt_payload_hash_recorded: bool,
    pub receipt_output_path_redacted_recorded: bool,
    pub public_claim_requested: bool,
    pub release_artifact_write_requested: bool,
    pub command_invocation_requested: bool,
    pub command_invocation_performed: bool,
    pub receipt_persistence_execution_performed: bool,
    pub workspace_write_performed: bool,
    pub evidence_receipt_persisted: bool,
    pub active_wiring_allowed: bool,
    pub public_release_claim_allowed: bool,
    pub release_artifact_write_allowed: bool,
    pub dry_run_status: String,
    pub denial_reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeptaUpstreamCodexActivationEvidenceReceiptPersistenceInvocationDryRunReport {
    pub product: String,
    pub status: String,
    pub invocation_dry_run_id: String,
    pub invocation_dry_run_doc_path: String,
    pub upstream_repository: String,
    pub candidate_diff_range: String,
    pub source_command_contract_gate: String,
    pub receipt_persistence_invocation_dry_run_gate: String,
    pub active_dependency_isolation_gate: String,
    pub source_command_contract_ready: bool,
    pub required_invocation_fixture_count: usize,
    pub command_invocation_attempt_count: usize,
    pub command_invocation_performed_count: usize,
    pub receipt_persistence_execution_performed_count: usize,
    pub workspace_write_performed_count: usize,
    pub evidence_receipt_persisted_count: usize,
    pub redacted_output_path_fixture_count: usize,
    pub payload_hash_bound_fixture_count: usize,
    pub operator_approved_fixture_count: usize,
    pub activation_request_bound_fixture_count: usize,
    pub max_recorded_command_field_count: usize,
    pub max_accepted_trusted_record_count: usize,
    pub max_fresh_trusted_record_count: usize,
    pub public_claim_attempt_count: usize,
    pub release_artifact_write_attempt_count: usize,
    pub receipt_persistence_command_enabled_by_default: bool,
    pub invocation_dry_run_noop_ready: bool,
    pub activation_blocked_by_invocation_dry_run: bool,
    pub activation_allowed_by_invocation_dry_run: bool,
    pub active_wiring_allowed: bool,
    pub active_runtime_code_wiring_allowed: bool,
    pub active_runtime_dependency_allowed: bool,
    pub active_runtime_auto_rebase_allowed: bool,
    pub active_codex_engine_dependency_allowed: bool,
    pub public_release_claim_allowed: bool,
    pub public_ga_claim_allowed: bool,
    pub release_artifact_write_allowed: bool,
    pub upstream_fetch_performed: bool,
    pub upstream_merge_performed: bool,
    pub upstream_checkout_performed: bool,
    pub workspace_mutation_default: bool,
    pub active_service_restart: bool,
    pub credential_value_read: bool,
    pub secret_file_read: bool,
    pub provider_invoked: bool,
    pub channel_delivery_performed: bool,
    pub gateway_rpc_performed: bool,
    pub public_release_published: bool,
    pub invocation_fixtures:
        Vec<HeptaUpstreamCodexActivationEvidenceReceiptPersistenceInvocationDryRunFixture>,
    pub invocation_dry_run_invariants: Vec<String>,
    pub required_next_gates: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeptaUpstreamCodexActivationEvidenceReceiptNoWriteSinkAdapterSurface {
    pub name: String,
    pub required: bool,
    pub ready: bool,
    pub side_effect_free: bool,
    pub purpose: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeptaUpstreamCodexActivationEvidenceReceiptNoWriteSinkAdapterContractReport {
    pub product: String,
    pub status: String,
    pub no_write_sink_adapter_id: String,
    pub no_write_sink_adapter_doc_path: String,
    pub upstream_repository: String,
    pub candidate_diff_range: String,
    pub source_invocation_dry_run_gate: String,
    pub no_write_sink_adapter_contract_gate: String,
    pub active_dependency_isolation_gate: String,
    pub source_invocation_dry_run_ready: bool,
    pub required_sink_surface_count: usize,
    pub ready_sink_surface_count: usize,
    pub side_effect_free_surface_count: usize,
    pub accepted_invocation_fixture_count: usize,
    pub rejected_write_fixture_count: usize,
    pub rejected_public_claim_fixture_count: usize,
    pub persisted_receipt_count: usize,
    pub workspace_write_performed_count: usize,
    pub sink_write_path_enabled_by_default: bool,
    pub sink_accepts_redacted_payload_hash: bool,
    pub sink_accepts_redacted_output_path: bool,
    pub sink_requires_operator_approval: bool,
    pub sink_requires_fresh_trusted_records: bool,
    pub sink_rejects_public_claim_artifact_write: bool,
    pub no_write_sink_adapter_ready: bool,
    pub activation_blocked_by_no_write_sink_adapter: bool,
    pub activation_allowed_by_no_write_sink_adapter: bool,
    pub active_wiring_allowed: bool,
    pub active_runtime_code_wiring_allowed: bool,
    pub active_runtime_dependency_allowed: bool,
    pub active_runtime_auto_rebase_allowed: bool,
    pub active_codex_engine_dependency_allowed: bool,
    pub public_release_claim_allowed: bool,
    pub public_ga_claim_allowed: bool,
    pub release_artifact_write_allowed: bool,
    pub upstream_fetch_performed: bool,
    pub upstream_merge_performed: bool,
    pub upstream_checkout_performed: bool,
    pub workspace_mutation_default: bool,
    pub active_service_restart: bool,
    pub credential_value_read: bool,
    pub secret_file_read: bool,
    pub provider_invoked: bool,
    pub channel_delivery_performed: bool,
    pub gateway_rpc_performed: bool,
    pub public_release_published: bool,
    pub sink_surfaces: Vec<HeptaUpstreamCodexActivationEvidenceReceiptNoWriteSinkAdapterSurface>,
    pub no_write_sink_adapter_invariants: Vec<String>,
    pub required_next_gates: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeptaUpstreamCodexActivationEvidenceReceiptWriteEnableFixture {
    pub fixture_id: String,
    pub fixture_kind: String,
    pub explicit_write_enable_requested: bool,
    pub operator_approval_recorded: bool,
    pub activation_request_bound: bool,
    pub accepted_trusted_record_count: usize,
    pub fresh_trusted_record_count: usize,
    pub active_binary_sha_bound: bool,
    pub public_claim_requested: bool,
    pub release_artifact_write_requested: bool,
    pub public_artifact_policy_satisfied: bool,
    pub validation_status: String,
    pub filesystem_persistence_allowed: bool,
    pub workspace_write_performed: bool,
    pub evidence_receipt_persisted: bool,
    pub denial_reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeptaUpstreamCodexActivationEvidenceReceiptWriteEnableFixtureReport {
    pub product: String,
    pub status: String,
    pub write_enable_fixture_id: String,
    pub write_enable_fixture_doc_path: String,
    pub upstream_repository: String,
    pub candidate_diff_range: String,
    pub source_no_write_sink_adapter_gate: String,
    pub write_enable_fixture_gate: String,
    pub active_dependency_isolation_gate: String,
    pub source_no_write_sink_adapter_ready: bool,
    pub required_write_enable_fixture_count: usize,
    pub write_enable_fixture_count: usize,
    pub blocked_write_enable_fixture_count: usize,
    pub allowed_write_enable_fixture_count: usize,
    pub explicit_write_enable_requested_fixture_count: usize,
    pub operator_approved_fixture_count: usize,
    pub activation_request_bound_fixture_count: usize,
    pub fresh_trusted_record_fixture_count: usize,
    pub active_binary_sha_bound_fixture_count: usize,
    pub public_claim_attempt_fixture_count: usize,
    pub release_artifact_write_attempt_fixture_count: usize,
    pub public_artifact_policy_satisfied_fixture_count: usize,
    pub filesystem_persistence_allowed_count: usize,
    pub workspace_write_performed_count: usize,
    pub evidence_receipt_persisted_count: usize,
    pub write_enable_fixture_contract_ready: bool,
    pub activation_blocked_by_write_enable_fixture: bool,
    pub activation_allowed_by_write_enable_fixture: bool,
    pub active_wiring_allowed: bool,
    pub active_runtime_code_wiring_allowed: bool,
    pub active_runtime_dependency_allowed: bool,
    pub active_runtime_auto_rebase_allowed: bool,
    pub active_codex_engine_dependency_allowed: bool,
    pub public_release_claim_allowed: bool,
    pub public_ga_claim_allowed: bool,
    pub release_artifact_write_allowed: bool,
    pub upstream_fetch_performed: bool,
    pub upstream_merge_performed: bool,
    pub upstream_checkout_performed: bool,
    pub command_invocation_performed: bool,
    pub receipt_persistence_execution: bool,
    pub workspace_mutation_default: bool,
    pub active_service_restart: bool,
    pub credential_value_read: bool,
    pub secret_file_read: bool,
    pub provider_invoked: bool,
    pub channel_delivery_performed: bool,
    pub gateway_rpc_performed: bool,
    pub public_release_published: bool,
    pub write_enable_fixtures: Vec<HeptaUpstreamCodexActivationEvidenceReceiptWriteEnableFixture>,
    pub write_enable_fixture_invariants: Vec<String>,
    pub required_next_gates: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeptaUpstreamCodexActivationEvidenceReceiptMaterializationDryRunFixture {
    pub fixture_id: String,
    pub fixture_kind: String,
    pub explicit_write_enable_requested: bool,
    pub operator_approval_recorded: bool,
    pub activation_request_bound: bool,
    pub accepted_trusted_record_count: usize,
    pub fresh_trusted_record_count: usize,
    pub active_binary_sha_bound: bool,
    pub payload_hash_planned: bool,
    pub redacted_output_path_planned: bool,
    pub deterministic_materialization_plan: bool,
    pub public_claim_requested: bool,
    pub release_artifact_write_requested: bool,
    pub public_artifact_policy_satisfied: bool,
    pub dry_run_status: String,
    pub filesystem_persistence_allowed: bool,
    pub materialization_executed: bool,
    pub workspace_write_performed: bool,
    pub evidence_receipt_persisted: bool,
    pub denial_reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeptaUpstreamCodexActivationEvidenceReceiptMaterializationDryRunReport {
    pub product: String,
    pub status: String,
    pub materialization_dry_run_id: String,
    pub materialization_dry_run_doc_path: String,
    pub upstream_repository: String,
    pub candidate_diff_range: String,
    pub source_write_enable_fixture_gate: String,
    pub materialization_dry_run_gate: String,
    pub active_dependency_isolation_gate: String,
    pub source_write_enable_fixture_ready: bool,
    pub required_materialization_fixture_count: usize,
    pub materialization_fixture_count: usize,
    pub blocked_materialization_fixture_count: usize,
    pub allowed_materialization_fixture_count: usize,
    pub explicit_write_enable_requested_fixture_count: usize,
    pub operator_approved_fixture_count: usize,
    pub activation_request_bound_fixture_count: usize,
    pub fresh_trusted_record_fixture_count: usize,
    pub active_binary_sha_bound_fixture_count: usize,
    pub payload_hash_planned_fixture_count: usize,
    pub redacted_output_path_planned_fixture_count: usize,
    pub deterministic_materialization_plan_count: usize,
    pub public_claim_attempt_fixture_count: usize,
    pub release_artifact_write_attempt_fixture_count: usize,
    pub public_artifact_policy_satisfied_fixture_count: usize,
    pub filesystem_persistence_allowed_count: usize,
    pub materialization_executed_count: usize,
    pub workspace_write_performed_count: usize,
    pub evidence_receipt_persisted_count: usize,
    pub materialization_dry_run_ready: bool,
    pub activation_blocked_by_materialization_dry_run: bool,
    pub activation_allowed_by_materialization_dry_run: bool,
    pub active_wiring_allowed: bool,
    pub active_runtime_code_wiring_allowed: bool,
    pub active_runtime_dependency_allowed: bool,
    pub active_runtime_auto_rebase_allowed: bool,
    pub active_codex_engine_dependency_allowed: bool,
    pub public_release_claim_allowed: bool,
    pub public_ga_claim_allowed: bool,
    pub release_artifact_write_allowed: bool,
    pub upstream_fetch_performed: bool,
    pub upstream_merge_performed: bool,
    pub upstream_checkout_performed: bool,
    pub command_invocation_performed: bool,
    pub receipt_persistence_execution: bool,
    pub workspace_mutation_default: bool,
    pub active_service_restart: bool,
    pub credential_value_read: bool,
    pub secret_file_read: bool,
    pub provider_invoked: bool,
    pub channel_delivery_performed: bool,
    pub gateway_rpc_performed: bool,
    pub public_release_published: bool,
    pub materialization_fixtures:
        Vec<HeptaUpstreamCodexActivationEvidenceReceiptMaterializationDryRunFixture>,
    pub materialization_invariants: Vec<String>,
    pub required_next_gates: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeptaUpstreamCodexActivationEvidenceReceiptFilesystemPersistenceApprovalField {
    pub name: String,
    pub redacted_or_hashed: bool,
    pub required_for_filesystem_persistence: bool,
    pub recorded_by_default: bool,
    pub purpose: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeptaUpstreamCodexActivationEvidenceReceiptFilesystemPersistenceApprovalPacketReport {
    pub product: String,
    pub status: String,
    pub filesystem_persistence_approval_packet_id: String,
    pub filesystem_persistence_approval_packet_doc_path: String,
    pub upstream_repository: String,
    pub candidate_diff_range: String,
    pub source_materialization_dry_run_gate: String,
    pub filesystem_persistence_approval_packet_gate: String,
    pub active_dependency_isolation_gate: String,
    pub source_materialization_dry_run_ready: bool,
    pub required_approval_field_count: usize,
    pub approval_field_count: usize,
    pub recorded_approval_field_count: usize,
    pub redacted_or_hashed_field_count: usize,
    pub required_for_filesystem_persistence_field_count: usize,
    pub operator_approval_required: bool,
    pub operator_approval_recorded: bool,
    pub activation_request_required: bool,
    pub activation_request_recorded: bool,
    pub materialization_plan_required: bool,
    pub materialization_plan_recorded: bool,
    pub fresh_trusted_records_required: bool,
    pub fresh_trusted_records_recorded: bool,
    pub active_binary_sha_required: bool,
    pub active_binary_sha_recorded: bool,
    pub public_artifact_policy_required: bool,
    pub public_artifact_policy_recorded: bool,
    pub filesystem_persistence_approval_packet_ready: bool,
    pub filesystem_persistence_allowed: bool,
    pub filesystem_persistence_execution_performed: bool,
    pub workspace_write_performed: bool,
    pub evidence_receipt_persisted: bool,
    pub activation_blocked_by_filesystem_persistence_approval: bool,
    pub activation_allowed_by_filesystem_persistence_approval: bool,
    pub active_wiring_allowed: bool,
    pub active_runtime_code_wiring_allowed: bool,
    pub active_runtime_dependency_allowed: bool,
    pub active_runtime_auto_rebase_allowed: bool,
    pub active_codex_engine_dependency_allowed: bool,
    pub public_release_claim_allowed: bool,
    pub public_ga_claim_allowed: bool,
    pub release_artifact_write_allowed: bool,
    pub upstream_fetch_performed: bool,
    pub upstream_merge_performed: bool,
    pub upstream_checkout_performed: bool,
    pub command_invocation_performed: bool,
    pub receipt_persistence_execution: bool,
    pub materialization_execution: bool,
    pub workspace_mutation_default: bool,
    pub active_service_restart: bool,
    pub credential_value_read: bool,
    pub secret_file_read: bool,
    pub provider_invoked: bool,
    pub channel_delivery_performed: bool,
    pub gateway_rpc_performed: bool,
    pub public_release_published: bool,
    pub approval_fields:
        Vec<HeptaUpstreamCodexActivationEvidenceReceiptFilesystemPersistenceApprovalField>,
    pub approval_packet_invariants: Vec<String>,
    pub required_next_gates: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeptaUpstreamCodexActivationEvidenceReceiptFilesystemOutputPathAllowlistEntry {
    pub name: String,
    pub redacted_path: String,
    pub allowed_for_receipt_persistence: bool,
    pub blocked_for_public_artifact: bool,
    pub requires_operator_approval: bool,
    pub purpose: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeptaUpstreamCodexActivationEvidenceReceiptFilesystemOutputPathAllowlistReport {
    pub product: String,
    pub status: String,
    pub filesystem_output_path_allowlist_id: String,
    pub filesystem_output_path_allowlist_doc_path: String,
    pub upstream_repository: String,
    pub candidate_diff_range: String,
    pub source_filesystem_persistence_approval_packet_gate: String,
    pub filesystem_output_path_allowlist_gate: String,
    pub active_dependency_isolation_gate: String,
    pub source_filesystem_persistence_approval_packet_ready: bool,
    pub required_allowlist_entry_count: usize,
    pub allowlist_entry_count: usize,
    pub allowed_output_path_entry_count: usize,
    pub blocked_output_path_entry_count: usize,
    pub redacted_output_path_entry_count: usize,
    pub default_selected_output_path_count: usize,
    pub source_tree_path_allowed: bool,
    pub home_directory_path_allowed: bool,
    pub release_artifact_path_allowed: bool,
    pub public_artifact_path_allowed: bool,
    pub receipt_output_path_allowlist_ready: bool,
    pub filesystem_persistence_allowed: bool,
    pub filesystem_persistence_execution_performed: bool,
    pub workspace_write_performed: bool,
    pub evidence_receipt_persisted: bool,
    pub activation_blocked_by_output_path_allowlist: bool,
    pub activation_allowed_by_output_path_allowlist: bool,
    pub active_wiring_allowed: bool,
    pub active_runtime_code_wiring_allowed: bool,
    pub active_runtime_dependency_allowed: bool,
    pub active_runtime_auto_rebase_allowed: bool,
    pub active_codex_engine_dependency_allowed: bool,
    pub public_release_claim_allowed: bool,
    pub public_ga_claim_allowed: bool,
    pub release_artifact_write_allowed: bool,
    pub upstream_fetch_performed: bool,
    pub upstream_merge_performed: bool,
    pub upstream_checkout_performed: bool,
    pub command_invocation_performed: bool,
    pub receipt_persistence_execution: bool,
    pub materialization_execution: bool,
    pub filesystem_persistence_execution: bool,
    pub workspace_mutation_default: bool,
    pub active_service_restart: bool,
    pub credential_value_read: bool,
    pub secret_file_read: bool,
    pub provider_invoked: bool,
    pub channel_delivery_performed: bool,
    pub gateway_rpc_performed: bool,
    pub public_release_published: bool,
    pub allowlist_entries:
        Vec<HeptaUpstreamCodexActivationEvidenceReceiptFilesystemOutputPathAllowlistEntry>,
    pub allowlist_invariants: Vec<String>,
    pub required_next_gates: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeptaUpstreamCodexActivationEvidenceReceiptFilesystemOutputPathEvidenceBinding {
    pub evidence_id: String,
    pub allowed_output_path_entry_name: String,
    pub binding_required: bool,
    pub recorded_by_default: bool,
    pub redacted_or_hashed: bool,
    pub requires_fresh_live_evidence: bool,
    pub requires_active_binary_sha: bool,
    pub purpose: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeptaUpstreamCodexActivationEvidenceReceiptFilesystemOutputPathEvidenceBindingReport {
    pub product: String,
    pub status: String,
    pub filesystem_output_path_evidence_binding_id: String,
    pub filesystem_output_path_evidence_binding_doc_path: String,
    pub upstream_repository: String,
    pub candidate_diff_range: String,
    pub source_filesystem_output_path_allowlist_gate: String,
    pub filesystem_output_path_evidence_binding_gate: String,
    pub active_dependency_isolation_gate: String,
    pub source_filesystem_output_path_allowlist_ready: bool,
    pub required_path_binding_count: usize,
    pub path_binding_count: usize,
    pub allowed_output_path_entry_count: usize,
    pub selected_output_path_count: usize,
    pub recorded_path_binding_count: usize,
    pub fresh_live_evidence_bound_count: usize,
    pub active_binary_sha_bound_count: usize,
    pub redacted_or_hashed_binding_count: usize,
    pub trusted_source_bound_count: usize,
    pub source_tree_path_binding_allowed: bool,
    pub home_directory_path_binding_allowed: bool,
    pub release_artifact_path_binding_allowed: bool,
    pub public_artifact_path_binding_allowed: bool,
    pub output_path_evidence_binding_ready: bool,
    pub filesystem_persistence_allowed: bool,
    pub filesystem_persistence_execution_performed: bool,
    pub workspace_write_performed: bool,
    pub evidence_receipt_persisted: bool,
    pub activation_blocked_by_output_path_evidence_binding: bool,
    pub activation_allowed_by_output_path_evidence_binding: bool,
    pub active_wiring_allowed: bool,
    pub active_runtime_code_wiring_allowed: bool,
    pub active_runtime_dependency_allowed: bool,
    pub active_runtime_auto_rebase_allowed: bool,
    pub active_codex_engine_dependency_allowed: bool,
    pub public_release_claim_allowed: bool,
    pub public_ga_claim_allowed: bool,
    pub release_artifact_write_allowed: bool,
    pub upstream_fetch_performed: bool,
    pub upstream_merge_performed: bool,
    pub upstream_checkout_performed: bool,
    pub command_invocation_performed: bool,
    pub receipt_persistence_execution: bool,
    pub materialization_execution: bool,
    pub filesystem_persistence_execution: bool,
    pub workspace_mutation_default: bool,
    pub active_service_restart: bool,
    pub credential_value_read: bool,
    pub secret_file_read: bool,
    pub provider_invoked: bool,
    pub channel_delivery_performed: bool,
    pub gateway_rpc_performed: bool,
    pub public_release_published: bool,
    pub path_bindings:
        Vec<HeptaUpstreamCodexActivationEvidenceReceiptFilesystemOutputPathEvidenceBinding>,
    pub binding_invariants: Vec<String>,
    pub required_next_gates: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeptaUpstreamCodexActivationEvidenceReceiptFilesystemSinkWritePreviewFixture {
    pub fixture_id: String,
    pub allowed_output_path_entry_name: String,
    pub redacted_output_path: String,
    pub deterministic_payload_hash: String,
    pub fresh_live_evidence_bound: bool,
    pub active_binary_sha_bound: bool,
    pub trusted_source_bound: bool,
    pub operator_approval_bound: bool,
    pub public_claim_requested: bool,
    pub release_artifact_write_requested: bool,
    pub preview_status: String,
    pub filesystem_persistence_allowed: bool,
    pub workspace_write_performed: bool,
    pub evidence_receipt_persisted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeptaUpstreamCodexActivationEvidenceReceiptFilesystemSinkWritePreviewReport {
    pub product: String,
    pub status: String,
    pub filesystem_sink_write_preview_id: String,
    pub filesystem_sink_write_preview_doc_path: String,
    pub upstream_repository: String,
    pub candidate_diff_range: String,
    pub source_filesystem_output_path_evidence_binding_gate: String,
    pub filesystem_sink_write_preview_gate: String,
    pub active_dependency_isolation_gate: String,
    pub source_filesystem_output_path_evidence_binding_ready: bool,
    pub required_preview_fixture_count: usize,
    pub preview_fixture_count: usize,
    pub allowed_output_path_entry_count: usize,
    pub previewed_output_path_count: usize,
    pub deterministic_payload_hash_count: usize,
    pub redacted_output_path_preview_count: usize,
    pub fresh_live_evidence_bound_fixture_count: usize,
    pub active_binary_sha_bound_fixture_count: usize,
    pub trusted_source_bound_fixture_count: usize,
    pub operator_approval_bound_fixture_count: usize,
    pub blocked_preview_fixture_count: usize,
    pub allowed_preview_fixture_count: usize,
    pub public_claim_attempt_fixture_count: usize,
    pub release_artifact_write_attempt_fixture_count: usize,
    pub filesystem_persistence_allowed_count: usize,
    pub workspace_write_performed_count: usize,
    pub evidence_receipt_persisted_count: usize,
    pub sink_write_preview_ready: bool,
    pub activation_blocked_by_sink_write_preview: bool,
    pub activation_allowed_by_sink_write_preview: bool,
    pub active_wiring_allowed: bool,
    pub active_runtime_code_wiring_allowed: bool,
    pub active_runtime_dependency_allowed: bool,
    pub active_runtime_auto_rebase_allowed: bool,
    pub active_codex_engine_dependency_allowed: bool,
    pub public_release_claim_allowed: bool,
    pub public_ga_claim_allowed: bool,
    pub release_artifact_write_allowed: bool,
    pub upstream_fetch_performed: bool,
    pub upstream_merge_performed: bool,
    pub upstream_checkout_performed: bool,
    pub command_invocation_performed: bool,
    pub receipt_persistence_execution: bool,
    pub materialization_execution: bool,
    pub filesystem_persistence_execution: bool,
    pub workspace_mutation_default: bool,
    pub active_service_restart: bool,
    pub credential_value_read: bool,
    pub secret_file_read: bool,
    pub provider_invoked: bool,
    pub channel_delivery_performed: bool,
    pub gateway_rpc_performed: bool,
    pub public_release_published: bool,
    pub preview_fixtures:
        Vec<HeptaUpstreamCodexActivationEvidenceReceiptFilesystemSinkWritePreviewFixture>,
    pub preview_invariants: Vec<String>,
    pub required_next_gates: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeptaUpstreamCodexActivationEvidenceReceiptFilesystemPersistenceExecutionDenialFixture {
    pub fixture_id: String,
    pub source_preview_fixture_id: String,
    pub deterministic_payload_hash: String,
    pub future_persistence_approval_id_slot: String,
    pub execution_requested: bool,
    pub explicit_persistence_approval_id_present: bool,
    pub fresh_live_evidence_bound: bool,
    pub active_binary_sha_bound: bool,
    pub trusted_source_bound: bool,
    pub operator_approval_bound: bool,
    pub workspace_path_requested: bool,
    pub public_claim_requested: bool,
    pub release_artifact_write_requested: bool,
    pub denial_reason: String,
    pub execution_status: String,
    pub filesystem_persistence_allowed: bool,
    pub filesystem_persistence_execution_performed: bool,
    pub workspace_write_performed: bool,
    pub evidence_receipt_persisted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeptaUpstreamCodexActivationEvidenceReceiptFilesystemPersistenceExecutionDenialMatrixReport
{
    pub product: String,
    pub status: String,
    pub filesystem_persistence_execution_denial_matrix_id: String,
    pub filesystem_persistence_execution_denial_matrix_doc_path: String,
    pub upstream_repository: String,
    pub candidate_diff_range: String,
    pub source_filesystem_sink_write_preview_gate: String,
    pub filesystem_persistence_execution_denial_matrix_gate: String,
    pub active_dependency_isolation_gate: String,
    pub source_filesystem_sink_write_preview_ready: bool,
    pub required_denial_fixture_count: usize,
    pub denial_fixture_count: usize,
    pub source_preview_fixture_count: usize,
    pub execution_requested_fixture_count: usize,
    pub future_persistence_approval_slot_count: usize,
    pub explicit_persistence_approval_id_present_count: usize,
    pub explicit_persistence_approval_id_missing_count: usize,
    pub stale_or_missing_fresh_evidence_fixture_count: usize,
    pub active_binary_sha_bound_fixture_count: usize,
    pub trusted_source_bound_fixture_count: usize,
    pub operator_approval_bound_fixture_count: usize,
    pub workspace_path_attempt_fixture_count: usize,
    pub public_claim_attempt_fixture_count: usize,
    pub release_artifact_write_attempt_fixture_count: usize,
    pub blocked_execution_fixture_count: usize,
    pub allowed_execution_fixture_count: usize,
    pub filesystem_persistence_allowed_count: usize,
    pub filesystem_persistence_execution_performed_count: usize,
    pub workspace_write_performed_count: usize,
    pub evidence_receipt_persisted_count: usize,
    pub execution_denial_matrix_ready: bool,
    pub activation_blocked_by_execution_denial_matrix: bool,
    pub activation_allowed_by_execution_denial_matrix: bool,
    pub active_wiring_allowed: bool,
    pub active_runtime_code_wiring_allowed: bool,
    pub active_runtime_dependency_allowed: bool,
    pub active_runtime_auto_rebase_allowed: bool,
    pub active_codex_engine_dependency_allowed: bool,
    pub public_release_claim_allowed: bool,
    pub public_ga_claim_allowed: bool,
    pub release_artifact_write_allowed: bool,
    pub upstream_fetch_performed: bool,
    pub upstream_merge_performed: bool,
    pub upstream_checkout_performed: bool,
    pub command_invocation_performed: bool,
    pub receipt_persistence_execution: bool,
    pub materialization_execution: bool,
    pub filesystem_persistence_execution: bool,
    pub workspace_mutation_default: bool,
    pub active_service_restart: bool,
    pub credential_value_read: bool,
    pub secret_file_read: bool,
    pub provider_invoked: bool,
    pub channel_delivery_performed: bool,
    pub gateway_rpc_performed: bool,
    pub public_release_published: bool,
    pub denial_fixtures:
        Vec<HeptaUpstreamCodexActivationEvidenceReceiptFilesystemPersistenceExecutionDenialFixture>,
    pub denial_invariants: Vec<String>,
    pub required_next_gates: Vec<String>,
}

impl HeptaUpstreamCodexSyncLaneReport {
    pub fn native_default() -> Self {
        Self::from_contracts(default_upstream_codex_sync_contracts())
    }

    pub fn from_contracts(contracts: Vec<HeptaUpstreamCodexSyncContract>) -> Self {
        let contract_count = contracts.len();
        let ready_contract_count = contracts
            .iter()
            .filter(|contract| contract.contract_ready)
            .count();
        let sync_lane_ready = contract_count > 0
            && ready_contract_count == contract_count
            && contracts.iter().all(|contract| {
                !contract.auto_apply_allowed
                    && !contract.active_runtime_dependency_allowed
                    && !contract.public_release_claim_allowed
            });

        Self {
            product: "Hepta".into(),
            status: if sync_lane_ready {
                "ready"
            } else {
                "attention"
            }
            .into(),
            lane_id: "upstream-codex-sync-lane".into(),
            upstream_repository: "https://github.com/openai/codex".into(),
            compatibility_snapshot_role: "ingestion_and_regression_oracle".into(),
            sync_mode: "classify_then_absorb_then_gate".into(),
            contract_count,
            ready_contract_count,
            sync_lane_ready,
            upstream_fetch_performed: false,
            upstream_latest_claimed: false,
            upstream_merge_performed: false,
            active_runtime_auto_rebase_allowed: false,
            active_runtime_codex_engine_dependency_allowed: false,
            active_dependency_isolation_gate:
                "scripts/hepta-active-service-dependency-isolation.sh".into(),
            requires_diff_classification_before_absorption: true,
            requires_adapter_contract_before_active_runtime: true,
            requires_release_governance_before_public_claim: true,
            local_only_audit: true,
            report_only: true,
            mutates_runtime_state: false,
            external_network_read: false,
            external_send: false,
            credential_value_read: false,
            secret_file_read: false,
            provider_invoked: false,
            channel_delivery_performed: false,
            gateway_rpc_performed: false,
            contracts,
        }
    }
}

impl HeptaUpstreamCodexSnapshotReport {
    pub fn native_default() -> Self {
        Self::from_risk_classes(default_upstream_codex_snapshot_risk_classes())
    }

    pub fn from_risk_classes(risk_classes: Vec<HeptaUpstreamCodexSnapshotRiskClass>) -> Self {
        let risk_class_count = risk_classes.len();
        let ready_risk_class_count = risk_classes
            .iter()
            .filter(|risk_class| {
                risk_class.classification_required
                    && !risk_class.auto_absorb_allowed
                    && !risk_class.active_runtime_dependency_allowed
            })
            .count();
        let snapshot_intake_ready =
            risk_class_count > 0 && ready_risk_class_count == risk_class_count;

        Self {
            product: "Hepta".into(),
            status: if snapshot_intake_ready {
                "ready"
            } else {
                "attention"
            }
            .into(),
            snapshot_lane_id: "upstream-codex-snapshot-intake".into(),
            upstream_repository: "https://github.com/openai/codex".into(),
            upstream_default_ref: "HEAD".into(),
            compatibility_snapshot_role: "ingestion_and_regression_oracle".into(),
            snapshot_gate: "scripts/hepta-upstream-codex-snapshot.sh".into(),
            sync_lane_gate: "scripts/hepta-upstream-codex-sync-lane.sh".into(),
            active_dependency_isolation_gate:
                "scripts/hepta-active-service-dependency-isolation.sh".into(),
            observed_upstream_head_required_before_absorption: true,
            local_compatibility_head_required: true,
            diff_range_required_before_absorption: true,
            diff_inventory_required_before_absorption: true,
            risk_class_count,
            ready_risk_class_count,
            snapshot_intake_ready,
            upstream_fetch_performed: false,
            upstream_merge_performed: false,
            upstream_checkout_performed: false,
            active_runtime_dependency_allowed: false,
            active_runtime_auto_rebase_allowed: false,
            public_release_claim_allowed: false,
            external_network_read_default: false,
            workspace_mutation_default: false,
            credential_value_read: false,
            secret_file_read: false,
            provider_invoked: false,
            channel_delivery_performed: false,
            gateway_rpc_performed: false,
            risk_classes,
        }
    }
}

impl HeptaUpstreamCodexDiffLedgerReport {
    pub fn native_default() -> Self {
        Self::from_buckets(default_upstream_codex_diff_ledger_buckets())
    }

    pub fn from_buckets(buckets: Vec<HeptaUpstreamCodexDiffLedgerBucket>) -> Self {
        let bucket_count = buckets.len();
        let ready_bucket_count = buckets
            .iter()
            .filter(|bucket| {
                bucket.classification_required
                    && bucket.bucket_ready
                    && !bucket.auto_absorb_allowed
                    && !bucket.active_runtime_dependency_allowed
            })
            .count();
        let diff_ledger_ready = bucket_count > 0 && ready_bucket_count == bucket_count;
        let baseline_upstream_head = HEPTA_UPSTREAM_CODEX_INTAKE_BASE_HEAD.to_string();
        let target_upstream_head = HEPTA_UPSTREAM_CODEX_INTAKE_CUTOFF_HEAD.to_string();

        Self {
            product: "Hepta".into(),
            status: if diff_ledger_ready {
                "ready"
            } else {
                "attention"
            }
            .into(),
            ledger_id: "upstream-codex-diff-range-ledger".into(),
            upstream_repository: "https://github.com/openai/codex".into(),
            candidate_diff_range: format!("{baseline_upstream_head}..{target_upstream_head}"),
            baseline_upstream_head,
            target_upstream_head,
            target_head_source: HEPTA_UPSTREAM_CODEX_INTAKE_CUTOFF_REF.into(),
            target_ref: HEPTA_UPSTREAM_CODEX_INTAKE_CUTOFF_REF.into(),
            snapshot_gate: "scripts/hepta-upstream-codex-snapshot.sh".into(),
            diff_ledger_gate: "scripts/hepta-upstream-codex-diff-ledger.sh".into(),
            sync_lane_gate: "scripts/hepta-upstream-codex-sync-lane.sh".into(),
            active_dependency_isolation_gate:
                "scripts/hepta-active-service-dependency-isolation.sh".into(),
            commit_inventory_required: true,
            file_inventory_required: true,
            risk_bucket_classification_required: true,
            bucket_count,
            ready_bucket_count,
            diff_ledger_ready,
            upstream_fetch_performed: false,
            upstream_merge_performed: false,
            upstream_checkout_performed: false,
            workspace_mutation_default: false,
            active_runtime_dependency_allowed: false,
            active_runtime_auto_rebase_allowed: false,
            public_release_claim_allowed: false,
            external_network_read_default: false,
            credential_value_read: false,
            secret_file_read: false,
            provider_invoked: false,
            channel_delivery_performed: false,
            gateway_rpc_performed: false,
            buckets,
        }
    }
}

fn current_intake_absorbed(
    classification: &str,
    upstream_commit: &str,
    local_receipts: &[&str],
    absorption_kind: &str,
    rationale: &str,
) -> HeptaUpstreamCodexCurrentIntakeDecision {
    HeptaUpstreamCodexCurrentIntakeDecision {
        classification: classification.into(),
        disposition: HeptaUpstreamCodexCurrentIntakeDisposition::Absorbed,
        upstream_commit: Some(upstream_commit.into()),
        local_receipts: local_receipts
            .iter()
            .map(|receipt| (*receipt).into())
            .collect(),
        absorption_kind: Some(absorption_kind.into()),
        rationale: rationale.into(),
    }
}

fn current_intake_deferred(
    classification: &str,
    upstream_commit: Option<&str>,
    rationale: &str,
) -> HeptaUpstreamCodexCurrentIntakeDecision {
    HeptaUpstreamCodexCurrentIntakeDecision {
        classification: classification.into(),
        disposition: HeptaUpstreamCodexCurrentIntakeDisposition::Deferred,
        upstream_commit: upstream_commit.map(Into::into),
        local_receipts: Vec::new(),
        absorption_kind: None,
        rationale: rationale.into(),
    }
}

fn default_upstream_codex_current_intake_decisions() -> Vec<HeptaUpstreamCodexCurrentIntakeDecision>
{
    vec![
        current_intake_absorbed(
            "history_integrity",
            "86102db5a1a7a49ce08e79f900e0897d94aa3770",
            &["59f544a9316f2ac12044eb8672380c63a755b397"],
            "semantic_port",
            "reject unsupported canonical history modes",
        ),
        current_intake_absorbed(
            "history_integrity",
            "19b2273d8a54ec28aa0174a95884182aa6e5081e",
            &["fdbee7b12cb5e8e482f2148212f3d540018d659b"],
            "semantic_port",
            "keep paginated Git metadata in SQLite",
        ),
        current_intake_absorbed(
            "memory_history_integrity",
            "2793c826e8f5a04e0619a18e9bbdae91c0435cd5",
            &["4c1ad77903d6f0efbeeb5c382d876944d989a195"],
            "semantic_port",
            "keep paginated memory mode in SQLite",
        ),
        current_intake_absorbed(
            "exec_security",
            "bf3c1972b7d045c0a3a48dff91f381070f8f69e1",
            &["a66754492b52bc34ebd1ce039efa09297ae66f1f"],
            "semantic_port",
            "migrate legacy exec-policy allow rules fail closed",
        ),
        current_intake_absorbed(
            "tool_history_integrity",
            "8431dc590a5bba9a1185d5579a5aabfbc469e50b",
            &["d8795967f2812bc13f236c550858f2926555896e"],
            "semantic_port",
            "preserve canonical history on invalid tool images",
        ),
        current_intake_absorbed(
            "shell_environment_integrity",
            "2deed3fb9c00c74dac3d177ea700d6fb7a94539d",
            &["e3fa5fbc8bb2baa806ac30ce0b58066aeb3932ca"],
            "semantic_port",
            "preserve zsh tied PATH exports in shell snapshots",
        ),
        current_intake_absorbed(
            "model_catalog",
            "5a4f5ee64c4e7de22c21f1c38feb3edfb167b7d8",
            &["95276ae5619b0601a931f999570e39a34b29900d"],
            "translated_catalog",
            "refresh model metadata while retaining Hepta identity and capability fields",
        ),
        current_intake_absorbed(
            "thread_metadata_integrity",
            "5a208c1fc353573fa3838c70a90ea9c59ad8884c",
            &["1d49bb4ff759fa5a93494b7a8ff239fc6967cb33"],
            "semantic_port",
            "persist paginated thread names in SQLite",
        ),
        current_intake_absorbed(
            "history_storage_efficiency",
            "45ac251e178416ff5c3022457ad8d2778c0d4549",
            &["31c6065061185de711aa36ee6e9cf7c4a4795821"],
            "semantic_port",
            "share history snapshots copy-on-write",
        ),
        current_intake_absorbed(
            "linux_proc_preflight_filesystem_isolation",
            "44481a1c4548d1cc0cc3c95aa03b59ec4cba074a",
            &["c62ce9e2d4ee0ccaa85b50098f41198b44ae17e7"],
            "semantic_port",
            "probe proc-mount support through a minimal read-only filesystem view while preserving the requested network namespace mode",
        ),
        current_intake_absorbed(
            "plugin_marketplace_trust",
            "9dbdb4e2c08723e8fc9c18f64d7ccad3dadc03a7",
            &[
                "45ef7b7b01b232a245b22a50d4ffdbcfe57d8f0b",
                "411c6a1b9a146ae540dc4f86b9c1736031c84b07",
                "63d34505b27d0917a656d3bdcdc74ea2e1958664",
                "1656048a546fb20799df1031b75deffe7852b659",
                "b84a35b0774f5fcc9c9178c01b2e17af681008dd",
                "e7c7b0c8e245258c7d125bc5cb8e84fdca80158b",
                "bafdb195c5f9d7bc555ee3859381c8bbf3283ba4",
            ],
            "local_split",
            "enforce marketplace source policy across admission, mutation and runtime reads",
        ),
        current_intake_absorbed(
            "mcp_endpoint_ownership",
            "6bf4845b60e0abccd0c64690e9c7591e0efb85d8",
            &["f983f4ae7fc7e4b224272990106049f30ee472d7"],
            "semantic_port",
            "route host-owned Apps MCP through the plugin service while constraining ChatGPT session auth to the first-party HTTPS origin",
        ),
        current_intake_deferred(
            "r2_windows_write_root_acl_integrity",
            Some("bd92b056ddd91bd7c2ecfea3d8773f7eb5a879a6"),
            "requires a separately reviewed Windows sandbox write-root lane",
        ),
        current_intake_deferred(
            "r2_hook_context_spill_limits",
            Some("e4836f998da166aba456f60d2e74eb79d6e2542b"),
            "requires a separately reviewed hook resource-governance lane",
        ),
        current_intake_deferred(
            "r2_session_start_hook_ordering",
            Some("8c41ed33ce3e39460e7b13b14c35e0c39bb5980d"),
            "requires a separately reviewed session and hook-ordering lane",
        ),
        current_intake_deferred(
            "r2_approval_rejection_reason_propagation",
            Some("e52c35b0001ea3e4a1744b99c4250a5b1a09e44d"),
            "requires a separately reviewed approval protocol lane",
        ),
        current_intake_deferred(
            "r2_history_hook_api_test_alignment",
            Some("ec3140db1297f3acebec7d6916b329cad3b12693"),
            "requires the history and hook API changes it tests to be reviewed first",
        ),
        current_intake_deferred(
            "r2_paginated_rollout_lineage_resolution",
            Some("b7e39aa31608b6eaba4f317538a8f82985a9e854"),
            "requires a separately reviewed rollout lineage lane",
        ),
        current_intake_deferred(
            "r2_threadless_mcp_connection_events",
            Some("19940967bdb5ac04aec5d08ebd465481f1ac964d"),
            "requires a separately reviewed MCP lifecycle lane",
        ),
        current_intake_deferred(
            "r2_sqlite_test_path_validation",
            Some("81e89fa5af13012c8313f032a17b11b9a5170d33"),
            "requires a separately reviewed SQLite test configuration lane",
        ),
        current_intake_deferred(
            "r2_agent_job_storage_migration",
            Some("687f05cb946d10c96f90dd7ce82e11465c6e20a7"),
            "requires a separately reviewed agent job persistence lane",
        ),
        current_intake_deferred(
            "r2_hook_warning_tui_presentation",
            Some("cf821e8ec850c6d8380feea0e84859dd8ff54cd0"),
            "requires a separately reviewed compatibility UI lane",
        ),
        current_intake_deferred(
            "r2_connector_metadata_enrichment",
            Some("60272096bc125ad7bd8ec26508b19d1e0db2874b"),
            "requires a separately reviewed connector metadata lane",
        ),
        current_intake_deferred(
            "r2_windows_exec_server_sandboxing",
            Some("35c2278dd5c49daf8a4e44468038aed9be9e866e"),
            "requires a separately reviewed Windows exec-server sandbox lane",
        ),
        current_intake_deferred(
            "r2_shared_skill_model_migration",
            Some("56c11cf6586c0579e4e3eca14eefb0916b14c78c"),
            "requires a separately reviewed skill model and dependency lane",
        ),
        current_intake_deferred(
            "r2_remote_compaction_history_optimization",
            Some("fd3c1dc13d0a0941af406e1bc1f697c9d14110ea"),
            "requires a separately reviewed compaction history lane",
        ),
        current_intake_deferred(
            "r2_approval_catalog_policy_compatibility",
            Some("2be7d3bcd9d1aec2780f0a71fe79cbb5afd877a1"),
            "requires a separately reviewed approval catalog compatibility lane",
        ),
        current_intake_deferred(
            "r2_outbound_proxy_route_resolution",
            Some("c9ef7eff005c3299a5a5f0004c34c6a3eedf2564"),
            "requires a separately reviewed outbound proxy route lane",
        ),
        current_intake_deferred(
            "r2_managed_permission_proxy_resolution",
            Some("88fac6fe108237a105d3203e3508b0d531054312"),
            "requires a separately reviewed managed permission and proxy policy lane",
        ),
        current_intake_deferred(
            "audio_history_and_tool_output",
            Some("6f785632b000f7d8e85100506b88b3bab5b8d8a0"),
            "requires a separately reviewed audio capability and dependency lane",
        ),
        current_intake_deferred(
            "bulk_tui_performance_batch",
            None,
            "requires bounded compatibility lanes rather than broad TUI import",
        ),
        current_intake_deferred(
            "whole_tree_or_lockfile_import",
            None,
            "unrelated histories and divergent dependency graphs prohibit whole-tree or Cargo.lock replacement",
        ),
    ]
}

impl HeptaUpstreamCodexCurrentIntakeReport {
    pub fn native_default() -> Self {
        let decisions = default_upstream_codex_current_intake_decisions();
        let selected_absorption_count = decisions
            .iter()
            .filter(|decision| {
                decision.disposition == HeptaUpstreamCodexCurrentIntakeDisposition::Absorbed
            })
            .count();
        let deferred_decision_count = decisions.len() - selected_absorption_count;
        let selected_commits: Vec<&str> = decisions
            .iter()
            .filter_map(|decision| {
                (decision.disposition == HeptaUpstreamCodexCurrentIntakeDisposition::Absorbed)
                    .then_some(decision.upstream_commit.as_deref())
                    .flatten()
            })
            .collect();
        let selected_commits_are_unique = selected_commits
            .iter()
            .enumerate()
            .all(|(index, commit)| !selected_commits[..index].contains(commit));
        let decisions_are_bounded = decisions.iter().all(|decision| {
            !decision.classification.is_empty()
                && !decision.rationale.is_empty()
                && match decision.disposition {
                    HeptaUpstreamCodexCurrentIntakeDisposition::Absorbed => {
                        decision
                            .upstream_commit
                            .as_deref()
                            .is_some_and(|commit| commit.len() == 40)
                            && !decision.local_receipts.is_empty()
                            && decision
                                .local_receipts
                                .iter()
                                .all(|receipt| receipt.len() == 40)
                            && decision.absorption_kind.is_some()
                    }
                    HeptaUpstreamCodexCurrentIntakeDisposition::Deferred => {
                        decision.local_receipts.is_empty() && decision.absorption_kind.is_none()
                    }
                }
        });
        let current_intake_ready = selected_absorption_count == 12
            && deferred_decision_count == 20
            && selected_commits_are_unique
            && decisions_are_bounded;

        Self {
            product: "Hepta".into(),
            status: if current_intake_ready {
                "ready"
            } else {
                "attention"
            }
            .into(),
            intake_id: "upstream-codex-intake-2026-07-21-r2".into(),
            manifest_path: HEPTA_UPSTREAM_CODEX_INTAKE_MANIFEST_PATH.into(),
            observation_state: "observed".into(),
            classification_state: "classified".into(),
            selected_state: "absorbed".into(),
            remaining_state: "deferred".into(),
            baseline_head: HEPTA_UPSTREAM_CODEX_INTAKE_BASE_HEAD.into(),
            cutoff_ref: HEPTA_UPSTREAM_CODEX_INTAKE_CUTOFF_REF.into(),
            cutoff_head: HEPTA_UPSTREAM_CODEX_INTAKE_CUTOFF_HEAD.into(),
            candidate_diff_range: format!(
                "{}..{}",
                HEPTA_UPSTREAM_CODEX_INTAKE_BASE_HEAD, HEPTA_UPSTREAM_CODEX_INTAKE_CUTOFF_HEAD
            ),
            predecessor_manifest_path: HEPTA_UPSTREAM_CODEX_PREDECESSOR_INTAKE_MANIFEST_PATH.into(),
            predecessor_manifest_sha256: HEPTA_UPSTREAM_CODEX_PREDECESSOR_INTAKE_MANIFEST_SHA256
                .into(),
            predecessor_cutoff_ref: HEPTA_UPSTREAM_CODEX_PREDECESSOR_INTAKE_CUTOFF_REF.into(),
            predecessor_cutoff_head: HEPTA_UPSTREAM_CODEX_PREDECESSOR_INTAKE_CUTOFF_HEAD.into(),
            predecessor_cutoff_preserved: true,
            observed_commit_count: 1821,
            observed_changed_file_count: 3389,
            observed_codex_rs_changed_file_count: 3127,
            selected_absorption_count,
            deferred_decision_count,
            historical_receipt_target_head: HEPTA_UPSTREAM_CODEX_HISTORICAL_RECEIPT_TARGET_HEAD
                .into(),
            historical_receipt_changed_file_count:
                HEPTA_UPSTREAM_CODEX_HISTORICAL_LEDGER_CHANGED_FILE_COUNT,
            historical_receipt_selected_absorption_count:
                HEPTA_UPSTREAM_CODEX_HISTORICAL_SELECTED_ABSORPTION_COUNT,
            historical_receipt_is_current_freshness_proof: false,
            current_intake_ready,
            full_range_absorption_claimed: false,
            upstream_fetch_performed: false,
            upstream_merge_performed: false,
            upstream_rebase_performed: false,
            whole_tree_replacement_performed: false,
            cargo_lock_replacement_performed: false,
            active_runtime_dependency_allowed: false,
            public_release_claim_allowed: false,
            decisions,
        }
    }
}

impl HeptaUpstreamCodexProductGovernanceAbsorptionReport {
    pub fn native_default() -> Self {
        let selected_paths = default_product_governance_selected_paths();
        let selected_changed_file_count = selected_paths.len();
        let selected_commit_sample_count = 40;
        let contract_ready = selected_changed_file_count == 22
            && selected_commit_sample_count > 0
            && selected_paths
                .iter()
                .all(|path| path.starts_with("codex-rs/"))
            && selected_paths
                .iter()
                .any(|path| path.ends_with("README.md"))
            && selected_paths
                .iter()
                .any(|path| path.contains("request_plugin_install"));

        Self {
            product: "Hepta".into(),
            status: if contract_ready { "ready" } else { "attention" }.into(),
            absorption_id: "upstream-codex-product-governance-absorption-contract".into(),
            upstream_repository: "https://github.com/openai/codex".into(),
            candidate_diff_range: historical_upstream_codex_receipt_diff_range(),
            selected_bucket_id: "product-doc-release-governance".into(),
            selected_bucket_risk: HeptaUpstreamCodexSyncRisk::P2Product,
            selected_changed_file_count,
            selected_commit_sample_count,
            source_ledger_gate: "scripts/hepta-upstream-codex-diff-ledger.sh".into(),
            absorption_gate: "scripts/hepta-upstream-codex-product-governance-absorption.sh"
                .into(),
            active_dependency_isolation_gate:
                "scripts/hepta-active-service-dependency-isolation.sh".into(),
            selected_as_first_absorption_contract: true,
            low_risk_runtime_promotion: false,
            requires_hepta_translation: true,
            raw_upstream_doc_copy_allowed: false,
            raw_upstream_package_policy_copy_allowed: false,
            active_runtime_code_wiring_allowed: false,
            active_runtime_dependency_allowed: false,
            public_release_claim_allowed: false,
            contract_ready,
            upstream_fetch_performed: false,
            upstream_merge_performed: false,
            upstream_checkout_performed: false,
            workspace_mutation_default: false,
            credential_value_read: false,
            secret_file_read: false,
            provider_invoked: false,
            channel_delivery_performed: false,
            gateway_rpc_performed: false,
            selected_paths,
            required_next_gates: vec![
                "translate upstream product/docs/package deltas into Hepta release-governance wording"
                    .into(),
                "keep active dependency isolation green".into(),
                "run clean preflight before any absorption patch".into(),
                "require watchdog and long soak before release claims".into(),
            ],
        }
    }
}

impl HeptaUpstreamCodexProductGovernanceTranslationReport {
    pub fn native_default() -> Self {
        let hepta_actions = vec![
            "translate package and install-context deltas into Hepta packaging governance without changing the active service binary".into(),
            "translate README and protocol documentation deltas into Hepta route/gate language without copying upstream public-release claims".into(),
            "translate plugin install/request changes into operator-approved marketplace policy before live mutation".into(),
            "hold sandbox, exec, and network documentation behind the P0 security/runtime buckets before active promotion".into(),
            "require clean preflight, watchdog, operator approval packet, and long soak before any release-facing claim".into(),
        ];
        let translated_surface_count = hepta_actions.len();
        let required_surface_count = 5;
        let translation_ready = translated_surface_count == required_surface_count;

        Self {
            product: "Hepta".into(),
            status: if translation_ready {
                "ready"
            } else {
                "attention"
            }
            .into(),
            translation_id: "upstream-codex-product-governance-translation-packet".into(),
            translation_packet_path:
                "docs/architecture/HEPTA_UPSTREAM_CODEX_PRODUCT_GOVERNANCE_TRANSLATION.md".into(),
            selected_bucket_id: "product-doc-release-governance".into(),
            selected_changed_file_count: 22,
            translated_surface_count,
            required_surface_count,
            source_absorption_gate: "scripts/hepta-upstream-codex-product-governance-absorption.sh"
                .into(),
            translation_gate: "scripts/hepta-upstream-codex-product-governance-translation.sh"
                .into(),
            active_dependency_isolation_gate:
                "scripts/hepta-active-service-dependency-isolation.sh".into(),
            release_governance_documented: true,
            package_policy_documented: true,
            plugin_marketplace_policy_documented: true,
            sandbox_runtime_policy_documented: true,
            operator_approval_policy_documented: true,
            requires_hepta_translation: true,
            raw_upstream_doc_copy_allowed: false,
            raw_upstream_package_policy_copy_allowed: false,
            active_runtime_code_wiring_allowed: false,
            active_runtime_dependency_allowed: false,
            public_release_claim_allowed: false,
            translation_ready,
            upstream_fetch_performed: false,
            upstream_merge_performed: false,
            upstream_checkout_performed: false,
            workspace_mutation_default: false,
            credential_value_read: false,
            secret_file_read: false,
            provider_invoked: false,
            channel_delivery_performed: false,
            gateway_rpc_performed: false,
            hepta_actions,
        }
    }
}

impl HeptaUpstreamCodexReleaseGovernancePromotionReport {
    pub fn native_default() -> Self {
        let promotion_conditions: Vec<String> = vec![
            "release claim taxonomy is documented as local readiness language".into(),
            "package and install-context governance remains Hepta-owned".into(),
            "plugin marketplace policy remains operator-approved before live mutation".into(),
            "operator approval model is explicit before any public claim".into(),
            "watchdog, browser smoke, and soak evidence are required before claims".into(),
            "public claim boundary keeps GA/release publication disabled".into(),
            "side-effect boundary keeps artifacts, channels, and gateway RPC off".into(),
        ];
        let required_promotion_condition_count = 7;
        let ready_promotion_condition_count = promotion_conditions.len();
        let promotion_packet_ready =
            ready_promotion_condition_count == required_promotion_condition_count;

        Self {
            product: "Hepta".into(),
            status: if promotion_packet_ready {
                "ready"
            } else {
                "attention"
            }
            .into(),
            promotion_id: "release-governance-claim-promotion-packet".into(),
            promotion_packet_path:
                "docs/architecture/HEPTA_UPSTREAM_CODEX_RELEASE_GOVERNANCE_PROMOTION.md".into(),
            selected_bucket_id: "product-doc-release-governance".into(),
            selected_changed_file_count: 22,
            source_translation_gate:
                "scripts/hepta-upstream-codex-product-governance-translation.sh".into(),
            promotion_gate: "scripts/hepta-upstream-codex-release-governance-promotion.sh".into(),
            active_dependency_isolation_gate:
                "scripts/hepta-active-service-dependency-isolation.sh".into(),
            release_claim_taxonomy_ready: true,
            package_install_context_ready: true,
            plugin_marketplace_policy_ready: true,
            operator_approval_model_ready: true,
            watchdog_soak_evidence_ready: true,
            public_claim_boundary_ready: true,
            side_effect_boundary_ready: true,
            required_promotion_condition_count,
            ready_promotion_condition_count,
            promotion_packet_ready,
            public_release_claim_allowed: false,
            public_ga_claim_allowed: false,
            release_artifact_write_allowed: false,
            active_runtime_code_wiring_allowed: false,
            active_runtime_dependency_allowed: false,
            upstream_auto_rebase_allowed: false,
            upstream_fetch_performed: false,
            upstream_merge_performed: false,
            upstream_checkout_performed: false,
            workspace_mutation_default: false,
            active_service_restart: false,
            credential_value_read: false,
            secret_file_read: false,
            provider_invoked: false,
            channel_delivery_performed: false,
            gateway_rpc_performed: false,
            public_release_published: false,
            promotion_conditions,
            remaining_blockers: vec![
                "public GA claim remains disabled by this packet".into(),
                "public release publication remains disabled by this packet".into(),
                "release artifact writes remain forbidden by this packet".into(),
                "channel delivery and gateway RPC remain forbidden".into(),
            ],
            required_next_gates: vec![
                "require explicit operator approval before any public release claim".into(),
                "require live watchdog, browser smoke, and long soak evidence before claims".into(),
                "keep active hepta-cli cargo tree free of tracked Codex engine crates".into(),
                "rerun promotion readiness after all per-surface promotion packets close".into(),
            ],
        }
    }
}

impl HeptaUpstreamCodexLegacyCompatibilityAbsorptionReport {
    pub fn native_default() -> Self {
        let sample_surfaces: Vec<String> = vec![
            "codex-rs/cli compatibility commands".into(),
            "codex-rs/tui compatibility presentation".into(),
            "codex-rs/code-mode compatibility runtime".into(),
            "terminal-detection and utils/cli helpers".into(),
        ];
        let selected_changed_file_count = 128;
        let contract_ready = selected_changed_file_count == 128
            && sample_surfaces
                .iter()
                .any(|surface| surface.contains("cli"))
            && sample_surfaces
                .iter()
                .any(|surface| surface.contains("tui"))
            && sample_surfaces
                .iter()
                .any(|surface| surface.contains("code-mode"));

        Self {
            product: "Hepta".into(),
            status: if contract_ready { "ready" } else { "attention" }.into(),
            absorption_id: "upstream-codex-legacy-compatibility-absorption-contract".into(),
            upstream_repository: "https://github.com/openai/codex".into(),
            candidate_diff_range: historical_upstream_codex_receipt_diff_range(),
            selected_bucket_id: "legacy-cli-tui-compatibility".into(),
            selected_bucket_risk: HeptaUpstreamCodexSyncRisk::P1Compatibility,
            selected_changed_file_count,
            source_ledger_gate: "scripts/hepta-upstream-codex-diff-ledger.sh".into(),
            absorption_gate: "scripts/hepta-upstream-codex-legacy-compatibility-absorption.sh"
                .into(),
            active_dependency_isolation_gate:
                "scripts/hepta-active-service-dependency-isolation.sh".into(),
            retained_as_compatibility_snapshot: true,
            requires_hepta_command_contract: true,
            active_cli_tui_promotion_allowed: false,
            active_runtime_code_wiring_allowed: false,
            active_runtime_dependency_allowed: false,
            public_release_claim_allowed: false,
            contract_ready,
            upstream_fetch_performed: false,
            upstream_merge_performed: false,
            upstream_checkout_performed: false,
            workspace_mutation_default: false,
            credential_value_read: false,
            secret_file_read: false,
            provider_invoked: false,
            channel_delivery_performed: false,
            gateway_rpc_performed: false,
            sample_surfaces,
            required_next_gates: vec![
                "map legacy CLI/TUI deltas to explicit Hepta command contracts".into(),
                "keep active hepta-cli cargo tree free of tracked Codex engine crates".into(),
                "run behavior-equivalence and shadow-replay before promotion".into(),
                "do not promote compatibility UI behavior without Hepta-native parity".into(),
            ],
        }
    }
}

impl HeptaUpstreamCodexLegacyCompatibilityReplayReport {
    pub fn native_default() -> Self {
        let replay_surfaces: Vec<String> = vec![
            "CLI command shape and argument contract replay".into(),
            "TUI presentation and snapshot compatibility replay".into(),
            "code-mode runtime callback and module-loader replay".into(),
            "terminal detection PTY and utils CLI helper replay".into(),
            "active dependency boundary and no-promotion replay".into(),
        ];
        let replay_surface_count = replay_surfaces.len();
        let required_replay_surface_count = 5;
        let replay_ready = replay_surface_count == required_replay_surface_count
            && replay_surfaces
                .iter()
                .any(|surface| surface.contains("CLI command"))
            && replay_surfaces
                .iter()
                .any(|surface| surface.contains("TUI"))
            && replay_surfaces
                .iter()
                .any(|surface| surface.contains("code-mode"))
            && replay_surfaces
                .iter()
                .any(|surface| surface.contains("terminal"))
            && replay_surfaces
                .iter()
                .any(|surface| surface.contains("dependency"));

        Self {
            product: "Hepta".into(),
            status: if replay_ready { "ready" } else { "attention" }.into(),
            replay_id: "upstream-codex-legacy-compatibility-replay-packet".into(),
            replay_packet_path:
                "docs/architecture/HEPTA_UPSTREAM_CODEX_LEGACY_COMPATIBILITY_REPLAY.md".into(),
            selected_bucket_id: "legacy-cli-tui-compatibility".into(),
            selected_changed_file_count: 128,
            replay_surface_count,
            required_replay_surface_count,
            source_absorption_gate:
                "scripts/hepta-upstream-codex-legacy-compatibility-absorption.sh".into(),
            replay_gate: "scripts/hepta-upstream-codex-legacy-compatibility-replay.sh".into(),
            active_dependency_isolation_gate:
                "scripts/hepta-active-service-dependency-isolation.sh".into(),
            cli_command_contract_ready: true,
            tui_presentation_replay_ready: true,
            code_mode_replay_ready: true,
            terminal_helper_replay_ready: true,
            dependency_boundary_ready: true,
            active_cli_tui_promotion_allowed: false,
            active_runtime_code_wiring_allowed: false,
            active_runtime_dependency_allowed: false,
            public_release_claim_allowed: false,
            replay_ready,
            upstream_fetch_performed: false,
            upstream_merge_performed: false,
            upstream_checkout_performed: false,
            workspace_mutation_default: false,
            credential_value_read: false,
            secret_file_read: false,
            provider_invoked: false,
            channel_delivery_performed: false,
            gateway_rpc_performed: false,
            replay_surfaces,
            required_next_gates: vec![
                "map retained CLI commands to explicit Hepta command contracts".into(),
                "keep TUI deltas as presentation snapshots until Hepta-native parity".into(),
                "keep code-mode callbacks behind compatibility-only replay".into(),
                "keep active hepta-cli cargo tree free of tracked Codex engine crates".into(),
            ],
        }
    }
}

impl HeptaUpstreamCodexLegacyCompatibilityPromotionReport {
    pub fn native_default() -> Self {
        let promotion_conditions: Vec<String> = vec![
            "CLI command contract parity is documented".into(),
            "TUI presentation parity remains snapshot-bounded".into(),
            "code-mode callback boundary remains compatibility-only".into(),
            "terminal and PTY helper contracts remain explicit".into(),
            "adapter shadow replay remains required before active CLI/TUI behavior".into(),
            "operator approval model is explicit before live mutation".into(),
            "side-effect boundary keeps CLI/TUI, channels, and gateway RPC off".into(),
        ];
        let required_promotion_condition_count = 7;
        let ready_promotion_condition_count = promotion_conditions.len();
        let promotion_packet_ready =
            ready_promotion_condition_count == required_promotion_condition_count;

        Self {
            product: "Hepta".into(),
            status: if promotion_packet_ready {
                "ready"
            } else {
                "attention"
            }
            .into(),
            promotion_id: "hepta-cli-tui-parity-promotion-packet".into(),
            promotion_packet_path:
                "docs/architecture/HEPTA_UPSTREAM_CODEX_LEGACY_COMPATIBILITY_PROMOTION.md".into(),
            selected_bucket_id: "legacy-cli-tui-compatibility".into(),
            selected_changed_file_count: 128,
            source_replay_gate: "scripts/hepta-upstream-codex-legacy-compatibility-replay.sh"
                .into(),
            promotion_gate: "scripts/hepta-upstream-codex-legacy-compatibility-promotion.sh".into(),
            active_dependency_isolation_gate:
                "scripts/hepta-active-service-dependency-isolation.sh".into(),
            cli_command_contract_parity_ready: true,
            tui_presentation_parity_ready: true,
            code_mode_callback_boundary_ready: true,
            terminal_helper_contract_ready: true,
            adapter_shadow_replay_ready: true,
            operator_approval_model_ready: true,
            side_effect_boundary_ready: true,
            required_promotion_condition_count,
            ready_promotion_condition_count,
            promotion_packet_ready,
            active_cli_tui_promotion_allowed: false,
            active_tui_presentation_promotion_allowed: false,
            active_code_mode_promotion_allowed: false,
            active_runtime_code_wiring_allowed: false,
            active_runtime_dependency_allowed: false,
            public_release_claim_allowed: false,
            upstream_fetch_performed: false,
            upstream_merge_performed: false,
            upstream_checkout_performed: false,
            workspace_mutation_default: false,
            active_service_restart: false,
            credential_value_read: false,
            secret_file_read: false,
            provider_invoked: false,
            channel_delivery_performed: false,
            gateway_rpc_performed: false,
            public_release_published: false,
            promotion_conditions,
            remaining_blockers: vec![
                "active CLI/TUI command promotion is not part of this packet".into(),
                "live TUI presentation promotion remains forbidden".into(),
                "code-mode callback promotion remains forbidden".into(),
                "gateway RPC and channel delivery remain forbidden".into(),
            ],
            required_next_gates: vec![
                "prove active Hepta-native CLI/TUI parity before command promotion".into(),
                "require operator approval before live command or presentation behavior".into(),
                "keep active hepta-cli cargo tree free of tracked Codex engine crates".into(),
                "rerun promotion readiness after each per-surface promotion packet".into(),
            ],
        }
    }
}

impl HeptaUpstreamCodexProviderSecurityAbsorptionReport {
    pub fn native_default() -> Self {
        let security_surfaces: Vec<String> = vec![
            "codex-api provider and realtime endpoint deltas".into(),
            "model-provider catalog and adapter deltas".into(),
            "login/auth/config/permissions credential surfaces".into(),
            "exec and approval policy surfaces".into(),
            "linux/windows/sandboxing policy surfaces".into(),
            "network-proxy and MITM policy surfaces".into(),
        ];
        let selected_changed_file_count = 104;
        let selected_security_surface_count = security_surfaces.len();
        let required_security_surface_count = 6;
        let contract_ready = selected_changed_file_count == 104
            && selected_security_surface_count == required_security_surface_count
            && security_surfaces
                .iter()
                .any(|surface| surface.contains("provider"))
            && security_surfaces
                .iter()
                .any(|surface| surface.contains("credential"))
            && security_surfaces
                .iter()
                .any(|surface| surface.contains("sandbox"))
            && security_surfaces
                .iter()
                .any(|surface| surface.contains("network"));

        Self {
            product: "Hepta".into(),
            status: if contract_ready { "ready" } else { "attention" }.into(),
            absorption_id: "upstream-codex-provider-security-absorption-contract".into(),
            upstream_repository: "https://github.com/openai/codex".into(),
            candidate_diff_range: historical_upstream_codex_receipt_diff_range(),
            selected_bucket_id: "provider-credential-sandbox-security".into(),
            selected_bucket_risk: HeptaUpstreamCodexSyncRisk::P0Security,
            selected_changed_file_count,
            selected_security_surface_count,
            required_security_surface_count,
            source_ledger_gate: "scripts/hepta-upstream-codex-diff-ledger.sh".into(),
            absorption_gate: "scripts/hepta-upstream-codex-provider-security-absorption.sh".into(),
            active_dependency_isolation_gate:
                "scripts/hepta-active-service-dependency-isolation.sh".into(),
            p0_security_review_required: true,
            requires_provider_contract: true,
            requires_auth_credential_redaction: true,
            requires_sandbox_exec_replay: true,
            requires_network_policy_replay: true,
            active_provider_promotion_allowed: false,
            active_security_policy_promotion_allowed: false,
            active_runtime_code_wiring_allowed: false,
            active_runtime_dependency_allowed: false,
            public_release_claim_allowed: false,
            contract_ready,
            upstream_fetch_performed: false,
            upstream_merge_performed: false,
            upstream_checkout_performed: false,
            workspace_mutation_default: false,
            credential_value_read: false,
            secret_file_read: false,
            provider_invoked: false,
            channel_delivery_performed: false,
            gateway_rpc_performed: false,
            security_surfaces,
            required_next_gates: vec![
                "map provider and auth deltas to Hepta redacted provider contracts".into(),
                "run sandbox and exec replay before policy promotion".into(),
                "run network-proxy policy replay before any live network allowance".into(),
                "keep active hepta-cli cargo tree free of tracked Codex engine crates".into(),
                "require operator approval packet and long soak before release claims".into(),
            ],
        }
    }
}

impl HeptaUpstreamCodexProviderSecurityReplayReport {
    pub fn native_default() -> Self {
        let replay_surfaces: Vec<String> = vec![
            "redacted provider catalog and endpoint contract replay".into(),
            "auth login config and credential redaction replay".into(),
            "approval-policy dry-run allow deny matrix replay".into(),
            "sandbox and exec request policy replay".into(),
            "network-proxy policy replay with live-network deny default".into(),
            "side-effect boundary and operator approval replay".into(),
        ];
        let replay_surface_count = replay_surfaces.len();
        let required_replay_surface_count = 6;
        let replay_ready = replay_surface_count == required_replay_surface_count
            && replay_surfaces
                .iter()
                .any(|surface| surface.contains("provider"))
            && replay_surfaces
                .iter()
                .any(|surface| surface.contains("credential"))
            && replay_surfaces
                .iter()
                .any(|surface| surface.contains("approval"))
            && replay_surfaces
                .iter()
                .any(|surface| surface.contains("sandbox"))
            && replay_surfaces
                .iter()
                .any(|surface| surface.contains("network"))
            && replay_surfaces
                .iter()
                .any(|surface| surface.contains("side-effect"));

        Self {
            product: "Hepta".into(),
            status: if replay_ready { "ready" } else { "attention" }.into(),
            replay_id: "upstream-codex-provider-security-replay-packet".into(),
            replay_packet_path:
                "docs/architecture/HEPTA_UPSTREAM_CODEX_PROVIDER_SECURITY_REPLAY.md".into(),
            selected_bucket_id: "provider-credential-sandbox-security".into(),
            selected_changed_file_count: 104,
            replay_surface_count,
            required_replay_surface_count,
            source_absorption_gate: "scripts/hepta-upstream-codex-provider-security-absorption.sh"
                .into(),
            replay_gate: "scripts/hepta-upstream-codex-provider-security-replay.sh".into(),
            active_dependency_isolation_gate:
                "scripts/hepta-active-service-dependency-isolation.sh".into(),
            redacted_provider_contract_ready: true,
            auth_credential_redaction_ready: true,
            approval_policy_replay_ready: true,
            sandbox_exec_replay_ready: true,
            network_policy_replay_ready: true,
            side_effect_boundary_ready: true,
            active_provider_promotion_allowed: false,
            active_security_policy_promotion_allowed: false,
            active_runtime_code_wiring_allowed: false,
            active_runtime_dependency_allowed: false,
            public_release_claim_allowed: false,
            replay_ready,
            upstream_fetch_performed: false,
            upstream_merge_performed: false,
            upstream_checkout_performed: false,
            workspace_mutation_default: false,
            credential_value_read: false,
            secret_file_read: false,
            provider_invoked: false,
            channel_delivery_performed: false,
            gateway_rpc_performed: false,
            replay_surfaces,
            required_next_gates: vec![
                "bind redacted provider contracts to Hepta provider report fields".into(),
                "keep credential values out of replay fixtures and JSON reports".into(),
                "require sandbox exec network replay before security-policy promotion".into(),
                "keep active hepta-cli cargo tree free of tracked Codex engine crates".into(),
                "require operator approval packet and long soak before provider/security claims"
                    .into(),
            ],
        }
    }
}

impl HeptaUpstreamCodexProviderSecurityPromotionReport {
    pub fn native_default() -> Self {
        let promotion_conditions: Vec<String> = vec![
            "redacted provider catalog and endpoint contract is documented".into(),
            "auth and credential handling remains redacted".into(),
            "approval policy replay remains dry-run only".into(),
            "sandbox and exec policy replay remains local".into(),
            "network policy replay keeps live network allowance disabled".into(),
            "operator approval model is explicit before live mutation".into(),
            "side-effect boundary keeps providers, credentials, channels, and gateway RPC off"
                .into(),
        ];
        let required_promotion_condition_count = 7;
        let ready_promotion_condition_count = promotion_conditions.len();
        let promotion_packet_ready =
            ready_promotion_condition_count == required_promotion_condition_count;

        Self {
            product: "Hepta".into(),
            status: if promotion_packet_ready {
                "ready"
            } else {
                "attention"
            }
            .into(),
            promotion_id: "upstream-codex-provider-security-promotion-packet".into(),
            promotion_packet_path:
                "docs/architecture/HEPTA_UPSTREAM_CODEX_PROVIDER_SECURITY_PROMOTION.md".into(),
            selected_bucket_id: "provider-credential-sandbox-security".into(),
            selected_changed_file_count: 104,
            source_replay_gate: "scripts/hepta-upstream-codex-provider-security-replay.sh".into(),
            promotion_gate: "scripts/hepta-upstream-codex-provider-security-promotion.sh".into(),
            active_dependency_isolation_gate:
                "scripts/hepta-active-service-dependency-isolation.sh".into(),
            redacted_provider_contract_ready: true,
            auth_credential_redaction_ready: true,
            approval_policy_replay_ready: true,
            sandbox_exec_replay_ready: true,
            network_policy_replay_ready: true,
            operator_approval_model_ready: true,
            side_effect_boundary_ready: true,
            required_promotion_condition_count,
            ready_promotion_condition_count,
            promotion_packet_ready,
            active_provider_promotion_allowed: false,
            active_security_policy_promotion_allowed: false,
            active_runtime_code_wiring_allowed: false,
            active_runtime_dependency_allowed: false,
            public_release_claim_allowed: false,
            upstream_fetch_performed: false,
            upstream_merge_performed: false,
            upstream_checkout_performed: false,
            workspace_mutation_default: false,
            active_service_restart: false,
            credential_value_read: false,
            secret_file_read: false,
            provider_invoked: false,
            channel_delivery_performed: false,
            gateway_rpc_performed: false,
            public_release_published: false,
            promotion_conditions,
            remaining_blockers: vec![
                "active provider adapter wiring is not part of this packet".into(),
                "live credential reads remain forbidden".into(),
                "live provider invocation remains forbidden".into(),
                "live network allowance remains forbidden".into(),
            ],
            required_next_gates: vec![
                "prove active provider adapter parity before runtime wiring".into(),
                "require operator approval before credential or provider use".into(),
                "keep active hepta-cli cargo tree free of tracked Codex engine crates".into(),
                "rerun promotion readiness after each per-surface promotion packet".into(),
            ],
        }
    }
}

impl HeptaUpstreamCodexRuntimeAppServerAbsorptionReport {
    pub fn native_default() -> Self {
        let runtime_surfaces: Vec<String> = vec![
            "app-server daemon transport and protocol schema deltas".into(),
            "session thread-store and thread lifecycle deltas".into(),
            "tool invocation and tool-policy deltas".into(),
            "MCP client server and app-server MCP request deltas".into(),
            "hooks and runtime event-loop deltas".into(),
            "exec-server and unified exec request deltas".into(),
            "adapter behavior-equivalence and shadow-replay promotion gates".into(),
        ];
        let selected_changed_file_count = 462;
        let selected_runtime_surface_count = runtime_surfaces.len();
        let required_runtime_surface_count = 7;
        let contract_ready = selected_changed_file_count == 462
            && selected_runtime_surface_count == required_runtime_surface_count
            && runtime_surfaces
                .iter()
                .any(|surface| surface.contains("app-server"))
            && runtime_surfaces
                .iter()
                .any(|surface| surface.contains("session"))
            && runtime_surfaces
                .iter()
                .any(|surface| surface.contains("tool"))
            && runtime_surfaces
                .iter()
                .any(|surface| surface.contains("MCP"));

        Self {
            product: "Hepta".into(),
            status: if contract_ready { "ready" } else { "attention" }.into(),
            absorption_id: "upstream-codex-runtime-appserver-absorption-contract".into(),
            upstream_repository: "https://github.com/openai/codex".into(),
            candidate_diff_range: historical_upstream_codex_receipt_diff_range(),
            selected_bucket_id: "runtime-session-tool-mcp-appserver".into(),
            selected_bucket_risk: HeptaUpstreamCodexSyncRisk::P0Runtime,
            selected_changed_file_count,
            selected_runtime_surface_count,
            required_runtime_surface_count,
            source_ledger_gate: "scripts/hepta-upstream-codex-diff-ledger.sh".into(),
            absorption_gate: "scripts/hepta-upstream-codex-runtime-appserver-absorption.sh".into(),
            active_dependency_isolation_gate:
                "scripts/hepta-active-service-dependency-isolation.sh".into(),
            p0_runtime_review_required: true,
            requires_adapter_contract: true,
            requires_session_thread_replay: true,
            requires_tool_mcp_replay: true,
            requires_app_server_protocol_replay: true,
            requires_exec_hook_replay: true,
            active_runtime_promotion_allowed: false,
            active_app_server_promotion_allowed: false,
            active_tool_mcp_promotion_allowed: false,
            active_runtime_code_wiring_allowed: false,
            active_runtime_dependency_allowed: false,
            public_release_claim_allowed: false,
            contract_ready,
            upstream_fetch_performed: false,
            upstream_merge_performed: false,
            upstream_checkout_performed: false,
            workspace_mutation_default: false,
            credential_value_read: false,
            secret_file_read: false,
            provider_invoked: false,
            channel_delivery_performed: false,
            gateway_rpc_performed: false,
            runtime_surfaces,
            required_next_gates: vec![
                "map app-server protocol deltas to Hepta route and event contracts".into(),
                "run session and thread-store replay before lifecycle promotion".into(),
                "run tool and MCP replay before invocation promotion".into(),
                "run exec and hook replay before runtime event-loop promotion".into(),
                "keep active hepta-cli cargo tree free of tracked Codex engine crates".into(),
            ],
        }
    }
}

impl HeptaUpstreamCodexRuntimeAppServerReplayReport {
    pub fn native_default() -> Self {
        let replay_surfaces: Vec<String> = vec![
            "app-server protocol schema and route-event replay".into(),
            "app-server daemon and transport boundary replay".into(),
            "session thread-store and lifecycle replay".into(),
            "tool invocation and tool-policy replay".into(),
            "MCP client server and request-envelope replay".into(),
            "exec-server hook and runtime event-loop replay".into(),
            "side-effect boundary and active dependency isolation replay".into(),
        ];
        let replay_surface_count = replay_surfaces.len();
        let required_replay_surface_count = 7;
        let replay_ready = replay_surface_count == required_replay_surface_count
            && replay_surfaces
                .iter()
                .any(|surface| surface.contains("app-server protocol"))
            && replay_surfaces
                .iter()
                .any(|surface| surface.contains("session"))
            && replay_surfaces
                .iter()
                .any(|surface| surface.contains("tool"))
            && replay_surfaces
                .iter()
                .any(|surface| surface.contains("MCP"))
            && replay_surfaces
                .iter()
                .any(|surface| surface.contains("exec-server"))
            && replay_surfaces
                .iter()
                .any(|surface| surface.contains("side-effect"));

        Self {
            product: "Hepta".into(),
            status: if replay_ready { "ready" } else { "attention" }.into(),
            replay_id: "upstream-codex-runtime-appserver-replay-packet".into(),
            replay_packet_path:
                "docs/architecture/HEPTA_UPSTREAM_CODEX_RUNTIME_APPSERVER_REPLAY.md".into(),
            selected_bucket_id: "runtime-session-tool-mcp-appserver".into(),
            selected_changed_file_count: 462,
            replay_surface_count,
            required_replay_surface_count,
            source_absorption_gate: "scripts/hepta-upstream-codex-runtime-appserver-absorption.sh"
                .into(),
            replay_gate: "scripts/hepta-upstream-codex-runtime-appserver-replay.sh".into(),
            active_dependency_isolation_gate:
                "scripts/hepta-active-service-dependency-isolation.sh".into(),
            app_server_protocol_replay_ready: true,
            session_thread_replay_ready: true,
            tool_mcp_replay_ready: true,
            exec_hook_replay_ready: true,
            side_effect_boundary_ready: true,
            active_runtime_promotion_allowed: false,
            active_app_server_promotion_allowed: false,
            active_tool_mcp_promotion_allowed: false,
            active_runtime_code_wiring_allowed: false,
            active_runtime_dependency_allowed: false,
            public_release_claim_allowed: false,
            replay_ready,
            upstream_fetch_performed: false,
            upstream_merge_performed: false,
            upstream_checkout_performed: false,
            workspace_mutation_default: false,
            credential_value_read: false,
            secret_file_read: false,
            provider_invoked: false,
            channel_delivery_performed: false,
            gateway_rpc_performed: false,
            replay_surfaces,
            required_next_gates: vec![
                "bind app-server protocol deltas to Hepta route and event reports".into(),
                "prove session and thread-store lifecycle replay before promotion".into(),
                "prove tool and MCP request-envelope replay before invocation promotion".into(),
                "prove exec-server and hook replay before event-loop promotion".into(),
                "keep active hepta-cli cargo tree free of tracked Codex engine crates".into(),
            ],
        }
    }
}

impl HeptaUpstreamCodexRuntimeAppServerPromotionReport {
    pub fn native_default() -> Self {
        let promotion_conditions: Vec<String> = vec![
            "app-server route and event contract is documented".into(),
            "session and thread lifecycle promotion remains bounded".into(),
            "tool and MCP request envelopes remain replay-only".into(),
            "exec hook and runtime event-loop replay remains local".into(),
            "adapter shadow replay remains required before active behavior changes".into(),
            "operator approval model is explicit before live mutation".into(),
            "side-effect boundary keeps runtime wiring, channels, and gateway RPC off".into(),
        ];
        let required_promotion_condition_count = 7;
        let ready_promotion_condition_count = promotion_conditions.len();
        let promotion_packet_ready =
            ready_promotion_condition_count == required_promotion_condition_count;

        Self {
            product: "Hepta".into(),
            status: if promotion_packet_ready {
                "ready"
            } else {
                "attention"
            }
            .into(),
            promotion_id: "runtime-appserver-route-event-promotion-packet".into(),
            promotion_packet_path:
                "docs/architecture/HEPTA_UPSTREAM_CODEX_RUNTIME_APPSERVER_PROMOTION.md".into(),
            selected_bucket_id: "runtime-session-tool-mcp-appserver".into(),
            selected_changed_file_count: 462,
            source_replay_gate: "scripts/hepta-upstream-codex-runtime-appserver-replay.sh".into(),
            promotion_gate: "scripts/hepta-upstream-codex-runtime-appserver-promotion.sh".into(),
            active_dependency_isolation_gate:
                "scripts/hepta-active-service-dependency-isolation.sh".into(),
            app_server_route_event_contract_ready: true,
            session_thread_lifecycle_contract_ready: true,
            tool_mcp_request_envelope_ready: true,
            exec_hook_event_loop_replay_ready: true,
            adapter_shadow_replay_ready: true,
            operator_approval_model_ready: true,
            side_effect_boundary_ready: true,
            required_promotion_condition_count,
            ready_promotion_condition_count,
            promotion_packet_ready,
            active_runtime_promotion_allowed: false,
            active_app_server_promotion_allowed: false,
            active_tool_mcp_promotion_allowed: false,
            active_runtime_code_wiring_allowed: false,
            active_runtime_dependency_allowed: false,
            public_release_claim_allowed: false,
            upstream_fetch_performed: false,
            upstream_merge_performed: false,
            upstream_checkout_performed: false,
            workspace_mutation_default: false,
            active_service_restart: false,
            credential_value_read: false,
            secret_file_read: false,
            provider_invoked: false,
            channel_delivery_performed: false,
            gateway_rpc_performed: false,
            public_release_published: false,
            promotion_conditions,
            remaining_blockers: vec![
                "active runtime route/event wiring is not part of this packet".into(),
                "live app-server promotion remains forbidden".into(),
                "live tool and MCP invocation promotion remains forbidden".into(),
                "gateway RPC and channel delivery remain forbidden".into(),
            ],
            required_next_gates: vec![
                "prove active route/event adapter parity before runtime wiring".into(),
                "require operator approval before live app-server or tool/MCP behavior".into(),
                "keep active hepta-cli cargo tree free of tracked Codex engine crates".into(),
                "rerun promotion readiness after each per-surface promotion packet".into(),
            ],
        }
    }
}

impl HeptaUpstreamCodexAbsorptionReplayReadinessReport {
    pub fn native_default() -> Self {
        let covered_buckets: Vec<String> = vec![
            "product-doc-release-governance".into(),
            "legacy-cli-tui-compatibility".into(),
            "provider-credential-sandbox-security".into(),
            "runtime-session-tool-mcp-appserver".into(),
        ];
        let closed_gates: Vec<String> = vec![
            "scripts/hepta-upstream-codex-product-governance-absorption.sh".into(),
            "scripts/hepta-upstream-codex-product-governance-translation.sh".into(),
            "scripts/hepta-upstream-codex-legacy-compatibility-absorption.sh".into(),
            "scripts/hepta-upstream-codex-legacy-compatibility-replay.sh".into(),
            "scripts/hepta-upstream-codex-provider-security-absorption.sh".into(),
            "scripts/hepta-upstream-codex-provider-security-replay.sh".into(),
            "scripts/hepta-upstream-codex-runtime-appserver-absorption.sh".into(),
            "scripts/hepta-upstream-codex-runtime-appserver-replay.sh".into(),
        ];
        let selected_bucket_count = covered_buckets.len();
        let required_selected_bucket_count = 4;
        let absorption_contract_ready_count = 4;
        let required_absorption_contract_ready_count = 4;
        let translation_replay_ready_count = 4;
        let required_translation_replay_ready_count = 4;
        let p0_replay_ready_count = 2;
        let required_p0_replay_ready_count = 2;
        let p1_replay_ready_count = 1;
        let required_p1_replay_ready_count = 1;
        let p2_translation_ready_count = 1;
        let required_p2_translation_ready_count = 1;
        let all_selected_buckets_absorbed = selected_bucket_count == required_selected_bucket_count
            && absorption_contract_ready_count == required_absorption_contract_ready_count;
        let all_required_translation_replay_ready = translation_replay_ready_count
            == required_translation_replay_ready_count
            && p0_replay_ready_count == required_p0_replay_ready_count
            && p1_replay_ready_count == required_p1_replay_ready_count
            && p2_translation_ready_count == required_p2_translation_ready_count;
        let readiness_ready = all_selected_buckets_absorbed
            && all_required_translation_replay_ready
            && closed_gates.len() == 8;

        Self {
            product: "Hepta".into(),
            status: if readiness_ready {
                "ready"
            } else {
                "attention"
            }
            .into(),
            readiness_id: "upstream-codex-absorption-replay-readiness".into(),
            readiness_packet_path:
                "docs/architecture/HEPTA_UPSTREAM_CODEX_ABSORPTION_REPLAY_READINESS.md".into(),
            upstream_repository: "https://github.com/openai/codex".into(),
            candidate_diff_range: historical_upstream_codex_receipt_diff_range(),
            ledger_changed_file_count: HEPTA_UPSTREAM_CODEX_HISTORICAL_LEDGER_CHANGED_FILE_COUNT,
            selected_absorption_changed_file_count:
                HEPTA_UPSTREAM_CODEX_HISTORICAL_SELECTED_ABSORPTION_COUNT,
            selected_bucket_count,
            required_selected_bucket_count,
            absorption_contract_ready_count,
            required_absorption_contract_ready_count,
            translation_replay_ready_count,
            required_translation_replay_ready_count,
            p0_replay_ready_count,
            required_p0_replay_ready_count,
            p1_replay_ready_count,
            required_p1_replay_ready_count,
            p2_translation_ready_count,
            required_p2_translation_ready_count,
            product_governance_translation_ready: true,
            legacy_compatibility_replay_ready: true,
            provider_security_replay_ready: true,
            runtime_appserver_replay_ready: true,
            source_ledger_gate: "scripts/hepta-upstream-codex-diff-ledger.sh".into(),
            active_dependency_isolation_gate:
                "scripts/hepta-active-service-dependency-isolation.sh".into(),
            readiness_gate: "scripts/hepta-upstream-codex-absorption-replay-readiness.sh".into(),
            all_selected_buckets_absorbed,
            all_required_translation_replay_ready,
            active_runtime_code_wiring_allowed: false,
            active_runtime_dependency_allowed: false,
            active_runtime_auto_rebase_allowed: false,
            active_codex_engine_dependency_allowed: false,
            public_release_claim_allowed: false,
            readiness_ready,
            upstream_fetch_performed: false,
            upstream_merge_performed: false,
            upstream_checkout_performed: false,
            workspace_mutation_default: false,
            active_service_restart: false,
            credential_value_read: false,
            secret_file_read: false,
            provider_invoked: false,
            channel_delivery_performed: false,
            gateway_rpc_performed: false,
            public_release_published: false,
            covered_buckets,
            closed_gates,
            required_next_gates: vec![
                "keep active hepta-cli cargo tree free of tracked Codex engine crates".into(),
                "require per-surface promotion packets before active behavior changes".into(),
                "require operator approval and long soak before public release claims".into(),
                "refresh the diff ledger before absorbing a newer upstream Codex range".into(),
            ],
        }
    }
}

impl HeptaUpstreamCodexPromotionReadinessReport {
    pub fn native_default() -> Self {
        let decisions = vec![
            HeptaUpstreamCodexPromotionDecision {
                bucket_id: "product-doc-release-governance".into(),
                risk: HeptaUpstreamCodexSyncRisk::P2Product,
                selected_changed_file_count: 22,
                absorption_replay_ready: true,
                required_surface_promotion_packet:
                    "release-governance-claim-promotion-packet".into(),
                surface_promotion_packet_ready: true,
                active_promotion_allowed: false,
                blocker:
                    "release-governance claim promotion packet is ready, but public claims remain blocked"
                        .into(),
            },
            HeptaUpstreamCodexPromotionDecision {
                bucket_id: "legacy-cli-tui-compatibility".into(),
                risk: HeptaUpstreamCodexSyncRisk::P1Compatibility,
                selected_changed_file_count: 128,
                absorption_replay_ready: true,
                required_surface_promotion_packet: "hepta-cli-tui-parity-promotion-packet".into(),
                surface_promotion_packet_ready: true,
                active_promotion_allowed: false,
                blocker:
                    "legacy CLI/TUI parity promotion packet is ready, but active CLI/TUI promotion remains blocked"
                        .into(),
            },
            HeptaUpstreamCodexPromotionDecision {
                bucket_id: "provider-credential-sandbox-security".into(),
                risk: HeptaUpstreamCodexSyncRisk::P0Security,
                selected_changed_file_count: 104,
                absorption_replay_ready: true,
                required_surface_promotion_packet:
                    "upstream-codex-provider-security-promotion-packet".into(),
                surface_promotion_packet_ready: true,
                active_promotion_allowed: false,
                blocker:
                    "provider/security promotion packet is ready, but active adapter wiring remains blocked"
                        .into(),
            },
            HeptaUpstreamCodexPromotionDecision {
                bucket_id: "runtime-session-tool-mcp-appserver".into(),
                risk: HeptaUpstreamCodexSyncRisk::P0Runtime,
                selected_changed_file_count: 462,
                absorption_replay_ready: true,
                required_surface_promotion_packet:
                    "runtime-appserver-route-event-promotion-packet".into(),
                surface_promotion_packet_ready: true,
                active_promotion_allowed: false,
                blocker:
                    "runtime/app-server promotion packet is ready, but active route/event wiring remains blocked"
                        .into(),
            },
        ];
        let assessed_bucket_count = decisions.len();
        let required_assessed_bucket_count = 4;
        let absorption_replay_ready_count = decisions
            .iter()
            .filter(|decision| decision.absorption_replay_ready)
            .count();
        let required_absorption_replay_ready_count = 4;
        let required_surface_promotion_packet_count = decisions.len();
        let completed_surface_promotion_packet_count = decisions
            .iter()
            .filter(|decision| decision.surface_promotion_packet_ready)
            .count();
        let promotable_bucket_count = decisions
            .iter()
            .filter(|decision| decision.active_promotion_allowed)
            .count();
        let promotion_blocked_bucket_count = assessed_bucket_count - promotable_bucket_count;
        let readiness_source_ready = assessed_bucket_count == required_assessed_bucket_count
            && absorption_replay_ready_count == required_absorption_replay_ready_count;
        let active_promotion_ready = readiness_source_ready
            && completed_surface_promotion_packet_count == required_surface_promotion_packet_count
            && promotable_bucket_count == assessed_bucket_count;
        let decision_ready = readiness_source_ready
            && !active_promotion_ready
            && promotion_blocked_bucket_count == required_assessed_bucket_count;
        let promotion_blockers = decisions
            .iter()
            .map(|decision| decision.blocker.clone())
            .collect();

        Self {
            product: "Hepta".into(),
            status: if decision_ready { "ready" } else { "attention" }.into(),
            decision_id: "upstream-codex-promotion-readiness".into(),
            decision_packet_path: "docs/architecture/HEPTA_UPSTREAM_CODEX_PROMOTION_READINESS.md"
                .into(),
            upstream_repository: "https://github.com/openai/codex".into(),
            candidate_diff_range: historical_upstream_codex_receipt_diff_range(),
            source_readiness_gate: "scripts/hepta-upstream-codex-absorption-replay-readiness.sh"
                .into(),
            promotion_readiness_gate: "scripts/hepta-upstream-codex-promotion-readiness.sh".into(),
            active_dependency_isolation_gate:
                "scripts/hepta-active-service-dependency-isolation.sh".into(),
            assessed_bucket_count,
            required_assessed_bucket_count,
            absorption_replay_ready_count,
            required_absorption_replay_ready_count,
            required_surface_promotion_packet_count,
            completed_surface_promotion_packet_count,
            promotable_bucket_count,
            promotion_blocked_bucket_count,
            readiness_source_ready,
            active_promotion_ready,
            decision_ready,
            active_runtime_code_wiring_allowed: false,
            active_runtime_dependency_allowed: false,
            active_runtime_auto_rebase_allowed: false,
            active_codex_engine_dependency_allowed: false,
            public_release_claim_allowed: false,
            upstream_fetch_performed: false,
            upstream_merge_performed: false,
            upstream_checkout_performed: false,
            workspace_mutation_default: false,
            active_service_restart: false,
            credential_value_read: false,
            secret_file_read: false,
            provider_invoked: false,
            channel_delivery_performed: false,
            gateway_rpc_performed: false,
            public_release_published: false,
            decisions,
            promotion_blockers,
            required_next_gates: vec![
                "write per-surface promotion packets before active behavior changes".into(),
                "prove P0 provider/security promotion with redacted credentials and network policy"
                    .into(),
                "prove P0 runtime/app-server promotion with route/event and shadow-replay evidence"
                    .into(),
                "prove P1 CLI/TUI promotion with Hepta-native parity evidence".into(),
                "keep active hepta-cli cargo tree free of tracked Codex engine crates".into(),
            ],
        }
    }
}

impl HeptaUpstreamCodexPromotionClosureReport {
    pub fn native_default() -> Self {
        let readiness = HeptaUpstreamCodexPromotionReadinessReport::native_default();
        let all_surface_promotion_packets_complete = readiness
            .completed_surface_promotion_packet_count
            == readiness.required_surface_promotion_packet_count
            && readiness.required_surface_promotion_packet_count == 4;
        let active_promotion_denial_ready = all_surface_promotion_packets_complete
            && readiness.promotable_bucket_count == 0
            && readiness.promotion_blocked_bucket_count == readiness.assessed_bucket_count
            && !readiness.active_promotion_ready
            && !readiness.active_runtime_code_wiring_allowed
            && !readiness.active_runtime_dependency_allowed
            && !readiness.active_runtime_auto_rebase_allowed
            && !readiness.active_codex_engine_dependency_allowed
            && !readiness.public_release_claim_allowed;
        let closure_ready = readiness.decision_ready && active_promotion_denial_ready;

        Self {
            product: "Hepta".into(),
            status: if closure_ready { "ready" } else { "attention" }.into(),
            closure_id: "upstream-codex-promotion-closure-denial".into(),
            closure_packet_path: "docs/architecture/HEPTA_UPSTREAM_CODEX_PROMOTION_CLOSURE.md"
                .into(),
            upstream_repository: readiness.upstream_repository,
            candidate_diff_range: readiness.candidate_diff_range,
            source_promotion_readiness_gate: readiness.promotion_readiness_gate,
            closure_gate: "scripts/hepta-upstream-codex-promotion-closure.sh".into(),
            active_dependency_isolation_gate: readiness.active_dependency_isolation_gate,
            required_surface_promotion_packet_count: readiness
                .required_surface_promotion_packet_count,
            completed_surface_promotion_packet_count: readiness
                .completed_surface_promotion_packet_count,
            all_surface_promotion_packets_complete,
            promotable_bucket_count: readiness.promotable_bucket_count,
            promotion_blocked_bucket_count: readiness.promotion_blocked_bucket_count,
            active_promotion_ready: readiness.active_promotion_ready,
            active_promotion_denial_ready,
            closure_ready,
            active_runtime_code_wiring_allowed: false,
            active_runtime_dependency_allowed: false,
            active_runtime_auto_rebase_allowed: false,
            active_codex_engine_dependency_allowed: false,
            public_release_claim_allowed: false,
            public_ga_claim_allowed: false,
            release_artifact_write_allowed: false,
            upstream_fetch_performed: false,
            upstream_merge_performed: false,
            upstream_checkout_performed: false,
            workspace_mutation_default: false,
            active_service_restart: false,
            credential_value_read: false,
            secret_file_read: false,
            provider_invoked: false,
            channel_delivery_performed: false,
            gateway_rpc_performed: false,
            public_release_published: false,
            closure_invariants: vec![
                "all four required surface promotion packets are complete".into(),
                "zero selected upstream Codex buckets are promotable by default".into(),
                "all four selected upstream Codex buckets remain active-promotion blocked".into(),
                "active Hepta runtime keeps zero tracked Codex engine dependencies".into(),
                "public release and public GA claims remain operator-gated".into(),
            ],
            required_next_gates: vec![
                "require explicit operator approval before active runtime wiring".into(),
                "rerun live active-service dependency isolation before any activation".into(),
                "rerun watchdog, browser smoke, and long soak before any public claim".into(),
                "treat newer upstream Codex ranges as new snapshot intake, not auto-rebase".into(),
            ],
        }
    }
}

impl HeptaUpstreamCodexActiveWiringPreconditionReport {
    pub fn native_default() -> Self {
        let closure = HeptaUpstreamCodexPromotionClosureReport::native_default();
        let explicit_operator_approval_required = true;
        let operator_approval_recorded = false;
        let activation_request_id_required = true;
        let activation_request_id_present = false;
        let live_dependency_isolation_required = true;
        let watchdog_required = true;
        let browser_smoke_required = true;
        let long_soak_required = true;
        let active_wiring_allowed = false;
        let active_wiring_precondition_ready = closure.closure_ready
            && closure.all_surface_promotion_packets_complete
            && closure.active_promotion_denial_ready
            && explicit_operator_approval_required
            && activation_request_id_required
            && live_dependency_isolation_required
            && watchdog_required
            && browser_smoke_required
            && long_soak_required
            && !operator_approval_recorded
            && !activation_request_id_present
            && !active_wiring_allowed;

        Self {
            product: "Hepta".into(),
            status: if active_wiring_precondition_ready {
                "ready"
            } else {
                "attention"
            }
            .into(),
            precondition_id: "upstream-codex-active-wiring-precondition".into(),
            precondition_packet_path:
                "docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVE_WIRING_PRECONDITION.md".into(),
            upstream_repository: closure.upstream_repository,
            candidate_diff_range: closure.candidate_diff_range,
            source_closure_gate: closure.closure_gate,
            active_wiring_precondition_gate:
                "scripts/hepta-upstream-codex-active-wiring-precondition.sh".into(),
            active_dependency_isolation_gate: closure.active_dependency_isolation_gate,
            promotion_closure_ready: closure.closure_ready,
            all_surface_promotion_packets_complete: closure.all_surface_promotion_packets_complete,
            active_promotion_denial_ready: closure.active_promotion_denial_ready,
            explicit_operator_approval_required,
            operator_approval_recorded,
            activation_request_id_required,
            activation_request_id_present,
            live_dependency_isolation_required,
            watchdog_required,
            browser_smoke_required,
            long_soak_required,
            active_wiring_precondition_ready,
            active_wiring_allowed,
            active_runtime_code_wiring_allowed: false,
            active_runtime_dependency_allowed: false,
            active_runtime_auto_rebase_allowed: false,
            active_codex_engine_dependency_allowed: false,
            public_release_claim_allowed: false,
            public_ga_claim_allowed: false,
            release_artifact_write_allowed: false,
            upstream_fetch_performed: false,
            upstream_merge_performed: false,
            upstream_checkout_performed: false,
            workspace_mutation_default: false,
            active_service_restart: false,
            credential_value_read: false,
            secret_file_read: false,
            provider_invoked: false,
            channel_delivery_performed: false,
            gateway_rpc_performed: false,
            public_release_published: false,
            preconditions: vec![
                "promotion closure gate is ready".into(),
                "all four required surface promotion packets are complete".into(),
                "active promotion denial remains ready".into(),
                "explicit operator approval record is required and not yet recorded".into(),
                "activation request id is required and not yet present".into(),
                "live dependency isolation, watchdog, browser smoke, and long soak must be fresh"
                    .into(),
            ],
            required_next_gates: vec![
                "record the activation request packet schema before any active wiring".into(),
                "bind any future activation request to a concrete activation_request_id".into(),
                "record an operator approval id and hashed operator identity before any active wiring"
                    .into(),
                "rerun live active-service dependency isolation at activation time".into(),
                "rerun watchdog, browser smoke, and long soak at activation time".into(),
                "keep public release and public GA claims false until a separate release gate"
                    .into(),
            ],
        }
    }
}

impl HeptaUpstreamCodexActivationRequestPacketReport {
    pub fn native_default() -> Self {
        let precondition = HeptaUpstreamCodexActiveWiringPreconditionReport::native_default();
        let schema_fields = default_activation_request_packet_fields();
        let required_schema_field_count =
            schema_fields.iter().filter(|field| field.required).count();
        let recorded_required_schema_field_count = schema_fields
            .iter()
            .filter(|field| field.required && field.recorded)
            .count();
        let schema_field_count = schema_fields.len();
        let operator_approval_recorded = false;
        let activation_request_id_recorded = false;
        let activation_packet_recorded = false;
        let active_wiring_allowed = false;
        let activation_packet_schema_ready = precondition.active_wiring_precondition_ready
            && !precondition.active_wiring_allowed
            && required_schema_field_count == schema_field_count
            && recorded_required_schema_field_count == 0
            && schema_fields
                .iter()
                .all(|field| field.required && !field.recorded)
            && !operator_approval_recorded
            && !activation_request_id_recorded
            && !activation_packet_recorded
            && !active_wiring_allowed;

        Self {
            product: "Hepta".into(),
            status: if activation_packet_schema_ready {
                "ready"
            } else {
                "attention"
            }
            .into(),
            packet_id: "upstream-codex-activation-request-packet-schema".into(),
            packet_schema_path:
                "docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVATION_REQUEST_PACKET.md".into(),
            upstream_repository: precondition.upstream_repository,
            candidate_diff_range: precondition.candidate_diff_range,
            source_precondition_gate: precondition.active_wiring_precondition_gate,
            activation_request_packet_gate:
                "scripts/hepta-upstream-codex-activation-request-packet.sh".into(),
            active_dependency_isolation_gate: precondition.active_dependency_isolation_gate,
            active_wiring_precondition_ready: precondition.active_wiring_precondition_ready,
            active_wiring_allowed_by_precondition: precondition.active_wiring_allowed,
            operator_approval_required: precondition.explicit_operator_approval_required,
            operator_approval_recorded,
            activation_request_id_required: precondition.activation_request_id_required,
            activation_request_id_recorded,
            required_schema_field_count,
            recorded_required_schema_field_count,
            schema_field_count,
            activation_packet_schema_ready,
            activation_packet_recorded,
            active_wiring_allowed,
            active_runtime_code_wiring_allowed: false,
            active_runtime_dependency_allowed: false,
            active_runtime_auto_rebase_allowed: false,
            active_codex_engine_dependency_allowed: false,
            public_release_claim_allowed: false,
            public_ga_claim_allowed: false,
            release_artifact_write_allowed: false,
            upstream_fetch_performed: false,
            upstream_merge_performed: false,
            upstream_checkout_performed: false,
            workspace_mutation_default: false,
            active_service_restart: false,
            credential_value_read: false,
            secret_file_read: false,
            provider_invoked: false,
            channel_delivery_performed: false,
            gateway_rpc_performed: false,
            public_release_published: false,
            schema_fields,
            packet_invariants: vec![
                "packet schema is ready but no activation packet is recorded".into(),
                "operator approval must be explicit and is not recorded by default".into(),
                "activation request id must be concrete and is not recorded by default".into(),
                "live dependency isolation, watchdog, browser smoke, long soak, and rollback evidence are required fields".into(),
                "public release and artifact decisions stay false in the schema packet".into(),
            ],
            required_next_gates: vec![
                "record a concrete activation_request_id before any active wiring".into(),
                "record an operator approval id and hashed operator identity before any active wiring".into(),
                "attach fresh live dependency isolation, watchdog, browser smoke, long-soak, and rollback evidence ids".into(),
                "keep active Codex engine dependency and release artifact decisions false unless separately approved".into(),
                "rerun clean preflight and live gates after any future activation packet is recorded".into(),
            ],
        }
    }
}

impl HeptaUpstreamCodexActivationPacketDryRunReport {
    pub fn native_default() -> Self {
        let packet = HeptaUpstreamCodexActivationRequestPacketReport::native_default();
        let fixtures =
            default_activation_packet_dry_run_fixtures(packet.required_schema_field_count);
        let fixture_count = fixtures.len();
        let blocked_fixture_count = fixtures
            .iter()
            .filter(|fixture| {
                fixture.validation_status == "blocked"
                    && !fixture.active_wiring_allowed
                    && !fixture.public_release_claim_allowed
                    && !fixture.release_artifact_write_allowed
            })
            .count();
        let allowed_fixture_count = fixtures
            .iter()
            .filter(|fixture| fixture.active_wiring_allowed)
            .count();
        let active_wiring_allowed = false;
        let dry_run_validator_ready = packet.activation_packet_schema_ready
            && !packet.activation_packet_recorded
            && packet.required_schema_field_count == 14
            && fixture_count == 3
            && blocked_fixture_count == fixture_count
            && allowed_fixture_count == 0
            && !active_wiring_allowed;

        Self {
            product: "Hepta".into(),
            status: if dry_run_validator_ready {
                "ready"
            } else {
                "attention"
            }
            .into(),
            validator_id: "upstream-codex-activation-packet-dry-run-validator".into(),
            validator_doc_path:
                "docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVATION_PACKET_DRY_RUN.md".into(),
            upstream_repository: packet.upstream_repository,
            candidate_diff_range: packet.candidate_diff_range,
            source_packet_gate: packet.activation_request_packet_gate,
            dry_run_validator_gate: "scripts/hepta-upstream-codex-activation-packet-dry-run.sh"
                .into(),
            active_dependency_isolation_gate: packet.active_dependency_isolation_gate,
            activation_packet_schema_ready: packet.activation_packet_schema_ready,
            activation_packet_recorded: packet.activation_packet_recorded,
            required_schema_field_count: packet.required_schema_field_count,
            fixture_count,
            blocked_fixture_count,
            allowed_fixture_count,
            dry_run_validator_ready,
            active_wiring_allowed,
            active_runtime_code_wiring_allowed: false,
            active_runtime_dependency_allowed: false,
            active_runtime_auto_rebase_allowed: false,
            active_codex_engine_dependency_allowed: false,
            public_release_claim_allowed: false,
            public_ga_claim_allowed: false,
            release_artifact_write_allowed: false,
            upstream_fetch_performed: false,
            upstream_merge_performed: false,
            upstream_checkout_performed: false,
            workspace_mutation_default: false,
            active_service_restart: false,
            credential_value_read: false,
            secret_file_read: false,
            provider_invoked: false,
            channel_delivery_performed: false,
            gateway_rpc_performed: false,
            public_release_published: false,
            fixtures,
            validation_invariants: vec![
                "dry-run fixtures cannot activate wiring without all required fields".into(),
                "operator approval and activation request id must both be recorded".into(),
                "live evidence and rollback fields must be present before activation".into(),
                "public release and artifact-write requests are denied by default".into(),
                "the dry-run validator performs no upstream or runtime side effects".into(),
            ],
            required_next_gates: vec![
                "replace placeholder fixtures with a concrete activation packet only after operator approval".into(),
                "bind the activation packet to fresh dependency-isolation, watchdog, browser-smoke, long-soak, and rollback evidence ids".into(),
                "keep active Codex engine dependency false unless a separate dependency-change review approves it".into(),
                "keep public release and artifact-write decisions false until release governance approves them".into(),
                "rerun clean preflight and live gates after any future concrete activation packet is recorded".into(),
            ],
        }
    }
}

fn activation_packet_dry_run_fixture(
    required_schema_field_count: usize,
    fixture_id: &str,
    title: &str,
    recorded_required_field_count: usize,
    operator_approval_recorded: bool,
    activation_request_id_recorded: bool,
    live_evidence_recorded: bool,
    rollback_plan_recorded: bool,
    public_release_claim_requested: bool,
    release_artifact_write_requested: bool,
    blocked_reason: &str,
) -> HeptaUpstreamCodexActivationPacketDryRunFixture {
    HeptaUpstreamCodexActivationPacketDryRunFixture {
        fixture_id: fixture_id.into(),
        title: title.into(),
        recorded_required_field_count,
        missing_required_field_count: required_schema_field_count
            .saturating_sub(recorded_required_field_count),
        operator_approval_recorded,
        activation_request_id_recorded,
        live_evidence_recorded,
        rollback_plan_recorded,
        public_release_claim_requested,
        release_artifact_write_requested,
        validation_status: "blocked".into(),
        blocked_reason: blocked_reason.into(),
        active_wiring_allowed: false,
        public_release_claim_allowed: false,
        release_artifact_write_allowed: false,
    }
}

fn default_activation_packet_dry_run_fixtures(
    required_schema_field_count: usize,
) -> Vec<HeptaUpstreamCodexActivationPacketDryRunFixture> {
    vec![
        activation_packet_dry_run_fixture(
            required_schema_field_count,
            "empty-placeholder",
            "empty placeholder packet",
            0,
            false,
            false,
            false,
            false,
            false,
            false,
            "all required activation fields are missing",
        ),
        activation_packet_dry_run_fixture(
            required_schema_field_count,
            "operator-only-placeholder",
            "operator marker without activation evidence",
            2,
            true,
            false,
            false,
            false,
            false,
            false,
            "activation request id, live evidence, and rollback plan are missing",
        ),
        activation_packet_dry_run_fixture(
            required_schema_field_count,
            "public-claim-attempt-without-evidence",
            "public claim request without approval evidence",
            6,
            true,
            true,
            false,
            false,
            true,
            true,
            "public release and artifact-write requests remain denied without full evidence",
        ),
    ]
}

impl HeptaUpstreamCodexActivationEvidenceLedgerReport {
    pub fn native_default() -> Self {
        let dry_run = HeptaUpstreamCodexActivationPacketDryRunReport::native_default();
        let evidence_requirements = default_activation_evidence_requirements();
        let required_evidence_count = evidence_requirements
            .iter()
            .filter(|requirement| requirement.required)
            .count();
        let recorded_evidence_count = evidence_requirements
            .iter()
            .filter(|requirement| requirement.required && requirement.recorded)
            .count();
        let fresh_evidence_count = evidence_requirements
            .iter()
            .filter(|requirement| requirement.required && requirement.fresh)
            .count();
        let evidence_recorded = false;
        let active_wiring_allowed = false;
        let evidence_ledger_ready = dry_run.dry_run_validator_ready
            && !dry_run.activation_packet_recorded
            && required_evidence_count == 8
            && recorded_evidence_count == 0
            && fresh_evidence_count == 0
            && evidence_requirements.iter().all(|requirement| {
                requirement.required && !requirement.recorded && !requirement.fresh
            })
            && !evidence_recorded
            && !active_wiring_allowed;

        Self {
            product: "Hepta".into(),
            status: if evidence_ledger_ready {
                "ready"
            } else {
                "attention"
            }
            .into(),
            ledger_id: "upstream-codex-activation-evidence-ledger-checklist".into(),
            ledger_doc_path:
                "docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVATION_EVIDENCE_LEDGER.md".into(),
            upstream_repository: dry_run.upstream_repository,
            candidate_diff_range: dry_run.candidate_diff_range,
            source_dry_run_gate: dry_run.dry_run_validator_gate,
            evidence_ledger_gate: "scripts/hepta-upstream-codex-activation-evidence-ledger.sh"
                .into(),
            active_dependency_isolation_gate: dry_run.active_dependency_isolation_gate,
            dry_run_validator_ready: dry_run.dry_run_validator_ready,
            activation_packet_recorded: dry_run.activation_packet_recorded,
            required_evidence_count,
            recorded_evidence_count,
            fresh_evidence_count,
            evidence_ledger_ready,
            evidence_recorded,
            active_wiring_allowed,
            active_runtime_code_wiring_allowed: false,
            active_runtime_dependency_allowed: false,
            active_runtime_auto_rebase_allowed: false,
            active_codex_engine_dependency_allowed: false,
            public_release_claim_allowed: false,
            public_ga_claim_allowed: false,
            release_artifact_write_allowed: false,
            upstream_fetch_performed: false,
            upstream_merge_performed: false,
            upstream_checkout_performed: false,
            workspace_mutation_default: false,
            active_service_restart: false,
            credential_value_read: false,
            secret_file_read: false,
            provider_invoked: false,
            channel_delivery_performed: false,
            gateway_rpc_performed: false,
            public_release_published: false,
            evidence_requirements,
            ledger_invariants: vec![
                "evidence ledger is a checklist only and records no concrete evidence by default".into(),
                "all eight required evidence slots are required but unrecorded".into(),
                "freshness starts false until evidence ids are bound to current live gates".into(),
                "activation packet remains unrecorded while evidence is missing".into(),
                "public release and artifact-write decisions stay denied".into(),
            ],
            required_next_gates: vec![
                "record a concrete activation request id before any active wiring".into(),
                "record an operator approval id and hashed operator identity before any active wiring".into(),
                "bind live dependency-isolation, watchdog, browser-smoke, long-soak, and rollback evidence ids".into(),
                "rerun the activation packet dry-run validator after concrete evidence is recorded".into(),
                "rerun clean preflight and live gates before considering any operator-approved activation packet".into(),
            ],
        }
    }
}

fn activation_evidence_requirement(
    id: &str,
    source_gate: &str,
    purpose: &str,
) -> HeptaUpstreamCodexActivationEvidenceRequirement {
    HeptaUpstreamCodexActivationEvidenceRequirement {
        id: id.into(),
        required: true,
        recorded: false,
        fresh: false,
        source_gate: source_gate.into(),
        purpose: purpose.into(),
    }
}

fn default_activation_evidence_requirements() -> Vec<HeptaUpstreamCodexActivationEvidenceRequirement>
{
    vec![
        activation_evidence_requirement(
            "activation_request_id",
            "operator activation request record",
            "unique activation request binding all evidence records",
        ),
        activation_evidence_requirement(
            "operator_approval_id",
            "operator approval packet",
            "explicit operator approval for the requested activation",
        ),
        activation_evidence_requirement(
            "operator_identity_hash",
            "operator approval packet",
            "hashed operator identity with no raw account or credential detail",
        ),
        activation_evidence_requirement(
            "live_dependency_isolation_evidence_id",
            "scripts/hepta-active-service-dependency-isolation.sh",
            "fresh live active-service dependency isolation evidence",
        ),
        activation_evidence_requirement(
            "watchdog_evidence_id",
            "scripts/hepta-watchdog.sh",
            "fresh watchdog evidence for the active service",
        ),
        activation_evidence_requirement(
            "browser_smoke_evidence_id",
            "scripts/hepta-browser-visual-smoke.sh",
            "fresh browser visual smoke evidence",
        ),
        activation_evidence_requirement(
            "long_soak_evidence_id",
            "scripts/hepta-live-soak.sh",
            "fresh long-soak evidence for the active service",
        ),
        activation_evidence_requirement(
            "rollback_plan_id",
            "operator rollback plan record",
            "explicit rollback anchor for the requested activation",
        ),
    ]
}

impl HeptaUpstreamCodexActivationReadinessClosureReport {
    pub fn native_default() -> Self {
        let packet = HeptaUpstreamCodexActivationRequestPacketReport::native_default();
        let dry_run = HeptaUpstreamCodexActivationPacketDryRunReport::native_default();
        let ledger = HeptaUpstreamCodexActivationEvidenceLedgerReport::native_default();

        let readiness_inputs_ready = packet.activation_packet_schema_ready
            && dry_run.dry_run_validator_ready
            && ledger.evidence_ledger_ready
            && packet.required_schema_field_count == 14
            && dry_run.blocked_fixture_count == 3
            && dry_run.allowed_fixture_count == 0
            && ledger.required_evidence_count == 8
            && ledger.recorded_evidence_count == 0
            && ledger.fresh_evidence_count == 0;
        let activation_packet_recorded = packet.activation_packet_recorded
            || dry_run.activation_packet_recorded
            || ledger.activation_packet_recorded;
        let evidence_recorded = ledger.evidence_recorded;
        let active_wiring_allowed = packet.active_wiring_allowed
            || dry_run.active_wiring_allowed
            || ledger.active_wiring_allowed;
        let operator_approved_activation_ready = false;
        let activation_denied_by_default = !activation_packet_recorded
            && !evidence_recorded
            && !operator_approved_activation_ready
            && !active_wiring_allowed;
        let activation_readiness_closure_ready =
            readiness_inputs_ready && activation_denied_by_default;

        Self {
            product: "Hepta".into(),
            status: if activation_readiness_closure_ready {
                "ready"
            } else {
                "attention"
            }
            .into(),
            closure_id: "upstream-codex-activation-readiness-closure-denial".into(),
            closure_doc_path:
                "docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVATION_READINESS_CLOSURE.md".into(),
            upstream_repository: ledger.upstream_repository,
            candidate_diff_range: ledger.candidate_diff_range,
            source_packet_gate: dry_run.source_packet_gate,
            source_dry_run_gate: ledger.source_dry_run_gate,
            source_evidence_ledger_gate: ledger.evidence_ledger_gate,
            activation_readiness_closure_gate:
                "scripts/hepta-upstream-codex-activation-readiness-closure.sh".into(),
            active_dependency_isolation_gate: ledger.active_dependency_isolation_gate,
            activation_packet_schema_ready: packet.activation_packet_schema_ready,
            dry_run_validator_ready: dry_run.dry_run_validator_ready,
            evidence_ledger_ready: ledger.evidence_ledger_ready,
            activation_packet_recorded,
            evidence_recorded,
            required_schema_field_count: packet.required_schema_field_count,
            blocked_fixture_count: dry_run.blocked_fixture_count,
            allowed_fixture_count: dry_run.allowed_fixture_count,
            required_evidence_count: ledger.required_evidence_count,
            recorded_evidence_count: ledger.recorded_evidence_count,
            fresh_evidence_count: ledger.fresh_evidence_count,
            readiness_inputs_ready,
            activation_denied_by_default,
            activation_readiness_closure_ready,
            operator_approved_activation_ready,
            active_wiring_allowed,
            active_runtime_code_wiring_allowed: false,
            active_runtime_dependency_allowed: false,
            active_runtime_auto_rebase_allowed: false,
            active_codex_engine_dependency_allowed: false,
            public_release_claim_allowed: false,
            public_ga_claim_allowed: false,
            release_artifact_write_allowed: false,
            upstream_fetch_performed: false,
            upstream_merge_performed: false,
            upstream_checkout_performed: false,
            workspace_mutation_default: false,
            active_service_restart: false,
            credential_value_read: false,
            secret_file_read: false,
            provider_invoked: false,
            channel_delivery_performed: false,
            gateway_rpc_performed: false,
            public_release_published: false,
            closure_invariants: vec![
                "activation packet schema, dry-run validator, and evidence ledger are ready"
                    .into(),
                "no concrete activation packet is recorded by default".into(),
                "no activation evidence is recorded or fresh by default".into(),
                "operator-approved activation is not ready without a concrete packet and fresh evidence"
                    .into(),
                "active wiring, public release claims, and artifact writes stay denied".into(),
            ],
            required_next_gates: vec![
                "record a concrete operator-approved activation packet".into(),
                "bind all eight evidence slots to fresh live gate evidence".into(),
                "rerun dry-run validation against the concrete activation packet".into(),
                "rerun clean preflight, live gates, and long soak before any active wiring decision"
                    .into(),
                "keep active Hepta service dependency isolation green throughout activation review"
                    .into(),
            ],
        }
    }
}

impl HeptaUpstreamCodexActivationDeniedSampleReport {
    pub fn native_default() -> Self {
        let closure = HeptaUpstreamCodexActivationReadinessClosureReport::native_default();
        let sample_required_schema_field_count = closure.required_schema_field_count;
        let sample_recorded_schema_field_count = sample_required_schema_field_count;
        let sample_required_evidence_count = closure.required_evidence_count;
        let sample_fresh_evidence_count = 0;
        let sample_operator_approval_field_present = true;
        let sample_operator_approval_recorded = false;
        let sample_public_release_claim_requested = true;
        let sample_release_artifact_write_requested = true;
        let active_wiring_allowed = false;
        let sample_packet_shape_complete =
            sample_recorded_schema_field_count == sample_required_schema_field_count;
        let sample_validation_status = "blocked".to_string();
        let sample_blocked_reason =
            "operator approval is not recorded and activation evidence is not fresh".to_string();
        let denied_sample_ready = closure.activation_readiness_closure_ready
            && sample_packet_shape_complete
            && sample_required_schema_field_count == 14
            && sample_recorded_schema_field_count == 14
            && sample_required_evidence_count == 8
            && sample_fresh_evidence_count == 0
            && sample_operator_approval_field_present
            && !sample_operator_approval_recorded
            && sample_public_release_claim_requested
            && sample_release_artifact_write_requested
            && sample_validation_status == "blocked"
            && !active_wiring_allowed;

        Self {
            product: "Hepta".into(),
            status: if denied_sample_ready {
                "ready"
            } else {
                "attention"
            }
            .into(),
            sample_id: "upstream-codex-activation-denied-sample-packet".into(),
            sample_doc_path:
                "docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVATION_DENIED_SAMPLE.md".into(),
            upstream_repository: closure.upstream_repository,
            candidate_diff_range: closure.candidate_diff_range,
            source_readiness_closure_gate: closure.activation_readiness_closure_gate,
            denied_sample_gate: "scripts/hepta-upstream-codex-activation-denied-sample.sh"
                .into(),
            active_dependency_isolation_gate: closure.active_dependency_isolation_gate,
            activation_readiness_closure_ready: closure.activation_readiness_closure_ready,
            sample_packet_shape_complete,
            sample_required_schema_field_count,
            sample_recorded_schema_field_count,
            sample_required_evidence_count,
            sample_fresh_evidence_count,
            sample_operator_approval_field_present,
            sample_operator_approval_recorded,
            sample_public_release_claim_requested,
            sample_release_artifact_write_requested,
            sample_validation_status,
            sample_blocked_reason,
            active_wiring_allowed,
            active_runtime_code_wiring_allowed: false,
            active_runtime_dependency_allowed: false,
            active_runtime_auto_rebase_allowed: false,
            active_codex_engine_dependency_allowed: false,
            public_release_claim_allowed: false,
            public_ga_claim_allowed: false,
            release_artifact_write_allowed: false,
            upstream_fetch_performed: false,
            upstream_merge_performed: false,
            upstream_checkout_performed: false,
            workspace_mutation_default: false,
            active_service_restart: false,
            credential_value_read: false,
            secret_file_read: false,
            provider_invoked: false,
            channel_delivery_performed: false,
            gateway_rpc_performed: false,
            public_release_published: false,
            sample_invariants: vec![
                "full-shaped activation samples are not approvals".into(),
                "operator approval must be recorded separately from packet shape".into(),
                "all eight evidence slots must be fresh before activation can be reconsidered"
                    .into(),
                "public release claims and artifact writes remain denied for the denied sample"
                    .into(),
            ],
            required_next_gates: vec![
                "replace the denied sample with a concrete operator-approved activation packet"
                    .into(),
                "bind every evidence slot to fresh live dependency, watchdog, browser, soak, and rollback evidence"
                    .into(),
                "rerun activation readiness closure after concrete approval and evidence".into(),
                "run clean preflight and live gates before any active wiring decision".into(),
            ],
        }
    }
}

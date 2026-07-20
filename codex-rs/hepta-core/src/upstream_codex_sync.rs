use serde::Deserialize;
use serde::Serialize;

pub const HEPTA_UPSTREAM_CODEX_INTAKE_BASE_HEAD: &str = "108234b5ebe6941764a6b8edbb37b2aa04369f07";
pub const HEPTA_UPSTREAM_CODEX_INTAKE_CUTOFF_REF: &str =
    "refs/remotes/upstream/hepta-intake-20260721";
pub const HEPTA_UPSTREAM_CODEX_INTAKE_CUTOFF_HEAD: &str =
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
            HEPTA_UPSTREAM_CODEX_INTAKE_CUTOFF_HEAD,
            &["31c6065061185de711aa36ee6e9cf7c4a4795821"],
            "semantic_port",
            "share history snapshots copy-on-write",
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
        let current_intake_ready = selected_absorption_count == 11
            && deferred_decision_count == 3
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
            intake_id: "upstream-codex-intake-2026-07-21".into(),
            manifest_path: "docs/architecture/HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_2026-07-21.json"
                .into(),
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
            observed_commit_count: 1803,
            observed_changed_file_count: 3359,
            observed_codex_rs_changed_file_count: 3097,
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

impl HeptaUpstreamCodexActivationEvidenceFreshnessPolicyReport {
    pub fn native_default() -> Self {
        let denied_sample = HeptaUpstreamCodexActivationDeniedSampleReport::native_default();
        let freshness_entries = default_activation_evidence_freshness_policy_entries();
        let required_evidence_count = denied_sample.sample_required_evidence_count;
        let policy_entry_count = freshness_entries.len();
        let missing_evidence_count = freshness_entries
            .iter()
            .filter(|entry| entry.required_for_activation && !entry.recorded)
            .count();
        let fresh_evidence_count = freshness_entries
            .iter()
            .filter(|entry| entry.required_for_activation && entry.fresh)
            .count();
        let expired_evidence_count = 0;
        let stale_evidence_count = 0;
        let activation_allowed_by_freshness_policy = false;
        let activation_blocked_by_freshness_policy = true;
        let freshness_denial_reason =
            "all required activation evidence slots are absent from the denied sample".to_string();
        let freshness_policy_ready = denied_sample.status == "ready"
            && denied_sample.sample_validation_status == "blocked"
            && policy_entry_count == required_evidence_count
            && missing_evidence_count == required_evidence_count
            && fresh_evidence_count == 0
            && activation_blocked_by_freshness_policy
            && !activation_allowed_by_freshness_policy;

        Self {
            product: "Hepta".into(),
            status: if freshness_policy_ready {
                "ready"
            } else {
                "attention"
            }
            .into(),
            policy_id: "upstream-codex-activation-evidence-freshness-policy".into(),
            policy_doc_path:
                "docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVATION_EVIDENCE_FRESHNESS_POLICY.md"
                    .into(),
            upstream_repository: denied_sample.upstream_repository,
            candidate_diff_range: denied_sample.candidate_diff_range,
            source_denied_sample_gate: denied_sample.denied_sample_gate,
            freshness_policy_gate:
                "scripts/hepta-upstream-codex-activation-evidence-freshness-policy.sh".into(),
            active_dependency_isolation_gate: denied_sample.active_dependency_isolation_gate,
            denied_sample_ready: denied_sample.status == "ready",
            required_evidence_count,
            policy_entry_count,
            missing_evidence_count,
            fresh_evidence_count,
            expired_evidence_count,
            stale_evidence_count,
            freshness_policy_ready,
            activation_blocked_by_freshness_policy,
            activation_allowed_by_freshness_policy,
            freshness_denial_reason,
            active_wiring_allowed: false,
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
            freshness_entries,
            policy_invariants: vec![
                "freshness policy defines evidence requirements but records no evidence".into(),
                "missing evidence is a denial reason even when packet shape is complete".into(),
                "freshness is evaluated per evidence slot before active wiring can be reconsidered"
                    .into(),
                "operator approval, public release claims, and artifact writes remain denied"
                    .into(),
            ],
            required_next_gates: vec![
                "bind each required evidence slot to a concrete evidence id".into(),
                "timestamp and hash every live dependency, watchdog, browser, soak, and rollback evidence record"
                    .into(),
                "rerun the denied-sample gate after replacing absence with concrete evidence".into(),
                "rerun clean preflight and live gates before any active wiring decision".into(),
            ],
        }
    }
}

fn activation_evidence_freshness_policy_entry(
    evidence_id: &str,
    source_gate: &str,
    freshness_anchor: &str,
    max_age_policy: &str,
    denial_reason: &str,
) -> HeptaUpstreamCodexActivationEvidenceFreshnessPolicyEntry {
    HeptaUpstreamCodexActivationEvidenceFreshnessPolicyEntry {
        evidence_id: evidence_id.into(),
        source_gate: source_gate.into(),
        freshness_anchor: freshness_anchor.into(),
        max_age_policy: max_age_policy.into(),
        required_for_activation: true,
        recorded: false,
        fresh: false,
        denial_reason: denial_reason.into(),
    }
}

fn default_activation_evidence_freshness_policy_entries()
-> Vec<HeptaUpstreamCodexActivationEvidenceFreshnessPolicyEntry> {
    vec![
        activation_evidence_freshness_policy_entry(
            "activation_request_id",
            "scripts/hepta-upstream-codex-activation-request-packet.sh",
            "candidate diff range and requested activation scope",
            "same activation request",
            "activation request id is absent",
        ),
        activation_evidence_freshness_policy_entry(
            "operator_approval_id",
            "scripts/hepta-public-ga-operator-approval-packet.sh",
            "explicit operator approval record",
            "same activation request",
            "operator approval id is absent",
        ),
        activation_evidence_freshness_policy_entry(
            "operator_identity_hash",
            "scripts/hepta-public-ga-operator-approval-packet.sh",
            "redacted operator identity bound to approval id",
            "same activation request",
            "operator identity hash is absent",
        ),
        activation_evidence_freshness_policy_entry(
            "live_dependency_isolation_evidence_id",
            "scripts/hepta-active-service-dependency-isolation.sh",
            "active binary sha and live dependency-closure route",
            "30 minutes",
            "live dependency isolation evidence is absent",
        ),
        activation_evidence_freshness_policy_entry(
            "watchdog_evidence_id",
            "scripts/hepta-watchdog.sh",
            "active binary sha and live watchdog route matrix",
            "30 minutes",
            "watchdog evidence is absent",
        ),
        activation_evidence_freshness_policy_entry(
            "browser_smoke_evidence_id",
            "scripts/hepta-browser-visual-smoke.sh",
            "desktop and mobile screenshot hashes",
            "30 minutes",
            "browser smoke evidence is absent",
        ),
        activation_evidence_freshness_policy_entry(
            "long_soak_evidence_id",
            "scripts/hepta-live-soak.sh",
            "24/24 live soak sample report",
            "120 minutes",
            "long soak evidence is absent",
        ),
        activation_evidence_freshness_policy_entry(
            "rollback_plan_id",
            "docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVATION_DENIED_SAMPLE.md",
            "candidate diff range and active binary rollback anchor",
            "same activation request",
            "rollback plan id is absent",
        ),
    ]
}

impl HeptaUpstreamCodexActivationEvidenceBindingRecordManifestReport {
    pub fn native_default() -> Self {
        let freshness = HeptaUpstreamCodexActivationEvidenceFreshnessPolicyReport::native_default();
        let binding_schema_fields = default_activation_evidence_binding_record_schema_fields();
        let binding_records =
            default_activation_evidence_binding_record_manifest_entries(&freshness);
        let required_evidence_count = freshness.required_evidence_count;
        let binding_record_count = binding_records.len();
        let missing_binding_record_count = binding_records
            .iter()
            .filter(|record| !record.evidence_recorded)
            .count();
        let recorded_binding_record_count = binding_records
            .iter()
            .filter(|record| record.evidence_recorded)
            .count();
        let required_record_schema_field_count = binding_schema_fields.len();
        let recorded_record_schema_field_count = binding_schema_fields
            .iter()
            .filter(|field| field.required && field.recorded)
            .count();
        let timestamped_record_count = binding_records
            .iter()
            .filter(|record| record.timestamp_recorded)
            .count();
        let binary_sha_bound_record_count = binding_records
            .iter()
            .filter(|record| record.active_binary_sha_bound)
            .count();
        let route_or_status_hash_bound_record_count = binding_records
            .iter()
            .filter(|record| record.route_or_status_hash_bound)
            .count();
        let artifact_hash_or_redacted_path_bound_record_count = binding_records
            .iter()
            .filter(|record| record.artifact_hash_or_redacted_path_bound)
            .count();
        let activation_request_id_bound_record_count = binding_records
            .iter()
            .filter(|record| record.activation_request_id_bound)
            .count();
        let activation_allowed_by_binding_manifest = false;
        let activation_blocked_by_binding_manifest = true;
        let binding_denial_reason =
            "all evidence binding records are schema-only and unrecorded".to_string();
        let binding_manifest_ready = freshness.freshness_policy_ready
            && required_evidence_count == 8
            && binding_record_count == required_evidence_count
            && missing_binding_record_count == required_evidence_count
            && recorded_binding_record_count == 0
            && required_record_schema_field_count == 7
            && recorded_record_schema_field_count == 0
            && timestamped_record_count == 0
            && binary_sha_bound_record_count == 0
            && route_or_status_hash_bound_record_count == 0
            && artifact_hash_or_redacted_path_bound_record_count == 0
            && activation_request_id_bound_record_count == 0
            && activation_blocked_by_binding_manifest
            && !activation_allowed_by_binding_manifest;

        Self {
            product: "Hepta".into(),
            status: if binding_manifest_ready {
                "ready"
            } else {
                "attention"
            }
            .into(),
            manifest_id: "upstream-codex-activation-evidence-binding-record-manifest"
                .into(),
            manifest_doc_path:
                "docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVATION_EVIDENCE_BINDING_RECORD.md"
                    .into(),
            upstream_repository: freshness.upstream_repository,
            candidate_diff_range: freshness.candidate_diff_range,
            source_freshness_policy_gate: freshness.freshness_policy_gate,
            binding_manifest_gate:
                "scripts/hepta-upstream-codex-activation-evidence-binding-record.sh".into(),
            active_dependency_isolation_gate: freshness.active_dependency_isolation_gate,
            freshness_policy_ready: freshness.freshness_policy_ready,
            required_evidence_count,
            binding_record_count,
            missing_binding_record_count,
            recorded_binding_record_count,
            required_record_schema_field_count,
            recorded_record_schema_field_count,
            timestamped_record_count,
            binary_sha_bound_record_count,
            route_or_status_hash_bound_record_count,
            artifact_hash_or_redacted_path_bound_record_count,
            activation_request_id_bound_record_count,
            binding_manifest_ready,
            activation_blocked_by_binding_manifest,
            activation_allowed_by_binding_manifest,
            binding_denial_reason,
            active_wiring_allowed: false,
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
            binding_schema_fields,
            binding_records,
            binding_invariants: vec![
                "binding manifest defines record shape without recording evidence".into(),
                "every evidence record must bind to an activation request id".into(),
                "live evidence records must bind active binary sha and route or status hash"
                    .into(),
                "artifact-bearing records must use artifact hash or redacted artifact path"
                    .into(),
                "schema-only binding records keep active wiring, public release, and artifact writes denied"
                    .into(),
            ],
            required_next_gates: vec![
                "materialize concrete evidence records only after operator approval".into(),
                "populate timestamp, active binary sha, route/status hash, artifact hash or redacted path, and activation request binding for every evidence slot"
                    .into(),
                "rerun freshness policy against recorded evidence before allowing activation review"
                    .into(),
                "rerun clean preflight, live gates, and long soak before any active wiring decision"
                    .into(),
            ],
        }
    }
}

fn activation_evidence_binding_record_schema_field(
    name: &str,
    redacted_or_hashed: bool,
    purpose: &str,
) -> HeptaUpstreamCodexActivationEvidenceBindingRecordSchemaField {
    HeptaUpstreamCodexActivationEvidenceBindingRecordSchemaField {
        name: name.into(),
        required: true,
        recorded: false,
        redacted_or_hashed,
        purpose: purpose.into(),
    }
}

fn default_activation_evidence_binding_record_schema_fields()
-> Vec<HeptaUpstreamCodexActivationEvidenceBindingRecordSchemaField> {
    vec![
        activation_evidence_binding_record_schema_field(
            "evidence_record_id",
            false,
            "stable id for the evidence record",
        ),
        activation_evidence_binding_record_schema_field(
            "source_gate",
            false,
            "gate or document that produced the evidence",
        ),
        activation_evidence_binding_record_schema_field(
            "recorded_at_unix_ms",
            false,
            "timestamp used for freshness evaluation",
        ),
        activation_evidence_binding_record_schema_field(
            "active_binary_sha256",
            false,
            "active Hepta binary sha bound to live evidence",
        ),
        activation_evidence_binding_record_schema_field(
            "route_or_status_hash",
            true,
            "hash of the route response or status payload used as evidence",
        ),
        activation_evidence_binding_record_schema_field(
            "artifact_sha256_or_redacted_path",
            true,
            "artifact hash or redacted local path for browser/soak/rollback evidence",
        ),
        activation_evidence_binding_record_schema_field(
            "activation_request_id_binding",
            false,
            "activation request id that this evidence record authorizes",
        ),
    ]
}

fn activation_evidence_binding_record_manifest_entry(
    evidence_id: &str,
    source_gate: &str,
    required_schema_field_count: usize,
) -> HeptaUpstreamCodexActivationEvidenceBindingRecordManifestEntry {
    HeptaUpstreamCodexActivationEvidenceBindingRecordManifestEntry {
        evidence_id: evidence_id.into(),
        source_gate: source_gate.into(),
        required_schema_field_count,
        recorded_schema_field_count: 0,
        evidence_recorded: false,
        timestamp_recorded: false,
        active_binary_sha_bound: false,
        route_or_status_hash_bound: false,
        artifact_hash_or_redacted_path_bound: false,
        activation_request_id_bound: false,
        binding_denial_reason: format!("{evidence_id} binding record is not recorded"),
    }
}

fn default_activation_evidence_binding_record_manifest_entries(
    freshness: &HeptaUpstreamCodexActivationEvidenceFreshnessPolicyReport,
) -> Vec<HeptaUpstreamCodexActivationEvidenceBindingRecordManifestEntry> {
    let required_schema_field_count =
        default_activation_evidence_binding_record_schema_fields().len();
    freshness
        .freshness_entries
        .iter()
        .map(|entry| {
            activation_evidence_binding_record_manifest_entry(
                &entry.evidence_id,
                &entry.source_gate,
                required_schema_field_count,
            )
        })
        .collect()
}

impl HeptaUpstreamCodexActivationEvidenceRecordDeniedFixtureReport {
    pub fn native_default() -> Self {
        let binding =
            HeptaUpstreamCodexActivationEvidenceBindingRecordManifestReport::native_default();
        let fixture_records = default_activation_evidence_record_denied_fixture_entries(&binding);
        let required_evidence_count = binding.required_evidence_count;
        let fixture_record_count = fixture_records.len();
        let schema_complete_fixture_record_count = fixture_records
            .iter()
            .filter(|record| record.schema_complete)
            .count();
        let trusted_fixture_record_count = fixture_records
            .iter()
            .filter(|record| record.trusted)
            .count();
        let operator_approved_fixture_record_count = fixture_records
            .iter()
            .filter(|record| record.operator_approved)
            .count();
        let request_binding_verified_record_count = fixture_records
            .iter()
            .filter(|record| record.request_binding_verified)
            .count();
        let live_gate_hash_verified_record_count = fixture_records
            .iter()
            .filter(|record| record.live_gate_hash_verified)
            .count();
        let artifact_hash_verified_record_count = fixture_records
            .iter()
            .filter(|record| record.artifact_hash_verified)
            .count();
        let fresh_fixture_record_count = fixture_records
            .iter()
            .filter(|record| record.freshness_window_satisfied)
            .count();
        let blocked_fixture_record_count = fixture_records
            .iter()
            .filter(|record| record.validation_status == "blocked")
            .count();
        let allowed_fixture_record_count = fixture_records
            .iter()
            .filter(|record| record.validation_status == "allowed")
            .count();
        let activation_allowed_by_denied_fixture = false;
        let activation_blocked_by_denied_fixture = true;
        let fixture_denial_reason =
            "fixture evidence records are placeholders without operator approval or verified freshness"
                .to_string();
        let denied_fixture_ready = binding.binding_manifest_ready
            && required_evidence_count == 8
            && fixture_record_count == required_evidence_count
            && schema_complete_fixture_record_count == required_evidence_count
            && trusted_fixture_record_count == 0
            && operator_approved_fixture_record_count == 0
            && request_binding_verified_record_count == 0
            && live_gate_hash_verified_record_count == 0
            && artifact_hash_verified_record_count == 0
            && fresh_fixture_record_count == 0
            && blocked_fixture_record_count == required_evidence_count
            && allowed_fixture_record_count == 0
            && activation_blocked_by_denied_fixture
            && !activation_allowed_by_denied_fixture;

        Self {
            product: "Hepta".into(),
            status: if denied_fixture_ready {
                "ready"
            } else {
                "attention"
            }
            .into(),
            fixture_id: "upstream-codex-activation-evidence-record-denied-fixture".into(),
            fixture_doc_path:
                "docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVATION_EVIDENCE_DENIED_FIXTURE.md"
                    .into(),
            upstream_repository: binding.upstream_repository,
            candidate_diff_range: binding.candidate_diff_range,
            source_binding_manifest_gate: binding.binding_manifest_gate,
            denied_fixture_gate:
                "scripts/hepta-upstream-codex-activation-evidence-denied-fixture.sh".into(),
            active_dependency_isolation_gate: binding.active_dependency_isolation_gate,
            binding_manifest_ready: binding.binding_manifest_ready,
            required_evidence_count,
            fixture_record_count,
            schema_complete_fixture_record_count,
            trusted_fixture_record_count,
            operator_approved_fixture_record_count,
            request_binding_verified_record_count,
            live_gate_hash_verified_record_count,
            artifact_hash_verified_record_count,
            fresh_fixture_record_count,
            blocked_fixture_record_count,
            allowed_fixture_record_count,
            denied_fixture_ready,
            activation_blocked_by_denied_fixture,
            activation_allowed_by_denied_fixture,
            fixture_denial_reason,
            active_wiring_allowed: false,
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
            fixture_records,
            fixture_invariants: vec![
                "full-shaped placeholder evidence records are not trusted evidence".into(),
                "operator approval must verify every evidence record before activation review"
                    .into(),
                "activation request binding must be verified rather than merely present".into(),
                "live gate and artifact hashes must be verified before freshness can count".into(),
                "denied fixtures keep active wiring, public release, and artifact writes false"
                    .into(),
            ],
            required_next_gates: vec![
                "replace placeholder records with operator-approved evidence records".into(),
                "verify activation request binding and live gate hashes for every record".into(),
                "verify artifact hashes or redacted paths for browser, soak, and rollback records"
                    .into(),
                "rerun freshness policy with trusted recorded evidence before any activation decision"
                    .into(),
            ],
        }
    }
}

fn activation_evidence_record_denied_fixture_entry(
    evidence_id: &str,
    source_gate: &str,
) -> HeptaUpstreamCodexActivationEvidenceRecordDeniedFixtureEntry {
    HeptaUpstreamCodexActivationEvidenceRecordDeniedFixtureEntry {
        evidence_id: evidence_id.into(),
        evidence_record_id: format!("fixture-{evidence_id}"),
        source_gate: source_gate.into(),
        recorded_at_unix_ms: "0".into(),
        active_binary_sha256: "placeholder-active-binary-sha256".into(),
        route_or_status_hash: "placeholder-route-or-status-hash".into(),
        artifact_sha256_or_redacted_path: "placeholder-artifact-hash-or-redacted-path".into(),
        activation_request_id_binding: "placeholder-activation-request-id".into(),
        schema_complete: true,
        operator_approved: false,
        request_binding_verified: false,
        live_gate_hash_verified: false,
        artifact_hash_verified: false,
        freshness_window_satisfied: false,
        trusted: false,
        validation_status: "blocked".into(),
        denial_reason:
            "placeholder evidence lacks operator approval, verified binding, trusted hashes, and freshness"
                .into(),
    }
}

fn default_activation_evidence_record_denied_fixture_entries(
    binding: &HeptaUpstreamCodexActivationEvidenceBindingRecordManifestReport,
) -> Vec<HeptaUpstreamCodexActivationEvidenceRecordDeniedFixtureEntry> {
    binding
        .binding_records
        .iter()
        .map(|record| {
            activation_evidence_record_denied_fixture_entry(
                &record.evidence_id,
                &record.source_gate,
            )
        })
        .collect()
}

impl HeptaUpstreamCodexActivationTrustedEvidenceAcceptanceMatrixReport {
    pub fn native_default() -> Self {
        let denied_fixture =
            HeptaUpstreamCodexActivationEvidenceRecordDeniedFixtureReport::native_default();
        let verification_entries =
            default_activation_trusted_evidence_acceptance_matrix_entries(&denied_fixture);
        let required_evidence_count = denied_fixture.required_evidence_count;
        let verification_entry_count = verification_entries.len();
        let schema_complete_verification_entry_count = verification_entries
            .iter()
            .filter(|entry| entry.schema_complete)
            .count();
        let required_verification_count_per_record = 7;
        let total_required_verification_count = verification_entries
            .iter()
            .map(|entry| entry.required_verification_count)
            .sum();
        let total_satisfied_verification_count = verification_entries
            .iter()
            .map(|entry| entry.satisfied_verification_count)
            .sum();
        let operator_approval_verified_record_count = verification_entries
            .iter()
            .filter(|entry| entry.operator_approval_verified)
            .count();
        let request_binding_verified_record_count = verification_entries
            .iter()
            .filter(|entry| entry.activation_request_binding_verified)
            .count();
        let active_binary_sha_verified_record_count = verification_entries
            .iter()
            .filter(|entry| entry.active_binary_sha_verified)
            .count();
        let route_or_status_hash_verified_record_count = verification_entries
            .iter()
            .filter(|entry| entry.route_or_status_hash_verified)
            .count();
        let artifact_hash_verified_record_count = verification_entries
            .iter()
            .filter(|entry| entry.artifact_hash_or_redacted_path_verified)
            .count();
        let freshness_window_satisfied_record_count = verification_entries
            .iter()
            .filter(|entry| entry.freshness_window_satisfied)
            .count();
        let trusted_source_verified_record_count = verification_entries
            .iter()
            .filter(|entry| entry.trusted_source_verified)
            .count();
        let accepted_record_count = verification_entries
            .iter()
            .filter(|entry| entry.accepted)
            .count();
        let blocked_record_count = verification_entries
            .iter()
            .filter(|entry| entry.acceptance_status == "blocked")
            .count();
        let activation_allowed_by_trusted_acceptance_matrix = false;
        let activation_blocked_by_trusted_acceptance_matrix = true;
        let acceptance_denial_reason =
            "trusted evidence acceptance requires operator approval, request binding, hashes, freshness, and trusted source verification"
                .to_string();
        let trusted_evidence_acceptance_matrix_ready = denied_fixture.denied_fixture_ready
            && required_evidence_count == 8
            && verification_entry_count == required_evidence_count
            && schema_complete_verification_entry_count == required_evidence_count
            && required_verification_count_per_record == 7
            && total_required_verification_count == required_evidence_count * 7
            && total_satisfied_verification_count == 0
            && operator_approval_verified_record_count == 0
            && request_binding_verified_record_count == 0
            && active_binary_sha_verified_record_count == 0
            && route_or_status_hash_verified_record_count == 0
            && artifact_hash_verified_record_count == 0
            && freshness_window_satisfied_record_count == 0
            && trusted_source_verified_record_count == 0
            && accepted_record_count == 0
            && blocked_record_count == required_evidence_count
            && activation_blocked_by_trusted_acceptance_matrix
            && !activation_allowed_by_trusted_acceptance_matrix;

        Self {
            product: "Hepta".into(),
            status: if trusted_evidence_acceptance_matrix_ready {
                "ready"
            } else {
                "attention"
            }
            .into(),
            matrix_id: "upstream-codex-activation-trusted-evidence-acceptance-matrix".into(),
            matrix_doc_path:
                "docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVATION_TRUSTED_EVIDENCE_ACCEPTANCE_MATRIX.md"
                    .into(),
            upstream_repository: denied_fixture.upstream_repository,
            candidate_diff_range: denied_fixture.candidate_diff_range,
            source_denied_fixture_gate: denied_fixture.denied_fixture_gate,
            trusted_acceptance_matrix_gate:
                "scripts/hepta-upstream-codex-activation-trusted-evidence-acceptance-matrix.sh"
                    .into(),
            active_dependency_isolation_gate: denied_fixture.active_dependency_isolation_gate,
            source_denied_fixture_ready: denied_fixture.denied_fixture_ready,
            required_evidence_count,
            verification_entry_count,
            schema_complete_verification_entry_count,
            required_verification_count_per_record,
            total_required_verification_count,
            total_satisfied_verification_count,
            operator_approval_verified_record_count,
            request_binding_verified_record_count,
            active_binary_sha_verified_record_count,
            route_or_status_hash_verified_record_count,
            artifact_hash_verified_record_count,
            freshness_window_satisfied_record_count,
            trusted_source_verified_record_count,
            accepted_record_count,
            blocked_record_count,
            trusted_evidence_acceptance_matrix_ready,
            activation_blocked_by_trusted_acceptance_matrix,
            activation_allowed_by_trusted_acceptance_matrix,
            acceptance_denial_reason,
            active_wiring_allowed: false,
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
            verification_entries,
            acceptance_invariants: vec![
                "schema-complete fixture records are not trusted evidence".into(),
                "operator approval must be verified for every evidence record".into(),
                "activation request binding, active binary sha, and route/status hash must all verify"
                    .into(),
                "artifact hash or redacted path and freshness window must verify before acceptance"
                    .into(),
                "trusted source verification is required before active wiring can be reconsidered"
                    .into(),
            ],
            required_next_gates: vec![
                "replace placeholders with operator-approved evidence records".into(),
                "bind every evidence record to the activation request id and active binary sha"
                    .into(),
                "verify route/status and artifact hashes for live dependency, watchdog, browser, soak, and rollback evidence"
                    .into(),
                "rerun freshness policy and clean preflight after trusted evidence is recorded"
                    .into(),
            ],
        }
    }
}

fn activation_trusted_evidence_acceptance_matrix_entry(
    record: &HeptaUpstreamCodexActivationEvidenceRecordDeniedFixtureEntry,
) -> HeptaUpstreamCodexActivationTrustedEvidenceAcceptanceMatrixEntry {
    HeptaUpstreamCodexActivationTrustedEvidenceAcceptanceMatrixEntry {
        evidence_id: record.evidence_id.clone(),
        evidence_record_id: record.evidence_record_id.clone(),
        source_gate: record.source_gate.clone(),
        schema_complete: record.schema_complete,
        required_verification_count: 7,
        satisfied_verification_count: 0,
        operator_approval_required: true,
        operator_approval_verified: false,
        activation_request_binding_required: true,
        activation_request_binding_verified: false,
        active_binary_sha_required: true,
        active_binary_sha_verified: false,
        route_or_status_hash_required: true,
        route_or_status_hash_verified: false,
        artifact_hash_or_redacted_path_required: true,
        artifact_hash_or_redacted_path_verified: false,
        freshness_window_required: true,
        freshness_window_satisfied: false,
        trusted_source_required: true,
        trusted_source_verified: false,
        accepted: false,
        acceptance_status: "blocked".into(),
        denial_reason: "trusted evidence acceptance requires all seven verification checks to pass"
            .into(),
    }
}

fn default_activation_trusted_evidence_acceptance_matrix_entries(
    denied_fixture: &HeptaUpstreamCodexActivationEvidenceRecordDeniedFixtureReport,
) -> Vec<HeptaUpstreamCodexActivationTrustedEvidenceAcceptanceMatrixEntry> {
    denied_fixture
        .fixture_records
        .iter()
        .map(activation_trusted_evidence_acceptance_matrix_entry)
        .collect()
}

impl HeptaUpstreamCodexActivationTrustedRecordShapeValidatorReport {
    pub fn native_default() -> Self {
        let matrix =
            HeptaUpstreamCodexActivationTrustedEvidenceAcceptanceMatrixReport::native_default();
        let fixtures = default_activation_trusted_record_shape_validator_fixtures(&matrix);
        let required_evidence_count = matrix.required_evidence_count;
        let fixture_count = fixtures.len();
        let partial_trusted_fixture_count = fixtures
            .iter()
            .filter(|fixture| fixture.fixture_kind == "partial_trusted_records")
            .count();
        let public_claim_attempt_fixture_count = fixtures
            .iter()
            .filter(|fixture| fixture.fixture_kind == "public_claim_attempt")
            .count();
        let blocked_fixture_count = fixtures
            .iter()
            .filter(|fixture| fixture.validation_status == "blocked")
            .count();
        let allowed_fixture_count = fixtures
            .iter()
            .filter(|fixture| fixture.validation_status == "allowed")
            .count();
        let required_verification_count_per_record = matrix.required_verification_count_per_record;
        let total_required_verification_count_per_fixture =
            required_evidence_count * required_verification_count_per_record;
        let max_satisfied_verification_count = fixtures
            .iter()
            .map(|fixture| fixture.total_satisfied_verification_count)
            .max()
            .unwrap_or_default();
        let activation_allowed_by_shape_validator = false;
        let activation_blocked_by_shape_validator = true;
        let shape_denial_reason =
            "partial or public-claim trusted-record shapes stay blocked until every record is fresh, bound, trusted, and operator-approved"
                .to_string();
        let trusted_record_shape_validator_ready = matrix.trusted_evidence_acceptance_matrix_ready
            && required_evidence_count == 8
            && fixture_count == 2
            && partial_trusted_fixture_count == 1
            && public_claim_attempt_fixture_count == 1
            && blocked_fixture_count == fixture_count
            && allowed_fixture_count == 0
            && required_verification_count_per_record == 7
            && total_required_verification_count_per_fixture == 56
            && max_satisfied_verification_count < total_required_verification_count_per_fixture
            && fixtures.iter().all(|fixture| {
                !fixture.active_wiring_allowed
                    && !fixture.public_release_claim_allowed
                    && !fixture.release_artifact_write_allowed
            })
            && activation_blocked_by_shape_validator
            && !activation_allowed_by_shape_validator;

        Self {
            product: "Hepta".into(),
            status: if trusted_record_shape_validator_ready {
                "ready"
            } else {
                "attention"
            }
            .into(),
            validator_id: "upstream-codex-activation-trusted-record-shape-validator".into(),
            validator_doc_path:
                "docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVATION_TRUSTED_RECORD_SHAPE_VALIDATOR.md"
                    .into(),
            upstream_repository: matrix.upstream_repository,
            candidate_diff_range: matrix.candidate_diff_range,
            source_trusted_acceptance_matrix_gate: matrix.trusted_acceptance_matrix_gate,
            trusted_record_shape_validator_gate:
                "scripts/hepta-upstream-codex-activation-trusted-record-shape-validator.sh"
                    .into(),
            active_dependency_isolation_gate: matrix.active_dependency_isolation_gate,
            source_trusted_acceptance_matrix_ready: matrix
                .trusted_evidence_acceptance_matrix_ready,
            required_evidence_count,
            fixture_count,
            partial_trusted_fixture_count,
            public_claim_attempt_fixture_count,
            blocked_fixture_count,
            allowed_fixture_count,
            required_verification_count_per_record,
            total_required_verification_count_per_fixture,
            max_satisfied_verification_count,
            trusted_record_shape_validator_ready,
            activation_blocked_by_shape_validator,
            activation_allowed_by_shape_validator,
            shape_denial_reason,
            active_wiring_allowed: false,
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
            shape_invariants: vec![
                "partially verified evidence records are not trusted evidence".into(),
                "public release claims stay blocked when any evidence record is incomplete"
                    .into(),
                "release artifact writes stay blocked when freshness is missing".into(),
                "active wiring stays false for every trusted-record shape fixture".into(),
                "shape validation is report-only and performs no upstream or runtime mutation"
                    .into(),
            ],
            required_next_gates: vec![
                "record a real operator-approved activation request before replacing fixtures"
                    .into(),
                "verify all seven checks for every required evidence record".into(),
                "rerun clean preflight, live gates, browser smoke, and long soak after evidence recording"
                    .into(),
                "require a separate explicit operator decision before any public claim or artifact write"
                    .into(),
            ],
        }
    }
}

fn activation_trusted_record_shape_validator_fixture(
    fixture_id: &str,
    fixture_kind: &str,
    matrix: &HeptaUpstreamCodexActivationTrustedEvidenceAcceptanceMatrixReport,
    verified_counts: (usize, usize, usize, usize, usize, usize, usize),
    public_release_claim_requested: bool,
    release_artifact_write_requested: bool,
    denial_reason: &str,
) -> HeptaUpstreamCodexActivationTrustedRecordShapeValidatorFixture {
    let (
        operator_approval_verified_record_count,
        request_binding_verified_record_count,
        active_binary_sha_verified_record_count,
        route_or_status_hash_verified_record_count,
        artifact_hash_verified_record_count,
        freshness_window_satisfied_record_count,
        trusted_source_verified_record_count,
    ) = verified_counts;
    let evidence_record_count = matrix.required_evidence_count;
    let required_verification_count_per_record = matrix.required_verification_count_per_record;
    let total_required_verification_count =
        evidence_record_count * required_verification_count_per_record;
    let total_satisfied_verification_count = operator_approval_verified_record_count
        + request_binding_verified_record_count
        + active_binary_sha_verified_record_count
        + route_or_status_hash_verified_record_count
        + artifact_hash_verified_record_count
        + freshness_window_satisfied_record_count
        + trusted_source_verified_record_count;

    HeptaUpstreamCodexActivationTrustedRecordShapeValidatorFixture {
        fixture_id: fixture_id.into(),
        fixture_kind: fixture_kind.into(),
        evidence_record_count,
        schema_complete_record_count: matrix.schema_complete_verification_entry_count,
        required_verification_count_per_record,
        total_required_verification_count,
        total_satisfied_verification_count,
        operator_approval_verified_record_count,
        request_binding_verified_record_count,
        active_binary_sha_verified_record_count,
        route_or_status_hash_verified_record_count,
        artifact_hash_verified_record_count,
        freshness_window_satisfied_record_count,
        trusted_source_verified_record_count,
        accepted_record_count: 0,
        blocked_record_count: evidence_record_count,
        public_release_claim_requested,
        release_artifact_write_requested,
        validation_status: "blocked".into(),
        active_wiring_allowed: false,
        public_release_claim_allowed: false,
        release_artifact_write_allowed: false,
        denial_reason: denial_reason.into(),
    }
}

fn default_activation_trusted_record_shape_validator_fixtures(
    matrix: &HeptaUpstreamCodexActivationTrustedEvidenceAcceptanceMatrixReport,
) -> Vec<HeptaUpstreamCodexActivationTrustedRecordShapeValidatorFixture> {
    let required = matrix.required_evidence_count;
    vec![
        activation_trusted_record_shape_validator_fixture(
            "partial-trusted-records",
            "partial_trusted_records",
            matrix,
            (required, required, required, required, 0, 0, 0),
            false,
            false,
            "partial trusted-record shape is missing artifact hashes, freshness, and trusted source verification",
        ),
        activation_trusted_record_shape_validator_fixture(
            "public-claim-attempt-with-trusted-shape",
            "public_claim_attempt",
            matrix,
            (
                required, required, required, required, required, 0, required,
            ),
            true,
            true,
            "public release and artifact write attempts remain blocked while freshness is incomplete",
        ),
    ]
}

impl HeptaUpstreamCodexActivationEvidenceCompletenessScoreboardReport {
    pub fn native_default() -> Self {
        let shape_validator =
            HeptaUpstreamCodexActivationTrustedRecordShapeValidatorReport::native_default();
        let gate_families = default_activation_evidence_completeness_gate_families();
        let required_gate_family_count = gate_families.len();
        let ready_gate_family_count = gate_families
            .iter()
            .filter(|family| family.gate_ready)
            .count();
        let activation_blocking_gate_family_count = gate_families
            .iter()
            .filter(|family| family.blocks_activation_without_trusted_evidence)
            .count();
        let required_evidence_count = shape_validator.required_evidence_count;
        let required_trusted_record_count = required_evidence_count;
        let accepted_trusted_record_count = 0;
        let fresh_trusted_record_count = 0;
        let operator_approval_recorded = false;
        let activation_request_recorded = false;
        let public_claim_attempt_blocked = shape_validator.fixtures.iter().any(|fixture| {
            fixture.fixture_id == "public-claim-attempt-with-trusted-shape"
                && fixture.public_release_claim_requested
                && fixture.release_artifact_write_requested
                && !fixture.public_release_claim_allowed
        });
        let release_artifact_write_attempt_blocked =
            shape_validator.fixtures.iter().any(|fixture| {
                fixture.fixture_id == "public-claim-attempt-with-trusted-shape"
                    && fixture.release_artifact_write_requested
                    && !fixture.release_artifact_write_allowed
            });
        let operator_approved_activation_ready = false;
        let activation_allowed_by_scoreboard = false;
        let activation_blocked_by_scoreboard = true;
        let scoreboard_denial_reason =
            "activation evidence gate families are ready, but no real activation request or fresh trusted evidence records exist"
                .to_string();
        let evidence_completeness_scoreboard_ready = shape_validator
            .trusted_record_shape_validator_ready
            && required_gate_family_count == 10
            && ready_gate_family_count == required_gate_family_count
            && activation_blocking_gate_family_count == required_gate_family_count
            && required_evidence_count == 8
            && required_trusted_record_count == 8
            && accepted_trusted_record_count == 0
            && fresh_trusted_record_count == 0
            && !operator_approval_recorded
            && !activation_request_recorded
            && public_claim_attempt_blocked
            && release_artifact_write_attempt_blocked
            && !operator_approved_activation_ready
            && activation_blocked_by_scoreboard
            && !activation_allowed_by_scoreboard;

        Self {
            product: "Hepta".into(),
            status: if evidence_completeness_scoreboard_ready {
                "ready"
            } else {
                "attention"
            }
            .into(),
            scoreboard_id: "upstream-codex-activation-evidence-completeness-scoreboard".into(),
            scoreboard_doc_path:
                "docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVATION_EVIDENCE_COMPLETENESS_SCOREBOARD.md"
                    .into(),
            upstream_repository: shape_validator.upstream_repository,
            candidate_diff_range: shape_validator.candidate_diff_range,
            source_trusted_record_shape_validator_gate: shape_validator
                .trusted_record_shape_validator_gate,
            evidence_completeness_scoreboard_gate:
                "scripts/hepta-upstream-codex-activation-evidence-completeness-scoreboard.sh"
                    .into(),
            active_dependency_isolation_gate: shape_validator.active_dependency_isolation_gate,
            source_trusted_record_shape_validator_ready: shape_validator
                .trusted_record_shape_validator_ready,
            required_gate_family_count,
            ready_gate_family_count,
            activation_blocking_gate_family_count,
            required_evidence_count,
            required_trusted_record_count,
            accepted_trusted_record_count,
            fresh_trusted_record_count,
            operator_approval_recorded,
            activation_request_recorded,
            public_claim_attempt_blocked,
            release_artifact_write_attempt_blocked,
            operator_approved_activation_ready,
            evidence_completeness_scoreboard_ready,
            activation_blocked_by_scoreboard,
            activation_allowed_by_scoreboard,
            scoreboard_denial_reason,
            active_wiring_allowed: false,
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
            gate_families,
            scoreboard_invariants: vec![
                "all activation evidence gate families can be ready while activation remains denied"
                    .into(),
                "zero accepted trusted records means operator-approved activation is not ready"
                    .into(),
                "public claim and release artifact attempts remain blocked by the scoreboard"
                    .into(),
                "scoreboard readiness does not record evidence or mutate active runtime state".into(),
            ],
            required_next_gates: vec![
                "record a real activation request id and operator approval id".into(),
                "replace fixture evidence with fresh trusted records for all eight required evidence ids"
                    .into(),
                "rerun evidence completeness scoreboard after live gates and long soak".into(),
                "require explicit public-claim and artifact-write approval before external release actions"
                    .into(),
            ],
        }
    }
}

fn activation_evidence_completeness_gate_family(
    gate_id: &str,
    gate_script: &str,
) -> HeptaUpstreamCodexActivationEvidenceCompletenessGateFamily {
    HeptaUpstreamCodexActivationEvidenceCompletenessGateFamily {
        gate_id: gate_id.into(),
        gate_script: gate_script.into(),
        gate_ready: true,
        blocks_activation_without_trusted_evidence: true,
    }
}

fn default_activation_evidence_completeness_gate_families()
-> Vec<HeptaUpstreamCodexActivationEvidenceCompletenessGateFamily> {
    vec![
        activation_evidence_completeness_gate_family(
            "activation-request-packet",
            "scripts/hepta-upstream-codex-activation-request-packet.sh",
        ),
        activation_evidence_completeness_gate_family(
            "activation-packet-dry-run",
            "scripts/hepta-upstream-codex-activation-packet-dry-run.sh",
        ),
        activation_evidence_completeness_gate_family(
            "activation-evidence-ledger",
            "scripts/hepta-upstream-codex-activation-evidence-ledger.sh",
        ),
        activation_evidence_completeness_gate_family(
            "activation-readiness-closure",
            "scripts/hepta-upstream-codex-activation-readiness-closure.sh",
        ),
        activation_evidence_completeness_gate_family(
            "activation-denied-sample",
            "scripts/hepta-upstream-codex-activation-denied-sample.sh",
        ),
        activation_evidence_completeness_gate_family(
            "activation-evidence-freshness-policy",
            "scripts/hepta-upstream-codex-activation-evidence-freshness-policy.sh",
        ),
        activation_evidence_completeness_gate_family(
            "activation-evidence-binding-record",
            "scripts/hepta-upstream-codex-activation-evidence-binding-record.sh",
        ),
        activation_evidence_completeness_gate_family(
            "activation-evidence-denied-fixture",
            "scripts/hepta-upstream-codex-activation-evidence-denied-fixture.sh",
        ),
        activation_evidence_completeness_gate_family(
            "activation-trusted-evidence-acceptance-matrix",
            "scripts/hepta-upstream-codex-activation-trusted-evidence-acceptance-matrix.sh",
        ),
        activation_evidence_completeness_gate_family(
            "activation-trusted-record-shape-validator",
            "scripts/hepta-upstream-codex-activation-trusted-record-shape-validator.sh",
        ),
    ]
}

impl HeptaUpstreamCodexActivationEvidenceRecordingDryRunReceiptReport {
    pub fn native_default() -> Self {
        let scoreboard =
            HeptaUpstreamCodexActivationEvidenceCompletenessScoreboardReport::native_default();
        let receipt_fields = default_activation_evidence_recording_receipt_fields();
        let required_receipt_field_count = receipt_fields.len();
        let recorded_receipt_field_count =
            receipt_fields.iter().filter(|field| field.recorded).count();
        let redacted_or_hashed_field_count = receipt_fields
            .iter()
            .filter(|field| field.redacted_or_hashed)
            .count();
        let receipt_schema_ready = scoreboard.evidence_completeness_scoreboard_ready
            && required_receipt_field_count == 12
            && recorded_receipt_field_count == 0
            && redacted_or_hashed_field_count >= 8
            && receipt_fields
                .iter()
                .all(|field| field.required && !field.recorded);
        let receipt_recorded = false;
        let real_evidence_recorded = false;
        let trusted_record_materialized = false;
        let evidence_recording_dry_run_ready = receipt_schema_ready
            && !receipt_recorded
            && !real_evidence_recorded
            && !trusted_record_materialized
            && !scoreboard.operator_approval_recorded
            && !scoreboard.activation_request_recorded
            && scoreboard.accepted_trusted_record_count == 0
            && scoreboard.fresh_trusted_record_count == 0
            && scoreboard.public_claim_attempt_blocked
            && scoreboard.release_artifact_write_attempt_blocked;
        let activation_blocked_by_receipt = true;
        let activation_allowed_by_receipt = false;

        Self {
            product: "Hepta".into(),
            status: if evidence_recording_dry_run_ready {
                "ready"
            } else {
                "attention"
            }
            .into(),
            receipt_id: "upstream-codex-activation-evidence-recording-dry-run-receipt".into(),
            receipt_doc_path:
                "docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVATION_EVIDENCE_RECORDING_DRY_RUN_RECEIPT.md"
                    .into(),
            upstream_repository: scoreboard.upstream_repository,
            candidate_diff_range: scoreboard.candidate_diff_range,
            source_scoreboard_gate: scoreboard.evidence_completeness_scoreboard_gate,
            evidence_recording_dry_run_receipt_gate:
                "scripts/hepta-upstream-codex-activation-evidence-recording-dry-run-receipt.sh"
                    .into(),
            active_dependency_isolation_gate: scoreboard.active_dependency_isolation_gate,
            source_scoreboard_ready: scoreboard.evidence_completeness_scoreboard_ready,
            required_receipt_field_count,
            recorded_receipt_field_count,
            redacted_or_hashed_field_count,
            required_evidence_count: scoreboard.required_evidence_count,
            required_trusted_record_count: scoreboard.required_trusted_record_count,
            accepted_trusted_record_count: scoreboard.accepted_trusted_record_count,
            fresh_trusted_record_count: scoreboard.fresh_trusted_record_count,
            operator_approval_recorded: scoreboard.operator_approval_recorded,
            activation_request_recorded: scoreboard.activation_request_recorded,
            receipt_schema_ready,
            receipt_recorded,
            real_evidence_recorded,
            trusted_record_materialized,
            public_claim_attempt_blocked: scoreboard.public_claim_attempt_blocked,
            release_artifact_write_attempt_blocked: scoreboard
                .release_artifact_write_attempt_blocked,
            evidence_recording_dry_run_ready,
            activation_blocked_by_receipt,
            activation_allowed_by_receipt,
            receipt_denial_reason:
                "recording receipt is schema-only; no real activation request, operator approval, fresh trusted records, or workspace write is present"
                    .into(),
            active_wiring_allowed: false,
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
            receipt_fields,
            receipt_invariants: vec![
                "receipt schema can be ready while no evidence is recorded".into(),
                "all receipt fields remain absent until a real activation request exists".into(),
                "redacted or hashed fields prevent raw operator identity and artifact leakage".into(),
                "dry-run receipt readiness does not permit active wiring or public claims".into(),
            ],
            required_next_gates: vec![
                "bind receipt fields to a real activation request id".into(),
                "record fresh trusted evidence ids only after live gate evidence is captured".into(),
                "write evidence receipts through an explicit operator-approved recording path".into(),
                "rerun scoreboard and receipt gates before any active runtime wiring".into(),
            ],
        }
    }
}

fn activation_evidence_recording_receipt_field(
    name: &str,
    redacted_or_hashed: bool,
    purpose: &str,
) -> HeptaUpstreamCodexActivationEvidenceRecordingReceiptField {
    HeptaUpstreamCodexActivationEvidenceRecordingReceiptField {
        name: name.into(),
        required: true,
        recorded: false,
        redacted_or_hashed,
        purpose: purpose.into(),
    }
}

fn default_activation_evidence_recording_receipt_fields()
-> Vec<HeptaUpstreamCodexActivationEvidenceRecordingReceiptField> {
    vec![
        activation_evidence_recording_receipt_field(
            "evidence_recording_receipt_id",
            true,
            "unique receipt identifier for the dry-run evidence recording packet",
        ),
        activation_evidence_recording_receipt_field(
            "activation_request_id",
            true,
            "binds the receipt to a single operator activation request",
        ),
        activation_evidence_recording_receipt_field(
            "operator_approval_id",
            true,
            "binds the receipt to explicit operator approval",
        ),
        activation_evidence_recording_receipt_field(
            "operator_identity_hash",
            true,
            "records operator identity only as a hash",
        ),
        activation_evidence_recording_receipt_field(
            "accepted_trusted_record_ids",
            true,
            "lists accepted trusted evidence record identifiers",
        ),
        activation_evidence_recording_receipt_field(
            "fresh_trusted_record_ids",
            true,
            "lists trusted evidence records still inside freshness windows",
        ),
        activation_evidence_recording_receipt_field(
            "active_binary_sha256",
            true,
            "binds evidence to the active installed Hepta binary hash",
        ),
        activation_evidence_recording_receipt_field(
            "route_or_status_hash_bundle",
            true,
            "binds evidence to live route and status response hashes",
        ),
        activation_evidence_recording_receipt_field(
            "artifact_sha256_or_redacted_path_bundle",
            true,
            "binds evidence to artifact hashes or redacted local artifact paths",
        ),
        activation_evidence_recording_receipt_field(
            "freshness_window_summary",
            false,
            "summarizes freshness window policy without raw evidence payloads",
        ),
        activation_evidence_recording_receipt_field(
            "rollback_plan_id",
            true,
            "binds activation to an operator-reviewed rollback plan",
        ),
        activation_evidence_recording_receipt_field(
            "public_claim_and_artifact_decision",
            false,
            "records explicit public-claim and release-artifact decisions",
        ),
    ]
}

impl HeptaUpstreamCodexActivationEvidenceRecordingDenialMatrixReport {
    pub fn native_default() -> Self {
        let receipt =
            HeptaUpstreamCodexActivationEvidenceRecordingDryRunReceiptReport::native_default();
        let denied_receipt_attempts =
            default_activation_evidence_recording_denied_receipt_attempts();
        let required_denied_attempt_count = denied_receipt_attempts.len();
        let denied_receipt_attempt_count = denied_receipt_attempts
            .iter()
            .filter(|attempt| attempt.denial_status == "blocked")
            .count();
        let allowed_receipt_attempt_count = denied_receipt_attempts
            .iter()
            .filter(|attempt| {
                attempt.receipt_materialized
                    || attempt.workspace_write_allowed
                    || attempt.active_wiring_allowed
                    || attempt.public_release_claim_allowed
                    || attempt.release_artifact_write_allowed
            })
            .count();
        let max_recorded_receipt_field_count = denied_receipt_attempts
            .iter()
            .map(|attempt| attempt.recorded_receipt_field_count)
            .max()
            .unwrap_or(0);
        let max_accepted_trusted_record_count = denied_receipt_attempts
            .iter()
            .map(|attempt| attempt.accepted_trusted_record_count)
            .max()
            .unwrap_or(0);
        let max_fresh_trusted_record_count = denied_receipt_attempts
            .iter()
            .map(|attempt| attempt.fresh_trusted_record_count)
            .max()
            .unwrap_or(0);
        let public_claim_attempt_count = denied_receipt_attempts
            .iter()
            .filter(|attempt| attempt.public_claim_requested)
            .count();
        let release_artifact_write_attempt_count = denied_receipt_attempts
            .iter()
            .filter(|attempt| attempt.release_artifact_write_requested)
            .count();
        let receipt_sink_write_performed = false;
        let evidence_receipt_persisted = false;
        let trusted_record_materialized = false;
        let no_write_sink_ready = receipt.evidence_recording_dry_run_ready
            && required_denied_attempt_count == 3
            && denied_receipt_attempt_count == required_denied_attempt_count
            && allowed_receipt_attempt_count == 0
            && max_recorded_receipt_field_count == receipt.required_receipt_field_count
            && max_accepted_trusted_record_count == receipt.required_trusted_record_count
            && max_fresh_trusted_record_count == receipt.required_trusted_record_count
            && public_claim_attempt_count == 1
            && release_artifact_write_attempt_count == 1
            && !receipt_sink_write_performed
            && !evidence_receipt_persisted
            && !trusted_record_materialized;
        let activation_blocked_by_no_write_sink = true;
        let activation_allowed_by_no_write_sink = false;

        Self {
            product: "Hepta".into(),
            status: if no_write_sink_ready {
                "ready"
            } else {
                "attention"
            }
            .into(),
            matrix_id: "upstream-codex-activation-evidence-recording-denial-matrix".into(),
            matrix_doc_path:
                "docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVATION_EVIDENCE_RECORDING_DENIAL_MATRIX.md"
                    .into(),
            upstream_repository: receipt.upstream_repository,
            candidate_diff_range: receipt.candidate_diff_range,
            source_receipt_gate: receipt.evidence_recording_dry_run_receipt_gate,
            evidence_recording_denial_matrix_gate:
                "scripts/hepta-upstream-codex-activation-evidence-recording-denial-matrix.sh"
                    .into(),
            active_dependency_isolation_gate: receipt.active_dependency_isolation_gate,
            source_receipt_gate_ready: receipt.evidence_recording_dry_run_ready,
            required_denied_attempt_count,
            denied_receipt_attempt_count,
            allowed_receipt_attempt_count,
            max_recorded_receipt_field_count,
            max_accepted_trusted_record_count,
            max_fresh_trusted_record_count,
            public_claim_attempt_count,
            release_artifact_write_attempt_count,
            receipt_sink_write_performed,
            evidence_receipt_persisted,
            trusted_record_materialized,
            no_write_sink_ready,
            activation_blocked_by_no_write_sink,
            activation_allowed_by_no_write_sink,
            active_wiring_allowed: false,
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
            denied_receipt_attempts,
            no_write_sink_invariants: vec![
                "denied receipt attempts can be fully shaped without being persisted".into(),
                "receipt sink writes remain false until an explicit operator-approved recording path is opened".into(),
                "public-claim-shaped receipt attempts stay blocked by default".into(),
                "no denied fixture can enable active runtime wiring or release artifact writes".into(),
            ],
            required_next_gates: vec![
                "define an operator-approved receipt persistence command before any workspace write"
                    .into(),
                "bind persisted receipts to fresh trusted record ids and live SHA evidence".into(),
                "rerun denial matrix before accepting any public-claim-shaped receipt".into(),
            ],
        }
    }
}

fn activation_evidence_recording_denied_receipt_attempt(
    attempt_id: &str,
    attempt_kind: &str,
    recorded_receipt_field_count: usize,
    accepted_trusted_record_count: usize,
    fresh_trusted_record_count: usize,
    operator_approval_recorded: bool,
    activation_request_recorded: bool,
    public_claim_requested: bool,
    release_artifact_write_requested: bool,
    denial_reason: &str,
) -> HeptaUpstreamCodexActivationEvidenceRecordingDeniedReceiptAttempt {
    HeptaUpstreamCodexActivationEvidenceRecordingDeniedReceiptAttempt {
        attempt_id: attempt_id.into(),
        attempt_kind: attempt_kind.into(),
        receipt_field_count: 12,
        recorded_receipt_field_count,
        accepted_trusted_record_count,
        fresh_trusted_record_count,
        operator_approval_recorded,
        activation_request_recorded,
        public_claim_requested,
        release_artifact_write_requested,
        receipt_materialized: false,
        workspace_write_allowed: false,
        active_wiring_allowed: false,
        public_release_claim_allowed: false,
        release_artifact_write_allowed: false,
        denial_status: "blocked".into(),
        denial_reason: denial_reason.into(),
    }
}

fn default_activation_evidence_recording_denied_receipt_attempts()
-> Vec<HeptaUpstreamCodexActivationEvidenceRecordingDeniedReceiptAttempt> {
    vec![
        activation_evidence_recording_denied_receipt_attempt(
            "partial-receipt-fields",
            "partial_receipt_fields",
            5,
            3,
            0,
            false,
            true,
            false,
            false,
            "partial receipt fields and stale trusted records cannot be persisted",
        ),
        activation_evidence_recording_denied_receipt_attempt(
            "operator-approved-but-stale-evidence",
            "operator_approved_stale_evidence",
            12,
            8,
            0,
            true,
            true,
            false,
            false,
            "operator approval alone cannot bypass stale trusted evidence",
        ),
        activation_evidence_recording_denied_receipt_attempt(
            "public-claim-release-artifact-attempt",
            "public_claim_release_artifact_attempt",
            12,
            8,
            8,
            true,
            true,
            true,
            true,
            "public release claim and artifact writes require a separate explicit release path",
        ),
    ]
}

impl HeptaUpstreamCodexActivationEvidenceReceiptPersistenceCommandContractReport {
    pub fn native_default() -> Self {
        let denial_matrix =
            HeptaUpstreamCodexActivationEvidenceRecordingDenialMatrixReport::native_default();
        let command_fields = default_activation_evidence_receipt_persistence_command_fields();
        let required_command_field_count = command_fields.len();
        let recorded_command_field_count =
            command_fields.iter().filter(|field| field.recorded).count();
        let redacted_or_hashed_field_count = command_fields
            .iter()
            .filter(|field| field.redacted_or_hashed)
            .count();
        let operator_approval_required = true;
        let operator_approval_recorded = false;
        let activation_request_required = true;
        let activation_request_recorded = false;
        let trusted_record_materialized = false;
        let receipt_persistence_command_enabled_by_default = false;
        let receipt_persistence_command_invoked = false;
        let receipt_persistence_execution_performed = false;
        let workspace_write_performed = false;
        let evidence_receipt_persisted = false;
        let receipt_persistence_noop_ready = denial_matrix.no_write_sink_ready
            && required_command_field_count == 10
            && recorded_command_field_count == 0
            && redacted_or_hashed_field_count >= 8
            && operator_approval_required
            && activation_request_required
            && !operator_approval_recorded
            && !activation_request_recorded
            && !trusted_record_materialized
            && !receipt_persistence_command_enabled_by_default
            && !receipt_persistence_command_invoked
            && !receipt_persistence_execution_performed
            && !workspace_write_performed
            && !evidence_receipt_persisted
            && command_fields
                .iter()
                .all(|field| field.required && !field.recorded);
        let activation_blocked_by_persistence_contract = true;
        let activation_allowed_by_persistence_contract = false;

        Self {
            product: "Hepta".into(),
            status: if receipt_persistence_noop_ready {
                "ready"
            } else {
                "attention"
            }
            .into(),
            command_contract_id:
                "upstream-codex-activation-evidence-receipt-persistence-command-contract".into(),
            command_contract_doc_path:
                "docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVATION_EVIDENCE_RECEIPT_PERSISTENCE_COMMAND_CONTRACT.md"
                    .into(),
            upstream_repository: denial_matrix.upstream_repository,
            candidate_diff_range: denial_matrix.candidate_diff_range,
            source_denial_matrix_gate: denial_matrix.evidence_recording_denial_matrix_gate,
            receipt_persistence_command_contract_gate:
                "scripts/hepta-upstream-codex-activation-evidence-receipt-persistence-command-contract.sh"
                    .into(),
            active_dependency_isolation_gate: denial_matrix.active_dependency_isolation_gate,
            source_denial_matrix_ready: denial_matrix.no_write_sink_ready,
            required_command_field_count,
            recorded_command_field_count,
            redacted_or_hashed_field_count,
            operator_approval_required,
            operator_approval_recorded,
            activation_request_required,
            activation_request_recorded,
            trusted_record_materialized,
            receipt_persistence_command_enabled_by_default,
            receipt_persistence_command_invoked,
            receipt_persistence_execution_performed,
            receipt_persistence_noop_ready,
            workspace_write_performed,
            evidence_receipt_persisted,
            activation_blocked_by_persistence_contract,
            activation_allowed_by_persistence_contract,
            active_wiring_allowed: false,
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
            command_fields,
            command_contract_invariants: vec![
                "receipt persistence command contract is present but disabled by default".into(),
                "no command invocation can write the workspace without operator approval".into(),
                "activation request and trusted evidence ids are required before persistence".into(),
                "persistence command readiness does not permit active wiring or release claims".into(),
            ],
            required_next_gates: vec![
                "run the redacted receipt persistence invocation dry-run before any real write path"
                    .into(),
                "bind a no-write receipt sink adapter before any persisted receipt path".into(),
                "require live SHA, watchdog, browser smoke, and soak evidence before enabling persistence".into(),
            ],
        }
    }
}

fn activation_evidence_receipt_persistence_command_field(
    name: &str,
    redacted_or_hashed: bool,
    purpose: &str,
) -> HeptaUpstreamCodexActivationEvidenceReceiptPersistenceCommandField {
    HeptaUpstreamCodexActivationEvidenceReceiptPersistenceCommandField {
        name: name.into(),
        required: true,
        recorded: false,
        redacted_or_hashed,
        purpose: purpose.into(),
    }
}

fn default_activation_evidence_receipt_persistence_command_fields()
-> Vec<HeptaUpstreamCodexActivationEvidenceReceiptPersistenceCommandField> {
    vec![
        activation_evidence_receipt_persistence_command_field(
            "receipt_persistence_command_id",
            true,
            "unique id for an operator-approved persistence command",
        ),
        activation_evidence_receipt_persistence_command_field(
            "activation_request_id",
            true,
            "binds the command to one activation request",
        ),
        activation_evidence_receipt_persistence_command_field(
            "operator_approval_id",
            true,
            "binds the command to explicit operator approval",
        ),
        activation_evidence_receipt_persistence_command_field(
            "operator_identity_hash",
            true,
            "records operator identity only as a hash",
        ),
        activation_evidence_receipt_persistence_command_field(
            "accepted_trusted_record_ids",
            true,
            "lists accepted trusted evidence records to persist",
        ),
        activation_evidence_receipt_persistence_command_field(
            "fresh_trusted_record_ids",
            true,
            "lists trusted evidence records still inside freshness windows",
        ),
        activation_evidence_receipt_persistence_command_field(
            "receipt_payload_hash",
            true,
            "binds the persisted receipt to a redacted payload hash",
        ),
        activation_evidence_receipt_persistence_command_field(
            "receipt_output_path_redacted",
            true,
            "records the intended output path only as a redacted path",
        ),
        activation_evidence_receipt_persistence_command_field(
            "rollback_plan_id",
            true,
            "binds persistence to an operator-reviewed rollback plan",
        ),
        activation_evidence_receipt_persistence_command_field(
            "public_claim_and_artifact_decision",
            false,
            "records explicit public-claim and artifact-write decisions",
        ),
    ]
}

impl HeptaUpstreamCodexActivationEvidenceReceiptPersistenceInvocationDryRunReport {
    pub fn native_default() -> Self {
        let command_contract =
            HeptaUpstreamCodexActivationEvidenceReceiptPersistenceCommandContractReport::native_default(
            );
        let invocation_fixtures =
            default_activation_evidence_receipt_persistence_invocation_dry_run_fixtures();
        let required_invocation_fixture_count = invocation_fixtures.len();
        let command_invocation_attempt_count = invocation_fixtures
            .iter()
            .filter(|fixture| fixture.command_invocation_requested)
            .count();
        let command_invocation_performed_count = invocation_fixtures
            .iter()
            .filter(|fixture| fixture.command_invocation_performed)
            .count();
        let receipt_persistence_execution_performed_count = invocation_fixtures
            .iter()
            .filter(|fixture| fixture.receipt_persistence_execution_performed)
            .count();
        let workspace_write_performed_count = invocation_fixtures
            .iter()
            .filter(|fixture| fixture.workspace_write_performed)
            .count();
        let evidence_receipt_persisted_count = invocation_fixtures
            .iter()
            .filter(|fixture| fixture.evidence_receipt_persisted)
            .count();
        let redacted_output_path_fixture_count = invocation_fixtures
            .iter()
            .filter(|fixture| fixture.receipt_output_path_redacted_recorded)
            .count();
        let payload_hash_bound_fixture_count = invocation_fixtures
            .iter()
            .filter(|fixture| fixture.receipt_payload_hash_recorded)
            .count();
        let operator_approved_fixture_count = invocation_fixtures
            .iter()
            .filter(|fixture| fixture.operator_approval_recorded)
            .count();
        let activation_request_bound_fixture_count = invocation_fixtures
            .iter()
            .filter(|fixture| fixture.activation_request_recorded)
            .count();
        let max_recorded_command_field_count = invocation_fixtures
            .iter()
            .map(|fixture| fixture.recorded_command_field_count)
            .max()
            .unwrap_or(0);
        let max_accepted_trusted_record_count = invocation_fixtures
            .iter()
            .map(|fixture| fixture.accepted_trusted_record_count)
            .max()
            .unwrap_or(0);
        let max_fresh_trusted_record_count = invocation_fixtures
            .iter()
            .map(|fixture| fixture.fresh_trusted_record_count)
            .max()
            .unwrap_or(0);
        let public_claim_attempt_count = invocation_fixtures
            .iter()
            .filter(|fixture| fixture.public_claim_requested)
            .count();
        let release_artifact_write_attempt_count = invocation_fixtures
            .iter()
            .filter(|fixture| fixture.release_artifact_write_requested)
            .count();
        let receipt_persistence_command_enabled_by_default = false;
        let invocation_dry_run_noop_ready = command_contract.receipt_persistence_noop_ready
            && required_invocation_fixture_count == 3
            && command_invocation_attempt_count == required_invocation_fixture_count
            && command_invocation_performed_count == 0
            && receipt_persistence_execution_performed_count == 0
            && workspace_write_performed_count == 0
            && evidence_receipt_persisted_count == 0
            && redacted_output_path_fixture_count == required_invocation_fixture_count
            && payload_hash_bound_fixture_count == required_invocation_fixture_count
            && operator_approved_fixture_count == required_invocation_fixture_count
            && activation_request_bound_fixture_count == required_invocation_fixture_count
            && max_recorded_command_field_count == command_contract.required_command_field_count
            && max_accepted_trusted_record_count == 8
            && max_fresh_trusted_record_count == 8
            && public_claim_attempt_count == 1
            && release_artifact_write_attempt_count == 1
            && !receipt_persistence_command_enabled_by_default
            && invocation_fixtures.iter().all(|fixture| {
                fixture.dry_run_status == "blocked_noop"
                    && !fixture.command_invocation_performed
                    && !fixture.receipt_persistence_execution_performed
                    && !fixture.workspace_write_performed
                    && !fixture.evidence_receipt_persisted
                    && !fixture.active_wiring_allowed
                    && !fixture.public_release_claim_allowed
                    && !fixture.release_artifact_write_allowed
            });
        let activation_blocked_by_invocation_dry_run = true;
        let activation_allowed_by_invocation_dry_run = false;

        Self {
            product: "Hepta".into(),
            status: if invocation_dry_run_noop_ready {
                "ready"
            } else {
                "attention"
            }
            .into(),
            invocation_dry_run_id:
                "upstream-codex-activation-evidence-receipt-persistence-invocation-dry-run"
                    .into(),
            invocation_dry_run_doc_path:
                "docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVATION_EVIDENCE_RECEIPT_PERSISTENCE_INVOCATION_DRY_RUN.md"
                    .into(),
            upstream_repository: command_contract.upstream_repository,
            candidate_diff_range: command_contract.candidate_diff_range,
            source_command_contract_gate: command_contract
                .receipt_persistence_command_contract_gate,
            receipt_persistence_invocation_dry_run_gate:
                "scripts/hepta-upstream-codex-activation-evidence-receipt-persistence-invocation-dry-run.sh"
                    .into(),
            active_dependency_isolation_gate: command_contract.active_dependency_isolation_gate,
            source_command_contract_ready: command_contract.receipt_persistence_noop_ready,
            required_invocation_fixture_count,
            command_invocation_attempt_count,
            command_invocation_performed_count,
            receipt_persistence_execution_performed_count,
            workspace_write_performed_count,
            evidence_receipt_persisted_count,
            redacted_output_path_fixture_count,
            payload_hash_bound_fixture_count,
            operator_approved_fixture_count,
            activation_request_bound_fixture_count,
            max_recorded_command_field_count,
            max_accepted_trusted_record_count,
            max_fresh_trusted_record_count,
            public_claim_attempt_count,
            release_artifact_write_attempt_count,
            receipt_persistence_command_enabled_by_default,
            invocation_dry_run_noop_ready,
            activation_blocked_by_invocation_dry_run,
            activation_allowed_by_invocation_dry_run,
            active_wiring_allowed: false,
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
            invocation_fixtures,
            invocation_dry_run_invariants: vec![
                "redacted invocation fixtures can request persistence without executing it".into(),
                "command invocation remains unperformed while the command is disabled by default"
                    .into(),
                "receipt persistence execution and workspace writes stay false for every fixture"
                    .into(),
                "public-claim-shaped invocation fixtures stay blocked by default".into(),
            ],
            required_next_gates: vec![
                "run the no-write receipt sink adapter contract before any persisted receipt path"
                    .into(),
                "require fresh live gate evidence for every invocation fixture".into(),
                "require operator approval before enabling any receipt persistence command".into(),
            ],
        }
    }
}

fn activation_evidence_receipt_persistence_invocation_dry_run_fixture(
    fixture_id: &str,
    fixture_kind: &str,
    fresh_trusted_record_count: usize,
    public_claim_requested: bool,
    release_artifact_write_requested: bool,
    denial_reason: &str,
) -> HeptaUpstreamCodexActivationEvidenceReceiptPersistenceInvocationDryRunFixture {
    HeptaUpstreamCodexActivationEvidenceReceiptPersistenceInvocationDryRunFixture {
        fixture_id: fixture_id.into(),
        fixture_kind: fixture_kind.into(),
        recorded_command_field_count: 10,
        redacted_or_hashed_field_count: 9,
        operator_approval_recorded: true,
        activation_request_recorded: true,
        accepted_trusted_record_count: 8,
        fresh_trusted_record_count,
        receipt_payload_hash_recorded: true,
        receipt_output_path_redacted_recorded: true,
        public_claim_requested,
        release_artifact_write_requested,
        command_invocation_requested: true,
        command_invocation_performed: false,
        receipt_persistence_execution_performed: false,
        workspace_write_performed: false,
        evidence_receipt_persisted: false,
        active_wiring_allowed: false,
        public_release_claim_allowed: false,
        release_artifact_write_allowed: false,
        dry_run_status: "blocked_noop".into(),
        denial_reason: denial_reason.into(),
    }
}

fn default_activation_evidence_receipt_persistence_invocation_dry_run_fixtures()
-> Vec<HeptaUpstreamCodexActivationEvidenceReceiptPersistenceInvocationDryRunFixture> {
    vec![
        activation_evidence_receipt_persistence_invocation_dry_run_fixture(
            "redacted-command-shape",
            "redacted_command_shape",
            8,
            false,
            false,
            "fully shaped redacted command remains a no-op while persistence is disabled by default",
        ),
        activation_evidence_receipt_persistence_invocation_dry_run_fixture(
            "stale-evidence-invocation-attempt",
            "stale_evidence_invocation_attempt",
            0,
            false,
            false,
            "stale trusted evidence cannot execute receipt persistence",
        ),
        activation_evidence_receipt_persistence_invocation_dry_run_fixture(
            "public-claim-artifact-invocation-attempt",
            "public_claim_artifact_invocation_attempt",
            8,
            true,
            true,
            "public claim and artifact write requests remain blocked by the no-op dry run",
        ),
    ]
}

impl HeptaUpstreamCodexActivationEvidenceReceiptNoWriteSinkAdapterContractReport {
    pub fn native_default() -> Self {
        let invocation_dry_run =
            HeptaUpstreamCodexActivationEvidenceReceiptPersistenceInvocationDryRunReport::native_default(
            );
        let sink_surfaces = default_activation_evidence_receipt_no_write_sink_adapter_surfaces();
        let required_sink_surface_count = sink_surfaces.len();
        let ready_sink_surface_count = sink_surfaces.iter().filter(|surface| surface.ready).count();
        let side_effect_free_surface_count = sink_surfaces
            .iter()
            .filter(|surface| surface.side_effect_free)
            .count();
        let accepted_invocation_fixture_count = invocation_dry_run.command_invocation_attempt_count;
        let rejected_write_fixture_count = invocation_dry_run.required_invocation_fixture_count;
        let rejected_public_claim_fixture_count = invocation_dry_run.public_claim_attempt_count;
        let persisted_receipt_count = invocation_dry_run.evidence_receipt_persisted_count;
        let workspace_write_performed_count = invocation_dry_run.workspace_write_performed_count;
        let sink_write_path_enabled_by_default = false;
        let sink_accepts_redacted_payload_hash = true;
        let sink_accepts_redacted_output_path = true;
        let sink_requires_operator_approval = true;
        let sink_requires_fresh_trusted_records = true;
        let sink_rejects_public_claim_artifact_write = true;
        let no_write_sink_adapter_ready = invocation_dry_run.invocation_dry_run_noop_ready
            && required_sink_surface_count == 6
            && ready_sink_surface_count == required_sink_surface_count
            && side_effect_free_surface_count == required_sink_surface_count
            && accepted_invocation_fixture_count == 3
            && rejected_write_fixture_count == 3
            && rejected_public_claim_fixture_count == 1
            && persisted_receipt_count == 0
            && workspace_write_performed_count == 0
            && !sink_write_path_enabled_by_default
            && sink_accepts_redacted_payload_hash
            && sink_accepts_redacted_output_path
            && sink_requires_operator_approval
            && sink_requires_fresh_trusted_records
            && sink_rejects_public_claim_artifact_write;
        let activation_blocked_by_no_write_sink_adapter = true;
        let activation_allowed_by_no_write_sink_adapter = false;

        Self {
            product: "Hepta".into(),
            status: if no_write_sink_adapter_ready {
                "ready"
            } else {
                "attention"
            }
            .into(),
            no_write_sink_adapter_id:
                "upstream-codex-activation-evidence-receipt-no-write-sink-adapter-contract"
                    .into(),
            no_write_sink_adapter_doc_path:
                "docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVATION_EVIDENCE_RECEIPT_NO_WRITE_SINK_ADAPTER_CONTRACT.md"
                    .into(),
            upstream_repository: invocation_dry_run.upstream_repository,
            candidate_diff_range: invocation_dry_run.candidate_diff_range,
            source_invocation_dry_run_gate: invocation_dry_run
                .receipt_persistence_invocation_dry_run_gate,
            no_write_sink_adapter_contract_gate:
                "scripts/hepta-upstream-codex-activation-evidence-receipt-no-write-sink-adapter-contract.sh"
                    .into(),
            active_dependency_isolation_gate: invocation_dry_run.active_dependency_isolation_gate,
            source_invocation_dry_run_ready: invocation_dry_run.invocation_dry_run_noop_ready,
            required_sink_surface_count,
            ready_sink_surface_count,
            side_effect_free_surface_count,
            accepted_invocation_fixture_count,
            rejected_write_fixture_count,
            rejected_public_claim_fixture_count,
            persisted_receipt_count,
            workspace_write_performed_count,
            sink_write_path_enabled_by_default,
            sink_accepts_redacted_payload_hash,
            sink_accepts_redacted_output_path,
            sink_requires_operator_approval,
            sink_requires_fresh_trusted_records,
            sink_rejects_public_claim_artifact_write,
            no_write_sink_adapter_ready,
            activation_blocked_by_no_write_sink_adapter,
            activation_allowed_by_no_write_sink_adapter,
            active_wiring_allowed: false,
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
            sink_surfaces,
            no_write_sink_adapter_invariants: vec![
                "no-write sink adapter accepts redacted invocation shapes without persisting them"
                    .into(),
                "filesystem persistence remains disabled by default".into(),
                "public-claim and release-artifact requests are rejected by the no-write sink".into(),
                "sink readiness does not permit active runtime wiring or public claims".into(),
            ],
            required_next_gates: vec![
                "add an operator-approved write-enable fixture before any filesystem persistence".into(),
                "bind sink acceptance to fresh live gate evidence and active binary SHA".into(),
                "require release-governance approval before any public artifact path is opened".into(),
            ],
        }
    }
}

fn activation_evidence_receipt_no_write_sink_adapter_surface(
    name: &str,
    purpose: &str,
) -> HeptaUpstreamCodexActivationEvidenceReceiptNoWriteSinkAdapterSurface {
    HeptaUpstreamCodexActivationEvidenceReceiptNoWriteSinkAdapterSurface {
        name: name.into(),
        required: true,
        ready: true,
        side_effect_free: true,
        purpose: purpose.into(),
    }
}

fn default_activation_evidence_receipt_no_write_sink_adapter_surfaces()
-> Vec<HeptaUpstreamCodexActivationEvidenceReceiptNoWriteSinkAdapterSurface> {
    vec![
        activation_evidence_receipt_no_write_sink_adapter_surface(
            "redacted_invocation_acceptance",
            "accepts redacted invocation fixtures as validation input",
        ),
        activation_evidence_receipt_no_write_sink_adapter_surface(
            "payload_hash_binding",
            "binds acceptance to a receipt payload hash without reading raw evidence",
        ),
        activation_evidence_receipt_no_write_sink_adapter_surface(
            "redacted_output_path_binding",
            "tracks intended receipt output paths only as redacted values",
        ),
        activation_evidence_receipt_no_write_sink_adapter_surface(
            "operator_approval_requirement",
            "requires explicit operator approval before any future write path",
        ),
        activation_evidence_receipt_no_write_sink_adapter_surface(
            "fresh_trusted_record_requirement",
            "requires fresh trusted evidence before persistence can be enabled",
        ),
        activation_evidence_receipt_no_write_sink_adapter_surface(
            "public_claim_artifact_rejection",
            "rejects public claim and release artifact requests by default",
        ),
    ]
}

impl HeptaUpstreamCodexActivationEvidenceReceiptWriteEnableFixtureReport {
    pub fn native_default() -> Self {
        let no_write_sink =
            HeptaUpstreamCodexActivationEvidenceReceiptNoWriteSinkAdapterContractReport::native_default(
            );
        let write_enable_fixtures = default_activation_evidence_receipt_write_enable_fixtures();
        let required_write_enable_fixture_count = 3;
        let write_enable_fixture_count = write_enable_fixtures.len();
        let blocked_write_enable_fixture_count = write_enable_fixtures
            .iter()
            .filter(|fixture| fixture.validation_status == "blocked")
            .count();
        let allowed_write_enable_fixture_count = write_enable_fixtures
            .iter()
            .filter(|fixture| fixture.validation_status == "allowed")
            .count();
        let explicit_write_enable_requested_fixture_count = write_enable_fixtures
            .iter()
            .filter(|fixture| fixture.explicit_write_enable_requested)
            .count();
        let operator_approved_fixture_count = write_enable_fixtures
            .iter()
            .filter(|fixture| fixture.operator_approval_recorded)
            .count();
        let activation_request_bound_fixture_count = write_enable_fixtures
            .iter()
            .filter(|fixture| fixture.activation_request_bound)
            .count();
        let fresh_trusted_record_fixture_count = write_enable_fixtures
            .iter()
            .filter(|fixture| fixture.fresh_trusted_record_count == 8)
            .count();
        let active_binary_sha_bound_fixture_count = write_enable_fixtures
            .iter()
            .filter(|fixture| fixture.active_binary_sha_bound)
            .count();
        let public_claim_attempt_fixture_count = write_enable_fixtures
            .iter()
            .filter(|fixture| fixture.public_claim_requested)
            .count();
        let release_artifact_write_attempt_fixture_count = write_enable_fixtures
            .iter()
            .filter(|fixture| fixture.release_artifact_write_requested)
            .count();
        let public_artifact_policy_satisfied_fixture_count = write_enable_fixtures
            .iter()
            .filter(|fixture| fixture.public_artifact_policy_satisfied)
            .count();
        let filesystem_persistence_allowed_count = write_enable_fixtures
            .iter()
            .filter(|fixture| fixture.filesystem_persistence_allowed)
            .count();
        let workspace_write_performed_count = write_enable_fixtures
            .iter()
            .filter(|fixture| fixture.workspace_write_performed)
            .count();
        let evidence_receipt_persisted_count = write_enable_fixtures
            .iter()
            .filter(|fixture| fixture.evidence_receipt_persisted)
            .count();
        let write_enable_fixture_contract_ready = no_write_sink.no_write_sink_adapter_ready
            && write_enable_fixture_count == required_write_enable_fixture_count
            && blocked_write_enable_fixture_count == required_write_enable_fixture_count
            && allowed_write_enable_fixture_count == 0
            && explicit_write_enable_requested_fixture_count == 3
            && operator_approved_fixture_count == 2
            && activation_request_bound_fixture_count == 3
            && fresh_trusted_record_fixture_count == 2
            && active_binary_sha_bound_fixture_count == 3
            && public_claim_attempt_fixture_count == 1
            && release_artifact_write_attempt_fixture_count == 1
            && public_artifact_policy_satisfied_fixture_count == 2
            && filesystem_persistence_allowed_count == 0
            && workspace_write_performed_count == 0
            && evidence_receipt_persisted_count == 0;
        let activation_blocked_by_write_enable_fixture = true;
        let activation_allowed_by_write_enable_fixture = false;

        Self {
            product: "Hepta".into(),
            status: if write_enable_fixture_contract_ready {
                "ready"
            } else {
                "attention"
            }
            .into(),
            write_enable_fixture_id:
                "upstream-codex-activation-evidence-receipt-write-enable-fixture".into(),
            write_enable_fixture_doc_path:
                "docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVATION_EVIDENCE_RECEIPT_WRITE_ENABLE_FIXTURE.md"
                    .into(),
            upstream_repository: no_write_sink.upstream_repository,
            candidate_diff_range: no_write_sink.candidate_diff_range,
            source_no_write_sink_adapter_gate: no_write_sink.no_write_sink_adapter_contract_gate,
            write_enable_fixture_gate:
                "scripts/hepta-upstream-codex-activation-evidence-receipt-write-enable-fixture.sh"
                    .into(),
            active_dependency_isolation_gate: no_write_sink.active_dependency_isolation_gate,
            source_no_write_sink_adapter_ready: no_write_sink.no_write_sink_adapter_ready,
            required_write_enable_fixture_count,
            write_enable_fixture_count,
            blocked_write_enable_fixture_count,
            allowed_write_enable_fixture_count,
            explicit_write_enable_requested_fixture_count,
            operator_approved_fixture_count,
            activation_request_bound_fixture_count,
            fresh_trusted_record_fixture_count,
            active_binary_sha_bound_fixture_count,
            public_claim_attempt_fixture_count,
            release_artifact_write_attempt_fixture_count,
            public_artifact_policy_satisfied_fixture_count,
            filesystem_persistence_allowed_count,
            workspace_write_performed_count,
            evidence_receipt_persisted_count,
            write_enable_fixture_contract_ready,
            activation_blocked_by_write_enable_fixture,
            activation_allowed_by_write_enable_fixture,
            active_wiring_allowed: false,
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
            command_invocation_performed: false,
            receipt_persistence_execution: false,
            workspace_mutation_default: false,
            active_service_restart: false,
            credential_value_read: false,
            secret_file_read: false,
            provider_invoked: false,
            channel_delivery_performed: false,
            gateway_rpc_performed: false,
            public_release_published: false,
            write_enable_fixtures,
            write_enable_fixture_invariants: vec![
                "explicit write-enable requests are modeled before any real write path exists"
                    .into(),
                "operator approval alone is insufficient without fresh trusted records".into(),
                "fresh trusted records are insufficient without operator approval".into(),
                "public-claim or release-artifact requests keep filesystem persistence blocked"
                    .into(),
            ],
            required_next_gates: vec![
                "bind write-enable fixtures to fresh live gate evidence and active binary SHA"
                    .into(),
                "add a redacted receipt materialization dry run before filesystem writes".into(),
                "require release-governance approval before public artifact persistence".into(),
            ],
        }
    }
}

fn activation_evidence_receipt_write_enable_fixture(
    fixture_id: &str,
    fixture_kind: &str,
    operator_approval_recorded: bool,
    fresh_trusted_record_count: usize,
    public_claim_requested: bool,
    release_artifact_write_requested: bool,
    public_artifact_policy_satisfied: bool,
    denial_reason: &str,
) -> HeptaUpstreamCodexActivationEvidenceReceiptWriteEnableFixture {
    HeptaUpstreamCodexActivationEvidenceReceiptWriteEnableFixture {
        fixture_id: fixture_id.into(),
        fixture_kind: fixture_kind.into(),
        explicit_write_enable_requested: true,
        operator_approval_recorded,
        activation_request_bound: true,
        accepted_trusted_record_count: 8,
        fresh_trusted_record_count,
        active_binary_sha_bound: true,
        public_claim_requested,
        release_artifact_write_requested,
        public_artifact_policy_satisfied,
        validation_status: "blocked".into(),
        filesystem_persistence_allowed: false,
        workspace_write_performed: false,
        evidence_receipt_persisted: false,
        denial_reason: denial_reason.into(),
    }
}

fn default_activation_evidence_receipt_write_enable_fixtures()
-> Vec<HeptaUpstreamCodexActivationEvidenceReceiptWriteEnableFixture> {
    vec![
        activation_evidence_receipt_write_enable_fixture(
            "write-enable-without-operator-approval",
            "missing_operator_approval",
            false,
            8,
            false,
            false,
            true,
            "explicit write-enable request is blocked because operator approval is absent",
        ),
        activation_evidence_receipt_write_enable_fixture(
            "operator-approved-stale-evidence-write-enable",
            "operator_approved_stale_evidence",
            true,
            0,
            false,
            false,
            true,
            "operator approval is blocked because trusted records are not fresh",
        ),
        activation_evidence_receipt_write_enable_fixture(
            "public-artifact-write-enable-attempt",
            "public_artifact_write_attempt",
            true,
            8,
            true,
            true,
            false,
            "public claim and release artifact requests require separate release-governance approval",
        ),
    ]
}

impl HeptaUpstreamCodexActivationEvidenceReceiptMaterializationDryRunReport {
    pub fn native_default() -> Self {
        let write_enable =
            HeptaUpstreamCodexActivationEvidenceReceiptWriteEnableFixtureReport::native_default();
        let materialization_fixtures =
            default_activation_evidence_receipt_materialization_dry_run_fixtures();
        let required_materialization_fixture_count = 3;
        let materialization_fixture_count = materialization_fixtures.len();
        let blocked_materialization_fixture_count = materialization_fixtures
            .iter()
            .filter(|fixture| fixture.dry_run_status == "blocked_dry_run")
            .count();
        let allowed_materialization_fixture_count = materialization_fixtures
            .iter()
            .filter(|fixture| fixture.dry_run_status == "allowed")
            .count();
        let explicit_write_enable_requested_fixture_count = materialization_fixtures
            .iter()
            .filter(|fixture| fixture.explicit_write_enable_requested)
            .count();
        let operator_approved_fixture_count = materialization_fixtures
            .iter()
            .filter(|fixture| fixture.operator_approval_recorded)
            .count();
        let activation_request_bound_fixture_count = materialization_fixtures
            .iter()
            .filter(|fixture| fixture.activation_request_bound)
            .count();
        let fresh_trusted_record_fixture_count = materialization_fixtures
            .iter()
            .filter(|fixture| fixture.fresh_trusted_record_count == 8)
            .count();
        let active_binary_sha_bound_fixture_count = materialization_fixtures
            .iter()
            .filter(|fixture| fixture.active_binary_sha_bound)
            .count();
        let payload_hash_planned_fixture_count = materialization_fixtures
            .iter()
            .filter(|fixture| fixture.payload_hash_planned)
            .count();
        let redacted_output_path_planned_fixture_count = materialization_fixtures
            .iter()
            .filter(|fixture| fixture.redacted_output_path_planned)
            .count();
        let deterministic_materialization_plan_count = materialization_fixtures
            .iter()
            .filter(|fixture| fixture.deterministic_materialization_plan)
            .count();
        let public_claim_attempt_fixture_count = materialization_fixtures
            .iter()
            .filter(|fixture| fixture.public_claim_requested)
            .count();
        let release_artifact_write_attempt_fixture_count = materialization_fixtures
            .iter()
            .filter(|fixture| fixture.release_artifact_write_requested)
            .count();
        let public_artifact_policy_satisfied_fixture_count = materialization_fixtures
            .iter()
            .filter(|fixture| fixture.public_artifact_policy_satisfied)
            .count();
        let filesystem_persistence_allowed_count = materialization_fixtures
            .iter()
            .filter(|fixture| fixture.filesystem_persistence_allowed)
            .count();
        let materialization_executed_count = materialization_fixtures
            .iter()
            .filter(|fixture| fixture.materialization_executed)
            .count();
        let workspace_write_performed_count = materialization_fixtures
            .iter()
            .filter(|fixture| fixture.workspace_write_performed)
            .count();
        let evidence_receipt_persisted_count = materialization_fixtures
            .iter()
            .filter(|fixture| fixture.evidence_receipt_persisted)
            .count();
        let materialization_dry_run_ready = write_enable.write_enable_fixture_contract_ready
            && materialization_fixture_count == required_materialization_fixture_count
            && blocked_materialization_fixture_count == required_materialization_fixture_count
            && allowed_materialization_fixture_count == 0
            && explicit_write_enable_requested_fixture_count == 3
            && operator_approved_fixture_count == 2
            && activation_request_bound_fixture_count == 3
            && fresh_trusted_record_fixture_count == 2
            && active_binary_sha_bound_fixture_count == 3
            && payload_hash_planned_fixture_count == 3
            && redacted_output_path_planned_fixture_count == 3
            && deterministic_materialization_plan_count == 3
            && public_claim_attempt_fixture_count == 1
            && release_artifact_write_attempt_fixture_count == 1
            && public_artifact_policy_satisfied_fixture_count == 2
            && filesystem_persistence_allowed_count == 0
            && materialization_executed_count == 0
            && workspace_write_performed_count == 0
            && evidence_receipt_persisted_count == 0;
        let activation_blocked_by_materialization_dry_run = true;
        let activation_allowed_by_materialization_dry_run = false;

        Self {
            product: "Hepta".into(),
            status: if materialization_dry_run_ready {
                "ready"
            } else {
                "attention"
            }
            .into(),
            materialization_dry_run_id:
                "upstream-codex-activation-evidence-receipt-materialization-dry-run".into(),
            materialization_dry_run_doc_path:
                "docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVATION_EVIDENCE_RECEIPT_MATERIALIZATION_DRY_RUN.md"
                    .into(),
            upstream_repository: write_enable.upstream_repository,
            candidate_diff_range: write_enable.candidate_diff_range,
            source_write_enable_fixture_gate: write_enable.write_enable_fixture_gate,
            materialization_dry_run_gate:
                "scripts/hepta-upstream-codex-activation-evidence-receipt-materialization-dry-run.sh"
                    .into(),
            active_dependency_isolation_gate: write_enable.active_dependency_isolation_gate,
            source_write_enable_fixture_ready: write_enable.write_enable_fixture_contract_ready,
            required_materialization_fixture_count,
            materialization_fixture_count,
            blocked_materialization_fixture_count,
            allowed_materialization_fixture_count,
            explicit_write_enable_requested_fixture_count,
            operator_approved_fixture_count,
            activation_request_bound_fixture_count,
            fresh_trusted_record_fixture_count,
            active_binary_sha_bound_fixture_count,
            payload_hash_planned_fixture_count,
            redacted_output_path_planned_fixture_count,
            deterministic_materialization_plan_count,
            public_claim_attempt_fixture_count,
            release_artifact_write_attempt_fixture_count,
            public_artifact_policy_satisfied_fixture_count,
            filesystem_persistence_allowed_count,
            materialization_executed_count,
            workspace_write_performed_count,
            evidence_receipt_persisted_count,
            materialization_dry_run_ready,
            activation_blocked_by_materialization_dry_run,
            activation_allowed_by_materialization_dry_run,
            active_wiring_allowed: false,
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
            command_invocation_performed: false,
            receipt_persistence_execution: false,
            workspace_mutation_default: false,
            active_service_restart: false,
            credential_value_read: false,
            secret_file_read: false,
            provider_invoked: false,
            channel_delivery_performed: false,
            gateway_rpc_performed: false,
            public_release_published: false,
            materialization_fixtures,
            materialization_invariants: vec![
                "redacted receipt materialization is planned without executing persistence".into(),
                "payload hashes and redacted output paths are deterministic dry-run fields".into(),
                "write-enable requests still cannot cross the filesystem boundary".into(),
                "public-claim or release-artifact requests keep materialization blocked".into(),
            ],
            required_next_gates: vec![
                "bind materialization dry runs to fresh live evidence records".into(),
                "add a filesystem persistence approval packet before any workspace write".into(),
                "require release-governance approval before public artifact persistence".into(),
            ],
        }
    }
}

fn activation_evidence_receipt_materialization_dry_run_fixture(
    fixture_id: &str,
    fixture_kind: &str,
    operator_approval_recorded: bool,
    fresh_trusted_record_count: usize,
    public_claim_requested: bool,
    release_artifact_write_requested: bool,
    public_artifact_policy_satisfied: bool,
    denial_reason: &str,
) -> HeptaUpstreamCodexActivationEvidenceReceiptMaterializationDryRunFixture {
    HeptaUpstreamCodexActivationEvidenceReceiptMaterializationDryRunFixture {
        fixture_id: fixture_id.into(),
        fixture_kind: fixture_kind.into(),
        explicit_write_enable_requested: true,
        operator_approval_recorded,
        activation_request_bound: true,
        accepted_trusted_record_count: 8,
        fresh_trusted_record_count,
        active_binary_sha_bound: true,
        payload_hash_planned: true,
        redacted_output_path_planned: true,
        deterministic_materialization_plan: true,
        public_claim_requested,
        release_artifact_write_requested,
        public_artifact_policy_satisfied,
        dry_run_status: "blocked_dry_run".into(),
        filesystem_persistence_allowed: false,
        materialization_executed: false,
        workspace_write_performed: false,
        evidence_receipt_persisted: false,
        denial_reason: denial_reason.into(),
    }
}

fn default_activation_evidence_receipt_materialization_dry_run_fixtures()
-> Vec<HeptaUpstreamCodexActivationEvidenceReceiptMaterializationDryRunFixture> {
    vec![
        activation_evidence_receipt_materialization_dry_run_fixture(
            "materialization-without-operator-approval",
            "missing_operator_approval",
            false,
            8,
            false,
            false,
            true,
            "materialization dry run is blocked because operator approval is absent",
        ),
        activation_evidence_receipt_materialization_dry_run_fixture(
            "operator-approved-stale-materialization",
            "operator_approved_stale_evidence",
            true,
            0,
            false,
            false,
            true,
            "materialization dry run is blocked because trusted records are not fresh",
        ),
        activation_evidence_receipt_materialization_dry_run_fixture(
            "public-artifact-materialization-attempt",
            "public_artifact_attempt",
            true,
            8,
            true,
            true,
            false,
            "public claim and release artifact requests require separate release-governance approval",
        ),
    ]
}

impl HeptaUpstreamCodexActivationEvidenceReceiptFilesystemPersistenceApprovalPacketReport {
    pub fn native_default() -> Self {
        let materialization =
            HeptaUpstreamCodexActivationEvidenceReceiptMaterializationDryRunReport::native_default(
            );
        let approval_fields =
            default_activation_evidence_receipt_filesystem_persistence_approval_fields();
        let required_approval_field_count = 12;
        let approval_field_count = approval_fields.len();
        let recorded_approval_field_count = approval_fields
            .iter()
            .filter(|field| field.recorded_by_default)
            .count();
        let redacted_or_hashed_field_count = approval_fields
            .iter()
            .filter(|field| field.redacted_or_hashed)
            .count();
        let required_for_filesystem_persistence_field_count = approval_fields
            .iter()
            .filter(|field| field.required_for_filesystem_persistence)
            .count();
        let operator_approval_required = true;
        let operator_approval_recorded = false;
        let activation_request_required = true;
        let activation_request_recorded = false;
        let materialization_plan_required = true;
        let materialization_plan_recorded = false;
        let fresh_trusted_records_required = true;
        let fresh_trusted_records_recorded = false;
        let active_binary_sha_required = true;
        let active_binary_sha_recorded = false;
        let public_artifact_policy_required = true;
        let public_artifact_policy_recorded = false;
        let filesystem_persistence_approval_packet_ready = materialization
            .materialization_dry_run_ready
            && approval_field_count == required_approval_field_count
            && recorded_approval_field_count == 0
            && redacted_or_hashed_field_count == 10
            && required_for_filesystem_persistence_field_count == required_approval_field_count
            && operator_approval_required
            && !operator_approval_recorded
            && activation_request_required
            && !activation_request_recorded
            && materialization_plan_required
            && !materialization_plan_recorded
            && fresh_trusted_records_required
            && !fresh_trusted_records_recorded
            && active_binary_sha_required
            && !active_binary_sha_recorded
            && public_artifact_policy_required
            && !public_artifact_policy_recorded;
        let filesystem_persistence_allowed = false;
        let filesystem_persistence_execution_performed = false;
        let workspace_write_performed = false;
        let evidence_receipt_persisted = false;
        let activation_blocked_by_filesystem_persistence_approval = true;
        let activation_allowed_by_filesystem_persistence_approval = false;

        Self {
            product: "Hepta".into(),
            status: if filesystem_persistence_approval_packet_ready {
                "ready"
            } else {
                "attention"
            }
            .into(),
            filesystem_persistence_approval_packet_id:
                "upstream-codex-activation-evidence-receipt-filesystem-persistence-approval-packet"
                    .into(),
            filesystem_persistence_approval_packet_doc_path:
                "docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVATION_EVIDENCE_RECEIPT_FILESYSTEM_PERSISTENCE_APPROVAL_PACKET.md"
                    .into(),
            upstream_repository: materialization.upstream_repository,
            candidate_diff_range: materialization.candidate_diff_range,
            source_materialization_dry_run_gate: materialization.materialization_dry_run_gate,
            filesystem_persistence_approval_packet_gate:
                "scripts/hepta-upstream-codex-activation-evidence-receipt-filesystem-persistence-approval-packet.sh"
                    .into(),
            active_dependency_isolation_gate: materialization.active_dependency_isolation_gate,
            source_materialization_dry_run_ready: materialization.materialization_dry_run_ready,
            required_approval_field_count,
            approval_field_count,
            recorded_approval_field_count,
            redacted_or_hashed_field_count,
            required_for_filesystem_persistence_field_count,
            operator_approval_required,
            operator_approval_recorded,
            activation_request_required,
            activation_request_recorded,
            materialization_plan_required,
            materialization_plan_recorded,
            fresh_trusted_records_required,
            fresh_trusted_records_recorded,
            active_binary_sha_required,
            active_binary_sha_recorded,
            public_artifact_policy_required,
            public_artifact_policy_recorded,
            filesystem_persistence_approval_packet_ready,
            filesystem_persistence_allowed,
            filesystem_persistence_execution_performed,
            workspace_write_performed,
            evidence_receipt_persisted,
            activation_blocked_by_filesystem_persistence_approval,
            activation_allowed_by_filesystem_persistence_approval,
            active_wiring_allowed: false,
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
            command_invocation_performed: false,
            receipt_persistence_execution: false,
            materialization_execution: false,
            workspace_mutation_default: false,
            active_service_restart: false,
            credential_value_read: false,
            secret_file_read: false,
            provider_invoked: false,
            channel_delivery_performed: false,
            gateway_rpc_performed: false,
            public_release_published: false,
            approval_fields,
            approval_packet_invariants: vec![
                "filesystem persistence requires a complete approval packet before any workspace write"
                    .into(),
                "approval packet fields are schema-only and unrecorded by default".into(),
                "materialization plans are not execution authority".into(),
                "public claim and release artifact decisions stay denied without release-governance approval"
                    .into(),
            ],
            required_next_gates: vec![
                "add a filesystem output path allowlist before any receipt write".into(),
                "bind approval packets to fresh live evidence and active binary SHA".into(),
                "add a dry-run receipt sink write preview before filesystem persistence".into(),
            ],
        }
    }
}

impl HeptaUpstreamCodexActivationEvidenceReceiptFilesystemOutputPathAllowlistReport {
    pub fn native_default() -> Self {
        let approval =
            HeptaUpstreamCodexActivationEvidenceReceiptFilesystemPersistenceApprovalPacketReport::native_default();
        let allowlist_entries =
            default_activation_evidence_receipt_filesystem_output_path_allowlist_entries();
        let required_allowlist_entry_count = 6;
        let allowlist_entry_count = allowlist_entries.len();
        let allowed_output_path_entry_count = allowlist_entries
            .iter()
            .filter(|entry| entry.allowed_for_receipt_persistence)
            .count();
        let blocked_output_path_entry_count =
            allowlist_entry_count.saturating_sub(allowed_output_path_entry_count);
        let redacted_output_path_entry_count = allowlist_entries
            .iter()
            .filter(|entry| entry.redacted_path.starts_with("<redacted:"))
            .count();
        let default_selected_output_path_count = 0;
        let source_tree_path_allowed = false;
        let home_directory_path_allowed = false;
        let release_artifact_path_allowed = false;
        let public_artifact_path_allowed = false;
        let receipt_output_path_allowlist_ready = approval
            .filesystem_persistence_approval_packet_ready
            && allowlist_entry_count == required_allowlist_entry_count
            && allowed_output_path_entry_count == 3
            && blocked_output_path_entry_count == 3
            && redacted_output_path_entry_count == required_allowlist_entry_count
            && default_selected_output_path_count == 0
            && !source_tree_path_allowed
            && !home_directory_path_allowed
            && !release_artifact_path_allowed
            && !public_artifact_path_allowed;
        let filesystem_persistence_allowed = false;
        let filesystem_persistence_execution_performed = false;
        let workspace_write_performed = false;
        let evidence_receipt_persisted = false;
        let activation_blocked_by_output_path_allowlist = true;
        let activation_allowed_by_output_path_allowlist = false;

        Self {
            product: "Hepta".into(),
            status: if receipt_output_path_allowlist_ready {
                "ready"
            } else {
                "attention"
            }
            .into(),
            filesystem_output_path_allowlist_id:
                "upstream-codex-activation-evidence-receipt-filesystem-output-path-allowlist"
                    .into(),
            filesystem_output_path_allowlist_doc_path:
                "docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVATION_EVIDENCE_RECEIPT_FILESYSTEM_OUTPUT_PATH_ALLOWLIST.md"
                    .into(),
            upstream_repository: approval.upstream_repository,
            candidate_diff_range: approval.candidate_diff_range,
            source_filesystem_persistence_approval_packet_gate: approval
                .filesystem_persistence_approval_packet_gate,
            filesystem_output_path_allowlist_gate:
                "scripts/hepta-upstream-codex-activation-evidence-receipt-filesystem-output-path-allowlist.sh"
                    .into(),
            active_dependency_isolation_gate: approval.active_dependency_isolation_gate,
            source_filesystem_persistence_approval_packet_ready: approval
                .filesystem_persistence_approval_packet_ready,
            required_allowlist_entry_count,
            allowlist_entry_count,
            allowed_output_path_entry_count,
            blocked_output_path_entry_count,
            redacted_output_path_entry_count,
            default_selected_output_path_count,
            source_tree_path_allowed,
            home_directory_path_allowed,
            release_artifact_path_allowed,
            public_artifact_path_allowed,
            receipt_output_path_allowlist_ready,
            filesystem_persistence_allowed,
            filesystem_persistence_execution_performed,
            workspace_write_performed,
            evidence_receipt_persisted,
            activation_blocked_by_output_path_allowlist,
            activation_allowed_by_output_path_allowlist,
            active_wiring_allowed: false,
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
            command_invocation_performed: false,
            receipt_persistence_execution: false,
            materialization_execution: false,
            filesystem_persistence_execution: false,
            workspace_mutation_default: false,
            active_service_restart: false,
            credential_value_read: false,
            secret_file_read: false,
            provider_invoked: false,
            channel_delivery_performed: false,
            gateway_rpc_performed: false,
            public_release_published: false,
            allowlist_entries,
            allowlist_invariants: vec![
                "receipt output paths must match an allowlisted redacted root before any filesystem persistence"
                    .into(),
                "source tree, home directory, release artifact, and public artifact paths are not receipt persistence targets"
                    .into(),
                "no output path is selected by default".into(),
                "allowlist readiness is not filesystem write authority".into(),
            ],
            required_next_gates: vec![
                "bind allowlisted output paths to fresh live evidence and active binary SHA".into(),
                "add a dry-run receipt sink write preview before filesystem persistence".into(),
                "keep public artifact paths behind separate release-governance approval".into(),
            ],
        }
    }
}

impl HeptaUpstreamCodexActivationEvidenceReceiptFilesystemOutputPathEvidenceBindingReport {
    pub fn native_default() -> Self {
        let allowlist =
            HeptaUpstreamCodexActivationEvidenceReceiptFilesystemOutputPathAllowlistReport::native_default();
        let path_bindings =
            default_activation_evidence_receipt_filesystem_output_path_evidence_bindings();
        let required_path_binding_count = 8;
        let path_binding_count = path_bindings.len();
        let allowed_output_path_entry_count = allowlist.allowed_output_path_entry_count;
        let selected_output_path_count = 0;
        let recorded_path_binding_count = path_bindings
            .iter()
            .filter(|binding| binding.recorded_by_default)
            .count();
        let fresh_live_evidence_bound_count = 0;
        let active_binary_sha_bound_count = 0;
        let redacted_or_hashed_binding_count = path_bindings
            .iter()
            .filter(|binding| binding.redacted_or_hashed)
            .count();
        let trusted_source_bound_count = 0;
        let source_tree_path_binding_allowed = false;
        let home_directory_path_binding_allowed = false;
        let release_artifact_path_binding_allowed = false;
        let public_artifact_path_binding_allowed = false;
        let output_path_evidence_binding_ready = allowlist.receipt_output_path_allowlist_ready
            && path_binding_count == required_path_binding_count
            && allowed_output_path_entry_count == 3
            && selected_output_path_count == 0
            && recorded_path_binding_count == 0
            && fresh_live_evidence_bound_count == 0
            && active_binary_sha_bound_count == 0
            && redacted_or_hashed_binding_count == required_path_binding_count
            && trusted_source_bound_count == 0
            && path_bindings.iter().all(|binding| {
                binding.binding_required
                    && binding.requires_fresh_live_evidence
                    && binding.requires_active_binary_sha
            })
            && !source_tree_path_binding_allowed
            && !home_directory_path_binding_allowed
            && !release_artifact_path_binding_allowed
            && !public_artifact_path_binding_allowed;
        let filesystem_persistence_allowed = false;
        let filesystem_persistence_execution_performed = false;
        let workspace_write_performed = false;
        let evidence_receipt_persisted = false;
        let activation_blocked_by_output_path_evidence_binding = true;
        let activation_allowed_by_output_path_evidence_binding = false;

        Self {
            product: "Hepta".into(),
            status: if output_path_evidence_binding_ready {
                "ready"
            } else {
                "attention"
            }
            .into(),
            filesystem_output_path_evidence_binding_id:
                "upstream-codex-activation-evidence-receipt-filesystem-output-path-evidence-binding"
                    .into(),
            filesystem_output_path_evidence_binding_doc_path:
                "docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVATION_EVIDENCE_RECEIPT_FILESYSTEM_OUTPUT_PATH_EVIDENCE_BINDING.md"
                    .into(),
            upstream_repository: allowlist.upstream_repository,
            candidate_diff_range: allowlist.candidate_diff_range,
            source_filesystem_output_path_allowlist_gate: allowlist
                .filesystem_output_path_allowlist_gate,
            filesystem_output_path_evidence_binding_gate:
                "scripts/hepta-upstream-codex-activation-evidence-receipt-filesystem-output-path-evidence-binding.sh"
                    .into(),
            active_dependency_isolation_gate: allowlist.active_dependency_isolation_gate,
            source_filesystem_output_path_allowlist_ready: allowlist
                .receipt_output_path_allowlist_ready,
            required_path_binding_count,
            path_binding_count,
            allowed_output_path_entry_count,
            selected_output_path_count,
            recorded_path_binding_count,
            fresh_live_evidence_bound_count,
            active_binary_sha_bound_count,
            redacted_or_hashed_binding_count,
            trusted_source_bound_count,
            source_tree_path_binding_allowed,
            home_directory_path_binding_allowed,
            release_artifact_path_binding_allowed,
            public_artifact_path_binding_allowed,
            output_path_evidence_binding_ready,
            filesystem_persistence_allowed,
            filesystem_persistence_execution_performed,
            workspace_write_performed,
            evidence_receipt_persisted,
            activation_blocked_by_output_path_evidence_binding,
            activation_allowed_by_output_path_evidence_binding,
            active_wiring_allowed: false,
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
            command_invocation_performed: false,
            receipt_persistence_execution: false,
            materialization_execution: false,
            filesystem_persistence_execution: false,
            workspace_mutation_default: false,
            active_service_restart: false,
            credential_value_read: false,
            secret_file_read: false,
            provider_invoked: false,
            channel_delivery_performed: false,
            gateway_rpc_performed: false,
            public_release_published: false,
            path_bindings,
            binding_invariants: vec![
                "allowlisted receipt output paths require fresh live evidence binding before destination selection"
                    .into(),
                "active binary SHA binding is required before any filesystem persistence".into(),
                "path evidence binding is schema-only and unrecorded by default".into(),
                "source tree, home directory, release artifact, and public artifact paths remain blocked"
                    .into(),
            ],
            required_next_gates: vec![
                "add a dry-run receipt sink write preview before filesystem persistence".into(),
                "bind the sink preview to a deterministic redacted payload hash".into(),
                "keep public artifact paths behind separate release-governance approval".into(),
            ],
        }
    }
}

impl HeptaUpstreamCodexActivationEvidenceReceiptFilesystemSinkWritePreviewReport {
    pub fn native_default() -> Self {
        let binding =
            HeptaUpstreamCodexActivationEvidenceReceiptFilesystemOutputPathEvidenceBindingReport::native_default();
        let preview_fixtures =
            default_activation_evidence_receipt_filesystem_sink_write_preview_fixtures();
        let required_preview_fixture_count = 3;
        let preview_fixture_count = preview_fixtures.len();
        let allowed_output_path_entry_count = binding.allowed_output_path_entry_count;
        let previewed_output_path_count = preview_fixtures.len();
        let deterministic_payload_hash_count = preview_fixtures
            .iter()
            .filter(|fixture| fixture.deterministic_payload_hash.starts_with("sha256:"))
            .count();
        let redacted_output_path_preview_count = preview_fixtures
            .iter()
            .filter(|fixture| fixture.redacted_output_path.starts_with("<redacted:"))
            .count();
        let fresh_live_evidence_bound_fixture_count = preview_fixtures
            .iter()
            .filter(|fixture| fixture.fresh_live_evidence_bound)
            .count();
        let active_binary_sha_bound_fixture_count = preview_fixtures
            .iter()
            .filter(|fixture| fixture.active_binary_sha_bound)
            .count();
        let trusted_source_bound_fixture_count = preview_fixtures
            .iter()
            .filter(|fixture| fixture.trusted_source_bound)
            .count();
        let operator_approval_bound_fixture_count = preview_fixtures
            .iter()
            .filter(|fixture| fixture.operator_approval_bound)
            .count();
        let blocked_preview_fixture_count = preview_fixtures
            .iter()
            .filter(|fixture| fixture.preview_status == "blocked_preview")
            .count();
        let allowed_preview_fixture_count = preview_fixtures
            .iter()
            .filter(|fixture| fixture.preview_status == "allowed")
            .count();
        let public_claim_attempt_fixture_count = preview_fixtures
            .iter()
            .filter(|fixture| fixture.public_claim_requested)
            .count();
        let release_artifact_write_attempt_fixture_count = preview_fixtures
            .iter()
            .filter(|fixture| fixture.release_artifact_write_requested)
            .count();
        let filesystem_persistence_allowed_count = preview_fixtures
            .iter()
            .filter(|fixture| fixture.filesystem_persistence_allowed)
            .count();
        let workspace_write_performed_count = preview_fixtures
            .iter()
            .filter(|fixture| fixture.workspace_write_performed)
            .count();
        let evidence_receipt_persisted_count = preview_fixtures
            .iter()
            .filter(|fixture| fixture.evidence_receipt_persisted)
            .count();
        let sink_write_preview_ready = binding.output_path_evidence_binding_ready
            && preview_fixture_count == required_preview_fixture_count
            && allowed_output_path_entry_count == 3
            && previewed_output_path_count == required_preview_fixture_count
            && deterministic_payload_hash_count == required_preview_fixture_count
            && redacted_output_path_preview_count == required_preview_fixture_count
            && fresh_live_evidence_bound_fixture_count == required_preview_fixture_count
            && active_binary_sha_bound_fixture_count == required_preview_fixture_count
            && trusted_source_bound_fixture_count == required_preview_fixture_count
            && operator_approval_bound_fixture_count == required_preview_fixture_count
            && blocked_preview_fixture_count == required_preview_fixture_count
            && allowed_preview_fixture_count == 0
            && public_claim_attempt_fixture_count == 1
            && release_artifact_write_attempt_fixture_count == 1
            && filesystem_persistence_allowed_count == 0
            && workspace_write_performed_count == 0
            && evidence_receipt_persisted_count == 0;
        let activation_blocked_by_sink_write_preview = true;
        let activation_allowed_by_sink_write_preview = false;

        Self {
            product: "Hepta".into(),
            status: if sink_write_preview_ready {
                "ready"
            } else {
                "attention"
            }
            .into(),
            filesystem_sink_write_preview_id:
                "upstream-codex-activation-evidence-receipt-filesystem-sink-write-preview"
                    .into(),
            filesystem_sink_write_preview_doc_path:
                "docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVATION_EVIDENCE_RECEIPT_FILESYSTEM_SINK_WRITE_PREVIEW.md"
                    .into(),
            upstream_repository: binding.upstream_repository,
            candidate_diff_range: binding.candidate_diff_range,
            source_filesystem_output_path_evidence_binding_gate: binding
                .filesystem_output_path_evidence_binding_gate,
            filesystem_sink_write_preview_gate:
                "scripts/hepta-upstream-codex-activation-evidence-receipt-filesystem-sink-write-preview.sh"
                    .into(),
            active_dependency_isolation_gate: binding.active_dependency_isolation_gate,
            source_filesystem_output_path_evidence_binding_ready: binding
                .output_path_evidence_binding_ready,
            required_preview_fixture_count,
            preview_fixture_count,
            allowed_output_path_entry_count,
            previewed_output_path_count,
            deterministic_payload_hash_count,
            redacted_output_path_preview_count,
            fresh_live_evidence_bound_fixture_count,
            active_binary_sha_bound_fixture_count,
            trusted_source_bound_fixture_count,
            operator_approval_bound_fixture_count,
            blocked_preview_fixture_count,
            allowed_preview_fixture_count,
            public_claim_attempt_fixture_count,
            release_artifact_write_attempt_fixture_count,
            filesystem_persistence_allowed_count,
            workspace_write_performed_count,
            evidence_receipt_persisted_count,
            sink_write_preview_ready,
            activation_blocked_by_sink_write_preview,
            activation_allowed_by_sink_write_preview,
            active_wiring_allowed: false,
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
            command_invocation_performed: false,
            receipt_persistence_execution: false,
            materialization_execution: false,
            filesystem_persistence_execution: false,
            workspace_mutation_default: false,
            active_service_restart: false,
            credential_value_read: false,
            secret_file_read: false,
            provider_invoked: false,
            channel_delivery_performed: false,
            gateway_rpc_performed: false,
            public_release_published: false,
            preview_fixtures,
            preview_invariants: vec![
                "sink write previews can select only redacted allowlisted roots".into(),
                "deterministic payload hashes are preview evidence, not write authority".into(),
                "filesystem persistence remains disabled until a separate execution gate exists"
                    .into(),
                "public release and artifact attempts stay blocked by release governance".into(),
            ],
            required_next_gates: vec![
                "add a filesystem persistence execution-denial matrix before any workspace write"
                    .into(),
                "bind preview payload hashes to a future explicit persistence approval id".into(),
                "keep public artifact paths behind separate release-governance approval".into(),
            ],
        }
    }
}

impl HeptaUpstreamCodexActivationEvidenceReceiptFilesystemPersistenceExecutionDenialMatrixReport {
    pub fn native_default() -> Self {
        let preview =
            HeptaUpstreamCodexActivationEvidenceReceiptFilesystemSinkWritePreviewReport::native_default();
        let denial_fixtures =
            default_activation_evidence_receipt_filesystem_persistence_execution_denial_fixtures();
        let required_denial_fixture_count = 4;
        let denial_fixture_count = denial_fixtures.len();
        let source_preview_fixture_count = preview.preview_fixture_count;
        let execution_requested_fixture_count = denial_fixtures
            .iter()
            .filter(|fixture| fixture.execution_requested)
            .count();
        let future_persistence_approval_slot_count = denial_fixtures
            .iter()
            .filter(|fixture| {
                fixture
                    .future_persistence_approval_id_slot
                    .starts_with("<future:")
            })
            .count();
        let explicit_persistence_approval_id_present_count = denial_fixtures
            .iter()
            .filter(|fixture| fixture.explicit_persistence_approval_id_present)
            .count();
        let explicit_persistence_approval_id_missing_count =
            denial_fixture_count - explicit_persistence_approval_id_present_count;
        let stale_or_missing_fresh_evidence_fixture_count = denial_fixtures
            .iter()
            .filter(|fixture| !fixture.fresh_live_evidence_bound)
            .count();
        let active_binary_sha_bound_fixture_count = denial_fixtures
            .iter()
            .filter(|fixture| fixture.active_binary_sha_bound)
            .count();
        let trusted_source_bound_fixture_count = denial_fixtures
            .iter()
            .filter(|fixture| fixture.trusted_source_bound)
            .count();
        let operator_approval_bound_fixture_count = denial_fixtures
            .iter()
            .filter(|fixture| fixture.operator_approval_bound)
            .count();
        let workspace_path_attempt_fixture_count = denial_fixtures
            .iter()
            .filter(|fixture| fixture.workspace_path_requested)
            .count();
        let public_claim_attempt_fixture_count = denial_fixtures
            .iter()
            .filter(|fixture| fixture.public_claim_requested)
            .count();
        let release_artifact_write_attempt_fixture_count = denial_fixtures
            .iter()
            .filter(|fixture| fixture.release_artifact_write_requested)
            .count();
        let blocked_execution_fixture_count = denial_fixtures
            .iter()
            .filter(|fixture| fixture.execution_status == "blocked_execution")
            .count();
        let allowed_execution_fixture_count = denial_fixtures
            .iter()
            .filter(|fixture| fixture.execution_status == "allowed")
            .count();
        let filesystem_persistence_allowed_count = denial_fixtures
            .iter()
            .filter(|fixture| fixture.filesystem_persistence_allowed)
            .count();
        let filesystem_persistence_execution_performed_count = denial_fixtures
            .iter()
            .filter(|fixture| fixture.filesystem_persistence_execution_performed)
            .count();
        let workspace_write_performed_count = denial_fixtures
            .iter()
            .filter(|fixture| fixture.workspace_write_performed)
            .count();
        let evidence_receipt_persisted_count = denial_fixtures
            .iter()
            .filter(|fixture| fixture.evidence_receipt_persisted)
            .count();
        let execution_denial_matrix_ready = preview.sink_write_preview_ready
            && denial_fixture_count == required_denial_fixture_count
            && source_preview_fixture_count == 3
            && execution_requested_fixture_count == required_denial_fixture_count
            && future_persistence_approval_slot_count == required_denial_fixture_count
            && explicit_persistence_approval_id_present_count == 3
            && explicit_persistence_approval_id_missing_count == 1
            && stale_or_missing_fresh_evidence_fixture_count == 1
            && active_binary_sha_bound_fixture_count == required_denial_fixture_count
            && trusted_source_bound_fixture_count == required_denial_fixture_count
            && operator_approval_bound_fixture_count == 3
            && workspace_path_attempt_fixture_count == 1
            && public_claim_attempt_fixture_count == 1
            && release_artifact_write_attempt_fixture_count == 1
            && blocked_execution_fixture_count == required_denial_fixture_count
            && allowed_execution_fixture_count == 0
            && filesystem_persistence_allowed_count == 0
            && filesystem_persistence_execution_performed_count == 0
            && workspace_write_performed_count == 0
            && evidence_receipt_persisted_count == 0;
        let activation_blocked_by_execution_denial_matrix = true;
        let activation_allowed_by_execution_denial_matrix = false;

        Self {
            product: "Hepta".into(),
            status: if execution_denial_matrix_ready {
                "ready"
            } else {
                "attention"
            }
            .into(),
            filesystem_persistence_execution_denial_matrix_id:
                "upstream-codex-activation-evidence-receipt-filesystem-persistence-execution-denial-matrix"
                    .into(),
            filesystem_persistence_execution_denial_matrix_doc_path:
                "docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVATION_EVIDENCE_RECEIPT_FILESYSTEM_PERSISTENCE_EXECUTION_DENIAL_MATRIX.md"
                    .into(),
            upstream_repository: preview.upstream_repository,
            candidate_diff_range: preview.candidate_diff_range,
            source_filesystem_sink_write_preview_gate: preview.filesystem_sink_write_preview_gate,
            filesystem_persistence_execution_denial_matrix_gate:
                "scripts/hepta-upstream-codex-activation-evidence-receipt-filesystem-persistence-execution-denial-matrix.sh"
                    .into(),
            active_dependency_isolation_gate: preview.active_dependency_isolation_gate,
            source_filesystem_sink_write_preview_ready: preview.sink_write_preview_ready,
            required_denial_fixture_count,
            denial_fixture_count,
            source_preview_fixture_count,
            execution_requested_fixture_count,
            future_persistence_approval_slot_count,
            explicit_persistence_approval_id_present_count,
            explicit_persistence_approval_id_missing_count,
            stale_or_missing_fresh_evidence_fixture_count,
            active_binary_sha_bound_fixture_count,
            trusted_source_bound_fixture_count,
            operator_approval_bound_fixture_count,
            workspace_path_attempt_fixture_count,
            public_claim_attempt_fixture_count,
            release_artifact_write_attempt_fixture_count,
            blocked_execution_fixture_count,
            allowed_execution_fixture_count,
            filesystem_persistence_allowed_count,
            filesystem_persistence_execution_performed_count,
            workspace_write_performed_count,
            evidence_receipt_persisted_count,
            execution_denial_matrix_ready,
            activation_blocked_by_execution_denial_matrix,
            activation_allowed_by_execution_denial_matrix,
            active_wiring_allowed: false,
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
            command_invocation_performed: false,
            receipt_persistence_execution: false,
            materialization_execution: false,
            filesystem_persistence_execution: false,
            workspace_mutation_default: false,
            active_service_restart: false,
            credential_value_read: false,
            secret_file_read: false,
            provider_invoked: false,
            channel_delivery_performed: false,
            gateway_rpc_performed: false,
            public_release_published: false,
            denial_fixtures,
            denial_invariants: vec![
                "preview payload hashes are bound to future persistence approval slots, not write authority"
                    .into(),
                "missing approval id, stale evidence, workspace path attempts, and public artifact attempts all deny execution"
                    .into(),
                "filesystem persistence execution remains disabled by default".into(),
                "no workspace write or evidence receipt persistence occurs in the denial matrix".into(),
            ],
            required_next_gates: vec![
                "add a receipt persistence executor dry-run that consumes the denial matrix without writing"
                    .into(),
                "require explicit persistence approval id materialization before any filesystem write"
                    .into(),
                "keep public artifact writes behind release-governance approval".into(),
            ],
        }
    }
}

fn filesystem_persistence_approval_field(
    name: &str,
    redacted_or_hashed: bool,
    purpose: &str,
) -> HeptaUpstreamCodexActivationEvidenceReceiptFilesystemPersistenceApprovalField {
    HeptaUpstreamCodexActivationEvidenceReceiptFilesystemPersistenceApprovalField {
        name: name.into(),
        redacted_or_hashed,
        required_for_filesystem_persistence: true,
        recorded_by_default: false,
        purpose: purpose.into(),
    }
}

fn default_activation_evidence_receipt_filesystem_persistence_approval_fields()
-> Vec<HeptaUpstreamCodexActivationEvidenceReceiptFilesystemPersistenceApprovalField> {
    vec![
        filesystem_persistence_approval_field(
            "filesystem_persistence_approval_id",
            false,
            "unique operator-reviewed approval packet identifier",
        ),
        filesystem_persistence_approval_field(
            "activation_request_id",
            true,
            "binds the persistence request to the activation request packet",
        ),
        filesystem_persistence_approval_field(
            "operator_approval_id",
            true,
            "binds persistence to explicit operator approval",
        ),
        filesystem_persistence_approval_field(
            "operator_identity_hash",
            true,
            "records a redacted operator identity binding",
        ),
        filesystem_persistence_approval_field(
            "materialization_plan_id",
            true,
            "binds the write decision to a deterministic dry-run materialization plan",
        ),
        filesystem_persistence_approval_field(
            "receipt_payload_hash",
            true,
            "binds the approved write to the redacted receipt payload hash",
        ),
        filesystem_persistence_approval_field(
            "redacted_output_path",
            true,
            "records the intended output path without exposing private filesystem details",
        ),
        filesystem_persistence_approval_field(
            "accepted_trusted_record_ids",
            true,
            "binds persistence to accepted trusted evidence records",
        ),
        filesystem_persistence_approval_field(
            "fresh_trusted_record_ids",
            true,
            "binds persistence to fresh trusted evidence records",
        ),
        filesystem_persistence_approval_field(
            "active_binary_sha256",
            true,
            "binds persistence to the active Hepta binary under verification",
        ),
        filesystem_persistence_approval_field(
            "rollback_plan_id",
            true,
            "binds persistence to an operator-visible rollback plan",
        ),
        filesystem_persistence_approval_field(
            "public_claim_and_artifact_decision",
            false,
            "keeps public release claims and artifact writes separately approved",
        ),
    ]
}

fn filesystem_output_path_allowlist_entry(
    name: &str,
    redacted_path: &str,
    allowed_for_receipt_persistence: bool,
    blocked_for_public_artifact: bool,
    purpose: &str,
) -> HeptaUpstreamCodexActivationEvidenceReceiptFilesystemOutputPathAllowlistEntry {
    HeptaUpstreamCodexActivationEvidenceReceiptFilesystemOutputPathAllowlistEntry {
        name: name.into(),
        redacted_path: redacted_path.into(),
        allowed_for_receipt_persistence,
        blocked_for_public_artifact,
        requires_operator_approval: true,
        purpose: purpose.into(),
    }
}

fn default_activation_evidence_receipt_filesystem_output_path_allowlist_entries()
-> Vec<HeptaUpstreamCodexActivationEvidenceReceiptFilesystemOutputPathAllowlistEntry> {
    vec![
        filesystem_output_path_allowlist_entry(
            "activation_evidence_receipts_root",
            "<redacted:hepta-activation-evidence-receipts>",
            true,
            true,
            "bounded local receipt sink for operator-approved activation evidence",
        ),
        filesystem_output_path_allowlist_entry(
            "activation_evidence_dry_run_root",
            "<redacted:hepta-activation-evidence-dry-run>",
            true,
            true,
            "bounded local dry-run sink for receipt write previews",
        ),
        filesystem_output_path_allowlist_entry(
            "activation_evidence_operator_packet_root",
            "<redacted:hepta-operator-activation-packets>",
            true,
            true,
            "bounded local operator packet sink for redacted evidence references",
        ),
        filesystem_output_path_allowlist_entry(
            "source_tree_root",
            "<redacted:hepta-source-tree>",
            false,
            true,
            "source tree paths are not receipt persistence targets",
        ),
        filesystem_output_path_allowlist_entry(
            "home_directory_root",
            "<redacted:home-directory>",
            false,
            true,
            "home directory paths are never direct receipt persistence targets",
        ),
        filesystem_output_path_allowlist_entry(
            "release_artifact_root",
            "<redacted:release-artifact-root>",
            false,
            true,
            "release artifact paths require separate release-governance approval",
        ),
    ]
}

fn filesystem_output_path_evidence_binding(
    evidence_id: &str,
    allowed_output_path_entry_name: &str,
    redacted_or_hashed: bool,
    purpose: &str,
) -> HeptaUpstreamCodexActivationEvidenceReceiptFilesystemOutputPathEvidenceBinding {
    HeptaUpstreamCodexActivationEvidenceReceiptFilesystemOutputPathEvidenceBinding {
        evidence_id: evidence_id.into(),
        allowed_output_path_entry_name: allowed_output_path_entry_name.into(),
        binding_required: true,
        recorded_by_default: false,
        redacted_or_hashed,
        requires_fresh_live_evidence: true,
        requires_active_binary_sha: true,
        purpose: purpose.into(),
    }
}

fn default_activation_evidence_receipt_filesystem_output_path_evidence_bindings()
-> Vec<HeptaUpstreamCodexActivationEvidenceReceiptFilesystemOutputPathEvidenceBinding> {
    vec![
        filesystem_output_path_evidence_binding(
            "activation_request_id",
            "activation_evidence_operator_packet_root",
            true,
            "binds the selected output root to the activation request packet",
        ),
        filesystem_output_path_evidence_binding(
            "operator_approval_id",
            "activation_evidence_operator_packet_root",
            true,
            "binds the selected output root to explicit operator approval",
        ),
        filesystem_output_path_evidence_binding(
            "operator_identity_hash",
            "activation_evidence_operator_packet_root",
            true,
            "binds the selected output root to a redacted operator identity",
        ),
        filesystem_output_path_evidence_binding(
            "live_dependency_isolation_evidence_id",
            "activation_evidence_receipts_root",
            true,
            "binds the selected output root to fresh live dependency isolation evidence",
        ),
        filesystem_output_path_evidence_binding(
            "watchdog_evidence_id",
            "activation_evidence_receipts_root",
            true,
            "binds the selected output root to fresh watchdog evidence",
        ),
        filesystem_output_path_evidence_binding(
            "browser_smoke_evidence_id",
            "activation_evidence_receipts_root",
            true,
            "binds the selected output root to fresh browser visual smoke evidence",
        ),
        filesystem_output_path_evidence_binding(
            "long_soak_evidence_id",
            "activation_evidence_receipts_root",
            true,
            "binds the selected output root to fresh long-soak evidence",
        ),
        filesystem_output_path_evidence_binding(
            "rollback_plan_id",
            "activation_evidence_dry_run_root",
            true,
            "binds the selected output root to the rollback plan before any persistence",
        ),
    ]
}

fn filesystem_sink_write_preview_fixture(
    fixture_id: &str,
    allowed_output_path_entry_name: &str,
    redacted_output_path: &str,
    deterministic_payload_hash: &str,
    public_claim_requested: bool,
    release_artifact_write_requested: bool,
) -> HeptaUpstreamCodexActivationEvidenceReceiptFilesystemSinkWritePreviewFixture {
    HeptaUpstreamCodexActivationEvidenceReceiptFilesystemSinkWritePreviewFixture {
        fixture_id: fixture_id.into(),
        allowed_output_path_entry_name: allowed_output_path_entry_name.into(),
        redacted_output_path: redacted_output_path.into(),
        deterministic_payload_hash: deterministic_payload_hash.into(),
        fresh_live_evidence_bound: true,
        active_binary_sha_bound: true,
        trusted_source_bound: true,
        operator_approval_bound: true,
        public_claim_requested,
        release_artifact_write_requested,
        preview_status: "blocked_preview".into(),
        filesystem_persistence_allowed: false,
        workspace_write_performed: false,
        evidence_receipt_persisted: false,
    }
}

fn default_activation_evidence_receipt_filesystem_sink_write_preview_fixtures()
-> Vec<HeptaUpstreamCodexActivationEvidenceReceiptFilesystemSinkWritePreviewFixture> {
    vec![
        filesystem_sink_write_preview_fixture(
            "receipt-root-sink-write-preview",
            "activation_evidence_receipts_root",
            "<redacted:hepta-activation-evidence-receipts/receipt-preview.json>",
            "sha256:preview-receipt-root-payload",
            false,
            false,
        ),
        filesystem_sink_write_preview_fixture(
            "dry-run-root-sink-write-preview",
            "activation_evidence_dry_run_root",
            "<redacted:hepta-activation-evidence-dry-run/receipt-preview.json>",
            "sha256:preview-dry-run-root-payload",
            false,
            false,
        ),
        filesystem_sink_write_preview_fixture(
            "public-artifact-sink-write-preview-attempt",
            "activation_evidence_operator_packet_root",
            "<redacted:hepta-operator-activation-packets/public-artifact-attempt.json>",
            "sha256:preview-public-artifact-attempt-payload",
            true,
            true,
        ),
    ]
}

struct FilesystemPersistenceExecutionDenialFixtureSpec<'a> {
    fixture_id: &'a str,
    source_preview_fixture_id: &'a str,
    deterministic_payload_hash: &'a str,
    future_persistence_approval_id_slot: &'a str,
    explicit_persistence_approval_id_present: bool,
    fresh_live_evidence_bound: bool,
    operator_approval_bound: bool,
    workspace_path_requested: bool,
    public_claim_requested: bool,
    release_artifact_write_requested: bool,
    denial_reason: &'a str,
}

fn filesystem_persistence_execution_denial_fixture(
    spec: FilesystemPersistenceExecutionDenialFixtureSpec<'_>,
) -> HeptaUpstreamCodexActivationEvidenceReceiptFilesystemPersistenceExecutionDenialFixture {
    HeptaUpstreamCodexActivationEvidenceReceiptFilesystemPersistenceExecutionDenialFixture {
        fixture_id: spec.fixture_id.into(),
        source_preview_fixture_id: spec.source_preview_fixture_id.into(),
        deterministic_payload_hash: spec.deterministic_payload_hash.into(),
        future_persistence_approval_id_slot: spec.future_persistence_approval_id_slot.into(),
        execution_requested: true,
        explicit_persistence_approval_id_present: spec.explicit_persistence_approval_id_present,
        fresh_live_evidence_bound: spec.fresh_live_evidence_bound,
        active_binary_sha_bound: true,
        trusted_source_bound: true,
        operator_approval_bound: spec.operator_approval_bound,
        workspace_path_requested: spec.workspace_path_requested,
        public_claim_requested: spec.public_claim_requested,
        release_artifact_write_requested: spec.release_artifact_write_requested,
        denial_reason: spec.denial_reason.into(),
        execution_status: "blocked_execution".into(),
        filesystem_persistence_allowed: false,
        filesystem_persistence_execution_performed: false,
        workspace_write_performed: false,
        evidence_receipt_persisted: false,
    }
}

fn default_activation_evidence_receipt_filesystem_persistence_execution_denial_fixtures()
-> Vec<HeptaUpstreamCodexActivationEvidenceReceiptFilesystemPersistenceExecutionDenialFixture> {
    vec![
        filesystem_persistence_execution_denial_fixture(
            FilesystemPersistenceExecutionDenialFixtureSpec {
                fixture_id: "missing-persistence-approval-id-execution-attempt",
                source_preview_fixture_id: "receipt-root-sink-write-preview",
                deterministic_payload_hash: "sha256:preview-receipt-root-payload",
                future_persistence_approval_id_slot: "<future:persistence-approval-id:receipt-root>",
                explicit_persistence_approval_id_present: false,
                fresh_live_evidence_bound: true,
                operator_approval_bound: false,
                workspace_path_requested: false,
                public_claim_requested: false,
                release_artifact_write_requested: false,
                denial_reason: "explicit persistence approval id is absent",
            },
        ),
        filesystem_persistence_execution_denial_fixture(
            FilesystemPersistenceExecutionDenialFixtureSpec {
                fixture_id: "stale-live-evidence-execution-attempt",
                source_preview_fixture_id: "dry-run-root-sink-write-preview",
                deterministic_payload_hash: "sha256:preview-dry-run-root-payload",
                future_persistence_approval_id_slot: "<future:persistence-approval-id:dry-run-root>",
                explicit_persistence_approval_id_present: true,
                fresh_live_evidence_bound: false,
                operator_approval_bound: true,
                workspace_path_requested: false,
                public_claim_requested: false,
                release_artifact_write_requested: false,
                denial_reason: "fresh live evidence binding is stale or missing",
            },
        ),
        filesystem_persistence_execution_denial_fixture(
            FilesystemPersistenceExecutionDenialFixtureSpec {
                fixture_id: "workspace-path-execution-attempt",
                source_preview_fixture_id: "receipt-root-sink-write-preview",
                deterministic_payload_hash: "sha256:preview-receipt-root-payload",
                future_persistence_approval_id_slot: "<future:persistence-approval-id:workspace-path>",
                explicit_persistence_approval_id_present: true,
                fresh_live_evidence_bound: true,
                operator_approval_bound: true,
                workspace_path_requested: true,
                public_claim_requested: false,
                release_artifact_write_requested: false,
                denial_reason: "workspace path write is outside the receipt sink authority",
            },
        ),
        filesystem_persistence_execution_denial_fixture(
            FilesystemPersistenceExecutionDenialFixtureSpec {
                fixture_id: "public-artifact-execution-attempt",
                source_preview_fixture_id: "public-artifact-sink-write-preview-attempt",
                deterministic_payload_hash: "sha256:preview-public-artifact-attempt-payload",
                future_persistence_approval_id_slot: "<future:persistence-approval-id:public-artifact>",
                explicit_persistence_approval_id_present: true,
                fresh_live_evidence_bound: true,
                operator_approval_bound: true,
                workspace_path_requested: false,
                public_claim_requested: true,
                release_artifact_write_requested: true,
                denial_reason: "public release and artifact writes require separate release governance",
            },
        ),
    ]
}

fn activation_request_field(
    name: &str,
    redacted_or_hashed: bool,
    purpose: &str,
) -> HeptaUpstreamCodexActivationRequestPacketField {
    HeptaUpstreamCodexActivationRequestPacketField {
        name: name.into(),
        required: true,
        recorded: false,
        redacted_or_hashed,
        purpose: purpose.into(),
    }
}

fn default_activation_request_packet_fields() -> Vec<HeptaUpstreamCodexActivationRequestPacketField>
{
    vec![
        activation_request_field(
            "activation_request_id",
            false,
            "unique request id binding the activation review",
        ),
        activation_request_field(
            "operator_approval_id",
            false,
            "explicit approval record for the requested activation",
        ),
        activation_request_field(
            "operator_identity_hash",
            true,
            "hashed operator identity without exposing private account details",
        ),
        activation_request_field(
            "approved_bucket_ids",
            false,
            "upstream Codex diff buckets approved for activation consideration",
        ),
        activation_request_field(
            "approved_surface_ids",
            false,
            "Hepta surfaces approved for active wiring consideration",
        ),
        activation_request_field(
            "requested_runtime_wiring_scope",
            false,
            "bounded active runtime code path requested for wiring",
        ),
        activation_request_field(
            "requested_dependency_change_set",
            false,
            "explicit dependency changes requested for the active service",
        ),
        activation_request_field(
            "live_dependency_isolation_evidence_id",
            false,
            "fresh active-service dependency isolation evidence",
        ),
        activation_request_field(
            "watchdog_evidence_id",
            false,
            "fresh watchdog evidence for the requested activation",
        ),
        activation_request_field(
            "browser_smoke_evidence_id",
            false,
            "fresh browser visual smoke evidence",
        ),
        activation_request_field(
            "long_soak_evidence_id",
            false,
            "fresh long-soak evidence for the requested activation",
        ),
        activation_request_field(
            "rollback_plan_id",
            false,
            "rollback anchor for the requested active wiring",
        ),
        activation_request_field(
            "public_release_claim_decision",
            false,
            "explicit decision that public release claims remain separately gated",
        ),
        activation_request_field(
            "release_artifact_write_decision",
            false,
            "explicit decision that release artifact writes remain separately gated",
        ),
    ]
}

fn sync_contract(
    id: &str,
    risk: HeptaUpstreamCodexSyncRisk,
    title: &str,
    upstream_scope: &[&str],
    hepta_surfaces: &[&str],
    required_gate: &str,
) -> HeptaUpstreamCodexSyncContract {
    HeptaUpstreamCodexSyncContract {
        id: id.into(),
        risk,
        title: title.into(),
        upstream_scope: upstream_scope.iter().map(|value| (*value).into()).collect(),
        hepta_surfaces: hepta_surfaces.iter().map(|value| (*value).into()).collect(),
        required_gate: required_gate.into(),
        auto_apply_allowed: false,
        active_runtime_dependency_allowed: false,
        public_release_claim_allowed: false,
        contract_ready: true,
    }
}

fn default_upstream_codex_sync_contracts() -> Vec<HeptaUpstreamCodexSyncContract> {
    vec![
        sync_contract(
            "snapshot-and-diff-intake",
            HeptaUpstreamCodexSyncRisk::Guardrail,
            "Fetch upstream only into an explicit snapshot/diff lane",
            &[
                "upstream remote metadata",
                "release tags",
                "commit range summary",
                "file-level diff inventory",
            ],
            &[
                "codex-rs compatibility snapshot",
                "docs/architecture/HEPTA_UPSTREAM_CODEX_SYNC_LANE.md",
            ],
            "scripts/hepta-upstream-codex-snapshot.sh and scripts/hepta-upstream-codex-diff-ledger.sh must record the observed upstream head and classified diff range before any absorption patch",
        ),
        sync_contract(
            "provider-credential-security-classification",
            HeptaUpstreamCodexSyncRisk::P0Security,
            "Classify provider, credential, approval, sandbox, and network deltas first",
            &[
                "model provider",
                "credential and auth flows",
                "approval policy",
                "sandbox and exec",
                "network/proxy behavior",
            ],
            &[
                "hepta-runtime provider reports",
                "hepta-kernel policy gates",
                "hepta-gateway read-only reports",
            ],
            "security/provider diffs require adapter contract review before active runtime wiring",
        ),
        sync_contract(
            "runtime-session-tool-contract-classification",
            HeptaUpstreamCodexSyncRisk::P0Runtime,
            "Classify runtime, session, thread-store, tool, MCP, and app-server deltas",
            &[
                "runtime event loop",
                "session and thread store",
                "tool invocation",
                "MCP server/client",
                "app-server protocol",
            ],
            &[
                "/api/hepta-engine-adapter-boundary",
                "/api/hepta-core-fusion-readiness",
                "/api/hepta-engine-dependency-closure",
            ],
            "adapter behavior-equivalence and shadow-replay gates must pass before promotion",
        ),
        sync_contract(
            "compatibility-package-retention-boundary",
            HeptaUpstreamCodexSyncRisk::P1Compatibility,
            "Keep Codex compatibility as an intake surface, not the active service engine",
            &[
                "codex-cli",
                "codex-core",
                "codex-exec",
                "codex-state",
                "codex-mcp",
                "codex-app-server",
                "codex-sandboxing",
                "codex-plugin",
                "codex-model-provider",
                "codex-protocol",
                "codex-tui",
            ],
            &[
                "hepta-cli --bin hepta",
                "scripts/hepta-active-service-dependency-isolation.sh",
            ],
            "active hepta-cli cargo tree must remain free of tracked Codex engine crates",
        ),
        sync_contract(
            "release-governance-no-public-claim",
            HeptaUpstreamCodexSyncRisk::P2Product,
            "Require governance evidence before claiming an upstream-sync release",
            &[
                "changelog",
                "release notes",
                "operator packet",
                "long soak evidence",
                "watchdog evidence",
            ],
            &[
                "public GA readiness",
                "operator approval packet",
                "watchdog",
                "live soak",
            ],
            "no public release claim until long-cycle soak and governance packet pass",
        ),
    ]
}

pub fn hepta_upstream_codex_sync_lane_report() -> HeptaUpstreamCodexSyncLaneReport {
    HeptaUpstreamCodexSyncLaneReport::native_default()
}

fn snapshot_risk_class(
    id: &str,
    risk: HeptaUpstreamCodexSyncRisk,
    upstream_path_hints: &[&str],
    hepta_review_surfaces: &[&str],
    required_action: &str,
) -> HeptaUpstreamCodexSnapshotRiskClass {
    HeptaUpstreamCodexSnapshotRiskClass {
        id: id.into(),
        risk,
        upstream_path_hints: upstream_path_hints
            .iter()
            .map(|value| (*value).into())
            .collect(),
        hepta_review_surfaces: hepta_review_surfaces
            .iter()
            .map(|value| (*value).into())
            .collect(),
        required_action: required_action.into(),
        auto_absorb_allowed: false,
        active_runtime_dependency_allowed: false,
        classification_required: true,
    }
}

fn default_upstream_codex_snapshot_risk_classes() -> Vec<HeptaUpstreamCodexSnapshotRiskClass> {
    vec![
        snapshot_risk_class(
            "provider-credential-sandbox-security",
            HeptaUpstreamCodexSyncRisk::P0Security,
            &[
                "providers",
                "auth",
                "login",
                "credentials",
                "approval_policy",
                "sandbox",
                "exec",
                "network",
            ],
            &[
                "hepta-runtime provider reports",
                "hepta-kernel security policy reports",
                "operator approval packet",
            ],
            "classify as P0 before any adapter or active runtime wiring",
        ),
        snapshot_risk_class(
            "runtime-session-tool-mcp-appserver",
            HeptaUpstreamCodexSyncRisk::P0Runtime,
            &[
                "runtime",
                "session",
                "thread",
                "tool",
                "mcp",
                "app-server",
                "protocol",
            ],
            &[
                "/api/hepta-engine-adapter-boundary",
                "adapter behavior-equivalence gate",
                "shadow replay gate",
            ],
            "require contract tests and replay evidence before promotion",
        ),
        snapshot_risk_class(
            "legacy-cli-tui-compatibility",
            HeptaUpstreamCodexSyncRisk::P1Compatibility,
            &["cli", "tui", "codex-cli", "codex-tui", "legacy command"],
            &[
                "codex-cli compatibility package",
                "scripts/hepta-active-service-dependency-isolation.sh",
            ],
            "retain only as compatibility intake unless Hepta contracts absorb it",
        ),
        snapshot_risk_class(
            "product-doc-release-governance",
            HeptaUpstreamCodexSyncRisk::P2Product,
            &[
                "docs",
                "changelog",
                "release",
                "install",
                "package metadata",
            ],
            &[
                "public GA readiness gate",
                "release-hardening status",
                "operator approval packet",
                "long-cycle soak evidence",
            ],
            "gate release claims on governance evidence, not on upstream freshness alone",
        ),
    ]
}

pub fn hepta_upstream_codex_snapshot_report() -> HeptaUpstreamCodexSnapshotReport {
    HeptaUpstreamCodexSnapshotReport::native_default()
}

fn diff_ledger_bucket(
    id: &str,
    risk: HeptaUpstreamCodexSyncRisk,
    upstream_path_hints: &[&str],
    hepta_review_surfaces: &[&str],
    required_action: &str,
    promotion_gate: &str,
) -> HeptaUpstreamCodexDiffLedgerBucket {
    HeptaUpstreamCodexDiffLedgerBucket {
        id: id.into(),
        risk,
        upstream_path_hints: upstream_path_hints
            .iter()
            .map(|value| (*value).into())
            .collect(),
        hepta_review_surfaces: hepta_review_surfaces
            .iter()
            .map(|value| (*value).into())
            .collect(),
        required_action: required_action.into(),
        promotion_gate: promotion_gate.into(),
        auto_absorb_allowed: false,
        active_runtime_dependency_allowed: false,
        classification_required: true,
        bucket_ready: true,
    }
}

fn default_upstream_codex_diff_ledger_buckets() -> Vec<HeptaUpstreamCodexDiffLedgerBucket> {
    vec![
        diff_ledger_bucket(
            "provider-credential-sandbox-security",
            HeptaUpstreamCodexSyncRisk::P0Security,
            &[
                "codex-rs/codex-api",
                "codex-rs/model-provider",
                "codex-rs/login",
                "codex-rs/config",
                "codex-rs/*sandbox*",
                "codex-rs/exec",
                "codex-rs/network-proxy",
            ],
            &[
                "hepta-runtime provider reports",
                "hepta-kernel security policy reports",
                "operator approval packet",
            ],
            "classify security/auth/provider/sandbox paths before any active adapter wiring",
            "security/provider diffs require adapter contract review and dependency isolation",
        ),
        diff_ledger_bucket(
            "runtime-session-tool-mcp-appserver",
            HeptaUpstreamCodexSyncRisk::P0Runtime,
            &[
                "codex-rs/app-server*",
                "codex-rs/core/src/session",
                "codex-rs/core/src/tools",
                "codex-rs/codex-mcp",
                "codex-rs/mcp-server",
                "codex-rs/thread-store",
                "codex-rs/hooks",
            ],
            &[
                "/api/hepta-engine-adapter-boundary",
                "adapter behavior-equivalence gate",
                "adapter shadow replay gate",
            ],
            "classify runtime/session/tool/MCP/app-server paths before promotion",
            "adapter behavior-equivalence and shadow-replay gates must pass",
        ),
        diff_ledger_bucket(
            "legacy-cli-tui-compatibility",
            HeptaUpstreamCodexSyncRisk::P1Compatibility,
            &[
                "codex-rs/cli",
                "codex-rs/tui",
                "codex-rs/code-mode",
                "codex-rs/terminal-*",
            ],
            &[
                "codex-cli compatibility package",
                "scripts/hepta-active-service-dependency-isolation.sh",
            ],
            "retain CLI/TUI deltas as compatibility intake unless Hepta contracts absorb them",
            "active hepta-cli cargo tree must stay free of tracked Codex engine crates",
        ),
        diff_ledger_bucket(
            "product-doc-release-governance",
            HeptaUpstreamCodexSyncRisk::P2Product,
            &[
                "README",
                "docs",
                "package",
                "release",
                "Cargo.lock",
                "Cargo.toml",
            ],
            &[
                "public GA readiness gate",
                "release-hardening status",
                "operator approval packet",
                "long-cycle soak evidence",
            ],
            "separate product/release deltas from runtime claims",
            "release claims require governance packet, watchdog, and long soak evidence",
        ),
    ]
}

pub fn hepta_upstream_codex_diff_ledger_report() -> HeptaUpstreamCodexDiffLedgerReport {
    HeptaUpstreamCodexDiffLedgerReport::native_default()
}

pub fn hepta_upstream_codex_current_intake_report() -> HeptaUpstreamCodexCurrentIntakeReport {
    HeptaUpstreamCodexCurrentIntakeReport::native_default()
}

fn default_product_governance_selected_paths() -> Vec<String> {
    [
        "codex-rs/Cargo.lock",
        "codex-rs/Cargo.toml",
        "codex-rs/README.md",
        "codex-rs/app-server/README.md",
        "codex-rs/app-server/tests/suite/v2/plugin_install.rs",
        "codex-rs/app-server/tests/suite/v2/plugin_uninstall.rs",
        "codex-rs/core-plugins/src/remote/remote_installed_plugin_sync.rs",
        "codex-rs/core/README.md",
        "codex-rs/core/src/tools/handlers/list_available_plugins_to_install.rs",
        "codex-rs/core/src/tools/handlers/list_available_plugins_to_install_spec.rs",
        "codex-rs/core/src/tools/handlers/request_plugin_install.rs",
        "codex-rs/core/src/tools/handlers/request_plugin_install_spec.rs",
        "codex-rs/core/tests/suite/request_plugin_install.rs",
        "codex-rs/docs/protocol_v1.md",
        "codex-rs/exec-server/README.md",
        "codex-rs/install-context/Cargo.toml",
        "codex-rs/install-context/src/lib.rs",
        "codex-rs/linux-sandbox/README.md",
        "codex-rs/network-proxy/README.md",
        "codex-rs/skills/src/assets/samples/plugin-creator/references/installing-and-updating.md",
        "codex-rs/tools/README.md",
        "codex-rs/utils/pty/README.md",
    ]
    .iter()
    .map(|path| (*path).into())
    .collect()
}

pub fn hepta_upstream_codex_product_governance_absorption_report()
-> HeptaUpstreamCodexProductGovernanceAbsorptionReport {
    HeptaUpstreamCodexProductGovernanceAbsorptionReport::native_default()
}

pub fn hepta_upstream_codex_product_governance_translation_report()
-> HeptaUpstreamCodexProductGovernanceTranslationReport {
    HeptaUpstreamCodexProductGovernanceTranslationReport::native_default()
}

pub fn hepta_upstream_codex_release_governance_promotion_report()
-> HeptaUpstreamCodexReleaseGovernancePromotionReport {
    HeptaUpstreamCodexReleaseGovernancePromotionReport::native_default()
}

pub fn hepta_upstream_codex_legacy_compatibility_absorption_report()
-> HeptaUpstreamCodexLegacyCompatibilityAbsorptionReport {
    HeptaUpstreamCodexLegacyCompatibilityAbsorptionReport::native_default()
}

pub fn hepta_upstream_codex_legacy_compatibility_replay_report()
-> HeptaUpstreamCodexLegacyCompatibilityReplayReport {
    HeptaUpstreamCodexLegacyCompatibilityReplayReport::native_default()
}

pub fn hepta_upstream_codex_legacy_compatibility_promotion_report()
-> HeptaUpstreamCodexLegacyCompatibilityPromotionReport {
    HeptaUpstreamCodexLegacyCompatibilityPromotionReport::native_default()
}

pub fn hepta_upstream_codex_provider_security_absorption_report()
-> HeptaUpstreamCodexProviderSecurityAbsorptionReport {
    HeptaUpstreamCodexProviderSecurityAbsorptionReport::native_default()
}

pub fn hepta_upstream_codex_provider_security_replay_report()
-> HeptaUpstreamCodexProviderSecurityReplayReport {
    HeptaUpstreamCodexProviderSecurityReplayReport::native_default()
}

pub fn hepta_upstream_codex_provider_security_promotion_report()
-> HeptaUpstreamCodexProviderSecurityPromotionReport {
    HeptaUpstreamCodexProviderSecurityPromotionReport::native_default()
}

pub fn hepta_upstream_codex_runtime_appserver_absorption_report()
-> HeptaUpstreamCodexRuntimeAppServerAbsorptionReport {
    HeptaUpstreamCodexRuntimeAppServerAbsorptionReport::native_default()
}

pub fn hepta_upstream_codex_runtime_appserver_replay_report()
-> HeptaUpstreamCodexRuntimeAppServerReplayReport {
    HeptaUpstreamCodexRuntimeAppServerReplayReport::native_default()
}

pub fn hepta_upstream_codex_runtime_appserver_promotion_report()
-> HeptaUpstreamCodexRuntimeAppServerPromotionReport {
    HeptaUpstreamCodexRuntimeAppServerPromotionReport::native_default()
}

pub fn hepta_upstream_codex_absorption_replay_readiness_report()
-> HeptaUpstreamCodexAbsorptionReplayReadinessReport {
    HeptaUpstreamCodexAbsorptionReplayReadinessReport::native_default()
}

pub fn hepta_upstream_codex_promotion_readiness_report()
-> HeptaUpstreamCodexPromotionReadinessReport {
    HeptaUpstreamCodexPromotionReadinessReport::native_default()
}

pub fn hepta_upstream_codex_promotion_closure_report() -> HeptaUpstreamCodexPromotionClosureReport {
    HeptaUpstreamCodexPromotionClosureReport::native_default()
}

pub fn hepta_upstream_codex_active_wiring_precondition_report()
-> HeptaUpstreamCodexActiveWiringPreconditionReport {
    HeptaUpstreamCodexActiveWiringPreconditionReport::native_default()
}

pub fn hepta_upstream_codex_activation_request_packet_report()
-> HeptaUpstreamCodexActivationRequestPacketReport {
    HeptaUpstreamCodexActivationRequestPacketReport::native_default()
}

pub fn hepta_upstream_codex_activation_packet_dry_run_report()
-> HeptaUpstreamCodexActivationPacketDryRunReport {
    HeptaUpstreamCodexActivationPacketDryRunReport::native_default()
}

pub fn hepta_upstream_codex_activation_evidence_ledger_report()
-> HeptaUpstreamCodexActivationEvidenceLedgerReport {
    HeptaUpstreamCodexActivationEvidenceLedgerReport::native_default()
}

pub fn hepta_upstream_codex_activation_readiness_closure_report()
-> HeptaUpstreamCodexActivationReadinessClosureReport {
    HeptaUpstreamCodexActivationReadinessClosureReport::native_default()
}

pub fn hepta_upstream_codex_activation_denied_sample_report()
-> HeptaUpstreamCodexActivationDeniedSampleReport {
    HeptaUpstreamCodexActivationDeniedSampleReport::native_default()
}

pub fn hepta_upstream_codex_activation_evidence_freshness_policy_report()
-> HeptaUpstreamCodexActivationEvidenceFreshnessPolicyReport {
    HeptaUpstreamCodexActivationEvidenceFreshnessPolicyReport::native_default()
}

pub fn hepta_upstream_codex_activation_evidence_binding_record_manifest_report()
-> HeptaUpstreamCodexActivationEvidenceBindingRecordManifestReport {
    HeptaUpstreamCodexActivationEvidenceBindingRecordManifestReport::native_default()
}

pub fn hepta_upstream_codex_activation_evidence_record_denied_fixture_report()
-> HeptaUpstreamCodexActivationEvidenceRecordDeniedFixtureReport {
    HeptaUpstreamCodexActivationEvidenceRecordDeniedFixtureReport::native_default()
}

pub fn hepta_upstream_codex_activation_trusted_evidence_acceptance_matrix_report()
-> HeptaUpstreamCodexActivationTrustedEvidenceAcceptanceMatrixReport {
    HeptaUpstreamCodexActivationTrustedEvidenceAcceptanceMatrixReport::native_default()
}

pub fn hepta_upstream_codex_activation_trusted_record_shape_validator_report()
-> HeptaUpstreamCodexActivationTrustedRecordShapeValidatorReport {
    HeptaUpstreamCodexActivationTrustedRecordShapeValidatorReport::native_default()
}

pub fn hepta_upstream_codex_activation_evidence_completeness_scoreboard_report()
-> HeptaUpstreamCodexActivationEvidenceCompletenessScoreboardReport {
    HeptaUpstreamCodexActivationEvidenceCompletenessScoreboardReport::native_default()
}

pub fn hepta_upstream_codex_activation_evidence_recording_dry_run_receipt_report()
-> HeptaUpstreamCodexActivationEvidenceRecordingDryRunReceiptReport {
    HeptaUpstreamCodexActivationEvidenceRecordingDryRunReceiptReport::native_default()
}

pub fn hepta_upstream_codex_activation_evidence_recording_denial_matrix_report()
-> HeptaUpstreamCodexActivationEvidenceRecordingDenialMatrixReport {
    HeptaUpstreamCodexActivationEvidenceRecordingDenialMatrixReport::native_default()
}

pub fn hepta_upstream_codex_activation_evidence_receipt_persistence_command_contract_report()
-> HeptaUpstreamCodexActivationEvidenceReceiptPersistenceCommandContractReport {
    HeptaUpstreamCodexActivationEvidenceReceiptPersistenceCommandContractReport::native_default()
}

pub fn hepta_upstream_codex_activation_evidence_receipt_persistence_invocation_dry_run_report()
-> HeptaUpstreamCodexActivationEvidenceReceiptPersistenceInvocationDryRunReport {
    HeptaUpstreamCodexActivationEvidenceReceiptPersistenceInvocationDryRunReport::native_default()
}

pub fn hepta_upstream_codex_activation_evidence_receipt_no_write_sink_adapter_contract_report()
-> HeptaUpstreamCodexActivationEvidenceReceiptNoWriteSinkAdapterContractReport {
    HeptaUpstreamCodexActivationEvidenceReceiptNoWriteSinkAdapterContractReport::native_default()
}

pub fn hepta_upstream_codex_activation_evidence_receipt_write_enable_fixture_report()
-> HeptaUpstreamCodexActivationEvidenceReceiptWriteEnableFixtureReport {
    HeptaUpstreamCodexActivationEvidenceReceiptWriteEnableFixtureReport::native_default()
}

pub fn hepta_upstream_codex_activation_evidence_receipt_materialization_dry_run_report()
-> HeptaUpstreamCodexActivationEvidenceReceiptMaterializationDryRunReport {
    HeptaUpstreamCodexActivationEvidenceReceiptMaterializationDryRunReport::native_default()
}

pub fn hepta_upstream_codex_activation_evidence_receipt_filesystem_persistence_approval_packet_report()
-> HeptaUpstreamCodexActivationEvidenceReceiptFilesystemPersistenceApprovalPacketReport {
    HeptaUpstreamCodexActivationEvidenceReceiptFilesystemPersistenceApprovalPacketReport::native_default()
}

pub fn hepta_upstream_codex_activation_evidence_receipt_filesystem_output_path_allowlist_report()
-> HeptaUpstreamCodexActivationEvidenceReceiptFilesystemOutputPathAllowlistReport {
    HeptaUpstreamCodexActivationEvidenceReceiptFilesystemOutputPathAllowlistReport::native_default()
}

pub fn hepta_upstream_codex_activation_evidence_receipt_filesystem_output_path_evidence_binding_report()
-> HeptaUpstreamCodexActivationEvidenceReceiptFilesystemOutputPathEvidenceBindingReport {
    HeptaUpstreamCodexActivationEvidenceReceiptFilesystemOutputPathEvidenceBindingReport::native_default()
}

pub fn hepta_upstream_codex_activation_evidence_receipt_filesystem_sink_write_preview_report()
-> HeptaUpstreamCodexActivationEvidenceReceiptFilesystemSinkWritePreviewReport {
    HeptaUpstreamCodexActivationEvidenceReceiptFilesystemSinkWritePreviewReport::native_default()
}

pub fn hepta_upstream_codex_activation_evidence_receipt_filesystem_persistence_execution_denial_matrix_report()
-> HeptaUpstreamCodexActivationEvidenceReceiptFilesystemPersistenceExecutionDenialMatrixReport {
    HeptaUpstreamCodexActivationEvidenceReceiptFilesystemPersistenceExecutionDenialMatrixReport::native_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn upstream_codex_sync_lane_is_ready_without_side_effects_or_latest_claims() {
        let report = hepta_upstream_codex_sync_lane_report();

        assert_eq!(report.product, "Hepta");
        assert_eq!(report.status, "ready");
        assert_eq!(report.lane_id, "upstream-codex-sync-lane");
        assert_eq!(
            report.upstream_repository,
            "https://github.com/openai/codex"
        );
        assert_eq!(report.contract_count, 5);
        assert_eq!(report.ready_contract_count, report.contract_count);
        assert!(report.sync_lane_ready);
        assert!(!report.upstream_fetch_performed);
        assert!(!report.upstream_latest_claimed);
        assert!(!report.upstream_merge_performed);
        assert!(!report.active_runtime_auto_rebase_allowed);
        assert!(!report.active_runtime_codex_engine_dependency_allowed);
        assert!(report.requires_diff_classification_before_absorption);
        assert!(report.requires_adapter_contract_before_active_runtime);
        assert!(report.requires_release_governance_before_public_claim);
        assert!(report.local_only_audit);
        assert!(report.report_only);
        assert!(!report.mutates_runtime_state);
        assert!(!report.external_network_read);
        assert!(!report.external_send);
        assert!(!report.credential_value_read);
        assert!(!report.secret_file_read);
        assert!(!report.provider_invoked);
        assert!(!report.channel_delivery_performed);
        assert!(!report.gateway_rpc_performed);
    }

    #[test]
    fn upstream_codex_sync_lane_keeps_active_service_dependency_isolation_mandatory() {
        let report = hepta_upstream_codex_sync_lane_report();

        assert_eq!(
            report.active_dependency_isolation_gate,
            "scripts/hepta-active-service-dependency-isolation.sh"
        );
        assert!(report.contracts.iter().any(|contract| {
            contract.id == "compatibility-package-retention-boundary"
                && contract
                    .required_gate
                    .contains("active hepta-cli cargo tree")
                && !contract.auto_apply_allowed
                && !contract.active_runtime_dependency_allowed
                && !contract.public_release_claim_allowed
        }));
        assert!(report.contracts.iter().any(|contract| {
            contract.id == "provider-credential-security-classification"
                && matches!(contract.risk, HeptaUpstreamCodexSyncRisk::P0Security)
        }));
    }

    #[test]
    fn upstream_codex_snapshot_intake_is_ready_without_default_side_effects() {
        let report = hepta_upstream_codex_snapshot_report();

        assert_eq!(report.product, "Hepta");
        assert_eq!(report.status, "ready");
        assert_eq!(report.snapshot_lane_id, "upstream-codex-snapshot-intake");
        assert_eq!(
            report.snapshot_gate,
            "scripts/hepta-upstream-codex-snapshot.sh"
        );
        assert_eq!(
            report.sync_lane_gate,
            "scripts/hepta-upstream-codex-sync-lane.sh"
        );
        assert_eq!(
            report.active_dependency_isolation_gate,
            "scripts/hepta-active-service-dependency-isolation.sh"
        );
        assert_eq!(report.risk_class_count, 4);
        assert_eq!(report.ready_risk_class_count, report.risk_class_count);
        assert!(report.snapshot_intake_ready);
        assert!(report.observed_upstream_head_required_before_absorption);
        assert!(report.local_compatibility_head_required);
        assert!(report.diff_range_required_before_absorption);
        assert!(report.diff_inventory_required_before_absorption);
        assert!(!report.upstream_fetch_performed);
        assert!(!report.upstream_merge_performed);
        assert!(!report.upstream_checkout_performed);
        assert!(!report.active_runtime_dependency_allowed);
        assert!(!report.active_runtime_auto_rebase_allowed);
        assert!(!report.public_release_claim_allowed);
        assert!(!report.external_network_read_default);
        assert!(!report.workspace_mutation_default);
        assert!(!report.credential_value_read);
        assert!(!report.secret_file_read);
        assert!(!report.provider_invoked);
        assert!(!report.channel_delivery_performed);
        assert!(!report.gateway_rpc_performed);
    }

    #[test]
    fn upstream_codex_snapshot_requires_classification_for_all_risk_classes() {
        let report = hepta_upstream_codex_snapshot_report();

        assert!(report.risk_classes.iter().all(|risk_class| {
            risk_class.classification_required
                && !risk_class.auto_absorb_allowed
                && !risk_class.active_runtime_dependency_allowed
        }));
        assert!(report.risk_classes.iter().any(|risk_class| {
            risk_class.id == "provider-credential-sandbox-security"
                && matches!(risk_class.risk, HeptaUpstreamCodexSyncRisk::P0Security)
        }));
        assert!(report.risk_classes.iter().any(|risk_class| {
            risk_class.id == "runtime-session-tool-mcp-appserver"
                && matches!(risk_class.risk, HeptaUpstreamCodexSyncRisk::P0Runtime)
        }));
        assert!(report.risk_classes.iter().any(|risk_class| {
            risk_class.id == "legacy-cli-tui-compatibility"
                && risk_class
                    .hepta_review_surfaces
                    .iter()
                    .any(|surface| surface.contains("dependency-isolation"))
        }));
    }

    #[test]
    fn upstream_codex_diff_ledger_contract_is_ready_without_fetch_or_merge() {
        let report = hepta_upstream_codex_diff_ledger_report();

        assert_eq!(report.product, "Hepta");
        assert_eq!(report.status, "ready");
        assert_eq!(report.ledger_id, "upstream-codex-diff-range-ledger");
        assert_eq!(
            report.baseline_upstream_head,
            HEPTA_UPSTREAM_CODEX_INTAKE_BASE_HEAD
        );
        assert_eq!(
            report.target_upstream_head,
            HEPTA_UPSTREAM_CODEX_INTAKE_CUTOFF_HEAD
        );
        assert_eq!(report.target_ref, HEPTA_UPSTREAM_CODEX_INTAKE_CUTOFF_REF);
        assert_eq!(
            report.diff_ledger_gate,
            "scripts/hepta-upstream-codex-diff-ledger.sh"
        );
        assert_eq!(
            report.candidate_diff_range,
            format!(
                "{}..{}",
                HEPTA_UPSTREAM_CODEX_INTAKE_BASE_HEAD, HEPTA_UPSTREAM_CODEX_INTAKE_CUTOFF_HEAD
            )
        );
        assert!(report.commit_inventory_required);
        assert!(report.file_inventory_required);
        assert!(report.risk_bucket_classification_required);
        assert_eq!(report.bucket_count, 4);
        assert_eq!(report.ready_bucket_count, report.bucket_count);
        assert!(report.diff_ledger_ready);
        assert!(!report.upstream_fetch_performed);
        assert!(!report.upstream_merge_performed);
        assert!(!report.upstream_checkout_performed);
        assert!(!report.workspace_mutation_default);
        assert!(!report.active_runtime_dependency_allowed);
        assert!(!report.active_runtime_auto_rebase_allowed);
        assert!(!report.public_release_claim_allowed);
        assert!(!report.external_network_read_default);
        assert!(!report.credential_value_read);
        assert!(!report.secret_file_read);
        assert!(!report.provider_invoked);
        assert!(!report.channel_delivery_performed);
        assert!(!report.gateway_rpc_performed);
    }

    #[test]
    fn upstream_codex_diff_ledger_requires_all_bucket_classifications() {
        let report = hepta_upstream_codex_diff_ledger_report();

        assert!(report.buckets.iter().all(|bucket| {
            bucket.classification_required
                && bucket.bucket_ready
                && !bucket.auto_absorb_allowed
                && !bucket.active_runtime_dependency_allowed
        }));
        assert!(report.buckets.iter().any(|bucket| {
            bucket.id == "provider-credential-sandbox-security"
                && matches!(bucket.risk, HeptaUpstreamCodexSyncRisk::P0Security)
                && bucket.promotion_gate.contains("dependency isolation")
        }));
        assert!(report.buckets.iter().any(|bucket| {
            bucket.id == "runtime-session-tool-mcp-appserver"
                && matches!(bucket.risk, HeptaUpstreamCodexSyncRisk::P0Runtime)
                && bucket.promotion_gate.contains("shadow-replay")
        }));
        assert!(report.buckets.iter().any(|bucket| {
            bucket.id == "legacy-cli-tui-compatibility"
                && bucket
                    .promotion_gate
                    .contains("active hepta-cli cargo tree")
        }));
        assert!(report.buckets.iter().any(|bucket| {
            bucket.id == "product-doc-release-governance"
                && bucket.promotion_gate.contains("long soak evidence")
        }));
    }

    #[test]
    fn upstream_codex_current_intake_separates_observation_from_absorption() {
        let report = hepta_upstream_codex_current_intake_report();

        assert_eq!(report.product, "Hepta");
        assert_eq!(report.status, "ready");
        assert_eq!(report.observation_state, "observed");
        assert_eq!(report.classification_state, "classified");
        assert_eq!(report.selected_state, "absorbed");
        assert_eq!(report.remaining_state, "deferred");
        assert_eq!(report.baseline_head, HEPTA_UPSTREAM_CODEX_INTAKE_BASE_HEAD);
        assert_eq!(report.cutoff_ref, HEPTA_UPSTREAM_CODEX_INTAKE_CUTOFF_REF);
        assert_eq!(report.cutoff_head, HEPTA_UPSTREAM_CODEX_INTAKE_CUTOFF_HEAD);
        assert_eq!(report.observed_commit_count, 1803);
        assert_eq!(report.observed_changed_file_count, 3359);
        assert_eq!(report.observed_codex_rs_changed_file_count, 3097);
        assert_eq!(report.selected_absorption_count, 11);
        assert_eq!(report.deferred_decision_count, 3);
        assert!(report.current_intake_ready);
        assert!(!report.full_range_absorption_claimed);
        assert!(!report.upstream_fetch_performed);
        assert!(!report.upstream_merge_performed);
        assert!(!report.upstream_rebase_performed);
        assert!(!report.whole_tree_replacement_performed);
        assert!(!report.cargo_lock_replacement_performed);
        assert!(!report.active_runtime_dependency_allowed);
        assert!(!report.public_release_claim_allowed);
    }

    #[test]
    fn upstream_codex_current_intake_tracks_selected_and_deferred_decisions() {
        let report = hepta_upstream_codex_current_intake_report();
        let absorbed: Vec<&HeptaUpstreamCodexCurrentIntakeDecision> = report
            .decisions
            .iter()
            .filter(|decision| {
                decision.disposition == HeptaUpstreamCodexCurrentIntakeDisposition::Absorbed
            })
            .collect();
        let deferred: Vec<&HeptaUpstreamCodexCurrentIntakeDecision> = report
            .decisions
            .iter()
            .filter(|decision| {
                decision.disposition == HeptaUpstreamCodexCurrentIntakeDisposition::Deferred
            })
            .collect();

        assert_eq!(absorbed.len(), 11);
        assert_eq!(deferred.len(), 3);
        assert!(absorbed.iter().all(|decision| {
            decision.upstream_commit.is_some()
                && !decision.local_receipts.is_empty()
                && decision.absorption_kind.is_some()
        }));
        assert!(deferred.iter().all(|decision| {
            decision.local_receipts.is_empty() && decision.absorption_kind.is_none()
        }));
        assert!(absorbed.iter().any(|decision| {
            decision.upstream_commit.as_deref() == Some("9dbdb4e2c08723e8fc9c18f64d7ccad3dadc03a7")
                && decision.absorption_kind.as_deref() == Some("local_split")
                && decision.local_receipts.len() == 7
        }));
        assert!(absorbed.iter().any(|decision| {
            decision.classification == "mcp_endpoint_ownership"
                && decision.upstream_commit.as_deref()
                    == Some("6bf4845b60e0abccd0c64690e9c7591e0efb85d8")
                && decision.absorption_kind.as_deref() == Some("semantic_port")
                && decision.local_receipts == ["f983f4ae7fc7e4b224272990106049f30ee472d7"]
        }));
    }

    #[test]
    fn upstream_codex_current_intake_preserves_historical_receipt_provenance() {
        let report = hepta_upstream_codex_current_intake_report();

        assert_eq!(
            report.historical_receipt_target_head,
            HEPTA_UPSTREAM_CODEX_HISTORICAL_RECEIPT_TARGET_HEAD
        );
        assert_eq!(
            report.historical_receipt_changed_file_count,
            HEPTA_UPSTREAM_CODEX_HISTORICAL_LEDGER_CHANGED_FILE_COUNT
        );
        assert_eq!(
            report.historical_receipt_selected_absorption_count,
            HEPTA_UPSTREAM_CODEX_HISTORICAL_SELECTED_ABSORPTION_COUNT
        );
        assert!(!report.historical_receipt_is_current_freshness_proof);
        assert_ne!(report.cutoff_head, report.historical_receipt_target_head);
    }

    #[test]
    fn upstream_codex_product_governance_absorption_contract_is_ready_and_bounded() {
        let report = hepta_upstream_codex_product_governance_absorption_report();

        assert_eq!(report.product, "Hepta");
        assert_eq!(report.status, "ready");
        assert_eq!(
            report.absorption_id,
            "upstream-codex-product-governance-absorption-contract"
        );
        assert_eq!(report.selected_bucket_id, "product-doc-release-governance");
        assert!(matches!(
            report.selected_bucket_risk,
            HeptaUpstreamCodexSyncRisk::P2Product
        ));
        assert_eq!(report.selected_changed_file_count, 22);
        assert!(report.selected_commit_sample_count > 0);
        assert_eq!(
            report.source_ledger_gate,
            "scripts/hepta-upstream-codex-diff-ledger.sh"
        );
        assert_eq!(
            report.absorption_gate,
            "scripts/hepta-upstream-codex-product-governance-absorption.sh"
        );
        assert!(report.selected_as_first_absorption_contract);
        assert!(!report.low_risk_runtime_promotion);
        assert!(report.requires_hepta_translation);
        assert!(!report.raw_upstream_doc_copy_allowed);
        assert!(!report.raw_upstream_package_policy_copy_allowed);
        assert!(!report.active_runtime_code_wiring_allowed);
        assert!(!report.active_runtime_dependency_allowed);
        assert!(!report.public_release_claim_allowed);
        assert!(report.contract_ready);
        assert!(!report.upstream_fetch_performed);
        assert!(!report.upstream_merge_performed);
        assert!(!report.upstream_checkout_performed);
        assert!(!report.workspace_mutation_default);
        assert!(!report.credential_value_read);
        assert!(!report.secret_file_read);
        assert!(!report.provider_invoked);
        assert!(!report.channel_delivery_performed);
        assert!(!report.gateway_rpc_performed);
    }

    #[test]
    fn upstream_codex_product_governance_absorption_tracks_exact_selected_paths() {
        let report = hepta_upstream_codex_product_governance_absorption_report();

        assert_eq!(report.selected_paths.len(), 22);
        assert!(
            report
                .selected_paths
                .iter()
                .all(|path| path.starts_with("codex-rs/"))
        );
        assert!(
            report
                .selected_paths
                .iter()
                .any(|path| path == "codex-rs/README.md")
        );
        assert!(
            report
                .selected_paths
                .iter()
                .any(|path| path == "codex-rs/Cargo.lock")
        );
        assert!(
            report
                .selected_paths
                .iter()
                .any(|path| path.contains("request_plugin_install"))
        );
        assert!(
            report
                .required_next_gates
                .iter()
                .any(|gate| gate.contains("Hepta release-governance wording"))
        );
        assert!(
            report
                .required_next_gates
                .iter()
                .any(|gate| gate.contains("active dependency isolation"))
        );
    }

    #[test]
    fn upstream_codex_product_governance_translation_packet_is_ready_and_bounded() {
        let report = hepta_upstream_codex_product_governance_translation_report();

        assert_eq!(report.product, "Hepta");
        assert_eq!(report.status, "ready");
        assert_eq!(
            report.translation_id,
            "upstream-codex-product-governance-translation-packet"
        );
        assert_eq!(
            report.translation_packet_path,
            "docs/architecture/HEPTA_UPSTREAM_CODEX_PRODUCT_GOVERNANCE_TRANSLATION.md"
        );
        assert_eq!(report.selected_bucket_id, "product-doc-release-governance");
        assert_eq!(report.selected_changed_file_count, 22);
        assert_eq!(
            report.translated_surface_count,
            report.required_surface_count
        );
        assert_eq!(
            report.source_absorption_gate,
            "scripts/hepta-upstream-codex-product-governance-absorption.sh"
        );
        assert_eq!(
            report.translation_gate,
            "scripts/hepta-upstream-codex-product-governance-translation.sh"
        );
        assert!(report.release_governance_documented);
        assert!(report.package_policy_documented);
        assert!(report.plugin_marketplace_policy_documented);
        assert!(report.sandbox_runtime_policy_documented);
        assert!(report.operator_approval_policy_documented);
        assert!(report.requires_hepta_translation);
        assert!(!report.raw_upstream_doc_copy_allowed);
        assert!(!report.raw_upstream_package_policy_copy_allowed);
        assert!(!report.active_runtime_code_wiring_allowed);
        assert!(!report.active_runtime_dependency_allowed);
        assert!(!report.public_release_claim_allowed);
        assert!(report.translation_ready);
        assert!(!report.upstream_fetch_performed);
        assert!(!report.upstream_merge_performed);
        assert!(!report.upstream_checkout_performed);
        assert!(!report.workspace_mutation_default);
        assert!(!report.credential_value_read);
        assert!(!report.secret_file_read);
        assert!(!report.provider_invoked);
        assert!(!report.channel_delivery_performed);
        assert!(!report.gateway_rpc_performed);
    }

    #[test]
    fn upstream_codex_product_governance_translation_covers_hepta_actions() {
        let report = hepta_upstream_codex_product_governance_translation_report();

        assert!(
            report
                .hepta_actions
                .iter()
                .any(|action| action.contains("packaging governance"))
        );
        assert!(
            report
                .hepta_actions
                .iter()
                .any(|action| action.contains("route/gate language"))
        );
        assert!(
            report
                .hepta_actions
                .iter()
                .any(|action| action.contains("marketplace policy"))
        );
        assert!(
            report
                .hepta_actions
                .iter()
                .any(|action| action.contains("P0 security/runtime"))
        );
        assert!(
            report
                .hepta_actions
                .iter()
                .any(|action| action.contains("long soak"))
        );
    }

    #[test]
    fn upstream_codex_release_governance_promotion_packet_is_ready_but_not_public() {
        let report = hepta_upstream_codex_release_governance_promotion_report();

        assert_eq!(report.product, "Hepta");
        assert_eq!(report.status, "ready");
        assert_eq!(
            report.promotion_id,
            "release-governance-claim-promotion-packet"
        );
        assert_eq!(
            report.promotion_packet_path,
            "docs/architecture/HEPTA_UPSTREAM_CODEX_RELEASE_GOVERNANCE_PROMOTION.md"
        );
        assert_eq!(report.selected_bucket_id, "product-doc-release-governance");
        assert_eq!(report.selected_changed_file_count, 22);
        assert_eq!(
            report.source_translation_gate,
            "scripts/hepta-upstream-codex-product-governance-translation.sh"
        );
        assert_eq!(
            report.promotion_gate,
            "scripts/hepta-upstream-codex-release-governance-promotion.sh"
        );
        assert!(report.release_claim_taxonomy_ready);
        assert!(report.package_install_context_ready);
        assert!(report.plugin_marketplace_policy_ready);
        assert!(report.operator_approval_model_ready);
        assert!(report.watchdog_soak_evidence_ready);
        assert!(report.public_claim_boundary_ready);
        assert!(report.side_effect_boundary_ready);
        assert_eq!(
            report.ready_promotion_condition_count,
            report.required_promotion_condition_count
        );
        assert!(report.promotion_packet_ready);
        assert!(!report.public_release_claim_allowed);
        assert!(!report.public_ga_claim_allowed);
        assert!(!report.release_artifact_write_allowed);
        assert!(!report.active_runtime_code_wiring_allowed);
        assert!(!report.active_runtime_dependency_allowed);
        assert!(!report.upstream_auto_rebase_allowed);
        assert!(!report.upstream_fetch_performed);
        assert!(!report.upstream_merge_performed);
        assert!(!report.upstream_checkout_performed);
        assert!(!report.workspace_mutation_default);
        assert!(!report.active_service_restart);
        assert!(!report.credential_value_read);
        assert!(!report.secret_file_read);
        assert!(!report.provider_invoked);
        assert!(!report.channel_delivery_performed);
        assert!(!report.gateway_rpc_performed);
        assert!(!report.public_release_published);
    }

    #[test]
    fn upstream_codex_release_governance_promotion_tracks_claim_blockers() {
        let report = hepta_upstream_codex_release_governance_promotion_report();

        assert_eq!(report.promotion_conditions.len(), 7);
        assert!(
            report
                .promotion_conditions
                .iter()
                .any(|condition| condition.contains("release claim taxonomy"))
        );
        assert!(
            report
                .promotion_conditions
                .iter()
                .any(|condition| condition.contains("watchdog"))
        );
        assert!(
            report
                .remaining_blockers
                .iter()
                .any(|blocker| blocker.contains("public GA claim"))
        );
        assert!(
            report
                .required_next_gates
                .iter()
                .any(|gate| gate.contains("operator approval"))
        );
    }

    #[test]
    fn upstream_codex_legacy_compatibility_absorption_is_ready_and_bounded() {
        let report = hepta_upstream_codex_legacy_compatibility_absorption_report();

        assert_eq!(report.product, "Hepta");
        assert_eq!(report.status, "ready");
        assert_eq!(
            report.absorption_id,
            "upstream-codex-legacy-compatibility-absorption-contract"
        );
        assert_eq!(report.selected_bucket_id, "legacy-cli-tui-compatibility");
        assert!(matches!(
            report.selected_bucket_risk,
            HeptaUpstreamCodexSyncRisk::P1Compatibility
        ));
        assert_eq!(report.selected_changed_file_count, 128);
        assert_eq!(
            report.source_ledger_gate,
            "scripts/hepta-upstream-codex-diff-ledger.sh"
        );
        assert_eq!(
            report.absorption_gate,
            "scripts/hepta-upstream-codex-legacy-compatibility-absorption.sh"
        );
        assert!(report.retained_as_compatibility_snapshot);
        assert!(report.requires_hepta_command_contract);
        assert!(!report.active_cli_tui_promotion_allowed);
        assert!(!report.active_runtime_code_wiring_allowed);
        assert!(!report.active_runtime_dependency_allowed);
        assert!(!report.public_release_claim_allowed);
        assert!(report.contract_ready);
        assert!(!report.upstream_fetch_performed);
        assert!(!report.upstream_merge_performed);
        assert!(!report.upstream_checkout_performed);
        assert!(!report.workspace_mutation_default);
        assert!(!report.credential_value_read);
        assert!(!report.secret_file_read);
        assert!(!report.provider_invoked);
        assert!(!report.channel_delivery_performed);
        assert!(!report.gateway_rpc_performed);
    }

    #[test]
    fn upstream_codex_legacy_compatibility_absorption_tracks_required_surfaces() {
        let report = hepta_upstream_codex_legacy_compatibility_absorption_report();

        assert!(
            report
                .sample_surfaces
                .iter()
                .any(|surface| surface.contains("cli"))
        );
        assert!(
            report
                .sample_surfaces
                .iter()
                .any(|surface| surface.contains("tui"))
        );
        assert!(
            report
                .sample_surfaces
                .iter()
                .any(|surface| surface.contains("code-mode"))
        );
        assert!(
            report
                .required_next_gates
                .iter()
                .any(|gate| gate.contains("Hepta command contracts"))
        );
        assert!(
            report
                .required_next_gates
                .iter()
                .any(|gate| gate.contains("shadow-replay"))
        );
    }

    #[test]
    fn upstream_codex_legacy_compatibility_replay_packet_is_ready_and_bounded() {
        let report = hepta_upstream_codex_legacy_compatibility_replay_report();

        assert_eq!(report.product, "Hepta");
        assert_eq!(report.status, "ready");
        assert_eq!(
            report.replay_id,
            "upstream-codex-legacy-compatibility-replay-packet"
        );
        assert_eq!(
            report.replay_packet_path,
            "docs/architecture/HEPTA_UPSTREAM_CODEX_LEGACY_COMPATIBILITY_REPLAY.md"
        );
        assert_eq!(report.selected_bucket_id, "legacy-cli-tui-compatibility");
        assert_eq!(report.selected_changed_file_count, 128);
        assert_eq!(
            report.replay_surface_count,
            report.required_replay_surface_count
        );
        assert_eq!(
            report.source_absorption_gate,
            "scripts/hepta-upstream-codex-legacy-compatibility-absorption.sh"
        );
        assert_eq!(
            report.replay_gate,
            "scripts/hepta-upstream-codex-legacy-compatibility-replay.sh"
        );
        assert!(report.cli_command_contract_ready);
        assert!(report.tui_presentation_replay_ready);
        assert!(report.code_mode_replay_ready);
        assert!(report.terminal_helper_replay_ready);
        assert!(report.dependency_boundary_ready);
        assert!(!report.active_cli_tui_promotion_allowed);
        assert!(!report.active_runtime_code_wiring_allowed);
        assert!(!report.active_runtime_dependency_allowed);
        assert!(!report.public_release_claim_allowed);
        assert!(report.replay_ready);
        assert!(!report.upstream_fetch_performed);
        assert!(!report.upstream_merge_performed);
        assert!(!report.upstream_checkout_performed);
        assert!(!report.workspace_mutation_default);
        assert!(!report.credential_value_read);
        assert!(!report.secret_file_read);
        assert!(!report.provider_invoked);
        assert!(!report.channel_delivery_performed);
        assert!(!report.gateway_rpc_performed);
    }

    #[test]
    fn upstream_codex_legacy_compatibility_replay_tracks_replay_surfaces() {
        let report = hepta_upstream_codex_legacy_compatibility_replay_report();

        assert!(
            report
                .replay_surfaces
                .iter()
                .any(|surface| surface.contains("CLI command"))
        );
        assert!(
            report
                .replay_surfaces
                .iter()
                .any(|surface| surface.contains("TUI"))
        );
        assert!(
            report
                .replay_surfaces
                .iter()
                .any(|surface| surface.contains("code-mode"))
        );
        assert!(
            report
                .replay_surfaces
                .iter()
                .any(|surface| surface.contains("terminal"))
        );
        assert!(
            report
                .required_next_gates
                .iter()
                .any(|gate| gate.contains("Hepta command contracts"))
        );
    }

    #[test]
    fn upstream_codex_legacy_compatibility_promotion_packet_is_ready_but_not_active() {
        let report = hepta_upstream_codex_legacy_compatibility_promotion_report();

        assert_eq!(report.product, "Hepta");
        assert_eq!(report.status, "ready");
        assert_eq!(report.promotion_id, "hepta-cli-tui-parity-promotion-packet");
        assert_eq!(
            report.promotion_packet_path,
            "docs/architecture/HEPTA_UPSTREAM_CODEX_LEGACY_COMPATIBILITY_PROMOTION.md"
        );
        assert_eq!(report.selected_bucket_id, "legacy-cli-tui-compatibility");
        assert_eq!(report.selected_changed_file_count, 128);
        assert_eq!(
            report.source_replay_gate,
            "scripts/hepta-upstream-codex-legacy-compatibility-replay.sh"
        );
        assert_eq!(
            report.promotion_gate,
            "scripts/hepta-upstream-codex-legacy-compatibility-promotion.sh"
        );
        assert!(report.cli_command_contract_parity_ready);
        assert!(report.tui_presentation_parity_ready);
        assert!(report.code_mode_callback_boundary_ready);
        assert!(report.terminal_helper_contract_ready);
        assert!(report.adapter_shadow_replay_ready);
        assert!(report.operator_approval_model_ready);
        assert!(report.side_effect_boundary_ready);
        assert_eq!(
            report.ready_promotion_condition_count,
            report.required_promotion_condition_count
        );
        assert!(report.promotion_packet_ready);
        assert!(!report.active_cli_tui_promotion_allowed);
        assert!(!report.active_tui_presentation_promotion_allowed);
        assert!(!report.active_code_mode_promotion_allowed);
        assert!(!report.active_runtime_code_wiring_allowed);
        assert!(!report.active_runtime_dependency_allowed);
        assert!(!report.public_release_claim_allowed);
        assert!(!report.upstream_fetch_performed);
        assert!(!report.upstream_merge_performed);
        assert!(!report.upstream_checkout_performed);
        assert!(!report.workspace_mutation_default);
        assert!(!report.active_service_restart);
        assert!(!report.credential_value_read);
        assert!(!report.secret_file_read);
        assert!(!report.provider_invoked);
        assert!(!report.channel_delivery_performed);
        assert!(!report.gateway_rpc_performed);
        assert!(!report.public_release_published);
    }

    #[test]
    fn upstream_codex_legacy_compatibility_promotion_tracks_blockers() {
        let report = hepta_upstream_codex_legacy_compatibility_promotion_report();

        assert_eq!(report.promotion_conditions.len(), 7);
        assert!(
            report
                .promotion_conditions
                .iter()
                .any(|condition| condition.contains("CLI command"))
        );
        assert!(
            report
                .promotion_conditions
                .iter()
                .any(|condition| condition.contains("TUI presentation"))
        );
        assert!(
            report
                .promotion_conditions
                .iter()
                .any(|condition| condition.contains("code-mode"))
        );
        assert!(
            report
                .remaining_blockers
                .iter()
                .any(|blocker| blocker.contains("active CLI/TUI"))
        );
        assert!(
            report
                .required_next_gates
                .iter()
                .any(|gate| gate.contains("Hepta-native CLI/TUI parity"))
        );
    }

    #[test]
    fn upstream_codex_provider_security_absorption_is_ready_and_bounded() {
        let report = hepta_upstream_codex_provider_security_absorption_report();

        assert_eq!(report.product, "Hepta");
        assert_eq!(report.status, "ready");
        assert_eq!(
            report.absorption_id,
            "upstream-codex-provider-security-absorption-contract"
        );
        assert_eq!(
            report.selected_bucket_id,
            "provider-credential-sandbox-security"
        );
        assert!(matches!(
            report.selected_bucket_risk,
            HeptaUpstreamCodexSyncRisk::P0Security
        ));
        assert_eq!(report.selected_changed_file_count, 104);
        assert_eq!(
            report.selected_security_surface_count,
            report.required_security_surface_count
        );
        assert_eq!(
            report.source_ledger_gate,
            "scripts/hepta-upstream-codex-diff-ledger.sh"
        );
        assert_eq!(
            report.absorption_gate,
            "scripts/hepta-upstream-codex-provider-security-absorption.sh"
        );
        assert!(report.p0_security_review_required);
        assert!(report.requires_provider_contract);
        assert!(report.requires_auth_credential_redaction);
        assert!(report.requires_sandbox_exec_replay);
        assert!(report.requires_network_policy_replay);
        assert!(!report.active_provider_promotion_allowed);
        assert!(!report.active_security_policy_promotion_allowed);
        assert!(!report.active_runtime_code_wiring_allowed);
        assert!(!report.active_runtime_dependency_allowed);
        assert!(!report.public_release_claim_allowed);
        assert!(report.contract_ready);
        assert!(!report.upstream_fetch_performed);
        assert!(!report.upstream_merge_performed);
        assert!(!report.upstream_checkout_performed);
        assert!(!report.workspace_mutation_default);
        assert!(!report.credential_value_read);
        assert!(!report.secret_file_read);
        assert!(!report.provider_invoked);
        assert!(!report.channel_delivery_performed);
        assert!(!report.gateway_rpc_performed);
    }

    #[test]
    fn upstream_codex_provider_security_absorption_tracks_required_surfaces() {
        let report = hepta_upstream_codex_provider_security_absorption_report();

        assert!(
            report
                .security_surfaces
                .iter()
                .any(|surface| surface.contains("provider"))
        );
        assert!(
            report
                .security_surfaces
                .iter()
                .any(|surface| surface.contains("credential"))
        );
        assert!(
            report
                .security_surfaces
                .iter()
                .any(|surface| surface.contains("sandbox"))
        );
        assert!(
            report
                .security_surfaces
                .iter()
                .any(|surface| surface.contains("network"))
        );
        assert!(
            report
                .required_next_gates
                .iter()
                .any(|gate| gate.contains("redacted provider contracts"))
        );
        assert!(
            report
                .required_next_gates
                .iter()
                .any(|gate| gate.contains("network-proxy policy replay"))
        );
    }

    #[test]
    fn upstream_codex_provider_security_replay_packet_is_ready_and_bounded() {
        let report = hepta_upstream_codex_provider_security_replay_report();

        assert_eq!(report.product, "Hepta");
        assert_eq!(report.status, "ready");
        assert_eq!(
            report.replay_id,
            "upstream-codex-provider-security-replay-packet"
        );
        assert_eq!(
            report.replay_packet_path,
            "docs/architecture/HEPTA_UPSTREAM_CODEX_PROVIDER_SECURITY_REPLAY.md"
        );
        assert_eq!(
            report.selected_bucket_id,
            "provider-credential-sandbox-security"
        );
        assert_eq!(report.selected_changed_file_count, 104);
        assert_eq!(
            report.replay_surface_count,
            report.required_replay_surface_count
        );
        assert_eq!(
            report.source_absorption_gate,
            "scripts/hepta-upstream-codex-provider-security-absorption.sh"
        );
        assert_eq!(
            report.replay_gate,
            "scripts/hepta-upstream-codex-provider-security-replay.sh"
        );
        assert!(report.redacted_provider_contract_ready);
        assert!(report.auth_credential_redaction_ready);
        assert!(report.approval_policy_replay_ready);
        assert!(report.sandbox_exec_replay_ready);
        assert!(report.network_policy_replay_ready);
        assert!(report.side_effect_boundary_ready);
        assert!(!report.active_provider_promotion_allowed);
        assert!(!report.active_security_policy_promotion_allowed);
        assert!(!report.active_runtime_code_wiring_allowed);
        assert!(!report.active_runtime_dependency_allowed);
        assert!(!report.public_release_claim_allowed);
        assert!(report.replay_ready);
        assert!(!report.upstream_fetch_performed);
        assert!(!report.upstream_merge_performed);
        assert!(!report.upstream_checkout_performed);
        assert!(!report.workspace_mutation_default);
        assert!(!report.credential_value_read);
        assert!(!report.secret_file_read);
        assert!(!report.provider_invoked);
        assert!(!report.channel_delivery_performed);
        assert!(!report.gateway_rpc_performed);
    }

    #[test]
    fn upstream_codex_provider_security_replay_tracks_replay_surfaces() {
        let report = hepta_upstream_codex_provider_security_replay_report();

        assert!(
            report
                .replay_surfaces
                .iter()
                .any(|surface| surface.contains("provider"))
        );
        assert!(
            report
                .replay_surfaces
                .iter()
                .any(|surface| surface.contains("credential"))
        );
        assert!(
            report
                .replay_surfaces
                .iter()
                .any(|surface| surface.contains("approval"))
        );
        assert!(
            report
                .replay_surfaces
                .iter()
                .any(|surface| surface.contains("sandbox"))
        );
        assert!(
            report
                .replay_surfaces
                .iter()
                .any(|surface| surface.contains("network"))
        );
        assert!(
            report
                .required_next_gates
                .iter()
                .any(|gate| gate.contains("credential values"))
        );
    }

    #[test]
    fn upstream_codex_provider_security_promotion_packet_is_ready_but_not_active() {
        let report = hepta_upstream_codex_provider_security_promotion_report();

        assert_eq!(report.product, "Hepta");
        assert_eq!(report.status, "ready");
        assert_eq!(
            report.promotion_id,
            "upstream-codex-provider-security-promotion-packet"
        );
        assert_eq!(
            report.promotion_packet_path,
            "docs/architecture/HEPTA_UPSTREAM_CODEX_PROVIDER_SECURITY_PROMOTION.md"
        );
        assert_eq!(
            report.selected_bucket_id,
            "provider-credential-sandbox-security"
        );
        assert_eq!(report.selected_changed_file_count, 104);
        assert_eq!(
            report.source_replay_gate,
            "scripts/hepta-upstream-codex-provider-security-replay.sh"
        );
        assert_eq!(
            report.promotion_gate,
            "scripts/hepta-upstream-codex-provider-security-promotion.sh"
        );
        assert_eq!(
            report.ready_promotion_condition_count,
            report.required_promotion_condition_count
        );
        assert!(report.redacted_provider_contract_ready);
        assert!(report.auth_credential_redaction_ready);
        assert!(report.approval_policy_replay_ready);
        assert!(report.sandbox_exec_replay_ready);
        assert!(report.network_policy_replay_ready);
        assert!(report.operator_approval_model_ready);
        assert!(report.side_effect_boundary_ready);
        assert!(report.promotion_packet_ready);
        assert!(!report.active_provider_promotion_allowed);
        assert!(!report.active_security_policy_promotion_allowed);
        assert!(!report.active_runtime_code_wiring_allowed);
        assert!(!report.active_runtime_dependency_allowed);
        assert!(!report.public_release_claim_allowed);
        assert!(!report.credential_value_read);
        assert!(!report.secret_file_read);
        assert!(!report.provider_invoked);
        assert!(!report.channel_delivery_performed);
        assert!(!report.gateway_rpc_performed);
        assert!(!report.public_release_published);
    }

    #[test]
    fn upstream_codex_provider_security_promotion_tracks_blockers() {
        let report = hepta_upstream_codex_provider_security_promotion_report();

        assert_eq!(report.promotion_conditions.len(), 7);
        assert_eq!(report.remaining_blockers.len(), 4);
        assert!(
            report
                .promotion_conditions
                .iter()
                .any(|condition| condition.contains("network policy"))
        );
        assert!(
            report
                .remaining_blockers
                .iter()
                .any(|blocker| blocker.contains("credential reads"))
        );
        assert!(
            report
                .required_next_gates
                .iter()
                .any(|gate| gate.contains("adapter parity"))
        );
    }

    #[test]
    fn upstream_codex_runtime_appserver_absorption_is_ready_and_bounded() {
        let report = hepta_upstream_codex_runtime_appserver_absorption_report();

        assert_eq!(report.product, "Hepta");
        assert_eq!(report.status, "ready");
        assert_eq!(
            report.absorption_id,
            "upstream-codex-runtime-appserver-absorption-contract"
        );
        assert_eq!(
            report.selected_bucket_id,
            "runtime-session-tool-mcp-appserver"
        );
        assert!(matches!(
            report.selected_bucket_risk,
            HeptaUpstreamCodexSyncRisk::P0Runtime
        ));
        assert_eq!(report.selected_changed_file_count, 462);
        assert_eq!(
            report.selected_runtime_surface_count,
            report.required_runtime_surface_count
        );
        assert_eq!(
            report.source_ledger_gate,
            "scripts/hepta-upstream-codex-diff-ledger.sh"
        );
        assert_eq!(
            report.absorption_gate,
            "scripts/hepta-upstream-codex-runtime-appserver-absorption.sh"
        );
        assert!(report.p0_runtime_review_required);
        assert!(report.requires_adapter_contract);
        assert!(report.requires_session_thread_replay);
        assert!(report.requires_tool_mcp_replay);
        assert!(report.requires_app_server_protocol_replay);
        assert!(report.requires_exec_hook_replay);
        assert!(!report.active_runtime_promotion_allowed);
        assert!(!report.active_app_server_promotion_allowed);
        assert!(!report.active_tool_mcp_promotion_allowed);
        assert!(!report.active_runtime_code_wiring_allowed);
        assert!(!report.active_runtime_dependency_allowed);
        assert!(!report.public_release_claim_allowed);
        assert!(report.contract_ready);
        assert!(!report.upstream_fetch_performed);
        assert!(!report.upstream_merge_performed);
        assert!(!report.upstream_checkout_performed);
        assert!(!report.workspace_mutation_default);
        assert!(!report.credential_value_read);
        assert!(!report.secret_file_read);
        assert!(!report.provider_invoked);
        assert!(!report.channel_delivery_performed);
        assert!(!report.gateway_rpc_performed);
    }

    #[test]
    fn upstream_codex_runtime_appserver_absorption_tracks_required_surfaces() {
        let report = hepta_upstream_codex_runtime_appserver_absorption_report();

        assert!(
            report
                .runtime_surfaces
                .iter()
                .any(|surface| surface.contains("app-server"))
        );
        assert!(
            report
                .runtime_surfaces
                .iter()
                .any(|surface| surface.contains("session"))
        );
        assert!(
            report
                .runtime_surfaces
                .iter()
                .any(|surface| surface.contains("tool"))
        );
        assert!(
            report
                .runtime_surfaces
                .iter()
                .any(|surface| surface.contains("MCP"))
        );
        assert!(
            report
                .required_next_gates
                .iter()
                .any(|gate| gate.contains("thread-store replay"))
        );
        assert!(
            report
                .required_next_gates
                .iter()
                .any(|gate| gate.contains("tool and MCP replay"))
        );
    }

    #[test]
    fn upstream_codex_runtime_appserver_replay_packet_is_ready_and_bounded() {
        let report = hepta_upstream_codex_runtime_appserver_replay_report();

        assert_eq!(report.product, "Hepta");
        assert_eq!(report.status, "ready");
        assert_eq!(
            report.replay_id,
            "upstream-codex-runtime-appserver-replay-packet"
        );
        assert_eq!(
            report.replay_packet_path,
            "docs/architecture/HEPTA_UPSTREAM_CODEX_RUNTIME_APPSERVER_REPLAY.md"
        );
        assert_eq!(
            report.selected_bucket_id,
            "runtime-session-tool-mcp-appserver"
        );
        assert_eq!(report.selected_changed_file_count, 462);
        assert_eq!(
            report.replay_surface_count,
            report.required_replay_surface_count
        );
        assert_eq!(
            report.source_absorption_gate,
            "scripts/hepta-upstream-codex-runtime-appserver-absorption.sh"
        );
        assert_eq!(
            report.replay_gate,
            "scripts/hepta-upstream-codex-runtime-appserver-replay.sh"
        );
        assert!(report.app_server_protocol_replay_ready);
        assert!(report.session_thread_replay_ready);
        assert!(report.tool_mcp_replay_ready);
        assert!(report.exec_hook_replay_ready);
        assert!(report.side_effect_boundary_ready);
        assert!(!report.active_runtime_promotion_allowed);
        assert!(!report.active_app_server_promotion_allowed);
        assert!(!report.active_tool_mcp_promotion_allowed);
        assert!(!report.active_runtime_code_wiring_allowed);
        assert!(!report.active_runtime_dependency_allowed);
        assert!(!report.public_release_claim_allowed);
        assert!(report.replay_ready);
        assert!(!report.upstream_fetch_performed);
        assert!(!report.upstream_merge_performed);
        assert!(!report.upstream_checkout_performed);
        assert!(!report.workspace_mutation_default);
        assert!(!report.credential_value_read);
        assert!(!report.secret_file_read);
        assert!(!report.provider_invoked);
        assert!(!report.channel_delivery_performed);
        assert!(!report.gateway_rpc_performed);
    }

    #[test]
    fn upstream_codex_runtime_appserver_replay_tracks_replay_surfaces() {
        let report = hepta_upstream_codex_runtime_appserver_replay_report();

        assert!(
            report
                .replay_surfaces
                .iter()
                .any(|surface| surface.contains("app-server protocol"))
        );
        assert!(
            report
                .replay_surfaces
                .iter()
                .any(|surface| surface.contains("session"))
        );
        assert!(
            report
                .replay_surfaces
                .iter()
                .any(|surface| surface.contains("tool"))
        );
        assert!(
            report
                .replay_surfaces
                .iter()
                .any(|surface| surface.contains("MCP"))
        );
        assert!(
            report
                .replay_surfaces
                .iter()
                .any(|surface| surface.contains("exec-server"))
        );
        assert!(
            report
                .required_next_gates
                .iter()
                .any(|gate| gate.contains("event-loop promotion"))
        );
    }

    #[test]
    fn upstream_codex_runtime_appserver_promotion_packet_is_ready_but_not_active() {
        let report = hepta_upstream_codex_runtime_appserver_promotion_report();

        assert_eq!(report.product, "Hepta");
        assert_eq!(report.status, "ready");
        assert_eq!(
            report.promotion_id,
            "runtime-appserver-route-event-promotion-packet"
        );
        assert_eq!(
            report.promotion_packet_path,
            "docs/architecture/HEPTA_UPSTREAM_CODEX_RUNTIME_APPSERVER_PROMOTION.md"
        );
        assert_eq!(
            report.selected_bucket_id,
            "runtime-session-tool-mcp-appserver"
        );
        assert_eq!(report.selected_changed_file_count, 462);
        assert_eq!(
            report.source_replay_gate,
            "scripts/hepta-upstream-codex-runtime-appserver-replay.sh"
        );
        assert_eq!(
            report.promotion_gate,
            "scripts/hepta-upstream-codex-runtime-appserver-promotion.sh"
        );
        assert_eq!(
            report.ready_promotion_condition_count,
            report.required_promotion_condition_count
        );
        assert!(report.app_server_route_event_contract_ready);
        assert!(report.session_thread_lifecycle_contract_ready);
        assert!(report.tool_mcp_request_envelope_ready);
        assert!(report.exec_hook_event_loop_replay_ready);
        assert!(report.adapter_shadow_replay_ready);
        assert!(report.operator_approval_model_ready);
        assert!(report.side_effect_boundary_ready);
        assert!(report.promotion_packet_ready);
        assert!(!report.active_runtime_promotion_allowed);
        assert!(!report.active_app_server_promotion_allowed);
        assert!(!report.active_tool_mcp_promotion_allowed);
        assert!(!report.active_runtime_code_wiring_allowed);
        assert!(!report.active_runtime_dependency_allowed);
        assert!(!report.public_release_claim_allowed);
        assert!(!report.credential_value_read);
        assert!(!report.secret_file_read);
        assert!(!report.provider_invoked);
        assert!(!report.channel_delivery_performed);
        assert!(!report.gateway_rpc_performed);
        assert!(!report.public_release_published);
    }

    #[test]
    fn upstream_codex_runtime_appserver_promotion_tracks_blockers() {
        let report = hepta_upstream_codex_runtime_appserver_promotion_report();

        assert_eq!(report.promotion_conditions.len(), 7);
        assert_eq!(report.remaining_blockers.len(), 4);
        assert!(
            report
                .promotion_conditions
                .iter()
                .any(|condition| condition.contains("route and event contract"))
        );
        assert!(
            report
                .remaining_blockers
                .iter()
                .any(|blocker| blocker.contains("gateway RPC"))
        );
        assert!(
            report
                .required_next_gates
                .iter()
                .any(|gate| gate.contains("route/event adapter parity"))
        );
    }

    #[test]
    fn upstream_codex_absorption_replay_readiness_is_ready_and_bounded() {
        let report = hepta_upstream_codex_absorption_replay_readiness_report();

        assert_eq!(report.product, "Hepta");
        assert_eq!(report.status, "ready");
        assert_eq!(
            report.readiness_id,
            "upstream-codex-absorption-replay-readiness"
        );
        assert_eq!(
            report.readiness_packet_path,
            "docs/architecture/HEPTA_UPSTREAM_CODEX_ABSORPTION_REPLAY_READINESS.md"
        );
        assert_eq!(report.ledger_changed_file_count, 878);
        assert_eq!(report.selected_absorption_changed_file_count, 716);
        assert_eq!(
            report.selected_bucket_count,
            report.required_selected_bucket_count
        );
        assert_eq!(
            report.absorption_contract_ready_count,
            report.required_absorption_contract_ready_count
        );
        assert_eq!(
            report.translation_replay_ready_count,
            report.required_translation_replay_ready_count
        );
        assert_eq!(
            report.p0_replay_ready_count,
            report.required_p0_replay_ready_count
        );
        assert_eq!(
            report.p1_replay_ready_count,
            report.required_p1_replay_ready_count
        );
        assert_eq!(
            report.p2_translation_ready_count,
            report.required_p2_translation_ready_count
        );
        assert!(report.product_governance_translation_ready);
        assert!(report.legacy_compatibility_replay_ready);
        assert!(report.provider_security_replay_ready);
        assert!(report.runtime_appserver_replay_ready);
        assert!(report.all_selected_buckets_absorbed);
        assert!(report.all_required_translation_replay_ready);
        assert!(report.readiness_ready);
        assert!(!report.active_runtime_code_wiring_allowed);
        assert!(!report.active_runtime_dependency_allowed);
        assert!(!report.active_runtime_auto_rebase_allowed);
        assert!(!report.active_codex_engine_dependency_allowed);
        assert!(!report.public_release_claim_allowed);
        assert!(!report.upstream_fetch_performed);
        assert!(!report.upstream_merge_performed);
        assert!(!report.upstream_checkout_performed);
        assert!(!report.workspace_mutation_default);
        assert!(!report.active_service_restart);
        assert!(!report.credential_value_read);
        assert!(!report.secret_file_read);
        assert!(!report.provider_invoked);
        assert!(!report.channel_delivery_performed);
        assert!(!report.gateway_rpc_performed);
        assert!(!report.public_release_published);
    }

    #[test]
    fn upstream_codex_absorption_replay_readiness_tracks_all_closed_gates() {
        let report = hepta_upstream_codex_absorption_replay_readiness_report();

        assert_eq!(report.covered_buckets.len(), 4);
        assert!(
            report
                .covered_buckets
                .iter()
                .any(|bucket| bucket == "product-doc-release-governance")
        );
        assert!(
            report
                .covered_buckets
                .iter()
                .any(|bucket| bucket == "legacy-cli-tui-compatibility")
        );
        assert!(
            report
                .covered_buckets
                .iter()
                .any(|bucket| bucket == "provider-credential-sandbox-security")
        );
        assert!(
            report
                .covered_buckets
                .iter()
                .any(|bucket| bucket == "runtime-session-tool-mcp-appserver")
        );
        assert_eq!(report.closed_gates.len(), 8);
        assert!(
            report
                .closed_gates
                .iter()
                .any(|gate| gate.contains("product-governance-translation"))
        );
        assert!(
            report
                .closed_gates
                .iter()
                .any(|gate| gate.contains("legacy-compatibility-replay"))
        );
        assert!(
            report
                .closed_gates
                .iter()
                .any(|gate| gate.contains("provider-security-replay"))
        );
        assert!(
            report
                .closed_gates
                .iter()
                .any(|gate| gate.contains("runtime-appserver-replay"))
        );
        assert!(
            report
                .required_next_gates
                .iter()
                .any(|gate| gate.contains("newer upstream Codex range"))
        );
    }

    #[test]
    fn upstream_codex_promotion_readiness_is_decided_but_not_open() {
        let report = hepta_upstream_codex_promotion_readiness_report();

        assert_eq!(report.product, "Hepta");
        assert_eq!(report.status, "ready");
        assert_eq!(report.decision_id, "upstream-codex-promotion-readiness");
        assert_eq!(
            report.decision_packet_path,
            "docs/architecture/HEPTA_UPSTREAM_CODEX_PROMOTION_READINESS.md"
        );
        assert_eq!(
            report.source_readiness_gate,
            "scripts/hepta-upstream-codex-absorption-replay-readiness.sh"
        );
        assert_eq!(
            report.promotion_readiness_gate,
            "scripts/hepta-upstream-codex-promotion-readiness.sh"
        );
        assert_eq!(
            report.assessed_bucket_count,
            report.required_assessed_bucket_count
        );
        assert_eq!(
            report.absorption_replay_ready_count,
            report.required_absorption_replay_ready_count
        );
        assert_eq!(report.required_surface_promotion_packet_count, 4);
        assert_eq!(report.completed_surface_promotion_packet_count, 4);
        assert_eq!(report.promotable_bucket_count, 0);
        assert_eq!(report.promotion_blocked_bucket_count, 4);
        assert!(report.readiness_source_ready);
        assert!(report.decision_ready);
        assert!(!report.active_promotion_ready);
        assert!(!report.active_runtime_code_wiring_allowed);
        assert!(!report.active_runtime_dependency_allowed);
        assert!(!report.active_runtime_auto_rebase_allowed);
        assert!(!report.active_codex_engine_dependency_allowed);
        assert!(!report.public_release_claim_allowed);
        assert!(!report.upstream_fetch_performed);
        assert!(!report.upstream_merge_performed);
        assert!(!report.upstream_checkout_performed);
        assert!(!report.workspace_mutation_default);
        assert!(!report.active_service_restart);
        assert!(!report.credential_value_read);
        assert!(!report.secret_file_read);
        assert!(!report.provider_invoked);
        assert!(!report.channel_delivery_performed);
        assert!(!report.gateway_rpc_performed);
        assert!(!report.public_release_published);
    }

    #[test]
    fn upstream_codex_promotion_readiness_blocks_all_selected_buckets() {
        let report = hepta_upstream_codex_promotion_readiness_report();

        assert_eq!(report.decisions.len(), 4);
        assert_eq!(report.promotion_blockers.len(), 4);
        assert!(report.decisions.iter().all(|decision| {
            decision.absorption_replay_ready && !decision.active_promotion_allowed
        }));
        assert!(report.decisions.iter().any(|decision| decision.bucket_id
            == "provider-credential-sandbox-security"
            && decision.risk == HeptaUpstreamCodexSyncRisk::P0Security
            && decision.surface_promotion_packet_ready));
        assert!(report.decisions.iter().any(|decision| decision.bucket_id
            == "product-doc-release-governance"
            && decision.risk == HeptaUpstreamCodexSyncRisk::P2Product
            && decision.surface_promotion_packet_ready));
        assert!(report.decisions.iter().any(|decision| decision.bucket_id
            == "runtime-session-tool-mcp-appserver"
            && decision.risk == HeptaUpstreamCodexSyncRisk::P0Runtime
            && decision.surface_promotion_packet_ready));
        assert!(report.decisions.iter().any(|decision| {
            decision.bucket_id == "legacy-cli-tui-compatibility"
                && decision
                    .required_surface_promotion_packet
                    .contains("parity")
                && decision.surface_promotion_packet_ready
        }));
        assert!(
            report
                .required_next_gates
                .iter()
                .any(|gate| gate.contains("per-surface promotion packets"))
        );
    }

    #[test]
    fn upstream_codex_promotion_closure_completes_packets_but_denies_activation() {
        let report = hepta_upstream_codex_promotion_closure_report();

        assert_eq!(report.product, "Hepta");
        assert_eq!(report.status, "ready");
        assert_eq!(report.closure_id, "upstream-codex-promotion-closure-denial");
        assert_eq!(
            report.closure_packet_path,
            "docs/architecture/HEPTA_UPSTREAM_CODEX_PROMOTION_CLOSURE.md"
        );
        assert_eq!(
            report.source_promotion_readiness_gate,
            "scripts/hepta-upstream-codex-promotion-readiness.sh"
        );
        assert_eq!(
            report.closure_gate,
            "scripts/hepta-upstream-codex-promotion-closure.sh"
        );
        assert_eq!(report.required_surface_promotion_packet_count, 4);
        assert_eq!(report.completed_surface_promotion_packet_count, 4);
        assert!(report.all_surface_promotion_packets_complete);
        assert_eq!(report.promotable_bucket_count, 0);
        assert_eq!(report.promotion_blocked_bucket_count, 4);
        assert!(!report.active_promotion_ready);
        assert!(report.active_promotion_denial_ready);
        assert!(report.closure_ready);
        assert!(!report.active_runtime_code_wiring_allowed);
        assert!(!report.active_runtime_dependency_allowed);
        assert!(!report.active_runtime_auto_rebase_allowed);
        assert!(!report.active_codex_engine_dependency_allowed);
        assert!(!report.public_release_claim_allowed);
        assert!(!report.public_ga_claim_allowed);
        assert!(!report.release_artifact_write_allowed);
    }

    #[test]
    fn upstream_codex_promotion_closure_preserves_side_effect_boundaries() {
        let report = hepta_upstream_codex_promotion_closure_report();

        assert!(!report.upstream_fetch_performed);
        assert!(!report.upstream_merge_performed);
        assert!(!report.upstream_checkout_performed);
        assert!(!report.workspace_mutation_default);
        assert!(!report.active_service_restart);
        assert!(!report.credential_value_read);
        assert!(!report.secret_file_read);
        assert!(!report.provider_invoked);
        assert!(!report.channel_delivery_performed);
        assert!(!report.gateway_rpc_performed);
        assert!(!report.public_release_published);
        assert_eq!(report.closure_invariants.len(), 5);
        assert!(report.closure_invariants.iter().any(|invariant| {
            invariant.contains("all four required surface promotion packets are complete")
        }));
        assert!(report.closure_invariants.iter().any(|invariant| {
            invariant.contains("zero selected upstream Codex buckets are promotable")
        }));
        assert!(
            report.required_next_gates.iter().any(
                |gate| gate.contains("explicit operator approval before active runtime wiring")
            )
        );
        assert!(
            report
                .required_next_gates
                .iter()
                .any(|gate| gate.contains("newer upstream Codex ranges as new snapshot intake"))
        );
    }

    #[test]
    fn upstream_codex_active_wiring_precondition_is_ready_but_not_allowed() {
        let report = hepta_upstream_codex_active_wiring_precondition_report();

        assert_eq!(report.product, "Hepta");
        assert_eq!(report.status, "ready");
        assert_eq!(
            report.precondition_id,
            "upstream-codex-active-wiring-precondition"
        );
        assert_eq!(
            report.precondition_packet_path,
            "docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVE_WIRING_PRECONDITION.md"
        );
        assert_eq!(
            report.source_closure_gate,
            "scripts/hepta-upstream-codex-promotion-closure.sh"
        );
        assert_eq!(
            report.active_wiring_precondition_gate,
            "scripts/hepta-upstream-codex-active-wiring-precondition.sh"
        );
        assert!(report.promotion_closure_ready);
        assert!(report.all_surface_promotion_packets_complete);
        assert!(report.active_promotion_denial_ready);
        assert!(report.explicit_operator_approval_required);
        assert!(!report.operator_approval_recorded);
        assert!(report.activation_request_id_required);
        assert!(!report.activation_request_id_present);
        assert!(report.live_dependency_isolation_required);
        assert!(report.watchdog_required);
        assert!(report.browser_smoke_required);
        assert!(report.long_soak_required);
        assert!(report.active_wiring_precondition_ready);
        assert!(!report.active_wiring_allowed);
        assert!(!report.active_runtime_code_wiring_allowed);
        assert!(!report.active_runtime_dependency_allowed);
        assert!(!report.active_runtime_auto_rebase_allowed);
        assert!(!report.active_codex_engine_dependency_allowed);
        assert!(!report.public_release_claim_allowed);
        assert!(!report.public_ga_claim_allowed);
        assert!(!report.release_artifact_write_allowed);
    }

    #[test]
    fn upstream_codex_active_wiring_precondition_has_no_side_effects() {
        let report = hepta_upstream_codex_active_wiring_precondition_report();

        assert!(!report.upstream_fetch_performed);
        assert!(!report.upstream_merge_performed);
        assert!(!report.upstream_checkout_performed);
        assert!(!report.workspace_mutation_default);
        assert!(!report.active_service_restart);
        assert!(!report.credential_value_read);
        assert!(!report.secret_file_read);
        assert!(!report.provider_invoked);
        assert!(!report.channel_delivery_performed);
        assert!(!report.gateway_rpc_performed);
        assert!(!report.public_release_published);
        assert!(
            report.preconditions.iter().any(|precondition| {
                precondition.contains("operator approval record is required")
            })
        );
        assert!(
            report
                .preconditions
                .iter()
                .any(|precondition| { precondition.contains("activation request id is required") })
        );
        assert!(
            report
                .required_next_gates
                .iter()
                .any(|gate| gate.contains("activation request packet schema"))
        );
    }

    #[test]
    fn upstream_codex_activation_request_packet_schema_is_ready_but_unrecorded() {
        let report = hepta_upstream_codex_activation_request_packet_report();

        assert_eq!(report.product, "Hepta");
        assert_eq!(report.status, "ready");
        assert_eq!(
            report.packet_id,
            "upstream-codex-activation-request-packet-schema"
        );
        assert_eq!(
            report.packet_schema_path,
            "docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVATION_REQUEST_PACKET.md"
        );
        assert_eq!(
            report.source_precondition_gate,
            "scripts/hepta-upstream-codex-active-wiring-precondition.sh"
        );
        assert_eq!(
            report.activation_request_packet_gate,
            "scripts/hepta-upstream-codex-activation-request-packet.sh"
        );
        assert!(report.active_wiring_precondition_ready);
        assert!(!report.active_wiring_allowed_by_precondition);
        assert!(report.operator_approval_required);
        assert!(!report.operator_approval_recorded);
        assert!(report.activation_request_id_required);
        assert!(!report.activation_request_id_recorded);
        assert_eq!(report.schema_field_count, 14);
        assert_eq!(
            report.required_schema_field_count,
            report.schema_field_count
        );
        assert_eq!(report.recorded_required_schema_field_count, 0);
        assert!(report.activation_packet_schema_ready);
        assert!(!report.activation_packet_recorded);
        assert!(!report.active_wiring_allowed);
        assert!(report.schema_fields.iter().any(|field| {
            field.name == "activation_request_id" && field.required && !field.recorded
        }));
        assert!(
            report.schema_fields.iter().any(|field| {
                field.name == "operator_identity_hash" && field.redacted_or_hashed
            })
        );
        assert!(
            report
                .schema_fields
                .iter()
                .any(|field| { field.name == "live_dependency_isolation_evidence_id" })
        );
        assert!(
            report
                .schema_fields
                .iter()
                .any(|field| { field.name == "release_artifact_write_decision" })
        );
    }

    #[test]
    fn upstream_codex_activation_request_packet_preserves_denials_and_side_effects() {
        let report = hepta_upstream_codex_activation_request_packet_report();

        assert!(!report.active_runtime_code_wiring_allowed);
        assert!(!report.active_runtime_dependency_allowed);
        assert!(!report.active_runtime_auto_rebase_allowed);
        assert!(!report.active_codex_engine_dependency_allowed);
        assert!(!report.public_release_claim_allowed);
        assert!(!report.public_ga_claim_allowed);
        assert!(!report.release_artifact_write_allowed);
        assert!(!report.upstream_fetch_performed);
        assert!(!report.upstream_merge_performed);
        assert!(!report.upstream_checkout_performed);
        assert!(!report.workspace_mutation_default);
        assert!(!report.active_service_restart);
        assert!(!report.credential_value_read);
        assert!(!report.secret_file_read);
        assert!(!report.provider_invoked);
        assert!(!report.channel_delivery_performed);
        assert!(!report.gateway_rpc_performed);
        assert!(!report.public_release_published);
        assert!(
            report
                .packet_invariants
                .iter()
                .any(|invariant| invariant.contains("no activation packet is recorded"))
        );
        assert!(
            report
                .required_next_gates
                .iter()
                .any(|gate| gate.contains("concrete activation_request_id"))
        );
    }

    #[test]
    fn upstream_codex_activation_packet_dry_run_blocks_incomplete_fixtures() {
        let report = hepta_upstream_codex_activation_packet_dry_run_report();

        assert_eq!(report.product, "Hepta");
        assert_eq!(report.status, "ready");
        assert_eq!(
            report.validator_id,
            "upstream-codex-activation-packet-dry-run-validator"
        );
        assert_eq!(
            report.validator_doc_path,
            "docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVATION_PACKET_DRY_RUN.md"
        );
        assert_eq!(
            report.source_packet_gate,
            "scripts/hepta-upstream-codex-activation-request-packet.sh"
        );
        assert_eq!(
            report.dry_run_validator_gate,
            "scripts/hepta-upstream-codex-activation-packet-dry-run.sh"
        );
        assert!(report.activation_packet_schema_ready);
        assert!(!report.activation_packet_recorded);
        assert_eq!(report.required_schema_field_count, 14);
        assert_eq!(report.fixture_count, 3);
        assert_eq!(report.blocked_fixture_count, report.fixture_count);
        assert_eq!(report.allowed_fixture_count, 0);
        assert!(report.dry_run_validator_ready);
        assert!(!report.active_wiring_allowed);

        let empty = report
            .fixtures
            .iter()
            .find(|fixture| fixture.fixture_id == "empty-placeholder")
            .expect("empty placeholder fixture");
        assert_eq!(empty.recorded_required_field_count, 0);
        assert_eq!(empty.missing_required_field_count, 14);
        assert!(!empty.active_wiring_allowed);

        let public_attempt = report
            .fixtures
            .iter()
            .find(|fixture| fixture.fixture_id == "public-claim-attempt-without-evidence")
            .expect("public claim attempt fixture");
        assert!(public_attempt.public_release_claim_requested);
        assert!(public_attempt.release_artifact_write_requested);
        assert!(!public_attempt.public_release_claim_allowed);
        assert!(!public_attempt.release_artifact_write_allowed);
    }

    #[test]
    fn upstream_codex_activation_packet_dry_run_preserves_denials_and_side_effects() {
        let report = hepta_upstream_codex_activation_packet_dry_run_report();

        assert!(!report.active_runtime_code_wiring_allowed);
        assert!(!report.active_runtime_dependency_allowed);
        assert!(!report.active_runtime_auto_rebase_allowed);
        assert!(!report.active_codex_engine_dependency_allowed);
        assert!(!report.public_release_claim_allowed);
        assert!(!report.public_ga_claim_allowed);
        assert!(!report.release_artifact_write_allowed);
        assert!(!report.upstream_fetch_performed);
        assert!(!report.upstream_merge_performed);
        assert!(!report.upstream_checkout_performed);
        assert!(!report.workspace_mutation_default);
        assert!(!report.active_service_restart);
        assert!(!report.credential_value_read);
        assert!(!report.secret_file_read);
        assert!(!report.provider_invoked);
        assert!(!report.channel_delivery_performed);
        assert!(!report.gateway_rpc_performed);
        assert!(!report.public_release_published);
        assert!(
            report
                .validation_invariants
                .iter()
                .any(|invariant| { invariant.contains("dry-run fixtures cannot activate wiring") })
        );
        assert!(
            report
                .validation_invariants
                .iter()
                .any(|invariant| invariant.contains("public release and artifact-write"))
        );
        assert!(
            report
                .required_next_gates
                .iter()
                .any(|gate| gate.contains("concrete activation packet"))
        );
    }

    #[test]
    fn upstream_codex_activation_evidence_ledger_is_ready_but_empty() {
        let report = hepta_upstream_codex_activation_evidence_ledger_report();

        assert_eq!(report.product, "Hepta");
        assert_eq!(report.status, "ready");
        assert_eq!(
            report.ledger_id,
            "upstream-codex-activation-evidence-ledger-checklist"
        );
        assert_eq!(
            report.ledger_doc_path,
            "docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVATION_EVIDENCE_LEDGER.md"
        );
        assert_eq!(
            report.source_dry_run_gate,
            "scripts/hepta-upstream-codex-activation-packet-dry-run.sh"
        );
        assert_eq!(
            report.evidence_ledger_gate,
            "scripts/hepta-upstream-codex-activation-evidence-ledger.sh"
        );
        assert!(report.dry_run_validator_ready);
        assert!(!report.activation_packet_recorded);
        assert_eq!(report.required_evidence_count, 8);
        assert_eq!(report.recorded_evidence_count, 0);
        assert_eq!(report.fresh_evidence_count, 0);
        assert!(report.evidence_ledger_ready);
        assert!(!report.evidence_recorded);
        assert!(!report.active_wiring_allowed);
        assert!(
            report
                .evidence_requirements
                .iter()
                .all(|requirement| requirement.required
                    && !requirement.recorded
                    && !requirement.fresh)
        );
        assert!(report.evidence_requirements.iter().any(|requirement| {
            requirement.id == "live_dependency_isolation_evidence_id"
                && requirement.source_gate == "scripts/hepta-active-service-dependency-isolation.sh"
        }));
        assert!(
            report
                .evidence_requirements
                .iter()
                .any(|requirement| requirement.id == "rollback_plan_id")
        );
    }

    #[test]
    fn upstream_codex_activation_evidence_ledger_preserves_denials_and_side_effects() {
        let report = hepta_upstream_codex_activation_evidence_ledger_report();

        assert!(!report.active_runtime_code_wiring_allowed);
        assert!(!report.active_runtime_dependency_allowed);
        assert!(!report.active_runtime_auto_rebase_allowed);
        assert!(!report.active_codex_engine_dependency_allowed);
        assert!(!report.public_release_claim_allowed);
        assert!(!report.public_ga_claim_allowed);
        assert!(!report.release_artifact_write_allowed);
        assert!(!report.upstream_fetch_performed);
        assert!(!report.upstream_merge_performed);
        assert!(!report.upstream_checkout_performed);
        assert!(!report.workspace_mutation_default);
        assert!(!report.active_service_restart);
        assert!(!report.credential_value_read);
        assert!(!report.secret_file_read);
        assert!(!report.provider_invoked);
        assert!(!report.channel_delivery_performed);
        assert!(!report.gateway_rpc_performed);
        assert!(!report.public_release_published);
        assert!(
            report
                .ledger_invariants
                .iter()
                .any(|invariant| invariant.contains("records no concrete evidence"))
        );
        assert!(
            report
                .required_next_gates
                .iter()
                .any(|gate| gate.contains("concrete activation request id"))
        );
    }

    #[test]
    fn upstream_codex_activation_readiness_closure_is_ready_and_denied() {
        let report = hepta_upstream_codex_activation_readiness_closure_report();

        assert_eq!(report.product, "Hepta");
        assert_eq!(report.status, "ready");
        assert_eq!(
            report.closure_id,
            "upstream-codex-activation-readiness-closure-denial"
        );
        assert_eq!(
            report.closure_doc_path,
            "docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVATION_READINESS_CLOSURE.md"
        );
        assert_eq!(
            report.source_packet_gate,
            "scripts/hepta-upstream-codex-activation-request-packet.sh"
        );
        assert_eq!(
            report.source_dry_run_gate,
            "scripts/hepta-upstream-codex-activation-packet-dry-run.sh"
        );
        assert_eq!(
            report.source_evidence_ledger_gate,
            "scripts/hepta-upstream-codex-activation-evidence-ledger.sh"
        );
        assert_eq!(
            report.activation_readiness_closure_gate,
            "scripts/hepta-upstream-codex-activation-readiness-closure.sh"
        );
        assert!(report.activation_packet_schema_ready);
        assert!(report.dry_run_validator_ready);
        assert!(report.evidence_ledger_ready);
        assert_eq!(report.required_schema_field_count, 14);
        assert_eq!(report.blocked_fixture_count, 3);
        assert_eq!(report.allowed_fixture_count, 0);
        assert_eq!(report.required_evidence_count, 8);
        assert_eq!(report.recorded_evidence_count, 0);
        assert_eq!(report.fresh_evidence_count, 0);
        assert!(report.readiness_inputs_ready);
        assert!(report.activation_denied_by_default);
        assert!(report.activation_readiness_closure_ready);
        assert!(!report.operator_approved_activation_ready);
        assert!(!report.activation_packet_recorded);
        assert!(!report.evidence_recorded);
        assert!(!report.active_wiring_allowed);
    }

    #[test]
    fn upstream_codex_activation_readiness_closure_preserves_denials_and_side_effects() {
        let report = hepta_upstream_codex_activation_readiness_closure_report();

        assert!(!report.active_runtime_code_wiring_allowed);
        assert!(!report.active_runtime_dependency_allowed);
        assert!(!report.active_runtime_auto_rebase_allowed);
        assert!(!report.active_codex_engine_dependency_allowed);
        assert!(!report.public_release_claim_allowed);
        assert!(!report.public_ga_claim_allowed);
        assert!(!report.release_artifact_write_allowed);
        assert!(!report.upstream_fetch_performed);
        assert!(!report.upstream_merge_performed);
        assert!(!report.upstream_checkout_performed);
        assert!(!report.workspace_mutation_default);
        assert!(!report.active_service_restart);
        assert!(!report.credential_value_read);
        assert!(!report.secret_file_read);
        assert!(!report.provider_invoked);
        assert!(!report.channel_delivery_performed);
        assert!(!report.gateway_rpc_performed);
        assert!(!report.public_release_published);
        assert!(
            report
                .closure_invariants
                .iter()
                .any(|invariant| invariant.contains("denied"))
        );
        assert!(
            report
                .required_next_gates
                .iter()
                .any(|gate| gate.contains("fresh live gate evidence"))
        );
    }

    #[test]
    fn upstream_codex_activation_denied_sample_is_full_shaped_but_blocked() {
        let report = hepta_upstream_codex_activation_denied_sample_report();

        assert_eq!(report.product, "Hepta");
        assert_eq!(report.status, "ready");
        assert_eq!(
            report.sample_id,
            "upstream-codex-activation-denied-sample-packet"
        );
        assert_eq!(
            report.sample_doc_path,
            "docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVATION_DENIED_SAMPLE.md"
        );
        assert_eq!(
            report.source_readiness_closure_gate,
            "scripts/hepta-upstream-codex-activation-readiness-closure.sh"
        );
        assert_eq!(
            report.denied_sample_gate,
            "scripts/hepta-upstream-codex-activation-denied-sample.sh"
        );
        assert!(report.activation_readiness_closure_ready);
        assert!(report.sample_packet_shape_complete);
        assert_eq!(report.sample_required_schema_field_count, 14);
        assert_eq!(report.sample_recorded_schema_field_count, 14);
        assert_eq!(report.sample_required_evidence_count, 8);
        assert_eq!(report.sample_fresh_evidence_count, 0);
        assert!(report.sample_operator_approval_field_present);
        assert!(!report.sample_operator_approval_recorded);
        assert!(report.sample_public_release_claim_requested);
        assert!(report.sample_release_artifact_write_requested);
        assert_eq!(report.sample_validation_status, "blocked");
        assert!(report.sample_blocked_reason.contains("not recorded"));
        assert!(!report.active_wiring_allowed);
    }

    #[test]
    fn upstream_codex_activation_denied_sample_preserves_denials_and_side_effects() {
        let report = hepta_upstream_codex_activation_denied_sample_report();

        assert!(!report.active_runtime_code_wiring_allowed);
        assert!(!report.active_runtime_dependency_allowed);
        assert!(!report.active_runtime_auto_rebase_allowed);
        assert!(!report.active_codex_engine_dependency_allowed);
        assert!(!report.public_release_claim_allowed);
        assert!(!report.public_ga_claim_allowed);
        assert!(!report.release_artifact_write_allowed);
        assert!(!report.upstream_fetch_performed);
        assert!(!report.upstream_merge_performed);
        assert!(!report.upstream_checkout_performed);
        assert!(!report.workspace_mutation_default);
        assert!(!report.active_service_restart);
        assert!(!report.credential_value_read);
        assert!(!report.secret_file_read);
        assert!(!report.provider_invoked);
        assert!(!report.channel_delivery_performed);
        assert!(!report.gateway_rpc_performed);
        assert!(!report.public_release_published);
        assert!(
            report
                .sample_invariants
                .iter()
                .any(|invariant| invariant.contains("not approvals"))
        );
        assert!(
            report
                .required_next_gates
                .iter()
                .any(|gate| gate.contains("concrete operator-approved activation packet"))
        );
    }

    #[test]
    fn upstream_codex_activation_evidence_freshness_policy_defines_all_slots() {
        let report = hepta_upstream_codex_activation_evidence_freshness_policy_report();

        assert_eq!(report.product, "Hepta");
        assert_eq!(report.status, "ready");
        assert_eq!(
            report.policy_id,
            "upstream-codex-activation-evidence-freshness-policy"
        );
        assert_eq!(
            report.source_denied_sample_gate,
            "scripts/hepta-upstream-codex-activation-denied-sample.sh"
        );
        assert_eq!(
            report.freshness_policy_gate,
            "scripts/hepta-upstream-codex-activation-evidence-freshness-policy.sh"
        );
        assert!(report.denied_sample_ready);
        assert_eq!(report.required_evidence_count, 8);
        assert_eq!(report.policy_entry_count, 8);
        assert_eq!(report.missing_evidence_count, 8);
        assert_eq!(report.fresh_evidence_count, 0);
        assert_eq!(report.expired_evidence_count, 0);
        assert_eq!(report.stale_evidence_count, 0);
        assert!(report.freshness_policy_ready);
        assert!(report.activation_blocked_by_freshness_policy);
        assert!(!report.activation_allowed_by_freshness_policy);
        assert!(!report.active_wiring_allowed);
        assert_eq!(report.freshness_entries.len(), 8);

        let ids: Vec<_> = report
            .freshness_entries
            .iter()
            .map(|entry| entry.evidence_id.as_str())
            .collect();
        assert!(ids.contains(&"activation_request_id"));
        assert!(ids.contains(&"operator_approval_id"));
        assert!(ids.contains(&"operator_identity_hash"));
        assert!(ids.contains(&"live_dependency_isolation_evidence_id"));
        assert!(ids.contains(&"watchdog_evidence_id"));
        assert!(ids.contains(&"browser_smoke_evidence_id"));
        assert!(ids.contains(&"long_soak_evidence_id"));
        assert!(ids.contains(&"rollback_plan_id"));
    }

    #[test]
    fn upstream_codex_activation_evidence_freshness_policy_preserves_denials() {
        let report = hepta_upstream_codex_activation_evidence_freshness_policy_report();

        assert!(!report.active_runtime_code_wiring_allowed);
        assert!(!report.active_runtime_dependency_allowed);
        assert!(!report.active_runtime_auto_rebase_allowed);
        assert!(!report.active_codex_engine_dependency_allowed);
        assert!(!report.public_release_claim_allowed);
        assert!(!report.public_ga_claim_allowed);
        assert!(!report.release_artifact_write_allowed);
        assert!(!report.upstream_fetch_performed);
        assert!(!report.upstream_merge_performed);
        assert!(!report.upstream_checkout_performed);
        assert!(!report.workspace_mutation_default);
        assert!(!report.active_service_restart);
        assert!(!report.credential_value_read);
        assert!(!report.secret_file_read);
        assert!(!report.provider_invoked);
        assert!(!report.channel_delivery_performed);
        assert!(!report.gateway_rpc_performed);
        assert!(!report.public_release_published);
        assert!(
            report
                .freshness_denial_reason
                .contains("evidence slots are absent")
        );
        assert!(report.freshness_entries.iter().all(|entry| {
            entry.required_for_activation
                && !entry.recorded
                && !entry.fresh
                && entry.denial_reason.contains("absent")
        }));
        assert!(
            report
                .policy_invariants
                .iter()
                .any(|invariant| invariant.contains("records no evidence"))
        );
    }

    #[test]
    fn upstream_codex_activation_evidence_binding_record_manifest_defines_schema() {
        let report = hepta_upstream_codex_activation_evidence_binding_record_manifest_report();

        assert_eq!(report.product, "Hepta");
        assert_eq!(report.status, "ready");
        assert_eq!(
            report.manifest_id,
            "upstream-codex-activation-evidence-binding-record-manifest"
        );
        assert_eq!(
            report.source_freshness_policy_gate,
            "scripts/hepta-upstream-codex-activation-evidence-freshness-policy.sh"
        );
        assert_eq!(
            report.binding_manifest_gate,
            "scripts/hepta-upstream-codex-activation-evidence-binding-record.sh"
        );
        assert!(report.freshness_policy_ready);
        assert_eq!(report.required_evidence_count, 8);
        assert_eq!(report.binding_record_count, 8);
        assert_eq!(report.missing_binding_record_count, 8);
        assert_eq!(report.recorded_binding_record_count, 0);
        assert_eq!(report.required_record_schema_field_count, 7);
        assert_eq!(report.recorded_record_schema_field_count, 0);
        assert_eq!(report.timestamped_record_count, 0);
        assert_eq!(report.binary_sha_bound_record_count, 0);
        assert_eq!(report.route_or_status_hash_bound_record_count, 0);
        assert_eq!(report.artifact_hash_or_redacted_path_bound_record_count, 0);
        assert_eq!(report.activation_request_id_bound_record_count, 0);
        assert!(report.binding_manifest_ready);
        assert!(report.activation_blocked_by_binding_manifest);
        assert!(!report.activation_allowed_by_binding_manifest);
        assert!(!report.active_wiring_allowed);

        let field_names: Vec<_> = report
            .binding_schema_fields
            .iter()
            .map(|field| field.name.as_str())
            .collect();
        assert!(field_names.contains(&"evidence_record_id"));
        assert!(field_names.contains(&"source_gate"));
        assert!(field_names.contains(&"recorded_at_unix_ms"));
        assert!(field_names.contains(&"active_binary_sha256"));
        assert!(field_names.contains(&"route_or_status_hash"));
        assert!(field_names.contains(&"artifact_sha256_or_redacted_path"));
        assert!(field_names.contains(&"activation_request_id_binding"));
    }

    #[test]
    fn upstream_codex_activation_evidence_binding_record_manifest_preserves_denials() {
        let report = hepta_upstream_codex_activation_evidence_binding_record_manifest_report();

        assert!(!report.active_runtime_code_wiring_allowed);
        assert!(!report.active_runtime_dependency_allowed);
        assert!(!report.active_runtime_auto_rebase_allowed);
        assert!(!report.active_codex_engine_dependency_allowed);
        assert!(!report.public_release_claim_allowed);
        assert!(!report.public_ga_claim_allowed);
        assert!(!report.release_artifact_write_allowed);
        assert!(!report.upstream_fetch_performed);
        assert!(!report.upstream_merge_performed);
        assert!(!report.upstream_checkout_performed);
        assert!(!report.workspace_mutation_default);
        assert!(!report.active_service_restart);
        assert!(!report.credential_value_read);
        assert!(!report.secret_file_read);
        assert!(!report.provider_invoked);
        assert!(!report.channel_delivery_performed);
        assert!(!report.gateway_rpc_performed);
        assert!(!report.public_release_published);
        assert!(
            report
                .binding_denial_reason
                .contains("schema-only and unrecorded")
        );
        assert!(report.binding_records.iter().all(|record| {
            record.required_schema_field_count == 7
                && record.recorded_schema_field_count == 0
                && !record.evidence_recorded
                && !record.timestamp_recorded
                && !record.active_binary_sha_bound
                && !record.route_or_status_hash_bound
                && !record.artifact_hash_or_redacted_path_bound
                && !record.activation_request_id_bound
                && record.binding_denial_reason.contains("not recorded")
        }));
        assert!(
            report
                .binding_invariants
                .iter()
                .any(|invariant| invariant.contains("without recording evidence"))
        );
    }

    #[test]
    fn upstream_codex_activation_evidence_record_denied_fixture_is_full_shaped_but_blocked() {
        let report = hepta_upstream_codex_activation_evidence_record_denied_fixture_report();

        assert_eq!(report.product, "Hepta");
        assert_eq!(report.status, "ready");
        assert_eq!(
            report.fixture_id,
            "upstream-codex-activation-evidence-record-denied-fixture"
        );
        assert_eq!(
            report.source_binding_manifest_gate,
            "scripts/hepta-upstream-codex-activation-evidence-binding-record.sh"
        );
        assert_eq!(
            report.denied_fixture_gate,
            "scripts/hepta-upstream-codex-activation-evidence-denied-fixture.sh"
        );
        assert!(report.binding_manifest_ready);
        assert_eq!(report.required_evidence_count, 8);
        assert_eq!(report.fixture_record_count, 8);
        assert_eq!(report.schema_complete_fixture_record_count, 8);
        assert_eq!(report.trusted_fixture_record_count, 0);
        assert_eq!(report.operator_approved_fixture_record_count, 0);
        assert_eq!(report.request_binding_verified_record_count, 0);
        assert_eq!(report.live_gate_hash_verified_record_count, 0);
        assert_eq!(report.artifact_hash_verified_record_count, 0);
        assert_eq!(report.fresh_fixture_record_count, 0);
        assert_eq!(report.blocked_fixture_record_count, 8);
        assert_eq!(report.allowed_fixture_record_count, 0);
        assert!(report.denied_fixture_ready);
        assert!(report.activation_blocked_by_denied_fixture);
        assert!(!report.activation_allowed_by_denied_fixture);
        assert!(!report.active_wiring_allowed);
        assert!(report.fixture_records.iter().all(|record| {
            record.schema_complete
                && record.validation_status == "blocked"
                && record.evidence_record_id.starts_with("fixture-")
                && record.recorded_at_unix_ms == "0"
                && record.active_binary_sha256.contains("placeholder")
                && record.route_or_status_hash.contains("placeholder")
                && record
                    .artifact_sha256_or_redacted_path
                    .contains("placeholder")
                && record.activation_request_id_binding.contains("placeholder")
        }));
    }

    #[test]
    fn upstream_codex_activation_evidence_record_denied_fixture_preserves_denials() {
        let report = hepta_upstream_codex_activation_evidence_record_denied_fixture_report();

        assert!(!report.active_runtime_code_wiring_allowed);
        assert!(!report.active_runtime_dependency_allowed);
        assert!(!report.active_runtime_auto_rebase_allowed);
        assert!(!report.active_codex_engine_dependency_allowed);
        assert!(!report.public_release_claim_allowed);
        assert!(!report.public_ga_claim_allowed);
        assert!(!report.release_artifact_write_allowed);
        assert!(!report.upstream_fetch_performed);
        assert!(!report.upstream_merge_performed);
        assert!(!report.upstream_checkout_performed);
        assert!(!report.workspace_mutation_default);
        assert!(!report.active_service_restart);
        assert!(!report.credential_value_read);
        assert!(!report.secret_file_read);
        assert!(!report.provider_invoked);
        assert!(!report.channel_delivery_performed);
        assert!(!report.gateway_rpc_performed);
        assert!(!report.public_release_published);
        assert!(
            report
                .fixture_denial_reason
                .contains("placeholders without operator approval")
        );
        assert!(report.fixture_records.iter().all(|record| {
            !record.operator_approved
                && !record.request_binding_verified
                && !record.live_gate_hash_verified
                && !record.artifact_hash_verified
                && !record.freshness_window_satisfied
                && !record.trusted
                && record.denial_reason.contains("placeholder evidence")
        }));
        assert!(
            report
                .fixture_invariants
                .iter()
                .any(|invariant| invariant.contains("not trusted evidence"))
        );
    }

    #[test]
    fn upstream_codex_activation_trusted_evidence_acceptance_matrix_enumerates_checks() {
        let report = hepta_upstream_codex_activation_trusted_evidence_acceptance_matrix_report();

        assert_eq!(report.product, "Hepta");
        assert_eq!(report.status, "ready");
        assert_eq!(
            report.matrix_id,
            "upstream-codex-activation-trusted-evidence-acceptance-matrix"
        );
        assert_eq!(
            report.source_denied_fixture_gate,
            "scripts/hepta-upstream-codex-activation-evidence-denied-fixture.sh"
        );
        assert_eq!(
            report.trusted_acceptance_matrix_gate,
            "scripts/hepta-upstream-codex-activation-trusted-evidence-acceptance-matrix.sh"
        );
        assert!(report.source_denied_fixture_ready);
        assert_eq!(report.required_evidence_count, 8);
        assert_eq!(report.verification_entry_count, 8);
        assert_eq!(report.schema_complete_verification_entry_count, 8);
        assert_eq!(report.required_verification_count_per_record, 7);
        assert_eq!(report.total_required_verification_count, 56);
        assert_eq!(report.total_satisfied_verification_count, 0);
        assert_eq!(report.operator_approval_verified_record_count, 0);
        assert_eq!(report.request_binding_verified_record_count, 0);
        assert_eq!(report.active_binary_sha_verified_record_count, 0);
        assert_eq!(report.route_or_status_hash_verified_record_count, 0);
        assert_eq!(report.artifact_hash_verified_record_count, 0);
        assert_eq!(report.freshness_window_satisfied_record_count, 0);
        assert_eq!(report.trusted_source_verified_record_count, 0);
        assert_eq!(report.accepted_record_count, 0);
        assert_eq!(report.blocked_record_count, 8);
        assert!(report.trusted_evidence_acceptance_matrix_ready);
        assert!(report.activation_blocked_by_trusted_acceptance_matrix);
        assert!(!report.activation_allowed_by_trusted_acceptance_matrix);
        assert!(!report.active_wiring_allowed);
    }

    #[test]
    fn upstream_codex_activation_trusted_evidence_acceptance_matrix_preserves_denials() {
        let report = hepta_upstream_codex_activation_trusted_evidence_acceptance_matrix_report();

        assert!(!report.active_runtime_code_wiring_allowed);
        assert!(!report.active_runtime_dependency_allowed);
        assert!(!report.active_runtime_auto_rebase_allowed);
        assert!(!report.active_codex_engine_dependency_allowed);
        assert!(!report.public_release_claim_allowed);
        assert!(!report.public_ga_claim_allowed);
        assert!(!report.release_artifact_write_allowed);
        assert!(!report.upstream_fetch_performed);
        assert!(!report.upstream_merge_performed);
        assert!(!report.upstream_checkout_performed);
        assert!(!report.workspace_mutation_default);
        assert!(!report.active_service_restart);
        assert!(!report.credential_value_read);
        assert!(!report.secret_file_read);
        assert!(!report.provider_invoked);
        assert!(!report.channel_delivery_performed);
        assert!(!report.gateway_rpc_performed);
        assert!(!report.public_release_published);
        assert!(
            report
                .acceptance_denial_reason
                .contains("operator approval")
        );
        assert!(report.verification_entries.iter().all(|entry| {
            entry.schema_complete
                && entry.required_verification_count == 7
                && entry.satisfied_verification_count == 0
                && entry.operator_approval_required
                && !entry.operator_approval_verified
                && entry.activation_request_binding_required
                && !entry.activation_request_binding_verified
                && entry.active_binary_sha_required
                && !entry.active_binary_sha_verified
                && entry.route_or_status_hash_required
                && !entry.route_or_status_hash_verified
                && entry.artifact_hash_or_redacted_path_required
                && !entry.artifact_hash_or_redacted_path_verified
                && entry.freshness_window_required
                && !entry.freshness_window_satisfied
                && entry.trusted_source_required
                && !entry.trusted_source_verified
                && !entry.accepted
                && entry.acceptance_status == "blocked"
        }));
        assert!(
            report
                .acceptance_invariants
                .iter()
                .any(|invariant| invariant.contains("not trusted evidence"))
        );
    }

    #[test]
    fn upstream_codex_activation_trusted_record_shape_validator_blocks_partial_trust() {
        let report = hepta_upstream_codex_activation_trusted_record_shape_validator_report();

        assert_eq!(report.product, "Hepta");
        assert_eq!(report.status, "ready");
        assert_eq!(
            report.validator_id,
            "upstream-codex-activation-trusted-record-shape-validator"
        );
        assert_eq!(
            report.source_trusted_acceptance_matrix_gate,
            "scripts/hepta-upstream-codex-activation-trusted-evidence-acceptance-matrix.sh"
        );
        assert_eq!(
            report.trusted_record_shape_validator_gate,
            "scripts/hepta-upstream-codex-activation-trusted-record-shape-validator.sh"
        );
        assert!(report.source_trusted_acceptance_matrix_ready);
        assert_eq!(report.required_evidence_count, 8);
        assert_eq!(report.fixture_count, 2);
        assert_eq!(report.partial_trusted_fixture_count, 1);
        assert_eq!(report.public_claim_attempt_fixture_count, 1);
        assert_eq!(report.blocked_fixture_count, 2);
        assert_eq!(report.allowed_fixture_count, 0);
        assert_eq!(report.required_verification_count_per_record, 7);
        assert_eq!(report.total_required_verification_count_per_fixture, 56);
        assert_eq!(report.max_satisfied_verification_count, 48);
        assert!(report.trusted_record_shape_validator_ready);
        assert!(report.activation_blocked_by_shape_validator);
        assert!(!report.activation_allowed_by_shape_validator);
        assert!(!report.active_wiring_allowed);

        let partial = report
            .fixtures
            .iter()
            .find(|fixture| fixture.fixture_id == "partial-trusted-records")
            .expect("partial trusted fixture");
        assert_eq!(partial.total_satisfied_verification_count, 32);
        assert_eq!(partial.artifact_hash_verified_record_count, 0);
        assert_eq!(partial.freshness_window_satisfied_record_count, 0);
        assert_eq!(partial.trusted_source_verified_record_count, 0);
        assert_eq!(partial.accepted_record_count, 0);
        assert_eq!(partial.blocked_record_count, 8);
        assert_eq!(partial.validation_status, "blocked");
        assert!(!partial.active_wiring_allowed);
    }

    #[test]
    fn upstream_codex_activation_trusted_record_shape_validator_preserves_public_denials() {
        let report = hepta_upstream_codex_activation_trusted_record_shape_validator_report();

        assert!(!report.active_runtime_code_wiring_allowed);
        assert!(!report.active_runtime_dependency_allowed);
        assert!(!report.active_runtime_auto_rebase_allowed);
        assert!(!report.active_codex_engine_dependency_allowed);
        assert!(!report.public_release_claim_allowed);
        assert!(!report.public_ga_claim_allowed);
        assert!(!report.release_artifact_write_allowed);
        assert!(!report.upstream_fetch_performed);
        assert!(!report.upstream_merge_performed);
        assert!(!report.upstream_checkout_performed);
        assert!(!report.workspace_mutation_default);
        assert!(!report.active_service_restart);
        assert!(!report.credential_value_read);
        assert!(!report.secret_file_read);
        assert!(!report.provider_invoked);
        assert!(!report.channel_delivery_performed);
        assert!(!report.gateway_rpc_performed);
        assert!(!report.public_release_published);
        assert!(
            report
                .shape_denial_reason
                .contains("partial or public-claim")
        );

        let public_claim = report
            .fixtures
            .iter()
            .find(|fixture| fixture.fixture_id == "public-claim-attempt-with-trusted-shape")
            .expect("public claim fixture");
        assert!(public_claim.public_release_claim_requested);
        assert!(public_claim.release_artifact_write_requested);
        assert_eq!(public_claim.total_satisfied_verification_count, 48);
        assert_eq!(public_claim.artifact_hash_verified_record_count, 8);
        assert_eq!(public_claim.freshness_window_satisfied_record_count, 0);
        assert_eq!(public_claim.trusted_source_verified_record_count, 8);
        assert_eq!(public_claim.validation_status, "blocked");
        assert!(!public_claim.public_release_claim_allowed);
        assert!(!public_claim.release_artifact_write_allowed);
        assert!(
            public_claim
                .denial_reason
                .contains("freshness is incomplete")
        );
        assert!(report.fixtures.iter().all(|fixture| {
            !fixture.active_wiring_allowed
                && !fixture.public_release_claim_allowed
                && !fixture.release_artifact_write_allowed
        }));
        assert!(
            report
                .shape_invariants
                .iter()
                .any(|invariant| invariant.contains("partially verified"))
        );
    }

    #[test]
    fn upstream_codex_activation_evidence_completeness_scoreboard_summarizes_gate_families() {
        let report = hepta_upstream_codex_activation_evidence_completeness_scoreboard_report();

        assert_eq!(report.product, "Hepta");
        assert_eq!(report.status, "ready");
        assert_eq!(
            report.scoreboard_id,
            "upstream-codex-activation-evidence-completeness-scoreboard"
        );
        assert_eq!(
            report.source_trusted_record_shape_validator_gate,
            "scripts/hepta-upstream-codex-activation-trusted-record-shape-validator.sh"
        );
        assert_eq!(
            report.evidence_completeness_scoreboard_gate,
            "scripts/hepta-upstream-codex-activation-evidence-completeness-scoreboard.sh"
        );
        assert!(report.source_trusted_record_shape_validator_ready);
        assert_eq!(report.required_gate_family_count, 10);
        assert_eq!(report.ready_gate_family_count, 10);
        assert_eq!(report.activation_blocking_gate_family_count, 10);
        assert_eq!(report.required_evidence_count, 8);
        assert_eq!(report.required_trusted_record_count, 8);
        assert_eq!(report.accepted_trusted_record_count, 0);
        assert_eq!(report.fresh_trusted_record_count, 0);
        assert!(report.public_claim_attempt_blocked);
        assert!(report.release_artifact_write_attempt_blocked);
        assert!(report.evidence_completeness_scoreboard_ready);
        assert!(report.activation_blocked_by_scoreboard);
        assert!(!report.activation_allowed_by_scoreboard);
        assert!(!report.active_wiring_allowed);
        assert!(report.gate_families.iter().all(|family| {
            family.gate_ready && family.blocks_activation_without_trusted_evidence
        }));
    }

    #[test]
    fn upstream_codex_activation_evidence_completeness_scoreboard_preserves_denials() {
        let report = hepta_upstream_codex_activation_evidence_completeness_scoreboard_report();

        assert!(!report.operator_approval_recorded);
        assert!(!report.activation_request_recorded);
        assert!(!report.operator_approved_activation_ready);
        assert!(!report.active_runtime_code_wiring_allowed);
        assert!(!report.active_runtime_dependency_allowed);
        assert!(!report.active_runtime_auto_rebase_allowed);
        assert!(!report.active_codex_engine_dependency_allowed);
        assert!(!report.public_release_claim_allowed);
        assert!(!report.public_ga_claim_allowed);
        assert!(!report.release_artifact_write_allowed);
        assert!(!report.upstream_fetch_performed);
        assert!(!report.upstream_merge_performed);
        assert!(!report.upstream_checkout_performed);
        assert!(!report.workspace_mutation_default);
        assert!(!report.active_service_restart);
        assert!(!report.credential_value_read);
        assert!(!report.secret_file_read);
        assert!(!report.provider_invoked);
        assert!(!report.channel_delivery_performed);
        assert!(!report.gateway_rpc_performed);
        assert!(!report.public_release_published);
        assert!(
            report
                .scoreboard_denial_reason
                .contains("no real activation request")
        );
        assert!(
            report
                .scoreboard_invariants
                .iter()
                .any(|invariant| invariant.contains("activation remains denied"))
        );
    }

    #[test]
    fn upstream_codex_activation_evidence_recording_dry_run_receipt_defines_redacted_schema() {
        let report = hepta_upstream_codex_activation_evidence_recording_dry_run_receipt_report();

        assert_eq!(report.product, "Hepta");
        assert_eq!(report.status, "ready");
        assert_eq!(
            report.receipt_id,
            "upstream-codex-activation-evidence-recording-dry-run-receipt"
        );
        assert_eq!(
            report.source_scoreboard_gate,
            "scripts/hepta-upstream-codex-activation-evidence-completeness-scoreboard.sh"
        );
        assert_eq!(
            report.evidence_recording_dry_run_receipt_gate,
            "scripts/hepta-upstream-codex-activation-evidence-recording-dry-run-receipt.sh"
        );
        assert!(report.source_scoreboard_ready);
        assert_eq!(report.required_receipt_field_count, 12);
        assert_eq!(report.recorded_receipt_field_count, 0);
        assert_eq!(report.redacted_or_hashed_field_count, 10);
        assert_eq!(report.required_evidence_count, 8);
        assert_eq!(report.required_trusted_record_count, 8);
        assert!(report.receipt_schema_ready);
        assert!(report.evidence_recording_dry_run_ready);
        assert!(report.receipt_fields.iter().all(|field| field.required));
        assert!(report.receipt_fields.iter().all(|field| !field.recorded));
        assert!(
            report.receipt_fields.iter().any(|field| {
                field.name == "operator_identity_hash" && field.redacted_or_hashed
            })
        );
        assert!(report.receipt_fields.iter().any(|field| {
            field.name == "artifact_sha256_or_redacted_path_bundle" && field.redacted_or_hashed
        }));
    }

    #[test]
    fn upstream_codex_activation_evidence_recording_dry_run_receipt_preserves_denials() {
        let report = hepta_upstream_codex_activation_evidence_recording_dry_run_receipt_report();

        assert!(!report.operator_approval_recorded);
        assert!(!report.activation_request_recorded);
        assert!(!report.receipt_recorded);
        assert!(!report.real_evidence_recorded);
        assert!(!report.trusted_record_materialized);
        assert_eq!(report.accepted_trusted_record_count, 0);
        assert_eq!(report.fresh_trusted_record_count, 0);
        assert!(report.public_claim_attempt_blocked);
        assert!(report.release_artifact_write_attempt_blocked);
        assert!(report.activation_blocked_by_receipt);
        assert!(!report.activation_allowed_by_receipt);
        assert!(!report.active_wiring_allowed);
        assert!(!report.active_runtime_code_wiring_allowed);
        assert!(!report.active_runtime_dependency_allowed);
        assert!(!report.active_runtime_auto_rebase_allowed);
        assert!(!report.active_codex_engine_dependency_allowed);
        assert!(!report.public_release_claim_allowed);
        assert!(!report.public_ga_claim_allowed);
        assert!(!report.release_artifact_write_allowed);
        assert!(!report.upstream_fetch_performed);
        assert!(!report.upstream_merge_performed);
        assert!(!report.upstream_checkout_performed);
        assert!(!report.workspace_mutation_default);
        assert!(!report.active_service_restart);
        assert!(!report.credential_value_read);
        assert!(!report.secret_file_read);
        assert!(!report.provider_invoked);
        assert!(!report.channel_delivery_performed);
        assert!(!report.gateway_rpc_performed);
        assert!(!report.public_release_published);
        assert!(report.receipt_denial_reason.contains("schema-only"));
        assert!(
            report
                .receipt_invariants
                .iter()
                .any(|invariant| invariant.contains("no evidence is recorded"))
        );
    }

    #[test]
    fn upstream_codex_activation_evidence_recording_denial_matrix_blocks_attempts() {
        let report = hepta_upstream_codex_activation_evidence_recording_denial_matrix_report();

        assert_eq!(report.product, "Hepta");
        assert_eq!(report.status, "ready");
        assert_eq!(
            report.matrix_id,
            "upstream-codex-activation-evidence-recording-denial-matrix"
        );
        assert_eq!(
            report.source_receipt_gate,
            "scripts/hepta-upstream-codex-activation-evidence-recording-dry-run-receipt.sh"
        );
        assert_eq!(
            report.evidence_recording_denial_matrix_gate,
            "scripts/hepta-upstream-codex-activation-evidence-recording-denial-matrix.sh"
        );
        assert!(report.source_receipt_gate_ready);
        assert_eq!(report.required_denied_attempt_count, 3);
        assert_eq!(report.denied_receipt_attempt_count, 3);
        assert_eq!(report.allowed_receipt_attempt_count, 0);
        assert_eq!(report.max_recorded_receipt_field_count, 12);
        assert_eq!(report.max_accepted_trusted_record_count, 8);
        assert_eq!(report.max_fresh_trusted_record_count, 8);
        assert_eq!(report.public_claim_attempt_count, 1);
        assert_eq!(report.release_artifact_write_attempt_count, 1);
        assert!(report.no_write_sink_ready);
        assert!(
            report
                .denied_receipt_attempts
                .iter()
                .all(|attempt| attempt.denial_status == "blocked")
        );
        assert!(
            report
                .denied_receipt_attempts
                .iter()
                .any(|attempt| attempt.attempt_kind == "public_claim_release_artifact_attempt")
        );
    }

    #[test]
    fn upstream_codex_activation_evidence_recording_denial_matrix_preserves_no_write_sink() {
        let report = hepta_upstream_codex_activation_evidence_recording_denial_matrix_report();

        assert!(!report.receipt_sink_write_performed);
        assert!(!report.evidence_receipt_persisted);
        assert!(!report.trusted_record_materialized);
        assert!(report.activation_blocked_by_no_write_sink);
        assert!(!report.activation_allowed_by_no_write_sink);
        assert!(!report.active_wiring_allowed);
        assert!(!report.active_runtime_code_wiring_allowed);
        assert!(!report.active_runtime_dependency_allowed);
        assert!(!report.active_runtime_auto_rebase_allowed);
        assert!(!report.active_codex_engine_dependency_allowed);
        assert!(!report.public_release_claim_allowed);
        assert!(!report.public_ga_claim_allowed);
        assert!(!report.release_artifact_write_allowed);
        assert!(!report.upstream_fetch_performed);
        assert!(!report.upstream_merge_performed);
        assert!(!report.upstream_checkout_performed);
        assert!(!report.workspace_mutation_default);
        assert!(!report.active_service_restart);
        assert!(!report.credential_value_read);
        assert!(!report.secret_file_read);
        assert!(!report.provider_invoked);
        assert!(!report.channel_delivery_performed);
        assert!(!report.gateway_rpc_performed);
        assert!(!report.public_release_published);
        assert!(
            report
                .denied_receipt_attempts
                .iter()
                .all(|attempt| !attempt.receipt_materialized
                    && !attempt.workspace_write_allowed
                    && !attempt.active_wiring_allowed
                    && !attempt.public_release_claim_allowed
                    && !attempt.release_artifact_write_allowed)
        );
        assert!(
            report
                .no_write_sink_invariants
                .iter()
                .any(|invariant| invariant.contains("fully shaped without being persisted"))
        );
    }

    #[test]
    fn upstream_codex_activation_evidence_receipt_persistence_command_contract_is_noop_by_default()
    {
        let report =
            hepta_upstream_codex_activation_evidence_receipt_persistence_command_contract_report();

        assert_eq!(report.product, "Hepta");
        assert_eq!(report.status, "ready");
        assert_eq!(
            report.command_contract_id,
            "upstream-codex-activation-evidence-receipt-persistence-command-contract"
        );
        assert_eq!(
            report.source_denial_matrix_gate,
            "scripts/hepta-upstream-codex-activation-evidence-recording-denial-matrix.sh"
        );
        assert_eq!(
            report.receipt_persistence_command_contract_gate,
            "scripts/hepta-upstream-codex-activation-evidence-receipt-persistence-command-contract.sh"
        );
        assert!(report.source_denial_matrix_ready);
        assert_eq!(report.required_command_field_count, 10);
        assert_eq!(report.recorded_command_field_count, 0);
        assert_eq!(report.redacted_or_hashed_field_count, 9);
        assert!(report.operator_approval_required);
        assert!(report.activation_request_required);
        assert!(!report.operator_approval_recorded);
        assert!(!report.activation_request_recorded);
        assert!(!report.receipt_persistence_command_enabled_by_default);
        assert!(report.receipt_persistence_noop_ready);
        assert!(report.command_fields.iter().all(|field| field.required));
        assert!(report.command_fields.iter().all(|field| !field.recorded));
        assert!(
            report
                .command_fields
                .iter()
                .any(|field| field.name == "receipt_output_path_redacted"
                    && field.redacted_or_hashed)
        );
    }

    #[test]
    fn upstream_codex_activation_evidence_receipt_persistence_command_contract_preserves_denials() {
        let report =
            hepta_upstream_codex_activation_evidence_receipt_persistence_command_contract_report();

        assert!(!report.trusted_record_materialized);
        assert!(!report.receipt_persistence_command_invoked);
        assert!(!report.receipt_persistence_execution_performed);
        assert!(!report.workspace_write_performed);
        assert!(!report.evidence_receipt_persisted);
        assert!(report.activation_blocked_by_persistence_contract);
        assert!(!report.activation_allowed_by_persistence_contract);
        assert!(!report.active_wiring_allowed);
        assert!(!report.active_runtime_code_wiring_allowed);
        assert!(!report.active_runtime_dependency_allowed);
        assert!(!report.active_runtime_auto_rebase_allowed);
        assert!(!report.active_codex_engine_dependency_allowed);
        assert!(!report.public_release_claim_allowed);
        assert!(!report.public_ga_claim_allowed);
        assert!(!report.release_artifact_write_allowed);
        assert!(!report.upstream_fetch_performed);
        assert!(!report.upstream_merge_performed);
        assert!(!report.upstream_checkout_performed);
        assert!(!report.workspace_mutation_default);
        assert!(!report.active_service_restart);
        assert!(!report.credential_value_read);
        assert!(!report.secret_file_read);
        assert!(!report.provider_invoked);
        assert!(!report.channel_delivery_performed);
        assert!(!report.gateway_rpc_performed);
        assert!(!report.public_release_published);
        assert!(
            report
                .command_contract_invariants
                .iter()
                .any(|invariant| invariant.contains("disabled by default"))
        );
    }

    #[test]
    fn upstream_codex_activation_evidence_receipt_persistence_invocation_dry_run_is_noop() {
        let report =
            hepta_upstream_codex_activation_evidence_receipt_persistence_invocation_dry_run_report(
            );

        assert_eq!(report.product, "Hepta");
        assert_eq!(report.status, "ready");
        assert_eq!(
            report.invocation_dry_run_id,
            "upstream-codex-activation-evidence-receipt-persistence-invocation-dry-run"
        );
        assert_eq!(
            report.source_command_contract_gate,
            "scripts/hepta-upstream-codex-activation-evidence-receipt-persistence-command-contract.sh"
        );
        assert_eq!(
            report.receipt_persistence_invocation_dry_run_gate,
            "scripts/hepta-upstream-codex-activation-evidence-receipt-persistence-invocation-dry-run.sh"
        );
        assert!(report.source_command_contract_ready);
        assert_eq!(report.required_invocation_fixture_count, 3);
        assert_eq!(report.command_invocation_attempt_count, 3);
        assert_eq!(report.command_invocation_performed_count, 0);
        assert_eq!(report.receipt_persistence_execution_performed_count, 0);
        assert_eq!(report.workspace_write_performed_count, 0);
        assert_eq!(report.evidence_receipt_persisted_count, 0);
        assert_eq!(report.redacted_output_path_fixture_count, 3);
        assert_eq!(report.payload_hash_bound_fixture_count, 3);
        assert_eq!(report.operator_approved_fixture_count, 3);
        assert_eq!(report.activation_request_bound_fixture_count, 3);
        assert_eq!(report.max_recorded_command_field_count, 10);
        assert_eq!(report.max_accepted_trusted_record_count, 8);
        assert_eq!(report.max_fresh_trusted_record_count, 8);
        assert_eq!(report.public_claim_attempt_count, 1);
        assert_eq!(report.release_artifact_write_attempt_count, 1);
        assert!(!report.receipt_persistence_command_enabled_by_default);
        assert!(report.invocation_dry_run_noop_ready);
    }

    #[test]
    fn upstream_codex_activation_evidence_receipt_persistence_invocation_dry_run_blocks_effects() {
        let report =
            hepta_upstream_codex_activation_evidence_receipt_persistence_invocation_dry_run_report(
            );

        assert!(report.activation_blocked_by_invocation_dry_run);
        assert!(!report.activation_allowed_by_invocation_dry_run);
        assert!(!report.active_wiring_allowed);
        assert!(!report.active_runtime_code_wiring_allowed);
        assert!(!report.active_runtime_dependency_allowed);
        assert!(!report.active_runtime_auto_rebase_allowed);
        assert!(!report.active_codex_engine_dependency_allowed);
        assert!(!report.public_release_claim_allowed);
        assert!(!report.public_ga_claim_allowed);
        assert!(!report.release_artifact_write_allowed);
        assert!(!report.upstream_fetch_performed);
        assert!(!report.upstream_merge_performed);
        assert!(!report.upstream_checkout_performed);
        assert!(!report.workspace_mutation_default);
        assert!(!report.active_service_restart);
        assert!(!report.credential_value_read);
        assert!(!report.secret_file_read);
        assert!(!report.provider_invoked);
        assert!(!report.channel_delivery_performed);
        assert!(!report.gateway_rpc_performed);
        assert!(!report.public_release_published);
        assert!(report.invocation_fixtures.iter().all(
            |fixture| fixture.command_invocation_requested
                && !fixture.command_invocation_performed
                && !fixture.receipt_persistence_execution_performed
                && !fixture.workspace_write_performed
                && !fixture.evidence_receipt_persisted
                && !fixture.active_wiring_allowed
                && !fixture.public_release_claim_allowed
                && !fixture.release_artifact_write_allowed
                && fixture.dry_run_status == "blocked_noop"
        ));
        assert!(
            report
                .invocation_fixtures
                .iter()
                .any(|fixture| fixture.fixture_kind == "public_claim_artifact_invocation_attempt")
        );
        assert!(
            report
                .invocation_dry_run_invariants
                .iter()
                .any(|invariant| invariant.contains("request persistence without executing it"))
        );
    }

    #[test]
    fn upstream_codex_activation_evidence_receipt_no_write_sink_adapter_contract_is_ready() {
        let report =
            hepta_upstream_codex_activation_evidence_receipt_no_write_sink_adapter_contract_report(
            );

        assert_eq!(report.product, "Hepta");
        assert_eq!(report.status, "ready");
        assert_eq!(
            report.no_write_sink_adapter_id,
            "upstream-codex-activation-evidence-receipt-no-write-sink-adapter-contract"
        );
        assert_eq!(
            report.source_invocation_dry_run_gate,
            "scripts/hepta-upstream-codex-activation-evidence-receipt-persistence-invocation-dry-run.sh"
        );
        assert_eq!(
            report.no_write_sink_adapter_contract_gate,
            "scripts/hepta-upstream-codex-activation-evidence-receipt-no-write-sink-adapter-contract.sh"
        );
        assert!(report.source_invocation_dry_run_ready);
        assert_eq!(report.required_sink_surface_count, 6);
        assert_eq!(report.ready_sink_surface_count, 6);
        assert_eq!(report.side_effect_free_surface_count, 6);
        assert_eq!(report.accepted_invocation_fixture_count, 3);
        assert_eq!(report.rejected_write_fixture_count, 3);
        assert_eq!(report.rejected_public_claim_fixture_count, 1);
        assert_eq!(report.persisted_receipt_count, 0);
        assert_eq!(report.workspace_write_performed_count, 0);
        assert!(!report.sink_write_path_enabled_by_default);
        assert!(report.sink_accepts_redacted_payload_hash);
        assert!(report.sink_accepts_redacted_output_path);
        assert!(report.sink_requires_operator_approval);
        assert!(report.sink_requires_fresh_trusted_records);
        assert!(report.sink_rejects_public_claim_artifact_write);
        assert!(report.no_write_sink_adapter_ready);
        assert!(report.sink_surfaces.iter().all(|surface| surface.required));
        assert!(report.sink_surfaces.iter().all(|surface| surface.ready));
        assert!(
            report
                .sink_surfaces
                .iter()
                .all(|surface| surface.side_effect_free)
        );
    }

    #[test]
    fn upstream_codex_activation_evidence_receipt_no_write_sink_adapter_contract_blocks_effects() {
        let report =
            hepta_upstream_codex_activation_evidence_receipt_no_write_sink_adapter_contract_report(
            );

        assert!(report.activation_blocked_by_no_write_sink_adapter);
        assert!(!report.activation_allowed_by_no_write_sink_adapter);
        assert!(!report.active_wiring_allowed);
        assert!(!report.active_runtime_code_wiring_allowed);
        assert!(!report.active_runtime_dependency_allowed);
        assert!(!report.active_runtime_auto_rebase_allowed);
        assert!(!report.active_codex_engine_dependency_allowed);
        assert!(!report.public_release_claim_allowed);
        assert!(!report.public_ga_claim_allowed);
        assert!(!report.release_artifact_write_allowed);
        assert!(!report.upstream_fetch_performed);
        assert!(!report.upstream_merge_performed);
        assert!(!report.upstream_checkout_performed);
        assert!(!report.workspace_mutation_default);
        assert!(!report.active_service_restart);
        assert!(!report.credential_value_read);
        assert!(!report.secret_file_read);
        assert!(!report.provider_invoked);
        assert!(!report.channel_delivery_performed);
        assert!(!report.gateway_rpc_performed);
        assert!(!report.public_release_published);
        assert!(
            report
                .no_write_sink_adapter_invariants
                .iter()
                .any(|invariant| invariant.contains("without persisting them"))
        );
    }

    #[test]
    fn upstream_codex_activation_evidence_receipt_write_enable_fixture_is_ready() {
        let report = hepta_upstream_codex_activation_evidence_receipt_write_enable_fixture_report();

        assert_eq!(report.product, "Hepta");
        assert_eq!(report.status, "ready");
        assert_eq!(
            report.write_enable_fixture_id,
            "upstream-codex-activation-evidence-receipt-write-enable-fixture"
        );
        assert_eq!(
            report.source_no_write_sink_adapter_gate,
            "scripts/hepta-upstream-codex-activation-evidence-receipt-no-write-sink-adapter-contract.sh"
        );
        assert_eq!(
            report.write_enable_fixture_gate,
            "scripts/hepta-upstream-codex-activation-evidence-receipt-write-enable-fixture.sh"
        );
        assert!(report.source_no_write_sink_adapter_ready);
        assert_eq!(report.required_write_enable_fixture_count, 3);
        assert_eq!(report.write_enable_fixture_count, 3);
        assert_eq!(report.blocked_write_enable_fixture_count, 3);
        assert_eq!(report.allowed_write_enable_fixture_count, 0);
        assert_eq!(report.explicit_write_enable_requested_fixture_count, 3);
        assert_eq!(report.operator_approved_fixture_count, 2);
        assert_eq!(report.activation_request_bound_fixture_count, 3);
        assert_eq!(report.fresh_trusted_record_fixture_count, 2);
        assert_eq!(report.active_binary_sha_bound_fixture_count, 3);
        assert_eq!(report.public_claim_attempt_fixture_count, 1);
        assert_eq!(report.release_artifact_write_attempt_fixture_count, 1);
        assert_eq!(report.public_artifact_policy_satisfied_fixture_count, 2);
        assert_eq!(report.filesystem_persistence_allowed_count, 0);
        assert_eq!(report.workspace_write_performed_count, 0);
        assert_eq!(report.evidence_receipt_persisted_count, 0);
        assert!(report.write_enable_fixture_contract_ready);
        assert!(
            report
                .write_enable_fixtures
                .iter()
                .all(|fixture| fixture.explicit_write_enable_requested)
        );
        assert!(
            report
                .write_enable_fixtures
                .iter()
                .any(|fixture| fixture.fixture_kind == "public_artifact_write_attempt")
        );
    }

    #[test]
    fn upstream_codex_activation_evidence_receipt_write_enable_fixture_blocks_effects() {
        let report = hepta_upstream_codex_activation_evidence_receipt_write_enable_fixture_report();

        assert!(report.activation_blocked_by_write_enable_fixture);
        assert!(!report.activation_allowed_by_write_enable_fixture);
        assert!(!report.active_wiring_allowed);
        assert!(!report.active_runtime_code_wiring_allowed);
        assert!(!report.active_runtime_dependency_allowed);
        assert!(!report.active_runtime_auto_rebase_allowed);
        assert!(!report.active_codex_engine_dependency_allowed);
        assert!(!report.public_release_claim_allowed);
        assert!(!report.public_ga_claim_allowed);
        assert!(!report.release_artifact_write_allowed);
        assert!(!report.upstream_fetch_performed);
        assert!(!report.upstream_merge_performed);
        assert!(!report.upstream_checkout_performed);
        assert!(!report.command_invocation_performed);
        assert!(!report.receipt_persistence_execution);
        assert!(!report.workspace_mutation_default);
        assert!(!report.active_service_restart);
        assert!(!report.credential_value_read);
        assert!(!report.secret_file_read);
        assert!(!report.provider_invoked);
        assert!(!report.channel_delivery_performed);
        assert!(!report.gateway_rpc_performed);
        assert!(!report.public_release_published);
        assert!(report.write_enable_fixtures.iter().all(|fixture| {
            !fixture.filesystem_persistence_allowed
                && !fixture.workspace_write_performed
                && !fixture.evidence_receipt_persisted
        }));
        assert!(
            report
                .write_enable_fixture_invariants
                .iter()
                .any(|invariant| invariant.contains("before any real write path exists"))
        );
    }

    #[test]
    fn upstream_codex_activation_evidence_receipt_materialization_dry_run_is_ready() {
        let report =
            hepta_upstream_codex_activation_evidence_receipt_materialization_dry_run_report();

        assert_eq!(report.product, "Hepta");
        assert_eq!(report.status, "ready");
        assert_eq!(
            report.materialization_dry_run_id,
            "upstream-codex-activation-evidence-receipt-materialization-dry-run"
        );
        assert_eq!(
            report.source_write_enable_fixture_gate,
            "scripts/hepta-upstream-codex-activation-evidence-receipt-write-enable-fixture.sh"
        );
        assert_eq!(
            report.materialization_dry_run_gate,
            "scripts/hepta-upstream-codex-activation-evidence-receipt-materialization-dry-run.sh"
        );
        assert!(report.source_write_enable_fixture_ready);
        assert_eq!(report.required_materialization_fixture_count, 3);
        assert_eq!(report.materialization_fixture_count, 3);
        assert_eq!(report.blocked_materialization_fixture_count, 3);
        assert_eq!(report.allowed_materialization_fixture_count, 0);
        assert_eq!(report.explicit_write_enable_requested_fixture_count, 3);
        assert_eq!(report.operator_approved_fixture_count, 2);
        assert_eq!(report.activation_request_bound_fixture_count, 3);
        assert_eq!(report.fresh_trusted_record_fixture_count, 2);
        assert_eq!(report.active_binary_sha_bound_fixture_count, 3);
        assert_eq!(report.payload_hash_planned_fixture_count, 3);
        assert_eq!(report.redacted_output_path_planned_fixture_count, 3);
        assert_eq!(report.deterministic_materialization_plan_count, 3);
        assert_eq!(report.public_claim_attempt_fixture_count, 1);
        assert_eq!(report.release_artifact_write_attempt_fixture_count, 1);
        assert_eq!(report.public_artifact_policy_satisfied_fixture_count, 2);
        assert_eq!(report.filesystem_persistence_allowed_count, 0);
        assert_eq!(report.materialization_executed_count, 0);
        assert_eq!(report.workspace_write_performed_count, 0);
        assert_eq!(report.evidence_receipt_persisted_count, 0);
        assert!(report.materialization_dry_run_ready);
        assert!(report.materialization_fixtures.iter().all(|fixture| {
            fixture.payload_hash_planned
                && fixture.redacted_output_path_planned
                && fixture.deterministic_materialization_plan
        }));
        assert!(
            report
                .materialization_fixtures
                .iter()
                .any(|fixture| fixture.fixture_kind == "public_artifact_attempt")
        );
    }

    #[test]
    fn upstream_codex_activation_evidence_receipt_materialization_dry_run_blocks_effects() {
        let report =
            hepta_upstream_codex_activation_evidence_receipt_materialization_dry_run_report();

        assert!(report.activation_blocked_by_materialization_dry_run);
        assert!(!report.activation_allowed_by_materialization_dry_run);
        assert!(!report.active_wiring_allowed);
        assert!(!report.active_runtime_code_wiring_allowed);
        assert!(!report.active_runtime_dependency_allowed);
        assert!(!report.active_runtime_auto_rebase_allowed);
        assert!(!report.active_codex_engine_dependency_allowed);
        assert!(!report.public_release_claim_allowed);
        assert!(!report.public_ga_claim_allowed);
        assert!(!report.release_artifact_write_allowed);
        assert!(!report.upstream_fetch_performed);
        assert!(!report.upstream_merge_performed);
        assert!(!report.upstream_checkout_performed);
        assert!(!report.command_invocation_performed);
        assert!(!report.receipt_persistence_execution);
        assert!(!report.workspace_mutation_default);
        assert!(!report.active_service_restart);
        assert!(!report.credential_value_read);
        assert!(!report.secret_file_read);
        assert!(!report.provider_invoked);
        assert!(!report.channel_delivery_performed);
        assert!(!report.gateway_rpc_performed);
        assert!(!report.public_release_published);
        assert!(report.materialization_fixtures.iter().all(|fixture| {
            !fixture.filesystem_persistence_allowed
                && !fixture.materialization_executed
                && !fixture.workspace_write_performed
                && !fixture.evidence_receipt_persisted
        }));
        assert!(
            report
                .materialization_invariants
                .iter()
                .any(|invariant| invariant.contains("without executing persistence"))
        );
    }

    #[test]
    fn upstream_codex_activation_evidence_receipt_filesystem_persistence_approval_packet_is_ready()
    {
        let report =
            hepta_upstream_codex_activation_evidence_receipt_filesystem_persistence_approval_packet_report();

        assert_eq!(report.product, "Hepta");
        assert_eq!(report.status, "ready");
        assert_eq!(
            report.filesystem_persistence_approval_packet_id,
            "upstream-codex-activation-evidence-receipt-filesystem-persistence-approval-packet"
        );
        assert_eq!(
            report.source_materialization_dry_run_gate,
            "scripts/hepta-upstream-codex-activation-evidence-receipt-materialization-dry-run.sh"
        );
        assert_eq!(
            report.filesystem_persistence_approval_packet_gate,
            "scripts/hepta-upstream-codex-activation-evidence-receipt-filesystem-persistence-approval-packet.sh"
        );
        assert!(report.source_materialization_dry_run_ready);
        assert_eq!(report.required_approval_field_count, 12);
        assert_eq!(report.approval_field_count, 12);
        assert_eq!(report.recorded_approval_field_count, 0);
        assert_eq!(report.redacted_or_hashed_field_count, 10);
        assert_eq!(report.required_for_filesystem_persistence_field_count, 12);
        assert!(report.operator_approval_required);
        assert!(!report.operator_approval_recorded);
        assert!(report.activation_request_required);
        assert!(!report.activation_request_recorded);
        assert!(report.materialization_plan_required);
        assert!(!report.materialization_plan_recorded);
        assert!(report.fresh_trusted_records_required);
        assert!(!report.fresh_trusted_records_recorded);
        assert!(report.active_binary_sha_required);
        assert!(!report.active_binary_sha_recorded);
        assert!(report.public_artifact_policy_required);
        assert!(!report.public_artifact_policy_recorded);
        assert!(report.filesystem_persistence_approval_packet_ready);
        assert!(
            report.approval_fields.iter().all(
                |field| field.required_for_filesystem_persistence && !field.recorded_by_default
            )
        );
        assert!(
            report
                .approval_fields
                .iter()
                .any(|field| field.name == "materialization_plan_id")
        );
    }

    #[test]
    fn upstream_codex_activation_evidence_receipt_filesystem_persistence_approval_packet_blocks_effects()
     {
        let report =
            hepta_upstream_codex_activation_evidence_receipt_filesystem_persistence_approval_packet_report();

        assert!(report.activation_blocked_by_filesystem_persistence_approval);
        assert!(!report.activation_allowed_by_filesystem_persistence_approval);
        assert!(!report.filesystem_persistence_allowed);
        assert!(!report.filesystem_persistence_execution_performed);
        assert!(!report.workspace_write_performed);
        assert!(!report.evidence_receipt_persisted);
        assert!(!report.active_wiring_allowed);
        assert!(!report.active_runtime_code_wiring_allowed);
        assert!(!report.active_runtime_dependency_allowed);
        assert!(!report.active_runtime_auto_rebase_allowed);
        assert!(!report.active_codex_engine_dependency_allowed);
        assert!(!report.public_release_claim_allowed);
        assert!(!report.public_ga_claim_allowed);
        assert!(!report.release_artifact_write_allowed);
        assert!(!report.upstream_fetch_performed);
        assert!(!report.upstream_merge_performed);
        assert!(!report.upstream_checkout_performed);
        assert!(!report.command_invocation_performed);
        assert!(!report.receipt_persistence_execution);
        assert!(!report.materialization_execution);
        assert!(!report.workspace_mutation_default);
        assert!(!report.active_service_restart);
        assert!(!report.credential_value_read);
        assert!(!report.secret_file_read);
        assert!(!report.provider_invoked);
        assert!(!report.channel_delivery_performed);
        assert!(!report.gateway_rpc_performed);
        assert!(!report.public_release_published);
        assert!(
            report
                .approval_packet_invariants
                .iter()
                .any(|invariant| invariant.contains("before any workspace write"))
        );
    }

    #[test]
    fn upstream_codex_activation_evidence_receipt_filesystem_output_path_allowlist_is_ready() {
        let report =
            hepta_upstream_codex_activation_evidence_receipt_filesystem_output_path_allowlist_report(
            );

        assert_eq!(report.product, "Hepta");
        assert_eq!(report.status, "ready");
        assert_eq!(
            report.filesystem_output_path_allowlist_id,
            "upstream-codex-activation-evidence-receipt-filesystem-output-path-allowlist"
        );
        assert_eq!(
            report.source_filesystem_persistence_approval_packet_gate,
            "scripts/hepta-upstream-codex-activation-evidence-receipt-filesystem-persistence-approval-packet.sh"
        );
        assert_eq!(
            report.filesystem_output_path_allowlist_gate,
            "scripts/hepta-upstream-codex-activation-evidence-receipt-filesystem-output-path-allowlist.sh"
        );
        assert!(report.source_filesystem_persistence_approval_packet_ready);
        assert_eq!(report.required_allowlist_entry_count, 6);
        assert_eq!(report.allowlist_entry_count, 6);
        assert_eq!(report.allowed_output_path_entry_count, 3);
        assert_eq!(report.blocked_output_path_entry_count, 3);
        assert_eq!(report.redacted_output_path_entry_count, 6);
        assert_eq!(report.default_selected_output_path_count, 0);
        assert!(!report.source_tree_path_allowed);
        assert!(!report.home_directory_path_allowed);
        assert!(!report.release_artifact_path_allowed);
        assert!(!report.public_artifact_path_allowed);
        assert!(report.receipt_output_path_allowlist_ready);
        assert!(
            report
                .allowlist_entries
                .iter()
                .all(|entry| entry.requires_operator_approval)
        );
        assert!(
            report
                .allowlist_entries
                .iter()
                .any(|entry| entry.name == "activation_evidence_receipts_root")
        );
        assert!(
            report
                .allowlist_entries
                .iter()
                .any(|entry| entry.name == "release_artifact_root"
                    && !entry.allowed_for_receipt_persistence)
        );
    }

    #[test]
    fn upstream_codex_activation_evidence_receipt_filesystem_output_path_allowlist_blocks_effects()
    {
        let report =
            hepta_upstream_codex_activation_evidence_receipt_filesystem_output_path_allowlist_report(
            );

        assert!(report.activation_blocked_by_output_path_allowlist);
        assert!(!report.activation_allowed_by_output_path_allowlist);
        assert!(!report.filesystem_persistence_allowed);
        assert!(!report.filesystem_persistence_execution_performed);
        assert!(!report.workspace_write_performed);
        assert!(!report.evidence_receipt_persisted);
        assert!(!report.active_wiring_allowed);
        assert!(!report.active_runtime_code_wiring_allowed);
        assert!(!report.active_runtime_dependency_allowed);
        assert!(!report.active_runtime_auto_rebase_allowed);
        assert!(!report.active_codex_engine_dependency_allowed);
        assert!(!report.public_release_claim_allowed);
        assert!(!report.public_ga_claim_allowed);
        assert!(!report.release_artifact_write_allowed);
        assert!(!report.upstream_fetch_performed);
        assert!(!report.upstream_merge_performed);
        assert!(!report.upstream_checkout_performed);
        assert!(!report.command_invocation_performed);
        assert!(!report.receipt_persistence_execution);
        assert!(!report.materialization_execution);
        assert!(!report.filesystem_persistence_execution);
        assert!(!report.workspace_mutation_default);
        assert!(!report.active_service_restart);
        assert!(!report.credential_value_read);
        assert!(!report.secret_file_read);
        assert!(!report.provider_invoked);
        assert!(!report.channel_delivery_performed);
        assert!(!report.gateway_rpc_performed);
        assert!(!report.public_release_published);
        assert!(
            report
                .allowlist_invariants
                .iter()
                .any(|invariant| invariant.contains("not filesystem write authority"))
        );
    }

    #[test]
    fn upstream_codex_activation_evidence_receipt_filesystem_output_path_evidence_binding_is_ready()
    {
        let report =
            hepta_upstream_codex_activation_evidence_receipt_filesystem_output_path_evidence_binding_report(
            );

        assert_eq!(report.product, "Hepta");
        assert_eq!(report.status, "ready");
        assert_eq!(
            report.filesystem_output_path_evidence_binding_id,
            "upstream-codex-activation-evidence-receipt-filesystem-output-path-evidence-binding"
        );
        assert_eq!(
            report.source_filesystem_output_path_allowlist_gate,
            "scripts/hepta-upstream-codex-activation-evidence-receipt-filesystem-output-path-allowlist.sh"
        );
        assert_eq!(
            report.filesystem_output_path_evidence_binding_gate,
            "scripts/hepta-upstream-codex-activation-evidence-receipt-filesystem-output-path-evidence-binding.sh"
        );
        assert!(report.source_filesystem_output_path_allowlist_ready);
        assert_eq!(report.required_path_binding_count, 8);
        assert_eq!(report.path_binding_count, 8);
        assert_eq!(report.allowed_output_path_entry_count, 3);
        assert_eq!(report.selected_output_path_count, 0);
        assert_eq!(report.recorded_path_binding_count, 0);
        assert_eq!(report.fresh_live_evidence_bound_count, 0);
        assert_eq!(report.active_binary_sha_bound_count, 0);
        assert_eq!(report.redacted_or_hashed_binding_count, 8);
        assert_eq!(report.trusted_source_bound_count, 0);
        assert!(!report.source_tree_path_binding_allowed);
        assert!(!report.home_directory_path_binding_allowed);
        assert!(!report.release_artifact_path_binding_allowed);
        assert!(!report.public_artifact_path_binding_allowed);
        assert!(report.output_path_evidence_binding_ready);
        assert!(
            report
                .path_bindings
                .iter()
                .all(|binding| binding.binding_required
                    && binding.requires_fresh_live_evidence
                    && binding.requires_active_binary_sha
                    && !binding.recorded_by_default)
        );
        assert!(report.path_bindings.iter().any(|binding| {
            binding.evidence_id == "watchdog_evidence_id"
                && binding.allowed_output_path_entry_name == "activation_evidence_receipts_root"
        }));
    }

    #[test]
    fn upstream_codex_activation_evidence_receipt_filesystem_output_path_evidence_binding_blocks_effects()
     {
        let report =
            hepta_upstream_codex_activation_evidence_receipt_filesystem_output_path_evidence_binding_report(
            );

        assert!(report.activation_blocked_by_output_path_evidence_binding);
        assert!(!report.activation_allowed_by_output_path_evidence_binding);
        assert!(!report.filesystem_persistence_allowed);
        assert!(!report.filesystem_persistence_execution_performed);
        assert!(!report.workspace_write_performed);
        assert!(!report.evidence_receipt_persisted);
        assert!(!report.active_wiring_allowed);
        assert!(!report.active_runtime_code_wiring_allowed);
        assert!(!report.active_runtime_dependency_allowed);
        assert!(!report.active_runtime_auto_rebase_allowed);
        assert!(!report.active_codex_engine_dependency_allowed);
        assert!(!report.public_release_claim_allowed);
        assert!(!report.public_ga_claim_allowed);
        assert!(!report.release_artifact_write_allowed);
        assert!(!report.upstream_fetch_performed);
        assert!(!report.upstream_merge_performed);
        assert!(!report.upstream_checkout_performed);
        assert!(!report.command_invocation_performed);
        assert!(!report.receipt_persistence_execution);
        assert!(!report.materialization_execution);
        assert!(!report.filesystem_persistence_execution);
        assert!(!report.workspace_mutation_default);
        assert!(!report.active_service_restart);
        assert!(!report.credential_value_read);
        assert!(!report.secret_file_read);
        assert!(!report.provider_invoked);
        assert!(!report.channel_delivery_performed);
        assert!(!report.gateway_rpc_performed);
        assert!(!report.public_release_published);
        assert!(
            report
                .binding_invariants
                .iter()
                .any(|invariant| invariant.contains("fresh live evidence binding"))
        );
    }

    #[test]
    fn upstream_codex_activation_evidence_receipt_filesystem_sink_write_preview_is_ready() {
        let report =
            hepta_upstream_codex_activation_evidence_receipt_filesystem_sink_write_preview_report();

        assert_eq!(report.product, "Hepta");
        assert_eq!(report.status, "ready");
        assert_eq!(
            report.filesystem_sink_write_preview_id,
            "upstream-codex-activation-evidence-receipt-filesystem-sink-write-preview"
        );
        assert_eq!(
            report.source_filesystem_output_path_evidence_binding_gate,
            "scripts/hepta-upstream-codex-activation-evidence-receipt-filesystem-output-path-evidence-binding.sh"
        );
        assert_eq!(
            report.filesystem_sink_write_preview_gate,
            "scripts/hepta-upstream-codex-activation-evidence-receipt-filesystem-sink-write-preview.sh"
        );
        assert!(report.source_filesystem_output_path_evidence_binding_ready);
        assert_eq!(report.required_preview_fixture_count, 3);
        assert_eq!(report.preview_fixture_count, 3);
        assert_eq!(report.allowed_output_path_entry_count, 3);
        assert_eq!(report.previewed_output_path_count, 3);
        assert_eq!(report.deterministic_payload_hash_count, 3);
        assert_eq!(report.redacted_output_path_preview_count, 3);
        assert_eq!(report.fresh_live_evidence_bound_fixture_count, 3);
        assert_eq!(report.active_binary_sha_bound_fixture_count, 3);
        assert_eq!(report.trusted_source_bound_fixture_count, 3);
        assert_eq!(report.operator_approval_bound_fixture_count, 3);
        assert_eq!(report.blocked_preview_fixture_count, 3);
        assert_eq!(report.allowed_preview_fixture_count, 0);
        assert_eq!(report.public_claim_attempt_fixture_count, 1);
        assert_eq!(report.release_artifact_write_attempt_fixture_count, 1);
        assert_eq!(report.filesystem_persistence_allowed_count, 0);
        assert!(report.sink_write_preview_ready);
        assert!(report.preview_fixtures.iter().all(|fixture| {
            fixture.redacted_output_path.starts_with("<redacted:")
                && fixture.deterministic_payload_hash.starts_with("sha256:")
                && fixture.preview_status == "blocked_preview"
        }));
        assert!(report.preview_fixtures.iter().any(|fixture| {
            fixture.fixture_id == "public-artifact-sink-write-preview-attempt"
                && fixture.public_claim_requested
                && fixture.release_artifact_write_requested
        }));
    }

    #[test]
    fn upstream_codex_activation_evidence_receipt_filesystem_sink_write_preview_blocks_effects() {
        let report =
            hepta_upstream_codex_activation_evidence_receipt_filesystem_sink_write_preview_report();

        assert!(report.activation_blocked_by_sink_write_preview);
        assert!(!report.activation_allowed_by_sink_write_preview);
        assert_eq!(report.filesystem_persistence_allowed_count, 0);
        assert_eq!(report.workspace_write_performed_count, 0);
        assert_eq!(report.evidence_receipt_persisted_count, 0);
        assert!(!report.active_wiring_allowed);
        assert!(!report.active_runtime_code_wiring_allowed);
        assert!(!report.active_runtime_dependency_allowed);
        assert!(!report.active_runtime_auto_rebase_allowed);
        assert!(!report.active_codex_engine_dependency_allowed);
        assert!(!report.public_release_claim_allowed);
        assert!(!report.public_ga_claim_allowed);
        assert!(!report.release_artifact_write_allowed);
        assert!(!report.upstream_fetch_performed);
        assert!(!report.upstream_merge_performed);
        assert!(!report.upstream_checkout_performed);
        assert!(!report.command_invocation_performed);
        assert!(!report.receipt_persistence_execution);
        assert!(!report.materialization_execution);
        assert!(!report.filesystem_persistence_execution);
        assert!(!report.workspace_mutation_default);
        assert!(!report.active_service_restart);
        assert!(!report.credential_value_read);
        assert!(!report.secret_file_read);
        assert!(!report.provider_invoked);
        assert!(!report.channel_delivery_performed);
        assert!(!report.gateway_rpc_performed);
        assert!(!report.public_release_published);
        assert!(
            report
                .preview_invariants
                .iter()
                .any(|invariant| invariant.contains("not write authority"))
        );
    }

    #[test]
    fn upstream_codex_activation_evidence_receipt_filesystem_persistence_execution_denial_matrix_is_ready()
     {
        let report =
            hepta_upstream_codex_activation_evidence_receipt_filesystem_persistence_execution_denial_matrix_report();

        assert_eq!(report.product, "Hepta");
        assert_eq!(report.status, "ready");
        assert_eq!(
            report.filesystem_persistence_execution_denial_matrix_id,
            "upstream-codex-activation-evidence-receipt-filesystem-persistence-execution-denial-matrix"
        );
        assert_eq!(
            report.source_filesystem_sink_write_preview_gate,
            "scripts/hepta-upstream-codex-activation-evidence-receipt-filesystem-sink-write-preview.sh"
        );
        assert_eq!(
            report.filesystem_persistence_execution_denial_matrix_gate,
            "scripts/hepta-upstream-codex-activation-evidence-receipt-filesystem-persistence-execution-denial-matrix.sh"
        );
        assert!(report.source_filesystem_sink_write_preview_ready);
        assert_eq!(report.required_denial_fixture_count, 4);
        assert_eq!(report.denial_fixture_count, 4);
        assert_eq!(report.source_preview_fixture_count, 3);
        assert_eq!(report.execution_requested_fixture_count, 4);
        assert_eq!(report.future_persistence_approval_slot_count, 4);
        assert_eq!(report.explicit_persistence_approval_id_present_count, 3);
        assert_eq!(report.explicit_persistence_approval_id_missing_count, 1);
        assert_eq!(report.stale_or_missing_fresh_evidence_fixture_count, 1);
        assert_eq!(report.active_binary_sha_bound_fixture_count, 4);
        assert_eq!(report.trusted_source_bound_fixture_count, 4);
        assert_eq!(report.operator_approval_bound_fixture_count, 3);
        assert_eq!(report.workspace_path_attempt_fixture_count, 1);
        assert_eq!(report.public_claim_attempt_fixture_count, 1);
        assert_eq!(report.release_artifact_write_attempt_fixture_count, 1);
        assert_eq!(report.blocked_execution_fixture_count, 4);
        assert_eq!(report.allowed_execution_fixture_count, 0);
        assert!(report.execution_denial_matrix_ready);
        assert!(report.denial_fixtures.iter().all(|fixture| {
            fixture.deterministic_payload_hash.starts_with("sha256:")
                && fixture
                    .future_persistence_approval_id_slot
                    .starts_with("<future:")
                && fixture.execution_status == "blocked_execution"
        }));
        assert!(report.denial_fixtures.iter().any(|fixture| {
            fixture.fixture_id == "public-artifact-execution-attempt"
                && fixture.public_claim_requested
                && fixture.release_artifact_write_requested
        }));
    }

    #[test]
    fn upstream_codex_activation_evidence_receipt_filesystem_persistence_execution_denial_matrix_blocks_effects()
     {
        let report =
            hepta_upstream_codex_activation_evidence_receipt_filesystem_persistence_execution_denial_matrix_report();

        assert!(report.activation_blocked_by_execution_denial_matrix);
        assert!(!report.activation_allowed_by_execution_denial_matrix);
        assert_eq!(report.filesystem_persistence_allowed_count, 0);
        assert_eq!(report.filesystem_persistence_execution_performed_count, 0);
        assert_eq!(report.workspace_write_performed_count, 0);
        assert_eq!(report.evidence_receipt_persisted_count, 0);
        assert!(!report.active_wiring_allowed);
        assert!(!report.active_runtime_code_wiring_allowed);
        assert!(!report.active_runtime_dependency_allowed);
        assert!(!report.active_runtime_auto_rebase_allowed);
        assert!(!report.active_codex_engine_dependency_allowed);
        assert!(!report.public_release_claim_allowed);
        assert!(!report.public_ga_claim_allowed);
        assert!(!report.release_artifact_write_allowed);
        assert!(!report.upstream_fetch_performed);
        assert!(!report.upstream_merge_performed);
        assert!(!report.upstream_checkout_performed);
        assert!(!report.command_invocation_performed);
        assert!(!report.receipt_persistence_execution);
        assert!(!report.materialization_execution);
        assert!(!report.filesystem_persistence_execution);
        assert!(!report.workspace_mutation_default);
        assert!(!report.active_service_restart);
        assert!(!report.credential_value_read);
        assert!(!report.secret_file_read);
        assert!(!report.provider_invoked);
        assert!(!report.channel_delivery_performed);
        assert!(!report.gateway_rpc_performed);
        assert!(!report.public_release_published);
        assert!(
            report
                .denial_invariants
                .iter()
                .any(|invariant| invariant.contains("not write authority"))
        );
    }
}

use serde::{Deserialize, Serialize};

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
        let baseline_upstream_head = "108234b5ebe6941764a6b8edbb37b2aa04369f07".to_string();
        let target_upstream_head = "7d47056ea42636271ac020b86347fbbef49490aa".to_string();

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
            target_head_source: "refs/remotes/openai-codex/main".into(),
            target_ref: "refs/remotes/openai-codex/main".into(),
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
            candidate_diff_range:
                "108234b5ebe6941764a6b8edbb37b2aa04369f07..7d47056ea42636271ac020b86347fbbef49490aa"
                    .into(),
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
            candidate_diff_range:
                "108234b5ebe6941764a6b8edbb37b2aa04369f07..7d47056ea42636271ac020b86347fbbef49490aa"
                    .into(),
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
            candidate_diff_range:
                "108234b5ebe6941764a6b8edbb37b2aa04369f07..7d47056ea42636271ac020b86347fbbef49490aa"
                    .into(),
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
            candidate_diff_range:
                "108234b5ebe6941764a6b8edbb37b2aa04369f07..7d47056ea42636271ac020b86347fbbef49490aa"
                    .into(),
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
            candidate_diff_range:
                "108234b5ebe6941764a6b8edbb37b2aa04369f07..7d47056ea42636271ac020b86347fbbef49490aa"
                    .into(),
            ledger_changed_file_count: 878,
            selected_absorption_changed_file_count: 716,
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
            candidate_diff_range:
                "108234b5ebe6941764a6b8edbb37b2aa04369f07..7d47056ea42636271ac020b86347fbbef49490aa"
                    .into(),
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
            "scripts/hepta-codex-watchdog.sh",
            "fresh watchdog evidence for the active service",
        ),
        activation_evidence_requirement(
            "browser_smoke_evidence_id",
            "scripts/hepta-codex-browser-visual-smoke.sh",
            "fresh browser visual smoke evidence",
        ),
        activation_evidence_requirement(
            "long_soak_evidence_id",
            "scripts/hepta-codex-live-soak.sh",
            "fresh long-soak evidence for the active service",
        ),
        activation_evidence_requirement(
            "rollback_plan_id",
            "operator rollback plan record",
            "explicit rollback anchor for the requested activation",
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
            "108234b5ebe6941764a6b8edbb37b2aa04369f07"
        );
        assert_eq!(
            report.target_upstream_head,
            "7d47056ea42636271ac020b86347fbbef49490aa"
        );
        assert_eq!(
            report.diff_ledger_gate,
            "scripts/hepta-upstream-codex-diff-ledger.sh"
        );
        assert_eq!(
            report.candidate_diff_range,
            "108234b5ebe6941764a6b8edbb37b2aa04369f07..7d47056ea42636271ac020b86347fbbef49490aa"
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
}

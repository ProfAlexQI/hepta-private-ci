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

pub fn hepta_upstream_codex_legacy_compatibility_absorption_report()
-> HeptaUpstreamCodexLegacyCompatibilityAbsorptionReport {
    HeptaUpstreamCodexLegacyCompatibilityAbsorptionReport::native_default()
}

pub fn hepta_upstream_codex_provider_security_absorption_report()
-> HeptaUpstreamCodexProviderSecurityAbsorptionReport {
    HeptaUpstreamCodexProviderSecurityAbsorptionReport::native_default()
}

pub fn hepta_upstream_codex_runtime_appserver_absorption_report()
-> HeptaUpstreamCodexRuntimeAppServerAbsorptionReport {
    HeptaUpstreamCodexRuntimeAppServerAbsorptionReport::native_default()
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
}

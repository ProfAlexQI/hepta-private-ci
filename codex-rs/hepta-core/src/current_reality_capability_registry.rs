use serde::Serialize;

pub const CURRENT_REALITY_CAPABILITY_CATALOG_SCHEMA_VERSION: &str =
    "current_reality_capability_catalog_v1";
pub const CURRENT_REALITY_CAPABILITY_CATALOG_ID_SHA256: &str =
    "aa9a8ffc24bd806bc1c5205e0a32c7f1008a49e7401d4d32718161cbf0161df4";
pub const CURRENT_REALITY_CATALOG_INVARIANT_ID: &str = "current-reality-static-invariant-v1";

pub const CURRENT_REALITY_WORK_GRAPH_DURABLE_IDENTITY_ALIAS: &str =
    "workflow_workgraph_durable_identity";
pub const CURRENT_REALITY_WORK_GRAPH_DURABLE_IDENTITY_SUCCESSOR: &str =
    "hepta-systems-work-graph-current-state-inventory";
pub const CURRENT_REALITY_WORK_GRAPH_RECEIPT_TAIL_ALIAS: &str =
    "workflow_current_readback_receipt_tail";
pub const CURRENT_REALITY_WORK_GRAPH_RECEIPT_TAIL_SUCCESSOR: &str = "hepta-systems-work-graph-persistence-acceptance-effect-application-denial-receipt-retention-expiry-readback-acknowledgement-terminal-decision-non-promotion-receipt-preview";

pub const CURRENT_REALITY_CAPABILITY_IDS: [&str; 104] = [
    "plugins_contribution_point_abi",
    "plugins_loader_binding_fixture",
    "plugins_tool_contribution_inventory",
    "plugins_lifecycle_state_machine",
    "tools_invocation_source_of_truth",
    "tools_read_only_dispatch_preflight",
    "workflow_workgraph_durable_identity",
    "workflow_current_readback_receipt_tail",
    "workflow_temporal_lite_durable_store_adapter",
    "workflow_durable_store_test_only_append_fixture",
    "workflow_temporal_lite_append_only_event_store_test_implementation",
    "workflow_temporal_lite_append_only_event_store_minimal_local_persistence",
    "workflow_temporal_lite_deterministic_replay_validator_local_persistence_readback",
    "workflow_temporal_lite_checkpoint_and_rollback_anchor_local_persistence_readback",
    "workflow_temporal_lite_lease_idempotency_index_local_persistence_readback",
    "workflow_temporal_lite_event_log_sqlite_adapter_local_persistence_readback",
    "workflow_temporal_lite_work_graph_projection_local_persistence_readback",
    "workflow_temporal_lite_work_graph_projection_replay_alignment_local_persistence_readback",
    "workflow_temporal_lite_replay_alignment_checkpoint_consistency_local_persistence_readback",
    "workflow_temporal_lite_replay_alignment_rollback_consistency_local_persistence_readback",
    "workflow_temporal_lite_replay_alignment_recovery_window_local_persistence_readback",
    "workflow_temporal_lite_replay_alignment_recovery_receipt_local_persistence_readback",
    "workflow_temporal_lite_deterministic_replay_validator_feature_gated_readback",
    "workflow_temporal_lite_checkpoint_and_rollback_anchor_feature_gated_readback",
    "workflow_temporal_lite_lease_idempotency_index_feature_gated_readback",
    "workflow_temporal_lite_event_log_sqlite_adapter_feature_gated_readback",
    "workflow_temporal_lite_work_graph_projection_feature_gated_readback",
    "workflow_temporal_lite_work_graph_projection_replay_alignment_feature_gated_readback",
    "workflow_temporal_lite_replay_alignment_checkpoint_consistency_feature_gated_readback",
    "workflow_temporal_lite_replay_alignment_rollback_consistency_feature_gated_readback",
    "workflow_temporal_lite_replay_alignment_recovery_window_feature_gated_readback",
    "workflow_temporal_lite_replay_alignment_recovery_receipt_feature_gated_readback",
    "hepta_systems_gate_recursion_cost_boundary_readback",
    "hepta_systems_gate_recursion_lean_contract_readback",
    "hepta_systems_workgraph_legacy_gate_recursion_inventory_readback",
    "hepta_systems_tool_registry_minimal_read_only_invocation_ledger_receipt_readback",
    "hepta_systems_plugin_canonical_manifest_permission_activation_contract_readback",
    "hepta_systems_plugin_signature_trust_install_cache_boundary_readback",
    "hepta_systems_plugin_operator_evidence_acceptance_packet_readback",
    "hepta_systems_plugin_install_cache_noop_preflight_readback",
    "hepta_systems_plugin_install_cache_idempotency_denial_receipt_readback",
    "hepta_systems_plugin_install_cache_rollback_uninstall_noop_readback",
    "hepta_systems_plugin_dynamic_activation_connector_start_boundary_readback",
    "hepta_systems_plugin_tool_registry_registration_denial_receipt_readback",
    "hepta_systems_plugin_tool_invocation_noop_denial_receipt_readback",
    "hepta_systems_plugin_tool_invocation_policy_approval_ledger_boundary_readback",
    "hepta_systems_plugin_tool_invocation_feature_gated_read_only_status_dry_run_readback",
    "hepta_systems_plugin_tool_invocation_dry_run_receipt_ledger_preview_readback",
    "hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_packet_readback",
    "hepta_systems_plugin_tool_invocation_read_only_status_dry_run_acceptance_recording_boundary_readback",
    "hepta_systems_plugin_tool_invocation_read_only_status_dry_run_execution_open_preconditions_readback",
    "hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_packet_readback",
    "hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_boundary_readback",
    "hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_open_preconditions_readback",
    "hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_persistence_denial_receipt_readback",
    "hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_persistence_open_preconditions_readback",
    "hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_persistence_shadow_write_rehearsal_readback",
    "hepta_systems_tool_registry_shadow_registration_lookup_readback",
    "hepta_systems_matrix_report_single_render_cache_boundary_readback",
    "current_reality_matrix_compact_cache_boundary_readback",
    "dirty_worktree_release_boundary_owner_freeze_classification_rehearsal_without_git_mutation",
    "dirty_worktree_release_boundary_owner_freeze_classification_outcome_readback_without_git_mutation",
    "dirty_worktree_release_boundary_owner_freeze_classification_operator_packet_without_send",
    "dirty_worktree_release_boundary_owner_freeze_classification_operator_packet_git_mutation_boundary_readback_without_git_mutation",
    "dirty_worktree_release_boundary_owner_freeze_classification_operator_decision_checklist_without_git_mutation",
    "dirty_worktree_release_boundary_owner_freeze_classification_operator_decision_checklist_packet_readback_without_git_mutation",
    "dirty_worktree_release_boundary_owner_freeze_classification_operator_decision_recording_boundary_readback_without_recording",
    "dirty_worktree_release_boundary_owner_freeze_classification_operator_approval_acceptance_boundary_readback_without_acceptance",
    "dirty_worktree_release_boundary_owner_freeze_classification_operator_evidence_recording_boundary_readback_without_recording",
    "hepta_system_status_read_only_e2e",
    "hepta_system_status_internal_read_only_invocation",
    "hepta_system_status_operator_approval_protocol",
    "controlled_canary_readiness_plan",
    "dirty_worktree_release_boundary_inventory",
    "dirty_worktree_release_boundary_grouping_freeze_plan",
    "dirty_worktree_release_boundary_grouping_freeze_operator_readback",
    "dirty_worktree_release_boundary_actionable_clean_worktree_strategy",
    "dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet",
    "dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback",
    "dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback",
    "dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist",
    "dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback",
    "dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback",
    "dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback",
    "dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback",
    "dirty_worktree_release_boundary_release_risk_snapshot",
    "dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal",
    "dirty_worktree_release_boundary_test_only_rehearsal_outcome_readback",
    "controlled_live_readiness_audit",
    "controlled_live_readiness_denial_readback_index",
    "controlled_live_operator_packet_preview",
    "controlled_live_operator_packet_non_send_readback",
    "controlled_live_required_evidence_collection_plan",
    "controlled_live_required_evidence_readback_index",
    "controlled_live_required_evidence_gap_summary",
    "controlled_live_required_evidence_gap_diff_view",
    "controlled_live_required_evidence_gap_operator_readback",
    "controlled_live_required_evidence_gap_operator_packet_attachment",
    "controlled_live_required_evidence_gap_operator_packet_attachment_non_send_readback",
    "controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary_readback",
    "controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary_readback",
    "controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary_readback",
    "controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary_readback",
    "current_compact_capability_summary",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CurrentRealityCapabilityLayer {
    Plugin,
    Tooling,
    Workflow,
    Systems,
    Worktree,
    SystemStatus,
    ControlledLive,
    CurrentSummary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum CurrentRealityCapabilitySource {
    TypedReport { report_id: &'static str },
    CatalogInvariant { invariant_id: &'static str },
    CompatibilityAlias { successor_report_id: &'static str },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct CurrentRealityCapabilityDescriptor {
    pub id: &'static str,
    pub layer: CurrentRealityCapabilityLayer,
    pub source: CurrentRealityCapabilitySource,
    pub legacy_current_fact: Option<&'static str>,
}

pub fn current_reality_capability_catalog() -> Vec<CurrentRealityCapabilityDescriptor> {
    CURRENT_REALITY_CAPABILITY_IDS
        .iter()
        .copied()
        .map(|id| CurrentRealityCapabilityDescriptor {
            id,
            layer: capability_layer(id),
            source: capability_source(id),
            legacy_current_fact: capability_legacy_current_fact(id),
        })
        .collect()
}

/// Count the stable rows in the typed current-reality capability catalog.
pub const fn current_reality_capability_registry_count() -> usize {
    CURRENT_REALITY_CAPABILITY_IDS.len()
}

fn capability_layer(id: &'static str) -> CurrentRealityCapabilityLayer {
    if id.starts_with("plugins_") || id.starts_with("hepta_systems_plugin_") {
        CurrentRealityCapabilityLayer::Plugin
    } else if id.starts_with("tools_") || id.starts_with("hepta_systems_tool_") {
        CurrentRealityCapabilityLayer::Tooling
    } else if id.starts_with("workflow_") {
        CurrentRealityCapabilityLayer::Workflow
    } else if id.starts_with("dirty_worktree_") {
        CurrentRealityCapabilityLayer::Worktree
    } else if id.starts_with("hepta_system_status_") {
        CurrentRealityCapabilityLayer::SystemStatus
    } else if id.starts_with("controlled_") {
        CurrentRealityCapabilityLayer::ControlledLive
    } else if id.starts_with("current_") {
        CurrentRealityCapabilityLayer::CurrentSummary
    } else {
        CurrentRealityCapabilityLayer::Systems
    }
}

fn capability_source(id: &'static str) -> CurrentRealityCapabilitySource {
    match id {
        CURRENT_REALITY_WORK_GRAPH_DURABLE_IDENTITY_ALIAS => {
            CurrentRealityCapabilitySource::CompatibilityAlias {
                successor_report_id: CURRENT_REALITY_WORK_GRAPH_DURABLE_IDENTITY_SUCCESSOR,
            }
        }
        CURRENT_REALITY_WORK_GRAPH_RECEIPT_TAIL_ALIAS => {
            CurrentRealityCapabilitySource::CompatibilityAlias {
                successor_report_id: CURRENT_REALITY_WORK_GRAPH_RECEIPT_TAIL_SUCCESSOR,
            }
        }
        "plugins_contribution_point_abi" => CurrentRealityCapabilitySource::TypedReport {
            report_id: "hepta-systems-plugin-contribution-point-abi",
        },
        "plugins_loader_binding_fixture" => CurrentRealityCapabilitySource::TypedReport {
            report_id: "hepta-systems-plugin-contribution-point-loader-binding",
        },
        "plugins_tool_contribution_inventory" => CurrentRealityCapabilitySource::TypedReport {
            report_id: "hepta-systems-plugin-tool-contribution-inventory-preview",
        },
        "plugins_lifecycle_state_machine" => CurrentRealityCapabilitySource::TypedReport {
            report_id: "hepta-systems-plugin-lifecycle-state-machine",
        },
        "tools_invocation_source_of_truth" => CurrentRealityCapabilitySource::TypedReport {
            report_id: "hepta-systems-tool-registry-invocation-source-of-truth",
        },
        "tools_read_only_dispatch_preflight" => CurrentRealityCapabilitySource::TypedReport {
            report_id: "hepta-systems-tool-registry-read-only-dispatch-preflight",
        },
        "workflow_temporal_lite_durable_store_adapter" => {
            CurrentRealityCapabilitySource::TypedReport {
                report_id: "hepta-systems-workflow-durable-store-adapter",
            }
        }
        "hepta_system_status_read_only_e2e" => CurrentRealityCapabilitySource::TypedReport {
            report_id: "hepta-systems-hepta-system-status-read-only-e2e",
        },
        "hepta_system_status_internal_read_only_invocation" => {
            CurrentRealityCapabilitySource::TypedReport {
                report_id: "hepta-systems-hepta-system-status-internal-read-only-invocation",
            }
        }
        "hepta_system_status_operator_approval_protocol" => {
            CurrentRealityCapabilitySource::TypedReport {
                report_id: "hepta-systems-hepta-system-status-operator-approval-protocol",
            }
        }
        "controlled_canary_readiness_plan" => CurrentRealityCapabilitySource::TypedReport {
            report_id: "hepta-systems-controlled-canary-readiness-plan",
        },
        "dirty_worktree_release_boundary_inventory" => {
            typed_report("hepta-systems-dirty-worktree-release-boundary-inventory")
        }
        "dirty_worktree_release_boundary_grouping_freeze_plan" => {
            typed_report("hepta-systems-dirty-worktree-release-boundary-grouping-freeze-plan")
        }
        "dirty_worktree_release_boundary_grouping_freeze_operator_readback" => typed_report(
            "hepta-systems-dirty-worktree-release-boundary-grouping-freeze-operator-readback",
        ),
        "dirty_worktree_release_boundary_actionable_clean_worktree_strategy" => typed_report(
            "hepta-systems-dirty-worktree-release-boundary-actionable-clean-worktree-strategy",
        ),
        "dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet" => typed_report(
            "hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-packet",
        ),
        "dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback" => {
            typed_report(
                "hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-packet-non-send-readback",
            )
        }
        "dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback" => {
            typed_report(
                "hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-packet-git-mutation-boundary-readback",
            )
        }
        "dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist" => {
            typed_report(
                "hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-decision-checklist",
            )
        }
        "dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback" => {
            typed_report(
                "hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-decision-checklist-packet-readback",
            )
        }
        "dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback" => {
            typed_report(
                "hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-decision-recording-boundary-readback",
            )
        }
        "dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback" => {
            typed_report(
                "hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-approval-acceptance-boundary-readback",
            )
        }
        "dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback" => {
            typed_report(
                "hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-evidence-recording-boundary-readback",
            )
        }
        "dirty_worktree_release_boundary_release_risk_snapshot" => {
            typed_report("hepta-systems-dirty-worktree-release-boundary-release-risk-snapshot")
        }
        "dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal" => {
            typed_report(
                "hepta-systems-dirty-worktree-release-boundary-test-only-clean-worktree-strategy-rehearsal",
            )
        }
        "controlled_live_readiness_audit" => {
            typed_report("hepta-systems-controlled-live-readiness-audit")
        }
        "controlled_live_readiness_denial_readback_index" => {
            typed_report("hepta-systems-controlled-live-readiness-denial-readback-index")
        }
        "controlled_live_operator_packet_preview" => {
            typed_report("hepta-systems-controlled-live-operator-packet-preview")
        }
        "controlled_live_operator_packet_non_send_readback" => {
            typed_report("hepta-systems-controlled-live-operator-packet-non-send-readback")
        }
        "controlled_live_required_evidence_collection_plan" => {
            typed_report("hepta-systems-controlled-live-required-evidence-collection-plan")
        }
        "controlled_live_required_evidence_readback_index" => {
            typed_report("hepta-systems-controlled-live-required-evidence-readback-index")
        }
        "controlled_live_required_evidence_gap_summary" => {
            typed_report("hepta-systems-controlled-live-required-evidence-gap-summary")
        }
        "controlled_live_required_evidence_gap_diff_view" => {
            typed_report("hepta-systems-controlled-live-required-evidence-gap-diff-view")
        }
        "controlled_live_required_evidence_gap_operator_readback" => {
            typed_report("hepta-systems-controlled-live-required-evidence-gap-operator-readback")
        }
        "controlled_live_required_evidence_gap_operator_packet_attachment" => typed_report(
            "hepta-systems-controlled-live-required-evidence-gap-operator-packet-attachment",
        ),
        "controlled_live_required_evidence_gap_operator_packet_attachment_non_send_readback" => {
            typed_report(
                "hepta-systems-controlled-live-required-evidence-gap-operator-packet-attachment-non-send-readback",
            )
        }
        "controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary_readback" => {
            typed_report(
                "hepta-systems-controlled-live-required-evidence-gap-operator-packet-attachment-transport-boundary-readback",
            )
        }
        "controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary_readback" => {
            typed_report(
                "hepta-systems-controlled-live-required-evidence-gap-operator-packet-attachment-credential-boundary-readback",
            )
        }
        "controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary_readback" => {
            typed_report(
                "hepta-systems-controlled-live-required-evidence-gap-operator-packet-attachment-rollback-rehearsal-boundary-readback",
            )
        }
        "controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary_readback" => {
            typed_report(
                "hepta-systems-controlled-live-required-evidence-gap-operator-packet-attachment-kill-switch-rehearsal-boundary-readback",
            )
        }
        "current_compact_capability_summary" => CurrentRealityCapabilitySource::TypedReport {
            report_id: "hepta-systems-current-compact-capability-summary",
        },
        _ => CurrentRealityCapabilitySource::CatalogInvariant {
            invariant_id: CURRENT_REALITY_CATALOG_INVARIANT_ID,
        },
    }
}

const fn typed_report(report_id: &'static str) -> CurrentRealityCapabilitySource {
    CurrentRealityCapabilitySource::TypedReport { report_id }
}

fn capability_legacy_current_fact(id: &'static str) -> Option<&'static str> {
    match id {
        "plugins_contribution_point_abi" => {
            Some("8 contribution point kinds are policy-bound and runtime execution is disabled")
        }
        "plugins_loader_binding_fixture" => {
            Some("hepta-system manifest fixture is present and loader bindings are read-only")
        }
        "plugins_tool_contribution_inventory" => {
            Some("2 manifest candidates have schema, policy, ledger, and approval metadata")
        }
        "plugins_lifecycle_state_machine" => Some(
            "plugin lifecycle state-machine is restored and binds ABI, loader, fixture policy metadata, and tool preview contract",
        ),
        "tools_invocation_source_of_truth" => Some(
            "2 invocation sources are ready, but registration, invocation, ledger, and approval writes are disabled",
        ),
        "tools_read_only_dispatch_preflight" => Some(
            "plugin lifecycle-backed ToolRegistry dispatch preflight projects lookup, ledger, approval, and receipt without invocation",
        ),
        "workflow_workgraph_durable_identity" => Some(
            "durable identity fields are fixed while runtime, replay, rollback, and live execution are disabled",
        ),
        "workflow_current_readback_receipt_tail" => Some(
            "current WorkGraph tail is readback receipt preview; acknowledgement is next, but persistence remains disabled",
        ),
        "workflow_temporal_lite_durable_store_adapter" => Some(
            "Temporal-lite adapter plan carries 9 append-only event contracts through lease, idempotency, checkpoint, replay, rollback, and no-op receipt metadata behind a disabled feature gate",
        ),
        "hepta_system_status_read_only_e2e" => Some(
            "hepta-system status fixture, ToolRegistry preflight, workflow adapter noop receipt, and Native read-only console are threaded without invocation or persistence",
        ),
        "hepta_system_status_internal_read_only_invocation" => Some(
            "hepta-system status now materializes one internal read-only status payload from the MCP candidate while the app connector remains preflight-only and external network, credentials, mutation, persistence, and live execution stay disabled",
        ),
        "hepta_system_status_operator_approval_protocol" => Some(
            "hepta-system status operator approval protocol binds nonce, operator session, packet preview, and non-acceptance receipt while approval request, acceptance, broker writes, persistence, transport, credentials, and live execution remain disabled",
        ),
        "controlled_canary_readiness_plan" => Some(
            "controlled canary readiness is planned from the Phase 9 approval protocol and seven controlled-live blocker readbacks while canary activation, Gateway/Auth, Native POST, Telegram/channel transport, credentials, persistence, Public GA, and live execution remain disabled",
        ),
        "dirty_worktree_release_boundary_inventory" => Some(
            "dirty worktree release boundary is inventoried with read-only git status counts and buckets while staging, cleanup, release, canary activation, and live execution remain disabled",
        ),
        "dirty_worktree_release_boundary_grouping_freeze_plan" => Some(
            "dirty worktree release boundary inventory is grouped into top-level and scope freeze-plan buckets while freeze application, git mutation, evidence persistence, release, canary activation, and live remain disabled",
        ),
        "dirty_worktree_release_boundary_grouping_freeze_operator_readback" => Some(
            "dirty worktree grouping freeze plan is operator-readable and diffable while freeze application, git mutation, evidence persistence, release, canary activation, and live remain disabled",
        ),
        "dirty_worktree_release_boundary_actionable_clean_worktree_strategy" => Some(
            "dirty worktree release-boundary groups are converted into an operator-visible clean-worktree strategy while strategy application, git mutation, cleanup, evidence persistence, release, canary activation, and live remain disabled",
        ),
        "dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet" => Some(
            "dirty worktree clean-worktree strategy is packaged into an operator packet preview while packet send, packet persistence, strategy application, git mutation, evidence persistence, release, canary activation, and live remain disabled",
        ),
        "dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback" => {
            Some(
                "dirty worktree clean-worktree strategy operator packet is operator-visible, unsent, and unpersisted while readback persistence, git mutation, cleanup, evidence persistence, release, canary activation, and live remain disabled",
            )
        }
        "dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback" => {
            Some(
                "dirty worktree clean-worktree strategy operator packet git-mutation boundary is explicit while git add, index mutation, commit, push, reset, checkout, revert, cleanup, delete, evidence persistence, release, canary activation, and live remain disabled",
            )
        }
        "dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist" => {
            Some(
                "dirty worktree clean-worktree strategy operator decisions are collapsed into a pending checklist while decision recording, approval acceptance, evidence recording, git mutation, cleanup, release, canary activation, and live remain disabled",
            )
        }
        "dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback" => {
            Some(
                "dirty worktree clean-worktree strategy operator decision checklist is rendered as a packet/readback while packet send, persistence, decision recording, approval acceptance, git mutation, cleanup, release, canary activation, and live remain disabled",
            )
        }
        "dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback" => {
            Some(
                "dirty worktree clean-worktree strategy operator decision recording boundary is explicit while decision recording, decision receipt persistence, approval acceptance, evidence recording, git mutation, cleanup, release, canary activation, and live remain disabled",
            )
        }
        "dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback" => {
            Some(
                "dirty worktree clean-worktree strategy operator approval acceptance boundary is explicit while approval request, acceptance, recording, receipts, decision recording, evidence, git mutation, cleanup, release, canary activation, and live remain disabled",
            )
        }
        "dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback" => {
            Some(
                "dirty worktree clean-worktree strategy operator evidence recording boundary is explicit while evidence recording, persistence, receipts, approvals, decision recording, git mutation, cleanup, release, canary activation, and live remain disabled",
            )
        }
        "dirty_worktree_release_boundary_release_risk_snapshot" => Some(
            "dirty worktree release risk is collapsed into one critical, four high, and two medium risk entries while snapshot persistence, evidence recording, approval, decision recording, git mutation, cleanup, release, canary activation, and live remain disabled",
        ),
        "dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal" => Some(
            "dirty worktree test-only rehearsal is visible-only while mutation and live remain disabled",
        ),
        "controlled_live_readiness_audit" => Some(
            "controlled-live audit is ready-blocked with a clean worktree and six explicit approval/evidence blockers",
        ),
        "controlled_live_readiness_denial_readback_index" => Some(
            "seven stable readback slots expose the actual six-or-seven active blockers while waiver, acceptance, persistence, approval, and live execution remain disabled",
        ),
        "controlled_live_operator_packet_preview" => Some(
            "controlled-live operator packet preview assembles scope, payload hash, rollback owner, seven blocker readbacks, and required evidence while approval request, approval recording, persistence, and live remain disabled",
        ),
        "controlled_live_operator_packet_non_send_readback" => Some(
            "controlled-live operator packet non-send readback proves the packet is visible, unsent, unpersisted, and still not an approval request",
        ),
        "controlled_live_required_evidence_collection_plan" => Some(
            "controlled-live required evidence collection plan lists evidence for seven blockers while evidence recording, credential reads, approval acceptance, blocker waiver, persistence, and live remain disabled",
        ),
        "controlled_live_required_evidence_readback_index" => Some(
            "controlled-live required evidence readback index makes seven evidence requirements queryable and diffable while evidence recording, credential reads, approval acceptance, blocker waiver, persistence, and live remain disabled",
        ),
        "controlled_live_required_evidence_gap_summary" => Some(
            "controlled-live required evidence gap summary groups seven missing evidence gaps by owner and cutover risk while acceptance, recording, credential reads, waiver, persistence, and live remain disabled",
        ),
        "controlled_live_required_evidence_gap_diff_view" => Some(
            "controlled-live required evidence gap diff view keeps seven missing evidence gaps comparable across readbacks while acceptance, recording, credential reads, waiver, persistence, and live remain disabled",
        ),
        "controlled_live_required_evidence_gap_operator_readback" => Some(
            "controlled-live required evidence gap operator readback presents seven unchanged missing evidence gaps with stable operator readback routes while acceptance, recording, credential reads, waiver, persistence, and live remain disabled",
        ),
        "controlled_live_required_evidence_gap_operator_packet_attachment" => Some(
            "controlled-live required evidence gap operator packet attachment attaches seven unchanged missing operator readbacks to the local packet preview while approval, sending, evidence recording, packet/attachment persistence, credential reads, waiver, and live remain disabled",
        ),
        "controlled_live_required_evidence_gap_operator_packet_attachment_non_send_readback" => {
            Some(
                "controlled-live required evidence gap operator packet attachment non-send readback proves seven attached readbacks are visible, unsent, unpersisted, and not an approval request while approval, evidence recording, credential reads, waiver, transport, persistence, and live remain disabled",
            )
        }
        "controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary_readback" => {
            Some(
                "controlled-live required evidence gap operator packet attachment transport boundary readback makes Gateway/Auth, Native POST, Telegram transport, and channel send closed boundaries operator-visible while approval, evidence recording, credential reads, waiver, transport mutation, persistence, and live remain disabled",
            )
        }
        "controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary_readback" => {
            Some(
                "controlled-live required evidence gap operator packet attachment credential boundary readback makes credential reads, material loads, value exposure, and handle resolution closed and operator-visible while approval, evidence recording, waiver, transport mutation, persistence, and live remain disabled",
            )
        }
        "controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary_readback" => {
            Some(
                "controlled-live required evidence gap operator packet attachment rollback rehearsal boundary readback makes rollback rehearsal execution, rollback execution, rehearsal recording, and rehearsal receipt persistence closed and operator-visible while approval, credential reads, evidence recording, waiver, transport mutation, persistence, and live remain disabled",
            )
        }
        "controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary_readback" => {
            Some(
                "controlled-live required evidence gap operator packet attachment kill-switch rehearsal boundary readback makes kill-switch rehearsal execution, kill-switch mutation, rehearsal recording, and rehearsal receipt persistence closed and operator-visible while rollback rehearsal, approval, credential reads, evidence recording, waiver, transport mutation, persistence, and live remain disabled",
            )
        }
        "current_compact_capability_summary" => Some(
            "existing current compact summary is ready and keeps execution and Public GA disabled",
        ),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn typed_catalog_preserves_the_stable_104_id_contract() {
        let catalog = current_reality_capability_catalog();
        let unique = catalog.iter().map(|row| row.id).collect::<BTreeSet<_>>();

        assert_eq!(catalog.len(), 104);
        assert_eq!(unique.len(), 104);
        assert_eq!(current_reality_capability_registry_count(), 104);
        assert_eq!(catalog[0].id, "plugins_contribution_point_abi");
        assert_eq!(catalog[103].id, "current_compact_capability_summary");
        assert_eq!(
            CURRENT_REALITY_CAPABILITY_CATALOG_ID_SHA256,
            "aa9a8ffc24bd806bc1c5205e0a32c7f1008a49e7401d4d32718161cbf0161df4"
        );
    }

    #[test]
    fn retired_work_graph_rows_are_aliases_to_current_typed_successors() {
        let catalog = current_reality_capability_catalog();
        let durable = catalog
            .iter()
            .find(|row| row.id == CURRENT_REALITY_WORK_GRAPH_DURABLE_IDENTITY_ALIAS)
            .expect("durable identity alias");
        let receipt = catalog
            .iter()
            .find(|row| row.id == CURRENT_REALITY_WORK_GRAPH_RECEIPT_TAIL_ALIAS)
            .expect("receipt tail alias");

        assert_eq!(
            durable.source,
            CurrentRealityCapabilitySource::CompatibilityAlias {
                successor_report_id: CURRENT_REALITY_WORK_GRAPH_DURABLE_IDENTITY_SUCCESSOR,
            }
        );
        assert_eq!(
            receipt.source,
            CurrentRealityCapabilitySource::CompatibilityAlias {
                successor_report_id: CURRENT_REALITY_WORK_GRAPH_RECEIPT_TAIL_SUCCESSOR,
            }
        );
    }

    #[test]
    fn catalog_distinguishes_real_reports_from_legacy_static_invariants() {
        let catalog = current_reality_capability_catalog();
        let report_backed = catalog
            .iter()
            .filter(|row| {
                matches!(
                    row.source,
                    CurrentRealityCapabilitySource::TypedReport { .. }
                        | CurrentRealityCapabilitySource::CompatibilityAlias { .. }
                )
            })
            .count();
        let invariants = catalog
            .iter()
            .filter(|row| {
                matches!(
                    row.source,
                    CurrentRealityCapabilitySource::CatalogInvariant { .. }
                )
            })
            .count();

        assert_eq!(report_backed, 43);
        assert_eq!(invariants, 61);
        assert_eq!(report_backed + invariants, 104);
        assert!(catalog.iter().all(|row| {
            row.legacy_current_fact.is_some()
                == !matches!(
                    row.source,
                    CurrentRealityCapabilitySource::CatalogInvariant { .. }
                )
        }));
    }
}

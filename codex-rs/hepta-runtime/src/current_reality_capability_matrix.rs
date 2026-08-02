use std::any::type_name;
use std::collections::BTreeSet;

use hepta_core::CURRENT_REALITY_CAPABILITY_CATALOG_ID_SHA256;
use hepta_core::CURRENT_REALITY_CAPABILITY_CATALOG_SCHEMA_VERSION;
use hepta_core::CURRENT_REALITY_CAPABILITY_IDS;
use hepta_core::CURRENT_REALITY_WORK_GRAPH_DURABLE_IDENTITY_ALIAS;
use hepta_core::CURRENT_REALITY_WORK_GRAPH_RECEIPT_TAIL_ALIAS;
use hepta_core::CurrentRealityCapabilityDescriptor;
use hepta_core::CurrentRealityCapabilityLayer;
use hepta_core::CurrentRealityCapabilitySource;
use hepta_core::current_reality_capability_catalog;
use serde::Serialize;
use serde_json::Value;
use sha2::Digest;
use sha2::Sha256;
use thiserror::Error;

use crate::WorkGraphCurrentStateInventoryReport;
use crate::WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptPreviewReport;
use crate::hepta_work_graph_current_state_inventory_report;
use crate::hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_preview_report;

pub const CURRENT_REALITY_CAPABILITY_MATRIX_PAIR_ID: &str =
    "hepta-systems-current-reality-capability-matrix";
pub const CURRENT_REALITY_CAPABILITY_MATRIX_GATE: &str =
    "hepta_systems_current_reality_capability_matrix_gate";
pub const CURRENT_REALITY_CAPABILITY_MATRIX_SCHEMA_VERSION: &str =
    "current_reality_capability_matrix_v2";
pub const CURRENT_REALITY_CAPABILITY_MATRIX_COMPATIBILITY_MIGRATION: &str =
    "controlled_live_clean_worktree_boundary_v2";
pub const CURRENT_REALITY_CAPABILITY_MATRIX_NEXT_ACTION: &str =
    "close_controlled_live_evidence_before_status_canary_start";

const CONTROLLED_LIVE_AUDIT_ID: &str = "controlled_live_readiness_audit";
const CONTROLLED_LIVE_DENIAL_INDEX_ID: &str = "controlled_live_readiness_denial_readback_index";
const CURRENT_COMPACT_SUMMARY_ID: &str = "current_compact_capability_summary";

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum CurrentRealityCapabilityMatrixError {
    #[error("plugin manifest bytes are required")]
    MissingPluginManifest,
    #[error("plugin manifest is malformed: {0}")]
    MalformedPluginManifest(String),
    #[error("plugin fixture shape does not match the typed Hepta contract")]
    InvalidPluginFixtureShape,
    #[error("worktree observation counts are inconsistent")]
    InvalidWorktreeObservation,
    #[error("typed component inventory is incomplete or altered")]
    InvalidTypedComponentInventory,
    #[error("modern WorkGraph successor reports failed integrity")]
    InvalidWorkGraphSuccessors,
    #[error("capability observations are incomplete, reordered, duplicated, or altered")]
    InvalidCapabilityObservations,
    #[error("capability observations must include both ready and blocked rows")]
    MissingReadyBlockedBoundary,
    #[error("typed current-reality source serialization failed: {0}")]
    SourceSerialization(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct CurrentRealityPluginAssetObservation {
    pub skill_path_present: bool,
    pub mcp_servers_path_present: bool,
    pub apps_path_present: bool,
    pub skill_count: usize,
    pub mcp_server_count: usize,
    pub app_count: usize,
}

impl CurrentRealityPluginAssetObservation {
    pub const fn new(
        skill_path_present: bool,
        mcp_servers_path_present: bool,
        apps_path_present: bool,
        skill_count: usize,
        mcp_server_count: usize,
        app_count: usize,
    ) -> Self {
        Self {
            skill_path_present,
            mcp_servers_path_present,
            apps_path_present,
            skill_count,
            mcp_server_count,
            app_count,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CurrentRealityPluginManifestSummary {
    pub name: String,
    pub version: String,
    pub skills_path: String,
    pub mcp_servers_path: String,
    pub apps_path: String,
    pub skill_path_present: bool,
    pub mcp_servers_path_present: bool,
    pub apps_path_present: bool,
    pub skill_count: usize,
    pub mcp_server_count: usize,
    pub app_count: usize,
    pub tool_schema_count: usize,
    pub permission_count: usize,
    pub activation_event_count: usize,
    pub tool_policy_count: usize,
    pub manifest_line_count: usize,
    pub manifest_sha256: String,
}

impl CurrentRealityPluginManifestSummary {
    fn has_fixture_shape(&self) -> bool {
        self.name == "hepta-system"
            && !self.version.trim().is_empty()
            && self.skills_path == "./skills"
            && self.mcp_servers_path == "./.mcp.json"
            && self.apps_path == "./.app.json"
            && self.skill_path_present
            && self.mcp_servers_path_present
            && self.apps_path_present
            && self.skill_count == 1
            && self.mcp_server_count == 1
            && self.app_count == 1
            && self.tool_schema_count == 2
            && self.permission_count == 2
            && self.activation_event_count == 2
            && self.tool_policy_count == 2
            && self.manifest_line_count > 0
            && is_sha256(&self.manifest_sha256)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CurrentRealityPluginManifestObservation {
    manifest_bytes: Vec<u8>,
    assets: CurrentRealityPluginAssetObservation,
    summary: CurrentRealityPluginManifestSummary,
}

impl CurrentRealityPluginManifestObservation {
    pub fn from_manifest_bytes(
        manifest_bytes: Option<&[u8]>,
        assets: CurrentRealityPluginAssetObservation,
    ) -> Result<Self, CurrentRealityCapabilityMatrixError> {
        let manifest_bytes = manifest_bytes
            .filter(|bytes| !bytes.is_empty())
            .ok_or(CurrentRealityCapabilityMatrixError::MissingPluginManifest)?;
        let summary = parse_plugin_manifest_summary(manifest_bytes, assets)?;
        Ok(Self {
            manifest_bytes: manifest_bytes.to_vec(),
            assets,
            summary,
        })
    }

    pub fn summary(&self) -> &CurrentRealityPluginManifestSummary {
        &self.summary
    }

    fn has_integrity(&self) -> bool {
        parse_plugin_manifest_summary(&self.manifest_bytes, self.assets)
            .is_ok_and(|summary| summary == self.summary && summary.has_fixture_shape())
    }
}

fn parse_plugin_manifest_summary(
    bytes: &[u8],
    assets: CurrentRealityPluginAssetObservation,
) -> Result<CurrentRealityPluginManifestSummary, CurrentRealityCapabilityMatrixError> {
    let value: Value = serde_json::from_slice(bytes).map_err(|error| {
        CurrentRealityCapabilityMatrixError::MalformedPluginManifest(error.to_string())
    })?;
    let object = value.as_object().ok_or_else(|| {
        CurrentRealityCapabilityMatrixError::MalformedPluginManifest(
            "top-level value must be an object".to_string(),
        )
    })?;

    let required_string = |field: &str| -> Result<String, CurrentRealityCapabilityMatrixError> {
        object
            .get(field)
            .and_then(Value::as_str)
            .filter(|value| !value.trim().is_empty())
            .map(ToOwned::to_owned)
            .ok_or_else(|| {
                CurrentRealityCapabilityMatrixError::MalformedPluginManifest(format!(
                    "{field} must be a non-empty string"
                ))
            })
    };
    let object_len = |field: &str| -> Result<usize, CurrentRealityCapabilityMatrixError> {
        object
            .get(field)
            .and_then(Value::as_object)
            .map(|value| value.len())
            .ok_or_else(|| {
                CurrentRealityCapabilityMatrixError::MalformedPluginManifest(format!(
                    "{field} must be an object"
                ))
            })
    };

    Ok(CurrentRealityPluginManifestSummary {
        name: required_string("name")?,
        version: required_string("version")?,
        skills_path: required_string("skills")?,
        mcp_servers_path: required_string("mcpServers")?,
        apps_path: required_string("apps")?,
        skill_path_present: assets.skill_path_present,
        mcp_servers_path_present: assets.mcp_servers_path_present,
        apps_path_present: assets.apps_path_present,
        skill_count: assets.skill_count,
        mcp_server_count: assets.mcp_server_count,
        app_count: assets.app_count,
        tool_schema_count: object_len("toolSchemas")?,
        permission_count: object_len("permissions")?,
        activation_event_count: object_len("activationEvents")?,
        tool_policy_count: object_len("toolPolicies")?,
        manifest_line_count: source_line_count(bytes),
        manifest_sha256: sha256_hex(bytes),
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CurrentRealityWorktreeState {
    Clean,
    Dirty,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct CurrentRealityWorktreeObservation {
    pub git_status_entry_count: usize,
    pub git_untracked_count: usize,
    pub git_tracked_change_count: usize,
}

impl CurrentRealityWorktreeObservation {
    pub fn from_counts(
        git_status_entry_count: usize,
        git_tracked_change_count: usize,
        git_untracked_count: usize,
    ) -> Result<Self, CurrentRealityCapabilityMatrixError> {
        let observation = Self {
            git_status_entry_count,
            git_untracked_count,
            git_tracked_change_count,
        };
        if observation.has_integrity() {
            Ok(observation)
        } else {
            Err(CurrentRealityCapabilityMatrixError::InvalidWorktreeObservation)
        }
    }

    pub const fn clean() -> Self {
        Self {
            git_status_entry_count: 0,
            git_untracked_count: 0,
            git_tracked_change_count: 0,
        }
    }

    pub fn state(self) -> CurrentRealityWorktreeState {
        if self.git_status_entry_count == 0 {
            CurrentRealityWorktreeState::Clean
        } else {
            CurrentRealityWorktreeState::Dirty
        }
    }

    fn has_integrity(self) -> bool {
        self.git_status_entry_count == self.git_tracked_change_count + self.git_untracked_count
            && (self.git_status_entry_count > 0
                || (self.git_tracked_change_count == 0 && self.git_untracked_count == 0))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CurrentRealityTypedComponent {
    pub id: &'static str,
    pub rust_type: &'static str,
    pub compiled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CurrentRealityTypedComponentInventory {
    pub components: Vec<CurrentRealityTypedComponent>,
}

impl CurrentRealityTypedComponentInventory {
    pub fn compiled() -> Self {
        Self {
            components: vec![
                CurrentRealityTypedComponent {
                    id: "plugin_lifecycle_state_machine",
                    rust_type: type_name::<
                        codex_core_plugins::lifecycle_state_machine::PluginLifecycleStateMachinePlan,
                    >(),
                    compiled: true,
                },
                CurrentRealityTypedComponent {
                    id: "plugin_lifecycle_phase_summary",
                    rust_type: type_name::<
                        codex_core_plugins::lifecycle_phase_summary::PluginLifecyclePhaseSummary,
                    >(),
                    compiled: true,
                },
                CurrentRealityTypedComponent {
                    id: "workflow_durable_store_adapter",
                    rust_type: type_name::<
                        crate::workflow_durable_store_adapter::WorkflowDurableStoreAdapterReport,
                    >(),
                    compiled: true,
                },
                CurrentRealityTypedComponent {
                    id: "workflow_durable_store_append_plan",
                    rust_type: type_name::<
                        crate::workflow_durable_store_append_plan::WorkflowDurableStoreAppendPlanReport,
                    >(),
                    compiled: true,
                },
                CurrentRealityTypedComponent {
                    id: "workflow_durable_store_adapter_harness",
                    rust_type: type_name::<
                        crate::workflow_durable_store_adapter_harness::WorkflowDurableStoreAdapterHarnessReport,
                    >(),
                    compiled: true,
                },
            ],
        }
    }

    fn has_integrity(&self) -> bool {
        self == &Self::compiled()
            && self.components.len() == 5
            && self.components.iter().all(|entry| entry.compiled)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CurrentRealityWorkGraphSuccessorObservation {
    pub current_state_inventory: WorkGraphCurrentStateInventoryReport,
    pub terminal_non_promotion_receipt:
        WorkGraphPersistenceAcceptanceEffectDenialReceiptRetentionReadbackAckTerminalDecisionReceiptPreviewReport,
}

impl CurrentRealityWorkGraphSuccessorObservation {
    pub fn from_current_typed_reports() -> Self {
        Self {
            current_state_inventory: hepta_work_graph_current_state_inventory_report(),
            terminal_non_promotion_receipt:
                hepta_work_graph_persistence_acceptance_effect_application_denial_receipt_retention_expiry_readback_acknowledgement_terminal_decision_non_promotion_receipt_preview_report(),
        }
    }

    fn has_integrity(&self) -> bool {
        if self != &Self::from_current_typed_reports()
            || self.current_state_inventory.ready_for_scheduler_cutover
        {
            return false;
        }

        let inventory_effects = serde_json::to_value(&self.current_state_inventory.side_effects);
        let terminal_effects =
            serde_json::to_value(&self.terminal_non_promotion_receipt.side_effects);
        let terminal_report = serde_json::to_value(&self.terminal_non_promotion_receipt);
        inventory_effects.is_ok_and(|value| all_boolean_leaves_are_false(&value))
            && terminal_effects.is_ok_and(|value| all_boolean_leaves_are_false(&value))
            && terminal_report.is_ok_and(|value| {
                named_boolean_fields_are_false(
                    &value,
                    &[
                        "persisted",
                        "acceptance_allowed",
                        "receipt_recording_allowed",
                        "promotion_allowed",
                        "external_delivery_enabled",
                        "authority_granted",
                    ],
                )
            })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CurrentRealityCapabilityStatus {
    Ready,
    ReadyBlocked,
    Blocked,
}

impl CurrentRealityCapabilityStatus {
    const fn is_ready(self) -> bool {
        matches!(self, Self::Ready | Self::ReadyBlocked)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CurrentRealityCapabilitySourceObservation {
    pub capability_id: String,
    pub status: CurrentRealityCapabilityStatus,
    pub source_report_id: String,
    pub current_fact: String,
    source_payload: Vec<u8>,
    source_payload_sha256: String,
}

impl CurrentRealityCapabilitySourceObservation {
    pub fn new(
        capability_id: impl Into<String>,
        status: CurrentRealityCapabilityStatus,
        source_report_id: impl Into<String>,
        current_fact: impl Into<String>,
        source_payload: impl Into<Vec<u8>>,
    ) -> Result<Self, CurrentRealityCapabilityMatrixError> {
        let capability_id = capability_id.into();
        let source_report_id = source_report_id.into();
        let current_fact = current_fact.into();
        let source_payload = source_payload.into();
        if capability_id.is_empty()
            || source_report_id.is_empty()
            || source_report_id.contains('/')
            || current_fact.is_empty()
            || source_payload.is_empty()
        {
            return Err(CurrentRealityCapabilityMatrixError::InvalidCapabilityObservations);
        }
        let source_payload_sha256 = sha256_hex(&source_payload);
        Ok(Self {
            capability_id,
            status,
            source_report_id,
            current_fact,
            source_payload,
            source_payload_sha256,
        })
    }

    fn has_integrity(&self) -> bool {
        !self.capability_id.is_empty()
            && !self.source_report_id.is_empty()
            && !self.source_report_id.contains('/')
            && !self.current_fact.is_empty()
            && !self.source_payload.is_empty()
            && self.source_payload_sha256 == sha256_hex(&self.source_payload)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CurrentRealityCapabilityMatrixSources {
    pub plugin_manifest: CurrentRealityPluginManifestObservation,
    pub worktree: CurrentRealityWorktreeObservation,
    pub typed_components: CurrentRealityTypedComponentInventory,
    pub work_graph_successors: CurrentRealityWorkGraphSuccessorObservation,
    pub capability_observations: Vec<CurrentRealityCapabilitySourceObservation>,
}

impl CurrentRealityCapabilityMatrixSources {
    pub fn new(
        plugin_manifest: CurrentRealityPluginManifestObservation,
        worktree: CurrentRealityWorktreeObservation,
        typed_components: CurrentRealityTypedComponentInventory,
        work_graph_successors: CurrentRealityWorkGraphSuccessorObservation,
        capability_observations: Vec<CurrentRealityCapabilitySourceObservation>,
    ) -> Self {
        Self {
            plugin_manifest,
            worktree,
            typed_components,
            work_graph_successors,
            capability_observations,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CurrentRealityControlledLiveSlotState {
    ActiveBlocker,
    Satisfied,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CurrentRealityControlledLiveReadbackSlot {
    pub id: &'static str,
    pub blocker_id: &'static str,
    pub state: CurrentRealityControlledLiveSlotState,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CurrentRealityCapabilityRow {
    pub id: &'static str,
    pub layer: CurrentRealityCapabilityLayer,
    pub status: CurrentRealityCapabilityStatus,
    pub ready: bool,
    pub live_enabled: bool,
    pub source_report_id: String,
    pub source_sha256: String,
    pub current_fact: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatibility_alias_successor: Option<&'static str>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CurrentRealitySourceBinding {
    pub id: &'static str,
    pub schema_version: &'static str,
    pub generation: &'static str,
    pub line_count: usize,
    pub sha256: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct CurrentRealityCapabilityMatrixSideEffects {
    pub report_written: bool,
    pub git_index_mutated: bool,
    pub plugin_installed: bool,
    pub plugin_cache_mutated: bool,
    pub tool_registered: bool,
    pub tool_invoked: bool,
    pub tool_invocation_ledger_written: bool,
    pub approval_requested: bool,
    pub approval_accepted: bool,
    pub approval_broker_mutated: bool,
    pub evidence_recorded: bool,
    pub evidence_persisted: bool,
    pub workflow_event_log_mutated: bool,
    pub sqlite_written: bool,
    pub workgraph_execution_started: bool,
    pub replay_executed: bool,
    pub rollback_executed: bool,
    pub readback_receipt_persisted: bool,
    pub gateway_or_auth_mutated: bool,
    pub native_post_mutation_performed: bool,
    pub channel_send_performed: bool,
    pub provider_invoked: bool,
    pub model_invoked: bool,
    pub package_or_release_written: bool,
    pub public_ga_promoted: bool,
}

impl CurrentRealityCapabilityMatrixSideEffects {
    pub const fn none() -> Self {
        Self {
            report_written: false,
            git_index_mutated: false,
            plugin_installed: false,
            plugin_cache_mutated: false,
            tool_registered: false,
            tool_invoked: false,
            tool_invocation_ledger_written: false,
            approval_requested: false,
            approval_accepted: false,
            approval_broker_mutated: false,
            evidence_recorded: false,
            evidence_persisted: false,
            workflow_event_log_mutated: false,
            sqlite_written: false,
            workgraph_execution_started: false,
            replay_executed: false,
            rollback_executed: false,
            readback_receipt_persisted: false,
            gateway_or_auth_mutated: false,
            native_post_mutation_performed: false,
            channel_send_performed: false,
            provider_invoked: false,
            model_invoked: false,
            package_or_release_written: false,
            public_ga_promoted: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CurrentRealityCapabilityMatrixReport {
    pub runtime: &'static str,
    pub product: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub compatibility_migration: &'static str,
    pub matrix_date: &'static str,
    pub local_capability_count: usize,
    pub local_capability_ready_count: usize,
    pub local_capability_blocked_count: usize,
    pub live_enabled_count: usize,
    pub all_live_paths_blocked: bool,
    pub plugin_fixture_shape_ready: bool,
    pub plugin_manifest_present: bool,
    pub plugin_manifest_summary: CurrentRealityPluginManifestSummary,
    pub typed_component_count: usize,
    pub memory_filesystem_drift_tracked: bool,
    pub worktree_boundary_observed: bool,
    pub worktree_boundary_state: CurrentRealityWorktreeState,
    pub dirty_worktree_boundary_tracked: bool,
    pub git_status_entry_count: usize,
    pub git_untracked_count: usize,
    pub git_tracked_change_count: usize,
    pub controlled_live_audit_ready: bool,
    pub controlled_live_cutover_ready: bool,
    pub controlled_live_active_blocker_count: usize,
    pub controlled_live_satisfied_readback_slot_count: usize,
    pub controlled_live_readback_slot_count: usize,
    pub current_reality_capability_matrix_ready: bool,
    pub production_authority_granted: bool,
    pub write_authority_granted: bool,
    pub approval_authority_granted: bool,
    pub mutation_authority_granted: bool,
    pub evidence_authority_granted: bool,
    pub send_authority_granted: bool,
    pub capabilities: Vec<CurrentRealityCapabilityRow>,
    pub controlled_live_readback_slots: Vec<CurrentRealityControlledLiveReadbackSlot>,
    pub blockers: Vec<&'static str>,
    pub next_actions: Vec<&'static str>,
    pub next_migration_step: &'static str,
    pub source_bindings: Vec<CurrentRealitySourceBinding>,
    pub side_effect_free: bool,
    pub side_effects: CurrentRealityCapabilityMatrixSideEffects,
}

impl CurrentRealityCapabilityMatrixReport {
    pub fn has_current_reality_integrity(
        &self,
        sources: &CurrentRealityCapabilityMatrixSources,
    ) -> bool {
        current_reality_capability_matrix_report_from_sources(sources)
            .is_ok_and(|expected| expected == *self)
    }

    fn has_static_authority_boundaries(&self) -> bool {
        let expected_ids = CURRENT_REALITY_CAPABILITY_IDS.iter().copied();
        let actual_ids = self.capabilities.iter().map(|row| row.id);
        self.runtime == "hepta"
            && self.product == "Hepta"
            && self.surface == "current_reality_capability_matrix"
            && self.status == "blocked"
            && self.gate == CURRENT_REALITY_CAPABILITY_MATRIX_GATE
            && self.schema_version == CURRENT_REALITY_CAPABILITY_MATRIX_SCHEMA_VERSION
            && self.compatibility_migration
                == CURRENT_REALITY_CAPABILITY_MATRIX_COMPATIBILITY_MIGRATION
            && self.local_capability_count == CURRENT_REALITY_CAPABILITY_IDS.len()
            && self.local_capability_ready_count > 0
            && self.local_capability_blocked_count > 0
            && self.local_capability_ready_count + self.local_capability_blocked_count
                == self.local_capability_count
            && self.live_enabled_count == 0
            && self.all_live_paths_blocked
            && self.plugin_fixture_shape_ready
            && self.plugin_manifest_present
            && self.plugin_manifest_summary.has_fixture_shape()
            && self.typed_component_count == 5
            && self.memory_filesystem_drift_tracked
            && self.worktree_boundary_observed
            && self.git_status_entry_count
                == self.git_tracked_change_count + self.git_untracked_count
            && self.controlled_live_audit_ready
            && !self.controlled_live_cutover_ready
            && self.controlled_live_readback_slot_count == 7
            && ((self.worktree_boundary_state == CurrentRealityWorktreeState::Clean
                && self.controlled_live_active_blocker_count == 6
                && self.controlled_live_satisfied_readback_slot_count == 1
                && !self.dirty_worktree_boundary_tracked)
                || (self.worktree_boundary_state == CurrentRealityWorktreeState::Dirty
                    && self.controlled_live_active_blocker_count == 7
                    && self.controlled_live_satisfied_readback_slot_count == 0
                    && self.dirty_worktree_boundary_tracked))
            && !self.current_reality_capability_matrix_ready
            && !self.production_authority_granted
            && !self.write_authority_granted
            && !self.approval_authority_granted
            && !self.mutation_authority_granted
            && !self.evidence_authority_granted
            && !self.send_authority_granted
            && self.capabilities.iter().all(|row| !row.live_enabled)
            && expected_ids.eq(actual_ids)
            && self.source_bindings.len() == 7
            && self
                .source_bindings
                .iter()
                .all(|binding| binding.line_count > 0 && is_sha256(&binding.sha256))
            && self
                .blockers
                .contains(&"controlled_live_cutover_blocked_by_operator_approval_and_evidence")
            && self
                .blockers
                .contains(&"controlled_live_denial_readback_index_blocks_waiver_and_acceptance")
            && self
                .blockers
                .contains(&"live_and_public_ga_blocked_by_design")
            && self
                .next_actions
                .contains(&CURRENT_REALITY_CAPABILITY_MATRIX_NEXT_ACTION)
            && self.next_migration_step == CURRENT_REALITY_CAPABILITY_MATRIX_NEXT_ACTION
            && self.side_effect_free
            && self.side_effects == CurrentRealityCapabilityMatrixSideEffects::none()
    }
}

pub fn current_reality_capability_matrix_report_from_sources(
    sources: &CurrentRealityCapabilityMatrixSources,
) -> Result<CurrentRealityCapabilityMatrixReport, CurrentRealityCapabilityMatrixError> {
    if !sources.plugin_manifest.has_integrity() {
        return Err(CurrentRealityCapabilityMatrixError::InvalidPluginFixtureShape);
    }
    if !sources.worktree.has_integrity() {
        return Err(CurrentRealityCapabilityMatrixError::InvalidWorktreeObservation);
    }
    if !sources.typed_components.has_integrity() {
        return Err(CurrentRealityCapabilityMatrixError::InvalidTypedComponentInventory);
    }
    if !sources.work_graph_successors.has_integrity() {
        return Err(CurrentRealityCapabilityMatrixError::InvalidWorkGraphSuccessors);
    }
    validate_capability_observations(&sources.capability_observations)?;

    let catalog = current_reality_capability_catalog();
    let worktree_state = sources.worktree.state();
    let readback_slots = controlled_live_readback_slots(worktree_state);
    let active_blocker_count = readback_slots
        .iter()
        .filter(|slot| slot.state == CurrentRealityControlledLiveSlotState::ActiveBlocker)
        .count();
    let satisfied_readback_slot_count = readback_slots.len() - active_blocker_count;

    let current_state_bytes = serialize_source(
        &sources.work_graph_successors.current_state_inventory,
        "current WorkGraph inventory",
    )?;
    let terminal_receipt_bytes = serialize_source(
        &sources.work_graph_successors.terminal_non_promotion_receipt,
        "terminal non-promotion receipt",
    )?;
    let worktree_bytes = serialize_source(&sources.worktree, "worktree observation")?;
    let slot_bytes = serialize_source(&readback_slots, "controlled-live readback slots")?;

    let capabilities = catalog
        .iter()
        .zip(&sources.capability_observations)
        .map(|(descriptor, observation)| {
            capability_row(
                descriptor,
                observation,
                worktree_state,
                &current_state_bytes,
                &terminal_receipt_bytes,
                &worktree_bytes,
                &slot_bytes,
            )
        })
        .collect::<Vec<_>>();
    let ready_count = capabilities.iter().filter(|row| row.ready).count();
    let blocked_count = capabilities.len() - ready_count;
    if ready_count == 0 || blocked_count == 0 {
        return Err(CurrentRealityCapabilityMatrixError::MissingReadyBlockedBoundary);
    }

    let blockers = readback_slots
        .iter()
        .filter(|slot| slot.state == CurrentRealityControlledLiveSlotState::ActiveBlocker)
        .map(|slot| slot.blocker_id)
        .collect::<Vec<_>>();
    let source_bindings = source_bindings(
        sources,
        &current_state_bytes,
        &terminal_receipt_bytes,
        &worktree_bytes,
    )?;

    let report = CurrentRealityCapabilityMatrixReport {
        runtime: "hepta",
        product: "Hepta",
        surface: "current_reality_capability_matrix",
        status: "blocked",
        gate: CURRENT_REALITY_CAPABILITY_MATRIX_GATE,
        schema_version: CURRENT_REALITY_CAPABILITY_MATRIX_SCHEMA_VERSION,
        compatibility_migration: CURRENT_REALITY_CAPABILITY_MATRIX_COMPATIBILITY_MIGRATION,
        matrix_date: "2026-06-27",
        local_capability_count: capabilities.len(),
        local_capability_ready_count: ready_count,
        local_capability_blocked_count: blocked_count,
        live_enabled_count: 0,
        all_live_paths_blocked: true,
        plugin_fixture_shape_ready: true,
        plugin_manifest_present: true,
        plugin_manifest_summary: sources.plugin_manifest.summary.clone(),
        typed_component_count: sources.typed_components.components.len(),
        memory_filesystem_drift_tracked: true,
        worktree_boundary_observed: true,
        worktree_boundary_state: worktree_state,
        dirty_worktree_boundary_tracked: worktree_state == CurrentRealityWorktreeState::Dirty,
        git_status_entry_count: sources.worktree.git_status_entry_count,
        git_untracked_count: sources.worktree.git_untracked_count,
        git_tracked_change_count: sources.worktree.git_tracked_change_count,
        controlled_live_audit_ready: true,
        controlled_live_cutover_ready: false,
        controlled_live_active_blocker_count: active_blocker_count,
        controlled_live_satisfied_readback_slot_count: satisfied_readback_slot_count,
        controlled_live_readback_slot_count: readback_slots.len(),
        current_reality_capability_matrix_ready: false,
        production_authority_granted: false,
        write_authority_granted: false,
        approval_authority_granted: false,
        mutation_authority_granted: false,
        evidence_authority_granted: false,
        send_authority_granted: false,
        capabilities,
        controlled_live_readback_slots: readback_slots,
        blockers,
        next_actions: vec![CURRENT_REALITY_CAPABILITY_MATRIX_NEXT_ACTION],
        next_migration_step: CURRENT_REALITY_CAPABILITY_MATRIX_NEXT_ACTION,
        source_bindings,
        side_effect_free: true,
        side_effects: CurrentRealityCapabilityMatrixSideEffects::none(),
    };
    if report.has_static_authority_boundaries() {
        Ok(report)
    } else {
        Err(CurrentRealityCapabilityMatrixError::InvalidCapabilityObservations)
    }
}

fn validate_capability_observations(
    observations: &[CurrentRealityCapabilitySourceObservation],
) -> Result<(), CurrentRealityCapabilityMatrixError> {
    if observations.len() != CURRENT_REALITY_CAPABILITY_IDS.len()
        || !CURRENT_REALITY_CAPABILITY_IDS.iter().zip(observations).all(
            |(expected, observation)| {
                *expected == observation.capability_id && observation.has_integrity()
            },
        )
        || observations
            .iter()
            .map(|observation| observation.capability_id.as_str())
            .collect::<BTreeSet<_>>()
            .len()
            != CURRENT_REALITY_CAPABILITY_IDS.len()
    {
        return Err(CurrentRealityCapabilityMatrixError::InvalidCapabilityObservations);
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn capability_row(
    descriptor: &CurrentRealityCapabilityDescriptor,
    observation: &CurrentRealityCapabilitySourceObservation,
    worktree_state: CurrentRealityWorktreeState,
    current_state_bytes: &[u8],
    terminal_receipt_bytes: &[u8],
    worktree_bytes: &[u8],
    slot_bytes: &[u8],
) -> CurrentRealityCapabilityRow {
    let (status, source_report_id, source_sha256, current_fact, compatibility_alias_successor) =
        match descriptor.source {
            CurrentRealityCapabilitySource::CompatibilityAlias {
                successor_report_id,
            } if descriptor.id == CURRENT_REALITY_WORK_GRAPH_DURABLE_IDENTITY_ALIAS => (
                CurrentRealityCapabilityStatus::Ready,
                successor_report_id.to_string(),
                sha256_hex(current_state_bytes),
                "compatibility identity is derived from the current typed WorkGraph inventory"
                    .to_string(),
                Some(successor_report_id),
            ),
            CurrentRealityCapabilitySource::CompatibilityAlias {
                successor_report_id,
            } if descriptor.id == CURRENT_REALITY_WORK_GRAPH_RECEIPT_TAIL_ALIAS => (
                CurrentRealityCapabilityStatus::Ready,
                successor_report_id.to_string(),
                sha256_hex(terminal_receipt_bytes),
                "compatibility receipt tail is derived from the current typed terminal non-promotion receipt"
                    .to_string(),
                Some(successor_report_id),
            ),
            _ if descriptor.id == CONTROLLED_LIVE_AUDIT_ID => (
                CurrentRealityCapabilityStatus::ReadyBlocked,
                "hepta-systems-controlled-live-readiness-audit".to_string(),
                sha256_hex(worktree_bytes),
                match worktree_state {
                    CurrentRealityWorktreeState::Clean => "controlled-live audit is ready-blocked with a clean worktree and six explicit approval/evidence blockers",
                    CurrentRealityWorktreeState::Dirty => "controlled-live audit is ready-blocked with a dirty-worktree blocker plus six explicit approval/evidence blockers",
                }
                .to_string(),
                None,
            ),
            _ if descriptor.id == CONTROLLED_LIVE_DENIAL_INDEX_ID => (
                CurrentRealityCapabilityStatus::ReadyBlocked,
                "hepta-systems-controlled-live-readiness-denial-readback-index".to_string(),
                sha256_hex(slot_bytes),
                "seven stable readback slots expose the actual six-or-seven active blockers while waiver, acceptance, persistence, approval, and live execution remain disabled"
                    .to_string(),
                None,
            ),
            _ => (
                observation.status,
                observation.source_report_id.clone(),
                observation.source_payload_sha256.clone(),
                observation.current_fact.clone(),
                None,
            ),
        };

    CurrentRealityCapabilityRow {
        id: descriptor.id,
        layer: descriptor.layer,
        status,
        ready: status.is_ready(),
        live_enabled: false,
        source_report_id,
        source_sha256,
        current_fact,
        compatibility_alias_successor,
    }
}

fn controlled_live_readback_slots(
    worktree_state: CurrentRealityWorktreeState,
) -> Vec<CurrentRealityControlledLiveReadbackSlot> {
    vec![
        CurrentRealityControlledLiveReadbackSlot {
            id: "worktree_boundary",
            blocker_id: "dirty_worktree_boundary",
            state: if worktree_state == CurrentRealityWorktreeState::Clean {
                CurrentRealityControlledLiveSlotState::Satisfied
            } else {
                CurrentRealityControlledLiveSlotState::ActiveBlocker
            },
        },
        CurrentRealityControlledLiveReadbackSlot {
            id: "operator_approval",
            blocker_id: "controlled_live_cutover_blocked_by_operator_approval_and_evidence",
            state: CurrentRealityControlledLiveSlotState::ActiveBlocker,
        },
        CurrentRealityControlledLiveReadbackSlot {
            id: "denial_readback_waiver",
            blocker_id: "controlled_live_denial_readback_index_blocks_waiver_and_acceptance",
            state: CurrentRealityControlledLiveSlotState::ActiveBlocker,
        },
        CurrentRealityControlledLiveReadbackSlot {
            id: "required_evidence",
            blocker_id: "controlled_live_required_evidence_not_accepted",
            state: CurrentRealityControlledLiveSlotState::ActiveBlocker,
        },
        CurrentRealityControlledLiveReadbackSlot {
            id: "persistence_authority",
            blocker_id: "controlled_live_persistence_authority_not_granted",
            state: CurrentRealityControlledLiveSlotState::ActiveBlocker,
        },
        CurrentRealityControlledLiveReadbackSlot {
            id: "kill_switch_rehearsal",
            blocker_id: "controlled_live_kill_switch_rehearsal_not_accepted",
            state: CurrentRealityControlledLiveSlotState::ActiveBlocker,
        },
        CurrentRealityControlledLiveReadbackSlot {
            id: "live_activation",
            blocker_id: "live_and_public_ga_blocked_by_design",
            state: CurrentRealityControlledLiveSlotState::ActiveBlocker,
        },
    ]
}

#[derive(Serialize)]
struct CapabilityObservationReceipt<'a> {
    capability_id: &'a str,
    status: CurrentRealityCapabilityStatus,
    source_report_id: &'a str,
    current_fact: &'a str,
    source_payload_sha256: &'a str,
}

fn source_bindings(
    sources: &CurrentRealityCapabilityMatrixSources,
    current_state_bytes: &[u8],
    terminal_receipt_bytes: &[u8],
    worktree_bytes: &[u8],
) -> Result<Vec<CurrentRealitySourceBinding>, CurrentRealityCapabilityMatrixError> {
    let catalog_payload = format!("{}\n", CURRENT_REALITY_CAPABILITY_IDS.join("\n"));
    if sha256_hex(catalog_payload.as_bytes()) != CURRENT_REALITY_CAPABILITY_CATALOG_ID_SHA256 {
        return Err(CurrentRealityCapabilityMatrixError::InvalidCapabilityObservations);
    }
    let typed_component_bytes = serialize_source(
        &sources.typed_components,
        "compiled typed component inventory",
    )?;
    let readiness_receipts = sources
        .capability_observations
        .iter()
        .map(|observation| CapabilityObservationReceipt {
            capability_id: &observation.capability_id,
            status: observation.status,
            source_report_id: &observation.source_report_id,
            current_fact: &observation.current_fact,
            source_payload_sha256: &observation.source_payload_sha256,
        })
        .collect::<Vec<_>>();
    let readiness_bytes = serialize_source(&readiness_receipts, "capability observations")?;

    Ok(vec![
        source_binding(
            "typed_capability_catalog",
            CURRENT_REALITY_CAPABILITY_CATALOG_SCHEMA_VERSION,
            "compiled_core_catalog",
            catalog_payload.as_bytes(),
        ),
        source_binding(
            "plugin_manifest",
            "codex_plugin_manifest_v1",
            "explicit_manifest_bytes",
            &sources.plugin_manifest.manifest_bytes,
        ),
        source_binding(
            "worktree_observation",
            "current_reality_worktree_observation_v1",
            "explicit_read_only_observation",
            worktree_bytes,
        ),
        source_binding(
            "typed_component_inventory",
            "current_reality_typed_component_inventory_v1",
            "compile_bound_types",
            &typed_component_bytes,
        ),
        source_binding(
            "work_graph_current_state_inventory",
            "work_graph_current_state_inventory_v1",
            "typed_successor_report",
            current_state_bytes,
        ),
        source_binding(
            "work_graph_terminal_non_promotion_receipt",
            "work_graph_terminal_non_promotion_receipt_preview_v1",
            "typed_successor_report",
            terminal_receipt_bytes,
        ),
        source_binding(
            "capability_observations",
            "current_reality_capability_observations_v1",
            "explicit_typed_observations",
            &readiness_bytes,
        ),
    ])
}

fn source_binding(
    id: &'static str,
    schema_version: &'static str,
    generation: &'static str,
    bytes: &[u8],
) -> CurrentRealitySourceBinding {
    CurrentRealitySourceBinding {
        id,
        schema_version,
        generation,
        line_count: source_line_count(bytes),
        sha256: sha256_hex(bytes),
    }
}

fn serialize_source<T: Serialize>(
    value: &T,
    name: &str,
) -> Result<Vec<u8>, CurrentRealityCapabilityMatrixError> {
    serde_json::to_vec(value).map_err(|error| {
        CurrentRealityCapabilityMatrixError::SourceSerialization(format!("{name}: {error}"))
    })
}

fn source_line_count(bytes: &[u8]) -> usize {
    if bytes.is_empty() {
        0
    } else {
        bytes.iter().filter(|byte| **byte == b'\n').count() + usize::from(!bytes.ends_with(b"\n"))
    }
}

fn sha256_hex(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    digest.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn is_sha256(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn all_boolean_leaves_are_false(value: &Value) -> bool {
    match value {
        Value::Bool(value) => !value,
        Value::Array(values) => values.iter().all(all_boolean_leaves_are_false),
        Value::Object(values) => values.values().all(all_boolean_leaves_are_false),
        _ => true,
    }
}

fn named_boolean_fields_are_false(value: &Value, field_names: &[&str]) -> bool {
    match value {
        Value::Array(values) => values
            .iter()
            .all(|value| named_boolean_fields_are_false(value, field_names)),
        Value::Object(values) => values.iter().all(|(name, value)| {
            !(field_names.contains(&name.as_str()) && value == &Value::Bool(true))
                && named_boolean_fields_are_false(value, field_names)
        }),
        _ => true,
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    fn plugin_manifest_fixture() -> Vec<u8> {
        serde_json::to_vec_pretty(&json!({
            "name": "hepta-system",
            "version": "0.0.0-fixture",
            "skills": "./skills",
            "mcpServers": "./.mcp.json",
            "apps": "./.app.json",
            "toolSchemas": {"mcp": {}, "app": {}},
            "permissions": {"mcp": {}, "app": {}},
            "activationEvents": {"mcp": [], "app": []},
            "toolPolicies": {"mcp": {}, "app": {}}
        }))
        .expect("serialize plugin fixture")
    }

    fn plugin_observation() -> CurrentRealityPluginManifestObservation {
        CurrentRealityPluginManifestObservation::from_manifest_bytes(
            Some(&plugin_manifest_fixture()),
            CurrentRealityPluginAssetObservation::new(true, true, true, 1, 1, 1),
        )
        .expect("typed plugin observation")
    }

    fn capability_observations() -> Vec<CurrentRealityCapabilitySourceObservation> {
        CURRENT_REALITY_CAPABILITY_IDS
            .iter()
            .map(|id| {
                let status = if *id == CURRENT_COMPACT_SUMMARY_ID {
                    CurrentRealityCapabilityStatus::Blocked
                } else {
                    CurrentRealityCapabilityStatus::Ready
                };
                CurrentRealityCapabilitySourceObservation::new(
                    *id,
                    status,
                    format!("typed-{id}"),
                    format!("typed source observation for {id}"),
                    serde_json::to_vec(&json!({"id": id, "status": status}))
                        .expect("serialize capability fixture"),
                )
                .expect("typed capability observation")
            })
            .collect()
    }

    fn fixture_sources(
        worktree: CurrentRealityWorktreeObservation,
    ) -> CurrentRealityCapabilityMatrixSources {
        CurrentRealityCapabilityMatrixSources::new(
            plugin_observation(),
            worktree,
            CurrentRealityTypedComponentInventory::compiled(),
            CurrentRealityWorkGraphSuccessorObservation::from_current_typed_reports(),
            capability_observations(),
        )
    }

    #[test]
    fn typed_catalog_digest_and_aliases_are_bound_without_a_generated_json_registry() {
        let payload = format!("{}\n", CURRENT_REALITY_CAPABILITY_IDS.join("\n"));
        assert_eq!(
            sha256_hex(payload.as_bytes()),
            CURRENT_REALITY_CAPABILITY_CATALOG_ID_SHA256
        );

        let catalog = current_reality_capability_catalog();
        assert_eq!(catalog.len(), 104);
        assert!(catalog.iter().any(|row| {
            row.id == CURRENT_REALITY_WORK_GRAPH_DURABLE_IDENTITY_ALIAS
                && matches!(
                    row.source,
                    CurrentRealityCapabilitySource::CompatibilityAlias { .. }
                )
        }));
        assert!(catalog.iter().any(|row| {
            row.id == CURRENT_REALITY_WORK_GRAPH_RECEIPT_TAIL_ALIAS
                && matches!(
                    row.source,
                    CurrentRealityCapabilitySource::CompatibilityAlias { .. }
                )
        }));
    }

    #[test]
    fn clean_and_dirty_worktrees_preserve_the_exact_six_or_seven_blocker_boundary() {
        let clean_sources = fixture_sources(CurrentRealityWorktreeObservation::clean());
        let clean = current_reality_capability_matrix_report_from_sources(&clean_sources)
            .expect("clean current-reality report");
        assert_eq!(clean.controlled_live_readback_slot_count, 7);
        assert_eq!(clean.controlled_live_active_blocker_count, 6);
        assert_eq!(clean.controlled_live_satisfied_readback_slot_count, 1);
        assert_eq!(
            clean.worktree_boundary_state,
            CurrentRealityWorktreeState::Clean
        );
        assert!(!clean.dirty_worktree_boundary_tracked);
        assert!(clean.has_current_reality_integrity(&clean_sources));

        let dirty_worktree = CurrentRealityWorktreeObservation::from_counts(3, 2, 1)
            .expect("valid dirty observation");
        let dirty_sources = fixture_sources(dirty_worktree);
        let dirty = current_reality_capability_matrix_report_from_sources(&dirty_sources)
            .expect("dirty current-reality report");
        assert_eq!(dirty.controlled_live_readback_slot_count, 7);
        assert_eq!(dirty.controlled_live_active_blocker_count, 7);
        assert_eq!(dirty.controlled_live_satisfied_readback_slot_count, 0);
        assert_eq!(
            dirty.worktree_boundary_state,
            CurrentRealityWorktreeState::Dirty
        );
        assert!(dirty.dirty_worktree_boundary_tracked);
        assert!(dirty.has_current_reality_integrity(&dirty_sources));
    }

    #[test]
    fn report_is_permanently_non_authoritative_and_side_effect_free() {
        let sources = fixture_sources(CurrentRealityWorktreeObservation::clean());
        let report = current_reality_capability_matrix_report_from_sources(&sources)
            .expect("typed current-reality report");

        assert!(!report.current_reality_capability_matrix_ready);
        assert!(!report.controlled_live_cutover_ready);
        assert!(!report.production_authority_granted);
        assert!(!report.write_authority_granted);
        assert!(!report.approval_authority_granted);
        assert!(!report.mutation_authority_granted);
        assert!(!report.evidence_authority_granted);
        assert!(!report.send_authority_granted);
        assert!(report.capabilities.iter().all(|row| !row.live_enabled));
        assert_eq!(
            report.side_effects,
            CurrentRealityCapabilityMatrixSideEffects::none()
        );
        assert!(all_boolean_leaves_are_false(
            &serde_json::to_value(report.side_effects).expect("serialize side effects")
        ));
    }

    #[test]
    fn missing_malformed_or_wrong_shape_plugin_manifests_fail_closed() {
        let assets = CurrentRealityPluginAssetObservation::new(true, true, true, 1, 1, 1);
        assert_eq!(
            CurrentRealityPluginManifestObservation::from_manifest_bytes(None, assets),
            Err(CurrentRealityCapabilityMatrixError::MissingPluginManifest)
        );
        assert!(matches!(
            CurrentRealityPluginManifestObservation::from_manifest_bytes(Some(b"{"), assets),
            Err(CurrentRealityCapabilityMatrixError::MalformedPluginManifest(_))
        ));

        let wrong_shape = CurrentRealityPluginManifestObservation::from_manifest_bytes(
            Some(&plugin_manifest_fixture()),
            CurrentRealityPluginAssetObservation::new(true, true, true, 0, 1, 1),
        )
        .expect("parse wrong-shape manifest");
        let mut wrong_shape_sources = fixture_sources(CurrentRealityWorktreeObservation::clean());
        wrong_shape_sources.plugin_manifest = wrong_shape;
        assert_eq!(
            current_reality_capability_matrix_report_from_sources(&wrong_shape_sources),
            Err(CurrentRealityCapabilityMatrixError::InvalidPluginFixtureShape)
        );
    }

    #[test]
    fn altered_work_graph_successors_fail_closed_without_reviving_retired_modules() {
        let mut sources = fixture_sources(CurrentRealityWorktreeObservation::clean());
        sources.work_graph_successors.current_state_inventory.status = "tampered";
        assert_eq!(
            current_reality_capability_matrix_report_from_sources(&sources),
            Err(CurrentRealityCapabilityMatrixError::InvalidWorkGraphSuccessors)
        );

        let mut sources = fixture_sources(CurrentRealityWorktreeObservation::clean());
        sources
            .work_graph_successors
            .terminal_non_promotion_receipt
            .receipt_count += 1;
        assert_eq!(
            current_reality_capability_matrix_report_from_sources(&sources),
            Err(CurrentRealityCapabilityMatrixError::InvalidWorkGraphSuccessors)
        );
    }

    #[test]
    fn missing_duplicate_or_payload_tampered_capability_observations_fail_closed() {
        let mut missing = fixture_sources(CurrentRealityWorktreeObservation::clean());
        missing.capability_observations.pop();
        assert_eq!(
            current_reality_capability_matrix_report_from_sources(&missing),
            Err(CurrentRealityCapabilityMatrixError::InvalidCapabilityObservations)
        );

        let mut duplicate = fixture_sources(CurrentRealityWorktreeObservation::clean());
        duplicate.capability_observations[1].capability_id =
            duplicate.capability_observations[0].capability_id.clone();
        assert_eq!(
            current_reality_capability_matrix_report_from_sources(&duplicate),
            Err(CurrentRealityCapabilityMatrixError::InvalidCapabilityObservations)
        );

        let mut tampered = fixture_sources(CurrentRealityWorktreeObservation::clean());
        tampered.capability_observations[0]
            .source_payload
            .extend_from_slice(b"tampered");
        assert_eq!(
            current_reality_capability_matrix_report_from_sources(&tampered),
            Err(CurrentRealityCapabilityMatrixError::InvalidCapabilityObservations)
        );
    }

    #[test]
    fn report_source_and_authority_tampering_breaks_integrity() {
        let sources = fixture_sources(CurrentRealityWorktreeObservation::clean());
        let report = current_reality_capability_matrix_report_from_sources(&sources)
            .expect("typed current-reality report");

        let mut source_tampered = report.clone();
        source_tampered.source_bindings[0].sha256 = "0".repeat(64);
        assert!(!source_tampered.has_current_reality_integrity(&sources));

        let mut authority_tampered = report.clone();
        authority_tampered.production_authority_granted = true;
        assert!(!authority_tampered.has_current_reality_integrity(&sources));

        let mut effect_tampered = report;
        effect_tampered.side_effects.channel_send_performed = true;
        assert!(!effect_tampered.has_current_reality_integrity(&sources));
    }

    #[test]
    fn typed_component_inventory_replaces_filesystem_presence_probes() {
        let inventory = CurrentRealityTypedComponentInventory::compiled();
        assert!(inventory.has_integrity());
        assert_eq!(inventory.components.len(), 5);
        assert!(inventory.components.iter().all(|entry| {
            entry.compiled && entry.rust_type.contains("::") && !entry.rust_type.contains('/')
        }));
    }
}

use std::any::type_name;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::path::Path;

use hepta_core::CURRENT_REALITY_CAPABILITY_CATALOG_ID_SHA256;
use hepta_core::CURRENT_REALITY_CAPABILITY_CATALOG_SCHEMA_VERSION;
use hepta_core::CURRENT_REALITY_CAPABILITY_IDS;
use hepta_core::CurrentRealityCapabilityDescriptor;
use hepta_core::CurrentRealityCapabilityLayer;
use hepta_core::CurrentRealityCapabilitySource;
use hepta_core::current_reality_capability_catalog;
use serde::Serialize;
use serde_json::Value;
use sha2::Digest;
use sha2::Sha256;
use thiserror::Error;

#[cfg(test)]
use hepta_core::CURRENT_REALITY_CATALOG_INVARIANT_ID;
#[cfg(test)]
use hepta_core::CURRENT_REALITY_WORK_GRAPH_DURABLE_IDENTITY_ALIAS;
#[cfg(test)]
use hepta_core::CURRENT_REALITY_WORK_GRAPH_RECEIPT_TAIL_ALIAS;

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

const REPORT_BACKED_CAPABILITY_COUNT: usize = 43;
const CATALOG_INVARIANT_CAPABILITY_COUNT: usize = 61;
const CURRENT_COMPACT_SUMMARY_REPORT_ID: &str = "hepta-systems-current-compact-capability-summary";

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

    fn from_dirty_worktree_observation(
        observation: &crate::DirtyWorktreeObservation,
    ) -> Result<Self, CurrentRealityCapabilityMatrixError> {
        let untracked = observation
            .entries
            .iter()
            .filter(|entry| entry.index_status == '?' && entry.worktree_status == '?')
            .count();
        Self::from_counts(
            observation.entries.len(),
            observation.entries.len().saturating_sub(untracked),
            untracked,
        )
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CurrentRealityCapabilitySourceKind {
    ReportBacked,
    CatalogInvariant,
}

impl CurrentRealityCapabilityStatus {
    const fn is_ready(self) -> bool {
        matches!(self, Self::Ready | Self::ReadyBlocked)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CurrentRealityCapabilitySourceObservation {
    pub capability_id: String,
    pub source_kind: CurrentRealityCapabilitySourceKind,
    pub status: CurrentRealityCapabilityStatus,
    pub source_report_id: String,
    pub current_fact: String,
    source_payload: Vec<u8>,
    source_payload_sha256: String,
    source_boolean_schema_sha256: String,
}

impl CurrentRealityCapabilitySourceObservation {
    pub fn new(
        capability_id: impl Into<String>,
        source_kind: CurrentRealityCapabilitySourceKind,
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
        let source_boolean_schema_sha256 = boolean_field_schema_sha256(&source_payload)
            .ok_or(CurrentRealityCapabilityMatrixError::InvalidCapabilityObservations)?;
        Ok(Self {
            capability_id,
            source_kind,
            status,
            source_report_id,
            current_fact,
            source_payload,
            source_payload_sha256,
            source_boolean_schema_sha256,
        })
    }

    fn has_integrity(&self) -> bool {
        !self.capability_id.is_empty()
            && !self.source_report_id.is_empty()
            && !self.source_report_id.contains('/')
            && !self.current_fact.is_empty()
            && !self.source_payload.is_empty()
            && self.source_payload_sha256 == sha256_hex(&self.source_payload)
            && boolean_field_schema_sha256(&self.source_payload)
                .is_some_and(|digest| digest == self.source_boolean_schema_sha256)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CurrentRealityCapabilityMatrixSources {
    plugin_manifest: CurrentRealityPluginManifestObservation,
    worktree: CurrentRealityWorktreeObservation,
    typed_components: CurrentRealityTypedComponentInventory,
    work_graph_successors: CurrentRealityWorkGraphSuccessorObservation,
    capability_observations: Vec<CurrentRealityCapabilitySourceObservation>,
}

impl CurrentRealityCapabilityMatrixSources {
    fn new(
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

    pub fn from_repository_inputs(
        repo_root: &Path,
        dirty_worktree: &crate::DirtyWorktreeObservation,
    ) -> Result<Self, CurrentRealityCapabilityMatrixError> {
        let manifest_path = repo_root.join("plugins/hepta-system/.codex-plugin/plugin.json");
        let manifest_bytes = std::fs::read(&manifest_path).map_err(|error| {
            CurrentRealityCapabilityMatrixError::MalformedPluginManifest(format!(
                "cannot read {}: {error}",
                manifest_path.display()
            ))
        })?;
        let plugin_reports =
            crate::plugin_compat_report::build_plugin_compat_reports(repo_root, &manifest_bytes)
                .map_err(|_| CurrentRealityCapabilityMatrixError::InvalidPluginFixtureShape)?;
        let plugin_manifest = CurrentRealityPluginManifestObservation::from_manifest_bytes(
            Some(&manifest_bytes),
            CurrentRealityPluginAssetObservation::new(true, true, true, 1, 1, 1),
        )?;

        let worktree =
            CurrentRealityWorktreeObservation::from_dirty_worktree_observation(dirty_worktree)?;
        let controlled_worktree = crate::ControlledLiveWorktreeObservation {
            status_entry_count: worktree.git_status_entry_count,
            untracked_count: worktree.git_untracked_count,
            tracked_change_count: worktree.git_tracked_change_count,
        };
        let work_graph_successors =
            CurrentRealityWorkGraphSuccessorObservation::from_current_typed_reports();
        let capability_observations = build_capability_observations(
            dirty_worktree,
            &controlled_worktree,
            &plugin_reports,
            &work_graph_successors,
            worktree.state(),
        )?;

        Ok(Self::new(
            plugin_manifest,
            worktree,
            CurrentRealityTypedComponentInventory::compiled(),
            work_graph_successors,
            capability_observations,
        ))
    }
}

#[derive(Serialize)]
struct CatalogInvariantSource<'a> {
    invariant_id: &'static str,
    capability_id: &'a str,
    legacy_ready: bool,
    legacy_live_enabled: bool,
}

#[derive(Serialize)]
struct CurrentCompactSummarySource {
    schema_version: &'static str,
    source_report_count: usize,
    ready_or_ready_blocked_count: usize,
    blocked_count: usize,
    execution_enabled_count: usize,
    public_ga_enabled_count: usize,
    source_report_sha256: Vec<String>,
}

fn build_capability_observations(
    dirty_worktree: &crate::DirtyWorktreeObservation,
    controlled_worktree: &crate::ControlledLiveWorktreeObservation,
    plugin_reports: &crate::plugin_compat_report::PluginCompatReportSet,
    work_graph_successors: &CurrentRealityWorkGraphSuccessorObservation,
    worktree_state: CurrentRealityWorktreeState,
) -> Result<Vec<CurrentRealityCapabilitySourceObservation>, CurrentRealityCapabilityMatrixError> {
    let catalog = current_reality_capability_catalog();
    let mut report_payloads = BTreeMap::<String, Value>::new();

    for descriptor in &catalog {
        let report_id = match descriptor.source {
            CurrentRealityCapabilitySource::TypedReport { report_id }
                if report_id != CURRENT_COMPACT_SUMMARY_REPORT_ID =>
            {
                report_id
            }
            CurrentRealityCapabilitySource::CompatibilityAlias {
                successor_report_id,
            } => successor_report_id,
            _ => continue,
        };
        let payload = dynamic_report_payload(
            report_id,
            dirty_worktree,
            controlled_worktree,
            plugin_reports,
            work_graph_successors,
        )?;
        if report_payloads
            .insert(report_id.to_string(), payload)
            .is_some()
        {
            return Err(CurrentRealityCapabilityMatrixError::InvalidCapabilityObservations);
        }
    }
    if report_payloads.len() != REPORT_BACKED_CAPABILITY_COUNT - 1 {
        return Err(CurrentRealityCapabilityMatrixError::InvalidCapabilityObservations);
    }

    let ready_or_ready_blocked_count = catalog
        .iter()
        .filter(|descriptor| legacy_capability_status(descriptor.id).is_ready())
        .count()
        .saturating_sub(1);
    let source_report_sha256 = report_payloads
        .values()
        .map(|value| {
            serialize_source(value, "compact summary source")
                .map(|bytes| sha256_hex(bytes.as_slice()))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let compact_summary = CurrentCompactSummarySource {
        schema_version: "current_compact_capability_summary_typed_v1",
        source_report_count: report_payloads.len(),
        ready_or_ready_blocked_count,
        blocked_count: 14,
        execution_enabled_count: 0,
        public_ga_enabled_count: 0,
        source_report_sha256,
    };
    report_payloads.insert(
        CURRENT_COMPACT_SUMMARY_REPORT_ID.to_string(),
        serde_json::to_value(compact_summary).map_err(|error| {
            CurrentRealityCapabilityMatrixError::SourceSerialization(format!(
                "compact summary: {error}"
            ))
        })?,
    );

    catalog
        .iter()
        .map(|descriptor| match descriptor.source {
            CurrentRealityCapabilitySource::CatalogInvariant { invariant_id } => {
                let payload = CatalogInvariantSource {
                    invariant_id,
                    capability_id: descriptor.id,
                    legacy_ready: true,
                    legacy_live_enabled: false,
                };
                CurrentRealityCapabilitySourceObservation::new(
                    descriptor.id,
                    CurrentRealityCapabilitySourceKind::CatalogInvariant,
                    CurrentRealityCapabilityStatus::Ready,
                    invariant_id,
                    format!(
                        "legacy catalog invariant {} remains ready and non-live",
                        descriptor.id
                    ),
                    serialize_source(&payload, "catalog invariant")?,
                )
            }
            CurrentRealityCapabilitySource::TypedReport { report_id } => report_backed_observation(
                descriptor,
                report_id,
                report_payloads.get(report_id),
                worktree_state,
            ),
            CurrentRealityCapabilitySource::CompatibilityAlias {
                successor_report_id,
            } => report_backed_observation(
                descriptor,
                successor_report_id,
                report_payloads.get(successor_report_id),
                worktree_state,
            ),
        })
        .collect()
}

fn dynamic_report_payload(
    report_id: &str,
    dirty_worktree: &crate::DirtyWorktreeObservation,
    controlled_worktree: &crate::ControlledLiveWorktreeObservation,
    plugin_reports: &crate::plugin_compat_report::PluginCompatReportSet,
    work_graph_successors: &CurrentRealityWorkGraphSuccessorObservation,
) -> Result<Value, CurrentRealityCapabilityMatrixError> {
    let value = match report_id {
        "hepta-systems-plugin-contribution-point-abi"
        | "hepta-systems-plugin-contribution-point-loader-binding"
        | "hepta-systems-plugin-tool-contribution-inventory-preview"
        | "hepta-systems-plugin-lifecycle-state-machine" => serde_json::to_value(
            plugin_reports
                .report(report_id)
                .ok_or(CurrentRealityCapabilityMatrixError::InvalidPluginFixtureShape)?,
        ),
        "hepta-systems-tool-registry-invocation-source-of-truth" => serde_json::to_value(
            codex_tools::hepta_system_tool_registry_invocation_source_of_truth_plan(),
        ),
        "hepta-systems-tool-registry-read-only-dispatch-preflight" => serde_json::to_value(
            codex_tools::hepta_system_tool_registry_read_only_dispatch_preflight_plan(),
        ),
        "hepta-systems-work-graph-current-state-inventory" => {
            serde_json::to_value(&work_graph_successors.current_state_inventory)
        }
        "hepta-systems-work-graph-persistence-acceptance-effect-application-denial-receipt-retention-expiry-readback-acknowledgement-terminal-decision-non-promotion-receipt-preview" => {
            serde_json::to_value(&work_graph_successors.terminal_non_promotion_receipt)
        }
        "hepta-systems-workflow-durable-store-adapter" => {
            return crate::typed_compat_report(report_id).map_err(|error| {
                CurrentRealityCapabilityMatrixError::SourceSerialization(format!(
                    "{report_id}: {error}"
                ))
            });
        }
        "hepta-systems-hepta-system-status-read-only-e2e" => {
            serde_json::to_value(crate::hepta_system_status_read_only_e2e_report())
        }
        "hepta-systems-hepta-system-status-internal-read-only-invocation" => serde_json::to_value(
            crate::hepta_system_status_internal_read_only_invocation_report(),
        ),
        "hepta-systems-hepta-system-status-operator-approval-protocol" => serde_json::to_value(
            crate::hepta_system_status_operator_approval_protocol_report(),
        ),
        id if id.starts_with("hepta-systems-dirty-worktree-") => {
            return crate::dirty_worktree_typed_compat_report(id, dirty_worktree).map_err(|error| {
                CurrentRealityCapabilityMatrixError::SourceSerialization(format!(
                    "{id}: {error}"
                ))
            });
        }
        id if id.starts_with("hepta-systems-controlled-") => {
            return crate::controlled_live_typed_compat_report(id, controlled_worktree).map_err(
                |error| {
                    CurrentRealityCapabilityMatrixError::SourceSerialization(format!(
                        "{id}: {error}"
                    ))
                },
            );
        }
        _ => return Err(CurrentRealityCapabilityMatrixError::InvalidCapabilityObservations),
    }
    .map_err(|error| {
        CurrentRealityCapabilityMatrixError::SourceSerialization(format!(
            "{report_id}: {error}"
        ))
    })?;
    Ok(value)
}

fn report_backed_observation(
    descriptor: &CurrentRealityCapabilityDescriptor,
    report_id: &'static str,
    payload: Option<&Value>,
    worktree_state: CurrentRealityWorktreeState,
) -> Result<CurrentRealityCapabilitySourceObservation, CurrentRealityCapabilityMatrixError> {
    let payload =
        payload.ok_or(CurrentRealityCapabilityMatrixError::InvalidCapabilityObservations)?;
    validate_dynamic_source_payload(report_id, payload)?;
    CurrentRealityCapabilitySourceObservation::new(
        descriptor.id,
        CurrentRealityCapabilitySourceKind::ReportBacked,
        legacy_capability_status(descriptor.id),
        report_id,
        legacy_current_fact(descriptor, worktree_state)?,
        serialize_source(payload, report_id)?,
    )
}

fn legacy_capability_status(id: &str) -> CurrentRealityCapabilityStatus {
    match id {
        "dirty_worktree_release_boundary_inventory"
        | "dirty_worktree_release_boundary_grouping_freeze_plan"
        | "dirty_worktree_release_boundary_grouping_freeze_operator_readback"
        | "dirty_worktree_release_boundary_actionable_clean_worktree_strategy"
        | "dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet"
        | "dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_non_send_readback"
        | "dirty_worktree_release_boundary_clean_worktree_strategy_operator_packet_git_mutation_boundary_readback"
        | "dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist"
        | "dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_checklist_packet_readback"
        | "dirty_worktree_release_boundary_clean_worktree_strategy_operator_decision_recording_boundary_readback"
        | "dirty_worktree_release_boundary_clean_worktree_strategy_operator_approval_acceptance_boundary_readback"
        | "dirty_worktree_release_boundary_clean_worktree_strategy_operator_evidence_recording_boundary_readback"
        | "dirty_worktree_release_boundary_release_risk_snapshot"
        | "dirty_worktree_release_boundary_test_only_clean_worktree_strategy_rehearsal" => {
            CurrentRealityCapabilityStatus::Blocked
        }
        "hepta_system_status_operator_approval_protocol"
        | "controlled_canary_readiness_plan"
        | "controlled_live_readiness_audit"
        | "controlled_live_readiness_denial_readback_index"
        | "controlled_live_operator_packet_preview"
        | "controlled_live_operator_packet_non_send_readback"
        | "controlled_live_required_evidence_collection_plan"
        | "controlled_live_required_evidence_readback_index"
        | "controlled_live_required_evidence_gap_summary"
        | "controlled_live_required_evidence_gap_diff_view"
        | "controlled_live_required_evidence_gap_operator_readback"
        | "controlled_live_required_evidence_gap_operator_packet_attachment"
        | "controlled_live_required_evidence_gap_operator_packet_attachment_non_send_readback"
        | "controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary_readback"
        | "controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary_readback"
        | "controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary_readback"
        | "controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary_readback" => {
            CurrentRealityCapabilityStatus::ReadyBlocked
        }
        _ => CurrentRealityCapabilityStatus::Ready,
    }
}

fn legacy_current_fact(
    descriptor: &CurrentRealityCapabilityDescriptor,
    worktree_state: CurrentRealityWorktreeState,
) -> Result<String, CurrentRealityCapabilityMatrixError> {
    if descriptor.id == "controlled_live_readiness_audit"
        && worktree_state == CurrentRealityWorktreeState::Dirty
    {
        return Ok("controlled-live audit is ready-blocked with a dirty worktree and seven explicit approval/evidence blockers".to_string());
    }
    descriptor
        .legacy_current_fact
        .map(ToOwned::to_owned)
        .ok_or(CurrentRealityCapabilityMatrixError::InvalidCapabilityObservations)
}

fn validate_dynamic_source_payload(
    report_id: &str,
    payload: &Value,
) -> Result<(), CurrentRealityCapabilityMatrixError> {
    let object = payload
        .as_object()
        .ok_or(CurrentRealityCapabilityMatrixError::InvalidCapabilityObservations)?;
    if report_id.starts_with("typed-current-reality-") {
        return Err(CurrentRealityCapabilityMatrixError::InvalidCapabilityObservations);
    }
    if report_id == CURRENT_COMPACT_SUMMARY_REPORT_ID {
        let valid = object.get("schema_version").and_then(Value::as_str)
            == Some("current_compact_capability_summary_typed_v1")
            && object.get("source_report_count").and_then(Value::as_u64) == Some(42)
            && object
                .get("ready_or_ready_blocked_count")
                .and_then(Value::as_u64)
                == Some(89)
            && object.get("blocked_count").and_then(Value::as_u64) == Some(14)
            && object
                .get("execution_enabled_count")
                .and_then(Value::as_u64)
                == Some(0)
            && object
                .get("public_ga_enabled_count")
                .and_then(Value::as_u64)
                == Some(0)
            && object
                .get("source_report_sha256")
                .and_then(Value::as_array)
                .is_some_and(|digests| {
                    digests.len() == 42
                        && digests
                            .iter()
                            .all(|digest| digest.as_str().is_some_and(is_sha256))
                });
        let (_, future_effect_boundary_open) = effect_boundary_summary(payload);
        return (valid && !future_effect_boundary_open)
            .then_some(())
            .ok_or(CurrentRealityCapabilityMatrixError::InvalidCapabilityObservations);
    }
    let positive_readiness = object
        .get("status")
        .and_then(Value::as_str)
        .is_some_and(|status| matches!(status, "ready" | "ready_blocked" | "pass" | "blocked"))
        || has_positive_readiness_field(payload);
    let side_effects_closed = source_effect_boundary_is_closed(report_id, payload);
    let authority_closed = named_boolean_fields_are_false(
        payload,
        &[
            "production_authority_granted",
            "write_authority_granted",
            "approval_authority_granted",
            "mutation_authority_granted",
            "evidence_authority_granted",
            "send_authority_granted",
            "ready_for_live_execution",
            "live_execution_allowed",
            "live_execution_started",
            "mutation_enabled",
        ],
    );
    if positive_readiness && side_effects_closed && authority_closed {
        Ok(())
    } else {
        Err(CurrentRealityCapabilityMatrixError::InvalidCapabilityObservations)
    }
}

fn source_effect_boundary_is_closed(report_id: &str, payload: &Value) -> bool {
    let Some(object) = payload.as_object() else {
        return false;
    };
    let source_specific_boundary = match report_id {
        "hepta-systems-tool-registry-invocation-source-of-truth" => {
            Some(tool_registry_invocation_boundary_is_closed(object))
        }
        "hepta-systems-tool-registry-read-only-dispatch-preflight" => {
            Some(tool_registry_dispatch_boundary_is_closed(object))
        }
        _ => None,
    };
    if let Some(source_specific_boundary) = source_specific_boundary {
        let (_, future_effect_boundary_open) = effect_boundary_summary(payload);
        return source_specific_boundary && !future_effect_boundary_open;
    }
    let actual_schema = object
        .get("side_effects")
        .and_then(closed_side_effect_schema_sha256);
    let expected_schema = expected_side_effect_schema_sha256(report_id);
    let explicit_boundary = actual_schema
        .as_deref()
        .zip(expected_schema)
        .is_some_and(|(actual, expected)| actual == expected);
    let (boundary_field_count, boundary_open) = effect_boundary_summary(payload);
    explicit_boundary && boundary_field_count > 0 && !boundary_open
}

fn closed_side_effect_schema_sha256(value: &Value) -> Option<String> {
    let object = value.as_object()?;
    if object.is_empty()
        || !object
            .values()
            .all(|value| matches!(value, Value::Bool(false)))
    {
        return None;
    }
    let mut fields = object.keys().map(String::as_str).collect::<Vec<_>>();
    fields.sort_unstable();
    serde_json::to_vec(&fields).ok().map(|mut bytes| {
        bytes.push(b'\n');
        sha256_hex(&bytes)
    })
}

fn expected_side_effect_schema_sha256(report_id: &str) -> Option<&'static str> {
    match report_id {
        "hepta-systems-controlled-canary-readiness-plan" => {
            Some("33ffb9d8bce8eafa31262d11d14c36c5c67a243a1f349a8ce92e3189459ddb3c")
        }
        "hepta-systems-controlled-live-operator-packet-non-send-readback" => {
            Some("b35a2479aa4be8d192319440af925b9a78fa63a9f2ffaa43d33d143b8ddc80f7")
        }
        "hepta-systems-controlled-live-operator-packet-preview" => {
            Some("8222634277baa8cae0a8204f7f24b07312b266416c72a214daa570b8bc74660d")
        }
        "hepta-systems-controlled-live-readiness-audit" => {
            Some("79bee53e9bfc6cf3a3f98cecbd9af2c569a09af90b34af7802968013c0c24539")
        }
        "hepta-systems-controlled-live-readiness-denial-readback-index" => {
            Some("ded46cd65bce1bb089659d9c8435c1cf3740a0d1de90ea498137ce4477b817d6")
        }
        "hepta-systems-controlled-live-required-evidence-collection-plan" => {
            Some("d057d49677be7f45936ac97e7e45eb7143cab95107e3de21b0a1ffd80273e59a")
        }
        "hepta-systems-controlled-live-required-evidence-gap-diff-view"
        | "hepta-systems-controlled-live-required-evidence-gap-operator-readback"
        | "hepta-systems-controlled-live-required-evidence-gap-summary"
        | "hepta-systems-controlled-live-required-evidence-readback-index" => {
            Some("388319d299bb9f254af66a19e6b985cef219dc5c6f51b4bf2f32c871f5a75d3a")
        }
        "hepta-systems-controlled-live-required-evidence-gap-operator-packet-attachment" => {
            Some("aa130136a7fd038acfb1a19c467bd39bc23df6dfdd7753dd2b95205ae7781eeb")
        }
        "hepta-systems-controlled-live-required-evidence-gap-operator-packet-attachment-credential-boundary-readback" => {
            Some("959a7e0eb791b86e99ed0cc604cf1cfbb9f1335b4c2402e6f053edd56cd5bced")
        }
        "hepta-systems-controlled-live-required-evidence-gap-operator-packet-attachment-kill-switch-rehearsal-boundary-readback" => {
            Some("f6b6ac8a4573955aa7f2eecfe68fcc486e5c7b7ae186925b4233924cb890ec2a")
        }
        "hepta-systems-controlled-live-required-evidence-gap-operator-packet-attachment-non-send-readback"
        | "hepta-systems-controlled-live-required-evidence-gap-operator-packet-attachment-transport-boundary-readback" => {
            Some("c564a26c8e0b6e5eb74f2aa812e55b09f72dd9fc5236e42402d7c7eead0c88f0")
        }
        "hepta-systems-controlled-live-required-evidence-gap-operator-packet-attachment-rollback-rehearsal-boundary-readback" => {
            Some("f75ca9e1d6c903d2d124b28d1e730b42f0c043e34dfff257d819aee03ccab7e4")
        }
        "hepta-systems-dirty-worktree-release-boundary-actionable-clean-worktree-strategy" => {
            Some("8d6bfa242eb77ec2dded21547c3f512360a294583d2d92acf9b36ad74828557f")
        }
        "hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-approval-acceptance-boundary-readback" => {
            Some("51a2d93ee6c9460d2d06a71f9c6f3ae3c235cb54f56cd008077e6f39472eda93")
        }
        "hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-decision-checklist" => {
            Some("2251f27601371377acbfed4b3bee36e79f460b76cbbcb627898289d8c1dfb92c")
        }
        "hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-decision-checklist-packet-readback" => {
            Some("7ec539fada8c9dd592269f60232db1af140a8edc2a2f5a8db22d9022f3328cf1")
        }
        "hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-decision-recording-boundary-readback" => {
            Some("b85db7dce0d6c0affab3b88996a532f48f31c560ef68637e8022ead3ec40951f")
        }
        "hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-evidence-recording-boundary-readback" => {
            Some("1aceea59f1d79bdcdaba6961f58be6828b5c8c91aae6433cec85516d40f989ff")
        }
        "hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-packet" => {
            Some("762f7116e09be4acf36aa5dba7139ddb0a0771c5aeba43ceac8e454784a54b32")
        }
        "hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-packet-git-mutation-boundary-readback" => {
            Some("497b29671d768324abf81759e03a218f5554f835066d3d87dab66f182f3c9bf5")
        }
        "hepta-systems-dirty-worktree-release-boundary-clean-worktree-strategy-operator-packet-non-send-readback" => {
            Some("474379271f3c572285012ca3c0ad49774873e57414487d7eb9ccc08faf577b88")
        }
        "hepta-systems-dirty-worktree-release-boundary-grouping-freeze-operator-readback"
        | "hepta-systems-dirty-worktree-release-boundary-grouping-freeze-plan" => {
            Some("cf88e5307ae34efedf1678d25317dbf5f61a34ddde89ff2659a4ccbea70c4376")
        }
        "hepta-systems-dirty-worktree-release-boundary-inventory" => {
            Some("aba194f638362e8e28ae2a023598c6621e0bcfce0fe988c636962b8e66c7b294")
        }
        "hepta-systems-dirty-worktree-release-boundary-release-risk-snapshot" => {
            Some("8c7288581a0268ec0e7139446c5d0a44f992c5f72d8357205b706fdee16fbfa5")
        }
        "hepta-systems-dirty-worktree-release-boundary-test-only-clean-worktree-strategy-rehearsal" => {
            Some("dbf9b734d07e2020c093dcc1e6f8c1777a6d7ee531fa1d7a5466ae5a325187e9")
        }
        "hepta-systems-hepta-system-status-read-only-e2e" => {
            Some("04f1f4d6914a386e2bf696836c6799ee123e33dea9e4c584907541dd4c3af285")
        }
        "hepta-systems-hepta-system-status-internal-read-only-invocation" => {
            Some("81c57b92228cd0bd7c41e1fb98f9c9802af464544c859dbf3b9e80101ce38f4d")
        }
        "hepta-systems-hepta-system-status-operator-approval-protocol" => {
            Some("3e221d50441c1876e0ce1bced57d757bde0ab3b666aced45f57ec4d7ed3eb272")
        }
        "hepta-systems-plugin-contribution-point-abi"
        | "hepta-systems-plugin-contribution-point-loader-binding"
        | "hepta-systems-plugin-lifecycle-state-machine"
        | "hepta-systems-plugin-tool-contribution-inventory-preview" => {
            Some("c882e4a624ad465a212a469d9a79a96adad6043ef2915094e965ff521dba56fd")
        }
        "hepta-systems-work-graph-current-state-inventory" => {
            Some("0064ffb7ce47a1fc58e9f742a99f9080d706ef108c61a40b87ec46cf868cc926")
        }
        "hepta-systems-work-graph-persistence-acceptance-effect-application-denial-receipt-retention-expiry-readback-acknowledgement-terminal-decision-non-promotion-receipt-preview" => {
            Some("337f84de22563cfb83e4c8886d98126509076aaa6a26688dc18c4d5806b6773f")
        }
        "hepta-systems-workflow-durable-store-adapter" => {
            Some("fced1e71328142a33162b1fad4b1d62232e2e0c05bd7fb15dd93f09cdfb628db")
        }
        _ => None,
    }
}

fn tool_registry_invocation_boundary_is_closed(object: &serde_json::Map<String, Value>) -> bool {
    object_boolean_fields_match(
        object,
        &[
            ("side_effect_free", true),
            ("router_registration_lookup_enabled", false),
            ("registry_source_of_truth_enabled", false),
            ("tool_registration_enabled", false),
            ("tool_invocation_enabled", false),
            ("ledger_written", false),
            ("approval_requested", false),
            ("live_mutation_ready", false),
        ],
    ) && entries_boolean_fields_match(
        object,
        &[
            ("side_effect_free", true),
            ("router_registration_lookup_enabled", false),
            ("registry_source_of_truth_enabled", false),
            ("tool_registration_enabled", false),
            ("tool_invocation_enabled", false),
            ("ledger_write_enabled", false),
            ("approval_request_enabled", false),
        ],
    )
}

fn tool_registry_dispatch_boundary_is_closed(object: &serde_json::Map<String, Value>) -> bool {
    object_boolean_fields_match(
        object,
        &[
            ("side_effect_free", true),
            ("read_only_dispatch_preflight_allowed", true),
            ("registry_dispatch_switch_enabled", false),
            ("router_registration_lookup_enabled", false),
            ("registry_lookup_executed", false),
            ("registry_source_of_truth_enabled", false),
            ("tool_registration_enabled", false),
            ("tool_invocation_enabled", false),
            ("ledger_written", false),
            ("approval_requested", false),
            ("result_receipt_written", false),
            ("live_mutation_ready", false),
        ],
    ) && entries_boolean_fields_match(
        object,
        &[
            ("side_effect_free", true),
            ("registry_dispatch_switch_enabled", false),
            ("router_registration_lookup_enabled", false),
            ("registry_lookup_executed", false),
            ("registry_source_of_truth_enabled", false),
            ("tool_registration_enabled", false),
            ("tool_invocation_enabled", false),
            ("ledger_write_enabled", false),
            ("approval_request_enabled", false),
            ("result_receipt_write_enabled", false),
        ],
    )
}

fn entries_boolean_fields_match(
    object: &serde_json::Map<String, Value>,
    expected: &[(&str, bool)],
) -> bool {
    object
        .get("entries")
        .and_then(Value::as_array)
        .is_some_and(|entries| {
            !entries.is_empty()
                && entries.iter().all(|entry| {
                    entry
                        .as_object()
                        .is_some_and(|entry| object_boolean_fields_match(entry, expected))
                })
        })
}

fn object_boolean_fields_match(
    object: &serde_json::Map<String, Value>,
    expected: &[(&str, bool)],
) -> bool {
    expected
        .iter()
        .all(|(name, expected)| object.get(*name) == Some(&Value::Bool(*expected)))
}

fn effect_boundary_summary(value: &Value) -> (usize, bool) {
    fn visit(value: &Value, count: &mut usize, open: &mut bool) {
        match value {
            Value::Array(values) => {
                for value in values {
                    visit(value, count, open);
                }
            }
            Value::Object(values) => {
                for (name, value) in values {
                    if let Value::Bool(actual) = value
                        && let Some(closed_value) = effect_boundary_closed_value(name)
                    {
                        *count += 1;
                        *open |= *actual != closed_value;
                    }
                    visit(value, count, open);
                }
            }
            _ => {}
        }
    }

    let mut count = 0;
    let mut open = false;
    visit(value, &mut count, &mut open);
    (count, open)
}

fn effect_boundary_closed_value(name: &str) -> Option<bool> {
    if name == "read_only_dispatch_preflight_allowed" {
        return Some(true);
    }
    if matches!(
        name,
        "side_effect_free"
            | "all_live_paths_blocked"
            | "all_runtime_execution_disabled"
            | "mutation_free"
    ) {
        return Some(true);
    }
    let sensitive = name
        .split(|character: char| !character.is_ascii_alphanumeric())
        .any(|token| {
            matches!(
                token,
                "activation"
                    | "approval"
                    | "authority"
                    | "credential"
                    | "cutover"
                    | "execution"
                    | "external"
                    | "install"
                    | "invocation"
                    | "ledger"
                    | "live"
                    | "mutation"
                    | "network"
                    | "package"
                    | "persistence"
                    | "production"
                    | "promotion"
                    | "publication"
                    | "release"
                    | "routing"
                    | "send"
                    | "traffic"
                    | "write"
            )
        });
    let closed_polarity = name.starts_with("blocks_")
        || [
            "_blocked",
            "_bound",
            "_closed",
            "_denied",
            "_disabled",
            "_free",
            "_guarded",
        ]
        .iter()
        .any(|suffix| name.ends_with(suffix))
        || name.starts_with("non_")
        || name.ends_with("_boundary_confirmed");
    let neutral_boundary_fact = ["_missing", "_present", "_required", "_resolved"]
        .iter()
        .any(|suffix| name.ends_with(suffix))
        || name == "blocks_cutover"
        || name.contains("_require_")
        || name.starts_with("required_for_")
        || (name.starts_with("source_") && (name.contains("_ready") || name.ends_with("_visible")))
        || name.ends_with("_audit_ready")
        || name.ends_with("_boundary_open")
        || name.ends_with("_inventory_ready")
        || name.ends_with("_preview_ready")
        || name.ends_with("_protocol_ready")
        || name.ends_with("_readback_ready")
        || name.ends_with("_readback_visible")
        || (name.starts_with("ready_for_") && name.ends_with("_preview"))
        || matches!(
            name,
            "all_dispatch_entries_keep_no_invocation_guard"
                | "all_forwarded_candidates_bound_to_invocation_source"
                | "all_invocation_sources_keep_approval_ledger_guard"
                | "internal_read_only_invocation_materialized"
                | "internal_read_only_invocation_ready"
                | "invocation_source_of_truth_plan_ready"
                | "invocation_source_ready"
                | "selected_for_internal_invocation"
        );
    if matches!(
        name,
        "dirty_worktree_release_boundary_open" | "source_dirty_worktree_release_boundary_open"
    ) {
        return None;
    }
    if sensitive && name.ends_with("_boundary_open") {
        return Some(false);
    }
    if neutral_boundary_fact {
        return None;
    }
    if sensitive && closed_polarity {
        return Some(true);
    }
    if sensitive {
        return Some(false);
    }
    if closed_polarity {
        return None;
    }
    if matches!(
        name,
        "credential_read"
            | "external_network_used"
            | "live_execution"
            | "mutation"
            | "production_authority"
            | "rollback_executed_side_effect"
            | "write_authority"
    ) || [
        "_acceptance",
        "_accepted",
        "_acquired",
        "_allowed",
        "_applied",
        "_attempted",
        "_created",
        "_deleted",
        "_enabled",
        "_executed",
        "_execution",
        "_exposed",
        "_granted",
        "_installed",
        "_invoked",
        "_loaded",
        "_mutated",
        "_performed",
        "_persisted",
        "_persistence",
        "_promoted",
        "_published",
        "_recorded",
        "_recording",
        "_request",
        "_requested",
        "_routed",
        "_sent",
        "_started",
        "_waived",
        "_write",
        "_written",
    ]
    .iter()
    .any(|suffix| name.ends_with(suffix))
    {
        Some(false)
    } else {
        None
    }
}

fn has_positive_readiness_field(value: &Value) -> bool {
    match value {
        Value::Array(values) => values.iter().any(has_positive_readiness_field),
        Value::Object(values) => values.iter().any(|(name, value)| {
            (name.ends_with("_ready") && value == &Value::Bool(true))
                || has_positive_readiness_field(value)
        }),
        _ => false,
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
    pub source_kind: CurrentRealityCapabilitySourceKind,
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
    pub id: String,
    pub schema_version: String,
    pub generation: String,
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
    pub report_backed_capability_count: usize,
    pub catalog_invariant_capability_count: usize,
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
            && self.local_capability_ready_count == 90
            && self.local_capability_blocked_count == 14
            && self.local_capability_ready_count + self.local_capability_blocked_count
                == self.local_capability_count
            && self.report_backed_capability_count == REPORT_BACKED_CAPABILITY_COUNT
            && self.catalog_invariant_capability_count == CATALOG_INVARIANT_CAPABILITY_COUNT
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
            && self.source_bindings.len() == REPORT_BACKED_CAPABILITY_COUNT + 5
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

    let worktree_bytes = serialize_source(&sources.worktree, "worktree observation")?;
    let capabilities = catalog
        .iter()
        .zip(&sources.capability_observations)
        .map(|(descriptor, observation)| capability_row(descriptor, observation))
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
    let source_bindings = source_bindings(sources, &worktree_bytes)?;

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
        report_backed_capability_count: capabilities
            .iter()
            .filter(|row| row.source_kind == CurrentRealityCapabilitySourceKind::ReportBacked)
            .count(),
        catalog_invariant_capability_count: capabilities
            .iter()
            .filter(|row| row.source_kind == CurrentRealityCapabilitySourceKind::CatalogInvariant)
            .count(),
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
    let catalog = current_reality_capability_catalog();
    if observations.len() != catalog.len()
        || !catalog
            .iter()
            .zip(observations)
            .all(|(descriptor, observation)| {
                observation_matches_descriptor(descriptor, observation)
            })
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

fn observation_matches_descriptor(
    descriptor: &CurrentRealityCapabilityDescriptor,
    observation: &CurrentRealityCapabilitySourceObservation,
) -> bool {
    if descriptor.id != observation.capability_id
        || observation.status != legacy_capability_status(descriptor.id)
        || !observation.has_integrity()
    {
        return false;
    }
    match descriptor.source {
        CurrentRealityCapabilitySource::CatalogInvariant { invariant_id } => {
            if observation.source_kind != CurrentRealityCapabilitySourceKind::CatalogInvariant
                || observation.source_report_id != invariant_id
            {
                return false;
            }
            serde_json::from_slice::<Value>(&observation.source_payload).is_ok_and(|payload| {
                payload.get("invariant_id").and_then(Value::as_str) == Some(invariant_id)
                    && payload.get("capability_id").and_then(Value::as_str) == Some(descriptor.id)
                    && payload.get("legacy_ready") == Some(&Value::Bool(true))
                    && payload.get("legacy_live_enabled") == Some(&Value::Bool(false))
            })
        }
        CurrentRealityCapabilitySource::TypedReport { report_id } => {
            report_observation_matches(report_id, observation)
        }
        CurrentRealityCapabilitySource::CompatibilityAlias {
            successor_report_id,
        } => report_observation_matches(successor_report_id, observation),
    }
}

fn report_observation_matches(
    report_id: &str,
    observation: &CurrentRealityCapabilitySourceObservation,
) -> bool {
    observation.source_kind == CurrentRealityCapabilitySourceKind::ReportBacked
        && observation.source_report_id == report_id
        && serde_json::from_slice::<Value>(&observation.source_payload)
            .is_ok_and(|payload| validate_dynamic_source_payload(report_id, &payload).is_ok())
}

fn capability_row(
    descriptor: &CurrentRealityCapabilityDescriptor,
    observation: &CurrentRealityCapabilitySourceObservation,
) -> CurrentRealityCapabilityRow {
    let compatibility_alias_successor = match descriptor.source {
        CurrentRealityCapabilitySource::CompatibilityAlias {
            successor_report_id,
        } => Some(successor_report_id),
        _ => None,
    };

    CurrentRealityCapabilityRow {
        id: descriptor.id,
        layer: descriptor.layer,
        source_kind: observation.source_kind,
        status: observation.status,
        ready: observation.status.is_ready(),
        live_enabled: false,
        source_report_id: observation.source_report_id.clone(),
        source_sha256: observation.source_payload_sha256.clone(),
        current_fact: observation.current_fact.clone(),
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
    source_kind: CurrentRealityCapabilitySourceKind,
    status: CurrentRealityCapabilityStatus,
    source_report_id: &'a str,
    current_fact: &'a str,
    source_payload_sha256: &'a str,
}

fn source_bindings(
    sources: &CurrentRealityCapabilityMatrixSources,
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
    let invariant_receipts = sources
        .capability_observations
        .iter()
        .filter(|observation| {
            observation.source_kind == CurrentRealityCapabilitySourceKind::CatalogInvariant
        })
        .map(|observation| CapabilityObservationReceipt {
            capability_id: &observation.capability_id,
            source_kind: observation.source_kind,
            status: observation.status,
            source_report_id: &observation.source_report_id,
            current_fact: &observation.current_fact,
            source_payload_sha256: &observation.source_payload_sha256,
        })
        .collect::<Vec<_>>();
    if invariant_receipts.len() != CATALOG_INVARIANT_CAPABILITY_COUNT {
        return Err(CurrentRealityCapabilityMatrixError::InvalidCapabilityObservations);
    }
    let invariant_bytes = serialize_source(&invariant_receipts, "catalog invariants")?;

    let mut bindings = vec![
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
            "catalog_invariants",
            "current_reality_catalog_invariants_v1",
            "explicit_legacy_non_live_invariants",
            &invariant_bytes,
        ),
    ];
    for observation in sources
        .capability_observations
        .iter()
        .filter(|observation| {
            observation.source_kind == CurrentRealityCapabilitySourceKind::ReportBacked
        })
    {
        let payload =
            serde_json::from_slice::<Value>(&observation.source_payload).map_err(|error| {
                CurrentRealityCapabilityMatrixError::SourceSerialization(format!(
                    "{}: {error}",
                    observation.source_report_id
                ))
            })?;
        let schema = payload
            .get("schema_version")
            .and_then(Value::as_str)
            .unwrap_or("typed_report_source_v1");
        bindings.push(source_binding(
            &observation.source_report_id,
            schema,
            "typed_report_constructor",
            &observation.source_payload,
        ));
    }
    if bindings.len() != REPORT_BACKED_CAPABILITY_COUNT + 5
        || bindings
            .iter()
            .map(|binding| binding.id.as_str())
            .collect::<BTreeSet<_>>()
            .len()
            != bindings.len()
    {
        return Err(CurrentRealityCapabilityMatrixError::InvalidCapabilityObservations);
    }
    Ok(bindings)
}

fn source_binding(
    id: impl Into<String>,
    schema_version: impl Into<String>,
    generation: impl Into<String>,
    bytes: &[u8],
) -> CurrentRealitySourceBinding {
    CurrentRealitySourceBinding {
        id: id.into(),
        schema_version: schema_version.into(),
        generation: generation.into(),
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

fn boolean_field_schema_sha256(bytes: &[u8]) -> Option<String> {
    fn visit(value: &Value, path: &mut Vec<String>, fields: &mut Vec<Vec<String>>) {
        match value {
            Value::Bool(_) => fields.push(path.clone()),
            Value::Array(values) => {
                for (index, value) in values.iter().enumerate() {
                    path.push(index.to_string());
                    visit(value, path, fields);
                    path.pop();
                }
            }
            Value::Object(values) => {
                for (name, value) in values {
                    path.push(name.clone());
                    visit(value, path, fields);
                    path.pop();
                }
            }
            _ => {}
        }
    }

    let value = serde_json::from_slice::<Value>(bytes).ok()?;
    let mut fields = Vec::new();
    visit(&value, &mut Vec::new(), &mut fields);
    fields.sort();
    serde_json::to_vec(&fields)
        .ok()
        .map(|serialized| sha256_hex(&serialized))
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
        _ => false,
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
    use std::path::PathBuf;

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

    fn repo_root() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .canonicalize()
            .expect("canonical repository root")
    }

    fn fixture_sources(porcelain: &[u8]) -> CurrentRealityCapabilityMatrixSources {
        let dirty = crate::DirtyWorktreeObservation::from_porcelain_v1_z(porcelain)
            .expect("typed dirty-worktree observation");
        CurrentRealityCapabilityMatrixSources::from_repository_inputs(&repo_root(), &dirty)
            .expect("repository-backed current-reality sources")
    }

    fn mutate_report_source(
        sources: &mut CurrentRealityCapabilityMatrixSources,
        report_id: &str,
        mutate: impl FnOnce(&mut Value),
    ) {
        let source = sources
            .capability_observations
            .iter_mut()
            .find(|source| {
                source.source_kind == CurrentRealityCapabilitySourceKind::ReportBacked
                    && source.source_report_id == report_id
            })
            .expect("report-backed source observation");
        let mut payload =
            serde_json::from_slice::<Value>(&source.source_payload).expect("typed source payload");
        mutate(&mut payload);
        source.source_payload = serde_json::to_vec(&payload).expect("serialize source payload");
        source.source_payload_sha256 = sha256_hex(&source.source_payload);
    }

    fn mutate_report_source_with_schema_rebind(
        sources: &mut CurrentRealityCapabilityMatrixSources,
        report_id: &str,
        mutate: impl FnOnce(&mut Value),
    ) {
        mutate_report_source(sources, report_id, mutate);
        let source = sources
            .capability_observations
            .iter_mut()
            .find(|source| {
                source.source_kind == CurrentRealityCapabilitySourceKind::ReportBacked
                    && source.source_report_id == report_id
            })
            .expect("report-backed source observation");
        source.source_boolean_schema_sha256 =
            boolean_field_schema_sha256(&source.source_payload).expect("boolean source schema");
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
        let clean_sources = fixture_sources(b"");
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

        let dirty_sources = fixture_sources(b" M tracked.rs\0?? untracked-a\0?? untracked-b\0");
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
        let sources = fixture_sources(b"");
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
    fn report_preserves_the_legacy_43_report_and_61_invariant_topology() {
        let sources = fixture_sources(b"");
        let report = current_reality_capability_matrix_report_from_sources(&sources)
            .expect("typed current-reality report");
        let report_backed = sources
            .capability_observations
            .iter()
            .filter(|source| source.source_kind == CurrentRealityCapabilitySourceKind::ReportBacked)
            .collect::<Vec<_>>();
        let invariants = sources
            .capability_observations
            .iter()
            .filter(|source| {
                source.source_kind == CurrentRealityCapabilitySourceKind::CatalogInvariant
            })
            .collect::<Vec<_>>();

        assert_eq!(report_backed.len(), 43);
        assert_eq!(invariants.len(), 61);
        assert_eq!(report.report_backed_capability_count, 43);
        assert_eq!(report.catalog_invariant_capability_count, 61);
        assert_eq!(report.local_capability_ready_count, 90);
        assert_eq!(report.local_capability_blocked_count, 14);
        assert_eq!(report.source_bindings.len(), 48);
        assert!(report_backed.iter().all(|source| {
            !source
                .source_report_id
                .starts_with("typed-current-reality-")
                && serde_json::from_slice::<Value>(&source.source_payload)
                    .is_ok_and(|payload| payload.is_object())
        }));
        assert!(
            invariants
                .iter()
                .all(|source| { source.source_report_id == CURRENT_REALITY_CATALOG_INVARIANT_ID })
        );
        assert_eq!(
            report.capabilities.iter().filter(|row| !row.ready).count(),
            14
        );
    }

    #[test]
    fn report_payload_and_catalog_invariant_tampering_fail_closed_after_rehash() {
        let mut dynamic_tampered = fixture_sources(b"");
        let dynamic = dynamic_tampered
            .capability_observations
            .iter_mut()
            .find(|source| source.source_kind == CurrentRealityCapabilitySourceKind::ReportBacked)
            .expect("report-backed source");
        let mut payload =
            serde_json::from_slice::<Value>(&dynamic.source_payload).expect("typed source payload");
        payload.as_object_mut().expect("object payload").insert(
            "production_authority_granted".to_string(),
            Value::Bool(true),
        );
        dynamic.source_payload = serde_json::to_vec(&payload).expect("serialize tampered payload");
        dynamic.source_payload_sha256 = sha256_hex(&dynamic.source_payload);
        assert_eq!(
            current_reality_capability_matrix_report_from_sources(&dynamic_tampered),
            Err(CurrentRealityCapabilityMatrixError::InvalidCapabilityObservations)
        );

        let mut invariant_tampered = fixture_sources(b"");
        let invariant = invariant_tampered
            .capability_observations
            .iter_mut()
            .find(|source| {
                source.source_kind == CurrentRealityCapabilitySourceKind::CatalogInvariant
            })
            .expect("catalog invariant source");
        let mut payload =
            serde_json::from_slice::<Value>(&invariant.source_payload).expect("invariant payload");
        payload
            .as_object_mut()
            .expect("object payload")
            .insert("legacy_live_enabled".to_string(), Value::Bool(true));
        invariant.source_payload = serde_json::to_vec(&payload).expect("serialize invariant");
        invariant.source_payload_sha256 = sha256_hex(&invariant.source_payload);
        assert_eq!(
            current_reality_capability_matrix_report_from_sources(&invariant_tampered),
            Err(CurrentRealityCapabilityMatrixError::InvalidCapabilityObservations)
        );
    }

    #[test]
    fn source_effect_boundaries_reject_known_and_future_live_fields_after_rehash() {
        let invocation_report_id = "hepta-systems-tool-registry-invocation-source-of-truth";

        let mut top_level_live = fixture_sources(b"");
        mutate_report_source(&mut top_level_live, invocation_report_id, |payload| {
            payload
                .as_object_mut()
                .expect("invocation source object")
                .insert("tool_invocation_enabled".to_string(), Value::Bool(true));
        });
        assert_eq!(
            current_reality_capability_matrix_report_from_sources(&top_level_live),
            Err(CurrentRealityCapabilityMatrixError::InvalidCapabilityObservations)
        );

        let mut future_invocation_field = fixture_sources(b"");
        mutate_report_source_with_schema_rebind(
            &mut future_invocation_field,
            invocation_report_id,
            |payload| {
                payload
                    .as_object_mut()
                    .expect("invocation source object")
                    .insert("future_live_bridge_active".to_string(), Value::Bool(true));
            },
        );
        assert_eq!(
            current_reality_capability_matrix_report_from_sources(&future_invocation_field),
            Err(CurrentRealityCapabilityMatrixError::InvalidCapabilityObservations)
        );

        let mut future_compact_field = fixture_sources(b"");
        mutate_report_source_with_schema_rebind(
            &mut future_compact_field,
            CURRENT_COMPACT_SUMMARY_REPORT_ID,
            |payload| {
                payload
                    .as_object_mut()
                    .expect("compact source object")
                    .insert("future_live_bridge_active".to_string(), Value::Bool(true));
            },
        );
        assert_eq!(
            current_reality_capability_matrix_report_from_sources(&future_compact_field),
            Err(CurrentRealityCapabilityMatrixError::InvalidCapabilityObservations)
        );

        let mut nested_live = fixture_sources(b"");
        mutate_report_source(&mut nested_live, invocation_report_id, |payload| {
            payload
                .get_mut("entries")
                .and_then(Value::as_array_mut)
                .and_then(|entries| entries.first_mut())
                .and_then(Value::as_object_mut)
                .expect("invocation source entry")
                .insert("ledger_write_enabled".to_string(), Value::Bool(true));
        });
        assert_eq!(
            current_reality_capability_matrix_report_from_sources(&nested_live),
            Err(CurrentRealityCapabilityMatrixError::InvalidCapabilityObservations)
        );

        let side_effect_report_id = fixture_sources(b"")
            .capability_observations
            .iter()
            .find_map(|source| {
                (source.source_kind == CurrentRealityCapabilitySourceKind::ReportBacked
                    && source.source_report_id != invocation_report_id
                    && source.source_report_id
                        != "hepta-systems-tool-registry-read-only-dispatch-preflight")
                    .then(|| {
                        serde_json::from_slice::<Value>(&source.source_payload)
                            .ok()
                            .filter(|payload| payload.get("side_effects").is_some())
                            .map(|_| source.source_report_id.clone())
                    })
                    .flatten()
            })
            .expect("report source with explicit side-effects boundary");

        let mut missing_boundary = fixture_sources(b"");
        mutate_report_source(&mut missing_boundary, &side_effect_report_id, |payload| {
            payload
                .as_object_mut()
                .expect("source object")
                .remove("side_effects");
        });
        assert_eq!(
            current_reality_capability_matrix_report_from_sources(&missing_boundary),
            Err(CurrentRealityCapabilityMatrixError::InvalidCapabilityObservations)
        );

        let mut deleted_boundary_field = fixture_sources(b"");
        mutate_report_source(
            &mut deleted_boundary_field,
            &side_effect_report_id,
            |payload| {
                let side_effects = payload
                    .get_mut("side_effects")
                    .and_then(Value::as_object_mut)
                    .expect("side-effects object");
                let field = side_effects
                    .keys()
                    .next()
                    .cloned()
                    .expect("side-effects field");
                side_effects.remove(&field);
            },
        );
        assert_eq!(
            current_reality_capability_matrix_report_from_sources(&deleted_boundary_field),
            Err(CurrentRealityCapabilityMatrixError::InvalidCapabilityObservations)
        );

        let mut non_boolean_boundary_field = fixture_sources(b"");
        mutate_report_source(
            &mut non_boolean_boundary_field,
            &side_effect_report_id,
            |payload| {
                let side_effects = payload
                    .get_mut("side_effects")
                    .and_then(Value::as_object_mut)
                    .expect("side-effects object");
                let field = side_effects
                    .keys()
                    .next()
                    .cloned()
                    .expect("side-effects field");
                side_effects.insert(field, Value::String("false".to_string()));
            },
        );
        assert_eq!(
            current_reality_capability_matrix_report_from_sources(&non_boolean_boundary_field),
            Err(CurrentRealityCapabilityMatrixError::InvalidCapabilityObservations)
        );

        let mut future_live_field = fixture_sources(b"");
        mutate_report_source(&mut future_live_field, &side_effect_report_id, |payload| {
            payload
                .as_object_mut()
                .expect("source object")
                .insert("future_live_bridge_active".to_string(), Value::Bool(true));
        });
        assert_eq!(
            current_reality_capability_matrix_report_from_sources(&future_live_field),
            Err(CurrentRealityCapabilityMatrixError::InvalidCapabilityObservations)
        );

        for (field, value) in [
            ("future_live_bridge_blocked", false),
            ("future_production_authority_closed", false),
            ("future_live_boundary_open", true),
        ] {
            let mut reversed_polarity = fixture_sources(b"");
            mutate_report_source(&mut reversed_polarity, &side_effect_report_id, |payload| {
                payload
                    .as_object_mut()
                    .expect("source object")
                    .insert(field.to_string(), Value::Bool(value));
            });
            assert_eq!(
                current_reality_capability_matrix_report_from_sources(&reversed_polarity),
                Err(CurrentRealityCapabilityMatrixError::InvalidCapabilityObservations),
                "reversed effect-boundary polarity must fail closed for {field}"
            );
        }
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
        let mut wrong_shape_sources = fixture_sources(b"");
        wrong_shape_sources.plugin_manifest = wrong_shape;
        assert_eq!(
            current_reality_capability_matrix_report_from_sources(&wrong_shape_sources),
            Err(CurrentRealityCapabilityMatrixError::InvalidPluginFixtureShape)
        );
    }

    #[test]
    fn altered_work_graph_successors_fail_closed_without_reviving_retired_modules() {
        let mut sources = fixture_sources(b"");
        sources.work_graph_successors.current_state_inventory.status = "tampered";
        assert_eq!(
            current_reality_capability_matrix_report_from_sources(&sources),
            Err(CurrentRealityCapabilityMatrixError::InvalidWorkGraphSuccessors)
        );

        let mut sources = fixture_sources(b"");
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
        let mut missing = fixture_sources(b"");
        missing.capability_observations.pop();
        assert_eq!(
            current_reality_capability_matrix_report_from_sources(&missing),
            Err(CurrentRealityCapabilityMatrixError::InvalidCapabilityObservations)
        );

        let mut duplicate = fixture_sources(b"");
        duplicate.capability_observations[1].capability_id =
            duplicate.capability_observations[0].capability_id.clone();
        assert_eq!(
            current_reality_capability_matrix_report_from_sources(&duplicate),
            Err(CurrentRealityCapabilityMatrixError::InvalidCapabilityObservations)
        );

        let mut tampered = fixture_sources(b"");
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
        let sources = fixture_sources(b"");
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

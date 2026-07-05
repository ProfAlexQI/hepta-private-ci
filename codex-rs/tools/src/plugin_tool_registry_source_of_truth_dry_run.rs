use crate::PluginToolContributionInventoryPreviewPlan;
use crate::ToolRegistryInventory;
use crate::ToolRegistryInventoryApprovalKind;
use crate::ToolRegistryInventorySideEffectLevel;
use crate::ToolRegistryInventorySource;
use crate::ToolRegistryInvocationGuardRoute;
use crate::hepta_system_plugin_tool_contribution_inventory_preview_plan;
use serde::Serialize;
use std::collections::BTreeSet;

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct PluginToolRegistrySourceOfTruthDryRunEntry {
    pub plugin_id: &'static str,
    pub contribution_kind: &'static str,
    pub candidate_tool_id: &'static str,
    pub planned_registry_source: ToolRegistryInventorySource,
    pub planned_registry_name: &'static str,
    pub owner: &'static str,
    pub has_input_schema: bool,
    pub has_output_schema: bool,
    pub side_effect_level: ToolRegistryInventorySideEffectLevel,
    pub approval_kind: ToolRegistryInventoryApprovalKind,
    pub auth_required: bool,
    pub timeout_ms: u64,
    pub ledger_required: bool,
    pub registry_entry_found: bool,
    pub duplicate_id: bool,
    pub guard_route: ToolRegistryInvocationGuardRoute,
    pub approval_required: bool,
    pub guard_blocked: bool,
    pub guard_blocked_reason: Option<&'static str>,
    pub source_of_truth_registration_enabled: bool,
    pub tool_invocation_enabled: bool,
    pub ledger_write_enabled: bool,
    pub approval_request_enabled: bool,
    pub dry_run_ready: bool,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct PluginToolRegistrySourceOfTruthDryRunPlan {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub plugin_id: &'static str,
    pub status: &'static str,
    pub source_preview_surface: &'static str,
    pub source_preview_ready: bool,
    pub planned_source_of_truth_surface: &'static str,
    pub hepta_system_manifest_present: bool,
    pub preview_candidate_count: usize,
    pub planned_registry_entry_count: usize,
    pub planned_mcp_entry_count: usize,
    pub planned_connector_entry_count: usize,
    pub duplicate_candidate_ids: Vec<String>,
    pub duplicate_registry_ids: Vec<String>,
    pub unbound_candidate_ids: Vec<String>,
    pub all_candidate_ids_unique: bool,
    pub all_preview_candidates_bound_to_registry: bool,
    pub all_candidates_have_schema: bool,
    pub all_candidates_have_risk_metadata: bool,
    pub all_candidates_require_ledger: bool,
    pub mutating_candidates_require_approval: bool,
    pub all_candidates_have_guard_route: bool,
    pub registry_invocation_guard_ready: bool,
    pub registry_source_of_truth_dry_run_ready: bool,
    pub registry_source_of_truth_enabled: bool,
    pub tool_registry_registration_enabled: bool,
    pub tool_invocation_enabled: bool,
    pub ledger_written: bool,
    pub approval_requested: bool,
    pub mcp_server_started: bool,
    pub app_connector_started: bool,
    pub live_mutation_ready: bool,
    pub side_effect_free: bool,
    pub next_migration_step: &'static str,
    pub entries: Vec<PluginToolRegistrySourceOfTruthDryRunEntry>,
}

pub fn hepta_system_plugin_tool_registry_source_of_truth_dry_run_plan()
-> PluginToolRegistrySourceOfTruthDryRunPlan {
    let preview = hepta_system_plugin_tool_contribution_inventory_preview_plan();
    plugin_tool_registry_source_of_truth_dry_run_plan(&preview)
}

pub fn plugin_tool_registry_source_of_truth_dry_run_plan(
    preview: &PluginToolContributionInventoryPreviewPlan,
) -> PluginToolRegistrySourceOfTruthDryRunPlan {
    let inventory = ToolRegistryInventory {
        entries: preview.candidate_inventory_entries.clone(),
    };
    let guard = inventory.plan_invocation_guard();
    let duplicate_candidate_ids = duplicate_candidate_ids(preview);
    let duplicate_registry_ids = guard.duplicate_ids.clone();
    let mut unbound_candidate_ids = Vec::new();
    let mut entries = Vec::new();

    for candidate in &preview.entries {
        let decision = inventory.plan_invocation_guard_for_tool(candidate.candidate_tool_id);
        if !decision.entry_found || decision.duplicate_id {
            unbound_candidate_ids.push(candidate.candidate_tool_id.to_string());
        }
        entries.push(PluginToolRegistrySourceOfTruthDryRunEntry {
            plugin_id: candidate.plugin_id,
            contribution_kind: candidate.contribution_point,
            candidate_tool_id: candidate.candidate_tool_id,
            planned_registry_source: candidate.candidate_inventory_source,
            planned_registry_name: candidate.candidate_name,
            owner: candidate.owner,
            has_input_schema: candidate.has_input_schema,
            has_output_schema: candidate.has_output_schema,
            side_effect_level: candidate.side_effect_level,
            approval_kind: candidate.approval_kind,
            auth_required: candidate.auth_required,
            timeout_ms: candidate.timeout_ms,
            ledger_required: candidate.ledger_required,
            registry_entry_found: decision.entry_found,
            duplicate_id: decision.duplicate_id,
            guard_route: decision.route,
            approval_required: decision.approval_required,
            guard_blocked: decision.blocked,
            guard_blocked_reason: decision.blocked_reason,
            source_of_truth_registration_enabled: false,
            tool_invocation_enabled: false,
            ledger_write_enabled: false,
            approval_request_enabled: false,
            dry_run_ready: decision.entry_found
                && !decision.duplicate_id
                && decision.route == ToolRegistryInvocationGuardRoute::RequireApprovalLedger
                && decision.approval_required
                && decision.ledger_required
                && !decision.blocked
                && decision.side_effect_free,
        });
    }

    let planned_mcp_entry_count = inventory
        .entries
        .iter()
        .filter(|entry| entry.source == ToolRegistryInventorySource::Mcp)
        .count();
    let planned_connector_entry_count = inventory
        .entries
        .iter()
        .filter(|entry| entry.source == ToolRegistryInventorySource::Connector)
        .count();
    let all_candidate_ids_unique =
        duplicate_candidate_ids.is_empty() && duplicate_registry_ids.is_empty();
    let all_preview_candidates_bound_to_registry = unbound_candidate_ids.is_empty()
        && preview.entries.len() == inventory.entries.len()
        && entries.iter().all(|entry| entry.registry_entry_found);
    let all_candidates_have_schema = preview.all_candidates_have_schema
        && inventory
            .entries
            .iter()
            .all(|entry| entry.has_input_schema && entry.has_output_schema);
    let all_candidates_have_risk_metadata = preview.all_candidates_have_risk_metadata
        && guard.block_unknown_metadata_count == 0
        && guard.block_invalid_policy_count == 0;
    let all_candidates_require_ledger =
        preview.all_candidates_require_ledger && guard.all_entries_require_ledger;
    let mutating_candidates_require_approval =
        preview.mutating_candidates_require_approval && guard.mutating_tools_require_approval;
    let all_candidates_have_guard_route = preview.all_candidates_have_guard_route
        && entries.iter().all(|entry| {
            entry.guard_route == ToolRegistryInvocationGuardRoute::RequireApprovalLedger
        });
    let registry_source_of_truth_dry_run_ready = preview.preview_ready
        && all_candidate_ids_unique
        && all_preview_candidates_bound_to_registry
        && all_candidates_have_schema
        && all_candidates_have_risk_metadata
        && all_candidates_require_ledger
        && mutating_candidates_require_approval
        && all_candidates_have_guard_route
        && guard.invocation_guard_ready
        && entries.iter().all(|entry| entry.dry_run_ready);

    PluginToolRegistrySourceOfTruthDryRunPlan {
        runtime: "hepta",
        surface: "plugin_tool_registry_source_of_truth_dry_run",
        plugin_id: preview.plugin_id,
        status: if registry_source_of_truth_dry_run_ready {
            "ready"
        } else {
            "blocked"
        },
        source_preview_surface: preview.surface,
        source_preview_ready: preview.preview_ready,
        planned_source_of_truth_surface: "tool_registry_inventory",
        hepta_system_manifest_present: preview.hepta_system_manifest_present,
        preview_candidate_count: preview.entries.len(),
        planned_registry_entry_count: inventory.entries.len(),
        planned_mcp_entry_count,
        planned_connector_entry_count,
        duplicate_candidate_ids,
        duplicate_registry_ids,
        unbound_candidate_ids,
        all_candidate_ids_unique,
        all_preview_candidates_bound_to_registry,
        all_candidates_have_schema,
        all_candidates_have_risk_metadata,
        all_candidates_require_ledger,
        mutating_candidates_require_approval,
        all_candidates_have_guard_route,
        registry_invocation_guard_ready: guard.invocation_guard_ready,
        registry_source_of_truth_dry_run_ready,
        registry_source_of_truth_enabled: false,
        tool_registry_registration_enabled: false,
        tool_invocation_enabled: false,
        ledger_written: false,
        approval_requested: false,
        mcp_server_started: false,
        app_connector_started: false,
        live_mutation_ready: false,
        side_effect_free: true,
        next_migration_step: "restore_tool_registry_invocation_source_of_truth_without_execution",
        entries,
    }
}

fn duplicate_candidate_ids(preview: &PluginToolContributionInventoryPreviewPlan) -> Vec<String> {
    let mut seen = BTreeSet::new();
    let mut duplicates = BTreeSet::new();
    for candidate in &preview.entries {
        if !seen.insert(candidate.candidate_tool_id.to_string()) {
            duplicates.insert(candidate.candidate_tool_id.to_string());
        }
    }
    duplicates.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plugin_tool_registry_source_of_truth_dry_run_binds_preview_candidates() {
        let plan = hepta_system_plugin_tool_registry_source_of_truth_dry_run_plan();

        assert_eq!(plan.status, "ready");
        assert_eq!(
            plan.source_preview_surface,
            "plugin_tool_contribution_inventory_preview"
        );
        assert!(plan.source_preview_ready);
        assert_eq!(
            plan.planned_source_of_truth_surface,
            "tool_registry_inventory"
        );
        assert!(plan.hepta_system_manifest_present);
        assert_eq!(plan.preview_candidate_count, 2);
        assert_eq!(plan.planned_registry_entry_count, 2);
        assert_eq!(plan.planned_mcp_entry_count, 1);
        assert_eq!(plan.planned_connector_entry_count, 1);
        assert_eq!(plan.duplicate_candidate_ids, Vec::<String>::new());
        assert_eq!(plan.duplicate_registry_ids, Vec::<String>::new());
        assert_eq!(plan.unbound_candidate_ids, Vec::<String>::new());
        assert!(plan.all_candidate_ids_unique);
        assert!(plan.all_preview_candidates_bound_to_registry);
        assert!(plan.registry_invocation_guard_ready);
        assert!(plan.registry_source_of_truth_dry_run_ready);
        assert_eq!(
            plan.entries
                .iter()
                .map(|entry| entry.planned_registry_source)
                .collect::<Vec<_>>(),
            [
                ToolRegistryInventorySource::Mcp,
                ToolRegistryInventorySource::Connector,
            ]
        );
    }

    #[test]
    fn plugin_tool_registry_source_of_truth_dry_run_requires_guard_metadata() {
        let plan = hepta_system_plugin_tool_registry_source_of_truth_dry_run_plan();

        assert!(plan.all_candidates_have_schema);
        assert!(plan.all_candidates_have_risk_metadata);
        assert!(plan.all_candidates_require_ledger);
        assert!(plan.mutating_candidates_require_approval);
        assert!(plan.all_candidates_have_guard_route);
        assert!(plan.entries.iter().all(|entry| {
            entry.guard_route == ToolRegistryInvocationGuardRoute::RequireApprovalLedger
                && entry.approval_required
                && entry.ledger_required
                && !entry.guard_blocked
                && entry.guard_blocked_reason.is_none()
                && entry.dry_run_ready
        }));
    }

    #[test]
    fn plugin_tool_registry_source_of_truth_dry_run_does_not_register_or_invoke() {
        let plan = hepta_system_plugin_tool_registry_source_of_truth_dry_run_plan();

        assert!(plan.registry_source_of_truth_dry_run_ready);
        assert!(!plan.registry_source_of_truth_enabled);
        assert!(!plan.tool_registry_registration_enabled);
        assert!(!plan.tool_invocation_enabled);
        assert!(!plan.ledger_written);
        assert!(!plan.approval_requested);
        assert!(!plan.mcp_server_started);
        assert!(!plan.app_connector_started);
        assert!(!plan.live_mutation_ready);
        assert!(plan.side_effect_free);
        assert!(plan.entries.iter().all(|entry| {
            !entry.source_of_truth_registration_enabled
                && !entry.tool_invocation_enabled
                && !entry.ledger_write_enabled
                && !entry.approval_request_enabled
        }));
    }

    #[test]
    fn plugin_tool_registry_source_of_truth_dry_run_blocks_duplicate_candidate_ids() {
        let mut preview = hepta_system_plugin_tool_contribution_inventory_preview_plan();
        preview.entries.push(preview.entries[0].clone());
        preview
            .candidate_inventory_entries
            .push(preview.candidate_inventory_entries[0].clone());

        let plan = plugin_tool_registry_source_of_truth_dry_run_plan(&preview);

        assert_eq!(plan.status, "blocked");
        assert_eq!(
            plan.duplicate_candidate_ids,
            vec!["preview:mcp:hepta-system@hepta-local:hepta_system_local_mcp"]
        );
        assert_eq!(
            plan.duplicate_registry_ids,
            vec!["preview:mcp:hepta-system@hepta-local:hepta_system_local_mcp"]
        );
        assert_eq!(
            plan.unbound_candidate_ids,
            vec![
                "preview:mcp:hepta-system@hepta-local:hepta_system_local_mcp",
                "preview:mcp:hepta-system@hepta-local:hepta_system_local_mcp",
            ]
        );
        assert!(!plan.all_candidate_ids_unique);
        assert!(!plan.all_preview_candidates_bound_to_registry);
        assert!(!plan.registry_source_of_truth_dry_run_ready);
    }
}

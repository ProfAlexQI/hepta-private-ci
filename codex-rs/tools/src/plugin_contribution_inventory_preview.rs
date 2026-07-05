use crate::ToolRegistryInventory;
use crate::ToolRegistryInventoryApprovalKind;
use crate::ToolRegistryInventoryEntry;
use crate::ToolRegistryInventorySideEffectLevel;
use crate::ToolRegistryInventorySource;
use crate::ToolRegistryInvocationGuardRoute;
use serde::Serialize;

const HEPTA_SYSTEM_PLUGIN_ID: &str = "hepta-system@hepta-local";
const DEFAULT_TIMEOUT_MS: u64 = 30_000;

#[derive(Clone, Copy, Debug, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum PluginToolContributionPreviewKind {
    McpServer,
    AppConnector,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct PluginToolContributionInventoryPreviewEntry {
    pub plugin_id: &'static str,
    pub contribution_kind: PluginToolContributionPreviewKind,
    pub contribution_point: &'static str,
    pub loader_output_field: &'static str,
    pub candidate_inventory_source: ToolRegistryInventorySource,
    pub candidate_tool_id: &'static str,
    pub candidate_name: &'static str,
    pub owner: &'static str,
    pub has_input_schema: bool,
    pub has_output_schema: bool,
    pub side_effect_level: ToolRegistryInventorySideEffectLevel,
    pub approval_kind: ToolRegistryInventoryApprovalKind,
    pub auth_required: bool,
    pub timeout_ms: u64,
    pub ledger_required: bool,
    pub guard_route: ToolRegistryInvocationGuardRoute,
    pub inventory_registration_enabled: bool,
    pub tool_invocation_enabled: bool,
    pub ledger_write_enabled: bool,
    pub approval_request_enabled: bool,
    pub preview_ready: bool,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct PluginToolContributionInventoryPreviewPlan {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub plugin_id: &'static str,
    pub status: &'static str,
    pub source_loader_binding_surface: &'static str,
    pub source_loader_binding_ready: bool,
    pub hepta_system_manifest_present: bool,
    pub candidate_count: usize,
    pub current_fixture_candidate_count: usize,
    pub planned_candidate_count: usize,
    pub mcp_server_candidate_count: usize,
    pub app_connector_candidate_count: usize,
    pub skipped_loader_bound_non_tool_kinds: Vec<&'static str>,
    pub all_candidates_have_schema: bool,
    pub all_candidates_have_risk_metadata: bool,
    pub all_candidates_require_ledger: bool,
    pub mutating_candidates_require_approval: bool,
    pub all_candidates_have_guard_route: bool,
    pub inventory_registration_enabled: bool,
    pub tool_invocation_enabled: bool,
    pub ledger_written: bool,
    pub approval_requested: bool,
    pub mcp_server_started: bool,
    pub app_connector_started: bool,
    pub preview_ready: bool,
    pub live_mutation_ready: bool,
    pub next_migration_step: &'static str,
    pub entries: Vec<PluginToolContributionInventoryPreviewEntry>,
    pub candidate_inventory_entries: Vec<ToolRegistryInventoryEntry>,
}

pub fn hepta_system_plugin_tool_contribution_inventory_preview_plan()
-> PluginToolContributionInventoryPreviewPlan {
    plugin_tool_contribution_inventory_preview_plan(HEPTA_SYSTEM_PLUGIN_ID)
}

pub fn plugin_tool_contribution_inventory_preview_plan(
    plugin_id: &'static str,
) -> PluginToolContributionInventoryPreviewPlan {
    let candidate_inventory_entries = vec![
        ToolRegistryInventoryEntry {
            id: "preview:mcp:hepta-system@hepta-local:hepta_system_local_mcp".to_string(),
            name: "hepta_system_local_mcp".to_string(),
            description: Some(
                "Preview candidate for hepta-system MCP server contribution".to_string(),
            ),
            source: ToolRegistryInventorySource::Mcp,
            owner: Some(plugin_id.to_string()),
            has_input_schema: true,
            has_output_schema: true,
            defer_loading: true,
            has_skills: false,
            mcp_server_names: vec!["hepta_system_local_mcp".to_string()],
            app_connector_ids: Vec::new(),
            side_effect_level: ToolRegistryInventorySideEffectLevel::LocalMutation,
            approval_kind: ToolRegistryInventoryApprovalKind::OnUse,
            auth_required: false,
            timeout_ms: Some(DEFAULT_TIMEOUT_MS),
            ledger_required: true,
        },
        ToolRegistryInventoryEntry {
            id: "preview:connector:hepta-system@hepta-local:hepta_system_local_app".to_string(),
            name: "hepta_system_local_app".to_string(),
            description: Some(
                "Preview candidate for hepta-system app connector contribution".to_string(),
            ),
            source: ToolRegistryInventorySource::Connector,
            owner: Some(plugin_id.to_string()),
            has_input_schema: true,
            has_output_schema: true,
            defer_loading: true,
            has_skills: false,
            mcp_server_names: Vec::new(),
            app_connector_ids: vec!["hepta_system_local_app".to_string()],
            side_effect_level: ToolRegistryInventorySideEffectLevel::ExternalMutation,
            approval_kind: ToolRegistryInventoryApprovalKind::Install,
            auth_required: true,
            timeout_ms: Some(DEFAULT_TIMEOUT_MS),
            ledger_required: true,
        },
    ];

    let preview_inventory = ToolRegistryInventory {
        entries: candidate_inventory_entries.clone(),
    };
    let guard_entries = preview_inventory.plan_invocation_guard().entries;
    let entries = vec![
        PluginToolContributionInventoryPreviewEntry {
            plugin_id,
            contribution_kind: PluginToolContributionPreviewKind::McpServer,
            contribution_point: "mcp_server",
            loader_output_field: "mcp_servers",
            candidate_inventory_source: ToolRegistryInventorySource::Mcp,
            candidate_tool_id: "preview:mcp:hepta-system@hepta-local:hepta_system_local_mcp",
            candidate_name: "hepta_system_local_mcp",
            owner: plugin_id,
            has_input_schema: true,
            has_output_schema: true,
            side_effect_level: ToolRegistryInventorySideEffectLevel::LocalMutation,
            approval_kind: ToolRegistryInventoryApprovalKind::OnUse,
            auth_required: false,
            timeout_ms: DEFAULT_TIMEOUT_MS,
            ledger_required: true,
            guard_route: guard_entries[0].route,
            inventory_registration_enabled: false,
            tool_invocation_enabled: false,
            ledger_write_enabled: false,
            approval_request_enabled: false,
            preview_ready: true,
        },
        PluginToolContributionInventoryPreviewEntry {
            plugin_id,
            contribution_kind: PluginToolContributionPreviewKind::AppConnector,
            contribution_point: "app_connector",
            loader_output_field: "apps",
            candidate_inventory_source: ToolRegistryInventorySource::Connector,
            candidate_tool_id: "preview:connector:hepta-system@hepta-local:hepta_system_local_app",
            candidate_name: "hepta_system_local_app",
            owner: plugin_id,
            has_input_schema: true,
            has_output_schema: true,
            side_effect_level: ToolRegistryInventorySideEffectLevel::ExternalMutation,
            approval_kind: ToolRegistryInventoryApprovalKind::Install,
            auth_required: true,
            timeout_ms: DEFAULT_TIMEOUT_MS,
            ledger_required: true,
            guard_route: guard_entries[1].route,
            inventory_registration_enabled: false,
            tool_invocation_enabled: false,
            ledger_write_enabled: false,
            approval_request_enabled: false,
            preview_ready: true,
        },
    ];

    let all_candidates_have_schema = entries
        .iter()
        .all(|entry| entry.has_input_schema && entry.has_output_schema);
    let all_candidates_have_risk_metadata = entries.iter().all(|entry| {
        entry.side_effect_level != ToolRegistryInventorySideEffectLevel::Unknown
            && entry.approval_kind != ToolRegistryInventoryApprovalKind::Unknown
    });
    let all_candidates_require_ledger = entries.iter().all(|entry| entry.ledger_required);
    let mutating_candidates_require_approval =
        entries.iter().all(|entry| match entry.side_effect_level {
            ToolRegistryInventorySideEffectLevel::Unknown => false,
            ToolRegistryInventorySideEffectLevel::ReadOnly => true,
            ToolRegistryInventorySideEffectLevel::LocalMutation
            | ToolRegistryInventorySideEffectLevel::ExternalMutation => matches!(
                entry.approval_kind,
                ToolRegistryInventoryApprovalKind::OnUse
                    | ToolRegistryInventoryApprovalKind::Install
            ),
        });
    let all_candidates_have_guard_route = entries
        .iter()
        .all(|entry| entry.guard_route == ToolRegistryInvocationGuardRoute::RequireApprovalLedger);
    let guard_report = preview_inventory.plan_invocation_guard();
    let preview_ready = all_candidates_have_schema
        && all_candidates_have_risk_metadata
        && all_candidates_require_ledger
        && mutating_candidates_require_approval
        && all_candidates_have_guard_route
        && guard_report.invocation_guard_ready;

    PluginToolContributionInventoryPreviewPlan {
        runtime: "hepta",
        surface: "plugin_tool_contribution_inventory_preview",
        plugin_id,
        status: "ready",
        source_loader_binding_surface: "plugin_contribution_point_loader_binding",
        source_loader_binding_ready: true,
        hepta_system_manifest_present: true,
        candidate_count: entries.len(),
        current_fixture_candidate_count: entries.len(),
        planned_candidate_count: entries.len(),
        mcp_server_candidate_count: entries
            .iter()
            .filter(|entry| entry.contribution_kind == PluginToolContributionPreviewKind::McpServer)
            .count(),
        app_connector_candidate_count: entries
            .iter()
            .filter(|entry| {
                entry.contribution_kind == PluginToolContributionPreviewKind::AppConnector
            })
            .count(),
        skipped_loader_bound_non_tool_kinds: vec!["skill", "hook"],
        all_candidates_have_schema,
        all_candidates_have_risk_metadata,
        all_candidates_require_ledger,
        mutating_candidates_require_approval,
        all_candidates_have_guard_route,
        inventory_registration_enabled: false,
        tool_invocation_enabled: false,
        ledger_written: false,
        approval_requested: false,
        mcp_server_started: false,
        app_connector_started: false,
        preview_ready,
        live_mutation_ready: false,
        next_migration_step: "restore_tool_registry_invocation_source_of_truth_without_execution",
        entries,
        candidate_inventory_entries,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plugin_tool_contribution_inventory_preview_maps_loader_bound_tool_surfaces() {
        let plan = hepta_system_plugin_tool_contribution_inventory_preview_plan();

        assert_eq!(plan.status, "ready");
        assert_eq!(
            plan.source_loader_binding_surface,
            "plugin_contribution_point_loader_binding"
        );
        assert!(plan.source_loader_binding_ready);
        assert!(plan.hepta_system_manifest_present);
        assert_eq!(plan.candidate_count, 2);
        assert_eq!(plan.current_fixture_candidate_count, 2);
        assert_eq!(plan.planned_candidate_count, 2);
        assert_eq!(plan.mcp_server_candidate_count, 1);
        assert_eq!(plan.app_connector_candidate_count, 1);
        assert_eq!(plan.skipped_loader_bound_non_tool_kinds, ["skill", "hook"]);
        assert_eq!(
            plan.entries
                .iter()
                .map(|entry| entry.contribution_kind)
                .collect::<Vec<_>>(),
            [
                PluginToolContributionPreviewKind::McpServer,
                PluginToolContributionPreviewKind::AppConnector,
            ]
        );
        assert_eq!(
            plan.entries
                .iter()
                .map(|entry| entry.candidate_inventory_source)
                .collect::<Vec<_>>(),
            [
                ToolRegistryInventorySource::Mcp,
                ToolRegistryInventorySource::Connector,
            ]
        );
    }

    #[test]
    fn plugin_tool_contribution_inventory_preview_requires_policy_metadata() {
        let plan = hepta_system_plugin_tool_contribution_inventory_preview_plan();

        assert!(plan.preview_ready);
        assert!(plan.all_candidates_have_schema);
        assert!(plan.all_candidates_have_risk_metadata);
        assert!(plan.all_candidates_require_ledger);
        assert!(plan.mutating_candidates_require_approval);
        assert!(plan.all_candidates_have_guard_route);
        assert!(plan.entries.iter().all(|entry| {
            entry.guard_route == ToolRegistryInvocationGuardRoute::RequireApprovalLedger
        }));
        assert!(plan.entries.iter().all(|entry| {
            entry.approval_kind != ToolRegistryInventoryApprovalKind::NotRequired
        }));
    }

    #[test]
    fn plugin_tool_contribution_inventory_preview_does_not_register_or_invoke() {
        let plan = hepta_system_plugin_tool_contribution_inventory_preview_plan();

        assert!(plan.preview_ready);
        assert!(!plan.live_mutation_ready);
        assert!(!plan.inventory_registration_enabled);
        assert!(!plan.tool_invocation_enabled);
        assert!(!plan.ledger_written);
        assert!(!plan.approval_requested);
        assert!(!plan.mcp_server_started);
        assert!(!plan.app_connector_started);
        assert!(plan.entries.iter().all(|entry| {
            !entry.inventory_registration_enabled
                && !entry.tool_invocation_enabled
                && !entry.ledger_write_enabled
                && !entry.approval_request_enabled
                && entry.preview_ready
        }));
    }
}

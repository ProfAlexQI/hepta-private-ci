use serde::Serialize;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PluginContributionPointKind {
    Skill,
    McpServer,
    Tool,
    AppConnector,
    Hook,
    Permission,
    ActivationEvent,
    LocalStorage,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PluginContributionPointSource {
    ManifestPath,
    ToolRegistryBridge,
    AppConnectorManifest,
    HookManifest,
    ReviewPolicy,
    ActivationPolicy,
    PluginDataRoot,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct PluginContributionPointAbiEntry {
    pub kind: PluginContributionPointKind,
    pub manifest_field: Option<&'static str>,
    pub source: PluginContributionPointSource,
    pub current_fixture_declared: bool,
    pub manifest_path_supported: bool,
    pub loader_path_supported: bool,
    pub future_bridge_required: bool,
    pub tool_registry_bridge_required: bool,
    pub approval_policy_required: bool,
    pub ledger_required: bool,
    pub permission_policy_required: bool,
    pub activation_policy_required: bool,
    pub local_storage_scope_required: bool,
    pub side_effect_level: &'static str,
    pub runtime_execution_enabled: bool,
    pub external_mutation_enabled: bool,
    pub credential_read_enabled: bool,
    pub provider_call_enabled: bool,
    pub gateway_mutation_enabled: bool,
    pub native_post_mutation_enabled: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct PluginContributionPointAbiMatrix {
    pub plugin_id: &'static str,
    pub surface: &'static str,
    pub registry_api: &'static str,
    pub abi_entry_count: usize,
    pub manifest_path_supported_count: usize,
    pub loader_path_supported_count: usize,
    pub current_fixture_declared_count: usize,
    pub future_bridge_required_count: usize,
    pub approval_policy_required_count: usize,
    pub ledger_required_count: usize,
    pub tool_registry_bridge_required: bool,
    pub permission_policy_required: bool,
    pub activation_policy_required: bool,
    pub local_storage_scoped_to_plugin_data_root: bool,
    pub all_entries_policy_bound: bool,
    pub all_runtime_execution_disabled: bool,
    pub all_live_paths_blocked: bool,
    pub abi_ready: bool,
    pub live_mutation_ready: bool,
    pub next_migration_step: &'static str,
    pub entries: Vec<PluginContributionPointAbiEntry>,
}

pub fn hepta_system_plugin_contribution_point_abi_matrix() -> PluginContributionPointAbiMatrix {
    plan_hepta_system_plugin_contribution_point_abi("hepta-system@hepta-local")
}

pub fn plan_hepta_system_plugin_contribution_point_abi(
    plugin_id: &'static str,
) -> PluginContributionPointAbiMatrix {
    let entries = vec![
        PluginContributionPointAbiEntry {
            kind: PluginContributionPointKind::Skill,
            manifest_field: Some("skills"),
            source: PluginContributionPointSource::ManifestPath,
            current_fixture_declared: false,
            manifest_path_supported: true,
            loader_path_supported: true,
            future_bridge_required: false,
            tool_registry_bridge_required: false,
            approval_policy_required: false,
            ledger_required: false,
            permission_policy_required: true,
            activation_policy_required: false,
            local_storage_scope_required: false,
            side_effect_level: "local_read_only",
            runtime_execution_enabled: false,
            external_mutation_enabled: false,
            credential_read_enabled: false,
            provider_call_enabled: false,
            gateway_mutation_enabled: false,
            native_post_mutation_enabled: false,
        },
        PluginContributionPointAbiEntry {
            kind: PluginContributionPointKind::McpServer,
            manifest_field: Some("mcpServers"),
            source: PluginContributionPointSource::ManifestPath,
            current_fixture_declared: false,
            manifest_path_supported: true,
            loader_path_supported: true,
            future_bridge_required: false,
            tool_registry_bridge_required: true,
            approval_policy_required: true,
            ledger_required: true,
            permission_policy_required: true,
            activation_policy_required: true,
            local_storage_scope_required: false,
            side_effect_level: "tool_policy_gated",
            runtime_execution_enabled: false,
            external_mutation_enabled: false,
            credential_read_enabled: false,
            provider_call_enabled: false,
            gateway_mutation_enabled: false,
            native_post_mutation_enabled: false,
        },
        PluginContributionPointAbiEntry {
            kind: PluginContributionPointKind::Tool,
            manifest_field: None,
            source: PluginContributionPointSource::ToolRegistryBridge,
            current_fixture_declared: false,
            manifest_path_supported: false,
            loader_path_supported: false,
            future_bridge_required: true,
            tool_registry_bridge_required: true,
            approval_policy_required: true,
            ledger_required: true,
            permission_policy_required: true,
            activation_policy_required: true,
            local_storage_scope_required: false,
            side_effect_level: "tool_policy_gated",
            runtime_execution_enabled: false,
            external_mutation_enabled: false,
            credential_read_enabled: false,
            provider_call_enabled: false,
            gateway_mutation_enabled: false,
            native_post_mutation_enabled: false,
        },
        PluginContributionPointAbiEntry {
            kind: PluginContributionPointKind::AppConnector,
            manifest_field: Some("apps"),
            source: PluginContributionPointSource::AppConnectorManifest,
            current_fixture_declared: false,
            manifest_path_supported: true,
            loader_path_supported: true,
            future_bridge_required: false,
            tool_registry_bridge_required: true,
            approval_policy_required: true,
            ledger_required: true,
            permission_policy_required: true,
            activation_policy_required: true,
            local_storage_scope_required: false,
            side_effect_level: "connector_policy_gated",
            runtime_execution_enabled: false,
            external_mutation_enabled: false,
            credential_read_enabled: false,
            provider_call_enabled: false,
            gateway_mutation_enabled: false,
            native_post_mutation_enabled: false,
        },
        PluginContributionPointAbiEntry {
            kind: PluginContributionPointKind::Hook,
            manifest_field: Some("hooks"),
            source: PluginContributionPointSource::HookManifest,
            current_fixture_declared: false,
            manifest_path_supported: true,
            loader_path_supported: true,
            future_bridge_required: false,
            tool_registry_bridge_required: false,
            approval_policy_required: true,
            ledger_required: true,
            permission_policy_required: true,
            activation_policy_required: true,
            local_storage_scope_required: false,
            side_effect_level: "hook_policy_gated",
            runtime_execution_enabled: false,
            external_mutation_enabled: false,
            credential_read_enabled: false,
            provider_call_enabled: false,
            gateway_mutation_enabled: false,
            native_post_mutation_enabled: false,
        },
        PluginContributionPointAbiEntry {
            kind: PluginContributionPointKind::Permission,
            manifest_field: None,
            source: PluginContributionPointSource::ReviewPolicy,
            current_fixture_declared: false,
            manifest_path_supported: false,
            loader_path_supported: false,
            future_bridge_required: true,
            tool_registry_bridge_required: false,
            approval_policy_required: true,
            ledger_required: false,
            permission_policy_required: true,
            activation_policy_required: false,
            local_storage_scope_required: false,
            side_effect_level: "policy_only",
            runtime_execution_enabled: false,
            external_mutation_enabled: false,
            credential_read_enabled: false,
            provider_call_enabled: false,
            gateway_mutation_enabled: false,
            native_post_mutation_enabled: false,
        },
        PluginContributionPointAbiEntry {
            kind: PluginContributionPointKind::ActivationEvent,
            manifest_field: None,
            source: PluginContributionPointSource::ActivationPolicy,
            current_fixture_declared: false,
            manifest_path_supported: false,
            loader_path_supported: false,
            future_bridge_required: true,
            tool_registry_bridge_required: false,
            approval_policy_required: true,
            ledger_required: false,
            permission_policy_required: true,
            activation_policy_required: true,
            local_storage_scope_required: false,
            side_effect_level: "activation_policy_only",
            runtime_execution_enabled: false,
            external_mutation_enabled: false,
            credential_read_enabled: false,
            provider_call_enabled: false,
            gateway_mutation_enabled: false,
            native_post_mutation_enabled: false,
        },
        PluginContributionPointAbiEntry {
            kind: PluginContributionPointKind::LocalStorage,
            manifest_field: None,
            source: PluginContributionPointSource::PluginDataRoot,
            current_fixture_declared: false,
            manifest_path_supported: false,
            loader_path_supported: false,
            future_bridge_required: true,
            tool_registry_bridge_required: false,
            approval_policy_required: false,
            ledger_required: false,
            permission_policy_required: true,
            activation_policy_required: false,
            local_storage_scope_required: true,
            side_effect_level: "local_scoped_storage_disabled",
            runtime_execution_enabled: false,
            external_mutation_enabled: false,
            credential_read_enabled: false,
            provider_call_enabled: false,
            gateway_mutation_enabled: false,
            native_post_mutation_enabled: false,
        },
    ];

    let manifest_path_supported_count = entries
        .iter()
        .filter(|entry| entry.manifest_path_supported)
        .count();
    let loader_path_supported_count = entries
        .iter()
        .filter(|entry| entry.loader_path_supported)
        .count();
    let current_fixture_declared_count = entries
        .iter()
        .filter(|entry| entry.current_fixture_declared)
        .count();
    let future_bridge_required_count = entries
        .iter()
        .filter(|entry| entry.future_bridge_required)
        .count();
    let approval_policy_required_count = entries
        .iter()
        .filter(|entry| entry.approval_policy_required)
        .count();
    let ledger_required_count = entries.iter().filter(|entry| entry.ledger_required).count();
    let tool_registry_bridge_required = entries
        .iter()
        .any(|entry| entry.tool_registry_bridge_required);
    let permission_policy_required = entries.iter().all(|entry| entry.permission_policy_required);
    let activation_policy_required = entries
        .iter()
        .filter(|entry| {
            matches!(
                entry.kind,
                PluginContributionPointKind::McpServer
                    | PluginContributionPointKind::Tool
                    | PluginContributionPointKind::AppConnector
                    | PluginContributionPointKind::Hook
                    | PluginContributionPointKind::ActivationEvent
            )
        })
        .all(|entry| entry.activation_policy_required);
    let local_storage_scoped_to_plugin_data_root = entries.iter().any(|entry| {
        entry.kind == PluginContributionPointKind::LocalStorage
            && entry.local_storage_scope_required
            && !entry.runtime_execution_enabled
    });
    let all_entries_policy_bound = entries
        .iter()
        .all(|entry| !entry.side_effect_level.is_empty() && entry.permission_policy_required);
    let all_runtime_execution_disabled =
        entries.iter().all(|entry| !entry.runtime_execution_enabled);
    let all_live_paths_blocked = entries.iter().all(|entry| {
        !entry.external_mutation_enabled
            && !entry.credential_read_enabled
            && !entry.provider_call_enabled
            && !entry.gateway_mutation_enabled
            && !entry.native_post_mutation_enabled
            && !entry.runtime_execution_enabled
    });
    let abi_ready = entries.len() == 8
        && manifest_path_supported_count == 4
        && loader_path_supported_count == 4
        && current_fixture_declared_count == 0
        && future_bridge_required_count == 4
        && tool_registry_bridge_required
        && permission_policy_required
        && activation_policy_required
        && local_storage_scoped_to_plugin_data_root
        && all_entries_policy_bound
        && all_runtime_execution_disabled
        && all_live_paths_blocked;

    PluginContributionPointAbiMatrix {
        plugin_id,
        surface: "plugin_contribution_point_abi",
        registry_api: "hepta.systems.pluginRegistry/v1",
        abi_entry_count: entries.len(),
        manifest_path_supported_count,
        loader_path_supported_count,
        current_fixture_declared_count,
        future_bridge_required_count,
        approval_policy_required_count,
        ledger_required_count,
        tool_registry_bridge_required,
        permission_policy_required,
        activation_policy_required,
        local_storage_scoped_to_plugin_data_root,
        all_entries_policy_bound,
        all_runtime_execution_disabled,
        all_live_paths_blocked,
        abi_ready,
        live_mutation_ready: false,
        next_migration_step: "bind_contribution_point_abi_to_manifest_loader_and_tool_registry",
        entries,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hepta_system_contribution_point_abi_covers_extension_points() {
        let matrix = hepta_system_plugin_contribution_point_abi_matrix();
        let kinds = matrix
            .entries
            .iter()
            .map(|entry| entry.kind)
            .collect::<Vec<_>>();

        assert_eq!(
            kinds,
            vec![
                PluginContributionPointKind::Skill,
                PluginContributionPointKind::McpServer,
                PluginContributionPointKind::Tool,
                PluginContributionPointKind::AppConnector,
                PluginContributionPointKind::Hook,
                PluginContributionPointKind::Permission,
                PluginContributionPointKind::ActivationEvent,
                PluginContributionPointKind::LocalStorage,
            ]
        );
        assert_eq!(matrix.abi_entry_count, 8);
        assert_eq!(matrix.manifest_path_supported_count, 4);
        assert_eq!(matrix.loader_path_supported_count, 4);
        assert_eq!(matrix.current_fixture_declared_count, 0);
        assert_eq!(matrix.future_bridge_required_count, 4);
        assert!(matrix.abi_ready);
    }

    #[test]
    fn contribution_point_abi_blocks_live_mutation_by_default() {
        let matrix = hepta_system_plugin_contribution_point_abi_matrix();

        assert!(matrix.all_runtime_execution_disabled);
        assert!(matrix.all_live_paths_blocked);
        assert!(!matrix.live_mutation_ready);
        assert!(matrix.entries.iter().all(|entry| {
            !entry.runtime_execution_enabled
                && !entry.external_mutation_enabled
                && !entry.credential_read_enabled
                && !entry.provider_call_enabled
                && !entry.gateway_mutation_enabled
                && !entry.native_post_mutation_enabled
        }));
    }

    #[test]
    fn contribution_point_abi_marks_future_bridges_without_enabling_execution() {
        let matrix = hepta_system_plugin_contribution_point_abi_matrix();
        let future_bridge_kinds = matrix
            .entries
            .iter()
            .filter_map(|entry| entry.future_bridge_required.then_some(entry.kind))
            .collect::<Vec<_>>();

        assert_eq!(
            future_bridge_kinds,
            vec![
                PluginContributionPointKind::Tool,
                PluginContributionPointKind::Permission,
                PluginContributionPointKind::ActivationEvent,
                PluginContributionPointKind::LocalStorage,
            ]
        );
        assert!(matrix.tool_registry_bridge_required);
        assert!(matrix.permission_policy_required);
        assert!(matrix.activation_policy_required);
        assert!(matrix.local_storage_scoped_to_plugin_data_root);
        assert_eq!(matrix.ledger_required_count, 4);
    }
}

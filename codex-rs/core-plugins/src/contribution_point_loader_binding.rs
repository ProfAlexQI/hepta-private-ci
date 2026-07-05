use crate::contribution_point_abi::PluginContributionPointAbiMatrix;
use crate::contribution_point_abi::PluginContributionPointKind;
use crate::contribution_point_abi::hepta_system_plugin_contribution_point_abi_matrix;
use serde::Serialize;
use serde_json::Value as JsonValue;
use std::fs;
use std::path::Path;

const HEPTA_SYSTEM_PLUGIN_MANIFEST_FIXTURE_RELATIVE_PATH: &str =
    "../../plugins/hepta-system/.codex-plugin/plugin.json";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PluginContributionPointLoaderBindingState {
    BoundToManifestLoader,
    FutureBridgeBlocked,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct PluginContributionPointLoaderBindingInputs {
    pub plugin_id: &'static str,
    pub hepta_system_manifest_present: bool,
    pub declared_manifest_fields: Vec<&'static str>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct PluginContributionPointLoaderBindingEntry {
    pub kind: PluginContributionPointKind,
    pub manifest_field: Option<&'static str>,
    pub loader_output_field: Option<&'static str>,
    pub binding_state: PluginContributionPointLoaderBindingState,
    pub current_fixture_declared: bool,
    pub manifest_loader_contract_ready: bool,
    pub future_bridge_required: bool,
    pub future_bridge_blocked: bool,
    pub abi_entry_ready: bool,
    pub tool_registry_registration_enabled: bool,
    pub runtime_execution_enabled: bool,
    pub local_storage_created: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct PluginContributionPointLoaderBindingPlan {
    pub plugin_id: &'static str,
    pub surface: &'static str,
    pub source_abi_surface: &'static str,
    pub source_abi_ready: bool,
    pub hepta_system_manifest_present: bool,
    pub abi_entry_count: usize,
    pub loader_contract_entry_count: usize,
    pub declared_manifest_field_count: usize,
    pub fixture_declared_bound_entry_count: usize,
    pub future_bridge_blocked_count: usize,
    pub unbound_without_future_bridge_count: usize,
    pub manifest_loader_fields: Vec<&'static str>,
    pub loader_output_fields: Vec<&'static str>,
    pub all_loader_bindings_have_abi_entries: bool,
    pub all_declared_manifest_paths_bound: bool,
    pub future_bridges_blocked_until_manifest_fields_exist: bool,
    pub loader_contract_ready: bool,
    pub current_fixture_binding_ready: bool,
    pub tool_registry_registration_enabled: bool,
    pub runtime_execution_enabled: bool,
    pub local_storage_created: bool,
    pub all_live_paths_blocked: bool,
    pub binding_ready: bool,
    pub live_mutation_ready: bool,
    pub next_migration_step: &'static str,
    pub entries: Vec<PluginContributionPointLoaderBindingEntry>,
}

impl PluginContributionPointLoaderBindingInputs {
    pub fn current_checkout_without_fixture(plugin_id: &'static str) -> Self {
        Self {
            plugin_id,
            hepta_system_manifest_present: false,
            declared_manifest_fields: Vec::new(),
        }
    }

    pub fn synthetic_fixture(
        plugin_id: &'static str,
        declared_manifest_fields: Vec<&'static str>,
    ) -> Self {
        Self {
            plugin_id,
            hepta_system_manifest_present: true,
            declared_manifest_fields,
        }
    }

    fn declares_manifest_field(&self, field: &'static str) -> bool {
        self.declared_manifest_fields.contains(&field)
    }
}

pub fn hepta_system_plugin_contribution_point_loader_binding_plan()
-> PluginContributionPointLoaderBindingPlan {
    let abi = hepta_system_plugin_contribution_point_abi_matrix();
    let inputs = hepta_system_plugin_contribution_point_loader_binding_inputs(abi.plugin_id);
    plan_plugin_contribution_point_loader_binding(&abi, &inputs)
}

pub fn hepta_system_plugin_contribution_point_loader_binding_inputs(
    plugin_id: &'static str,
) -> PluginContributionPointLoaderBindingInputs {
    try_hepta_system_plugin_contribution_point_loader_binding_inputs(plugin_id).unwrap_or_else(
        || PluginContributionPointLoaderBindingInputs::current_checkout_without_fixture(plugin_id),
    )
}

pub fn try_hepta_system_plugin_contribution_point_loader_binding_inputs(
    plugin_id: &'static str,
) -> Option<PluginContributionPointLoaderBindingInputs> {
    let manifest_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join(HEPTA_SYSTEM_PLUGIN_MANIFEST_FIXTURE_RELATIVE_PATH);
    let contents = fs::read_to_string(manifest_path).ok()?;
    let declared_manifest_fields = declared_loader_manifest_fields_from_json(&contents);
    Some(PluginContributionPointLoaderBindingInputs {
        plugin_id,
        hepta_system_manifest_present: true,
        declared_manifest_fields,
    })
}

pub fn declared_loader_manifest_fields_from_json(manifest_json: &str) -> Vec<&'static str> {
    let Ok(JsonValue::Object(manifest)) = serde_json::from_str::<JsonValue>(manifest_json) else {
        return Vec::new();
    };
    ["skills", "mcpServers", "apps", "hooks"]
        .into_iter()
        .filter(|field| manifest.get(*field).is_some())
        .collect()
}

pub fn plan_plugin_contribution_point_loader_binding(
    abi: &PluginContributionPointAbiMatrix,
    inputs: &PluginContributionPointLoaderBindingInputs,
) -> PluginContributionPointLoaderBindingPlan {
    let entries = abi
        .entries
        .iter()
        .map(|entry| {
            let loader_output_field = loader_output_field_for_contribution_kind(entry.kind);
            let manifest_loader_contract_ready = entry.manifest_path_supported
                && entry.loader_path_supported
                && entry.manifest_field.is_some()
                && loader_output_field.is_some();
            let current_fixture_declared = inputs.hepta_system_manifest_present
                && entry
                    .manifest_field
                    .is_some_and(|field| inputs.declares_manifest_field(field));
            let future_bridge_blocked = entry.future_bridge_required
                && entry.manifest_field.is_none()
                && loader_output_field.is_none()
                && !entry.runtime_execution_enabled;
            let binding_state = if manifest_loader_contract_ready {
                PluginContributionPointLoaderBindingState::BoundToManifestLoader
            } else {
                PluginContributionPointLoaderBindingState::FutureBridgeBlocked
            };

            PluginContributionPointLoaderBindingEntry {
                kind: entry.kind,
                manifest_field: entry.manifest_field,
                loader_output_field,
                binding_state,
                current_fixture_declared,
                manifest_loader_contract_ready,
                future_bridge_required: entry.future_bridge_required,
                future_bridge_blocked,
                abi_entry_ready: true,
                tool_registry_registration_enabled: false,
                runtime_execution_enabled: false,
                local_storage_created: false,
            }
        })
        .collect::<Vec<_>>();

    let loader_contract_entry_count = entries
        .iter()
        .filter(|entry| entry.manifest_loader_contract_ready)
        .count();
    let declared_manifest_field_count = inputs.declared_manifest_fields.len();
    let fixture_declared_bound_entry_count = entries
        .iter()
        .filter(|entry| entry.current_fixture_declared && entry.manifest_loader_contract_ready)
        .count();
    let future_bridge_blocked_count = entries
        .iter()
        .filter(|entry| entry.future_bridge_blocked)
        .count();
    let unbound_without_future_bridge_count = entries
        .iter()
        .filter(|entry| !entry.manifest_loader_contract_ready && !entry.future_bridge_blocked)
        .count();
    let manifest_loader_fields = entries
        .iter()
        .filter_map(|entry| {
            entry
                .manifest_loader_contract_ready
                .then_some(entry.manifest_field)
                .flatten()
        })
        .collect::<Vec<_>>();
    let loader_output_fields = entries
        .iter()
        .filter_map(|entry| {
            entry
                .manifest_loader_contract_ready
                .then_some(entry.loader_output_field)
                .flatten()
        })
        .collect::<Vec<_>>();
    let all_loader_bindings_have_abi_entries =
        entries.len() == abi.abi_entry_count && entries.iter().all(|entry| entry.abi_entry_ready);
    let all_declared_manifest_paths_bound =
        declared_manifest_field_count == fixture_declared_bound_entry_count;
    let future_bridges_blocked_until_manifest_fields_exist = entries
        .iter()
        .filter(|entry| entry.future_bridge_required)
        .all(|entry| entry.future_bridge_blocked);
    let loader_contract_ready = abi.abi_ready
        && loader_contract_entry_count == 4
        && future_bridge_blocked_count == 4
        && unbound_without_future_bridge_count == 0
        && manifest_loader_fields == vec!["skills", "mcpServers", "apps", "hooks"]
        && loader_output_fields == vec!["skill_roots", "mcp_servers", "apps", "hook_sources"]
        && all_loader_bindings_have_abi_entries
        && future_bridges_blocked_until_manifest_fields_exist;
    let current_fixture_binding_ready = inputs.hepta_system_manifest_present
        && declared_manifest_field_count > 0
        && all_declared_manifest_paths_bound;
    let tool_registry_registration_enabled = entries
        .iter()
        .any(|entry| entry.tool_registry_registration_enabled);
    let runtime_execution_enabled = entries.iter().any(|entry| entry.runtime_execution_enabled);
    let local_storage_created = entries.iter().any(|entry| entry.local_storage_created);
    let all_live_paths_blocked = abi.all_live_paths_blocked
        && !tool_registry_registration_enabled
        && !runtime_execution_enabled
        && !local_storage_created;
    let binding_ready = loader_contract_ready && all_declared_manifest_paths_bound;

    PluginContributionPointLoaderBindingPlan {
        plugin_id: inputs.plugin_id,
        surface: "plugin_contribution_point_loader_binding",
        source_abi_surface: abi.surface,
        source_abi_ready: abi.abi_ready,
        hepta_system_manifest_present: inputs.hepta_system_manifest_present,
        abi_entry_count: abi.abi_entry_count,
        loader_contract_entry_count,
        declared_manifest_field_count,
        fixture_declared_bound_entry_count,
        future_bridge_blocked_count,
        unbound_without_future_bridge_count,
        manifest_loader_fields,
        loader_output_fields,
        all_loader_bindings_have_abi_entries,
        all_declared_manifest_paths_bound,
        future_bridges_blocked_until_manifest_fields_exist,
        loader_contract_ready,
        current_fixture_binding_ready,
        tool_registry_registration_enabled,
        runtime_execution_enabled,
        local_storage_created,
        all_live_paths_blocked,
        binding_ready,
        live_mutation_ready: false,
        next_migration_step: "restore_plugin_tool_contribution_inventory_preview_without_registration",
        entries,
    }
}

fn loader_output_field_for_contribution_kind(
    kind: PluginContributionPointKind,
) -> Option<&'static str> {
    match kind {
        PluginContributionPointKind::Skill => Some("skill_roots"),
        PluginContributionPointKind::McpServer => Some("mcp_servers"),
        PluginContributionPointKind::AppConnector => Some("apps"),
        PluginContributionPointKind::Hook => Some("hook_sources"),
        PluginContributionPointKind::Tool
        | PluginContributionPointKind::Permission
        | PluginContributionPointKind::ActivationEvent
        | PluginContributionPointKind::LocalStorage => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contribution_point_abi::plan_hepta_system_plugin_contribution_point_abi;

    #[test]
    fn contribution_point_loader_binding_maps_current_loader_contract_to_abi_entries() {
        let plan = hepta_system_plugin_contribution_point_loader_binding_plan();

        assert!(plan.binding_ready);
        assert!(plan.loader_contract_ready);
        assert!(plan.current_fixture_binding_ready);
        assert_eq!(plan.source_abi_surface, "plugin_contribution_point_abi");
        assert_eq!(plan.abi_entry_count, 8);
        assert_eq!(plan.loader_contract_entry_count, 4);
        assert!(plan.hepta_system_manifest_present);
        assert_eq!(plan.declared_manifest_field_count, 3);
        assert_eq!(plan.fixture_declared_bound_entry_count, 3);
        assert_eq!(
            plan.manifest_loader_fields,
            vec!["skills", "mcpServers", "apps", "hooks"]
        );
        assert_eq!(
            plan.loader_output_fields,
            vec!["skill_roots", "mcp_servers", "apps", "hook_sources"]
        );
        assert!(plan.all_loader_bindings_have_abi_entries);
        assert!(plan.all_declared_manifest_paths_bound);
    }

    #[test]
    fn contribution_point_loader_binding_reads_hepta_system_fixture_fields() {
        let inputs = try_hepta_system_plugin_contribution_point_loader_binding_inputs(
            "hepta-system@hepta-local",
        )
        .expect("hepta-system manifest fixture should be present");

        assert!(inputs.hepta_system_manifest_present);
        assert_eq!(
            inputs.declared_manifest_fields,
            vec!["skills", "mcpServers", "apps"]
        );
    }

    #[test]
    fn contribution_point_loader_binding_counts_synthetic_fixture_declarations() {
        let abi = plan_hepta_system_plugin_contribution_point_abi("synthetic@hepta-local");
        let inputs = PluginContributionPointLoaderBindingInputs::synthetic_fixture(
            abi.plugin_id,
            vec!["skills", "mcpServers", "apps", "hooks"],
        );
        let plan = plan_plugin_contribution_point_loader_binding(&abi, &inputs);

        assert!(plan.binding_ready);
        assert!(plan.current_fixture_binding_ready);
        assert_eq!(plan.declared_manifest_field_count, 4);
        assert_eq!(plan.fixture_declared_bound_entry_count, 4);
        assert!(plan.all_declared_manifest_paths_bound);
    }

    #[test]
    fn contribution_point_loader_binding_keeps_future_bridges_blocked() {
        let plan = hepta_system_plugin_contribution_point_loader_binding_plan();
        let future_bridge_kinds = plan
            .entries
            .iter()
            .filter_map(|entry| entry.future_bridge_blocked.then_some(entry.kind))
            .collect::<Vec<_>>();

        assert_eq!(plan.future_bridge_blocked_count, 4);
        assert_eq!(plan.unbound_without_future_bridge_count, 0);
        assert_eq!(
            future_bridge_kinds,
            vec![
                PluginContributionPointKind::Tool,
                PluginContributionPointKind::Permission,
                PluginContributionPointKind::ActivationEvent,
                PluginContributionPointKind::LocalStorage,
            ]
        );
        assert!(plan.future_bridges_blocked_until_manifest_fields_exist);
    }

    #[test]
    fn contribution_point_loader_binding_does_not_register_or_execute_tools() {
        let plan = hepta_system_plugin_contribution_point_loader_binding_plan();

        assert!(!plan.tool_registry_registration_enabled);
        assert!(!plan.runtime_execution_enabled);
        assert!(!plan.local_storage_created);
        assert!(plan.all_live_paths_blocked);
        assert!(!plan.live_mutation_ready);
        assert!(plan.entries.iter().all(|entry| {
            !entry.tool_registry_registration_enabled
                && !entry.runtime_execution_enabled
                && !entry.local_storage_created
        }));
    }
}

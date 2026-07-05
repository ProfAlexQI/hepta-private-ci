use crate::contribution_point_abi::PluginContributionPointAbiMatrix;
use crate::contribution_point_abi::hepta_system_plugin_contribution_point_abi_matrix;
use crate::contribution_point_loader_binding::PluginContributionPointLoaderBindingPlan;
use crate::contribution_point_loader_binding::hepta_system_plugin_contribution_point_loader_binding_plan;
use crate::lifecycle_phase_summary::PluginLifecyclePhase;
use crate::lifecycle_phase_summary::PluginLifecyclePhaseSummary;
use crate::lifecycle_phase_summary::blocked_phase_count;
use crate::lifecycle_phase_summary::ready_phase_count;
use serde::Serialize;
use serde_json::Value as JsonValue;
use std::fs;
use std::path::Path;

const HEPTA_SYSTEM_PLUGIN_MANIFEST_FIXTURE_RELATIVE_PATH: &str =
    "../../plugins/hepta-system/.codex-plugin/plugin.json";

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
pub struct HeptaSystemPluginFixtureSummary {
    pub manifest_present: bool,
    pub skill_path_present: bool,
    pub mcp_servers_path_present: bool,
    pub apps_path_present: bool,
    pub skill_count: usize,
    pub mcp_server_count: usize,
    pub app_count: usize,
    pub hook_count: usize,
    pub tool_schema_count: usize,
    pub permission_count: usize,
    pub activation_event_count: usize,
    pub tool_policy_count: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct PluginLifecycleStateMachineInputs {
    pub plugin_id: &'static str,
    pub fixture: HeptaSystemPluginFixtureSummary,
    pub tool_preview_candidate_count: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct PluginLifecycleStateMachinePlan {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub plugin_id: &'static str,
    pub status: &'static str,
    pub source_abi_surface: &'static str,
    pub source_abi_ready: bool,
    pub source_loader_binding_surface: &'static str,
    pub source_loader_binding_ready: bool,
    pub source_tool_preview_surface: &'static str,
    pub source_tool_preview_candidate_count: usize,
    pub source_tool_preview_ready: bool,
    pub fixture_shape_ready: bool,
    pub fixture_policy_metadata_ready: bool,
    pub lifecycle_phase_count: usize,
    pub lifecycle_ready_phase_count: usize,
    pub lifecycle_blocked_phase_count: usize,
    pub lifecycle_state_machine_ready: bool,
    pub lifecycle_phase_summary_ready: bool,
    pub source_of_truth_ready: bool,
    pub restored_memory_state_machine: bool,
    pub restored_memory_phase_summary: bool,
    pub tool_registry_registration_enabled: bool,
    pub tool_invocation_enabled: bool,
    pub ledger_written: bool,
    pub approval_requested: bool,
    pub plugin_cache_mutated: bool,
    pub local_storage_created: bool,
    pub all_live_paths_blocked: bool,
    pub live_mutation_ready: bool,
    pub next_migration_step: &'static str,
    pub fixture: HeptaSystemPluginFixtureSummary,
    pub phases: Vec<PluginLifecyclePhaseSummary>,
}

impl PluginLifecycleStateMachineInputs {
    pub fn current_checkout_without_fixture(plugin_id: &'static str) -> Self {
        Self {
            plugin_id,
            fixture: HeptaSystemPluginFixtureSummary::default(),
            tool_preview_candidate_count: 0,
        }
    }

    pub fn synthetic_fixture(
        plugin_id: &'static str,
        fixture: HeptaSystemPluginFixtureSummary,
        tool_preview_candidate_count: usize,
    ) -> Self {
        Self {
            plugin_id,
            fixture,
            tool_preview_candidate_count,
        }
    }
}

pub fn hepta_system_plugin_lifecycle_state_machine_plan() -> PluginLifecycleStateMachinePlan {
    let abi = hepta_system_plugin_contribution_point_abi_matrix();
    let loader = hepta_system_plugin_contribution_point_loader_binding_plan();
    let inputs = hepta_system_plugin_lifecycle_state_machine_inputs(abi.plugin_id);
    plan_plugin_lifecycle_state_machine(&abi, &loader, &inputs)
}

pub fn hepta_system_plugin_lifecycle_state_machine_inputs(
    plugin_id: &'static str,
) -> PluginLifecycleStateMachineInputs {
    try_hepta_system_plugin_lifecycle_state_machine_inputs(plugin_id).unwrap_or_else(|| {
        PluginLifecycleStateMachineInputs::current_checkout_without_fixture(plugin_id)
    })
}

pub fn try_hepta_system_plugin_lifecycle_state_machine_inputs(
    plugin_id: &'static str,
) -> Option<PluginLifecycleStateMachineInputs> {
    let manifest_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join(HEPTA_SYSTEM_PLUGIN_MANIFEST_FIXTURE_RELATIVE_PATH);
    let plugin_root = manifest_path.parent()?.parent()?;
    let contents = fs::read_to_string(&manifest_path).ok()?;
    let fixture = hepta_system_fixture_summary_from_json_at(&contents, plugin_root);
    let tool_preview_candidate_count =
        usize::from(fixture.mcp_server_count > 0) + usize::from(fixture.app_count > 0);

    Some(PluginLifecycleStateMachineInputs {
        plugin_id,
        fixture,
        tool_preview_candidate_count,
    })
}

pub fn hepta_system_fixture_summary_from_json(
    manifest_json: &str,
) -> HeptaSystemPluginFixtureSummary {
    hepta_system_fixture_summary_from_json_with_resolver(manifest_json, |_| None)
}

pub fn hepta_system_fixture_summary_from_json_at(
    manifest_json: &str,
    manifest_dir: &Path,
) -> HeptaSystemPluginFixtureSummary {
    hepta_system_fixture_summary_from_json_with_resolver(manifest_json, |relative_path| {
        let path = manifest_dir.join(relative_path);
        path.exists().then_some(path)
    })
}

fn hepta_system_fixture_summary_from_json_with_resolver(
    manifest_json: &str,
    resolve_path: impl Fn(&str) -> Option<std::path::PathBuf>,
) -> HeptaSystemPluginFixtureSummary {
    let Ok(JsonValue::Object(manifest)) = serde_json::from_str::<JsonValue>(manifest_json) else {
        return HeptaSystemPluginFixtureSummary::default();
    };
    let skill_path = manifest_string_field(&manifest, "skills");
    let mcp_servers_path = manifest_string_field(&manifest, "mcpServers");
    let apps_path = manifest_string_field(&manifest, "apps");
    let skill_path_present = skill_path
        .as_deref()
        .and_then(&resolve_path)
        .is_some_and(|path| path.is_dir());
    let mcp_servers_path_present = mcp_servers_path
        .as_deref()
        .and_then(&resolve_path)
        .is_some_and(|path| path.is_file());
    let apps_path_present = apps_path
        .as_deref()
        .and_then(&resolve_path)
        .is_some_and(|path| path.is_file());
    let skill_count = skill_path
        .as_deref()
        .and_then(&resolve_path)
        .filter(|path| path.is_dir())
        .map_or_else(
            || manifest_inline_field_len(&manifest, "skills"),
            |path| count_skill_files(&path),
        );
    let mcp_server_count = mcp_servers_path
        .as_deref()
        .and_then(&resolve_path)
        .filter(|path| path.is_file())
        .and_then(|path| count_json_object_field(&path, "mcpServers"))
        .unwrap_or_else(|| manifest_inline_field_len(&manifest, "mcpServers"));
    let app_count = apps_path
        .as_deref()
        .and_then(&resolve_path)
        .filter(|path| path.is_file())
        .and_then(|path| count_json_object_field(&path, "apps"))
        .unwrap_or_else(|| manifest_inline_field_len(&manifest, "apps"));

    HeptaSystemPluginFixtureSummary {
        manifest_present: true,
        skill_path_present,
        mcp_servers_path_present,
        apps_path_present,
        skill_count,
        mcp_server_count,
        app_count,
        hook_count: manifest_inline_field_len(&manifest, "hooks"),
        tool_schema_count: manifest_inline_field_len(&manifest, "toolSchemas"),
        permission_count: manifest_inline_field_len(&manifest, "permissions"),
        activation_event_count: manifest_inline_field_len(&manifest, "activationEvents"),
        tool_policy_count: manifest_inline_field_len(&manifest, "toolPolicies"),
    }
}

pub fn plan_plugin_lifecycle_state_machine(
    abi: &PluginContributionPointAbiMatrix,
    loader: &PluginContributionPointLoaderBindingPlan,
    inputs: &PluginLifecycleStateMachineInputs,
) -> PluginLifecycleStateMachinePlan {
    let fixture_shape_ready = inputs.fixture.manifest_present
        && inputs.fixture.skill_path_present
        && inputs.fixture.mcp_servers_path_present
        && inputs.fixture.apps_path_present
        && inputs.fixture.skill_count == 1
        && inputs.fixture.mcp_server_count == 1
        && inputs.fixture.app_count == 1
        && inputs.fixture.tool_schema_count == 2
        && inputs.fixture.permission_count == 2
        && inputs.fixture.activation_event_count == 2
        && inputs.fixture.tool_policy_count == 2;
    let fixture_policy_metadata_ready = inputs.fixture.permission_count == 2
        && inputs.fixture.activation_event_count == 2
        && inputs.fixture.tool_policy_count == 2
        && abi.permission_policy_required
        && abi.activation_policy_required
        && abi.ledger_required_count == 4;
    let source_tool_preview_ready =
        inputs.tool_preview_candidate_count == 2 && loader.current_fixture_binding_ready;
    let tool_registry_registration_enabled = false;
    let tool_invocation_enabled = false;
    let ledger_written = false;
    let approval_requested = false;
    let plugin_cache_mutated = false;
    let local_storage_created = false;
    let all_live_paths_blocked = abi.all_live_paths_blocked
        && loader.all_live_paths_blocked
        && !tool_registry_registration_enabled
        && !tool_invocation_enabled
        && !ledger_written
        && !approval_requested
        && !plugin_cache_mutated
        && !local_storage_created;

    let phases = vec![
        phase_summary(
            PluginLifecyclePhase::ManifestFixtureDiscovered,
            "plugins/hepta-system/.codex-plugin/plugin.json",
            fixture_shape_ready,
            "hepta_system_fixture_shape_not_ready",
        ),
        phase_summary(
            PluginLifecyclePhase::ContributionPointAbiAudited,
            abi.surface,
            abi.abi_ready && abi.all_live_paths_blocked,
            "contribution_point_abi_not_ready",
        ),
        phase_summary(
            PluginLifecyclePhase::LoaderBindingAudited,
            loader.surface,
            loader.binding_ready && loader.all_live_paths_blocked,
            "loader_binding_not_ready",
        ),
        phase_summary(
            PluginLifecyclePhase::FixturePolicyMetadataAudited,
            "plugins/hepta-system/.codex-plugin/plugin.json",
            fixture_policy_metadata_ready,
            "fixture_policy_metadata_not_ready",
        ),
        phase_summary(
            PluginLifecyclePhase::ToolPreviewContractAudited,
            "plugin_tool_contribution_inventory_preview",
            source_tool_preview_ready,
            "tool_preview_contract_not_ready",
        ),
        phase_summary(
            PluginLifecyclePhase::LiveMutationBlocked,
            "plugin_lifecycle_state_machine",
            all_live_paths_blocked,
            "live_mutation_path_open",
        ),
    ];

    let lifecycle_ready_phase_count = ready_phase_count(&phases);
    let lifecycle_blocked_phase_count = blocked_phase_count(&phases);
    let lifecycle_phase_summary_ready =
        phases.len() == 6 && lifecycle_ready_phase_count == 6 && lifecycle_blocked_phase_count == 0;
    let lifecycle_state_machine_ready = lifecycle_phase_summary_ready
        && fixture_shape_ready
        && fixture_policy_metadata_ready
        && source_tool_preview_ready
        && all_live_paths_blocked;

    PluginLifecycleStateMachinePlan {
        runtime: "hepta",
        surface: "plugin_lifecycle_state_machine",
        plugin_id: inputs.plugin_id,
        status: if lifecycle_state_machine_ready {
            "ready"
        } else {
            "blocked"
        },
        source_abi_surface: abi.surface,
        source_abi_ready: abi.abi_ready,
        source_loader_binding_surface: loader.surface,
        source_loader_binding_ready: loader.binding_ready,
        source_tool_preview_surface: "plugin_tool_contribution_inventory_preview",
        source_tool_preview_candidate_count: inputs.tool_preview_candidate_count,
        source_tool_preview_ready,
        fixture_shape_ready,
        fixture_policy_metadata_ready,
        lifecycle_phase_count: phases.len(),
        lifecycle_ready_phase_count,
        lifecycle_blocked_phase_count,
        lifecycle_state_machine_ready,
        lifecycle_phase_summary_ready,
        source_of_truth_ready: lifecycle_state_machine_ready,
        restored_memory_state_machine: true,
        restored_memory_phase_summary: true,
        tool_registry_registration_enabled,
        tool_invocation_enabled,
        ledger_written,
        approval_requested,
        plugin_cache_mutated,
        local_storage_created,
        all_live_paths_blocked,
        live_mutation_ready: false,
        next_migration_step: "phase2_promote_tool_registry_to_read_only_dispatch_preflight_without_invocation",
        fixture: inputs.fixture.clone(),
        phases,
    }
}

fn phase_summary(
    phase: PluginLifecyclePhase,
    source_surface: &'static str,
    ready: bool,
    blocker: &'static str,
) -> PluginLifecyclePhaseSummary {
    if ready {
        PluginLifecyclePhaseSummary::ready(phase, source_surface)
    } else {
        PluginLifecyclePhaseSummary::blocked(phase, source_surface, blocker)
    }
}

fn manifest_string_field(
    manifest: &serde_json::Map<String, JsonValue>,
    field: &str,
) -> Option<String> {
    manifest
        .get(field)
        .and_then(JsonValue::as_str)
        .map(ToOwned::to_owned)
}

fn manifest_inline_field_len(manifest: &serde_json::Map<String, JsonValue>, field: &str) -> usize {
    match manifest.get(field) {
        Some(JsonValue::Array(items)) => items.len(),
        Some(JsonValue::Object(items)) => items.len(),
        Some(JsonValue::Null) | None => 0,
        Some(_) => 1,
    }
}

fn count_json_object_field(path: &Path, field: &str) -> Option<usize> {
    let contents = fs::read_to_string(path).ok()?;
    let JsonValue::Object(root) = serde_json::from_str::<JsonValue>(&contents).ok()? else {
        return None;
    };
    match root.get(field) {
        Some(JsonValue::Array(items)) => Some(items.len()),
        Some(JsonValue::Object(items)) => Some(items.len()),
        _ => None,
    }
}

fn count_skill_files(path: &Path) -> usize {
    let Ok(entries) = fs::read_dir(path) else {
        return 0;
    };

    entries
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .map(|path| {
            if path.is_dir() {
                count_skill_files(&path)
            } else if path.file_name().is_some_and(|name| name == "SKILL.md") {
                1
            } else {
                0
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contribution_point_abi::plan_hepta_system_plugin_contribution_point_abi;
    use crate::contribution_point_loader_binding::PluginContributionPointLoaderBindingInputs;
    use crate::contribution_point_loader_binding::plan_plugin_contribution_point_loader_binding;

    #[test]
    fn lifecycle_state_machine_converges_current_plugin_sources() {
        let plan = hepta_system_plugin_lifecycle_state_machine_plan();

        assert_eq!(plan.runtime, "hepta");
        assert_eq!(plan.surface, "plugin_lifecycle_state_machine");
        assert_eq!(plan.status, "ready");
        assert_eq!(plan.source_abi_surface, "plugin_contribution_point_abi");
        assert_eq!(
            plan.source_loader_binding_surface,
            "plugin_contribution_point_loader_binding"
        );
        assert!(plan.source_abi_ready);
        assert!(plan.source_loader_binding_ready);
        assert!(plan.source_tool_preview_ready);
        assert!(plan.lifecycle_state_machine_ready);
        assert!(plan.lifecycle_phase_summary_ready);
        assert!(plan.source_of_truth_ready);
        assert_eq!(plan.lifecycle_phase_count, 6);
        assert_eq!(plan.lifecycle_ready_phase_count, 6);
        assert_eq!(plan.lifecycle_blocked_phase_count, 0);
    }

    #[test]
    fn lifecycle_state_machine_reads_hepta_system_fixture_shape() {
        let inputs =
            try_hepta_system_plugin_lifecycle_state_machine_inputs("hepta-system@hepta-local")
                .expect("hepta-system manifest fixture should be present");

        assert!(inputs.fixture.manifest_present);
        assert!(inputs.fixture.skill_path_present);
        assert!(inputs.fixture.mcp_servers_path_present);
        assert!(inputs.fixture.apps_path_present);
        assert_eq!(inputs.fixture.skill_count, 1);
        assert_eq!(inputs.fixture.mcp_server_count, 1);
        assert_eq!(inputs.fixture.app_count, 1);
        assert_eq!(inputs.fixture.tool_schema_count, 2);
        assert_eq!(inputs.fixture.permission_count, 2);
        assert_eq!(inputs.fixture.activation_event_count, 2);
        assert_eq!(inputs.fixture.tool_policy_count, 2);
        assert_eq!(inputs.tool_preview_candidate_count, 2);
    }

    #[test]
    fn lifecycle_state_machine_restores_memory_surfaces_without_live_mutation() {
        let plan = hepta_system_plugin_lifecycle_state_machine_plan();

        assert!(plan.restored_memory_state_machine);
        assert!(plan.restored_memory_phase_summary);
        assert!(!plan.tool_registry_registration_enabled);
        assert!(!plan.tool_invocation_enabled);
        assert!(!plan.ledger_written);
        assert!(!plan.approval_requested);
        assert!(!plan.plugin_cache_mutated);
        assert!(!plan.local_storage_created);
        assert!(plan.all_live_paths_blocked);
        assert!(!plan.live_mutation_ready);
        assert!(plan.phases.iter().all(|phase| !phase.live_mutation_enabled));
    }

    #[test]
    fn lifecycle_state_machine_blocks_incomplete_fixture_without_enabling_live_paths() {
        let abi = plan_hepta_system_plugin_contribution_point_abi("synthetic@hepta-local");
        let loader_inputs = PluginContributionPointLoaderBindingInputs::synthetic_fixture(
            abi.plugin_id,
            vec!["skills"],
        );
        let loader = plan_plugin_contribution_point_loader_binding(&abi, &loader_inputs);
        let inputs = PluginLifecycleStateMachineInputs::synthetic_fixture(
            abi.plugin_id,
            HeptaSystemPluginFixtureSummary {
                manifest_present: true,
                skill_path_present: true,
                mcp_servers_path_present: false,
                apps_path_present: false,
                skill_count: 1,
                mcp_server_count: 0,
                app_count: 0,
                hook_count: 0,
                tool_schema_count: 0,
                permission_count: 0,
                activation_event_count: 0,
                tool_policy_count: 0,
            },
            0,
        );
        let plan = plan_plugin_lifecycle_state_machine(&abi, &loader, &inputs);

        assert_eq!(plan.status, "blocked");
        assert!(!plan.fixture_shape_ready);
        assert!(!plan.fixture_policy_metadata_ready);
        assert!(!plan.source_tool_preview_ready);
        assert!(plan.all_live_paths_blocked);
        assert!(!plan.live_mutation_ready);
        assert!(plan.lifecycle_blocked_phase_count >= 3);
    }
}

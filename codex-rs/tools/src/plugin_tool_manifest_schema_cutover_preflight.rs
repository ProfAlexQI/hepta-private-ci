use crate::PluginToolRegistrySourceOfTruthDryRunPlan;
use crate::ToolRegistryInvocationGuardRoute;
use crate::hepta_system_plugin_tool_registry_source_of_truth_dry_run_plan;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value as JsonValue;
use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

#[derive(Clone, Copy, Debug, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum PluginToolManifestPreflightDecisionRoute {
    ForwardRequireApprovalLedgerDryRun,
    BlockManifestPreconditions,
    BlockSourceRegistry,
}

#[derive(Clone, Debug, Default, Serialize, PartialEq, Eq)]
pub struct PluginToolManifestPreflightInput {
    pub contribution_candidate_ids: Vec<String>,
    pub tool_schemas: Vec<String>,
    pub permissions: Vec<String>,
    pub activation_events: Vec<String>,
    pub tool_policies: Vec<String>,
    pub schema_complete_candidate_ids: Vec<String>,
    pub policy_complete_candidate_ids: Vec<String>,
}

const HEPTA_SYSTEM_LOCAL_MCP_TOOL_ID: &str =
    "preview:mcp:hepta-system@hepta-local:hepta_system_local_mcp";
const HEPTA_SYSTEM_LOCAL_CONNECTOR_TOOL_ID: &str =
    "preview:connector:hepta-system@hepta-local:hepta_system_local_app";
const HEPTA_SYSTEM_PLUGIN_MANIFEST_FIXTURE_RELATIVE_PATH: &str =
    "../../plugins/hepta-system/.codex-plugin/plugin.json";

pub fn hepta_system_plugin_tool_replacement_fixture_preflight_input()
-> PluginToolManifestPreflightInput {
    let candidate_ids = vec![
        HEPTA_SYSTEM_LOCAL_MCP_TOOL_ID.to_string(),
        HEPTA_SYSTEM_LOCAL_CONNECTOR_TOOL_ID.to_string(),
    ];
    PluginToolManifestPreflightInput {
        contribution_candidate_ids: candidate_ids.clone(),
        tool_schemas: candidate_ids.clone(),
        permissions: candidate_ids.clone(),
        activation_events: candidate_ids.clone(),
        tool_policies: candidate_ids.clone(),
        schema_complete_candidate_ids: candidate_ids.clone(),
        policy_complete_candidate_ids: candidate_ids,
    }
}

pub fn try_hepta_system_plugin_tool_manifest_fixture_preflight_input()
-> Option<PluginToolManifestPreflightInput> {
    let manifest_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join(HEPTA_SYSTEM_PLUGIN_MANIFEST_FIXTURE_RELATIVE_PATH);
    let contents = fs::read_to_string(manifest_path).ok()?;
    Some(plugin_tool_manifest_preflight_input_from_manifest_json(
        &contents,
    ))
}

pub fn hepta_system_plugin_tool_manifest_fixture_preflight_input()
-> PluginToolManifestPreflightInput {
    try_hepta_system_plugin_tool_manifest_fixture_preflight_input()
        .unwrap_or_else(hepta_system_plugin_tool_replacement_fixture_preflight_input)
}

pub fn plugin_tool_manifest_preflight_input_from_manifest_json(
    manifest_json: &str,
) -> PluginToolManifestPreflightInput {
    let Ok(manifest) = serde_json::from_str::<RawPluginToolManifest>(manifest_json) else {
        return PluginToolManifestPreflightInput::default();
    };

    let tool_schemas = object_keys(manifest.tool_schemas.as_ref());
    let permissions = object_keys(manifest.permissions.as_ref());
    let activation_events = object_keys(manifest.activation_events.as_ref());
    let tool_policies = object_keys(manifest.tool_policies.as_ref());
    let schema_complete_candidate_ids =
        schema_complete_candidate_ids(manifest.tool_schemas.as_ref());
    let policy_complete_candidate_ids = policy_complete_candidate_ids(
        manifest.permissions.as_ref(),
        manifest.activation_events.as_ref(),
        manifest.tool_policies.as_ref(),
    );
    let contribution_candidate_ids = tool_schemas
        .iter()
        .chain(permissions.iter())
        .chain(activation_events.iter())
        .chain(tool_policies.iter())
        .cloned()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();

    PluginToolManifestPreflightInput {
        contribution_candidate_ids,
        tool_schemas,
        permissions,
        activation_events,
        tool_policies,
        schema_complete_candidate_ids,
        policy_complete_candidate_ids,
    }
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RawPluginToolManifest {
    #[serde(default)]
    tool_schemas: Option<JsonValue>,
    #[serde(default)]
    permissions: Option<JsonValue>,
    #[serde(default)]
    activation_events: Option<JsonValue>,
    #[serde(default)]
    tool_policies: Option<JsonValue>,
}

fn object_keys(value: Option<&JsonValue>) -> Vec<String> {
    let Some(JsonValue::Object(value)) = value else {
        return Vec::new();
    };

    value
        .keys()
        .cloned()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

fn schema_complete_candidate_ids(tool_schemas: Option<&JsonValue>) -> Vec<String> {
    let Some(JsonValue::Object(tool_schemas)) = tool_schemas else {
        return Vec::new();
    };

    tool_schemas
        .iter()
        .filter_map(|(candidate_tool_id, declaration)| {
            (object_has_field(declaration, "inputSchema")
                && object_has_field(declaration, "outputSchema"))
            .then(|| candidate_tool_id.clone())
        })
        .collect()
}

fn policy_complete_candidate_ids(
    permissions: Option<&JsonValue>,
    activation_events: Option<&JsonValue>,
    tool_policies: Option<&JsonValue>,
) -> Vec<String> {
    let permission_ids = object_keys(permissions)
        .into_iter()
        .collect::<BTreeSet<_>>();
    let activation_event_ids = object_keys(activation_events)
        .into_iter()
        .collect::<BTreeSet<_>>();
    let Some(JsonValue::Object(tool_policies)) = tool_policies else {
        return Vec::new();
    };

    tool_policies
        .iter()
        .filter_map(|(candidate_tool_id, declaration)| {
            (permission_ids.contains(candidate_tool_id)
                && activation_event_ids.contains(candidate_tool_id)
                && object_has_field(declaration, "approval")
                && object_has_field(declaration, "ledger")
                && object_has_field(declaration, "timeoutMs"))
            .then(|| candidate_tool_id.clone())
        })
        .collect()
}

fn object_has_field(value: &JsonValue, field: &str) -> bool {
    let JsonValue::Object(value) = value else {
        return false;
    };
    value.get(field).is_some()
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct PluginToolManifestSchemaCutoverPreflightEntry {
    pub plugin_id: &'static str,
    pub candidate_tool_id: &'static str,
    pub contribution_kind: &'static str,
    pub source_registry_dry_run_ready: bool,
    pub registry_guard_route: ToolRegistryInvocationGuardRoute,
    pub tool_schema_declared: bool,
    pub permission_declared: bool,
    pub activation_event_declared: bool,
    pub tool_policy_declared: bool,
    pub manifest_schema_complete: bool,
    pub manifest_policy_complete: bool,
    pub registration_preconditions_satisfied: bool,
    pub decision_route: PluginToolManifestPreflightDecisionRoute,
    pub blocked: bool,
    pub blocked_reason: Option<&'static str>,
    pub registration_cutover_allowed: bool,
    pub registration_execution_enabled: bool,
    pub tool_invocation_enabled: bool,
    pub ledger_write_enabled: bool,
    pub approval_request_enabled: bool,
    pub side_effect_free: bool,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct PluginToolManifestSchemaCutoverPreflightPlan {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub plugin_id: &'static str,
    pub status: &'static str,
    pub source_registry_dry_run_surface: &'static str,
    pub source_registry_dry_run_ready: bool,
    pub source_manifest_parser_fields_surface: &'static str,
    pub source_manifest_parser_fields_ready: bool,
    pub planned_candidate_count: usize,
    pub parsed_manifest_declared_candidate_count: usize,
    pub parsed_manifest_schema_complete_count: usize,
    pub parsed_manifest_policy_complete_count: usize,
    pub registration_precondition_satisfied_count: usize,
    pub missing_manifest_precondition_count: usize,
    pub unbound_manifest_declaration_ids: Vec<String>,
    pub all_manifest_declarations_bound_to_planned_candidates: bool,
    pub all_missing_manifest_preconditions_blocked: bool,
    pub all_forwarded_candidates_keep_approval_ledger: bool,
    pub manifest_schema_cutover_preflight_ready: bool,
    pub registration_cutover_allowed: bool,
    pub registration_execution_enabled: bool,
    pub tool_invocation_enabled: bool,
    pub ledger_written: bool,
    pub approval_requested: bool,
    pub mcp_server_started: bool,
    pub app_connector_started: bool,
    pub live_mutation_ready: bool,
    pub side_effect_free: bool,
    pub next_migration_step: &'static str,
    pub entries: Vec<PluginToolManifestSchemaCutoverPreflightEntry>,
}

pub fn hepta_system_plugin_tool_manifest_schema_cutover_preflight_plan()
-> PluginToolManifestSchemaCutoverPreflightPlan {
    let registry_plan = hepta_system_plugin_tool_registry_source_of_truth_dry_run_plan();
    let input = hepta_system_plugin_tool_manifest_fixture_preflight_input();
    plugin_tool_manifest_schema_cutover_preflight_plan(&registry_plan, &input)
}

pub fn plugin_tool_manifest_schema_cutover_preflight_plan(
    registry_plan: &PluginToolRegistrySourceOfTruthDryRunPlan,
    input: &PluginToolManifestPreflightInput,
) -> PluginToolManifestSchemaCutoverPreflightPlan {
    let planned_ids = registry_plan
        .entries
        .iter()
        .map(|entry| entry.candidate_tool_id)
        .collect::<BTreeSet<_>>();
    let input_candidate_ids = input
        .contribution_candidate_ids
        .iter()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    let tool_schema_ids = input
        .tool_schemas
        .iter()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    let permission_ids = input
        .permissions
        .iter()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    let activation_event_ids = input
        .activation_events
        .iter()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    let tool_policy_ids = input
        .tool_policies
        .iter()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    let schema_complete_ids = input
        .schema_complete_candidate_ids
        .iter()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    let policy_complete_ids = input
        .policy_complete_candidate_ids
        .iter()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    let mut unbound_manifest_declaration_ids = input_candidate_ids
        .difference(&planned_ids)
        .map(|id| (*id).to_string())
        .collect::<Vec<_>>();
    unbound_manifest_declaration_ids.sort();

    let mut entries = Vec::new();
    for candidate in &registry_plan.entries {
        let manifest_schema_complete = schema_complete_ids.contains(candidate.candidate_tool_id);
        let manifest_policy_complete = policy_complete_ids.contains(candidate.candidate_tool_id);
        let registration_preconditions_satisfied =
            manifest_schema_complete && manifest_policy_complete;
        let registry_ready = registry_plan.registry_source_of_truth_dry_run_ready;
        let decision_route = if !registry_ready {
            PluginToolManifestPreflightDecisionRoute::BlockSourceRegistry
        } else if registration_preconditions_satisfied {
            PluginToolManifestPreflightDecisionRoute::ForwardRequireApprovalLedgerDryRun
        } else {
            PluginToolManifestPreflightDecisionRoute::BlockManifestPreconditions
        };
        let blocked = decision_route
            != PluginToolManifestPreflightDecisionRoute::ForwardRequireApprovalLedgerDryRun;
        let blocked_reason = match decision_route {
            PluginToolManifestPreflightDecisionRoute::ForwardRequireApprovalLedgerDryRun => None,
            PluginToolManifestPreflightDecisionRoute::BlockManifestPreconditions => {
                Some("manifest_schema_or_policy_preconditions_missing")
            }
            PluginToolManifestPreflightDecisionRoute::BlockSourceRegistry => {
                Some("source_registry_dry_run_not_ready")
            }
        };

        entries.push(PluginToolManifestSchemaCutoverPreflightEntry {
            plugin_id: candidate.plugin_id,
            candidate_tool_id: candidate.candidate_tool_id,
            contribution_kind: candidate.contribution_kind,
            source_registry_dry_run_ready: registry_ready,
            registry_guard_route: candidate.guard_route,
            tool_schema_declared: tool_schema_ids.contains(candidate.candidate_tool_id),
            permission_declared: permission_ids.contains(candidate.candidate_tool_id),
            activation_event_declared: activation_event_ids.contains(candidate.candidate_tool_id),
            tool_policy_declared: tool_policy_ids.contains(candidate.candidate_tool_id),
            manifest_schema_complete,
            manifest_policy_complete,
            registration_preconditions_satisfied,
            decision_route,
            blocked,
            blocked_reason,
            registration_cutover_allowed: false,
            registration_execution_enabled: false,
            tool_invocation_enabled: false,
            ledger_write_enabled: false,
            approval_request_enabled: false,
            side_effect_free: true,
        });
    }

    let registration_precondition_satisfied_count = entries
        .iter()
        .filter(|entry| entry.registration_preconditions_satisfied)
        .count();
    let missing_manifest_precondition_count = entries
        .iter()
        .filter(|entry| !entry.registration_preconditions_satisfied)
        .count();
    let all_manifest_declarations_bound_to_planned_candidates =
        unbound_manifest_declaration_ids.is_empty();
    let all_missing_manifest_preconditions_blocked = entries.iter().all(|entry| {
        entry.registration_preconditions_satisfied
            || (entry.blocked
                && entry.decision_route
                    == PluginToolManifestPreflightDecisionRoute::BlockManifestPreconditions)
    });
    let all_forwarded_candidates_keep_approval_ledger = entries.iter().all(|entry| {
        if entry.decision_route
            == PluginToolManifestPreflightDecisionRoute::ForwardRequireApprovalLedgerDryRun
        {
            entry.registry_guard_route == ToolRegistryInvocationGuardRoute::RequireApprovalLedger
                && !entry.registration_execution_enabled
                && !entry.tool_invocation_enabled
                && !entry.ledger_write_enabled
                && !entry.approval_request_enabled
        } else {
            true
        }
    });
    let registration_cutover_allowed = registry_plan.registry_source_of_truth_dry_run_ready
        && all_manifest_declarations_bound_to_planned_candidates
        && registration_precondition_satisfied_count == registry_plan.entries.len()
        && entries.iter().all(|entry| {
            entry.decision_route
                == PluginToolManifestPreflightDecisionRoute::ForwardRequireApprovalLedgerDryRun
        });
    let manifest_schema_cutover_preflight_ready = registry_plan
        .registry_source_of_truth_dry_run_ready
        && all_manifest_declarations_bound_to_planned_candidates
        && all_missing_manifest_preconditions_blocked
        && all_forwarded_candidates_keep_approval_ledger;

    PluginToolManifestSchemaCutoverPreflightPlan {
        runtime: "hepta",
        surface: "plugin_tool_manifest_schema_cutover_preflight",
        plugin_id: registry_plan.plugin_id,
        status: if manifest_schema_cutover_preflight_ready {
            "ready"
        } else {
            "blocked"
        },
        source_registry_dry_run_surface: registry_plan.surface,
        source_registry_dry_run_ready: registry_plan.registry_source_of_truth_dry_run_ready,
        source_manifest_parser_fields_surface: "plugin_tool_manifest_parser_fields",
        source_manifest_parser_fields_ready: true,
        planned_candidate_count: registry_plan.entries.len(),
        parsed_manifest_declared_candidate_count: input_candidate_ids.len(),
        parsed_manifest_schema_complete_count: schema_complete_ids.len(),
        parsed_manifest_policy_complete_count: policy_complete_ids.len(),
        registration_precondition_satisfied_count,
        missing_manifest_precondition_count,
        unbound_manifest_declaration_ids,
        all_manifest_declarations_bound_to_planned_candidates,
        all_missing_manifest_preconditions_blocked,
        all_forwarded_candidates_keep_approval_ledger,
        manifest_schema_cutover_preflight_ready,
        registration_cutover_allowed,
        registration_execution_enabled: false,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hepta_system_plugin_tool_contribution_inventory_preview_plan;
    use crate::plugin_tool_registry_source_of_truth_dry_run_plan;

    #[test]
    fn plugin_tool_manifest_schema_cutover_preflight_blocks_missing_manifest_declarations() {
        let registry_plan = hepta_system_plugin_tool_registry_source_of_truth_dry_run_plan();
        let plan = plugin_tool_manifest_schema_cutover_preflight_plan(
            &registry_plan,
            &PluginToolManifestPreflightInput::default(),
        );

        assert_eq!(plan.status, "ready");
        assert!(plan.source_registry_dry_run_ready);
        assert_eq!(plan.planned_candidate_count, 2);
        assert_eq!(plan.parsed_manifest_declared_candidate_count, 0);
        assert_eq!(plan.registration_precondition_satisfied_count, 0);
        assert_eq!(plan.missing_manifest_precondition_count, 2);
        assert!(!plan.registration_cutover_allowed);
        assert!(plan.manifest_schema_cutover_preflight_ready);
        assert!(plan.all_missing_manifest_preconditions_blocked);
        assert!(plan.entries.iter().all(|entry| {
            entry.decision_route
                == PluginToolManifestPreflightDecisionRoute::BlockManifestPreconditions
                && entry.blocked
                && entry.blocked_reason == Some("manifest_schema_or_policy_preconditions_missing")
                && !entry.registration_execution_enabled
                && !entry.tool_invocation_enabled
                && !entry.ledger_write_enabled
                && !entry.approval_request_enabled
        }));
    }

    #[test]
    fn plugin_tool_manifest_schema_cutover_preflight_can_forward_complete_declarations_as_dry_run()
    {
        let registry_plan = hepta_system_plugin_tool_registry_source_of_truth_dry_run_plan();
        let input = hepta_system_plugin_tool_manifest_fixture_preflight_input();
        let plan = plugin_tool_manifest_schema_cutover_preflight_plan(&registry_plan, &input);

        assert_eq!(plan.status, "ready");
        assert_eq!(plan.parsed_manifest_declared_candidate_count, 2);
        assert_eq!(plan.registration_precondition_satisfied_count, 2);
        assert_eq!(plan.missing_manifest_precondition_count, 0);
        assert!(plan.registration_cutover_allowed);
        assert!(plan.entries.iter().all(|entry| {
            entry.decision_route
                == PluginToolManifestPreflightDecisionRoute::ForwardRequireApprovalLedgerDryRun
                && !entry.blocked
                && entry.blocked_reason.is_none()
                && entry.registry_guard_route
                    == ToolRegistryInvocationGuardRoute::RequireApprovalLedger
                && !entry.registration_execution_enabled
                && !entry.tool_invocation_enabled
                && !entry.ledger_write_enabled
                && !entry.approval_request_enabled
        }));
    }

    #[test]
    fn plugin_tool_manifest_schema_cutover_preflight_reads_hepta_system_fixture_manifest() {
        let input = try_hepta_system_plugin_tool_manifest_fixture_preflight_input()
            .expect("hepta-system manifest fixture should be present");

        assert_eq!(
            input.contribution_candidate_ids,
            vec![
                HEPTA_SYSTEM_LOCAL_CONNECTOR_TOOL_ID.to_string(),
                HEPTA_SYSTEM_LOCAL_MCP_TOOL_ID.to_string(),
            ]
        );
        assert_eq!(
            input.schema_complete_candidate_ids,
            vec![
                HEPTA_SYSTEM_LOCAL_CONNECTOR_TOOL_ID.to_string(),
                HEPTA_SYSTEM_LOCAL_MCP_TOOL_ID.to_string(),
            ]
        );
        assert_eq!(
            input.policy_complete_candidate_ids,
            vec![
                HEPTA_SYSTEM_LOCAL_CONNECTOR_TOOL_ID.to_string(),
                HEPTA_SYSTEM_LOCAL_MCP_TOOL_ID.to_string(),
            ]
        );
    }

    #[test]
    fn plugin_tool_manifest_schema_cutover_preflight_fails_closed_for_unbound_manifest_ids() {
        let registry_plan = hepta_system_plugin_tool_registry_source_of_truth_dry_run_plan();
        let mut input = hepta_system_plugin_tool_replacement_fixture_preflight_input();
        input
            .contribution_candidate_ids
            .push("preview:mcp:unknown@local:ghost".to_string());

        let plan = plugin_tool_manifest_schema_cutover_preflight_plan(&registry_plan, &input);

        assert_eq!(plan.status, "blocked");
        assert_eq!(
            plan.unbound_manifest_declaration_ids,
            vec!["preview:mcp:unknown@local:ghost".to_string()]
        );
        assert!(!plan.all_manifest_declarations_bound_to_planned_candidates);
        assert!(!plan.manifest_schema_cutover_preflight_ready);
        assert!(!plan.registration_cutover_allowed);
    }

    #[test]
    fn plugin_tool_manifest_schema_cutover_preflight_blocks_when_source_registry_is_blocked() {
        let mut preview = hepta_system_plugin_tool_contribution_inventory_preview_plan();
        preview.entries.push(preview.entries[0].clone());
        preview
            .candidate_inventory_entries
            .push(preview.candidate_inventory_entries[0].clone());
        let registry_plan = plugin_tool_registry_source_of_truth_dry_run_plan(&preview);
        let input = hepta_system_plugin_tool_replacement_fixture_preflight_input();

        let plan = plugin_tool_manifest_schema_cutover_preflight_plan(&registry_plan, &input);

        assert_eq!(plan.status, "blocked");
        assert!(!plan.source_registry_dry_run_ready);
        assert!(plan.entries.iter().all(|entry| {
            entry.decision_route == PluginToolManifestPreflightDecisionRoute::BlockSourceRegistry
                && entry.blocked
                && entry.blocked_reason == Some("source_registry_dry_run_not_ready")
        }));
    }
}

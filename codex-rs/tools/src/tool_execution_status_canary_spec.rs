use serde::Serialize;

use crate::ToolExecutionDispatchShadowPlan;
use crate::ToolExecutionDispatchShadowRoute;
use crate::ToolRegistryInvocationGuardRoute;
use crate::hepta_system_tool_execution_dispatch_shadow_plan;

pub const HEPTA_SYSTEM_STATUS_CANARY_TOOL_ID: &str =
    "preview:mcp:hepta-system@hepta-local:hepta_system_local_mcp";
pub const HEPTA_SYSTEM_STATUS_CANARY_NON_SELECTED_TOOL_ID: &str =
    "preview:connector:hepta-system@hepta-local:hepta_system_local_app";

#[derive(Clone, Copy, Debug, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum ToolExecutionStatusCanarySpecRoute {
    ReadOnlyStatusCanarySpecReady,
    PreflightOnlyNonSelectedCandidate,
    BlockedByDispatchShadow,
    BlockedByMissingCanaryScope,
    BlockedByMissingRollbackOrKillSwitch,
    BlockedByMissingReceiptOrReadback,
    BlockedByOperatorAcceptance,
    BlockedByCanarySwitch,
    BlockedByPrematureCanaryMutation,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct ToolExecutionStatusCanarySpecInput {
    pub canary_scope_declared: bool,
    pub canary_budget_declared: bool,
    pub rollback_anchor_declared: bool,
    pub kill_switch_declared: bool,
    pub readback_channel_declared: bool,
    pub result_receipt_schema_declared: bool,
    pub operator_acceptance_present: bool,
    pub canary_switch_enabled: bool,
    pub canary_execution_started: bool,
    pub ledger_persistence_enabled: bool,
    pub result_receipt_persistence_enabled: bool,
    pub rollback_executed: bool,
}

impl Default for ToolExecutionStatusCanarySpecInput {
    fn default() -> Self {
        Self {
            canary_scope_declared: true,
            canary_budget_declared: true,
            rollback_anchor_declared: true,
            kill_switch_declared: true,
            readback_channel_declared: true,
            result_receipt_schema_declared: true,
            operator_acceptance_present: false,
            canary_switch_enabled: false,
            canary_execution_started: false,
            ledger_persistence_enabled: false,
            result_receipt_persistence_enabled: false,
            rollback_executed: false,
        }
    }
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct ToolExecutionStatusCanarySpecEntry {
    pub plugin_id: &'static str,
    pub candidate_tool_id: &'static str,
    pub contribution_kind: &'static str,
    pub selected_for_status_canary: bool,
    pub source_dispatch_shadow_route: ToolExecutionDispatchShadowRoute,
    pub source_dispatch_shadow_ready: bool,
    pub source_registry_guard_route: ToolRegistryInvocationGuardRoute,
    pub source_registry_shadow_pipeline_ready: bool,
    pub source_registry_shadow_receipt_rehearsed: bool,
    pub canary_spec_route: ToolExecutionStatusCanarySpecRoute,
    pub canary_spec_ready: bool,
    pub canary_start_blocked: bool,
    pub canary_scope_declared: bool,
    pub canary_budget_declared: bool,
    pub rollback_anchor_declared: bool,
    pub kill_switch_declared: bool,
    pub readback_channel_declared: bool,
    pub result_receipt_schema_declared: bool,
    pub operator_acceptance_present: bool,
    pub canary_switch_enabled: bool,
    pub canary_execution_started: bool,
    pub registry_lookup_executed: bool,
    pub tool_registered_live: bool,
    pub execution_adapter_dispatched: bool,
    pub tool_invoked: bool,
    pub ledger_persisted: bool,
    pub result_receipt_persisted: bool,
    pub rollback_executed: bool,
    pub side_effect_free: bool,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct ToolExecutionStatusCanarySpecPlan {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub plugin_id: &'static str,
    pub status: &'static str,
    pub canary_tool_id: &'static str,
    pub source_dispatch_shadow_surface: &'static str,
    pub source_dispatch_shadow_ready: bool,
    pub source_registry_shadow_pipeline_ready: bool,
    pub canary_scope_declared: bool,
    pub canary_budget_declared: bool,
    pub rollback_anchor_declared: bool,
    pub kill_switch_declared: bool,
    pub readback_channel_declared: bool,
    pub result_receipt_schema_declared: bool,
    pub operator_acceptance_present: bool,
    pub canary_switch_enabled: bool,
    pub canary_execution_started: bool,
    pub candidate_count: usize,
    pub selected_candidate_count: usize,
    pub non_selected_preflight_only_count: usize,
    pub canary_spec_ready_count: usize,
    pub canary_spec_blocked_count: usize,
    pub rollback_anchor_count: usize,
    pub kill_switch_count: usize,
    pub readback_receipt_schema_count: usize,
    pub canary_start_blocked_count: usize,
    pub all_candidates_bound_to_dispatch_shadow: bool,
    pub all_non_selected_candidates_kept_preflight_only: bool,
    pub all_live_mutations_closed: bool,
    pub status_canary_spec_ready: bool,
    pub status_canary_start_allowed: bool,
    pub registry_lookup_executed: bool,
    pub tool_registered_live: bool,
    pub execution_adapter_dispatched: bool,
    pub tool_invoked: bool,
    pub ledger_persisted: bool,
    pub result_receipt_persisted: bool,
    pub rollback_executed: bool,
    pub live_mutation_ready: bool,
    pub side_effect_free: bool,
    pub next_step: &'static str,
    pub entries: Vec<ToolExecutionStatusCanarySpecEntry>,
}

pub fn hepta_system_tool_execution_status_canary_spec_plan() -> ToolExecutionStatusCanarySpecPlan {
    let dispatch = hepta_system_tool_execution_dispatch_shadow_plan();
    tool_execution_status_canary_spec_plan(
        &dispatch,
        &ToolExecutionStatusCanarySpecInput::default(),
    )
}

pub fn tool_execution_status_canary_spec_plan(
    dispatch: &ToolExecutionDispatchShadowPlan,
    input: &ToolExecutionStatusCanarySpecInput,
) -> ToolExecutionStatusCanarySpecPlan {
    let entries = dispatch
        .entries
        .iter()
        .map(|entry| {
            let selected = entry.candidate_tool_id == HEPTA_SYSTEM_STATUS_CANARY_TOOL_ID;
            let route = status_canary_route(
                selected,
                entry.dispatch_shadow_ready,
                entry.dispatch_shadow_route,
                input,
            );
            let canary_spec_ready = matches!(
                route,
                ToolExecutionStatusCanarySpecRoute::ReadOnlyStatusCanarySpecReady
                    | ToolExecutionStatusCanarySpecRoute::PreflightOnlyNonSelectedCandidate
            );
            let canary_start_blocked = selected
                && canary_spec_ready
                && !input.operator_acceptance_present
                && !input.canary_switch_enabled
                && !input.canary_execution_started;

            ToolExecutionStatusCanarySpecEntry {
                plugin_id: entry.plugin_id,
                candidate_tool_id: entry.candidate_tool_id,
                contribution_kind: entry.contribution_kind,
                selected_for_status_canary: selected,
                source_dispatch_shadow_route: entry.dispatch_shadow_route,
                source_dispatch_shadow_ready: entry.dispatch_shadow_ready,
                source_registry_guard_route: entry.registry_guard_route,
                source_registry_shadow_pipeline_ready: entry.registry_shadow_pipeline_ready,
                source_registry_shadow_receipt_rehearsed: entry.registry_shadow_receipt_rehearsed,
                canary_spec_route: route,
                canary_spec_ready,
                canary_start_blocked,
                canary_scope_declared: input.canary_scope_declared,
                canary_budget_declared: input.canary_budget_declared,
                rollback_anchor_declared: input.rollback_anchor_declared,
                kill_switch_declared: input.kill_switch_declared,
                readback_channel_declared: input.readback_channel_declared,
                result_receipt_schema_declared: input.result_receipt_schema_declared,
                operator_acceptance_present: input.operator_acceptance_present,
                canary_switch_enabled: input.canary_switch_enabled,
                canary_execution_started: input.canary_execution_started,
                registry_lookup_executed: false,
                tool_registered_live: false,
                execution_adapter_dispatched: false,
                tool_invoked: false,
                ledger_persisted: input.ledger_persistence_enabled,
                result_receipt_persisted: input.result_receipt_persistence_enabled,
                rollback_executed: input.rollback_executed,
                side_effect_free: !input.ledger_persistence_enabled
                    && !input.result_receipt_persistence_enabled
                    && !input.rollback_executed,
            }
        })
        .collect::<Vec<_>>();

    let selected_candidate_count = entries
        .iter()
        .filter(|entry| entry.selected_for_status_canary)
        .count();
    let non_selected_preflight_only_count = entries
        .iter()
        .filter(|entry| {
            !entry.selected_for_status_canary
                && entry.canary_spec_route
                    == ToolExecutionStatusCanarySpecRoute::PreflightOnlyNonSelectedCandidate
        })
        .count();
    let canary_spec_ready_count = entries
        .iter()
        .filter(|entry| entry.canary_spec_ready)
        .count();
    let canary_spec_blocked_count = entries.len() - canary_spec_ready_count;
    let rollback_anchor_count = entries
        .iter()
        .filter(|entry| entry.selected_for_status_canary && entry.rollback_anchor_declared)
        .count();
    let kill_switch_count = entries
        .iter()
        .filter(|entry| entry.selected_for_status_canary && entry.kill_switch_declared)
        .count();
    let readback_receipt_schema_count = entries
        .iter()
        .filter(|entry| {
            entry.selected_for_status_canary
                && entry.readback_channel_declared
                && entry.result_receipt_schema_declared
        })
        .count();
    let canary_start_blocked_count = entries
        .iter()
        .filter(|entry| entry.canary_start_blocked)
        .count();
    let all_candidates_bound_to_dispatch_shadow = dispatch.tool_execution_dispatch_shadow_ready
        && entries.iter().all(|entry| {
            entry.source_dispatch_shadow_ready && entry.source_registry_shadow_pipeline_ready
        });
    let all_non_selected_candidates_kept_preflight_only =
        selected_candidate_count == 1 && non_selected_preflight_only_count + 1 == entries.len();
    let all_live_mutations_closed = entries.iter().all(|entry| {
        !entry.registry_lookup_executed
            && !entry.tool_registered_live
            && !entry.execution_adapter_dispatched
            && !entry.tool_invoked
            && !entry.ledger_persisted
            && !entry.result_receipt_persisted
            && !entry.rollback_executed
            && entry.side_effect_free
    });
    let status_canary_spec_ready = dispatch.tool_execution_dispatch_shadow_ready
        && dispatch.source_registry_shadow_pipeline_ready
        && selected_candidate_count == 1
        && canary_spec_ready_count == entries.len()
        && canary_spec_blocked_count == 0
        && rollback_anchor_count == 1
        && kill_switch_count == 1
        && readback_receipt_schema_count == 1
        && canary_start_blocked_count == 1
        && all_candidates_bound_to_dispatch_shadow
        && all_non_selected_candidates_kept_preflight_only
        && all_live_mutations_closed;

    ToolExecutionStatusCanarySpecPlan {
        runtime: "hepta",
        surface: "tool_execution_status_canary_spec",
        plugin_id: dispatch.plugin_id,
        status: if status_canary_spec_ready {
            "ready"
        } else {
            "blocked"
        },
        canary_tool_id: HEPTA_SYSTEM_STATUS_CANARY_TOOL_ID,
        source_dispatch_shadow_surface: dispatch.surface,
        source_dispatch_shadow_ready: dispatch.tool_execution_dispatch_shadow_ready,
        source_registry_shadow_pipeline_ready: dispatch.source_registry_shadow_pipeline_ready,
        canary_scope_declared: input.canary_scope_declared,
        canary_budget_declared: input.canary_budget_declared,
        rollback_anchor_declared: input.rollback_anchor_declared,
        kill_switch_declared: input.kill_switch_declared,
        readback_channel_declared: input.readback_channel_declared,
        result_receipt_schema_declared: input.result_receipt_schema_declared,
        operator_acceptance_present: input.operator_acceptance_present,
        canary_switch_enabled: input.canary_switch_enabled,
        canary_execution_started: input.canary_execution_started,
        candidate_count: entries.len(),
        selected_candidate_count,
        non_selected_preflight_only_count,
        canary_spec_ready_count,
        canary_spec_blocked_count,
        rollback_anchor_count,
        kill_switch_count,
        readback_receipt_schema_count,
        canary_start_blocked_count,
        all_candidates_bound_to_dispatch_shadow,
        all_non_selected_candidates_kept_preflight_only,
        all_live_mutations_closed,
        status_canary_spec_ready,
        status_canary_start_allowed: false,
        registry_lookup_executed: false,
        tool_registered_live: false,
        execution_adapter_dispatched: false,
        tool_invoked: false,
        ledger_persisted: false,
        result_receipt_persisted: false,
        rollback_executed: false,
        live_mutation_ready: false,
        side_effect_free: all_live_mutations_closed,
        next_step: "operator_evidence_packet_for_status_canary_without_starting_canary",
        entries,
    }
}

fn status_canary_route(
    selected: bool,
    dispatch_shadow_ready: bool,
    dispatch_shadow_route: ToolExecutionDispatchShadowRoute,
    input: &ToolExecutionStatusCanarySpecInput,
) -> ToolExecutionStatusCanarySpecRoute {
    if !dispatch_shadow_ready
        || dispatch_shadow_route
            != ToolExecutionDispatchShadowRoute::DisabledExecutionDispatchShadow
    {
        ToolExecutionStatusCanarySpecRoute::BlockedByDispatchShadow
    } else if !input.canary_scope_declared || !input.canary_budget_declared {
        ToolExecutionStatusCanarySpecRoute::BlockedByMissingCanaryScope
    } else if !input.rollback_anchor_declared || !input.kill_switch_declared {
        ToolExecutionStatusCanarySpecRoute::BlockedByMissingRollbackOrKillSwitch
    } else if !input.readback_channel_declared || !input.result_receipt_schema_declared {
        ToolExecutionStatusCanarySpecRoute::BlockedByMissingReceiptOrReadback
    } else if input.operator_acceptance_present {
        ToolExecutionStatusCanarySpecRoute::BlockedByOperatorAcceptance
    } else if input.canary_switch_enabled {
        ToolExecutionStatusCanarySpecRoute::BlockedByCanarySwitch
    } else if input.canary_execution_started
        || input.ledger_persistence_enabled
        || input.result_receipt_persistence_enabled
        || input.rollback_executed
    {
        ToolExecutionStatusCanarySpecRoute::BlockedByPrematureCanaryMutation
    } else if selected {
        ToolExecutionStatusCanarySpecRoute::ReadOnlyStatusCanarySpecReady
    } else {
        ToolExecutionStatusCanarySpecRoute::PreflightOnlyNonSelectedCandidate
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hepta_system_plugin_tool_contribution_inventory_preview_plan;
    use crate::hepta_system_tool_execution_adapter_preflight_plan;
    use crate::hepta_system_tool_registry_read_only_dispatch_preflight_plan;
    use crate::tool_execution_dispatch_shadow_plan_with_registry_shadow_pipeline;
    use crate::tool_registry_shadow_pipeline_plan;

    #[test]
    fn status_canary_spec_selects_one_read_only_status_candidate_without_starting_canary() {
        let plan = hepta_system_tool_execution_status_canary_spec_plan();

        assert_eq!(plan.status, "ready");
        assert!(plan.status_canary_spec_ready);
        assert!(!plan.status_canary_start_allowed);
        assert_eq!(plan.canary_tool_id, HEPTA_SYSTEM_STATUS_CANARY_TOOL_ID);
        assert!(plan.source_dispatch_shadow_ready);
        assert!(plan.source_registry_shadow_pipeline_ready);
        assert_eq!(plan.candidate_count, 2);
        assert_eq!(plan.selected_candidate_count, 1);
        assert_eq!(plan.non_selected_preflight_only_count, 1);
        assert_eq!(plan.canary_spec_ready_count, 2);
        assert_eq!(plan.canary_spec_blocked_count, 0);
        assert_eq!(plan.rollback_anchor_count, 1);
        assert_eq!(plan.kill_switch_count, 1);
        assert_eq!(plan.readback_receipt_schema_count, 1);
        assert_eq!(plan.canary_start_blocked_count, 1);
        assert!(plan.all_candidates_bound_to_dispatch_shadow);
        assert!(plan.all_non_selected_candidates_kept_preflight_only);
        assert!(plan.all_live_mutations_closed);
        assert!(!plan.registry_lookup_executed);
        assert!(!plan.tool_registered_live);
        assert!(!plan.execution_adapter_dispatched);
        assert!(!plan.tool_invoked);
        assert!(!plan.ledger_persisted);
        assert!(!plan.result_receipt_persisted);
        assert!(!plan.live_mutation_ready);

        let selected = plan
            .entries
            .iter()
            .find(|entry| entry.selected_for_status_canary)
            .expect("selected status canary entry");
        assert_eq!(
            selected.candidate_tool_id,
            HEPTA_SYSTEM_STATUS_CANARY_TOOL_ID
        );
        assert_eq!(
            selected.canary_spec_route,
            ToolExecutionStatusCanarySpecRoute::ReadOnlyStatusCanarySpecReady
        );
        assert!(selected.canary_start_blocked);
        assert!(!selected.tool_invoked);

        let non_selected = plan
            .entries
            .iter()
            .find(|entry| !entry.selected_for_status_canary)
            .expect("non-selected connector entry");
        assert_eq!(
            non_selected.candidate_tool_id,
            HEPTA_SYSTEM_STATUS_CANARY_NON_SELECTED_TOOL_ID
        );
        assert_eq!(
            non_selected.canary_spec_route,
            ToolExecutionStatusCanarySpecRoute::PreflightOnlyNonSelectedCandidate
        );
    }

    #[test]
    fn status_canary_spec_fails_closed_when_dispatch_shadow_blocks() {
        let mut contributions = hepta_system_plugin_tool_contribution_inventory_preview_plan();
        let duplicate = contributions.candidate_inventory_entries[0].clone();
        contributions.candidate_inventory_entries.push(duplicate);
        let dispatch_preflight = hepta_system_tool_registry_read_only_dispatch_preflight_plan();
        let pipeline = tool_registry_shadow_pipeline_plan(&contributions, &dispatch_preflight);
        let adapter = hepta_system_tool_execution_adapter_preflight_plan();
        let dispatch = tool_execution_dispatch_shadow_plan_with_registry_shadow_pipeline(
            &adapter,
            &pipeline,
            &Default::default(),
        );

        let plan = tool_execution_status_canary_spec_plan(&dispatch, &Default::default());

        assert_eq!(pipeline.status, "blocked");
        assert_eq!(dispatch.status, "blocked");
        assert_eq!(plan.status, "blocked");
        assert!(!plan.status_canary_spec_ready);
        assert!(!plan.source_dispatch_shadow_ready);
        assert_eq!(plan.canary_spec_ready_count, 0);
        assert_eq!(plan.canary_spec_blocked_count, 2);
        assert!(plan.entries.iter().all(|entry| {
            entry.canary_spec_route == ToolExecutionStatusCanarySpecRoute::BlockedByDispatchShadow
                && !entry.canary_spec_ready
                && !entry.tool_invoked
                && !entry.ledger_persisted
                && !entry.result_receipt_persisted
        }));
    }

    #[test]
    fn status_canary_spec_blocks_operator_acceptance_switch_and_premature_mutation() {
        let dispatch = hepta_system_tool_execution_dispatch_shadow_plan();
        for input in [
            ToolExecutionStatusCanarySpecInput {
                operator_acceptance_present: true,
                ..Default::default()
            },
            ToolExecutionStatusCanarySpecInput {
                canary_switch_enabled: true,
                ..Default::default()
            },
            ToolExecutionStatusCanarySpecInput {
                canary_execution_started: true,
                ..Default::default()
            },
            ToolExecutionStatusCanarySpecInput {
                ledger_persistence_enabled: true,
                ..Default::default()
            },
        ] {
            let plan = tool_execution_status_canary_spec_plan(&dispatch, &input);

            assert_eq!(plan.status, "blocked");
            assert!(!plan.status_canary_spec_ready);
            assert!(!plan.status_canary_start_allowed);
            assert!(!plan.tool_invoked);
            assert!(!plan.result_receipt_persisted);
            assert!(plan.entries.iter().all(|entry| !entry.tool_invoked));
        }
    }
}

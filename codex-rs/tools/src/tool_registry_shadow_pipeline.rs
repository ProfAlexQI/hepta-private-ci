use serde::Serialize;

use crate::InMemoryToolRegistryShadow;
use crate::PluginToolContributionInventoryPreviewPlan;
use crate::ShadowLookupRoute;
use crate::ShadowRegistrationOutcome;
use crate::ShadowRegistrationRoute;
use crate::ToolInvocationLedgerRehearsalRoute;
use crate::ToolRegistryReadOnlyDispatchPreflightPlan;
use crate::hepta_system_plugin_tool_contribution_inventory_preview_plan;
use crate::hepta_system_tool_registry_read_only_dispatch_preflight_plan;

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct ToolRegistryShadowPipelineEntry {
    pub plugin_id: &'static str,
    pub candidate_tool_id: String,
    pub contribution_kind: &'static str,
    pub dispatch_preflight_ready: bool,
    pub registration_route: Option<ShadowRegistrationRoute>,
    pub shadow_lookup_route: ShadowLookupRoute,
    pub ledger_rehearsal_attempted: bool,
    pub ledger_rehearsal_route: Option<ToolInvocationLedgerRehearsalRoute>,
    pub receipt_id: Option<String>,
    pub approval_required: bool,
    pub registry_registered_live: bool,
    pub registry_lookup_executed_live: bool,
    pub tool_invoked: bool,
    pub ledger_persisted: bool,
    pub receipt_persisted: bool,
    pub side_effect_free: bool,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct ToolRegistryShadowPipelinePlan {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub plugin_id: &'static str,
    pub status: &'static str,
    pub source_contribution_surface: &'static str,
    pub source_contribution_ready: bool,
    pub source_dispatch_preflight_surface: &'static str,
    pub source_dispatch_preflight_ready: bool,
    pub contribution_candidate_count: usize,
    pub dispatch_candidate_count: usize,
    pub registration_outcome_count: usize,
    pub registered_count: usize,
    pub idempotent_replay_count: usize,
    pub duplicate_registration_count: usize,
    pub rejected_missing_ledger_count: usize,
    pub dispatch_preflight_ready_count: usize,
    pub lookup_ready_read_only_count: usize,
    pub lookup_requires_approval_ledger_count: usize,
    pub lookup_blocked_count: usize,
    pub ledger_rehearsal_attempted_count: usize,
    pub ledger_rehearsal_receipt_count: usize,
    pub ledger_rehearsal_blocked_count: usize,
    pub ledger_rehearsal_skipped_count: usize,
    pub all_candidates_registered_once: bool,
    pub all_dispatch_entries_resolved_through_shadow_registry: bool,
    pub all_ledger_rehearsals_recorded_in_memory: bool,
    pub all_live_mutations_closed: bool,
    pub shadow_pipeline_ready: bool,
    pub registry_registered_live: bool,
    pub registry_lookup_executed_live: bool,
    pub tool_invoked: bool,
    pub ledger_persisted: bool,
    pub receipt_persisted: bool,
    pub side_effect_free: bool,
    pub registration_outcomes: Vec<ShadowRegistrationOutcome>,
    pub entries: Vec<ToolRegistryShadowPipelineEntry>,
}

pub fn hepta_system_tool_registry_shadow_pipeline_plan() -> ToolRegistryShadowPipelinePlan {
    let contributions = hepta_system_plugin_tool_contribution_inventory_preview_plan();
    let dispatch = hepta_system_tool_registry_read_only_dispatch_preflight_plan();
    tool_registry_shadow_pipeline_plan(&contributions, &dispatch)
}

pub fn tool_registry_shadow_pipeline_plan(
    contributions: &PluginToolContributionInventoryPreviewPlan,
    dispatch: &ToolRegistryReadOnlyDispatchPreflightPlan,
) -> ToolRegistryShadowPipelinePlan {
    let mut registry = InMemoryToolRegistryShadow::new();
    let registration_outcomes = registry.register_plugin_contribution_preview_plan(contributions);

    let entries = dispatch
        .entries
        .iter()
        .map(|dispatch_entry| {
            let registration_route = registration_outcomes
                .iter()
                .rev()
                .find(|outcome| outcome.tool_id == dispatch_entry.candidate_tool_id)
                .map(|outcome| outcome.route);
            let lookup = registry.lookup(dispatch_entry.candidate_tool_id);
            let ledger_outcome = if dispatch_entry.dispatch_preflight_ready {
                Some(registry.rehearse_ledger_write(
                    dispatch_entry.candidate_tool_id,
                    format!(
                        "shadow-pipeline:{}:{}",
                        dispatch_entry.plugin_id, dispatch_entry.candidate_tool_id
                    ),
                ))
            } else {
                None
            };
            let receipt_id = ledger_outcome
                .as_ref()
                .and_then(|outcome| outcome.receipt.as_ref())
                .map(|receipt| receipt.receipt_id.clone());
            let ledger_rehearsal_route = ledger_outcome.as_ref().map(|outcome| outcome.route);
            let ledger_rehearsal_attempted = ledger_outcome.is_some();
            let ledger_persisted = ledger_outcome
                .as_ref()
                .is_some_and(|outcome| outcome.ledger_persisted);
            let receipt_persisted = ledger_outcome
                .as_ref()
                .is_some_and(|outcome| outcome.receipt_persisted);
            let tool_invoked = lookup.tool_invoked
                || ledger_outcome
                    .as_ref()
                    .is_some_and(|outcome| outcome.tool_invoked);
            let side_effect_free = lookup.side_effect_free
                && ledger_outcome
                    .as_ref()
                    .is_none_or(|outcome| outcome.side_effect_free);

            ToolRegistryShadowPipelineEntry {
                plugin_id: dispatch_entry.plugin_id,
                candidate_tool_id: dispatch_entry.candidate_tool_id.to_string(),
                contribution_kind: dispatch_entry.contribution_kind,
                dispatch_preflight_ready: dispatch_entry.dispatch_preflight_ready,
                registration_route,
                shadow_lookup_route: lookup.route,
                ledger_rehearsal_attempted,
                ledger_rehearsal_route,
                receipt_id,
                approval_required: lookup.approval_required,
                registry_registered_live: false,
                registry_lookup_executed_live: lookup.registry_lookup_executed_live,
                tool_invoked,
                ledger_persisted,
                receipt_persisted,
                side_effect_free,
            }
        })
        .collect::<Vec<_>>();

    let registered_count = registration_outcomes
        .iter()
        .filter(|outcome| outcome.route == ShadowRegistrationRoute::Registered)
        .count();
    let idempotent_replay_count = registration_outcomes
        .iter()
        .filter(|outcome| outcome.route == ShadowRegistrationRoute::IdempotentReplay)
        .count();
    let duplicate_registration_count = registration_outcomes
        .iter()
        .filter(|outcome| outcome.route == ShadowRegistrationRoute::DuplicateToolId)
        .count();
    let rejected_missing_ledger_count = registration_outcomes
        .iter()
        .filter(|outcome| outcome.route == ShadowRegistrationRoute::RejectedMissingLedger)
        .count();
    let dispatch_preflight_ready_count = entries
        .iter()
        .filter(|entry| entry.dispatch_preflight_ready)
        .count();
    let lookup_ready_read_only_count = entries
        .iter()
        .filter(|entry| entry.shadow_lookup_route == ShadowLookupRoute::ReadyReadOnly)
        .count();
    let lookup_requires_approval_ledger_count = entries
        .iter()
        .filter(|entry| entry.shadow_lookup_route == ShadowLookupRoute::RequiresApprovalLedger)
        .count();
    let lookup_blocked_count = entries
        .iter()
        .filter(|entry| lookup_blocked(entry.shadow_lookup_route))
        .count();
    let ledger_rehearsal_attempted_count = entries
        .iter()
        .filter(|entry| entry.ledger_rehearsal_attempted)
        .count();
    let ledger_rehearsal_receipt_count = entries
        .iter()
        .filter(|entry| {
            matches!(
                entry.ledger_rehearsal_route,
                Some(ToolInvocationLedgerRehearsalRoute::RehearsedReadOnlyReceipt)
                    | Some(ToolInvocationLedgerRehearsalRoute::RehearsedApprovalRequiredReceipt)
                    | Some(ToolInvocationLedgerRehearsalRoute::IdempotentReplay)
            ) && entry.receipt_id.is_some()
        })
        .count();
    let ledger_rehearsal_blocked_count = entries
        .iter()
        .filter(|entry| {
            matches!(
                entry.ledger_rehearsal_route,
                Some(ToolInvocationLedgerRehearsalRoute::BlockedMissingTool)
                    | Some(ToolInvocationLedgerRehearsalRoute::BlockedDuplicateToolId)
                    | Some(ToolInvocationLedgerRehearsalRoute::BlockedMissingLedger)
            )
        })
        .count();
    let ledger_rehearsal_skipped_count = entries
        .iter()
        .filter(|entry| !entry.ledger_rehearsal_attempted)
        .count();

    let all_candidates_registered_once = contributions.candidate_inventory_entries.len()
        == dispatch.entries.len()
        && registration_outcomes.len() == dispatch.entries.len()
        && registered_count == dispatch.entries.len()
        && idempotent_replay_count == 0
        && duplicate_registration_count == 0
        && rejected_missing_ledger_count == 0;
    let all_dispatch_entries_resolved_through_shadow_registry = entries.iter().all(|entry| {
        entry.dispatch_preflight_ready
            && matches!(
                entry.shadow_lookup_route,
                ShadowLookupRoute::ReadyReadOnly | ShadowLookupRoute::RequiresApprovalLedger
            )
            && entry.ledger_rehearsal_attempted
    });
    let all_ledger_rehearsals_recorded_in_memory = ledger_rehearsal_receipt_count
        == dispatch.entries.len()
        && ledger_rehearsal_blocked_count == 0
        && ledger_rehearsal_skipped_count == 0;
    let all_live_mutations_closed = entries.iter().all(|entry| {
        !entry.registry_registered_live
            && !entry.registry_lookup_executed_live
            && !entry.tool_invoked
            && !entry.ledger_persisted
            && !entry.receipt_persisted
            && entry.side_effect_free
    });
    let shadow_pipeline_ready = contributions.preview_ready
        && dispatch.read_only_dispatch_preflight_ready
        && all_candidates_registered_once
        && all_dispatch_entries_resolved_through_shadow_registry
        && all_ledger_rehearsals_recorded_in_memory
        && all_live_mutations_closed;

    ToolRegistryShadowPipelinePlan {
        runtime: "hepta",
        surface: "tool_registry_shadow_pipeline",
        plugin_id: dispatch.plugin_id,
        status: if shadow_pipeline_ready {
            "ready"
        } else {
            "blocked"
        },
        source_contribution_surface: contributions.surface,
        source_contribution_ready: contributions.preview_ready,
        source_dispatch_preflight_surface: dispatch.surface,
        source_dispatch_preflight_ready: dispatch.read_only_dispatch_preflight_ready,
        contribution_candidate_count: contributions.candidate_inventory_entries.len(),
        dispatch_candidate_count: dispatch.entries.len(),
        registration_outcome_count: registration_outcomes.len(),
        registered_count,
        idempotent_replay_count,
        duplicate_registration_count,
        rejected_missing_ledger_count,
        dispatch_preflight_ready_count,
        lookup_ready_read_only_count,
        lookup_requires_approval_ledger_count,
        lookup_blocked_count,
        ledger_rehearsal_attempted_count,
        ledger_rehearsal_receipt_count,
        ledger_rehearsal_blocked_count,
        ledger_rehearsal_skipped_count,
        all_candidates_registered_once,
        all_dispatch_entries_resolved_through_shadow_registry,
        all_ledger_rehearsals_recorded_in_memory,
        all_live_mutations_closed,
        shadow_pipeline_ready,
        registry_registered_live: false,
        registry_lookup_executed_live: false,
        tool_invoked: false,
        ledger_persisted: false,
        receipt_persisted: false,
        side_effect_free: all_live_mutations_closed,
        registration_outcomes,
        entries,
    }
}

fn lookup_blocked(route: ShadowLookupRoute) -> bool {
    matches!(
        route,
        ShadowLookupRoute::BlockedMissingLedger
            | ShadowLookupRoute::BlockedDuplicateToolId
            | ShadowLookupRoute::MissingTool
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ToolRegistryReadOnlyDispatchPreflightInput;
    use crate::hepta_system_tool_invocation_ledger_approval_preflight_plan;
    use crate::hepta_system_tool_invocation_receipt_projection_plan;
    use crate::hepta_system_tool_registry_invocation_source_of_truth_plan;
    use crate::hepta_system_tool_registry_router_lookup_shadow_plan;
    use crate::tool_registry_read_only_dispatch_preflight_plan;

    #[test]
    fn hepta_system_shadow_pipeline_runs_registry_lookup_and_ledger_rehearsal() {
        let plan = hepta_system_tool_registry_shadow_pipeline_plan();

        assert_eq!(plan.status, "ready");
        assert!(plan.shadow_pipeline_ready);
        assert_eq!(plan.contribution_candidate_count, 2);
        assert_eq!(plan.dispatch_candidate_count, 2);
        assert_eq!(plan.registration_outcome_count, 2);
        assert_eq!(plan.registered_count, 2);
        assert_eq!(plan.duplicate_registration_count, 0);
        assert_eq!(plan.rejected_missing_ledger_count, 0);
        assert_eq!(plan.dispatch_preflight_ready_count, 2);
        assert_eq!(plan.lookup_requires_approval_ledger_count, 2);
        assert_eq!(plan.lookup_ready_read_only_count, 0);
        assert_eq!(plan.lookup_blocked_count, 0);
        assert_eq!(plan.ledger_rehearsal_attempted_count, 2);
        assert_eq!(plan.ledger_rehearsal_receipt_count, 2);
        assert_eq!(plan.ledger_rehearsal_blocked_count, 0);
        assert_eq!(plan.ledger_rehearsal_skipped_count, 0);
        assert!(plan.all_candidates_registered_once);
        assert!(plan.all_dispatch_entries_resolved_through_shadow_registry);
        assert!(plan.all_ledger_rehearsals_recorded_in_memory);
        assert!(plan.all_live_mutations_closed);
        assert!(plan.side_effect_free);
        assert!(!plan.registry_registered_live);
        assert!(!plan.registry_lookup_executed_live);
        assert!(!plan.tool_invoked);
        assert!(!plan.ledger_persisted);
        assert!(!plan.receipt_persisted);

        assert!(plan.entries.iter().all(|entry| {
            entry.dispatch_preflight_ready
                && entry.registration_route == Some(ShadowRegistrationRoute::Registered)
                && entry.shadow_lookup_route == ShadowLookupRoute::RequiresApprovalLedger
                && entry.ledger_rehearsal_attempted
                && entry.ledger_rehearsal_route
                    == Some(ToolInvocationLedgerRehearsalRoute::RehearsedApprovalRequiredReceipt)
                && entry.receipt_id.is_some()
                && entry.approval_required
                && !entry.registry_registered_live
                && !entry.registry_lookup_executed_live
                && !entry.tool_invoked
                && !entry.ledger_persisted
                && !entry.receipt_persisted
                && entry.side_effect_free
        }));
    }

    #[test]
    fn shadow_pipeline_blocks_duplicate_contribution_before_dispatch_canary() {
        let mut contributions = hepta_system_plugin_tool_contribution_inventory_preview_plan();
        let duplicate = contributions.candidate_inventory_entries[0].clone();
        contributions.candidate_inventory_entries.push(duplicate);
        let dispatch = hepta_system_tool_registry_read_only_dispatch_preflight_plan();

        let plan = tool_registry_shadow_pipeline_plan(&contributions, &dispatch);

        assert_eq!(plan.status, "blocked");
        assert!(!plan.shadow_pipeline_ready);
        assert_eq!(plan.contribution_candidate_count, 3);
        assert_eq!(plan.registration_outcome_count, 3);
        assert_eq!(plan.registered_count, 2);
        assert_eq!(plan.duplicate_registration_count, 0);
        assert_eq!(plan.idempotent_replay_count, 1);
        assert_eq!(plan.lookup_blocked_count, 0);
        assert!(!plan.all_candidates_registered_once);
        assert!(plan.all_live_mutations_closed);
    }

    #[test]
    fn shadow_pipeline_fails_closed_when_dispatch_preflight_is_blocked() {
        let contributions = hepta_system_plugin_tool_contribution_inventory_preview_plan();
        let source = hepta_system_tool_registry_invocation_source_of_truth_plan();
        let lookup = hepta_system_tool_registry_router_lookup_shadow_plan();
        let ledger = hepta_system_tool_invocation_ledger_approval_preflight_plan();
        let receipt = hepta_system_tool_invocation_receipt_projection_plan();
        let dispatch = tool_registry_read_only_dispatch_preflight_plan(
            &source,
            &lookup,
            &ledger,
            &receipt,
            &ToolRegistryReadOnlyDispatchPreflightInput {
                tool_invocation_switch_enabled: true,
                ..Default::default()
            },
        );

        let plan = tool_registry_shadow_pipeline_plan(&contributions, &dispatch);

        assert_eq!(dispatch.status, "blocked");
        assert_eq!(plan.status, "blocked");
        assert!(!plan.shadow_pipeline_ready);
        assert_eq!(plan.dispatch_preflight_ready_count, 0);
        assert_eq!(plan.ledger_rehearsal_attempted_count, 0);
        assert_eq!(plan.ledger_rehearsal_receipt_count, 0);
        assert_eq!(plan.ledger_rehearsal_skipped_count, 2);
        assert_eq!(plan.lookup_requires_approval_ledger_count, 2);
        assert!(plan.all_live_mutations_closed);
        assert!(plan.entries.iter().all(|entry| {
            !entry.dispatch_preflight_ready
                && entry.registration_route == Some(ShadowRegistrationRoute::Registered)
                && entry.shadow_lookup_route == ShadowLookupRoute::RequiresApprovalLedger
                && !entry.ledger_rehearsal_attempted
                && entry.ledger_rehearsal_route.is_none()
                && entry.receipt_id.is_none()
                && !entry.tool_invoked
                && !entry.ledger_persisted
                && !entry.receipt_persisted
        }));
    }
}

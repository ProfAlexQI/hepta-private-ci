use crate::ToolRegistryInvocationGuardRoute;
use crate::ToolRegistryInvocationSourceOfTruthPlan;
use crate::ToolRegistryInvocationSourceRoute;
use crate::hepta_system_tool_registry_invocation_source_of_truth_plan;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum ToolRegistryRegistrationLookupCutoverRoute {
    ApprovalLedgerLookupDryRun,
    BlockedByInvocationSource,
    BlockedByLookupGuard,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct ToolRegistryRegistrationLookupCutoverPreflightEntry {
    pub plugin_id: &'static str,
    pub candidate_tool_id: &'static str,
    pub contribution_kind: &'static str,
    pub source_invocation_route: ToolRegistryInvocationSourceRoute,
    pub registry_guard_route: ToolRegistryInvocationGuardRoute,
    pub lookup_cutover_route: ToolRegistryRegistrationLookupCutoverRoute,
    pub lookup_precondition_satisfied: bool,
    pub lookup_preflight_binding_present: bool,
    pub router_registration_lookup_enabled: bool,
    pub registry_lookup_executed: bool,
    pub registry_source_of_truth_enabled: bool,
    pub tool_registration_enabled: bool,
    pub tool_invocation_enabled: bool,
    pub ledger_write_enabled: bool,
    pub approval_request_enabled: bool,
    pub side_effect_free: bool,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct ToolRegistryRegistrationLookupCutoverPreflightPlan {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub plugin_id: &'static str,
    pub status: &'static str,
    pub source_invocation_surface: &'static str,
    pub source_invocation_ready: bool,
    pub source_invocation_ready_count: usize,
    pub candidate_count: usize,
    pub lookup_precondition_satisfied_count: usize,
    pub lookup_blocked_count: usize,
    pub approval_ledger_lookup_dry_run_count: usize,
    pub all_invocation_sources_bound_to_lookup_preflight: bool,
    pub all_lookup_entries_keep_approval_ledger_guard: bool,
    pub registration_lookup_cutover_preflight_ready: bool,
    pub registration_lookup_cutover_allowed: bool,
    pub router_registration_lookup_enabled: bool,
    pub registry_lookup_executed: bool,
    pub registry_source_of_truth_enabled: bool,
    pub tool_registration_enabled: bool,
    pub tool_invocation_enabled: bool,
    pub ledger_written: bool,
    pub approval_requested: bool,
    pub live_mutation_ready: bool,
    pub side_effect_free: bool,
    pub next_migration_step: &'static str,
    pub entries: Vec<ToolRegistryRegistrationLookupCutoverPreflightEntry>,
}

pub fn hepta_system_tool_registry_registration_lookup_cutover_preflight_plan()
-> ToolRegistryRegistrationLookupCutoverPreflightPlan {
    let source = hepta_system_tool_registry_invocation_source_of_truth_plan();
    tool_registry_registration_lookup_cutover_preflight_plan(&source)
}

pub fn tool_registry_registration_lookup_cutover_preflight_plan(
    source: &ToolRegistryInvocationSourceOfTruthPlan,
) -> ToolRegistryRegistrationLookupCutoverPreflightPlan {
    let entries = source
        .entries
        .iter()
        .map(|entry| {
            let lookup_cutover_route = if entry.invocation_source_route
                != ToolRegistryInvocationSourceRoute::ApprovalLedgerDryRunSourceOnly
                || !entry.invocation_source_ready
            {
                ToolRegistryRegistrationLookupCutoverRoute::BlockedByInvocationSource
            } else if entry.registry_guard_route
                != ToolRegistryInvocationGuardRoute::RequireApprovalLedger
            {
                ToolRegistryRegistrationLookupCutoverRoute::BlockedByLookupGuard
            } else {
                ToolRegistryRegistrationLookupCutoverRoute::ApprovalLedgerLookupDryRun
            };
            let lookup_precondition_satisfied = lookup_cutover_route
                == ToolRegistryRegistrationLookupCutoverRoute::ApprovalLedgerLookupDryRun
                && !entry.router_registration_lookup_enabled
                && !entry.registry_source_of_truth_enabled
                && !entry.tool_registration_enabled
                && !entry.tool_invocation_enabled
                && !entry.ledger_write_enabled
                && !entry.approval_request_enabled;

            ToolRegistryRegistrationLookupCutoverPreflightEntry {
                plugin_id: entry.plugin_id,
                candidate_tool_id: entry.candidate_tool_id,
                contribution_kind: entry.contribution_kind,
                source_invocation_route: entry.invocation_source_route,
                registry_guard_route: entry.registry_guard_route,
                lookup_cutover_route,
                lookup_precondition_satisfied,
                lookup_preflight_binding_present: true,
                router_registration_lookup_enabled: false,
                registry_lookup_executed: false,
                registry_source_of_truth_enabled: false,
                tool_registration_enabled: false,
                tool_invocation_enabled: false,
                ledger_write_enabled: false,
                approval_request_enabled: false,
                side_effect_free: true,
            }
        })
        .collect::<Vec<_>>();

    let lookup_precondition_satisfied_count = entries
        .iter()
        .filter(|entry| entry.lookup_precondition_satisfied)
        .count();
    let approval_ledger_lookup_dry_run_count = entries
        .iter()
        .filter(|entry| {
            entry.lookup_cutover_route
                == ToolRegistryRegistrationLookupCutoverRoute::ApprovalLedgerLookupDryRun
        })
        .count();
    let lookup_blocked_count = entries.len() - lookup_precondition_satisfied_count;
    let all_invocation_sources_bound_to_lookup_preflight = entries
        .iter()
        .all(|entry| entry.lookup_preflight_binding_present)
        && lookup_precondition_satisfied_count == source.invocation_source_ready_count
        && approval_ledger_lookup_dry_run_count == source.approval_ledger_dry_run_source_count
        && lookup_precondition_satisfied_count == entries.len()
        && approval_ledger_lookup_dry_run_count == entries.len();
    let all_lookup_entries_keep_approval_ledger_guard = entries.iter().all(|entry| {
        if entry.lookup_cutover_route
            == ToolRegistryRegistrationLookupCutoverRoute::ApprovalLedgerLookupDryRun
        {
            entry.registry_guard_route == ToolRegistryInvocationGuardRoute::RequireApprovalLedger
                && !entry.router_registration_lookup_enabled
                && !entry.registry_lookup_executed
                && !entry.registry_source_of_truth_enabled
                && !entry.tool_registration_enabled
                && !entry.tool_invocation_enabled
                && !entry.ledger_write_enabled
                && !entry.approval_request_enabled
        } else {
            true
        }
    });
    let registration_lookup_cutover_preflight_ready = source.invocation_source_of_truth_plan_ready
        && all_invocation_sources_bound_to_lookup_preflight
        && all_lookup_entries_keep_approval_ledger_guard;
    let registration_lookup_cutover_allowed = registration_lookup_cutover_preflight_ready
        && lookup_precondition_satisfied_count == entries.len()
        && approval_ledger_lookup_dry_run_count == entries.len();

    ToolRegistryRegistrationLookupCutoverPreflightPlan {
        runtime: "hepta",
        surface: "tool_registry_registration_lookup_cutover_preflight",
        plugin_id: source.plugin_id,
        status: if registration_lookup_cutover_preflight_ready {
            "ready"
        } else {
            "blocked"
        },
        source_invocation_surface: source.surface,
        source_invocation_ready: source.invocation_source_of_truth_plan_ready,
        source_invocation_ready_count: source.invocation_source_ready_count,
        candidate_count: entries.len(),
        lookup_precondition_satisfied_count,
        lookup_blocked_count,
        approval_ledger_lookup_dry_run_count,
        all_invocation_sources_bound_to_lookup_preflight,
        all_lookup_entries_keep_approval_ledger_guard,
        registration_lookup_cutover_preflight_ready,
        registration_lookup_cutover_allowed,
        router_registration_lookup_enabled: false,
        registry_lookup_executed: false,
        registry_source_of_truth_enabled: false,
        tool_registration_enabled: false,
        tool_invocation_enabled: false,
        ledger_written: false,
        approval_requested: false,
        live_mutation_ready: false,
        side_effect_free: true,
        next_migration_step: "restore_tool_registry_router_lookup_shadow_without_registration",
        entries,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tool_registry_registration_lookup_cutover_preflight_binds_invocation_sources() {
        let plan = hepta_system_tool_registry_registration_lookup_cutover_preflight_plan();

        assert_eq!(plan.status, "ready");
        assert_eq!(
            plan.source_invocation_surface,
            "tool_registry_invocation_source_of_truth"
        );
        assert!(plan.source_invocation_ready);
        assert_eq!(plan.source_invocation_ready_count, 2);
        assert_eq!(plan.candidate_count, 2);
        assert_eq!(plan.lookup_precondition_satisfied_count, 2);
        assert_eq!(plan.lookup_blocked_count, 0);
        assert_eq!(plan.approval_ledger_lookup_dry_run_count, 2);
        assert!(plan.all_invocation_sources_bound_to_lookup_preflight);
        assert!(plan.registration_lookup_cutover_preflight_ready);
        assert!(plan.registration_lookup_cutover_allowed);
        assert!(plan.entries.iter().all(|entry| {
            entry.lookup_cutover_route
                == ToolRegistryRegistrationLookupCutoverRoute::ApprovalLedgerLookupDryRun
                && entry.lookup_precondition_satisfied
                && entry.registry_guard_route
                    == ToolRegistryInvocationGuardRoute::RequireApprovalLedger
        }));
    }

    #[test]
    fn tool_registry_registration_lookup_cutover_preflight_does_not_execute_lookup() {
        let plan = hepta_system_tool_registry_registration_lookup_cutover_preflight_plan();

        assert!(plan.registration_lookup_cutover_preflight_ready);
        assert!(!plan.router_registration_lookup_enabled);
        assert!(!plan.registry_lookup_executed);
        assert!(!plan.registry_source_of_truth_enabled);
        assert!(!plan.tool_registration_enabled);
        assert!(!plan.tool_invocation_enabled);
        assert!(!plan.ledger_written);
        assert!(!plan.approval_requested);
        assert!(!plan.live_mutation_ready);
        assert!(plan.side_effect_free);
        assert!(plan.entries.iter().all(|entry| {
            !entry.router_registration_lookup_enabled
                && !entry.registry_lookup_executed
                && !entry.registry_source_of_truth_enabled
                && !entry.tool_registration_enabled
                && !entry.tool_invocation_enabled
                && !entry.ledger_write_enabled
                && !entry.approval_request_enabled
        }));
    }

    #[test]
    fn tool_registry_registration_lookup_cutover_preflight_blocks_unready_invocation_sources() {
        let mut source = hepta_system_tool_registry_invocation_source_of_truth_plan();
        source.invocation_source_ready_count = 0;
        source.approval_ledger_dry_run_source_count = 0;
        for entry in &mut source.entries {
            entry.invocation_source_route =
                ToolRegistryInvocationSourceRoute::BlockedByRouterPreflight;
            entry.invocation_source_ready = false;
        }

        let plan = tool_registry_registration_lookup_cutover_preflight_plan(&source);

        assert_eq!(plan.status, "blocked");
        assert_eq!(plan.lookup_precondition_satisfied_count, 0);
        assert_eq!(plan.lookup_blocked_count, 2);
        assert_eq!(plan.approval_ledger_lookup_dry_run_count, 0);
        assert!(!plan.registration_lookup_cutover_preflight_ready);
        assert!(!plan.registration_lookup_cutover_allowed);
        assert!(plan.entries.iter().all(|entry| {
            entry.lookup_cutover_route
                == ToolRegistryRegistrationLookupCutoverRoute::BlockedByInvocationSource
        }));
    }

    #[test]
    fn tool_registry_registration_lookup_cutover_preflight_respects_source_readiness() {
        let source = ToolRegistryInvocationSourceOfTruthPlan {
            invocation_source_of_truth_plan_ready: false,
            ..hepta_system_tool_registry_invocation_source_of_truth_plan()
        };

        let plan = tool_registry_registration_lookup_cutover_preflight_plan(&source);

        assert_eq!(plan.status, "blocked");
        assert!(!plan.source_invocation_ready);
        assert!(!plan.registration_lookup_cutover_preflight_ready);
        assert!(!plan.registration_lookup_cutover_allowed);
    }
}

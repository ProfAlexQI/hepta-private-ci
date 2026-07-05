use std::collections::BTreeSet;

use serde::Serialize;

use crate::work_graph_append_only_event_intake_preview::work_graph_append_only_event_contracts;
use crate::work_graph_append_only_event_intake_preview::work_graph_append_only_event_routes;
use crate::work_graph_idempotency_readback_adapter_preview::work_graph_idempotency_readback_source_adapters;
use crate::work_graph_observability_timeline::work_graph_observability_timeline_adapter_previews;
use crate::work_graph_role_manifest_contract::work_graph_role_manifest_adapter_previews;
use crate::work_graph_scheduler_admission_controller::work_graph_scheduler_admission_adapter_previews;
use crate::work_graph_task_result_contract::work_graph_task_result_adapter_previews;
use crate::work_graph_unified_projection_audit_preview::WorkGraphUnifiedProjectionSourceAudit;
use crate::work_graph_unified_projection_audit_preview::work_graph_unified_projection_source_audits;

pub const WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_PREVIEW_GATE: &str =
    "hepta_work_graph_unified_projection_enforcement_readiness_preview_gate";
pub const WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_SCHEMA_VERSION: &str =
    "work_graph_unified_projection_enforcement_readiness_preview_v1";
pub const WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_RECOMMENDED_NEXT_GATE: &str =
    "hepta_work_graph_projection_adapter_gap_closure_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphUnifiedProjectionEnforcementReadinessPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub source_surface_count: usize,
    pub contract_ready_surface_count: usize,
    pub ready_surface_count: usize,
    pub blocked_surface_count: usize,
    pub decision_count: usize,
    pub blocker_count: usize,
    pub enforcement_stage_count: usize,
    pub required_prior_gate_count: usize,
    pub source_decisions: Vec<WorkGraphProjectionEnforcementSourceDecisionPreview>,
    pub blockers: Vec<WorkGraphProjectionEnforcementBlockerPreview>,
    pub enforcement_stages: Vec<WorkGraphProjectionEnforcementStagePreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub ready_for_projection_gap_closure_preview: bool,
    pub ready_for_projection_enforcement: bool,
    pub ready_for_append_only_store_enablement: bool,
    pub ready_for_task_result_enforcement: bool,
    pub ready_for_scheduler_admission_enforcement: bool,
    pub ready_for_live_execution: bool,
    pub side_effects: WorkGraphUnifiedProjectionEnforcementReadinessPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphProjectionEnforcementSourceDecisionPreview {
    pub source_surface_id: &'static str,
    pub source_category: &'static str,
    pub coverage_state: &'static str,
    pub requires_terminal_task_result: bool,
    pub projection_contract_ready: bool,
    pub unified_store_projection_ready: bool,
    pub timeline_projection_ready: bool,
    pub task_result_projection_ready: bool,
    pub role_manifest_projection_ready: bool,
    pub scheduler_admission_projection_ready: bool,
    pub append_only_route_ready: bool,
    pub store_idempotency_guard_ready: bool,
    pub idempotency_readback_adapter_present: bool,
    pub readback_probe_contract_ready: bool,
    pub source_blocker_ids: Vec<&'static str>,
    pub route_blocker_ids: Vec<&'static str>,
    pub enforcement_decision: &'static str,
    pub next_required_gate: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphProjectionEnforcementBlockerPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub required_before_projection_enforcement: bool,
    pub recommended_fix: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphProjectionEnforcementStagePreview {
    pub id: &'static str,
    pub input_gate_ids: Vec<&'static str>,
    pub observed_contract_count: usize,
    pub ready_contract_count: usize,
    pub hard_blocker_ids: Vec<&'static str>,
    pub enforcement_enabled: bool,
    pub next_gate: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphUnifiedProjectionEnforcementReadinessPreviewSideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub append_only_store_enabled: bool,
    pub projection_enforcement_enabled: bool,
    pub scheduler_admission_enforced: bool,
    pub task_result_enforcement_enabled: bool,
    pub role_manifest_enforcement_enabled: bool,
    pub timeline_persisted: bool,
    pub approval_recorded: bool,
    pub runtime_mutation_performed: bool,
    pub agent_spawn_performed: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_unified_projection_enforcement_readiness_preview_report()
-> WorkGraphUnifiedProjectionEnforcementReadinessPreviewReport {
    let source_decisions = work_graph_unified_projection_enforcement_source_decisions();
    let blockers = work_graph_unified_projection_enforcement_blockers();
    let enforcement_stages = work_graph_unified_projection_enforcement_stages();
    let required_prior_gates =
        work_graph_unified_projection_enforcement_readiness_required_prior_gates();
    let contract_ready_surface_count = source_decisions
        .iter()
        .filter(|decision| decision.projection_contract_ready)
        .count();
    let ready_surface_count = source_decisions
        .iter()
        .filter(|decision| decision.enforcement_decision == "allow_preview_only")
        .count();
    let blocked_surface_count = source_decisions.len() - ready_surface_count;

    WorkGraphUnifiedProjectionEnforcementReadinessPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "blocked",
        gate: WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_PREVIEW_GATE,
        schema_version: WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_SCHEMA_VERSION,
        preview_mode: "read_only_projection_enforcement_readiness_no_enforcement",
        source_surface_count: source_decisions.len(),
        contract_ready_surface_count,
        ready_surface_count,
        blocked_surface_count,
        decision_count: source_decisions.len(),
        blocker_count: blockers.len(),
        enforcement_stage_count: enforcement_stages.len(),
        required_prior_gate_count: required_prior_gates.len(),
        source_decisions,
        blockers,
        enforcement_stages,
        required_prior_gates,
        recommended_next_gate:
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_RECOMMENDED_NEXT_GATE,
        ready_for_projection_gap_closure_preview: true,
        ready_for_projection_enforcement: false,
        ready_for_append_only_store_enablement: false,
        ready_for_task_result_enforcement: false,
        ready_for_scheduler_admission_enforcement: false,
        ready_for_live_execution: false,
        side_effects: WorkGraphUnifiedProjectionEnforcementReadinessPreviewSideEffects::none(),
    }
}

pub fn work_graph_unified_projection_enforcement_source_decisions()
-> Vec<WorkGraphProjectionEnforcementSourceDecisionPreview> {
    let routes = work_graph_append_only_event_routes();
    let idempotency_adapter_sources = work_graph_idempotency_readback_source_adapters()
        .into_iter()
        .map(|adapter| adapter.source_surface_id)
        .collect::<BTreeSet<_>>();

    work_graph_unified_projection_source_audits()
        .into_iter()
        .map(|source| {
            let route = routes
                .iter()
                .find(|route| route.source_surface_id == source.source_surface_id);
            source_decision(source, route, &idempotency_adapter_sources)
        })
        .collect()
}

pub fn work_graph_unified_projection_enforcement_blockers()
-> Vec<WorkGraphProjectionEnforcementBlockerPreview> {
    let decisions = work_graph_unified_projection_enforcement_source_decisions();
    vec![
        blocker(
            "projection_adapters_missing_for_enforcement",
            "high",
            affected_sources(&decisions, |decision| {
                !decision.unified_store_projection_ready
                    || !decision.timeline_projection_ready
                    || !decision.task_result_projection_ready
            }),
            "close unified store, timeline, and terminal TaskResult projection gaps before enabling projection authority",
        ),
        blocker(
            "store_idempotency_guards_missing_for_enforcement",
            "high",
            affected_sources(&decisions, |decision| {
                !decision.store_idempotency_guard_ready
            }),
            "promote idempotency readback adapters into state-store guards before any append-only intake writes",
        ),
        blocker(
            "terminal_task_result_enforcement_disabled",
            "high",
            affected_sources(&decisions, |decision| {
                decision.requires_terminal_task_result
                    && decision
                        .route_blocker_ids
                        .contains(&"terminal_task_result_enforcement_disabled")
            }),
            "make every terminal worker, agent, scheduler, and handoff path emit the canonical TaskResult contract",
        ),
        blocker(
            "scheduler_admission_not_enforced",
            "high",
            affected_sources(&decisions, |decision| {
                has_suffix(&decision.source_blocker_ids, "_admission_not_enforced")
            }),
            "make dependency, lease, budget, approval, role, and idempotency checks authoritative before work start",
        ),
        blocker(
            "role_manifest_not_enforced",
            "medium",
            affected_sources(&decisions, |decision| {
                has_contains(&decision.source_blocker_ids, "role_manifest_not_enforced")
            }),
            "bind multi-agent, batch, worker, and handoff sources to role manifests with budgets and tool permissions",
        ),
        blocker(
            "append_only_store_enablement_disabled",
            "medium",
            decisions
                .iter()
                .map(|decision| decision.source_surface_id)
                .collect(),
            "keep projection enforcement disabled until WAL, readback, replay, and operator readiness gates are promoted",
        ),
    ]
}

pub fn work_graph_unified_projection_enforcement_stages()
-> Vec<WorkGraphProjectionEnforcementStagePreview> {
    let source_decisions = work_graph_unified_projection_enforcement_source_decisions();
    let routes = work_graph_append_only_event_routes();
    let task_result_adapters = work_graph_task_result_adapter_previews();
    let scheduler_adapters = work_graph_scheduler_admission_adapter_previews();
    let timeline_adapters = work_graph_observability_timeline_adapter_previews();
    let role_adapters = work_graph_role_manifest_adapter_previews();
    let idempotency_gap_adapters = work_graph_idempotency_readback_source_adapters();
    let event_contracts = work_graph_append_only_event_contracts();

    vec![
        stage(
            "unified_projection_contracts",
            vec!["hepta_work_graph_unified_projection_audit_preview_gate"],
            source_decisions.len(),
            source_decisions
                .iter()
                .filter(|decision| decision.projection_contract_ready)
                .count(),
            vec!["projection_adapters_missing_for_enforcement"],
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_RECOMMENDED_NEXT_GATE,
        ),
        stage(
            "timeline_observability_contracts",
            vec!["hepta_work_graph_observability_timeline_preview_gate"],
            timeline_adapters.len(),
            0,
            vec!["projection_adapters_missing_for_enforcement"],
            "hepta_work_graph_observability_timeline_preview_gate",
        ),
        stage(
            "append_only_event_intake_contracts",
            vec!["hepta_work_graph_append_only_event_intake_preview_gate"],
            routes.len(),
            routes
                .iter()
                .filter(|route| {
                    !route
                        .blocker_ids
                        .contains(&"source_projection_not_contract_ready")
                })
                .count(),
            vec!["append_only_store_enablement_disabled"],
            "hepta_work_graph_append_only_event_intake_preview_gate",
        ),
        stage(
            "idempotency_readback_gap_closures",
            vec!["hepta_work_graph_idempotency_readback_adapter_preview_gate"],
            idempotency_gap_adapters.len(),
            0,
            vec!["store_idempotency_guards_missing_for_enforcement"],
            "hepta_work_graph_state_store_persistence_preview_gate",
        ),
        stage(
            "terminal_task_result_contracts",
            vec!["hepta_work_graph_task_result_contract_preview_gate"],
            task_result_adapters.len(),
            0,
            vec!["terminal_task_result_enforcement_disabled"],
            "hepta_work_graph_terminal_task_result_wrapper_preview_gate",
        ),
        stage(
            "scheduler_admission_contracts",
            vec!["hepta_work_graph_scheduler_admission_controller_preview_gate"],
            scheduler_adapters.len(),
            0,
            vec!["scheduler_admission_not_enforced"],
            "hepta_work_graph_scheduler_admission_controller_preview_gate",
        ),
        stage(
            "role_manifest_contracts",
            vec!["hepta_work_graph_role_manifest_contract_preview_gate"],
            role_adapters.len(),
            0,
            vec!["role_manifest_not_enforced"],
            "hepta_work_graph_role_manifest_contract_preview_gate",
        ),
        stage(
            "append_only_store_enablement",
            vec![
                "hepta_work_graph_state_store_persistence_preview_gate",
                "hepta_work_graph_replay_readback_preview_gate",
            ],
            event_contracts.len(),
            0,
            vec!["append_only_store_enablement_disabled"],
            "hepta_work_graph_append_only_store_enablement_precondition_preview_gate",
        ),
    ]
}

pub fn work_graph_unified_projection_enforcement_readiness_required_prior_gates()
-> Vec<&'static str> {
    vec![
        "hepta_work_graph_contract_preview_gate",
        "hepta_work_graph_task_result_contract_preview_gate",
        "hepta_work_graph_scheduler_admission_controller_preview_gate",
        "hepta_work_graph_observability_timeline_preview_gate",
        "hepta_work_graph_role_manifest_contract_preview_gate",
        "hepta_work_graph_unified_state_store_preview_gate",
        "hepta_work_graph_adapter_projection_fixture_gate",
        "hepta_work_graph_unified_projection_audit_preview_gate",
        "hepta_work_graph_state_store_persistence_preview_gate",
        "hepta_work_graph_append_only_event_intake_preview_gate",
        "hepta_work_graph_replay_readback_preview_gate",
        "hepta_work_graph_idempotency_readback_adapter_preview_gate",
    ]
}

impl WorkGraphUnifiedProjectionEnforcementReadinessPreviewSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            append_only_store_enabled: false,
            projection_enforcement_enabled: false,
            scheduler_admission_enforced: false,
            task_result_enforcement_enabled: false,
            role_manifest_enforcement_enabled: false,
            timeline_persisted: false,
            approval_recorded: false,
            runtime_mutation_performed: false,
            agent_spawn_performed: false,
            external_send_performed: false,
            model_invoked: false,
        }
    }
}

fn source_decision(
    source: WorkGraphUnifiedProjectionSourceAudit,
    route: Option<
        &crate::work_graph_append_only_event_intake_preview::WorkGraphAppendOnlyEventRoutePreview,
    >,
    idempotency_adapter_sources: &BTreeSet<&'static str>,
) -> WorkGraphProjectionEnforcementSourceDecisionPreview {
    let readback_probe_contract_ready = route
        .map(|route| route.readback_probe_ids.len() >= route.target_collection_ids.len())
        .unwrap_or(false);
    let store_idempotency_guard_ready =
        route.and_then(|route| route.idempotency_guard_id).is_some();
    let route_blocker_ids = route
        .map(|route| route.blocker_ids.clone())
        .unwrap_or_default();
    let source_blocker_ids = source.blocker_ids.clone();
    let task_result_projection_ready =
        !source.requires_terminal_task_result || source.has_task_result_projection;
    let role_manifest_projection_ready =
        !source.requires_terminal_task_result || source.has_role_manifest_projection;
    let scheduler_admission_projection_ready =
        !scheduler_relevant(&source) || source.has_scheduler_admission_projection;
    let enforcement_decision = enforcement_decision_for(
        &source,
        route,
        task_result_projection_ready,
        readback_probe_contract_ready,
    );

    WorkGraphProjectionEnforcementSourceDecisionPreview {
        source_surface_id: source.source_surface_id,
        source_category: source.source_category,
        coverage_state: source.coverage_state,
        requires_terminal_task_result: source.requires_terminal_task_result,
        projection_contract_ready: source.coverage_state == "contract_ready_preview",
        unified_store_projection_ready: source.has_unified_store_projection,
        timeline_projection_ready: source.has_observability_timeline_projection,
        task_result_projection_ready,
        role_manifest_projection_ready,
        scheduler_admission_projection_ready,
        append_only_route_ready: route.is_some(),
        store_idempotency_guard_ready,
        idempotency_readback_adapter_present: idempotency_adapter_sources
            .contains(source.source_surface_id),
        readback_probe_contract_ready,
        source_blocker_ids,
        route_blocker_ids,
        enforcement_decision,
        next_required_gate: next_required_gate_for(enforcement_decision),
    }
}

fn enforcement_decision_for(
    source: &WorkGraphUnifiedProjectionSourceAudit,
    route: Option<
        &crate::work_graph_append_only_event_intake_preview::WorkGraphAppendOnlyEventRoutePreview,
    >,
    task_result_projection_ready: bool,
    readback_probe_contract_ready: bool,
) -> &'static str {
    if !source.has_unified_store_projection {
        "deny_missing_unified_store_projection"
    } else if !source.has_observability_timeline_projection {
        "deny_missing_timeline_projection"
    } else if !task_result_projection_ready {
        "deny_missing_task_result_projection"
    } else if route.is_none() {
        "deny_missing_append_only_route"
    } else if route.and_then(|route| route.idempotency_guard_id).is_none() {
        "deny_missing_store_idempotency_guard"
    } else if !readback_probe_contract_ready {
        "deny_missing_readback_probe"
    } else if route
        .map(|route| {
            route
                .blocker_ids
                .contains(&"terminal_task_result_enforcement_disabled")
        })
        .unwrap_or(false)
    {
        "deny_terminal_task_result_enforcement_disabled"
    } else if has_suffix(&source.blocker_ids, "_admission_not_enforced") {
        "deny_scheduler_admission_not_enforced"
    } else if has_contains(&source.blocker_ids, "role_manifest_not_enforced") {
        "deny_role_manifest_not_enforced"
    } else if route
        .map(|route| {
            route
                .blocker_ids
                .contains(&"append_only_store_disabled_by_design")
        })
        .unwrap_or(false)
    {
        "deny_append_only_store_disabled"
    } else {
        "allow_preview_only"
    }
}

fn next_required_gate_for(enforcement_decision: &str) -> &'static str {
    match enforcement_decision {
        "deny_missing_unified_store_projection" => {
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_RECOMMENDED_NEXT_GATE
        }
        "deny_missing_timeline_projection" => {
            "hepta_work_graph_observability_timeline_preview_gate"
        }
        "deny_missing_task_result_projection" => {
            "hepta_work_graph_task_result_contract_preview_gate"
        }
        "deny_missing_append_only_route" => {
            "hepta_work_graph_append_only_event_intake_preview_gate"
        }
        "deny_missing_store_idempotency_guard" => {
            "hepta_work_graph_idempotency_readback_adapter_preview_gate"
        }
        "deny_missing_readback_probe" => "hepta_work_graph_replay_readback_preview_gate",
        "deny_terminal_task_result_enforcement_disabled" => {
            "hepta_work_graph_terminal_task_result_wrapper_preview_gate"
        }
        "deny_scheduler_admission_not_enforced" => {
            "hepta_work_graph_scheduler_admission_controller_preview_gate"
        }
        "deny_role_manifest_not_enforced" => "hepta_work_graph_role_manifest_contract_preview_gate",
        "deny_append_only_store_disabled" => {
            "hepta_work_graph_append_only_store_enablement_precondition_preview_gate"
        }
        "allow_preview_only" => "hepta_work_graph_projection_enforcement_dry_run_preview_gate",
        _ => "hepta_work_graph_projection_adapter_gap_closure_preview_gate",
    }
}

fn scheduler_relevant(source: &WorkGraphUnifiedProjectionSourceAudit) -> bool {
    matches!(
        source.source_category,
        "multi_agent" | "batch_agent_jobs" | "runtime_scheduler"
    )
}

fn affected_sources(
    decisions: &[WorkGraphProjectionEnforcementSourceDecisionPreview],
    predicate: impl Fn(&WorkGraphProjectionEnforcementSourceDecisionPreview) -> bool,
) -> Vec<&'static str> {
    decisions
        .iter()
        .filter(|decision| predicate(decision))
        .map(|decision| decision.source_surface_id)
        .collect()
}

fn blocker(
    id: &'static str,
    severity: &'static str,
    affected_source_surface_ids: Vec<&'static str>,
    recommended_fix: &'static str,
) -> WorkGraphProjectionEnforcementBlockerPreview {
    WorkGraphProjectionEnforcementBlockerPreview {
        id,
        severity,
        affected_source_surface_ids,
        required_before_projection_enforcement: true,
        recommended_fix,
    }
}

fn stage(
    id: &'static str,
    input_gate_ids: Vec<&'static str>,
    observed_contract_count: usize,
    ready_contract_count: usize,
    hard_blocker_ids: Vec<&'static str>,
    next_gate: &'static str,
) -> WorkGraphProjectionEnforcementStagePreview {
    WorkGraphProjectionEnforcementStagePreview {
        id,
        input_gate_ids,
        observed_contract_count,
        ready_contract_count,
        hard_blocker_ids,
        enforcement_enabled: false,
        next_gate,
    }
}

fn has_suffix(values: &[&'static str], suffix: &str) -> bool {
    values.iter().any(|value| value.ends_with(suffix))
}

fn has_contains(values: &[&'static str], needle: &str) -> bool {
    values.iter().any(|value| value.contains(needle))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unified_projection_enforcement_readiness_covers_all_source_surfaces() {
        let report = hepta_work_graph_unified_projection_enforcement_readiness_preview_report();
        let source_ids = report
            .source_decisions
            .iter()
            .map(|decision| decision.source_surface_id)
            .collect::<Vec<_>>();

        assert_eq!(
            source_ids,
            [
                "update_plan_tool",
                "plan_mode_proposed_plan_blocks",
                "app_server_turn_plan_notification",
                "multi_agent_v2_thread_spawn",
                "multi_agent_v2_mailbox_wait",
                "hepta_runtime_multi_agent_reducer",
                "agent_jobs_batch_workers",
                "hepta_runtime_task_board",
                "hepta_runtime_worker_tasks",
                "hepta_runtime_scheduler_store",
                "hepta_runtime_approval_broker",
                "hepta_runtime_agent_harness",
            ]
        );
        assert_eq!(report.source_surface_count, 12);
        assert_eq!(report.contract_ready_surface_count, 5);
        assert_eq!(report.ready_surface_count, 0);
        assert_eq!(report.blocked_surface_count, 12);
    }

    #[test]
    fn unified_projection_enforcement_readiness_names_projection_and_idempotency_gaps() {
        let decisions = work_graph_unified_projection_enforcement_source_decisions();
        let missing_store_sources = decisions
            .iter()
            .filter(|decision| {
                decision.enforcement_decision == "deny_missing_unified_store_projection"
            })
            .map(|decision| decision.source_surface_id)
            .collect::<Vec<_>>();
        let missing_store_guard_sources = decisions
            .iter()
            .filter(|decision| !decision.store_idempotency_guard_ready)
            .map(|decision| decision.source_surface_id)
            .collect::<Vec<_>>();

        assert!(missing_store_sources.contains(&"hepta_runtime_task_board"));
        assert!(missing_store_sources.contains(&"multi_agent_v2_mailbox_wait"));
        assert_eq!(missing_store_guard_sources.len(), 5);
        assert!(
            decisions
                .iter()
                .filter(|decision| !decision.store_idempotency_guard_ready)
                .all(|decision| decision.idempotency_readback_adapter_present)
        );
    }

    #[test]
    fn unified_projection_enforcement_readiness_blocks_terminal_task_result_authority() {
        let decisions = work_graph_unified_projection_enforcement_source_decisions();
        let terminal_denials = decisions
            .iter()
            .filter(|decision| {
                decision.enforcement_decision == "deny_terminal_task_result_enforcement_disabled"
            })
            .map(|decision| decision.source_surface_id)
            .collect::<Vec<_>>();

        assert_eq!(
            terminal_denials,
            [
                "multi_agent_v2_thread_spawn",
                "agent_jobs_batch_workers",
                "hepta_runtime_worker_tasks",
                "hepta_runtime_scheduler_store",
                "hepta_runtime_agent_harness",
            ]
        );
        assert!(
            decisions
                .iter()
                .all(|decision| { decision.enforcement_decision != "allow_preview_only" })
        );
    }

    #[test]
    fn unified_projection_enforcement_readiness_declares_ordered_enforcement_stages() {
        let report = hepta_work_graph_unified_projection_enforcement_readiness_preview_report();
        let stage_ids = report
            .enforcement_stages
            .iter()
            .map(|stage| stage.id)
            .collect::<Vec<_>>();

        assert_eq!(
            stage_ids,
            [
                "unified_projection_contracts",
                "timeline_observability_contracts",
                "append_only_event_intake_contracts",
                "idempotency_readback_gap_closures",
                "terminal_task_result_contracts",
                "scheduler_admission_contracts",
                "role_manifest_contracts",
                "append_only_store_enablement",
            ]
        );
        assert_eq!(report.enforcement_stage_count, 8);
        assert!(
            report
                .enforcement_stages
                .iter()
                .all(|stage| !stage.enforcement_enabled)
        );
    }

    #[test]
    fn unified_projection_enforcement_readiness_keeps_all_side_effects_disabled() {
        let report = hepta_work_graph_unified_projection_enforcement_readiness_preview_report();

        assert_eq!(
            report.side_effects,
            WorkGraphUnifiedProjectionEnforcementReadinessPreviewSideEffects::none()
        );
        assert!(report.ready_for_projection_gap_closure_preview);
        assert!(!report.ready_for_projection_enforcement);
        assert!(!report.ready_for_append_only_store_enablement);
        assert!(!report.ready_for_task_result_enforcement);
        assert!(!report.ready_for_scheduler_admission_enforcement);
        assert!(!report.ready_for_live_execution);
        assert_eq!(
            report.recommended_next_gate,
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_RECOMMENDED_NEXT_GATE
        );
        assert_eq!(
            crate::work_graph_unified_projection_audit_preview::work_graph_unified_projection_coverage_gaps()
                .len(),
            5
        );
    }
}

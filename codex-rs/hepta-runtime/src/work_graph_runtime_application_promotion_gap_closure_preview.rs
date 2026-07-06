use serde::Serialize;

use crate::work_graph_unified_projection_enforcement_readiness_append_only_store_runtime_rerun_preview::{
    WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_APPEND_ONLY_STORE_RUNTIME_RERUN_PREVIEW_GATE,
    WorkGraphAppendOnlyStoreRuntimeRerunResidualBlockerPreview,
    WorkGraphAppendOnlyStoreRuntimeRerunSourceDecisionPreview,
    work_graph_unified_projection_enforcement_append_only_store_runtime_rerun_residual_blockers,
    work_graph_unified_projection_enforcement_append_only_store_runtime_rerun_source_decisions,
    work_graph_unified_projection_enforcement_readiness_append_only_store_runtime_rerun_required_prior_gates,
};

pub const WORK_GRAPH_RUNTIME_APPLICATION_PROMOTION_GAP_CLOSURE_PREVIEW_GATE: &str =
    "hepta_work_graph_runtime_application_promotion_gap_closure_preview_gate";
pub const WORK_GRAPH_RUNTIME_APPLICATION_PROMOTION_GAP_CLOSURE_SCHEMA_VERSION: &str =
    "work_graph_runtime_application_promotion_gap_closure_preview_v1";
pub const WORK_GRAPH_RUNTIME_APPLICATION_PROMOTION_GAP_CLOSURE_RECOMMENDED_NEXT_GATE: &str =
    "hepta_work_graph_runtime_application_promotion_gap_closure_readback_preview_gate";

const PROMOTION_EVIDENCE_FIELD_IDS: [&str; 8] = [
    "source_surface_id",
    "runtime_application_domain_ids",
    "runtime_rerun_enforcement_decision",
    "residual_source_blocker_ids",
    "promotion_binding_ids",
    "readback_probe_id",
    "operator_review_gate_ref",
    "side_effect_lock_ref",
];

const PROMOTION_DOMAINS: [(&str, &str, &str, &str); 5] = [
    (
        "projection_adapter_runtime_closure",
        "projection_adapter_runtime_closure_application_disabled",
        "p0",
        "projection adapter runtime closure remains a no-mutation contract until readback and operator review complete",
    ),
    (
        "store_guard_runtime_application",
        "store_guard_runtime_application_disabled",
        "p0",
        "store guard runtime application remains a no-mutation contract until durable-store write boundaries are promoted",
    ),
    (
        "terminal_task_result_runtime_wrapper",
        "terminal_task_result_runtime_application_disabled",
        "p0",
        "terminal TaskResult runtime wrapper remains detached until TaskResult persistence and wrapper execution are promoted",
    ),
    (
        "scheduler_admission_runtime_application",
        "scheduler_admission_runtime_application_disabled",
        "p1",
        "scheduler admission runtime wiring remains disabled until leases, dependencies, budgets, and approvals are promoted",
    ),
    (
        "role_manifest_runtime_application",
        "role_manifest_runtime_application_disabled",
        "p1",
        "role manifest runtime wiring remains disabled until tool permissions, role budgets, lanes, and terminal schemas are promoted",
    ),
];

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphRuntimeApplicationPromotionGapClosurePreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub upstream_runtime_rerun_gate: &'static str,
    pub runtime_application_primary_residual_source_count: usize,
    pub runtime_application_closure_source_count: usize,
    pub operator_review_decision_source_count: usize,
    pub promotion_plan_count: usize,
    pub promotion_domain_count: usize,
    pub promotion_binding_count: usize,
    pub readback_probe_binding_count: usize,
    pub evidence_field_ref_count: usize,
    pub promotion_group_count: usize,
    pub guard_count: usize,
    pub blocker_count: usize,
    pub required_prior_gate_count: usize,
    pub promotion_plans: Vec<WorkGraphRuntimeApplicationPromotionClosurePlanPreview>,
    pub promotion_bindings: Vec<WorkGraphRuntimeApplicationPromotionBindingPreview>,
    pub promotion_groups: Vec<WorkGraphRuntimeApplicationPromotionGroupPreview>,
    pub guards: Vec<WorkGraphRuntimeApplicationPromotionGuardPreview>,
    pub blockers: Vec<WorkGraphRuntimeApplicationPromotionBlockerPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub ready_for_runtime_application_promotion_readback_preview: bool,
    pub ready_for_runtime_application_promotion_application_preview: bool,
    pub ready_for_runtime_application_promotion: bool,
    pub ready_for_operator_review_side_effect_lock: bool,
    pub ready_for_append_only_store_enablement: bool,
    pub ready_for_projection_enforcement: bool,
    pub ready_for_live_execution: bool,
    pub side_effects: WorkGraphRuntimeApplicationPromotionGapClosurePreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphRuntimeApplicationPromotionClosurePlanPreview {
    pub closure_plan_id: String,
    pub source_surface_id: &'static str,
    pub source_category: &'static str,
    pub runtime_rerun_decision: &'static str,
    pub promotion_domain_ids: Vec<&'static str>,
    pub promotion_binding_ids: Vec<String>,
    pub readback_probe_id: String,
    pub evidence_field_ids: Vec<&'static str>,
    pub residual_source_blocker_ids: Vec<&'static str>,
    pub expected_post_closure_state: &'static str,
    pub closure_scope: &'static str,
    pub closure_state: &'static str,
    pub ready_for_readback_preview: bool,
    pub applies_to_runtime: bool,
    pub promotes_runtime_application: bool,
    pub attaches_runtime_wrapper: bool,
    pub enforces_scheduler_admission: bool,
    pub enforces_role_manifest: bool,
    pub mutates_store: bool,
    pub writes_wal: bool,
    pub records_approval: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphRuntimeApplicationPromotionBindingPreview {
    pub id: String,
    pub source_surface_id: &'static str,
    pub source_category: &'static str,
    pub promotion_domain_id: &'static str,
    pub closes_blocker_id: &'static str,
    pub required_evidence_field_ids: Vec<&'static str>,
    pub binding_state: &'static str,
    pub applies_to_runtime: bool,
    pub promotes_runtime_application: bool,
    pub writes_store: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphRuntimeApplicationPromotionGroupPreview {
    pub id: &'static str,
    pub priority: &'static str,
    pub promotion_domain_id: &'static str,
    pub closure_plan_ids: Vec<String>,
    pub promotion_binding_ids: Vec<String>,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub expected_contract_count_after_closure: usize,
    pub promotes_runtime_application: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphRuntimeApplicationPromotionGuardPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub scope: &'static str,
    pub enforced_in_preview: bool,
    pub prevents_runtime_mutation: bool,
    pub note: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphRuntimeApplicationPromotionBlockerPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub blocked_promotion_domain_ids: Vec<&'static str>,
    pub required_before_runtime_application_promotion: bool,
    pub recommended_fix: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphRuntimeApplicationPromotionGapClosurePreviewSideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub wal_written: bool,
    pub checkpoint_written: bool,
    pub durable_store_switch_enabled: bool,
    pub idempotency_index_mutated: bool,
    pub append_only_store_enabled: bool,
    pub runtime_application_promoted: bool,
    pub runtime_wrapper_attached: bool,
    pub scheduler_admission_enforced: bool,
    pub role_manifest_enforced: bool,
    pub task_result_enforcement_enabled: bool,
    pub task_result_persisted: bool,
    pub readback_executed: bool,
    pub rollback_executed: bool,
    pub approval_recorded: bool,
    pub operator_review_recorded: bool,
    pub runtime_mutation_performed: bool,
    pub agent_spawn_performed: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_runtime_application_promotion_gap_closure_preview_report()
-> WorkGraphRuntimeApplicationPromotionGapClosurePreviewReport {
    let upstream_decisions =
        work_graph_unified_projection_enforcement_append_only_store_runtime_rerun_source_decisions(
        );
    let promotion_plans =
        work_graph_runtime_application_promotion_gap_closure_plans_from(&upstream_decisions);
    let promotion_bindings =
        work_graph_runtime_application_promotion_gap_closure_bindings_from(&upstream_decisions);
    let promotion_groups =
        work_graph_runtime_application_promotion_gap_closure_groups_from(&upstream_decisions);
    let guards = work_graph_runtime_application_promotion_gap_closure_guards();
    let blockers = work_graph_runtime_application_promotion_gap_closure_blockers_from(
        &upstream_decisions,
        work_graph_unified_projection_enforcement_append_only_store_runtime_rerun_residual_blockers(
        ),
    );
    let required_prior_gates =
        work_graph_runtime_application_promotion_gap_closure_required_prior_gates();
    let runtime_application_primary_residual_source_count = upstream_decisions
        .iter()
        .filter(|decision| {
            decision
                .residual_source_blocker_ids
                .contains(&"runtime_application_residuals_not_promoted")
        })
        .count();
    let operator_review_decision_source_count = upstream_decisions
        .iter()
        .filter(|decision| {
            decision.append_only_store_runtime_rerun_enforcement_decision
                == "deny_operator_review_required"
        })
        .count();
    let evidence_field_ref_count = promotion_plans
        .iter()
        .map(|plan| plan.evidence_field_ids.len())
        .sum();

    WorkGraphRuntimeApplicationPromotionGapClosurePreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "blocked",
        gate: WORK_GRAPH_RUNTIME_APPLICATION_PROMOTION_GAP_CLOSURE_PREVIEW_GATE,
        schema_version: WORK_GRAPH_RUNTIME_APPLICATION_PROMOTION_GAP_CLOSURE_SCHEMA_VERSION,
        preview_mode: "read_only_runtime_application_promotion_gap_closure_no_enforcement",
        upstream_runtime_rerun_gate:
            WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_APPEND_ONLY_STORE_RUNTIME_RERUN_PREVIEW_GATE,
        runtime_application_primary_residual_source_count,
        runtime_application_closure_source_count: promotion_plans.len(),
        operator_review_decision_source_count,
        promotion_plan_count: promotion_plans.len(),
        promotion_domain_count: PROMOTION_DOMAINS.len(),
        promotion_binding_count: promotion_bindings.len(),
        readback_probe_binding_count: promotion_plans.len(),
        evidence_field_ref_count,
        promotion_group_count: promotion_groups.len(),
        guard_count: guards.len(),
        blocker_count: blockers.len(),
        required_prior_gate_count: required_prior_gates.len(),
        promotion_plans,
        promotion_bindings,
        promotion_groups,
        guards,
        blockers,
        required_prior_gates,
        recommended_next_gate:
            WORK_GRAPH_RUNTIME_APPLICATION_PROMOTION_GAP_CLOSURE_RECOMMENDED_NEXT_GATE,
        ready_for_runtime_application_promotion_readback_preview: true,
        ready_for_runtime_application_promotion_application_preview: false,
        ready_for_runtime_application_promotion: false,
        ready_for_operator_review_side_effect_lock: false,
        ready_for_append_only_store_enablement: false,
        ready_for_projection_enforcement: false,
        ready_for_live_execution: false,
        side_effects: WorkGraphRuntimeApplicationPromotionGapClosurePreviewSideEffects::none(),
    }
}

pub fn work_graph_runtime_application_promotion_gap_closure_plans()
-> Vec<WorkGraphRuntimeApplicationPromotionClosurePlanPreview> {
    let decisions =
        work_graph_unified_projection_enforcement_append_only_store_runtime_rerun_source_decisions(
        );
    work_graph_runtime_application_promotion_gap_closure_plans_from(&decisions)
}

pub fn work_graph_runtime_application_promotion_gap_closure_bindings()
-> Vec<WorkGraphRuntimeApplicationPromotionBindingPreview> {
    let decisions =
        work_graph_unified_projection_enforcement_append_only_store_runtime_rerun_source_decisions(
        );
    work_graph_runtime_application_promotion_gap_closure_bindings_from(&decisions)
}

pub fn work_graph_runtime_application_promotion_gap_closure_groups()
-> Vec<WorkGraphRuntimeApplicationPromotionGroupPreview> {
    let decisions =
        work_graph_unified_projection_enforcement_append_only_store_runtime_rerun_source_decisions(
        );
    work_graph_runtime_application_promotion_gap_closure_groups_from(&decisions)
}

pub fn work_graph_runtime_application_promotion_gap_closure_guards()
-> Vec<WorkGraphRuntimeApplicationPromotionGuardPreview> {
    vec![
        guard(
            "runtime_application_promotion_closure_preview_only",
            "critical",
            "closure",
        ),
        guard(
            "runtime_application_promotion_disabled",
            "critical",
            "runtime_application",
        ),
        guard("readback_execution_disabled", "critical", "readback"),
        guard("wal_write_boundary_disabled", "critical", "wal"),
        guard("durable_store_runtime_switch_disabled", "critical", "store"),
        guard("idempotency_mutation_disabled", "critical", "idempotency"),
        guard(
            "rollback_readback_execution_disabled",
            "critical",
            "rollback",
        ),
        guard("task_result_persistence_disabled", "high", "task_result"),
        guard(
            "scheduler_admission_runtime_enforcement_disabled",
            "high",
            "scheduler",
        ),
        guard(
            "role_manifest_runtime_enforcement_disabled",
            "high",
            "role_manifest",
        ),
        guard("operator_review_bypass_disabled", "high", "operator_review"),
    ]
}

pub fn work_graph_runtime_application_promotion_gap_closure_blockers()
-> Vec<WorkGraphRuntimeApplicationPromotionBlockerPreview> {
    let decisions =
        work_graph_unified_projection_enforcement_append_only_store_runtime_rerun_source_decisions(
        );
    work_graph_runtime_application_promotion_gap_closure_blockers_from(
        &decisions,
        work_graph_unified_projection_enforcement_append_only_store_runtime_rerun_residual_blockers(
        ),
    )
}

pub fn work_graph_runtime_application_promotion_gap_closure_required_prior_gates()
-> Vec<&'static str> {
    let mut gates =
        work_graph_unified_projection_enforcement_readiness_append_only_store_runtime_rerun_required_prior_gates();
    gates.push(WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_APPEND_ONLY_STORE_RUNTIME_RERUN_PREVIEW_GATE);
    gates
}

impl WorkGraphRuntimeApplicationPromotionGapClosurePreviewSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            wal_written: false,
            checkpoint_written: false,
            durable_store_switch_enabled: false,
            idempotency_index_mutated: false,
            append_only_store_enabled: false,
            runtime_application_promoted: false,
            runtime_wrapper_attached: false,
            scheduler_admission_enforced: false,
            role_manifest_enforced: false,
            task_result_enforcement_enabled: false,
            task_result_persisted: false,
            readback_executed: false,
            rollback_executed: false,
            approval_recorded: false,
            operator_review_recorded: false,
            runtime_mutation_performed: false,
            agent_spawn_performed: false,
            external_send_performed: false,
            model_invoked: false,
        }
    }
}

fn work_graph_runtime_application_promotion_gap_closure_plans_from(
    decisions: &[WorkGraphAppendOnlyStoreRuntimeRerunSourceDecisionPreview],
) -> Vec<WorkGraphRuntimeApplicationPromotionClosurePlanPreview> {
    decisions
        .iter()
        .filter(|decision| !promotion_domain_ids_for(decision).is_empty())
        .map(|decision| {
            let promotion_domain_ids = promotion_domain_ids_for(decision);
            let promotion_binding_ids = promotion_domain_ids
                .iter()
                .map(|domain_id| promotion_binding_id(decision.source_surface_id, domain_id))
                .collect::<Vec<_>>();
            WorkGraphRuntimeApplicationPromotionClosurePlanPreview {
                closure_plan_id: closure_plan_id(decision.source_surface_id),
                source_surface_id: decision.source_surface_id,
                source_category: decision.source_category,
                runtime_rerun_decision: decision.append_only_store_runtime_rerun_enforcement_decision,
                promotion_domain_ids,
                promotion_binding_ids,
                readback_probe_id: readback_probe_id(decision.source_surface_id),
                evidence_field_ids: PROMOTION_EVIDENCE_FIELD_IDS.to_vec(),
                residual_source_blocker_ids: decision.residual_source_blocker_ids.clone(),
                expected_post_closure_state:
                    "runtime_application_promotion_contract_ready_preview_after_application",
                closure_scope: "runtime_application_promotion_gap_closure",
                closure_state: "readback_pending_no_mutation",
                ready_for_readback_preview: true,
                applies_to_runtime: false,
                promotes_runtime_application: false,
                attaches_runtime_wrapper: false,
                enforces_scheduler_admission: false,
                enforces_role_manifest: false,
                mutates_store: false,
                writes_wal: false,
                records_approval: false,
            }
        })
        .collect()
}

fn work_graph_runtime_application_promotion_gap_closure_bindings_from(
    decisions: &[WorkGraphAppendOnlyStoreRuntimeRerunSourceDecisionPreview],
) -> Vec<WorkGraphRuntimeApplicationPromotionBindingPreview> {
    let mut bindings = Vec::new();
    for decision in decisions {
        for (domain_id, blocker_id, _, _) in PROMOTION_DOMAINS {
            if decision.residual_source_blocker_ids.contains(&blocker_id) {
                bindings.push(WorkGraphRuntimeApplicationPromotionBindingPreview {
                    id: promotion_binding_id(decision.source_surface_id, domain_id),
                    source_surface_id: decision.source_surface_id,
                    source_category: decision.source_category,
                    promotion_domain_id: domain_id,
                    closes_blocker_id: blocker_id,
                    required_evidence_field_ids: PROMOTION_EVIDENCE_FIELD_IDS.to_vec(),
                    binding_state: "readback_pending_no_mutation",
                    applies_to_runtime: false,
                    promotes_runtime_application: false,
                    writes_store: false,
                });
            }
        }
    }
    bindings
}

fn work_graph_runtime_application_promotion_gap_closure_groups_from(
    decisions: &[WorkGraphAppendOnlyStoreRuntimeRerunSourceDecisionPreview],
) -> Vec<WorkGraphRuntimeApplicationPromotionGroupPreview> {
    PROMOTION_DOMAINS
        .iter()
        .map(|(domain_id, blocker_id, priority, _)| {
            let affected_source_surface_ids = decisions
                .iter()
                .filter(|decision| decision.residual_source_blocker_ids.contains(blocker_id))
                .map(|decision| decision.source_surface_id)
                .collect::<Vec<_>>();
            let closure_plan_ids = affected_source_surface_ids
                .iter()
                .map(|source| closure_plan_id(source))
                .collect::<Vec<_>>();
            let promotion_binding_ids = affected_source_surface_ids
                .iter()
                .map(|source| promotion_binding_id(source, domain_id))
                .collect::<Vec<_>>();
            WorkGraphRuntimeApplicationPromotionGroupPreview {
                id: runtime_application_promotion_group_id(domain_id),
                priority,
                promotion_domain_id: domain_id,
                expected_contract_count_after_closure: affected_source_surface_ids.len(),
                closure_plan_ids,
                promotion_binding_ids,
                affected_source_surface_ids,
                promotes_runtime_application: false,
            }
        })
        .collect()
}

fn work_graph_runtime_application_promotion_gap_closure_blockers_from(
    decisions: &[WorkGraphAppendOnlyStoreRuntimeRerunSourceDecisionPreview],
    upstream_blockers: Vec<WorkGraphAppendOnlyStoreRuntimeRerunResidualBlockerPreview>,
) -> Vec<WorkGraphRuntimeApplicationPromotionBlockerPreview> {
    let mut blockers = upstream_blockers
        .into_iter()
        .map(
            |blocker| WorkGraphRuntimeApplicationPromotionBlockerPreview {
                id: blocker.id,
                severity: blocker.severity,
                affected_source_surface_ids: blocker.affected_source_surface_ids,
                blocked_promotion_domain_ids: blocked_promotion_domains_for(blocker.id),
                required_before_runtime_application_promotion: true,
                recommended_fix: blocker.recommended_fix,
            },
        )
        .collect::<Vec<_>>();
    blockers.push(WorkGraphRuntimeApplicationPromotionBlockerPreview {
        id: "runtime_application_promotion_readback_missing",
        severity: "high",
        affected_source_surface_ids: decisions
            .iter()
            .filter(|decision| !promotion_domain_ids_for(decision).is_empty())
            .map(|decision| decision.source_surface_id)
            .collect(),
        blocked_promotion_domain_ids: PROMOTION_DOMAINS
            .iter()
            .map(|(domain_id, _, _, _)| *domain_id)
            .collect(),
        required_before_runtime_application_promotion: true,
        recommended_fix:
            "read back every runtime application promotion closure plan before application preview",
    });
    blockers
}

fn promotion_domain_ids_for(
    decision: &WorkGraphAppendOnlyStoreRuntimeRerunSourceDecisionPreview,
) -> Vec<&'static str> {
    PROMOTION_DOMAINS
        .iter()
        .filter(|(_, blocker_id, _, _)| decision.residual_source_blocker_ids.contains(blocker_id))
        .map(|(domain_id, _, _, _)| *domain_id)
        .collect()
}

fn blocked_promotion_domains_for(blocker_id: &'static str) -> Vec<&'static str> {
    let domains = PROMOTION_DOMAINS
        .iter()
        .filter(|(domain_id, domain_blocker_id, _, _)| {
            blocker_id == *domain_blocker_id
                || blocker_id == "runtime_application_residuals_not_promoted"
                || blocker_id == "readback_execution_disabled"
                || blocker_id == "durable_store_runtime_switch_disabled"
                || blocker_id == "wal_write_boundary_not_enabled"
                || blocker_id == "idempotency_index_mutation_disabled"
                || blocker_id == "rollback_readback_not_executed"
                || blocker_id == "operator_review_required"
                || (*domain_id == "terminal_task_result_runtime_wrapper"
                    && blocker_id == "task_result_persistence_disabled")
        })
        .map(|(domain_id, _, _, _)| *domain_id)
        .collect::<Vec<_>>();
    if domains.is_empty() {
        vec!["runtime_application_promotion"]
    } else {
        domains
    }
}

fn guard(
    id: &'static str,
    severity: &'static str,
    scope: &'static str,
) -> WorkGraphRuntimeApplicationPromotionGuardPreview {
    WorkGraphRuntimeApplicationPromotionGuardPreview {
        id,
        severity,
        scope,
        enforced_in_preview: true,
        prevents_runtime_mutation: true,
        note: "preview records contracts only and does not promote runtime application",
    }
}

fn closure_plan_id(source: &str) -> String {
    format!("runtime_application_promotion_closure_plan__{source}")
}

fn promotion_binding_id(source: &str, domain_id: &str) -> String {
    format!("runtime_application_promotion_binding__{source}__{domain_id}")
}

fn readback_probe_id(source: &str) -> String {
    format!("runtime_application_promotion_readback_probe__{source}")
}

fn runtime_application_promotion_group_id(domain_id: &'static str) -> &'static str {
    match domain_id {
        "projection_adapter_runtime_closure" => {
            "projection_adapter_runtime_closure_promotion_group"
        }
        "store_guard_runtime_application" => "store_guard_runtime_application_promotion_group",
        "terminal_task_result_runtime_wrapper" => {
            "terminal_task_result_runtime_wrapper_promotion_group"
        }
        "scheduler_admission_runtime_application" => {
            "scheduler_admission_runtime_application_promotion_group"
        }
        "role_manifest_runtime_application" => "role_manifest_runtime_application_promotion_group",
        _ => "runtime_application_promotion_group",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runtime_application_promotion_closure_covers_all_runtime_application_domains() {
        let report = hepta_work_graph_runtime_application_promotion_gap_closure_preview_report();
        let domain_counts = report
            .promotion_groups
            .iter()
            .map(|group| {
                (
                    group.promotion_domain_id,
                    group.affected_source_surface_ids.len(),
                )
            })
            .collect::<Vec<_>>();

        assert_eq!(report.runtime_application_primary_residual_source_count, 7);
        assert_eq!(report.operator_review_decision_source_count, 5);
        assert_eq!(report.runtime_application_closure_source_count, 12);
        assert_eq!(report.promotion_plan_count, 12);
        assert_eq!(report.promotion_domain_count, 5);
        assert_eq!(report.promotion_binding_count, 27);
        assert_eq!(report.readback_probe_binding_count, 12);
        assert_eq!(report.evidence_field_ref_count, 96);
        assert_eq!(
            domain_counts,
            [
                ("projection_adapter_runtime_closure", 7),
                ("store_guard_runtime_application", 5),
                ("terminal_task_result_runtime_wrapper", 6),
                ("scheduler_admission_runtime_application", 5),
                ("role_manifest_runtime_application", 4),
            ]
        );
    }

    #[test]
    fn runtime_application_promotion_closure_keeps_runtime_mutation_disabled() {
        let report = hepta_work_graph_runtime_application_promotion_gap_closure_preview_report();

        assert_eq!(report.guard_count, 11);
        assert_eq!(report.blocker_count, 13);
        assert_eq!(report.required_prior_gate_count, 44);
        assert_eq!(
            report.required_prior_gates.last(),
            Some(
                &WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_APPEND_ONLY_STORE_RUNTIME_RERUN_PREVIEW_GATE
            )
        );
        assert!(report.ready_for_runtime_application_promotion_readback_preview);
        assert!(!report.ready_for_runtime_application_promotion_application_preview);
        assert!(!report.ready_for_runtime_application_promotion);
        assert!(!report.ready_for_operator_review_side_effect_lock);
        assert_eq!(
            report.recommended_next_gate,
            WORK_GRAPH_RUNTIME_APPLICATION_PROMOTION_GAP_CLOSURE_RECOMMENDED_NEXT_GATE
        );
        assert_eq!(
            report.side_effects,
            WorkGraphRuntimeApplicationPromotionGapClosurePreviewSideEffects::none()
        );
        assert!(report.promotion_plans.iter().all(|plan| {
            plan.ready_for_readback_preview
                && !plan.applies_to_runtime
                && !plan.promotes_runtime_application
                && !plan.mutates_store
                && !plan.writes_wal
                && !plan.records_approval
        }));
    }
}

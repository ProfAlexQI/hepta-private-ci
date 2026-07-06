use serde::Serialize;

use crate::work_graph_unified_projection_enforcement_readiness_role_manifest_rerun_preview::WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_ROLE_MANIFEST_RERUN_PREVIEW_GATE;
use crate::work_graph_unified_projection_enforcement_readiness_role_manifest_rerun_preview::WorkGraphRoleManifestRerunSourceDecisionPreview;
use crate::work_graph_unified_projection_enforcement_readiness_role_manifest_rerun_preview::work_graph_unified_projection_enforcement_readiness_role_manifest_rerun_required_prior_gates;
use crate::work_graph_unified_projection_enforcement_readiness_role_manifest_rerun_preview::work_graph_unified_projection_enforcement_role_manifest_rerun_residual_blockers;
use crate::work_graph_unified_projection_enforcement_readiness_role_manifest_rerun_preview::work_graph_unified_projection_enforcement_role_manifest_rerun_source_decisions;

pub const WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_ENABLEMENT_PREVIEW_GATE: &str =
    "hepta_work_graph_append_only_store_runtime_enablement_preview_gate";
pub const WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_ENABLEMENT_SCHEMA_VERSION: &str =
    "work_graph_append_only_store_runtime_enablement_preview_v1";
pub const WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_ENABLEMENT_RECOMMENDED_NEXT_GATE: &str =
    "hepta_work_graph_append_only_store_runtime_enablement_readback_preview_gate";

const RUNTIME_ENABLEMENT_STAGE_IDS: [&str; 6] = [
    "durable_store_runtime_switch",
    "wal_write_boundary",
    "idempotency_mutation_policy",
    "rollback_readback_execution_gate",
    "operator_review_side_effect_lock",
    "runtime_application_promotion",
];

const RUNTIME_ENABLEMENT_EVIDENCE_FIELDS: [&str; 8] = [
    "runtime_store_switch_contract_ref",
    "wal_write_boundary_ref",
    "idempotency_mutation_policy_ref",
    "rollback_readback_gate_ref",
    "operator_review_side_effect_lock_ref",
    "runtime_application_promotion_ref",
    "previous_role_manifest_rerun_decision_ref",
    "no_mutation_guard_ref",
];

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyStoreRuntimeEnablementPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub source_surface_count: usize,
    pub runtime_enablement_source_count: usize,
    pub runtime_enablement_plan_count: usize,
    pub runtime_stage_plan_count: usize,
    pub runtime_stage_source_ref_count: usize,
    pub runtime_stage_contract_ref_count: usize,
    pub runtime_plan_stage_ref_count: usize,
    pub runtime_plan_evidence_field_ref_count: usize,
    pub runtime_application_residual_source_count: usize,
    pub operator_review_residual_source_count: usize,
    pub guard_count: usize,
    pub blocker_count: usize,
    pub required_prior_gate_count: usize,
    pub runtime_enablement_plans: Vec<WorkGraphAppendOnlyStoreRuntimeEnablementSourcePlanPreview>,
    pub runtime_stage_plans: Vec<WorkGraphAppendOnlyStoreRuntimeEnablementStagePlanPreview>,
    pub guards: Vec<WorkGraphAppendOnlyStoreRuntimeEnablementGuardPreview>,
    pub blockers: Vec<WorkGraphAppendOnlyStoreRuntimeEnablementBlockerPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub ready_for_append_only_store_runtime_enablement_readback_preview: bool,
    pub ready_for_append_only_store_enablement: bool,
    pub ready_for_projection_enforcement: bool,
    pub ready_for_scheduler_admission_enforcement: bool,
    pub ready_for_role_manifest_enforcement: bool,
    pub ready_for_live_execution: bool,
    pub side_effects: WorkGraphAppendOnlyStoreRuntimeEnablementPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyStoreRuntimeEnablementSourcePlanPreview {
    pub source_surface_id: &'static str,
    pub source_category: &'static str,
    pub runtime_enablement_plan_id: String,
    pub previous_enforcement_decision: &'static str,
    pub runtime_enablement_state: &'static str,
    pub required_runtime_stage_ids: Vec<&'static str>,
    pub residual_source_blocker_ids: Vec<&'static str>,
    pub expected_evidence_field_ids: Vec<&'static str>,
    pub runtime_enablement_contract_ready_preview: bool,
    pub applies_to_runtime: bool,
    pub enables_append_only_store: bool,
    pub writes_wal: bool,
    pub writes_checkpoint: bool,
    pub mutates_idempotency_index: bool,
    pub executes_readback: bool,
    pub executes_rollback: bool,
    pub records_approval: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyStoreRuntimeEnablementStagePlanPreview {
    pub id: &'static str,
    pub priority: &'static str,
    pub category: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub required_contract_ref_ids: Vec<&'static str>,
    pub expected_runtime_state: &'static str,
    pub prerequisite_gate_ids: Vec<&'static str>,
    pub contract_ready_preview: bool,
    pub runtime_enabled_after_preview: bool,
    pub writes_wal: bool,
    pub mutates_idempotency_index: bool,
    pub executes_readback: bool,
    pub requires_operator_review: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyStoreRuntimeEnablementGuardPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub guard_scope: &'static str,
    pub required_before_runtime_enablement: bool,
    pub satisfied_by_preview: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyStoreRuntimeEnablementBlockerPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub category: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub affected_runtime_stage_ids: Vec<&'static str>,
    pub affected_runtime_enablement_plan_ids: Vec<String>,
    pub required_before_runtime_enablement: bool,
    pub recommended_fix: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyStoreRuntimeEnablementPreviewSideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub wal_written: bool,
    pub checkpoint_written: bool,
    pub idempotency_index_mutated: bool,
    pub append_only_store_enabled: bool,
    pub projection_enforcement_enabled: bool,
    pub scheduler_admission_enforced: bool,
    pub role_manifest_enforcement_enabled: bool,
    pub task_result_enforcement_enabled: bool,
    pub task_result_persisted: bool,
    pub readback_executed: bool,
    pub rollback_executed: bool,
    pub runtime_application_promoted: bool,
    pub approval_recorded: bool,
    pub runtime_mutation_performed: bool,
    pub agent_spawn_performed: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_append_only_store_runtime_enablement_preview_report()
-> WorkGraphAppendOnlyStoreRuntimeEnablementPreviewReport {
    let source_decisions =
        work_graph_unified_projection_enforcement_role_manifest_rerun_source_decisions();
    let runtime_enablement_plans = work_graph_append_only_store_runtime_enablement_source_plans();
    let runtime_stage_plans = work_graph_append_only_store_runtime_enablement_stage_plans();
    let guards = work_graph_append_only_store_runtime_enablement_guards();
    let blockers = work_graph_append_only_store_runtime_enablement_blockers();
    let required_prior_gates =
        work_graph_append_only_store_runtime_enablement_required_prior_gates();
    let runtime_application_residual_sources =
        sources_for_upstream_blocker("runtime_application_residuals_not_promoted");
    let operator_review_sources = sources_for_upstream_blocker("operator_review_required");

    WorkGraphAppendOnlyStoreRuntimeEnablementPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "blocked",
        gate: WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_ENABLEMENT_PREVIEW_GATE,
        schema_version: WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_ENABLEMENT_SCHEMA_VERSION,
        preview_mode: "read_only_append_only_store_runtime_enablement_preview_no_store_or_runtime_mutation",
        source_surface_count: source_decisions.len(),
        runtime_enablement_source_count: runtime_enablement_plans.len(),
        runtime_enablement_plan_count: runtime_enablement_plans.len(),
        runtime_stage_plan_count: runtime_stage_plans.len(),
        runtime_stage_source_ref_count: runtime_stage_plans
            .iter()
            .map(|stage| stage.affected_source_surface_ids.len())
            .sum(),
        runtime_stage_contract_ref_count: runtime_stage_plans
            .iter()
            .map(|stage| stage.required_contract_ref_ids.len())
            .sum(),
        runtime_plan_stage_ref_count: runtime_enablement_plans
            .iter()
            .map(|plan| plan.required_runtime_stage_ids.len())
            .sum(),
        runtime_plan_evidence_field_ref_count: runtime_enablement_plans
            .iter()
            .map(|plan| plan.expected_evidence_field_ids.len())
            .sum(),
        runtime_application_residual_source_count: runtime_application_residual_sources.len(),
        operator_review_residual_source_count: operator_review_sources.len(),
        guard_count: guards.len(),
        blocker_count: blockers.len(),
        required_prior_gate_count: required_prior_gates.len(),
        runtime_enablement_plans,
        runtime_stage_plans,
        guards,
        blockers,
        required_prior_gates,
        recommended_next_gate:
            WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_ENABLEMENT_RECOMMENDED_NEXT_GATE,
        ready_for_append_only_store_runtime_enablement_readback_preview: true,
        ready_for_append_only_store_enablement: false,
        ready_for_projection_enforcement: false,
        ready_for_scheduler_admission_enforcement: false,
        ready_for_role_manifest_enforcement: false,
        ready_for_live_execution: false,
        side_effects: WorkGraphAppendOnlyStoreRuntimeEnablementPreviewSideEffects::none(),
    }
}

pub fn work_graph_append_only_store_runtime_enablement_source_plans()
-> Vec<WorkGraphAppendOnlyStoreRuntimeEnablementSourcePlanPreview> {
    work_graph_unified_projection_enforcement_role_manifest_rerun_source_decisions()
        .into_iter()
        .filter(|decision| {
            has_blocker(
                &decision.residual_source_blocker_ids,
                "append_only_store_runtime_enablement_disabled",
            )
        })
        .map(runtime_enablement_plan)
        .collect()
}

pub fn work_graph_append_only_store_runtime_enablement_stage_plans()
-> Vec<WorkGraphAppendOnlyStoreRuntimeEnablementStagePlanPreview> {
    vec![
        runtime_stage_plan(
            "durable_store_runtime_switch",
            "p0",
            "durable_store_switch",
            runtime_source_ids(),
            vec![
                "durable_store_enablement_switch_contract_ready",
                "state_store_persistence_contract_ready",
                "append_only_event_intake_contract_ready",
                "redacted_payload_contract_ready",
                "operator_disable_switch_contract_ready",
            ],
            false,
            false,
            false,
            false,
        ),
        runtime_stage_plan(
            "wal_write_boundary",
            "p0",
            "wal_boundary",
            runtime_source_ids(),
            vec![
                "wal_append_record_contract_ready",
                "wal_ordering_contract_ready",
                "wal_redaction_contract_ready",
                "checkpoint_boundary_contract_ready",
                "replay_cursor_contract_ready",
                "durable_store_no_rewrite_contract_ready",
            ],
            true,
            false,
            false,
            false,
        ),
        runtime_stage_plan(
            "idempotency_mutation_policy",
            "p0",
            "idempotency_policy",
            runtime_source_ids(),
            vec![
                "idempotency_key_formula_contract_ready",
                "idempotency_collision_policy_contract_ready",
                "idempotency_mutation_policy_contract_ready",
                "idempotency_replay_evidence_contract_ready",
                "idempotency_readback_probe_contract_ready",
            ],
            false,
            true,
            false,
            false,
        ),
        runtime_stage_plan(
            "rollback_readback_execution_gate",
            "p0",
            "rollback_readback",
            runtime_source_ids(),
            vec![
                "rollback_execution_gate_contract_ready",
                "readback_probe_execution_contract_ready",
                "replay_receipt_contract_ready",
                "checkpoint_readback_contract_ready",
                "rollback_non_promotion_contract_ready",
            ],
            false,
            false,
            true,
            false,
        ),
        runtime_stage_plan(
            "operator_review_side_effect_lock",
            "p0",
            "operator_review",
            sources_for_upstream_blocker("operator_review_required"),
            vec![
                "operator_review_receipt_contract_ready",
                "side_effect_lock_contract_ready",
                "approval_recording_boundary_contract_ready",
            ],
            false,
            false,
            false,
            true,
        ),
        runtime_stage_plan(
            "runtime_application_promotion",
            "p0",
            "runtime_application",
            sources_for_upstream_blocker("runtime_application_residuals_not_promoted"),
            vec![
                "projection_runtime_application_contract_ready",
                "store_guard_runtime_application_contract_ready",
                "terminal_task_result_runtime_application_contract_ready",
                "scheduler_admission_runtime_application_contract_ready",
                "role_manifest_runtime_application_contract_ready",
            ],
            false,
            false,
            false,
            false,
        ),
    ]
}

pub fn work_graph_append_only_store_runtime_enablement_guards()
-> Vec<WorkGraphAppendOnlyStoreRuntimeEnablementGuardPreview> {
    vec![
        guard(
            "runtime_enablement_preview_only",
            "medium",
            "preview_boundary",
        ),
        guard(
            "durable_store_switch_not_enabled",
            "critical",
            "durable_store_switch",
        ),
        guard("wal_write_boundary_not_enabled", "critical", "wal_boundary"),
        guard(
            "idempotency_index_mutation_disabled",
            "critical",
            "idempotency_index",
        ),
        guard(
            "rollback_readback_execution_disabled",
            "critical",
            "rollback_readback",
        ),
        guard("operator_review_required", "high", "operator_review"),
        guard(
            "runtime_application_promotion_disabled",
            "high",
            "runtime_application",
        ),
        guard(
            "scheduler_role_runtime_application_disabled",
            "high",
            "scheduler_role",
        ),
        guard(
            "append_only_store_readback_required",
            "high",
            "readback_preview",
        ),
        guard(
            "side_effect_lock_not_established",
            "critical",
            "side_effect_lock",
        ),
    ]
}

pub fn work_graph_append_only_store_runtime_enablement_blockers()
-> Vec<WorkGraphAppendOnlyStoreRuntimeEnablementBlockerPreview> {
    let runtime_sources = runtime_source_ids();
    vec![
        runtime_blocker(
            "durable_store_runtime_switch_disabled",
            "critical",
            "durable_store_switch",
            runtime_sources.clone(),
            vec!["durable_store_runtime_switch"],
            "keep the durable store runtime switch disabled until WAL, replay, operator-review, and rollback gates are promoted",
        ),
        upstream_runtime_blocker(
            "append_only_store_runtime_enablement_disabled",
            "critical",
            "durable_store_switch",
            vec!["durable_store_runtime_switch"],
            "promote the runtime switch only after all append-only runtime enablement readback assertions pass",
        ),
        upstream_runtime_blocker(
            "wal_write_boundary_not_enabled",
            "critical",
            "wal_boundary",
            vec!["wal_write_boundary"],
            "keep WAL writes disabled until write-boundary, ordering, redaction, checkpoint, and replay contracts are promoted",
        ),
        upstream_runtime_blocker(
            "idempotency_index_mutation_disabled",
            "critical",
            "idempotency_policy",
            vec!["idempotency_mutation_policy"],
            "keep idempotency indexes immutable until mutation policy, collision handling, and replay evidence are enforced",
        ),
        upstream_runtime_blocker(
            "rollback_readback_not_executed",
            "critical",
            "rollback_readback",
            vec!["rollback_readback_execution_gate"],
            "execute rollback and readback gates before any append-only store runtime enablement",
        ),
        upstream_runtime_blocker(
            "operator_review_required",
            "high",
            "operator_review",
            vec!["operator_review_side_effect_lock"],
            "require operator review and a side-effect lock before promotion from preview to runtime",
        ),
        upstream_runtime_blocker(
            "projection_adapter_runtime_closure_application_disabled",
            "high",
            "runtime_application",
            vec!["runtime_application_promotion"],
            "promote projection adapter runtime application only after readback and operator-review gates are satisfied",
        ),
        upstream_runtime_blocker(
            "store_guard_runtime_application_disabled",
            "high",
            "runtime_application",
            vec!["runtime_application_promotion"],
            "attach store idempotency guards only after durable-store intake and replay evidence are promoted",
        ),
        upstream_runtime_blocker(
            "terminal_task_result_runtime_application_disabled",
            "high",
            "runtime_application",
            vec!["runtime_application_promotion"],
            "attach terminal TaskResult wrappers only after persistence and runtime application gates are promoted",
        ),
        upstream_runtime_blocker(
            "scheduler_admission_runtime_application_disabled",
            "high",
            "runtime_application",
            vec!["runtime_application_promotion"],
            "apply scheduler admission runtime wiring only after leases, budgets, approvals, and store writes are promoted",
        ),
        upstream_runtime_blocker(
            "role_manifest_runtime_application_disabled",
            "high",
            "runtime_application",
            vec!["runtime_application_promotion"],
            "apply role manifests to runtime only after tool permissions, budgets, lanes, and terminal schema bindings are promoted",
        ),
        upstream_runtime_blocker(
            "runtime_application_residuals_not_promoted",
            "high",
            "runtime_application",
            vec!["runtime_application_promotion"],
            "promote projection, store-guard, TaskResult, scheduler, and role runtime applications before enabling the append-only store",
        ),
        runtime_blocker(
            "append_only_store_runtime_enablement_readback_missing",
            "high",
            "readback_preview",
            runtime_sources,
            RUNTIME_ENABLEMENT_STAGE_IDS.to_vec(),
            "read back runtime enablement plans before any append-only store runtime switch can be promoted",
        ),
    ]
}

pub fn work_graph_append_only_store_runtime_enablement_required_prior_gates() -> Vec<&'static str> {
    let mut gates =
        work_graph_unified_projection_enforcement_readiness_role_manifest_rerun_required_prior_gates();
    gates
        .push(WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_ROLE_MANIFEST_RERUN_PREVIEW_GATE);
    gates
}

fn runtime_enablement_plan(
    decision: WorkGraphRoleManifestRerunSourceDecisionPreview,
) -> WorkGraphAppendOnlyStoreRuntimeEnablementSourcePlanPreview {
    WorkGraphAppendOnlyStoreRuntimeEnablementSourcePlanPreview {
        source_surface_id: decision.source_surface_id,
        source_category: decision.source_category,
        runtime_enablement_plan_id: runtime_enablement_plan_id(decision.source_surface_id),
        previous_enforcement_decision: decision.role_manifest_rerun_enforcement_decision,
        runtime_enablement_state: "runtime_enablement_blocked_preview_only",
        required_runtime_stage_ids: RUNTIME_ENABLEMENT_STAGE_IDS.to_vec(),
        residual_source_blocker_ids: decision.residual_source_blocker_ids,
        expected_evidence_field_ids: RUNTIME_ENABLEMENT_EVIDENCE_FIELDS.to_vec(),
        runtime_enablement_contract_ready_preview: true,
        applies_to_runtime: false,
        enables_append_only_store: false,
        writes_wal: false,
        writes_checkpoint: false,
        mutates_idempotency_index: false,
        executes_readback: false,
        executes_rollback: false,
        records_approval: false,
    }
}

fn runtime_stage_plan(
    id: &'static str,
    priority: &'static str,
    category: &'static str,
    affected_source_surface_ids: Vec<&'static str>,
    required_contract_ref_ids: Vec<&'static str>,
    writes_wal: bool,
    mutates_idempotency_index: bool,
    executes_readback: bool,
    requires_operator_review: bool,
) -> WorkGraphAppendOnlyStoreRuntimeEnablementStagePlanPreview {
    WorkGraphAppendOnlyStoreRuntimeEnablementStagePlanPreview {
        id,
        priority,
        category,
        affected_source_surface_ids,
        required_contract_ref_ids,
        expected_runtime_state: "contract_ready_preview_runtime_disabled",
        prerequisite_gate_ids: work_graph_append_only_store_runtime_enablement_required_prior_gates(
        ),
        contract_ready_preview: true,
        runtime_enabled_after_preview: false,
        writes_wal,
        mutates_idempotency_index,
        executes_readback,
        requires_operator_review,
    }
}

fn guard(
    id: &'static str,
    severity: &'static str,
    guard_scope: &'static str,
) -> WorkGraphAppendOnlyStoreRuntimeEnablementGuardPreview {
    WorkGraphAppendOnlyStoreRuntimeEnablementGuardPreview {
        id,
        severity,
        guard_scope,
        required_before_runtime_enablement: true,
        satisfied_by_preview: false,
    }
}

fn upstream_runtime_blocker(
    id: &'static str,
    severity: &'static str,
    category: &'static str,
    affected_runtime_stage_ids: Vec<&'static str>,
    recommended_fix: &'static str,
) -> WorkGraphAppendOnlyStoreRuntimeEnablementBlockerPreview {
    runtime_blocker(
        id,
        severity,
        category,
        sources_for_upstream_blocker(id),
        affected_runtime_stage_ids,
        recommended_fix,
    )
}

fn runtime_blocker(
    id: &'static str,
    severity: &'static str,
    category: &'static str,
    affected_source_surface_ids: Vec<&'static str>,
    affected_runtime_stage_ids: Vec<&'static str>,
    recommended_fix: &'static str,
) -> WorkGraphAppendOnlyStoreRuntimeEnablementBlockerPreview {
    WorkGraphAppendOnlyStoreRuntimeEnablementBlockerPreview {
        id,
        severity,
        category,
        affected_runtime_enablement_plan_ids: affected_source_surface_ids
            .iter()
            .map(|source| runtime_enablement_plan_id(source))
            .collect(),
        affected_source_surface_ids,
        affected_runtime_stage_ids,
        required_before_runtime_enablement: true,
        recommended_fix,
    }
}

fn runtime_source_ids() -> Vec<&'static str> {
    work_graph_append_only_store_runtime_enablement_source_plans()
        .into_iter()
        .map(|plan| plan.source_surface_id)
        .collect()
}

fn sources_for_upstream_blocker(blocker_id: &str) -> Vec<&'static str> {
    work_graph_unified_projection_enforcement_role_manifest_rerun_residual_blockers()
        .into_iter()
        .find(|blocker| blocker.id == blocker_id)
        .map(|blocker| blocker.affected_source_surface_ids)
        .unwrap_or_default()
}

fn runtime_enablement_plan_id(source_surface_id: &str) -> String {
    format!("append_only_store_runtime_enablement_{source_surface_id}_preview")
}

fn has_blocker(blockers: &[&'static str], blocker_id: &str) -> bool {
    blockers.iter().any(|blocker| *blocker == blocker_id)
}

impl WorkGraphAppendOnlyStoreRuntimeEnablementPreviewSideEffects {
    const fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            wal_written: false,
            checkpoint_written: false,
            idempotency_index_mutated: false,
            append_only_store_enabled: false,
            projection_enforcement_enabled: false,
            scheduler_admission_enforced: false,
            role_manifest_enforcement_enabled: false,
            task_result_enforcement_enabled: false,
            task_result_persisted: false,
            readback_executed: false,
            rollback_executed: false,
            runtime_application_promoted: false,
            approval_recorded: false,
            runtime_mutation_performed: false,
            agent_spawn_performed: false,
            external_send_performed: false,
            model_invoked: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runtime_enablement_preview_derives_twelve_blocked_sources() {
        let report = hepta_work_graph_append_only_store_runtime_enablement_preview_report();

        assert_eq!(report.source_surface_count, 12);
        assert_eq!(report.runtime_enablement_source_count, 12);
        assert_eq!(report.runtime_enablement_plan_count, 12);
        assert_eq!(report.runtime_stage_plan_count, 6);
        assert_eq!(report.runtime_application_residual_source_count, 7);
        assert_eq!(report.operator_review_residual_source_count, 7);
        assert_eq!(report.status, "blocked");
        assert_eq!(
            report.recommended_next_gate,
            WORK_GRAPH_APPEND_ONLY_STORE_RUNTIME_ENABLEMENT_RECOMMENDED_NEXT_GATE
        );
        assert!(report.ready_for_append_only_store_runtime_enablement_readback_preview);
        assert!(!report.ready_for_append_only_store_enablement);
    }

    #[test]
    fn runtime_enablement_plans_keep_runtime_disabled() {
        let plans = work_graph_append_only_store_runtime_enablement_source_plans();

        assert_eq!(plans.len(), 12);
        assert!(plans.iter().all(|plan| {
            plan.previous_enforcement_decision
                == "deny_runtime_append_only_store_enablement_disabled"
                && plan.runtime_enablement_state == "runtime_enablement_blocked_preview_only"
                && plan.required_runtime_stage_ids == RUNTIME_ENABLEMENT_STAGE_IDS
                && plan.expected_evidence_field_ids == RUNTIME_ENABLEMENT_EVIDENCE_FIELDS
                && plan.runtime_enablement_contract_ready_preview
                && !plan.applies_to_runtime
                && !plan.enables_append_only_store
                && !plan.writes_wal
                && !plan.writes_checkpoint
                && !plan.mutates_idempotency_index
                && !plan.executes_readback
                && !plan.executes_rollback
                && !plan.records_approval
        }));
        assert_eq!(
            plans
                .iter()
                .filter(|plan| has_blocker(
                    &plan.residual_source_blocker_ids,
                    "scheduler_admission_runtime_application_disabled",
                ))
                .count(),
            5
        );
        assert_eq!(
            plans
                .iter()
                .filter(|plan| has_blocker(
                    &plan.residual_source_blocker_ids,
                    "role_manifest_runtime_application_disabled",
                ))
                .count(),
            4
        );
    }

    #[test]
    fn runtime_enablement_stages_capture_no_mutation_boundary() {
        let report = hepta_work_graph_append_only_store_runtime_enablement_preview_report();
        let stage_counts = report
            .runtime_stage_plans
            .iter()
            .map(|stage| (stage.id, stage.affected_source_surface_ids.len()))
            .collect::<Vec<_>>();

        assert_eq!(
            stage_counts,
            [
                ("durable_store_runtime_switch", 12),
                ("wal_write_boundary", 12),
                ("idempotency_mutation_policy", 12),
                ("rollback_readback_execution_gate", 12),
                ("operator_review_side_effect_lock", 7),
                ("runtime_application_promotion", 7),
            ]
        );
        assert_eq!(report.runtime_stage_source_ref_count, 62);
        assert_eq!(report.runtime_stage_contract_ref_count, 29);
        assert_eq!(report.runtime_plan_stage_ref_count, 72);
        assert_eq!(report.runtime_plan_evidence_field_ref_count, 96);
        assert!(report.runtime_stage_plans.iter().all(|stage| {
            stage.contract_ready_preview
                && !stage.runtime_enabled_after_preview
                && stage.prerequisite_gate_ids.last().copied()
                    == Some(WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_ROLE_MANIFEST_RERUN_PREVIEW_GATE)
        }));
    }

    #[test]
    fn runtime_enablement_blockers_preserve_all_residuals() {
        let report = hepta_work_graph_append_only_store_runtime_enablement_preview_report();
        let blocker_counts = report
            .blockers
            .iter()
            .map(|blocker| (blocker.id, blocker.affected_source_surface_ids.len()))
            .collect::<Vec<_>>();

        assert_eq!(
            blocker_counts,
            [
                ("durable_store_runtime_switch_disabled", 12),
                ("append_only_store_runtime_enablement_disabled", 12),
                ("wal_write_boundary_not_enabled", 12),
                ("idempotency_index_mutation_disabled", 12),
                ("rollback_readback_not_executed", 12),
                ("operator_review_required", 7),
                ("projection_adapter_runtime_closure_application_disabled", 7),
                ("store_guard_runtime_application_disabled", 5),
                ("terminal_task_result_runtime_application_disabled", 6),
                ("scheduler_admission_runtime_application_disabled", 5),
                ("role_manifest_runtime_application_disabled", 4),
                ("runtime_application_residuals_not_promoted", 7),
                ("append_only_store_runtime_enablement_readback_missing", 12),
            ]
        );
        assert_eq!(report.blocker_count, 13);
        assert!(
            report
                .blockers
                .iter()
                .all(|blocker| blocker.required_before_runtime_enablement)
        );
        assert_eq!(report.guard_count, 10);
        assert_eq!(report.required_prior_gate_count, 40);
    }

    #[test]
    fn runtime_enablement_preview_preserves_no_side_effect_boundary() {
        let report = hepta_work_graph_append_only_store_runtime_enablement_preview_report();

        assert_eq!(
            report.side_effects,
            WorkGraphAppendOnlyStoreRuntimeEnablementPreviewSideEffects::none()
        );
    }
}

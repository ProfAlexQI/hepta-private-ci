use std::collections::BTreeSet;

use serde::Serialize;

use crate::work_graph_append_only_event_intake_preview::WORK_GRAPH_APPEND_ONLY_EVENT_INTAKE_PREVIEW_GATE;
use crate::work_graph_append_only_event_intake_preview::work_graph_append_only_event_contracts;
use crate::work_graph_append_only_event_intake_preview::work_graph_append_only_event_routes;
use crate::work_graph_state_store_persistence_preview::work_graph_state_store_checkpoint_contracts;
use crate::work_graph_state_store_persistence_preview::work_graph_state_store_idempotency_guards;
use crate::work_graph_state_store_persistence_preview::work_graph_state_store_persistence_wal_operations;
use crate::work_graph_state_store_persistence_preview::work_graph_state_store_readback_probes;
use crate::work_graph_store_idempotency_guard_gap_closure_application_preview::work_graph_store_idempotency_guard_gap_closure_application_source_outcomes;
use crate::work_graph_terminal_task_result_enforcement_gap_closure_application_preview::work_graph_terminal_task_result_enforcement_gap_closure_application_source_outcomes;
use crate::work_graph_unified_projection_enforcement_readiness_terminal_task_result_rerun_preview::WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_TERMINAL_TASK_RESULT_RERUN_PREVIEW_GATE;
use crate::work_graph_unified_projection_enforcement_readiness_terminal_task_result_rerun_preview::WorkGraphTerminalTaskResultRerunSourceDecisionPreview;
use crate::work_graph_unified_projection_enforcement_readiness_terminal_task_result_rerun_preview::work_graph_unified_projection_enforcement_readiness_terminal_task_result_rerun_required_prior_gates;
use crate::work_graph_unified_projection_enforcement_readiness_terminal_task_result_rerun_preview::work_graph_unified_projection_enforcement_terminal_task_result_rerun_residual_blockers;
use crate::work_graph_unified_projection_enforcement_readiness_terminal_task_result_rerun_preview::work_graph_unified_projection_enforcement_terminal_task_result_rerun_source_decisions;

pub const WORK_GRAPH_APPEND_ONLY_STORE_ENABLEMENT_PRECONDITION_PREVIEW_GATE: &str =
    "hepta_work_graph_append_only_store_enablement_precondition_preview_gate";
pub const WORK_GRAPH_APPEND_ONLY_STORE_ENABLEMENT_PRECONDITION_SCHEMA_VERSION: &str =
    "work_graph_append_only_store_enablement_precondition_preview_v1";
pub const WORK_GRAPH_APPEND_ONLY_STORE_ENABLEMENT_PRECONDITION_RECOMMENDED_NEXT_GATE: &str =
    "hepta_work_graph_append_only_store_enablement_precondition_readback_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyStoreEnablementPreconditionPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub source_surface_count: usize,
    pub append_only_disabled_source_count: usize,
    pub source_precondition_decision_count: usize,
    pub append_only_store_precondition_ready_source_count: usize,
    pub append_only_store_precondition_blocked_source_count: usize,
    pub append_only_event_contract_count: usize,
    pub append_only_event_route_count: usize,
    pub wal_operation_count: usize,
    pub checkpoint_contract_count: usize,
    pub existing_idempotency_guard_count: usize,
    pub candidate_idempotency_guard_count: usize,
    pub combined_idempotency_guard_source_count: usize,
    pub readback_probe_count: usize,
    pub terminal_task_result_contract_ready_source_count: usize,
    pub precondition_count: usize,
    pub blocker_count: usize,
    pub enablement_stage_count: usize,
    pub required_prior_gate_count: usize,
    pub source_precondition_decisions:
        Vec<WorkGraphAppendOnlyStoreEnablementSourcePreconditionPreview>,
    pub preconditions: Vec<WorkGraphAppendOnlyStoreEnablementPreconditionPreview>,
    pub blockers: Vec<WorkGraphAppendOnlyStoreEnablementPreconditionBlockerPreview>,
    pub enablement_stages: Vec<WorkGraphAppendOnlyStoreEnablementStagePreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub ready_for_precondition_readback_preview: bool,
    pub ready_for_append_only_store_enablement: bool,
    pub ready_for_projection_enforcement: bool,
    pub ready_for_scheduler_admission_enforcement: bool,
    pub ready_for_role_manifest_enforcement: bool,
    pub ready_for_live_execution: bool,
    pub side_effects: WorkGraphAppendOnlyStoreEnablementPreconditionPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyStoreEnablementSourcePreconditionPreview {
    pub source_surface_id: &'static str,
    pub source_category: &'static str,
    pub previous_readiness_decision: &'static str,
    pub append_only_precondition_decision: &'static str,
    pub projection_contract_ready: bool,
    pub store_idempotency_guard_ready: bool,
    pub terminal_task_result_contract_ready: bool,
    pub append_only_route_ready: bool,
    pub readback_probe_contract_ready: bool,
    pub required_precondition_ids: Vec<&'static str>,
    pub blocker_ids: Vec<&'static str>,
    pub ready_for_append_only_store_enablement: bool,
    pub next_required_gate: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyStoreEnablementPreconditionPreview {
    pub id: &'static str,
    pub category: &'static str,
    pub severity: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub required_contract_refs: Vec<&'static str>,
    pub satisfied_by_preview_contracts: bool,
    pub satisfied_for_enablement: bool,
    pub blocker_id: &'static str,
    pub recommended_closure_gate: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyStoreEnablementPreconditionBlockerPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub category: &'static str,
    pub affected_precondition_ids: Vec<&'static str>,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub required_before_append_only_store_enablement: bool,
    pub recommended_fix: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyStoreEnablementStagePreview {
    pub id: &'static str,
    pub observed_contract_count: usize,
    pub preview_ready_contract_count: usize,
    pub enablement_ready_contract_count: usize,
    pub hard_blocker_ids: Vec<&'static str>,
    pub enablement_enabled: bool,
    pub next_gate: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyStoreEnablementPreconditionPreviewSideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub wal_written: bool,
    pub checkpoint_written: bool,
    pub idempotency_index_mutated: bool,
    pub append_only_store_enabled: bool,
    pub projection_enforcement_enabled: bool,
    pub readback_executed: bool,
    pub rollback_executed: bool,
    pub scheduler_admission_enforced: bool,
    pub role_manifest_enforcement_enabled: bool,
    pub task_result_enforcement_enabled: bool,
    pub runtime_wrapper_attached: bool,
    pub approval_recorded: bool,
    pub runtime_mutation_performed: bool,
    pub agent_spawn_performed: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_append_only_store_enablement_precondition_preview_report()
-> WorkGraphAppendOnlyStoreEnablementPreconditionPreviewReport {
    let source_precondition_decisions =
        work_graph_append_only_store_enablement_source_precondition_decisions();
    let preconditions = work_graph_append_only_store_enablement_preconditions();
    let blockers = work_graph_append_only_store_enablement_precondition_blockers();
    let enablement_stages = work_graph_append_only_store_enablement_stages();
    let required_prior_gates =
        work_graph_append_only_store_enablement_precondition_required_prior_gates();
    let append_only_event_contracts = work_graph_append_only_event_contracts();
    let append_only_event_routes = work_graph_append_only_event_routes();
    let wal_operations = work_graph_state_store_persistence_wal_operations();
    let checkpoint_contracts = work_graph_state_store_checkpoint_contracts();
    let existing_idempotency_guards = work_graph_state_store_idempotency_guards();
    let candidate_idempotency_guard_sources =
        work_graph_store_idempotency_guard_gap_closure_application_source_outcomes();
    let combined_idempotency_guard_source_count = existing_idempotency_guards
        .iter()
        .map(|guard| guard.source_surface_id)
        .chain(
            candidate_idempotency_guard_sources
                .iter()
                .map(|outcome| outcome.source_surface_id),
        )
        .collect::<BTreeSet<_>>()
        .len();
    let readback_probes = work_graph_state_store_readback_probes();
    let terminal_task_result_contract_ready_source_count =
        work_graph_terminal_task_result_enforcement_gap_closure_application_source_outcomes()
            .into_iter()
            .filter(|outcome| outcome.terminal_task_result_contract_ready_preview)
            .count();
    let append_only_store_precondition_ready_source_count = source_precondition_decisions
        .iter()
        .filter(|decision| decision.ready_for_append_only_store_enablement)
        .count();

    WorkGraphAppendOnlyStoreEnablementPreconditionPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "blocked",
        gate: WORK_GRAPH_APPEND_ONLY_STORE_ENABLEMENT_PRECONDITION_PREVIEW_GATE,
        schema_version: WORK_GRAPH_APPEND_ONLY_STORE_ENABLEMENT_PRECONDITION_SCHEMA_VERSION,
        preview_mode: "read_only_append_only_store_enablement_precondition_preview_no_store_enablement",
        source_surface_count: source_precondition_decisions.len(),
        append_only_disabled_source_count: source_precondition_decisions
            .iter()
            .filter(|decision| {
                decision
                    .blocker_ids
                    .contains(&"append_only_store_enablement_disabled")
            })
            .count(),
        source_precondition_decision_count: source_precondition_decisions.len(),
        append_only_store_precondition_ready_source_count,
        append_only_store_precondition_blocked_source_count: source_precondition_decisions.len()
            - append_only_store_precondition_ready_source_count,
        append_only_event_contract_count: append_only_event_contracts.len(),
        append_only_event_route_count: append_only_event_routes.len(),
        wal_operation_count: wal_operations.len(),
        checkpoint_contract_count: checkpoint_contracts.len(),
        existing_idempotency_guard_count: existing_idempotency_guards.len(),
        candidate_idempotency_guard_count: candidate_idempotency_guard_sources.len(),
        combined_idempotency_guard_source_count,
        readback_probe_count: readback_probes.len(),
        terminal_task_result_contract_ready_source_count,
        precondition_count: preconditions.len(),
        blocker_count: blockers.len(),
        enablement_stage_count: enablement_stages.len(),
        required_prior_gate_count: required_prior_gates.len(),
        source_precondition_decisions,
        preconditions,
        blockers,
        enablement_stages,
        required_prior_gates,
        recommended_next_gate:
            WORK_GRAPH_APPEND_ONLY_STORE_ENABLEMENT_PRECONDITION_RECOMMENDED_NEXT_GATE,
        ready_for_precondition_readback_preview: true,
        ready_for_append_only_store_enablement: false,
        ready_for_projection_enforcement: false,
        ready_for_scheduler_admission_enforcement: false,
        ready_for_role_manifest_enforcement: false,
        ready_for_live_execution: false,
        side_effects: WorkGraphAppendOnlyStoreEnablementPreconditionPreviewSideEffects::none(),
    }
}

pub fn work_graph_append_only_store_enablement_source_precondition_decisions()
-> Vec<WorkGraphAppendOnlyStoreEnablementSourcePreconditionPreview> {
    work_graph_unified_projection_enforcement_terminal_task_result_rerun_source_decisions()
        .into_iter()
        .map(append_only_store_enablement_source_decision)
        .collect()
}

pub fn work_graph_append_only_store_enablement_preconditions()
-> Vec<WorkGraphAppendOnlyStoreEnablementPreconditionPreview> {
    let decisions =
        work_graph_unified_projection_enforcement_terminal_task_result_rerun_source_decisions();
    let all_sources = all_source_surface_ids(&decisions);
    let scheduler_sources = affected_sources(&decisions, |decision| {
        has_suffix(
            &decision.residual_source_blocker_ids,
            "_admission_not_enforced",
        )
    });
    let role_manifest_sources = affected_sources(&decisions, |decision| {
        has_contains(
            &decision.residual_source_blocker_ids,
            "role_manifest_not_enforced",
        )
    });
    let terminal_sources = affected_sources(&decisions, |decision| {
        decision.covered_by_terminal_task_result_application_preview
    });
    let wal_operation_ids = work_graph_state_store_persistence_wal_operations()
        .into_iter()
        .map(|operation| operation.id)
        .collect::<Vec<_>>();
    let checkpoint_and_readback_refs = work_graph_state_store_checkpoint_contracts()
        .into_iter()
        .map(|checkpoint| checkpoint.id)
        .chain(
            work_graph_state_store_readback_probes()
                .into_iter()
                .map(|probe| probe.id),
        )
        .collect::<Vec<_>>();
    let idempotency_guard_refs = work_graph_state_store_idempotency_guards()
        .into_iter()
        .map(|guard| guard.id)
        .chain(
            work_graph_store_idempotency_guard_gap_closure_application_source_outcomes()
                .into_iter()
                .map(|outcome| outcome.candidate_guard_id),
        )
        .collect::<Vec<_>>();
    let event_contract_refs = work_graph_append_only_event_contracts()
        .into_iter()
        .map(|contract| contract.id)
        .collect::<Vec<_>>();

    vec![
        precondition(
            "durable_store_enablement_switch",
            "durable_store_switch",
            "critical",
            all_sources.clone(),
            event_contract_refs,
            true,
            false,
            "durable_store_enablement_disabled",
            "hepta_work_graph_append_only_store_enablement_switch_preview_gate",
        ),
        precondition(
            "wal_append_boundary_contract",
            "wal_boundary",
            "critical",
            all_sources.clone(),
            wal_operation_ids,
            true,
            false,
            "wal_write_boundary_not_enabled",
            "hepta_work_graph_append_only_store_wal_boundary_readback_preview_gate",
        ),
        precondition(
            "idempotency_mutation_policy",
            "idempotency_mutation_policy",
            "critical",
            all_sources.clone(),
            idempotency_guard_refs,
            true,
            false,
            "idempotency_index_mutation_disabled",
            "hepta_work_graph_append_only_store_idempotency_mutation_policy_preview_gate",
        ),
        precondition(
            "rollback_readback_gate",
            "rollback_readback_gate",
            "critical",
            all_sources.clone(),
            checkpoint_and_readback_refs,
            true,
            false,
            "rollback_readback_not_executed",
            "hepta_work_graph_append_only_store_rollback_readback_preview_gate",
        ),
        precondition(
            "operator_review_and_side_effect_lock",
            "operator_review",
            "high",
            terminal_sources,
            vec![
                "operator_review_required",
                "side_effect_lock_required",
                "runtime_application_receipts_required",
            ],
            false,
            false,
            "operator_review_required",
            "hepta_work_graph_append_only_store_operator_review_preview_gate",
        ),
        precondition(
            "scheduler_admission_enforcement_precondition",
            "scheduler_admission",
            "high",
            scheduler_sources,
            vec![
                "dependency_gate",
                "lease_gate",
                "budget_gate",
                "approval_gate",
                "idempotency_gate",
            ],
            false,
            false,
            "scheduler_admission_not_enforced",
            "hepta_work_graph_scheduler_admission_controller_preview_gate",
        ),
        precondition(
            "role_manifest_enforcement_precondition",
            "role_manifest",
            "medium",
            role_manifest_sources,
            vec![
                "role_capabilities",
                "tool_permissions",
                "budget_limits",
                "lane_boundaries",
            ],
            false,
            false,
            "role_manifest_not_enforced",
            "hepta_work_graph_role_manifest_contract_preview_gate",
        ),
    ]
}

pub fn work_graph_append_only_store_enablement_precondition_blockers()
-> Vec<WorkGraphAppendOnlyStoreEnablementPreconditionBlockerPreview> {
    let decisions =
        work_graph_unified_projection_enforcement_terminal_task_result_rerun_source_decisions();
    let preconditions = work_graph_append_only_store_enablement_preconditions();
    let all_sources = all_source_surface_ids(&decisions);
    let terminal_sources = affected_sources(&decisions, |decision| {
        decision.covered_by_terminal_task_result_application_preview
    });
    let scheduler_sources = affected_sources(&decisions, |decision| {
        has_suffix(
            &decision.residual_source_blocker_ids,
            "_admission_not_enforced",
        )
    });
    let role_manifest_sources = affected_sources(&decisions, |decision| {
        has_contains(
            &decision.residual_source_blocker_ids,
            "role_manifest_not_enforced",
        )
    });
    let projection_runtime_sources =
        residual_sources("projection_adapter_runtime_closure_application_disabled");
    let store_runtime_sources = residual_sources("store_guard_runtime_application_disabled");

    vec![
        blocker(
            "durable_store_enablement_disabled",
            "critical",
            "durable_store_switch",
            vec!["durable_store_enablement_switch"],
            all_sources.clone(),
            "keep append-only store disabled until the operator accepts durable write boundaries and rollback plan",
        ),
        blocker(
            "wal_write_boundary_not_enabled",
            "critical",
            "wal_boundary",
            vec!["wal_append_boundary_contract"],
            all_sources.clone(),
            "promote WAL append contracts only after readback and replay fixtures prove deterministic recovery",
        ),
        blocker(
            "idempotency_index_mutation_disabled",
            "critical",
            "idempotency_mutation_policy",
            vec!["idempotency_mutation_policy"],
            all_sources.clone(),
            "bind every source to a collision policy and mutation-safe idempotency index before writes",
        ),
        blocker(
            "rollback_readback_not_executed",
            "critical",
            "rollback_readback_gate",
            vec!["rollback_readback_gate"],
            all_sources.clone(),
            "execute readback, replay, and rollback fixtures before any append-only store enablement",
        ),
        blocker(
            "operator_review_required",
            "high",
            "operator_review",
            vec!["operator_review_and_side_effect_lock"],
            terminal_sources,
            "operator review must accept side-effect locks, terminal TaskResult persistence, and durable store switch",
        ),
        blocker(
            "scheduler_admission_not_enforced",
            "high",
            "scheduler_admission",
            vec!["scheduler_admission_enforcement_precondition"],
            scheduler_sources,
            "scheduler admission must enforce dependency, lease, budget, approval, and idempotency gates before work start",
        ),
        blocker(
            "role_manifest_not_enforced",
            "medium",
            "role_manifest",
            vec!["role_manifest_enforcement_precondition"],
            role_manifest_sources,
            "role manifests must bind capabilities, tools, budgets, reducers, and lane permissions before agent paths can append",
        ),
        blocker(
            "runtime_application_residuals_not_promoted",
            "high",
            "runtime_application",
            preconditions
                .iter()
                .map(|precondition| precondition.id)
                .collect(),
            union_sources(&projection_runtime_sources, &store_runtime_sources),
            "projection adapter closures and store guards remain preview-only and cannot write into WorkGraph runtime state",
        ),
    ]
}

pub fn work_graph_append_only_store_enablement_stages()
-> Vec<WorkGraphAppendOnlyStoreEnablementStagePreview> {
    let decisions =
        work_graph_unified_projection_enforcement_terminal_task_result_rerun_source_decisions();
    let event_contract_count = work_graph_append_only_event_contracts().len();
    let wal_operation_count = work_graph_state_store_persistence_wal_operations().len();
    let idempotency_source_count = work_graph_state_store_idempotency_guards()
        .into_iter()
        .map(|guard| guard.source_surface_id)
        .chain(
            work_graph_store_idempotency_guard_gap_closure_application_source_outcomes()
                .into_iter()
                .map(|outcome| outcome.source_surface_id),
        )
        .collect::<BTreeSet<_>>()
        .len();
    let readback_probe_count = work_graph_state_store_readback_probes().len();
    let scheduler_source_count = affected_sources(&decisions, |decision| {
        has_suffix(
            &decision.residual_source_blocker_ids,
            "_admission_not_enforced",
        )
    })
    .len();
    let role_manifest_source_count = affected_sources(&decisions, |decision| {
        has_contains(
            &decision.residual_source_blocker_ids,
            "role_manifest_not_enforced",
        )
    })
    .len();

    vec![
        stage(
            "contract_readiness_snapshot",
            decisions.len(),
            decisions.len(),
            0,
            vec!["durable_store_enablement_disabled"],
        ),
        stage(
            "append_only_event_intake_contracts",
            event_contract_count,
            event_contract_count,
            0,
            vec!["wal_write_boundary_not_enabled"],
        ),
        stage(
            "wal_and_idempotency_boundary",
            wal_operation_count + idempotency_source_count,
            wal_operation_count + idempotency_source_count,
            0,
            vec![
                "wal_write_boundary_not_enabled",
                "idempotency_index_mutation_disabled",
            ],
        ),
        stage(
            "rollback_readback_boundary",
            readback_probe_count,
            readback_probe_count,
            0,
            vec!["rollback_readback_not_executed"],
        ),
        stage(
            "admission_role_operator_policy",
            scheduler_source_count + role_manifest_source_count,
            0,
            0,
            vec![
                "scheduler_admission_not_enforced",
                "role_manifest_not_enforced",
                "operator_review_required",
            ],
        ),
    ]
}

pub fn work_graph_append_only_store_enablement_precondition_required_prior_gates()
-> Vec<&'static str> {
    let mut gates =
        work_graph_unified_projection_enforcement_readiness_terminal_task_result_rerun_required_prior_gates();
    push_unique(
        &mut gates,
        WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_TERMINAL_TASK_RESULT_RERUN_PREVIEW_GATE,
    );
    push_unique(&mut gates, WORK_GRAPH_APPEND_ONLY_EVENT_INTAKE_PREVIEW_GATE);
    gates
}

impl WorkGraphAppendOnlyStoreEnablementPreconditionPreviewSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            wal_written: false,
            checkpoint_written: false,
            idempotency_index_mutated: false,
            append_only_store_enabled: false,
            projection_enforcement_enabled: false,
            readback_executed: false,
            rollback_executed: false,
            scheduler_admission_enforced: false,
            role_manifest_enforcement_enabled: false,
            task_result_enforcement_enabled: false,
            runtime_wrapper_attached: false,
            approval_recorded: false,
            runtime_mutation_performed: false,
            agent_spawn_performed: false,
            external_send_performed: false,
            model_invoked: false,
        }
    }
}

fn append_only_store_enablement_source_decision(
    decision: WorkGraphTerminalTaskResultRerunSourceDecisionPreview,
) -> WorkGraphAppendOnlyStoreEnablementSourcePreconditionPreview {
    let blocker_ids = append_only_store_enablement_blockers_for_source(&decision);
    let append_only_precondition_decision =
        if blocker_ids.contains(&"scheduler_admission_not_enforced") {
            "deny_scheduler_admission_not_enforced"
        } else if blocker_ids.contains(&"role_manifest_not_enforced") {
            "deny_role_manifest_not_enforced"
        } else {
            "deny_append_only_store_enablement_preconditions_missing"
        };

    WorkGraphAppendOnlyStoreEnablementSourcePreconditionPreview {
        source_surface_id: decision.source_surface_id,
        source_category: decision.source_category,
        previous_readiness_decision: decision.terminal_task_result_rerun_enforcement_decision,
        append_only_precondition_decision,
        projection_contract_ready: decision.projection_contract_ready,
        store_idempotency_guard_ready: decision.store_idempotency_guard_ready,
        terminal_task_result_contract_ready: decision.terminal_task_result_contract_ready,
        append_only_route_ready: decision.append_only_route_ready,
        readback_probe_contract_ready: decision.readback_probe_contract_ready,
        required_precondition_ids: required_preconditions_for_source(&decision),
        blocker_ids,
        ready_for_append_only_store_enablement: false,
        next_required_gate:
            WORK_GRAPH_APPEND_ONLY_STORE_ENABLEMENT_PRECONDITION_RECOMMENDED_NEXT_GATE,
    }
}

fn required_preconditions_for_source(
    decision: &WorkGraphTerminalTaskResultRerunSourceDecisionPreview,
) -> Vec<&'static str> {
    let mut preconditions = vec![
        "durable_store_enablement_switch",
        "wal_append_boundary_contract",
        "idempotency_mutation_policy",
        "rollback_readback_gate",
    ];
    if decision.covered_by_terminal_task_result_application_preview {
        preconditions.push("operator_review_and_side_effect_lock");
    }
    if has_suffix(
        &decision.residual_source_blocker_ids,
        "_admission_not_enforced",
    ) {
        preconditions.push("scheduler_admission_enforcement_precondition");
    }
    if has_contains(
        &decision.residual_source_blocker_ids,
        "role_manifest_not_enforced",
    ) {
        preconditions.push("role_manifest_enforcement_precondition");
    }
    preconditions
}

fn append_only_store_enablement_blockers_for_source(
    decision: &WorkGraphTerminalTaskResultRerunSourceDecisionPreview,
) -> Vec<&'static str> {
    let mut blockers = vec![
        "durable_store_enablement_disabled",
        "wal_write_boundary_not_enabled",
        "idempotency_index_mutation_disabled",
        "rollback_readback_not_executed",
    ];
    if decision.covered_by_terminal_task_result_application_preview {
        blockers.push("operator_review_required");
    }
    if has_suffix(
        &decision.residual_source_blocker_ids,
        "_admission_not_enforced",
    ) {
        blockers.push("scheduler_admission_not_enforced");
    }
    if has_contains(
        &decision.residual_source_blocker_ids,
        "role_manifest_not_enforced",
    ) {
        blockers.push("role_manifest_not_enforced");
    }
    if decision
        .residual_route_blocker_ids
        .contains(&"append_only_store_disabled_by_design")
    {
        blockers.push("append_only_store_enablement_disabled");
    }
    blockers
}

fn residual_sources(blocker_id: &'static str) -> Vec<&'static str> {
    work_graph_unified_projection_enforcement_terminal_task_result_rerun_residual_blockers()
        .into_iter()
        .find(|blocker| blocker.id == blocker_id)
        .map(|blocker| blocker.affected_source_surface_ids)
        .unwrap_or_default()
}

fn all_source_surface_ids(
    decisions: &[WorkGraphTerminalTaskResultRerunSourceDecisionPreview],
) -> Vec<&'static str> {
    decisions
        .iter()
        .map(|decision| decision.source_surface_id)
        .collect()
}

fn affected_sources(
    decisions: &[WorkGraphTerminalTaskResultRerunSourceDecisionPreview],
    predicate: impl Fn(&WorkGraphTerminalTaskResultRerunSourceDecisionPreview) -> bool,
) -> Vec<&'static str> {
    decisions
        .iter()
        .filter(|decision| predicate(decision))
        .map(|decision| decision.source_surface_id)
        .collect()
}

fn union_sources(left: &[&'static str], right: &[&'static str]) -> Vec<&'static str> {
    left.iter()
        .chain(right.iter())
        .copied()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

fn precondition(
    id: &'static str,
    category: &'static str,
    severity: &'static str,
    affected_source_surface_ids: Vec<&'static str>,
    required_contract_refs: Vec<&'static str>,
    satisfied_by_preview_contracts: bool,
    satisfied_for_enablement: bool,
    blocker_id: &'static str,
    recommended_closure_gate: &'static str,
) -> WorkGraphAppendOnlyStoreEnablementPreconditionPreview {
    WorkGraphAppendOnlyStoreEnablementPreconditionPreview {
        id,
        category,
        severity,
        affected_source_surface_ids,
        required_contract_refs,
        satisfied_by_preview_contracts,
        satisfied_for_enablement,
        blocker_id,
        recommended_closure_gate,
    }
}

fn blocker(
    id: &'static str,
    severity: &'static str,
    category: &'static str,
    affected_precondition_ids: Vec<&'static str>,
    affected_source_surface_ids: Vec<&'static str>,
    recommended_fix: &'static str,
) -> WorkGraphAppendOnlyStoreEnablementPreconditionBlockerPreview {
    WorkGraphAppendOnlyStoreEnablementPreconditionBlockerPreview {
        id,
        severity,
        category,
        affected_precondition_ids,
        affected_source_surface_ids,
        required_before_append_only_store_enablement: true,
        recommended_fix,
    }
}

fn stage(
    id: &'static str,
    observed_contract_count: usize,
    preview_ready_contract_count: usize,
    enablement_ready_contract_count: usize,
    hard_blocker_ids: Vec<&'static str>,
) -> WorkGraphAppendOnlyStoreEnablementStagePreview {
    WorkGraphAppendOnlyStoreEnablementStagePreview {
        id,
        observed_contract_count,
        preview_ready_contract_count,
        enablement_ready_contract_count,
        hard_blocker_ids,
        enablement_enabled: false,
        next_gate: WORK_GRAPH_APPEND_ONLY_STORE_ENABLEMENT_PRECONDITION_RECOMMENDED_NEXT_GATE,
    }
}

fn push_unique(gates: &mut Vec<&'static str>, gate: &'static str) {
    if !gates.contains(&gate) {
        gates.push(gate);
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
    fn append_only_store_enablement_precondition_summarizes_ready_contracts() {
        let report = hepta_work_graph_append_only_store_enablement_precondition_preview_report();

        assert_eq!(report.source_surface_count, 12);
        assert_eq!(report.append_only_disabled_source_count, 12);
        assert_eq!(report.source_precondition_decision_count, 12);
        assert_eq!(report.append_only_store_precondition_ready_source_count, 0);
        assert_eq!(
            report.append_only_store_precondition_blocked_source_count,
            12
        );
        assert_eq!(report.append_only_event_contract_count, 9);
        assert_eq!(report.append_only_event_route_count, 12);
        assert_eq!(report.wal_operation_count, 6);
        assert_eq!(report.checkpoint_contract_count, 4);
        assert_eq!(report.existing_idempotency_guard_count, 12);
        assert_eq!(report.candidate_idempotency_guard_count, 5);
        assert_eq!(report.combined_idempotency_guard_source_count, 12);
        assert_eq!(report.readback_probe_count, 6);
        assert_eq!(report.terminal_task_result_contract_ready_source_count, 6);
    }

    #[test]
    fn append_only_store_enablement_precondition_declares_source_decisions() {
        let decisions = work_graph_append_only_store_enablement_source_precondition_decisions();
        let decision_counts = [
            "deny_append_only_store_enablement_preconditions_missing",
            "deny_scheduler_admission_not_enforced",
            "deny_role_manifest_not_enforced",
        ]
        .into_iter()
        .map(|decision| {
            (
                decision,
                decisions
                    .iter()
                    .filter(|source| source.append_only_precondition_decision == decision)
                    .count(),
            )
        })
        .collect::<Vec<_>>();

        assert_eq!(
            decision_counts,
            [
                ("deny_append_only_store_enablement_preconditions_missing", 6),
                ("deny_scheduler_admission_not_enforced", 5),
                ("deny_role_manifest_not_enforced", 1),
            ]
        );
        assert!(
            decisions
                .iter()
                .all(|decision| decision.projection_contract_ready
                    && decision.store_idempotency_guard_ready
                    && decision.terminal_task_result_contract_ready
                    && decision.append_only_route_ready
                    && decision.readback_probe_contract_ready
                    && !decision.ready_for_append_only_store_enablement)
        );
    }

    #[test]
    fn append_only_store_enablement_precondition_declares_hard_preconditions() {
        let report = hepta_work_graph_append_only_store_enablement_precondition_preview_report();
        let precondition_counts = report
            .preconditions
            .iter()
            .map(|precondition| {
                (
                    precondition.id,
                    precondition.affected_source_surface_ids.len(),
                    precondition.satisfied_by_preview_contracts,
                    precondition.satisfied_for_enablement,
                )
            })
            .collect::<Vec<_>>();

        assert_eq!(report.precondition_count, 7);
        assert_eq!(
            precondition_counts,
            [
                ("durable_store_enablement_switch", 12, true, false),
                ("wal_append_boundary_contract", 12, true, false),
                ("idempotency_mutation_policy", 12, true, false),
                ("rollback_readback_gate", 12, true, false),
                ("operator_review_and_side_effect_lock", 6, false, false),
                (
                    "scheduler_admission_enforcement_precondition",
                    5,
                    false,
                    false,
                ),
                ("role_manifest_enforcement_precondition", 4, false, false),
            ]
        );
    }

    #[test]
    fn append_only_store_enablement_precondition_keeps_blockers_explicit() {
        let report = hepta_work_graph_append_only_store_enablement_precondition_preview_report();
        let blocker_counts = report
            .blockers
            .iter()
            .map(|blocker| (blocker.id, blocker.affected_source_surface_ids.len()))
            .collect::<Vec<_>>();

        assert_eq!(report.blocker_count, 8);
        assert_eq!(
            blocker_counts,
            [
                ("durable_store_enablement_disabled", 12),
                ("wal_write_boundary_not_enabled", 12),
                ("idempotency_index_mutation_disabled", 12),
                ("rollback_readback_not_executed", 12),
                ("operator_review_required", 6),
                ("scheduler_admission_not_enforced", 5),
                ("role_manifest_not_enforced", 4),
                ("runtime_application_residuals_not_promoted", 7),
            ]
        );
        assert!(
            report
                .blockers
                .iter()
                .all(|blocker| blocker.required_before_append_only_store_enablement)
        );
    }

    #[test]
    fn append_only_store_enablement_precondition_declares_next_frontier_and_side_effects() {
        let report = hepta_work_graph_append_only_store_enablement_precondition_preview_report();

        assert_eq!(report.enablement_stage_count, 5);
        assert_eq!(report.required_prior_gate_count, 28);
        assert_eq!(
            report.required_prior_gates.last().copied(),
            Some(WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_TERMINAL_TASK_RESULT_RERUN_PREVIEW_GATE)
        );
        assert_eq!(
            report.recommended_next_gate,
            WORK_GRAPH_APPEND_ONLY_STORE_ENABLEMENT_PRECONDITION_RECOMMENDED_NEXT_GATE
        );
        assert!(report.ready_for_precondition_readback_preview);
        assert!(!report.ready_for_append_only_store_enablement);
        assert!(!report.ready_for_projection_enforcement);
        assert!(!report.ready_for_scheduler_admission_enforcement);
        assert!(!report.ready_for_role_manifest_enforcement);
        assert!(!report.ready_for_live_execution);
        assert_eq!(
            report.side_effects,
            WorkGraphAppendOnlyStoreEnablementPreconditionPreviewSideEffects::none()
        );
        assert!(
            report
                .enablement_stages
                .iter()
                .all(|stage| !stage.enablement_enabled)
        );
    }
}

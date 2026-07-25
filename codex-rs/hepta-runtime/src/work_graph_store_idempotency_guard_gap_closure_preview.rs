use serde::Serialize;

use crate::work_graph_idempotency_readback_adapter_preview::WorkGraphIdempotencySourceAdapterPreview;
use crate::work_graph_idempotency_readback_adapter_preview::WorkGraphReplayKeyContractPreview;
use crate::work_graph_idempotency_readback_adapter_preview::WorkGraphSourceReadbackProbeContractPreview;
use crate::work_graph_idempotency_readback_adapter_preview::work_graph_idempotency_readback_probe_contracts;
use crate::work_graph_idempotency_readback_adapter_preview::work_graph_idempotency_readback_replay_key_contracts;
use crate::work_graph_idempotency_readback_adapter_preview::work_graph_idempotency_readback_source_adapters;
use crate::work_graph_state_store_persistence_preview::work_graph_state_store_idempotency_guards;
use crate::work_graph_unified_projection_enforcement_readiness_rerun_preview::WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_RERUN_PREVIEW_GATE;
use crate::work_graph_unified_projection_enforcement_readiness_rerun_preview::work_graph_unified_projection_enforcement_readiness_rerun_required_prior_gates;
use crate::work_graph_unified_projection_enforcement_readiness_rerun_preview::work_graph_unified_projection_enforcement_rerun_source_decisions;

pub const WORK_GRAPH_STORE_IDEMPOTENCY_GUARD_GAP_CLOSURE_PREVIEW_GATE: &str =
    "hepta_work_graph_store_idempotency_guard_gap_closure_preview_gate";
pub const WORK_GRAPH_STORE_IDEMPOTENCY_GUARD_GAP_CLOSURE_SCHEMA_VERSION: &str =
    "work_graph_store_idempotency_guard_gap_closure_preview_v1";
pub const WORK_GRAPH_STORE_IDEMPOTENCY_GUARD_GAP_CLOSURE_RECOMMENDED_NEXT_GATE: &str =
    "hepta_work_graph_store_idempotency_guard_gap_closure_readback_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphStoreIdempotencyGuardGapClosurePreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub rerun_store_guard_gap_count: usize,
    pub idempotency_adapter_count: usize,
    pub existing_state_store_guard_count: usize,
    pub existing_guard_gap_count: usize,
    pub closure_plan_count: usize,
    pub candidate_guard_count: usize,
    pub guard_binding_count: usize,
    pub guard_probe_binding_count: usize,
    pub expected_collection_ref_count: usize,
    pub readback_probe_contract_ref_count: usize,
    pub task_result_guard_dependency_count: usize,
    pub blocker_count: usize,
    pub required_prior_gate_count: usize,
    pub closure_plans: Vec<WorkGraphStoreIdempotencyGuardClosurePlanPreview>,
    pub candidate_guards: Vec<WorkGraphStoreIdempotencyCandidateGuardPreview>,
    pub guard_bindings: Vec<WorkGraphStoreIdempotencyGuardBindingPreview>,
    pub guard_probe_bindings: Vec<WorkGraphStoreIdempotencyGuardProbeBindingPreview>,
    pub blockers: Vec<WorkGraphStoreIdempotencyGuardGapClosureBlockerPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub ready_for_store_idempotency_guard_gap_closure_readback_preview: bool,
    pub ready_for_runtime_guard_application: bool,
    pub ready_for_append_only_store_enablement: bool,
    pub ready_for_projection_enforcement: bool,
    pub ready_for_live_execution: bool,
    pub side_effects: WorkGraphStoreIdempotencyGuardGapClosurePreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphStoreIdempotencyGuardClosurePlanPreview {
    pub id: &'static str,
    pub source_surface_id: &'static str,
    pub source_category: &'static str,
    pub rerun_enforcement_decision: &'static str,
    pub adapter_id: &'static str,
    pub replay_key_contract_id: &'static str,
    pub candidate_guard_id: &'static str,
    pub key_fields: Vec<&'static str>,
    pub expected_collection_ids: Vec<&'static str>,
    pub readback_probe_contract_ids: Vec<&'static str>,
    pub collision_policy: &'static str,
    pub closure_state: &'static str,
    pub requires_task_result_wrapper: bool,
    pub runtime_guard_attached: bool,
    pub mutates_idempotency_index: bool,
    pub enables_store_write: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphStoreIdempotencyCandidateGuardPreview {
    pub id: &'static str,
    pub source_surface_id: &'static str,
    pub derived_from_adapter_id: &'static str,
    pub derived_from_replay_key_contract_id: &'static str,
    pub key_fields: Vec<&'static str>,
    pub key_formula: &'static str,
    pub replay_scope: &'static str,
    pub collision_policy: &'static str,
    pub redaction_policy: &'static str,
    pub required_before_append_only_intake: bool,
    pub mutates_idempotency_index: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphStoreIdempotencyGuardBindingPreview {
    pub id: &'static str,
    pub source_surface_id: &'static str,
    pub candidate_guard_id: &'static str,
    pub adapter_id: &'static str,
    pub replay_key_contract_id: &'static str,
    pub existing_state_store_guard_present: bool,
    pub adapter_replay_key_contract_present: bool,
    pub readback_probe_count: usize,
    pub expected_collection_ids: Vec<&'static str>,
    pub requires_task_result_wrapper: bool,
    pub closure_state: &'static str,
    pub no_runtime_application: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphStoreIdempotencyGuardProbeBindingPreview {
    pub id: &'static str,
    pub source_surface_id: &'static str,
    pub candidate_guard_id: &'static str,
    pub readback_probe_contract_ids: Vec<&'static str>,
    pub target_collection_ids: Vec<&'static str>,
    pub readback_evidence_fields: Vec<&'static str>,
    pub drift_detector_ids: Vec<&'static str>,
    pub performs_readback: bool,
    pub mutates_store: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphStoreIdempotencyGuardGapClosureBlockerPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub required_before_projection_enforcement: bool,
    pub recommended_fix: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphStoreIdempotencyGuardGapClosurePreviewSideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub wal_written: bool,
    pub idempotency_index_mutated: bool,
    pub store_guard_attached: bool,
    pub append_only_store_enabled: bool,
    pub projection_enforcement_enabled: bool,
    pub readback_performed: bool,
    pub task_result_enforcement_enabled: bool,
    pub scheduler_admission_enforced: bool,
    pub role_manifest_enforcement_enabled: bool,
    pub approval_recorded: bool,
    pub runtime_mutation_performed: bool,
    pub agent_spawn_performed: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_store_idempotency_guard_gap_closure_preview_report()
-> WorkGraphStoreIdempotencyGuardGapClosurePreviewReport {
    let closure_plans = work_graph_store_idempotency_guard_gap_closure_plans();
    let candidate_guards = work_graph_store_idempotency_guard_gap_candidate_guards();
    let guard_bindings = work_graph_store_idempotency_guard_gap_bindings();
    let guard_probe_bindings = work_graph_store_idempotency_guard_gap_probe_bindings();
    let blockers = work_graph_store_idempotency_guard_gap_closure_blockers();
    let required_prior_gates =
        work_graph_store_idempotency_guard_gap_closure_required_prior_gates();
    let expected_collection_ref_count = closure_plans
        .iter()
        .map(|plan| plan.expected_collection_ids.len())
        .sum();
    let readback_probe_contract_ref_count = closure_plans
        .iter()
        .map(|plan| plan.readback_probe_contract_ids.len())
        .sum();
    let task_result_guard_dependency_count = closure_plans
        .iter()
        .filter(|plan| plan.requires_task_result_wrapper)
        .count();
    let existing_guard_gap_count = guard_bindings
        .iter()
        .filter(|binding| !binding.existing_state_store_guard_present)
        .count();

    WorkGraphStoreIdempotencyGuardGapClosurePreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "blocked",
        gate: WORK_GRAPH_STORE_IDEMPOTENCY_GUARD_GAP_CLOSURE_PREVIEW_GATE,
        schema_version: WORK_GRAPH_STORE_IDEMPOTENCY_GUARD_GAP_CLOSURE_SCHEMA_VERSION,
        preview_mode: "read_only_store_idempotency_guard_gap_closure_no_index_write",
        rerun_store_guard_gap_count: closure_plans.len(),
        idempotency_adapter_count: work_graph_idempotency_readback_source_adapters().len(),
        existing_state_store_guard_count: work_graph_state_store_idempotency_guards().len(),
        existing_guard_gap_count,
        closure_plan_count: closure_plans.len(),
        candidate_guard_count: candidate_guards.len(),
        guard_binding_count: guard_bindings.len(),
        guard_probe_binding_count: guard_probe_bindings.len(),
        expected_collection_ref_count,
        readback_probe_contract_ref_count,
        task_result_guard_dependency_count,
        blocker_count: blockers.len(),
        required_prior_gate_count: required_prior_gates.len(),
        closure_plans,
        candidate_guards,
        guard_bindings,
        guard_probe_bindings,
        blockers,
        required_prior_gates,
        recommended_next_gate: WORK_GRAPH_STORE_IDEMPOTENCY_GUARD_GAP_CLOSURE_RECOMMENDED_NEXT_GATE,
        ready_for_store_idempotency_guard_gap_closure_readback_preview: true,
        ready_for_runtime_guard_application: false,
        ready_for_append_only_store_enablement: false,
        ready_for_projection_enforcement: false,
        ready_for_live_execution: false,
        side_effects: WorkGraphStoreIdempotencyGuardGapClosurePreviewSideEffects::none(),
    }
}

pub fn work_graph_store_idempotency_guard_gap_closure_plans()
-> Vec<WorkGraphStoreIdempotencyGuardClosurePlanPreview> {
    work_graph_unified_projection_enforcement_rerun_source_decisions()
        .into_iter()
        .filter(|decision| {
            decision.rerun_enforcement_decision == "deny_missing_store_idempotency_guard"
        })
        .filter_map(|decision| {
            let adapter = adapter_for_source(decision.source_surface_id)?;
            let replay_key = replay_key_for_adapter(&adapter)?;
            Some(closure_plan(
                closure_plan_id_for_source(decision.source_surface_id),
                decision.source_surface_id,
                decision.source_category,
                decision.rerun_enforcement_decision,
                adapter,
                replay_key,
            ))
        })
        .collect()
}

pub fn work_graph_store_idempotency_guard_gap_candidate_guards()
-> Vec<WorkGraphStoreIdempotencyCandidateGuardPreview> {
    work_graph_store_idempotency_guard_gap_closure_plans()
        .iter()
        .filter_map(|plan| {
            let adapter = adapter_for_source(plan.source_surface_id)?;
            let replay_key = replay_key_for_adapter(&adapter)?;
            Some(candidate_guard(
                plan.candidate_guard_id,
                plan.source_surface_id,
                adapter.id,
                replay_key,
            ))
        })
        .collect()
}

pub fn work_graph_store_idempotency_guard_gap_bindings()
-> Vec<WorkGraphStoreIdempotencyGuardBindingPreview> {
    let existing_guards = work_graph_state_store_idempotency_guards();
    work_graph_store_idempotency_guard_gap_closure_plans()
        .iter()
        .filter_map(|plan| {
            let adapter = adapter_for_source(plan.source_surface_id)?;
            let replay_key = replay_key_for_adapter(&adapter)?;
            Some(WorkGraphStoreIdempotencyGuardBindingPreview {
                id: guard_binding_id_for_source(plan.source_surface_id),
                source_surface_id: plan.source_surface_id,
                candidate_guard_id: plan.candidate_guard_id,
                adapter_id: adapter.id,
                replay_key_contract_id: replay_key.id,
                existing_state_store_guard_present: existing_guards
                    .iter()
                    .any(|guard| guard.source_surface_id == plan.source_surface_id),
                adapter_replay_key_contract_present: true,
                readback_probe_count: probes_for_source(plan.source_surface_id).len(),
                expected_collection_ids: adapter.expected_collection_ids.clone(),
                requires_task_result_wrapper: adapter.requires_task_result_wrapper,
                closure_state: "candidate_guard_defined_state_store_binding_not_applied",
                no_runtime_application: true,
            })
        })
        .collect()
}

pub fn work_graph_store_idempotency_guard_gap_probe_bindings()
-> Vec<WorkGraphStoreIdempotencyGuardProbeBindingPreview> {
    work_graph_store_idempotency_guard_gap_closure_plans()
        .iter()
        .map(|plan| {
            let probes = probes_for_source(plan.source_surface_id);
            WorkGraphStoreIdempotencyGuardProbeBindingPreview {
                id: guard_probe_binding_id_for_source(plan.source_surface_id),
                source_surface_id: plan.source_surface_id,
                candidate_guard_id: plan.candidate_guard_id,
                readback_probe_contract_ids: probes.iter().map(|probe| probe.id).collect(),
                target_collection_ids: unique_static(
                    probes.iter().map(|probe| probe.collection_id).collect(),
                ),
                readback_evidence_fields: unique_static(
                    probes
                        .iter()
                        .flat_map(|probe| probe.evidence_fields.iter().copied())
                        .collect(),
                ),
                drift_detector_ids: unique_static(
                    probes
                        .iter()
                        .flat_map(|probe| probe.drift_detector_ids.iter().copied())
                        .collect(),
                ),
                performs_readback: false,
                mutates_store: false,
            }
        })
        .collect()
}

pub fn work_graph_store_idempotency_guard_gap_closure_blockers()
-> Vec<WorkGraphStoreIdempotencyGuardGapClosureBlockerPreview> {
    let source_ids = store_guard_gap_source_surface_ids();
    vec![
        blocker(
            "runtime_guard_application_disabled",
            "high",
            source_ids.clone(),
            "bind candidate guards to runtime adapters only after readback and operator review promote this preview",
        ),
        blocker(
            "state_store_guard_persistence_disabled",
            "high",
            source_ids.clone(),
            "keep state-store guard rows as preview contracts until append-only intake and WAL replay gates are promoted",
        ),
        blocker(
            "append_only_store_enablement_disabled",
            "high",
            source_ids.clone(),
            "do not allow append-only writes until guard collisions, replay, and readback are deterministic",
        ),
        blocker(
            "task_result_enforcement_disabled",
            "high",
            vec![
                "hepta_runtime_multi_agent_reducer",
                "hepta_runtime_task_board",
            ],
            "TaskResult-producing guard closures need terminal TaskResult enforcement before runtime use",
        ),
        blocker(
            "readback_execution_disabled",
            "medium",
            source_ids.clone(),
            "verify guard readback contracts in a dedicated readback preview before any runtime application",
        ),
        blocker(
            "operator_review_required",
            "medium",
            source_ids,
            "require operator review of guard formulas, collision policy, and redaction before promotion",
        ),
    ]
}

pub fn work_graph_store_idempotency_guard_gap_closure_required_prior_gates() -> Vec<&'static str> {
    let mut gates =
        work_graph_unified_projection_enforcement_readiness_rerun_required_prior_gates();
    gates.push(WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_RERUN_PREVIEW_GATE);
    gates
}

impl WorkGraphStoreIdempotencyGuardGapClosurePreviewSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            wal_written: false,
            idempotency_index_mutated: false,
            store_guard_attached: false,
            append_only_store_enabled: false,
            projection_enforcement_enabled: false,
            readback_performed: false,
            task_result_enforcement_enabled: false,
            scheduler_admission_enforced: false,
            role_manifest_enforcement_enabled: false,
            approval_recorded: false,
            runtime_mutation_performed: false,
            agent_spawn_performed: false,
            external_send_performed: false,
            model_invoked: false,
        }
    }
}

fn adapter_for_source(source_surface_id: &str) -> Option<WorkGraphIdempotencySourceAdapterPreview> {
    work_graph_idempotency_readback_source_adapters()
        .into_iter()
        .find(|adapter| adapter.source_surface_id == source_surface_id)
}

fn replay_key_for_adapter(
    adapter: &WorkGraphIdempotencySourceAdapterPreview,
) -> Option<WorkGraphReplayKeyContractPreview> {
    work_graph_idempotency_readback_replay_key_contracts()
        .into_iter()
        .find(|contract| contract.id == adapter.replay_key_contract_id)
}

fn probes_for_source(source_surface_id: &str) -> Vec<WorkGraphSourceReadbackProbeContractPreview> {
    work_graph_idempotency_readback_probe_contracts()
        .into_iter()
        .filter(|probe| probe.source_surface_id == source_surface_id)
        .collect()
}

fn closure_plan(
    id: &'static str,
    source_surface_id: &'static str,
    source_category: &'static str,
    rerun_enforcement_decision: &'static str,
    adapter: WorkGraphIdempotencySourceAdapterPreview,
    replay_key: WorkGraphReplayKeyContractPreview,
) -> WorkGraphStoreIdempotencyGuardClosurePlanPreview {
    WorkGraphStoreIdempotencyGuardClosurePlanPreview {
        id,
        source_surface_id,
        source_category,
        rerun_enforcement_decision,
        adapter_id: adapter.id,
        replay_key_contract_id: replay_key.id,
        candidate_guard_id: candidate_guard_id_for_source(source_surface_id),
        key_fields: replay_key.key_fields,
        expected_collection_ids: adapter.expected_collection_ids,
        readback_probe_contract_ids: adapter.readback_probe_contract_ids,
        collision_policy: replay_key.collision_policy,
        closure_state: "candidate_guard_preview_only_runtime_not_attached",
        requires_task_result_wrapper: adapter.requires_task_result_wrapper,
        runtime_guard_attached: false,
        mutates_idempotency_index: false,
        enables_store_write: false,
    }
}

fn candidate_guard(
    id: &'static str,
    source_surface_id: &'static str,
    adapter_id: &'static str,
    replay_key: WorkGraphReplayKeyContractPreview,
) -> WorkGraphStoreIdempotencyCandidateGuardPreview {
    WorkGraphStoreIdempotencyCandidateGuardPreview {
        id,
        source_surface_id,
        derived_from_adapter_id: adapter_id,
        derived_from_replay_key_contract_id: replay_key.id,
        key_fields: replay_key.key_fields,
        key_formula: replay_key.key_formula,
        replay_scope: replay_key.replay_scope,
        collision_policy: replay_key.collision_policy,
        redaction_policy: replay_key.redaction_policy,
        required_before_append_only_intake: true,
        mutates_idempotency_index: false,
    }
}

fn blocker(
    id: &'static str,
    severity: &'static str,
    affected_source_surface_ids: Vec<&'static str>,
    recommended_fix: &'static str,
) -> WorkGraphStoreIdempotencyGuardGapClosureBlockerPreview {
    WorkGraphStoreIdempotencyGuardGapClosureBlockerPreview {
        id,
        severity,
        affected_source_surface_ids,
        required_before_projection_enforcement: true,
        recommended_fix,
    }
}

fn store_guard_gap_source_surface_ids() -> Vec<&'static str> {
    work_graph_store_idempotency_guard_gap_closure_plans()
        .iter()
        .map(|plan| plan.source_surface_id)
        .collect()
}

fn unique_static(values: Vec<&'static str>) -> Vec<&'static str> {
    values.into_iter().fold(Vec::new(), |mut acc, value| {
        if !acc.contains(&value) {
            acc.push(value);
        }
        acc
    })
}

fn closure_plan_id_for_source(source_surface_id: &str) -> &'static str {
    match source_surface_id {
        "plan_mode_proposed_plan_blocks" => {
            "close_plan_mode_proposed_plan_blocks_store_idempotency_guard_gap"
        }
        "app_server_turn_plan_notification" => {
            "close_app_server_turn_plan_notification_store_idempotency_guard_gap"
        }
        "multi_agent_v2_mailbox_wait" => {
            "close_multi_agent_v2_mailbox_wait_store_idempotency_guard_gap"
        }
        "hepta_runtime_multi_agent_reducer" => {
            "close_hepta_runtime_multi_agent_reducer_store_idempotency_guard_gap"
        }
        "hepta_runtime_task_board" => "close_hepta_runtime_task_board_store_idempotency_guard_gap",
        _ => "close_unknown_store_idempotency_guard_gap",
    }
}

fn candidate_guard_id_for_source(source_surface_id: &str) -> &'static str {
    match source_surface_id {
        "plan_mode_proposed_plan_blocks" => {
            "plan_mode_proposed_plan_blocks_store_idempotency_guard"
        }
        "app_server_turn_plan_notification" => {
            "app_server_turn_plan_notification_store_idempotency_guard"
        }
        "multi_agent_v2_mailbox_wait" => "multi_agent_v2_mailbox_wait_store_idempotency_guard",
        "hepta_runtime_multi_agent_reducer" => {
            "hepta_runtime_multi_agent_reducer_store_idempotency_guard"
        }
        "hepta_runtime_task_board" => "hepta_runtime_task_board_store_idempotency_guard",
        _ => "unknown_store_idempotency_guard",
    }
}

fn guard_binding_id_for_source(source_surface_id: &str) -> &'static str {
    match source_surface_id {
        "plan_mode_proposed_plan_blocks" => "plan_mode_proposed_plan_blocks_store_guard_binding",
        "app_server_turn_plan_notification" => {
            "app_server_turn_plan_notification_store_guard_binding"
        }
        "multi_agent_v2_mailbox_wait" => "multi_agent_v2_mailbox_wait_store_guard_binding",
        "hepta_runtime_multi_agent_reducer" => {
            "hepta_runtime_multi_agent_reducer_store_guard_binding"
        }
        "hepta_runtime_task_board" => "hepta_runtime_task_board_store_guard_binding",
        _ => "unknown_store_guard_binding",
    }
}

fn guard_probe_binding_id_for_source(source_surface_id: &str) -> &'static str {
    match source_surface_id {
        "plan_mode_proposed_plan_blocks" => {
            "plan_mode_proposed_plan_blocks_store_guard_readback_binding"
        }
        "app_server_turn_plan_notification" => {
            "app_server_turn_plan_notification_store_guard_readback_binding"
        }
        "multi_agent_v2_mailbox_wait" => "multi_agent_v2_mailbox_wait_store_guard_readback_binding",
        "hepta_runtime_multi_agent_reducer" => {
            "hepta_runtime_multi_agent_reducer_store_guard_readback_binding"
        }
        "hepta_runtime_task_board" => "hepta_runtime_task_board_store_guard_readback_binding",
        _ => "unknown_store_guard_readback_binding",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_idempotency_guard_gap_closure_targets_rerun_gap_sources() {
        let report = hepta_work_graph_store_idempotency_guard_gap_closure_preview_report();
        let source_surface_ids = report
            .closure_plans
            .iter()
            .map(|plan| plan.source_surface_id)
            .collect::<Vec<_>>();

        assert_eq!(
            source_surface_ids,
            [
                "plan_mode_proposed_plan_blocks",
                "app_server_turn_plan_notification",
                "multi_agent_v2_mailbox_wait",
                "hepta_runtime_multi_agent_reducer",
                "hepta_runtime_task_board",
            ]
        );
        assert_eq!(report.rerun_store_guard_gap_count, 5);
        assert_eq!(report.closure_plan_count, 5);
        assert_eq!(report.candidate_guard_count, 5);
        assert_eq!(report.existing_guard_gap_count, 0);
    }

    #[test]
    fn store_idempotency_guard_gap_closure_derives_guards_from_adapters() {
        let report = hepta_work_graph_store_idempotency_guard_gap_closure_preview_report();
        let guard_ids = report
            .candidate_guards
            .iter()
            .map(|guard| guard.id)
            .collect::<Vec<_>>();

        assert_eq!(
            guard_ids,
            [
                "plan_mode_proposed_plan_blocks_store_idempotency_guard",
                "app_server_turn_plan_notification_store_idempotency_guard",
                "multi_agent_v2_mailbox_wait_store_idempotency_guard",
                "hepta_runtime_multi_agent_reducer_store_idempotency_guard",
                "hepta_runtime_task_board_store_idempotency_guard",
            ]
        );
        assert_eq!(report.expected_collection_ref_count, 14);
        assert_eq!(report.readback_probe_contract_ref_count, 14);
        assert_eq!(report.task_result_guard_dependency_count, 2);
        assert!(
            report
                .candidate_guards
                .iter()
                .all(|guard| !guard.key_fields.is_empty()
                    && guard.required_before_append_only_intake
                    && !guard.mutates_idempotency_index)
        );
    }

    #[test]
    fn store_idempotency_guard_gap_closure_keeps_bindings_preview_only() {
        let report = hepta_work_graph_store_idempotency_guard_gap_closure_preview_report();
        let probe_counts = report
            .guard_bindings
            .iter()
            .map(|binding| binding.readback_probe_count)
            .collect::<Vec<_>>();

        assert_eq!(probe_counts, [3, 3, 2, 2, 4]);
        assert!(report.guard_bindings.iter().all(|binding| {
            binding.existing_state_store_guard_present
                && binding.adapter_replay_key_contract_present
                && binding.no_runtime_application
        }));
        assert!(
            report
                .guard_probe_bindings
                .iter()
                .all(|binding| !binding.performs_readback && !binding.mutates_store)
        );
    }

    #[test]
    fn store_idempotency_guard_gap_closure_preserves_blockers_and_next_frontier() {
        let report = hepta_work_graph_store_idempotency_guard_gap_closure_preview_report();
        let blocker_counts = report
            .blockers
            .iter()
            .map(|blocker| (blocker.id, blocker.affected_source_surface_ids.len()))
            .collect::<Vec<_>>();

        assert_eq!(
            blocker_counts,
            [
                ("runtime_guard_application_disabled", 5),
                ("state_store_guard_persistence_disabled", 5),
                ("append_only_store_enablement_disabled", 5),
                ("task_result_enforcement_disabled", 2),
                ("readback_execution_disabled", 5),
                ("operator_review_required", 5),
            ]
        );
        assert_eq!(report.blocker_count, 6);
        assert_eq!(report.required_prior_gate_count, 17);
        assert_eq!(
            report.required_prior_gates.last().copied(),
            Some(WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_RERUN_PREVIEW_GATE)
        );
        assert_eq!(
            report.recommended_next_gate,
            WORK_GRAPH_STORE_IDEMPOTENCY_GUARD_GAP_CLOSURE_RECOMMENDED_NEXT_GATE
        );
        assert!(report.ready_for_store_idempotency_guard_gap_closure_readback_preview);
        assert!(!report.ready_for_runtime_guard_application);
    }

    #[test]
    fn store_idempotency_guard_gap_closure_keeps_side_effects_disabled() {
        let side_effects =
            hepta_work_graph_store_idempotency_guard_gap_closure_preview_report().side_effects;

        assert_eq!(
            side_effects,
            WorkGraphStoreIdempotencyGuardGapClosurePreviewSideEffects {
                filesystem_written: false,
                graph_state_persisted: false,
                wal_written: false,
                idempotency_index_mutated: false,
                store_guard_attached: false,
                append_only_store_enabled: false,
                projection_enforcement_enabled: false,
                readback_performed: false,
                task_result_enforcement_enabled: false,
                scheduler_admission_enforced: false,
                role_manifest_enforcement_enabled: false,
                approval_recorded: false,
                runtime_mutation_performed: false,
                agent_spawn_performed: false,
                external_send_performed: false,
                model_invoked: false,
            }
        );
    }
}

use serde::Serialize;

use crate::work_graph_append_only_work_graph_events_replay_readback_readback_preview::WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_REPLAY_READBACK_READBACK_PREVIEW_GATE;
use crate::work_graph_append_only_work_graph_events_replay_readback_readback_preview::WorkGraphEventsReplayReadbackBlockerMappingAssertionPreview;
use crate::work_graph_append_only_work_graph_events_replay_readback_readback_preview::WorkGraphEventsReplayReadbackEvidenceFieldAssertionPreview;
use crate::work_graph_append_only_work_graph_events_replay_readback_readback_preview::WorkGraphEventsReplayReadbackGuardAssertionPreview;
use crate::work_graph_append_only_work_graph_events_replay_readback_readback_preview::WorkGraphEventsReplayReadbackReadbackBlockerPreview;
use crate::work_graph_append_only_work_graph_events_replay_readback_readback_preview::WorkGraphEventsReplayReadbackReadbackPlanPreview;
use crate::work_graph_append_only_work_graph_events_replay_readback_readback_preview::WorkGraphEventsReplayReadbackStageAssertionPreview;
use crate::work_graph_append_only_work_graph_events_replay_readback_readback_preview::hepta_work_graph_append_only_work_graph_events_replay_readback_readback_preview_report;
use crate::work_graph_append_only_work_graph_events_replay_readback_readback_preview::work_graph_append_only_work_graph_events_replay_readback_readback_required_prior_gates;

pub const WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_REPLAY_READBACK_APPLICATION_PREVIEW_GATE: &str =
    "hepta_work_graph_append_only_work_graph_events_replay_readback_application_preview_gate";
pub const WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_REPLAY_READBACK_APPLICATION_SCHEMA_VERSION:
    &str = "work_graph_append_only_work_graph_events_replay_readback_application_preview_v1";
pub const WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_REPLAY_READBACK_APPLICATION_RECOMMENDED_NEXT_GATE:
    &str = "hepta_work_graph_unified_projection_enforcement_readiness_work_graph_events_replay_readback_rerun_preview_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyWorkGraphEventsReplayReadbackApplicationPreviewReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub readback_plan_count: usize,
    pub application_plan_count: usize,
    pub source_outcome_count: usize,
    pub replay_readback_contract_ready_preview_count: usize,
    pub stage_application_count: usize,
    pub evidence_field_application_count: usize,
    pub guard_application_count: usize,
    pub blocker_application_count: usize,
    pub application_guard_count: usize,
    pub blocker_count: usize,
    pub required_prior_gate_count: usize,
    pub application_plans: Vec<WorkGraphEventsReplayReadbackApplicationPlanPreview>,
    pub source_outcomes: Vec<WorkGraphEventsReplayReadbackApplicationSourceOutcomePreview>,
    pub stage_applications: Vec<WorkGraphEventsReplayReadbackStageApplicationPreview>,
    pub evidence_field_applications:
        Vec<WorkGraphEventsReplayReadbackEvidenceFieldApplicationPreview>,
    pub guard_applications: Vec<WorkGraphEventsReplayReadbackGuardApplicationPreview>,
    pub blocker_applications: Vec<WorkGraphEventsReplayReadbackBlockerApplicationPreview>,
    pub application_guards: Vec<WorkGraphEventsReplayReadbackApplicationGuardPreview>,
    pub blockers: Vec<WorkGraphEventsReplayReadbackApplicationBlockerPreview>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub ready_for_replay_readback_readiness_rerun_preview: bool,
    pub ready_for_append_only_work_graph_events: bool,
    pub ready_for_replay_readback_execution: bool,
    pub ready_for_runtime_adapter_enforcement: bool,
    pub ready_for_live_execution: bool,
    pub side_effects: WorkGraphAppendOnlyWorkGraphEventsReplayReadbackApplicationPreviewSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphEventsReplayReadbackApplicationPlanPreview {
    pub application_plan_id: String,
    pub readback_source_surface_id: &'static str,
    pub source_category: &'static str,
    pub replay_readback_plan_id: String,
    pub application_state: &'static str,
    pub readback_verified_by_preview: bool,
    pub replay_readback_contract_ready_preview: bool,
    pub applies_to_runtime: bool,
    pub persists_work_graph_events: bool,
    pub writes_wal: bool,
    pub writes_checkpoint: bool,
    pub executes_replay: bool,
    pub executes_readback: bool,
    pub executes_rollback: bool,
    pub mutates_idempotency_index: bool,
    pub enforces_adapter_projection: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphEventsReplayReadbackApplicationSourceOutcomePreview {
    pub source_surface_id: &'static str,
    pub source_category: &'static str,
    pub application_plan_id: String,
    pub post_application_replay_readback_state: &'static str,
    pub replay_readback_contract_ready_preview: bool,
    pub ready_for_replay_readback_readiness_rerun_preview: bool,
    pub ready_for_append_only_work_graph_events: bool,
    pub ready_for_replay_readback_execution: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphEventsReplayReadbackStageApplicationPreview {
    pub application_id: String,
    pub stage_id: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub required_contract_ref_ids: Vec<&'static str>,
    pub contract_ready_preview: bool,
    pub persists_work_graph_events: bool,
    pub executes_replay: bool,
    pub executes_readback: bool,
    pub executes_rollback: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphEventsReplayReadbackEvidenceFieldApplicationPreview {
    pub application_id: String,
    pub source_surface_id: &'static str,
    pub evidence_field_ids: Vec<&'static str>,
    pub evidence_contract_ready_preview: bool,
    pub persists_evidence: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphEventsReplayReadbackGuardApplicationPreview {
    pub application_id: String,
    pub guard_id: &'static str,
    pub guard_scope: &'static str,
    pub required_before_replay_readback_execution: bool,
    pub satisfied_by_preview: bool,
    pub mutates_runtime: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphEventsReplayReadbackBlockerApplicationPreview {
    pub application_id: String,
    pub blocker_id: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub affected_replay_readback_stage_ids: Vec<&'static str>,
    pub expected_blocker_state: &'static str,
    pub readback_verified_by_preview: bool,
    pub clears_application_missing_blocker: bool,
    pub mutates_runtime: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphEventsReplayReadbackApplicationGuardPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub guard_scope: &'static str,
    pub required_before_append_only_events: bool,
    pub satisfied_by_preview: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphEventsReplayReadbackApplicationBlockerPreview {
    pub id: &'static str,
    pub severity: &'static str,
    pub category: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub affected_application_plan_ids: Vec<String>,
    pub required_before_append_only_events: bool,
    pub recommended_fix: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphAppendOnlyWorkGraphEventsReplayReadbackApplicationPreviewSideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub work_graph_events_persisted: bool,
    pub wal_written: bool,
    pub checkpoint_written: bool,
    pub replay_executed: bool,
    pub readback_executed: bool,
    pub rollback_executed: bool,
    pub idempotency_index_mutated: bool,
    pub adapter_projection_enforced: bool,
    pub runtime_mutation_performed: bool,
    pub agent_spawn_performed: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_append_only_work_graph_events_replay_readback_application_preview_report()
-> WorkGraphAppendOnlyWorkGraphEventsReplayReadbackApplicationPreviewReport {
    let readback_report =
        hepta_work_graph_append_only_work_graph_events_replay_readback_readback_preview_report();
    let application_plans =
        work_graph_append_only_work_graph_events_replay_readback_application_plans_from(
            &readback_report.readback_plans,
        );
    let source_outcomes =
        work_graph_append_only_work_graph_events_replay_readback_application_source_outcomes_from(
            &application_plans,
        );
    let application_guards =
        work_graph_append_only_work_graph_events_replay_readback_application_guards();
    let blockers =
        work_graph_append_only_work_graph_events_replay_readback_application_blockers_from(
            &application_plans,
            &readback_report.blockers,
        );
    let required_prior_gates =
        work_graph_append_only_work_graph_events_replay_readback_application_required_prior_gates();

    WorkGraphAppendOnlyWorkGraphEventsReplayReadbackApplicationPreviewReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_REPLAY_READBACK_APPLICATION_PREVIEW_GATE,
        schema_version:
            WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_REPLAY_READBACK_APPLICATION_SCHEMA_VERSION,
        preview_mode:
            "read_only_append_only_work_graph_events_replay_readback_application_preview_no_mutation",
        readback_plan_count: readback_report.readback_plan_count,
        application_plan_count: application_plans.len(),
        source_outcome_count: source_outcomes.len(),
        replay_readback_contract_ready_preview_count: source_outcomes
            .iter()
            .filter(|outcome| outcome.replay_readback_contract_ready_preview)
            .count(),
        stage_application_count: readback_report.stage_assertions.len(),
        evidence_field_application_count: readback_report.evidence_field_assertions.len(),
        guard_application_count: readback_report.guard_assertions.len(),
        blocker_application_count: readback_report.blocker_mapping_assertions.len(),
        application_guard_count: application_guards.len(),
        blocker_count: blockers.len(),
        required_prior_gate_count: required_prior_gates.len(),
        stage_applications:
            work_graph_append_only_work_graph_events_replay_readback_stage_applications_from(
                &readback_report.stage_assertions,
            ),
        evidence_field_applications:
            work_graph_append_only_work_graph_events_replay_readback_evidence_field_applications_from(
                &readback_report.evidence_field_assertions,
            ),
        guard_applications:
            work_graph_append_only_work_graph_events_replay_readback_guard_applications_from(
                &readback_report.guard_assertions,
            ),
        blocker_applications:
            work_graph_append_only_work_graph_events_replay_readback_blocker_applications_from(
                &readback_report.blocker_mapping_assertions,
            ),
        application_guards,
        application_plans,
        source_outcomes,
        blockers,
        required_prior_gates,
        recommended_next_gate:
            WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_REPLAY_READBACK_APPLICATION_RECOMMENDED_NEXT_GATE,
        ready_for_replay_readback_readiness_rerun_preview: true,
        ready_for_append_only_work_graph_events: false,
        ready_for_replay_readback_execution: false,
        ready_for_runtime_adapter_enforcement: false,
        ready_for_live_execution: false,
        side_effects:
            WorkGraphAppendOnlyWorkGraphEventsReplayReadbackApplicationPreviewSideEffects::none(),
    }
}

pub fn work_graph_append_only_work_graph_events_replay_readback_application_plans()
-> Vec<WorkGraphEventsReplayReadbackApplicationPlanPreview> {
    let readback_report =
        hepta_work_graph_append_only_work_graph_events_replay_readback_readback_preview_report();
    work_graph_append_only_work_graph_events_replay_readback_application_plans_from(
        &readback_report.readback_plans,
    )
}

pub fn work_graph_append_only_work_graph_events_replay_readback_application_source_outcomes()
-> Vec<WorkGraphEventsReplayReadbackApplicationSourceOutcomePreview> {
    work_graph_append_only_work_graph_events_replay_readback_application_source_outcomes_from(
        &work_graph_append_only_work_graph_events_replay_readback_application_plans(),
    )
}

pub fn work_graph_append_only_work_graph_events_replay_readback_application_blockers()
-> Vec<WorkGraphEventsReplayReadbackApplicationBlockerPreview> {
    let readback_report =
        hepta_work_graph_append_only_work_graph_events_replay_readback_readback_preview_report();
    work_graph_append_only_work_graph_events_replay_readback_application_blockers_from(
        &work_graph_append_only_work_graph_events_replay_readback_application_plans(),
        &readback_report.blockers,
    )
}

pub fn work_graph_append_only_work_graph_events_replay_readback_application_required_prior_gates()
-> Vec<&'static str> {
    let mut gates =
        work_graph_append_only_work_graph_events_replay_readback_readback_required_prior_gates();
    gates.push(WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_REPLAY_READBACK_READBACK_PREVIEW_GATE);
    gates
}

fn work_graph_append_only_work_graph_events_replay_readback_application_plans_from(
    readback_plans: &[WorkGraphEventsReplayReadbackReadbackPlanPreview],
) -> Vec<WorkGraphEventsReplayReadbackApplicationPlanPreview> {
    readback_plans
        .iter()
        .map(|plan| WorkGraphEventsReplayReadbackApplicationPlanPreview {
            application_plan_id: format!(
                "{}_append_only_work_graph_events_replay_readback_application",
                plan.source_surface_id
            ),
            readback_source_surface_id: plan.source_surface_id,
            source_category: plan.source_category,
            replay_readback_plan_id: plan.replay_readback_plan_id.clone(),
            application_state:
                "work_graph_events_replay_readback_contract_ready_preview_after_application",
            readback_verified_by_preview: plan.readback_status == "readback_plan_ready",
            replay_readback_contract_ready_preview: true,
            applies_to_runtime: false,
            persists_work_graph_events: false,
            writes_wal: false,
            writes_checkpoint: false,
            executes_replay: false,
            executes_readback: false,
            executes_rollback: false,
            mutates_idempotency_index: false,
            enforces_adapter_projection: false,
        })
        .collect()
}

fn work_graph_append_only_work_graph_events_replay_readback_application_source_outcomes_from(
    application_plans: &[WorkGraphEventsReplayReadbackApplicationPlanPreview],
) -> Vec<WorkGraphEventsReplayReadbackApplicationSourceOutcomePreview> {
    application_plans
        .iter()
        .map(
            |plan| WorkGraphEventsReplayReadbackApplicationSourceOutcomePreview {
                source_surface_id: plan.readback_source_surface_id,
                source_category: plan.source_category,
                application_plan_id: plan.application_plan_id.clone(),
                post_application_replay_readback_state: plan.application_state,
                replay_readback_contract_ready_preview: plan.replay_readback_contract_ready_preview,
                ready_for_replay_readback_readiness_rerun_preview: true,
                ready_for_append_only_work_graph_events: false,
                ready_for_replay_readback_execution: false,
            },
        )
        .collect()
}

fn work_graph_append_only_work_graph_events_replay_readback_stage_applications_from(
    assertions: &[WorkGraphEventsReplayReadbackStageAssertionPreview],
) -> Vec<WorkGraphEventsReplayReadbackStageApplicationPreview> {
    assertions
        .iter()
        .map(
            |assertion| WorkGraphEventsReplayReadbackStageApplicationPreview {
                application_id: format!("{}_stage_application", assertion.stage_id),
                stage_id: assertion.stage_id,
                affected_source_surface_ids: assertion.affected_source_surface_ids.clone(),
                required_contract_ref_ids: assertion.required_contract_ref_ids.clone(),
                contract_ready_preview: assertion.contract_ready_preview,
                persists_work_graph_events: false,
                executes_replay: false,
                executes_readback: false,
                executes_rollback: false,
            },
        )
        .collect()
}

fn work_graph_append_only_work_graph_events_replay_readback_evidence_field_applications_from(
    assertions: &[WorkGraphEventsReplayReadbackEvidenceFieldAssertionPreview],
) -> Vec<WorkGraphEventsReplayReadbackEvidenceFieldApplicationPreview> {
    assertions
        .iter()
        .map(
            |assertion| WorkGraphEventsReplayReadbackEvidenceFieldApplicationPreview {
                application_id: format!(
                    "{}_evidence_field_application",
                    assertion.source_surface_id
                ),
                source_surface_id: assertion.source_surface_id,
                evidence_field_ids: assertion.evidence_field_ids.clone(),
                evidence_contract_ready_preview: assertion.evidence_contract_ready_preview,
                persists_evidence: false,
            },
        )
        .collect()
}

fn work_graph_append_only_work_graph_events_replay_readback_guard_applications_from(
    assertions: &[WorkGraphEventsReplayReadbackGuardAssertionPreview],
) -> Vec<WorkGraphEventsReplayReadbackGuardApplicationPreview> {
    assertions
        .iter()
        .map(
            |assertion| WorkGraphEventsReplayReadbackGuardApplicationPreview {
                application_id: format!("{}_guard_application", assertion.guard_id),
                guard_id: assertion.guard_id,
                guard_scope: assertion.guard_scope,
                required_before_replay_readback_execution: assertion
                    .required_before_replay_readback_execution,
                satisfied_by_preview: assertion.satisfied_by_preview,
                mutates_runtime: false,
            },
        )
        .collect()
}

fn work_graph_append_only_work_graph_events_replay_readback_blocker_applications_from(
    assertions: &[WorkGraphEventsReplayReadbackBlockerMappingAssertionPreview],
) -> Vec<WorkGraphEventsReplayReadbackBlockerApplicationPreview> {
    assertions
        .iter()
        .map(
            |assertion| WorkGraphEventsReplayReadbackBlockerApplicationPreview {
                application_id: format!("{}_blocker_application", assertion.blocker_id),
                blocker_id: assertion.blocker_id,
                affected_source_surface_ids: assertion.affected_source_surface_ids.clone(),
                affected_replay_readback_stage_ids: assertion
                    .affected_replay_readback_stage_ids
                    .clone(),
                expected_blocker_state:
                    "mapped_for_work_graph_events_replay_readback_rerun_preview",
                readback_verified_by_preview: true,
                clears_application_missing_blocker: assertion.blocker_id
                    == "append_only_work_graph_events_replay_readback_readback_missing",
                mutates_runtime: false,
            },
        )
        .collect()
}

fn work_graph_append_only_work_graph_events_replay_readback_application_guards()
-> Vec<WorkGraphEventsReplayReadbackApplicationGuardPreview> {
    vec![
        application_guard("no_work_graph_event_persistence", "critical", "event_store"),
        application_guard("no_wal_write", "critical", "wal"),
        application_guard("no_checkpoint_write", "critical", "checkpoint"),
        application_guard("no_replay_execution", "critical", "replay"),
        application_guard("no_readback_execution", "critical", "readback"),
        application_guard("no_rollback_execution", "critical", "rollback"),
        application_guard("no_idempotency_index_mutation", "critical", "idempotency"),
        application_guard(
            "no_adapter_projection_enforcement",
            "critical",
            "adapter_projection",
        ),
        application_guard("no_agent_spawn", "high", "agent_spawn"),
        application_guard(
            "no_external_send_or_model_invocation",
            "high",
            "external_effects",
        ),
        application_guard(
            "no_append_only_events_promotion_without_rerun",
            "high",
            "readiness_rerun",
        ),
    ]
}

fn work_graph_append_only_work_graph_events_replay_readback_application_blockers_from(
    application_plans: &[WorkGraphEventsReplayReadbackApplicationPlanPreview],
    readback_blockers: &[WorkGraphEventsReplayReadbackReadbackBlockerPreview],
) -> Vec<WorkGraphEventsReplayReadbackApplicationBlockerPreview> {
    let all_sources = application_plans
        .iter()
        .map(|plan| plan.readback_source_surface_id)
        .collect::<Vec<_>>();
    let all_plan_ids = application_plans
        .iter()
        .map(|plan| plan.application_plan_id.clone())
        .collect::<Vec<_>>();
    let partial_gap_sources = readback_blockers
        .iter()
        .find(|blocker| blocker.id == "canonical_adapter_projection_partial_or_gap")
        .map(|blocker| blocker.affected_source_surface_ids.clone())
        .unwrap_or_default();

    vec![
        application_blocker(
            "append_only_work_graph_events_disabled",
            "high",
            "append_only_fact_source",
            all_sources.clone(),
            all_plan_ids.clone(),
            "keep WorkGraph event persistence disabled until replay/readback readiness rerun is verified",
        ),
        application_blocker(
            "replay_readback_execution_disabled",
            "high",
            "replay_readback_execution",
            all_sources.clone(),
            all_plan_ids.clone(),
            "keep replay/readback execution disabled until append-only events are promoted behind operator review",
        ),
        application_blocker(
            "runtime_canonical_adapter_enforcement_disabled",
            "high",
            "runtime_adapter_enforcement",
            all_sources.clone(),
            all_plan_ids.clone(),
            "keep canonical adapter enforcement disabled until append-only events replay/readback is promoted",
        ),
        application_blocker(
            "canonical_adapter_projection_partial_or_gap",
            "high",
            "projection_coverage",
            partial_gap_sources,
            all_plan_ids.clone(),
            "close partial/gap adapter source mappings before authoritative event replay/readback",
        ),
        application_blocker(
            "work_graph_events_replay_readback_readiness_rerun_missing",
            "medium",
            "readiness_rerun",
            all_sources,
            all_plan_ids,
            "rerun enforcement readiness after no-execution replay/readback application outcomes are available",
        ),
    ]
}

fn application_guard(
    id: &'static str,
    severity: &'static str,
    guard_scope: &'static str,
) -> WorkGraphEventsReplayReadbackApplicationGuardPreview {
    WorkGraphEventsReplayReadbackApplicationGuardPreview {
        id,
        severity,
        guard_scope,
        required_before_append_only_events: true,
        satisfied_by_preview: true,
    }
}

fn application_blocker(
    id: &'static str,
    severity: &'static str,
    category: &'static str,
    affected_source_surface_ids: Vec<&'static str>,
    affected_application_plan_ids: Vec<String>,
    recommended_fix: &'static str,
) -> WorkGraphEventsReplayReadbackApplicationBlockerPreview {
    WorkGraphEventsReplayReadbackApplicationBlockerPreview {
        id,
        severity,
        category,
        affected_source_surface_ids,
        affected_application_plan_ids,
        required_before_append_only_events: true,
        recommended_fix,
    }
}

impl WorkGraphAppendOnlyWorkGraphEventsReplayReadbackApplicationPreviewSideEffects {
    const fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            work_graph_events_persisted: false,
            wal_written: false,
            checkpoint_written: false,
            replay_executed: false,
            readback_executed: false,
            rollback_executed: false,
            idempotency_index_mutated: false,
            adapter_projection_enforced: false,
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
    fn replay_readback_application_maps_readback_verified_sources() {
        let plans = work_graph_append_only_work_graph_events_replay_readback_application_plans_from(
            &sample_readback_plans(),
        );
        let outcomes =
            work_graph_append_only_work_graph_events_replay_readback_application_source_outcomes_from(
                &plans,
            );

        assert_eq!(plans.len(), 2);
        assert_eq!(outcomes.len(), 2);
        assert!(plans.iter().all(|plan| {
            plan.application_state
                == "work_graph_events_replay_readback_contract_ready_preview_after_application"
                && plan.readback_verified_by_preview
                && plan.replay_readback_contract_ready_preview
                && !plan.applies_to_runtime
        }));
        assert!(outcomes.iter().all(|outcome| {
            outcome.replay_readback_contract_ready_preview
                && outcome.ready_for_replay_readback_readiness_rerun_preview
                && !outcome.ready_for_append_only_work_graph_events
                && !outcome.ready_for_replay_readback_execution
        }));
    }

    #[test]
    fn replay_readback_application_keeps_runtime_mutation_disabled() {
        let plans = work_graph_append_only_work_graph_events_replay_readback_application_plans_from(
            &sample_readback_plans(),
        );

        assert!(plans.iter().all(|plan| {
            plan.readback_verified_by_preview
                && plan.replay_readback_contract_ready_preview
                && !plan.applies_to_runtime
                && !plan.persists_work_graph_events
                && !plan.writes_wal
                && !plan.writes_checkpoint
                && !plan.executes_replay
                && !plan.executes_readback
                && !plan.executes_rollback
                && !plan.mutates_idempotency_index
                && !plan.enforces_adapter_projection
        }));
    }

    #[test]
    fn replay_readback_application_artifacts_remain_preview_only() {
        let stage_applications =
            work_graph_append_only_work_graph_events_replay_readback_stage_applications_from(
                &sample_stage_assertions(),
            );
        let evidence_applications =
            work_graph_append_only_work_graph_events_replay_readback_evidence_field_applications_from(
                &sample_evidence_field_assertions(),
            );
        let guard_applications =
            work_graph_append_only_work_graph_events_replay_readback_guard_applications_from(
                &sample_guard_assertions(),
            );

        assert!(stage_applications.iter().all(|application| {
            application.contract_ready_preview
                && !application.persists_work_graph_events
                && !application.executes_replay
                && !application.executes_readback
                && !application.executes_rollback
        }));
        assert!(
            evidence_applications
                .iter()
                .all(|application| application.evidence_contract_ready_preview
                    && !application.persists_evidence)
        );
        assert!(guard_applications.iter().all(|application| {
            application.required_before_replay_readback_execution
                && !application.satisfied_by_preview
                && !application.mutates_runtime
        }));
    }

    #[test]
    fn replay_readback_application_tracks_remaining_blockers_and_side_effects() {
        let plans = work_graph_append_only_work_graph_events_replay_readback_application_plans_from(
            &sample_readback_plans(),
        );
        let blockers =
            work_graph_append_only_work_graph_events_replay_readback_application_blockers_from(
                &plans,
                &sample_readback_blockers(),
            );
        let blocker_applications =
            work_graph_append_only_work_graph_events_replay_readback_blocker_applications_from(
                &sample_blocker_mapping_assertions(),
            );
        let application_guards =
            work_graph_append_only_work_graph_events_replay_readback_application_guards();
        let blocker_ids = blockers
            .iter()
            .map(|blocker| blocker.id)
            .collect::<Vec<_>>();

        assert_eq!(
            blocker_ids,
            vec![
                "append_only_work_graph_events_disabled",
                "replay_readback_execution_disabled",
                "runtime_canonical_adapter_enforcement_disabled",
                "canonical_adapter_projection_partial_or_gap",
                "work_graph_events_replay_readback_readiness_rerun_missing"
            ]
        );
        assert!(
            blocker_applications
                .iter()
                .any(|application| application.clears_application_missing_blocker)
        );
        assert!(
            application_guards
                .iter()
                .all(|guard| guard.required_before_append_only_events && guard.satisfied_by_preview)
        );
        assert_eq!(
            blockers
                .iter()
                .find(|blocker| blocker.id == "canonical_adapter_projection_partial_or_gap")
                .map(|blocker| blocker.affected_source_surface_ids.len()),
            Some(1)
        );
        assert!(
            !WorkGraphAppendOnlyWorkGraphEventsReplayReadbackApplicationPreviewSideEffects::none()
                .work_graph_events_persisted
        );
    }

    fn sample_readback_plans() -> Vec<WorkGraphEventsReplayReadbackReadbackPlanPreview> {
        vec![
            sample_readback_plan("update_plan_tool", "planning"),
            sample_readback_plan("multi_agent_v2_thread_spawn", "multi_agent"),
        ]
    }

    fn sample_readback_plan(
        source_surface_id: &'static str,
        source_category: &'static str,
    ) -> WorkGraphEventsReplayReadbackReadbackPlanPreview {
        WorkGraphEventsReplayReadbackReadbackPlanPreview {
            source_surface_id,
            source_category,
            replay_readback_plan_id: format!(
                "{source_surface_id}_append_only_work_graph_events_replay_readback"
            ),
            expected_stage_count: 8,
            expected_evidence_field_count: 10,
            expected_residual_blocker_count: 3,
            readback_status: "readback_plan_ready",
            readback_execution_enabled: false,
            replay_execution_enabled: false,
            rollback_execution_enabled: false,
            persists_work_graph_events: false,
            next_required_gate:
                WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_REPLAY_READBACK_APPLICATION_PREVIEW_GATE,
        }
    }

    fn sample_stage_assertions() -> Vec<WorkGraphEventsReplayReadbackStageAssertionPreview> {
        vec![WorkGraphEventsReplayReadbackStageAssertionPreview {
            stage_id: "work_graph_events_replay_cursor_contract",
            affected_source_surface_ids: vec!["update_plan_tool"],
            required_contract_ref_ids: vec!["shadow_replay_cursor_contract_ready"],
            contract_ready_preview: true,
            execution_enabled_after_readback: false,
            persistence_enabled_after_readback: false,
        }]
    }

    fn sample_evidence_field_assertions()
    -> Vec<WorkGraphEventsReplayReadbackEvidenceFieldAssertionPreview> {
        vec![WorkGraphEventsReplayReadbackEvidenceFieldAssertionPreview {
            source_surface_id: "update_plan_tool",
            evidence_field_ids: vec!["source_surface_id", "replay_cursor_contract_id"],
            evidence_contract_ready_preview: true,
            persists_evidence_after_readback: false,
        }]
    }

    fn sample_guard_assertions() -> Vec<WorkGraphEventsReplayReadbackGuardAssertionPreview> {
        vec![WorkGraphEventsReplayReadbackGuardAssertionPreview {
            guard_id: "replay_execution_disabled",
            severity: "critical",
            guard_scope: "replay",
            required_before_replay_readback_execution: true,
            satisfied_by_preview: false,
        }]
    }

    fn sample_blocker_mapping_assertions()
    -> Vec<WorkGraphEventsReplayReadbackBlockerMappingAssertionPreview> {
        vec![
            WorkGraphEventsReplayReadbackBlockerMappingAssertionPreview {
                blocker_id: "append_only_work_graph_events_replay_readback_readback_missing",
                affected_source_surface_ids: vec!["update_plan_tool"],
                affected_replay_readback_stage_ids: vec![
                    "work_graph_events_replay_cursor_contract",
                ],
                blocks_replay_readback_execution: true,
            },
        ]
    }

    fn sample_readback_blockers() -> Vec<WorkGraphEventsReplayReadbackReadbackBlockerPreview> {
        vec![WorkGraphEventsReplayReadbackReadbackBlockerPreview {
            id: "canonical_adapter_projection_partial_or_gap",
            severity: "high",
            affected_source_surface_ids: vec!["update_plan_tool"],
            recommended_fix: "close partial/gap adapter source mappings before authoritative event replay/readback",
        }]
    }
}

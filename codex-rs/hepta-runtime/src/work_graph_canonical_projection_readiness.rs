use serde::Serialize;

use crate::wg_upe_tnc_r15_rerun_preview::WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_WORK_GRAPH_EVENTS_EVENT_STORE_CUTOVER_TERMINAL_NO_CUTOVER_RECEIPT_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_CLOSEOUT_RECEIPT_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_CLOSEOUT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RERUN_PREVIEW_GATE as RECEIPT15_RERUN_GATE;
use crate::work_graph_adapter_projection_fixture::WORK_GRAPH_ADAPTER_PROJECTION_FIXTURE_GATE;
use crate::work_graph_append_only_event_intake_preview::WORK_GRAPH_APPEND_ONLY_EVENT_INTAKE_PREVIEW_GATE;
use crate::work_graph_observability_timeline::WORK_GRAPH_OBSERVABILITY_TIMELINE_PREVIEW_GATE;
use crate::work_graph_role_manifest_contract::WORK_GRAPH_ROLE_MANIFEST_CONTRACT_PREVIEW_GATE;
use crate::work_graph_scheduler_admission_controller::WORK_GRAPH_SCHEDULER_ADMISSION_CONTROLLER_PREVIEW_GATE;
use crate::work_graph_task_result_contract::WORK_GRAPH_TASK_RESULT_CONTRACT_PREVIEW_GATE;

pub const WORK_GRAPH_CANONICAL_PROJECTION_READINESS_GATE: &str =
    "hepta_work_graph_canonical_projection_readiness_gate";
pub const WORK_GRAPH_CANONICAL_PROJECTION_READINESS_SCHEMA_VERSION: &str =
    "work_graph_canonical_projection_readiness_v1";
pub const WORK_GRAPH_CANONICAL_PROJECTION_READINESS_RECOMMENDED_NEXT_GATE: &str =
    "hepta_work_graph_task_result_envelope_report_only_validator_gate";
pub const WORK_GRAPH_CANONICAL_PROJECTION_READINESS_RECEIPT15_SOURCE_SURFACE_COUNT: usize = 7;
pub const WORK_GRAPH_CANONICAL_PROJECTION_READINESS_RECEIPT15_READY_SURFACE_COUNT: usize = 7;
pub const WORK_GRAPH_CANONICAL_PROJECTION_READINESS_RECEIPT15_BLOCKED_SURFACE_COUNT: usize = 0;
pub const WORK_GRAPH_CANONICAL_PROJECTION_READINESS_RECEIPT15_REQUIRED_PRIOR_GATE_COUNT: usize =
    180;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphCanonicalProjectionReadinessReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub receipt_tail_frozen_at: &'static str,
    pub receipt15_source_surface_count: usize,
    pub receipt15_ready_surface_count: usize,
    pub receipt15_blocked_surface_count: usize,
    pub receipt15_required_prior_gate_count: usize,
    pub contract_count: usize,
    pub contract_ready_count: usize,
    pub blocker_count: usize,
    pub contracts: Vec<WorkGraphCanonicalProjectionContractReadiness>,
    pub blockers: Vec<WorkGraphCanonicalProjectionReadinessBlocker>,
    pub recommended_next_gate: &'static str,
    pub ready_for_task_result_envelope_report_only_validator: bool,
    pub ready_for_scheduler_admission_dry_run_enforcement: bool,
    pub ready_for_append_only_event_store_shadow_path: bool,
    pub ready_for_live_execution: bool,
    pub side_effects: WorkGraphCanonicalProjectionReadinessSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphCanonicalProjectionContractReadiness {
    pub id: &'static str,
    pub source_gate: &'static str,
    pub canonical_surface: &'static str,
    pub ready: bool,
    pub enforcement_enabled: bool,
    pub persistence_enabled: bool,
    pub note: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphCanonicalProjectionReadinessBlocker {
    pub id: &'static str,
    pub severity: &'static str,
    pub affected_contract_ids: Vec<&'static str>,
    pub recommended_fix: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphCanonicalProjectionReadinessSideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub work_graph_event_persisted: bool,
    pub event_store_enabled: bool,
    pub scheduler_admission_enforced: bool,
    pub task_result_enforcement_enabled: bool,
    pub receipt_tail_extended: bool,
    pub runtime_mutation_performed: bool,
    pub agent_spawn_performed: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_canonical_projection_readiness_report()
-> WorkGraphCanonicalProjectionReadinessReport {
    let contracts = work_graph_canonical_projection_readiness_contracts();
    let blockers = work_graph_canonical_projection_readiness_blockers(&contracts);
    let contract_ready_count = contracts.iter().filter(|contract| contract.ready).count();

    WorkGraphCanonicalProjectionReadinessReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_CANONICAL_PROJECTION_READINESS_GATE,
        schema_version: WORK_GRAPH_CANONICAL_PROJECTION_READINESS_SCHEMA_VERSION,
        preview_mode: "read_only_receipt15_canonical_projection_rollup_no_tail_extension",
        receipt_tail_frozen_at: RECEIPT15_RERUN_GATE,
        receipt15_source_surface_count:
            WORK_GRAPH_CANONICAL_PROJECTION_READINESS_RECEIPT15_SOURCE_SURFACE_COUNT,
        receipt15_ready_surface_count:
            WORK_GRAPH_CANONICAL_PROJECTION_READINESS_RECEIPT15_READY_SURFACE_COUNT,
        receipt15_blocked_surface_count:
            WORK_GRAPH_CANONICAL_PROJECTION_READINESS_RECEIPT15_BLOCKED_SURFACE_COUNT,
        receipt15_required_prior_gate_count:
            WORK_GRAPH_CANONICAL_PROJECTION_READINESS_RECEIPT15_REQUIRED_PRIOR_GATE_COUNT,
        contract_count: contracts.len(),
        contract_ready_count,
        blocker_count: blockers.len(),
        contracts,
        blockers,
        recommended_next_gate: WORK_GRAPH_CANONICAL_PROJECTION_READINESS_RECOMMENDED_NEXT_GATE,
        ready_for_task_result_envelope_report_only_validator: true,
        ready_for_scheduler_admission_dry_run_enforcement: true,
        ready_for_append_only_event_store_shadow_path: true,
        ready_for_live_execution: false,
        side_effects: WorkGraphCanonicalProjectionReadinessSideEffects::none(),
    }
}

pub fn work_graph_canonical_projection_readiness_contracts()
-> Vec<WorkGraphCanonicalProjectionContractReadiness> {
    vec![
        contract(
            "receipt15_terminal_no_cutover_proof",
            RECEIPT15_RERUN_GATE,
            "event_store_cutover_terminal_no_cutover",
            "receipt15 proves ready 7 / blocked 0 / residual blockers 0 without enabling event-store cutover",
        ),
        contract(
            "adapter_projection_fixture",
            WORK_GRAPH_ADAPTER_PROJECTION_FIXTURE_GATE,
            "canonical_projection_fixture",
            "existing planning, agent, worker, task-board, scheduler, approval, and handoff surfaces are projectable",
        ),
        contract(
            "task_result_contract",
            WORK_GRAPH_TASK_RESULT_CONTRACT_PREVIEW_GATE,
            "terminal_task_result_contract",
            "canonical TaskResult fields exist and now need report-only envelope validation at producers",
        ),
        contract(
            "scheduler_admission_controller",
            WORK_GRAPH_SCHEDULER_ADMISSION_CONTROLLER_PREVIEW_GATE,
            "scheduler_admission_checks",
            "dependency, lease, approval, idempotency, budget, TaskResult, and side-effect checks exist in preview",
        ),
        contract(
            "role_manifest_contract",
            WORK_GRAPH_ROLE_MANIFEST_CONTRACT_PREVIEW_GATE,
            "agent_role_agent_card_manifest",
            "role capability, tool permission, budget, verifier, lane, and trace policy are modeled but not enforced",
        ),
        contract(
            "append_only_event_intake",
            WORK_GRAPH_APPEND_ONLY_EVENT_INTAKE_PREVIEW_GATE,
            "append_only_work_graph_event_shadow_path",
            "redacted append-only event routes exist as preview contracts with persistence disabled",
        ),
        contract(
            "observability_timeline",
            WORK_GRAPH_OBSERVABILITY_TIMELINE_PREVIEW_GATE,
            "trace_guardrail_timeline",
            "traceable plan, spawn, mailbox, tool, result, artifact, approval, and guardrail events are previewable",
        ),
    ]
}

pub fn work_graph_canonical_projection_readiness_blockers(
    contracts: &[WorkGraphCanonicalProjectionContractReadiness],
) -> Vec<WorkGraphCanonicalProjectionReadinessBlocker> {
    let missing = contracts
        .iter()
        .filter(|contract| !contract.ready)
        .map(|contract| contract.id)
        .collect::<Vec<_>>();

    if missing.is_empty() {
        Vec::new()
    } else {
        vec![WorkGraphCanonicalProjectionReadinessBlocker {
            id: "canonical_projection_contract_missing",
            severity: "high",
            affected_contract_ids: missing,
            recommended_fix: "restore all canonical projection contracts before TaskResultEnvelope or scheduler admission promotion",
        }]
    }
}

impl WorkGraphCanonicalProjectionReadinessSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            work_graph_event_persisted: false,
            event_store_enabled: false,
            scheduler_admission_enforced: false,
            task_result_enforcement_enabled: false,
            receipt_tail_extended: false,
            runtime_mutation_performed: false,
            agent_spawn_performed: false,
            external_send_performed: false,
            model_invoked: false,
        }
    }
}

fn contract(
    id: &'static str,
    source_gate: &'static str,
    canonical_surface: &'static str,
    note: &'static str,
) -> WorkGraphCanonicalProjectionContractReadiness {
    WorkGraphCanonicalProjectionContractReadiness {
        id,
        source_gate,
        canonical_surface,
        ready: true,
        enforcement_enabled: false,
        persistence_enabled: false,
        note,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_projection_readiness_freezes_receipt15_tail() {
        let report = hepta_work_graph_canonical_projection_readiness_report();

        assert_eq!(report.receipt_tail_frozen_at, RECEIPT15_RERUN_GATE);
        assert_eq!(report.receipt15_source_surface_count, 7);
        assert_eq!(report.receipt15_ready_surface_count, 7);
        assert_eq!(report.receipt15_blocked_surface_count, 0);
        assert_eq!(report.receipt15_required_prior_gate_count, 180);
        assert!(!report.side_effects.receipt_tail_extended);
    }

    #[test]
    fn canonical_projection_readiness_names_rollup_contracts() {
        let report = hepta_work_graph_canonical_projection_readiness_report();
        let contract_ids = report
            .contracts
            .iter()
            .map(|contract| contract.id)
            .collect::<Vec<_>>();

        assert_eq!(
            contract_ids,
            [
                "receipt15_terminal_no_cutover_proof",
                "adapter_projection_fixture",
                "task_result_contract",
                "scheduler_admission_controller",
                "role_manifest_contract",
                "append_only_event_intake",
                "observability_timeline",
            ]
        );
        assert_eq!(report.contract_count, 7);
        assert_eq!(report.contract_ready_count, 7);
        assert_eq!(report.blocker_count, 0);
    }

    #[test]
    fn canonical_projection_readiness_recommends_task_result_envelope_next() {
        let report = hepta_work_graph_canonical_projection_readiness_report();

        assert_eq!(
            report.recommended_next_gate,
            WORK_GRAPH_CANONICAL_PROJECTION_READINESS_RECOMMENDED_NEXT_GATE
        );
        assert!(report.ready_for_task_result_envelope_report_only_validator);
        assert!(report.ready_for_scheduler_admission_dry_run_enforcement);
        assert!(report.ready_for_append_only_event_store_shadow_path);
        assert!(!report.ready_for_live_execution);
    }

    #[test]
    fn canonical_projection_readiness_keeps_all_side_effects_false() {
        let report = hepta_work_graph_canonical_projection_readiness_report();

        assert_eq!(
            report.side_effects,
            WorkGraphCanonicalProjectionReadinessSideEffects::none()
        );
        assert!(
            report
                .contracts
                .iter()
                .all(|contract| !contract.enforcement_enabled && !contract.persistence_enabled)
        );
    }
}

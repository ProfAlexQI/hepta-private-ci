use std::collections::BTreeMap;

use serde::Serialize;

use crate::work_graph_source_id_alignment_readback::WORK_GRAPH_SOURCE_ID_ALIGNMENT_READBACK_GATE;
use crate::work_graph_source_id_alignment_readback::hepta_work_graph_source_id_alignment_readback_report;
use crate::work_graph_task_result_contract::WORK_GRAPH_TASK_RESULT_CONTRACT_PREVIEW_GATE;
use crate::work_graph_task_result_contract::hepta_work_graph_task_result_contract_preview_report;
use crate::work_graph_terminal_envelope_readback::WORK_GRAPH_TERMINAL_ENVELOPE_READBACK_GATE;
use crate::work_graph_terminal_envelope_readback::hepta_work_graph_terminal_envelope_readback_report;

pub const WORK_GRAPH_TASK_RESULT_CONTRACT_FIELD_GAP_READBACK_GATE: &str =
    "hepta_work_graph_task_result_contract_field_gap_readback_gate";
pub const WORK_GRAPH_TASK_RESULT_CONTRACT_FIELD_GAP_READBACK_SCHEMA_VERSION: &str =
    "work_graph_task_result_contract_field_gap_readback_v1";
pub const WORK_GRAPH_TASK_RESULT_CONTRACT_FIELD_GAP_READBACK_RECOMMENDED_NEXT_GATE: &str =
    "hepta_work_graph_append_only_event_store_shadow_path_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTaskResultContractFieldGapReadbackReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub required_wire_field_count: usize,
    pub terminal_source_count: usize,
    pub terminal_source_full_contract_count: usize,
    pub gap_source_count: usize,
    pub contract_required_field_gap_count: usize,
    pub contract_terminal_field_gap_count: usize,
    pub field_readbacks: Vec<WorkGraphTaskResultContractFieldGapReadbackSource>,
    pub blockers: Vec<WorkGraphTaskResultContractFieldGapReadbackBlocker>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub task_result_contract_field_gap_readback_complete: bool,
    pub ready_for_append_only_event_store_shadow_path: bool,
    pub ready_for_task_result_enforcement: bool,
    pub ready_for_live_execution: bool,
    pub side_effects: WorkGraphTaskResultContractFieldGapReadbackSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTaskResultContractFieldGapReadbackSource {
    pub source_surface_id: &'static str,
    pub canonical_node_kind: &'static str,
    pub task_result_contract_adapter_state: &'static str,
    pub covered_contract_wire_fields: Vec<&'static str>,
    pub missing_contract_required_wire_fields: Vec<&'static str>,
    pub missing_contract_terminal_wire_fields: Vec<&'static str>,
    pub readback_decision: &'static str,
    pub live_enforcement_enabled: bool,
    pub next_gap_step: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTaskResultContractFieldGapReadbackBlocker {
    pub id: &'static str,
    pub severity: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub blocks_live_execution: bool,
    pub recommended_fix: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphTaskResultContractFieldGapReadbackSideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub work_graph_event_persisted: bool,
    pub adapter_projection_enforced: bool,
    pub task_result_enforcement_enabled: bool,
    pub scheduler_admission_enforced: bool,
    pub live_admission_enforcement_enabled: bool,
    pub runtime_mutation_performed: bool,
    pub approval_recorded: bool,
    pub agent_spawn_performed: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_task_result_contract_field_gap_readback_report()
-> WorkGraphTaskResultContractFieldGapReadbackReport {
    let contract = hepta_work_graph_task_result_contract_preview_report();
    let terminal_readback = hepta_work_graph_terminal_envelope_readback_report();
    let source_alignment = hepta_work_graph_source_id_alignment_readback_report();
    let required_wire_fields = contract
        .required_fields
        .iter()
        .map(|field| field.wire_name)
        .collect::<Vec<_>>();
    let adapters_by_source = contract
        .adapter_previews
        .iter()
        .map(|adapter| (adapter.source_surface_id, adapter))
        .collect::<BTreeMap<_, _>>();

    let field_readbacks = terminal_readback
        .terminal_sources
        .iter()
        .map(|source| {
            let contract_adapter = adapters_by_source.get(source.source_surface_id);
            let covered_contract_wire_fields = contract_adapter
                .map(|adapter| adapter.covered_wire_fields.clone())
                .unwrap_or_default();
            let complete = contract_adapter.is_some()
                && source.missing_contract_required_wire_fields.is_empty()
                && source.missing_contract_terminal_wire_fields.is_empty()
                && covered_contract_wire_fields == required_wire_fields;

            WorkGraphTaskResultContractFieldGapReadbackSource {
                source_surface_id: source.source_surface_id,
                canonical_node_kind: source.canonical_node_kind,
                task_result_contract_adapter_state: if contract_adapter.is_some() {
                    "present_report_only"
                } else {
                    "missing"
                },
                covered_contract_wire_fields,
                missing_contract_required_wire_fields: source
                    .missing_contract_required_wire_fields
                    .clone(),
                missing_contract_terminal_wire_fields: source
                    .missing_contract_terminal_wire_fields
                    .clone(),
                readback_decision: if complete {
                    "task_result_contract_fields_complete_report_only"
                } else {
                    "task_result_contract_field_gap_remaining"
                },
                live_enforcement_enabled: false,
                next_gap_step: if complete {
                    "append_only_event_store_shadow_path"
                } else {
                    "fill_task_result_contract_wire_field_projection"
                },
            }
        })
        .collect::<Vec<_>>();

    let terminal_source_full_contract_count = field_readbacks
        .iter()
        .filter(|readback| {
            readback.readback_decision == "task_result_contract_fields_complete_report_only"
        })
        .count();
    let gap_source_count = field_readbacks
        .iter()
        .filter(|readback| {
            !readback.missing_contract_required_wire_fields.is_empty()
                || !readback.missing_contract_terminal_wire_fields.is_empty()
                || readback.task_result_contract_adapter_state != "present_report_only"
        })
        .count();
    let contract_required_field_gap_count = field_readbacks
        .iter()
        .map(|readback| readback.missing_contract_required_wire_fields.len())
        .sum();
    let contract_terminal_field_gap_count = field_readbacks
        .iter()
        .map(|readback| readback.missing_contract_terminal_wire_fields.len())
        .sum();
    let task_result_contract_field_gap_readback_complete = gap_source_count == 0
        && contract_required_field_gap_count == 0
        && contract_terminal_field_gap_count == 0
        && source_alignment.source_id_alignment_readback_complete;
    let blockers = field_gap_readback_blockers(
        &field_readbacks,
        task_result_contract_field_gap_readback_complete,
    );

    WorkGraphTaskResultContractFieldGapReadbackReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_TASK_RESULT_CONTRACT_FIELD_GAP_READBACK_GATE,
        schema_version: WORK_GRAPH_TASK_RESULT_CONTRACT_FIELD_GAP_READBACK_SCHEMA_VERSION,
        preview_mode: "read_only_task_result_contract_field_gap_readback_no_enforcement",
        required_wire_field_count: required_wire_fields.len(),
        terminal_source_count: field_readbacks.len(),
        terminal_source_full_contract_count,
        gap_source_count,
        contract_required_field_gap_count,
        contract_terminal_field_gap_count,
        field_readbacks,
        blockers,
        required_prior_gates: vec![
            WORK_GRAPH_TASK_RESULT_CONTRACT_PREVIEW_GATE,
            WORK_GRAPH_TERMINAL_ENVELOPE_READBACK_GATE,
            WORK_GRAPH_SOURCE_ID_ALIGNMENT_READBACK_GATE,
        ],
        recommended_next_gate:
            WORK_GRAPH_TASK_RESULT_CONTRACT_FIELD_GAP_READBACK_RECOMMENDED_NEXT_GATE,
        task_result_contract_field_gap_readback_complete,
        ready_for_append_only_event_store_shadow_path:
            task_result_contract_field_gap_readback_complete,
        ready_for_task_result_enforcement: false,
        ready_for_live_execution: false,
        side_effects: WorkGraphTaskResultContractFieldGapReadbackSideEffects::none(),
    }
}

impl WorkGraphTaskResultContractFieldGapReadbackSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            work_graph_event_persisted: false,
            adapter_projection_enforced: false,
            task_result_enforcement_enabled: false,
            scheduler_admission_enforced: false,
            live_admission_enforcement_enabled: false,
            runtime_mutation_performed: false,
            approval_recorded: false,
            agent_spawn_performed: false,
            external_send_performed: false,
            model_invoked: false,
        }
    }
}

fn field_gap_readback_blockers(
    field_readbacks: &[WorkGraphTaskResultContractFieldGapReadbackSource],
    task_result_contract_field_gap_readback_complete: bool,
) -> Vec<WorkGraphTaskResultContractFieldGapReadbackBlocker> {
    let mut blockers = Vec::new();
    if !task_result_contract_field_gap_readback_complete {
        blockers.push(blocker(
            "task_result_contract_field_gap_remaining",
            "high",
            field_readbacks
                .iter()
                .filter(|readback| {
                    readback.task_result_contract_adapter_state != "present_report_only"
                        || !readback.missing_contract_required_wire_fields.is_empty()
                        || !readback.missing_contract_terminal_wire_fields.is_empty()
                })
                .map(|readback| readback.source_surface_id)
                .collect(),
            "fill all required and terminal TaskResult contract wire fields before event-store shadow promotion",
        ));
    }

    blockers.push(blocker(
        "append_only_event_store_shadow_path_not_enabled",
        "high",
        field_readbacks
            .iter()
            .map(|readback| readback.source_surface_id)
            .collect(),
        "shadow-write TaskResult events and replay them before enabling durable WorkGraph enforcement",
    ));
    blockers.push(blocker(
        "task_result_contract_live_enforcement_disabled",
        "high",
        field_readbacks
            .iter()
            .map(|readback| readback.source_surface_id)
            .collect(),
        "keep TaskResult contract validation report-only until append-only event-store, replay, and operator review gates pass",
    ));

    blockers
}

fn blocker(
    id: &'static str,
    severity: &'static str,
    affected_source_surface_ids: Vec<&'static str>,
    recommended_fix: &'static str,
) -> WorkGraphTaskResultContractFieldGapReadbackBlocker {
    WorkGraphTaskResultContractFieldGapReadbackBlocker {
        id,
        severity,
        affected_source_surface_ids,
        blocks_live_execution: true,
        recommended_fix,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_result_contract_field_gap_readback_covers_terminal_sources() {
        let report = hepta_work_graph_task_result_contract_field_gap_readback_report();
        let source_ids = report
            .field_readbacks
            .iter()
            .map(|readback| readback.source_surface_id)
            .collect::<Vec<_>>();

        assert_eq!(
            source_ids,
            [
                "multi_agent_v2_thread_spawn",
                "hepta_runtime_multi_agent_reducer",
                "agent_jobs_batch_workers",
                "hepta_runtime_worker_tasks",
                "hepta_runtime_scheduler_store",
                "hepta_runtime_agent_harness",
            ]
        );
        assert_eq!(report.required_wire_field_count, 11);
        assert_eq!(report.terminal_source_count, 6);
        assert_eq!(report.terminal_source_full_contract_count, 6);
        assert_eq!(report.gap_source_count, 0);
    }

    #[test]
    fn task_result_contract_field_gap_readback_has_zero_missing_fields() {
        let report = hepta_work_graph_task_result_contract_field_gap_readback_report();

        assert_eq!(report.contract_required_field_gap_count, 0);
        assert_eq!(report.contract_terminal_field_gap_count, 0);
        assert!(report.field_readbacks.iter().all(|readback| {
            readback.task_result_contract_adapter_state == "present_report_only"
                && readback.missing_contract_required_wire_fields.is_empty()
                && readback.missing_contract_terminal_wire_fields.is_empty()
                && readback.readback_decision == "task_result_contract_fields_complete_report_only"
                && readback.next_gap_step == "append_only_event_store_shadow_path"
                && !readback.live_enforcement_enabled
        }));
    }

    #[test]
    fn task_result_contract_field_gap_readback_keeps_live_blockers_visible() {
        let report = hepta_work_graph_task_result_contract_field_gap_readback_report();
        let blocker_ids = report
            .blockers
            .iter()
            .map(|blocker| blocker.id)
            .collect::<Vec<_>>();

        assert!(!blocker_ids.contains(&"task_result_contract_field_gap_remaining"));
        assert!(blocker_ids.contains(&"append_only_event_store_shadow_path_not_enabled"));
        assert!(blocker_ids.contains(&"task_result_contract_live_enforcement_disabled"));
        assert!(report.task_result_contract_field_gap_readback_complete);
        assert!(report.ready_for_append_only_event_store_shadow_path);
        assert!(!report.ready_for_task_result_enforcement);
        assert!(!report.ready_for_live_execution);
    }

    #[test]
    fn task_result_contract_field_gap_readback_is_non_mutating() {
        let report = hepta_work_graph_task_result_contract_field_gap_readback_report();

        assert_eq!(
            report.required_prior_gates,
            [
                WORK_GRAPH_TASK_RESULT_CONTRACT_PREVIEW_GATE,
                WORK_GRAPH_TERMINAL_ENVELOPE_READBACK_GATE,
                WORK_GRAPH_SOURCE_ID_ALIGNMENT_READBACK_GATE,
            ]
        );
        assert_eq!(
            report.recommended_next_gate,
            WORK_GRAPH_TASK_RESULT_CONTRACT_FIELD_GAP_READBACK_RECOMMENDED_NEXT_GATE
        );
        assert_eq!(
            report.side_effects,
            WorkGraphTaskResultContractFieldGapReadbackSideEffects::none()
        );
    }
}

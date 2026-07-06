use serde::Serialize;

use crate::work_graph_adapter_task_result_index::{
    WORK_GRAPH_ADAPTER_TASK_RESULT_INDEX_GATE, hepta_work_graph_adapter_task_result_index_report,
};
use crate::work_graph_task_result_envelope_report_only_validator::WORK_GRAPH_TASK_RESULT_ENVELOPE_REPORT_ONLY_VALIDATOR_GATE;

pub const WORK_GRAPH_TERMINAL_ENVELOPE_READBACK_GATE: &str =
    "hepta_work_graph_terminal_envelope_readback_gate";
pub const WORK_GRAPH_TERMINAL_ENVELOPE_READBACK_SCHEMA_VERSION: &str =
    "work_graph_terminal_envelope_readback_v1";
pub const WORK_GRAPH_TERMINAL_ENVELOPE_READBACK_RECOMMENDED_NEXT_GATE: &str =
    "hepta_work_graph_source_id_alignment_readback_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalEnvelopeReadbackReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub terminal_source_count: usize,
    pub terminal_envelope_adapter_count: usize,
    pub terminal_envelope_preview_count: usize,
    pub readback_consistent_source_count: usize,
    pub missing_terminal_envelope_adapter_count: usize,
    pub missing_terminal_envelope_preview_count: usize,
    pub missing_terminal_envelope_wire_field_count: usize,
    pub task_result_contract_required_field_gap_count: usize,
    pub task_result_contract_terminal_field_gap_count: usize,
    pub terminal_sources: Vec<WorkGraphTerminalEnvelopeReadbackSource>,
    pub blockers: Vec<WorkGraphTerminalEnvelopeReadbackBlocker>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub ready_for_source_id_alignment_readback: bool,
    pub ready_for_task_result_enforcement: bool,
    pub ready_for_live_execution: bool,
    pub side_effects: WorkGraphTerminalEnvelopeReadbackSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalEnvelopeReadbackSource {
    pub source_surface_id: &'static str,
    pub canonical_node_kind: &'static str,
    pub task_result_envelope_adapter_state: &'static str,
    pub task_result_envelope_preview_state: &'static str,
    pub missing_envelope_wire_fields: Vec<&'static str>,
    pub missing_contract_required_wire_fields: Vec<&'static str>,
    pub missing_contract_terminal_wire_fields: Vec<&'static str>,
    pub readback_decision: &'static str,
    pub live_enforcement_enabled: bool,
    pub next_readback_step: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalEnvelopeReadbackBlocker {
    pub id: &'static str,
    pub severity: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub blocks_live_execution: bool,
    pub recommended_fix: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphTerminalEnvelopeReadbackSideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub work_graph_event_persisted: bool,
    pub task_result_enforcement_enabled: bool,
    pub scheduler_admission_enforced: bool,
    pub live_admission_enforcement_enabled: bool,
    pub runtime_mutation_performed: bool,
    pub agent_spawn_performed: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_terminal_envelope_readback_report()
-> WorkGraphTerminalEnvelopeReadbackReport {
    let index_report = hepta_work_graph_adapter_task_result_index_report();
    let terminal_sources = index_report
        .source_index
        .iter()
        .filter(|entry| entry.terminal_task_result_required)
        .map(|entry| {
            let readback_consistent = entry.task_result_envelope_adapter_state
                == "present_report_only"
                && entry.task_result_envelope_preview_state == "present_report_only"
                && entry.missing_envelope_wire_fields.is_empty();
            WorkGraphTerminalEnvelopeReadbackSource {
                source_surface_id: entry.source_surface_id,
                canonical_node_kind: entry.canonical_node_kind,
                task_result_envelope_adapter_state: entry.task_result_envelope_adapter_state,
                task_result_envelope_preview_state: entry.task_result_envelope_preview_state,
                missing_envelope_wire_fields: entry.missing_envelope_wire_fields.clone(),
                missing_contract_required_wire_fields: entry
                    .missing_contract_required_wire_fields
                    .clone(),
                missing_contract_terminal_wire_fields: entry
                    .missing_contract_terminal_wire_fields
                    .clone(),
                readback_decision: if readback_consistent {
                    "terminal_envelope_readback_consistent_report_only"
                } else {
                    "terminal_envelope_readback_missing_report_only"
                },
                live_enforcement_enabled: false,
                next_readback_step: if readback_consistent {
                    "source_id_alignment_readback"
                } else {
                    "add_terminal_task_result_envelope_preview"
                },
            }
        })
        .collect::<Vec<_>>();

    let terminal_envelope_adapter_count = terminal_sources
        .iter()
        .filter(|source| source.task_result_envelope_adapter_state == "present_report_only")
        .count();
    let terminal_envelope_preview_count = terminal_sources
        .iter()
        .filter(|source| source.task_result_envelope_preview_state == "present_report_only")
        .count();
    let readback_consistent_source_count = terminal_sources
        .iter()
        .filter(|source| {
            source.readback_decision == "terminal_envelope_readback_consistent_report_only"
        })
        .count();
    let missing_terminal_envelope_adapter_count =
        terminal_sources.len() - terminal_envelope_adapter_count;
    let missing_terminal_envelope_preview_count =
        terminal_sources.len() - terminal_envelope_preview_count;
    let missing_terminal_envelope_wire_field_count = terminal_sources
        .iter()
        .map(|source| source.missing_envelope_wire_fields.len())
        .sum();
    let task_result_contract_required_field_gap_count = terminal_sources
        .iter()
        .map(|source| source.missing_contract_required_wire_fields.len())
        .sum();
    let task_result_contract_terminal_field_gap_count = terminal_sources
        .iter()
        .map(|source| source.missing_contract_terminal_wire_fields.len())
        .sum();
    let blockers = terminal_envelope_readback_blockers(
        &terminal_sources,
        task_result_contract_required_field_gap_count,
    );
    let ready_for_source_id_alignment_readback =
        readback_consistent_source_count == terminal_sources.len();

    WorkGraphTerminalEnvelopeReadbackReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_TERMINAL_ENVELOPE_READBACK_GATE,
        schema_version: WORK_GRAPH_TERMINAL_ENVELOPE_READBACK_SCHEMA_VERSION,
        preview_mode: "read_only_terminal_envelope_readback_no_live_enforcement",
        terminal_source_count: terminal_sources.len(),
        terminal_envelope_adapter_count,
        terminal_envelope_preview_count,
        readback_consistent_source_count,
        missing_terminal_envelope_adapter_count,
        missing_terminal_envelope_preview_count,
        missing_terminal_envelope_wire_field_count,
        task_result_contract_required_field_gap_count,
        task_result_contract_terminal_field_gap_count,
        terminal_sources,
        blockers,
        required_prior_gates: vec![
            WORK_GRAPH_TASK_RESULT_ENVELOPE_REPORT_ONLY_VALIDATOR_GATE,
            WORK_GRAPH_ADAPTER_TASK_RESULT_INDEX_GATE,
        ],
        recommended_next_gate: WORK_GRAPH_TERMINAL_ENVELOPE_READBACK_RECOMMENDED_NEXT_GATE,
        ready_for_source_id_alignment_readback,
        ready_for_task_result_enforcement: false,
        ready_for_live_execution: false,
        side_effects: WorkGraphTerminalEnvelopeReadbackSideEffects::none(),
    }
}

impl WorkGraphTerminalEnvelopeReadbackSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            work_graph_event_persisted: false,
            task_result_enforcement_enabled: false,
            scheduler_admission_enforced: false,
            live_admission_enforcement_enabled: false,
            runtime_mutation_performed: false,
            agent_spawn_performed: false,
            external_send_performed: false,
            model_invoked: false,
        }
    }
}

fn terminal_envelope_readback_blockers(
    terminal_sources: &[WorkGraphTerminalEnvelopeReadbackSource],
    task_result_contract_required_field_gap_count: usize,
) -> Vec<WorkGraphTerminalEnvelopeReadbackBlocker> {
    let mut blockers = Vec::new();
    let missing_envelope_sources = terminal_sources
        .iter()
        .filter(|source| {
            source.task_result_envelope_adapter_state != "present_report_only"
                || source.task_result_envelope_preview_state != "present_report_only"
                || !source.missing_envelope_wire_fields.is_empty()
        })
        .map(|source| source.source_surface_id)
        .collect::<Vec<_>>();
    if !missing_envelope_sources.is_empty() {
        blockers.push(blocker(
            "terminal_task_result_envelope_readback_missing",
            "high",
            missing_envelope_sources,
            "add report-only terminal envelope adapters and preview envelopes for every terminal source",
        ));
    }

    if task_result_contract_required_field_gap_count > 0 {
        blockers.push(blocker(
            "task_result_contract_required_fields_partial",
            "medium",
            terminal_sources
                .iter()
                .filter(|source| !source.missing_contract_required_wire_fields.is_empty())
                .map(|source| source.source_surface_id)
                .collect(),
            "fill missing TaskResult contract field projections before enabling validator enforcement",
        ));
    }

    blockers.push(blocker(
        "terminal_envelope_live_enforcement_disabled",
        "high",
        terminal_sources
            .iter()
            .map(|source| source.source_surface_id)
            .collect(),
        "keep terminal envelope validation report-only until source alignment, contract field, event-store, replay, and operator review gates pass",
    ));

    blockers
}

fn blocker(
    id: &'static str,
    severity: &'static str,
    affected_source_surface_ids: Vec<&'static str>,
    recommended_fix: &'static str,
) -> WorkGraphTerminalEnvelopeReadbackBlocker {
    WorkGraphTerminalEnvelopeReadbackBlocker {
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
    fn terminal_envelope_readback_covers_all_terminal_sources() {
        let report = hepta_work_graph_terminal_envelope_readback_report();
        let source_ids = report
            .terminal_sources
            .iter()
            .map(|source| source.source_surface_id)
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
        assert_eq!(report.terminal_source_count, 6);
        assert_eq!(report.terminal_envelope_adapter_count, 6);
        assert_eq!(report.terminal_envelope_preview_count, 6);
        assert_eq!(report.readback_consistent_source_count, 6);
    }

    #[test]
    fn terminal_envelope_readback_has_no_missing_envelope_fields() {
        let report = hepta_work_graph_terminal_envelope_readback_report();

        assert_eq!(report.missing_terminal_envelope_adapter_count, 0);
        assert_eq!(report.missing_terminal_envelope_preview_count, 0);
        assert_eq!(report.missing_terminal_envelope_wire_field_count, 0);
        assert!(report.terminal_sources.iter().all(|source| {
            source.readback_decision == "terminal_envelope_readback_consistent_report_only"
                && source.next_readback_step == "source_id_alignment_readback"
                && !source.live_enforcement_enabled
        }));
    }

    #[test]
    fn terminal_envelope_readback_keeps_contract_and_live_blockers_visible() {
        let report = hepta_work_graph_terminal_envelope_readback_report();
        let blocker_ids = report
            .blockers
            .iter()
            .map(|blocker| blocker.id)
            .collect::<Vec<_>>();

        assert_eq!(report.task_result_contract_required_field_gap_count, 0);
        assert_eq!(report.task_result_contract_terminal_field_gap_count, 0);
        assert!(!blocker_ids.contains(&"terminal_task_result_envelope_readback_missing"));
        assert!(!blocker_ids.contains(&"task_result_contract_required_fields_partial"));
        assert!(blocker_ids.contains(&"terminal_envelope_live_enforcement_disabled"));
    }

    #[test]
    fn terminal_envelope_readback_is_non_mutating() {
        let report = hepta_work_graph_terminal_envelope_readback_report();

        assert_eq!(
            report.required_prior_gates,
            [
                WORK_GRAPH_TASK_RESULT_ENVELOPE_REPORT_ONLY_VALIDATOR_GATE,
                WORK_GRAPH_ADAPTER_TASK_RESULT_INDEX_GATE,
            ]
        );
        assert_eq!(
            report.recommended_next_gate,
            WORK_GRAPH_TERMINAL_ENVELOPE_READBACK_RECOMMENDED_NEXT_GATE
        );
        assert!(report.ready_for_source_id_alignment_readback);
        assert!(!report.ready_for_task_result_enforcement);
        assert!(!report.ready_for_live_execution);
        assert_eq!(
            report.side_effects,
            WorkGraphTerminalEnvelopeReadbackSideEffects::none()
        );
    }
}

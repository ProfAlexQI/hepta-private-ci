use std::collections::BTreeSet;

use serde::Serialize;

use crate::work_graph_adapter_task_result_index::{
    WORK_GRAPH_ADAPTER_TASK_RESULT_INDEX_GATE, hepta_work_graph_adapter_task_result_index_report,
};
use crate::work_graph_current_state_inventory::{
    WORK_GRAPH_CURRENT_STATE_INVENTORY_GATE, work_graph_current_state_source_surfaces,
};
use crate::work_graph_terminal_envelope_readback::{
    WORK_GRAPH_TERMINAL_ENVELOPE_READBACK_GATE, hepta_work_graph_terminal_envelope_readback_report,
};

pub const WORK_GRAPH_SOURCE_ID_ALIGNMENT_READBACK_GATE: &str =
    "hepta_work_graph_source_id_alignment_readback_gate";
pub const WORK_GRAPH_SOURCE_ID_ALIGNMENT_READBACK_SCHEMA_VERSION: &str =
    "work_graph_source_id_alignment_readback_v1";
pub const WORK_GRAPH_SOURCE_ID_ALIGNMENT_READBACK_RECOMMENDED_NEXT_GATE: &str =
    "hepta_work_graph_task_result_contract_field_gap_readback_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphSourceIdAlignmentReadbackReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub current_state_source_surface_count: usize,
    pub canonical_adapter_source_surface_count: usize,
    pub alignment_entry_count: usize,
    pub direct_alignment_count: usize,
    pub current_state_alias_alignment_count: usize,
    pub canonical_shadow_only_alignment_count: usize,
    pub unresolved_current_state_source_count: usize,
    pub unresolved_canonical_adapter_source_count: usize,
    pub terminal_envelope_readback_consistent_source_count: usize,
    pub task_result_contract_required_field_gap_count: usize,
    pub task_result_contract_terminal_field_gap_count: usize,
    pub alignments: Vec<WorkGraphSourceIdAlignmentReadbackEntry>,
    pub unresolved_current_state_source_ids: Vec<&'static str>,
    pub unresolved_canonical_adapter_source_ids: Vec<&'static str>,
    pub blockers: Vec<WorkGraphSourceIdAlignmentReadbackBlocker>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub source_id_alignment_readback_complete: bool,
    pub ready_for_task_result_contract_field_gap_readback: bool,
    pub ready_for_append_only_event_store_shadow_path: bool,
    pub ready_for_live_execution: bool,
    pub side_effects: WorkGraphSourceIdAlignmentReadbackSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphSourceIdAlignmentReadbackEntry {
    pub current_state_source_id: Option<&'static str>,
    pub canonical_adapter_source_id: Option<&'static str>,
    pub alignment_kind: &'static str,
    pub current_state_present: bool,
    pub canonical_adapter_present: bool,
    pub drift_resolved: bool,
    pub live_enforcement_enabled: bool,
    pub rationale: &'static str,
    pub next_alignment_step: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphSourceIdAlignmentReadbackBlocker {
    pub id: &'static str,
    pub severity: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub blocks_live_execution: bool,
    pub recommended_fix: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphSourceIdAlignmentReadbackSideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub work_graph_event_persisted: bool,
    pub source_id_alias_enforced: bool,
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

pub fn hepta_work_graph_source_id_alignment_readback_report()
-> WorkGraphSourceIdAlignmentReadbackReport {
    let current_state_sources = work_graph_current_state_source_surfaces();
    let adapter_index = hepta_work_graph_adapter_task_result_index_report();
    let terminal_readback = hepta_work_graph_terminal_envelope_readback_report();

    let canonical_source_ids = adapter_index
        .source_index
        .iter()
        .map(|entry| entry.source_surface_id)
        .collect::<BTreeSet<_>>();

    let mut alignments = current_state_sources
        .iter()
        .map(|surface| alignment_for_current_source(surface.id, &canonical_source_ids))
        .collect::<Vec<_>>();

    let mut mapped_canonical_source_ids = alignments
        .iter()
        .filter_map(|entry| entry.canonical_adapter_source_id)
        .collect::<BTreeSet<_>>();

    let canonical_shadow_only_source_ids = canonical_source_ids
        .iter()
        .copied()
        .filter(|source_id| {
            !mapped_canonical_source_ids.contains(source_id)
                && canonical_shadow_only_source_ids().contains(source_id)
        })
        .collect::<Vec<_>>();
    for source_id in canonical_shadow_only_source_ids {
        alignments.push(canonical_shadow_only_alignment(source_id));
        mapped_canonical_source_ids.insert(source_id);
    }

    let unresolved_current_state_source_ids = alignments
        .iter()
        .filter(|entry| entry.current_state_present && !entry.drift_resolved)
        .filter_map(|entry| entry.current_state_source_id)
        .collect::<Vec<_>>();
    let unresolved_canonical_adapter_source_ids = canonical_source_ids
        .iter()
        .copied()
        .filter(|source_id| !mapped_canonical_source_ids.contains(source_id))
        .collect::<Vec<_>>();

    let direct_alignment_count = alignments
        .iter()
        .filter(|entry| entry.alignment_kind == "direct_match")
        .count();
    let current_state_alias_alignment_count = alignments
        .iter()
        .filter(|entry| entry.current_state_present && entry.alignment_kind != "direct_match")
        .count();
    let canonical_shadow_only_alignment_count = alignments
        .iter()
        .filter(|entry| entry.alignment_kind == "canonical_shadow_only")
        .count();
    let source_id_alignment_readback_complete = unresolved_current_state_source_ids.is_empty()
        && unresolved_canonical_adapter_source_ids.is_empty()
        && terminal_readback.ready_for_source_id_alignment_readback;
    let blockers = source_id_alignment_readback_blockers(
        &alignments,
        &unresolved_current_state_source_ids,
        &unresolved_canonical_adapter_source_ids,
        terminal_readback.task_result_contract_required_field_gap_count,
        &terminal_readback.terminal_sources,
    );

    WorkGraphSourceIdAlignmentReadbackReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_SOURCE_ID_ALIGNMENT_READBACK_GATE,
        schema_version: WORK_GRAPH_SOURCE_ID_ALIGNMENT_READBACK_SCHEMA_VERSION,
        preview_mode: "read_only_source_id_alignment_readback_no_live_enforcement",
        current_state_source_surface_count: current_state_sources.len(),
        canonical_adapter_source_surface_count: adapter_index
            .canonical_adapter_source_surface_count,
        alignment_entry_count: alignments.len(),
        direct_alignment_count,
        current_state_alias_alignment_count,
        canonical_shadow_only_alignment_count,
        unresolved_current_state_source_count: unresolved_current_state_source_ids.len(),
        unresolved_canonical_adapter_source_count: unresolved_canonical_adapter_source_ids.len(),
        terminal_envelope_readback_consistent_source_count: terminal_readback
            .readback_consistent_source_count,
        task_result_contract_required_field_gap_count: terminal_readback
            .task_result_contract_required_field_gap_count,
        task_result_contract_terminal_field_gap_count: terminal_readback
            .task_result_contract_terminal_field_gap_count,
        alignments,
        unresolved_current_state_source_ids,
        unresolved_canonical_adapter_source_ids,
        blockers,
        required_prior_gates: vec![
            WORK_GRAPH_CURRENT_STATE_INVENTORY_GATE,
            WORK_GRAPH_ADAPTER_TASK_RESULT_INDEX_GATE,
            WORK_GRAPH_TERMINAL_ENVELOPE_READBACK_GATE,
        ],
        recommended_next_gate: WORK_GRAPH_SOURCE_ID_ALIGNMENT_READBACK_RECOMMENDED_NEXT_GATE,
        source_id_alignment_readback_complete,
        ready_for_task_result_contract_field_gap_readback: source_id_alignment_readback_complete,
        ready_for_append_only_event_store_shadow_path: false,
        ready_for_live_execution: false,
        side_effects: WorkGraphSourceIdAlignmentReadbackSideEffects::none(),
    }
}

impl WorkGraphSourceIdAlignmentReadbackSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            graph_state_persisted: false,
            work_graph_event_persisted: false,
            source_id_alias_enforced: false,
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

fn alignment_for_current_source(
    current_source_id: &'static str,
    canonical_source_ids: &BTreeSet<&'static str>,
) -> WorkGraphSourceIdAlignmentReadbackEntry {
    if canonical_source_ids.contains(current_source_id) {
        return alignment(
            Some(current_source_id),
            Some(current_source_id),
            "direct_match",
            true,
            true,
            true,
            "current-state inventory and canonical adapter inventory use the same source id",
            "keep_read_only_alignment_until_task_result_contract_field_gap_readback",
        );
    }

    match current_source_id {
        "plan_mode_proposed_plan" => alignment(
            Some("plan_mode_proposed_plan"),
            Some("plan_mode_proposed_plan_blocks"),
            "renamed_alias",
            true,
            true,
            canonical_source_ids.contains("plan_mode_proposed_plan_blocks"),
            "canonical adapter inventory represents Plan Mode proposed plans as normalized plan block rows",
            "document_alias_before_event_store_shadow_path",
        ),
        "codex_agent_graph_store" => alignment(
            Some("codex_agent_graph_store"),
            Some("multi_agent_v2_thread_spawn"),
            "covered_by_canonical_surface",
            true,
            true,
            canonical_source_ids.contains("multi_agent_v2_thread_spawn"),
            "agent graph store is the backing edge store for multi-agent thread spawn rather than a separate canonical WorkGraph source",
            "keep_backing_store_read_only_until_append_only_event_store_shadow_path",
        ),
        _ => alignment(
            Some(current_source_id),
            None,
            "unresolved_current_state_only",
            true,
            false,
            false,
            "current-state source has no canonical adapter mapping",
            "add_canonical_adapter_source_or_explicit_alias",
        ),
    }
}

fn canonical_shadow_only_alignment(
    canonical_source_id: &'static str,
) -> WorkGraphSourceIdAlignmentReadbackEntry {
    alignment(
        None,
        Some(canonical_source_id),
        "canonical_shadow_only",
        false,
        true,
        true,
        "canonical adapter surface is intentionally represented as shadow/operator-control evidence before live current-state ownership exists",
        "keep_shadow_only_until_operator_control_projection_is_read_back",
    )
}

fn canonical_shadow_only_source_ids() -> BTreeSet<&'static str> {
    BTreeSet::from(["hepta_runtime_approval_broker"])
}

fn source_id_alignment_readback_blockers(
    alignments: &[WorkGraphSourceIdAlignmentReadbackEntry],
    unresolved_current_state_source_ids: &[&'static str],
    unresolved_canonical_adapter_source_ids: &[&'static str],
    task_result_contract_required_field_gap_count: usize,
    terminal_sources: &[crate::work_graph_terminal_envelope_readback::WorkGraphTerminalEnvelopeReadbackSource],
) -> Vec<WorkGraphSourceIdAlignmentReadbackBlocker> {
    let mut blockers = Vec::new();
    if !unresolved_current_state_source_ids.is_empty()
        || !unresolved_canonical_adapter_source_ids.is_empty()
    {
        blockers.push(blocker(
            "source_id_alignment_unresolved",
            "medium",
            unresolved_current_state_source_ids
                .iter()
                .chain(unresolved_canonical_adapter_source_ids.iter())
                .copied()
                .collect(),
            "add a direct adapter source id, a documented alias, or a shadow-only classification before promotion",
        ));
    }

    let canonical_shadow_only_sources = alignments
        .iter()
        .filter(|entry| entry.alignment_kind == "canonical_shadow_only")
        .filter_map(|entry| entry.canonical_adapter_source_id)
        .collect::<Vec<_>>();
    if !canonical_shadow_only_sources.is_empty() {
        blockers.push(blocker(
            "canonical_approval_broker_shadow_only",
            "medium",
            canonical_shadow_only_sources,
            "read back the approval broker as an explicit operator-control current-state surface before enabling enforcement",
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
        "source_id_alignment_live_enforcement_disabled",
        "high",
        alignments
            .iter()
            .filter_map(|entry| entry.canonical_adapter_source_id.or(entry.current_state_source_id))
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect(),
        "keep source-id aliases report-only until TaskResult fields, append-only event-store shadow path, replay, and operator review pass",
    ));

    blockers
}

fn alignment(
    current_state_source_id: Option<&'static str>,
    canonical_adapter_source_id: Option<&'static str>,
    alignment_kind: &'static str,
    current_state_present: bool,
    canonical_adapter_present: bool,
    drift_resolved: bool,
    rationale: &'static str,
    next_alignment_step: &'static str,
) -> WorkGraphSourceIdAlignmentReadbackEntry {
    WorkGraphSourceIdAlignmentReadbackEntry {
        current_state_source_id,
        canonical_adapter_source_id,
        alignment_kind,
        current_state_present,
        canonical_adapter_present,
        drift_resolved,
        live_enforcement_enabled: false,
        rationale,
        next_alignment_step,
    }
}

fn blocker(
    id: &'static str,
    severity: &'static str,
    affected_source_surface_ids: Vec<&'static str>,
    recommended_fix: &'static str,
) -> WorkGraphSourceIdAlignmentReadbackBlocker {
    WorkGraphSourceIdAlignmentReadbackBlocker {
        id,
        severity,
        affected_source_surface_ids,
        blocks_live_execution: true,
        recommended_fix,
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;

    #[test]
    fn source_id_alignment_readback_resolves_known_source_drift() {
        let report = hepta_work_graph_source_id_alignment_readback_report();

        assert_eq!(report.current_state_source_surface_count, 12);
        assert_eq!(report.canonical_adapter_source_surface_count, 12);
        assert_eq!(report.alignment_entry_count, 13);
        assert_eq!(report.direct_alignment_count, 10);
        assert_eq!(report.current_state_alias_alignment_count, 2);
        assert_eq!(report.canonical_shadow_only_alignment_count, 1);
        assert_eq!(report.unresolved_current_state_source_count, 0);
        assert_eq!(report.unresolved_canonical_adapter_source_count, 0);
        assert!(report.unresolved_current_state_source_ids.is_empty());
        assert!(report.unresolved_canonical_adapter_source_ids.is_empty());
        assert!(report.source_id_alignment_readback_complete);
    }

    #[test]
    fn source_id_alignment_readback_documents_alias_and_shadow_rows() {
        let report = hepta_work_graph_source_id_alignment_readback_report();
        let by_current = report
            .alignments
            .iter()
            .filter_map(|entry| entry.current_state_source_id.map(|id| (id, entry)))
            .collect::<BTreeMap<_, _>>();
        let shadow_entries = report
            .alignments
            .iter()
            .filter(|entry| entry.alignment_kind == "canonical_shadow_only")
            .collect::<Vec<_>>();

        assert_eq!(
            by_current["plan_mode_proposed_plan"].canonical_adapter_source_id,
            Some("plan_mode_proposed_plan_blocks")
        );
        assert_eq!(
            by_current["plan_mode_proposed_plan"].alignment_kind,
            "renamed_alias"
        );
        assert_eq!(
            by_current["codex_agent_graph_store"].canonical_adapter_source_id,
            Some("multi_agent_v2_thread_spawn")
        );
        assert_eq!(
            by_current["codex_agent_graph_store"].alignment_kind,
            "covered_by_canonical_surface"
        );
        assert_eq!(shadow_entries.len(), 1);
        assert_eq!(
            shadow_entries[0].canonical_adapter_source_id,
            Some("hepta_runtime_approval_broker")
        );
    }

    #[test]
    fn source_id_alignment_readback_keeps_live_blockers_visible() {
        let report = hepta_work_graph_source_id_alignment_readback_report();
        let blocker_ids = report
            .blockers
            .iter()
            .map(|blocker| blocker.id)
            .collect::<Vec<_>>();

        assert_eq!(report.terminal_envelope_readback_consistent_source_count, 6);
        assert_eq!(report.task_result_contract_required_field_gap_count, 0);
        assert_eq!(report.task_result_contract_terminal_field_gap_count, 0);
        assert!(!blocker_ids.contains(&"source_id_alignment_unresolved"));
        assert!(blocker_ids.contains(&"canonical_approval_broker_shadow_only"));
        assert!(!blocker_ids.contains(&"task_result_contract_required_fields_partial"));
        assert!(blocker_ids.contains(&"source_id_alignment_live_enforcement_disabled"));
        assert!(report.ready_for_task_result_contract_field_gap_readback);
        assert!(!report.ready_for_append_only_event_store_shadow_path);
        assert!(!report.ready_for_live_execution);
    }

    #[test]
    fn source_id_alignment_readback_is_non_mutating() {
        let report = hepta_work_graph_source_id_alignment_readback_report();

        assert_eq!(
            report.required_prior_gates,
            [
                WORK_GRAPH_CURRENT_STATE_INVENTORY_GATE,
                WORK_GRAPH_ADAPTER_TASK_RESULT_INDEX_GATE,
                WORK_GRAPH_TERMINAL_ENVELOPE_READBACK_GATE,
            ]
        );
        assert_eq!(
            report.recommended_next_gate,
            WORK_GRAPH_SOURCE_ID_ALIGNMENT_READBACK_RECOMMENDED_NEXT_GATE
        );
        assert!(report.alignments.iter().all(|entry| {
            entry.drift_resolved
                && !entry.live_enforcement_enabled
                && (entry.current_state_present || entry.canonical_adapter_present)
        }));
        assert_eq!(
            report.side_effects,
            WorkGraphSourceIdAlignmentReadbackSideEffects::none()
        );
    }
}

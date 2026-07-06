use std::collections::{BTreeMap, BTreeSet};

use serde::Serialize;

use crate::work_graph_canonical_adapter_inventory_preview::{
    WORK_GRAPH_CANONICAL_ADAPTER_INVENTORY_PREVIEW_GATE,
    hepta_work_graph_canonical_adapter_inventory_preview_report,
};
use crate::work_graph_current_state_inventory::{
    WORK_GRAPH_CURRENT_STATE_INVENTORY_GATE, WorkGraphP0GapInventory,
    work_graph_current_state_p0_gaps, work_graph_current_state_source_surfaces,
};
use crate::work_graph_task_result_contract::{
    WORK_GRAPH_TASK_RESULT_CONTRACT_PREVIEW_GATE, work_graph_task_result_adapter_previews,
    work_graph_task_result_required_fields,
};
use crate::work_graph_task_result_envelope_report_only_validator::{
    WORK_GRAPH_TASK_RESULT_ENVELOPE_REPORT_ONLY_VALIDATOR_GATE,
    hepta_work_graph_task_result_envelope_report_only_validator_report,
};

pub const WORK_GRAPH_ADAPTER_TASK_RESULT_INDEX_GATE: &str =
    "hepta_work_graph_adapter_task_result_index_gate";
pub const WORK_GRAPH_ADAPTER_TASK_RESULT_INDEX_SCHEMA_VERSION: &str =
    "work_graph_adapter_task_result_index_v1";
pub const WORK_GRAPH_ADAPTER_TASK_RESULT_INDEX_RECOMMENDED_NEXT_GATE: &str =
    "hepta_work_graph_terminal_envelope_readback_gate";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAdapterTaskResultIndexReport {
    pub product: &'static str,
    pub runtime: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub preview_mode: &'static str,
    pub current_state_source_surface_count: usize,
    pub current_state_p0_gap_count: usize,
    pub canonical_adapter_source_surface_count: usize,
    pub canonical_adapter_count: usize,
    pub task_result_contract_required_field_count: usize,
    pub task_result_envelope_source_adapter_count: usize,
    pub task_result_envelope_source_count: usize,
    pub indexed_source_count: usize,
    pub scheduler_entrypoint_source_count: usize,
    pub scheduler_entrypoint_ready_count: usize,
    pub terminal_task_result_required_count: usize,
    pub terminal_task_result_full_envelope_source_count: usize,
    pub missing_envelope_adapter_count: usize,
    pub missing_envelope_preview_count: usize,
    pub contract_required_field_gap_count: usize,
    pub contract_terminal_field_gap_count: usize,
    pub current_state_only_source_ids: Vec<&'static str>,
    pub canonical_adapter_only_source_ids: Vec<&'static str>,
    pub source_index: Vec<WorkGraphAdapterTaskResultIndexEntry>,
    pub blockers: Vec<WorkGraphAdapterTaskResultIndexBlocker>,
    pub required_prior_gates: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub report_only_index_attached: bool,
    pub live_enforcement_enabled: bool,
    pub ready_for_scheduler_admission_dry_run_enforcement: bool,
    pub ready_for_live_execution: bool,
    pub side_effects: WorkGraphAdapterTaskResultIndexSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAdapterTaskResultIndexEntry {
    pub source_surface_id: &'static str,
    pub source_category: &'static str,
    pub canonical_node_kind: &'static str,
    pub scheduler_entrypoint_source: bool,
    pub terminal_task_result_required: bool,
    pub canonical_inventory_state: &'static str,
    pub task_result_contract_adapter_state: &'static str,
    pub task_result_envelope_adapter_state: &'static str,
    pub task_result_envelope_preview_state: &'static str,
    pub covered_contract_wire_fields: Vec<&'static str>,
    pub covered_envelope_wire_fields: Vec<&'static str>,
    pub missing_contract_required_wire_fields: Vec<&'static str>,
    pub missing_contract_terminal_wire_fields: Vec<&'static str>,
    pub missing_envelope_wire_fields: Vec<&'static str>,
    pub inherited_blocker_ids: Vec<&'static str>,
    pub report_only_index_decision: &'static str,
    pub live_enforcement_enabled: bool,
    pub next_index_step: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkGraphAdapterTaskResultIndexBlocker {
    pub id: &'static str,
    pub severity: &'static str,
    pub affected_source_surface_ids: Vec<&'static str>,
    pub blocks_live_execution: bool,
    pub recommended_fix: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct WorkGraphAdapterTaskResultIndexSideEffects {
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub work_graph_event_persisted: bool,
    pub adapter_projection_enforced: bool,
    pub task_result_enforcement_enabled: bool,
    pub scheduler_admission_enforced: bool,
    pub live_admission_enforcement_enabled: bool,
    pub runtime_mutation_performed: bool,
    pub agent_spawn_performed: bool,
    pub external_send_performed: bool,
    pub model_invoked: bool,
}

pub fn hepta_work_graph_adapter_task_result_index_report() -> WorkGraphAdapterTaskResultIndexReport
{
    let current_state_sources = work_graph_current_state_source_surfaces();
    let current_state_p0_gaps = work_graph_current_state_p0_gaps();
    let canonical_inventory = hepta_work_graph_canonical_adapter_inventory_preview_report();
    let task_result_contract_adapters = work_graph_task_result_adapter_previews();
    let task_result_envelope = hepta_work_graph_task_result_envelope_report_only_validator_report();

    let task_result_fields = work_graph_task_result_required_fields();
    let required_wire_fields = task_result_fields
        .iter()
        .filter(|field| field.required)
        .map(|field| field.wire_name)
        .collect::<Vec<_>>();
    let terminal_wire_fields = task_result_fields
        .iter()
        .filter(|field| field.terminal_required)
        .map(|field| field.wire_name)
        .collect::<Vec<_>>();

    let contract_adapter_by_source = task_result_contract_adapters
        .iter()
        .map(|adapter| (adapter.source_surface_id, adapter))
        .collect::<BTreeMap<_, _>>();
    let envelope_adapter_by_source = task_result_envelope
        .source_adapters
        .iter()
        .map(|adapter| (adapter.source_surface_id, adapter))
        .collect::<BTreeMap<_, _>>();
    let envelope_preview_sources = task_result_envelope
        .source_envelopes
        .iter()
        .map(|envelope| envelope.source_surface_id)
        .collect::<BTreeSet<_>>();
    let scheduler_entrypoint_sources = scheduler_entrypoint_source_ids()
        .into_iter()
        .collect::<BTreeSet<_>>();

    let source_index = canonical_inventory
        .adapters
        .iter()
        .map(|adapter| {
            let contract_adapter = contract_adapter_by_source.get(adapter.source_surface_id);
            let envelope_adapter = envelope_adapter_by_source.get(adapter.source_surface_id);
            let covered_contract_wire_fields = contract_adapter
                .map(|adapter| adapter.covered_wire_fields.clone())
                .unwrap_or_default();
            let covered_envelope_wire_fields = envelope_adapter
                .map(|adapter| adapter.covered_wire_fields.clone())
                .unwrap_or_default();
            let missing_contract_required_wire_fields = missing_fields_for_required_source(
                adapter.terminal_task_result_required,
                &required_wire_fields,
                &covered_contract_wire_fields,
            );
            let missing_contract_terminal_wire_fields = missing_fields_for_required_source(
                adapter.terminal_task_result_required,
                &terminal_wire_fields,
                &covered_contract_wire_fields,
            );
            let missing_envelope_wire_fields = missing_fields_for_required_source(
                adapter.terminal_task_result_required,
                &required_wire_fields,
                &covered_envelope_wire_fields,
            );
            let scheduler_entrypoint_source =
                scheduler_entrypoint_sources.contains(adapter.source_surface_id);
            let envelope_adapter_present = envelope_adapter.is_some();
            let envelope_preview_present =
                envelope_preview_sources.contains(adapter.source_surface_id);
            let envelope_adapter_state = envelope_adapter_state(
                adapter.terminal_task_result_required,
                envelope_adapter_present,
            );
            let envelope_preview_state = envelope_preview_state(
                adapter.terminal_task_result_required,
                envelope_preview_present,
            );
            let decision = report_only_index_decision(
                scheduler_entrypoint_source,
                adapter.terminal_task_result_required,
                envelope_adapter_present,
                envelope_preview_present,
                missing_envelope_wire_fields.is_empty(),
            );
            let next_index_step = next_index_step(
                scheduler_entrypoint_source,
                adapter.terminal_task_result_required,
                envelope_adapter_present,
                missing_contract_required_wire_fields.is_empty(),
                adapter.next_inventory_step,
            );

            WorkGraphAdapterTaskResultIndexEntry {
                source_surface_id: adapter.source_surface_id,
                source_category: adapter.source_category,
                canonical_node_kind: adapter.canonical_node_kind,
                scheduler_entrypoint_source,
                terminal_task_result_required: adapter.terminal_task_result_required,
                canonical_inventory_state: adapter.canonical_inventory_state,
                task_result_contract_adapter_state: adapter.task_result_adapter_state,
                task_result_envelope_adapter_state: envelope_adapter_state,
                task_result_envelope_preview_state: envelope_preview_state,
                covered_contract_wire_fields,
                covered_envelope_wire_fields,
                missing_contract_required_wire_fields,
                missing_contract_terminal_wire_fields,
                missing_envelope_wire_fields,
                inherited_blocker_ids: adapter.inventory_blocker_ids.clone(),
                report_only_index_decision: decision,
                live_enforcement_enabled: false,
                next_index_step,
            }
        })
        .collect::<Vec<_>>();

    let current_state_ids = current_state_sources
        .iter()
        .map(|surface| surface.id)
        .collect::<BTreeSet<_>>();
    let canonical_source_ids = canonical_inventory
        .adapters
        .iter()
        .map(|adapter| adapter.source_surface_id)
        .collect::<BTreeSet<_>>();
    let current_state_only_source_ids = current_state_ids
        .difference(&canonical_source_ids)
        .copied()
        .collect::<Vec<_>>();
    let canonical_adapter_only_source_ids = canonical_source_ids
        .difference(&current_state_ids)
        .copied()
        .collect::<Vec<_>>();

    let scheduler_entrypoint_source_count = scheduler_entrypoint_sources.len();
    let scheduler_entrypoint_ready_count = source_index
        .iter()
        .filter(|entry| {
            entry.scheduler_entrypoint_source
                && entry.task_result_envelope_adapter_state == "present_report_only"
                && entry.task_result_envelope_preview_state == "present_report_only"
        })
        .count();
    let terminal_task_result_full_envelope_source_count = source_index
        .iter()
        .filter(|entry| {
            entry.terminal_task_result_required
                && entry.task_result_envelope_adapter_state == "present_report_only"
                && entry.missing_envelope_wire_fields.is_empty()
        })
        .count();
    let missing_envelope_adapter_count = source_index
        .iter()
        .filter(|entry| {
            entry.terminal_task_result_required
                && entry.task_result_envelope_adapter_state == "missing"
        })
        .count();
    let missing_envelope_preview_count = source_index
        .iter()
        .filter(|entry| {
            entry.terminal_task_result_required
                && entry.task_result_envelope_preview_state == "missing"
        })
        .count();
    let contract_required_field_gap_count = source_index
        .iter()
        .map(|entry| entry.missing_contract_required_wire_fields.len())
        .sum();
    let contract_terminal_field_gap_count = source_index
        .iter()
        .map(|entry| entry.missing_contract_terminal_wire_fields.len())
        .sum();
    let blockers = adapter_task_result_index_blockers(
        &current_state_p0_gaps,
        &current_state_only_source_ids,
        &canonical_adapter_only_source_ids,
        &source_index,
    );
    let ready_for_scheduler_admission_dry_run_enforcement =
        scheduler_entrypoint_ready_count == scheduler_entrypoint_source_count;

    WorkGraphAdapterTaskResultIndexReport {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        gate: WORK_GRAPH_ADAPTER_TASK_RESULT_INDEX_GATE,
        schema_version: WORK_GRAPH_ADAPTER_TASK_RESULT_INDEX_SCHEMA_VERSION,
        preview_mode: "read_only_adapter_task_result_index_no_live_enforcement",
        current_state_source_surface_count: current_state_sources.len(),
        current_state_p0_gap_count: current_state_p0_gaps.len(),
        canonical_adapter_source_surface_count: canonical_inventory.source_surface_count,
        canonical_adapter_count: canonical_inventory.canonical_adapter_count,
        task_result_contract_required_field_count: required_wire_fields.len(),
        task_result_envelope_source_adapter_count: task_result_envelope.source_adapter_count,
        task_result_envelope_source_count: task_result_envelope.source_envelope_count,
        indexed_source_count: source_index.len(),
        scheduler_entrypoint_source_count,
        scheduler_entrypoint_ready_count,
        terminal_task_result_required_count: canonical_inventory
            .terminal_task_result_required_count,
        terminal_task_result_full_envelope_source_count,
        missing_envelope_adapter_count,
        missing_envelope_preview_count,
        contract_required_field_gap_count,
        contract_terminal_field_gap_count,
        current_state_only_source_ids,
        canonical_adapter_only_source_ids,
        source_index,
        blockers,
        required_prior_gates: vec![
            WORK_GRAPH_CURRENT_STATE_INVENTORY_GATE,
            WORK_GRAPH_CANONICAL_ADAPTER_INVENTORY_PREVIEW_GATE,
            WORK_GRAPH_TASK_RESULT_CONTRACT_PREVIEW_GATE,
            WORK_GRAPH_TASK_RESULT_ENVELOPE_REPORT_ONLY_VALIDATOR_GATE,
        ],
        recommended_next_gate: WORK_GRAPH_ADAPTER_TASK_RESULT_INDEX_RECOMMENDED_NEXT_GATE,
        report_only_index_attached: true,
        live_enforcement_enabled: false,
        ready_for_scheduler_admission_dry_run_enforcement,
        ready_for_live_execution: false,
        side_effects: WorkGraphAdapterTaskResultIndexSideEffects::none(),
    }
}

impl WorkGraphAdapterTaskResultIndexSideEffects {
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
            agent_spawn_performed: false,
            external_send_performed: false,
            model_invoked: false,
        }
    }
}

fn adapter_task_result_index_blockers(
    current_state_p0_gaps: &[WorkGraphP0GapInventory],
    current_state_only_source_ids: &[&'static str],
    canonical_adapter_only_source_ids: &[&'static str],
    source_index: &[WorkGraphAdapterTaskResultIndexEntry],
) -> Vec<WorkGraphAdapterTaskResultIndexBlocker> {
    let mut blockers = current_state_p0_gaps
        .iter()
        .map(|gap| {
            blocker(
                gap.id,
                gap.severity,
                gap.source_surface_ids.clone(),
                gap.recommended_next_action,
            )
        })
        .collect::<Vec<_>>();

    let source_id_alignment_drift = current_state_only_source_ids
        .iter()
        .chain(canonical_adapter_only_source_ids.iter())
        .copied()
        .collect::<Vec<_>>();
    if !source_id_alignment_drift.is_empty() {
        blockers.push(blocker(
            "source_surface_id_alignment_drift",
            "medium",
            source_id_alignment_drift,
            "normalize current-state and canonical adapter source ids before using the index as an enforced fact source",
        ));
    }

    let missing_terminal_envelope_sources = source_index
        .iter()
        .filter(|entry| {
            entry.terminal_task_result_required
                && entry.task_result_envelope_adapter_state == "missing"
        })
        .map(|entry| entry.source_surface_id)
        .collect::<Vec<_>>();
    if !missing_terminal_envelope_sources.is_empty() {
        blockers.push(blocker(
            "canonical_terminal_envelope_sources_missing",
            "high",
            missing_terminal_envelope_sources,
            "add report-only TaskResultEnvelope adapters for every canonical terminal source before live promotion",
        ));
    }

    let contract_field_gap_sources = source_index
        .iter()
        .filter(|entry| {
            entry.terminal_task_result_required
                && !entry.missing_contract_required_wire_fields.is_empty()
        })
        .map(|entry| entry.source_surface_id)
        .collect::<Vec<_>>();
    if !contract_field_gap_sources.is_empty() {
        blockers.push(blocker(
            "task_result_contract_required_fields_partial",
            "medium",
            contract_field_gap_sources,
            "fill missing TaskResult contract field projections before enabling validator enforcement",
        ));
    }

    let envelope_sources = source_index
        .iter()
        .filter(|entry| entry.task_result_envelope_adapter_state == "present_report_only")
        .map(|entry| entry.source_surface_id)
        .collect::<Vec<_>>();
    blockers.push(blocker(
        "task_result_envelope_live_enforcement_disabled",
        "high",
        envelope_sources,
        "keep envelope validation report-only until scheduler dry-run, shadow event store, replay, and operator review pass",
    ));

    blockers
}

fn scheduler_entrypoint_source_ids() -> Vec<&'static str> {
    vec![
        "multi_agent_v2_thread_spawn",
        "agent_jobs_batch_workers",
        "hepta_runtime_task_board",
        "hepta_runtime_worker_tasks",
    ]
}

fn missing_fields_for_required_source(
    required_source: bool,
    required_fields: &[&'static str],
    covered_fields: &[&'static str],
) -> Vec<&'static str> {
    if !required_source {
        return Vec::new();
    }
    let covered_fields = covered_fields.iter().copied().collect::<BTreeSet<_>>();
    required_fields
        .iter()
        .copied()
        .filter(|field| !covered_fields.contains(field))
        .collect()
}

fn envelope_adapter_state(required: bool, present: bool) -> &'static str {
    if present {
        "present_report_only"
    } else if required {
        "missing"
    } else {
        "not_required"
    }
}

fn envelope_preview_state(required: bool, present: bool) -> &'static str {
    if present {
        "present_report_only"
    } else if required {
        "missing"
    } else {
        "not_required"
    }
}

fn report_only_index_decision(
    scheduler_entrypoint_source: bool,
    terminal_task_result_required: bool,
    envelope_adapter_present: bool,
    envelope_preview_present: bool,
    envelope_wire_fields_complete: bool,
) -> &'static str {
    if scheduler_entrypoint_source && envelope_adapter_present && envelope_preview_present {
        "allow_scheduler_dry_run_report_only"
    } else if terminal_task_result_required && !envelope_adapter_present {
        "block_live_terminal_promotion_missing_envelope"
    } else if terminal_task_result_required && !envelope_wire_fields_complete {
        "block_live_terminal_promotion_partial_envelope"
    } else if envelope_adapter_present {
        "auxiliary_envelope_preview"
    } else {
        "no_terminal_result_required"
    }
}

fn next_index_step(
    scheduler_entrypoint_source: bool,
    terminal_task_result_required: bool,
    envelope_adapter_present: bool,
    contract_required_fields_complete: bool,
    fallback_next_step: &'static str,
) -> &'static str {
    if scheduler_entrypoint_source && envelope_adapter_present {
        "use_scheduler_admission_dry_run_explanation"
    } else if terminal_task_result_required && !envelope_adapter_present {
        "add_task_result_envelope_adapter_preview"
    } else if terminal_task_result_required && !contract_required_fields_complete {
        "fill_task_result_contract_wire_field_projection"
    } else if envelope_adapter_present {
        "keep_report_only_until_append_only_event_store_shadow_path"
    } else {
        fallback_next_step
    }
}

fn blocker(
    id: &'static str,
    severity: &'static str,
    affected_source_surface_ids: Vec<&'static str>,
    recommended_fix: &'static str,
) -> WorkGraphAdapterTaskResultIndexBlocker {
    WorkGraphAdapterTaskResultIndexBlocker {
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
    fn adapter_task_result_index_covers_canonical_surfaces() {
        let report = hepta_work_graph_adapter_task_result_index_report();

        assert_eq!(report.current_state_source_surface_count, 12);
        assert_eq!(report.current_state_p0_gap_count, 5);
        assert_eq!(report.canonical_adapter_source_surface_count, 12);
        assert_eq!(report.canonical_adapter_count, 12);
        assert_eq!(report.indexed_source_count, 12);
        assert_eq!(report.task_result_contract_required_field_count, 11);
        assert_eq!(report.task_result_envelope_source_adapter_count, 7);
        assert_eq!(report.task_result_envelope_source_count, 7);
        assert_eq!(report.terminal_task_result_required_count, 6);
        assert_eq!(report.terminal_task_result_full_envelope_source_count, 6);
    }

    #[test]
    fn adapter_task_result_index_marks_scheduler_sources_ready_for_dry_run() {
        let report = hepta_work_graph_adapter_task_result_index_report();
        let entries_by_id = report
            .source_index
            .iter()
            .map(|entry| (entry.source_surface_id, entry))
            .collect::<BTreeMap<_, _>>();

        assert_eq!(report.scheduler_entrypoint_source_count, 4);
        assert_eq!(report.scheduler_entrypoint_ready_count, 4);
        assert!(report.ready_for_scheduler_admission_dry_run_enforcement);
        for source in [
            "multi_agent_v2_thread_spawn",
            "agent_jobs_batch_workers",
            "hepta_runtime_task_board",
            "hepta_runtime_worker_tasks",
        ] {
            let entry = entries_by_id[source];
            assert_eq!(
                entry.report_only_index_decision,
                "allow_scheduler_dry_run_report_only"
            );
            assert_eq!(
                entry.task_result_envelope_adapter_state,
                "present_report_only"
            );
            assert!(!entry.live_enforcement_enabled);
        }
    }

    #[test]
    fn adapter_task_result_index_keeps_live_gaps_visible() {
        let report = hepta_work_graph_adapter_task_result_index_report();
        let blocker_ids = report
            .blockers
            .iter()
            .map(|blocker| blocker.id)
            .collect::<Vec<_>>();

        assert_eq!(
            report.current_state_only_source_ids,
            ["codex_agent_graph_store", "plan_mode_proposed_plan"]
        );
        assert_eq!(
            report.canonical_adapter_only_source_ids,
            [
                "hepta_runtime_approval_broker",
                "plan_mode_proposed_plan_blocks"
            ]
        );
        assert_eq!(report.missing_envelope_adapter_count, 0);
        assert_eq!(report.missing_envelope_preview_count, 0);
        assert_eq!(report.contract_required_field_gap_count, 0);
        assert_eq!(report.contract_terminal_field_gap_count, 0);
        assert!(blocker_ids.contains(&"source_surface_id_alignment_drift"));
        assert!(!blocker_ids.contains(&"canonical_terminal_envelope_sources_missing"));
        assert!(!blocker_ids.contains(&"task_result_contract_required_fields_partial"));
        assert!(blocker_ids.contains(&"task_result_envelope_live_enforcement_disabled"));
    }

    #[test]
    fn adapter_task_result_index_is_report_only() {
        let report = hepta_work_graph_adapter_task_result_index_report();

        assert_eq!(
            report.required_prior_gates,
            [
                WORK_GRAPH_CURRENT_STATE_INVENTORY_GATE,
                WORK_GRAPH_CANONICAL_ADAPTER_INVENTORY_PREVIEW_GATE,
                WORK_GRAPH_TASK_RESULT_CONTRACT_PREVIEW_GATE,
                WORK_GRAPH_TASK_RESULT_ENVELOPE_REPORT_ONLY_VALIDATOR_GATE,
            ]
        );
        assert_eq!(
            report.recommended_next_gate,
            WORK_GRAPH_ADAPTER_TASK_RESULT_INDEX_RECOMMENDED_NEXT_GATE
        );
        assert!(report.report_only_index_attached);
        assert!(!report.live_enforcement_enabled);
        assert!(!report.ready_for_live_execution);
        assert_eq!(
            report.side_effects,
            WorkGraphAdapterTaskResultIndexSideEffects::none()
        );
    }
}

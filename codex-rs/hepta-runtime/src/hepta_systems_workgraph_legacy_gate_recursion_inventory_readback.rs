use crate::hepta_systems_gate_recursion_lean_contract_readback::hepta_systems_gate_recursion_lean_contract_readback_report;
use serde::Serialize;

pub const HEPTA_SYSTEMS_WORKGRAPH_LEGACY_GATE_RECURSION_INVENTORY_READBACK_GATE: &str =
    "hepta_systems_workgraph_legacy_gate_recursion_inventory_readback_gate";
pub const HEPTA_SYSTEMS_WORKGRAPH_LEGACY_GATE_RECURSION_INVENTORY_READBACK_SCHEMA_VERSION: &str =
    "hepta_systems_workgraph_legacy_gate_recursion_inventory_readback_v1";
pub const HEPTA_SYSTEMS_WORKGRAPH_LEGACY_GATE_RECURSION_INVENTORY_READBACK_RECOMMENDED_NEXT_GATE:
    &str = "hepta_systems_tool_registry_minimal_read_only_invocation_ledger_receipt_readback";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HeptaSystemsWorkgraphLegacyGateRecursionInventoryReadbackReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub source_lean_contract_ready: bool,
    pub source_legacy_inventory_target_count: usize,
    pub inventory_scope: &'static str,
    pub inventory_entry_count: usize,
    pub route_declared_count: usize,
    pub legacy_full_upstream_gate_chain_count: usize,
    pub full_upstream_gate_chain_allowed_count: usize,
    pub source_report_smoke_migration_target_count: usize,
    pub targeted_rust_test_contract_target_count: usize,
    pub terminal_no_cutover_receipt_chain_count: usize,
    pub replay_idempotency_chain_count: usize,
    pub closeout_chain_count: usize,
    pub runtime_write_boundary_chain_count: usize,
    pub operator_review_packet_chain_count: usize,
    pub required_prior_gate_count_total: usize,
    pub required_prior_gate_count_max: usize,
    pub source_report_semantics_change_allowed: bool,
    pub matrix_cache_write_allowed: bool,
    pub compact_cache_persistence_allowed: bool,
    pub workflow_execution_allowed: bool,
    pub replay_execution_allowed: bool,
    pub event_log_write_allowed: bool,
    pub sqlite_write_allowed: bool,
    pub live_execution_allowed: bool,
    pub workgraph_legacy_gate_recursion_inventory_ready: bool,
    pub entries: Vec<HeptaSystemsWorkgraphLegacyGateRecursionInventoryReadbackEntry>,
    pub blockers: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub side_effects: HeptaSystemsWorkgraphLegacyGateRecursionInventoryReadbackSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HeptaSystemsWorkgraphLegacyGateRecursionInventoryReadbackEntry {
    pub entry_id: &'static str,
    pub gate_family: &'static str,
    pub route: &'static str,
    pub required_prior_gate_count: usize,
    pub observed_risk: &'static str,
    pub projected_contract: &'static str,
    pub projected_in_memory: bool,
    pub route_declared: bool,
    pub legacy_full_upstream_gate_chain_present: bool,
    pub migrate_to_source_report_smoke: bool,
    pub targeted_rust_test_contract_required: bool,
    pub full_upstream_gate_chain_allowed: bool,
    pub source_report_semantics_changed: bool,
    pub matrix_cache_written: bool,
    pub compact_cache_persisted: bool,
    pub workflow_execution_started: bool,
    pub replay_executed: bool,
    pub event_log_written: bool,
    pub sqlite_written: bool,
    pub live_execution_started: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct HeptaSystemsWorkgraphLegacyGateRecursionInventoryReadbackSideEffects {
    pub filesystem_written: bool,
    pub source_report_semantics_changed: bool,
    pub full_upstream_gate_chain_invoked: bool,
    pub cargo_test_executed_by_report: bool,
    pub matrix_cache_written: bool,
    pub compact_cache_persisted: bool,
    pub workflow_execution_started: bool,
    pub replay_executed: bool,
    pub event_log_written: bool,
    pub sqlite_written: bool,
    pub provider_invoked: bool,
    pub model_invoked: bool,
    pub gateway_or_auth_mutated: bool,
    pub native_post_mutation_performed: bool,
    pub channel_send_performed: bool,
    pub package_or_release_written: bool,
    pub public_ga_promoted: bool,
    pub live_execution_started: bool,
}

pub fn hepta_systems_workgraph_legacy_gate_recursion_inventory_readback_report()
-> HeptaSystemsWorkgraphLegacyGateRecursionInventoryReadbackReport {
    let source = hepta_systems_gate_recursion_lean_contract_readback_report();
    let entries = hepta_systems_workgraph_legacy_gate_recursion_inventory_readback_entries();
    let inventory_entry_count = entries.len();
    let route_declared_count = entries.iter().filter(|entry| entry.route_declared).count();
    let legacy_full_upstream_gate_chain_count = entries
        .iter()
        .filter(|entry| entry.legacy_full_upstream_gate_chain_present)
        .count();
    let full_upstream_gate_chain_allowed_count = entries
        .iter()
        .filter(|entry| entry.full_upstream_gate_chain_allowed)
        .count();
    let source_report_smoke_migration_target_count = entries
        .iter()
        .filter(|entry| entry.migrate_to_source_report_smoke)
        .count();
    let targeted_rust_test_contract_target_count = entries
        .iter()
        .filter(|entry| entry.targeted_rust_test_contract_required)
        .count();
    let terminal_no_cutover_receipt_chain_count = entries
        .iter()
        .filter(|entry| entry.gate_family == "terminal_no_cutover_receipt_chain")
        .count();
    let replay_idempotency_chain_count = entries
        .iter()
        .filter(|entry| entry.gate_family == "replay_idempotency_chain")
        .count();
    let closeout_chain_count = entries
        .iter()
        .filter(|entry| entry.gate_family == "closeout_receipt_chain")
        .count();
    let runtime_write_boundary_chain_count = entries
        .iter()
        .filter(|entry| entry.gate_family == "runtime_write_boundary_chain")
        .count();
    let operator_review_packet_chain_count = entries
        .iter()
        .filter(|entry| entry.gate_family == "operator_review_packet_chain")
        .count();
    let required_prior_gate_count_total = entries
        .iter()
        .map(|entry| entry.required_prior_gate_count)
        .sum();
    let required_prior_gate_count_max = entries
        .iter()
        .map(|entry| entry.required_prior_gate_count)
        .max()
        .unwrap_or_default();

    let workgraph_legacy_gate_recursion_inventory_ready = source.lean_contract_readback_ready
        && source.legacy_recursion_inventory_count == 2
        && inventory_entry_count == 8
        && route_declared_count == 8
        && legacy_full_upstream_gate_chain_count == 8
        && full_upstream_gate_chain_allowed_count == 0
        && source_report_smoke_migration_target_count == 8
        && targeted_rust_test_contract_target_count == 8
        && terminal_no_cutover_receipt_chain_count == 1
        && replay_idempotency_chain_count == 1
        && closeout_chain_count == 2
        && runtime_write_boundary_chain_count == 3
        && operator_review_packet_chain_count == 1
        && required_prior_gate_count_total == 663
        && required_prior_gate_count_max == 116
        && entries.iter().all(|entry| {
            entry.projected_in_memory
                && entry.route_declared
                && entry.legacy_full_upstream_gate_chain_present
                && entry.migrate_to_source_report_smoke
                && entry.targeted_rust_test_contract_required
                && !entry.full_upstream_gate_chain_allowed
                && !entry.source_report_semantics_changed
                && !entry.matrix_cache_written
                && !entry.compact_cache_persisted
                && !entry.workflow_execution_started
                && !entry.replay_executed
                && !entry.event_log_written
                && !entry.sqlite_written
                && !entry.live_execution_started
        });

    HeptaSystemsWorkgraphLegacyGateRecursionInventoryReadbackReport {
        runtime: "hepta",
        surface: "hepta_systems_workgraph_legacy_gate_recursion_inventory_readback",
        status: if workgraph_legacy_gate_recursion_inventory_ready {
            "ready_blocked"
        } else {
            "blocked"
        },
        gate: HEPTA_SYSTEMS_WORKGRAPH_LEGACY_GATE_RECURSION_INVENTORY_READBACK_GATE,
        schema_version:
            HEPTA_SYSTEMS_WORKGRAPH_LEGACY_GATE_RECURSION_INVENTORY_READBACK_SCHEMA_VERSION,
        source_lean_contract_ready: source.lean_contract_readback_ready,
        source_legacy_inventory_target_count: source.legacy_recursion_inventory_count,
        inventory_scope: "legacy_workgraph_rerun_preview_gate_chains_with_required_prior_gate_count",
        inventory_entry_count,
        route_declared_count,
        legacy_full_upstream_gate_chain_count,
        full_upstream_gate_chain_allowed_count,
        source_report_smoke_migration_target_count,
        targeted_rust_test_contract_target_count,
        terminal_no_cutover_receipt_chain_count,
        replay_idempotency_chain_count,
        closeout_chain_count,
        runtime_write_boundary_chain_count,
        operator_review_packet_chain_count,
        required_prior_gate_count_total,
        required_prior_gate_count_max,
        source_report_semantics_change_allowed: false,
        matrix_cache_write_allowed: false,
        compact_cache_persistence_allowed: false,
        workflow_execution_allowed: false,
        replay_execution_allowed: false,
        event_log_write_allowed: false,
        sqlite_write_allowed: false,
        live_execution_allowed: false,
        workgraph_legacy_gate_recursion_inventory_ready,
        entries,
        blockers: vec![
            "legacy_workgraph_full_upstream_gate_chain_inventory_required",
            "source_report_smoke_migration_required",
            "targeted_rust_test_contract_required",
            "source_report_semantics_change_disabled",
            "matrix_cache_write_disabled",
            "compact_cache_persistence_disabled",
            "workflow_execution_disabled",
            "replay_execution_disabled",
            "event_log_write_disabled",
            "sqlite_write_disabled",
            "live_execution_disabled",
        ],
        recommended_next_gate:
            HEPTA_SYSTEMS_WORKGRAPH_LEGACY_GATE_RECURSION_INVENTORY_READBACK_RECOMMENDED_NEXT_GATE,
        side_effects: HeptaSystemsWorkgraphLegacyGateRecursionInventoryReadbackSideEffects::none(),
    }
}

pub fn hepta_systems_workgraph_legacy_gate_recursion_inventory_readback_entries()
-> Vec<HeptaSystemsWorkgraphLegacyGateRecursionInventoryReadbackEntry> {
    vec![
        inventory_entry(
            "terminal_no_cutover_receipt_acknowledgement_rerun_preview",
            "terminal_no_cutover_receipt_chain",
            "scripts/hepta-systems-work-graph-unified-projection-enforcement-readiness-work-graph-events-event-store-cutover-terminal-no-cutover-receipt-acknowledgement-rerun-preview-gate.sh",
            96,
            "rerun preview gate carries a 96-step prior-gate ladder before the next receipt acknowledgement suffix",
        ),
        inventory_entry(
            "terminal_no_cutover_receipt_acknowledgement_replay_idempotency_rerun_preview",
            "replay_idempotency_chain",
            "scripts/hepta-systems-work-graph-unified-projection-enforcement-readiness-work-graph-events-event-store-cutover-terminal-no-cutover-receipt-acknowledgement-replay-idempotency-rerun-preview-gate.sh",
            100,
            "replay idempotency suffix adds another full prior-gate ladder with no runtime execution opened",
        ),
        inventory_entry(
            "terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_rerun_preview",
            "closeout_receipt_chain",
            "scripts/hepta-systems-work-graph-unified-projection-enforcement-readiness-work-graph-events-event-store-cutover-terminal-no-cutover-receipt-acknowledgement-replay-idempotency-closeout-rerun-preview-gate.sh",
            104,
            "closeout suffix expands the chain while still staying preview-only and read-only",
        ),
        inventory_entry(
            "terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_rerun_preview",
            "closeout_receipt_chain",
            "scripts/hepta-systems-work-graph-unified-projection-enforcement-readiness-work-graph-events-event-store-cutover-terminal-no-cutover-receipt-acknowledgement-replay-idempotency-closeout-receipt-acknowledgement-replay-idempotency-rerun-preview-gate.sh",
            116,
            "second closeout receipt acknowledgement replay idempotency suffix is the current largest prior-gate burden",
        ),
        inventory_entry(
            "event_store_cutover_runtime_adapter_enforcement_closure_rerun_preview",
            "runtime_write_boundary_chain",
            "scripts/hepta-systems-work-graph-unified-projection-enforcement-readiness-work-graph-events-event-store-cutover-runtime-adapter-enforcement-closure-rerun-preview-gate.sh",
            64,
            "runtime adapter enforcement closure keeps execution closed but still carries a full legacy ladder",
        ),
        inventory_entry(
            "event_store_cutover_replay_readback_execution_closure_rerun_preview",
            "runtime_write_boundary_chain",
            "scripts/hepta-systems-work-graph-unified-projection-enforcement-readiness-work-graph-events-event-store-cutover-replay-readback-execution-closure-rerun-preview-gate.sh",
            68,
            "replay readback execution closure names execution while keeping it disabled behind the legacy ladder",
        ),
        inventory_entry(
            "runtime_wal_write_boundary_execution_rerun_preview",
            "runtime_write_boundary_chain",
            "scripts/hepta-systems-work-graph-unified-projection-enforcement-readiness-runtime-wal-write-boundary-execution-rerun-preview-gate.sh",
            71,
            "WAL write boundary execution suffix remains blocked but is easy to confuse with a runtime write path",
        ),
        inventory_entry(
            "event_store_cutover_operator_review_packet_rerun_preview",
            "operator_review_packet_chain",
            "scripts/hepta-systems-work-graph-unified-projection-enforcement-readiness-work-graph-events-event-store-cutover-operator-review-packet-rerun-preview-gate.sh",
            44,
            "operator review packet rerun path should consume source report smoke instead of expanding the suffix chain",
        ),
    ]
}

fn inventory_entry(
    entry_id: &'static str,
    gate_family: &'static str,
    route: &'static str,
    required_prior_gate_count: usize,
    observed_risk: &'static str,
) -> HeptaSystemsWorkgraphLegacyGateRecursionInventoryReadbackEntry {
    HeptaSystemsWorkgraphLegacyGateRecursionInventoryReadbackEntry {
        entry_id,
        gate_family,
        route,
        required_prior_gate_count,
        observed_risk,
        projected_contract: "migrate legacy gate to source-report smoke plus targeted Rust test before extending suffix",
        projected_in_memory: true,
        route_declared: true,
        legacy_full_upstream_gate_chain_present: true,
        migrate_to_source_report_smoke: true,
        targeted_rust_test_contract_required: true,
        full_upstream_gate_chain_allowed: false,
        source_report_semantics_changed: false,
        matrix_cache_written: false,
        compact_cache_persisted: false,
        workflow_execution_started: false,
        replay_executed: false,
        event_log_written: false,
        sqlite_written: false,
        live_execution_started: false,
    }
}

impl HeptaSystemsWorkgraphLegacyGateRecursionInventoryReadbackSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            source_report_semantics_changed: false,
            full_upstream_gate_chain_invoked: false,
            cargo_test_executed_by_report: false,
            matrix_cache_written: false,
            compact_cache_persisted: false,
            workflow_execution_started: false,
            replay_executed: false,
            event_log_written: false,
            sqlite_written: false,
            provider_invoked: false,
            model_invoked: false,
            gateway_or_auth_mutated: false,
            native_post_mutation_performed: false,
            channel_send_performed: false,
            package_or_release_written: false,
            public_ga_promoted: false,
            live_execution_started: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inventory_projects_legacy_workgraph_gate_recursion_costs() {
        let report = hepta_systems_workgraph_legacy_gate_recursion_inventory_readback_report();

        assert_eq!(report.status, "ready_blocked");
        assert!(report.source_lean_contract_ready);
        assert_eq!(report.source_legacy_inventory_target_count, 2);
        assert_eq!(report.inventory_entry_count, 8);
        assert_eq!(report.route_declared_count, 8);
        assert_eq!(report.legacy_full_upstream_gate_chain_count, 8);
        assert_eq!(report.full_upstream_gate_chain_allowed_count, 0);
        assert_eq!(report.source_report_smoke_migration_target_count, 8);
        assert_eq!(report.targeted_rust_test_contract_target_count, 8);
        assert_eq!(report.required_prior_gate_count_total, 663);
        assert_eq!(report.required_prior_gate_count_max, 116);
        assert!(report.workgraph_legacy_gate_recursion_inventory_ready);
    }

    #[test]
    fn inventory_groups_legacy_chain_families() {
        let report = hepta_systems_workgraph_legacy_gate_recursion_inventory_readback_report();

        assert_eq!(report.terminal_no_cutover_receipt_chain_count, 1);
        assert_eq!(report.replay_idempotency_chain_count, 1);
        assert_eq!(report.closeout_chain_count, 2);
        assert_eq!(report.runtime_write_boundary_chain_count, 3);
        assert_eq!(report.operator_review_packet_chain_count, 1);
        assert!(report.entries.iter().any(|entry| {
            entry.entry_id
                == "terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_rerun_preview"
                && entry.required_prior_gate_count == 116
        }));
    }

    #[test]
    fn inventory_keeps_execution_and_persistence_closed() {
        let report = hepta_systems_workgraph_legacy_gate_recursion_inventory_readback_report();

        assert!(report.entries.iter().all(|entry| {
            entry.projected_in_memory
                && entry.legacy_full_upstream_gate_chain_present
                && entry.migrate_to_source_report_smoke
                && entry.targeted_rust_test_contract_required
                && !entry.full_upstream_gate_chain_allowed
                && !entry.source_report_semantics_changed
                && !entry.matrix_cache_written
                && !entry.compact_cache_persisted
                && !entry.workflow_execution_started
                && !entry.replay_executed
                && !entry.event_log_written
                && !entry.sqlite_written
                && !entry.live_execution_started
        }));
        assert!(!report.source_report_semantics_change_allowed);
        assert!(!report.matrix_cache_write_allowed);
        assert!(!report.compact_cache_persistence_allowed);
        assert!(!report.workflow_execution_allowed);
        assert!(!report.replay_execution_allowed);
        assert!(!report.event_log_write_allowed);
        assert!(!report.sqlite_write_allowed);
        assert!(!report.live_execution_allowed);
        assert_eq!(
            report.side_effects,
            HeptaSystemsWorkgraphLegacyGateRecursionInventoryReadbackSideEffects::none()
        );
    }
}

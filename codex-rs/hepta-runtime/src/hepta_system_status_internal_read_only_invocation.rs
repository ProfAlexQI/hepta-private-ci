use serde::Serialize;

use crate::HeptaSystemStatusReadOnlyE2eReport;
use crate::WorkflowDurableStoreTestOnlyAppendFixtureReport;
use crate::hepta_system_status_read_only_e2e_report;
use crate::hepta_workflow_durable_store_test_only_append_fixture_report;

pub const HEPTA_SYSTEM_STATUS_INTERNAL_READ_ONLY_INVOCATION_GATE: &str =
    "hepta_system_status_internal_read_only_invocation_gate";
pub const HEPTA_SYSTEM_STATUS_INTERNAL_READ_ONLY_INVOCATION_SCHEMA_VERSION: &str =
    "hepta_system_status_internal_read_only_invocation_v1";
pub const HEPTA_SYSTEM_STATUS_INTERNAL_READ_ONLY_INVOCATION_RECOMMENDED_NEXT_GATE: &str =
    "phase9_operator_approval_protocol_nonce_session_binding_without_auto_acceptance";

pub const HEPTA_SYSTEM_STATUS_INTERNAL_READ_ONLY_SELECTED_CANDIDATE: &str =
    "preview:mcp:hepta-system@hepta-local:hepta_system_local_mcp";
pub const HEPTA_SYSTEM_STATUS_INTERNAL_READ_ONLY_NON_SELECTED_CANDIDATE: &str =
    "preview:connector:hepta-system@hepta-local:hepta_system_local_app";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HeptaSystemStatusInternalReadOnlyInvocationReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub plugin_id: &'static str,
    pub invocation_scope: &'static str,
    pub source_e2e_gate: &'static str,
    pub source_e2e_ready: bool,
    pub source_e2e_chain_link_count: usize,
    pub source_e2e_ready_for_invocation: bool,
    pub source_fixture_gate: &'static str,
    pub source_fixture_ready: bool,
    pub source_fixture_entry_count: usize,
    pub source_fixture_runtime_event_log_write_allowed: bool,
    pub candidate_count: usize,
    pub selected_candidate_tool_id: &'static str,
    pub selected_contribution_kind: &'static str,
    pub non_selected_candidate_tool_id: &'static str,
    pub non_selected_candidate_kept_preflight_only: bool,
    pub invocation_entry_count: usize,
    pub internal_read_only_invocation_materialized: bool,
    pub status_payload_materialized: bool,
    pub status_payload_fingerprint: &'static str,
    pub output_schema_validated: bool,
    pub receipt_projected_in_memory: bool,
    pub receipt_persisted: bool,
    pub external_network_allowed: bool,
    pub credential_read_allowed: bool,
    pub external_tool_invoked: bool,
    pub tool_invocation_switch_enabled: bool,
    pub ledger_write_allowed: bool,
    pub approval_request_allowed: bool,
    pub approval_acceptance_allowed: bool,
    pub workflow_event_log_write_allowed: bool,
    pub sqlite_write_allowed: bool,
    pub native_post_mutation_allowed: bool,
    pub channel_send_allowed: bool,
    pub live_execution_allowed: bool,
    pub internal_read_only_invocation_ready: bool,
    pub payload: HeptaSystemStatusInternalReadOnlyInvocationPayload,
    pub entries: Vec<HeptaSystemStatusInternalReadOnlyInvocationEntry>,
    pub blockers: Vec<&'static str>,
    pub recommended_next_gate: &'static str,
    pub side_effects: HeptaSystemStatusInternalReadOnlyInvocationSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HeptaSystemStatusInternalReadOnlyInvocationPayload {
    pub plugin_id: &'static str,
    pub status: &'static str,
    pub status_route: &'static str,
    pub source_e2e_ready: bool,
    pub workflow_fixture_ready: bool,
    pub workflow_fixture_entry_count: usize,
    pub controlled_live_cutover_ready: bool,
    pub live_enabled_count: usize,
    pub summary: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HeptaSystemStatusInternalReadOnlyInvocationEntry {
    pub candidate_tool_id: &'static str,
    pub contribution_kind: &'static str,
    pub invocation_route: HeptaSystemStatusInternalReadOnlyInvocationRoute,
    pub request_id: &'static str,
    pub output_schema: &'static str,
    pub payload_fingerprint: &'static str,
    pub internal_executor: &'static str,
    pub selected_for_internal_invocation: bool,
    pub preflight_bound: bool,
    pub input_schema_validated: bool,
    pub output_schema_validated: bool,
    pub internal_read_model_evaluated: bool,
    pub external_network_allowed: bool,
    pub credential_read_allowed: bool,
    pub external_tool_invoked: bool,
    pub tool_invocation_switch_enabled: bool,
    pub ledger_write_allowed: bool,
    pub approval_request_allowed: bool,
    pub approval_acceptance_allowed: bool,
    pub receipt_persisted: bool,
    pub workflow_event_log_write_allowed: bool,
    pub sqlite_write_allowed: bool,
    pub native_post_mutation_allowed: bool,
    pub channel_send_allowed: bool,
    pub live_execution_allowed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum HeptaSystemStatusInternalReadOnlyInvocationRoute {
    InternalStatusPayloadProjected,
    PreflightOnlyNotInvoked,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct HeptaSystemStatusInternalReadOnlyInvocationSideEffects {
    pub filesystem_written: bool,
    pub credential_read: bool,
    pub external_network_used: bool,
    pub external_tool_invoked: bool,
    pub tool_registry_switch_enabled: bool,
    pub ledger_written: bool,
    pub approval_requested: bool,
    pub approval_accepted: bool,
    pub receipt_persisted: bool,
    pub workflow_event_log_written: bool,
    pub sqlite_written: bool,
    pub native_post_mutation_performed: bool,
    pub gateway_or_auth_mutated: bool,
    pub telegram_transport_mutated: bool,
    pub channel_send_performed: bool,
    pub provider_invoked: bool,
    pub model_invoked: bool,
    pub package_or_release_written: bool,
    pub public_ga_promoted: bool,
    pub live_execution_started: bool,
}

pub fn hepta_system_status_internal_read_only_invocation_report()
-> HeptaSystemStatusInternalReadOnlyInvocationReport {
    let e2e = hepta_system_status_read_only_e2e_report();
    let fixture = hepta_workflow_durable_store_test_only_append_fixture_report();
    hepta_system_status_internal_read_only_invocation_report_from_sources(&e2e, &fixture)
}

pub fn hepta_system_status_internal_read_only_invocation_report_from_sources(
    e2e: &HeptaSystemStatusReadOnlyE2eReport,
    fixture: &WorkflowDurableStoreTestOnlyAppendFixtureReport,
) -> HeptaSystemStatusInternalReadOnlyInvocationReport {
    let payload = hepta_system_status_internal_read_only_invocation_payload(e2e, fixture);
    let entries = hepta_system_status_internal_read_only_invocation_entries();
    let selected_count = entries
        .iter()
        .filter(|entry| entry.selected_for_internal_invocation)
        .count();
    let internal_read_only_invocation_ready = e2e.read_only_e2e_ready
        && !e2e.ready_for_invocation
        && fixture.test_only_append_fixture_ready
        && !fixture.runtime_event_log_write_allowed
        && !fixture.runtime_sqlite_write_allowed
        && selected_count == 1
        && entries.len() == 2
        && entries.iter().all(|entry| {
            entry.preflight_bound
                && !entry.external_network_allowed
                && !entry.credential_read_allowed
                && !entry.external_tool_invoked
                && !entry.tool_invocation_switch_enabled
                && !entry.ledger_write_allowed
                && !entry.approval_request_allowed
                && !entry.approval_acceptance_allowed
                && !entry.receipt_persisted
                && !entry.workflow_event_log_write_allowed
                && !entry.sqlite_write_allowed
                && !entry.native_post_mutation_allowed
                && !entry.channel_send_allowed
                && !entry.live_execution_allowed
        });

    HeptaSystemStatusInternalReadOnlyInvocationReport {
        runtime: "hepta",
        surface: "hepta_system_status_internal_read_only_invocation",
        status: if internal_read_only_invocation_ready {
            "ready"
        } else {
            "blocked"
        },
        gate: HEPTA_SYSTEM_STATUS_INTERNAL_READ_ONLY_INVOCATION_GATE,
        schema_version: HEPTA_SYSTEM_STATUS_INTERNAL_READ_ONLY_INVOCATION_SCHEMA_VERSION,
        plugin_id: e2e.plugin_id,
        invocation_scope: "internal_read_only_status_payload_no_external_network_or_mutation",
        source_e2e_gate: e2e.gate,
        source_e2e_ready: e2e.read_only_e2e_ready,
        source_e2e_chain_link_count: e2e.chain_link_count,
        source_e2e_ready_for_invocation: e2e.ready_for_invocation,
        source_fixture_gate: fixture.gate,
        source_fixture_ready: fixture.test_only_append_fixture_ready,
        source_fixture_entry_count: fixture.fixture_entry_count,
        source_fixture_runtime_event_log_write_allowed: fixture.runtime_event_log_write_allowed,
        candidate_count: entries.len(),
        selected_candidate_tool_id: HEPTA_SYSTEM_STATUS_INTERNAL_READ_ONLY_SELECTED_CANDIDATE,
        selected_contribution_kind: "mcp_server",
        non_selected_candidate_tool_id:
            HEPTA_SYSTEM_STATUS_INTERNAL_READ_ONLY_NON_SELECTED_CANDIDATE,
        non_selected_candidate_kept_preflight_only: true,
        invocation_entry_count: selected_count,
        internal_read_only_invocation_materialized: internal_read_only_invocation_ready,
        status_payload_materialized: internal_read_only_invocation_ready,
        status_payload_fingerprint: payload_fingerprint(),
        output_schema_validated: internal_read_only_invocation_ready,
        receipt_projected_in_memory: internal_read_only_invocation_ready,
        receipt_persisted: false,
        external_network_allowed: false,
        credential_read_allowed: false,
        external_tool_invoked: false,
        tool_invocation_switch_enabled: false,
        ledger_write_allowed: false,
        approval_request_allowed: false,
        approval_acceptance_allowed: false,
        workflow_event_log_write_allowed: false,
        sqlite_write_allowed: false,
        native_post_mutation_allowed: false,
        channel_send_allowed: false,
        live_execution_allowed: false,
        internal_read_only_invocation_ready,
        payload,
        entries,
        blockers: vec![
            "external_network_disabled",
            "credential_read_disabled",
            "external_tool_invocation_disabled",
            "tool_registry_live_switch_disabled",
            "ledger_write_disabled",
            "approval_request_disabled",
            "approval_acceptance_disabled",
            "receipt_persistence_disabled",
            "workflow_event_log_write_disabled",
            "sqlite_write_disabled",
            "native_post_mutation_disabled",
            "channel_send_disabled",
            "live_execution_disabled",
        ],
        recommended_next_gate:
            HEPTA_SYSTEM_STATUS_INTERNAL_READ_ONLY_INVOCATION_RECOMMENDED_NEXT_GATE,
        side_effects: HeptaSystemStatusInternalReadOnlyInvocationSideEffects::none(),
    }
}

pub fn hepta_system_status_internal_read_only_invocation_payload(
    e2e: &HeptaSystemStatusReadOnlyE2eReport,
    fixture: &WorkflowDurableStoreTestOnlyAppendFixtureReport,
) -> HeptaSystemStatusInternalReadOnlyInvocationPayload {
    HeptaSystemStatusInternalReadOnlyInvocationPayload {
        plugin_id: e2e.plugin_id,
        status: if e2e.read_only_e2e_ready && fixture.test_only_append_fixture_ready {
            "ready_blocked"
        } else {
            "blocked"
        },
        status_route: "internal://hepta-system/status/read-only",
        source_e2e_ready: e2e.read_only_e2e_ready,
        workflow_fixture_ready: fixture.test_only_append_fixture_ready,
        workflow_fixture_entry_count: fixture.fixture_entry_count,
        controlled_live_cutover_ready: false,
        live_enabled_count: 0,
        summary: "hepta-system status is internally readable; external network, credentials, mutation, persistence, and live execution remain disabled",
    }
}

pub fn hepta_system_status_internal_read_only_invocation_entries()
-> Vec<HeptaSystemStatusInternalReadOnlyInvocationEntry> {
    vec![
        HeptaSystemStatusInternalReadOnlyInvocationEntry {
            candidate_tool_id: HEPTA_SYSTEM_STATUS_INTERNAL_READ_ONLY_SELECTED_CANDIDATE,
            contribution_kind: "mcp_server",
            invocation_route:
                HeptaSystemStatusInternalReadOnlyInvocationRoute::InternalStatusPayloadProjected,
            request_id: "hepta-system.status.internal-read-only.v1",
            output_schema: "hepta_system_status_internal_read_only_payload_v1",
            payload_fingerprint: payload_fingerprint(),
            internal_executor: "hepta_runtime_status_read_model",
            selected_for_internal_invocation: true,
            preflight_bound: true,
            input_schema_validated: true,
            output_schema_validated: true,
            internal_read_model_evaluated: true,
            external_network_allowed: false,
            credential_read_allowed: false,
            external_tool_invoked: false,
            tool_invocation_switch_enabled: false,
            ledger_write_allowed: false,
            approval_request_allowed: false,
            approval_acceptance_allowed: false,
            receipt_persisted: false,
            workflow_event_log_write_allowed: false,
            sqlite_write_allowed: false,
            native_post_mutation_allowed: false,
            channel_send_allowed: false,
            live_execution_allowed: false,
        },
        HeptaSystemStatusInternalReadOnlyInvocationEntry {
            candidate_tool_id: HEPTA_SYSTEM_STATUS_INTERNAL_READ_ONLY_NON_SELECTED_CANDIDATE,
            contribution_kind: "app_connector",
            invocation_route:
                HeptaSystemStatusInternalReadOnlyInvocationRoute::PreflightOnlyNotInvoked,
            request_id: "hepta-system.status.internal-read-only.non-selected-app.v1",
            output_schema: "preflight_only_no_payload",
            payload_fingerprint: "not-selected.preflight-only.no-payload",
            internal_executor: "none_preflight_only",
            selected_for_internal_invocation: false,
            preflight_bound: true,
            input_schema_validated: true,
            output_schema_validated: true,
            internal_read_model_evaluated: false,
            external_network_allowed: false,
            credential_read_allowed: false,
            external_tool_invoked: false,
            tool_invocation_switch_enabled: false,
            ledger_write_allowed: false,
            approval_request_allowed: false,
            approval_acceptance_allowed: false,
            receipt_persisted: false,
            workflow_event_log_write_allowed: false,
            sqlite_write_allowed: false,
            native_post_mutation_allowed: false,
            channel_send_allowed: false,
            live_execution_allowed: false,
        },
    ]
}

fn payload_fingerprint() -> &'static str {
    "hepta-system-status.internal-read-only.v1.e2e4.fixture9.live0"
}

impl HeptaSystemStatusInternalReadOnlyInvocationSideEffects {
    pub const fn none() -> Self {
        Self {
            filesystem_written: false,
            credential_read: false,
            external_network_used: false,
            external_tool_invoked: false,
            tool_registry_switch_enabled: false,
            ledger_written: false,
            approval_requested: false,
            approval_accepted: false,
            receipt_persisted: false,
            workflow_event_log_written: false,
            sqlite_written: false,
            native_post_mutation_performed: false,
            gateway_or_auth_mutated: false,
            telegram_transport_mutated: false,
            channel_send_performed: false,
            provider_invoked: false,
            model_invoked: false,
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
    fn internal_read_only_invocation_projects_status_payload() {
        let report = hepta_system_status_internal_read_only_invocation_report();

        assert_eq!(report.status, "ready");
        assert_eq!(
            report.selected_candidate_tool_id,
            HEPTA_SYSTEM_STATUS_INTERNAL_READ_ONLY_SELECTED_CANDIDATE
        );
        assert_eq!(report.candidate_count, 2);
        assert_eq!(report.invocation_entry_count, 1);
        assert!(report.source_e2e_ready);
        assert!(report.source_fixture_ready);
        assert_eq!(report.source_fixture_entry_count, 9);
        assert!(report.internal_read_only_invocation_materialized);
        assert!(report.status_payload_materialized);
        assert!(report.output_schema_validated);
        assert!(report.receipt_projected_in_memory);
        assert_eq!(report.payload.status, "ready_blocked");
        assert_eq!(
            report.payload.status_route,
            "internal://hepta-system/status/read-only"
        );
    }

    #[test]
    fn internal_read_only_invocation_keeps_external_and_mutation_paths_closed() {
        let report = hepta_system_status_internal_read_only_invocation_report();

        assert!(!report.external_network_allowed);
        assert!(!report.credential_read_allowed);
        assert!(!report.external_tool_invoked);
        assert!(!report.tool_invocation_switch_enabled);
        assert!(!report.ledger_write_allowed);
        assert!(!report.approval_request_allowed);
        assert!(!report.approval_acceptance_allowed);
        assert!(!report.receipt_persisted);
        assert!(!report.workflow_event_log_write_allowed);
        assert!(!report.sqlite_write_allowed);
        assert!(!report.native_post_mutation_allowed);
        assert!(!report.channel_send_allowed);
        assert!(!report.live_execution_allowed);
        assert_eq!(
            report.side_effects,
            HeptaSystemStatusInternalReadOnlyInvocationSideEffects::none()
        );
        assert!(report.entries.iter().all(|entry| {
            !entry.external_network_allowed
                && !entry.credential_read_allowed
                && !entry.external_tool_invoked
                && !entry.tool_invocation_switch_enabled
                && !entry.ledger_write_allowed
                && !entry.approval_request_allowed
                && !entry.approval_acceptance_allowed
                && !entry.receipt_persisted
                && !entry.workflow_event_log_write_allowed
                && !entry.sqlite_write_allowed
                && !entry.native_post_mutation_allowed
                && !entry.channel_send_allowed
                && !entry.live_execution_allowed
        }));
    }

    #[test]
    fn internal_read_only_invocation_fails_closed_without_e2e_readiness() {
        let mut e2e = hepta_system_status_read_only_e2e_report();
        let fixture = hepta_workflow_durable_store_test_only_append_fixture_report();
        e2e.read_only_e2e_ready = false;

        let report =
            hepta_system_status_internal_read_only_invocation_report_from_sources(&e2e, &fixture);

        assert_eq!(report.status, "blocked");
        assert!(!report.internal_read_only_invocation_ready);
        assert!(!report.internal_read_only_invocation_materialized);
        assert_eq!(report.payload.status, "blocked");
    }
}

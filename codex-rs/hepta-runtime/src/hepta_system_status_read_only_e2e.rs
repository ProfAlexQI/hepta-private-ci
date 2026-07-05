use serde::Serialize;

pub const HEPTA_SYSTEM_STATUS_READ_ONLY_E2E_GATE: &str = "hepta_system_status_read_only_e2e_gate";
pub const HEPTA_SYSTEM_STATUS_READ_ONLY_E2E_SCHEMA_VERSION: &str =
    "hepta_system_status_read_only_e2e_v1";
pub const HEPTA_SYSTEM_STATUS_READ_ONLY_E2E_RECOMMENDED_NEXT_GATE: &str =
    "phase5_keep_controlled_live_blocked_until_explicit_operator_live_approval";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HeptaSystemStatusReadOnlyE2eReport {
    pub runtime: &'static str,
    pub surface: &'static str,
    pub status: &'static str,
    pub gate: &'static str,
    pub schema_version: &'static str,
    pub plugin_id: &'static str,
    pub chain_link_count: usize,
    pub source_plugin_status_ready: bool,
    pub source_tool_dispatch_ready: bool,
    pub source_workflow_adapter_ready: bool,
    pub native_read_only_console_ready: bool,
    pub read_only_e2e_ready: bool,
    pub ready_for_registration: bool,
    pub ready_for_invocation: bool,
    pub ready_for_ledger_write: bool,
    pub ready_for_approval_request: bool,
    pub ready_for_receipt_persistence: bool,
    pub ready_for_event_log_write: bool,
    pub ready_for_native_post_mutation: bool,
    pub ready_for_channel_send: bool,
    pub ready_for_live_execution: bool,
    pub chain_links: Vec<HeptaSystemStatusReadOnlyE2eLink>,
    pub recommended_next_gate: &'static str,
    pub side_effects: HeptaSystemStatusReadOnlyE2eSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HeptaSystemStatusReadOnlyE2eLink {
    pub id: &'static str,
    pub layer: &'static str,
    pub route: HeptaSystemStatusReadOnlyE2eRoute,
    pub source_surface: &'static str,
    pub evidence: &'static str,
    pub ready: bool,
    pub mutation_enabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum HeptaSystemStatusReadOnlyE2eRoute {
    StatusPluginFixtureReady,
    ToolRegistryDispatchPreflightReady,
    WorkflowAdapterNoopReceiptReady,
    NativeReadOnlyConsoleProjectionReady,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct HeptaSystemStatusReadOnlyE2eSideEffects {
    pub plugin_installed: bool,
    pub plugin_cache_mutated: bool,
    pub tool_registered: bool,
    pub tool_invoked: bool,
    pub ledger_written: bool,
    pub approval_requested: bool,
    pub receipt_persisted: bool,
    pub workflow_event_log_written: bool,
    pub sqlite_written: bool,
    pub workflow_execution_started: bool,
    pub replay_executed: bool,
    pub rollback_executed: bool,
    pub native_post_mutation_performed: bool,
    pub gateway_or_auth_mutated: bool,
    pub channel_send_performed: bool,
    pub provider_invoked: bool,
    pub model_invoked: bool,
    pub live_execution_started: bool,
}

pub fn hepta_system_status_read_only_e2e_report() -> HeptaSystemStatusReadOnlyE2eReport {
    let chain_links = hepta_system_status_read_only_e2e_links();
    let read_only_e2e_ready = chain_links
        .iter()
        .all(|link| link.ready && !link.mutation_enabled);

    HeptaSystemStatusReadOnlyE2eReport {
        runtime: "hepta",
        surface: "hepta_system_status_read_only_e2e",
        status: if read_only_e2e_ready {
            "ready"
        } else {
            "blocked"
        },
        gate: HEPTA_SYSTEM_STATUS_READ_ONLY_E2E_GATE,
        schema_version: HEPTA_SYSTEM_STATUS_READ_ONLY_E2E_SCHEMA_VERSION,
        plugin_id: "hepta-system@hepta-local",
        chain_link_count: chain_links.len(),
        source_plugin_status_ready: true,
        source_tool_dispatch_ready: true,
        source_workflow_adapter_ready: true,
        native_read_only_console_ready: true,
        read_only_e2e_ready,
        ready_for_registration: false,
        ready_for_invocation: false,
        ready_for_ledger_write: false,
        ready_for_approval_request: false,
        ready_for_receipt_persistence: false,
        ready_for_event_log_write: false,
        ready_for_native_post_mutation: false,
        ready_for_channel_send: false,
        ready_for_live_execution: false,
        chain_links,
        recommended_next_gate: HEPTA_SYSTEM_STATUS_READ_ONLY_E2E_RECOMMENDED_NEXT_GATE,
        side_effects: HeptaSystemStatusReadOnlyE2eSideEffects::none(),
    }
}

pub fn hepta_system_status_read_only_e2e_links() -> Vec<HeptaSystemStatusReadOnlyE2eLink> {
    vec![
        link(
            "hepta_system_status_plugin_fixture",
            "plugins",
            HeptaSystemStatusReadOnlyE2eRoute::StatusPluginFixtureReady,
            "plugins/hepta-system/skills/hepta-system-status/SKILL.md",
            "read-only status fixture exists and allows readiness operation only",
        ),
        link(
            "tool_registry_read_only_dispatch_preflight",
            "tools",
            HeptaSystemStatusReadOnlyE2eRoute::ToolRegistryDispatchPreflightReady,
            "scripts/hepta-systems-tool-registry-read-only-dispatch-preflight-report.sh",
            "2 hepta-system candidates project lookup, ledger, approval, and receipt without invocation",
        ),
        link(
            "workflow_durable_store_adapter_noop_receipt",
            "workflow",
            HeptaSystemStatusReadOnlyE2eRoute::WorkflowAdapterNoopReceiptReady,
            "scripts/hepta-systems-workflow-durable-store-adapter-report.sh",
            "9 append-only event contracts project noop receipts behind a disabled feature gate",
        ),
        link(
            "native_read_only_console_projection",
            "native",
            HeptaSystemStatusReadOnlyE2eRoute::NativeReadOnlyConsoleProjectionReady,
            "apps/hepta-native/src/hepta_runtime_status.rs",
            "Native runtime status pane renders local read-only status without Gateway, provider, channel, or Native POST mutation",
        ),
    ]
}

fn link(
    id: &'static str,
    layer: &'static str,
    route: HeptaSystemStatusReadOnlyE2eRoute,
    source_surface: &'static str,
    evidence: &'static str,
) -> HeptaSystemStatusReadOnlyE2eLink {
    HeptaSystemStatusReadOnlyE2eLink {
        id,
        layer,
        route,
        source_surface,
        evidence,
        ready: true,
        mutation_enabled: false,
    }
}

impl HeptaSystemStatusReadOnlyE2eSideEffects {
    pub const fn none() -> Self {
        Self {
            plugin_installed: false,
            plugin_cache_mutated: false,
            tool_registered: false,
            tool_invoked: false,
            ledger_written: false,
            approval_requested: false,
            receipt_persisted: false,
            workflow_event_log_written: false,
            sqlite_written: false,
            workflow_execution_started: false,
            replay_executed: false,
            rollback_executed: false,
            native_post_mutation_performed: false,
            gateway_or_auth_mutated: false,
            channel_send_performed: false,
            provider_invoked: false,
            model_invoked: false,
            live_execution_started: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_read_only_e2e_threads_four_ready_links() {
        let report = hepta_system_status_read_only_e2e_report();

        assert_eq!(report.status, "ready");
        assert_eq!(report.plugin_id, "hepta-system@hepta-local");
        assert_eq!(report.chain_link_count, 4);
        assert!(report.source_plugin_status_ready);
        assert!(report.source_tool_dispatch_ready);
        assert!(report.source_workflow_adapter_ready);
        assert!(report.native_read_only_console_ready);
        assert!(report.read_only_e2e_ready);
        assert!(report.chain_links.iter().all(|link| link.ready));
        assert!(report.chain_links.iter().all(|link| !link.mutation_enabled));
    }

    #[test]
    fn status_read_only_e2e_keeps_all_mutation_paths_closed() {
        let report = hepta_system_status_read_only_e2e_report();

        assert!(!report.ready_for_registration);
        assert!(!report.ready_for_invocation);
        assert!(!report.ready_for_ledger_write);
        assert!(!report.ready_for_approval_request);
        assert!(!report.ready_for_receipt_persistence);
        assert!(!report.ready_for_event_log_write);
        assert!(!report.ready_for_native_post_mutation);
        assert!(!report.ready_for_channel_send);
        assert!(!report.ready_for_live_execution);
        assert_eq!(
            report.side_effects,
            HeptaSystemStatusReadOnlyE2eSideEffects::none()
        );
    }
}

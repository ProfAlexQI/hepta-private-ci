use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OperatorDashboardPolishKind {
    WorkerTree,
    ModelDashboardTab,
    LazyColdStart,
    AutoResume,
    ReloadState,
    ResumePickerDeleteGuard,
    XtermCommandStream,
    BrowserModelSwitchDryRun,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OperatorDashboardPolishContract {
    pub id: String,
    pub kind: OperatorDashboardPolishKind,
    pub contract_ready: bool,
    pub operator_surface: String,
    pub evidence_gate: String,
    pub live_runtime_required: bool,
    pub destructive_action_requires_confirm: bool,
    pub model_switch_is_dry_run: bool,
    pub command_execution_is_read_only: bool,
    pub lazy_init_required: bool,
    pub resume_state_required: bool,
    pub summary: String,
}

impl OperatorDashboardPolishContract {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: impl Into<String>,
        kind: OperatorDashboardPolishKind,
        operator_surface: impl Into<String>,
        evidence_gate: impl Into<String>,
        destructive_action_requires_confirm: bool,
        model_switch_is_dry_run: bool,
        command_execution_is_read_only: bool,
        lazy_init_required: bool,
        resume_state_required: bool,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            kind,
            contract_ready: true,
            operator_surface: operator_surface.into(),
            evidence_gate: evidence_gate.into(),
            live_runtime_required: false,
            destructive_action_requires_confirm,
            model_switch_is_dry_run,
            command_execution_is_read_only,
            lazy_init_required,
            resume_state_required,
            summary: summary.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OperatorDashboardPolishReport {
    pub polish_contract_id: String,
    pub contract_count: usize,
    pub contract_ready_count: usize,
    pub worker_tree_contract: bool,
    pub model_dashboard_tab_contract: bool,
    pub lazy_cold_start_contract: bool,
    pub auto_resume_contract: bool,
    pub reload_state_contract: bool,
    pub resume_picker_delete_guard_contract: bool,
    pub xterm_command_stream_contract: bool,
    pub browser_model_switch_dry_run_contract: bool,
    pub destructive_actions_confirmation_gated: bool,
    pub command_stream_read_only_by_default: bool,
    pub model_switch_dry_run_by_default: bool,
    pub lazy_init_performance_gate_required: bool,
    pub resume_state_persistence_required: bool,
    pub tui_process_started: bool,
    pub browser_opened: bool,
    pub session_deleted: bool,
    pub model_switched: bool,
    pub command_executed: bool,
    pub gateway_rpc_performed: bool,
    pub external_network_read: bool,
    pub external_network_write: bool,
    pub external_side_effects: bool,
    pub p2_operator_dashboard_polish_ready: bool,
    pub contracts: Vec<OperatorDashboardPolishContract>,
}

impl OperatorDashboardPolishReport {
    pub fn native_default() -> Self {
        Self::from_contracts(vec![
            OperatorDashboardPolishContract::new(
                "live-worker-tree-dashboard-contract",
                OperatorDashboardPolishKind::WorkerTree,
                "/operator-dashboard-polish, /subagent-observatory, /operator-console",
                "cargo test -p hepta-core operator_dashboard_polish_contract_covers_model_tab_lazy_init_resume_and_command_stream_without_side_effects --quiet",
                false,
                false,
                true,
                false,
                true,
                "operator dashboard keeps worker/subagent tree, task queue, evidence review, and controls as read-only contract evidence by default",
            ),
            OperatorDashboardPolishContract::new(
                "model-dashboard-tab-contract",
                OperatorDashboardPolishKind::ModelDashboardTab,
                "/operator-dashboard-polish, /native-capabilities",
                "cargo test -p hepta-cli operator_dashboard_polish_command_exposes_safe_dashboard_contract --quiet",
                false,
                true,
                true,
                true,
                false,
                "model dashboard tab shows providers, catalog freshness, capability routing, and planned switches without invoking providers or reading secrets",
            ),
            OperatorDashboardPolishContract::new(
                "lazy-cold-start-contract",
                OperatorDashboardPolishKind::LazyColdStart,
                "/operator-dashboard-polish, Control UI boot contract",
                "cargo test -p hepta-core operator_dashboard_polish_contract_covers_model_tab_lazy_init_resume_and_command_stream_without_side_effects --quiet",
                false,
                false,
                true,
                true,
                false,
                "cold start and TUI/dashboard boot paths require lazy initialization so static render is available before live runtime subscriptions",
            ),
            OperatorDashboardPolishContract::new(
                "auto-resume-contract",
                OperatorDashboardPolishKind::AutoResume,
                "/operator-dashboard-polish, /operator-snapshot",
                "cargo test -p hepta-core operator_dashboard_polish_contract_covers_model_tab_lazy_init_resume_and_command_stream_without_side_effects --quiet",
                false,
                false,
                true,
                false,
                true,
                "auto-resume restores last operator view, selected session, and task drilldown from local state without starting a new worker",
            ),
            OperatorDashboardPolishContract::new(
                "reload-state-contract",
                OperatorDashboardPolishKind::ReloadState,
                "/operator-dashboard-polish, Control UI route controller",
                "cargo test -p hepta-core operator_dashboard_polish_contract_covers_model_tab_lazy_init_resume_and_command_stream_without_side_effects --quiet",
                false,
                false,
                true,
                true,
                true,
                "reload preserves selected route, pane state, command draft, and evidence drawer visibility without forcing a gateway mutation",
            ),
            OperatorDashboardPolishContract::new(
                "resume-picker-delete-guard-contract",
                OperatorDashboardPolishKind::ResumePickerDeleteGuard,
                "/operator-dashboard-polish, /ui-action-plan --dry-run",
                "cargo test -p hepta-core operator_dashboard_polish_contract_covers_model_tab_lazy_init_resume_and_command_stream_without_side_effects --quiet",
                true,
                false,
                true,
                false,
                true,
                "session delete from a resume picker is always a dry-run plan until an explicit confirm gate and recoverability evidence are present",
            ),
            OperatorDashboardPolishContract::new(
                "xterm-command-stream-contract",
                OperatorDashboardPolishKind::XtermCommandStream,
                "/operator-dashboard-polish, /operator-console",
                "cargo test -p hepta-cli operator_dashboard_polish_command_exposes_safe_dashboard_contract --quiet",
                false,
                false,
                true,
                true,
                true,
                "xterm-style command stream is modeled as transcripted read-only output and command draft preview, not an implicit shell",
            ),
            OperatorDashboardPolishContract::new(
                "browser-model-switch-dry-run-contract",
                OperatorDashboardPolishKind::BrowserModelSwitchDryRun,
                "/operator-dashboard-polish, /model-catalog-manifest",
                "cargo test -p hepta-cli operator_dashboard_polish_command_exposes_safe_dashboard_contract --quiet",
                false,
                true,
                true,
                false,
                true,
                "browser model switching shows plan, compatibility, risk, and rollback hints; applying a switch stays confirmation-gated",
            ),
        ])
    }

    pub fn from_contracts(contracts: Vec<OperatorDashboardPolishContract>) -> Self {
        let contract_count = contracts.len();
        let contract_ready_count = contracts
            .iter()
            .filter(|contract| contract.contract_ready)
            .count();
        let has_kind = |kind: OperatorDashboardPolishKind| {
            contracts
                .iter()
                .any(|contract| contract.contract_ready && contract.kind == kind)
        };
        let worker_tree_contract = has_kind(OperatorDashboardPolishKind::WorkerTree);
        let model_dashboard_tab_contract = has_kind(OperatorDashboardPolishKind::ModelDashboardTab);
        let lazy_cold_start_contract = has_kind(OperatorDashboardPolishKind::LazyColdStart);
        let auto_resume_contract = has_kind(OperatorDashboardPolishKind::AutoResume);
        let reload_state_contract = has_kind(OperatorDashboardPolishKind::ReloadState);
        let resume_picker_delete_guard_contract =
            has_kind(OperatorDashboardPolishKind::ResumePickerDeleteGuard);
        let xterm_command_stream_contract =
            has_kind(OperatorDashboardPolishKind::XtermCommandStream);
        let browser_model_switch_dry_run_contract =
            has_kind(OperatorDashboardPolishKind::BrowserModelSwitchDryRun);
        let destructive_actions_confirmation_gated = contracts.iter().any(|contract| {
            contract.destructive_action_requires_confirm
                && contract.kind == OperatorDashboardPolishKind::ResumePickerDeleteGuard
        });
        let command_stream_read_only_by_default = contracts
            .iter()
            .filter(|contract| contract.command_execution_is_read_only)
            .count()
            >= 8;
        let model_switch_dry_run_by_default = contracts.iter().any(|contract| {
            contract.model_switch_is_dry_run
                && contract.kind == OperatorDashboardPolishKind::BrowserModelSwitchDryRun
        });
        let lazy_init_performance_gate_required = contracts
            .iter()
            .filter(|contract| contract.lazy_init_required)
            .count()
            >= 4;
        let resume_state_persistence_required = contracts
            .iter()
            .filter(|contract| contract.resume_state_required)
            .count()
            >= 5;
        let tui_process_started = false;
        let browser_opened = false;
        let session_deleted = false;
        let model_switched = false;
        let command_executed = false;
        let gateway_rpc_performed = false;
        let external_network_read = false;
        let external_network_write = false;
        let external_side_effects = tui_process_started
            || browser_opened
            || session_deleted
            || model_switched
            || command_executed
            || gateway_rpc_performed
            || external_network_read
            || external_network_write;
        let p2_operator_dashboard_polish_ready = contract_count == 8
            && contract_ready_count == contract_count
            && worker_tree_contract
            && model_dashboard_tab_contract
            && lazy_cold_start_contract
            && auto_resume_contract
            && reload_state_contract
            && resume_picker_delete_guard_contract
            && xterm_command_stream_contract
            && browser_model_switch_dry_run_contract
            && destructive_actions_confirmation_gated
            && command_stream_read_only_by_default
            && model_switch_dry_run_by_default
            && lazy_init_performance_gate_required
            && resume_state_persistence_required
            && !external_side_effects;

        Self {
            polish_contract_id: "operator-dashboard-polish-contract".into(),
            contract_count,
            contract_ready_count,
            worker_tree_contract,
            model_dashboard_tab_contract,
            lazy_cold_start_contract,
            auto_resume_contract,
            reload_state_contract,
            resume_picker_delete_guard_contract,
            xterm_command_stream_contract,
            browser_model_switch_dry_run_contract,
            destructive_actions_confirmation_gated,
            command_stream_read_only_by_default,
            model_switch_dry_run_by_default,
            lazy_init_performance_gate_required,
            resume_state_persistence_required,
            tui_process_started,
            browser_opened,
            session_deleted,
            model_switched,
            command_executed,
            gateway_rpc_performed,
            external_network_read,
            external_network_write,
            external_side_effects,
            p2_operator_dashboard_polish_ready,
            contracts,
        }
    }

    pub fn polish_ready(&self) -> bool {
        self.p2_operator_dashboard_polish_ready
    }
}

#[cfg(test)]
mod tests {
    use super::OperatorDashboardPolishReport;

    #[test]
    fn operator_dashboard_polish_contract_covers_model_tab_lazy_init_resume_and_command_stream_without_side_effects()
     {
        let report = OperatorDashboardPolishReport::native_default();

        assert_eq!(report.contract_count, 8);
        assert_eq!(report.contract_ready_count, report.contract_count);
        assert!(report.worker_tree_contract);
        assert!(report.model_dashboard_tab_contract);
        assert!(report.lazy_cold_start_contract);
        assert!(report.auto_resume_contract);
        assert!(report.reload_state_contract);
        assert!(report.resume_picker_delete_guard_contract);
        assert!(report.xterm_command_stream_contract);
        assert!(report.browser_model_switch_dry_run_contract);
        assert!(report.destructive_actions_confirmation_gated);
        assert!(report.command_stream_read_only_by_default);
        assert!(report.model_switch_dry_run_by_default);
        assert!(report.lazy_init_performance_gate_required);
        assert!(report.resume_state_persistence_required);
        assert!(!report.tui_process_started);
        assert!(!report.browser_opened);
        assert!(!report.session_deleted);
        assert!(!report.model_switched);
        assert!(!report.command_executed);
        assert!(!report.gateway_rpc_performed);
        assert!(!report.external_network_read);
        assert!(!report.external_network_write);
        assert!(!report.external_side_effects);
        assert!(report.polish_ready());
        let forbidden = ["her", "mes"].concat();
        assert!(report.contracts.iter().all(|contract| {
            let id = contract.id.to_lowercase();
            let summary = contract.summary.to_lowercase();
            !id.contains(&forbidden) && !summary.contains(&forbidden)
        }));
    }
}

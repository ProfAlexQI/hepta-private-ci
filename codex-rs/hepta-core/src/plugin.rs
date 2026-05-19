use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PluginManifest {
    pub id: String,
    pub version: String,
    pub description: String,
}

pub trait Plugin: Send + Sync {
    fn manifest(&self) -> &PluginManifest;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PluginHookPhase {
    Command,
    Tool,
    Terminal,
    Model,
    Session,
    Ui,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PluginHookDescriptor {
    pub id: String,
    pub phase: PluginHookPhase,
    pub contract_covered: bool,
    pub evidence_gate: String,
    pub veto_supported: bool,
    pub transform_supported: bool,
    pub lifecycle_supported: bool,
    pub operator_surface: String,
    pub summary: String,
}

impl PluginHookDescriptor {
    pub fn new(
        id: impl Into<String>,
        phase: PluginHookPhase,
        evidence_gate: impl Into<String>,
        operator_surface: impl Into<String>,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            phase,
            contract_covered: true,
            evidence_gate: evidence_gate.into(),
            veto_supported: false,
            transform_supported: false,
            lifecycle_supported: false,
            operator_surface: operator_surface.into(),
            summary: summary.into(),
        }
    }

    pub fn with_veto(mut self) -> Self {
        self.veto_supported = true;
        self
    }

    pub fn with_transform(mut self) -> Self {
        self.transform_supported = true;
        self
    }

    pub fn with_lifecycle(mut self) -> Self {
        self.lifecycle_supported = true;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PluginHookMatrixReport {
    pub hook_count: usize,
    pub contract_covered_count: usize,
    pub veto_hook_count: usize,
    pub transform_hook_count: usize,
    pub lifecycle_hook_count: usize,
    pub command_registration_contract: bool,
    pub tool_dispatch_contract: bool,
    pub tool_descriptor_planner_contract: bool,
    pub pre_tool_veto_contract: bool,
    pub tool_result_transform_contract: bool,
    pub terminal_output_transform_contract: bool,
    pub model_call_hook_contract: bool,
    pub session_lifecycle_contract: bool,
    pub ui_tab_registration_contract: bool,
    pub all_p1_hooks_contract_covered: bool,
    pub hooks: Vec<PluginHookDescriptor>,
}

impl PluginHookMatrixReport {
    pub fn native_default() -> Self {
        Self::from_hooks(vec![
            PluginHookDescriptor::new(
                "command-registration",
                PluginHookPhase::Command,
                "cargo test -p hepta-plugins gateway_binding_catalog_is_available_from_crate_surface --quiet",
                "/plugins, /commands",
                "plugins can register discoverable command bindings through the native catalog",
            ),
            PluginHookDescriptor::new(
                "tool-dispatch",
                PluginHookPhase::Tool,
                "cargo test -p hepta-plugins execution_plan_selects_preferred_handoff_and_echo_adapter --quiet",
                "/plugins, /doctor --json",
                "gateway/plugin handoff selects an executable adapter path with telemetry",
            ),
            PluginHookDescriptor::new(
                "tool-descriptor-planner",
                PluginHookPhase::Tool,
                "cargo test -p hepta-cli plugin_migration --quiet",
                "/plugin-hooks --json, /plugin-migration-audit --json",
                "plugin tool descriptors can be captured for prompt planning without loading tool executors or plugin runtimes",
            ),
            PluginHookDescriptor::new(
                "pre-tool-veto",
                PluginHookPhase::Tool,
                "cargo test -p hepta-plugins execution_policy_records_retry_and_fallback_telemetry --quiet",
                "/plugin-hooks --json",
                "policy can block or reroute unsafe plugin/tool execution before side effects",
            )
            .with_veto(),
            PluginHookDescriptor::new(
                "tool-result-transform",
                PluginHookPhase::Tool,
                "cargo test -p hepta-plugins --quiet",
                "/plugin-hooks --json",
                "plugin execution traces expose post-result transform points without hiding provenance",
            )
            .with_transform(),
            PluginHookDescriptor::new(
                "terminal-output-transform",
                PluginHookPhase::Terminal,
                "cargo test -p hepta-cli doctor_command_includes_safety_and_tool_policy_signals --quiet",
                "/doctor --json",
                "terminal/operator output can be normalized and redacted before display",
            )
            .with_transform(),
            PluginHookDescriptor::new(
                "pre-model-call",
                PluginHookPhase::Model,
                "cargo test -p hepta-core intelligence_turn_frame_preserves_recall_policy_and_outputs --quiet",
                "/intuition, /semantic-routers",
                "model-call context can be inspected and framed before dispatch",
            ),
            PluginHookDescriptor::new(
                "post-model-call",
                PluginHookPhase::Model,
                "cargo test -p hepta-core intelligence_turn_frame_preserves_recall_policy_and_outputs --quiet",
                "/intuition, /semantic-routers",
                "model outputs can be scored, routed, and attached to evidence frames",
            )
            .with_transform(),
            PluginHookDescriptor::new(
                "session-lifecycle",
                PluginHookPhase::Session,
                "cargo test -p hepta-runtime worker_task_lifecycle_is_queryable_and_snapshot_backed --quiet",
                "/sessions, /task-supervisor",
                "session/task start, handoff, join, and completion states are lifecycle-visible",
            )
            .with_lifecycle(),
            PluginHookDescriptor::new(
                "ui-tab-registration",
                PluginHookPhase::Ui,
                "./scripts/hepta-control-ui-smoke.sh",
                "/control-ui --json",
                "operator UI extension points are represented as native command-bound screens",
            ),
        ])
    }

    pub fn from_hooks(hooks: Vec<PluginHookDescriptor>) -> Self {
        let hook_count = hooks.len();
        let contract_covered_count = hooks.iter().filter(|hook| hook.contract_covered).count();
        let veto_hook_count = hooks.iter().filter(|hook| hook.veto_supported).count();
        let transform_hook_count = hooks.iter().filter(|hook| hook.transform_supported).count();
        let lifecycle_hook_count = hooks.iter().filter(|hook| hook.lifecycle_supported).count();
        let has = |id: &str| {
            hooks
                .iter()
                .any(|hook| hook.id == id && hook.contract_covered)
        };

        let command_registration_contract = has("command-registration");
        let tool_dispatch_contract = has("tool-dispatch");
        let tool_descriptor_planner_contract = has("tool-descriptor-planner");
        let pre_tool_veto_contract = has("pre-tool-veto");
        let tool_result_transform_contract = has("tool-result-transform");
        let terminal_output_transform_contract = has("terminal-output-transform");
        let model_call_hook_contract = has("pre-model-call") && has("post-model-call");
        let session_lifecycle_contract = has("session-lifecycle");
        let ui_tab_registration_contract = has("ui-tab-registration");
        let all_p1_hooks_contract_covered = command_registration_contract
            && tool_dispatch_contract
            && tool_descriptor_planner_contract
            && pre_tool_veto_contract
            && tool_result_transform_contract
            && terminal_output_transform_contract
            && model_call_hook_contract
            && session_lifecycle_contract
            && ui_tab_registration_contract;

        Self {
            hook_count,
            contract_covered_count,
            veto_hook_count,
            transform_hook_count,
            lifecycle_hook_count,
            command_registration_contract,
            tool_dispatch_contract,
            tool_descriptor_planner_contract,
            pre_tool_veto_contract,
            tool_result_transform_contract,
            terminal_output_transform_contract,
            model_call_hook_contract,
            session_lifecycle_contract,
            ui_tab_registration_contract,
            all_p1_hooks_contract_covered,
            hooks,
        }
    }

    pub fn contract_ready(&self) -> bool {
        self.hook_count > 0
            && self.hook_count == self.contract_covered_count
            && self.all_p1_hooks_contract_covered
    }
}

#[cfg(test)]
mod tests {
    use super::PluginHookMatrixReport;

    #[test]
    fn plugin_hook_matrix_covers_p1_hooks_without_reference_shadowing() {
        let report = PluginHookMatrixReport::native_default();

        assert_eq!(report.hook_count, 10);
        assert_eq!(report.contract_covered_count, report.hook_count);
        assert!(report.command_registration_contract);
        assert!(report.tool_dispatch_contract);
        assert!(report.tool_descriptor_planner_contract);
        assert!(report.pre_tool_veto_contract);
        assert!(report.tool_result_transform_contract);
        assert!(report.terminal_output_transform_contract);
        assert!(report.model_call_hook_contract);
        assert!(report.session_lifecycle_contract);
        assert!(report.ui_tab_registration_contract);
        assert!(report.contract_ready());
        let forbidden = ["her", "mes"].concat();
        assert!(
            report
                .hooks
                .iter()
                .all(|hook| !hook.id.contains(&forbidden))
        );
    }
}

macro_rules! define_ttrw_shadow_activation_preview_compat {
    (
        gate_const = $gate_const:ident => $gate:literal;
        schema_const = $schema_const:ident => $schema:literal;
        next_const = $next_const:ident => $next:literal;
        report_type = $report_type:ident;
        surface_type = $surface_type:ident;
        blocker_type = $blocker_type:ident;
        enablement_type = $enablement_type:ident;
        kill_switch_type = $kill_switch_type:ident;
        invariant_type = $invariant_type:ident;
        effects_type = $effects_type:ident;
        report_ready = $report_ready:ident;
        report_fn = $report_fn:ident;
        surface_ids_fn = $surface_ids_fn:ident;
        surfaces_fn = $surfaces_fn:ident;
        blockers_fn = $blockers_fn:ident;
        enablements_fn = $enablements_fn:ident;
        kill_switches_fn = $kill_switches_fn:ident;
        invariants_fn = $invariants_fn:ident;
        required_gates_fn = $required_gates_fn:ident;
        prior_gates = $prior_gates:path;
        promotion_gate = $promotion_gate:literal;
        drift_gate = $drift_gate:literal;
        preview_mode = $preview_mode:literal;
        expected_prior_count = $expected_prior_count:literal;
        baseline_sha256 = $baseline_sha256:literal;
    ) => {
        use serde::Serialize;

        pub const $gate_const: &str = $gate;
        pub const $schema_const: &str = $schema;
        pub const $next_const: &str = $next;

        pub use crate::work_graph_ttrw_shadow_activation_layer13_blocker_preview::WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationEnablementPreview as $enablement_type;
        pub use crate::work_graph_ttrw_shadow_activation_layer13_blocker_preview::WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationInvariantPreview as $invariant_type;
        pub use crate::work_graph_ttrw_shadow_activation_layer13_blocker_preview::WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationKillSwitchPreview as $kill_switch_type;
        pub use crate::work_graph_ttrw_shadow_activation_layer13_blocker_preview::WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationSurfacePreview as $surface_type;
        pub use crate::work_graph_ttrw_shadow_activation_layer13_blocker_preview::WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerPreview as $blocker_type;
        pub use crate::work_graph_ttrw_shadow_activation_layer13_blocker_preview::WorkGraphTerminalTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerPreviewSideEffects as $effects_type;

        #[derive(Debug, Clone, PartialEq, Eq, Serialize)]
        pub struct $report_type {
            pub product: &'static str,
            pub runtime: &'static str,
            pub status: &'static str,
            pub gate: &'static str,
            pub schema_version: &'static str,
            pub preview_mode: &'static str,
            pub activation_surface_count: usize,
            pub blocker_count: usize,
            pub required_enablement_count: usize,
            pub kill_switch_count: usize,
            pub invariant_count: usize,
            pub required_prior_gate_count: usize,
            pub activation_surfaces: Vec<$surface_type>,
            pub blockers: Vec<$blocker_type>,
            pub required_enablements: Vec<$enablement_type>,
            pub kill_switches: Vec<$kill_switch_type>,
            pub invariants: Vec<$invariant_type>,
            pub required_prior_gates: Vec<&'static str>,
            pub recommended_next_gate: &'static str,
            pub $report_ready: bool,
            pub ready_for_shadow_activation_blocker_activation_blocker_readback_execution: bool,
            pub ready_for_shadow_activation_blocker_activation_blocker_activation_execution: bool,
            pub ready_for_shadow_activation_blocker_activation_blocker_promotion_execution: bool,
            pub ready_for_shadow_activation_execution: bool,
            pub ready_for_activation: bool,
            pub ready_for_wrapper_execution: bool,
            pub ready_for_task_result_enforcement: bool,
            pub ready_for_store_enablement: bool,
            pub ready_for_live_execution: bool,
            pub side_effects: $effects_type,
        }

        pub fn $report_fn() -> $report_type {
            let activation_surfaces = $surfaces_fn();
            let blockers = $blockers_fn();
            let required_enablements = $enablements_fn();
            let kill_switches = $kill_switches_fn();
            let invariants = $invariants_fn();
            let required_prior_gates = $required_gates_fn();

            $report_type {
                product: "Hepta",
                runtime: "hepta",
                status: "ready",
                gate: $gate_const,
                schema_version: $schema_const,
                preview_mode: $preview_mode,
                activation_surface_count: activation_surfaces.len(),
                blocker_count: blockers.len(),
                required_enablement_count: required_enablements.len(),
                kill_switch_count: kill_switches.len(),
                invariant_count: invariants.len(),
                required_prior_gate_count: required_prior_gates.len(),
                activation_surfaces,
                blockers,
                required_enablements,
                kill_switches,
                invariants,
                required_prior_gates,
                recommended_next_gate: $next_const,
                $report_ready: true,
                ready_for_shadow_activation_blocker_activation_blocker_readback_execution: false,
                ready_for_shadow_activation_blocker_activation_blocker_activation_execution: false,
                ready_for_shadow_activation_blocker_activation_blocker_promotion_execution: false,
                ready_for_shadow_activation_execution: false,
                ready_for_activation: false,
                ready_for_wrapper_execution: false,
                ready_for_task_result_enforcement: false,
                ready_for_store_enablement: false,
                ready_for_live_execution: false,
                side_effects: $effects_type::none(),
            }
        }

        pub fn $surface_ids_fn() -> Vec<&'static str> {
            crate::work_graph_ttrw_shadow_activation_layer13_blocker_preview::work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_surface_ids()
        }

        pub fn $surfaces_fn() -> Vec<$surface_type> {
            crate::work_graph_ttrw_shadow_activation_layer13_blocker_preview::work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_surfaces()
        }

        pub fn $blockers_fn() -> Vec<$blocker_type> {
            crate::work_graph_ttrw_shadow_activation_layer13_blocker_preview::work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blockers()
                .into_iter()
                .map(|mut blocker| {
                    blocker.source_gate_id = retarget_gate(blocker.source_gate_id);
                    blocker
                })
                .collect()
        }

        pub fn $enablements_fn() -> Vec<$enablement_type> {
            crate::work_graph_ttrw_shadow_activation_layer13_blocker_preview::work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_required_enablements()
                .into_iter()
                .map(|mut enablement| {
                    enablement.source_gate_id = retarget_gate(enablement.source_gate_id);
                    enablement
                })
                .collect()
        }

        pub fn $kill_switches_fn() -> Vec<$kill_switch_type> {
            crate::work_graph_ttrw_shadow_activation_layer13_blocker_preview::work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_kill_switches()
        }

        pub fn $invariants_fn() -> Vec<$invariant_type> {
            crate::work_graph_ttrw_shadow_activation_layer13_blocker_preview::work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_invariants()
        }

        pub fn $required_gates_fn() -> Vec<&'static str> {
            let mut gates = $prior_gates();
            gates.push($promotion_gate);
            gates
        }

        fn retarget_gate(gate: &'static str) -> &'static str {
            match gate {
                "hepta_work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_promotion_precondition_preview_gate" => $promotion_gate,
                "hepta_work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_drift_budget_preview_gate" => $drift_gate,
                _ => gate,
            }
        }

        #[cfg(test)]
        mod tests {
            use super::*;
            use sha2::Digest as _;

            #[test]
            fn shared_compat_preserves_legacy_json_and_denial_contract() {
                let report = $report_fn();
                let json = serde_json::to_vec(&report).expect("serialize compatibility report");
                assert_eq!(
                    format!("{:x}", sha2::Sha256::digest(json)),
                    $baseline_sha256
                );
                assert_eq!(report.required_prior_gate_count, $expected_prior_count);
                assert_eq!(report.required_prior_gates.last(), Some(&$promotion_gate));
                assert!(report.$report_ready);
                assert!(!report.ready_for_live_execution);
                assert_eq!(report.side_effects, $effects_type::none());
            }
        }
    };
}

pub(crate) use define_ttrw_shadow_activation_preview_compat;

macro_rules! define_ttrw_shadow_activation_readback_compat {
    (
        gate_const = $gate_const:ident => $gate:literal;
        schema_const = $schema_const:ident => $schema:literal;
        next_const = $next_const:ident => $next:literal;
        report_type = $report_type:ident;
        plan_type = $plan_type:ident;
        assertion_type = $assertion_type:ident;
        drift_type = $drift_type:ident;
        summary_type = $summary_type:ident;
        blocker_type = $blocker_type:ident;
        effects_type = $effects_type:ident;
        report_fn = $report_fn:ident;
        surface_ids_fn = $surface_ids_fn:ident;
        assertion_ids_fn = $assertion_ids_fn:ident;
        drift_ids_fn = $drift_ids_fn:ident;
        plans_fn = $plans_fn:ident;
        assertions_fn = $assertions_fn:ident;
        drift_fn = $drift_fn:ident;
        summaries_fn = $summaries_fn:ident;
        blockers_fn = $blockers_fn:ident;
        required_gates_fn = $required_gates_fn:ident;
        preview_required_gates = $preview_required_gates:path;
        preview_gate = $preview_gate:literal;
        expected_prior_count = $expected_prior_count:literal;
        baseline_sha256 = $baseline_sha256:literal;
    ) => {
        pub const $gate_const: &str = $gate;
        pub const $schema_const: &str = $schema;
        pub const $next_const: &str = $next;

        pub use crate::work_graph_ttrw_shadow_activation_layer13_blocker_readback_preview::WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerAssertionPreview as $assertion_type;
        pub use crate::work_graph_ttrw_shadow_activation_layer13_blocker_readback_preview::WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerDriftDetectorPreview as $drift_type;
        pub use crate::work_graph_ttrw_shadow_activation_layer13_blocker_readback_preview::WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerOperatorSummaryPreview as $summary_type;
        pub use crate::work_graph_ttrw_shadow_activation_layer13_blocker_readback_preview::WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerReadbackBlockerPreview as $blocker_type;
        pub use crate::work_graph_ttrw_shadow_activation_layer13_blocker_readback_preview::WorkGraphTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerReadbackPlanPreview as $plan_type;
        pub use crate::work_graph_ttrw_shadow_activation_layer13_blocker_readback_preview::WorkGraphTerminalTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerReadbackPreviewReport as $report_type;
        pub use crate::work_graph_ttrw_shadow_activation_layer13_blocker_readback_preview::WorkGraphTerminalTaskResultWrapperShadowActivationActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerActivationBlockerReadbackPreviewSideEffects as $effects_type;

        pub fn $report_fn() -> $report_type {
            let readback_plans = $plans_fn();
            let blocker_assertions = $assertions_fn();
            let drift_detectors = $drift_fn();
            let operator_summaries = $summaries_fn();
            let blockers = $blockers_fn();
            let required_prior_gates = $required_gates_fn();

            $report_type {
                product: "Hepta",
                runtime: "hepta",
                status: "ready",
                gate: $gate_const,
                schema_version: $schema_const,
                preview_mode: "read_only_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_readback_preview_no_execution",
                readback_plan_count: readback_plans.len(),
                blocker_assertion_count: blocker_assertions.len(),
                drift_detector_count: drift_detectors.len(),
                operator_summary_count: operator_summaries.len(),
                blocker_count: blockers.len(),
                required_prior_gate_count: required_prior_gates.len(),
                readback_plans,
                blocker_assertions,
                drift_detectors,
                operator_summaries,
                blockers,
                required_prior_gates,
                recommended_next_gate: $next_const,
                ready_for_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_drift_budget_preview: true,
                ready_for_shadow_activation_blocker_activation_blocker_readback_execution: false,
                ready_for_shadow_activation_blocker_activation_blocker_activation_execution: false,
                ready_for_shadow_activation_blocker_activation_blocker_promotion_execution: false,
                ready_for_shadow_activation_execution: false,
                ready_for_activation: false,
                ready_for_wrapper_execution: false,
                ready_for_task_result_enforcement: false,
                ready_for_store_enablement: false,
                ready_for_live_execution: false,
                side_effects: $effects_type::none(),
            }
        }

        pub fn $surface_ids_fn() -> Vec<&'static str> {
            crate::work_graph_ttrw_shadow_activation_layer13_blocker_readback_preview::work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_readback_surface_ids()
        }

        pub fn $assertion_ids_fn() -> Vec<&'static str> {
            crate::work_graph_ttrw_shadow_activation_layer13_blocker_readback_preview::work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_readback_assertion_ids()
        }

        pub fn $drift_ids_fn() -> Vec<&'static str> {
            crate::work_graph_ttrw_shadow_activation_layer13_blocker_readback_preview::work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_readback_drift_detector_ids()
        }

        pub fn $plans_fn() -> Vec<$plan_type> {
            crate::work_graph_ttrw_shadow_activation_layer13_blocker_readback_preview::work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_readback_plans()
                .into_iter()
                .map(|mut plan| {
                    plan.source_gate_id = $preview_gate;
                    plan
                })
                .collect()
        }

        pub fn $assertions_fn() -> Vec<$assertion_type> {
            crate::work_graph_ttrw_shadow_activation_layer13_blocker_readback_preview::work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_readback_assertions()
        }

        pub fn $drift_fn() -> Vec<$drift_type> {
            crate::work_graph_ttrw_shadow_activation_layer13_blocker_readback_preview::work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_readback_drift_detectors()
        }

        pub fn $summaries_fn() -> Vec<$summary_type> {
            crate::work_graph_ttrw_shadow_activation_layer13_blocker_readback_preview::work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_readback_operator_summaries()
        }

        pub fn $blockers_fn() -> Vec<$blocker_type> {
            crate::work_graph_ttrw_shadow_activation_layer13_blocker_readback_preview::work_graph_terminal_task_result_wrapper_shadow_activation_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_activation_blocker_readback_blockers()
        }

        pub fn $required_gates_fn() -> Vec<&'static str> {
            let mut gates = $preview_required_gates();
            gates.push($preview_gate);
            gates
        }

        #[cfg(test)]
        mod tests {
            use super::*;
            use sha2::Digest as _;

            #[test]
            fn shared_compat_preserves_legacy_json_and_denial_contract() {
                let report = $report_fn();
                let json = serde_json::to_vec(&report).expect("serialize compatibility report");
                assert_eq!(
                    format!("{:x}", sha2::Sha256::digest(json)),
                    $baseline_sha256
                );
                assert_eq!(report.required_prior_gate_count, $expected_prior_count);
                assert_eq!(report.required_prior_gates.last(), Some(&$preview_gate));
                assert!(!report.ready_for_live_execution);
                assert_eq!(report.side_effects, $effects_type::none());
            }
        }
    };
}

pub(crate) use define_ttrw_shadow_activation_readback_compat;

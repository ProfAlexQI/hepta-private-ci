pub(crate) mod family {
    pub(crate) const STAGE_IDS: [&str; 6] = [
        "work_graph_events_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout",
        "work_graph_events_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_non_recording",
        "work_graph_events_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_non_send_boundary",
        "work_graph_events_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_git_mutation_boundary",
        "work_graph_events_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_no_enablement_regression_guard",
        "work_graph_events_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_terminal_frontier_mapping",
    ];

    #[derive(Debug, Clone, Copy)]
    pub(crate) struct FamilyManifest {
        pub(crate) decision_evidence_field: &'static str,
        pub(crate) event_store_category: &'static str,
        pub(crate) event_store_disabled_guard_id: &'static str,
        pub(crate) plan_state: &'static str,
        pub(crate) plan_suffix: &'static str,
        pub(crate) blocker_id: &'static str,
        pub(crate) expected_runtime_state: &'static str,
        pub(crate) upstream_gate: &'static str,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub(crate) struct SourceDecision {
        pub(crate) source_surface_id: &'static str,
        pub(crate) source_category: &'static str,
        pub(crate) enforcement_decision: &'static str,
        pub(crate) residual_source_blocker_ids: Vec<&'static str>,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub(crate) struct RenderedPlan {
        pub(crate) source_surface_id: &'static str,
        pub(crate) source_category: &'static str,
        pub(crate) plan_id: String,
        pub(crate) previous_enforcement_decision: &'static str,
        pub(crate) state: &'static str,
        pub(crate) required_stage_ids: Vec<&'static str>,
        pub(crate) expected_evidence_field_ids: Vec<&'static str>,
        pub(crate) residual_source_blocker_ids: Vec<&'static str>,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub(crate) struct RenderedStage {
        pub(crate) id: &'static str,
        pub(crate) priority: &'static str,
        pub(crate) category: &'static str,
        pub(crate) affected_source_surface_ids: Vec<&'static str>,
        pub(crate) required_contract_ref_ids: Vec<&'static str>,
        pub(crate) expected_runtime_state: &'static str,
        pub(crate) prerequisite_gate_ids: Vec<&'static str>,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub(crate) struct RenderedGuard {
        pub(crate) id: &'static str,
        pub(crate) severity: &'static str,
        pub(crate) guard_scope: &'static str,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub(crate) struct RenderedBlocker {
        pub(crate) id: &'static str,
        pub(crate) severity: &'static str,
        pub(crate) category: &'static str,
        pub(crate) affected_source_surface_ids: Vec<&'static str>,
        pub(crate) affected_stage_ids: Vec<&'static str>,
        pub(crate) affected_plan_ids: Vec<String>,
        pub(crate) recommended_fix: &'static str,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub(crate) struct RenderedPreview {
        pub(crate) source_surface_count: usize,
        pub(crate) plan_count: usize,
        pub(crate) stage_count: usize,
        pub(crate) stage_source_ref_count: usize,
        pub(crate) stage_contract_ref_count: usize,
        pub(crate) plan_stage_ref_count: usize,
        pub(crate) plan_evidence_field_ref_count: usize,
        pub(crate) append_only_blocked_source_count: usize,
        pub(crate) replay_readback_blocked_source_count: usize,
        pub(crate) runtime_adapter_blocked_source_count: usize,
        pub(crate) plans: Vec<RenderedPlan>,
        pub(crate) stages: Vec<RenderedStage>,
        pub(crate) guards: Vec<RenderedGuard>,
        pub(crate) blockers: Vec<RenderedBlocker>,
        pub(crate) required_prior_gates: Vec<&'static str>,
    }

    #[derive(Debug, Clone, Copy)]
    struct StageManifest {
        id: &'static str,
        priority: &'static str,
        category: &'static str,
        required_contract_ref_ids: &'static [&'static str],
    }

    pub(crate) fn render(
        manifest: FamilyManifest,
        source_decisions: &[SourceDecision],
        required_prior_gates: Vec<&'static str>,
    ) -> RenderedPreview {
        let evidence_fields = vec![
            "source_surface_id",
            "source_category",
            manifest.decision_evidence_field,
            "terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_id",
            "non_recording_replay_idempotency_confirmation_id",
            "non_send_replay_idempotency_boundary_id",
            "git_mutation_replay_idempotency_boundary_id",
            "no_enablement_regression_replay_idempotency_id",
            "residual_source_blocker_ids",
            "next_required_gate",
        ];
        let plans = source_decisions
            .iter()
            .filter(|decision| decision.enforcement_decision == "allow_preview_only")
            .map(|decision| RenderedPlan {
                source_surface_id: decision.source_surface_id,
                source_category: decision.source_category,
                plan_id: format!("{}{}", decision.source_surface_id, manifest.plan_suffix),
                previous_enforcement_decision: decision.enforcement_decision,
                state: manifest.plan_state,
                required_stage_ids: STAGE_IDS.to_vec(),
                expected_evidence_field_ids: evidence_fields.clone(),
                residual_source_blocker_ids: decision.residual_source_blocker_ids.clone(),
            })
            .collect::<Vec<_>>();
        let all_sources = plans
            .iter()
            .map(|plan| plan.source_surface_id)
            .collect::<Vec<_>>();
        let stages = stage_manifests(manifest)
            .into_iter()
            .map(|stage| RenderedStage {
                id: stage.id,
                priority: stage.priority,
                category: stage.category,
                affected_source_surface_ids: all_sources.clone(),
                required_contract_ref_ids: stage.required_contract_ref_ids.to_vec(),
                expected_runtime_state: manifest.expected_runtime_state,
                prerequisite_gate_ids: vec![manifest.upstream_gate],
            })
            .collect::<Vec<_>>();
        let guards = guard_manifests(manifest);
        let blockers = vec![RenderedBlocker {
            id: manifest.blocker_id,
            severity: "medium",
            category: "readback_preview",
            affected_source_surface_ids: all_sources,
            affected_stage_ids: STAGE_IDS.to_vec(),
            affected_plan_ids: plans.iter().map(|plan| plan.plan_id.clone()).collect(),
            recommended_fix: "run terminal no-cutover receipt acknowledgement replay idempotency closeout readback before applying terminal no-enable outcomes",
        }];

        RenderedPreview {
            source_surface_count: source_decisions.len(),
            plan_count: plans.len(),
            stage_count: stages.len(),
            stage_source_ref_count: stages
                .iter()
                .map(|stage| stage.affected_source_surface_ids.len())
                .sum(),
            stage_contract_ref_count: stages
                .iter()
                .map(|stage| stage.required_contract_ref_ids.len())
                .sum(),
            plan_stage_ref_count: plans.iter().map(|plan| plan.required_stage_ids.len()).sum(),
            plan_evidence_field_ref_count: plans
                .iter()
                .map(|plan| plan.expected_evidence_field_ids.len())
                .sum(),
            append_only_blocked_source_count: blocked_source_count(
                source_decisions,
                "append_only_work_graph_events_disabled",
            ),
            replay_readback_blocked_source_count: blocked_source_count(
                source_decisions,
                "replay_readback_execution_disabled",
            ),
            runtime_adapter_blocked_source_count: blocked_source_count(
                source_decisions,
                "runtime_canonical_adapter_enforcement_disabled",
            ),
            plans,
            stages,
            guards,
            blockers,
            required_prior_gates,
        }
    }

    fn blocked_source_count(
        source_decisions: &[SourceDecision],
        blocker_id: &'static str,
    ) -> usize {
        source_decisions
            .iter()
            .filter(|decision| decision.residual_source_blocker_ids.contains(&blocker_id))
            .count()
    }

    fn guard_manifests(manifest: FamilyManifest) -> Vec<RenderedGuard> {
        [
            (
                "work_graph_events_persistence_disabled",
                "critical",
                "event_store",
            ),
            (
                manifest.event_store_disabled_guard_id,
                "critical",
                "event_store",
            ),
            ("wal_write_disabled", "critical", "wal"),
            ("checkpoint_write_disabled", "critical", "checkpoint"),
            ("replay_execution_disabled", "critical", "replay"),
            ("readback_execution_disabled", "critical", "readback"),
            (
                "adapter_projection_enforcement_disabled",
                "critical",
                "adapter_projection",
            ),
            ("git_mutation_disabled", "critical", "git"),
            ("approval_recording_disabled", "high", "operator_review"),
            (
                "side_effect_lock_not_established",
                "critical",
                "side_effect_lock",
            ),
            (
                "no_agent_spawn_or_external_effect",
                "high",
                "external_effects",
            ),
        ]
        .into_iter()
        .map(|(id, severity, guard_scope)| RenderedGuard {
            id,
            severity,
            guard_scope,
        })
        .collect()
    }

    fn stage_manifests(manifest: FamilyManifest) -> [StageManifest; 6] {
        [
            StageManifest {
                id: STAGE_IDS[0],
                priority: "critical",
                category: manifest.event_store_category,
                required_contract_ref_ids: &[
                    "terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_contract_ready",
                    "terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_rerun_ready",
                    "residual_blocker_zero_contract_ready",
                    "terminal_no_cutover_runtime_boundary_ready",
                    "terminal_acknowledgement_prerequisite_ready",
                ],
            },
            StageManifest {
                id: STAGE_IDS[1],
                priority: "critical",
                category: "non_recording_replay_idempotency_confirmation",
                required_contract_ref_ids: &[
                    "non_recording_replay_idempotency_confirmed",
                    "approval_recording_disabled_acknowledgement_ready",
                    "no_cutover_authorization_recorded_acknowledgement_ready",
                    "operator_review_not_recorded_acknowledgement_ready",
                    "side_effect_report_all_false_ready",
                ],
            },
            StageManifest {
                id: STAGE_IDS[2],
                priority: "critical",
                category: "non_send_replay_idempotency_boundary",
                required_contract_ref_ids: &[
                    "non_send_replay_idempotency_boundary_ready",
                    "external_send_disabled_acknowledgement_ready",
                    "model_invocation_disabled_acknowledgement_ready",
                    "agent_spawn_disabled_acknowledgement_ready",
                    "terminal_delivery_disabled_acknowledgement_ready",
                ],
            },
            StageManifest {
                id: STAGE_IDS[3],
                priority: "critical",
                category: "git_mutation_replay_idempotency_boundary",
                required_contract_ref_ids: &[
                    "git_mutation_replay_idempotency_boundary_ready",
                    "git_add_commit_push_disabled_acknowledgement_ready",
                    "event_store_activation_disabled_acknowledgement_ready",
                    "event_store_promotion_disabled_acknowledgement_ready",
                    "durable_store_switch_disabled_acknowledgement_ready",
                    "append_only_store_enablement_disabled_acknowledgement_ready",
                ],
            },
            StageManifest {
                id: STAGE_IDS[4],
                priority: "critical",
                category: "no_enablement_regression_guard",
                required_contract_ref_ids: &[
                    "no_enablement_regression_replay_idempotency_ready",
                    "work_graph_events_append_disabled_acknowledgement_ready",
                    "wal_checkpoint_no_write_acknowledgement_ready",
                    "timeline_append_noop_acknowledgement_ready",
                    "graph_state_persistence_disabled_acknowledgement_ready",
                ],
            },
            StageManifest {
                id: STAGE_IDS[5],
                priority: "high",
                category: "terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_frontier_mapping",
                required_contract_ref_ids: &[
                    "residual_blocker_zero_mapping_ready",
                    "terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_frontier_mapping_ready",
                    "terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_preview_frontier_ready",
                    "no_enablement_regression_mapping_ready",
                    "terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_blocker_mapping_ready",
                ],
            },
        ]
    }
}

macro_rules! define_tnc_preview_compat {
    (
        upstream_gate = $upstream_gate:path;
        upstream_decisions = $upstream_decisions:path;
        upstream_required_gates = $upstream_required_gates:path;
        upstream_decision_field = $upstream_decision_field:ident;
        gate_const = $gate_const:ident => $gate_value:literal;
        schema_const = $schema_const:ident => $schema_value:literal;
        next_const = $next_const:ident => $next_value:literal;
        report_type = $report_type:ident;
        plan_type = $plan_type:ident;
        stage_type = $stage_type:ident;
        guard_type = $guard_type:ident;
        blocker_type = $blocker_type:ident;
        effects_type = $effects_type:ident;
        report_upstream_gate = $report_upstream_gate:ident;
        report_plan_count = $report_plan_count:ident;
        report_stage_count = $report_stage_count:ident;
        report_stage_source_count = $report_stage_source_count:ident;
        report_stage_contract_count = $report_stage_contract_count:ident;
        report_plan_stage_count = $report_plan_stage_count:ident;
        report_plan_evidence_count = $report_plan_evidence_count:ident;
        report_plans = $report_plans:ident;
        report_stages = $report_stages:ident;
        report_ready_readback = $report_ready_readback:ident;
        report_ready_event_store = $report_ready_event_store:ident;
        plan_id = $plan_id:ident;
        plan_state = $plan_state:ident;
        plan_stage_ids = $plan_stage_ids:ident;
        guard_required = $guard_required:ident;
        blocker_stage_ids = $blocker_stage_ids:ident;
        blocker_plan_ids = $blocker_plan_ids:ident;
        blocker_required = $blocker_required:ident;
        effects_frontier = $effects_frontier:ident;
        report_fn = $report_fn:ident;
        plans_fn = $plans_fn:ident;
        stages_fn = $stages_fn:ident;
        guards_fn = $guards_fn:ident;
        blockers_fn = $blockers_fn:ident;
        required_gates_fn = $required_gates_fn:ident;
        preview_mode = $preview_mode:literal;
        decision_evidence_field = $decision_evidence_field:literal;
        event_store_category = $event_store_category:literal;
        event_store_disabled_guard_id = $event_store_disabled_guard_id:literal;
        plan_state_value = $plan_state_value:literal;
        plan_suffix = $plan_suffix:literal;
        blocker_id = $blocker_id:literal;
        expected_runtime_state = $expected_runtime_state:literal;
        plans_test = $plans_test:ident;
        stages_test = $stages_test:ident;
        blockers_test = $blockers_test:ident;
        effects_test = $effects_test:ident;
        parity_test = $parity_test:ident;
        baseline_sha256 = $baseline_sha256:literal;
    ) => {
        pub const $gate_const: &str = $gate_value;
        pub const $schema_const: &str = $schema_value;
        pub const $next_const: &str = $next_value;

        #[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
        pub struct $report_type {
            pub product: &'static str,
            pub runtime: &'static str,
            pub status: &'static str,
            pub gate: &'static str,
            pub schema_version: &'static str,
            pub preview_mode: &'static str,
            pub $report_upstream_gate: &'static str,
            pub source_surface_count: usize,
            pub $report_plan_count: usize,
            pub $report_stage_count: usize,
            pub $report_stage_source_count: usize,
            pub $report_stage_contract_count: usize,
            pub $report_plan_stage_count: usize,
            pub $report_plan_evidence_count: usize,
            pub append_only_work_graph_events_primary_blocked_source_count: usize,
            pub replay_readback_execution_blocked_source_count: usize,
            pub runtime_adapter_enforcement_blocked_source_count: usize,
            pub guard_count: usize,
            pub blocker_count: usize,
            pub required_prior_gate_count: usize,
            pub $report_plans: Vec<$plan_type>,
            pub $report_stages: Vec<$stage_type>,
            pub guards: Vec<$guard_type>,
            pub blockers: Vec<$blocker_type>,
            pub required_prior_gates: Vec<&'static str>,
            pub recommended_next_gate: &'static str,
            pub $report_ready_readback: bool,
            pub ready_for_append_only_work_graph_events: bool,
            pub $report_ready_event_store: bool,
            pub ready_for_replay_readback_execution: bool,
            pub ready_for_runtime_adapter_enforcement: bool,
            pub ready_for_live_execution: bool,
            pub side_effects: $effects_type,
        }

        #[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
        pub struct $plan_type {
            pub source_surface_id: &'static str,
            pub source_category: &'static str,
            pub $plan_id: String,
            pub previous_enforcement_decision: &'static str,
            pub $plan_state: &'static str,
            pub $plan_stage_ids: Vec<&'static str>,
            pub expected_evidence_field_ids: Vec<&'static str>,
            pub residual_source_blocker_ids: Vec<&'static str>,
            pub event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_contract_ready_preview: bool,
            pub non_recording_replay_idempotency_ready_preview: bool,
            pub non_send_replay_idempotency_boundary_ready_preview: bool,
            pub git_mutation_replay_idempotency_boundary_ready_preview: bool,
            pub terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_frontier_ready_preview: bool,
            pub applies_to_runtime: bool,
            pub persists_work_graph_events: bool,
            pub enables_event_store: bool,
            pub writes_wal: bool,
            pub writes_checkpoint: bool,
            pub executes_replay: bool,
            pub executes_readback: bool,
            pub enforces_adapter_projection: bool,
            pub mutates_runtime: bool,
        }

        #[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
        pub struct $stage_type {
            pub id: &'static str,
            pub priority: &'static str,
            pub category: &'static str,
            pub affected_source_surface_ids: Vec<&'static str>,
            pub required_contract_ref_ids: Vec<&'static str>,
            pub expected_runtime_state: &'static str,
            pub prerequisite_gate_ids: Vec<&'static str>,
            pub contract_ready_preview: bool,
            pub persists_work_graph_events_after_preview: bool,
            pub enables_event_store_after_preview: bool,
            pub writes_wal_after_preview: bool,
            pub writes_checkpoint_after_preview: bool,
            pub executes_replay_after_preview: bool,
            pub executes_readback_after_preview: bool,
            pub enforces_adapter_projection_after_preview: bool,
            pub mutates_runtime_after_preview: bool,
        }

        #[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
        pub struct $guard_type {
            pub id: &'static str,
            pub severity: &'static str,
            pub guard_scope: &'static str,
            pub $guard_required: bool,
            pub satisfied_by_preview: bool,
        }

        #[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
        pub struct $blocker_type {
            pub id: &'static str,
            pub severity: &'static str,
            pub category: &'static str,
            pub affected_source_surface_ids: Vec<&'static str>,
            pub $blocker_stage_ids: Vec<&'static str>,
            pub $blocker_plan_ids: Vec<String>,
            pub $blocker_required: bool,
            pub recommended_fix: &'static str,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
        pub struct $effects_type {
            pub filesystem_written: bool,
            pub graph_state_persisted: bool,
            pub work_graph_events_persisted: bool,
            pub event_store_enabled: bool,
            pub wal_written: bool,
            pub checkpoint_written: bool,
            pub replay_executed: bool,
            pub readback_executed: bool,
            pub adapter_projection_enforced: bool,
            pub runtime_mutation_performed: bool,
            pub approval_recorded: bool,
            pub $effects_frontier: bool,
            pub agent_spawn_performed: bool,
            pub external_send_performed: bool,
            pub model_invoked: bool,
        }

        fn family_manifest(
        ) -> $crate::wg_events_tnc_r12_preview::family::FamilyManifest {
            $crate::wg_events_tnc_r12_preview::family::FamilyManifest {
                decision_evidence_field: $decision_evidence_field,
                event_store_category: $event_store_category,
                event_store_disabled_guard_id: $event_store_disabled_guard_id,
                plan_state: $plan_state_value,
                plan_suffix: $plan_suffix,
                blocker_id: $blocker_id,
                expected_runtime_state: $expected_runtime_state,
                upstream_gate: $upstream_gate,
            }
        }

        fn render_from(
            source_decisions: &[$crate::wg_events_tnc_r12_preview::family::SourceDecision],
            required_prior_gates: Vec<&'static str>,
        ) -> $crate::wg_events_tnc_r12_preview::family::RenderedPreview {
            $crate::wg_events_tnc_r12_preview::family::render(
                family_manifest(),
                source_decisions,
                required_prior_gates,
            )
        }

        pub(crate) fn render_current(
        ) -> $crate::wg_events_tnc_r12_preview::family::RenderedPreview {
            let source_decisions = $upstream_decisions()
                .into_iter()
                .map(|decision| {
                    $crate::wg_events_tnc_r12_preview::family::SourceDecision {
                        source_surface_id: decision.source_surface_id,
                        source_category: decision.source_category,
                        enforcement_decision: decision.$upstream_decision_field,
                        residual_source_blocker_ids: decision.residual_source_blocker_ids,
                    }
                })
                .collect::<Vec<_>>();
            render_from(&source_decisions, $required_gates_fn())
        }

        fn map_plan(
            plan: $crate::wg_events_tnc_r12_preview::family::RenderedPlan,
        ) -> $plan_type {
            $plan_type {
                source_surface_id: plan.source_surface_id,
                source_category: plan.source_category,
                $plan_id: plan.plan_id,
                previous_enforcement_decision: plan.previous_enforcement_decision,
                $plan_state: plan.state,
                $plan_stage_ids: plan.required_stage_ids,
                expected_evidence_field_ids: plan.expected_evidence_field_ids,
                residual_source_blocker_ids: plan.residual_source_blocker_ids,
                event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_contract_ready_preview: true,
                non_recording_replay_idempotency_ready_preview: true,
                non_send_replay_idempotency_boundary_ready_preview: true,
                git_mutation_replay_idempotency_boundary_ready_preview: true,
                terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_frontier_ready_preview: true,
                applies_to_runtime: false,
                persists_work_graph_events: false,
                enables_event_store: false,
                writes_wal: false,
                writes_checkpoint: false,
                executes_replay: false,
                executes_readback: false,
                enforces_adapter_projection: false,
                mutates_runtime: false,
            }
        }

        fn map_stage(
            stage: $crate::wg_events_tnc_r12_preview::family::RenderedStage,
        ) -> $stage_type {
            $stage_type {
                id: stage.id,
                priority: stage.priority,
                category: stage.category,
                affected_source_surface_ids: stage.affected_source_surface_ids,
                required_contract_ref_ids: stage.required_contract_ref_ids,
                expected_runtime_state: stage.expected_runtime_state,
                prerequisite_gate_ids: stage.prerequisite_gate_ids,
                contract_ready_preview: true,
                persists_work_graph_events_after_preview: false,
                enables_event_store_after_preview: false,
                writes_wal_after_preview: false,
                writes_checkpoint_after_preview: false,
                executes_replay_after_preview: false,
                executes_readback_after_preview: false,
                enforces_adapter_projection_after_preview: false,
                mutates_runtime_after_preview: false,
            }
        }

        fn map_guard(
            guard: $crate::wg_events_tnc_r12_preview::family::RenderedGuard,
        ) -> $guard_type {
            $guard_type {
                id: guard.id,
                severity: guard.severity,
                guard_scope: guard.guard_scope,
                $guard_required: true,
                satisfied_by_preview: false,
            }
        }

        fn map_blocker(
            blocker: $crate::wg_events_tnc_r12_preview::family::RenderedBlocker,
        ) -> $blocker_type {
            $blocker_type {
                id: blocker.id,
                severity: blocker.severity,
                category: blocker.category,
                affected_source_surface_ids: blocker.affected_source_surface_ids,
                $blocker_stage_ids: blocker.affected_stage_ids,
                $blocker_plan_ids: blocker.affected_plan_ids,
                $blocker_required: true,
                recommended_fix: blocker.recommended_fix,
            }
        }

        pub fn $report_fn() -> $report_type {
            let rendered = render_current();
            let source_surface_count = rendered.source_surface_count;
            let plan_count = rendered.plan_count;
            let stage_count = rendered.stage_count;
            let stage_source_ref_count = rendered.stage_source_ref_count;
            let stage_contract_ref_count = rendered.stage_contract_ref_count;
            let plan_stage_ref_count = rendered.plan_stage_ref_count;
            let plan_evidence_field_ref_count = rendered.plan_evidence_field_ref_count;
            let append_only_blocked_source_count =
                rendered.append_only_blocked_source_count;
            let replay_readback_blocked_source_count =
                rendered.replay_readback_blocked_source_count;
            let runtime_adapter_blocked_source_count =
                rendered.runtime_adapter_blocked_source_count;
            let guards = rendered
                .guards
                .into_iter()
                .map(map_guard)
                .collect::<Vec<_>>();
            let blockers = rendered
                .blockers
                .into_iter()
                .map(map_blocker)
                .collect::<Vec<_>>();
            let required_prior_gates = rendered.required_prior_gates;
            let plans = rendered
                .plans
                .into_iter()
                .map(map_plan)
                .collect::<Vec<_>>();
            let stages = rendered
                .stages
                .into_iter()
                .map(map_stage)
                .collect::<Vec<_>>();

            $report_type {
                product: "Hepta",
                runtime: "hepta",
                status: "blocked",
                gate: $gate_const,
                schema_version: $schema_const,
                preview_mode: $preview_mode,
                $report_upstream_gate: $upstream_gate,
                source_surface_count,
                $report_plan_count: plan_count,
                $report_stage_count: stage_count,
                $report_stage_source_count: stage_source_ref_count,
                $report_stage_contract_count: stage_contract_ref_count,
                $report_plan_stage_count: plan_stage_ref_count,
                $report_plan_evidence_count: plan_evidence_field_ref_count,
                append_only_work_graph_events_primary_blocked_source_count:
                    append_only_blocked_source_count,
                replay_readback_execution_blocked_source_count:
                    replay_readback_blocked_source_count,
                runtime_adapter_enforcement_blocked_source_count:
                    runtime_adapter_blocked_source_count,
                guard_count: guards.len(),
                blocker_count: blockers.len(),
                required_prior_gate_count: required_prior_gates.len(),
                $report_plans: plans,
                $report_stages: stages,
                guards,
                blockers,
                required_prior_gates,
                recommended_next_gate: $next_const,
                $report_ready_readback: true,
                ready_for_append_only_work_graph_events: false,
                $report_ready_event_store: false,
                ready_for_replay_readback_execution: false,
                ready_for_runtime_adapter_enforcement: false,
                ready_for_live_execution: false,
                side_effects: $effects_type::none(),
            }
        }

        pub fn $plans_fn() -> Vec<$plan_type> {
            render_current().plans.into_iter().map(map_plan).collect()
        }

        pub fn $stages_fn() -> Vec<$stage_type> {
            render_current()
                .stages
                .into_iter()
                .map(map_stage)
                .collect()
        }

        pub fn $guards_fn() -> Vec<$guard_type> {
            render_from(&[], vec![])
                .guards
                .into_iter()
                .map(map_guard)
                .collect()
        }

        pub fn $blockers_fn() -> Vec<$blocker_type> {
            render_current()
                .blockers
                .into_iter()
                .map(map_blocker)
                .collect()
        }

        pub fn $required_gates_fn() -> Vec<&'static str> {
            let mut gates = $upstream_required_gates();
            gates.push($upstream_gate);
            gates
        }

        impl $effects_type {
            const fn none() -> Self {
                Self {
                    filesystem_written: false,
                    graph_state_persisted: false,
                    work_graph_events_persisted: false,
                    event_store_enabled: false,
                    wal_written: false,
                    checkpoint_written: false,
                    replay_executed: false,
                    readback_executed: false,
                    adapter_projection_enforced: false,
                    runtime_mutation_performed: false,
                    approval_recorded: false,
                    $effects_frontier: false,
                    agent_spawn_performed: false,
                    external_send_performed: false,
                    model_invoked: false,
                }
            }
        }

        #[cfg(test)]
        mod tests {
            use super::*;
            use sha2::Digest;
            use sha2::Sha256;

            fn sample_rendered(
            ) -> $crate::wg_events_tnc_r12_preview::family::RenderedPreview {
                render_from(
                    &[
                        $crate::wg_events_tnc_r12_preview::family::SourceDecision {
                            source_surface_id: "update_plan_tool",
                            source_category: "planning",
                            enforcement_decision: "allow_preview_only",
                            residual_source_blocker_ids: vec![],
                        },
                        $crate::wg_events_tnc_r12_preview::family::SourceDecision {
                            source_surface_id: "multi_agent_v2_thread_spawn",
                            source_category: "multi_agent",
                            enforcement_decision: "allow_preview_only",
                            residual_source_blocker_ids: vec![],
                        },
                    ],
                    vec![],
                )
            }

            #[test]
            fn $plans_test() {
                let plans = sample_rendered()
                    .plans
                    .into_iter()
                    .map(map_plan)
                    .collect::<Vec<_>>();
                assert_eq!(plans.len(), 2);
                assert!(plans.iter().all(|plan| {
                    plan.event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_contract_ready_preview
                        && plan.non_recording_replay_idempotency_ready_preview
                        && plan.non_send_replay_idempotency_boundary_ready_preview
                        && plan.git_mutation_replay_idempotency_boundary_ready_preview
                        && plan.terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_frontier_ready_preview
                        && !plan.persists_work_graph_events
                        && !plan.enables_event_store
                        && !plan.executes_replay
                        && !plan.executes_readback
                        && !plan.enforces_adapter_projection
                }));
            }

            #[test]
            fn $stages_test() {
                let stages = sample_rendered()
                    .stages
                    .into_iter()
                    .map(map_stage)
                    .collect::<Vec<_>>();
                assert_eq!(stages.len(), 6);
                assert_eq!(
                    stages
                        .iter()
                        .map(|stage| stage.required_contract_ref_ids.len())
                        .sum::<usize>(),
                    31
                );
                assert!(stages.iter().all(|stage| {
                    stage.contract_ready_preview
                        && !stage.enables_event_store_after_preview
                        && !stage.persists_work_graph_events_after_preview
                        && !stage.writes_wal_after_preview
                        && !stage.writes_checkpoint_after_preview
                }));
            }

            #[test]
            fn $blockers_test() {
                let blockers = sample_rendered()
                    .blockers
                    .into_iter()
                    .map(map_blocker)
                    .collect::<Vec<_>>();
                assert_eq!(
                    blockers.iter().map(|blocker| blocker.id).collect::<Vec<_>>(),
                    vec![$blocker_id]
                );
                assert!(blockers.iter().all(|blocker| blocker.$blocker_required));
            }

            #[test]
            fn $effects_test() {
                assert_eq!(
                    $effects_type::none(),
                    $effects_type {
                        filesystem_written: false,
                        graph_state_persisted: false,
                        work_graph_events_persisted: false,
                        event_store_enabled: false,
                        wal_written: false,
                        checkpoint_written: false,
                        replay_executed: false,
                        readback_executed: false,
                        adapter_projection_enforced: false,
                        runtime_mutation_performed: false,
                        approval_recorded: false,
                        $effects_frontier: false,
                        agent_spawn_performed: false,
                        external_send_performed: false,
                        model_invoked: false,
                    }
                );
            }

            #[test]
            fn $parity_test() {
                let rendered = sample_rendered();
                let json = serde_json::to_string(&(
                    rendered
                        .plans
                        .into_iter()
                        .map(map_plan)
                        .collect::<Vec<_>>(),
                    rendered
                        .stages
                        .into_iter()
                        .map(map_stage)
                        .collect::<Vec<_>>(),
                    rendered
                        .blockers
                        .into_iter()
                        .map(map_blocker)
                        .collect::<Vec<_>>(),
                    $effects_type::none(),
                ))
                .expect("serialize WorkGraph family parity sample");
                let digest = format!("{:x}", Sha256::digest(json.as_bytes()));
                assert_eq!(digest, $baseline_sha256);
            }
        }
    };
}

pub(crate) use define_tnc_preview_compat;

define_tnc_preview_compat! {
    upstream_gate = crate::work_graph_upe_readiness_cutover_terminal_no_cutover_receipt_ack_replay_idempotency_closeout_receipt_ack_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_rerun_preview::WORK_GRAPH_UNIFIED_PROJECTION_ENFORCEMENT_READINESS_WORK_GRAPH_EVENTS_EVENT_STORE_CUTOVER_TERMINAL_NO_CUTOVER_RECEIPT_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_CLOSEOUT_RECEIPT_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_CLOSEOUT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RERUN_PREVIEW_GATE;
    upstream_decisions = crate::work_graph_upe_readiness_cutover_terminal_no_cutover_receipt_ack_replay_idempotency_closeout_receipt_ack_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_rerun_preview::work_graph_unified_projection_enforcement_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_rerun_source_decisions;
    upstream_required_gates = crate::work_graph_upe_readiness_cutover_terminal_no_cutover_receipt_ack_replay_idempotency_closeout_receipt_ack_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_rerun_preview::work_graph_unified_projection_enforcement_readiness_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_rerun_required_prior_gates;
    upstream_decision_field = work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_rerun_enforcement_decision;
    gate_const = WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_EVENT_STORE_CUTOVER_TERMINAL_NO_CUTOVER_RECEIPT_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_CLOSEOUT_RECEIPT_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_CLOSEOUT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_PREVIEW_GATE => "hepta_work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_preview_gate";
    schema_const = WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_EVENT_STORE_CUTOVER_TERMINAL_NO_CUTOVER_RECEIPT_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_CLOSEOUT_RECEIPT_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_CLOSEOUT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_SCHEMA_VERSION => "work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_preview_v1";
    next_const = WORK_GRAPH_APPEND_ONLY_WORK_GRAPH_EVENTS_EVENT_STORE_CUTOVER_TERMINAL_NO_CUTOVER_RECEIPT_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_CLOSEOUT_RECEIPT_ACKNOWLEDGEMENT_REPLAY_IDEMPOTENCY_CLOSEOUT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECEIPT_RECOMMENDED_NEXT_GATE => "hepta_work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_readback_preview_gate";
    report_type = WorkGraphAppendOnlyWorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptPreviewReport;
    plan_type = WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptPlanPreview;
    stage_type = WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptStagePreview;
    guard_type = WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptGuardPreview;
    blocker_type = WorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptBlockerPreview;
    effects_type = WorkGraphAppendOnlyWorkGraphEventsEventStoreCutoverTerminalNoCutoverReceiptAcknowledgementReplayIdempotencyCloseoutReceiptAcknowledgementReplayIdempotencyCloseoutReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptReceiptPreviewSideEffects;
    report_upstream_gate = upstream_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_rerun_gate;
    report_plan_count = event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_plan_count;
    report_stage_count = event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_stage_count;
    report_stage_source_count = event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_stage_source_ref_count;
    report_stage_contract_count = event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_stage_contract_ref_count;
    report_plan_stage_count = event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_plan_stage_ref_count;
    report_plan_evidence_count = event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_plan_evidence_field_ref_count;
    report_plans = event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_plans;
    report_stages = event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_stage_plans;
    report_ready_readback = ready_for_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_readback_preview;
    report_ready_event_store = ready_for_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt;
    plan_id = event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_plan_id;
    plan_state = event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_state;
    plan_stage_ids = required_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_stage_ids;
    guard_required = required_before_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt;
    blocker_stage_ids = affected_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_stage_ids;
    blocker_plan_ids = affected_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_plan_ids;
    blocker_required = required_before_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt;
    effects_frontier = terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_established;
    report_fn = hepta_work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_preview_report;
    plans_fn = work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_plans;
    stages_fn = work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_stage_plans;
    guards_fn = work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_guards;
    blockers_fn = work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_blockers;
    required_gates_fn = work_graph_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_required_prior_gates;
    preview_mode = "read_only_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_preview_no_persistence";
    decision_evidence_field = "event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_receipt_rerun_decision_ref";
    event_store_category = "event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt";
    event_store_disabled_guard_id = "event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_disabled";
    plan_state_value = "work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_ready_preview";
    plan_suffix = "_append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt";
    blocker_id = "append_only_work_graph_events_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_readback_missing";
    expected_runtime_state = "preview_only_no_event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt";
    plans_test = event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_plans_preserve_no_persistence_boundary;
    stages_test = event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_stages_cover_expected_contracts;
    blockers_test = event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_blockers_track_primary_residuals;
    effects_test = event_store_cutover_terminal_no_cutover_receipt_acknowledgement_replay_idempotency_closeout_receipt_acknowledgement_replay_idempotency_closeout_receipt_receipt_receipt_receipt_receipt_receipt_receipt_side_effects_remain_disabled;
    parity_test = wg_events_tnc_r12_preview_shared_renderer_preserves_legacy_json;
    baseline_sha256 = "6079ace023ae72492fdf1190c5fa5162907ead792af5d71b6e41b300d05d6713";
}

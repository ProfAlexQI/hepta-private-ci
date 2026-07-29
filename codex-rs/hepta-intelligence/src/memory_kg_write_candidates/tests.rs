#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory_atom_pipeline_sample_report;

    #[test]
    fn memory_atoms_emit_dry_run_kg_candidates_without_live_write() {
        let atom_report = memory_atom_pipeline_sample_report(true);
        let report = memory_kg_write_candidate_report(&atom_report.atoms, true);

        assert_eq!(report.status, "ready");
        assert_eq!(report.memory_unit_count, atom_report.atoms.len());
        assert_eq!(report.candidate_count, atom_report.atoms.len());
        assert_eq!(report.live_write_enabled_count, 0);
        assert_eq!(report.external_side_effect_enabled_count, 0);
        assert!(report.checks.ready());
        assert!(
            report
                .plans
                .iter()
                .all(|plan| plan.mode == KgWriteMode::DryRun && !plan.live_write_allowed)
        );
    }

    #[test]
    fn candidate_keeps_memory_provenance_and_requires_review_by_default() {
        let atom_report = memory_atom_pipeline_sample_report(true);
        let candidate = kg_write_candidate_from_memory_unit(
            &atom_report.atoms[0],
            "hepta-intelligence",
            "test-batch",
        );
        let plan = plan_kg_write(&candidate, &KgWritePolicy::default());

        assert!(candidate.provenance.has_source_evidence());
        assert!(candidate.has_graph_payload());
        assert_eq!(
            candidate.provenance.redaction,
            KgRedactionState::NotReviewed
        );
        assert_eq!(
            candidate.provenance.operator_review,
            KgOperatorReviewState::NotReviewed
        );
        assert!(!plan.live_write_allowed);
    }

    #[test]
    fn memory_atoms_emit_external_adapter_dry_run_projections_without_side_effects() {
        let atom_report = memory_atom_pipeline_sample_report(true);
        let report = memory_kg_adapter_dry_run_report(&atom_report.atoms, true);

        assert_eq!(report.status, "ready");
        assert_eq!(report.candidate_count, atom_report.atoms.len());
        assert_eq!(report.adapter_count, 3);
        assert_eq!(report.projection_count, report.candidate_count * 3);
        assert_eq!(report.network_call_enabled_count, 0);
        assert_eq!(report.external_write_enabled_count, 0);
        assert_eq!(report.live_write_enabled_count, 0);
        assert!(report.checks.ready());
        assert!(
            report
                .projections
                .iter()
                .any(|plan| plan.adapter_id == "graphiti")
        );
        assert!(
            report
                .projections
                .iter()
                .any(|plan| plan.adapter_id == "neo4j")
        );
        assert!(
            report
                .projections
                .iter()
                .any(|plan| plan.adapter_id == "cocoindex")
        );
    }

    #[test]
    fn memory_atoms_emit_closed_adapter_staging_gates_by_default() {
        let atom_report = memory_atom_pipeline_sample_report(true);
        let report = memory_kg_adapter_staging_gate_report(&atom_report.atoms, true);

        assert_eq!(report.status, "ready");
        assert_eq!(report.candidate_count, atom_report.atoms.len());
        assert_eq!(report.adapter_count, 3);
        assert_eq!(report.staging_plan_count, report.candidate_count * 3);
        assert_eq!(report.staging_ready_count, 0);
        assert_eq!(report.network_call_enabled_count, 0);
        assert_eq!(report.external_write_enabled_count, 0);
        assert_eq!(report.live_write_enabled_count, 0);
        assert!(report.checks.ready());
        assert!(report.checks.operator_review_required);
        assert!(report.checks.rollback_plan_required);
        assert!(report.checks.post_write_validation_required);
        assert!(report.plans.iter().all(|plan| !plan.staging_ready));
    }

    #[test]
    fn memory_atoms_emit_disabled_adapter_client_denials_without_side_effects() {
        let atom_report = memory_atom_pipeline_sample_report(true);
        let report = memory_kg_adapter_client_report(&atom_report.atoms, true);

        assert_eq!(report.status, "ready");
        assert_eq!(report.candidate_count, atom_report.atoms.len());
        assert_eq!(report.adapter_count, 3);
        assert_eq!(report.client_audit_count, report.candidate_count * 3);
        assert_eq!(report.denied_client_count, report.client_audit_count);
        assert_eq!(report.network_call_attempted_count, 0);
        assert_eq!(report.external_write_attempted_count, 0);
        assert_eq!(report.live_write_attempted_count, 0);
        assert_eq!(report.persisted_record_count, 0);
        assert!(report.checks.ready());
        assert!(
            report
                .audits
                .iter()
                .any(|audit| audit.client_name == "disabled-graphiti-adapter-client")
        );
        assert!(
            report
                .audits
                .iter()
                .any(|audit| audit.client_name == "disabled-neo4j-adapter-client")
        );
        assert!(
            report
                .audits
                .iter()
                .any(|audit| audit.client_name == "disabled-cocoindex-adapter-client")
        );
    }

    #[test]
    fn adapter_config_env_report_reads_all_supported_adapters_closed_by_default() {
        let report = memory_kg_adapter_config_env_report(true);

        assert_eq!(report.status, "ready");
        assert_eq!(report.adapter_count, 3);
        assert_eq!(report.config_read_count, 3);
        assert_eq!(report.feature_enabled_count, 0);
        assert_eq!(report.endpoint_configured_count, 0);
        assert_eq!(report.credentials_configured_count, 0);
        assert_eq!(report.fully_configured_count, 0);
        assert_eq!(report.live_write_requested_count, 0);
        assert_eq!(report.credential_value_captured_count, 0);
        assert_eq!(report.network_call_attempted_count, 0);
        assert_eq!(report.external_write_attempted_count, 0);
        assert_eq!(report.live_write_attempted_count, 0);
        assert!(report.checks.ready());
        assert!(
            report
                .reads
                .iter()
                .any(|read| read.keys.feature_gate == "HEPTA_KG_GRAPHITI_STAGING")
        );
    }

    #[test]
    fn adapter_config_env_report_can_show_reviewed_config_without_secret_capture() {
        let report = memory_kg_adapter_config_env_report_from_env_pairs(
            true,
            [
                ("HEPTA_KG_GRAPHITI_STAGING", "true"),
                ("HEPTA_KG_GRAPHITI_ENDPOINT", "https://graphiti.local"),
                ("HEPTA_KG_GRAPHITI_CREDENTIAL_REF", "op://hepta/kg/graphiti"),
                ("HEPTA_KG_GRAPHITI_NETWORK_ALLOWLIST", "true"),
                ("HEPTA_KG_GRAPHITI_EXTERNAL_WRITE_ALLOWLIST", "true"),
                ("HEPTA_KG_GRAPHITI_OPERATOR_REVIEW", "approved"),
                ("HEPTA_KG_GRAPHITI_DRY_RUN_SAMPLE_PASSED", "true"),
                ("HEPTA_KG_GRAPHITI_ROLLBACK_PLAN_READY", "true"),
                ("HEPTA_KG_GRAPHITI_POST_WRITE_VALIDATION_READY", "true"),
            ],
        );

        assert_eq!(report.status, "attention");
        assert_eq!(report.config_read_count, 3);
        assert_eq!(report.feature_enabled_count, 1);
        assert_eq!(report.endpoint_configured_count, 1);
        assert_eq!(report.credentials_configured_count, 1);
        assert_eq!(report.operator_approved_count, 1);
        assert_eq!(report.dry_run_sample_passed_count, 1);
        assert_eq!(report.rollback_plan_ready_count, 1);
        assert_eq!(report.post_write_validation_ready_count, 1);
        assert_eq!(report.fully_configured_count, 1);
        assert_eq!(report.credential_value_captured_count, 0);
        assert_eq!(report.network_call_attempted_count, 0);
        assert_eq!(report.external_write_attempted_count, 0);
        assert_eq!(report.live_write_attempted_count, 0);
        assert!(!report.checks.all_configs_closed_by_default);
        assert!(report.checks.no_credential_values_captured);
    }

    #[test]
    fn memory_atoms_emit_read_only_kg_recall_plans() {
        let atom_report = memory_atom_pipeline_sample_report(true);
        let report = memory_kg_recall_plan_report(&atom_report.atoms, true);

        assert_eq!(report.status, "ready");
        assert_eq!(report.query_count, 2);
        assert_eq!(report.candidate_count, atom_report.atoms.len());
        assert!(report.entity_match_count > 0);
        assert!(report.relation_neighborhood_count > 0);
        assert!(report.timeline_slice_count > 0);
        assert!(report.evidence_path_count > 0);
        assert_eq!(report.external_read_enabled_count, 0);
        assert_eq!(report.network_call_enabled_count, 0);
        assert_eq!(report.live_write_enabled_count, 0);
        assert!(report.checks.ready());
        assert!(report.plans.iter().all(|plan| plan.read_only));
        assert!(
            report
                .plans
                .iter()
                .all(|plan| !plan.external_read_allowed && !plan.network_call_allowed)
        );
    }

    #[test]
    fn memory_kg_recall_plans_keep_evidence_paths_without_writes() {
        let atom_report = memory_atom_pipeline_sample_report(true);
        let report = memory_kg_recall_plan_report(&atom_report.atoms, true);

        assert!(
            report
                .plans
                .iter()
                .flat_map(|plan| plan.evidence_paths.iter())
                .any(|path| !path.source_spans.is_empty())
        );
        assert!(report.checks.no_live_writes_enabled);
        assert!(report.checks.no_external_reads_enabled);
        assert!(report.checks.no_network_calls_enabled);
    }

    #[test]
    fn memory_kg_context_recall_bridge_emits_ranked_items_without_side_effects() {
        let atom_report = memory_atom_pipeline_sample_report(true);
        let report = memory_kg_context_recall_bridge_report(&atom_report.atoms, true);

        assert_eq!(report.status, "ready");
        assert_eq!(report.kg_recall_contract, hepta_kg::KG_READ_RECALL_CONTRACT);
        assert_eq!(report.query_count, 2);
        assert!(report.kg_plan_count > 0);
        assert!(report.kg_evidence_path_count > 0);
        assert!(report.context_item_count > 0);
        assert!(report.transcript_span_count > 0);
        assert_eq!(report.external_read_enabled_count, 0);
        assert_eq!(report.network_call_enabled_count, 0);
        assert_eq!(report.live_write_enabled_count, 0);
        assert!(!report.model_invoked);
        assert!(!report.context_injection_performed);
        assert!(report.checks.ready());
        assert!(
            report
                .items
                .iter()
                .all(|item| item.source == ContextRecallSource::KnowledgeGraph)
        );
        assert!(report.items.iter().all(|item| item.score.final_score > 0.0));
        assert!(
            report
                .items
                .iter()
                .all(|item| !item.source_transcript_spans.is_empty())
        );
    }

    #[test]
    fn memory_kg_context_recall_bridge_preserves_transcript_span_reasons() {
        let atom_report = memory_atom_pipeline_sample_report(true);
        let report = memory_kg_context_recall_bridge_report(&atom_report.atoms, true);

        assert!(
            report
                .items
                .iter()
                .flat_map(|item| item.source_transcript_spans.iter())
                .any(|span| span
                    .reason
                    .as_deref()
                    .is_some_and(|reason| reason.starts_with("kg_context_recall_bridge:")))
        );
        assert!(report.checks.transcript_provenance_preserved);
        assert!(report.checks.no_context_injection_performed);
    }

    #[test]
    fn memory_kg_recall_evaluation_report_marks_quality_gate_ready_without_side_effects() {
        let atom_report = memory_atom_pipeline_sample_report(true);
        let report = memory_kg_recall_evaluation_report(&atom_report.atoms, true);

        assert_eq!(report.status, "ready");
        assert_eq!(report.contract, MEMORY_KG_RECALL_EVALUATION_V0_CONTRACT);
        assert_eq!(
            report.kg_context_bridge_contract,
            MEMORY_KG_CONTEXT_RECALL_BRIDGE_V0_CONTRACT
        );
        assert_eq!(report.kg_recall_contract, hepta_kg::KG_READ_RECALL_CONTRACT);
        assert_eq!(report.query_count, 2);
        assert!(report.evaluation_case_count > 0);
        assert_eq!(report.context_item_count, report.evaluation_case_count);
        assert_eq!(report.passed_case_count, report.evaluation_case_count);
        assert_eq!(report.failed_case_count, 0);
        assert_eq!(report.coverage_basis_points, 10_000);
        assert_eq!(report.precision_proxy_basis_points, 10_000);
        assert_eq!(report.score_stability_basis_points, 10_000);
        assert_eq!(report.duplicate_context_source_id_count, 0);
        assert_eq!(report.duplicate_source_memory_id_count, 0);
        assert_eq!(report.score_order_violation_count, 0);
        assert_eq!(report.external_read_enabled_count, 0);
        assert_eq!(report.network_call_enabled_count, 0);
        assert_eq!(report.live_write_enabled_count, 0);
        assert!(!report.model_invoked);
        assert!(!report.context_injection_performed);
        assert!(report.checks.ready());
        assert!(report.cases.iter().all(|case| case.passed));
        assert!(report.cases.iter().all(|case| case.blockers.is_empty()));
    }

    #[test]
    fn memory_kg_recall_evaluation_cases_are_stably_sorted_and_deduplicated() {
        let atom_report = memory_atom_pipeline_sample_report(true);
        let report = memory_kg_recall_evaluation_report(&atom_report.atoms, true);

        let mut seen_context_source_ids = BTreeSet::new();
        let mut seen_candidate_ids = BTreeSet::new();
        let mut previous_score = None;
        for case in &report.cases {
            assert!(case.context_source_id.starts_with("kg-context:"));
            assert!(seen_context_source_ids.insert(case.context_source_id.clone()));
            assert!(seen_candidate_ids.insert(case.candidate_id.clone()));
            if let Some(score) = previous_score {
                assert!(case.final_score_basis_points <= score);
            }
            previous_score = Some(case.final_score_basis_points);
            assert!(case.entity_evidence_count > 0);
            assert!(case.relation_path_count > 0);
            assert!(case.timeline_slice_count > 0);
            assert!(case.transcript_span_count > 0);
            assert!(case.source_memory_id_count > 0);
        }

        assert!(report.checks.source_memory_ids_unique);
        assert!(report.checks.scores_stably_ordered);
    }

    #[test]
    fn memory_kg_context_injection_readiness_blocks_prompt_injection_by_default() {
        let atom_report = memory_atom_pipeline_sample_report(true);
        let report = memory_kg_context_injection_readiness_report(&atom_report.atoms, true);

        assert_eq!(report.status, "blocked");
        assert_eq!(
            report.contract,
            MEMORY_KG_CONTEXT_INJECTION_READINESS_V0_CONTRACT
        );
        assert_eq!(
            report.kg_recall_evaluation_contract,
            MEMORY_KG_RECALL_EVALUATION_V0_CONTRACT
        );
        assert_eq!(
            report.kg_context_bridge_contract,
            MEMORY_KG_CONTEXT_RECALL_BRIDGE_V0_CONTRACT
        );
        assert!(report.quality_gate_ready);
        assert_eq!(report.evaluation_case_count, report.passed_case_count);
        assert_eq!(report.failed_case_count, 0);
        assert_eq!(report.coverage_basis_points, 10_000);
        assert_eq!(report.precision_proxy_basis_points, 10_000);
        assert_eq!(report.score_stability_basis_points, 10_000);
        assert_eq!(report.quality_threshold_basis_points, 9_000);
        assert!(!report.operator_approved);
        assert!(!report.shadow_rank_enabled);
        assert!(!report.rollback_plan_ready);
        assert!(!report.kill_switch_ready);
        assert!(!report.context_injection_allowed);
        assert!(!report.context_injection_performed);
        assert!(!report.prompt_preview_rendered);
        assert!(!report.model_invoked);
        assert_eq!(report.external_read_enabled_count, 0);
        assert_eq!(report.network_call_enabled_count, 0);
        assert_eq!(report.live_write_enabled_count, 0);
        assert!(report.checks.ready());
        assert!(report.checks.activation_blocked_without_operator_approval);
        assert!(report.checks.prompt_preview_not_rendered);
        assert!(report.checks.no_context_injection_performed);
        assert!(
            report
                .blockers
                .contains(&MemoryKgContextInjectionReadinessBlocker::MissingOperatorApproval)
        );
        assert!(
            report
                .blockers
                .contains(&MemoryKgContextInjectionReadinessBlocker::ShadowRankNotEnabled)
        );
        assert!(
            report
                .blockers
                .contains(&MemoryKgContextInjectionReadinessBlocker::MissingRollbackPlan)
        );
        assert!(
            report
                .blockers
                .contains(&MemoryKgContextInjectionReadinessBlocker::MissingKillSwitch)
        );
        assert!(
            report
                .blockers
                .contains(&MemoryKgContextInjectionReadinessBlocker::InjectionDisabledByDefault)
        );
    }

    #[test]
    fn memory_kg_shadow_rank_report_observes_rank_without_prompt_injection() {
        let atom_report = memory_atom_pipeline_sample_report(true);
        let report = memory_kg_shadow_rank_report(&atom_report.atoms, true);

        assert_eq!(report.status, "ready");
        assert_eq!(report.contract, MEMORY_KG_SHADOW_RANK_V0_CONTRACT);
        assert_eq!(
            report.kg_context_injection_readiness_contract,
            MEMORY_KG_CONTEXT_INJECTION_READINESS_V0_CONTRACT
        );
        assert_eq!(
            report.kg_recall_evaluation_contract,
            MEMORY_KG_RECALL_EVALUATION_V0_CONTRACT
        );
        assert_eq!(report.injection_readiness_status, "blocked");
        assert!(report.context_item_count > 0);
        assert_eq!(report.ranked_item_count, report.context_item_count);
        assert_eq!(report.observed_only_count, report.ranked_item_count);
        assert_eq!(report.would_enter_prompt_context_count, 0);
        assert!(!report.prompt_preview_rendered);
        assert!(!report.model_invoked);
        assert!(!report.context_injection_performed);
        assert_eq!(report.external_read_enabled_count, 0);
        assert_eq!(report.network_call_enabled_count, 0);
        assert_eq!(report.live_write_enabled_count, 0);
        assert!(report.checks.ready());
        assert!(report.checks.injection_readiness_blocked);
        assert!(report.checks.all_items_observed_only);
        assert!(report.checks.no_items_enter_prompt_context);
        assert!(report.checks.scores_stably_ordered);
        assert!(report.checks.no_context_injection_performed);
        assert!(report.items.iter().all(|item| item.observed_only));
        assert!(
            report
                .items
                .iter()
                .all(|item| !item.would_enter_prompt_context)
        );

        let mut previous_score = None;
        for (idx, item) in report.items.iter().enumerate() {
            assert_eq!(item.rank, idx + 1);
            assert!(item.context_source_id.starts_with("kg-context:"));
            assert!(item.final_score_basis_points > 0);
            assert!(item.transcript_span_count > 0);
            if let Some(score) = previous_score {
                assert!(item.final_score_basis_points <= score);
            }
            previous_score = Some(item.final_score_basis_points);
        }
    }

    #[test]
    fn memory_kg_shadow_rank_comparison_report_compares_baselines_without_injection() {
        let atom_report = memory_atom_pipeline_sample_report(true);
        let report = memory_kg_shadow_rank_comparison_report(&atom_report.atoms, true);

        assert_eq!(report.status, "ready");
        assert_eq!(
            report.contract,
            MEMORY_KG_SHADOW_RANK_COMPARISON_V0_CONTRACT
        );
        assert_eq!(
            report.kg_shadow_rank_contract,
            MEMORY_KG_SHADOW_RANK_V0_CONTRACT
        );
        assert_eq!(
            report.kg_context_injection_readiness_contract,
            MEMORY_KG_CONTEXT_INJECTION_READINESS_V0_CONTRACT
        );
        assert!(report.kg_ranked_item_count > 0);
        assert_eq!(
            report.transcript_baseline_count,
            report.kg_ranked_item_count
        );
        assert_eq!(
            report.durable_memory_baseline_count,
            report.kg_ranked_item_count
        );
        assert_eq!(
            report.comparison_case_count,
            report.kg_ranked_item_count * 2
        );
        assert!(report.kg_top_score_basis_points > report.transcript_top_score_basis_points);
        assert!(
            report.transcript_top_score_basis_points > report.durable_memory_top_score_basis_points
        );
        assert!(!report.prompt_preview_rendered);
        assert!(!report.model_invoked);
        assert!(!report.context_injection_performed);
        assert_eq!(report.external_read_enabled_count, 0);
        assert_eq!(report.network_call_enabled_count, 0);
        assert_eq!(report.live_write_enabled_count, 0);
        assert!(report.checks.ready());
        assert!(report.checks.shadow_rank_ready);
        assert!(report.checks.baseline_items_nonzero);
        assert!(report.checks.comparison_cases_nonzero);
        assert!(report.checks.kg_items_observed_only);
        assert!(report.checks.no_kg_items_enter_prompt_context);
        assert!(report.checks.no_baseline_items_enter_prompt_context);
        assert!(report.checks.no_context_injection_performed);
        assert!(
            report
                .cases
                .iter()
                .all(|case| !case.kg_would_enter_prompt_context
                    && !case.baseline_would_enter_prompt_context)
        );
        assert!(
            report
                .cases
                .iter()
                .any(|case| case.baseline_kind == MemoryKgShadowRankBaselineKind::Transcript)
        );
        assert!(
            report
                .cases
                .iter()
                .any(|case| case.baseline_kind == MemoryKgShadowRankBaselineKind::DurableMemory)
        );
        assert!(
            report
                .cases
                .iter()
                .all(|case| case.kg_score_delta_basis_points > 0)
        );
        assert!(
            report
                .cases
                .iter()
                .all(|case| case.baseline_source_id.contains("-baseline:"))
        );
    }

    #[test]
    fn memory_kg_shadow_rank_drift_report_gates_rank_and_delta_stability() {
        let atom_report = memory_atom_pipeline_sample_report(true);
        let report = memory_kg_shadow_rank_drift_report(&atom_report.atoms, true);

        assert_eq!(report.status, "ready");
        assert_eq!(report.verdict, "stable");
        assert_eq!(report.contract, MEMORY_KG_SHADOW_RANK_DRIFT_V0_CONTRACT);
        assert_eq!(
            report.kg_shadow_rank_comparison_contract,
            MEMORY_KG_SHADOW_RANK_COMPARISON_V0_CONTRACT
        );
        assert_eq!(
            report.kg_shadow_rank_contract,
            MEMORY_KG_SHADOW_RANK_V0_CONTRACT
        );
        assert_eq!(report.top_n_limit, 6);
        assert!(report.kg_ranked_item_count > 0);
        assert!(report.top_n_kg_rank_count > 0);
        assert_eq!(
            report.expected_drift_case_count,
            report.top_n_kg_rank_count * 2
        );
        assert_eq!(report.drift_case_count, report.expected_drift_case_count);
        assert_eq!(report.stable_case_count, report.drift_case_count);
        assert_eq!(report.drifted_case_count, 0);
        assert_eq!(report.transcript_case_count, report.top_n_kg_rank_count);
        assert_eq!(report.durable_memory_case_count, report.top_n_kg_rank_count);
        assert!(
            report.max_observed_score_delta_basis_points
                <= report.durable_memory_delta_threshold_basis_points
        );
        assert_eq!(report.transcript_delta_threshold_basis_points, 250);
        assert_eq!(report.durable_memory_delta_threshold_basis_points, 500);
        assert!(!report.prompt_preview_rendered);
        assert!(!report.model_invoked);
        assert!(!report.context_injection_performed);
        assert_eq!(report.external_read_enabled_count, 0);
        assert_eq!(report.network_call_enabled_count, 0);
        assert_eq!(report.live_write_enabled_count, 0);
        assert!(report.checks.ready());
        assert!(report.checks.comparison_ready);
        assert!(report.checks.top_n_cases_nonzero);
        assert!(report.checks.top_n_coverage_complete);
        assert!(report.checks.baseline_kind_coverage_stable);
        assert!(report.checks.rank_order_stable);
        assert!(report.checks.score_delta_within_thresholds);
        assert!(report.checks.prompt_flags_stable);
        assert!(report.checks.no_context_injection_performed);
        assert!(report.cases.iter().all(|case| case.stable));
        assert!(report.cases.iter().all(|case| case.rank_delta == 0));
        assert!(
            report
                .cases
                .iter()
                .all(|case| case.score_delta_within_threshold && case.prompt_flags_stable)
        );
    }

    #[test]
    fn memory_kg_prompt_preview_approval_packet_blocks_prompt_preview_until_operator_approval() {
        let atom_report = memory_atom_pipeline_sample_report(true);
        let report = memory_kg_prompt_preview_approval_packet_report(&atom_report.atoms, true);

        assert_eq!(report.status, "blocked");
        assert_eq!(
            report.verdict,
            "blocked_until_operator_prompt_preview_approval_rollback_and_kill_switch"
        );
        assert_eq!(
            report.contract,
            MEMORY_KG_PROMPT_PREVIEW_APPROVAL_PACKET_V0_CONTRACT
        );
        assert_eq!(
            report.kg_shadow_rank_drift_contract,
            MEMORY_KG_SHADOW_RANK_DRIFT_V0_CONTRACT
        );
        assert_eq!(
            report.kg_context_injection_readiness_contract,
            MEMORY_KG_CONTEXT_INJECTION_READINESS_V0_CONTRACT
        );
        assert_eq!(
            report.approval_packet_mode,
            "draft_redacted_refs_only_no_prompt_preview"
        );
        assert!(report.drift_case_count > 0);
        assert_eq!(report.approval_item_count, report.drift_case_count);
        assert_eq!(
            report.redacted_context_ref_count,
            report.approval_item_count
        );
        assert_eq!(report.stable_case_count, report.drift_case_count);
        assert_eq!(report.drifted_case_count, 0);
        assert!(!report.operator_approval_recorded);
        assert!(!report.rollback_plan_ready);
        assert!(!report.kill_switch_ready);
        assert!(!report.approval_packet_accepted);
        assert!(!report.prompt_preview_allowed);
        assert!(!report.prompt_preview_rendered);
        assert!(!report.prompt_payload_materialized);
        assert!(!report.context_injection_allowed);
        assert!(!report.context_injection_performed);
        assert!(!report.model_invoked);
        assert_eq!(report.external_read_enabled_count, 0);
        assert_eq!(report.network_call_enabled_count, 0);
        assert_eq!(report.live_write_enabled_count, 0);
        assert!(report.checks.ready());
        assert!(report.checks.drift_gate_stable);
        assert!(report.checks.approval_items_cover_drift_cases);
        assert!(report.checks.redacted_refs_present);
        assert!(report.checks.operator_approval_required);
        assert!(report.checks.prompt_preview_disabled_by_default);
        assert!(report.checks.prompt_preview_not_rendered);
        assert!(report.checks.prompt_payload_not_materialized);
        assert!(report.checks.context_injection_disabled_by_default);
        assert!(report.checks.no_context_injection_performed);
        assert!(
            report
                .blockers
                .contains(&MemoryKgPromptPreviewApprovalPacketBlocker::MissingOperatorApproval)
        );
        assert!(
            report
                .blockers
                .contains(&MemoryKgPromptPreviewApprovalPacketBlocker::MissingRollbackPlan)
        );
        assert!(
            report
                .blockers
                .contains(&MemoryKgPromptPreviewApprovalPacketBlocker::MissingKillSwitch)
        );
        assert!(report.items.iter().all(|item| {
            !item.prompt_preview_included
                && !item.context_injection_allowed
                && item.operator_approval_required
                && item
                    .redacted_context_ref
                    .starts_with("kg-shadow-rank-drift-ref:")
        }));
    }

    #[test]
    fn memory_kg_prompt_preview_operator_evidence_blocks_until_evidence_is_complete() {
        let atom_report = memory_atom_pipeline_sample_report(true);
        let report = memory_kg_prompt_preview_operator_evidence_report(&atom_report.atoms, true);

        assert_eq!(report.status, "blocked");
        assert_eq!(
            report.verdict,
            "blocked_until_operator_evidence_packet_is_complete_and_signed"
        );
        assert_eq!(
            report.contract,
            MEMORY_KG_PROMPT_PREVIEW_OPERATOR_EVIDENCE_V0_CONTRACT
        );
        assert_eq!(
            report.approval_packet_contract,
            MEMORY_KG_PROMPT_PREVIEW_APPROVAL_PACKET_V0_CONTRACT
        );
        assert_eq!(report.approval_packet_status, "blocked");
        assert_eq!(
            report.evidence_gate_mode,
            "operator_evidence_requirements_only_no_prompt_preview"
        );
        assert!(!report.operator_approval_evidence_present);
        assert!(!report.rollback_plan_evidence_present);
        assert!(!report.kill_switch_evidence_present);
        assert!(!report.reviewer_identity_present);
        assert!(report.reviewer_identity_redacted);
        assert!(!report.approval_timestamp_present);
        assert!(!report.signed_approval_digest_present);
        assert!(!report.bounded_preview_scope_present);
        assert_eq!(report.required_evidence_count, 7);
        assert_eq!(
            report.missing_evidence_count,
            report.required_evidence_count
        );
        assert!(!report.prompt_preview_allowed);
        assert!(!report.prompt_preview_rendered);
        assert!(!report.prompt_payload_materialized);
        assert!(!report.context_injection_allowed);
        assert!(!report.context_injection_performed);
        assert!(!report.model_invoked);
        assert_eq!(report.external_read_enabled_count, 0);
        assert_eq!(report.network_call_enabled_count, 0);
        assert_eq!(report.live_write_enabled_count, 0);
        assert!(report.checks.ready());
        assert!(report.checks.approval_packet_contract_linked);
        assert!(report.checks.approval_packet_checks_ready);
        assert!(report.checks.approval_packet_not_accepted);
        assert!(report.checks.evidence_requirements_all_blocking);
        assert!(report.checks.operator_approval_evidence_required);
        assert!(report.checks.rollback_plan_evidence_required);
        assert!(report.checks.kill_switch_evidence_required);
        assert!(report.checks.reviewer_identity_required);
        assert!(report.checks.signed_approval_digest_required);
        assert!(report.checks.bounded_preview_scope_required);
        assert!(report.checks.prompt_preview_disabled);
        assert!(report.checks.prompt_payload_not_materialized);
        assert!(report.checks.context_injection_disabled);
        assert!(report.checks.no_model_invoked);
        assert!(report.checks.no_context_injection_performed);
        assert!(report.checks.no_external_reads_enabled);
        assert!(report.checks.no_network_calls_enabled);
        assert!(report.checks.no_live_writes_enabled);
        assert!(
            report
                .blockers
                .contains(&MemoryKgPromptPreviewOperatorEvidenceBlocker::ApprovalPacketNotAccepted)
        );
        assert!(report.blockers.contains(
            &MemoryKgPromptPreviewOperatorEvidenceBlocker::MissingOperatorApprovalEvidence
        ));
        assert!(
            report.blockers.contains(
                &MemoryKgPromptPreviewOperatorEvidenceBlocker::MissingSignedApprovalDigest
            )
        );
        assert!(report.requirements.iter().all(|requirement| {
            !requirement.present
                && requirement.blocks_prompt_preview
                && requirement
                    .redacted_evidence_ref
                    .starts_with("missing:kg-prompt-preview-evidence:")
        }));
    }

    #[test]
    fn memory_kg_prompt_preview_redaction_diff_suppresses_raw_prompt_and_payload() {
        let atom_report = memory_atom_pipeline_sample_report(true);
        let report = memory_kg_prompt_preview_redaction_diff_report(&atom_report.atoms, true);

        assert_eq!(report.status, "blocked");
        assert_eq!(
            report.verdict,
            "blocked_until_redacted_diff_review_and_operator_evidence_are_complete"
        );
        assert_eq!(
            report.contract,
            MEMORY_KG_PROMPT_PREVIEW_REDACTION_DIFF_V0_CONTRACT
        );
        assert_eq!(
            report.operator_evidence_contract,
            MEMORY_KG_PROMPT_PREVIEW_OPERATOR_EVIDENCE_V0_CONTRACT
        );
        assert_eq!(report.operator_evidence_status, "blocked");
        assert_eq!(
            report.redaction_diff_mode,
            "redacted_requirement_refs_only_no_prompt_or_payload"
        );
        assert_eq!(report.required_evidence_count, 7);
        assert_eq!(
            report.missing_evidence_count,
            report.required_evidence_count
        );
        assert_eq!(report.diff_item_count, report.required_evidence_count);
        assert_eq!(report.redacted_ref_count, report.diff_item_count);
        assert_eq!(report.raw_prompt_diff_count, 0);
        assert_eq!(report.prompt_text_included_count, 0);
        assert_eq!(report.payload_text_included_count, 0);
        assert!(report.redacted_diff_reported);
        assert!(!report.prompt_preview_allowed);
        assert!(!report.prompt_preview_rendered);
        assert!(!report.prompt_payload_materialized);
        assert!(!report.context_injection_allowed);
        assert!(!report.context_injection_performed);
        assert!(!report.model_invoked);
        assert_eq!(report.external_read_enabled_count, 0);
        assert_eq!(report.network_call_enabled_count, 0);
        assert_eq!(report.live_write_enabled_count, 0);
        assert!(report.checks.ready());
        assert!(report.checks.operator_evidence_contract_linked);
        assert!(report.checks.operator_evidence_checks_ready);
        assert!(report.checks.operator_evidence_missing_requirements);
        assert!(report.checks.redacted_diff_items_nonzero);
        assert!(report.checks.redacted_refs_present);
        assert!(report.checks.redacted_diff_items_cover_requirements);
        assert!(report.checks.raw_prompt_diff_suppressed);
        assert!(report.checks.prompt_text_excluded);
        assert!(report.checks.payload_text_excluded);
        assert!(report.checks.prompt_preview_disabled);
        assert!(report.checks.prompt_payload_not_materialized);
        assert!(report.checks.context_injection_disabled);
        assert!(report.checks.no_model_invoked);
        assert!(report.checks.no_context_injection_performed);
        assert!(report.checks.no_external_reads_enabled);
        assert!(report.checks.no_network_calls_enabled);
        assert!(report.checks.no_live_writes_enabled);
        assert!(
            report
                .blockers
                .contains(&MemoryKgPromptPreviewRedactionDiffBlocker::OperatorEvidenceIncomplete)
        );
        assert!(
            report
                .blockers
                .contains(&MemoryKgPromptPreviewRedactionDiffBlocker::RawPromptDiffSuppressed)
        );
        assert!(report.items.iter().all(|item| {
            !item.raw_before_included
                && !item.raw_after_included
                && !item.prompt_text_included
                && !item.payload_text_included
                && !item.operator_evidence_present
                && item.blocks_prompt_preview
                && item
                    .redacted_before_ref
                    .starts_with("redacted-diff:before:")
                && item.redacted_after_ref.starts_with("redacted-diff:after:")
        }));
    }

    #[test]
    fn memory_kg_prompt_preview_rollback_kill_switch_requires_safety_evidence() {
        let atom_report = memory_atom_pipeline_sample_report(true);
        let report = memory_kg_prompt_preview_rollback_kill_switch_report(&atom_report.atoms, true);

        assert_eq!(report.status, "blocked");
        assert_eq!(
            report.verdict,
            "blocked_until_rollback_plan_and_kill_switch_evidence_are_recorded"
        );
        assert_eq!(
            report.contract,
            MEMORY_KG_PROMPT_PREVIEW_ROLLBACK_KILL_SWITCH_V0_CONTRACT
        );
        assert_eq!(
            report.redaction_diff_contract,
            MEMORY_KG_PROMPT_PREVIEW_REDACTION_DIFF_V0_CONTRACT
        );
        assert_eq!(report.redaction_diff_status, "blocked");
        assert_eq!(
            report.redaction_diff_mode,
            "redacted_requirement_refs_only_no_prompt_or_payload"
        );
        assert_eq!(report.required_evidence_count, 7);
        assert_eq!(
            report.missing_evidence_count,
            report.required_evidence_count
        );
        assert_eq!(report.required_control_count, 4);
        assert_eq!(report.missing_control_count, report.required_control_count);
        assert_eq!(report.rollback_control_count, 2);
        assert_eq!(report.kill_switch_control_count, 2);
        assert!(!report.rollback_plan_ready);
        assert!(!report.rollback_exercise_ready);
        assert!(!report.kill_switch_ready);
        assert!(!report.kill_switch_dry_run_ready);
        assert_eq!(report.redacted_ref_count, report.required_evidence_count);
        assert_eq!(report.raw_prompt_diff_count, 0);
        assert_eq!(report.prompt_text_included_count, 0);
        assert_eq!(report.payload_text_included_count, 0);
        assert!(!report.prompt_preview_allowed);
        assert!(!report.prompt_preview_rendered);
        assert!(!report.prompt_payload_materialized);
        assert!(!report.context_injection_allowed);
        assert!(!report.context_injection_performed);
        assert!(!report.model_invoked);
        assert_eq!(report.external_read_enabled_count, 0);
        assert_eq!(report.network_call_enabled_count, 0);
        assert_eq!(report.live_write_enabled_count, 0);
        assert!(report.checks.ready());
        assert!(report.checks.redaction_diff_contract_linked);
        assert!(report.checks.redaction_diff_checks_ready);
        assert!(report.checks.redaction_diff_blocked);
        assert!(report.checks.only_redacted_refs_reported);
        assert!(report.checks.rollback_controls_nonzero);
        assert!(report.checks.kill_switch_controls_nonzero);
        assert!(report.checks.controls_all_missing_and_blocking);
        assert!(report.checks.rollback_plan_required);
        assert!(report.checks.rollback_exercise_required);
        assert!(report.checks.kill_switch_required);
        assert!(report.checks.kill_switch_dry_run_required);
        assert!(report.checks.prompt_preview_disabled);
        assert!(report.checks.prompt_payload_not_materialized);
        assert!(report.checks.context_injection_disabled);
        assert!(report.checks.no_model_invoked);
        assert!(report.checks.no_context_injection_performed);
        assert!(report.checks.no_external_reads_enabled);
        assert!(report.checks.no_network_calls_enabled);
        assert!(report.checks.no_live_writes_enabled);
        assert!(report.blockers.contains(
            &MemoryKgPromptPreviewRollbackKillSwitchBlocker::RollbackPlanEvidenceMissing
        ));
        assert!(
            report.blockers.contains(
                &MemoryKgPromptPreviewRollbackKillSwitchBlocker::KillSwitchEvidenceMissing
            )
        );
        assert!(report.controls.iter().all(|control| {
            !control.present
                && control.blocks_prompt_preview
                && !control.allows_context_injection
                && control
                    .redacted_evidence_ref
                    .starts_with("missing:kg-prompt-preview-safety:")
        }));
    }

    #[test]
    fn memory_kg_prompt_preview_context_handoff_blocks_injection_until_final_evidence() {
        let atom_report = memory_atom_pipeline_sample_report(true);
        let report = memory_kg_prompt_preview_context_handoff_report(&atom_report.atoms, true);

        assert_eq!(report.status, "blocked");
        assert_eq!(
            report.verdict,
            "blocked_until_operator_evidence_safety_controls_redacted_diff_review_and_context_handoff_approval_exist"
        );
        assert_eq!(
            report.contract,
            MEMORY_KG_PROMPT_PREVIEW_CONTEXT_HANDOFF_V0_CONTRACT
        );
        assert_eq!(
            report.safety_gate_contract,
            MEMORY_KG_PROMPT_PREVIEW_ROLLBACK_KILL_SWITCH_V0_CONTRACT
        );
        assert_eq!(report.safety_gate_status, "blocked");
        assert_eq!(
            report.redaction_diff_contract,
            MEMORY_KG_PROMPT_PREVIEW_REDACTION_DIFF_V0_CONTRACT
        );
        assert_eq!(report.required_evidence_count, 7);
        assert_eq!(
            report.missing_evidence_count,
            report.required_evidence_count
        );
        assert_eq!(report.required_control_count, 4);
        assert_eq!(report.missing_control_count, report.required_control_count);
        assert_eq!(report.handoff_requirement_count, 6);
        assert_eq!(
            report.missing_handoff_requirement_count,
            report.handoff_requirement_count
        );
        assert_eq!(report.redacted_ref_count, report.required_evidence_count);
        assert_eq!(report.raw_prompt_diff_count, 0);
        assert_eq!(report.prompt_text_included_count, 0);
        assert_eq!(report.payload_text_included_count, 0);
        assert!(!report.redacted_diff_review_present);
        assert!(!report.context_handoff_approval_present);
        assert!(!report.prompt_preview_allowed);
        assert!(!report.prompt_preview_rendered);
        assert!(!report.prompt_payload_materialized);
        assert!(!report.context_injection_allowed);
        assert!(!report.context_injection_performed);
        assert!(!report.model_invoked);
        assert_eq!(report.external_read_enabled_count, 0);
        assert_eq!(report.network_call_enabled_count, 0);
        assert_eq!(report.live_write_enabled_count, 0);
        assert!(report.checks.ready());
        assert!(report.checks.safety_gate_contract_linked);
        assert!(report.checks.safety_gate_checks_ready);
        assert!(report.checks.safety_gate_blocked);
        assert!(report.checks.operator_evidence_incomplete);
        assert!(report.checks.safety_controls_incomplete);
        assert!(report.checks.handoff_requirements_nonzero);
        assert!(report.checks.handoff_requirements_all_missing_and_blocking);
        assert!(report.checks.redacted_refs_only);
        assert!(report.checks.redacted_diff_review_required);
        assert!(report.checks.context_handoff_approval_required);
        assert!(report.checks.prompt_preview_disabled);
        assert!(report.checks.prompt_payload_not_materialized);
        assert!(report.checks.context_injection_disabled);
        assert!(report.checks.no_model_invoked);
        assert!(report.checks.no_context_injection_performed);
        assert!(report.checks.no_external_reads_enabled);
        assert!(report.checks.no_network_calls_enabled);
        assert!(report.checks.no_live_writes_enabled);
        assert!(
            report
                .blockers
                .contains(&MemoryKgPromptPreviewContextHandoffBlocker::OperatorEvidenceIncomplete)
        );
        assert!(
            report
                .blockers
                .contains(&MemoryKgPromptPreviewContextHandoffBlocker::SafetyControlsIncomplete)
        );
        assert!(
            report
                .blockers
                .contains(&MemoryKgPromptPreviewContextHandoffBlocker::RedactedDiffReviewMissing)
        );
        assert!(
            report.blockers.contains(
                &MemoryKgPromptPreviewContextHandoffBlocker::ContextHandoffApprovalMissing
            )
        );
        assert!(report.requirements.iter().all(|requirement| {
            !requirement.present
                && requirement.blocks_context_injection
                && requirement
                    .redacted_evidence_ref
                    .starts_with("missing:kg-prompt-preview-context-handoff:")
        }));
    }

    #[test]
    fn memory_kg_prompt_preview_preflight_blocks_ci_promotion_until_gate_chain_closes() {
        let atom_report = memory_atom_pipeline_sample_report(true);
        let report = memory_kg_prompt_preview_preflight_report(&atom_report.atoms, true);

        assert_eq!(report.status, "blocked");
        assert_eq!(
            report.verdict,
            "blocked_until_prompt_preview_gate_chain_evidence_review_approval_and_ci_promotion_exist"
        );
        assert_eq!(
            report.contract,
            MEMORY_KG_PROMPT_PREVIEW_PREFLIGHT_V0_CONTRACT
        );
        assert_eq!(
            report.context_handoff_contract,
            MEMORY_KG_PROMPT_PREVIEW_CONTEXT_HANDOFF_V0_CONTRACT
        );
        assert_eq!(report.context_handoff_status, "blocked");
        assert_eq!(report.source_gate_count, 5);
        assert_eq!(report.ready_source_gate_count, report.source_gate_count);
        assert_eq!(report.blocked_source_gate_count, report.source_gate_count);
        assert_eq!(
            report.report_only_source_gate_count,
            report.source_gate_count
        );
        assert_eq!(report.required_operator_evidence_count, 7);
        assert_eq!(
            report.missing_operator_evidence_count,
            report.required_operator_evidence_count
        );
        assert_eq!(report.required_safety_control_count, 4);
        assert_eq!(
            report.missing_safety_control_count,
            report.required_safety_control_count
        );
        assert_eq!(report.required_handoff_requirement_count, 6);
        assert_eq!(
            report.missing_handoff_requirement_count,
            report.required_handoff_requirement_count
        );
        assert_eq!(report.missing_final_review_approval_count, 2);
        assert_eq!(report.required_total_preflight_requirement_count, 19);
        assert_eq!(
            report.missing_total_preflight_requirement_count,
            report.required_total_preflight_requirement_count
        );
        assert_eq!(
            report.redacted_ref_count,
            report.required_operator_evidence_count
        );
        assert_eq!(report.raw_prompt_diff_count, 0);
        assert_eq!(report.prompt_text_included_count, 0);
        assert_eq!(report.payload_text_included_count, 0);
        assert!(!report.redacted_diff_review_present);
        assert!(!report.context_handoff_approval_present);
        assert!(!report.prompt_preview_allowed);
        assert!(!report.prompt_preview_rendered);
        assert!(!report.prompt_payload_materialized);
        assert!(!report.context_injection_allowed);
        assert!(!report.context_injection_performed);
        assert!(!report.model_invoked);
        assert!(!report.ci_promotion_allowed);
        assert!(!report.preflight_execution_performed);
        assert_eq!(report.external_read_enabled_count, 0);
        assert_eq!(report.network_call_enabled_count, 0);
        assert_eq!(report.live_write_enabled_count, 0);
        assert!(report.checks.ready());
        assert!(report.checks.source_gates_nonzero);
        assert!(report.checks.source_gates_all_linked);
        assert!(report.checks.source_gates_all_checks_ready);
        assert!(report.checks.source_gates_all_blocked);
        assert!(report.checks.source_gates_all_report_only);
        assert!(report.checks.context_handoff_contract_linked);
        assert!(report.checks.context_handoff_checks_ready);
        assert!(report.checks.context_handoff_blocked);
        assert!(report.checks.operator_evidence_incomplete);
        assert!(report.checks.safety_controls_incomplete);
        assert!(report.checks.handoff_requirements_incomplete);
        assert!(report.checks.redacted_diff_review_required);
        assert!(report.checks.context_handoff_approval_required);
        assert!(report.checks.redacted_refs_only);
        assert!(report.checks.prompt_preview_disabled);
        assert!(report.checks.prompt_payload_not_materialized);
        assert!(report.checks.context_injection_disabled);
        assert!(report.checks.no_model_invoked);
        assert!(report.checks.no_context_injection_performed);
        assert!(report.checks.no_external_reads_enabled);
        assert!(report.checks.no_network_calls_enabled);
        assert!(report.checks.no_live_writes_enabled);
        assert!(report.checks.ci_promotion_disabled);
        assert!(report.checks.no_preflight_execution_performed);
        assert!(
            report
                .blockers
                .contains(&MemoryKgPromptPreviewPreflightBlocker::PromptPreviewGateChainBlocked)
        );
        assert!(
            report
                .blockers
                .contains(&MemoryKgPromptPreviewPreflightBlocker::CiPromotionDisabled)
        );
        assert!(report.source_gates.iter().all(|source_gate| {
            source_gate.checks_ready
                && source_gate.status == "blocked"
                && source_gate.blocks_prompt_preview
                && source_gate.blocks_context_injection
                && source_gate.report_only
        }));
    }
}

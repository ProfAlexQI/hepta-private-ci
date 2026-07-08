use super::*;

pub(super) fn context_plane_activation_status_fixture() -> ContextPlaneStatusReport {
    ContextPlaneStatusReport {
        sections: vec![
            ContextPlaneStatusEntry::ready(ContextPlaneStatusSection::SourceRegistry, 1),
            ContextPlaneStatusEntry::shadow(ContextPlaneStatusSection::AdaptiveBudgetAllocation, 1),
            ContextPlaneStatusEntry::ready(ContextPlaneStatusSection::MemoryTaxonomy, 1),
            ContextPlaneStatusEntry::ready(ContextPlaneStatusSection::MemoryFormationReceipts, 1),
            ContextPlaneStatusEntry::ready(ContextPlaneStatusSection::MemoryFormationQueue, 1),
            ContextPlaneStatusEntry::ready(ContextPlaneStatusSection::MemoryTemporalFacts, 1),
            ContextPlaneStatusEntry::ready(ContextPlaneStatusSection::MemoryTemporalFactGraph, 1),
            ContextPlaneStatusEntry::shadow(
                ContextPlaneStatusSection::MemoryTemporalGraphShadowEval,
                4,
            ),
            ContextPlaneStatusEntry::ready(ContextPlaneStatusSection::EvalHarnessSeed, 2),
            ContextPlaneStatusEntry::shadow(
                ContextPlaneStatusSection::AdaptiveAllocatorEvalShadow,
                4,
            ),
            ContextPlaneStatusEntry::ready(ContextPlaneStatusSection::RecallQualityGate, 2),
            ContextPlaneStatusEntry::shadow(ContextPlaneStatusSection::MemoryProviderBoundary, 1),
            {
                let mut entry = ContextPlaneStatusEntry::shadow(
                    ContextPlaneStatusSection::MemoryProviderV2Boundary,
                    6,
                );
                entry.memory_provider_v2_lifecycle_required_count = 6;
                entry.memory_provider_v2_lifecycle_pass_count = 6;
                entry.memory_provider_v2_query_check_pass = true;
                entry.memory_provider_v2_update_context_check_pass = true;
                entry.memory_provider_v2_propose_write_check_pass = true;
                entry.memory_provider_v2_add_check_pass = true;
                entry.memory_provider_v2_clear_check_pass = true;
                entry.memory_provider_v2_close_check_pass = true;
                entry.memory_provider_v2_candidate_count = 1;
                entry.memory_provider_v2_operator_review_required_count = 1;
                entry
            },
            ContextPlaneStatusEntry::shadow(
                ContextPlaneStatusSection::MemoryShadowCanaryReadiness,
                3,
            ),
            {
                let mut entry = ContextPlaneStatusEntry::shadow(
                    ContextPlaneStatusSection::MemoryShadowCanaryPromotionReadiness,
                    9,
                );
                entry.canary_promotion_required_stable_window_count = 1;
                entry.canary_promotion_observed_stable_window_count = 1;
                entry.canary_promotion_required_pass_streak = 3;
                entry.canary_promotion_observed_pass_streak = 3;
                entry.canary_promotion_checklist_required_count = 4;
                entry.canary_promotion_checklist_pass_count = 4;
                entry.canary_promotion_readiness_check_pass = true;
                entry.canary_promotion_negative_rehearsal_check_pass = true;
                entry.canary_promotion_audit_digest_check_pass = true;
                entry.canary_promotion_audit_freshness_check_pass = true;
                entry.canary_promotion_rollback_rehearsal_count = 3;
                entry.canary_promotion_rollback_rehearsal_pass_count = 3;
                entry.canary_promotion_kill_switch_rehearsal_count = 3;
                entry.canary_promotion_kill_switch_rehearsal_pass_count = 3;
                entry.canary_promotion_soak_readback_window_count = 3;
                entry.canary_promotion_soak_readback_pass_count = 3;
                entry
            },
            ContextPlaneStatusEntry::disabled(ContextPlaneStatusSection::SourceAwareFrontDoor),
        ],
        ..ContextPlaneStatusReport::default()
    }
}

#[test]
fn context_plane_activation_blocker_matrix_explains_disabled_runtime_activation() {
    let status = context_plane_activation_status_fixture();
    let matrix = ContextPlaneActivationBlockerMatrix::from_status(&status);

    assert!(matrix.has_matrix_integrity());
    assert_eq!(matrix.rows.len(), 17);
    assert_eq!(matrix.satisfied_count(), 9);
    assert_eq!(matrix.blocker_count, 8);
    assert!(!matrix.activation_allowed);
    assert_eq!(
        matrix.threshold_satisfied(ContextPlaneActivationTarget::SourceRegistry),
        Some(true)
    );
    assert_eq!(
        matrix.threshold_satisfied(ContextPlaneActivationTarget::AdaptiveAllocatorEvalShadow),
        Some(true)
    );
    assert_eq!(
        matrix.threshold_satisfied(ContextPlaneActivationTarget::RecallQualityGate),
        Some(true)
    );
    assert_eq!(
        matrix.blocker_reason(ContextPlaneActivationTarget::AdaptiveBudgetAllocation),
        Some(ContextPlaneActivationBlockerReason::AdaptiveBudgetAllocationShadowOnly)
    );
    assert_eq!(
        matrix.blocker_reason(ContextPlaneActivationTarget::MemoryProviderBoundary),
        Some(ContextPlaneActivationBlockerReason::MemoryProviderBoundaryShadowOnly)
    );
    assert_eq!(
        matrix.blocker_reason(ContextPlaneActivationTarget::MemoryProviderV2Boundary),
        Some(ContextPlaneActivationBlockerReason::MemoryProviderV2BoundaryShadowOnly)
    );
    assert_eq!(
        matrix.blocker_reason(ContextPlaneActivationTarget::MemoryShadowCanaryReadiness),
        Some(ContextPlaneActivationBlockerReason::MemoryShadowCanaryReadinessShadowOnly)
    );
    assert_eq!(
        matrix.blocker_reason(ContextPlaneActivationTarget::MemoryShadowCanaryPromotionReadiness),
        Some(ContextPlaneActivationBlockerReason::MemoryShadowCanaryPromotionReadinessShadowOnly)
    );
    assert_eq!(
        matrix.blocker_reason(ContextPlaneActivationTarget::MemoryTemporalGraphShadowEval),
        Some(ContextPlaneActivationBlockerReason::TemporalGraphShadowEvalShadowOnly)
    );
    assert_eq!(
        matrix.blocker_reason(ContextPlaneActivationTarget::SourceAwareFrontDoor),
        Some(ContextPlaneActivationBlockerReason::SourceAwareFrontDoorDisabled)
    );
    assert_eq!(
        matrix.blocker_reason(ContextPlaneActivationTarget::OperatorApproval),
        Some(ContextPlaneActivationBlockerReason::OperatorApprovalMissing)
    );
    assert!(!matrix.production_write);
    assert!(!matrix.graph_write);
    assert!(!matrix.runtime_activation);
    assert!(!matrix.adaptive_allocator_runtime_activation);
    assert!(!matrix.source_aware_runtime_activation);
    assert!(!matrix.prompt_assembly_change);
    assert!(!matrix.operator_activation_allowed);

    let json = serde_json::to_string(&matrix).expect("activation matrix should serialize");
    assert!(json.contains("source_registry"));
    assert!(json.contains("adaptive_budget_allocation"));
    assert!(json.contains("memory_formation_queue"));
    assert!(json.contains("memory_temporal_graph_shadow_eval"));
    assert!(json.contains("recall_quality_gate"));
    assert!(json.contains("memory_provider_boundary"));
    assert!(json.contains("memory_provider_v2_boundary"));
    assert!(json.contains("memory_provider_v2_lifecycle_pass_count"));
    assert!(json.contains("memory_provider_v2_propose_write_check_pass"));
    assert!(json.contains("memory_provider_v2_close_check_pass"));
    assert!(json.contains("memory_shadow_canary_readiness"));
    assert!(json.contains("memory_shadow_canary_promotion_readiness"));
    assert!(json.contains("recall_quality_blocking_reason_count"));
    assert!(json.contains("recall_quality_blocking_reasons"));
    assert!(json.contains("adaptive_budget_allocation_shadow_only"));
    assert!(json.contains("temporal_graph_shadow_eval_shadow_only"));
    assert!(json.contains("memory_provider_boundary_shadow_only"));
    assert!(json.contains("memory_provider_v2_boundary_shadow_only"));
    assert!(json.contains("memory_shadow_canary_readiness_shadow_only"));
    assert!(json.contains("memory_shadow_canary_promotion_readiness_shadow_only"));
    assert!(json.contains("canary_promotion_checklist_pass_count"));
    assert!(json.contains("canary_promotion_negative_rehearsal_check_pass"));
    assert!(json.contains("canary_promotion_audit_digest_check_pass"));
    assert!(json.contains("canary_promotion_audit_freshness_check_pass"));
    assert!(json.contains("canary_promotion_rollback_rehearsal_pass_count"));
    assert!(json.contains("canary_promotion_kill_switch_rehearsal_pass_count"));
    assert!(json.contains("canary_promotion_soak_readback_pass_count"));
    assert!(json.contains("source_aware_front_door_disabled"));
    assert!(json.contains("operator_approval_missing"));
    assert!(!json.contains("prompt_text"));
    assert!(!json.contains("transcript_text"));
    assert!(!json.contains("memory_text"));
    assert!(!json.contains("answer_text"));
    assert!(!json.contains("source_id"));
    assert!(!json.contains("session-"));
    assert!(!json.contains("memory-"));
    assert!(!json.contains("activation-test"));
    assert!(!json.contains("\"activation_allowed\":true"));
    assert!(!json.contains("\"production_write\":true"));
    assert!(!json.contains("\"graph_write\":true"));
    assert!(!json.contains("\"runtime_activation\":true"));
    assert!(!json.contains("\"adaptive_allocator_runtime_activation\":true"));
    assert!(!json.contains("\"source_aware_runtime_activation\":true"));
    assert!(!json.contains("\"prompt_assembly_change\":true"));
    assert!(!json.contains("\"operator_activation_allowed\":true"));
}

#[test]
fn context_plane_activation_blocker_matrix_rejects_canary_promotion_checklist_false_green() {
    let status = context_plane_activation_status_fixture();
    let matrix = ContextPlaneActivationBlockerMatrix::from_status(&status);
    assert!(matrix.has_matrix_integrity());

    let mut partial_rehearsal = matrix.clone();
    partial_rehearsal
        .rows
        .iter_mut()
        .find(|row| {
            row.target == ContextPlaneActivationTarget::MemoryShadowCanaryPromotionReadiness
        })
        .expect("memory shadow canary promotion readiness activation row should exist")
        .canary_promotion_rollback_rehearsal_pass_count = 2;
    assert!(!partial_rehearsal.has_matrix_integrity());

    let mut blocker_false_green = matrix.clone();
    blocker_false_green
        .rows
        .iter_mut()
        .find(|row| {
            row.target == ContextPlaneActivationTarget::MemoryShadowCanaryPromotionReadiness
        })
        .expect("memory shadow canary promotion readiness activation row should exist")
        .canary_promotion_blocker_count = 1;
    assert!(!blocker_false_green.has_matrix_integrity());

    let mut non_promotion_leak = matrix.clone();
    non_promotion_leak
        .rows
        .iter_mut()
        .find(|row| row.target == ContextPlaneActivationTarget::SourceRegistry)
        .expect("source registry activation row should exist")
        .canary_promotion_checklist_pass_count = 1;
    assert!(!non_promotion_leak.has_matrix_integrity());
}

#[test]
fn context_plane_activation_blocker_matrix_rejects_memory_provider_v2_lifecycle_false_green() {
    let status = context_plane_activation_status_fixture();
    let matrix = ContextPlaneActivationBlockerMatrix::from_status(&status);
    assert!(matrix.has_matrix_integrity());

    let mut partial_lifecycle = matrix.clone();
    partial_lifecycle
        .rows
        .iter_mut()
        .find(|row| row.target == ContextPlaneActivationTarget::MemoryProviderV2Boundary)
        .expect("memory provider v2 activation row should exist")
        .memory_provider_v2_close_check_pass = false;
    assert!(!partial_lifecycle.has_matrix_integrity());

    let mut inflated_pass_count = matrix.clone();
    inflated_pass_count
        .rows
        .iter_mut()
        .find(|row| row.target == ContextPlaneActivationTarget::MemoryProviderV2Boundary)
        .expect("memory provider v2 activation row should exist")
        .memory_provider_v2_lifecycle_pass_count = 7;
    assert!(!inflated_pass_count.has_matrix_integrity());

    let mut non_provider_v2_leak = matrix.clone();
    non_provider_v2_leak
        .rows
        .iter_mut()
        .find(|row| row.target == ContextPlaneActivationTarget::MemoryProviderBoundary)
        .expect("memory provider boundary activation row should exist")
        .memory_provider_v2_lifecycle_pass_count = 1;
    assert!(!non_provider_v2_leak.has_matrix_integrity());
}

#[test]
fn context_plane_activation_blocker_matrix_blocks_side_effect_flags_without_activation() {
    let mut status = context_plane_activation_status_fixture();
    let source_registry = status
        .sections
        .iter_mut()
        .find(|entry| entry.section == ContextPlaneStatusSection::SourceRegistry)
        .expect("source registry status row should exist");
    source_registry.production_write = true;

    let matrix = ContextPlaneActivationBlockerMatrix::from_status(&status);

    assert!(matrix.has_matrix_integrity());
    assert_eq!(matrix.rows.len(), 17);
    assert_eq!(matrix.satisfied_count(), 8);
    assert_eq!(matrix.blocker_count, 9);
    assert_eq!(
        matrix.threshold_satisfied(ContextPlaneActivationTarget::SourceRegistry),
        Some(false)
    );
    assert_eq!(
        matrix.blocker_reason(ContextPlaneActivationTarget::SourceRegistry),
        Some(ContextPlaneActivationBlockerReason::SideEffectFlagEnabled)
    );
    assert_eq!(
        matrix.blocker_reason(ContextPlaneActivationTarget::AdaptiveBudgetAllocation),
        Some(ContextPlaneActivationBlockerReason::AdaptiveBudgetAllocationShadowOnly)
    );
    assert_eq!(
        matrix.blocker_reason(ContextPlaneActivationTarget::MemoryProviderBoundary),
        Some(ContextPlaneActivationBlockerReason::MemoryProviderBoundaryShadowOnly)
    );
    assert_eq!(
        matrix.blocker_reason(ContextPlaneActivationTarget::MemoryShadowCanaryReadiness),
        Some(ContextPlaneActivationBlockerReason::MemoryShadowCanaryReadinessShadowOnly)
    );
    assert_eq!(
        matrix.blocker_reason(ContextPlaneActivationTarget::MemoryShadowCanaryPromotionReadiness),
        Some(ContextPlaneActivationBlockerReason::MemoryShadowCanaryPromotionReadinessShadowOnly)
    );
    assert_eq!(
        matrix.blocker_reason(ContextPlaneActivationTarget::MemoryTemporalGraphShadowEval),
        Some(ContextPlaneActivationBlockerReason::TemporalGraphShadowEvalShadowOnly)
    );
    assert_eq!(
        matrix.blocker_reason(ContextPlaneActivationTarget::SourceAwareFrontDoor),
        Some(ContextPlaneActivationBlockerReason::SourceAwareFrontDoorDisabled)
    );
    assert_eq!(
        matrix.blocker_reason(ContextPlaneActivationTarget::OperatorApproval),
        Some(ContextPlaneActivationBlockerReason::OperatorApprovalMissing)
    );
    assert!(!matrix.activation_allowed);
    assert!(!matrix.production_write);
    assert!(!matrix.graph_write);
    assert!(!matrix.runtime_activation);
    assert!(!matrix.adaptive_allocator_runtime_activation);
    assert!(!matrix.source_aware_runtime_activation);
    assert!(!matrix.prompt_assembly_change);
    assert!(!matrix.operator_activation_allowed);

    let json = serde_json::to_string(&matrix).expect("side-effect blocker matrix should serialize");
    assert!(json.contains("side_effect_flag_enabled"));
    assert!(!json.contains("prompt_text"));
    assert!(!json.contains("transcript_text"));
    assert!(!json.contains("memory_text"));
    assert!(!json.contains("answer_text"));
    assert!(!json.contains("source_id"));
    assert!(!json.contains("\"activation_allowed\":true"));
    assert!(!json.contains("\"production_write\":true"));
    assert!(!json.contains("\"graph_write\":true"));
    assert!(!json.contains("\"runtime_activation\":true"));
    assert!(!json.contains("\"adaptive_allocator_runtime_activation\":true"));
    assert!(!json.contains("\"source_aware_runtime_activation\":true"));
    assert!(!json.contains("\"prompt_assembly_change\":true"));
    assert!(!json.contains("\"operator_activation_allowed\":true"));

    let mut report_side_effect_status = context_plane_activation_status_fixture();
    report_side_effect_status.runtime_activation = true;
    let report_side_effect_matrix =
        ContextPlaneActivationBlockerMatrix::from_status(&report_side_effect_status);

    assert!(report_side_effect_matrix.has_matrix_integrity());
    assert_eq!(report_side_effect_matrix.satisfied_count(), 0);
    assert_eq!(report_side_effect_matrix.blocker_count, 17);
    assert_eq!(
        report_side_effect_matrix.blocker_reason(ContextPlaneActivationTarget::RecallQualityGate),
        Some(ContextPlaneActivationBlockerReason::SideEffectFlagEnabled)
    );
    assert_eq!(
        report_side_effect_matrix.blocker_reason(ContextPlaneActivationTarget::OperatorApproval),
        Some(ContextPlaneActivationBlockerReason::OperatorApprovalMissing)
    );
    assert!(!report_side_effect_matrix.runtime_activation);
    assert!(!report_side_effect_matrix.activation_allowed);
}

#[test]
fn context_plane_activation_blocker_matrix_rolls_up_recall_quality_blockers_without_payloads() {
    let mut status = context_plane_activation_status_fixture();
    let recall_quality_entry = status
        .sections
        .iter_mut()
        .find(|entry| entry.section == ContextPlaneStatusSection::RecallQualityGate)
        .expect("recall quality status row should exist");
    recall_quality_entry.status = ContextPlaneStatusKind::Blocked;
    recall_quality_entry.blocker_count = 2;
    recall_quality_entry.recall_quality_blocking_reason_count = 2;
    recall_quality_entry.recall_quality_blocking_reasons = vec![
        ContextMemoryRecallQualityGateBlockerReason::AnswerQualityRegression,
        ContextMemoryRecallQualityGateBlockerReason::SideEffectFlagEnabled,
    ];
    recall_quality_entry.prompt_assembly_change = true;

    let matrix = ContextPlaneActivationBlockerMatrix::from_status(&status);

    assert!(matrix.has_matrix_integrity());
    assert_eq!(matrix.rows.len(), 17);
    assert_eq!(matrix.satisfied_count(), 8);
    assert_eq!(matrix.blocker_count, 9);
    let recall_quality_row = matrix
        .row_for_target(ContextPlaneActivationTarget::RecallQualityGate)
        .expect("recall quality activation row should exist");
    assert_eq!(
        recall_quality_row.blocker_reason,
        ContextPlaneActivationBlockerReason::SideEffectFlagEnabled
    );
    assert_eq!(recall_quality_row.recall_quality_blocking_reason_count, 2);
    assert_eq!(
        recall_quality_row.recall_quality_blocking_reasons,
        vec![
            ContextMemoryRecallQualityGateBlockerReason::AnswerQualityRegression,
            ContextMemoryRecallQualityGateBlockerReason::SideEffectFlagEnabled,
        ]
    );
    assert!(
        matrix
            .rows
            .iter()
            .filter(|row| row.target != ContextPlaneActivationTarget::RecallQualityGate)
            .all(|row| row.recall_quality_blocking_reasons.is_empty())
    );
    assert!(!matrix.activation_allowed);
    assert!(!matrix.production_write);
    assert!(!matrix.graph_write);
    assert!(!matrix.runtime_activation);
    assert!(!matrix.adaptive_allocator_runtime_activation);
    assert!(!matrix.source_aware_runtime_activation);
    assert!(!matrix.prompt_assembly_change);
    assert!(!matrix.operator_activation_allowed);

    let json =
        serde_json::to_string(&matrix).expect("recall-quality blocker matrix should serialize");
    assert!(json.contains("recall_quality_blocking_reason_count"));
    assert!(json.contains("recall_quality_blocking_reasons"));
    assert!(json.contains("answer_quality_regression"));
    assert!(json.contains("side_effect_flag_enabled"));
    assert!(!json.contains("fixture_id_hash"));
    assert!(!json.contains("prompt_text"));
    assert!(!json.contains("transcript_text"));
    assert!(!json.contains("memory_text"));
    assert!(!json.contains("answer_text"));
    assert!(!json.contains("source_id"));
    assert!(!json.contains("session-"));
    assert!(!json.contains("memory-"));
    assert!(!json.contains("activation-test"));
    assert!(!json.contains("\"activation_allowed\":true"));
    assert!(!json.contains("\"production_write\":true"));
    assert!(!json.contains("\"graph_write\":true"));
    assert!(!json.contains("\"runtime_activation\":true"));
    assert!(!json.contains("\"adaptive_allocator_runtime_activation\":true"));
    assert!(!json.contains("\"source_aware_runtime_activation\":true"));
    assert!(!json.contains("\"prompt_assembly_change\":true"));
    assert!(!json.contains("\"operator_activation_allowed\":true"));
}

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
            ContextPlaneStatusEntry::ready(ContextPlaneStatusSection::EvalHarnessSeed, 2),
            ContextPlaneStatusEntry::shadow(
                ContextPlaneStatusSection::AdaptiveAllocatorEvalShadow,
                4,
            ),
            ContextPlaneStatusEntry::ready(ContextPlaneStatusSection::RecallQualityGate, 2),
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
    assert_eq!(matrix.rows.len(), 12);
    assert_eq!(matrix.satisfied_count(), 9);
    assert_eq!(matrix.blocker_count, 3);
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
    assert!(json.contains("recall_quality_gate"));
    assert!(json.contains("recall_quality_blocking_reason_count"));
    assert!(json.contains("recall_quality_blocking_reasons"));
    assert!(json.contains("adaptive_budget_allocation_shadow_only"));
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
    assert_eq!(matrix.rows.len(), 12);
    assert_eq!(matrix.satisfied_count(), 8);
    assert_eq!(matrix.blocker_count, 4);
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
    assert_eq!(report_side_effect_matrix.blocker_count, 12);
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
    assert_eq!(matrix.rows.len(), 12);
    assert_eq!(matrix.satisfied_count(), 8);
    assert_eq!(matrix.blocker_count, 4);
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

use serde::Deserialize;
use serde::Serialize;

use super::super::super::CONTEXT_PLANE_ACTIVATION_BLOCKER_SCHEMA_VERSION;
use super::super::status::ContextPlaneStatusKind;
use super::super::status::ContextPlaneStatusReport;
use super::super::status::ContextPlaneStatusSection;
use super::super::status::context_plane_status_report_has_side_effect_flag;
use super::super::status::status_entry_for_section;
use super::row::ContextPlaneActivationBlockerRow;
use super::target::ContextPlaneActivationBlockerReason;
use super::target::ContextPlaneActivationTarget;
use super::target::activation_target_order;

/// Observational activation-blocker matrix for context-plane promotion.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ContextPlaneActivationBlockerMatrix {
    pub schema_version: u32,
    pub rows: Vec<ContextPlaneActivationBlockerRow>,
    pub blocker_count: usize,
    pub activation_allowed: bool,
    pub production_write: bool,
    pub graph_write: bool,
    pub runtime_activation: bool,
    pub adaptive_allocator_runtime_activation: bool,
    pub source_aware_runtime_activation: bool,
    pub prompt_assembly_change: bool,
    pub operator_activation_allowed: bool,
}

impl Default for ContextPlaneActivationBlockerMatrix {
    fn default() -> Self {
        Self {
            schema_version: CONTEXT_PLANE_ACTIVATION_BLOCKER_SCHEMA_VERSION,
            rows: Vec::new(),
            blocker_count: 0,
            activation_allowed: false,
            production_write: false,
            graph_write: false,
            runtime_activation: false,
            adaptive_allocator_runtime_activation: false,
            source_aware_runtime_activation: false,
            prompt_assembly_change: false,
            operator_activation_allowed: false,
        }
    }
}

impl ContextPlaneActivationBlockerMatrix {
    pub fn from_status(status: &ContextPlaneStatusReport) -> Self {
        let report_side_effect_flag_enabled =
            context_plane_status_report_has_side_effect_flag(status);
        let mut rows = vec![
            row_from_section(
                ContextPlaneActivationTarget::SourceRegistry,
                status,
                ContextPlaneStatusSection::SourceRegistry,
                ContextPlaneStatusKind::Ready,
                report_side_effect_flag_enabled,
            ),
            row_from_section(
                ContextPlaneActivationTarget::AdaptiveBudgetAllocation,
                status,
                ContextPlaneStatusSection::AdaptiveBudgetAllocation,
                ContextPlaneStatusKind::Ready,
                report_side_effect_flag_enabled,
            ),
            row_from_section(
                ContextPlaneActivationTarget::MemoryTaxonomy,
                status,
                ContextPlaneStatusSection::MemoryTaxonomy,
                ContextPlaneStatusKind::Ready,
                report_side_effect_flag_enabled,
            ),
            row_from_section(
                ContextPlaneActivationTarget::MemoryFormationReceipts,
                status,
                ContextPlaneStatusSection::MemoryFormationReceipts,
                ContextPlaneStatusKind::Ready,
                report_side_effect_flag_enabled,
            ),
            row_from_section(
                ContextPlaneActivationTarget::MemoryFormationQueue,
                status,
                ContextPlaneStatusSection::MemoryFormationQueue,
                ContextPlaneStatusKind::Ready,
                report_side_effect_flag_enabled,
            ),
            row_from_section(
                ContextPlaneActivationTarget::MemoryNamespacePolicy,
                status,
                ContextPlaneStatusSection::MemoryNamespacePolicy,
                ContextPlaneStatusKind::Ready,
                report_side_effect_flag_enabled,
            ),
            row_from_section(
                ContextPlaneActivationTarget::MemoryWriteChainReadiness,
                status,
                ContextPlaneStatusSection::MemoryWriteChainReadiness,
                ContextPlaneStatusKind::Ready,
                report_side_effect_flag_enabled,
            ),
            row_from_section(
                ContextPlaneActivationTarget::MemoryWriteChainReceiptFreshness,
                status,
                ContextPlaneStatusSection::MemoryWriteChainReceiptFreshness,
                ContextPlaneStatusKind::Ready,
                report_side_effect_flag_enabled,
            ),
            row_from_section(
                ContextPlaneActivationTarget::MemoryTemporalFacts,
                status,
                ContextPlaneStatusSection::MemoryTemporalFacts,
                ContextPlaneStatusKind::Ready,
                report_side_effect_flag_enabled,
            ),
            row_from_section(
                ContextPlaneActivationTarget::MemoryTemporalFactGraph,
                status,
                ContextPlaneStatusSection::MemoryTemporalFactGraph,
                ContextPlaneStatusKind::Ready,
                report_side_effect_flag_enabled,
            ),
            row_from_section(
                ContextPlaneActivationTarget::MemoryTemporalGraphShadowEval,
                status,
                ContextPlaneStatusSection::MemoryTemporalGraphShadowEval,
                ContextPlaneStatusKind::Ready,
                report_side_effect_flag_enabled,
            ),
            row_from_section(
                ContextPlaneActivationTarget::MemoryTemporalGraphShadowStore,
                status,
                ContextPlaneStatusSection::MemoryTemporalGraphShadowStore,
                ContextPlaneStatusKind::Ready,
                report_side_effect_flag_enabled,
            ),
            row_from_section(
                ContextPlaneActivationTarget::MemoryTemporalGraphShadowReplay,
                status,
                ContextPlaneStatusSection::MemoryTemporalGraphShadowReplay,
                ContextPlaneStatusKind::Ready,
                report_side_effect_flag_enabled,
            ),
            row_from_section(
                ContextPlaneActivationTarget::MemoryTemporalGraphShadowTraversalDiff,
                status,
                ContextPlaneStatusSection::MemoryTemporalGraphShadowTraversalDiff,
                ContextPlaneStatusKind::Ready,
                report_side_effect_flag_enabled,
            ),
            row_from_section(
                ContextPlaneActivationTarget::MemoryTemporalGraphShadowTraversalQuality,
                status,
                ContextPlaneStatusSection::MemoryTemporalGraphShadowTraversalQuality,
                ContextPlaneStatusKind::Ready,
                report_side_effect_flag_enabled,
            ),
            row_from_section(
                ContextPlaneActivationTarget::MemoryTemporalGraphShadowRetrievalCanaryGuard,
                status,
                ContextPlaneStatusSection::MemoryTemporalGraphShadowRetrievalCanaryGuard,
                ContextPlaneStatusKind::Ready,
                report_side_effect_flag_enabled,
            ),
            row_from_section(
                ContextPlaneActivationTarget::MemoryTemporalGraphShadowRetrievalRollbackKillSwitch,
                status,
                ContextPlaneStatusSection::MemoryTemporalGraphShadowRetrievalRollbackKillSwitch,
                ContextPlaneStatusKind::Ready,
                report_side_effect_flag_enabled,
            ),
            row_from_section(
                ContextPlaneActivationTarget::EvalHarnessSeed,
                status,
                ContextPlaneStatusSection::EvalHarnessSeed,
                ContextPlaneStatusKind::Ready,
                report_side_effect_flag_enabled,
            ),
            row_from_section(
                ContextPlaneActivationTarget::AdaptiveAllocatorEvalShadow,
                status,
                ContextPlaneStatusSection::AdaptiveAllocatorEvalShadow,
                ContextPlaneStatusKind::Shadow,
                report_side_effect_flag_enabled,
            ),
            row_from_section(
                ContextPlaneActivationTarget::RecallQualityGate,
                status,
                ContextPlaneStatusSection::RecallQualityGate,
                ContextPlaneStatusKind::Ready,
                report_side_effect_flag_enabled,
            ),
            row_from_section(
                ContextPlaneActivationTarget::MemoryRankedRecallShadowEval,
                status,
                ContextPlaneStatusSection::MemoryRankedRecallShadowEval,
                ContextPlaneStatusKind::Ready,
                report_side_effect_flag_enabled,
            ),
            row_from_section(
                ContextPlaneActivationTarget::MemoryProviderBoundary,
                status,
                ContextPlaneStatusSection::MemoryProviderBoundary,
                ContextPlaneStatusKind::Ready,
                report_side_effect_flag_enabled,
            ),
            row_from_section(
                ContextPlaneActivationTarget::MemoryProviderV2Boundary,
                status,
                ContextPlaneStatusSection::MemoryProviderV2Boundary,
                ContextPlaneStatusKind::Ready,
                report_side_effect_flag_enabled,
            ),
            row_from_section(
                ContextPlaneActivationTarget::MemoryShadowCanaryReadiness,
                status,
                ContextPlaneStatusSection::MemoryShadowCanaryReadiness,
                ContextPlaneStatusKind::Ready,
                report_side_effect_flag_enabled,
            ),
            row_from_section(
                ContextPlaneActivationTarget::MemoryShadowCanaryPromotionReadiness,
                status,
                ContextPlaneStatusSection::MemoryShadowCanaryPromotionReadiness,
                ContextPlaneStatusKind::Ready,
                report_side_effect_flag_enabled,
            ),
            row_from_section(
                ContextPlaneActivationTarget::SourceAwareFrontDoor,
                status,
                ContextPlaneStatusSection::SourceAwareFrontDoor,
                ContextPlaneStatusKind::Ready,
                report_side_effect_flag_enabled,
            ),
            ContextPlaneActivationBlockerRow::blocked(
                ContextPlaneActivationTarget::OperatorApproval,
                ContextPlaneStatusKind::Disabled,
                ContextPlaneStatusKind::Ready,
                ContextPlaneActivationBlockerReason::OperatorApprovalMissing,
            ),
        ];
        rows.sort_by_key(|row| activation_target_order(row.target));
        let blocker_count = rows
            .iter()
            .filter(|row| row.blocker_reason.is_blocking())
            .count();

        Self {
            rows,
            blocker_count,
            ..Self::default()
        }
    }

    pub fn has_matrix_integrity(&self) -> bool {
        self.schema_version == CONTEXT_PLANE_ACTIVATION_BLOCKER_SCHEMA_VERSION
            && self.rows.len() == 27
            && self.has_required_targets()
            && self.blocker_count
                == self
                    .rows
                    .iter()
                    .filter(|row| row.blocker_reason.is_blocking())
                    .count()
            && self
                .rows
                .iter()
                .all(ContextPlaneActivationBlockerRow::has_row_integrity)
            && !self.activation_allowed
            && !self.production_write
            && !self.graph_write
            && !self.runtime_activation
            && !self.adaptive_allocator_runtime_activation
            && !self.source_aware_runtime_activation
            && !self.prompt_assembly_change
            && !self.operator_activation_allowed
    }

    fn has_required_targets(&self) -> bool {
        [
            ContextPlaneActivationTarget::SourceRegistry,
            ContextPlaneActivationTarget::AdaptiveBudgetAllocation,
            ContextPlaneActivationTarget::MemoryTaxonomy,
            ContextPlaneActivationTarget::MemoryFormationReceipts,
            ContextPlaneActivationTarget::MemoryFormationQueue,
            ContextPlaneActivationTarget::MemoryNamespacePolicy,
            ContextPlaneActivationTarget::MemoryWriteChainReadiness,
            ContextPlaneActivationTarget::MemoryWriteChainReceiptFreshness,
            ContextPlaneActivationTarget::MemoryTemporalFacts,
            ContextPlaneActivationTarget::MemoryTemporalFactGraph,
            ContextPlaneActivationTarget::MemoryTemporalGraphShadowEval,
            ContextPlaneActivationTarget::MemoryTemporalGraphShadowStore,
            ContextPlaneActivationTarget::MemoryTemporalGraphShadowReplay,
            ContextPlaneActivationTarget::MemoryTemporalGraphShadowTraversalDiff,
            ContextPlaneActivationTarget::MemoryTemporalGraphShadowTraversalQuality,
            ContextPlaneActivationTarget::MemoryTemporalGraphShadowRetrievalCanaryGuard,
            ContextPlaneActivationTarget::MemoryTemporalGraphShadowRetrievalRollbackKillSwitch,
            ContextPlaneActivationTarget::EvalHarnessSeed,
            ContextPlaneActivationTarget::AdaptiveAllocatorEvalShadow,
            ContextPlaneActivationTarget::RecallQualityGate,
            ContextPlaneActivationTarget::MemoryRankedRecallShadowEval,
            ContextPlaneActivationTarget::MemoryProviderBoundary,
            ContextPlaneActivationTarget::MemoryProviderV2Boundary,
            ContextPlaneActivationTarget::MemoryShadowCanaryReadiness,
            ContextPlaneActivationTarget::MemoryShadowCanaryPromotionReadiness,
            ContextPlaneActivationTarget::SourceAwareFrontDoor,
            ContextPlaneActivationTarget::OperatorApproval,
        ]
        .into_iter()
        .all(|target| self.row_for_target(target).is_some())
    }

    pub fn row_for_target(
        &self,
        target: ContextPlaneActivationTarget,
    ) -> Option<&ContextPlaneActivationBlockerRow> {
        self.rows.iter().find(|row| row.target == target)
    }

    pub fn blocker_reason(
        &self,
        target: ContextPlaneActivationTarget,
    ) -> Option<ContextPlaneActivationBlockerReason> {
        self.row_for_target(target).map(|row| row.blocker_reason)
    }

    pub fn threshold_satisfied(&self, target: ContextPlaneActivationTarget) -> Option<bool> {
        self.row_for_target(target)
            .map(|row| row.threshold_satisfied)
    }

    pub fn satisfied_count(&self) -> usize {
        self.rows
            .iter()
            .filter(|row| row.threshold_satisfied)
            .count()
    }
}

fn row_from_section(
    target: ContextPlaneActivationTarget,
    status: &ContextPlaneStatusReport,
    section: ContextPlaneStatusSection,
    required_status: ContextPlaneStatusKind,
    report_side_effect_flag_enabled: bool,
) -> ContextPlaneActivationBlockerRow {
    ContextPlaneActivationBlockerRow::from_status_entry(
        target,
        status_entry_for_section(status, section),
        required_status,
        report_side_effect_flag_enabled,
    )
}

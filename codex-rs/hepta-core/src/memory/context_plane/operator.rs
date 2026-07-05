use std::collections::BTreeMap;

use serde::Deserialize;
use serde::Serialize;

use super::super::CONTEXT_PLANE_OPERATOR_APPROVAL_PACKET_SCHEMA_VERSION;
use super::super::ContextMemoryRecallQualityGateBlockerReason;
use super::activation::ContextPlaneActivationBlockerMatrix;
use super::activation::ContextPlaneActivationBlockerReason;
use super::activation::ContextPlaneActivationTarget;
use super::activation::activation_blocker_reason_order;
use super::status::ContextPlaneStatusKind;

/// Approval scope that must be covered before any context-plane promotion.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextPlaneOperatorApprovalScope {
    AdaptiveBudgetAllocationRuntime,
    SourceAwareRuntimeActivation,
    ProductionMemoryWrite,
    GraphWrite,
    PromptAssemblyChange,
    OperatorActivation,
    #[default]
    Unknown,
}

impl ContextPlaneOperatorApprovalScope {
    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
}

/// Count of a blocker reason in a payload-light operator approval packet.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct ContextPlaneOperatorApprovalBlockerReasonCount {
    pub reason: ContextPlaneActivationBlockerReason,
    pub count: usize,
}

impl ContextPlaneOperatorApprovalBlockerReasonCount {
    pub fn has_count_integrity(&self) -> bool {
        !self.reason.is_unknown() && self.reason.is_blocking() && self.count > 0
    }
}

/// Count of one recall-quality blocker reason in an operator approval dry-run.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContextPlaneOperatorApprovalRecallQualityBlockerReasonCount {
    pub reason: ContextMemoryRecallQualityGateBlockerReason,
    pub count: usize,
}

impl ContextPlaneOperatorApprovalRecallQualityBlockerReasonCount {
    pub fn has_count_integrity(&self) -> bool {
        self.count > 0
    }
}

/// Aggregated readiness thresholds captured for an operator approval dry-run.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct ContextPlaneOperatorApprovalThresholdSnapshot {
    pub total_row_count: usize,
    pub threshold_satisfied_count: usize,
    pub blocker_count: usize,
    pub required_ready_count: usize,
    pub required_shadow_count: usize,
}

impl ContextPlaneOperatorApprovalThresholdSnapshot {
    pub fn from_matrix(matrix: &ContextPlaneActivationBlockerMatrix) -> Self {
        Self {
            total_row_count: matrix.rows.len(),
            threshold_satisfied_count: matrix.satisfied_count(),
            blocker_count: matrix.blocker_count,
            required_ready_count: matrix
                .rows
                .iter()
                .filter(|row| row.required_status == ContextPlaneStatusKind::Ready)
                .count(),
            required_shadow_count: matrix
                .rows
                .iter()
                .filter(|row| row.required_status == ContextPlaneStatusKind::Shadow)
                .count(),
        }
    }

    pub fn has_snapshot_integrity(&self) -> bool {
        self.total_row_count == 12
            && self.threshold_satisfied_count + self.blocker_count == self.total_row_count
            && self.required_ready_count + self.required_shadow_count == self.total_row_count
    }
}

/// Payload-light dry-run packet describing what approval would be required.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct ContextPlaneOperatorApprovalPacket {
    pub schema_version: u32,
    pub dry_run_only: bool,
    pub approval_required: bool,
    pub activation_command_present: bool,
    pub matrix_row_count: usize,
    pub threshold_satisfied_count: usize,
    pub blocker_count: usize,
    pub threshold_snapshot: ContextPlaneOperatorApprovalThresholdSnapshot,
    pub blocker_reason_counts: Vec<ContextPlaneOperatorApprovalBlockerReasonCount>,
    pub recall_quality_blocking_reason_count: usize,
    pub recall_quality_blocking_reason_counts:
        Vec<ContextPlaneOperatorApprovalRecallQualityBlockerReasonCount>,
    pub required_approval_scopes: Vec<ContextPlaneOperatorApprovalScope>,
    pub production_write: bool,
    pub graph_write: bool,
    pub runtime_activation: bool,
    pub adaptive_allocator_runtime_activation: bool,
    pub source_aware_runtime_activation: bool,
    pub prompt_assembly_change: bool,
    pub operator_activation_allowed: bool,
}

impl Default for ContextPlaneOperatorApprovalPacket {
    fn default() -> Self {
        Self {
            schema_version: CONTEXT_PLANE_OPERATOR_APPROVAL_PACKET_SCHEMA_VERSION,
            dry_run_only: true,
            approval_required: true,
            activation_command_present: false,
            matrix_row_count: 0,
            threshold_satisfied_count: 0,
            blocker_count: 0,
            threshold_snapshot: ContextPlaneOperatorApprovalThresholdSnapshot::default(),
            blocker_reason_counts: Vec::new(),
            recall_quality_blocking_reason_count: 0,
            recall_quality_blocking_reason_counts: Vec::new(),
            required_approval_scopes: Vec::new(),
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

impl ContextPlaneOperatorApprovalPacket {
    pub fn from_matrix(matrix: &ContextPlaneActivationBlockerMatrix) -> Self {
        let mut blocker_reason_counts = BTreeMap::new();
        let mut recall_quality_blocking_reason_counts = BTreeMap::new();
        for row in matrix
            .rows
            .iter()
            .filter(|row| row.blocker_reason.is_blocking())
        {
            *blocker_reason_counts.entry(row.blocker_reason).or_insert(0) += 1;
            if row.target == ContextPlaneActivationTarget::RecallQualityGate {
                for reason in &row.recall_quality_blocking_reasons {
                    *recall_quality_blocking_reason_counts
                        .entry(*reason)
                        .or_insert(0) += 1;
                }
            }
        }

        let mut blocker_reason_counts = blocker_reason_counts
            .into_iter()
            .map(|(reason, count)| ContextPlaneOperatorApprovalBlockerReasonCount { reason, count })
            .collect::<Vec<_>>();
        blocker_reason_counts.sort_by_key(|entry| activation_blocker_reason_order(entry.reason));

        let mut recall_quality_blocking_reason_counts =
            recall_quality_blocking_reason_counts
                .into_iter()
                .map(|(reason, count)| {
                    ContextPlaneOperatorApprovalRecallQualityBlockerReasonCount { reason, count }
                })
                .collect::<Vec<_>>();
        recall_quality_blocking_reason_counts
            .sort_by_key(|entry| recall_quality_blocker_reason_order(entry.reason));
        let recall_quality_blocking_reason_count = recall_quality_blocking_reason_counts.len();

        let required_approval_scopes = required_operator_approval_scopes();

        Self {
            dry_run_only: true,
            approval_required: true,
            activation_command_present: false,
            matrix_row_count: matrix.rows.len(),
            threshold_satisfied_count: matrix.satisfied_count(),
            blocker_count: matrix.blocker_count,
            threshold_snapshot: ContextPlaneOperatorApprovalThresholdSnapshot::from_matrix(matrix),
            blocker_reason_counts,
            recall_quality_blocking_reason_count,
            recall_quality_blocking_reason_counts,
            required_approval_scopes,
            production_write: matrix.production_write,
            graph_write: matrix.graph_write,
            runtime_activation: matrix.runtime_activation,
            adaptive_allocator_runtime_activation: matrix.adaptive_allocator_runtime_activation,
            source_aware_runtime_activation: matrix.source_aware_runtime_activation,
            prompt_assembly_change: matrix.prompt_assembly_change,
            operator_activation_allowed: matrix.operator_activation_allowed,
            ..Self::default()
        }
    }

    pub fn has_packet_integrity(&self) -> bool {
        self.schema_version == CONTEXT_PLANE_OPERATOR_APPROVAL_PACKET_SCHEMA_VERSION
            && self.dry_run_only
            && self.approval_required
            && !self.activation_command_present
            && self.matrix_row_count == 12
            && self.threshold_satisfied_count + self.blocker_count == self.matrix_row_count
            && self.threshold_snapshot.has_snapshot_integrity()
            && self.threshold_snapshot.total_row_count == self.matrix_row_count
            && self.threshold_snapshot.threshold_satisfied_count == self.threshold_satisfied_count
            && self.threshold_snapshot.blocker_count == self.blocker_count
            && self.blocker_count == self.blocker_reason_count_total()
            && self
                .blocker_reason_counts
                .iter()
                .all(ContextPlaneOperatorApprovalBlockerReasonCount::has_count_integrity)
            && self.has_recall_quality_blocking_reason_count_integrity()
            && self.has_required_approval_scopes()
            && !self.production_write
            && !self.graph_write
            && !self.runtime_activation
            && !self.adaptive_allocator_runtime_activation
            && !self.source_aware_runtime_activation
            && !self.prompt_assembly_change
            && !self.operator_activation_allowed
    }

    pub fn blocker_reason_count_total(&self) -> usize {
        self.blocker_reason_counts
            .iter()
            .map(|entry| entry.count)
            .sum()
    }

    pub fn blocker_reason_count(
        &self,
        reason: ContextPlaneActivationBlockerReason,
    ) -> Option<usize> {
        self.blocker_reason_counts
            .iter()
            .find(|entry| entry.reason == reason)
            .map(|entry| entry.count)
    }

    pub fn recall_quality_blocking_reason_count_total(&self) -> usize {
        self.recall_quality_blocking_reason_counts
            .iter()
            .map(|entry| entry.count)
            .sum()
    }

    pub fn recall_quality_blocking_reason_count_for(
        &self,
        reason: ContextMemoryRecallQualityGateBlockerReason,
    ) -> Option<usize> {
        self.recall_quality_blocking_reason_counts
            .iter()
            .find(|entry| entry.reason == reason)
            .map(|entry| entry.count)
    }

    pub fn required_scope_count(&self) -> usize {
        self.required_approval_scopes.len()
    }

    fn has_required_approval_scopes(&self) -> bool {
        let expected = required_operator_approval_scopes();
        self.required_approval_scopes == expected
            && self
                .required_approval_scopes
                .iter()
                .all(|scope| !scope.is_unknown())
    }

    fn has_recall_quality_blocking_reason_count_integrity(&self) -> bool {
        let reasons_are_unique = self
            .recall_quality_blocking_reason_counts
            .iter()
            .enumerate()
            .all(|(index, entry)| {
                !self.recall_quality_blocking_reason_counts[..index]
                    .iter()
                    .any(|prior| prior.reason == entry.reason)
            });

        self.recall_quality_blocking_reason_count
            == self.recall_quality_blocking_reason_counts.len()
            && reasons_are_unique
            && self.recall_quality_blocking_reason_counts.iter().all(
                ContextPlaneOperatorApprovalRecallQualityBlockerReasonCount::has_count_integrity,
            )
    }
}

pub(in crate::memory) fn required_operator_approval_scopes()
-> Vec<ContextPlaneOperatorApprovalScope> {
    vec![
        ContextPlaneOperatorApprovalScope::AdaptiveBudgetAllocationRuntime,
        ContextPlaneOperatorApprovalScope::SourceAwareRuntimeActivation,
        ContextPlaneOperatorApprovalScope::ProductionMemoryWrite,
        ContextPlaneOperatorApprovalScope::GraphWrite,
        ContextPlaneOperatorApprovalScope::PromptAssemblyChange,
        ContextPlaneOperatorApprovalScope::OperatorActivation,
    ]
}

fn recall_quality_blocker_reason_order(
    reason: ContextMemoryRecallQualityGateBlockerReason,
) -> usize {
    match reason {
        ContextMemoryRecallQualityGateBlockerReason::MissingCriticalFactRegression => 0,
        ContextMemoryRecallQualityGateBlockerReason::RecallCoverageRegression => 1,
        ContextMemoryRecallQualityGateBlockerReason::PrecisionRegression => 2,
        ContextMemoryRecallQualityGateBlockerReason::SafetyLeak => 3,
        ContextMemoryRecallQualityGateBlockerReason::AnswerQualityRegression => 4,
        ContextMemoryRecallQualityGateBlockerReason::SideEffectFlagEnabled => 5,
    }
}

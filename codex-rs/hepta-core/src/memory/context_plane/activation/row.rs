use serde::Deserialize;
use serde::Serialize;

use super::super::super::ContextMemoryRecallQualityGateBlockerReason;
use super::super::status::ContextPlaneStatusEntry;
use super::super::status::ContextPlaneStatusKind;
use super::super::status::context_plane_status_entry_has_side_effect_flag;
use super::target::ContextPlaneActivationBlockerReason;
use super::target::ContextPlaneActivationTarget;

/// One activation-readiness threshold row.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ContextPlaneActivationBlockerRow {
    pub target: ContextPlaneActivationTarget,
    pub observed_status: ContextPlaneStatusKind,
    pub required_status: ContextPlaneStatusKind,
    pub threshold_satisfied: bool,
    pub blocker_reason: ContextPlaneActivationBlockerReason,
    pub recall_quality_blocking_reason_count: usize,
    pub recall_quality_blocking_reasons: Vec<ContextMemoryRecallQualityGateBlockerReason>,
    pub production_write: bool,
    pub graph_write: bool,
    pub runtime_activation: bool,
    pub prompt_assembly_change: bool,
    pub operator_activation_allowed: bool,
}

impl ContextPlaneActivationBlockerRow {
    fn satisfied(
        target: ContextPlaneActivationTarget,
        observed_status: ContextPlaneStatusKind,
        required_status: ContextPlaneStatusKind,
    ) -> Self {
        Self {
            target,
            observed_status,
            required_status,
            threshold_satisfied: true,
            blocker_reason: ContextPlaneActivationBlockerReason::None,
            ..Self::default()
        }
    }

    pub(super) fn blocked(
        target: ContextPlaneActivationTarget,
        observed_status: ContextPlaneStatusKind,
        required_status: ContextPlaneStatusKind,
        blocker_reason: ContextPlaneActivationBlockerReason,
    ) -> Self {
        Self {
            target,
            observed_status,
            required_status,
            threshold_satisfied: false,
            blocker_reason,
            ..Self::default()
        }
    }

    fn from_required_status(
        target: ContextPlaneActivationTarget,
        observed_status: ContextPlaneStatusKind,
        required_status: ContextPlaneStatusKind,
    ) -> Self {
        if observed_status == required_status {
            return Self::satisfied(target, observed_status, required_status);
        }

        let reason = match (target, observed_status) {
            (_, ContextPlaneStatusKind::Unknown) => {
                ContextPlaneActivationBlockerReason::StatusMissing
            }
            (_, ContextPlaneStatusKind::Blocked) => {
                ContextPlaneActivationBlockerReason::SectionBlocked
            }
            (
                ContextPlaneActivationTarget::AdaptiveBudgetAllocation,
                ContextPlaneStatusKind::Shadow,
            ) => ContextPlaneActivationBlockerReason::AdaptiveBudgetAllocationShadowOnly,
            (
                ContextPlaneActivationTarget::MemoryTemporalGraphShadowEval,
                ContextPlaneStatusKind::Shadow,
            ) => ContextPlaneActivationBlockerReason::TemporalGraphShadowEvalShadowOnly,
            (
                ContextPlaneActivationTarget::MemoryProviderBoundary,
                ContextPlaneStatusKind::Shadow,
            ) => ContextPlaneActivationBlockerReason::MemoryProviderBoundaryShadowOnly,
            (
                ContextPlaneActivationTarget::MemoryShadowCanaryReadiness,
                ContextPlaneStatusKind::Shadow,
            ) => ContextPlaneActivationBlockerReason::MemoryShadowCanaryReadinessShadowOnly,
            (
                ContextPlaneActivationTarget::SourceAwareFrontDoor,
                ContextPlaneStatusKind::Disabled,
            ) => ContextPlaneActivationBlockerReason::SourceAwareFrontDoorDisabled,
            (_, ContextPlaneStatusKind::Shadow) => {
                ContextPlaneActivationBlockerReason::SectionShadowOnly
            }
            (_, ContextPlaneStatusKind::Disabled) => {
                ContextPlaneActivationBlockerReason::SectionDisabled
            }
            _ => ContextPlaneActivationBlockerReason::UnexpectedStatus,
        };

        Self::blocked(target, observed_status, required_status, reason)
    }

    pub(super) fn from_status_entry(
        target: ContextPlaneActivationTarget,
        entry: Option<&ContextPlaneStatusEntry>,
        required_status: ContextPlaneStatusKind,
        report_side_effect_flag_enabled: bool,
    ) -> Self {
        let observed_status = entry
            .map(|entry| entry.status)
            .unwrap_or(ContextPlaneStatusKind::Unknown);
        let entry_side_effect_flag_enabled = entry
            .map(context_plane_status_entry_has_side_effect_flag)
            .unwrap_or(false);
        if report_side_effect_flag_enabled || entry_side_effect_flag_enabled {
            return Self::blocked(
                target,
                observed_status,
                required_status,
                ContextPlaneActivationBlockerReason::SideEffectFlagEnabled,
            )
            .with_recall_quality_rollup(target, entry);
        }

        Self::from_required_status(target, observed_status, required_status)
            .with_recall_quality_rollup(target, entry)
    }

    fn with_recall_quality_rollup(
        mut self,
        target: ContextPlaneActivationTarget,
        entry: Option<&ContextPlaneStatusEntry>,
    ) -> Self {
        if target == ContextPlaneActivationTarget::RecallQualityGate
            && let Some(entry) = entry
        {
            self.recall_quality_blocking_reason_count = entry.recall_quality_blocking_reason_count;
            self.recall_quality_blocking_reasons = entry.recall_quality_blocking_reasons.clone();
        }
        self
    }

    pub fn has_row_integrity(&self) -> bool {
        !self.target.is_unknown()
            && !self.observed_status.is_unknown()
            && !self.required_status.is_unknown()
            && !self.blocker_reason.is_unknown()
            && self.threshold_satisfied != self.blocker_reason.is_blocking()
            && self.has_recall_quality_rollup_integrity()
            && !self.production_write
            && !self.graph_write
            && !self.runtime_activation
            && !self.prompt_assembly_change
            && !self.operator_activation_allowed
    }

    fn has_recall_quality_rollup_integrity(&self) -> bool {
        if self.target != ContextPlaneActivationTarget::RecallQualityGate {
            return self.recall_quality_blocking_reason_count == 0
                && self.recall_quality_blocking_reasons.is_empty();
        }

        let reasons_are_unique = self
            .recall_quality_blocking_reasons
            .iter()
            .enumerate()
            .all(|(index, reason)| !self.recall_quality_blocking_reasons[..index].contains(reason));

        self.recall_quality_blocking_reason_count == self.recall_quality_blocking_reasons.len()
            && reasons_are_unique
            && (!self.threshold_satisfied || self.recall_quality_blocking_reasons.is_empty())
            && (self.observed_status != ContextPlaneStatusKind::Blocked
                || !self.recall_quality_blocking_reasons.is_empty())
    }
}

use serde::Deserialize;
use serde::Serialize;

use super::super::super::ContextMemoryRecallQualityGateBlockerReason;
use super::super::status::ContextPlaneStatusEntry;
use super::super::status::ContextPlaneStatusKind;
use super::super::status::context_plane_status_entry_has_side_effect_flag;
use super::target::ContextPlaneActivationBlockerReason;
use super::target::ContextPlaneActivationTarget;

const CANARY_PROMOTION_CHECKLIST_REQUIRED_COUNT: usize = 4;
const MEMORY_PROVIDER_V2_LIFECYCLE_REQUIRED_COUNT: usize = 6;

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
    pub canary_promotion_required_stable_window_count: usize,
    pub canary_promotion_observed_stable_window_count: usize,
    pub canary_promotion_required_pass_streak: usize,
    pub canary_promotion_observed_pass_streak: usize,
    pub canary_promotion_blocker_count: usize,
    pub canary_promotion_checklist_required_count: usize,
    pub canary_promotion_checklist_pass_count: usize,
    pub canary_promotion_readiness_check_pass: bool,
    pub canary_promotion_negative_rehearsal_check_pass: bool,
    pub canary_promotion_audit_digest_check_pass: bool,
    pub canary_promotion_audit_freshness_check_pass: bool,
    pub canary_promotion_rollback_rehearsal_count: usize,
    pub canary_promotion_rollback_rehearsal_pass_count: usize,
    pub canary_promotion_kill_switch_rehearsal_count: usize,
    pub canary_promotion_kill_switch_rehearsal_pass_count: usize,
    pub canary_promotion_soak_readback_window_count: usize,
    pub canary_promotion_soak_readback_pass_count: usize,
    pub memory_provider_v2_lifecycle_required_count: usize,
    pub memory_provider_v2_lifecycle_pass_count: usize,
    pub memory_provider_v2_query_check_pass: bool,
    pub memory_provider_v2_update_context_check_pass: bool,
    pub memory_provider_v2_propose_write_check_pass: bool,
    pub memory_provider_v2_add_check_pass: bool,
    pub memory_provider_v2_clear_check_pass: bool,
    pub memory_provider_v2_close_check_pass: bool,
    pub memory_provider_v2_candidate_count: usize,
    pub memory_provider_v2_operator_review_required_count: usize,
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
                ContextPlaneActivationTarget::MemoryProviderV2Boundary,
                ContextPlaneStatusKind::Shadow,
            ) => ContextPlaneActivationBlockerReason::MemoryProviderV2BoundaryShadowOnly,
            (
                ContextPlaneActivationTarget::MemoryShadowCanaryReadiness,
                ContextPlaneStatusKind::Shadow,
            ) => ContextPlaneActivationBlockerReason::MemoryShadowCanaryReadinessShadowOnly,
            (
                ContextPlaneActivationTarget::MemoryShadowCanaryPromotionReadiness,
                ContextPlaneStatusKind::Shadow,
            ) => {
                ContextPlaneActivationBlockerReason::MemoryShadowCanaryPromotionReadinessShadowOnly
            }
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
            .with_recall_quality_rollup(target, entry)
            .with_canary_promotion_rollup(target, entry)
            .with_memory_provider_v2_rollup(target, entry);
        }

        Self::from_required_status(target, observed_status, required_status)
            .with_recall_quality_rollup(target, entry)
            .with_canary_promotion_rollup(target, entry)
            .with_memory_provider_v2_rollup(target, entry)
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

    fn with_canary_promotion_rollup(
        mut self,
        target: ContextPlaneActivationTarget,
        entry: Option<&ContextPlaneStatusEntry>,
    ) -> Self {
        if target == ContextPlaneActivationTarget::MemoryShadowCanaryPromotionReadiness
            && let Some(entry) = entry
        {
            self.canary_promotion_required_stable_window_count =
                entry.canary_promotion_required_stable_window_count;
            self.canary_promotion_observed_stable_window_count =
                entry.canary_promotion_observed_stable_window_count;
            self.canary_promotion_required_pass_streak =
                entry.canary_promotion_required_pass_streak;
            self.canary_promotion_observed_pass_streak =
                entry.canary_promotion_observed_pass_streak;
            self.canary_promotion_blocker_count = entry.canary_promotion_blocker_count;
            self.canary_promotion_checklist_required_count =
                entry.canary_promotion_checklist_required_count;
            self.canary_promotion_checklist_pass_count =
                entry.canary_promotion_checklist_pass_count;
            self.canary_promotion_readiness_check_pass =
                entry.canary_promotion_readiness_check_pass;
            self.canary_promotion_negative_rehearsal_check_pass =
                entry.canary_promotion_negative_rehearsal_check_pass;
            self.canary_promotion_audit_digest_check_pass =
                entry.canary_promotion_audit_digest_check_pass;
            self.canary_promotion_audit_freshness_check_pass =
                entry.canary_promotion_audit_freshness_check_pass;
            self.canary_promotion_rollback_rehearsal_count =
                entry.canary_promotion_rollback_rehearsal_count;
            self.canary_promotion_rollback_rehearsal_pass_count =
                entry.canary_promotion_rollback_rehearsal_pass_count;
            self.canary_promotion_kill_switch_rehearsal_count =
                entry.canary_promotion_kill_switch_rehearsal_count;
            self.canary_promotion_kill_switch_rehearsal_pass_count =
                entry.canary_promotion_kill_switch_rehearsal_pass_count;
            self.canary_promotion_soak_readback_window_count =
                entry.canary_promotion_soak_readback_window_count;
            self.canary_promotion_soak_readback_pass_count =
                entry.canary_promotion_soak_readback_pass_count;
        }
        self
    }

    fn with_memory_provider_v2_rollup(
        mut self,
        target: ContextPlaneActivationTarget,
        entry: Option<&ContextPlaneStatusEntry>,
    ) -> Self {
        if target == ContextPlaneActivationTarget::MemoryProviderV2Boundary
            && let Some(entry) = entry
        {
            self.memory_provider_v2_lifecycle_required_count =
                entry.memory_provider_v2_lifecycle_required_count;
            self.memory_provider_v2_lifecycle_pass_count =
                entry.memory_provider_v2_lifecycle_pass_count;
            self.memory_provider_v2_query_check_pass = entry.memory_provider_v2_query_check_pass;
            self.memory_provider_v2_update_context_check_pass =
                entry.memory_provider_v2_update_context_check_pass;
            self.memory_provider_v2_propose_write_check_pass =
                entry.memory_provider_v2_propose_write_check_pass;
            self.memory_provider_v2_add_check_pass = entry.memory_provider_v2_add_check_pass;
            self.memory_provider_v2_clear_check_pass = entry.memory_provider_v2_clear_check_pass;
            self.memory_provider_v2_close_check_pass = entry.memory_provider_v2_close_check_pass;
            self.memory_provider_v2_candidate_count = entry.memory_provider_v2_candidate_count;
            self.memory_provider_v2_operator_review_required_count =
                entry.memory_provider_v2_operator_review_required_count;
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
            && self.has_canary_promotion_rollup_integrity()
            && self.has_memory_provider_v2_rollup_integrity()
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

    fn has_canary_promotion_rollup_integrity(&self) -> bool {
        let counts = [
            self.canary_promotion_required_stable_window_count,
            self.canary_promotion_observed_stable_window_count,
            self.canary_promotion_required_pass_streak,
            self.canary_promotion_observed_pass_streak,
            self.canary_promotion_blocker_count,
            self.canary_promotion_checklist_required_count,
            self.canary_promotion_checklist_pass_count,
            self.canary_promotion_rollback_rehearsal_count,
            self.canary_promotion_rollback_rehearsal_pass_count,
            self.canary_promotion_kill_switch_rehearsal_count,
            self.canary_promotion_kill_switch_rehearsal_pass_count,
            self.canary_promotion_soak_readback_window_count,
            self.canary_promotion_soak_readback_pass_count,
        ];
        let checks = [
            self.canary_promotion_readiness_check_pass,
            self.canary_promotion_negative_rehearsal_check_pass,
            self.canary_promotion_audit_digest_check_pass,
            self.canary_promotion_audit_freshness_check_pass,
        ];

        if self.target != ContextPlaneActivationTarget::MemoryShadowCanaryPromotionReadiness {
            return counts.iter().all(|count| *count == 0) && checks.iter().all(|check| !check);
        }

        let checklist_pass_count = checks.iter().filter(|check| **check).count();
        let no_promotion_blockers = self.canary_promotion_blocker_count == 0;
        let checklist_complete = self.canary_promotion_checklist_pass_count
            == self.canary_promotion_checklist_required_count;
        let stable_window_complete = self.canary_promotion_observed_stable_window_count
            == self.canary_promotion_required_stable_window_count;
        let pass_streak_complete = self.canary_promotion_observed_pass_streak
            == self.canary_promotion_required_pass_streak;
        let rollback_rehearsal_complete = self.canary_promotion_rollback_rehearsal_pass_count
            == self.canary_promotion_rollback_rehearsal_count;
        let kill_switch_rehearsal_complete = self.canary_promotion_kill_switch_rehearsal_pass_count
            == self.canary_promotion_kill_switch_rehearsal_count;
        let soak_readback_complete = self.canary_promotion_soak_readback_pass_count
            == self.canary_promotion_soak_readback_window_count;

        self.canary_promotion_required_stable_window_count > 0
            && self.canary_promotion_observed_stable_window_count
                <= self.canary_promotion_required_stable_window_count
            && self.canary_promotion_required_pass_streak > 0
            && self.canary_promotion_observed_pass_streak
                <= self.canary_promotion_required_pass_streak
            && self.canary_promotion_rollback_rehearsal_count > 0
            && self.canary_promotion_rollback_rehearsal_pass_count
                <= self.canary_promotion_rollback_rehearsal_count
            && self.canary_promotion_kill_switch_rehearsal_count > 0
            && self.canary_promotion_kill_switch_rehearsal_pass_count
                <= self.canary_promotion_kill_switch_rehearsal_count
            && self.canary_promotion_soak_readback_window_count > 0
            && self.canary_promotion_soak_readback_pass_count
                <= self.canary_promotion_soak_readback_window_count
            && self.canary_promotion_checklist_required_count
                == CANARY_PROMOTION_CHECKLIST_REQUIRED_COUNT
            && self.canary_promotion_checklist_pass_count == checklist_pass_count
            && self.canary_promotion_checklist_pass_count
                <= self.canary_promotion_checklist_required_count
            && no_promotion_blockers == checklist_complete
            && (!no_promotion_blockers
                || (stable_window_complete
                    && pass_streak_complete
                    && rollback_rehearsal_complete
                    && kill_switch_rehearsal_complete
                    && soak_readback_complete))
            && (self.canary_promotion_blocker_count == 0 || self.blocker_reason.is_blocking())
    }

    fn has_memory_provider_v2_rollup_integrity(&self) -> bool {
        let counts = [
            self.memory_provider_v2_lifecycle_required_count,
            self.memory_provider_v2_lifecycle_pass_count,
            self.memory_provider_v2_candidate_count,
            self.memory_provider_v2_operator_review_required_count,
        ];
        let checks = [
            self.memory_provider_v2_query_check_pass,
            self.memory_provider_v2_update_context_check_pass,
            self.memory_provider_v2_propose_write_check_pass,
            self.memory_provider_v2_add_check_pass,
            self.memory_provider_v2_clear_check_pass,
            self.memory_provider_v2_close_check_pass,
        ];

        if self.target != ContextPlaneActivationTarget::MemoryProviderV2Boundary {
            return counts.iter().all(|count| *count == 0) && checks.iter().all(|check| !check);
        }

        let lifecycle_pass_count = checks.iter().filter(|check| **check).count();
        self.memory_provider_v2_lifecycle_required_count
            == MEMORY_PROVIDER_V2_LIFECYCLE_REQUIRED_COUNT
            && self.memory_provider_v2_lifecycle_pass_count == lifecycle_pass_count
            && self.memory_provider_v2_lifecycle_pass_count
                <= self.memory_provider_v2_lifecycle_required_count
            && self.memory_provider_v2_operator_review_required_count
                <= self.memory_provider_v2_candidate_count
            && (self.memory_provider_v2_lifecycle_pass_count
                == self.memory_provider_v2_lifecycle_required_count
                || self.blocker_reason.is_blocking())
    }
}

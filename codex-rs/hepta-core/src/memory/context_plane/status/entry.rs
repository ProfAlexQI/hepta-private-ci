use serde::Deserialize;
use serde::Serialize;

use super::section::ContextPlaneStatusKind;
use super::section::ContextPlaneStatusSection;
use crate::memory::ContextMemoryRecallQualityGateBlockerReason;
use crate::memory::ContextMemoryRecallQualityGateReport;
use crate::memory::ContextMemoryShadowCanaryPromotionReadinessReport;
use crate::memory::ContextMemoryShadowQualityTrendSnapshotReport;
use crate::memory::ContextMemoryTemporalGraphShadowEvalReport;
use crate::memory::MemoryProviderReport;

const CANARY_PROMOTION_CHECKLIST_REQUIRED_COUNT: usize = 4;

/// One payload-light context-plane status row.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ContextPlaneStatusEntry {
    pub section: ContextPlaneStatusSection,
    pub status: ContextPlaneStatusKind,
    pub observed_count: usize,
    pub omitted_count: usize,
    pub blocker_count: usize,
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
    pub production_write: bool,
    pub graph_write: bool,
    pub runtime_activation: bool,
    pub prompt_assembly_change: bool,
    pub operator_activation_allowed: bool,
}

impl ContextPlaneStatusEntry {
    pub(in crate::memory) fn ready(
        section: ContextPlaneStatusSection,
        observed_count: usize,
    ) -> Self {
        Self {
            section,
            status: ContextPlaneStatusKind::Ready,
            observed_count,
            ..Self::default()
        }
    }

    pub(in crate::memory) fn shadow(
        section: ContextPlaneStatusSection,
        observed_count: usize,
    ) -> Self {
        Self {
            section,
            status: ContextPlaneStatusKind::Shadow,
            observed_count,
            ..Self::default()
        }
    }

    pub(in crate::memory) fn disabled(section: ContextPlaneStatusSection) -> Self {
        Self {
            section,
            status: ContextPlaneStatusKind::Disabled,
            observed_count: 1,
            ..Self::default()
        }
    }

    pub(in crate::memory::context_plane::status) fn from_integrity(
        section: ContextPlaneStatusSection,
        integrity: bool,
        observed_count: usize,
        omitted_count: usize,
    ) -> Self {
        Self {
            section,
            status: if integrity {
                ContextPlaneStatusKind::Ready
            } else {
                ContextPlaneStatusKind::Blocked
            },
            observed_count,
            omitted_count,
            blocker_count: usize::from(!integrity),
            ..Self::default()
        }
    }

    pub(in crate::memory::context_plane::status) fn shadow_from_integrity(
        section: ContextPlaneStatusSection,
        integrity: bool,
        observed_count: usize,
        omitted_count: usize,
    ) -> Self {
        Self {
            section,
            status: if integrity {
                ContextPlaneStatusKind::Shadow
            } else {
                ContextPlaneStatusKind::Blocked
            },
            observed_count,
            omitted_count,
            blocker_count: usize::from(!integrity),
            ..Self::default()
        }
    }

    pub(in crate::memory::context_plane::status) fn from_recall_quality_gate(
        recall_quality_gate: &ContextMemoryRecallQualityGateReport,
    ) -> Self {
        let recall_quality_blocking_reasons =
            recall_quality_status_blocking_reasons(recall_quality_gate);
        let status = if recall_quality_gate.has_quality_gate_integrity() {
            ContextPlaneStatusKind::Ready
        } else {
            ContextPlaneStatusKind::Blocked
        };
        let blocker_count = match status {
            ContextPlaneStatusKind::Blocked => recall_quality_blocking_reasons.len().max(1),
            ContextPlaneStatusKind::Ready
            | ContextPlaneStatusKind::Shadow
            | ContextPlaneStatusKind::Disabled
            | ContextPlaneStatusKind::Unknown => 0,
        };

        Self {
            section: ContextPlaneStatusSection::RecallQualityGate,
            status,
            observed_count: recall_quality_gate.fixture_count,
            omitted_count: recall_quality_gate.missing_critical_fact_count,
            blocker_count,
            recall_quality_blocking_reason_count: recall_quality_blocking_reasons.len(),
            recall_quality_blocking_reasons,
            production_write: recall_quality_gate.production_write,
            graph_write: recall_quality_gate.graph_write,
            runtime_activation: recall_quality_gate.runtime_activation,
            prompt_assembly_change: recall_quality_gate.prompt_assembly_change,
            operator_activation_allowed: recall_quality_gate.operator_activation_allowed,
            ..Self::default()
        }
    }

    pub(in crate::memory::context_plane::status) fn from_temporal_graph_shadow_eval(
        temporal_graph_shadow_eval: &ContextMemoryTemporalGraphShadowEvalReport,
    ) -> Self {
        let has_integrity = temporal_graph_shadow_eval.has_temporal_graph_shadow_integrity();

        Self {
            section: ContextPlaneStatusSection::MemoryTemporalGraphShadowEval,
            status: if has_integrity {
                ContextPlaneStatusKind::Shadow
            } else {
                ContextPlaneStatusKind::Blocked
            },
            observed_count: temporal_graph_shadow_eval.fixture_count(),
            omitted_count: temporal_graph_shadow_eval
                .fixture_count()
                .saturating_sub(temporal_graph_shadow_eval.fixture_pass_count()),
            blocker_count: usize::from(!has_integrity),
            production_write: temporal_graph_shadow_eval.production_write,
            graph_write: temporal_graph_shadow_eval.graph_write,
            runtime_activation: temporal_graph_shadow_eval.runtime_activation,
            prompt_assembly_change: temporal_graph_shadow_eval.prompt_assembly_change,
            operator_activation_allowed: temporal_graph_shadow_eval.operator_activation_allowed,
            ..Self::default()
        }
    }

    pub(in crate::memory::context_plane::status) fn from_memory_provider_report(
        provider_report: &MemoryProviderReport,
    ) -> Self {
        let has_integrity = provider_report.has_provider_boundary_integrity();

        Self {
            section: ContextPlaneStatusSection::MemoryProviderBoundary,
            status: if has_integrity {
                ContextPlaneStatusKind::Shadow
            } else {
                ContextPlaneStatusKind::Blocked
            },
            observed_count: 1,
            blocker_count: usize::from(!has_integrity),
            production_write: provider_report.update_context.write_performed,
            runtime_activation: provider_report.update_context.runtime_activation,
            prompt_assembly_change: provider_report.update_context.prompt_payload_exported
                || provider_report.update_context.ranked_payload_exported,
            ..Self::default()
        }
    }

    pub(in crate::memory::context_plane::status) fn from_memory_shadow_canary_readiness(
        trend_snapshot: &ContextMemoryShadowQualityTrendSnapshotReport,
    ) -> Self {
        let has_integrity = trend_snapshot.has_shadow_quality_trend_snapshot_integrity();
        let blocker_count = if has_integrity {
            0
        } else {
            trend_snapshot.regression_window_blocking_count.max(1)
        };

        Self {
            section: ContextPlaneStatusSection::MemoryShadowCanaryReadiness,
            status: if has_integrity {
                ContextPlaneStatusKind::Shadow
            } else {
                ContextPlaneStatusKind::Blocked
            },
            observed_count: trend_snapshot.window_observation_count,
            omitted_count: trend_snapshot.regression_window_blocking_count,
            blocker_count,
            production_write: trend_snapshot.production_write || trend_snapshot.production_route,
            graph_write: trend_snapshot.graph_write,
            runtime_activation: trend_snapshot.runtime_activation,
            prompt_assembly_change: trend_snapshot.prompt_assembly_change,
            operator_activation_allowed: trend_snapshot.operator_activation_allowed,
            ..Self::default()
        }
    }

    pub(in crate::memory::context_plane::status) fn from_memory_shadow_canary_promotion_readiness(
        promotion_readiness: &ContextMemoryShadowCanaryPromotionReadinessReport,
    ) -> Self {
        let has_integrity = promotion_readiness.has_shadow_canary_promotion_readiness_integrity();
        let canary_promotion_checklist_pass_count = if has_integrity {
            CANARY_PROMOTION_CHECKLIST_REQUIRED_COUNT
        } else {
            0
        };
        let blocker_count = if has_integrity {
            0
        } else {
            promotion_readiness.promotion_blocker_count.max(1)
        };

        Self {
            section: ContextPlaneStatusSection::MemoryShadowCanaryPromotionReadiness,
            status: if has_integrity {
                ContextPlaneStatusKind::Shadow
            } else {
                ContextPlaneStatusKind::Blocked
            },
            observed_count: promotion_readiness.rollback_rehearsal_count
                + promotion_readiness.kill_switch_rehearsal_count
                + promotion_readiness.soak_readback_window_count,
            omitted_count: promotion_readiness.promotion_blocker_count,
            blocker_count,
            canary_promotion_required_stable_window_count: promotion_readiness
                .required_stable_window_count,
            canary_promotion_observed_stable_window_count: promotion_readiness
                .observed_stable_window_count,
            canary_promotion_required_pass_streak: promotion_readiness.required_pass_streak,
            canary_promotion_observed_pass_streak: promotion_readiness.observed_pass_streak,
            canary_promotion_blocker_count: promotion_readiness.promotion_blocker_count,
            canary_promotion_checklist_required_count: CANARY_PROMOTION_CHECKLIST_REQUIRED_COUNT,
            canary_promotion_checklist_pass_count,
            canary_promotion_readiness_check_pass: has_integrity,
            canary_promotion_negative_rehearsal_check_pass: has_integrity,
            canary_promotion_audit_digest_check_pass: has_integrity,
            canary_promotion_audit_freshness_check_pass: has_integrity,
            canary_promotion_rollback_rehearsal_count: promotion_readiness.rollback_rehearsal_count,
            canary_promotion_rollback_rehearsal_pass_count: promotion_readiness
                .rollback_rehearsal_pass_count,
            canary_promotion_kill_switch_rehearsal_count: promotion_readiness
                .kill_switch_rehearsal_count,
            canary_promotion_kill_switch_rehearsal_pass_count: promotion_readiness
                .kill_switch_rehearsal_pass_count,
            canary_promotion_soak_readback_window_count: promotion_readiness
                .soak_readback_window_count,
            canary_promotion_soak_readback_pass_count: promotion_readiness.soak_readback_pass_count,
            production_write: promotion_readiness.production_write
                || promotion_readiness.production_route
                || promotion_readiness.history_persistence_write
                || promotion_readiness.canary_promotion_route_opened
                || promotion_readiness.rollback_write,
            graph_write: promotion_readiness.graph_write,
            runtime_activation: promotion_readiness.runtime_activation,
            prompt_assembly_change: promotion_readiness.prompt_assembly_change,
            operator_activation_allowed: promotion_readiness.operator_activation_allowed,
            ..Self::default()
        }
    }

    pub fn has_status_integrity(&self) -> bool {
        !self.section.is_unknown()
            && !self.status.is_unknown()
            && (self.status == ContextPlaneStatusKind::Blocked) == (self.blocker_count > 0)
            && self.has_recall_quality_blocker_integrity()
            && self.has_canary_promotion_checklist_integrity()
            && !self.production_write
            && !self.graph_write
            && !self.runtime_activation
            && !self.prompt_assembly_change
            && !self.operator_activation_allowed
    }

    fn has_recall_quality_blocker_integrity(&self) -> bool {
        if self.section != ContextPlaneStatusSection::RecallQualityGate {
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
            && (self.status == ContextPlaneStatusKind::Ready)
                == self.recall_quality_blocking_reasons.is_empty()
    }

    fn has_canary_promotion_checklist_integrity(&self) -> bool {
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

        if self.section != ContextPlaneStatusSection::MemoryShadowCanaryPromotionReadiness {
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
            && self.canary_promotion_blocker_count == self.blocker_count
            && no_promotion_blockers == checklist_complete
            && (!no_promotion_blockers
                || (stable_window_complete
                    && pass_streak_complete
                    && rollback_rehearsal_complete
                    && kill_switch_rehearsal_complete
                    && soak_readback_complete))
            && (self.status == ContextPlaneStatusKind::Shadow)
                == (no_promotion_blockers && checklist_complete)
    }
}

pub(in crate::memory::context_plane) fn context_plane_status_entry_has_side_effect_flag(
    entry: &ContextPlaneStatusEntry,
) -> bool {
    entry.production_write
        || entry.graph_write
        || entry.runtime_activation
        || entry.prompt_assembly_change
        || entry.operator_activation_allowed
}

fn recall_quality_status_blocking_reasons(
    recall_quality_gate: &ContextMemoryRecallQualityGateReport,
) -> Vec<ContextMemoryRecallQualityGateBlockerReason> {
    let mut reasons = Vec::new();
    for fixture in &recall_quality_gate.fixture_matrix {
        for reason in &fixture.blocking_reasons {
            if !reasons.contains(reason) {
                reasons.push(*reason);
            }
        }
    }
    reasons
}

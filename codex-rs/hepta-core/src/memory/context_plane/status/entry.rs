use serde::Deserialize;
use serde::Serialize;

use super::section::ContextPlaneStatusKind;
use super::section::ContextPlaneStatusSection;
use crate::memory::ContextMemoryRecallQualityGateBlockerReason;
use crate::memory::ContextMemoryRecallQualityGateReport;
use crate::memory::ContextMemoryTemporalGraphShadowEvalReport;
use crate::memory::MemoryProviderReport;

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

    pub fn has_status_integrity(&self) -> bool {
        !self.section.is_unknown()
            && !self.status.is_unknown()
            && (self.status == ContextPlaneStatusKind::Blocked) == (self.blocker_count > 0)
            && self.has_recall_quality_blocker_integrity()
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

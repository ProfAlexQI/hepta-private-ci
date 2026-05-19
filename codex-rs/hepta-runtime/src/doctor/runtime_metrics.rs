use hepta_core::DoctorRuntimeMetrics;

use crate::RuntimeKernel;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct DoctorRuntimeStatInputs {
    pub registered_providers: usize,
    pub registered_tools: usize,
    pub sessions: usize,
    pub raw_session_records: usize,
    pub memories: usize,
    pub history_entries: usize,
    pub active_session_pending_approvals: usize,
    pub approval_scoped_sessions: usize,
    pub total_topic_sessions: usize,
    pub total_topic_graph_edges: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct DoctorRuntimeStats {
    pub registered_providers: usize,
    pub registered_tools: usize,
    pub sessions: usize,
    pub raw_session_records: usize,
    pub memories: usize,
    pub history_entries: usize,
    pub active_session_pending_approvals: usize,
    pub approval_scoped_sessions: usize,
    pub total_topic_sessions: usize,
    pub total_topic_graph_edges: usize,
    pub active_topic_sessions: usize,
    pub active_topic_sessions_with_transcript_provenance: usize,
    pub active_topic_sessions_missing_transcript_provenance: usize,
    pub active_session_recall_transcript_evidence_spans: usize,
    pub active_session_recall_omitted_items: usize,
    pub active_session_intuition_transcript_evidence_spans: usize,
    pub active_session_intuition_foreground_topic_sessions: usize,
}

impl DoctorRuntimeStats {
    pub(super) fn into_v2_runtime_metrics(self) -> DoctorRuntimeMetrics {
        DoctorRuntimeMetrics {
            registered_providers: self.registered_providers,
            registered_tools: self.registered_tools,
            sessions: self.sessions,
            raw_session_records: self.raw_session_records,
            memories: self.memories,
            history_entries: self.history_entries,
            active_session_pending_approvals: self.active_session_pending_approvals,
            approval_scoped_sessions: self.approval_scoped_sessions,
            topic_sessions: self.total_topic_sessions,
            topic_graph_edges: self.total_topic_graph_edges,
            active_topic_sessions: self.active_topic_sessions,
            active_topic_sessions_with_transcript_provenance: self
                .active_topic_sessions_with_transcript_provenance,
            active_topic_sessions_missing_transcript_provenance: self
                .active_topic_sessions_missing_transcript_provenance,
            active_session_recall_transcript_evidence_spans: self
                .active_session_recall_transcript_evidence_spans,
            active_session_recall_omitted_items: self.active_session_recall_omitted_items,
            active_session_intuition_transcript_evidence_spans: self
                .active_session_intuition_transcript_evidence_spans,
            active_session_intuition_foreground_topic_sessions: self
                .active_session_intuition_foreground_topic_sessions,
        }
    }
}

pub(super) fn collect_runtime_stats(
    runtime: &RuntimeKernel,
    active_session_id: &str,
    _snapshot: &crate::RuntimeSnapshot,
    inputs: DoctorRuntimeStatInputs,
) -> DoctorRuntimeStats {
    let provenance = runtime.provenance_overview(active_session_id).ok();

    DoctorRuntimeStats {
        registered_providers: inputs.registered_providers,
        registered_tools: inputs.registered_tools,
        sessions: inputs.sessions,
        raw_session_records: inputs.raw_session_records,
        memories: inputs.memories,
        history_entries: inputs.history_entries,
        active_session_pending_approvals: inputs.active_session_pending_approvals,
        approval_scoped_sessions: inputs.approval_scoped_sessions,
        total_topic_sessions: inputs.total_topic_sessions,
        total_topic_graph_edges: inputs.total_topic_graph_edges,
        active_topic_sessions: provenance
            .as_ref()
            .map(|report| report.active_topic_sessions)
            .unwrap_or(0),
        active_topic_sessions_with_transcript_provenance: provenance
            .as_ref()
            .map(|report| report.active_topic_sessions_with_transcript_provenance)
            .unwrap_or(0),
        active_topic_sessions_missing_transcript_provenance: provenance
            .as_ref()
            .map(|report| report.active_topic_sessions_missing_transcript_provenance)
            .unwrap_or(0),
        active_session_recall_transcript_evidence_spans: provenance
            .as_ref()
            .map(|report| report.recall_transcript_evidence_spans)
            .unwrap_or(0),
        active_session_recall_omitted_items: provenance
            .as_ref()
            .map(|report| report.recall_omitted_items)
            .unwrap_or(0),
        active_session_intuition_transcript_evidence_spans: provenance
            .as_ref()
            .map(|report| report.intuition_transcript_evidence_spans)
            .unwrap_or(0),
        active_session_intuition_foreground_topic_sessions: provenance
            .as_ref()
            .map(|report| report.intuition_foreground_topic_sessions)
            .unwrap_or(0),
    }
}

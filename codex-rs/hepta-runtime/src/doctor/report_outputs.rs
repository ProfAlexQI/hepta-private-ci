use hepta_core::DoctorCheckOutcome;
use hepta_core::DoctorReportV2;
use hepta_core::ModelRef;

use super::DoctorCheck;
use super::DoctorProviderProbe;
use super::DoctorReport;
use super::DoctorStatus;
use super::runtime_metrics::DoctorRuntimeStats;
use super::v2;

pub(super) fn build_legacy_report(
    overall_status: DoctorStatus,
    active_model: ModelRef,
    active_session_id: String,
    stats: DoctorRuntimeStats,
    provider_probes: Vec<DoctorProviderProbe>,
    integrity_checks: Vec<DoctorCheck>,
) -> DoctorReport {
    DoctorReport {
        overall_status,
        active_model,
        registered_providers: stats.registered_providers,
        registered_tools: stats.registered_tools,
        active_session_id,
        sessions: stats.sessions,
        raw_session_records: stats.raw_session_records,
        memories: stats.memories,
        history_entries: stats.history_entries,
        active_session_pending_approvals: stats.active_session_pending_approvals,
        approval_scoped_sessions: stats.approval_scoped_sessions,
        total_topic_sessions: stats.total_topic_sessions,
        total_topic_graph_edges: stats.total_topic_graph_edges,
        active_topic_sessions: stats.active_topic_sessions,
        active_topic_sessions_with_transcript_provenance: stats
            .active_topic_sessions_with_transcript_provenance,
        active_topic_sessions_missing_transcript_provenance: stats
            .active_topic_sessions_missing_transcript_provenance,
        active_session_recall_transcript_evidence_spans: stats
            .active_session_recall_transcript_evidence_spans,
        active_session_recall_omitted_items: stats.active_session_recall_omitted_items,
        active_session_intuition_transcript_evidence_spans: stats
            .active_session_intuition_transcript_evidence_spans,
        active_session_intuition_foreground_topic_sessions: stats
            .active_session_intuition_foreground_topic_sessions,
        provider_probes,
        integrity_checks,
    }
}

pub(super) fn build_v2_report(
    active_model: ModelRef,
    active_session_id: String,
    stats: DoctorRuntimeStats,
    v2_checks: Vec<DoctorCheckOutcome>,
    observed_at_unix_ms: u64,
) -> DoctorReportV2 {
    v2::assemble_report(
        active_model,
        active_session_id,
        v2_checks,
        observed_at_unix_ms,
        stats.into_v2_runtime_metrics(),
    )
}

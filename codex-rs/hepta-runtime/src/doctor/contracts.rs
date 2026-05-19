use hepta_core::ModelRef;
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DoctorStatus {
    Ok,
    Warn,
    Fail,
}

impl From<DoctorStatus> for hepta_core::DoctorStatus {
    fn from(value: DoctorStatus) -> Self {
        match value {
            DoctorStatus::Ok => Self::Ok,
            DoctorStatus::Warn => Self::Warn,
            DoctorStatus::Fail => Self::Fail,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DoctorProviderProbe {
    pub provider_name: String,
    pub model: Option<ModelRef>,
    pub status: DoctorStatus,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DoctorCheck {
    pub name: String,
    pub status: DoctorStatus,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DoctorReport {
    pub overall_status: DoctorStatus,
    pub active_model: ModelRef,
    pub registered_providers: usize,
    pub registered_tools: usize,
    pub active_session_id: String,
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
    pub provider_probes: Vec<DoctorProviderProbe>,
    pub integrity_checks: Vec<DoctorCheck>,
}

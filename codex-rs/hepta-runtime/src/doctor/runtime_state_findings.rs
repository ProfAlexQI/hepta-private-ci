use std::collections::HashSet;

use hepta_core::{MemoryRecord, ModelRef, SessionRecord, TopicSessionStatus};

use super::integrity;
use crate::{RuntimeKernel, RuntimeSnapshot, SessionApprovalState, SessionExport, TurnRecord};

pub(super) struct RuntimeStateFindings {
    pub(super) known_session_ids: HashSet<String>,
    pub(super) active_registered: bool,
    pub(super) active_session_exists: bool,
    pub(super) active_session_archived: bool,
    pub(super) duplicate_sessions: Vec<String>,
    pub(super) orphan_history: Vec<String>,
    pub(super) unknown_granted: Vec<String>,
    pub(super) unknown_pending: Vec<String>,
    pub(super) duplicate_session_models: Vec<String>,
    pub(super) orphan_session_models: Vec<String>,
    pub(super) unknown_session_models: Vec<String>,
    pub(super) duplicate_memories: Vec<String>,
    pub(super) topic_sessions_missing_transcript_provenance: Vec<String>,
    pub(super) active_topic_session_count: usize,
    pub(super) active_topic_sessions_with_transcript_provenance: usize,
    pub(super) snapshot_roundtrip: bool,
    pub(super) active_export_serializable: bool,
    pub(super) total_granted_approvals: usize,
    pub(super) total_pending_approvals: usize,
}

pub(super) fn collect_runtime_state_findings(
    runtime: &RuntimeKernel,
    active_model: &ModelRef,
    available_models: &[ModelRef],
    raw_sessions: &[SessionRecord],
    raw_memories: &[MemoryRecord],
    history: &[TurnRecord],
    approval_sessions: &[SessionApprovalState],
    snapshot: &RuntimeSnapshot,
) -> RuntimeStateFindings {
    let known_session_ids = raw_sessions
        .iter()
        .map(|session| session.session_id.0.clone())
        .collect::<HashSet<_>>();
    let total_granted_approvals = approval_sessions
        .iter()
        .map(|session| session.granted_tools.len())
        .sum::<usize>();
    let total_pending_approvals = approval_sessions
        .iter()
        .map(|session| session.pending.len())
        .sum::<usize>();

    let provenance_findings = topic_session_transcript_provenance_findings(snapshot);

    RuntimeStateFindings {
        active_registered: available_models.iter().any(|model| model == active_model),
        active_session_exists: raw_sessions
            .iter()
            .any(|session| session.session_id.0 == snapshot.active_session_id),
        active_session_archived: raw_sessions
            .iter()
            .find(|session| session.session_id.0 == snapshot.active_session_id)
            .and_then(|session| session.archived_at_unix_ms)
            .is_some(),
        duplicate_sessions: integrity::duplicate_values(
            raw_sessions
                .iter()
                .map(|session| session.session_id.0.clone()),
        ),
        orphan_history: history_session_findings(history, &known_session_ids),
        unknown_granted: unknown_granted_tool_findings(runtime, approval_sessions),
        unknown_pending: unknown_pending_tool_findings(runtime, approval_sessions),
        duplicate_session_models: integrity::duplicate_values(
            snapshot
                .session_models
                .iter()
                .map(|binding| binding.session_id.clone()),
        ),
        orphan_session_models: orphan_session_model_findings(snapshot, &known_session_ids),
        unknown_session_models: unknown_session_model_findings(snapshot, available_models),
        duplicate_memories: integrity::duplicate_values(
            raw_memories.iter().map(|memory| memory.id.clone()),
        ),
        topic_sessions_missing_transcript_provenance: provenance_findings.0,
        active_topic_session_count: provenance_findings.1,
        active_topic_sessions_with_transcript_provenance: provenance_findings.2,
        snapshot_roundtrip: snapshot_roundtrip_ok(snapshot),
        active_export_serializable: active_session_export_roundtrip_ok(
            runtime,
            &snapshot.active_session_id,
        ),
        total_granted_approvals,
        total_pending_approvals,
        known_session_ids,
    }
}

fn history_session_findings(
    history: &[TurnRecord],
    known_session_ids: &HashSet<String>,
) -> Vec<String> {
    history
        .iter()
        .filter_map(|item| {
            if known_session_ids.contains(&item.session_id) {
                None
            } else {
                Some(item.session_id.clone())
            }
        })
        .collect()
}

fn unknown_granted_tool_findings(
    runtime: &RuntimeKernel,
    approval_sessions: &[SessionApprovalState],
) -> Vec<String> {
    approval_sessions
        .iter()
        .flat_map(|session| session.granted_tools.iter())
        .filter_map(|tool_name| {
            if runtime.tools.contains(tool_name) {
                None
            } else {
                Some(tool_name.clone())
            }
        })
        .collect()
}

fn unknown_pending_tool_findings(
    runtime: &RuntimeKernel,
    approval_sessions: &[SessionApprovalState],
) -> Vec<String> {
    approval_sessions
        .iter()
        .flat_map(|session| session.pending.iter())
        .filter_map(|approval| {
            if runtime.tools.contains(&approval.tool_name) {
                None
            } else {
                Some(approval.tool_name.clone())
            }
        })
        .collect()
}

fn orphan_session_model_findings(
    snapshot: &RuntimeSnapshot,
    known_session_ids: &HashSet<String>,
) -> Vec<String> {
    snapshot
        .session_models
        .iter()
        .filter_map(|binding| {
            if known_session_ids.contains(&binding.session_id) {
                None
            } else {
                Some(binding.session_id.clone())
            }
        })
        .collect()
}

fn unknown_session_model_findings(
    snapshot: &RuntimeSnapshot,
    available_models: &[ModelRef],
) -> Vec<String> {
    snapshot
        .session_models
        .iter()
        .filter_map(|binding| {
            if available_models
                .iter()
                .any(|candidate| candidate == &binding.selected_model)
            {
                None
            } else {
                Some(format!(
                    "{} -> {}/{}",
                    binding.session_id,
                    binding.selected_model.provider,
                    binding.selected_model.model
                ))
            }
        })
        .collect()
}

pub(super) fn topic_session_transcript_provenance_findings(
    snapshot: &RuntimeSnapshot,
) -> (Vec<String>, usize, usize) {
    let active_topic_sessions = snapshot
        .topic_sessions
        .iter()
        .filter(|topic_session| topic_session.status == TopicSessionStatus::Active)
        .collect::<Vec<_>>();
    let missing = active_topic_sessions
        .iter()
        .filter_map(|topic_session| {
            if topic_session.linked_transcript_spans.is_empty() {
                Some(topic_session.topic_session_id.clone())
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    let total = active_topic_sessions.len();
    let with_provenance = total.saturating_sub(missing.len());

    (missing, total, with_provenance)
}

fn snapshot_roundtrip_ok(snapshot: &RuntimeSnapshot) -> bool {
    serde_json::to_string(snapshot)
        .ok()
        .and_then(|json| serde_json::from_str::<RuntimeSnapshot>(&json).ok())
        .map(|parsed| parsed == *snapshot)
        .unwrap_or(false)
}

fn active_session_export_roundtrip_ok(runtime: &RuntimeKernel, session_id: &str) -> bool {
    runtime
        .session_export(session_id)
        .ok()
        .and_then(|export| serde_json::to_string(&export).ok())
        .and_then(|json| serde_json::from_str::<SessionExport>(&json).ok())
        .map(|export| export.session.session_id.0 == session_id)
        .unwrap_or(false)
}

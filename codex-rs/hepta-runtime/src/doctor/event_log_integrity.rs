use std::collections::HashSet;

use hepta_core::EventKind;

use super::DoctorCheck;
use super::integrity;
use crate::EventRecord;

pub(super) const SESSION_REFS_KNOWN: &str = "event log session references known";
pub(super) const MONOTONIC_TIMESTAMPS: &str = "event log timestamps monotonic";

pub(super) fn collect_event_log_integrity_checks(
    events: &[EventRecord],
    known_session_ids: &HashSet<String>,
) -> Vec<DoctorCheck> {
    let unknown_session_refs = unknown_session_ref_findings(events, known_session_ids);
    let checked_session_refs = events
        .iter()
        .filter(|record| record.event.session_id.is_some())
        .count();
    let descending_timestamps = descending_timestamp_findings(events);

    vec![
        integrity::doctor_check(
            SESSION_REFS_KNOWN,
            integrity::status_from_findings(&unknown_session_refs),
            integrity::joined_values_or_count(&unknown_session_refs, checked_session_refs),
        ),
        integrity::doctor_check(
            MONOTONIC_TIMESTAMPS,
            integrity::status_from_findings(&descending_timestamps),
            integrity::joined_values_or_count(&descending_timestamps, events.len()),
        ),
    ]
}

fn unknown_session_ref_findings(
    events: &[EventRecord],
    known_session_ids: &HashSet<String>,
) -> Vec<String> {
    events
        .iter()
        .filter_map(|record| {
            let session_id = record.event.session_id.as_ref()?;
            if known_session_ids.contains(&session_id.0)
                || is_seeded_bootstrap_event(record)
                || is_agent_runtime_scoped_event(record)
            {
                None
            } else {
                Some(format!(
                    "{}@{}",
                    event_label(record),
                    record.emitted_at_unix_ms
                ))
            }
        })
        .collect()
}

fn descending_timestamp_findings(events: &[EventRecord]) -> Vec<String> {
    events
        .windows(2)
        .filter_map(|pair| {
            let previous = &pair[0];
            let current = &pair[1];
            if current.emitted_at_unix_ms < previous.emitted_at_unix_ms {
                Some(format!(
                    "{}@{} after {}@{}",
                    event_label(current),
                    current.emitted_at_unix_ms,
                    event_label(previous),
                    previous.emitted_at_unix_ms
                ))
            } else {
                None
            }
        })
        .collect()
}

fn is_seeded_bootstrap_event(record: &EventRecord) -> bool {
    record.event.kind == EventKind::SessionStarted
        && matches!(
            record.event.session_id.as_ref(),
            Some(session_id) if session_id.0 == "bootstrap"
        )
}

fn is_agent_runtime_scoped_event(record: &EventRecord) -> bool {
    let Some(session_id) = record.event.session_id.as_ref() else {
        return false;
    };
    if !session_id.0.starts_with("agent:") {
        return false;
    }

    matches!(
        record.event.kind,
        EventKind::SessionStarted
            | EventKind::SessionDeleted
            | EventKind::MessageReceived
            | EventKind::ModelCalled
            | EventKind::ToolInvoked
            | EventKind::ApprovalRequested
            | EventKind::ApprovalGranted
            | EventKind::MemoryWritten
            | EventKind::AgentRegistered
            | EventKind::AgentMessageQueued
            | EventKind::AgentPaused
            | EventKind::AgentResumed
            | EventKind::AgentStopped
            | EventKind::AgentSteered
            | EventKind::AgentDrained
            | EventKind::AgentRunStarted
            | EventKind::AgentRunCompleted
            | EventKind::AgentRunFailed
    )
}

fn event_label(record: &EventRecord) -> String {
    match record.event.session_id.as_ref() {
        Some(session_id) => format!("{:?}/{}", record.event.kind, session_id.0),
        None => format!("{:?}", record.event.kind),
    }
}

#[cfg(test)]
mod tests {
    use hepta_core::Event;
    use hepta_core::SessionId;

    use super::super::DoctorStatus;
    use super::*;

    fn event_record(
        emitted_at_unix_ms: u64,
        kind: EventKind,
        session_id: Option<&str>,
    ) -> EventRecord {
        EventRecord {
            emitted_at_unix_ms,
            event: Event {
                kind,
                session_id: session_id.map(|session_id| SessionId(session_id.into())),
                agent_id: None,
                correlation_id: None,
                summary: "event integrity test".into(),
                payload: None,
            },
        }
    }

    #[test]
    fn event_log_integrity_allows_seeded_bootstrap_event() {
        let checks = collect_event_log_integrity_checks(
            &[event_record(
                1,
                EventKind::SessionStarted,
                Some("bootstrap"),
            )],
            &HashSet::from(["session-main".to_string()]),
        );

        let session_refs = checks
            .iter()
            .find(|check| check.name == SESSION_REFS_KNOWN)
            .expect("session reference check should exist");

        assert_eq!(session_refs.status, DoctorStatus::Ok);
        assert_eq!(session_refs.detail, "1");
    }

    #[test]
    fn event_log_integrity_flags_unknown_session_refs() {
        let checks = collect_event_log_integrity_checks(
            &[event_record(7, EventKind::MessageReceived, Some("missing"))],
            &HashSet::from(["session-main".to_string()]),
        );

        let session_refs = checks
            .iter()
            .find(|check| check.name == SESSION_REFS_KNOWN)
            .expect("session reference check should exist");

        assert_eq!(session_refs.status, DoctorStatus::Fail);
        assert_eq!(session_refs.detail, "MessageReceived/missing@7");
    }

    #[test]
    fn event_log_integrity_allows_agent_scoped_runtime_events() {
        let checks = collect_event_log_integrity_checks(
            &[
                event_record(7, EventKind::AgentRegistered, Some("agent:agent-77")),
                event_record(8, EventKind::MessageReceived, Some("agent:agent-77")),
                event_record(9, EventKind::SessionDeleted, Some("agent:agent-77")),
            ],
            &HashSet::from(["session-main".to_string()]),
        );

        let session_refs = checks
            .iter()
            .find(|check| check.name == SESSION_REFS_KNOWN)
            .expect("session reference check should exist");

        assert_eq!(session_refs.status, DoctorStatus::Ok);
        assert_eq!(session_refs.detail, "3");
    }

    #[test]
    fn event_log_integrity_flags_descending_timestamps() {
        let checks = collect_event_log_integrity_checks(
            &[
                event_record(42, EventKind::SessionSwitched, Some("session-main")),
                event_record(41, EventKind::MessageReceived, Some("session-main")),
            ],
            &HashSet::from(["session-main".to_string()]),
        );

        let monotonic = checks
            .iter()
            .find(|check| check.name == MONOTONIC_TIMESTAMPS)
            .expect("monotonic timestamp check should exist");

        assert_eq!(monotonic.status, DoctorStatus::Fail);
        assert_eq!(
            monotonic.detail,
            "MessageReceived/session-main@41 after SessionSwitched/session-main@42"
        );
    }
}

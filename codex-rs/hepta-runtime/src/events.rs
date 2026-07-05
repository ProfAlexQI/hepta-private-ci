use super::RuntimeKernel;
use super::current_unix_ms;
use hepta_core::CorrelationId;
use hepta_core::Event;
use hepta_core::EventKind;
use hepta_core::HeptaError;
use hepta_core::SessionId;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventRecord {
    pub emitted_at_unix_ms: u64,
    pub event: Event,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventQueryReport {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<EventKind>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    pub limit: usize,
    pub matched_count: usize,
    pub returned_count: usize,
    pub omitted_count: usize,
    pub truncated: bool,
    pub events: Vec<EventRecord>,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct EventState {
    records: Vec<EventRecord>,
}

impl EventState {
    pub(crate) fn new_with_boot_event() -> Self {
        Self {
            records: vec![EventRecord {
                emitted_at_unix_ms: current_unix_ms().unwrap_or(0),
                event: default_boot_event(),
            }],
        }
    }

    pub(crate) fn snapshot(&self) -> Vec<EventRecord> {
        self.records.clone()
    }

    pub(crate) fn replace(&mut self, records: Vec<EventRecord>) {
        self.records = records;
    }

    fn query(
        &self,
        limit: usize,
        kind: Option<&EventKind>,
        session_id: Option<&str>,
    ) -> Vec<EventRecord> {
        self.query_report(limit, kind, session_id).events
    }

    fn query_report(
        &self,
        limit: usize,
        kind: Option<&EventKind>,
        session_id: Option<&str>,
    ) -> EventQueryReport {
        let filtered = self
            .records
            .iter()
            .filter(|record| {
                kind.map(|kind| record.event.kind == *kind).unwrap_or(true)
                    && session_id
                        .map(|session_id| {
                            record
                                .event
                                .session_id
                                .as_ref()
                                .map(|record_session_id| record_session_id.0 == session_id)
                                .unwrap_or(false)
                        })
                        .unwrap_or(true)
            })
            .cloned()
            .collect::<Vec<_>>();
        let matched_count = filtered.len();
        let events = if limit == 0 || limit >= matched_count {
            filtered
        } else {
            filtered[matched_count - limit..].to_vec()
        };
        let returned_count = events.len();
        let omitted_count = matched_count.saturating_sub(returned_count);
        EventQueryReport {
            kind: kind.cloned(),
            session_id: session_id.map(str::to_owned),
            limit,
            matched_count,
            returned_count,
            omitted_count,
            truncated: omitted_count > 0,
            events,
        }
    }

    fn emit(
        &mut self,
        kind: EventKind,
        session_id: Option<SessionId>,
        correlation_id: Option<CorrelationId>,
        summary: String,
        payload: Option<Value>,
    ) -> Result<(), HeptaError> {
        self.records.push(EventRecord {
            emitted_at_unix_ms: current_unix_ms()?,
            event: Event {
                kind,
                session_id,
                agent_id: None,
                correlation_id,
                summary,
                payload,
            },
        });
        Ok(())
    }
}

fn default_boot_event() -> Event {
    Event {
        kind: EventKind::SessionStarted,
        session_id: Some(SessionId("bootstrap".into())),
        agent_id: None,
        correlation_id: None,
        summary: "hepta runtime booted".into(),
        payload: None,
    }
}

pub(crate) fn format_event_record(record: &EventRecord) -> String {
    let session_id = record
        .event
        .session_id
        .as_ref()
        .map(|session_id| session_id.0.as_str())
        .unwrap_or("global");
    format!(
        "  - {}: {:?}, {}",
        session_id,
        record.event.kind,
        summarize_line(&record.event.summary, 72)
    )
}

pub(crate) fn summarize_line(value: &str, max_chars: usize) -> String {
    let compact = value.split_whitespace().collect::<Vec<_>>().join(" ");
    if compact.chars().count() <= max_chars {
        compact
    } else {
        format!(
            "{}...",
            compact
                .chars()
                .take(max_chars.saturating_sub(3))
                .collect::<String>()
        )
    }
}

impl RuntimeKernel {
    pub fn boot_event(&self) -> Event {
        default_boot_event()
    }

    pub fn events(&self, limit: usize) -> Result<Vec<EventRecord>, HeptaError> {
        self.query_events(limit, None, None)
    }

    pub fn query_events(
        &self,
        limit: usize,
        kind: Option<&EventKind>,
        session_id: Option<&str>,
    ) -> Result<Vec<EventRecord>, HeptaError> {
        let guard = self
            .event_state
            .lock()
            .map_err(|_| HeptaError("event state mutex poisoned".into()))?;
        Ok(guard.query(limit, kind, session_id))
    }

    pub fn query_events_report(
        &self,
        limit: usize,
        kind: Option<&EventKind>,
        session_id: Option<&str>,
    ) -> Result<EventQueryReport, HeptaError> {
        let guard = self
            .event_state
            .lock()
            .map_err(|_| HeptaError("event state mutex poisoned".into()))?;
        Ok(guard.query_report(limit, kind, session_id))
    }

    pub(crate) fn emit_event(
        &self,
        kind: EventKind,
        session_id: Option<SessionId>,
        correlation_id: Option<CorrelationId>,
        summary: String,
    ) -> Result<(), HeptaError> {
        self.emit_event_with_payload(kind, session_id, correlation_id, summary, None)
    }

    pub(crate) fn emit_event_with_payload(
        &self,
        kind: EventKind,
        session_id: Option<SessionId>,
        correlation_id: Option<CorrelationId>,
        summary: String,
        payload: Option<Value>,
    ) -> Result<(), HeptaError> {
        let mut guard = self
            .event_state
            .lock()
            .map_err(|_| HeptaError("event state mutex poisoned".into()))?;
        guard.emit(kind, session_id, correlation_id, summary, payload)
    }
}

#[cfg(test)]
mod tests {
    use hepta_core::Event;
    use hepta_core::EventKind;
    use hepta_core::SessionId;

    use super::EventRecord;
    use super::EventState;
    use super::format_event_record;
    use super::summarize_line;

    #[test]
    fn format_event_record_uses_global_fallback_and_compacts_whitespace() {
        let record = EventRecord {
            emitted_at_unix_ms: 7,
            event: Event {
                kind: EventKind::MessageReceived,
                session_id: None,
                agent_id: None,
                correlation_id: None,
                summary: "multi\n  line\t summary".into(),
                payload: None,
            },
        };

        assert_eq!(
            format_event_record(&record),
            "  - global: MessageReceived, multi line summary"
        );
    }

    #[test]
    fn format_event_record_uses_session_scope_when_present() {
        let record = EventRecord {
            emitted_at_unix_ms: 11,
            event: Event {
                kind: EventKind::SessionRenamed,
                session_id: Some(SessionId("alpha".into())),
                agent_id: None,
                correlation_id: None,
                summary: "renamed alpha workspace".into(),
                payload: None,
            },
        };

        assert_eq!(
            format_event_record(&record),
            "  - alpha: SessionRenamed, renamed alpha workspace"
        );
    }

    #[test]
    fn summarize_line_compacts_and_truncates_shared_rendering_text() {
        assert_eq!(
            summarize_line("multi\n  line\t summary", 32),
            "multi line summary"
        );
        assert_eq!(
            summarize_line("alpha beta gamma delta epsilon", 13),
            "alpha beta..."
        );
    }

    #[test]
    fn event_query_report_preserves_match_counts_and_truncation_metadata() {
        let mut state = EventState {
            records: Vec::new(),
        };
        for (session, summary) in [("alpha", "one"), ("beta", "two"), ("alpha", "three")] {
            state
                .emit(
                    EventKind::SessionSwitched,
                    Some(SessionId(session.into())),
                    None,
                    summary.into(),
                    None,
                )
                .expect("event should emit");
        }

        let report = state.query_report(1, Some(&EventKind::SessionSwitched), Some("alpha"));

        assert_eq!(report.kind, Some(EventKind::SessionSwitched));
        assert_eq!(report.session_id.as_deref(), Some("alpha"));
        assert_eq!(report.limit, 1);
        assert_eq!(report.matched_count, 2);
        assert_eq!(report.returned_count, 1);
        assert_eq!(report.omitted_count, 1);
        assert!(report.truncated);
        assert_eq!(report.events[0].event.summary, "three");

        let unlimited = state.query_report(0, Some(&EventKind::SessionSwitched), Some("alpha"));
        assert_eq!(unlimited.matched_count, 2);
        assert_eq!(unlimited.returned_count, 2);
        assert_eq!(unlimited.omitted_count, 0);
        assert!(!unlimited.truncated);
    }
}

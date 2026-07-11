use std::collections::BTreeMap;

use super::EventRecord;
use super::RuntimeEventDigest;
use super::RuntimeEventKindTally;
use super::RuntimeEventSessionTally;

pub(super) fn build(events: Vec<EventRecord>) -> RuntimeEventDigest {
    let kinds = tally_kinds(&events);
    let sessions = tally_sessions(&events);

    RuntimeEventDigest {
        events,
        kinds,
        sessions,
    }
}

fn tally_kinds(events: &[EventRecord]) -> Vec<RuntimeEventKindTally> {
    let mut kinds = BTreeMap::<String, usize>::new();
    for record in events {
        *kinds.entry(format!("{:?}", record.event.kind)).or_default() += 1;
    }

    let mut kinds = kinds
        .into_iter()
        .map(|(kind, count)| RuntimeEventKindTally { kind, count })
        .collect::<Vec<_>>();
    kinds.sort_by(|left, right| {
        right
            .count
            .cmp(&left.count)
            .then_with(|| left.kind.cmp(&right.kind))
    });
    kinds
}

fn tally_sessions(events: &[EventRecord]) -> Vec<RuntimeEventSessionTally> {
    let mut sessions = BTreeMap::<Option<String>, RuntimeEventSessionTally>::new();

    for record in events {
        let session_id = record.event.session_id.as_ref().map(|id| id.0.clone());
        let entry =
            sessions
                .entry(session_id.clone())
                .or_insert_with(|| RuntimeEventSessionTally {
                    session_id: session_id.clone(),
                    count: 0,
                    latest_event: record.clone(),
                });
        entry.count += 1;
        entry.latest_event = record.clone();
    }

    let mut sessions = sessions.into_values().collect::<Vec<_>>();
    sessions.sort_by(|left, right| {
        right
            .count
            .cmp(&left.count)
            .then_with(|| session_label(&left.session_id).cmp(session_label(&right.session_id)))
    });
    sessions
}

fn session_label(session_id: &Option<String>) -> &str {
    session_id.as_deref().unwrap_or("global")
}

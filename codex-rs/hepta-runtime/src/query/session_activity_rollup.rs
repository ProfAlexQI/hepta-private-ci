use super::RuntimeSessionActivityOverview;
use super::RuntimeSessionActivitySlice;

pub(super) fn build(sessions: Vec<RuntimeSessionActivitySlice>) -> RuntimeSessionActivityOverview {
    let active_sessions = count_matching(&sessions, |activity| activity.session.is_active);
    let archived_sessions = count_matching(&sessions, |activity| {
        activity.session.archived_at_unix_ms.is_some()
    });
    let sessions_with_history = count_matching(&sessions, |activity| !activity.history.is_empty());
    let sessions_with_events = count_matching(&sessions, |activity| !activity.events.is_empty());
    let sessions_with_topic_state = count_matching(&sessions, |activity| {
        activity.session.topic_session_count > 0 || activity.session.topic_graph_edge_count > 0
    });
    let total_topic_sessions = sessions
        .iter()
        .map(|activity| activity.session.topic_session_count)
        .sum();
    let total_topic_graph_edges = sessions
        .iter()
        .map(|activity| activity.session.topic_graph_edge_count)
        .sum();

    RuntimeSessionActivityOverview {
        sessions,
        active_sessions,
        archived_sessions,
        sessions_with_history,
        sessions_with_events,
        sessions_with_topic_state,
        total_topic_sessions,
        total_topic_graph_edges,
    }
}

fn count_matching(
    sessions: &[RuntimeSessionActivitySlice],
    predicate: impl Fn(&RuntimeSessionActivitySlice) -> bool,
) -> usize {
    sessions
        .iter()
        .filter(|activity| predicate(activity))
        .count()
}

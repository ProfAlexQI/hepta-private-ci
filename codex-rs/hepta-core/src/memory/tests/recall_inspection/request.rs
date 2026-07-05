use super::*;

#[test]
fn context_recall_request_normalizes_blank_queries_and_builds_queries() {
    let request = ContextRecallRequest {
        session_id: SessionId("session-42".into()),
        query_text: Some("   ".into()),
        recent_window_limit: 8,
        transcript_limit: 3,
        memory_limit: 2,
        allow_cross_session: true,
    };

    assert_eq!(request.normalized_query_text(), None);
    assert!(!request.has_query_text());
    assert_eq!(
        request.transcript_query(),
        TranscriptQuery {
            session_id: Some(SessionId("session-42".into())),
            text: String::new(),
            limit: 3,
        }
    );
    assert_eq!(
        request.memory_query(),
        MemoryQuery {
            text: String::new(),
            limit: 2,
        }
    );
}

#[test]
fn context_recall_request_cross_session_flag_does_not_change_portable_queries() {
    let session_scoped = ContextRecallRequest {
        session_id: SessionId("session-42".into()),
        query_text: Some(" timeout ".into()),
        recent_window_limit: 8,
        transcript_limit: 3,
        memory_limit: 2,
        allow_cross_session: false,
    };
    let mut cross_session = session_scoped.clone();
    cross_session.allow_cross_session = true;

    assert_eq!(
        session_scoped.transcript_query(),
        cross_session.transcript_query()
    );
    assert_eq!(session_scoped.memory_query(), cross_session.memory_query());
    assert_eq!(
        cross_session.transcript_query(),
        TranscriptQuery {
            session_id: Some(SessionId("session-42".into())),
            text: "timeout".into(),
            limit: 3,
        }
    );
    assert_eq!(
        cross_session.memory_query(),
        MemoryQuery {
            text: "timeout".into(),
            limit: 2,
        }
    );
}

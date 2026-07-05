use super::*;

#[test]
fn session_record_roundtrips_through_json() {
    let record = SessionRecord {
        session_id: SessionId("session-42".into()),
        agent_id: AgentId("builder".into()),
        title: "Foundation lane".into(),
        created_at_unix_ms: 1,
        last_active_unix_ms: 2,
        last_user_intent_summary: Some("stabilize contracts".into()),
        archived_at_unix_ms: None,
    };

    let json = serde_json::to_string(&record).expect("session record should serialize");
    let parsed: SessionRecord =
        serde_json::from_str(&json).expect("session record should deserialize");

    assert_eq!(parsed, record);
}

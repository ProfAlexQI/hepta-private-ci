use super::*;

#[test]
fn session_agent_inventory_rolls_up_sessions_by_agent() {
    let inventory = SessionAgentInventory::from_records(&[
        SessionRecord {
            session_id: SessionId("session-2".into()),
            agent_id: AgentId("reviewer".into()),
            title: "Reviewer lane".into(),
            created_at_unix_ms: 1,
            last_active_unix_ms: 40,
            last_user_intent_summary: None,
            archived_at_unix_ms: Some(50),
        },
        SessionRecord {
            session_id: SessionId("session-1".into()),
            agent_id: AgentId("builder".into()),
            title: "Builder lane".into(),
            created_at_unix_ms: 2,
            last_active_unix_ms: 10,
            last_user_intent_summary: Some("stabilize contracts".into()),
            archived_at_unix_ms: None,
        },
        SessionRecord {
            session_id: SessionId("session-3".into()),
            agent_id: AgentId("builder".into()),
            title: "Builder archive".into(),
            created_at_unix_ms: 3,
            last_active_unix_ms: 25,
            last_user_intent_summary: None,
            archived_at_unix_ms: Some(30),
        },
        SessionRecord {
            session_id: SessionId("session-4".into()),
            agent_id: AgentId("   ".into()),
            title: "Blank agent".into(),
            created_at_unix_ms: 4,
            last_active_unix_ms: 60,
            last_user_intent_summary: None,
            archived_at_unix_ms: None,
        },
    ]);

    assert_eq!(inventory.total_session_count, 4);
    assert_eq!(inventory.blank_agent_id_session_count, 1);
    assert_eq!(inventory.agent_count(), 2);
    assert_eq!(inventory.inventoried_session_count(), 3);
    assert!(!inventory.is_empty());
    assert_eq!(inventory.agents[0].agent_id.0, "builder");
    assert_eq!(inventory.agents[0].session_count, 2);
    assert_eq!(inventory.agents[0].active_session_count, 1);
    assert_eq!(inventory.agents[0].archived_session_count, 1);
    assert_eq!(inventory.agents[0].latest_activity_unix_ms, 25);
    assert_eq!(inventory.agents[1].agent_id.0, "reviewer");
    assert_eq!(inventory.agents[1].session_count, 1);
    assert_eq!(inventory.agents[1].active_session_count, 0);
    assert_eq!(inventory.agents[1].archived_session_count, 1);
    assert_eq!(inventory.agents[1].latest_activity_unix_ms, 40);
}

#[test]
fn session_agent_inventory_roundtrips_through_json() {
    let inventory = SessionAgentInventory::from_records(&[
        SessionRecord {
            session_id: SessionId("session-1".into()),
            agent_id: AgentId("builder".into()),
            title: "Foundation lane".into(),
            created_at_unix_ms: 1,
            last_active_unix_ms: 2,
            last_user_intent_summary: Some("inventory roundtrip".into()),
            archived_at_unix_ms: None,
        },
        SessionRecord {
            session_id: SessionId("session-2".into()),
            agent_id: AgentId("reviewer".into()),
            title: "Review lane".into(),
            created_at_unix_ms: 3,
            last_active_unix_ms: 4,
            last_user_intent_summary: None,
            archived_at_unix_ms: Some(5),
        },
    ]);

    let json = serde_json::to_string(&inventory).expect("inventory should serialize");
    let parsed: SessionAgentInventory =
        serde_json::from_str(&json).expect("inventory should deserialize");

    assert_eq!(parsed, inventory);
}

#[test]
fn session_agent_inventory_deserializes_from_sparse_json() {
    let parsed: SessionAgentInventory =
        serde_json::from_str("{}").expect("sparse inventory should deserialize with defaults");

    assert_eq!(parsed, SessionAgentInventory::default());
    assert_eq!(parsed.agent_count(), 0);
    assert_eq!(parsed.inventoried_session_count(), 0);
    assert!(parsed.is_empty());
}

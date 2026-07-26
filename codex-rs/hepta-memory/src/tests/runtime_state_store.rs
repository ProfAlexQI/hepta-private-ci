use std::os::unix::fs::PermissionsExt;

use hepta_core::AgentId;
use hepta_core::MemoryRecord;
use hepta_core::MemoryScope;
use hepta_core::MessageRole;
use hepta_core::SessionId;
use hepta_core::SessionRecord;
use hepta_core::TranscriptEntry;
use hepta_core::TranscriptEntryKind;

use crate::DurableIntegrityKey;
use crate::InMemoryStore;

fn key(byte: u8) -> DurableIntegrityKey {
    DurableIntegrityKey::from_bytes([byte; 32])
}

fn session(session_id: &str, title: &str) -> SessionRecord {
    SessionRecord {
        session_id: SessionId(session_id.to_string()),
        agent_id: AgentId("runtime-state-test".into()),
        title: title.to_string(),
        created_at_unix_ms: 1,
        last_active_unix_ms: 2,
        last_user_intent_summary: None,
        archived_at_unix_ms: None,
    }
}

#[test]
fn durable_store_recovers_committed_session_state() {
    let root = tempfile::tempdir().expect("tempdir");
    std::fs::set_permissions(root.path(), std::fs::Permissions::from_mode(0o700))
        .expect("secure root");
    let path = root.path().join("runtime-state.json");

    let store = InMemoryStore::bootstrap_durable(&path, key(7)).expect("bootstrap state");
    let snapshot = crate::StoreSnapshot {
        sessions: vec![session("session-a", "persisted")],
        memories: vec![MemoryRecord {
            id: "memory-a".into(),
            scope: MemoryScope::LongTerm,
            content: "remembered".into(),
        }],
        transcripts: vec![TranscriptEntry {
            entry_id: "entry-a".into(),
            session_id: SessionId("session-a".into()),
            sequence: 1,
            kind: TranscriptEntryKind::Message,
            role: Some(MessageRole::User),
            content: "persisted transcript".into(),
            created_at_unix_ms: 3,
            tool_name: None,
            correlation_id: None,
            summary_of_range: None,
        }],
    };
    store.restore(snapshot.clone()).expect("commit state");
    drop(store);

    let recovered = InMemoryStore::open_durable(&path, key(7)).expect("recover state");
    assert_eq!(recovered.snapshot().expect("snapshot state"), snapshot);
}

#[test]
fn durable_store_rejects_wrong_key_and_payload_tampering() {
    let root = tempfile::tempdir().expect("tempdir");
    std::fs::set_permissions(root.path(), std::fs::Permissions::from_mode(0o700))
        .expect("secure root");
    let path = root.path().join("runtime-state.json");
    let store = InMemoryStore::bootstrap_durable(&path, key(9)).expect("bootstrap state");
    store
        .upsert_session_sync(session("session-a", "original"))
        .expect("commit session");
    drop(store);

    assert!(InMemoryStore::open_durable(&path, key(8)).is_err());

    let mut envelope: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&path).expect("read state")).expect("parse state");
    envelope["payload"]["snapshot"]["sessions"][0]["title"] =
        serde_json::Value::String("tampered".into());
    std::fs::write(
        &path,
        serde_json::to_vec(&envelope).expect("serialize tamper"),
    )
    .expect("write tamper");
    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600)).expect("restore mode");

    assert!(InMemoryStore::open_durable(&path, key(9)).is_err());
}

#[test]
fn durable_store_rejects_replaced_path_before_next_commit() {
    let root = tempfile::tempdir().expect("tempdir");
    std::fs::set_permissions(root.path(), std::fs::Permissions::from_mode(0o700))
        .expect("secure root");
    let path = root.path().join("runtime-state.json");
    let store = InMemoryStore::bootstrap_durable(&path, key(3)).expect("bootstrap state");
    let bytes = std::fs::read(&path).expect("read state");
    std::fs::remove_file(&path).expect("remove state path");
    std::fs::write(&path, bytes).expect("replace state path");
    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600))
        .expect("secure replacement");

    let error = store
        .upsert_session_sync(session("session-a", "must-not-commit"))
        .expect_err("replacement must fail closed");
    assert!(error.0.contains("path identity changed"));
    assert!(store.list_sessions().expect("list sessions").is_empty());
}

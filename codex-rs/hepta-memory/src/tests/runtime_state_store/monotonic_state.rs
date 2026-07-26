use super::*;

#[test]
fn durable_store_exposes_authenticated_monotonic_state() {
    let root = tempfile::tempdir().expect("tempdir");
    std::fs::set_permissions(root.path(), std::fs::Permissions::from_mode(0o700))
        .expect("secure root");
    let path = root.path().join("runtime-state.json");
    let store = InMemoryStore::bootstrap_durable(&path, key(11)).expect("bootstrap state");
    let initial = store
        .runtime_state_monotonic_state()
        .expect("initial monotonic state")
        .expect("durable state");
    assert_eq!(initial.generation(), 0);
    assert!(initial.state_hash().starts_with("sha256:"));

    store
        .upsert_session_sync(session("session-a", "anchored"))
        .expect("commit session");
    let advanced = store
        .runtime_state_monotonic_state()
        .expect("advanced monotonic state")
        .expect("durable state");
    assert_eq!(advanced.generation(), 1);
    assert_ne!(advanced.state_hash(), initial.state_hash());

    let ephemeral = InMemoryStore::default();
    assert_eq!(
        ephemeral
            .runtime_state_monotonic_state()
            .expect("ephemeral projection"),
        None
    );
}

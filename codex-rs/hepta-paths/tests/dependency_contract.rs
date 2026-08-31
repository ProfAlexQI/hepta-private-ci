use codex_hepta_paths::HeptaStateRoot;
use pretty_assertions::assert_eq;

#[test]
fn temporary_state_root_keeps_runtime_layout_inside_selected_domain() {
    let temp = tempfile::tempdir().expect("temporary directory");
    let state_path = temp.path().join("state");
    let root = HeptaStateRoot::parse(state_path.clone()).expect("valid temporary state root");
    let layout = root.layout();

    assert_eq!(root.as_path(), state_path.as_path());
    assert!(layout.runtime_root().starts_with(root.as_path()));
    assert!(layout.outcomes_database().starts_with(root.as_path()));
    assert!(layout.preferences_database().starts_with(root.as_path()));
}

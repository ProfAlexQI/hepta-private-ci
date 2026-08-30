use codex_hepta_contracts::GovernanceMode;
use pretty_assertions::assert_eq;

use super::GovernanceState;

#[test]
fn disabled_state_is_explicitly_shadow_and_unavailable() {
    let state = GovernanceState::disabled();

    assert!(!state.enabled);
    assert_eq!(&state.mode, &GovernanceMode::Shadow);
    match &state.evidence {
        Ok(_) => panic!("disabled governance must not expose an evidence store"),
        Err(error) => assert_eq!(error.as_ref(), "governance disabled"),
    }

    let claims = state.claims.lock().expect("governance claims lock");
    assert_eq!(claims.owned.len(), 0);
    assert_eq!(claims.blocked_replays.len(), 0);
}

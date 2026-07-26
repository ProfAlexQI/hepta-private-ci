use super::*;

fn spec(capability: &'static str, boundary: &'static str) -> GateSpec {
    GateSpec {
        method: "GET",
        pattern: "/api/test",
        source_command: "/test --json",
        capability,
        side_effect_boundary: boundary,
    }
}

#[test]
fn gate_spec_centralizes_route_guard_semantics() {
    let dry_run = GateSpec {
        method: "POST",
        pattern: "/api/actions/test",
        source_command: "/actions test --dry-run --json",
        capability: "test-action",
        side_effect_boundary: "dry-run plan only; no mutation",
    };

    assert!(dry_run.is_post());
    assert!(dry_run.is_dry_run());
    assert!(dry_run.is_guarded());
    assert!(dry_run.requires_confirmation());
    assert!(!dry_run.is_read_only());
}

#[test]
fn receipt_state_machine_prefers_the_most_advanced_declared_state() {
    assert_eq!(
        spec("packet-precondition", "read-only precondition").receipt_state(),
        Some(ReceiptState::Precondition)
    );
    assert_eq!(
        spec("result-receipt-denial", "read-only denial").receipt_state(),
        Some(ReceiptState::Receipt)
    );
    assert_eq!(
        spec(
            "result-receipt-retention-expiry-denial",
            "read-only receipt retention denial"
        )
        .receipt_state(),
        Some(ReceiptState::Retention)
    );
    assert_eq!(
        spec(
            "result-receipt-terminal-decision-denial",
            "read-only receipt terminal denial"
        )
        .receipt_state(),
        Some(ReceiptState::Terminal)
    );
    assert_eq!(ReceiptStateMachine::ORDERED_STATES.len(), 6);
    assert!(ReceiptStateMachine::contains_label("terminal"));
    assert!(!ReceiptStateMachine::contains_label("unknown"));
}

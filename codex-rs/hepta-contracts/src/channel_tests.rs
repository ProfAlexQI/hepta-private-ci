use super::*;

fn digest(value: &str) -> Sha256Digest {
    Sha256Digest::for_bytes(value.as_bytes())
}

fn scope() -> ChannelScope {
    ChannelScope {
        adapter_id: ChannelAdapterId::new("test-adapter").expect("valid adapter"),
        installation_sha256: digest("installation"),
        account_sha256: digest("account"),
        conversation_sha256: digest("conversation"),
        principal_sha256: digest("principal"),
    }
}
#[test]
fn ingress_event_uses_source_identity_and_digest_only_payload_cursor() {
    let event = ChannelIngressEvent::new(
        scope(),
        digest("provider-event-7"),
        digest("private message body"),
        channel_target_thread_sha256("thread-1").expect("target thread"),
        Some(digest("cursor-6")),
        digest("cursor-7"),
        1_000,
    )
    .expect("valid event");
    validate_ingress_event(&event).expect("valid binding");
    let mut changed = event.clone();
    changed.scope.conversation_sha256 = digest("other conversation");
    assert_ne!(
        event.event_id,
        ChannelIngressEventId::for_event(&changed.scope, &changed.source_event_sha256),
    );
    let serialized = serde_json::to_string(&event).expect("serialize event");
    assert!(!serialized.contains("private message body"));
    assert!(!serialized.contains("provider-event-7"));
    assert!(!serialized.contains("cursor-7"));
}

#[test]
fn ingress_cursor_advances_only_for_definitive_terminals() {
    assert!(
        ChannelIngressTerminal::Accepted {
            thread_id: "thread-1".to_string(),
            turn_id: "turn-1".to_string(),
        }
        .advances_cursor()
    );
    assert!(
        ChannelIngressTerminal::Rejected {
            reason_code: "unsupported_event".to_string(),
        }
        .advances_cursor()
    );
    assert!(
        !ChannelIngressTerminal::Indeterminate {
            reason_code: "app_server_timeout".to_string(),
        }
        .advances_cursor()
    );
}

#[test]
fn accepted_ingress_receipt_must_match_the_claimed_target_thread() {
    let event = ChannelIngressEvent::new(
        scope(),
        digest("event"),
        digest("payload"),
        channel_target_thread_sha256("thread-1").expect("target thread"),
        None,
        digest("cursor"),
        1,
    )
    .expect("valid event");
    let substituted = ChannelIngressReceipt::new(
        event,
        ChannelIngressTerminal::Accepted {
            thread_id: "thread-2".to_string(),
            turn_id: "turn-1".to_string(),
        },
    );

    assert!(validate_ingress_receipt(&substituted).is_err());
}
#[test]
fn adapter_and_reason_codes_reject_free_form_channel_data() {
    assert!(ChannelAdapterId::new("Telegram Bot").is_err());
    let event = ChannelIngressEvent::new(
        scope(),
        digest("event"),
        digest("payload"),
        channel_target_thread_sha256("thread-1").expect("target thread"),
        None,
        digest("cursor"),
        1,
    )
    .expect("valid event");
    let invalid = ChannelIngressReceipt::new(
        event,
        ChannelIngressTerminal::Rejected {
            reason_code: "Bad Request with raw text".to_string(),
        },
    );
    assert!(validate_ingress_receipt(&invalid).is_err());
}

#[test]
fn ingress_ids_have_fixed_canonical_oracles() {
    let event = ChannelIngressEvent::new(
        scope(),
        digest("provider-event-7"),
        digest("private message body"),
        channel_target_thread_sha256("thread-1").expect("target thread"),
        Some(digest("cursor-6")),
        digest("cursor-7"),
        1_000,
    )
    .expect("valid event");
    let receipt = ChannelIngressReceipt::new(
        event.clone(),
        ChannelIngressTerminal::Rejected {
            reason_code: "unsupported_event".to_string(),
        },
    );
    assert_eq!(
        event.event_id.as_str(),
        "channel-ingress:v1:0b6656dd14e090d53e829c69c694d131a72c13afc0600776b2852d4109316e57"
    );
    assert_eq!(
        event.target_thread_sha256.as_str(),
        "d3be8dd555f0f307456ff0ef91aec643fb3da1ae27fc40056ba4b0d86122e251"
    );
    assert_eq!(
        receipt.receipt_id.as_str(),
        "channel-ingress-receipt:v1:026297b76c311cfc5216c85a073874f65b7edfdd323c49611d9a7dbf9fcc8ff6"
    );

    let mut rebound = event.clone();
    rebound.target_thread_sha256 = channel_target_thread_sha256("thread-2").expect("target");
    assert_eq!(rebound.event_id, event.event_id);
    assert_ne!(rebound, event);
}

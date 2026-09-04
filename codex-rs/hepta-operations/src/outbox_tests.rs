use super::*;

fn stable_id(value: &str) -> StableId {
    let result = StableId::new(value);
    let Ok(value) = result else {
        panic!("test id rejected");
    };
    value
}

fn generation(value: u64) -> Generation {
    let result = Generation::new(value);
    let Ok(value) = result else {
        panic!("test generation rejected");
    };
    value
}

fn intent() -> OutboxIntent {
    OutboxIntent {
        intent_id: stable_id("intent:1"),
        operation_id: stable_id("operation:1"),
        destination: stable_id("cognitive.store"),
        payload_digest: Digest32::of_bytes(b"payload"),
    }
}

#[test]
fn claim_and_ack_are_generation_fenced() {
    let intent = intent();
    let mut outbox = Outbox::default();
    assert_eq!(outbox.enqueue(intent.clone()), Ok(()));
    assert!(outbox.claim(&intent.intent_id, generation(4)).is_ok());
    assert_eq!(
        outbox.acknowledge(
            &intent.intent_id,
            generation(3),
            Digest32::of_bytes(b"ack"),
        ),
        Err(OperationError::StaleGeneration)
    );
    assert_eq!(
        outbox.acknowledge(
            &intent.intent_id,
            generation(4),
            Digest32::of_bytes(b"ack"),
        ),
        Ok(())
    );
}

#[test]
fn exact_enqueue_and_ack_replay_are_idempotent() {
    let intent = intent();
    let mut outbox = Outbox::default();
    assert_eq!(outbox.enqueue(intent.clone()), Ok(()));
    assert_eq!(outbox.enqueue(intent.clone()), Ok(()));
    assert!(outbox.claim(&intent.intent_id, generation(4)).is_ok());
    let ack = Digest32::of_bytes(b"ack");
    assert_eq!(outbox.acknowledge(&intent.intent_id, generation(4), ack), Ok(()));
    assert_eq!(outbox.acknowledge(&intent.intent_id, generation(4), ack), Ok(()));
}

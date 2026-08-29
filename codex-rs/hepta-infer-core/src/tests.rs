use std::collections::HashSet;

use super::*;

fn must<T>(result: Result<T>) -> T {
    match result {
        Ok(value) => value,
        Err(error) => panic!("unexpected error: {error}"),
    }
}

fn digest(fill: char) -> Digest {
    must(Digest::parse(&format!(
        "sha256:{}",
        fill.to_string().repeat(64)
    )))
}

fn request(name: &str, tenant: &str, tuple: Digest) -> InferenceRequest {
    InferenceRequest {
        identity: RequestIdentity {
            tenant_id: must(TenantId::parse(tenant)),
            workspace_id: must(WorkspaceId::parse("workspace-a")),
            agent_id: must(AgentId::parse("agent-a")),
            task_id: must(TaskId::parse("task-a")),
            request_id: must(RequestId::parse(name)),
        },
        agent_generation: 1,
        request_generation: 1,
        cancel_generation: 0,
        deadline_unix_ms: 10_000,
        model_tuple_digest: tuple,
        policy_digest: digest('b'),
        resource_budget_id: must(ResourceBudgetId::parse("budget-a")),
        prompt_digest: digest('c'),
        prompt_byte_length: 12,
        output_token_limit: 64,
        authority: AuthoritySnapshot::qualification_only_closed(),
    }
}

fn controller(tuple: Digest) -> Controller {
    let mut tuples = HashSet::new();
    tuples.insert(tuple);
    must(Controller::new(
        ControllerConfig {
            max_queue: 4,
            max_per_tenant: 2,
            registered_tuples: tuples,
            authority: AuthoritySnapshot::qualification_only_closed(),
        },
        7,
    ))
}

#[test]
fn canonical_protocol_round_trips_without_payload_text() {
    let message = ClientMessage::Admit(request("request-a", "tenant-a", digest('a')));
    let encoded = must(message.encode_canonical());
    assert_eq!(must(ClientMessage::decode_canonical(&encoded)), message);

    let mut non_canonical = vec![0x98, 18];
    non_canonical.extend_from_slice(&encoded[1..]);
    assert_eq!(
        ClientMessage::decode_canonical(&non_canonical),
        Err(InferError::ProtocolNonCanonical)
    );
}

#[test]
fn unknown_tuple_and_authority_escalation_fail_before_queueing() {
    let tuple = digest('a');
    let mut controller = controller(tuple.clone());
    let unknown = request("request-unknown", "tenant-a", digest('d'));
    assert_eq!(
        controller.admit(unknown, 1),
        Err(InferError::UnknownModelTuple)
    );

    let mut elevated = request("request-elevated", "tenant-a", tuple);
    elevated.authority.production_writer = true;
    assert_eq!(
        controller.admit(elevated, 1),
        Err(InferError::AuthorityEscalation)
    );
    assert_eq!(controller.snapshot().queued_requests, 0);
}

#[test]
fn queue_is_bounded_per_tenant_and_globally() {
    let tuple = digest('a');
    let mut controller = controller(tuple.clone());
    must(controller.admit(request("request-1", "tenant-a", tuple.clone()), 1));
    must(controller.admit(request("request-2", "tenant-a", tuple.clone()), 1));
    assert_eq!(
        controller.admit(request("request-3", "tenant-a", tuple.clone()), 1),
        Err(InferError::TenantQueueFull)
    );
    must(controller.admit(request("request-3", "tenant-b", tuple.clone()), 1));
    must(controller.admit(request("request-4", "tenant-c", tuple.clone()), 1));
    assert_eq!(
        controller.admit(request("request-5", "tenant-d", tuple), 1),
        Err(InferError::QueueFull)
    );
}

#[test]
fn cancellation_fence_is_strict_and_terminal_is_immutable() {
    let tuple = digest('a');
    let mut controller = controller(tuple.clone());
    let request_id = must(RequestId::parse("request-cancel"));
    must(controller.admit(request(request_id.as_str(), "tenant-a", tuple), 1));
    assert_eq!(
        controller.cancel(&request_id, 1, 0, 7),
        Err(InferError::StaleCancelGeneration)
    );
    let receipt = must(controller.cancel(&request_id, 1, 1, 7));
    assert_eq!(receipt.terminal_state, LifecycleState::Cancelled);
    assert_eq!(
        receipt.authority,
        AuthoritySnapshot::qualification_only_closed()
    );
    assert_eq!(
        controller.cancel(&request_id, 1, 2, 7),
        Err(InferError::TerminalState)
    );
}

#[test]
fn restart_invalidates_old_generation_and_forces_terminal_receipts() {
    let tuple = digest('a');
    let mut controller = controller(tuple.clone());
    let request_id = must(RequestId::parse("request-restart"));
    must(controller.admit(request(request_id.as_str(), "tenant-a", tuple), 1));
    must(controller.start(&request_id, 1, 7));
    let receipts = must(controller.restart_backend(7));
    assert_eq!(controller.backend_generation(), 8);
    assert_eq!(receipts.len(), 1);
    assert!(receipts[0].forced_worker_termination);
    assert_eq!(receipts[0].terminal_state, LifecycleState::FailedClosed);
    assert_eq!(
        controller.publish_token(
            EventFence {
                request_id: &request_id,
                request_generation: 1,
                backend_generation: 7,
                sequence: 3,
            },
            &digest('d'),
            1,
        ),
        Err(InferError::StaleBackendGeneration)
    );
}

#[test]
fn deterministic_two_tenant_replay_produces_identical_receipts() {
    fn replay(tuple: Digest) -> Vec<TerminalReceipt> {
        let mut controller = controller(tuple.clone());
        let left = must(RequestId::parse("request-left"));
        let right = must(RequestId::parse("request-right"));
        must(controller.admit(request(left.as_str(), "tenant-left", tuple.clone()), 1));
        must(controller.admit(request(right.as_str(), "tenant-right", tuple), 1));
        must(controller.start(&left, 1, 7));
        must(controller.start(&right, 1, 7));
        vec![
            must(controller.complete(
                EventFence {
                    request_id: &left,
                    request_generation: 1,
                    backend_generation: 7,
                    sequence: 3,
                },
                digest('d'),
                4,
            )),
            must(controller.complete(
                EventFence {
                    request_id: &right,
                    request_generation: 1,
                    backend_generation: 7,
                    sequence: 3,
                },
                digest('e'),
                4,
            )),
        ]
    }

    assert_eq!(replay(digest('a')), replay(digest('a')));
}

#[test]
fn response_receipt_round_trip_preserves_generation_fence() {
    let receipt = TerminalReceipt {
        request_id: must(RequestId::parse("request-receipt")),
        request_generation: 2,
        cancel_generation: 3,
        backend_generation: 4,
        terminal_state: LifecycleState::Completed,
        last_sequence: 5,
        output_tokens: 6,
        result_digest: Some(digest('f')),
        forced_worker_termination: false,
        authority: AuthoritySnapshot::qualification_only_closed(),
    };
    let message = ServerMessage::Receipt(receipt);
    let encoded = must(message.encode_canonical());
    assert_eq!(must(ServerMessage::decode_canonical(&encoded)), message);
}

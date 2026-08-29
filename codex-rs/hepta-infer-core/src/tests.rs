use std::collections::HashSet;

use super::*;

fn must<T, E: std::fmt::Display>(result: std::result::Result<T, E>) -> T {
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

fn request(
    tuple: Digest,
    request_id: &str,
    tenant: &str,
    output_token_limit: u32,
    deadline_unix_ms: u64,
) -> InferenceRequest {
    InferenceRequest {
        identity: RequestIdentity {
            tenant_id: must(TenantId::parse(tenant)),
            workspace_id: must(WorkspaceId::parse("workspace-a")),
            agent_id: must(AgentId::parse("agent-a")),
            task_id: must(TaskId::parse("task-a")),
            request_id: must(RequestId::parse(request_id)),
        },
        agent_generation: 1,
        request_generation: 1,
        cancel_generation: 0,
        deadline_unix_ms,
        model_tuple_digest: tuple,
        policy_digest: digest('b'),
        resource_budget_id: must(ResourceBudgetId::parse("budget-a")),
        prompt_digest: digest('c'),
        prompt_byte_length: 12,
        output_token_limit,
        authority: AuthoritySnapshot::qualification_only_closed(),
    }
}

fn controller(tuple: Digest, max_queue: usize, max_per_tenant: usize) -> Controller {
    let mut registered_tuples = HashSet::new();
    registered_tuples.insert(tuple);
    must(Controller::new(
        ControllerConfig {
            max_queue,
            max_per_tenant,
            registered_tuples,
            authority: AuthoritySnapshot::qualification_only_closed(),
        },
        7,
    ))
}

#[test]
fn protocol_messages_have_explicit_roles() {
    let request_id = must(RequestId::parse("request-role"));
    assert_eq!(
        ClientMessage::Ping { nonce: 1 }.required_role(),
        MessageRole::PublicClient
    );
    assert_eq!(
        ClientMessage::Start {
            request_id: request_id.clone(),
            request_generation: 1,
            backend_generation: 7,
        }
        .required_role(),
        MessageRole::Worker
    );
    assert_eq!(
        ClientMessage::RestartBackend {
            expected_generation: 7,
        }
        .required_role(),
        MessageRole::Operator
    );
    assert!(ClientMessage::GetReceipt {
        request_id,
        request_generation: 1,
        backend_generation: 7,
        minimum_sequence: 1,
    }
    .is_public_client_operation());
}

#[test]
fn canonical_protocol_round_trip_preserves_digest_only_request() {
    let tuple = digest('a');
    let message = ClientMessage::Admit(request(
        tuple,
        "request-protocol",
        "tenant-a",
        8,
        u64::MAX,
    ));
    let encoded = must(message.encode_canonical());
    assert!(encoded.len() <= MAX_FRAME_BYTES);
    assert_eq!(must(ClientMessage::decode_canonical(&encoded)), message);
    assert!(!String::from_utf8_lossy(&encoded).contains("raw prompt"));
}

#[test]
fn inflight_limits_cannot_be_bypassed_by_starting_requests() {
    let tuple = digest('a');
    let mut controller = controller(tuple.clone(), 2, 1);
    let first = request(tuple.clone(), "request-a1", "tenant-a", 4, u64::MAX);
    let first_id = first.identity.request_id.clone();
    must(controller.admit(first, 1));
    must(controller.start(&first_id, 1, 7));

    assert_eq!(
        controller.admit(
            request(tuple.clone(), "request-a2", "tenant-a", 4, u64::MAX),
            1,
        ),
        Err(InferError::TenantInflightFull)
    );

    let second = request(tuple.clone(), "request-b1", "tenant-b", 4, u64::MAX);
    let second_id = second.identity.request_id.clone();
    must(controller.admit(second, 1));
    must(controller.start(&second_id, 1, 7));
    assert_eq!(controller.inflight_requests(), 2);
    assert_eq!(controller.snapshot().running_requests, 2);

    assert_eq!(
        controller.admit(request(tuple, "request-c1", "tenant-c", 4, u64::MAX), 1),
        Err(InferError::InflightFull)
    );
}

#[test]
fn token_chain_count_and_output_limit_are_controller_enforced() {
    let tuple = digest('a');
    let mut controller = controller(tuple.clone(), 4, 2);
    let request = request(tuple, "request-token", "tenant-a", 1, u64::MAX);
    let request_id = request.identity.request_id.clone();
    must(controller.admit(request, 1));
    must(controller.start(&request_id, 1, 7));
    must(controller.publish_token(
        EventFence {
            request_id: &request_id,
            request_generation: 1,
            backend_generation: 7,
            sequence: 3,
        },
        &digest('d'),
        2,
    ));

    assert_eq!(
        controller.publish_token(
            EventFence {
                request_id: &request_id,
                request_generation: 1,
                backend_generation: 7,
                sequence: 4,
            },
            &digest('e'),
            2,
        ),
        Err(InferError::OutputTokenLimitExceeded)
    );

    let expected = must(controller.current_token_chain_digest(&request_id, 1)).clone();
    assert_eq!(
        controller.complete(
            EventFence {
                request_id: &request_id,
                request_generation: 1,
                backend_generation: 7,
                sequence: 4,
            },
            expected.clone(),
            2,
        ),
        Err(InferError::OutputTokenCountMismatch)
    );
    assert_eq!(
        controller.complete(
            EventFence {
                request_id: &request_id,
                request_generation: 1,
                backend_generation: 7,
                sequence: 4,
            },
            digest('f'),
            1,
        ),
        Err(InferError::ResultDigestMismatch)
    );

    let receipt = must(controller.complete(
        EventFence {
            request_id: &request_id,
            request_generation: 1,
            backend_generation: 7,
            sequence: 4,
        },
        expected.clone(),
        1,
    ));
    assert_eq!(receipt.output_tokens, 1);
    assert_eq!(receipt.result_digest, Some(expected));
    assert_eq!(controller.inflight_requests(), 0);
    assert_eq!(controller.snapshot().running_requests, 0);
}

#[test]
fn running_cancel_requires_worker_acknowledgement_path() {
    let tuple = digest('a');
    let mut controller = controller(tuple.clone(), 2, 2);
    let request = request(tuple, "request-running-cancel", "tenant-a", 4, u64::MAX);
    let request_id = request.identity.request_id.clone();
    must(controller.admit(request, 1));
    must(controller.start(&request_id, 1, 7));
    assert_eq!(
        controller.cancel(&request_id, 1, 1, 7),
        Err(InferError::WorkerCancellationRequired)
    );
    assert_eq!(controller.inflight_requests(), 1);
    let receipts = must(controller.restart_backend(7));
    assert_eq!(receipts.len(), 1);
    assert!(receipts[0].forced_worker_termination);
    assert_eq!(controller.inflight_requests(), 0);
}

#[test]
fn queued_cancel_releases_accounting_and_terminal_can_be_forgotten() {
    let tuple = digest('a');
    let mut controller = controller(tuple.clone(), 1, 1);
    let request = request(tuple.clone(), "request-queued-cancel", "tenant-a", 4, u64::MAX);
    let request_id = request.identity.request_id.clone();
    must(controller.admit(request, 1));
    let receipt = must(controller.cancel(&request_id, 1, 1, 7));
    assert_eq!(receipt.terminal_state, LifecycleState::Cancelled);
    assert_eq!(controller.inflight_requests(), 0);
    assert_eq!(controller.snapshot().terminal_receipts, 1);
    assert_eq!(must(controller.forget_terminal(&request_id)), receipt);
    assert_eq!(controller.snapshot().terminal_receipts, 0);

    must(controller.admit(
        request(tuple, "request-queued-cancel", "tenant-a", 4, u64::MAX),
        1,
    ));
}

#[test]
fn active_deadline_expiry_is_terminal_and_releases_capacity() {
    let tuple = digest('a');
    let mut controller = controller(tuple.clone(), 1, 1);
    let request = request(tuple.clone(), "request-expired", "tenant-a", 4, 10);
    let request_id = request.identity.request_id.clone();
    must(controller.admit(request, 1));
    let receipts = must(controller.expire_deadlines(10));
    assert_eq!(receipts.len(), 1);
    assert_eq!(receipts[0].terminal_state, LifecycleState::FailedClosed);
    assert!(!receipts[0].forced_worker_termination);
    assert_eq!(controller.inflight_requests(), 0);
    must(controller.forget_terminal(&request_id));
    must(controller.admit(
        request(tuple, "request-after-expiry", "tenant-a", 4, u64::MAX),
        11,
    ));
}

#[test]
fn token_chain_and_receipt_are_deterministic() {
    fn execute(tuple: Digest) -> TerminalReceipt {
        let mut controller = controller(tuple.clone(), 2, 2);
        let request = request(tuple, "request-replay", "tenant-a", 2, u64::MAX);
        let request_id = request.identity.request_id.clone();
        must(controller.admit(request, 1));
        must(controller.start(&request_id, 1, 7));
        must(controller.publish_token(
            EventFence {
                request_id: &request_id,
                request_generation: 1,
                backend_generation: 7,
                sequence: 3,
            },
            &digest('d'),
            3,
        ));
        let expected = must(controller.current_token_chain_digest(&request_id, 1)).clone();
        must(controller.complete(
            EventFence {
                request_id: &request_id,
                request_generation: 1,
                backend_generation: 7,
                sequence: 4,
            },
            expected,
            1,
        ))
    }

    assert_eq!(execute(digest('a')), execute(digest('a')));
}

#[test]
fn unknown_tuple_and_open_authority_fail_closed() {
    let tuple = digest('a');
    let mut controller = controller(tuple, 2, 2);
    assert_eq!(
        controller.admit(
            request(digest('f'), "request-unknown", "tenant-a", 2, u64::MAX),
            1,
        ),
        Err(InferError::UnknownModelTuple)
    );

    let mut open = request(
        digest('a'),
        "request-authority",
        "tenant-a",
        2,
        u64::MAX,
    );
    open.authority.production_listener = true;
    assert_eq!(
        controller.admit(open, 1),
        Err(InferError::AuthorityEscalation)
    );
}

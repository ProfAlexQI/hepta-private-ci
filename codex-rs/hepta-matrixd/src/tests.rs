use std::sync::Arc;
use std::sync::Mutex as StdMutex;

use codex_app_server_protocol::JSONRPCErrorError;
use codex_app_server_protocol::UserInput;
use codex_hepta_contracts::AgentId;
use codex_hepta_matrix_protocol::MatrixEventId;
use codex_hepta_matrix_protocol::MatrixRoomId;
use codex_utils_absolute_path::AbsolutePathBuf;
use pretty_assertions::assert_eq;

use super::*;

#[derive(Clone, Default)]
struct FakeTransport {
    state: Arc<StdMutex<FakeState>>,
}

#[derive(Default)]
struct FakeState {
    project_request: Option<BridgeProjectCreate>,
    project: Option<BridgeProject>,
    threads: Vec<BridgeThread>,
    queue: Vec<BridgeQueuedSubmission>,
    history: Vec<BridgeTurnClientMessage>,
    thread_start_calls: usize,
    queue_add_calls: usize,
    hide_threads_from_list: bool,
}

impl FakeTransport {
    fn snapshot(&self) -> FakeSnapshot {
        let state = self
            .state
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        FakeSnapshot {
            thread_start_calls: state.thread_start_calls,
            queue_add_calls: state.queue_add_calls,
            queue: state.queue.clone(),
            history: state.history.clone(),
        }
    }

    fn complete_queued_message(&self, client_id: &str, turn_id: &str) {
        let mut state = self
            .state
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        state
            .queue
            .retain(|queued| queued.client_user_message_id != client_id);
        state.history.push(BridgeTurnClientMessage {
            turn_id: turn_id.to_string(),
            client_user_message_id: client_id.to_string(),
        });
    }

    fn duplicate_queued_message(&self, client_id: &str) {
        let mut state = self
            .state
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let existing = state
            .queue
            .iter()
            .find(|queued| queued.client_user_message_id == client_id)
            .expect("queued message")
            .clone();
        state.queue.push(BridgeQueuedSubmission {
            id: format!("{}-duplicate", existing.id),
            client_user_message_id: existing.client_user_message_id,
        });
    }

    fn hide_threads_from_list(&self, hide: bool) {
        self.state
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .hide_threads_from_list = hide;
    }
}

struct FakeSnapshot {
    thread_start_calls: usize,
    queue_add_calls: usize,
    queue: Vec<BridgeQueuedSubmission>,
    history: Vec<BridgeTurnClientMessage>,
}

impl MatrixAppServerTransport for FakeTransport {
    fn create_project(&self, request: BridgeProjectCreate) -> BridgeFuture<'_, BridgeProject> {
        Box::pin(async move {
            let mut state = self
                .state
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            if let Some(existing_request) = state.project_request.as_ref()
                && existing_request != &request
            {
                return Err(MatrixBridgeError::Protocol(
                    "fake received project/create idempotency drift".to_string(),
                ));
            }
            state.project_request.get_or_insert_with(|| request.clone());
            let project = state
                .project
                .get_or_insert_with(|| BridgeProject {
                    id: "project-room-a".to_string(),
                    roots: request.roots,
                    metadata: request.metadata,
                })
                .clone();
            Ok(project)
        })
    }

    fn list_threads(
        &self,
        request: BridgeThreadList,
    ) -> BridgeFuture<'_, BridgePage<BridgeThread>> {
        Box::pin(async move {
            let state = self
                .state
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            let data = if state.hide_threads_from_list {
                Vec::new()
            } else {
                state
                    .threads
                    .iter()
                    .filter(|thread| {
                        thread.project_id.as_deref() == Some(request.project_id.as_str())
                            && thread.cwd == request.cwd
                    })
                    .cloned()
                    .collect()
            };
            Ok(BridgePage {
                data,
                next_cursor: None,
            })
        })
    }

    fn start_thread(&self, request: BridgeThreadStart) -> BridgeFuture<'_, BridgeThread> {
        Box::pin(async move {
            let mut state = self
                .state
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            state.thread_start_calls += 1;
            let thread = BridgeThread {
                id: format!("thread-{}", state.thread_start_calls),
                project_id: Some(request.project_id),
                cwd: request.cwd,
                ephemeral: false,
                thread_source: Some(BRIDGE_THREAD_SOURCE.to_string()),
            };
            state.threads.push(thread.clone());
            Ok(thread)
        })
    }

    fn list_queue(
        &self,
        request: BridgeQueueList,
    ) -> BridgeFuture<'_, BridgePage<BridgeQueuedSubmission>> {
        Box::pin(async move {
            let state = self
                .state
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            Ok(page(&state.queue, request.cursor, request.limit))
        })
    }

    fn list_turn_client_messages(
        &self,
        request: BridgeTurnList,
    ) -> BridgeFuture<'_, BridgePage<BridgeTurnClientMessage>> {
        Box::pin(async move {
            let state = self
                .state
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            Ok(page(&state.history, request.cursor, request.limit))
        })
    }

    fn add_queue(&self, request: BridgeQueueAdd) -> BridgeFuture<'_, BridgeQueuedSubmission> {
        Box::pin(async move {
            let mut state = self
                .state
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            state.queue_add_calls += 1;
            let queued = BridgeQueuedSubmission {
                id: format!("queue-{}", state.queue_add_calls),
                client_user_message_id: request.client_user_message_id,
            };
            state.queue.push(queued.clone());
            Ok(queued)
        })
    }
}

fn page<T: Clone>(values: &[T], cursor: Option<String>, limit: u32) -> BridgePage<T> {
    let offset = cursor
        .as_deref()
        .unwrap_or("0")
        .parse::<usize>()
        .expect("fake cursor should be numeric");
    let limit = usize::try_from(limit).expect("page limit should fit usize");
    let end = offset.saturating_add(limit).min(values.len());
    BridgePage {
        data: values[offset..end].to_vec(),
        next_cursor: (end < values.len()).then(|| end.to_string()),
    }
}

fn agent_id() -> AgentId {
    AgentId::parse("018f4f72-5f8f-7cc1-8f55-df9fb3aa2c12").expect("agent id")
}

fn room_id() -> MatrixRoomId {
    MatrixRoomId::parse("!room-a:example.test").expect("room id")
}

fn event_id() -> MatrixEventId {
    MatrixEventId::parse("$event-a").expect("event id")
}

fn config() -> MatrixBridgeConfig {
    MatrixBridgeConfig::new(
        agent_id(),
        AbsolutePathBuf::from_absolute_path("/tmp/hepta-matrix-agent-a")
            .expect("absolute workspace"),
    )
}

fn input() -> Vec<UserInput> {
    vec![UserInput::Text {
        text: "hello from Matrix".to_string(),
        text_elements: Vec::new(),
    }]
}

fn turns_list_server_error(
    method: &str,
    code: i64,
    message: String,
    data: Option<serde_json::Value>,
) -> TypedRequestError {
    TypedRequestError::Server {
        method: method.to_string(),
        source: JSONRPCErrorError {
            code,
            data,
            message,
        },
    }
}

#[test]
fn only_exact_first_page_unmaterialized_turns_error_falls_back_to_empty_history() {
    let thread_id = "thread-before-first-message";
    let expected_message = format!(
        "thread {thread_id} is not materialized yet; thread/turns/list is unavailable before first user message"
    );
    let exact = turns_list_server_error(
        "thread/turns/list",
        JSON_RPC_INVALID_REQUEST_CODE,
        expected_message.clone(),
        None,
    );
    assert!(is_unmaterialized_thread_turns_list_error(
        &exact, thread_id, None
    ));
    assert!(!is_unmaterialized_thread_turns_list_error(
        &exact,
        thread_id,
        Some("cursor-1")
    ));

    let wrong_method = turns_list_server_error(
        "thread/read",
        JSON_RPC_INVALID_REQUEST_CODE,
        expected_message.clone(),
        None,
    );
    assert!(!is_unmaterialized_thread_turns_list_error(
        &wrong_method,
        thread_id,
        None
    ));
    let wrong_code =
        turns_list_server_error("thread/turns/list", -32_603, expected_message.clone(), None);
    assert!(!is_unmaterialized_thread_turns_list_error(
        &wrong_code,
        thread_id,
        None
    ));
    let wrong_thread = turns_list_server_error(
        "thread/turns/list",
        JSON_RPC_INVALID_REQUEST_CODE,
        expected_message.replace(thread_id, "another-thread"),
        None,
    );
    assert!(!is_unmaterialized_thread_turns_list_error(
        &wrong_thread,
        thread_id,
        None
    ));
    let unexpected_data = turns_list_server_error(
        "thread/turns/list",
        JSON_RPC_INVALID_REQUEST_CODE,
        expected_message,
        Some(serde_json::json!({"unexpected": true})),
    );
    assert!(!is_unmaterialized_thread_turns_list_error(
        &unexpected_data,
        thread_id,
        None
    ));
}

#[tokio::test]
async fn first_message_recovers_thread_after_crash_without_second_thread_start() {
    let fake = FakeTransport::default();
    let first_process =
        MatrixAppServerBridge::new(config(), fake.clone()).expect("first bridge process");
    let first = first_process
        .ensure_room_thread(&room_id())
        .await
        .expect("first process starts the thread");
    assert!(!first.recovered);
    drop(first_process);

    let restarted_process =
        MatrixAppServerBridge::new(config(), fake.clone()).expect("restarted bridge process");
    let submitted = restarted_process
        .submit_matrix_event(&room_id(), &event_id(), input())
        .await
        .expect("restarted process recovers and queues first message");

    assert!(submitted.binding.recovered);
    assert_eq!(submitted.binding.thread_id, first.thread_id);
    assert!(matches!(
        submitted.state,
        MatrixSubmissionState::Queued { .. }
    ));
    let snapshot = fake.snapshot();
    assert_eq!(snapshot.thread_start_calls, 1);
    assert_eq!(snapshot.queue_add_calls, 1);
}

#[tokio::test]
async fn durable_thread_identity_survives_unmaterialized_thread_list() {
    let fake = FakeTransport::default();
    let first_process =
        MatrixAppServerBridge::new(config(), fake.clone()).expect("first bridge process");
    let first = first_process
        .ensure_room_thread(&room_id())
        .await
        .expect("first process starts the thread");
    assert!(!first.recovered);
    drop(first_process);

    // Real App Server intentionally omits a new thread from thread/list until
    // the first queued user message materializes it in state.  Recovery must
    // use the exact durable ID instead of creating a replacement.
    fake.hide_threads_from_list(true);
    let restarted =
        MatrixAppServerBridge::new(config(), fake.clone()).expect("restarted bridge process");
    let recovered = restarted
        .reconcile_room_thread(&room_id(), &first.thread_id)
        .await
        .expect("durable thread reconciles before materialization");
    assert_eq!(recovered.thread_id, first.thread_id);
    assert!(recovered.recovered);
    assert_eq!(fake.snapshot().thread_start_calls, 1);
}

#[tokio::test]
async fn durable_thread_identity_rejects_a_different_materialized_thread() {
    let fake = FakeTransport::default();
    let first_process =
        MatrixAppServerBridge::new(config(), fake.clone()).expect("first bridge process");
    let first = first_process
        .ensure_room_thread(&room_id())
        .await
        .expect("first process starts the thread");

    let error = first_process
        .reconcile_room_thread(&room_id(), "different-durable-thread")
        .await
        .expect_err("materialized thread drift must fail closed");
    assert!(matches!(error, MatrixBridgeError::Protocol(_)));
    assert_eq!(fake.snapshot().thread_start_calls, 1);
    assert_ne!(first.thread_id, "different-durable-thread");
}

#[tokio::test]
async fn duplicate_client_id_reconciles_from_queue_then_full_turn_history() {
    let fake = FakeTransport::default();
    let bridge = MatrixAppServerBridge::new(config(), fake.clone()).expect("bridge");
    let first = bridge
        .submit_matrix_event(&room_id(), &event_id(), input())
        .await
        .expect("first submission");
    let second = bridge
        .submit_matrix_event(&room_id(), &event_id(), input())
        .await
        .expect("queue reconciliation");
    assert!(matches!(
        second.state,
        MatrixSubmissionState::ReconciledQueued { .. }
    ));
    assert_eq!(fake.snapshot().queue_add_calls, 1);

    fake.complete_queued_message(&first.client_user_message_id, "turn-1");
    let third = bridge
        .submit_matrix_event(&room_id(), &event_id(), input())
        .await
        .expect("full-history reconciliation");
    assert_eq!(
        third.state,
        MatrixSubmissionState::ReconciledTurn {
            turn_id: "turn-1".to_string(),
        }
    );
    let snapshot = fake.snapshot();
    assert_eq!(snapshot.queue_add_calls, 1);
    assert!(snapshot.queue.is_empty());
    assert_eq!(snapshot.history.len(), 1);
}

#[tokio::test]
async fn duplicate_queue_rows_reconcile_without_a_third_admission() {
    let fake = FakeTransport::default();
    let bridge = MatrixAppServerBridge::new(config(), fake.clone()).expect("bridge");
    let first = bridge
        .submit_matrix_event(&room_id(), &event_id(), input())
        .await
        .expect("first submission");

    fake.duplicate_queued_message(&first.client_user_message_id);
    let replay = bridge
        .submit_matrix_event(&room_id(), &event_id(), input())
        .await
        .expect("duplicate queue rows reconcile");

    assert_eq!(
        DELIVERY_GUARANTEE,
        DeliveryGuarantee::ExactlyOncePersistedCoreAdmissionPerClientMessageId
    );
    assert!(matches!(
        replay.state,
        MatrixSubmissionState::ReconciledQueued { .. }
    ));
    assert_eq!(fake.snapshot().queue_add_calls, 1);
}

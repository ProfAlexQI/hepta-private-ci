use std::sync::Arc;
use std::sync::Mutex as StdMutex;

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
            let data = state
                .threads
                .iter()
                .filter(|thread| {
                    thread.project_id.as_deref() == Some(request.project_id.as_str())
                        && thread.cwd == request.cwd
                })
                .cloned()
                .collect();
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

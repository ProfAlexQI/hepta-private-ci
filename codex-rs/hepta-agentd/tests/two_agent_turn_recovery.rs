#![cfg(unix)]

use std::path::Path;
use std::sync::Arc;
use std::sync::Condvar;
use std::sync::Mutex;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::time::Duration;
use std::time::Instant;

use anyhow::Context;
use anyhow::Result;
use anyhow::bail;
use anyhow::ensure;
use app_test_support::MockResponsesConfig;
use codex_app_server_client::AppServerEvent;
use codex_app_server_client::RemoteAppServerClient;
use codex_app_server_protocol::ClientRequest;
use codex_app_server_protocol::RequestId;
use codex_app_server_protocol::ServerNotification;
use codex_app_server_protocol::ThreadItem;
use codex_app_server_protocol::ThreadQueueAddParams;
use codex_app_server_protocol::ThreadQueueAddResponse;
use codex_app_server_protocol::ThreadQueueListParams;
use codex_app_server_protocol::ThreadQueueListResponse;
use codex_app_server_protocol::ThreadReadParams;
use codex_app_server_protocol::ThreadReadResponse;
use codex_app_server_protocol::ThreadResumeParams;
use codex_app_server_protocol::ThreadResumeResponse;
use codex_app_server_protocol::ThreadStartParams;
use codex_app_server_protocol::ThreadStartResponse;
use codex_app_server_protocol::TurnRecoverParams;
use codex_app_server_protocol::TurnRecoverResponse;
use codex_app_server_protocol::TurnStartParams;
use codex_app_server_protocol::TurnStartResponse;
use codex_app_server_protocol::TurnStatus;
use codex_app_server_protocol::UserInput;
use codex_hepta_agentd::AgentdClient;
use codex_hepta_contracts::AgentId;
use codex_hepta_fleet::AgentLifecycle;
use core_test_support::responses;
use serde_json::Value;
use tokio::time::timeout;
use wiremock::Mock;
use wiremock::Request;
use wiremock::Respond;
use wiremock::ResponseTemplate;
use wiremock::matchers::method;
use wiremock::matchers::path_regex;

mod support;

use support::fleet::AgentFixture;
use support::fleet::FleetHarness;
use support::fleet::connect_app_server;
use support::fleet::connect_app_server_with_experimental;

const AGENT_A: &str = "018f4f72-5f8f-7cc1-8f55-df9fb3aa2c12";
const AGENT_B: &str = "019153a4-3088-7e03-a56a-9b1964f75dd3";
const ORIGINAL_CLIENT_ID: &str = "agent-a-interrupted-original";
const QUEUED_CLIENT_ID: &str = "agent-a-durable-queued";
const B_CLIENT_ID: &str = "agent-b-uninterrupted";
const B_QUEUED_CLIENT_ID: &str = "agent-b-durable-queued";
const ORIGINAL_TEXT: &str = "Agent A must recover this exact interrupted turn.";
const QUEUED_TEXT: &str = "Agent A must dispatch this durable message exactly once.";
const B_TEXT: &str = "Agent B keeps serving while Agent A is replaced.";
const B_QUEUED_TEXT: &str = "Agent B must retain this durable message while Agent A restarts.";
const EVENT_TIMEOUT: Duration = Duration::from_secs(30);
const STABLE_REQUEST_WINDOW: Duration = Duration::from_millis(500);

struct QualificationClient {
    inner: RemoteAppServerClient,
    next_request_id: i64,
}

impl QualificationClient {
    async fn connect(agent: &AgentFixture, control: &AgentdClient) -> Result<Self> {
        let ingress = control.session_ingress().await?;
        ensure!(
            ingress.socket_path == agent.layout.app_server_socket(),
            "control plane returned the wrong App Server socket"
        );
        // `turn/recover` is a stable protocol method. This qualification harness
        // does not name a product caller. Keep the client explicitly outside the
        // experimental protocol negotiation used by the agent-local queue adapter;
        // that flag controls method visibility, not authority.
        // The default support connection pins `experimental_api = false`.
        let inner = connect_app_server(
            &ingress.socket_path,
            "hepta-two-agent-turn-recovery-e2e",
            512,
        )
        .await?;
        let codex_home = inner
            .codex_home()
            .context("stable App Server initialize response omitted Codex home")?;
        let expected_home = agent.layout.home_root().to_string_lossy();
        ensure!(
            codex_home == expected_home.as_ref(),
            "App Server initialized against the wrong per-Agent home"
        );
        Ok(Self {
            inner,
            next_request_id: 1,
        })
    }

    fn request_id(&mut self) -> RequestId {
        let request_id = self.next_request_id;
        self.next_request_id += 1;
        RequestId::Integer(request_id)
    }

    async fn start_thread(&mut self, workspace: &Path) -> Result<ThreadStartResponse> {
        let request_id = self.request_id();
        let response = self
            .inner
            .request_typed(ClientRequest::ThreadStart {
                request_id,
                params: ThreadStartParams {
                    cwd: Some(workspace.to_string_lossy().into_owned()),
                    ephemeral: Some(false),
                    ..ThreadStartParams::default()
                },
            })
            .await?;
        Ok(response)
    }

    async fn start_turn(
        &mut self,
        thread_id: &str,
        client_id: &str,
        text: &str,
    ) -> Result<TurnStartResponse> {
        let request_id = self.request_id();
        Ok(self
            .inner
            .request_typed(ClientRequest::TurnStart {
                request_id,
                params: TurnStartParams {
                    thread_id: thread_id.to_string(),
                    client_user_message_id: Some(client_id.to_string()),
                    input: vec![text_input(text)],
                    ..TurnStartParams::default()
                },
            })
            .await?)
    }

    async fn resume(&mut self, thread_id: &str) -> Result<ThreadResumeResponse> {
        let request_id = self.request_id();
        Ok(self
            .inner
            .request_typed(ClientRequest::ThreadResume {
                request_id,
                params: ThreadResumeParams {
                    thread_id: thread_id.to_string(),
                    ..ThreadResumeParams::default()
                },
            })
            .await?)
    }

    async fn recover(&mut self, thread_id: &str, turn_id: &str) -> Result<TurnRecoverResponse> {
        let request_id = self.request_id();
        Ok(self
            .inner
            .request_typed(ClientRequest::TurnRecover {
                request_id,
                params: TurnRecoverParams {
                    thread_id: thread_id.to_string(),
                    turn_id: turn_id.to_string(),
                },
            })
            .await?)
    }

    async fn read(&mut self, thread_id: &str) -> Result<ThreadReadResponse> {
        let request_id = self.request_id();
        Ok(self
            .inner
            .request_typed(ClientRequest::ThreadRead {
                request_id,
                params: ThreadReadParams {
                    thread_id: thread_id.to_string(),
                    include_turns: true,
                },
            })
            .await?)
    }

    async fn wait_turn_then_queue(
        &mut self,
        thread_id: &str,
        preceding_turn_id: &str,
        queued_client_id: &str,
    ) -> Result<String> {
        timeout(EVENT_TIMEOUT, async {
            let mut event_index = 0_usize;
            let mut preceding_completed_at = None;
            let mut queued_started_at = None;
            let mut queued_turn_id = None;
            let mut queued_completed = false;
            loop {
                let event = self
                    .inner
                    .next_event()
                    .await
                    .context("App Server event stream closed while waiting for turn/queue")?;
                event_index += 1;
                match event {
                    AppServerEvent::ServerNotification(notification) => match notification.as_ref()
                    {
                        ServerNotification::ItemStarted(started)
                            if started.thread_id == thread_id
                                && matches!(
                                    &started.item,
                                    ThreadItem::UserMessage { client_id, .. }
                                        if client_id.as_deref() == Some(queued_client_id)
                                ) =>
                        {
                            ensure!(
                                queued_turn_id
                                    .as_ref()
                                    .is_none_or(|turn_id| turn_id == &started.turn_id),
                                "queued client id started under more than one turn"
                            );
                            queued_started_at.get_or_insert(event_index);
                            queued_turn_id = Some(started.turn_id.clone());
                        }
                        ServerNotification::TurnCompleted(completed)
                            if completed.thread_id == thread_id =>
                        {
                            ensure!(
                                completed.turn.status == TurnStatus::Completed,
                                "preceding/queued turn ended with {:#?}",
                                completed.turn
                            );
                            if completed.turn.id == preceding_turn_id {
                                preceding_completed_at.get_or_insert(event_index);
                            }
                            if queued_turn_id.as_deref() == Some(completed.turn.id.as_str()) {
                                queued_completed = true;
                            }
                        }
                        _ => {}
                    },
                    AppServerEvent::Disconnected { message } => {
                        bail!("App Server disconnected during recovery: {message}");
                    }
                    AppServerEvent::Lagged { skipped } => {
                        bail!("App Server event client lagged by {skipped} events");
                    }
                    AppServerEvent::ServerRequest(_) => {}
                }
                if preceding_completed_at.is_some() && queued_completed {
                    let preceding_completed_at = preceding_completed_at
                        .context("preceding completion position disappeared")?;
                    let queued_started_at =
                        queued_started_at.context("queued user item never started")?;
                    ensure!(
                        preceding_completed_at < queued_started_at,
                        "queued dispatch began before the preceding turn reached terminal state"
                    );
                    return queued_turn_id.context("queued turn id disappeared");
                }
            }
        })
        .await
        .context("timed out waiting for preceding and queued turns")?
    }
}

/// Agent-local durable queue boundary. Queue APIs remain explicitly experimental
/// upstream, so this adapter negotiates their visibility without granting Matrix or
/// Robrix any tool, approval, or control authority.
struct QueueAdapter {
    inner: RemoteAppServerClient,
    next_request_id: i64,
}

impl QueueAdapter {
    async fn connect(agent: &AgentFixture, control: &AgentdClient) -> Result<Self> {
        let ingress = control.session_ingress().await?;
        ensure!(ingress.socket_path == agent.layout.app_server_socket());
        let inner = connect_app_server_with_experimental(
            &ingress.socket_path,
            "hepta-agent-local-durable-queue-e2e",
            64,
            /*experimental_api*/ true,
        )
        .await?;
        let codex_home = inner
            .codex_home()
            .context("queue App Server initialize response omitted Codex home")?;
        let expected_home = agent.layout.home_root().to_string_lossy();
        ensure!(
            codex_home == expected_home.as_ref(),
            "queue adapter initialized against the wrong per-Agent home"
        );
        Ok(Self {
            inner,
            next_request_id: 1,
        })
    }

    fn request_id(&mut self) -> RequestId {
        let request_id = self.next_request_id;
        self.next_request_id += 1;
        RequestId::Integer(request_id)
    }

    async fn add(
        &mut self,
        thread_id: &str,
        client_user_message_id: &str,
        text: &str,
    ) -> Result<ThreadQueueAddResponse> {
        let request_id = self.request_id();
        Ok(self
            .inner
            .request_typed(ClientRequest::ThreadQueueAdd {
                request_id,
                params: ThreadQueueAddParams {
                    thread_id: thread_id.to_string(),
                    input: vec![text_input(text)],
                    client_user_message_id: client_user_message_id.to_string(),
                },
            })
            .await?)
    }

    async fn list(&mut self, thread_id: &str) -> Result<ThreadQueueListResponse> {
        let request_id = self.request_id();
        Ok(self
            .inner
            .request_typed(ClientRequest::ThreadQueueList {
                request_id,
                params: ThreadQueueListParams {
                    thread_id: thread_id.to_string(),
                    cursor: None,
                    limit: None,
                },
            })
            .await?)
    }
}

#[derive(Default)]
struct SequenceState {
    requests: Mutex<Vec<Request>>,
    calls: AtomicUsize,
}

struct DelayedFirstSequence {
    state: Arc<SequenceState>,
    bodies: Vec<String>,
}

impl Respond for DelayedFirstSequence {
    fn respond(&self, request: &Request) -> ResponseTemplate {
        self.state
            .requests
            .lock()
            .expect("request capture lock poisoned")
            .push(request.clone());
        let index = self.state.calls.fetch_add(1, Ordering::AcqRel);
        let body = self
            .bodies
            .get(index)
            .unwrap_or_else(|| panic!("unexpected model request {index}"));
        let response = ResponseTemplate::new(200)
            .insert_header("content-type", "text/event-stream")
            .set_body_string(body.clone());
        if index == 0 {
            // The request has crossed the physical-send boundary, but the old
            // process cannot receive a terminal model response before SIGKILL.
            response.set_delay(Duration::from_secs(120))
        } else {
            response
        }
    }
}

#[derive(Default)]
struct ResponseGate {
    released: Mutex<bool>,
    changed: Condvar,
}

impl ResponseGate {
    fn release(&self) {
        let mut released = self.released.lock().expect("response gate lock poisoned");
        *released = true;
        self.changed.notify_all();
    }

    fn wait(&self) {
        let released = self.released.lock().expect("response gate lock poisoned");
        let (released, wait) = self
            .changed
            .wait_timeout_while(released, EVENT_TIMEOUT, |released| !*released)
            .expect("response gate lock poisoned while waiting");
        assert!(
            *released && !wait.timed_out(),
            "timed out waiting to release the gated model response"
        );
    }
}

struct GatedFirstSequence {
    state: Arc<SequenceState>,
    gate: Arc<ResponseGate>,
    bodies: Vec<String>,
}

impl Respond for GatedFirstSequence {
    fn respond(&self, request: &Request) -> ResponseTemplate {
        self.state
            .requests
            .lock()
            .expect("request capture lock poisoned")
            .push(request.clone());
        let index = self.state.calls.fetch_add(1, Ordering::AcqRel);
        let body = self
            .bodies
            .get(index)
            .unwrap_or_else(|| panic!("unexpected gated model request {index}"));
        if index == 0 {
            self.gate.wait();
        }
        ResponseTemplate::new(200)
            .insert_header("content-type", "text/event-stream")
            .set_body_string(body.clone())
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn killed_agent_recovers_same_turn_then_dispatches_queue_once_while_peer_stays_live()
-> Result<()> {
    let mut fleet = FleetHarness::new()?;
    let agent_a = fleet.register(AGENT_A, "workspace-a")?;
    let agent_b = fleet.register(AGENT_B, "workspace-b")?;

    let model_a = responses::start_mock_server().await;
    let model_b = responses::start_mock_server().await;
    MockResponsesConfig::new(&model_a.uri()).write(agent_a.layout.home_root())?;
    MockResponsesConfig::new(&model_b.uri()).write(agent_b.layout.home_root())?;

    let model_a_state = Arc::new(SequenceState::default());
    Mock::given(method("POST"))
        .and(path_regex(".*/responses$"))
        .respond_with(DelayedFirstSequence {
            state: Arc::clone(&model_a_state),
            bodies: vec![
                final_sse("agent-a-killed-response"),
                final_sse("agent-a-recovered-response"),
                final_sse("agent-a-queued-response"),
            ],
        })
        .mount(&model_a)
        .await;
    let model_b_state = Arc::new(SequenceState::default());
    let model_b_gate = Arc::new(ResponseGate::default());
    Mock::given(method("POST"))
        .and(path_regex(".*/responses$"))
        .respond_with(GatedFirstSequence {
            state: Arc::clone(&model_b_state),
            gate: Arc::clone(&model_b_gate),
            bodies: vec![
                final_sse("agent-b-original-response"),
                final_sse("agent-b-queued-response"),
            ],
        })
        .mount(&model_b)
        .await;

    fleet.start(&agent_a)?;
    fleet.start(&agent_b)?;
    let (control_a, initial_health_a) = fleet.wait_ready(&agent_a, 1).await?;
    let (control_b, initial_health_b) = fleet.wait_ready(&agent_b, 1).await?;
    let initial_generation_b = generation(&fleet, &agent_b.agent_id)?;

    let mut client_a = QualificationClient::connect(&agent_a, &control_a).await?;
    let mut client_b = QualificationClient::connect(&agent_b, &control_b).await?;
    let mut queue_a = QueueAdapter::connect(&agent_a, &control_a).await?;
    let mut queue_b = QueueAdapter::connect(&agent_b, &control_b).await?;
    let thread_a = client_a.start_thread(&agent_a.workspace).await?.thread;
    let thread_b = client_b.start_thread(&agent_b.workspace).await?.thread;
    ensure!(!thread_a.ephemeral && thread_a.path.is_some());
    ensure!(!thread_b.ephemeral && thread_b.path.is_some());
    ensure!(queue_b.list(&thread_b.id).await?.data.is_empty());

    let original = client_a
        .start_turn(&thread_a.id, ORIGINAL_CLIENT_ID, ORIGINAL_TEXT)
        .await?;
    let original_turn_id = original.turn.id.clone();
    ensure!(original.turn.status == TurnStatus::InProgress);
    wait_for_request_count(&model_a_state, 1).await?;

    let queued = queue_a
        .add(&thread_a.id, QUEUED_CLIENT_ID, QUEUED_TEXT)
        .await?
        .queued_submission;
    ensure!(queued.client_user_message_id == QUEUED_CLIENT_ID);
    ensure!(queued.input == vec![text_input(QUEUED_TEXT)]);
    ensure!(
        queue_a.list(&thread_a.id).await?.data == vec![queued.clone()],
        "durable queue changed before Agent A was killed"
    );

    let cross_agent_resume = client_b
        .resume(&thread_a.id)
        .await
        .expect_err("Agent B must not resume Agent A's private thread");
    assert_cross_agent_thread_not_found(&cross_agent_resume, "resume")?;
    let cross_agent_recover = client_b
        .recover(&thread_a.id, &original_turn_id)
        .await
        .expect_err("Agent B must not recover Agent A's private turn");
    assert_cross_agent_thread_not_found(&cross_agent_recover, "recovery")?;
    ensure!(model_a_state.calls.load(Ordering::Acquire) == 1);
    ensure!(model_b_state.calls.load(Ordering::Acquire) == 0);

    let b_turn = client_b
        .start_turn(&thread_b.id, B_CLIENT_ID, B_TEXT)
        .await?;
    let b_turn_id = b_turn.turn.id.clone();
    ensure!(b_turn.turn.status == TurnStatus::InProgress);
    wait_for_request_count(&model_b_state, 1).await?;
    let b_queued = queue_b
        .add(&thread_b.id, B_QUEUED_CLIENT_ID, B_QUEUED_TEXT)
        .await?
        .queued_submission;
    ensure!(b_queued.client_user_message_id == B_QUEUED_CLIENT_ID);
    ensure!(b_queued.input == vec![text_input(B_QUEUED_TEXT)]);
    ensure!(
        queue_b.list(&thread_b.id).await?.data == vec![b_queued.clone()],
        "Agent B durable queue changed before Agent A was killed"
    );
    let b_before_failure = client_b.read(&thread_b.id).await?;
    assert_turn_once_with_status(
        &b_before_failure.thread.turns,
        &b_turn_id,
        TurnStatus::InProgress,
    )?;
    assert_user_item_once(&b_before_failure.thread.turns, B_CLIENT_ID, B_TEXT)?;

    fleet.supervisor.kill(&agent_a.agent_id)?;
    wait_inactive(&mut fleet, &agent_a.agent_id).await?;
    drop(client_a);
    drop(queue_a);

    let b_during_failure = control_b.health().await?;
    ensure!(b_during_failure.ready);
    ensure!(b_during_failure.process_id == initial_health_b.process_id);
    ensure!(generation(&fleet, &agent_b.agent_id)? == initial_generation_b);
    ensure!(queue_b.list(&thread_b.id).await?.data == vec![b_queued.clone()]);
    let b_during_failure = client_b.read(&thread_b.id).await?;
    assert_turn_once_with_status(
        &b_during_failure.thread.turns,
        &b_turn_id,
        TurnStatus::InProgress,
    )?;
    assert_user_item_once(&b_during_failure.thread.turns, B_CLIENT_ID, B_TEXT)?;
    ensure!(model_b_state.calls.load(Ordering::Acquire) == 1);

    fleet
        .supervisor
        .restart(&agent_a.agent_id, Instant::now())?;
    let restarted_generation_a = generation(&fleet, &agent_a.agent_id)?;
    ensure!(restarted_generation_a > 1);
    let restarted_control_a = fleet.control_client(&agent_a, restarted_generation_a)?;
    let restarted_health_a = fleet
        .wait_until_ready(&agent_a.agent_id, &restarted_control_a)
        .await?;
    ensure!(restarted_health_a.process_id != initial_health_a.process_id);

    let b_after_restart = control_b.health().await?;
    ensure!(b_after_restart.ready);
    ensure!(b_after_restart.process_id == initial_health_b.process_id);
    ensure!(generation(&fleet, &agent_b.agent_id)? == initial_generation_b);
    ensure!(queue_b.list(&thread_b.id).await?.data == vec![b_queued.clone()]);
    let b_after_restart = client_b.read(&thread_b.id).await?;
    assert_turn_once_with_status(
        &b_after_restart.thread.turns,
        &b_turn_id,
        TurnStatus::InProgress,
    )?;
    assert_user_item_once(&b_after_restart.thread.turns, B_CLIENT_ID, B_TEXT)?;
    ensure!(model_b_state.calls.load(Ordering::Acquire) == 1);

    let mut restarted_a = QualificationClient::connect(&agent_a, &restarted_control_a).await?;
    let mut restarted_queue_a = QueueAdapter::connect(&agent_a, &restarted_control_a).await?;
    let resumed = restarted_a.resume(&thread_a.id).await?;
    ensure!(resumed.thread.id == thread_a.id);
    let interrupted_tail = resumed
        .thread
        .turns
        .last()
        .context("cold resume omitted the stale in-progress tail")?;
    ensure!(interrupted_tail.id == original_turn_id);
    ensure!(interrupted_tail.status == TurnStatus::Interrupted);
    assert_user_item_once(&resumed.thread.turns, ORIGINAL_CLIENT_ID, ORIGINAL_TEXT)?;
    let queue_after_resume = restarted_queue_a.list(&thread_a.id).await?;
    ensure!(
        queue_after_resume.data == vec![queued],
        "cold normalization dispatched or lost the durable queue before turn/recover; \
         queue={:?}; model_calls={}; turns={:?}",
        queue_after_resume.data,
        model_a_state.calls.load(Ordering::Acquire),
        resumed
            .thread
            .turns
            .iter()
            .map(|turn| (&turn.id, &turn.status))
            .collect::<Vec<_>>()
    );

    let recovered = restarted_a.recover(&thread_a.id, &original_turn_id).await?;
    ensure!(recovered.turn.id == original_turn_id);
    ensure!(recovered.turn.status == TurnStatus::InProgress);
    let queued_turn_id = restarted_a
        .wait_turn_then_queue(&thread_a.id, &original_turn_id, QUEUED_CLIENT_ID)
        .await?;
    ensure!(queued_turn_id != original_turn_id);
    ensure!(restarted_queue_a.list(&thread_a.id).await?.data.is_empty());

    let persisted = restarted_a.read(&thread_a.id).await?;
    ensure!(
        persisted
            .thread
            .turns
            .iter()
            .filter(|turn| turn.id == original_turn_id)
            .count()
            == 1,
        "recovery duplicated the original turn identity in persisted history"
    );
    assert_user_item_once(&persisted.thread.turns, ORIGINAL_CLIENT_ID, ORIGINAL_TEXT)?;
    assert_user_item_once(&persisted.thread.turns, QUEUED_CLIENT_ID, QUEUED_TEXT)?;
    assert_turn_once_with_status(
        &persisted.thread.turns,
        &original_turn_id,
        TurnStatus::Completed,
    )?;
    assert_turn_once_with_status(
        &persisted.thread.turns,
        &queued_turn_id,
        TurnStatus::Completed,
    )?;

    wait_for_request_count(&model_a_state, 3).await?;
    let b_before_release = control_b.health().await?;
    ensure!(b_before_release.ready);
    ensure!(b_before_release.process_id == initial_health_b.process_id);
    ensure!(generation(&fleet, &agent_b.agent_id)? == initial_generation_b);
    ensure!(queue_b.list(&thread_b.id).await?.data == vec![b_queued.clone()]);
    let b_before_release = client_b.read(&thread_b.id).await?;
    assert_turn_once_with_status(
        &b_before_release.thread.turns,
        &b_turn_id,
        TurnStatus::InProgress,
    )?;
    assert_user_item_once(&b_before_release.thread.turns, B_CLIENT_ID, B_TEXT)?;
    ensure!(model_b_state.calls.load(Ordering::Acquire) == 1);

    model_b_gate.release();
    let b_queued_turn_id = client_b
        .wait_turn_then_queue(&thread_b.id, &b_turn_id, B_QUEUED_CLIENT_ID)
        .await?;
    ensure!(b_queued_turn_id != b_turn_id);
    wait_for_request_count(&model_b_state, 2).await?;

    let final_health_b = control_b.health().await?;
    ensure!(final_health_b.ready);
    ensure!(final_health_b.process_id == initial_health_b.process_id);
    ensure!(generation(&fleet, &agent_b.agent_id)? == initial_generation_b);
    ensure!(queue_b.list(&thread_b.id).await?.data.is_empty());
    let persisted_b = client_b.read(&thread_b.id).await?;
    assert_turn_once_with_status(&persisted_b.thread.turns, &b_turn_id, TurnStatus::Completed)?;
    assert_turn_once_with_status(
        &persisted_b.thread.turns,
        &b_queued_turn_id,
        TurnStatus::Completed,
    )?;
    assert_user_item_once(&persisted_b.thread.turns, B_CLIENT_ID, B_TEXT)?;
    assert_user_item_once(&persisted_b.thread.turns, B_QUEUED_CLIENT_ID, B_QUEUED_TEXT)?;

    assert_request_count_stable(&model_a_state, 3, "Agent A").await?;
    assert_request_count_stable(&model_b_state, 2, "Agent B").await?;
    let requests_a = model_a_state
        .requests
        .lock()
        .expect("request capture lock poisoned")
        .clone();
    ensure!(requests_a.len() == 3);
    let bodies_a = requests_a
        .iter()
        .map(Request::body_json::<Value>)
        .collect::<Result<Vec<_>, _>>()?;
    ensure!(
        bodies_a
            .iter()
            .all(|body| request_user_text_count(body, ORIGINAL_TEXT) <= 1),
        "a physical model request contained duplicate original user items"
    );
    ensure!(
        bodies_a
            .iter()
            .filter(|body| request_user_text_count(body, QUEUED_TEXT) == 1)
            .count()
            == 1,
        "queued user message reached the model other than exactly once"
    );
    ensure!(
        bodies_a
            .iter()
            .all(|body| request_user_text_count(body, QUEUED_TEXT) <= 1),
        "queued user message was duplicated within a physical request"
    );
    let requests_b = model_b_state
        .requests
        .lock()
        .expect("request capture lock poisoned")
        .clone();
    ensure!(requests_b.len() == 2);
    let bodies_b = requests_b
        .iter()
        .map(Request::body_json::<Value>)
        .collect::<Result<Vec<_>, _>>()?;
    ensure!(request_user_text_count(&bodies_b[0], B_TEXT) == 1);
    ensure!(
        bodies_b
            .iter()
            .all(|body| request_user_text_count(body, B_TEXT) <= 1),
        "Agent B original user message was duplicated within a physical request"
    );
    ensure!(
        bodies_b
            .iter()
            .filter(|body| request_user_text_count(body, B_QUEUED_TEXT) == 1)
            .count()
            == 1,
        "Agent B queued user message reached the model other than exactly once"
    );
    ensure!(
        bodies_b
            .iter()
            .all(|body| request_user_text_count(body, B_QUEUED_TEXT) <= 1),
        "Agent B queued user message was duplicated within a physical request"
    );
    ensure!(model_a_state.calls.load(Ordering::Acquire) == 3);
    ensure!(model_b_state.calls.load(Ordering::Acquire) == 2);

    restarted_a.inner.shutdown().await?;
    restarted_queue_a.inner.shutdown().await?;
    client_b.inner.shutdown().await?;
    queue_b.inner.shutdown().await?;
    Ok(())
}

fn text_input(text: &str) -> UserInput {
    UserInput::Text {
        text: text.to_string(),
        text_elements: Vec::new(),
    }
}

fn final_sse(response_id: &str) -> String {
    responses::sse(vec![
        responses::ev_response_created(response_id),
        responses::ev_assistant_message(&format!("message-{response_id}"), "done"),
        responses::ev_completed(response_id),
    ])
}

async fn wait_for_request_count(state: &SequenceState, expected: usize) -> Result<()> {
    timeout(EVENT_TIMEOUT, async {
        while state.calls.load(Ordering::Acquire) < expected {
            tokio::time::sleep(Duration::from_millis(20)).await;
        }
    })
    .await
    .with_context(|| format!("model did not receive {expected} physical requests"))?;
    Ok(())
}

async fn assert_request_count_stable(
    state: &SequenceState,
    expected: usize,
    agent: &str,
) -> Result<()> {
    let deadline = Instant::now() + STABLE_REQUEST_WINDOW;
    loop {
        let observed = state.calls.load(Ordering::Acquire);
        ensure!(
            observed == expected,
            "{agent} physical request count changed during the stable window: expected {expected}, found {observed}"
        );
        if Instant::now() >= deadline {
            return Ok(());
        }
        tokio::time::sleep(Duration::from_millis(20)).await;
    }
}

async fn wait_inactive(fleet: &mut FleetHarness, agent_id: &AgentId) -> Result<()> {
    let deadline = Instant::now() + Duration::from_secs(10);
    loop {
        let report = fleet.supervisor.tick(Instant::now());
        ensure!(
            report.faults.is_empty(),
            "supervisor faults while killing {agent_id}: {:?}",
            report.faults
        );
        let inactive = fleet
            .supervisor
            .snapshot(agent_id)
            .is_some_and(|snapshot| !snapshot.active);
        let stopped = fleet
            .registry
            .load()?
            .agent(agent_id)
            .is_some_and(|record| record.lifecycle.lifecycle == AgentLifecycle::Stopped);
        if inactive && stopped {
            return Ok(());
        }
        if Instant::now() >= deadline {
            bail!("timed out waiting for Agent {agent_id} SIGKILL");
        }
        tokio::time::sleep(Duration::from_millis(20)).await;
    }
}

fn generation(fleet: &FleetHarness, agent_id: &AgentId) -> Result<u64> {
    Ok(fleet
        .registry
        .load()?
        .agent(agent_id)
        .with_context(|| format!("Agent {agent_id} missing from registry"))?
        .lifecycle
        .generation)
}

fn assert_user_item_once(
    turns: &[codex_app_server_protocol::Turn],
    expected_client_id: &str,
    expected_text: &str,
) -> Result<()> {
    let matching = turns
        .iter()
        .flat_map(|turn| &turn.items)
        .filter(|item| {
            matches!(
                item,
                ThreadItem::UserMessage { client_id, content, .. }
                    if client_id.as_deref() == Some(expected_client_id)
                        && content == &vec![text_input(expected_text)]
            )
        })
        .count();
    ensure!(
        matching == 1,
        "expected one persisted user item for {expected_client_id}, found {matching}"
    );
    Ok(())
}

fn assert_turn_once_with_status(
    turns: &[codex_app_server_protocol::Turn],
    expected_turn_id: &str,
    expected_status: TurnStatus,
) -> Result<()> {
    let matching = turns
        .iter()
        .filter(|turn| turn.id == expected_turn_id)
        .collect::<Vec<_>>();
    ensure!(
        matching.len() == 1,
        "expected one logical turn {expected_turn_id}, found {}",
        matching.len()
    );
    ensure!(
        matching[0].status == expected_status,
        "turn {expected_turn_id} had status {:?}, expected {expected_status:?}",
        matching[0].status
    );
    Ok(())
}

fn assert_cross_agent_thread_not_found(error: &anyhow::Error, operation: &str) -> Result<()> {
    let message = error.to_string();
    ensure!(
        message.contains("thread not found") || message.contains("no rollout found for thread id"),
        "cross-Agent {operation} failed for an unexpected reason: {error:#}"
    );
    Ok(())
}

fn request_user_text_count(body: &Value, expected_text: &str) -> usize {
    body.get("input")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter(|item| {
            item.get("type").and_then(Value::as_str) == Some("message")
                && item.get("role").and_then(Value::as_str) == Some("user")
        })
        .filter_map(|item| item.get("content").and_then(Value::as_array))
        .flatten()
        .filter(|span| {
            span.get("type").and_then(Value::as_str) == Some("input_text")
                && span.get("text").and_then(Value::as_str) == Some(expected_text)
        })
        .count()
}

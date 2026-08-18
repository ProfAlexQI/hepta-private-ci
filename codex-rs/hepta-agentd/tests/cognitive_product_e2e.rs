#![cfg(unix)]

use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::time::Duration;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use anyhow::Context;
use anyhow::Result;
use anyhow::anyhow;
use anyhow::ensure;
use app_test_support::MockResponsesConfig;
use codex_app_server_client::AppServerEvent;
use codex_app_server_client::RemoteAppServerClient;
use codex_app_server_protocol::ClientRequest;
use codex_app_server_protocol::RequestId;
use codex_app_server_protocol::ServerNotification;
use codex_app_server_protocol::ThreadStartParams;
use codex_app_server_protocol::ThreadStartResponse;
use codex_app_server_protocol::TurnStartParams;
use codex_app_server_protocol::TurnStartResponse;
use codex_app_server_protocol::TurnStatus;
use codex_app_server_protocol::UserInput as V2UserInput;
use codex_hepta_agentd::AgentdClient;
use codex_hepta_agentd::MemoryFederationCapabilityState;
use codex_hepta_agentd::MemoryFederationScopeKind;
use codex_hepta_automation::AutomationSchedule;
use codex_hepta_automation::AutomationTaskDraft;
use codex_hepta_memory::CognitiveAccess;
use codex_hepta_memory::CognitiveScope;
use codex_hepta_memory::CognitiveStore;
use codex_hepta_memory::LedgerSourceKind;
use codex_hepta_memory::MemoryDraft;
use codex_hepta_memory::MemoryLifecycleState;
use codex_hepta_memory::MemoryRevisionDraft;
use codex_hepta_memory::MemoryVerification;
use codex_hepta_memory::SourceDraft;
use codex_state::SqliteConfig;
use codex_utils_absolute_path::AbsolutePathBuf;
use core_test_support::responses;
use core_test_support::responses::ResponseMock;
use core_test_support::responses::ResponsesRequest;
use serde_json::Value;
use serde_json::json;
use tokio::time::timeout;
use wiremock::Mock;
use wiremock::Request;
use wiremock::Respond;
use wiremock::ResponseTemplate;
use wiremock::matchers::method;
use wiremock::matchers::path;

mod support;

use support::fleet::AgentFixture;
use support::fleet::FleetHarness;
use support::fleet::connect_app_server;

const AGENT_A: &str = "018f4f72-5f8f-7cc1-8f55-df9fb3aa2c12";
const AGENT_B: &str = "019153a4-3088-7e03-a56a-9b1964f75dd3";
const AGENT_C: &str = "019153a4-3088-7e03-a56a-9b1964f75dd4";
const AGENT_D: &str = "019153a4-3088-7e03-a56a-9b1964f75dd5";
const AGENT_E: &str = "019153a4-3088-7e03-a56a-9b1964f75dd6";
const COGNITIVE_NAMESPACE: &str = "hepta_cognitive";
const COGNITIVE_REFERENCE_OPEN: &str = "<hepta_memory_reference schema=\"1\">";
const COGNITIVE_REFERENCE_CLOSE: &str = "</hepta_memory_reference>";
const TURN_TIMEOUT: Duration = Duration::from_secs(20);

struct ProductClient {
    inner: RemoteAppServerClient,
    next_request_id: i64,
}

impl ProductClient {
    async fn connect(agent: &AgentFixture, control: &AgentdClient) -> Result<Self> {
        let ingress = control.session_ingress().await?;
        ensure!(
            ingress.socket_path == agent.layout.app_server_socket(),
            "control plane returned the wrong App Server socket"
        );
        let inner =
            connect_app_server(&ingress.socket_path, "hepta-cognitive-product-e2e", 256).await?;
        let codex_home = inner
            .codex_home()
            .context("App Server initialize response omitted product home")?;
        let expected_home = agent.layout.home_root().to_string_lossy();
        ensure!(
            codex_home == expected_home.as_ref(),
            "App Server initialized against the wrong product home"
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

    async fn start_thread(&mut self, workspace: &Path) -> Result<String> {
        let request_id = self.request_id();
        let response: ThreadStartResponse = self
            .inner
            .request_typed(ClientRequest::ThreadStart {
                request_id,
                params: ThreadStartParams {
                    cwd: Some(workspace.to_string_lossy().into_owned()),
                    ephemeral: Some(true),
                    ..ThreadStartParams::default()
                },
            })
            .await?;
        ensure!(
            response.cwd.as_path() == workspace,
            "thread started in the wrong workspace"
        );
        Ok(response.thread.id)
    }

    async fn run_turn(&mut self, thread_id: &str, text: &str) -> Result<String> {
        let request_id = self.request_id();
        let response: TurnStartResponse = self
            .inner
            .request_typed(ClientRequest::TurnStart {
                request_id,
                params: TurnStartParams {
                    thread_id: thread_id.to_string(),
                    client_user_message_id: None,
                    input: vec![V2UserInput::Text {
                        text: text.to_string(),
                        text_elements: Vec::new(),
                    }],
                    ..TurnStartParams::default()
                },
            })
            .await?;
        let turn_id = response.turn.id;
        self.wait_for_turn(thread_id, &turn_id).await?;
        Ok(turn_id)
    }

    async fn wait_for_turn(&mut self, thread_id: &str, turn_id: &str) -> Result<()> {
        timeout(TURN_TIMEOUT, async {
            loop {
                let event = self
                    .inner
                    .next_event()
                    .await
                    .context("App Server event stream closed before turn/completed")?;
                if let AppServerEvent::ServerNotification(notification) = event
                    && let ServerNotification::TurnCompleted(completed) = notification.as_ref()
                    && completed.thread_id == thread_id
                    && completed.turn.id == turn_id
                {
                    ensure!(
                        completed.turn.status == TurnStatus::Completed,
                        "turn ended with {:#?}",
                        completed.turn
                    );
                    return Ok::<(), anyhow::Error>(());
                }
            }
        })
        .await
        .context("timed out waiting for turn/completed")??;
        Ok(())
    }

    async fn shutdown(self) -> Result<()> {
        self.inner.shutdown().await?;
        Ok(())
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn real_agentd_remember_recall_correct_and_forget_revalidate_physical_sends() -> Result<()> {
    const ORIGINAL: &str = "Project Aurora deadline is Friday.";
    const CORRECTED: &str = "Project Aurora deadline is Monday.";
    const FORGET_REASON: &str = "Project Aurora deadline memory should be forgotten.";
    const REMEMBER_CALL: &str = "remember-aurora";
    const CORRECT_CALL: &str = "correct-aurora";
    const FORGET_CALL: &str = "forget-aurora";

    let mut fleet = FleetHarness::new()?;
    let agent = fleet.register(AGENT_A, "workspace-a")?;
    let model = responses::start_mock_server().await;
    MockResponsesConfig::new(&model.uri()).write(agent.layout.home_root())?;
    fleet.start(&agent)?;
    let (control, _) = fleet.wait_ready(&agent, 1).await?;
    let mut product = ProductClient::connect(&agent, &control).await?;

    let thread_a = product.start_thread(&agent.workspace).await?;
    let remember = responses::mount_sse_sequence(
        &model,
        vec![
            tool_sse(
                "remember-response",
                REMEMBER_CALL,
                "remember",
                json!({
                    "stable_key": "project-aurora-deadline",
                    "content": ORIGINAL,
                    "scope": "workspace_private"
                }),
            ),
            final_sse("remember-final"),
        ],
    )
    .await;
    product.run_turn(&thread_a, ORIGINAL).await?;
    let remember_requests = remember.requests();
    ensure!(
        remember_requests.len() == 2,
        "remember must perform two physical sends"
    );
    ensure!(
        remember_requests[0]
            .tool_by_name(COGNITIVE_NAMESPACE, "remember")
            .is_some(),
        "the real ToolRegistry did not advertise cognitive remember"
    );
    assert_no_cognitive_reference(&remember_requests[0])?;
    let remembered = tool_output(&remember, REMEMBER_CALL)?;
    ensure!(remembered["operation"] == "remembered");
    ensure!(remembered["revision"] == 1);
    ensure!(remembered["verification"] == "verified");
    let memory_id = remembered["memory_id"]
        .as_str()
        .context("remember output omitted memory_id")?
        .to_string();

    let thread_b = product.start_thread(&agent.workspace).await?;
    let recall = responses::mount_sse_sequence(&model, vec![final_sse("recall-final")]).await;
    product
        .run_turn(&thread_b, "What is the Project Aurora deadline?")
        .await?;
    let recall_request = recall.single_request();
    assert_single_memory_reference(&recall_request, &memory_id, 1, ORIGINAL)?;

    let correct = responses::mount_sse_sequence(
        &model,
        vec![
            tool_sse(
                "correct-response",
                CORRECT_CALL,
                "correct",
                json!({
                    "memory_id": memory_id,
                    "expected_revision": 1,
                    "content": CORRECTED
                }),
            ),
            final_sse("correct-final"),
        ],
    )
    .await;
    product.run_turn(&thread_b, CORRECTED).await?;
    let correct_requests = correct.requests();
    ensure!(
        correct_requests.len() == 2,
        "correct must perform two physical sends"
    );
    assert_single_memory_reference(&correct_requests[0], &memory_id, 1, ORIGINAL)?;
    assert_no_cognitive_reference(&correct_requests[1])?;
    let corrected = tool_output(&correct, CORRECT_CALL)?;
    ensure!(corrected["operation"] == "corrected");
    ensure!(corrected["revision"] == 2);

    let rerecall = responses::mount_sse_sequence(&model, vec![final_sse("rerecall-final")]).await;
    product
        .run_turn(&thread_b, "Repeat the Project Aurora deadline.")
        .await?;
    let rerecall_request = rerecall.single_request();
    assert_single_memory_reference(&rerecall_request, &memory_id, 2, CORRECTED)?;
    ensure!(
        !attachment_text(&rerecall_request)?.contains(ORIGINAL),
        "superseded content reached a later physical send"
    );

    let forget = responses::mount_sse_sequence(
        &model,
        vec![
            tool_sse(
                "forget-response",
                FORGET_CALL,
                "forget",
                json!({
                    "memory_id": memory_id,
                    "expected_revision": 2,
                    "reason": FORGET_REASON
                }),
            ),
            final_sse("forget-final"),
        ],
    )
    .await;
    product.run_turn(&thread_b, FORGET_REASON).await?;
    let forget_requests = forget.requests();
    ensure!(
        forget_requests.len() == 2,
        "forget must perform two physical sends"
    );
    assert_single_memory_reference(&forget_requests[0], &memory_id, 2, CORRECTED)?;
    assert_no_cognitive_reference(&forget_requests[1])?;
    let forgotten = tool_output(&forget, FORGET_CALL)?;
    ensure!(forgotten["operation"] == "forgotten");
    ensure!(forgotten["revision"] == 3);
    ensure!(forgotten["lifecycle"]["state"] == "tombstoned");

    let after_forget =
        responses::mount_sse_sequence(&model, vec![final_sse("after-forget-final")]).await;
    product
        .run_turn(&thread_b, "What is the Project Aurora deadline?")
        .await?;
    assert_no_cognitive_reference(&after_forget.single_request())?;

    product.shutdown().await?;
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn two_real_agentd_app_servers_never_cross_recall() -> Result<()> {
    const MEMORY_A: &str = "Agent Alpha keeps the unique crimson cedar marker.";
    const MEMORY_B: &str = "Agent Beta keeps the unique cobalt ocean marker.";

    let mut fleet = FleetHarness::new()?;
    let agent_a = fleet.register(AGENT_A, "workspace-a")?;
    let agent_b = fleet.register(AGENT_B, "workspace-b")?;
    let model_a = responses::start_mock_server().await;
    let model_b = responses::start_mock_server().await;
    MockResponsesConfig::new(&model_a.uri()).write(agent_a.layout.home_root())?;
    MockResponsesConfig::new(&model_b.uri()).write(agent_b.layout.home_root())?;
    seed_verified_agent_memory(&agent_a, "agent-alpha-marker", MEMORY_A).await?;
    seed_verified_agent_memory(&agent_b, "agent-beta-marker", MEMORY_B).await?;

    fleet.start(&agent_a)?;
    fleet.start(&agent_b)?;
    let (control_a, _) = fleet.wait_ready(&agent_a, 1).await?;
    let (control_b, _) = fleet.wait_ready(&agent_b, 1).await?;
    let mut product_a = ProductClient::connect(&agent_a, &control_a).await?;
    let mut product_b = ProductClient::connect(&agent_b, &control_b).await?;
    let thread_a = product_a.start_thread(&agent_a.workspace).await?;
    let thread_b = product_b.start_thread(&agent_b.workspace).await?;

    let recall_a = responses::mount_sse_sequence(&model_a, vec![final_sse("agent-a-final")]).await;
    product_a
        .run_turn(&thread_a, "Recall the unique crimson cedar marker.")
        .await?;
    let request_a = recall_a.single_request();
    let text_a = attachment_text(&request_a)?;
    ensure!(
        text_a.contains(MEMORY_A),
        "agent A did not recall its own memory"
    );
    ensure!(
        !text_a.contains(MEMORY_B),
        "agent A crossed into agent B memory"
    );

    let recall_b = responses::mount_sse_sequence(&model_b, vec![final_sse("agent-b-final")]).await;
    product_b
        .run_turn(&thread_b, "Recall the unique cobalt ocean marker.")
        .await?;
    let request_b = recall_b.single_request();
    let text_b = attachment_text(&request_b)?;
    ensure!(
        text_b.contains(MEMORY_B),
        "agent B did not recall its own memory"
    );
    ensure!(
        !text_b.contains(MEMORY_A),
        "agent B crossed into agent A memory"
    );

    product_a.shutdown().await?;
    product_b.shutdown().await?;
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 6)]
async fn running_consumer_observes_owner_control_grant_and_revoke_without_restart() -> Result<()> {
    const SHARED: &str = "Owner Alpha shares the unique heliotrope observatory marker.";
    const LOCAL: &str = "Consumer Beta keeps the local heliotrope observatory notebook.";

    let mut fleet = FleetHarness::new()?;
    let owner = fleet.register(AGENT_A, "workspace-owner")?;
    let consumer = fleet.register(AGENT_B, "workspace-consumer")?;
    let owner_model = responses::start_mock_server().await;
    let consumer_model = responses::start_mock_server().await;
    MockResponsesConfig::new(&owner_model.uri()).write(owner.layout.home_root())?;
    MockResponsesConfig::new(&consumer_model.uri()).write(consumer.layout.home_root())?;
    seed_verified_agent_memory(&owner, "shared-observatory-marker", SHARED).await?;
    seed_verified_agent_memory(&consumer, "local-observatory-marker", LOCAL).await?;

    fleet.start(&owner)?;
    let (owner_control, _) = fleet.wait_ready(&owner, 1).await?;
    fleet.start(&consumer)?;
    let (consumer_control, _) = fleet.wait_ready(&consumer, 1).await?;
    let mut product = ProductClient::connect(&consumer, &consumer_control).await?;
    let thread = product.start_thread(&consumer.workspace).await?;

    let before = responses::mount_sse_once(&consumer_model, final_sse("before-grant")).await;
    product
        .run_turn(&thread, "Recall the heliotrope observatory marker.")
        .await
        .context("before-grant consumer turn")?;
    ensure!(
        !raw_requests_contain(&before, SHARED),
        "consumer saw owner memory before an explicit grant"
    );
    ensure!(
        raw_requests_contain(&before, LOCAL),
        "consumer did not retain its local cognitive memory before federation"
    );

    let capability = owner_control
        .memory_federation_grant(
            consumer.agent_id.clone(),
            MemoryFederationScopeKind::AgentPrivate,
            3_600,
        )
        .await?;
    ensure!(capability.state == MemoryFederationCapabilityState::Granted);
    ensure!(capability.owner_agent_id == owner.agent_id);
    ensure!(capability.consumer_agent_id == consumer.agent_id);
    let listed = owner_control.memory_federation_list(16).await?;
    ensure!(listed.as_slice() == std::slice::from_ref(&capability));
    ensure!(
        owner_control
            .memory_federation_status(capability.capability_id.clone())
            .await?
            == Some(capability.clone())
    );

    let after = responses::mount_sse_once(&consumer_model, final_sse("after-grant")).await;
    product
        .run_turn(&thread, "Recall the heliotrope observatory marker.")
        .await
        .context("after-grant consumer turn")?;
    ensure!(
        raw_requests_contain(&after, SHARED),
        "already-running consumer did not dynamically observe the owner grant"
    );
    ensure!(
        raw_requests_contain(&after, LOCAL),
        "single Cognitive proposal dropped local memory while merging federation"
    );
    ensure!(
        raw_requests_contain(&after, owner.agent_id.as_str()),
        "federated context omitted source AgentId provenance"
    );

    let nested_workspace = consumer.workspace.join("other-scope");
    std::fs::create_dir(&nested_workspace)?;
    let nested_workspace = nested_workspace.canonicalize()?;
    let nested_thread = product.start_thread(&nested_workspace).await?;
    let wrong_scope = responses::mount_sse_once(&consumer_model, final_sse("wrong-scope")).await;
    product
        .run_turn(&nested_thread, "Recall the heliotrope observatory marker.")
        .await
        .context("cross-workspace consumer turn")?;
    ensure!(
        !raw_requests_contain(&wrong_scope, SHARED),
        "grant escaped its registry-derived consumer workspace binding"
    );

    let self_grant = owner_control
        .memory_federation_grant(
            owner.agent_id.clone(),
            MemoryFederationScopeKind::AgentPrivate,
            3_600,
        )
        .await
        .expect_err("owner cannot grant to itself")
        .to_string();
    ensure!(self_grant.contains("another registered AgentId"));

    let first_seen = Arc::new(AtomicBool::new(false));
    let responder = DelayedToolSequence::new(
        Arc::clone(&first_seen),
        tool_sse(
            "federated-revalidate-tool",
            "federated-revalidate-recall",
            "recall",
            json!({"query": "local no-op query"}),
        ),
        final_sse("federated-revalidate-final"),
    );
    let delayed_start = consumer_model
        .received_requests()
        .await
        .unwrap_or_default()
        .len();
    Mock::given(method("POST"))
        .and(path("/v1/responses"))
        .respond_with(responder)
        .up_to_n_times(3)
        .mount(&consumer_model)
        .await;
    let turn = product.run_turn(&thread, "Recall the heliotrope observatory marker.");
    let revoke = async {
        timeout(Duration::from_secs(5), async {
            while !first_seen.load(Ordering::Acquire) {
                tokio::time::sleep(Duration::from_millis(5)).await;
            }
        })
        .await
        .context("first physical send did not reach the delayed model")?;
        let revoked = owner_control
            .memory_federation_revoke(capability.capability_id.clone())
            .await?;
        Ok::<_, anyhow::Error>(revoked)
    };
    let (turn_result, revoked) = tokio::join!(turn, revoke);
    turn_result.context("revalidation consumer turn")?;
    let revoked = revoked?;
    ensure!(revoked.state == MemoryFederationCapabilityState::Revoked);
    ensure!(revoked.revision == capability.revision + 1);

    let captured = consumer_model.received_requests().await.unwrap_or_default();
    let delayed = captured
        .iter()
        .skip(delayed_start)
        .map(|request| String::from_utf8_lossy(&request.body).into_owned())
        .collect::<Vec<_>>();
    ensure!(
        (2..=3).contains(&delayed.len()),
        "delayed model received an unexpected number of sends: {}",
        delayed.len()
    );
    ensure!(
        delayed[0].contains(SHARED),
        "prepared federated context was absent from the first physical send"
    );
    ensure!(
        delayed
            .iter()
            .skip(1)
            .all(|request| !request.contains(SHARED)),
        "revoked federated context survived a retry/follow-up physical-send revalidation"
    );
    ensure!(
        delayed.iter().all(|request| request.contains(LOCAL)),
        "local cognitive context was lost while federated context was revoked"
    );

    let owner_database = owner.layout.cognitive_root().join("cognitive_1.sqlite3");
    let unavailable_database = owner.layout.cognitive_root().join("cognitive_1.offline");
    std::fs::rename(&owner_database, &unavailable_database)?;
    let unavailable =
        responses::mount_sse_once(&consumer_model, final_sse("owner-unavailable")).await;
    product
        .run_turn(
            &thread,
            "Continue while the federation source is unavailable.",
        )
        .await
        .context("unavailable-source consumer turn")?;
    ensure!(
        !raw_requests_contain(&unavailable, SHARED),
        "unavailable owner source did not degrade to no federated context"
    );
    ensure!(
        consumer_control.health().await?.ready,
        "owner-source failure incorrectly blocked normal consumer turns"
    );
    std::fs::rename(&unavailable_database, &owner_database)?;

    product.shutdown().await?;
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn five_running_agents_share_only_with_the_explicit_consumer() -> Result<()> {
    const SHARED: &str = "Only Agent B may read the unique umber lighthouse marker.";

    let mut fleet = FleetHarness::new()?;
    let owner = fleet.register(AGENT_A, "workspace-a-five")?;
    let consumer = fleet.register(AGENT_B, "workspace-b-five")?;
    let agent_c = fleet.register(AGENT_C, "workspace-c-five")?;
    let agent_d = fleet.register(AGENT_D, "workspace-d-five")?;
    let agent_e = fleet.register(AGENT_E, "workspace-e-five")?;
    let fixtures = [&owner, &consumer, &agent_c, &agent_d, &agent_e];
    let mut models = Vec::new();
    for fixture in fixtures {
        let model = responses::start_mock_server().await;
        MockResponsesConfig::new(&model.uri()).write(fixture.layout.home_root())?;
        models.push(model);
    }
    seed_verified_agent_memory(&owner, "five-agent-shared-marker", SHARED).await?;

    let mut controls = Vec::new();
    for fixture in fixtures {
        fleet.start(fixture)?;
        controls.push(fleet.wait_ready(fixture, 1).await?.0);
    }
    controls[0]
        .memory_federation_grant(
            consumer.agent_id.clone(),
            MemoryFederationScopeKind::AgentPrivate,
            3_600,
        )
        .await?;

    for (index, fixture) in fixtures.iter().enumerate().skip(1) {
        let mut product = ProductClient::connect(fixture, &controls[index]).await?;
        let thread = product.start_thread(&fixture.workspace).await?;
        let response =
            responses::mount_sse_once(&models[index], final_sse(&format!("five-agent-{index}")))
                .await;
        product
            .run_turn(&thread, "Recall the unique umber lighthouse marker.")
            .await?;
        ensure!(
            raw_requests_contain(&response, SHARED) == (index == 1),
            "five-Agent federation leaked to the wrong consumer index {index}"
        );
        product.shutdown().await?;
    }
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn unavailable_cognitive_store_starts_and_fails_open_with_typed_tools() -> Result<()> {
    const UNAVAILABLE_CALL: &str = "unavailable-remember";
    const DIRECTIVE: &str = "Remember the unavailable runtime probe.";

    let mut fleet = FleetHarness::new()?;
    let agent = fleet.register(AGENT_A, "workspace-a")?;
    let model = responses::start_mock_server().await;
    MockResponsesConfig::new(&model.uri()).write(agent.layout.home_root())?;
    let blocking_path = agent.layout.cognitive_root().join("cognitive_1.sqlite3");
    std::fs::create_dir(&blocking_path)?;

    fleet.start(&agent)?;
    let (control, _) = fleet.wait_ready(&agent, 1).await?;
    ensure!(control.health().await?.ready, "agentd did not stay ready");
    let mut product = ProductClient::connect(&agent, &control).await?;
    let thread = product.start_thread(&agent.workspace).await?;
    let unavailable = responses::mount_sse_sequence(
        &model,
        vec![
            tool_sse(
                "unavailable-response",
                UNAVAILABLE_CALL,
                "remember",
                json!({
                    "stable_key": "unavailable-probe",
                    "content": DIRECTIVE,
                    "scope": "workspace_private"
                }),
            ),
            final_sse("unavailable-final"),
        ],
    )
    .await;
    product.run_turn(&thread, DIRECTIVE).await?;
    let requests = unavailable.requests();
    ensure!(
        requests.len() == 2,
        "typed unavailable tool must reach a follow-up send"
    );
    assert_no_cognitive_reference(&requests[0])?;
    ensure!(
        requests[0]
            .tool_by_name(COGNITIVE_NAMESPACE, "remember")
            .is_some(),
        "Unavailable incorrectly removed explicit cognitive tools"
    );
    for operation in ["recall", "correct", "forget", "explain"] {
        ensure!(
            requests[0]
                .tool_by_name(COGNITIVE_NAMESPACE, operation)
                .is_some(),
            "Unavailable incorrectly removed {operation}"
        );
    }
    assert_no_cognitive_reference(&requests[1])?;
    let output = unavailable
        .function_call_output_text(UNAVAILABLE_CALL)
        .context("unavailable tool output did not reach the model")?;
    let typed: Value = serde_json::from_str(&output)?;
    ensure!(typed["error"]["code"] == "hepta_cognitive_unavailable");
    ensure!(
        typed["error"]["message"]
            .as_str()
            .is_some_and(|message| message.contains("storage_unavailable")),
        "typed error omitted the sanitized stable reason"
    );
    ensure!(
        !output.contains(blocking_path.to_string_lossy().as_ref())
            && !output.to_ascii_lowercase().contains("sqlite"),
        "typed unavailable error leaked storage details"
    );

    product.shutdown().await?;
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn unavailable_automation_store_keeps_real_agentd_and_app_server_ready() -> Result<()> {
    let mut fleet = FleetHarness::new()?;
    let agent = fleet.register(AGENT_A, "workspace-a")?;
    let model = responses::start_mock_server().await;
    MockResponsesConfig::new(&model.uri()).write(agent.layout.home_root())?;
    let blocking_path = agent.layout.automation_root().join("automation_1.sqlite3");
    std::fs::create_dir(&blocking_path)?;

    fleet.start(&agent)?;
    let (control, _) = fleet.wait_ready(&agent, 1).await?;
    ensure!(control.health().await?.ready, "agentd did not stay ready");
    let error = control
        .automation_list(1)
        .await
        .expect_err("unavailable automation must return a typed error")
        .to_string();
    ensure!(
        error.contains("automation_unavailable"),
        "typed automation error omitted its stable code: {error}"
    );
    ensure!(
        !error.contains(blocking_path.to_string_lossy().as_ref())
            && !error.to_ascii_lowercase().contains("sqlite"),
        "typed automation error leaked storage details"
    );

    let mut product = ProductClient::connect(&agent, &control).await?;
    let thread = product.start_thread(&agent.workspace).await?;
    let normal = responses::mount_sse_sequence(
        &model,
        vec![final_sse("automation-unavailable-normal-turn")],
    )
    .await;
    product
        .run_turn(&thread, "Continue normal work without automation storage.")
        .await?;
    ensure!(
        normal.requests().len() == 1,
        "normal App Server turn did not survive automation storage outage"
    );
    product.shutdown().await?;
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn runtime_automation_store_failure_does_not_end_real_agentd_or_normal_turns() -> Result<()> {
    let mut fleet = FleetHarness::new()?;
    let agent = fleet.register(AGENT_A, "workspace-a")?;
    let model = responses::start_mock_server().await;
    MockResponsesConfig::new(&model.uri()).write(agent.layout.home_root())?;

    fleet.start(&agent)?;
    let (control, _) = fleet.wait_ready(&agent, 1).await?;
    let mut product = ProductClient::connect(&agent, &control).await?;
    let thread = product.start_thread(&agent.workspace).await?;
    let created_at_ms = u64::try_from(SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis())?;
    let task = control
        .automation_create(AutomationTaskDraft::new(
            thread.clone(),
            "runtime automation store failure probe",
            AutomationSchedule::Once,
            created_at_ms + 60_000,
            created_at_ms,
        ))
        .await?;
    ensure!(
        control.automation_list(1).await? == [task],
        "automation was not available before runtime sabotage"
    );

    let database_path = agent.layout.automation_root().join("automation_1.sqlite3");
    let sqlite_home = AbsolutePathBuf::from_absolute_path(agent.layout.automation_root())?;
    let sabotage = SqliteConfig::from_sqlite_home(sqlite_home)
        .open_durable_evidence_pool(&database_path)
        .await?;
    sqlx::query("DROP TABLE automation_tasks")
        .execute(&sabotage)
        .await?;
    sabotage.close().await;

    let error = timeout(Duration::from_secs(5), async {
        loop {
            match control.automation_list(1).await {
                Ok(_) => tokio::time::sleep(Duration::from_millis(25)).await,
                Err(error) => {
                    let rendered = error.to_string();
                    if rendered.contains("automation_unavailable") {
                        return Ok::<String, anyhow::Error>(rendered);
                    }
                    return Err(anyhow!(
                        "unexpected automation control failure after store sabotage: {rendered}"
                    ));
                }
            }
        }
    })
    .await
    .context("runtime automation store failure was not quarantined")??;
    ensure!(
        !error.contains(database_path.to_string_lossy().as_ref())
            && !error.to_ascii_lowercase().contains("sqlite"),
        "runtime automation error leaked storage details"
    );
    ensure!(
        control.health().await?.ready,
        "automation failure incorrectly stopped the real agentd"
    );

    let normal = responses::mount_sse_sequence(
        &model,
        vec![final_sse("runtime-automation-unavailable-normal-turn")],
    )
    .await;
    product
        .run_turn(
            &thread,
            "Continue normal work after the automation store failed.",
        )
        .await?;
    ensure!(
        normal.requests().len() == 1,
        "normal model turn did not survive runtime automation failure"
    );

    product.shutdown().await?;
    Ok(())
}

fn tool_sse(response_id: &str, call_id: &str, operation: &str, arguments: Value) -> String {
    responses::sse(vec![
        responses::ev_response_created(response_id),
        responses::ev_function_call_with_namespace(
            call_id,
            COGNITIVE_NAMESPACE,
            operation,
            &arguments.to_string(),
        ),
        responses::ev_completed(response_id),
    ])
}

fn final_sse(response_id: &str) -> String {
    responses::sse(vec![
        responses::ev_response_created(response_id),
        responses::ev_assistant_message(&format!("message-{response_id}"), "done"),
        responses::ev_completed(response_id),
    ])
}

struct DelayedToolSequence {
    call_count: AtomicUsize,
    first_seen: Arc<AtomicBool>,
    first: String,
    second: String,
}

impl DelayedToolSequence {
    fn new(first_seen: Arc<AtomicBool>, first: String, second: String) -> Self {
        Self {
            call_count: AtomicUsize::new(0),
            first_seen,
            first,
            second,
        }
    }
}

impl Respond for DelayedToolSequence {
    fn respond(&self, _request: &Request) -> ResponseTemplate {
        let call = self.call_count.fetch_add(1, Ordering::AcqRel);
        let body = match call {
            0 => {
                self.first_seen.store(true, Ordering::Release);
                // TurnInput preparation and the first physical request have
                // completed, while the model response is still withheld. This
                // gives owner control a deterministic revocation window before
                // the tool follow-up creates the next physical send.
                std::thread::sleep(Duration::from_millis(750));
                &self.first
            }
            _ => &self.second,
        };
        ResponseTemplate::new(200)
            .insert_header("content-type", "text/event-stream")
            .set_body_string(body.clone())
    }
}

fn raw_requests_contain(mock: &ResponseMock, needle: &str) -> bool {
    mock.requests()
        .iter()
        .any(|request| request.body_contains_text(needle))
}

fn tool_output(mock: &ResponseMock, call_id: &str) -> Result<Value> {
    let output = mock
        .function_call_output_text(call_id)
        .with_context(|| format!("tool output for {call_id} did not reach the model"))?;
    serde_json::from_str(&output).context("cognitive tool output was not JSON")
}

fn cognitive_attachments(request: &ResponsesRequest) -> Result<Vec<Value>> {
    request
        .message_input_texts("user")
        .into_iter()
        .filter(|text| text.starts_with(COGNITIVE_REFERENCE_OPEN))
        .map(|text| {
            let envelope = text
                .strip_prefix(COGNITIVE_REFERENCE_OPEN)
                .and_then(|text| text.strip_suffix(COGNITIVE_REFERENCE_CLOSE))
                .map(str::trim)
                .context("malformed cognitive reference wrapper")?;
            let envelope: Value = serde_json::from_str(envelope)?;
            ensure!(envelope["trust"] == "quoted_untrusted_reference");
            let summary = envelope["summary"]
                .as_str()
                .context("cognitive reference omitted its quoted summary")?;
            let attachment: Value = serde_json::from_str(summary)?;
            ensure!(attachment["schema_version"] == 1);
            ensure!(attachment["source"] == "verified_versioned_memory");
            Ok(attachment)
        })
        .collect()
}

fn assert_single_memory_reference(
    request: &ResponsesRequest,
    memory_id: &str,
    revision: u64,
    content: &str,
) -> Result<()> {
    let attachments = cognitive_attachments(request)?;
    ensure!(
        attachments.len() == 1,
        "expected exactly one cognitive attachment"
    );
    let memories = attachments[0]["memories"]
        .as_array()
        .context("cognitive attachment omitted memories")?;
    ensure!(memories.len() == 1, "expected exactly one attached memory");
    ensure!(memories[0]["memory_id"] == memory_id);
    ensure!(memories[0]["revision"] == revision);
    ensure!(memories[0]["content"] == content);
    Ok(())
}

fn assert_no_cognitive_reference(request: &ResponsesRequest) -> Result<()> {
    ensure!(
        cognitive_attachments(request)?.is_empty(),
        "unexpected cognitive reference reached a physical provider request"
    );
    Ok(())
}

fn attachment_text(request: &ResponsesRequest) -> Result<String> {
    let attachments = cognitive_attachments(request)?;
    ensure!(
        !attachments.is_empty(),
        "physical request contained no cognitive attachment"
    );
    Ok(serde_json::to_string(&attachments)?)
}

async fn seed_verified_agent_memory(
    agent: &AgentFixture,
    stable_key: &str,
    content: &str,
) -> Result<()> {
    let store = CognitiveStore::open(&agent.layout).await?;
    ensure!(store.owner_agent_id() == &agent.agent_id);
    let access = CognitiveAccess::agent_private(agent.agent_id.clone());
    let scope = CognitiveScope::AgentPrivate;
    let now = i64::try_from(SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs())?;
    let citation = store
        .append_source(
            &access,
            &SourceDraft {
                scope: scope.clone(),
                kind: LedgerSourceKind::ExplicitMemoryDirective,
                event_key: format!("seed:{stable_key}"),
                content: content.as_bytes().to_vec(),
                observed_at_unix_seconds: now,
            },
        )
        .await?;
    store
        .remember_memory(
            &access,
            &MemoryDraft {
                stable_key: stable_key.to_string(),
                revision: MemoryRevisionDraft {
                    scope,
                    content: content.to_string(),
                    verification: MemoryVerification::Verified,
                    lifecycle: MemoryLifecycleState::Active,
                    valid_from_unix_seconds: now,
                    valid_to_unix_seconds: None,
                    citations: vec![citation],
                },
            },
        )
        .await?;
    Ok(())
}

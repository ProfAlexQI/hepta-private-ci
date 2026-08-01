use super::*;
use crate::session::tests::make_session_and_context;
use crate::session::tests::make_session_and_context_with_rx;
use crate::state::ActiveTurn;
use crate::tools::context::ToolInvocation;
use crate::tools::context::ToolPayload;
use crate::turn_diff_tracker::TurnDiffTracker;
use codex_protocol::ThreadId;
use codex_protocol::protocol::EventMsg;
use codex_protocol::protocol::SessionSource;
use codex_protocol::protocol::SubAgentSource;
use codex_protocol::request_user_input::RequestUserInputResponse;
use pretty_assertions::assert_eq;
use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::test]
async fn multi_agent_v2_request_user_input_rejects_subagent_threads() {
    let (session, mut turn) = make_session_and_context().await;
    turn.session_source = SessionSource::SubAgent(SubAgentSource::ThreadSpawn {
        parent_thread_id: ThreadId::new(),
        depth: 1,
        agent_path: None,
        agent_nickname: None,
        agent_role: None,
    });

    let result = RequestUserInputHandler {
        available_modes: Vec::new(),
    }
    .handle(ToolInvocation {
        session: Arc::new(session),
        turn: Arc::new(turn),
        cancellation_token: tokio_util::sync::CancellationToken::new(),
        tracker: Arc::new(Mutex::new(TurnDiffTracker::default())),
        call_id: "call-1".to_string(),
        tool_name: codex_tools::ToolName::plain(REQUEST_USER_INPUT_TOOL_NAME),
        source: crate::tools::context::ToolCallSource::Direct,
        payload: ToolPayload::Function {
            arguments: json!({
                "questions": [{
                    "header": "Hdr",
                    "question": "Pick one",
                    "id": "pick_one",
                    "options": [
                        {
                            "label": "A",
                            "description": "A"
                        },
                        {
                            "label": "B",
                            "description": "B"
                        }
                    ]
                }]
            })
            .to_string(),
        },
    })
    .await;

    let Err(err) = result else {
        panic!("sub-agent request_user_input should fail");
    };
    assert_eq!(
        err,
        FunctionCallError::RespondToModel(
            "request_user_input can only be used by the root thread".to_string(),
        )
    );
}

async fn assert_request_user_input_blocking_for_mode(mode: ModeKind, expected: bool) {
    let (session, mut turn, events) = make_session_and_context_with_rx().await;
    Arc::get_mut(&mut turn)
        .expect("turn context should be uniquely owned")
        .collaboration_mode
        .mode = mode;
    *session.active_turn.lock().await = Some(ActiveTurn::default());

    let request = tokio::spawn({
        let session = Arc::clone(&session);
        let turn = Arc::clone(&turn);
        async move {
            RequestUserInputHandler {
                available_modes: vec![mode],
            }
            .handle(ToolInvocation {
                session,
                turn,
                cancellation_token: tokio_util::sync::CancellationToken::new(),
                tracker: Arc::new(Mutex::new(TurnDiffTracker::default())),
                call_id: "call-1".to_string(),
                tool_name: codex_tools::ToolName::plain(REQUEST_USER_INPUT_TOOL_NAME),
                source: crate::tools::context::ToolCallSource::Direct,
                payload: ToolPayload::Function {
                    arguments: json!({
                        "questions": [{
                            "header": "Hdr",
                            "question": "Pick one",
                            "id": "pick_one",
                            "options": [
                                { "label": "A", "description": "A" },
                                { "label": "B", "description": "B" }
                            ]
                        }]
                    })
                    .to_string(),
                },
            })
            .await
        }
    });

    let event = events.recv().await.expect("request_user_input event");
    let EventMsg::RequestUserInput(request_event) = event.msg else {
        panic!("expected request_user_input event");
    };
    assert_eq!(request_event.call_id, "call-1");
    assert_eq!(request_event.is_blocking, expected);
    assert_eq!(request_event.auto_resolution_ms, None);

    session
        .notify_user_input_response(
            &request_event.turn_id,
            RequestUserInputResponse {
                answers: HashMap::new(),
            },
        )
        .await;

    request
        .await
        .expect("request_user_input handler task should finish")
        .expect("request_user_input handler should succeed");
}

#[tokio::test]
async fn request_user_input_is_non_blocking_outside_plan_mode() {
    assert_request_user_input_blocking_for_mode(ModeKind::Default, false).await;
}

#[tokio::test]
async fn request_user_input_is_blocking_in_plan_mode() {
    assert_request_user_input_blocking_for_mode(ModeKind::Plan, true).await;
}

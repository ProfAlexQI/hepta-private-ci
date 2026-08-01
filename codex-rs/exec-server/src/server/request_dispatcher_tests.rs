use std::sync::Arc;

use codex_app_server_protocol::JSONRPCError;
use codex_app_server_protocol::JSONRPCErrorError;
use codex_app_server_protocol::JSONRPCRequest;
use codex_app_server_protocol::JSONRPCResponse;
use codex_app_server_protocol::RequestId;
use pretty_assertions::assert_eq;
use tokio::sync::mpsc;
use tokio::sync::watch;

use super::DispatchResult;
use super::RequestDispatcher;
use crate::ExecServerRuntimePaths;
use crate::rpc::RpcNotificationSender;
use crate::rpc::RpcServerOutboundMessage;
use crate::server::ExecServerHandler;
use crate::server::registry::build_router;
use crate::server::session_registry::SessionRegistry;

#[tokio::test]
async fn malformed_message_returns_invalid_request_without_closing_connection() {
    let (dispatcher, mut outgoing_rx, _disconnected_tx) = test_dispatcher();

    assert_eq!(
        dispatcher
            .handle_malformed_message("not JSON-RPC".to_string())
            .await,
        DispatchResult::Completed
    );
    assert_eq!(
        outgoing_rx.recv().await,
        Some(RpcServerOutboundMessage::Error {
            request_id: RequestId::Integer(-1),
            error: JSONRPCErrorError {
                code: -32600,
                data: None,
                message: "not JSON-RPC".to_string(),
            },
        })
    );
}

#[tokio::test]
async fn unknown_request_returns_method_not_found_without_closing_connection() {
    let (mut dispatcher, mut outgoing_rx, _disconnected_tx) = test_dispatcher();

    assert_eq!(
        dispatcher
            .dispatch_request(JSONRPCRequest {
                id: RequestId::Integer(7),
                method: "unknown/method".to_string(),
                params: None,
                trace: None,
            })
            .await,
        DispatchResult::Completed
    );
    assert_eq!(
        outgoing_rx.recv().await,
        Some(RpcServerOutboundMessage::Error {
            request_id: RequestId::Integer(7),
            error: JSONRPCErrorError {
                code: -32601,
                data: None,
                message: "exec-server stub does not implement `unknown/method` yet".to_string(),
            },
        })
    );
}

#[test]
fn unexpected_client_responses_and_errors_close_connection() {
    let (dispatcher, _outgoing_rx, _disconnected_tx) = test_dispatcher();

    assert_eq!(
        dispatcher.handle_response(JSONRPCResponse {
            id: RequestId::Integer(11),
            result: serde_json::json!({}),
        }),
        DispatchResult::ConnectionClosed
    );
    assert_eq!(
        dispatcher.handle_error(JSONRPCError {
            id: RequestId::Integer(12),
            error: JSONRPCErrorError {
                code: -32000,
                data: None,
                message: "unexpected".to_string(),
            },
        }),
        DispatchResult::ConnectionClosed
    );
}

fn test_dispatcher() -> (
    RequestDispatcher,
    mpsc::Receiver<RpcServerOutboundMessage>,
    watch::Sender<bool>,
) {
    let (outgoing_tx, outgoing_rx) = mpsc::channel(8);
    let notifications = RpcNotificationSender::new(outgoing_tx.clone());
    let handler = Arc::new(ExecServerHandler::new(
        SessionRegistry::new(),
        notifications,
        test_runtime_paths(),
    ));
    let (disconnected_tx, disconnected_rx) = watch::channel(false);
    (
        RequestDispatcher::new(
            Arc::new(build_router()),
            handler,
            outgoing_tx,
            disconnected_rx,
        ),
        outgoing_rx,
        disconnected_tx,
    )
}

fn test_runtime_paths() -> ExecServerRuntimePaths {
    ExecServerRuntimePaths::new(
        std::env::current_exe().expect("current executable"),
        /*codex_linux_sandbox_exe*/ None,
    )
    .expect("runtime paths")
}

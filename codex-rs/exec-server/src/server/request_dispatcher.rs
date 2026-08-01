use std::sync::Arc;

use codex_app_server_protocol::JSONRPCError;
use codex_app_server_protocol::JSONRPCNotification;
use codex_app_server_protocol::JSONRPCRequest;
use codex_app_server_protocol::JSONRPCResponse;
use codex_app_server_protocol::RequestId;
use tokio::sync::mpsc;
use tokio::sync::watch;
use tracing::debug;
use tracing::warn;

use crate::rpc::RpcRouter;
use crate::rpc::RpcServerOutboundMessage;
use crate::rpc::invalid_request;
use crate::rpc::method_not_found;
use crate::server::ExecServerHandler;

/// Dispatches one inbound JSON-RPC message at a time for a connection.
///
/// The connection loop deliberately awaits each operation before accepting the
/// next message. Keeping that ordering policy here prevents a long-running
/// request from being bypassed by a later request or notification.
pub(super) struct RequestDispatcher {
    router: Arc<RpcRouter<ExecServerHandler>>,
    handler: Arc<ExecServerHandler>,
    outgoing_tx: mpsc::Sender<RpcServerOutboundMessage>,
    disconnected_rx: watch::Receiver<bool>,
}

impl RequestDispatcher {
    pub(super) fn new(
        router: Arc<RpcRouter<ExecServerHandler>>,
        handler: Arc<ExecServerHandler>,
        outgoing_tx: mpsc::Sender<RpcServerOutboundMessage>,
        disconnected_rx: watch::Receiver<bool>,
    ) -> Self {
        Self {
            router,
            handler,
            outgoing_tx,
            disconnected_rx,
        }
    }

    pub(super) async fn handle_malformed_message(&self, reason: String) -> DispatchResult {
        warn!("ignoring malformed exec-server message: {reason}");
        if self
            .outgoing_tx
            .send(RpcServerOutboundMessage::Error {
                request_id: RequestId::Integer(-1),
                error: invalid_request(reason),
            })
            .await
            .is_err()
        {
            return DispatchResult::ConnectionClosed;
        }

        DispatchResult::Completed
    }

    pub(super) async fn handle_notification(
        &mut self,
        notification: JSONRPCNotification,
    ) -> DispatchResult {
        let Some(route) = self.router.notification_route(notification.method.as_str()) else {
            warn!(
                "closing exec-server connection after unexpected notification: {}",
                notification.method
            );
            return DispatchResult::ConnectionClosed;
        };
        let result = tokio::select! {
            result = route(Arc::clone(&self.handler), notification) => result,
            _ = self.disconnected_rx.changed() => {
                debug!("exec-server transport disconnected while handling notification");
                return DispatchResult::ConnectionClosed;
            }
        };
        if let Err(error) = result {
            warn!("closing exec-server connection after protocol error: {error}");
            return DispatchResult::ConnectionClosed;
        }

        DispatchResult::Completed
    }

    pub(super) fn handle_response(&self, response: JSONRPCResponse) -> DispatchResult {
        warn!(
            "closing exec-server connection after unexpected client response: {:?}",
            response.id
        );
        DispatchResult::ConnectionClosed
    }

    pub(super) fn handle_error(&self, error: JSONRPCError) -> DispatchResult {
        warn!(
            "closing exec-server connection after unexpected client error: {:?}",
            error.id
        );
        DispatchResult::ConnectionClosed
    }

    pub(super) async fn dispatch_request(&mut self, request: JSONRPCRequest) -> DispatchResult {
        let Some(route) = self.router.request_route(request.method.as_str()) else {
            if self
                .outgoing_tx
                .send(RpcServerOutboundMessage::Error {
                    request_id: request.id,
                    error: method_not_found(format!(
                        "exec-server stub does not implement `{}` yet",
                        request.method
                    )),
                })
                .await
                .is_err()
            {
                return DispatchResult::ConnectionClosed;
            }
            return DispatchResult::Completed;
        };

        let message = tokio::select! {
            message = route(Arc::clone(&self.handler), request) => message,
            _ = self.disconnected_rx.changed() => {
                debug!("exec-server transport disconnected while handling request");
                return DispatchResult::ConnectionClosed;
            }
        };
        if let Some(message) = message
            && self.outgoing_tx.send(message).await.is_err()
        {
            return DispatchResult::ConnectionClosed;
        }

        DispatchResult::Completed
    }
}

#[derive(Debug, Eq, PartialEq)]
pub(super) enum DispatchResult {
    Completed,
    ConnectionClosed,
}

#[cfg(test)]
#[path = "request_dispatcher_tests.rs"]
mod tests;

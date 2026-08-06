use crate::session::session::Session;
use codex_hepta_contracts::ChannelIngressEvent;
use codex_hepta_contracts::ChannelIngressEventId;
use codex_hepta_contracts::ChannelScope;
use codex_hepta_contracts::Sha256Digest;
use codex_hepta_contracts::channel_target_thread_sha256;
use codex_hepta_contracts::validate_ingress_event;
use tokio::sync::oneshot;

const MAX_CHANNEL_INGRESS_PAYLOAD_BYTES: usize = 64 * 1024;
const DENY_PROVIDER_AND_TOOLS_V1_CANONICAL: &str = concat!(
    "schema=hepta.channel_execution_policy.v1\n",
    "provider_http=deny\n",
    "provider_websocket=deny\n",
    "provider_compaction=deny\n",
    "provider_prewarm=deny\n",
    "tool_dispatch=deny\n",
);

/// Result of the hidden in-process channel ingress preflight.
///
/// `ObservedReady` is deliberately not a turn admission. The preflight does
/// not reserve or start a turn and therefore cannot be projected as an
/// accepted channel receipt. Its ordered state reads are a non-reserving,
/// best-effort observation that may immediately become stale; they are not an
/// atomic session snapshot.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ChannelIngressPreflightOutcome {
    ObservedReady {
        event_id: ChannelIngressEventId,
        /// Domain-separated binding of the complete typed event and the
        /// fixed no-execution canary selector observed by this preflight.
        preflight_binding_sha256: Sha256Digest,
    },
    /// Rejected either while validating the bounded envelope before enqueue,
    /// or while observing session state after dequeue.
    Rejected {
        reason: ChannelIngressPreflightRejection,
    },
    /// A valid bounded envelope did not enter this session FIFO.
    NotEnqueued {
        reason: ChannelIngressPreflightNotEnqueued,
    },
    /// The valid envelope entered this FIFO, but its processor or response
    /// was lost. This is indeterminate and cannot be projected as a receipt.
    ResponseLost,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ChannelIngressPreflightRejection {
    InvalidEvent,
    EmptyPayload,
    PayloadTooLarge,
    PayloadDigestMismatch,
    TargetThreadMismatch,
    ActiveTurn,
    PendingMailbox,
    DurableSleep,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ChannelIngressPreflightNotEnqueued {
    QueueFull,
    QueueClosed,
}

/// Fixed deny-only policy identity for the first in-process channel canary.
///
/// This digest is a secret-free selector, not execution authority. Callers do
/// not provide it to the Core preflight command.
pub fn channel_canary_deny_provider_and_tools_v1_sha256() -> Sha256Digest {
    Sha256Digest::for_bytes(DENY_PROVIDER_AND_TOOLS_V1_CANONICAL.as_bytes())
}

/// Core-private queue envelope. Keeping the exact event and bounded payload
/// in this non-cloneable command prevents cancellation from degrading it into
/// an ordinary protocol submission.
pub(crate) struct ChannelIngressPreflightCommand {
    event: ChannelIngressEvent,
    payload: Box<str>,
    response: oneshot::Sender<ChannelIngressPreflightOutcome>,
}

impl ChannelIngressPreflightCommand {
    pub(crate) fn new(
        event: ChannelIngressEvent,
        payload: String,
        response: oneshot::Sender<ChannelIngressPreflightOutcome>,
    ) -> Result<Self, ChannelIngressPreflightRejection> {
        validate_envelope(&event, &payload)?;
        Ok(Self {
            event,
            payload: payload.into_boxed_str(),
            response,
        })
    }

    pub(crate) async fn dispatch(self, session: &Session) {
        if self.response.is_closed() {
            return;
        }
        let outcome = evaluate(session, &self.event, &self.payload).await;
        let _ = self.response.send(outcome);
    }
}

async fn evaluate(
    session: &Session,
    event: &ChannelIngressEvent,
    payload: &str,
) -> ChannelIngressPreflightOutcome {
    let reject = |reason| ChannelIngressPreflightOutcome::Rejected { reason };

    if let Err(reason) = validate_envelope(event, payload) {
        return reject(reason);
    }

    let target_thread_sha256 = match channel_target_thread_sha256(&session.thread_id.to_string()) {
        Ok(digest) => digest,
        Err(_) => return reject(ChannelIngressPreflightRejection::TargetThreadMismatch),
    };
    if target_thread_sha256 != event.target_thread_sha256 {
        return reject(ChannelIngressPreflightRejection::TargetThreadMismatch);
    }

    if session.active_turn.lock().await.is_some() {
        return reject(ChannelIngressPreflightRejection::ActiveTurn);
    }
    if session.input_queue.has_pending_mailbox_items().await {
        return reject(ChannelIngressPreflightRejection::PendingMailbox);
    }
    if session.has_outstanding_durable_sleep() {
        return reject(ChannelIngressPreflightRejection::DurableSleep);
    }

    ChannelIngressPreflightOutcome::ObservedReady {
        event_id: event.event_id.clone(),
        preflight_binding_sha256: preflight_binding_sha256(event),
    }
}

fn validate_envelope(
    event: &ChannelIngressEvent,
    payload: &str,
) -> Result<(), ChannelIngressPreflightRejection> {
    // Reject an already allocated input before hashing, boxing, or retaining
    // it in the shared session FIFO.
    if payload.is_empty() {
        return Err(ChannelIngressPreflightRejection::EmptyPayload);
    }
    if payload.len() > MAX_CHANNEL_INGRESS_PAYLOAD_BYTES {
        return Err(ChannelIngressPreflightRejection::PayloadTooLarge);
    }
    if validate_ingress_event(event).is_err() {
        return Err(ChannelIngressPreflightRejection::InvalidEvent);
    }
    if Sha256Digest::for_bytes(payload.as_bytes()) != event.payload_sha256 {
        return Err(ChannelIngressPreflightRejection::PayloadDigestMismatch);
    }
    Ok(())
}

fn preflight_binding_sha256(event: &ChannelIngressEvent) -> Sha256Digest {
    let ChannelIngressEvent {
        schema_version,
        event_id,
        scope,
        source_event_sha256,
        payload_sha256,
        target_thread_sha256,
        predecessor_cursor_sha256,
        next_cursor_sha256,
        received_at_unix_ms,
    } = event;
    let ChannelScope {
        adapter_id,
        installation_sha256,
        account_sha256,
        conversation_sha256,
        principal_sha256,
    } = scope;
    let schema_version = schema_version.to_string();
    let received_at_unix_ms = received_at_unix_ms.to_string();
    let predecessor_tag = if predecessor_cursor_sha256.is_some() {
        "some"
    } else {
        "none"
    };
    let predecessor = predecessor_cursor_sha256
        .as_ref()
        .map(Sha256Digest::as_str)
        .unwrap_or_default();
    let policy = channel_canary_deny_provider_and_tools_v1_sha256();
    let mut canonical = Vec::new();
    for part in [
        "hepta-channel-ingress-core-preflight-v1",
        schema_version.as_str(),
        event_id.as_str(),
        adapter_id.as_str(),
        installation_sha256.as_str(),
        account_sha256.as_str(),
        conversation_sha256.as_str(),
        principal_sha256.as_str(),
        source_event_sha256.as_str(),
        payload_sha256.as_str(),
        target_thread_sha256.as_str(),
        predecessor_tag,
        predecessor,
        next_cursor_sha256.as_str(),
        received_at_unix_ms.as_str(),
        policy.as_str(),
    ] {
        canonical.extend_from_slice(&(part.len() as u64).to_be_bytes());
        canonical.extend_from_slice(part.as_bytes());
    }
    Sha256Digest::for_bytes(&canonical)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agent::AgentStatus;
    use crate::session::SessionCommand;
    use crate::session::SessionIo;
    use crate::session::completed_session_loop_termination;
    use crate::session::session_loop_termination_from_handle;
    use crate::session::submission_loop;
    use codex_hepta_contracts::ChannelAdapterId;
    use codex_hepta_contracts::ChannelScope;
    use codex_protocol::AgentPath;
    use codex_protocol::protocol::EventMsg;
    use codex_protocol::protocol::InterAgentCommunication;
    use codex_protocol::protocol::Op;
    use codex_protocol::protocol::Submission;
    use futures::FutureExt;
    use sha2::Digest;
    use sha2::Sha256;
    use std::sync::Arc;
    use tokio::sync::watch;

    fn digest(value: &str) -> Sha256Digest {
        Sha256Digest::for_bytes(value.as_bytes())
    }

    fn event_for(thread_id: &str, payload: &str) -> ChannelIngressEvent {
        ChannelIngressEvent::new(
            ChannelScope {
                adapter_id: ChannelAdapterId::new("native.app_server.loopback.v1")
                    .expect("adapter id"),
                installation_sha256: digest("installation"),
                account_sha256: digest("account"),
                conversation_sha256: digest("conversation"),
                principal_sha256: digest("principal"),
            },
            digest("source-event"),
            digest(payload),
            channel_target_thread_sha256(thread_id).expect("target thread"),
            None,
            digest("next-cursor"),
            1,
        )
        .expect("ingress event")
    }

    async fn running_io(session: Arc<Session>) -> SessionIo {
        let config = session.get_config().await;
        let (tx_sub, rx_sub) = async_channel::bounded(4);
        let (_tx_event, rx_event) = async_channel::unbounded();
        let handle = tokio::spawn(async move {
            submission_loop(session, config, rx_sub).await;
        });
        SessionIo {
            tx_sub,
            rx_event,
            agent_status: watch::channel(AgentStatus::PendingInit).1,
            session_loop_termination: session_loop_termination_from_handle(handle),
        }
    }

    #[test]
    fn fixed_deny_policy_digest_is_stable() {
        assert_eq!(
            channel_canary_deny_provider_and_tools_v1_sha256().as_str(),
            "08328af9fa2bd497f41ed8fe9307e87739958c5b9f4fe0768567db92628ad30f",
        );
    }

    #[test]
    fn canonical_policy_is_not_an_accidental_double_hash() {
        assert_eq!(
            channel_canary_deny_provider_and_tools_v1_sha256(),
            Sha256Digest::for_bytes(DENY_PROVIDER_AND_TOOLS_V1_CANONICAL.as_bytes()),
        );
        assert_eq!(
            format!(
                "{:x}",
                Sha256::digest(DENY_PROVIDER_AND_TOOLS_V1_CANONICAL.as_bytes())
            ),
            channel_canary_deny_provider_and_tools_v1_sha256().as_str(),
        );
    }

    #[tokio::test]
    async fn exact_preflight_observes_ready_without_turn_history_or_events() {
        let (session, _turn, rx_event) =
            crate::session::tests::make_session_and_context_with_rx().await;
        let history_before = session.clone_history().await.into_raw_items();
        let event = event_for(&session.thread_id.to_string(), "bounded channel input");
        let expected_event_id = event.event_id.clone();
        let expected_binding = preflight_binding_sha256(&event);
        let io = running_io(Arc::clone(&session)).await;

        assert_eq!(
            io.preflight_channel_ingress(event, "bounded channel input".to_string())
                .await,
            ChannelIngressPreflightOutcome::ObservedReady {
                event_id: expected_event_id,
                preflight_binding_sha256: expected_binding,
            }
        );
        assert!(session.active_turn.lock().await.is_none());
        assert_eq!(
            session.clone_history().await.into_raw_items(),
            history_before
        );
        while let Ok(event) = rx_event.try_recv() {
            assert!(!matches!(
                event.msg,
                EventMsg::TurnStarted(_) | EventMsg::TurnComplete(_)
            ));
        }

        io.shutdown_and_wait().await.expect("shutdown session loop");
    }

    #[tokio::test]
    async fn payload_target_and_schema_substitution_fail_closed() {
        let (session, _turn) = crate::session::tests::make_session_and_context().await;
        let session = Arc::new(session);
        let exact = event_for(&session.thread_id.to_string(), "exact payload");
        let io = running_io(Arc::clone(&session)).await;

        assert_eq!(
            io.preflight_channel_ingress(exact.clone(), "substituted payload".to_string())
                .await,
            ChannelIngressPreflightOutcome::Rejected {
                reason: ChannelIngressPreflightRejection::PayloadDigestMismatch,
            }
        );

        let mut wrong_target = exact.clone();
        wrong_target.target_thread_sha256 = digest("foreign-thread");
        assert_eq!(
            io.preflight_channel_ingress(wrong_target, "exact payload".to_string())
                .await,
            ChannelIngressPreflightOutcome::Rejected {
                reason: ChannelIngressPreflightRejection::TargetThreadMismatch,
            }
        );

        let mut wrong_schema = exact;
        wrong_schema.schema_version += 1;
        assert_eq!(
            io.preflight_channel_ingress(wrong_schema, "exact payload".to_string())
                .await,
            ChannelIngressPreflightOutcome::Rejected {
                reason: ChannelIngressPreflightRejection::InvalidEvent,
            }
        );
        assert!(session.active_turn.lock().await.is_none());

        io.shutdown_and_wait().await.expect("shutdown session loop");
    }

    #[tokio::test]
    async fn active_turn_pending_mailbox_and_durable_sleep_are_read_only_rejections() {
        let (session, _turn) = crate::session::tests::make_session_and_context().await;
        let event = event_for(&session.thread_id.to_string(), "exact payload");

        *session.active_turn.lock().await = Some(crate::state::ActiveTurn::default());
        assert_eq!(
            evaluate(&session, &event, "exact payload",).await,
            ChannelIngressPreflightOutcome::Rejected {
                reason: ChannelIngressPreflightRejection::ActiveTurn,
            }
        );
        assert!(session.active_turn.lock().await.is_some());

        *session.active_turn.lock().await = None;
        session
            .input_queue
            .enqueue_mailbox_communication(
                InterAgentCommunication::new(
                    AgentPath::root(),
                    AgentPath::root(),
                    Vec::new(),
                    "pending".to_string(),
                    true,
                ),
                None,
            )
            .await;
        assert_eq!(
            evaluate(&session, &event, "exact payload",).await,
            ChannelIngressPreflightOutcome::Rejected {
                reason: ChannelIngressPreflightRejection::PendingMailbox,
            }
        );
        session.input_queue.drain_mailbox_input_items().await;

        session
            .services
            .thread_extension_data
            .insert(codex_extension_items::sleep::SleepItem {
                id: "sleep-1".to_string(),
                duration_ms: 1,
            });
        assert_eq!(
            evaluate(&session, &event, "exact payload",).await,
            ChannelIngressPreflightOutcome::Rejected {
                reason: ChannelIngressPreflightRejection::DurableSleep,
            }
        );
    }

    #[test]
    fn complete_event_and_fixed_selector_have_one_stable_preflight_binding() {
        let exact = event_for("00000000-0000-0000-0000-000000000000", "payload");
        let binding = preflight_binding_sha256(&exact);
        assert_eq!(
            binding.as_str(),
            "ecc2088b7fa4041c931a5d032e4c6ffcc5b346f419c6b43468147808c2d7aba8",
        );

        let mut substitutions = Vec::new();
        let mut changed = exact.clone();
        changed.schema_version += 1;
        substitutions.push(changed);
        let mut changed = exact.clone();
        changed.event_id =
            ChannelIngressEventId::for_event(&changed.scope, &digest("changed-event-identity"));
        substitutions.push(changed);
        let mut changed = exact.clone();
        changed.scope.adapter_id =
            ChannelAdapterId::new("native.app_server.changed.v1").expect("adapter id");
        substitutions.push(changed);
        let mut changed = exact.clone();
        changed.scope.installation_sha256 = digest("changed-installation");
        substitutions.push(changed);
        let mut changed = exact.clone();
        changed.scope.account_sha256 = digest("changed-account");
        substitutions.push(changed);
        let mut changed = exact.clone();
        changed.scope.conversation_sha256 = digest("changed-conversation");
        substitutions.push(changed);
        let mut changed = exact.clone();
        changed.scope.principal_sha256 = digest("changed-principal");
        substitutions.push(changed);
        let mut changed = exact.clone();
        changed.source_event_sha256 = digest("changed-source");
        substitutions.push(changed);
        let mut changed = exact.clone();
        changed.payload_sha256 = digest("changed-payload");
        substitutions.push(changed);
        let mut changed = exact.clone();
        changed.target_thread_sha256 = digest("changed-target");
        substitutions.push(changed);
        let mut changed = exact.clone();
        changed.predecessor_cursor_sha256 = Some(digest("predecessor"));
        substitutions.push(changed);
        let mut changed = exact.clone();
        changed.next_cursor_sha256 = digest("changed-cursor");
        substitutions.push(changed);
        let mut changed = exact;
        changed.received_at_unix_ms += 1;
        substitutions.push(changed);

        for changed in substitutions {
            assert_ne!(binding, preflight_binding_sha256(&changed));
        }
    }

    #[tokio::test]
    async fn oversized_payload_is_rejected_before_entering_the_session_fifo() {
        let (session, _turn) = crate::session::tests::make_session_and_context().await;
        let payload = "x".repeat(MAX_CHANNEL_INGRESS_PAYLOAD_BYTES + 1);
        let event = event_for(&session.thread_id.to_string(), &payload);
        let (tx_sub, rx_sub) = async_channel::bounded(1);
        let (_tx_event, rx_event) = async_channel::unbounded();
        let io = SessionIo {
            tx_sub,
            rx_event,
            agent_status: watch::channel(AgentStatus::PendingInit).1,
            session_loop_termination: completed_session_loop_termination(),
        };

        assert_eq!(
            io.preflight_channel_ingress(event, payload).await,
            ChannelIngressPreflightOutcome::Rejected {
                reason: ChannelIngressPreflightRejection::PayloadTooLarge,
            }
        );
        assert!(matches!(
            rx_sub.try_recv(),
            Err(async_channel::TryRecvError::Empty)
        ));
    }

    #[tokio::test]
    async fn submission_loop_preserves_protocol_and_preflight_fifo_order() {
        let (session, _turn) = crate::session::tests::make_session_and_context().await;
        let session = Arc::new(session);
        *session.active_turn.lock().await = Some(crate::state::ActiveTurn::default());
        let event = event_for(&session.thread_id.to_string(), "payload");
        let expected_binding = preflight_binding_sha256(&event);
        let (tx_sub, rx_sub) = async_channel::bounded(4);

        let (before_tx, before_rx) = tokio::sync::oneshot::channel();
        tx_sub
            .send(SessionCommand::ChannelIngressPreflight(Box::new(
                ChannelIngressPreflightCommand::new(
                    event.clone(),
                    "payload".to_string(),
                    before_tx,
                )
                .expect("bounded preflight"),
            )))
            .await
            .expect("queue first preflight");
        tx_sub
            .send(SessionCommand::Protocol(Box::new(Submission {
                id: "interrupt".to_string(),
                op: Op::Interrupt,
                client_user_message_id: None,
                trace: None,
                parent_turn_id: None,
            })))
            .await
            .expect("queue interrupt");
        let (after_tx, after_rx) = tokio::sync::oneshot::channel();
        tx_sub
            .send(SessionCommand::ChannelIngressPreflight(Box::new(
                ChannelIngressPreflightCommand::new(event.clone(), "payload".to_string(), after_tx)
                    .expect("bounded preflight"),
            )))
            .await
            .expect("queue second preflight");
        tx_sub
            .send(SessionCommand::Protocol(Box::new(Submission {
                id: "shutdown".to_string(),
                op: Op::Shutdown,
                client_user_message_id: None,
                trace: None,
                parent_turn_id: None,
            })))
            .await
            .expect("queue shutdown");
        drop(tx_sub);

        submission_loop(Arc::clone(&session), session.get_config().await, rx_sub).await;

        assert_eq!(
            before_rx.await.expect("first preflight response"),
            ChannelIngressPreflightOutcome::Rejected {
                reason: ChannelIngressPreflightRejection::ActiveTurn,
            }
        );
        assert_eq!(
            after_rx.await.expect("second preflight response"),
            ChannelIngressPreflightOutcome::ObservedReady {
                event_id: event.event_id,
                preflight_binding_sha256: expected_binding,
            }
        );
    }

    #[tokio::test]
    async fn full_and_closed_queue_are_definitive_not_enqueued() {
        let (session, _turn) = crate::session::tests::make_session_and_context().await;
        let event = event_for(&session.thread_id.to_string(), "payload");
        let (tx_sub, rx_sub) = async_channel::bounded(1);
        tx_sub
            .try_send(SessionCommand::Protocol(Box::new(Submission {
                id: "occupied".to_string(),
                op: Op::Interrupt,
                client_user_message_id: None,
                trace: None,
                parent_turn_id: None,
            })))
            .expect("fill queue");
        let (_tx_event, rx_event) = async_channel::unbounded();
        let full_io = SessionIo {
            tx_sub,
            rx_event,
            agent_status: watch::channel(AgentStatus::PendingInit).1,
            session_loop_termination: completed_session_loop_termination(),
        };
        assert_eq!(
            full_io
                .preflight_channel_ingress(event.clone(), "payload".to_string())
                .await,
            ChannelIngressPreflightOutcome::NotEnqueued {
                reason: ChannelIngressPreflightNotEnqueued::QueueFull,
            }
        );
        drop(rx_sub);

        let (tx_sub, rx_sub) = async_channel::bounded(1);
        drop(rx_sub);
        let (_tx_event, rx_event) = async_channel::unbounded();
        let closed_io = SessionIo {
            tx_sub,
            rx_event,
            agent_status: watch::channel(AgentStatus::PendingInit).1,
            session_loop_termination: completed_session_loop_termination(),
        };
        assert_eq!(
            closed_io
                .preflight_channel_ingress(event, "payload".to_string())
                .await,
            ChannelIngressPreflightOutcome::NotEnqueued {
                reason: ChannelIngressPreflightNotEnqueued::QueueClosed,
            }
        );
    }

    #[tokio::test]
    async fn session_termination_after_enqueue_is_response_lost_without_variant_fallback() {
        let (session, _turn) = crate::session::tests::make_session_and_context().await;
        let event = event_for(&session.thread_id.to_string(), "payload");
        let (tx_sub, rx_sub) = async_channel::bounded(1);
        let (_tx_event, rx_event) = async_channel::unbounded();
        let io = SessionIo {
            tx_sub,
            rx_event,
            agent_status: watch::channel(AgentStatus::PendingInit).1,
            session_loop_termination: completed_session_loop_termination(),
        };

        assert_eq!(
            io.preflight_channel_ingress(event, "payload".to_string())
                .await,
            ChannelIngressPreflightOutcome::ResponseLost
        );
        assert!(matches!(
            rx_sub.recv().await.expect("queued preflight"),
            SessionCommand::ChannelIngressPreflight(_)
        ));
    }

    #[tokio::test]
    async fn cancellation_and_processor_loss_cannot_fall_back_to_protocol_input() {
        let (session, _turn) = crate::session::tests::make_session_and_context().await;
        let session = Arc::new(session);
        let event = event_for(&session.thread_id.to_string(), "payload");
        let (tx_sub, rx_sub) = async_channel::bounded(1);
        let (_tx_event, rx_event) = async_channel::unbounded();
        let io = Arc::new(SessionIo {
            tx_sub,
            rx_event,
            agent_status: watch::channel(AgentStatus::PendingInit).1,
            session_loop_termination: futures::future::pending::<()>().boxed().shared(),
        });

        let waiting = {
            let io = Arc::clone(&io);
            let event = event.clone();
            tokio::spawn(async move {
                io.preflight_channel_ingress(event, "payload".to_string())
                    .await
            })
        };
        let command = rx_sub.recv().await.expect("queued preflight");
        assert!(matches!(
            &command,
            SessionCommand::ChannelIngressPreflight(_)
        ));
        waiting.abort();
        assert!(
            waiting
                .await
                .expect_err("cancelled caller join")
                .is_cancelled()
        );
        let SessionCommand::ChannelIngressPreflight(command) = command else {
            unreachable!("command was checked above")
        };
        (*command).dispatch(session.as_ref()).await;
        assert!(session.active_turn.lock().await.is_none());
        assert!(session.clone_history().await.raw_items().is_empty());

        let response_lost = {
            let io = Arc::clone(&io);
            tokio::spawn(async move {
                io.preflight_channel_ingress(event, "payload".to_string())
                    .await
            })
        };
        drop(rx_sub.recv().await.expect("queued preflight"));
        assert_eq!(
            response_lost.await.expect("response-loss join"),
            ChannelIngressPreflightOutcome::ResponseLost
        );
    }
}

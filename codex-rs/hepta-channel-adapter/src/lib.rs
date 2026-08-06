#![forbid(unsafe_code)]

use codex_app_server_client::InProcessAppServerRequestHandle;
use codex_app_server_client::InProcessChannelIngressNotEnqueued;
use codex_app_server_client::InProcessChannelIngressPrepareOutcome;
use codex_app_server_client::InProcessChannelIngressRejection;
use codex_app_server_client::InProcessChannelIngressResponseLost;
use codex_app_server_client::InProcessChannelIngressTakeOutcome;
use codex_app_server_client::InProcessChannelIngressUnavailable;
use codex_hepta_contracts::ChannelIngressEvent;
use codex_hepta_contracts::ChannelIngressEventId;
use codex_hepta_contracts::Sha256Digest;
use codex_hepta_contracts::channel_target_thread_sha256;
use codex_protocol::ThreadId;

pub const MAX_CHANNEL_TEXT_BYTES: usize = 64 * 1024;
pub const NATIVE_LOOPBACK_ADAPTER_ID: &str = "native.app_server.loopback.v1";

pub struct ChannelInboundText {
    text: String,
    payload_sha256: Sha256Digest,
}

impl ChannelInboundText {
    pub fn new(text: impl Into<String>) -> Result<Self, ChannelAdapterError> {
        let text = text.into();
        if text.is_empty() {
            return Err(ChannelAdapterError::EmptyPayload);
        }
        if text.len() > MAX_CHANNEL_TEXT_BYTES {
            return Err(ChannelAdapterError::PayloadTooLarge);
        }
        let payload_sha256 = Sha256Digest::for_bytes(text.as_bytes());
        Ok(Self {
            text,
            payload_sha256,
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum ChannelAdapterError {
    #[error("channel payload is empty")]
    EmptyPayload,
    #[error("channel payload exceeds the fixed adapter bound")]
    PayloadTooLarge,
    #[error("channel event is not addressed to the native loopback adapter")]
    AdapterMismatch,
    #[error("channel event payload digest does not match the exact text")]
    PayloadDigestMismatch,
    #[error("channel event target does not match the exact thread")]
    TargetThreadMismatch,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NativeChannelIngressOutcome {
    ObservedReady {
        event_id: ChannelIngressEventId,
        preflight_binding_sha256: Sha256Digest,
    },
    Rejected {
        reason: InProcessChannelIngressRejection,
    },
    Unavailable {
        reason: InProcessChannelIngressUnavailable,
    },
    NotEnqueued {
        reason: InProcessChannelIngressNotEnqueued,
    },
    ResponseLost {
        phase: InProcessChannelIngressResponseLost,
    },
}

pub struct NativeChannelIngressAdapter;

impl NativeChannelIngressAdapter {
    /// Performs the host-owned prepare/take pair without starting or reserving
    /// a turn. Success remains a best-effort observation, never a receipt.
    pub async fn observe(
        request_handle: &InProcessAppServerRequestHandle,
        thread_id: ThreadId,
        event: ChannelIngressEvent,
        input: ChannelInboundText,
    ) -> Result<NativeChannelIngressOutcome, ChannelAdapterError> {
        validate_native_input(thread_id, &event, &input)?;
        let prepare = request_handle
            .prepare_channel_ingress(thread_id, event, input.text)
            .await;
        let outcome = match prepare {
            InProcessChannelIngressPrepareOutcome::Prepared { capability } => {
                request_handle.take_channel_ingress(capability).await.into()
            }
            InProcessChannelIngressPrepareOutcome::Rejected { reason } => {
                NativeChannelIngressOutcome::Rejected { reason }
            }
            InProcessChannelIngressPrepareOutcome::NotEnqueued { reason } => {
                NativeChannelIngressOutcome::NotEnqueued { reason }
            }
            InProcessChannelIngressPrepareOutcome::ResponseLost { phase } => {
                NativeChannelIngressOutcome::ResponseLost { phase }
            }
        };
        Ok(outcome)
    }
}

impl From<InProcessChannelIngressTakeOutcome> for NativeChannelIngressOutcome {
    fn from(outcome: InProcessChannelIngressTakeOutcome) -> Self {
        match outcome {
            InProcessChannelIngressTakeOutcome::ObservedReady {
                event_id,
                preflight_binding_sha256,
            } => Self::ObservedReady {
                event_id,
                preflight_binding_sha256,
            },
            InProcessChannelIngressTakeOutcome::Rejected { reason } => Self::Rejected { reason },
            InProcessChannelIngressTakeOutcome::Unavailable { reason } => {
                Self::Unavailable { reason }
            }
            InProcessChannelIngressTakeOutcome::NotEnqueued { reason } => {
                Self::NotEnqueued { reason }
            }
            InProcessChannelIngressTakeOutcome::ResponseLost { phase } => {
                Self::ResponseLost { phase }
            }
        }
    }
}

fn validate_native_input(
    thread_id: ThreadId,
    event: &ChannelIngressEvent,
    input: &ChannelInboundText,
) -> Result<(), ChannelAdapterError> {
    if event.scope.adapter_id.as_str() != NATIVE_LOOPBACK_ADAPTER_ID {
        return Err(ChannelAdapterError::AdapterMismatch);
    }
    if event.payload_sha256 != input.payload_sha256 {
        return Err(ChannelAdapterError::PayloadDigestMismatch);
    }
    let target = channel_target_thread_sha256(&thread_id.to_string())
        .map_err(|_| ChannelAdapterError::TargetThreadMismatch)?;
    if event.target_thread_sha256 != target {
        return Err(ChannelAdapterError::TargetThreadMismatch);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use codex_hepta_contracts::ChannelAdapterId;
    use codex_hepta_contracts::ChannelScope;

    fn event(thread_id: ThreadId, payload: &str) -> ChannelIngressEvent {
        ChannelIngressEvent::new(
            ChannelScope {
                adapter_id: ChannelAdapterId::new(NATIVE_LOOPBACK_ADAPTER_ID).expect("adapter id"),
                installation_sha256: Sha256Digest::for_bytes(b"installation"),
                account_sha256: Sha256Digest::for_bytes(b"account"),
                conversation_sha256: Sha256Digest::for_bytes(b"conversation"),
                principal_sha256: Sha256Digest::for_bytes(b"principal"),
            },
            Sha256Digest::for_bytes(b"source"),
            Sha256Digest::for_bytes(payload.as_bytes()),
            channel_target_thread_sha256(&thread_id.to_string()).expect("target"),
            None,
            Sha256Digest::for_bytes(b"cursor"),
            1,
        )
        .expect("event")
    }

    #[test]
    fn native_input_requires_exact_adapter_payload_and_thread() {
        let thread_id = ThreadId::new();
        let input = ChannelInboundText::new("payload").expect("input");
        let exact = event(thread_id, "payload");
        validate_native_input(thread_id, &exact, &input).expect("exact input");

        let mut changed = exact.clone();
        changed.scope.adapter_id = ChannelAdapterId::new("telegram.v1").expect("adapter");
        assert_eq!(
            validate_native_input(thread_id, &changed, &input),
            Err(ChannelAdapterError::AdapterMismatch)
        );
        let mut changed = exact;
        changed.payload_sha256 = Sha256Digest::for_bytes(b"other");
        assert_eq!(
            validate_native_input(thread_id, &changed, &input),
            Err(ChannelAdapterError::PayloadDigestMismatch)
        );
        let changed = event(ThreadId::new(), "payload");
        assert_eq!(
            validate_native_input(thread_id, &changed, &input),
            Err(ChannelAdapterError::TargetThreadMismatch)
        );
    }

    #[test]
    fn channel_text_is_bounded_before_host_queues() {
        assert!(matches!(
            ChannelInboundText::new(""),
            Err(ChannelAdapterError::EmptyPayload)
        ));
        assert!(matches!(
            ChannelInboundText::new("x".repeat(MAX_CHANNEL_TEXT_BYTES + 1)),
            Err(ChannelAdapterError::PayloadTooLarge)
        ));
    }
}

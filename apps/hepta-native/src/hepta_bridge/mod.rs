//! Hepta's narrow integration boundary for the Robrix product shell.
//!
//! This module deliberately contains no Matrix client and no Hepta runtime.
//! The default adapter is disabled, so merely linking the product shell cannot
//! send a Matrix event, mutate runtime state, or approve an action. A trusted
//! host may provide an adapter later, behind this contract.

mod adapter;
mod contract;
mod presenter;

pub use adapter::{BridgeAdapterError, BridgeCapabilities};
pub use contract::{
    ActionIntent, ActionKind, ApprovalId, BridgeEntityId, BridgeMetadata, BridgeProblem,
    BridgeReceipt, BridgeRecord, BridgeRequest, BridgeRequestKind, BridgeSnapshot, BridgeUpdate,
    BridgeUpdateKind, ConversationBinding, CorrelationId, Cursor, HEPTA_BRIDGE_SCHEMA_VERSION,
    IdempotencyKey, MirrorPolicy, OpaquePayloadHash, Origin, PreparedActionId, Provenance,
    Redaction, RedactionStatus, Revision, SessionId, TimestampMillis,
};
pub use presenter::{
    BridgePresenter, DEFAULT_PRESENTATION_PAYLOAD_CAP_BYTES, MAX_PRESENTATION_PAYLOAD_CAP_BYTES,
    PresentationDisposition, PresentationFallback, PresentedBridgeUpdate,
};

use adapter::{DisabledBridgeAdapter, GuardedBridgeAdapter};

/// The only production bridge surface currently available to the product UI.
/// It is intentionally disabled and cannot be swapped for a live transport by
/// downstream UI code. A future runtime adapter must land with its own
/// authorization and live-integration gate.
pub struct HeptaBridge {
    adapter: GuardedBridgeAdapter<DisabledBridgeAdapter>,
}

impl Default for HeptaBridge {
    fn default() -> Self {
        Self {
            adapter: GuardedBridgeAdapter::disabled(),
        }
    }
}

impl HeptaBridge {
    pub fn capabilities(&self) -> BridgeCapabilities { self.adapter.capabilities() }

    pub fn submit(
        &mut self,
        request: BridgeRequest,
    ) -> Result<Vec<PresentedBridgeUpdate>, BridgeAdapterError> {
        let presenter = BridgePresenter::default();
        self.adapter
            .handle(request)
            .map(|updates| updates.iter().map(|update| presenter.present(update)).collect())
    }
}

#[cfg(test)]
mod production_tests {
    use super::*;
    use crate::hepta_bridge::contract::{
        BridgeRequestKind,
        tests_support::{binding, request_metadata},
    };

    #[test]
    fn production_bridge_is_forced_disabled() {
        let mut bridge = HeptaBridge::default();
        let request = BridgeRequest {
            metadata: request_metadata("request-1"),
            binding: binding(),
            request: BridgeRequestKind::Snapshot,
        };

        assert_eq!(bridge.capabilities(), BridgeCapabilities::default());
        assert_eq!(
            bridge.submit(request),
            Err(BridgeAdapterError::CapabilityDisabled)
        );
    }
}

//! Hepta's narrow integration boundary for the Robrix product shell.
//!
//! This module deliberately contains no Matrix client and no Hepta runtime.
//! The default adapter is disabled, so merely linking the product shell cannot
//! send a Matrix event, mutate runtime state, or approve an action. A trusted
//! host may provide an authenticated snapshot executor behind this contract.

mod adapter;
mod contract;
mod live_adapter;
mod live_policy;
mod presenter;

pub use adapter::{BridgeAdapterError, BridgeCapabilities};
pub use contract::{
    ActionIntent, ActionKind, ApprovalId, BridgeEntityId, BridgeMetadata, BridgeProblem,
    BridgeReceipt, BridgeRecord, BridgeRequest, BridgeRequestKind, BridgeSnapshot, BridgeUpdate,
    BridgeUpdateKind, ConversationBinding, CorrelationId, Cursor, HEPTA_BRIDGE_SCHEMA_VERSION,
    IdempotencyKey, MirrorPolicy, OpaquePayloadHash, Origin, PreparedActionId, Provenance,
    Redaction, RedactionStatus, Revision, SessionId, TimestampMillis,
};
pub use live_policy::{
    HEPTA_LIVE_BRIDGE_SNAPSHOT_PATH, LiveBridgeActivationContext, LiveBridgeBlocker,
    LiveBridgePreflight,
};
pub use live_adapter::{
    AuthenticatedLiveBridgeBinding, LiveSnapshotGet, LiveSnapshotHttpExecutor,
    LiveSnapshotHttpResponse, MAX_LIVE_SNAPSHOT_RESPONSE_BYTES,
};
pub use presenter::{
    BridgePresenter, DEFAULT_PRESENTATION_PAYLOAD_CAP_BYTES, MAX_PRESENTATION_PAYLOAD_CAP_BYTES,
    PresentationDisposition, PresentationFallback, PresentedBridgeUpdate,
};

use adapter::{BridgeTransport, DisabledBridgeAdapter, GuardedBridgeAdapter};
use live_adapter::LoopbackSnapshotAdapter;

/// Product bridge facade. It is side-effect-free and disabled by default.
///
/// Post-login orchestration may construct the snapshot-only live form only by
/// supplying an authenticated executor whose concrete run/session/sequence
/// binding matches [`LiveBridgeActivationContext`]. No environment variable or
/// Matrix event can enable the bridge implicitly.
pub struct HeptaBridge {
    adapter: GuardedBridgeAdapter<Box<dyn BridgeTransport>>,
}

impl Default for HeptaBridge {
    fn default() -> Self {
        Self {
            adapter: GuardedBridgeAdapter::new(Box::new(DisabledBridgeAdapter)),
        }
    }
}

impl HeptaBridge {
    pub fn try_live<E>(
        context: &LiveBridgeActivationContext<'_>,
        executor: E,
    ) -> Result<Self, BridgeAdapterError>
    where
        E: LiveSnapshotHttpExecutor + 'static,
    {
        let adapter = LoopbackSnapshotAdapter::try_new(context, executor)?;
        Ok(Self {
            adapter: GuardedBridgeAdapter::new(Box::new(adapter)),
        })
    }

    /// Drops the authenticated executor and all captured bridge binding state.
    /// Login failure and logout orchestration must call this before returning
    /// to the login surface.
    pub fn disable(&mut self) {
        self.adapter = GuardedBridgeAdapter::new(Box::new(DisabledBridgeAdapter));
    }

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

    const ENDPOINT: &str = "http://127.0.0.1:47821/api/hepta-native-bridge/v1/snapshot";
    const RUN_IDENTIFIER_SHA256: &str =
        "7777777777777777777777777777777777777777777777777777777777777777";

    struct UnavailableExecutor {
        binding: AuthenticatedLiveBridgeBinding,
    }

    impl LiveSnapshotHttpExecutor for UnavailableExecutor {
        fn authenticated_binding(&self) -> &AuthenticatedLiveBridgeBinding {
            &self.binding
        }

        fn execute_get(
            &mut self,
            _request: &LiveSnapshotGet,
        ) -> Result<LiveSnapshotHttpResponse, BridgeAdapterError> {
            Err(BridgeAdapterError::TransportUnavailable)
        }
    }

    fn live_context() -> LiveBridgeActivationContext<'static> {
        LiveBridgeActivationContext {
            matrix_session_authenticated: true,
            explicit_user_opt_in: true,
            endpoint: ENDPOINT,
            authenticated_session_id: "session-7".into(),
            run_identifier_sha256: RUN_IDENTIFIER_SHA256,
            initial_sequence: 3,
            authoritative_snapshot_contract: true,
        }
    }

    #[test]
    fn production_bridge_is_disabled_by_default() {
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

    #[test]
    fn explicit_live_construction_is_snapshot_only_and_disable_drops_it() {
        let binding =
            AuthenticatedLiveBridgeBinding::try_new("session-7".into(), RUN_IDENTIFIER_SHA256, 3)
                .unwrap();
        let mut bridge =
            HeptaBridge::try_live(&live_context(), UnavailableExecutor { binding }).unwrap();

        assert_eq!(
            bridge.capabilities(),
            BridgeCapabilities {
                snapshot: true,
                ..BridgeCapabilities::default()
            }
        );
        bridge.disable();
        assert_eq!(bridge.capabilities(), BridgeCapabilities::default());
    }
}

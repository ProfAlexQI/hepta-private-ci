//! Hepta's narrow integration boundary for the Robrix product shell.
//!
//! This module deliberately contains no Matrix client and no Hepta runtime.
//! The default adapter is disabled, so merely linking the product shell cannot
//! send a Matrix event, mutate runtime state, or approve an action. A trusted
//! host may provide an authenticated snapshot executor behind this contract.

mod adapter;
mod backend_activation;
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
pub(crate) use backend_activation::BackendAuthenticatedBridgeActivation;
use live_adapter::LoopbackSnapshotAdapter;

/// App lifecycle transitions that must invalidate every authenticated Hepta
/// transport and its captured run/session/sequence binding.
///
/// Matrix login success is included deliberately: the current `LoginAction`
/// carries no authoritative Hepta backend handshake. It may reveal the Matrix
/// product shell, but it cannot preserve or activate a Hepta transport.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum HeptaBridgeLifecycleEvent {
    MatrixLoginSuccessWithoutBackendBinding,
    LoginFailure,
    LogoutSuccess,
    ClearAppState,
}

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
    /// Low-level constructor available only to bridge unit tests. Production
    /// App activation must consume the sealed backend activation type below.
    #[cfg(test)]
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

    /// Replaces the disabled facade only from explicit backend-owned
    /// activation material. The binding comes exclusively from the injected
    /// authenticated executor; Matrix login state is not accepted as a Hepta
    /// session binding.
    ///
    /// Any failed activation leaves the facade disabled and drops the supplied
    /// executor. This prevents a stale authenticated transport from surviving
    /// a failed rebind attempt.
    pub(crate) fn activate_from_authenticated_backend<E>(
        &mut self,
        activation: BackendAuthenticatedBridgeActivation<E>,
    ) -> Result<(), BridgeAdapterError>
    where
        E: LiveSnapshotHttpExecutor + 'static,
    {
        self.disable();

        let (
            matrix_session_authenticated,
            _matrix_user_id,
            endpoint,
            explicit_user_opt_in,
            authoritative_snapshot_contract,
            executor,
        ) = activation.into_parts();
        let binding = executor.authenticated_binding().clone();
        let context = LiveBridgeActivationContext {
            matrix_session_authenticated,
            explicit_user_opt_in,
            endpoint: &endpoint,
            authenticated_session_id: binding.session_id().clone(),
            run_identifier_sha256: binding.run_identifier_sha256(),
            initial_sequence: binding.initial_sequence(),
            authoritative_snapshot_contract,
        };
        let adapter = LoopbackSnapshotAdapter::try_new(&context, executor)?;
        self.adapter = GuardedBridgeAdapter::new(Box::new(adapter));
        Ok(())
    }

    /// Applies an App lifecycle transition that invalidates the current
    /// transport. All variants intentionally share the same fail-closed drop
    /// behavior.
    pub(crate) fn handle_app_lifecycle_event(&mut self, _event: HeptaBridgeLifecycleEvent) {
        self.disable();
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
    use std::sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
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

    struct DropProbeExecutor {
        binding: AuthenticatedLiveBridgeBinding,
        drops: Arc<AtomicUsize>,
    }

    impl Drop for DropProbeExecutor {
        fn drop(&mut self) {
            self.drops.fetch_add(1, Ordering::SeqCst);
        }
    }

    impl LiveSnapshotHttpExecutor for DropProbeExecutor {
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

    #[test]
    fn app_lifecycle_transitions_drop_executor_and_binding() {
        for event in [
            HeptaBridgeLifecycleEvent::LoginFailure,
            HeptaBridgeLifecycleEvent::LogoutSuccess,
            HeptaBridgeLifecycleEvent::ClearAppState,
        ] {
            let drops = Arc::new(AtomicUsize::new(0));
            let binding = AuthenticatedLiveBridgeBinding::try_new(
                "session-7".into(),
                RUN_IDENTIFIER_SHA256,
                3,
            )
            .unwrap();
            let activation = BackendAuthenticatedBridgeActivation::for_test(
                "@alex:example.test",
                ENDPOINT,
                true,
                true,
                DropProbeExecutor {
                    binding,
                    drops: Arc::clone(&drops),
                },
            );
            let mut bridge = HeptaBridge::default();
            bridge
                .activate_from_authenticated_backend(activation)
                .unwrap();

            bridge.handle_app_lifecycle_event(event);

            assert_eq!(bridge.capabilities(), BridgeCapabilities::default());
            assert_eq!(drops.load(Ordering::SeqCst), 1);
        }
    }

    #[test]
    fn matrix_login_success_without_backend_binding_cannot_activate_bridge() {
        let mut bridge = HeptaBridge::default();

        bridge.handle_app_lifecycle_event(
            HeptaBridgeLifecycleEvent::MatrixLoginSuccessWithoutBackendBinding,
        );

        assert_eq!(bridge.capabilities(), BridgeCapabilities::default());
    }

    #[test]
    fn rejected_backend_activation_drops_executor_and_leaves_bridge_disabled() {
        let drops = Arc::new(AtomicUsize::new(0));
        let binding = AuthenticatedLiveBridgeBinding::try_new(
            "session-7".into(),
            RUN_IDENTIFIER_SHA256,
            3,
        )
        .unwrap();
        let activation = BackendAuthenticatedBridgeActivation::for_test(
            "@alex:example.test",
            "https://example.invalid/api/hepta-native-bridge/v1/snapshot",
            true,
            true,
            DropProbeExecutor {
                binding,
                drops: Arc::clone(&drops),
            },
        );
        let mut bridge = HeptaBridge::default();

        assert_eq!(
            bridge.activate_from_authenticated_backend(activation),
            Err(BridgeAdapterError::LivePreflightBlocked)
        );
        assert_eq!(bridge.capabilities(), BridgeCapabilities::default());
        assert_eq!(drops.load(Ordering::SeqCst), 1);
    }
}

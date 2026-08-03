use url::Url;

use super::{
    adapter::{BridgeAdapterError, BridgeCapabilities, BridgeTransport, GuardedBridgeAdapter},
    contract::{
        BridgeRequest, BridgeRequestKind, BridgeUpdate, BridgeUpdateKind, CorrelationId, SessionId,
    },
    live_policy::{LiveBridgeActivationContext, LiveBridgePreflight},
};

/// Maximum authoritative snapshot envelope accepted by the Native UI.
///
/// The future host transport may impose a lower streaming limit, but it must
/// never buffer more than this UI boundary before deserializing.
pub const MAX_LIVE_SNAPSHOT_RESPONSE_BYTES: usize = 1024 * 1024;

const JSON_CONTENT_TYPE: &str = "application/json";
const NO_STORE: &str = "no-store";

/// Complete, body-free descriptor for the only HTTP operation the live seam
/// can request.
///
/// There is deliberately no caller-controlled method or request body. A host
/// implementation receives this type only through `execute_get`, which keeps
/// the UI side of the contract read-only by construction.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct LiveSnapshotGet {
    endpoint: Url,
    session_id: SessionId,
    correlation_id: CorrelationId,
}

impl LiveSnapshotGet {
    pub(crate) fn endpoint(&self) -> &Url {
        &self.endpoint
    }

    pub(crate) fn session_id(&self) -> &SessionId {
        &self.session_id
    }

    pub(crate) fn correlation_id(&self) -> &CorrelationId {
        &self.correlation_id
    }

    pub(crate) fn accept(&self) -> &'static str {
        JSON_CONTENT_TYPE
    }

    pub(crate) fn cache_control(&self) -> &'static str {
        NO_STORE
    }
}

/// Minimal response shape returned by a future authenticated loopback host.
///
/// It is not a live receipt. The adapter validates and discards these HTTP
/// details after extracting a contract-safe `BridgeUpdate`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct LiveSnapshotHttpResponse {
    /// Final URL after transport processing. It must remain equal to the
    /// validated loopback endpoint; redirects are not accepted.
    pub final_endpoint: Url,
    pub status: u16,
    pub content_type: String,
    pub cache_control: String,
    pub body: Vec<u8>,
}

/// Authenticated host boundary for the snapshot-only adapter.
///
/// No production implementation exists in this crate. The eventual backend
/// lane must implement this trait only after an authenticated handshake and
/// return the exact bound session identifier. Tests use an in-memory executor
/// and never open a socket.
pub(crate) trait LiveSnapshotHttpExecutor: Send {
    fn authenticated_session_id(&self) -> &SessionId;

    fn execute_get(
        &mut self,
        request: &LiveSnapshotGet,
    ) -> Result<LiveSnapshotHttpResponse, BridgeAdapterError>;
}

/// Snapshot-only transport seam. Construction is possible only after the
/// side-effect-free live preflight passes and the injected host asserts a
/// concrete, non-empty authenticated session binding.
pub(crate) struct LoopbackSnapshotAdapter<E: LiveSnapshotHttpExecutor> {
    endpoint: Url,
    executor: E,
}

impl<E: LiveSnapshotHttpExecutor> LoopbackSnapshotAdapter<E> {
    pub(crate) fn try_new(
        context: &LiveBridgeActivationContext<'_>,
        executor: E,
    ) -> Result<Self, BridgeAdapterError> {
        let preflight = LiveBridgePreflight::evaluate(context);
        if !preflight.eligible_for_adapter_construction {
            return Err(BridgeAdapterError::LivePreflightBlocked);
        }
        if executor.authenticated_session_id().is_blank() {
            return Err(BridgeAdapterError::InvalidRequest(
                "authenticated bridge session id is missing",
            ));
        }

        let endpoint = Url::parse(context.endpoint).map_err(|_| {
            BridgeAdapterError::InvalidRequest("preflight endpoint could not be parsed")
        })?;
        Ok(Self { endpoint, executor })
    }

    pub(crate) fn into_guarded(self) -> GuardedBridgeAdapter<Self> {
        GuardedBridgeAdapter::new(self)
    }

    fn validate_response(
        response: LiveSnapshotHttpResponse,
        expected_endpoint: &Url,
        expected_session: &SessionId,
        expected_correlation: &CorrelationId,
    ) -> Result<BridgeUpdate, BridgeAdapterError> {
        if response.final_endpoint != *expected_endpoint {
            return Err(BridgeAdapterError::InvalidSnapshotResponse(
                "redirects or endpoint changes are not allowed",
            ));
        }
        if response.status != 200 {
            return Err(BridgeAdapterError::InvalidSnapshotResponse(
                "HTTP status must be 200",
            ));
        }
        if response.content_type.trim() != JSON_CONTENT_TYPE {
            return Err(BridgeAdapterError::InvalidSnapshotResponse(
                "Content-Type must be application/json",
            ));
        }
        if response.cache_control.trim() != NO_STORE {
            return Err(BridgeAdapterError::InvalidSnapshotResponse(
                "Cache-Control must be no-store",
            ));
        }
        if response.body.is_empty() || response.body.len() > MAX_LIVE_SNAPSHOT_RESPONSE_BYTES {
            return Err(BridgeAdapterError::InvalidSnapshotResponse(
                "response body size is outside the accepted range",
            ));
        }

        let update: BridgeUpdate = serde_json::from_slice(&response.body).map_err(|_| {
            BridgeAdapterError::InvalidSnapshotResponse(
                "body is not the exact BridgeUpdate JSON contract",
            )
        })?;
        if !matches!(update.update, BridgeUpdateKind::Snapshot { .. }) {
            return Err(BridgeAdapterError::InvalidSnapshotResponse(
                "response update must be a snapshot",
            ));
        }
        if &update.metadata.session_id != expected_session
            || &update.metadata.correlation_id != expected_correlation
        {
            return Err(BridgeAdapterError::InvalidSnapshotResponse(
                "response session or correlation binding does not match the request",
            ));
        }
        Ok(update)
    }
}

impl<E: LiveSnapshotHttpExecutor> BridgeTransport for LoopbackSnapshotAdapter<E> {
    fn capabilities(&self) -> BridgeCapabilities {
        BridgeCapabilities {
            snapshot: true,
            ..BridgeCapabilities::default()
        }
    }

    fn handle(&mut self, request: BridgeRequest) -> Result<Vec<BridgeUpdate>, BridgeAdapterError> {
        if !request.is_contract_valid() {
            return Err(BridgeAdapterError::InvalidRequest(
                "snapshot request does not satisfy the bridge contract",
            ));
        }
        if !matches!(request.request, BridgeRequestKind::Snapshot) {
            return Err(BridgeAdapterError::CapabilityDisabled);
        }
        if &request.metadata.session_id != self.executor.authenticated_session_id() {
            return Err(BridgeAdapterError::InvalidRequest(
                "request session does not match the authenticated bridge session",
            ));
        }

        let descriptor = LiveSnapshotGet {
            endpoint: self.endpoint.clone(),
            session_id: request.metadata.session_id.clone(),
            correlation_id: request.metadata.correlation_id.clone(),
        };
        let response = self.executor.execute_get(&descriptor)?;
        if self.executor.authenticated_session_id() != &descriptor.session_id {
            return Err(BridgeAdapterError::InvalidSnapshotResponse(
                "authenticated bridge session changed during the request",
            ));
        }
        let update = Self::validate_response(
            response,
            &descriptor.endpoint,
            &descriptor.session_id,
            &descriptor.correlation_id,
        )?;
        Ok(vec![update])
    }
}

/// Internal construction seam for a future authenticated post-login host.
/// Product `HeptaBridge::default()` does not call or expose this function.
pub(crate) fn guarded_loopback_snapshot_adapter<E: LiveSnapshotHttpExecutor>(
    context: &LiveBridgeActivationContext<'_>,
    executor: E,
) -> Result<GuardedBridgeAdapter<LoopbackSnapshotAdapter<E>>, BridgeAdapterError> {
    LoopbackSnapshotAdapter::try_new(context, executor).map(|adapter| adapter.into_guarded())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hepta_bridge::contract::{
        BridgeSnapshot, BridgeUpdateKind, Redaction,
        tests_support::{binding, metadata, request_metadata},
    };

    const ENDPOINT: &str = "http://127.0.0.1:47821/api/hepta-native-bridge/v1/snapshot";

    #[derive(Debug)]
    struct MemoryExecutor {
        authenticated_session_id: SessionId,
        response: Option<Result<LiveSnapshotHttpResponse, BridgeAdapterError>>,
        observed: Vec<LiveSnapshotGet>,
        rotate_session_after_request: Option<SessionId>,
    }

    impl MemoryExecutor {
        fn returning(update: BridgeUpdate) -> Self {
            Self {
                authenticated_session_id: "session-7".into(),
                response: Some(Ok(LiveSnapshotHttpResponse {
                    final_endpoint: Url::parse(ENDPOINT).unwrap(),
                    status: 200,
                    content_type: JSON_CONTENT_TYPE.into(),
                    cache_control: NO_STORE.into(),
                    body: serde_json::to_vec(&update).unwrap(),
                })),
                observed: Vec::new(),
                rotate_session_after_request: None,
            }
        }
    }

    impl LiveSnapshotHttpExecutor for MemoryExecutor {
        fn authenticated_session_id(&self) -> &SessionId {
            &self.authenticated_session_id
        }

        fn execute_get(
            &mut self,
            request: &LiveSnapshotGet,
        ) -> Result<LiveSnapshotHttpResponse, BridgeAdapterError> {
            self.observed.push(request.clone());
            let response = self
                .response
                .take()
                .unwrap_or(Err(BridgeAdapterError::TransportUnavailable));
            if let Some(session_id) = self.rotate_session_after_request.take() {
                self.authenticated_session_id = session_id;
            }
            response
        }
    }

    fn context() -> LiveBridgeActivationContext<'static> {
        LiveBridgeActivationContext {
            matrix_session_authenticated: true,
            explicit_user_opt_in: true,
            endpoint: ENDPOINT,
            authenticated_session_binding: true,
            authoritative_snapshot_contract: true,
        }
    }

    fn request() -> BridgeRequest {
        BridgeRequest {
            metadata: request_metadata("request-1"),
            binding: binding(),
            request: BridgeRequestKind::Snapshot,
        }
    }

    fn snapshot_update() -> BridgeUpdate {
        BridgeUpdate {
            metadata: metadata("snapshot-1", Redaction::not_required()),
            binding: binding(),
            update: BridgeUpdateKind::Snapshot {
                snapshot: BridgeSnapshot::default(),
            },
        }
    }

    #[test]
    fn construction_requires_every_preflight_condition() {
        let mut blocked = context();
        blocked.explicit_user_opt_in = false;

        let result = LoopbackSnapshotAdapter::try_new(
            &blocked,
            MemoryExecutor::returning(snapshot_update()),
        );

        assert!(matches!(
            result,
            Err(BridgeAdapterError::LivePreflightBlocked)
        ));
    }

    #[test]
    fn snapshot_request_is_get_only_and_bound_to_session_and_correlation() {
        let mut adapter = LoopbackSnapshotAdapter::try_new(
            &context(),
            MemoryExecutor::returning(snapshot_update()),
        )
        .unwrap();

        let updates = adapter.handle(request()).unwrap();

        assert_eq!(updates, vec![snapshot_update()]);
        assert_eq!(adapter.executor.observed.len(), 1);
        let observed = &adapter.executor.observed[0];
        assert_eq!(observed.endpoint().as_str(), ENDPOINT);
        assert_eq!(observed.session_id().as_str(), "session-7");
        assert_eq!(observed.correlation_id().as_str(), "correlation-11");
        assert_eq!(observed.accept(), "application/json");
        assert_eq!(observed.cache_control(), "no-store");
    }

    #[test]
    fn authenticated_session_mismatch_fails_before_transport() {
        let mut executor = MemoryExecutor::returning(snapshot_update());
        executor.authenticated_session_id = "another-session".into();
        let mut adapter = LoopbackSnapshotAdapter::try_new(&context(), executor).unwrap();

        assert_eq!(
            adapter.handle(request()),
            Err(BridgeAdapterError::InvalidRequest(
                "request session does not match the authenticated bridge session"
            ))
        );
        assert!(adapter.executor.observed.is_empty());
    }

    #[test]
    fn authenticated_session_rotation_during_transport_fails_closed() {
        let mut executor = MemoryExecutor::returning(snapshot_update());
        executor.rotate_session_after_request = Some("rotated-session".into());
        let mut adapter = LoopbackSnapshotAdapter::try_new(&context(), executor).unwrap();

        assert_eq!(
            adapter.handle(request()),
            Err(BridgeAdapterError::InvalidSnapshotResponse(
                "authenticated bridge session changed during the request"
            ))
        );
        assert_eq!(adapter.executor.observed.len(), 1);
    }

    #[test]
    fn response_must_be_a_no_store_json_snapshot() {
        let update = snapshot_update();
        let invalid_responses = [
            LiveSnapshotHttpResponse {
                final_endpoint: Url::parse(
                    "http://127.0.0.1:47821/api/hepta-native-bridge/v1/redirected",
                )
                .unwrap(),
                status: 200,
                content_type: JSON_CONTENT_TYPE.into(),
                cache_control: NO_STORE.into(),
                body: serde_json::to_vec(&update).unwrap(),
            },
            LiveSnapshotHttpResponse {
                final_endpoint: Url::parse(ENDPOINT).unwrap(),
                status: 204,
                content_type: JSON_CONTENT_TYPE.into(),
                cache_control: NO_STORE.into(),
                body: serde_json::to_vec(&update).unwrap(),
            },
            LiveSnapshotHttpResponse {
                final_endpoint: Url::parse(ENDPOINT).unwrap(),
                status: 200,
                content_type: "text/plain".into(),
                cache_control: NO_STORE.into(),
                body: serde_json::to_vec(&update).unwrap(),
            },
            LiveSnapshotHttpResponse {
                final_endpoint: Url::parse(ENDPOINT).unwrap(),
                status: 200,
                content_type: JSON_CONTENT_TYPE.into(),
                cache_control: "private".into(),
                body: serde_json::to_vec(&update).unwrap(),
            },
        ];

        for response in invalid_responses {
            let mut executor = MemoryExecutor::returning(snapshot_update());
            executor.response = Some(Ok(response));
            let mut adapter = LoopbackSnapshotAdapter::try_new(&context(), executor).unwrap();
            assert!(matches!(
                adapter.handle(request()),
                Err(BridgeAdapterError::InvalidSnapshotResponse(_))
            ));
        }
    }

    #[test]
    fn non_snapshot_and_cross_correlation_responses_fail_closed() {
        let mut non_snapshot = snapshot_update();
        non_snapshot.update = BridgeUpdateKind::Error {
            problem: crate::hepta_bridge::BridgeProblem {
                code: "not-a-snapshot".into(),
                user_safe_message: "Unavailable".into(),
                retryable: false,
            },
        };
        let mut wrong_correlation = snapshot_update();
        wrong_correlation.metadata.correlation_id = "other-correlation".into();

        for update in [non_snapshot, wrong_correlation] {
            let mut adapter =
                LoopbackSnapshotAdapter::try_new(&context(), MemoryExecutor::returning(update))
                    .unwrap();
            assert!(matches!(
                adapter.handle(request()),
                Err(BridgeAdapterError::InvalidSnapshotResponse(_))
            ));
        }
    }

    #[test]
    fn guarded_adapter_exposes_snapshot_capability_only() {
        let adapter = guarded_loopback_snapshot_adapter(
            &context(),
            MemoryExecutor::returning(snapshot_update()),
        )
        .unwrap();
        let capabilities = adapter.capabilities();

        assert!(capabilities.snapshot);
        assert!(!capabilities.subscribe);
        assert!(!capabilities.prepare);
        assert!(!capabilities.confirm);
        assert!(!capabilities.reject);
        assert!(!capabilities.cancel);
    }
}

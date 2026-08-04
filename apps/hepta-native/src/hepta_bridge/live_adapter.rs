use url::Url;

use super::{
    adapter::{BridgeAdapterError, BridgeCapabilities, BridgeTransport},
    contract::{
        BridgeRequest, BridgeRequestKind, BridgeUpdate, BridgeUpdateKind, CorrelationId, Revision,
        SessionId,
    },
    live_policy::{LiveBridgeActivationContext, LiveBridgePreflight, is_sha256},
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
pub struct LiveSnapshotGet {
    endpoint: Url,
    session_id: SessionId,
    correlation_id: CorrelationId,
    run_identifier_sha256: String,
    expected_sequence: u64,
}

impl LiveSnapshotGet {
    pub(super) fn try_new(
        endpoint: Url,
        session_id: SessionId,
        correlation_id: CorrelationId,
        run_identifier_sha256: impl Into<String>,
        expected_sequence: u64,
    ) -> Result<Self, BridgeAdapterError> {
        let run_identifier_sha256 = run_identifier_sha256.into();
        if !session_id.is_live_transport_safe()
            || !correlation_id.is_live_transport_safe()
            || !is_sha256(&run_identifier_sha256)
        {
            return Err(BridgeAdapterError::InvalidRequest(
                "live snapshot request binding is not bounded and HTTP-header safe",
            ));
        }
        Ok(Self {
            endpoint,
            session_id,
            correlation_id,
            run_identifier_sha256,
            expected_sequence,
        })
    }

    pub fn endpoint(&self) -> &Url {
        &self.endpoint
    }

    pub fn session_id(&self) -> &SessionId {
        &self.session_id
    }

    pub fn correlation_id(&self) -> &CorrelationId {
        &self.correlation_id
    }

    pub fn run_identifier_sha256(&self) -> &str {
        &self.run_identifier_sha256
    }

    pub fn expected_sequence(&self) -> u64 {
        self.expected_sequence
    }

    pub fn accept(&self) -> &'static str {
        JSON_CONTENT_TYPE
    }

    pub fn cache_control(&self) -> &'static str {
        NO_STORE
    }
}

/// Concrete authenticated binding negotiated by the trusted host.
///
/// The binding contains no credential material. Authentication remains the
/// executor's responsibility; Native captures these values at construction
/// and rejects any session, run, or sequence change before presenting data.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AuthenticatedLiveBridgeBinding {
    session_id: SessionId,
    run_identifier_sha256: String,
    initial_sequence: u64,
}

impl AuthenticatedLiveBridgeBinding {
    pub fn try_new(
        session_id: SessionId,
        run_identifier_sha256: impl Into<String>,
        initial_sequence: u64,
    ) -> Result<Self, BridgeAdapterError> {
        let run_identifier_sha256 = run_identifier_sha256.into();
        if !session_id.is_live_transport_safe() {
            return Err(BridgeAdapterError::InvalidRequest(
                "authenticated bridge session id is missing, oversized, or not HTTP-header safe",
            ));
        }
        if !is_sha256(&run_identifier_sha256) {
            return Err(BridgeAdapterError::InvalidRequest(
                "authenticated bridge run identifier is invalid",
            ));
        }
        if initial_sequence == 0 {
            return Err(BridgeAdapterError::InvalidRequest(
                "authenticated bridge initial sequence is invalid",
            ));
        }
        Ok(Self {
            session_id,
            run_identifier_sha256,
            initial_sequence,
        })
    }

    pub fn session_id(&self) -> &SessionId {
        &self.session_id
    }

    pub fn run_identifier_sha256(&self) -> &str {
        &self.run_identifier_sha256
    }

    pub fn initial_sequence(&self) -> u64 {
        self.initial_sequence
    }
}

/// Minimal response shape returned by a future authenticated loopback host.
///
/// It is not a live receipt. The adapter validates and discards these HTTP
/// details after extracting a contract-safe `BridgeUpdate`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LiveSnapshotHttpResponse {
    /// Final URL after transport processing. It must remain equal to the
    /// validated loopback endpoint; redirects are not accepted.
    pub final_endpoint: Url,
    pub status: u16,
    pub content_type: String,
    pub cache_control: String,
    pub authenticated_session_id: SessionId,
    pub authenticated_correlation_id: CorrelationId,
    pub run_identifier_sha256: String,
    pub sequence: u64,
    pub body: Vec<u8>,
}

/// Authenticated host boundary for the snapshot-only adapter.
///
/// The backend-owned implementation must authenticate before construction,
/// perform exactly the described GET, bound response buffering, reject
/// redirects, and return response binding metadata without logging secrets.
/// The UI crate deliberately does not derive this executor from environment
/// variables or create a credential itself.
pub trait LiveSnapshotHttpExecutor: Send {
    fn authenticated_binding(&self) -> &AuthenticatedLiveBridgeBinding;

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
    binding: AuthenticatedLiveBridgeBinding,
    next_sequence: u64,
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
        let binding = executor.authenticated_binding().clone();
        if binding.session_id() != &context.authenticated_session_id
            || binding.run_identifier_sha256() != context.run_identifier_sha256
            || binding.initial_sequence() != context.initial_sequence
        {
            return Err(BridgeAdapterError::InvalidRequest(
                "authenticated bridge binding does not match the in-process activation context",
            ));
        }

        let endpoint = Url::parse(context.endpoint).map_err(|_| {
            BridgeAdapterError::InvalidRequest("preflight endpoint could not be parsed")
        })?;
        let next_sequence = binding.initial_sequence();
        Ok(Self {
            endpoint,
            binding,
            next_sequence,
            executor,
        })
    }

    fn validate_response(
        response: LiveSnapshotHttpResponse,
        expected_endpoint: &Url,
        expected_binding: &AuthenticatedLiveBridgeBinding,
        expected_correlation: &CorrelationId,
        expected_sequence: u64,
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
        if response.authenticated_session_id != *expected_binding.session_id()
            || response.authenticated_correlation_id != *expected_correlation
            || response.run_identifier_sha256 != expected_binding.run_identifier_sha256()
            || response.sequence != expected_sequence
        {
            return Err(BridgeAdapterError::InvalidSnapshotResponse(
                "response run, session, correlation, or sequence binding does not match the request",
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
        if &update.metadata.session_id != expected_binding.session_id()
            || &update.metadata.correlation_id != expected_correlation
            || update.metadata.revision != Revision(expected_sequence)
            || update.binding.revision != Revision(expected_sequence)
        {
            return Err(BridgeAdapterError::InvalidSnapshotResponse(
                "response envelope revision, session, or correlation binding does not match the request",
            ));
        }
        let BridgeUpdateKind::Snapshot { snapshot } = &update.update else {
            unreachable!("snapshot update checked above");
        };
        if snapshot.revision != Revision(expected_sequence) {
            return Err(BridgeAdapterError::InvalidSnapshotResponse(
                "snapshot revision does not match the response sequence",
            ));
        }
        if !update.is_contract_valid() || !update.is_presenter_safe() {
            return Err(BridgeAdapterError::InvalidSnapshotResponse(
                "response envelope is invalid or not presenter safe",
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
        if &request.metadata.session_id != self.binding.session_id() {
            return Err(BridgeAdapterError::InvalidRequest(
                "request session does not match the authenticated bridge session",
            ));
        }

        let descriptor = LiveSnapshotGet::try_new(
            self.endpoint.clone(),
            request.metadata.session_id.clone(),
            request.metadata.correlation_id.clone(),
            self.binding.run_identifier_sha256(),
            self.next_sequence,
        )?;
        let response = self.executor.execute_get(&descriptor)?;
        if self.executor.authenticated_binding() != &self.binding {
            return Err(BridgeAdapterError::InvalidSnapshotResponse(
                "authenticated bridge run or session changed during the request",
            ));
        }
        let update = Self::validate_response(
            response,
            &descriptor.endpoint,
            &self.binding,
            &descriptor.correlation_id,
            descriptor.expected_sequence,
        )?;
        self.next_sequence = self.next_sequence.checked_add(1).ok_or(
            BridgeAdapterError::InvalidSnapshotResponse("bridge sequence overflowed"),
        )?;
        Ok(vec![update])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hepta_bridge::contract::{
        BridgeSnapshot, BridgeUpdateKind, Redaction,
        MAX_BRIDGE_CORRELATION_ID_BYTES,
        tests_support::{binding, metadata, request_metadata},
    };

    const ENDPOINT: &str = "http://127.0.0.1:47821/api/hepta-native-bridge/v1/snapshot";
    const RUN_IDENTIFIER_SHA256: &str =
        "7777777777777777777777777777777777777777777777777777777777777777";

    #[derive(Debug)]
    struct MemoryExecutor {
        binding: AuthenticatedLiveBridgeBinding,
        response: Option<Result<LiveSnapshotHttpResponse, BridgeAdapterError>>,
        observed: Vec<LiveSnapshotGet>,
        rotate_binding_after_request: Option<AuthenticatedLiveBridgeBinding>,
    }

    impl MemoryExecutor {
        fn returning(update: BridgeUpdate) -> Self {
            let binding = AuthenticatedLiveBridgeBinding::try_new(
                "session-7".into(),
                RUN_IDENTIFIER_SHA256,
                3,
            )
            .unwrap();
            Self {
                response: Some(Ok(Self::http_response(&binding, update))),
                binding,
                observed: Vec::new(),
                rotate_binding_after_request: None,
            }
        }

        fn http_response(
            binding: &AuthenticatedLiveBridgeBinding,
            update: BridgeUpdate,
        ) -> LiveSnapshotHttpResponse {
            LiveSnapshotHttpResponse {
                final_endpoint: Url::parse(ENDPOINT).unwrap(),
                status: 200,
                content_type: JSON_CONTENT_TYPE.into(),
                cache_control: NO_STORE.into(),
                authenticated_session_id: binding.session_id().clone(),
                authenticated_correlation_id: update.metadata.correlation_id.clone(),
                run_identifier_sha256: binding.run_identifier_sha256().into(),
                sequence: binding.initial_sequence(),
                body: serde_json::to_vec(&update).unwrap(),
            }
        }
    }

    impl LiveSnapshotHttpExecutor for MemoryExecutor {
        fn authenticated_binding(&self) -> &AuthenticatedLiveBridgeBinding {
            &self.binding
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
            if let Some(binding) = self.rotate_binding_after_request.take() {
                self.binding = binding;
            }
            response
        }
    }

    fn context() -> LiveBridgeActivationContext<'static> {
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
                snapshot: BridgeSnapshot {
                    revision: Revision(3),
                    ..BridgeSnapshot::default()
                },
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
        assert_eq!(observed.run_identifier_sha256(), RUN_IDENTIFIER_SHA256);
        assert_eq!(observed.expected_sequence(), 3);
        assert_eq!(observed.accept(), "application/json");
        assert_eq!(observed.cache_control(), "no-store");
    }

    #[test]
    fn oversized_binding_never_reaches_a_custom_executor() {
        let mut adapter = LoopbackSnapshotAdapter::try_new(
            &context(),
            MemoryExecutor::returning(snapshot_update()),
        )
        .unwrap();
        let mut oversized = request();
        oversized.metadata.correlation_id =
            CorrelationId::from("c".repeat(MAX_BRIDGE_CORRELATION_ID_BYTES + 1));

        assert_eq!(
            adapter.handle(oversized),
            Err(BridgeAdapterError::InvalidRequest(
                "snapshot request does not satisfy the bridge contract"
            ))
        );
        assert!(adapter.executor.observed.is_empty());
    }

    #[test]
    fn authenticated_session_mismatch_fails_during_construction() {
        let mut executor = MemoryExecutor::returning(snapshot_update());
        executor.binding = AuthenticatedLiveBridgeBinding::try_new(
            "another-session".into(),
            RUN_IDENTIFIER_SHA256,
            3,
        )
        .unwrap();

        assert!(matches!(
            LoopbackSnapshotAdapter::try_new(&context(), executor),
            Err(BridgeAdapterError::InvalidRequest(
                "authenticated bridge binding does not match the in-process activation context"
            ))
        ));
    }

    #[test]
    fn authenticated_session_rotation_during_transport_fails_closed() {
        let mut executor = MemoryExecutor::returning(snapshot_update());
        executor.rotate_binding_after_request = Some(
            AuthenticatedLiveBridgeBinding::try_new(
                "rotated-session".into(),
                RUN_IDENTIFIER_SHA256,
                3,
            )
            .unwrap(),
        );
        let mut adapter = LoopbackSnapshotAdapter::try_new(&context(), executor).unwrap();

        assert_eq!(
            adapter.handle(request()),
            Err(BridgeAdapterError::InvalidSnapshotResponse(
                "authenticated bridge run or session changed during the request"
            ))
        );
        assert_eq!(adapter.executor.observed.len(), 1);
    }

    #[test]
    fn response_must_be_a_no_store_json_snapshot() {
        let update = snapshot_update();
        let binding = MemoryExecutor::returning(update.clone()).binding;
        let mut redirected = MemoryExecutor::http_response(&binding, update.clone());
        redirected.final_endpoint =
            Url::parse("http://127.0.0.1:47821/api/hepta-native-bridge/v1/redirected").unwrap();
        let mut wrong_status = MemoryExecutor::http_response(&binding, update.clone());
        wrong_status.status = 204;
        let mut wrong_content_type = MemoryExecutor::http_response(&binding, update.clone());
        wrong_content_type.content_type = "text/plain".into();
        let mut cacheable = MemoryExecutor::http_response(&binding, update);
        cacheable.cache_control = "private".into();
        let invalid_responses = [redirected, wrong_status, wrong_content_type, cacheable];

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
    fn stale_gap_and_cross_run_sequences_fail_closed() {
        let update = snapshot_update();
        for mutation in ["stale", "gap", "cross-run"] {
            let mut executor = MemoryExecutor::returning(update.clone());
            let response = executor.response.as_mut().unwrap().as_mut().unwrap();
            match mutation {
                "stale" => response.sequence = 2,
                "gap" => response.sequence = 4,
                "cross-run" => response.run_identifier_sha256 = "8".repeat(64),
                _ => unreachable!(),
            }
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

        let mut executor = MemoryExecutor::returning(snapshot_update());
        executor
            .response
            .as_mut()
            .unwrap()
            .as_mut()
            .unwrap()
            .authenticated_correlation_id = "other-correlation".into();
        let mut adapter = LoopbackSnapshotAdapter::try_new(&context(), executor).unwrap();
        assert!(matches!(
            adapter.handle(request()),
            Err(BridgeAdapterError::InvalidSnapshotResponse(_))
        ));
    }
}

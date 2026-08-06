use crate::auth::SharedAuthProvider;
use crate::common::CompactionInput;
use crate::endpoint::session::EndpointSession;
use crate::error::ApiError;
use crate::provider::Provider;
use codex_client::HttpTransport;
use codex_client::RequestTelemetry;
use codex_protocol::models::ResponseItem;
use http::HeaderMap;
use http::Method;
use serde::Deserialize;
use std::sync::Arc;
use std::sync::OnceLock;
use std::time::Duration;

const X_CODEX_TURN_STATE_HEADER: &str = "x-codex-turn-state";

pub struct CompactClient<T: HttpTransport> {
    session: EndpointSession<T>,
}

impl<T: HttpTransport> CompactClient<T> {
    pub fn new(transport: T, provider: Provider, auth: SharedAuthProvider) -> Self {
        Self {
            session: EndpointSession::new(transport, provider, auth),
        }
    }

    pub fn with_telemetry(self, request: Option<Arc<dyn RequestTelemetry>>) -> Self {
        Self {
            session: self.session.with_request_telemetry(request),
        }
    }

    fn path() -> &'static str {
        "responses/compact"
    }

    pub async fn compact(
        &self,
        body: serde_json::Value,
        extra_headers: HeaderMap,
        request_timeout: Duration,
        turn_state: Option<&OnceLock<String>>,
    ) -> Result<Vec<ResponseItem>, ApiError> {
        self.compact_with_retry_mode(
            body,
            extra_headers,
            request_timeout,
            turn_state,
            /*single_transport_attempt*/ false,
        )
        .await
    }

    /// Compacts one request without transparent transport retries.
    ///
    /// This is used when the host durably claims every physical provider send.
    /// A later host retry must call this method again after acquiring a fresh
    /// attempt lease.
    pub async fn compact_single_attempt(
        &self,
        body: serde_json::Value,
        extra_headers: HeaderMap,
        request_timeout: Duration,
        turn_state: Option<&OnceLock<String>>,
    ) -> Result<Vec<ResponseItem>, ApiError> {
        self.compact_with_retry_mode(
            body,
            extra_headers,
            request_timeout,
            turn_state,
            /*single_transport_attempt*/ true,
        )
        .await
    }

    async fn compact_with_retry_mode(
        &self,
        body: serde_json::Value,
        extra_headers: HeaderMap,
        request_timeout: Duration,
        turn_state: Option<&OnceLock<String>>,
        single_transport_attempt: bool,
    ) -> Result<Vec<ResponseItem>, ApiError> {
        let resp = self
            .execute(
                body,
                extra_headers,
                request_timeout,
                single_transport_attempt,
            )
            .await?;
        if let Some(turn_state) = turn_state
            && let Some(header_value) = resp
                .headers
                .get(X_CODEX_TURN_STATE_HEADER)
                .and_then(|value| value.to_str().ok())
        {
            let _ = turn_state.set(header_value.to_string());
        }
        let parsed: CompactHistoryResponse =
            serde_json::from_slice(&resp.body).map_err(|e| ApiError::Stream(e.to_string()))?;
        Ok(parsed.output)
    }

    async fn execute(
        &self,
        body: serde_json::Value,
        extra_headers: HeaderMap,
        request_timeout: Duration,
        single_transport_attempt: bool,
    ) -> Result<codex_client::Response, ApiError> {
        let configure = |req: &mut codex_client::Request| {
            req.timeout = Some(request_timeout);
        };
        if single_transport_attempt {
            self.session
                .execute_with_single_attempt(
                    Method::POST,
                    Self::path(),
                    extra_headers,
                    Some(body),
                    configure,
                )
                .await
        } else {
            self.session
                .execute_with(
                    Method::POST,
                    Self::path(),
                    extra_headers,
                    Some(body),
                    configure,
                )
                .await
        }
    }

    pub async fn compact_input(
        &self,
        input: &CompactionInput<'_>,
        extra_headers: HeaderMap,
        request_timeout: Duration,
        turn_state: Option<&OnceLock<String>>,
    ) -> Result<Vec<ResponseItem>, ApiError> {
        let body = serde_json::to_value(input)
            .map_err(|e| ApiError::Stream(format!("failed to encode compaction input: {e}")))?;
        self.compact(body, extra_headers, request_timeout, turn_state)
            .await
    }
}

#[derive(Debug, Deserialize)]
struct CompactHistoryResponse {
    output: Vec<ResponseItem>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::AuthProvider;
    use crate::provider::RetryConfig;
    use bytes::Bytes;
    use codex_client::Request;
    use codex_client::Response;
    use codex_client::StreamResponse;
    use codex_client::TransportError;
    use http::StatusCode;
    use std::sync::atomic::AtomicUsize;
    use std::sync::atomic::Ordering;

    #[derive(Clone, Default)]
    struct DummyTransport;

    impl HttpTransport for DummyTransport {
        async fn execute(&self, _req: Request) -> Result<Response, TransportError> {
            Err(TransportError::Build("execute should not run".to_string()))
        }

        async fn stream(&self, _req: Request) -> Result<StreamResponse, TransportError> {
            Err(TransportError::Build("stream should not run".to_string()))
        }
    }

    #[derive(Clone, Default)]
    struct NoAuth;

    impl AuthProvider for NoAuth {
        fn add_auth_headers(&self, _headers: &mut HeaderMap) {}
    }

    #[derive(Clone, Default)]
    struct FlakyUnaryTransport {
        attempts: Arc<AtomicUsize>,
    }

    impl FlakyUnaryTransport {
        fn attempts(&self) -> usize {
            self.attempts.load(Ordering::SeqCst)
        }
    }

    impl HttpTransport for FlakyUnaryTransport {
        async fn execute(&self, _req: Request) -> Result<Response, TransportError> {
            let attempt = self.attempts.fetch_add(1, Ordering::SeqCst);
            if attempt == 0 {
                return Err(TransportError::Network(
                    "first compact attempt fails".to_string(),
                ));
            }
            Ok(Response {
                status: StatusCode::OK,
                headers: HeaderMap::new(),
                body: Bytes::from_static(br#"{"output":[]}"#),
            })
        }

        async fn stream(&self, _req: Request) -> Result<StreamResponse, TransportError> {
            Err(TransportError::Build("stream should not run".to_string()))
        }
    }

    fn retrying_provider() -> Provider {
        Provider {
            name: "test".to_string(),
            base_url: "https://example.test/v1".to_string(),
            query_params: None,
            headers: HeaderMap::new(),
            retry: RetryConfig {
                max_attempts: 3,
                base_delay: Duration::from_millis(1),
                retry_429: false,
                retry_5xx: false,
                retry_transport: true,
            },
            stream_idle_timeout: Duration::from_secs(1),
        }
    }

    #[test]
    fn path_is_responses_compact() {
        assert_eq!(CompactClient::<DummyTransport>::path(), "responses/compact");
    }

    #[tokio::test]
    async fn single_attempt_compact_disables_internal_retry() {
        let transport = FlakyUnaryTransport::default();
        let client = CompactClient::new(transport.clone(), retrying_provider(), Arc::new(NoAuth));

        let result = client
            .compact_single_attempt(
                serde_json::json!({"model": "gpt-test", "input": []}),
                HeaderMap::new(),
                Duration::from_secs(1),
                /*turn_state*/ None,
            )
            .await;

        assert!(result.is_err());
        assert_eq!(transport.attempts(), 1);
    }
}

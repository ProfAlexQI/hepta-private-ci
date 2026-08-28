use futures::StreamExt;
use futures::stream::BoxStream;
use semver::Version;
use serde_json::Value as JsonValue;
use std::io;
use std::time::Duration;

use crate::line_buffer::LineBuffer;
use crate::parser::pull_events_from_value;
use crate::pull::PullEvent;
use crate::pull::PullProgressReporter;
use crate::url::base_url_to_host_root;
use crate::url::is_openai_compatible_base_url;
use codex_core::config::Config;
use codex_http_client::ClientRouteClass;
use codex_http_client::HttpClientFactory;
#[cfg(test)]
use codex_http_client::OutboundProxyPolicy;
use codex_http_client::RouteAwareClientPool;
use codex_http_client::RouteAwareRequestError;
use codex_model_provider_info::ModelProviderInfo;
use codex_model_provider_info::OLLAMA_OSS_PROVIDER_ID;
#[cfg(test)]
use codex_model_provider_info::WireApi;
#[cfg(test)]
use codex_model_provider_info::create_oss_provider_with_base_url;

const OLLAMA_CONNECTION_ERROR: &str = "No running Ollama server detected. Start it with: `ollama serve` (after installing). Install instructions: https://github.com/ollama/ollama?tab=readme-ov-file#ollama";
const OLLAMA_CONNECTION_TIMEOUT: Duration = Duration::from_secs(5);
const MAX_PULL_FRAME_BYTES: usize = 1024 * 1024;
const MAX_REMOTE_ERROR_CHARS: usize = 512;

/// Client for interacting with a local Ollama instance.
pub struct OllamaClient {
    client: RouteAwareClientPool,
    host_root: String,
    uses_openai_compat: bool,
}

impl OllamaClient {
    /// Construct a client for the built-in open-source model provider and verify
    /// that a local Ollama server is reachable.
    pub async fn try_from_oss_provider(config: &Config) -> io::Result<Self> {
        let provider = config
            .model_providers
            .get(OLLAMA_OSS_PROVIDER_ID)
            .ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::NotFound,
                    format!("Built-in provider {OLLAMA_OSS_PROVIDER_ID} not found"),
                )
            })?;

        Self::try_from_provider(provider, config.http_client_factory()).await
    }

    #[cfg(test)]
    async fn try_from_provider_with_base_url(base_url: &str) -> io::Result<Self> {
        let provider = create_oss_provider_with_base_url(base_url, WireApi::Responses);
        Self::try_from_provider(
            &provider,
            HttpClientFactory::new(OutboundProxyPolicy::ReqwestDefault),
        )
        .await
    }

    /// Build a client from a provider definition and verify the server is reachable.
    pub(crate) async fn try_from_provider(
        provider: &ModelProviderInfo,
        http_client_factory: HttpClientFactory,
    ) -> io::Result<Self> {
        let base_url = provider.base_url.as_ref().ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "Ollama provider must have a base_url",
            )
        })?;
        let uses_openai_compat = is_openai_compatible_base_url(base_url);
        let host_root = base_url_to_host_root(base_url);
        let client = RouteAwareClientPool::with_connect_timeout(
            http_client_factory,
            ClientRouteClass::Other,
            OLLAMA_CONNECTION_TIMEOUT,
        )
        .with_legacy_custom_ca_fallback();
        let client = Self {
            client,
            host_root,
            uses_openai_compat,
        };
        client.probe_server().await?;
        Ok(client)
    }

    /// Probe whether the server is reachable by hitting the configured health endpoint.
    async fn probe_server(&self) -> io::Result<()> {
        let endpoint = if self.uses_openai_compat {
            "/v1/models"
        } else {
            "/api/tags"
        };
        let url = self.endpoint(endpoint);
        let response = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|error| match error {
                RouteAwareRequestError::Route(error) => {
                    tracing::warn!(error = %error, "Failed to initialize Ollama HTTP transport");
                    io::Error::other(error)
                }
                error => {
                    tracing::warn!(error = ?error, "Failed to connect to Ollama server");
                    io::Error::other(OLLAMA_CONNECTION_ERROR)
                }
            })?;
        if response.status().is_success() {
            return Ok(());
        }

        tracing::warn!(
            endpoint = endpoint,
            status = response.status().as_u16(),
            "Ollama readiness probe failed"
        );
        Err(io::Error::other(OLLAMA_CONNECTION_ERROR))
    }

    /// Return the list of model names known to the local Ollama instance.
    ///
    /// HTTP and payload failures are not represented as an empty model list.
    pub async fn fetch_models(&self) -> io::Result<Vec<String>> {
        let endpoint = if self.uses_openai_compat {
            "/v1/models"
        } else {
            "/api/tags"
        };
        let response = self
            .client
            .get(self.endpoint(endpoint))
            .send()
            .await
            .map_err(|error| request_error("models", error))?;
        if !response.status().is_success() {
            return Err(status_error("models", response.status().as_u16()));
        }

        let value = response
            .json::<JsonValue>()
            .await
            .map_err(|error| invalid_payload("models", error))?;
        let entries = if self.uses_openai_compat {
            value.get("data")
        } else {
            value.get("models")
        }
        .and_then(JsonValue::as_array)
        .ok_or_else(|| invalid_shape("models", "missing model array"))?;

        let field = if self.uses_openai_compat { "id" } else { "name" };
        let mut models = Vec::with_capacity(entries.len());
        for entry in entries {
            let model = entry
                .get(field)
                .and_then(JsonValue::as_str)
                .map(str::trim)
                .filter(|model| !model.is_empty())
                .ok_or_else(|| invalid_shape("models", "invalid model identifier"))?;
            validate_model_identifier(model)?;
            models.push(model.to_string());
        }
        Ok(models)
    }

    /// Query the server for its version string.
    ///
    /// A non-success response, missing field, or unparsable version is a typed
    /// readiness failure rather than evidence of compatibility.
    pub async fn fetch_version(&self) -> io::Result<Option<Version>> {
        let response = self
            .client
            .get(self.endpoint("/api/version"))
            .send()
            .await
            .map_err(|error| request_error("version", error))?;
        if !response.status().is_success() {
            return Err(status_error("version", response.status().as_u16()));
        }

        let value = response
            .json::<JsonValue>()
            .await
            .map_err(|error| invalid_payload("version", error))?;
        let version = value
            .get("version")
            .and_then(JsonValue::as_str)
            .map(str::trim)
            .filter(|version| !version.is_empty())
            .ok_or_else(|| invalid_shape("version", "missing version string"))?;
        let normalized = version.trim_start_matches('v');
        Version::parse(normalized)
            .map(Some)
            .map_err(|_| invalid_shape("version", "invalid semantic version"))
    }

    /// Start an explicit model pull and emit bounded streaming events.
    ///
    /// Product readiness never calls this method implicitly. Every transport,
    /// UTF-8, JSON, server, and frame-size failure becomes a terminal
    /// [`PullEvent::Error`].
    pub async fn pull_model_stream(
        &self,
        model: &str,
    ) -> io::Result<BoxStream<'static, PullEvent>> {
        validate_model_identifier(model)?;
        let response = self
            .client
            .post(self.endpoint("/api/pull"))
            .json(&serde_json::json!({"model": model, "stream": true}))
            .send()
            .await
            .map_err(|error| request_error("pull", error))?;
        if !response.status().is_success() {
            return Err(status_error("pull", response.status().as_u16()));
        }

        let mut stream = response.bytes_stream();
        let mut buffer = LineBuffer::default();
        let events = async_stream::stream! {
            while let Some(chunk) = stream.next().await {
                let bytes = match chunk {
                    Ok(bytes) => bytes,
                    Err(error) => {
                        yield PullEvent::Error(format!(
                            "OLLAMA_PULL_TRANSPORT_ERROR: {}",
                            sanitize_remote_message(&error.to_string())
                        ));
                        return;
                    }
                };
                buffer.extend_from_slice(&bytes);

                while let Some(line) = buffer.take_line() {
                    match decode_pull_frame(&line) {
                        Ok(decoded) => {
                            let terminal = decoded.iter().any(|event| matches!(event, PullEvent::Success));
                            for event in decoded {
                                yield event;
                            }
                            if terminal {
                                return;
                            }
                        }
                        Err(error) => {
                            yield PullEvent::Error(error);
                            return;
                        }
                    }
                }

                if buffer.len() > MAX_PULL_FRAME_BYTES {
                    yield PullEvent::Error(format!(
                        "OLLAMA_PULL_FRAME_TOO_LARGE: maximum={MAX_PULL_FRAME_BYTES}"
                    ));
                    return;
                }
            }

            if let Some(frame) = buffer.take_remaining() {
                match decode_pull_frame(&frame) {
                    Ok(decoded) => {
                        for event in decoded {
                            yield event;
                        }
                    }
                    Err(error) => yield PullEvent::Error(error),
                }
            }
        };

        Ok(Box::pin(events))
    }

    /// Explicit operator helper to pull a model and drive a progress reporter.
    pub async fn pull_with_reporter(
        &self,
        model: &str,
        reporter: &mut dyn PullProgressReporter,
    ) -> io::Result<()> {
        reporter.on_event(&PullEvent::Status(format!("Pulling model {model}...")))?;
        let mut stream = self.pull_model_stream(model).await?;
        while let Some(event) = stream.next().await {
            reporter.on_event(&event)?;
            match event {
                PullEvent::Success => return Ok(()),
                PullEvent::Error(error) => {
                    return Err(io::Error::other(format!("Pull failed: {error}")));
                }
                PullEvent::ChunkProgress { .. } | PullEvent::Status(_) => {}
            }
        }
        Err(io::Error::new(
            io::ErrorKind::UnexpectedEof,
            "OLLAMA_PULL_UNEXPECTED_EOF: stream ended without success",
        ))
    }

    fn endpoint(&self, endpoint: &str) -> String {
        format!("{}{}", self.host_root.trim_end_matches('/'), endpoint)
    }

    #[cfg(test)]
    fn from_host_root(host_root: impl Into<String>) -> Self {
        let client = RouteAwareClientPool::with_connect_timeout(
            HttpClientFactory::new(OutboundProxyPolicy::ReqwestDefault),
            ClientRouteClass::Other,
            OLLAMA_CONNECTION_TIMEOUT,
        );
        Self {
            client,
            host_root: host_root.into(),
            uses_openai_compat: false,
        }
    }
}

fn decode_pull_frame(frame: &[u8]) -> Result<Vec<PullEvent>, String> {
    if frame.len() > MAX_PULL_FRAME_BYTES {
        return Err(format!(
            "OLLAMA_PULL_FRAME_TOO_LARGE: maximum={MAX_PULL_FRAME_BYTES}"
        ));
    }
    let frame = frame.strip_suffix(b"\n").unwrap_or(frame);
    let frame = frame.strip_suffix(b"\r").unwrap_or(frame);
    if frame.is_empty() {
        return Ok(Vec::new());
    }

    let text = std::str::from_utf8(frame)
        .map_err(|_| "OLLAMA_PULL_INVALID_UTF8".to_string())?;
    let value = serde_json::from_str::<JsonValue>(text)
        .map_err(|_| "OLLAMA_PULL_INVALID_JSON".to_string())?;
    if let Some(error) = value.get("error").and_then(JsonValue::as_str) {
        return Err(format!(
            "OLLAMA_PULL_SERVER_ERROR: {}",
            sanitize_remote_message(error)
        ));
    }

    let events = pull_events_from_value(&value);
    if events.is_empty() {
        return Err("OLLAMA_PULL_UNRECOGNIZED_EVENT".to_string());
    }
    Ok(events)
}

fn validate_model_identifier(model: &str) -> io::Result<()> {
    if model.is_empty()
        || model.len() > 512
        || model != model.trim()
        || model.chars().any(char::is_control)
    {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "OLLAMA_INVALID_MODEL_IDENTIFIER",
        ));
    }
    Ok(())
}

fn request_error(operation: &str, error: impl std::fmt::Display) -> io::Error {
    io::Error::other(format!(
        "OLLAMA_REQUEST_ERROR operation={operation}: {}",
        sanitize_remote_message(&error.to_string())
    ))
}

fn status_error(operation: &str, status: u16) -> io::Error {
    io::Error::other(format!(
        "OLLAMA_HTTP_STATUS operation={operation} status={status}"
    ))
}

fn invalid_payload(operation: &str, error: impl std::fmt::Display) -> io::Error {
    io::Error::new(
        io::ErrorKind::InvalidData,
        format!(
            "OLLAMA_INVALID_JSON operation={operation}: {}",
            sanitize_remote_message(&error.to_string())
        ),
    )
}

fn invalid_shape(operation: &str, reason: &str) -> io::Error {
    io::Error::new(
        io::ErrorKind::InvalidData,
        format!("OLLAMA_INVALID_PAYLOAD operation={operation}: {reason}"),
    )
}

fn sanitize_remote_message(message: &str) -> String {
    let mut sanitized = String::new();
    let mut written = 0usize;
    for character in message.chars() {
        if written >= MAX_REMOTE_ERROR_CHARS {
            break;
        }
        if character.is_control() {
            if character.is_whitespace() {
                sanitized.push(' ');
                written += 1;
            }
        } else {
            sanitized.push(character);
            written += 1;
        }
    }
    sanitized
}

#[cfg(test)]
mod tests {
    #![allow(clippy::expect_used, clippy::unwrap_used)]

    use super::*;
    use assert_matches::assert_matches;
    use pretty_assertions::assert_eq;

    fn network_disabled() -> bool {
        std::env::var(codex_core::spawn::CODEX_SANDBOX_NETWORK_DISABLED_ENV_VAR).is_ok()
    }

    #[tokio::test]
    async fn fetch_models_native_happy_path() {
        if network_disabled() {
            return;
        }
        let server = wiremock::MockServer::start().await;
        wiremock::Mock::given(wiremock::matchers::method("GET"))
            .and(wiremock::matchers::path("/api/tags"))
            .respond_with(wiremock::ResponseTemplate::new(200).set_body_json(
                serde_json::json!({"models": [{"name": "llama3.2:3b"}]}),
            ))
            .mount(&server)
            .await;

        let client = OllamaClient::from_host_root(server.uri());
        assert_eq!(
            client.fetch_models().await.expect("models"),
            vec!["llama3.2:3b"]
        );
    }

    #[tokio::test]
    async fn fetch_models_non_success_is_not_empty_list() {
        if network_disabled() {
            return;
        }
        let server = wiremock::MockServer::start().await;
        wiremock::Mock::given(wiremock::matchers::method("GET"))
            .and(wiremock::matchers::path("/api/tags"))
            .respond_with(wiremock::ResponseTemplate::new(503))
            .mount(&server)
            .await;

        let error = OllamaClient::from_host_root(server.uri())
            .fetch_models()
            .await
            .expect_err("503 must fail");
        assert!(error.to_string().contains("OLLAMA_HTTP_STATUS"));
        assert!(error.to_string().contains("status=503"));
    }

    #[tokio::test]
    async fn fetch_models_rejects_malformed_entry() {
        if network_disabled() {
            return;
        }
        let server = wiremock::MockServer::start().await;
        wiremock::Mock::given(wiremock::matchers::method("GET"))
            .and(wiremock::matchers::path("/api/tags"))
            .respond_with(
                wiremock::ResponseTemplate::new(200)
                    .set_body_json(serde_json::json!({"models": [{}]})),
            )
            .mount(&server)
            .await;

        let error = OllamaClient::from_host_root(server.uri())
            .fetch_models()
            .await
            .expect_err("malformed entry must fail");
        assert_eq!(error.kind(), io::ErrorKind::InvalidData);
    }

    #[tokio::test]
    async fn fetch_version_rejects_non_success_and_bad_semver() {
        if network_disabled() {
            return;
        }
        let server = wiremock::MockServer::start().await;
        wiremock::Mock::given(wiremock::matchers::method("GET"))
            .and(wiremock::matchers::path("/api/version"))
            .respond_with(wiremock::ResponseTemplate::new(200).set_body_json(
                serde_json::json!({"version": "not-semver"}),
            ))
            .mount(&server)
            .await;

        let error = OllamaClient::from_host_root(server.uri())
            .fetch_version()
            .await
            .expect_err("bad version must fail");
        assert!(error.to_string().contains("invalid semantic version"));
    }

    #[tokio::test]
    async fn pull_stream_emits_one_success_for_trailing_frame_without_newline() {
        if network_disabled() {
            return;
        }
        let server = wiremock::MockServer::start().await;
        wiremock::Mock::given(wiremock::matchers::method("POST"))
            .and(wiremock::matchers::path("/api/pull"))
            .respond_with(
                wiremock::ResponseTemplate::new(200)
                    .set_body_raw(r#"{"status":"success"}"#, "application/x-ndjson"),
            )
            .mount(&server)
            .await;

        let events = OllamaClient::from_host_root(server.uri())
            .pull_model_stream("fixture")
            .await
            .expect("start stream")
            .collect::<Vec<_>>()
            .await;
        assert_matches!(
            events.as_slice(),
            [PullEvent::Status(status), PullEvent::Success] if status == "success"
        );
    }

    #[tokio::test]
    async fn pull_stream_turns_invalid_json_into_terminal_error() {
        if network_disabled() {
            return;
        }
        let server = wiremock::MockServer::start().await;
        wiremock::Mock::given(wiremock::matchers::method("POST"))
            .and(wiremock::matchers::path("/api/pull"))
            .respond_with(
                wiremock::ResponseTemplate::new(200)
                    .set_body_raw("not-json\n", "application/x-ndjson"),
            )
            .mount(&server)
            .await;

        let events = OllamaClient::from_host_root(server.uri())
            .pull_model_stream("fixture")
            .await
            .expect("start stream")
            .collect::<Vec<_>>()
            .await;
        assert_matches!(
            events.as_slice(),
            [PullEvent::Error(error)] if error == "OLLAMA_PULL_INVALID_JSON"
        );
    }

    #[tokio::test]
    async fn pull_stream_rejects_oversized_frame() {
        if network_disabled() {
            return;
        }
        let server = wiremock::MockServer::start().await;
        wiremock::Mock::given(wiremock::matchers::method("POST"))
            .and(wiremock::matchers::path("/api/pull"))
            .respond_with(wiremock::ResponseTemplate::new(200).set_body_raw(
                "x".repeat(MAX_PULL_FRAME_BYTES + 1),
                "application/x-ndjson",
            ))
            .mount(&server)
            .await;

        let events = OllamaClient::from_host_root(server.uri())
            .pull_model_stream("fixture")
            .await
            .expect("start stream")
            .collect::<Vec<_>>()
            .await;
        assert_matches!(
            events.as_slice(),
            [PullEvent::Error(error)] if error.contains("FRAME_TOO_LARGE")
        );
    }

    #[tokio::test]
    async fn probe_supports_native_and_openai_compatible_endpoints() {
        if network_disabled() {
            return;
        }
        let server = wiremock::MockServer::start().await;
        wiremock::Mock::given(wiremock::matchers::method("GET"))
            .and(wiremock::matchers::path("/api/tags"))
            .respond_with(wiremock::ResponseTemplate::new(200))
            .mount(&server)
            .await;
        wiremock::Mock::given(wiremock::matchers::method("GET"))
            .and(wiremock::matchers::path("/v1/models"))
            .respond_with(wiremock::ResponseTemplate::new(200))
            .mount(&server)
            .await;

        OllamaClient::from_host_root(server.uri())
            .probe_server()
            .await
            .expect("native probe");
        OllamaClient::try_from_provider_with_base_url(&format!("{}/v1", server.uri()))
            .await
            .expect("OpenAI-compatible probe");
    }
}

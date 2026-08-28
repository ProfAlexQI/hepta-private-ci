mod client;
mod line_buffer;
mod parser;
mod pull;
mod url;

pub use client::OllamaClient;
use codex_core::config::Config;
pub use pull::CliProgressReporter;
pub use pull::PullEvent;
pub use pull::PullProgressReporter;
pub use pull::TuiProgressReporter;
use semver::Version;

/// Default OSS model to use when `--oss` is passed without an explicit `-m`.
pub const DEFAULT_OSS_MODEL: &str = "gpt-oss:20b";

/// Prepare the local Ollama environment when `--oss` is selected.
///
/// Readiness is observation-only. A normal inference startup may not download or
/// install a model as a side effect; missing models must be installed through an
/// explicit operator action before this fence can pass.
pub async fn ensure_oss_ready(config: &Config, client: &OllamaClient) -> std::io::Result<()> {
    let model = config.model.as_deref().unwrap_or(DEFAULT_OSS_MODEL);
    client::validate_model_identifier(model)?;
    let models = client.fetch_models().await?;
    if models.iter().any(|candidate| candidate == model) {
        return Ok(());
    }

    Err(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        format!(
            "OLLAMA_MODEL_NOT_INSTALLED model={model}; automatic model installation is disabled. Run `ollama pull {model}` explicitly and retry."
        ),
    ))
}

fn min_responses_version() -> Version {
    Version::new(0, 13, 4)
}

fn supports_responses(version: &Version) -> bool {
    *version == Version::new(0, 0, 0) || *version >= min_responses_version()
}

/// Ensure the running Ollama server is new enough to support the Responses API.
///
/// Missing, non-success, malformed, or unparsable version evidence fails closed.
pub async fn ensure_responses_supported(client: &OllamaClient) -> std::io::Result<()> {
    let version = client.fetch_version().await?.ok_or_else(|| {
        std::io::Error::other("OLLAMA_VERSION_UNKNOWN: refusing to assume Responses API support")
    })?;

    if supports_responses(&version) {
        return Ok(());
    }

    let minimum = min_responses_version();
    Err(std::io::Error::other(format!(
        "OLLAMA_VERSION_UNSUPPORTED current={version} minimum={minimum}"
    )))
}

#[cfg(test)]
mod tests {
    #![allow(clippy::expect_used, clippy::unwrap_used)]

    use super::*;
    use codex_model_provider_info::WireApi;
    use codex_model_provider_info::create_oss_provider_with_base_url;

    fn network_disabled() -> bool {
        std::env::var(codex_core::spawn::CODEX_SANDBOX_NETWORK_DISABLED_ENV_VAR).is_ok()
    }

    #[tokio::test]
    async fn responses_version_at_cutoff_passes() {
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
            .and(wiremock::matchers::path("/api/version"))
            .respond_with(
                wiremock::ResponseTemplate::new(200)
                    .set_body_json(serde_json::json!({"version": "0.13.4"})),
            )
            .mount(&server)
            .await;

        let provider = create_oss_provider_with_base_url(&server.uri(), WireApi::Responses);
        let client = OllamaClient::try_from_provider(&provider)
            .await
            .expect("client");
        ensure_responses_supported(&client)
            .await
            .expect("supported version");
    }

    #[tokio::test]
    async fn missing_version_endpoint_fails_closed() {
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
            .and(wiremock::matchers::path("/api/version"))
            .respond_with(wiremock::ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let provider = create_oss_provider_with_base_url(&server.uri(), WireApi::Responses);
        let client = OllamaClient::try_from_provider(&provider)
            .await
            .expect("client");
        let error = ensure_responses_supported(&client)
            .await
            .expect_err("missing version must fail");
        assert!(error.to_string().contains("OLLAMA_HTTP_STATUS"));
    }

    #[test]
    fn supports_development_zero_and_cutoff() {
        assert!(supports_responses(&Version::new(0, 0, 0)));
        assert!(!supports_responses(&Version::new(0, 13, 3)));
        assert!(supports_responses(&Version::new(0, 13, 4)));
        assert!(supports_responses(&Version::new(0, 14, 0)));
    }
}

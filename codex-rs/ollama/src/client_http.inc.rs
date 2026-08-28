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

        Self::try_from_provider(provider).await
    }

    #[cfg(test)]
    async fn try_from_provider_with_base_url(base_url: &str) -> io::Result<Self> {
        let provider = create_oss_provider_with_base_url(base_url, WireApi::Responses);
        Self::try_from_provider(&provider).await
    }

    /// Build a client from a provider definition and verify the server is reachable.
    pub(crate) async fn try_from_provider(provider: &ModelProviderInfo) -> io::Result<Self> {
        let base_url = provider.base_url.as_ref().ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "Ollama provider must have a base_url",
            )
        })?;
        validate_loopback_http_base_url(base_url)?;
        let uses_openai_compat = is_openai_compatible_base_url(base_url);
        let host_root = base_url_to_host_root(base_url);
        let client = HttpClientBuilder::new()
            .without_redirects()
            .without_request_logging()
            .connect_timeout(OLLAMA_CONNECTION_TIMEOUT)
            .build_direct()
            .map_err(|error| {
                io::Error::other(format!(
                    "OLLAMA_DIRECT_CLIENT_BUILD_FAILED: {}",
                    sanitize_remote_message(&error.to_string())
                ))
            })?;
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
            .timeout(OLLAMA_CONNECTION_TIMEOUT)
            .send()
            .await
            .map_err(|error| {
                tracing::warn!(
                    failure_class = if error.is_timeout() {
                        "timeout"
                    } else {
                        "request"
                    },
                    "Failed to connect directly to the loopback Ollama server"
                );
                io::Error::other(OLLAMA_CONNECTION_ERROR)
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
            .timeout(OLLAMA_REQUEST_TIMEOUT)
            .send()
            .await
            .map_err(|error| request_error("models", error))?;
        if !response.status().is_success() {
            return Err(status_error("models", response.status().as_u16()));
        }

        let body = read_bounded_control_body(response, "models").await?;
        let value = serde_json::from_slice::<JsonValue>(&body)
            .map_err(|error| invalid_payload("models", error))?;
        let entries = if self.uses_openai_compat {
            value.get("data")
        } else {
            value.get("models")
        }
        .and_then(JsonValue::as_array)
        .ok_or_else(|| invalid_shape("models", "missing model array"))?;

        let field = if self.uses_openai_compat {
            "id"
        } else {
            "name"
        };
        let mut models = Vec::with_capacity(entries.len());
        for entry in entries {
            let model = entry
                .get(field)
                .and_then(JsonValue::as_str)
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
            .timeout(OLLAMA_REQUEST_TIMEOUT)
            .send()
            .await
            .map_err(|error| request_error("version", error))?;
        if !response.status().is_success() {
            return Err(status_error("version", response.status().as_u16()));
        }

        let body = read_bounded_control_body(response, "version").await?;
        let value = serde_json::from_slice::<JsonValue>(&body)
            .map_err(|error| invalid_payload("version", error))?;
        let version = value
            .get("version")
            .and_then(JsonValue::as_str)
            .filter(|version| !version.is_empty() && *version == version.trim())
            .ok_or_else(|| invalid_shape("version", "missing or non-canonical version string"))?;
        let normalized = version.strip_prefix('v').unwrap_or(version);
        Version::parse(normalized)
            .map(Some)
            .map_err(|_| invalid_shape("version", "invalid semantic version"))
    }
}

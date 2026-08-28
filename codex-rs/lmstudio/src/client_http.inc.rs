impl LMStudioClient {
    pub async fn try_from_provider(config: &Config) -> io::Result<Self> {
        let provider = config
            .model_providers
            .get(LMSTUDIO_OSS_PROVIDER_ID)
            .ok_or_else(|| {
                coded_error(
                    io::ErrorKind::NotFound,
                    "LMSTUDIO_PROVIDER_MISSING",
                    format!("built-in provider {LMSTUDIO_OSS_PROVIDER_ID} not found"),
                )
            })?;
        let base_url = provider.base_url.as_ref().ok_or_else(|| {
            coded_error(
                io::ErrorKind::InvalidData,
                "LMSTUDIO_BASE_URL_MISSING",
                "provider must have a base_url",
            )
        })?;
        validate_loopback_http_base_url(base_url)?;

        let client = HttpClientBuilder::new()
            .without_redirects()
            .without_request_logging()
            .connect_timeout(LMSTUDIO_CONNECTION_TIMEOUT)
            .build_direct()
            .map_err(|error| {
                coded_error(
                    io::ErrorKind::Other,
                    "LMSTUDIO_DIRECT_CLIENT_BUILD_FAILED",
                    sanitize_diagnostic(&error.to_string()),
                )
            })?;
        let client = Self {
            client,
            base_url: base_url.to_string(),
        };
        client.check_server().await?;
        Ok(client)
    }

    async fn check_server(&self) -> io::Result<()> {
        let response = self
            .client
            .get(self.endpoint("/models"))
            .timeout(LMSTUDIO_REQUEST_TIMEOUT)
            .send()
            .await
            .map_err(|error| {
                coded_error(
                    io::ErrorKind::ConnectionRefused,
                    "LMSTUDIO_SERVER_UNREACHABLE",
                    format!("{LMSTUDIO_CONNECTION_ERROR} {error}"),
                )
            })?;
        if response.status().is_success() {
            return Ok(());
        }
        Err(coded_error(
            io::ErrorKind::ConnectionRefused,
            "LMSTUDIO_HTTP_STATUS",
            format!("operation=server_probe status={}", response.status().as_u16()),
        ))
    }

    /// Load an already-installed model by sending a bounded minimal Responses request.
    pub async fn load_model(&self, model: &str) -> io::Result<()> {
        validate_model_identifier(model)?;
        let response = self
            .client
            .post(self.endpoint("/responses"))
            .timeout(LMSTUDIO_REQUEST_TIMEOUT)
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({
                "model": model,
                "input": "",
                "max_output_tokens": 1
            }))
            .send()
            .await
            .map_err(|error| {
                coded_error(
                    io::ErrorKind::ConnectionAborted,
                    "LMSTUDIO_LOAD_REQUEST_FAILED",
                    sanitize_diagnostic(&error.to_string()),
                )
            })?;
        if !response.status().is_success() {
            return Err(coded_error(
                io::ErrorKind::Other,
                "LMSTUDIO_LOAD_HTTP_STATUS",
                format!("status={}", response.status().as_u16()),
            ));
        }
        tracing::info!(model = model, "LM Studio model readiness probe passed");
        Ok(())
    }

    /// Return the complete, validated list of models available on the server.
    pub async fn fetch_models(&self) -> io::Result<Vec<String>> {
        let response = self
            .client
            .get(self.endpoint("/models"))
            .timeout(LMSTUDIO_REQUEST_TIMEOUT)
            .send()
            .await
            .map_err(|error| {
                coded_error(
                    io::ErrorKind::ConnectionAborted,
                    "LMSTUDIO_MODELS_REQUEST_FAILED",
                    sanitize_diagnostic(&error.to_string()),
                )
            })?;
        if !response.status().is_success() {
            return Err(coded_error(
                io::ErrorKind::Other,
                "LMSTUDIO_MODELS_HTTP_STATUS",
                format!("status={}", response.status().as_u16()),
            ));
        }

        let body = read_bounded_control_body(response, "models").await?;
        let value = serde_json::from_slice::<serde_json::Value>(&body).map_err(|error| {
            coded_error(
                io::ErrorKind::InvalidData,
                "LMSTUDIO_MODELS_INVALID_JSON",
                sanitize_diagnostic(&error.to_string()),
            )
        })?;
        let entries = value
            .get("data")
            .and_then(serde_json::Value::as_array)
            .ok_or_else(|| {
                coded_error(
                    io::ErrorKind::InvalidData,
                    "LMSTUDIO_MODELS_INVALID_PAYLOAD",
                    "missing data array",
                )
            })?;
        let mut models = Vec::with_capacity(entries.len());
        for entry in entries {
            let model = entry
                .get("id")
                .and_then(serde_json::Value::as_str)
                .ok_or_else(|| {
                    coded_error(
                        io::ErrorKind::InvalidData,
                        "LMSTUDIO_MODELS_INVALID_PAYLOAD",
                        "model entry is missing id",
                    )
                })?;
            validate_model_identifier(model)?;
            models.push(model.to_string());
        }
        Ok(models)
    }

    /// Explicit operator-only model download.
    ///
    /// The executable must resolve to a regular canonical file and match the
    /// SHA-256 value supplied through `CODEX_LMS_CLI_SHA256`. The command has a
    /// hard timeout, is killed on cancellation/drop, and captures only bounded
    /// diagnostic stderr.
    pub async fn download_model(&self, model: &str) -> io::Result<()> {
        validate_model_identifier(model)?;
        let executable = Self::find_lms()?;
        verify_lms_cli_provenance(&executable).await?;
        run_download_command(&executable, model, LMS_DOWNLOAD_TIMEOUT).await
    }

    fn endpoint(&self, suffix: &str) -> String {
        format!("{}{}", self.base_url.trim_end_matches('/'), suffix)
    }

    fn find_lms() -> io::Result<PathBuf> {
        Self::find_lms_with_home_dir(None)
    }

    fn find_lms_with_home_dir(home_dir: Option<&str>) -> io::Result<PathBuf> {
        if let Ok(path) = which::which("lms") {
            return canonical_executable(path);
        }

        let home = match home_dir {
            Some(home) => home.to_string(),
            None => {
                #[cfg(unix)]
                {
                    std::env::var("HOME").unwrap_or_default()
                }
                #[cfg(windows)]
                {
                    std::env::var("USERPROFILE").unwrap_or_default()
                }
            }
        };
        #[cfg(unix)]
        let fallback = PathBuf::from(home).join(".lmstudio/bin/lms");
        #[cfg(windows)]
        let fallback = PathBuf::from(home).join(".lmstudio/bin/lms.exe");

        if fallback.exists() {
            canonical_executable(fallback)
        } else {
            Err(coded_error(
                io::ErrorKind::NotFound,
                "LMSTUDIO_CLI_NOT_FOUND",
                "install LM Studio and make the `lms` CLI available",
            ))
        }
    }
}

use crate::sha256::digest_reader;
use codex_core::config::Config;
use codex_http_client::ClientRouteClass;
use codex_http_client::RouteAwareClientPool;
use codex_model_provider_info::LMSTUDIO_OSS_PROVIDER_ID;
use std::fs::File;
use std::io;
use std::path::Path;
use std::path::PathBuf;
use std::process::Stdio;
use std::time::Duration;
use tokio::io::AsyncRead;
use tokio::io::AsyncReadExt;

#[derive(Clone)]
pub struct LMStudioClient {
    client: RouteAwareClientPool,
    base_url: String,
}

const LMSTUDIO_CONNECTION_ERROR: &str = "LM Studio is not responding. Install from https://lmstudio.ai/download and run `lms server start`.";
const LMSTUDIO_CONNECTION_TIMEOUT: Duration = Duration::from_secs(5);
const LMSTUDIO_REQUEST_TIMEOUT: Duration = Duration::from_secs(60);
const LMS_DOWNLOAD_TIMEOUT: Duration = Duration::from_secs(15 * 60);
const LMS_CLI_SHA256_ENV: &str = "CODEX_LMS_CLI_SHA256";
const MAX_STDERR_BYTES: usize = 4096;

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

        let client = RouteAwareClientPool::with_connect_timeout(
            config.http_client_factory(),
            ClientRouteClass::Other,
            LMSTUDIO_CONNECTION_TIMEOUT,
        );
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

        let value = response
            .json::<serde_json::Value>()
            .await
            .map_err(|error| {
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

async fn verify_lms_cli_provenance(path: &Path) -> io::Result<()> {
    let expected = std::env::var(LMS_CLI_SHA256_ENV).map_err(|_| {
        coded_error(
            io::ErrorKind::PermissionDenied,
            "LMSTUDIO_CLI_DIGEST_REQUIRED",
            format!("set {LMS_CLI_SHA256_ENV}=sha256:<64 lowercase hex>"),
        )
    })?;
    verify_lms_cli_with_expected(path, &expected).await
}

async fn verify_lms_cli_with_expected(path: &Path, expected: &str) -> io::Result<()> {
    let expected = parse_sha256_binding(expected)?;
    let path = path.to_path_buf();
    let actual = tokio::task::spawn_blocking(move || {
        let mut file = File::open(&path)?;
        digest_reader(&mut file)
    })
    .await
    .map_err(|error| {
        coded_error(
            io::ErrorKind::Other,
            "LMSTUDIO_CLI_HASH_TASK_FAILED",
            sanitize_diagnostic(&error.to_string()),
        )
    })??;
    if actual == expected {
        return Ok(());
    }
    Err(coded_error(
        io::ErrorKind::PermissionDenied,
        "LMSTUDIO_CLI_DIGEST_MISMATCH",
        format!("expected=sha256:{expected} actual=sha256:{actual}"),
    ))
}

async fn run_download_command(path: &Path, model: &str, timeout: Duration) -> io::Result<()> {
    let mut command = tokio::process::Command::new(path);
    command
        .args(["get", "--yes", model])
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .stderr(Stdio::piped())
        .kill_on_drop(true);
    let mut child = command.spawn().map_err(|error| {
        coded_error(
            io::ErrorKind::Other,
            "LMSTUDIO_CLI_SPAWN_FAILED",
            sanitize_diagnostic(&error.to_string()),
        )
    })?;
    let stderr = child.stderr.take().ok_or_else(|| {
        coded_error(
            io::ErrorKind::Other,
            "LMSTUDIO_CLI_STDERR_UNAVAILABLE",
            "failed to capture bounded stderr",
        )
    })?;

    let completion = tokio::time::timeout(timeout, async {
        let wait = child.wait();
        let read = read_bounded_stderr(stderr);
        tokio::try_join!(wait, read)
    })
    .await;

    let (status, stderr) = match completion {
        Ok(result) => result?,
        Err(_) => {
            let _ = child.kill().await;
            let _ = child.wait().await;
            return Err(coded_error(
                io::ErrorKind::TimedOut,
                "LMSTUDIO_CLI_TIMEOUT",
                format!("model download exceeded {} seconds", timeout.as_secs()),
            ));
        }
    };

    if !status.success() {
        return Err(coded_error(
            io::ErrorKind::Other,
            "LMSTUDIO_CLI_EXIT_FAILURE",
            format!(
                "exit_code={} stderr={}",
                status.code().unwrap_or(-1),
                sanitize_diagnostic(&String::from_utf8_lossy(&stderr))
            ),
        ));
    }
    tracing::info!(model = model, executable = %path.display(), "LM Studio model downloaded explicitly");
    Ok(())
}

async fn read_bounded_stderr(mut stderr: impl AsyncRead + Unpin) -> io::Result<Vec<u8>> {
    let mut captured = Vec::with_capacity(MAX_STDERR_BYTES);
    let mut buffer = [0u8; 1024];
    loop {
        let read = stderr.read(&mut buffer).await?;
        if read == 0 {
            break;
        }
        let remaining = MAX_STDERR_BYTES.saturating_sub(captured.len());
        if remaining != 0 {
            captured.extend_from_slice(&buffer[..read.min(remaining)]);
        }
    }
    Ok(captured)
}

fn canonical_executable(path: impl AsRef<Path>) -> io::Result<PathBuf> {
    let canonical = std::fs::canonicalize(path.as_ref()).map_err(|error| {
        coded_error(
            io::ErrorKind::NotFound,
            "LMSTUDIO_CLI_CANONICALIZE_FAILED",
            sanitize_diagnostic(&error.to_string()),
        )
    })?;
    let metadata = std::fs::metadata(&canonical).map_err(|error| {
        coded_error(
            io::ErrorKind::NotFound,
            "LMSTUDIO_CLI_METADATA_FAILED",
            sanitize_diagnostic(&error.to_string()),
        )
    })?;
    if !metadata.is_file() {
        return Err(coded_error(
            io::ErrorKind::InvalidData,
            "LMSTUDIO_CLI_NOT_REGULAR_FILE",
            canonical.display().to_string(),
        ));
    }
    Ok(canonical)
}

fn parse_sha256_binding(binding: &str) -> io::Result<String> {
    let digest = binding.strip_prefix("sha256:").ok_or_else(|| {
        coded_error(
            io::ErrorKind::InvalidInput,
            "LMSTUDIO_CLI_DIGEST_INVALID",
            "expected sha256:<64 lowercase hex>",
        )
    })?;
    if digest.len() != 64
        || !digest
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err(coded_error(
            io::ErrorKind::InvalidInput,
            "LMSTUDIO_CLI_DIGEST_INVALID",
            "expected sha256:<64 lowercase hex>",
        ));
    }
    Ok(digest.to_string())
}

fn validate_model_identifier(model: &str) -> io::Result<()> {
    if model.is_empty()
        || model.len() > 512
        || model != model.trim()
        || model.chars().any(char::is_control)
    {
        return Err(coded_error(
            io::ErrorKind::InvalidInput,
            "LMSTUDIO_INVALID_MODEL_IDENTIFIER",
            "model identifier is empty, oversized, padded, or contains control characters",
        ));
    }
    Ok(())
}

fn coded_error(kind: io::ErrorKind, code: &str, detail: impl AsRef<str>) -> io::Error {
    io::Error::new(kind, format!("{code}: {}", detail.as_ref()))
}

fn sanitize_diagnostic(value: &str) -> String {
    let mut output = String::new();
    for character in value.chars().take(MAX_STDERR_BYTES) {
        if character.is_control() {
            if character.is_whitespace() {
                output.push(' ');
            }
        } else {
            output.push(character);
        }
    }
    output
}

#[cfg(test)]
mod tests {
    #![allow(clippy::expect_used, clippy::unwrap_used)]

    use super::*;
    use crate::sha256::digest_bytes;
    use std::sync::atomic::AtomicU64;
    use std::sync::atomic::Ordering;

    static NEXT_TEST_DIR: AtomicU64 = AtomicU64::new(0);

    fn network_disabled() -> bool {
        std::env::var(codex_core::spawn::CODEX_SANDBOX_NETWORK_DISABLED_ENV_VAR).is_ok()
    }

    fn client_from_host_root(host_root: impl Into<String>) -> LMStudioClient {
        let client = RouteAwareClientPool::with_connect_timeout(
            codex_http_client::HttpClientFactory::new(
                codex_http_client::OutboundProxyPolicy::ReqwestDefault,
            ),
            ClientRouteClass::Other,
            LMSTUDIO_CONNECTION_TIMEOUT,
        );
        LMStudioClient {
            client,
            base_url: host_root.into(),
        }
    }

    #[tokio::test]
    async fn fetch_models_happy_path() {
        if network_disabled() {
            return;
        }
        let server = wiremock::MockServer::start().await;
        wiremock::Mock::given(wiremock::matchers::method("GET"))
            .and(wiremock::matchers::path("/models"))
            .respond_with(
                wiremock::ResponseTemplate::new(200).set_body_json(
                    serde_json::json!({"data": [{"id": "openai/gpt-oss-20b"}]}),
                ),
            )
            .mount(&server)
            .await;
        assert_eq!(
            client_from_host_root(server.uri())
                .fetch_models()
                .await
                .expect("models"),
            vec!["openai/gpt-oss-20b"]
        );
    }

    #[tokio::test]
    async fn fetch_models_rejects_non_success_and_malformed_entries() {
        if network_disabled() {
            return;
        }
        let server = wiremock::MockServer::start().await;
        wiremock::Mock::given(wiremock::matchers::method("GET"))
            .and(wiremock::matchers::path("/models"))
            .respond_with(wiremock::ResponseTemplate::new(500))
            .mount(&server)
            .await;
        let error = client_from_host_root(server.uri())
            .fetch_models()
            .await
            .expect_err("500 must fail");
        assert!(error.to_string().contains("LMSTUDIO_MODELS_HTTP_STATUS"));
    }

    #[test]
    fn sha256_binding_is_strict() {
        assert!(parse_sha256_binding(&format!("sha256:{}", "a".repeat(64))).is_ok());
        assert!(parse_sha256_binding(&format!("sha256:{}", "A".repeat(64))).is_err());
        assert!(parse_sha256_binding(&"a".repeat(64)).is_err());
        assert!(parse_sha256_binding("sha256:abc").is_err());
    }

    #[tokio::test]
    async fn executable_provenance_matches_exact_digest() {
        let directory = test_directory();
        let executable = directory.join("lms-fixture");
        std::fs::write(&executable, b"fixture executable").expect("write executable");
        let canonical = canonical_executable(&executable).expect("canonical executable");
        let expected = format!("sha256:{}", digest_bytes(b"fixture executable"));
        verify_lms_cli_with_expected(&canonical, &expected)
            .await
            .expect("matching digest");
        let error = verify_lms_cli_with_expected(
            &canonical,
            &format!("sha256:{}", "0".repeat(64)),
        )
        .await
        .expect_err("mismatch must fail");
        assert!(error.to_string().contains("LMSTUDIO_CLI_DIGEST_MISMATCH"));
        std::fs::remove_dir_all(directory).expect("remove fixture");
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn explicit_download_command_is_killed_on_timeout() {
        use std::os::unix::fs::PermissionsExt;

        let directory = test_directory();
        let executable = directory.join("lms-timeout");
        std::fs::write(&executable, b"#!/bin/sh\nwhile :; do :; done\n").expect("write script");
        let mut permissions = std::fs::metadata(&executable)
            .expect("metadata")
            .permissions();
        permissions.set_mode(0o700);
        std::fs::set_permissions(&executable, permissions).expect("permissions");
        let error = run_download_command(&executable, "fixture", Duration::from_millis(50))
            .await
            .expect_err("timeout must fail");
        assert_eq!(error.kind(), io::ErrorKind::TimedOut);
        assert!(error.to_string().contains("LMSTUDIO_CLI_TIMEOUT"));
        std::fs::remove_dir_all(directory).expect("remove fixture");
    }

    #[test]
    fn model_identifier_is_bounded() {
        assert!(validate_model_identifier("openai/gpt-oss-20b").is_ok());
        assert!(validate_model_identifier("").is_err());
        assert!(validate_model_identifier(" padded").is_err());
        assert!(validate_model_identifier("bad\nmodel").is_err());
        assert!(validate_model_identifier(&"x".repeat(513)).is_err());
    }

    #[test]
    fn missing_fallback_is_typed() {
        let result = LMStudioClient::find_lms_with_home_dir(Some("/path/that/does/not/exist"));
        if let Err(error) = result {
            assert!(error.to_string().contains("LMSTUDIO_CLI_NOT_FOUND"));
        }
    }

    fn test_directory() -> PathBuf {
        let id = NEXT_TEST_DIR.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "codex-lmstudio-inf0c-{}-{id}",
            std::process::id()
        ));
        std::fs::create_dir_all(&path).expect("create fixture directory");
        path
    }
}

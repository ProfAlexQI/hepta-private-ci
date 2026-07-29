//! Runtime support for Model Context Protocol (MCP) servers.
//!
//! This module contains data that describes the runtime environment in which MCP
//! servers execute, plus the sandbox state payload sent to capable servers and a
//! tiny shared metrics helper. Transport startup and orchestration live in
//! [`crate::rmcp_client`] and [`crate::connection_manager`].

use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use anyhow::bail;
use codex_config::McpServerConfig;
use codex_exec_server::Environment;
use codex_exec_server::HttpClient;
use codex_exec_server::ReqwestHttpClient;
use codex_exec_server::RouteAwareHttpClient;
use codex_http_client::HttpClientFactory;
use codex_protocol::models::PermissionProfile;
use codex_protocol::protocol::SandboxPolicy;

use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SandboxState {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permission_profile: Option<PermissionProfile>,
    pub sandbox_policy: SandboxPolicy,
    pub codex_linux_sandbox_exe: Option<PathBuf>,
    pub sandbox_cwd: PathBuf,
    #[serde(default)]
    pub use_legacy_landlock: bool,
}

/// Runtime placement information used when starting MCP server transports.
///
/// `McpConfig` describes what servers exist. This value describes where those
/// servers should run for the current caller. Keep it explicit at manager
/// construction time so status/snapshot paths and real sessions make the same
/// local-vs-remote decision. `fallback_cwd` is not a per-server override; it is
/// used when a stdio server omits `cwd` and the launcher needs a concrete
/// process working directory.
#[derive(Clone)]
pub struct McpRuntimeEnvironment {
    environment: Arc<Environment>,
    fallback_cwd: PathBuf,
    local_http_client: Arc<dyn HttpClient>,
}

impl McpRuntimeEnvironment {
    pub fn new(environment: Arc<Environment>, fallback_cwd: PathBuf) -> Self {
        Self {
            environment,
            fallback_cwd,
            local_http_client: Arc::new(ReqwestHttpClient),
        }
    }

    /// Creates a runtime whose local HTTP capability follows the effective
    /// session proxy and custom-CA policy.
    pub fn new_with_http_client_factory(
        environment: Arc<Environment>,
        fallback_cwd: PathBuf,
        http_client_factory: HttpClientFactory,
    ) -> Self {
        Self {
            environment,
            fallback_cwd,
            local_http_client: Arc::new(RouteAwareHttpClient::new(http_client_factory)),
        }
    }

    pub(crate) fn environment(&self) -> Arc<Environment> {
        Arc::clone(&self.environment)
    }

    pub(crate) fn fallback_cwd(&self) -> PathBuf {
        self.fallback_cwd.clone()
    }

    /// Resolve the HTTP capability from the same placement decision used when
    /// starting the MCP transport.
    pub fn http_client_for_server(&self, server: &McpServerConfig) -> Result<Arc<dyn HttpClient>> {
        match server.experimental_environment.as_deref() {
            None | Some("local") => Ok(Arc::clone(&self.local_http_client)),
            Some("remote") => {
                if !self.environment.is_remote() {
                    bail!("remote MCP server requires a remote environment");
                }
                Ok(self.environment.get_http_client())
            }
            Some(environment) => {
                bail!("unsupported experimental_environment `{environment}` for MCP server")
            }
        }
    }
}

pub(crate) fn emit_duration(metric: &str, duration: Duration, tags: &[(&str, &str)]) {
    if let Some(metrics) = codex_otel::global() {
        let _ = metrics.record_duration(metric, duration, tags);
    }
}

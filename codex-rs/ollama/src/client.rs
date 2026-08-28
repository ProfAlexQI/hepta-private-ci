use futures::StreamExt;
use futures::stream::BoxStream;
use semver::Version;
use serde_json::Value as JsonValue;
use std::io;
use std::net::IpAddr;
use std::net::ToSocketAddrs;
use std::time::Duration;

use crate::line_buffer::LineBuffer;
use crate::parser::pull_events_from_value;
use crate::pull::PullEvent;
use crate::pull::PullProgressReporter;
use crate::url::base_url_to_host_root;
use crate::url::is_openai_compatible_base_url;
use codex_core::config::Config;
use codex_http_client::HttpClient;
use codex_http_client::HttpClientBuilder;
use codex_model_provider_info::ModelProviderInfo;
use codex_model_provider_info::OLLAMA_OSS_PROVIDER_ID;
#[cfg(test)]
use codex_model_provider_info::WireApi;
#[cfg(test)]
use codex_model_provider_info::create_oss_provider_with_base_url;

const OLLAMA_CONNECTION_ERROR: &str =
    "No running Ollama server detected. Start it with: `ollama serve` (after installing). Install instructions: https://github.com/ollama/ollama?tab=readme-ov-file#ollama";
const OLLAMA_CONNECTION_TIMEOUT: Duration = Duration::from_secs(5);
const OLLAMA_REQUEST_TIMEOUT: Duration = Duration::from_secs(60);
const OLLAMA_PULL_IDLE_TIMEOUT: Duration = Duration::from_secs(60);
const MAX_PULL_FRAME_BYTES: usize = 1024 * 1024;
const MAX_CONTROL_RESPONSE_BYTES: usize = 4 * 1024 * 1024;
const MAX_REMOTE_ERROR_CHARS: usize = 512;

/// Client for interacting with a local Ollama instance.
pub struct OllamaClient {
    client: HttpClient,
    host_root: String,
    uses_openai_compat: bool,
}

include!("client_http.inc.rs");
include!("client_pull.inc.rs");
include!("client_support.inc.rs");

#[cfg(test)]
mod tests {
    #![allow(clippy::expect_used, clippy::unwrap_used)]

    use super::*;
    use assert_matches::assert_matches;
    use pretty_assertions::assert_eq;

    include!("client_tests_http.inc.rs");
    include!("client_tests_pull.inc.rs");
}

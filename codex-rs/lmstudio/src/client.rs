use crate::sha256::digest_reader;
use codex_core::config::Config;
use codex_http_client::HttpClient;
use codex_http_client::HttpClientBuilder;
use codex_model_provider_info::LMSTUDIO_OSS_PROVIDER_ID;
use std::fs::File;
use std::io;
use std::net::IpAddr;
use std::net::ToSocketAddrs;
use std::path::Path;
use std::path::PathBuf;
use std::process::Stdio;
use std::time::Duration;
use tokio::io::AsyncRead;
use tokio::io::AsyncReadExt;

#[derive(Clone)]
pub struct LMStudioClient {
    client: HttpClient,
    base_url: String,
}

const LMSTUDIO_CONNECTION_ERROR: &str = "LM Studio is not responding. Install from https://lmstudio.ai/download and run `lms server start`.";
const LMSTUDIO_CONNECTION_TIMEOUT: Duration = Duration::from_secs(5);
const LMSTUDIO_REQUEST_TIMEOUT: Duration = Duration::from_secs(60);
const LMS_DOWNLOAD_TIMEOUT: Duration = Duration::from_secs(15 * 60);
const LMS_CLI_SHA256_ENV: &str = "CODEX_LMS_CLI_SHA256";
const MAX_STDERR_BYTES: usize = 4096;
const MAX_CONTROL_RESPONSE_BYTES: usize = 4 * 1024 * 1024;

include!("client_http.inc.rs");
include!("client_control.inc.rs");
include!("client_command.inc.rs");
include!("client_support.inc.rs");

#[cfg(test)]
mod tests {
    #![allow(clippy::expect_used, clippy::unwrap_used)]

    use super::*;
    use crate::sha256::digest_bytes;
    use std::sync::atomic::AtomicU64;
    use std::sync::atomic::Ordering;

    static NEXT_TEST_DIR: AtomicU64 = AtomicU64::new(0);

    include!("client_tests_http.inc.rs");
    include!("client_tests_command.inc.rs");
}

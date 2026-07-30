use std::net::IpAddr;
use std::sync::OnceLock;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use codex_http_client::ClientRouteClass;
use codex_http_client::HttpClientFactory;
use codex_http_client::OutboundProxyPolicy;
use codex_http_client::RouteAwareClientPool;
use reqwest::Url;
use serde_json::Value;

const MAX_JSON_RESPONSE_BYTES: usize = 2 * 1024 * 1024;
const MAX_TEXT_RESPONSE_BYTES: usize = 8 * 1024 * 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutboundHttpCapability {
    TelegramBotApi,
    LiteralLoopbackProvider,
    OpenAiCodexApi,
    DuckDuckGoHtml,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EgressMethod {
    Get,
    Post,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EgressAuthorization {
    LegacyGovernedTelegram,
    LiteralLoopback,
    EffectBound {
        admission_hash: String,
        effect_plan_hash: String,
    },
}

pub type JsonMethod = EgressMethod;

#[derive(Debug, Clone)]
pub struct TextEgressRequest {
    pub authorization: EgressAuthorization,
    pub capability: OutboundHttpCapability,
    pub method: EgressMethod,
    pub url: String,
    pub query: Vec<(String, String)>,
    pub headers: Vec<(String, String)>,
    pub body: Option<Vec<u8>>,
    pub timeout: Duration,
    pub max_response_bytes: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextEgressResponse {
    pub status: u16,
    pub body: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct JsonEgressRequest {
    pub authorization: EgressAuthorization,
    pub capability: OutboundHttpCapability,
    pub method: JsonMethod,
    pub url: String,
    pub query: Vec<(String, String)>,
    pub bearer_token: Option<String>,
    pub body: Option<Value>,
    pub timeout: Duration,
}

#[derive(Debug, Clone, PartialEq)]
pub struct JsonEgressResponse {
    pub status: u16,
    pub body: Value,
}

fn validate_destination(capability: OutboundHttpCapability, url: &Url) -> Result<(), String> {
    if !url.username().is_empty() || url.password().is_some() || url.fragment().is_some() {
        return Err(
            "outbound destination contains forbidden authority or fragment material".into(),
        );
    }
    let https_host = |host: &str| {
        url.scheme() == "https"
            && url.host_str() == Some(host)
            && matches!(url.port_or_known_default(), Some(443))
    };
    match capability {
        OutboundHttpCapability::TelegramBotApi if https_host("api.telegram.org") => {}
        OutboundHttpCapability::OpenAiCodexApi
            if https_host("chatgpt.com") || https_host("auth.openai.com") => {}
        OutboundHttpCapability::DuckDuckGoHtml if https_host("duckduckgo.com") => {}
        OutboundHttpCapability::LiteralLoopbackProvider => {
            let host = url
                .host_str()
                .and_then(|host| host.parse::<IpAddr>().ok())
                .filter(IpAddr::is_loopback);
            if url.scheme() != "http" || host.is_none() {
                return Err(
                    "loopback provider capability requires a literal loopback HTTP address".into(),
                );
            }
        }
        OutboundHttpCapability::TelegramBotApi => {
            return Err("Telegram egress capability denied a non-Telegram destination".into());
        }
        OutboundHttpCapability::OpenAiCodexApi => {
            return Err("OpenAI Codex egress capability denied an unapproved destination".into());
        }
        OutboundHttpCapability::DuckDuckGoHtml => {
            return Err("web search egress capability denied a non-DuckDuckGo destination".into());
        }
    }
    Ok(())
}

fn validate_authorization(
    capability: OutboundHttpCapability,
    authorization: &EgressAuthorization,
) -> Result<(), String> {
    match (capability, authorization) {
        (OutboundHttpCapability::TelegramBotApi, EgressAuthorization::LegacyGovernedTelegram)
        | (OutboundHttpCapability::LiteralLoopbackProvider, EgressAuthorization::LiteralLoopback) => {
            Ok(())
        }
        (
            OutboundHttpCapability::OpenAiCodexApi | OutboundHttpCapability::DuckDuckGoHtml,
            EgressAuthorization::EffectBound {
                admission_hash,
                effect_plan_hash,
            },
        ) if content_hash_is_valid(admission_hash) && content_hash_is_valid(effect_plan_hash) => {
            Ok(())
        }
        _ => Err("egress capability lacks a matching execution authorization".into()),
    }
}

fn content_hash_is_valid(value: &str) -> bool {
    value.strip_prefix("sha256:").is_some_and(|hex| {
        hex.len() == 64
            && hex
                .bytes()
                .all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
    })
}

struct WorkerClients {
    proxied: RouteAwareClientPool,
    direct: RouteAwareClientPool,
}

impl WorkerClients {
    fn new() -> Self {
        Self {
            proxied: RouteAwareClientPool::new_without_redirects_or_request_logging(
                HttpClientFactory::new(OutboundProxyPolicy::RespectSystemProxy),
                ClientRouteClass::Api,
            ),
            direct: RouteAwareClientPool::new_without_redirects_or_request_logging(
                HttpClientFactory::new(OutboundProxyPolicy::DirectOnly),
                ClientRouteClass::Api,
            ),
        }
    }

    fn for_capability(&self, capability: OutboundHttpCapability) -> &RouteAwareClientPool {
        match capability {
            OutboundHttpCapability::LiteralLoopbackProvider => &self.direct,
            _ => &self.proxied,
        }
    }
}

async fn execute_text_async(
    clients: &WorkerClients,
    request: TextEgressRequest,
) -> Result<TextEgressResponse, String> {
    let mut url =
        Url::parse(&request.url).map_err(|_| "invalid outbound destination".to_string())?;
    validate_destination(request.capability, &url)?;
    validate_authorization(request.capability, &request.authorization)?;
    if !request.query.is_empty() {
        url.query_pairs_mut().extend_pairs(&request.query);
    }
    if request.max_response_bytes == 0 || request.max_response_bytes > MAX_TEXT_RESPONSE_BYTES {
        return Err("outbound response bound is invalid".into());
    }
    let pool = clients.for_capability(request.capability);
    let mut outbound = match request.method {
        EgressMethod::Get => pool.get(url),
        EgressMethod::Post => pool.post(url),
    }
    .timeout(request.timeout);
    for (name, value) in request.headers {
        outbound = outbound.header(name, value);
    }
    if let Some(body) = request.body {
        outbound = outbound.body(body);
    }
    let response = outbound
        .send()
        .await
        .map_err(|_| "outbound request failed".to_string())?;
    let status = response.status().as_u16();
    let bytes = response
        .bytes()
        .await
        .map_err(|_| "outbound response read failed".to_string())?;
    if bytes.len() > request.max_response_bytes {
        return Err("outbound response exceeded its bounded response size".into());
    }
    Ok(TextEgressResponse {
        status,
        body: bytes.to_vec(),
    })
}

struct Work {
    request: TextEgressRequest,
    response: mpsc::SyncSender<Result<TextEgressResponse, String>>,
}

fn worker_sender() -> Result<&'static mpsc::SyncSender<Work>, String> {
    static WORKER: OnceLock<Result<mpsc::SyncSender<Work>, String>> = OnceLock::new();
    WORKER
        .get_or_init(|| {
            let (sender, receiver) = mpsc::sync_channel::<Work>(64);
            thread::Builder::new()
                .name("hepta-egress".into())
                .spawn(move || {
                    let Ok(runtime) = tokio::runtime::Builder::new_current_thread()
                        .enable_all()
                        .build()
                    else {
                        return;
                    };
                    let clients = WorkerClients::new();
                    while let Ok(work) = receiver.recv() {
                        let result = runtime.block_on(execute_text_async(&clients, work.request));
                        let _ = work.response.send(result);
                    }
                })
                .map_err(|_| "failed to construct egress worker".to_string())?;
            Ok(sender)
        })
        .as_ref()
        .map_err(Clone::clone)
}

pub fn execute_text(request: TextEgressRequest) -> Result<TextEgressResponse, String> {
    let (sender, receiver) = mpsc::sync_channel(1);
    worker_sender()?
        .send(Work {
            request,
            response: sender,
        })
        .map_err(|_| "egress worker is unavailable".to_string())?;
    receiver
        .recv()
        .map_err(|_| "egress worker ended before returning a response".to_string())?
}

pub fn execute_json(request: JsonEgressRequest) -> Result<JsonEgressResponse, String> {
    let mut headers = Vec::new();
    if let Some(token) = request
        .bearer_token
        .filter(|token| !token.trim().is_empty())
    {
        headers.push(("authorization".into(), format!("Bearer {token}")));
    }
    let body = request
        .body
        .map(|body| serde_json::to_vec(&body).expect("serializing JSON values cannot fail"));
    if body.is_some() {
        headers.push(("content-type".into(), "application/json".into()));
    }
    let response = execute_text(TextEgressRequest {
        authorization: request.authorization,
        capability: request.capability,
        method: request.method,
        url: request.url,
        query: request.query,
        headers,
        body,
        timeout: request.timeout,
        max_response_bytes: MAX_JSON_RESPONSE_BYTES,
    })?;
    let body = serde_json::from_slice(&response.body)
        .map_err(|_| "outbound response was not valid JSON".to_string())?;
    Ok(JsonEgressResponse {
        status: response.status,
        body,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capability_destination_validation_is_fail_closed() {
        let telegram = Url::parse("https://api.telegram.org/botREDACTED/getUpdates").unwrap();
        validate_destination(OutboundHttpCapability::TelegramBotApi, &telegram).unwrap();
        assert!(
            validate_destination(
                OutboundHttpCapability::TelegramBotApi,
                &Url::parse("https://example.com/").unwrap()
            )
            .is_err()
        );
        assert!(
            validate_destination(
                OutboundHttpCapability::LiteralLoopbackProvider,
                &Url::parse("http://127.0.0.1:1234/v1/chat").unwrap()
            )
            .is_ok()
        );
        assert!(
            validate_destination(
                OutboundHttpCapability::OpenAiCodexApi,
                &Url::parse("https://chatgpt.com/backend-api/codex/responses").unwrap()
            )
            .is_ok()
        );
        assert!(
            validate_destination(
                OutboundHttpCapability::OpenAiCodexApi,
                &Url::parse("https://evil.example/codex").unwrap()
            )
            .is_err()
        );
        assert!(
            validate_destination(
                OutboundHttpCapability::DuckDuckGoHtml,
                &Url::parse("https://duckduckgo.com/html/").unwrap()
            )
            .is_ok()
        );
        assert!(
            validate_authorization(
                OutboundHttpCapability::OpenAiCodexApi,
                &EgressAuthorization::LiteralLoopback
            )
            .is_err()
        );
        assert!(
            validate_authorization(
                OutboundHttpCapability::OpenAiCodexApi,
                &EgressAuthorization::EffectBound {
                    admission_hash: format!("sha256:{}", "a".repeat(64)),
                    effect_plan_hash: format!("sha256:{}", "b".repeat(64)),
                }
            )
            .is_ok()
        );
    }
}

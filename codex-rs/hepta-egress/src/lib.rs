use std::net::IpAddr;
use std::thread;
use std::time::Duration;

use codex_http_client::ClientRouteClass;
use codex_http_client::HttpClientFactory;
use codex_http_client::OutboundProxyPolicy;
use codex_http_client::RouteAwareClientPool;
use reqwest::Url;
use serde_json::Value;

const MAX_JSON_RESPONSE_BYTES: usize = 2 * 1024 * 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutboundHttpCapability {
    TelegramBotApi,
    LiteralLoopbackProvider,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JsonMethod {
    Get,
    Post,
}

#[derive(Debug, Clone)]
pub struct JsonEgressRequest {
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
    match capability {
        OutboundHttpCapability::TelegramBotApi => {
            if url.scheme() != "https"
                || url.host_str() != Some("api.telegram.org")
                || !matches!(url.port_or_known_default(), Some(443))
            {
                return Err("Telegram egress capability denied a non-Telegram destination".into());
            }
        }
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
    }
    Ok(())
}

async fn execute_async(request: JsonEgressRequest) -> Result<JsonEgressResponse, String> {
    let mut url =
        Url::parse(&request.url).map_err(|_| "invalid outbound destination".to_string())?;
    validate_destination(request.capability, &url)?;
    if !request.query.is_empty() {
        url.query_pairs_mut().extend_pairs(&request.query);
    }
    let proxy_policy = match request.capability {
        OutboundHttpCapability::TelegramBotApi => OutboundProxyPolicy::RespectSystemProxy,
        OutboundHttpCapability::LiteralLoopbackProvider => OutboundProxyPolicy::DirectOnly,
    };
    let pool = RouteAwareClientPool::new_without_redirects_or_request_logging(
        HttpClientFactory::new(proxy_policy),
        ClientRouteClass::Api,
    );
    let mut outbound = match request.method {
        JsonMethod::Get => pool.get(url),
        JsonMethod::Post => pool.post(url),
    }
    .timeout(request.timeout);
    if let Some(token) = request
        .bearer_token
        .filter(|token| !token.trim().is_empty())
    {
        outbound = outbound.header("authorization", format!("Bearer {token}"));
    }
    if let Some(body) = request.body {
        outbound = outbound.json(&body);
    }
    let response = outbound.send().await.map_err(|error| error.to_string())?;
    let status = response.status().as_u16();
    let bytes = response.bytes().await.map_err(|error| error.to_string())?;
    if bytes.len() > MAX_JSON_RESPONSE_BYTES {
        return Err("outbound JSON response exceeded the bounded response size".into());
    }
    let body = serde_json::from_slice(&bytes)
        .map_err(|_| "outbound response was not valid JSON".to_string())?;
    Ok(JsonEgressResponse { status, body })
}

pub fn execute_json(request: JsonEgressRequest) -> Result<JsonEgressResponse, String> {
    thread::Builder::new()
        .name("hepta-egress".into())
        .spawn(move || {
            let runtime = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .map_err(|error| format!("failed to construct egress runtime: {error}"))?;
            runtime.block_on(execute_async(request))
        })
        .map_err(|error| format!("failed to construct egress worker: {error}"))?
        .join()
        .map_err(|_| "egress worker panicked".to_string())?
}

#[cfg(test)]
mod tests {
    use super::*;

    fn request(capability: OutboundHttpCapability, url: &str) -> JsonEgressRequest {
        JsonEgressRequest {
            capability,
            method: JsonMethod::Post,
            url: url.into(),
            query: Vec::new(),
            bearer_token: Some("secret".into()),
            body: Some(serde_json::json!({})),
            timeout: Duration::from_millis(20),
        }
    }

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
            execute_json(request(
                OutboundHttpCapability::LiteralLoopbackProvider,
                "http://example.com/v1/chat"
            ))
            .is_err()
        );
    }
}

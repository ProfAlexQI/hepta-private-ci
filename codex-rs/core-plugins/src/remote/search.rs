use super::RemotePluginCatalogError;
use super::RemotePluginListResponse;
use super::RemotePluginScope;
use super::RemotePluginServiceConfig;
use super::RemotePluginSummary;
use super::build_remote_plugin_summary;
use super::ensure_chatgpt_auth;
use codex_login::CodexAuth;
use codex_login::default_client::default_headers;
use hepta_egress::EgressAuthorization;
use hepta_egress::EgressMethod;
use hepta_egress::OutboundHttpCapability;
use hepta_egress::TextEgressRequest;
use hepta_egress::TextEgressResponse;
use http::StatusCode;
use serde_json::Value;
use std::fmt;
use std::time::Duration;
use tracing::instrument;
use url::Url;

pub const DEFAULT_PLUGIN_SEARCH_LIMIT: u32 = 16;
pub const MAX_PLUGIN_SEARCH_LIMIT: u32 = 50;
pub const MAX_PLUGIN_SEARCH_TERM_BYTES: usize = 512;
pub const MAX_PLUGIN_SEARCH_TERM_CHARS: usize = 256;
pub const MAX_PLUGIN_SEARCH_CURSOR_BYTES: usize = 2 * 1024;
pub const MAX_PLUGIN_SEARCH_CURSOR_CHARS: usize = 1024;
pub const MAX_PLUGIN_SEARCH_RESPONSE_BYTES: usize = 512 * 1024;

const PLUGIN_SEARCH_TIMEOUT: Duration = Duration::from_secs(15);
const MAX_PLUGIN_SEARCH_RESPONSE_TEXT_BYTES: usize = 256 * 1024;
const MAX_PLUGIN_SEARCH_RESPONSE_TEXT_CHARS: usize = 128 * 1024;
const MAX_PLUGIN_SEARCH_TEXT_FIELD_BYTES: usize = 8 * 1024;
const MAX_PLUGIN_SEARCH_TEXT_FIELD_CHARS: usize = 4 * 1024;
const MAX_PLUGIN_SEARCH_JSON_DEPTH: usize = 32;
const MAX_PLUGIN_SEARCH_JSON_NODES: usize = 16 * 1024;

/// Search parameters forwarded to plugin-service after app-server validation.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct RemotePluginSearchRequest<'a> {
    pub query: &'a str,
    pub scope: Option<RemotePluginScope>,
    pub limit: u32,
    pub page_token: Option<&'a str>,
}

impl fmt::Debug for RemotePluginSearchRequest<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RemotePluginSearchRequest")
            .field("query", &"<redacted>")
            .field("scope", &self.scope)
            .field("limit", &self.limit)
            .field("page_token_present", &self.page_token.is_some())
            .finish()
    }
}

/// One uncached, read-only page of remote plugin search results.
#[derive(Debug, Clone, PartialEq)]
pub struct RemotePluginSearchPage {
    pub plugins: Vec<RemotePluginSummary>,
    pub next_page_token: Option<String>,
}

/// Searches plugin-service through Hepta's effect-bound egress capability.
///
/// This function never reads or populates the remote catalog cache and has no install,
/// activation, or mutation authority.
#[instrument(
    level = "debug",
    skip_all,
    fields(plugin.scope = ?search.scope, plugin.limit = search.limit)
)]
pub async fn search_remote_plugins(
    config: &RemotePluginServiceConfig,
    auth: Option<&CodexAuth>,
    authorization: EgressAuthorization,
    search: RemotePluginSearchRequest<'_>,
) -> Result<RemotePluginSearchPage, RemotePluginCatalogError> {
    let request = build_remote_plugin_search_request(config, auth, authorization, search)?;
    let url_for_error = request.url.clone();
    let response = tokio::task::spawn_blocking(move || hepta_egress::execute_text(request))
        .await
        .map_err(|_| {
            RemotePluginCatalogError::GovernedEgress("egress worker did not complete".into())
        })?
        .map_err(RemotePluginCatalogError::GovernedEgress)?;
    decode_remote_plugin_search_response(&url_for_error, search.limit, response)
}

fn build_remote_plugin_search_request(
    config: &RemotePluginServiceConfig,
    auth: Option<&CodexAuth>,
    authorization: EgressAuthorization,
    search: RemotePluginSearchRequest<'_>,
) -> Result<TextEgressRequest, RemotePluginCatalogError> {
    let auth = ensure_chatgpt_auth(auth)?;
    validate_search_request(search)?;

    let mut url = Url::parse(config.chatgpt_base_url.trim_end_matches('/'))
        .map_err(RemotePluginCatalogError::InvalidBaseUrl)?;
    if url.query().is_some() || url.fragment().is_some() {
        return Err(RemotePluginCatalogError::InvalidBaseUrlPath);
    }
    {
        let mut segments = url
            .path_segments_mut()
            .map_err(|()| RemotePluginCatalogError::InvalidBaseUrlPath)?;
        segments.pop_if_empty();
        segments.push("ps");
        segments.push("plugins");
        segments.push("search");
    }

    let mut query = vec![
        ("q".to_string(), search.query.to_string()),
        ("limit".to_string(), search.limit.to_string()),
    ];
    if let Some(scope) = search.scope {
        query.push(("scope".to_string(), scope.api_value().to_string()));
    }
    if let Some(page_token) = search.page_token {
        query.push(("pageToken".to_string(), page_token.to_string()));
    }

    let mut headers = default_headers();
    headers.extend(codex_model_provider::auth_provider_from_auth(auth).to_auth_headers());
    let headers = headers
        .iter()
        .map(|(name, value)| {
            value
                .to_str()
                .map(|value| (name.as_str().to_string(), value.to_string()))
                .map_err(|_| {
                    RemotePluginCatalogError::UnexpectedResponse(
                        "remote plugin search auth headers were not valid text".into(),
                    )
                })
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(TextEgressRequest {
        authorization,
        capability: OutboundHttpCapability::OpenAiCodexApi,
        method: EgressMethod::Get,
        url: url.to_string(),
        query,
        headers,
        body: None,
        timeout: PLUGIN_SEARCH_TIMEOUT,
        max_response_bytes: MAX_PLUGIN_SEARCH_RESPONSE_BYTES,
    })
}

fn validate_search_request(
    search: RemotePluginSearchRequest<'_>,
) -> Result<(), RemotePluginCatalogError> {
    let query_len = search.query.len();
    if search.query.trim().is_empty()
        || query_len > MAX_PLUGIN_SEARCH_TERM_BYTES
        || search.query.chars().count() > MAX_PLUGIN_SEARCH_TERM_CHARS
    {
        return Err(RemotePluginCatalogError::UnexpectedResponse(
            "remote plugin search term violated its non-empty text bound".into(),
        ));
    }
    if !(1..=MAX_PLUGIN_SEARCH_LIMIT).contains(&search.limit) {
        return Err(RemotePluginCatalogError::UnexpectedResponse(
            "remote plugin search limit violated its count bound".into(),
        ));
    }
    if search.page_token.is_some_and(|token| {
        token.is_empty()
            || token.len() > MAX_PLUGIN_SEARCH_CURSOR_BYTES
            || token.chars().count() > MAX_PLUGIN_SEARCH_CURSOR_CHARS
    }) {
        return Err(RemotePluginCatalogError::UnexpectedResponse(
            "remote plugin search cursor violated its text bound".into(),
        ));
    }
    Ok(())
}

fn decode_remote_plugin_search_response(
    url_for_error: &str,
    requested_limit: u32,
    response: TextEgressResponse,
) -> Result<RemotePluginSearchPage, RemotePluginCatalogError> {
    let status = StatusCode::from_u16(response.status).map_err(|_| {
        RemotePluginCatalogError::UnexpectedResponse(
            "remote plugin search returned an invalid HTTP status".into(),
        )
    })?;
    if !status.is_success() {
        return Err(RemotePluginCatalogError::UnexpectedStatus {
            url: url_for_error.to_string(),
            status,
            body: "remote plugin search response body redacted".into(),
        });
    }
    if response.body.len() > MAX_PLUGIN_SEARCH_RESPONSE_BYTES {
        return Err(RemotePluginCatalogError::SearchResponseBound("byte"));
    }

    let value: Value = serde_json::from_slice(&response.body).map_err(|source| {
        RemotePluginCatalogError::Decode {
            url: url_for_error.to_string(),
            source,
        }
    })?;
    validate_response_json_shape(&value)?;
    let response: RemotePluginListResponse =
        serde_json::from_value(value).map_err(|source| RemotePluginCatalogError::Decode {
            url: url_for_error.to_string(),
            source,
        })?;
    let result_limit = usize::try_from(requested_limit)
        .unwrap_or(usize::MAX)
        .min(MAX_PLUGIN_SEARCH_LIMIT as usize);
    if response.plugins.len() > result_limit {
        return Err(RemotePluginCatalogError::SearchResponseBound(
            "result count",
        ));
    }
    if response
        .pagination
        .next_page_token
        .as_ref()
        .is_some_and(|token| {
            token.is_empty()
                || token.len() > MAX_PLUGIN_SEARCH_CURSOR_BYTES
                || token.chars().count() > MAX_PLUGIN_SEARCH_CURSOR_CHARS
        })
    {
        return Err(RemotePluginCatalogError::SearchResponseBound("cursor text"));
    }

    let plugins = response
        .plugins
        .iter()
        .map(|plugin| build_remote_plugin_summary(plugin, /*installed_plugin*/ None))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(RemotePluginSearchPage {
        plugins,
        next_page_token: response.pagination.next_page_token,
    })
}

fn validate_response_json_shape(value: &Value) -> Result<(), RemotePluginCatalogError> {
    fn visit(
        value: &Value,
        depth: usize,
        nodes: &mut usize,
        text_bytes: &mut usize,
        text_chars: &mut usize,
    ) -> Result<(), RemotePluginCatalogError> {
        if depth > MAX_PLUGIN_SEARCH_JSON_DEPTH {
            return Err(RemotePluginCatalogError::SearchResponseBound("JSON depth"));
        }
        *nodes = nodes.saturating_add(1);
        if *nodes > MAX_PLUGIN_SEARCH_JSON_NODES {
            return Err(RemotePluginCatalogError::SearchResponseBound(
                "JSON node count",
            ));
        }
        match value {
            Value::String(text) => {
                let chars = text.chars().count();
                if text.len() > MAX_PLUGIN_SEARCH_TEXT_FIELD_BYTES
                    || chars > MAX_PLUGIN_SEARCH_TEXT_FIELD_CHARS
                {
                    return Err(RemotePluginCatalogError::SearchResponseBound(
                        "individual text field",
                    ));
                }
                *text_bytes = text_bytes.saturating_add(text.len());
                *text_chars = text_chars.saturating_add(chars);
                if *text_bytes > MAX_PLUGIN_SEARCH_RESPONSE_TEXT_BYTES
                    || *text_chars > MAX_PLUGIN_SEARCH_RESPONSE_TEXT_CHARS
                {
                    return Err(RemotePluginCatalogError::SearchResponseBound(
                        "aggregate text",
                    ));
                }
            }
            Value::Array(values) => {
                for value in values {
                    visit(value, depth + 1, nodes, text_bytes, text_chars)?;
                }
            }
            Value::Object(values) => {
                for (key, value) in values {
                    let key_chars = key.chars().count();
                    if key.len() > MAX_PLUGIN_SEARCH_TEXT_FIELD_BYTES
                        || key_chars > MAX_PLUGIN_SEARCH_TEXT_FIELD_CHARS
                    {
                        return Err(RemotePluginCatalogError::SearchResponseBound(
                            "object key text",
                        ));
                    }
                    *text_bytes = text_bytes.saturating_add(key.len());
                    *text_chars = text_chars.saturating_add(key_chars);
                    if *text_bytes > MAX_PLUGIN_SEARCH_RESPONSE_TEXT_BYTES
                        || *text_chars > MAX_PLUGIN_SEARCH_RESPONSE_TEXT_CHARS
                    {
                        return Err(RemotePluginCatalogError::SearchResponseBound(
                            "aggregate text",
                        ));
                    }
                    visit(value, depth + 1, nodes, text_bytes, text_chars)?;
                }
            }
            Value::Null | Value::Bool(_) | Value::Number(_) => {}
        }
        Ok(())
    }

    visit(value, 0, &mut 0, &mut 0, &mut 0)
}

#[cfg(test)]
#[path = "search_tests.rs"]
mod tests;

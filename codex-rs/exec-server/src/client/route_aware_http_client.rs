//! Route-aware local HTTP capability implementation.
//!
//! The executor-facing [`crate::HttpClient`] abstraction must use the same
//! outbound proxy and custom-CA policy as the rest of Codex. Keeping the
//! route-aware pool behind this adapter also prevents MCP OAuth and transport
//! callers from silently falling back to an unrelated `reqwest::Client`.

use std::time::Duration;

use codex_app_server_protocol::JSONRPCErrorError;
use codex_http_client::ClientRouteClass;
use codex_http_client::HttpClientFactory;
use codex_http_client::RouteAwareClientPool;
use futures::FutureExt;
use futures::future::BoxFuture;
use http::HeaderMap;
use http::HeaderName;
use http::HeaderValue;
use http::Method;
use url::Url;

use super::HttpResponseBodyStream;
use crate::HttpClient;
use crate::HttpRedirectPolicy;
use crate::client::ExecServerError;
use crate::protocol::HttpHeader;
use crate::protocol::HttpRequestParams;
use crate::protocol::HttpRequestResponse;
use crate::rpc::internal_error;
use crate::rpc::invalid_params;

/// HTTP capability backed by the shared route-aware transport.
#[derive(Clone)]
pub struct RouteAwareHttpClient {
    follow_redirects: RouteAwareClientPool,
    stop_redirects: RouteAwareClientPool,
}

pub(crate) struct PendingRouteAwareHttpBodyStream {
    pub(crate) response: codex_http_client::HttpResponse,
}

struct RouteAwareHttpRequestRunner {
    client: RouteAwareClientPool,
}

impl RouteAwareHttpClient {
    pub fn new(http_client_factory: HttpClientFactory) -> Self {
        Self {
            follow_redirects:
                RouteAwareClientPool::with_chatgpt_cloudflare_cookies_without_request_logging(
                    http_client_factory.clone(),
                    // Proxy routing comes from the factory. The class only labels diagnostics.
                    ClientRouteClass::Other,
                ),
            stop_redirects:
                RouteAwareClientPool::with_chatgpt_cloudflare_cookies_without_redirects_or_request_logging(
                    http_client_factory,
                    ClientRouteClass::Other,
                ),
        }
    }

    fn runner(&self, redirect_policy: HttpRedirectPolicy) -> RouteAwareHttpRequestRunner {
        RouteAwareHttpRequestRunner {
            client: match redirect_policy {
                HttpRedirectPolicy::Follow => self.follow_redirects.clone(),
                HttpRedirectPolicy::Stop => self.stop_redirects.clone(),
            },
        }
    }
}

impl HttpClient for RouteAwareHttpClient {
    fn http_request(
        &self,
        params: HttpRequestParams,
    ) -> BoxFuture<'_, Result<HttpRequestResponse, ExecServerError>> {
        async move {
            let (response, _) = self
                .runner(HttpRedirectPolicy::Follow)
                .run(HttpRequestParams {
                    stream_response: false,
                    ..params
                })
                .await
                .map_err(|error| ExecServerError::HttpRequest(error.message))?;
            Ok(response)
        }
        .boxed()
    }

    fn http_request_stream(
        &self,
        params: HttpRequestParams,
    ) -> BoxFuture<'_, Result<(HttpRequestResponse, HttpResponseBodyStream), ExecServerError>> {
        async move {
            let (response, pending_stream) = self
                .runner(HttpRedirectPolicy::Follow)
                .run(HttpRequestParams {
                    stream_response: true,
                    ..params
                })
                .await
                .map_err(|error| ExecServerError::HttpRequest(error.message))?;
            let pending_stream = pending_stream.ok_or_else(|| {
                ExecServerError::Protocol(
                    "http request stream did not return a response body stream".to_string(),
                )
            })?;
            Ok((
                response,
                HttpResponseBodyStream::local(pending_stream.response),
            ))
        }
        .boxed()
    }

    fn http_request_stream_with_redirect_policy(
        &self,
        params: HttpRequestParams,
        redirect_policy: HttpRedirectPolicy,
    ) -> BoxFuture<'_, Result<(HttpRequestResponse, HttpResponseBodyStream), ExecServerError>> {
        async move {
            let (response, pending_stream) = self
                .runner(redirect_policy)
                .run(HttpRequestParams {
                    stream_response: true,
                    ..params
                })
                .await
                .map_err(|error| ExecServerError::HttpRequest(error.message))?;
            let pending_stream = pending_stream.ok_or_else(|| {
                ExecServerError::Protocol(
                    "http request stream did not return a response body stream".to_string(),
                )
            })?;
            Ok((
                response,
                HttpResponseBodyStream::local(pending_stream.response),
            ))
        }
        .boxed()
    }
}

impl RouteAwareHttpRequestRunner {
    async fn run(
        &self,
        params: HttpRequestParams,
    ) -> Result<(HttpRequestResponse, Option<PendingRouteAwareHttpBodyStream>), JSONRPCErrorError>
    {
        let method = Method::from_bytes(params.method.as_bytes())
            .map_err(|error| invalid_params(format!("http/request method is invalid: {error}")))?;
        let url = Url::parse(&params.url)
            .map_err(|error| invalid_params(format!("http/request url is invalid: {error}")))?;
        match url.scheme() {
            "http" | "https" => {}
            scheme => {
                return Err(invalid_params(format!(
                    "http/request only supports http and https URLs, got {scheme}"
                )));
            }
        }

        let headers = Self::build_headers(params.headers)?;
        let mut request = self.client.request(method, url).headers(headers);
        if let Some(body) = params.body {
            request = request.body(body.into_inner());
        }
        if let Some(timeout_ms) = params.timeout_ms {
            request = request.timeout(Duration::from_millis(timeout_ms));
        }

        let response = request
            .send()
            .await
            .map_err(|error| internal_error(format!("http/request failed: {error}")))?;
        let status = response.status().as_u16();
        let headers = Self::response_headers(response.headers());

        if params.stream_response {
            return Ok((
                HttpRequestResponse {
                    status,
                    headers,
                    body: Vec::new().into(),
                },
                Some(PendingRouteAwareHttpBodyStream { response }),
            ));
        }

        let body = response.bytes().await.map_err(|error| {
            internal_error(format!(
                "failed to read http/request response body: {error}"
            ))
        })?;

        Ok((
            HttpRequestResponse {
                status,
                headers,
                body: body.to_vec().into(),
            },
            None,
        ))
    }

    fn build_headers(headers: Vec<HttpHeader>) -> Result<HeaderMap, JSONRPCErrorError> {
        let mut header_map = HeaderMap::new();
        for header in headers {
            let name = HeaderName::from_bytes(header.name.as_bytes()).map_err(|error| {
                invalid_params(format!("http/request header name is invalid: {error}"))
            })?;
            let value = HeaderValue::from_str(&header.value).map_err(|error| {
                invalid_params(format!(
                    "http/request header value is invalid for {}: {error}",
                    header.name
                ))
            })?;
            header_map.append(name, value);
        }
        Ok(header_map)
    }

    fn response_headers(headers: &HeaderMap) -> Vec<HttpHeader> {
        headers
            .iter()
            .filter_map(|(name, value)| {
                Some(HttpHeader {
                    name: name.as_str().to_string(),
                    value: value.to_str().ok()?.to_string(),
                })
            })
            .collect()
    }
}

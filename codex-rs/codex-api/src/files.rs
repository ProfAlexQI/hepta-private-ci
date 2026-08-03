use std::time::Duration;

use crate::AuthProvider;
use bytes::Bytes;
use codex_http_client::RouteAwareClientPool;
use codex_http_client::RouteAwareRequestBuilder;
use codex_http_client::RouteAwareRequestError;
use futures::Stream;
use http::Method;
use http::StatusCode;
use http::header::CONTENT_LENGTH;
use serde::Deserialize;
use tokio::time::Instant;

pub const OPENAI_FILE_URI_PREFIX: &str = "sediment://";
pub const OPENAI_FILE_UPLOAD_LIMIT_BYTES: u64 = 512 * 1024 * 1024;

const OPENAI_FILE_REQUEST_TIMEOUT: Duration = Duration::from_secs(60);
const OPENAI_FILE_FINALIZE_TIMEOUT: Duration = Duration::from_secs(30);
const OPENAI_FILE_FINALIZE_RETRY_DELAY: Duration = Duration::from_millis(250);
const OPENAI_FILE_USE_CASE: &str = "codex";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UploadedOpenAiFile {
    pub file_id: String,
    pub uri: String,
    pub download_url: String,
    pub file_name: String,
    pub file_size_bytes: u64,
    pub mime_type: Option<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum OpenAiFileError {
    #[error(
        "file `{file_name}` is too large: {size_bytes} bytes exceeds the limit of {limit_bytes} bytes"
    )]
    FileTooLarge {
        file_name: String,
        size_bytes: u64,
        limit_bytes: u64,
    },
    #[error("failed to send OpenAI file request to {url}: {source}")]
    Request {
        url: String,
        #[source]
        source: RouteAwareRequestError,
    },
    #[error("OpenAI file request to {url} failed with status {status}: {body}")]
    UnexpectedStatus {
        url: String,
        status: StatusCode,
        body: String,
    },
    #[error("failed to parse OpenAI file response from {url}: {source}")]
    Decode {
        url: String,
        #[source]
        source: serde_json::Error,
    },
    #[error("OpenAI file upload for `{file_id}` is not ready yet")]
    UploadNotReady { file_id: String },
    #[error("OpenAI file upload for `{file_id}` failed: {message}")]
    UploadFailed { file_id: String, message: String },
}

#[derive(Deserialize)]
struct CreateFileResponse {
    file_id: String,
    upload_url: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
struct DownloadLinkResponse {
    status: String,
    download_url: Option<String>,
    file_name: Option<String>,
    mime_type: Option<String>,
    error_message: Option<String>,
}

pub fn openai_file_uri(file_id: &str) -> String {
    format!("{OPENAI_FILE_URI_PREFIX}{file_id}")
}

pub async fn upload_openai_file(
    base_url: &str,
    auth: &dyn AuthProvider,
    client_pool: &RouteAwareClientPool,
    file_name: String,
    file_size_bytes: u64,
    contents: impl Stream<Item = std::io::Result<Bytes>> + Send + 'static,
) -> Result<UploadedOpenAiFile, OpenAiFileError> {
    if file_size_bytes > OPENAI_FILE_UPLOAD_LIMIT_BYTES {
        return Err(OpenAiFileError::FileTooLarge {
            file_name,
            size_bytes: file_size_bytes,
            limit_bytes: OPENAI_FILE_UPLOAD_LIMIT_BYTES,
        });
    }

    let create_url = format!("{}/files", base_url.trim_end_matches('/'));
    let create_response = authorized_request(client_pool, auth, Method::POST, &create_url)
        .json(&serde_json::json!({
            "file_name": file_name.as_str(),
            "file_size": file_size_bytes,
            "use_case": OPENAI_FILE_USE_CASE,
        }))
        .send()
        .await
        .map_err(|source| OpenAiFileError::Request {
            url: create_url.clone(),
            source,
        })?;
    let create_status = create_response.status();
    let create_body = create_response.text().await.unwrap_or_default();
    if !create_status.is_success() {
        return Err(OpenAiFileError::UnexpectedStatus {
            url: create_url,
            status: create_status,
            body: create_body,
        });
    }
    let create_payload: CreateFileResponse =
        serde_json::from_str(&create_body).map_err(|source| OpenAiFileError::Decode {
            url: create_url.clone(),
            source,
        })?;

    let upload_response = client_pool
        .put(&create_payload.upload_url)
        .timeout(OPENAI_FILE_REQUEST_TIMEOUT)
        .header("x-ms-blob-type", "BlockBlob")
        .header(CONTENT_LENGTH, file_size_bytes)
        .body_stream(contents)
        .send()
        .await
        .map_err(|source| OpenAiFileError::Request {
            url: create_payload.upload_url.clone(),
            source,
        })?;
    let upload_status = upload_response.status();
    let upload_body = upload_response.text().await.unwrap_or_default();
    if !upload_status.is_success() {
        return Err(OpenAiFileError::UnexpectedStatus {
            url: create_payload.upload_url.clone(),
            status: upload_status,
            body: upload_body,
        });
    }

    let finalize_url = format!(
        "{}/files/{}/uploaded",
        base_url.trim_end_matches('/'),
        create_payload.file_id,
    );
    let finalize_started_at = Instant::now();
    loop {
        let finalize_response = authorized_request(client_pool, auth, Method::POST, &finalize_url)
            .json(&serde_json::json!({}))
            .send()
            .await
            .map_err(|source| OpenAiFileError::Request {
                url: finalize_url.clone(),
                source,
            })?;
        let finalize_status = finalize_response.status();
        let finalize_body = finalize_response.text().await.unwrap_or_default();
        if !finalize_status.is_success() {
            return Err(OpenAiFileError::UnexpectedStatus {
                url: finalize_url.clone(),
                status: finalize_status,
                body: finalize_body,
            });
        }
        let finalize_payload: DownloadLinkResponse =
            serde_json::from_str(&finalize_body).map_err(|source| OpenAiFileError::Decode {
                url: finalize_url.clone(),
                source,
            })?;

        match finalize_payload.status.as_str() {
            "success" => {
                return Ok(UploadedOpenAiFile {
                    file_id: create_payload.file_id.clone(),
                    uri: openai_file_uri(&create_payload.file_id),
                    download_url: finalize_payload.download_url.ok_or_else(|| {
                        OpenAiFileError::UploadFailed {
                            file_id: create_payload.file_id.clone(),
                            message: "missing download_url".to_string(),
                        }
                    })?,
                    file_name: finalize_payload.file_name.unwrap_or(file_name),
                    file_size_bytes,
                    mime_type: finalize_payload.mime_type,
                });
            }
            "retry" => {
                if finalize_started_at.elapsed() >= OPENAI_FILE_FINALIZE_TIMEOUT {
                    return Err(OpenAiFileError::UploadNotReady {
                        file_id: create_payload.file_id,
                    });
                }
                tokio::time::sleep(OPENAI_FILE_FINALIZE_RETRY_DELAY).await;
            }
            _ => {
                return Err(OpenAiFileError::UploadFailed {
                    file_id: create_payload.file_id,
                    message: finalize_payload
                        .error_message
                        .unwrap_or_else(|| "upload finalization returned an error".to_string()),
                });
            }
        }
    }
}

fn authorized_request(
    client_pool: &RouteAwareClientPool,
    auth: &dyn AuthProvider,
    method: Method,
    url: &str,
) -> RouteAwareRequestBuilder {
    let mut headers = http::HeaderMap::new();
    auth.add_auth_headers(&mut headers);

    client_pool
        .request(method, url)
        .timeout(OPENAI_FILE_REQUEST_TIMEOUT)
        .headers(headers)
}

#[cfg(test)]
mod tests {
    use super::*;
    use codex_http_client::ClientRouteClass;
    use codex_http_client::HttpClientFactory;
    use codex_http_client::OutboundProxyPolicy;
    use http::HeaderMap;
    use http::header::AUTHORIZATION;
    use http::header::HeaderValue;
    use pretty_assertions::assert_eq;
    use std::io::Read;
    use std::io::Write;
    use std::net::SocketAddr;
    use std::net::TcpListener;
    use std::sync::Arc;
    use std::sync::atomic::AtomicUsize;
    use std::sync::atomic::Ordering;
    use std::thread::JoinHandle;
    use wiremock::Mock;
    use wiremock::MockServer;
    use wiremock::Request;
    use wiremock::ResponseTemplate;
    use wiremock::matchers::body_json;
    use wiremock::matchers::header;
    use wiremock::matchers::method;
    use wiremock::matchers::path;

    #[derive(Clone, Copy)]
    struct ChatGptTestAuth;

    impl AuthProvider for ChatGptTestAuth {
        fn add_auth_headers(&self, headers: &mut HeaderMap) {
            headers.insert(AUTHORIZATION, HeaderValue::from_static("Bearer token"));
            headers.insert("ChatGPT-Account-ID", HeaderValue::from_static("account_id"));
        }
    }

    fn chatgpt_auth() -> ChatGptTestAuth {
        ChatGptTestAuth
    }

    fn base_url_for(server: &MockServer) -> String {
        format!("{}/backend-api", server.uri())
    }

    fn route_aware_client_pool() -> RouteAwareClientPool {
        RouteAwareClientPool::new_without_request_logging(
            HttpClientFactory::new(OutboundProxyPolicy::ReqwestDefault),
            ClientRouteClass::Api,
        )
        .with_legacy_custom_ca_fallback()
    }

    struct NoDirectEgressProxy {
        base_url: String,
        upload_url: String,
        address: SocketAddr,
        thread: JoinHandle<Vec<String>>,
    }

    fn spawn_no_direct_egress_proxy() -> NoDirectEgressProxy {
        let listener = TcpListener::bind(("127.0.0.1", 0)).expect("proxy listener should bind");
        let address = listener
            .local_addr()
            .expect("proxy listener should have an address");
        listener
            .set_nonblocking(true)
            .expect("proxy listener should become nonblocking");

        let nonce = address.port();
        let base_url = format!("http://files-{nonce}.hepta.invalid/backend-api");
        let upload_url = format!("http://blob-{nonce}.hepta.invalid/upload/file_proxy");
        let create_url = format!("{base_url}/files");
        let finalize_url = format!("{base_url}/files/file_proxy/uploaded");
        let expected_request_lines = [
            format!("POST {create_url} HTTP/1.1"),
            format!("PUT {upload_url} HTTP/1.1"),
            format!("POST {finalize_url} HTTP/1.1"),
        ];
        let create_body = serde_json::json!({
            "file_id": "file_proxy",
            "upload_url": upload_url,
        })
        .to_string();
        let finalize_body = serde_json::json!({
            "status": "success",
            "download_url": "https://download.example/file_proxy",
            "file_name": "proxy.txt",
            "mime_type": "text/plain",
        })
        .to_string();
        let thread = std::thread::spawn(move || {
            let mut requests = Vec::new();
            for (index, expected_request_line) in expected_request_lines.into_iter().enumerate() {
                let deadline = std::time::Instant::now() + Duration::from_secs(5);
                let (mut stream, _) = loop {
                    match listener.accept() {
                        Ok(connection) => break connection,
                        Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                            assert!(
                                std::time::Instant::now() < deadline,
                                "proxy should receive request {}",
                                index + 1
                            );
                            std::thread::sleep(Duration::from_millis(10));
                        }
                        Err(error) => panic!("proxy listener should accept: {error}"),
                    }
                };
                // Windows inherits the listener's nonblocking mode on the accepted socket.
                // Return the connected socket to blocking mode before applying the bounded
                // read timeout so the first read cannot spuriously fail with WSAEWOULDBLOCK.
                stream
                    .set_nonblocking(false)
                    .expect("accepted proxy stream should become blocking");
                stream
                    .set_read_timeout(Some(Duration::from_secs(5)))
                    .expect("proxy stream should get a read timeout");
                let request = read_proxy_http_message(&mut stream);
                assert_eq!(
                    request.lines().next(),
                    Some(expected_request_line.as_str()),
                    "request must use HTTP proxy absolute-form"
                );
                let response_body = match index {
                    0 => create_body.as_str(),
                    1 => "",
                    2 => finalize_body.as_str(),
                    _ => unreachable!("exactly three file provider requests are expected"),
                };
                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{response_body}",
                    response_body.len()
                );
                stream
                    .write_all(response.as_bytes())
                    .expect("proxy should write response");
                requests.push(request);
            }
            requests
        });

        NoDirectEgressProxy {
            base_url,
            upload_url,
            address,
            thread,
        }
    }

    fn read_proxy_http_message(stream: &mut impl Read) -> String {
        let mut buffer = Vec::new();
        let mut chunk = [0_u8; 4096];
        loop {
            let bytes_read = stream.read(&mut chunk).expect("proxy request should read");
            if bytes_read == 0 {
                break;
            }
            buffer.extend_from_slice(&chunk[..bytes_read]);
            let Some(header_end) = buffer.windows(4).position(|window| window == b"\r\n\r\n")
            else {
                continue;
            };
            let body_start = header_end + 4;
            let headers = String::from_utf8_lossy(&buffer[..body_start]);
            let content_length = headers
                .lines()
                .filter_map(|line| line.split_once(':'))
                .find_map(|(name, value)| {
                    name.eq_ignore_ascii_case("content-length")
                        .then(|| value.trim().parse::<usize>().ok())
                        .flatten()
                })
                .unwrap_or(0);
            if buffer.len() >= body_start + content_length {
                break;
            }
        }
        String::from_utf8_lossy(&buffer).into_owned()
    }

    #[tokio::test]
    async fn upload_openai_file_returns_canonical_uri() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/backend-api/files"))
            .and(header("chatgpt-account-id", "account_id"))
            .and(body_json(serde_json::json!({
                "file_name": "hello.txt",
                "file_size": 5,
                "use_case": "codex",
            })))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(serde_json::json!({"file_id": "file_123", "upload_url": format!("{}/upload/file_123", server.uri())})),
            )
            .mount(&server)
            .await;
        Mock::given(method("PUT"))
            .and(path("/upload/file_123"))
            .and(header("content-length", "5"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&server)
            .await;
        let finalize_attempts = Arc::new(AtomicUsize::new(0));
        let finalize_attempts_responder = Arc::clone(&finalize_attempts);
        let download_url = format!("{}/download/file_123", server.uri());
        Mock::given(method("POST"))
            .and(path("/backend-api/files/file_123/uploaded"))
            .respond_with(move |_request: &Request| {
                if finalize_attempts_responder.fetch_add(1, Ordering::SeqCst) == 0 {
                    return ResponseTemplate::new(200).set_body_json(serde_json::json!({
                        "status": "retry"
                    }));
                }

                ResponseTemplate::new(200).set_body_json(serde_json::json!({
                    "status": "success",
                    "download_url": download_url,
                    "file_name": "hello.txt",
                    "mime_type": "text/plain",
                    "file_size_bytes": 5
                }))
            })
            .mount(&server)
            .await;

        let base_url = base_url_for(&server);
        let contents = futures::stream::once(async {
            Ok::<Bytes, std::io::Error>(Bytes::from_static(b"hello"))
        });
        let uploaded = upload_openai_file(
            &base_url,
            &chatgpt_auth(),
            &route_aware_client_pool(),
            "hello.txt".to_string(),
            5,
            contents,
        )
        .await
        .expect("upload succeeds");

        assert_eq!(uploaded.file_id, "file_123");
        assert_eq!(uploaded.uri, "sediment://file_123");
        assert_eq!(
            uploaded.download_url,
            format!("{}/download/file_123", server.uri())
        );
        assert_eq!(uploaded.file_name, "hello.txt");
        assert_eq!(uploaded.mime_type, Some("text/plain".to_string()));
        assert_eq!(finalize_attempts.load(Ordering::SeqCst), 2);
    }

    #[tokio::test]
    async fn upload_openai_file_has_no_direct_egress_bypass() {
        let proxy = spawn_no_direct_egress_proxy();
        let proxy_url = format!("http://{}", proxy.address);
        let create_url = format!("{}/files", proxy.base_url);
        let finalize_url = format!("{}/files/file_proxy/uploaded", proxy.base_url);
        for request_url in [&create_url, &proxy.upload_url, &finalize_url] {
            codex_http_client::cache_system_proxy_route_for_test(request_url, proxy_url.clone());
        }
        let pool = RouteAwareClientPool::new_without_request_logging(
            HttpClientFactory::new(OutboundProxyPolicy::RespectSystemProxy),
            ClientRouteClass::Api,
        );
        let contents = futures::stream::once(async {
            Ok::<Bytes, std::io::Error>(Bytes::from_static(b"proxy"))
        });

        let uploaded = tokio::time::timeout(
            Duration::from_secs(5),
            upload_openai_file(
                &proxy.base_url,
                &chatgpt_auth(),
                &pool,
                "proxy.txt".to_string(),
                5,
                contents,
            ),
        )
        .await
        .expect("file upload should complete through the configured proxy")
        .expect("file upload should succeed without direct DNS or egress");

        assert_eq!(uploaded.file_id, "file_proxy");
        assert_eq!(uploaded.uri, "sediment://file_proxy");
        let requests = proxy
            .thread
            .join()
            .expect("proxy should observe all file provider requests");
        assert_eq!(requests.len(), 3);
        assert_eq!(
            requests
                .iter()
                .filter_map(|request| request.lines().next())
                .collect::<Vec<_>>(),
            vec![
                format!("POST {create_url} HTTP/1.1"),
                format!("PUT {} HTTP/1.1", proxy.upload_url),
                format!("POST {finalize_url} HTTP/1.1"),
            ]
        );
    }
}

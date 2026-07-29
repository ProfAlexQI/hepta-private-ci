use super::*;
use codex_app_server_protocol::PluginAuthPolicy;
use codex_app_server_protocol::PluginInstallPolicy;
use codex_app_server_protocol::PluginInterface;
use codex_login::CodexAuth;
use codex_utils_absolute_path::AbsolutePathBuf;
use pretty_assertions::assert_eq;
use serde_json::json;
use std::collections::BTreeMap;
use std::fs;
use std::io::Read;
use std::io::Write;
use std::net::SocketAddr;
use std::net::TcpListener;
use std::path::Path;
use std::path::PathBuf;
use std::thread::JoinHandle;
use tempfile::TempDir;
use wiremock::Mock;
use wiremock::MockServer;
use wiremock::ResponseTemplate;
use wiremock::matchers::body_json;
use wiremock::matchers::header;
use wiremock::matchers::method;
use wiremock::matchers::path;
use wiremock::matchers::query_param;
use wiremock::matchers::query_param_is_missing;

fn test_config(server: &MockServer) -> RemotePluginServiceConfig {
    RemotePluginServiceConfig::new(
        format!("{}/backend-api", server.uri()),
        codex_http_client::HttpClientFactory::new(
            codex_http_client::OutboundProxyPolicy::ReqwestDefault,
        ),
    )
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
    let base_url = format!("http://control-{nonce}.hepta.invalid/backend-api");
    let upload_url = format!("http://upload-{nonce}.hepta.invalid/blob/file_123");
    let upload_request_url = format!("{base_url}/public/plugins/workspace/upload-url");
    let finalize_url = format!("{base_url}/public/plugins/workspace");
    let expected_request_lines = [
        format!("POST {upload_request_url} HTTP/1.1"),
        format!("PUT {upload_url} HTTP/1.1"),
        format!("POST {finalize_url} HTTP/1.1"),
    ];
    let upload_response_body = json!({
        "file_id": "file_123",
        "upload_url": upload_url,
        "etag": "\"upload_etag_123\"",
    })
    .to_string();
    let finalize_response_body = json!({
        "plugin_id": "plugins_123",
        "share_url": "https://chatgpt.example/plugins/share/share-key-1",
    })
    .to_string();
    let thread = std::thread::spawn(move || {
        let mut requests = Vec::new();
        for (index, expected_request_line) in expected_request_lines.into_iter().enumerate() {
            let deadline = std::time::Instant::now() + std::time::Duration::from_secs(5);
            let (mut stream, _) = loop {
                match listener.accept() {
                    Ok(connection) => break connection,
                    Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                        assert!(
                            std::time::Instant::now() < deadline,
                            "proxy should receive request {}",
                            index + 1
                        );
                        std::thread::sleep(std::time::Duration::from_millis(10));
                    }
                    Err(error) => panic!("proxy listener should accept: {error}"),
                }
            };
            stream
                .set_read_timeout(Some(std::time::Duration::from_secs(5)))
                .expect("proxy stream should get a read timeout");
            let request = read_proxy_http_message(&mut stream);
            assert_eq!(
                request.lines().next(),
                Some(expected_request_line.as_str()),
                "request must use HTTP proxy absolute-form"
            );
            let response_body = match index {
                0 => upload_response_body.as_str(),
                1 => "",
                2 => finalize_response_body.as_str(),
                _ => unreachable!("exactly three provider HTTP requests are expected"),
            };
            let response = format!(
                "HTTP/1.1 201 Created\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{response_body}",
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
    let mut header_len = None;
    loop {
        let bytes_read = stream.read(&mut chunk).expect("proxy request should read");
        if bytes_read == 0 {
            break;
        }
        buffer.extend_from_slice(&chunk[..bytes_read]);
        let Some(header_end) = buffer.windows(4).position(|window| window == b"\r\n\r\n") else {
            continue;
        };
        let body_start = header_end + 4;
        header_len = Some(body_start);
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
    let header_len = header_len.expect("proxy request should include complete headers");
    String::from_utf8(buffer[..header_len].to_vec()).expect("proxy headers should be UTF-8")
}

fn test_auth() -> CodexAuth {
    CodexAuth::create_dummy_chatgpt_auth_for_testing()
}

fn write_file(path: &Path, contents: &str) {
    fs::create_dir_all(path.parent().expect("file should have a parent")).unwrap();
    fs::write(path, contents).unwrap();
}

fn write_test_plugin(root: &Path, plugin_name: &str) -> PathBuf {
    let plugin_path = root.join(plugin_name);
    write_file(
        &plugin_path.join(".codex-plugin/plugin.json"),
        &format!(r#"{{"name":"{plugin_name}"}}"#),
    );
    write_file(
        &plugin_path.join("skills/example/SKILL.md"),
        "# Example\n\nA test skill.\n",
    );
    plugin_path
}

fn write_plugin_share_local_path_mapping(
    codex_home: &Path,
    remote_plugin_id: &str,
    plugin_path: &AbsolutePathBuf,
) {
    write_file(
        &codex_home.join(".tmp/plugin-share-local-paths-v1.json"),
        &format!(
            "{}\n",
            serde_json::to_string_pretty(&json!({
                "localPluginPathsByRemotePluginId": {
                    remote_plugin_id: plugin_path,
                },
            }))
            .unwrap()
        ),
    );
}

fn archive_file_entries(archive_bytes: &[u8]) -> BTreeMap<String, Vec<u8>> {
    let decoder = flate2::read::GzDecoder::new(archive_bytes);
    let mut archive = tar::Archive::new(decoder);
    archive
        .entries()
        .unwrap()
        .filter_map(|entry| {
            let mut entry = entry.unwrap();
            if !entry.header().entry_type().is_file() {
                return None;
            }
            let path = entry.path().unwrap().to_string_lossy().into_owned();
            let mut contents = Vec::new();
            entry.read_to_end(&mut contents).unwrap();
            Some((path, contents))
        })
        .collect()
}

fn remote_plugin_json(plugin_id: &str) -> serde_json::Value {
    json!({
        "id": plugin_id,
        "name": "demo-plugin",
        "scope": "WORKSPACE",
        "discoverability": "PRIVATE",
        "installation_policy": "AVAILABLE",
        "authentication_policy": "ON_USE",
        "release": {
            "version": "0.1.0",
            "display_name": "Demo Plugin",
            "description": "Demo plugin description",
            "interface": {
                "short_description": "A demo plugin",
                "capabilities": ["Read", "Write"]
            },
            "skills": []
        }
    })
}

fn remote_plugin_json_with_share_url_and_principals(
    plugin_id: &str,
    share_url: Option<&str>,
    share_principals: serde_json::Value,
) -> serde_json::Value {
    let mut plugin = remote_plugin_json(plugin_id);
    let serde_json::Value::Object(fields) = &mut plugin else {
        unreachable!("plugin json should be an object");
    };
    fields.insert("discoverability".to_string(), json!("PRIVATE"));
    fields.insert("share_url".to_string(), json!(share_url));
    fields.insert("share_principals".to_string(), share_principals);
    plugin
}

fn installed_remote_plugin_json(plugin_id: &str) -> serde_json::Value {
    let mut plugin = remote_plugin_json(plugin_id);
    let serde_json::Value::Object(fields) = &mut plugin else {
        unreachable!("plugin json should be an object");
    };
    fields.insert("enabled".to_string(), json!(true));
    fields.insert("disabled_skill_names".to_string(), json!([]));
    plugin
}

fn empty_pagination_json() -> serde_json::Value {
    json!({
        "next_page_token": null
    })
}

fn expected_plugin_interface() -> PluginInterface {
    PluginInterface {
        display_name: Some("Demo Plugin".to_string()),
        short_description: Some("A demo plugin".to_string()),
        long_description: None,
        developer_name: None,
        category: None,
        capabilities: vec!["Read".to_string(), "Write".to_string()],
        website_url: None,
        privacy_policy_url: None,
        terms_of_service_url: None,
        default_prompt: None,
        brand_color: None,
        composer_icon: None,
        composer_icon_url: None,
        logo: None,
        logo_url: None,
        screenshots: Vec::new(),
        screenshot_urls: Vec::new(),
    }
}

#[tokio::test]
async fn save_remote_plugin_share_has_no_direct_egress_bypass() {
    let codex_home = TempDir::new().unwrap();
    let temp_dir = TempDir::new().unwrap();
    let plugin_path =
        AbsolutePathBuf::try_from(write_test_plugin(temp_dir.path(), "demo-plugin")).unwrap();
    let proxy = spawn_no_direct_egress_proxy();
    let proxy_url = format!("http://{}", proxy.address);
    let upload_request_url = format!("{}/public/plugins/workspace/upload-url", proxy.base_url);
    let finalize_url = format!("{}/public/plugins/workspace", proxy.base_url);
    for request_url in [&upload_request_url, &proxy.upload_url, &finalize_url] {
        codex_http_client::cache_system_proxy_route_for_test(request_url, proxy_url.clone());
    }
    let config = RemotePluginServiceConfig::new(
        proxy.base_url,
        codex_http_client::HttpClientFactory::new(
            codex_http_client::OutboundProxyPolicy::RespectSystemProxy,
        ),
    );
    let auth = test_auth();

    let result = tokio::time::timeout(
        std::time::Duration::from_secs(5),
        save_remote_plugin_share(
            &config,
            Some(&auth),
            codex_home.path(),
            &plugin_path,
            /*remote_plugin_id*/ None,
            RemotePluginShareAccessPolicy {
                discoverability: Some(RemotePluginShareDiscoverability::Private),
                share_targets: None,
            },
        ),
    )
    .await
    .expect("plugin share should complete through the configured proxy")
    .expect("plugin share should succeed without direct DNS or egress");

    assert_eq!(
        result,
        RemotePluginShareSaveResult {
            remote_plugin_id: "plugins_123".to_string(),
            share_url: Some("https://chatgpt.example/plugins/share/share-key-1".to_string()),
        }
    );
    let requests = proxy
        .thread
        .join()
        .expect("proxy should observe all provider requests");
    assert_eq!(requests.len(), 3);
    assert_eq!(
        requests
            .iter()
            .filter_map(|request| request.lines().next())
            .collect::<Vec<_>>(),
        vec![
            format!("POST {upload_request_url} HTTP/1.1"),
            format!("PUT {} HTTP/1.1", proxy.upload_url),
            format!("POST {finalize_url} HTTP/1.1"),
        ]
    );
}

#[tokio::test]
async fn save_remote_plugin_share_creates_workspace_plugin() {
    let codex_home = TempDir::new().unwrap();
    let temp_dir = TempDir::new().unwrap();
    let plugin_path =
        AbsolutePathBuf::try_from(write_test_plugin(temp_dir.path(), "demo-plugin")).unwrap();
    let archive_size = archive_plugin_for_upload(plugin_path.as_path())
        .unwrap()
        .len();
    let server = MockServer::start().await;
    let config = test_config(&server);
    let auth = test_auth();

    Mock::given(method("POST"))
        .and(path("/backend-api/public/plugins/workspace/upload-url"))
        .and(header("authorization", "Bearer Access Token"))
        .and(header("chatgpt-account-id", "account_id"))
        .and(body_json(json!({
            "filename": "demo-plugin.tar.gz",
            "mime_type": "application/gzip",
            "size_bytes": archive_size,
        })))
        .respond_with(ResponseTemplate::new(201).set_body_json(json!({
            "file_id": "file_123",
            "upload_url": format!("{}/upload/file_123", server.uri()),
            "etag": "\"upload_etag_123\"",
        })))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("PUT"))
        .and(path("/upload/file_123"))
        .and(header("x-ms-blob-type", "BlockBlob"))
        .and(header("content-type", "application/gzip"))
        .respond_with(ResponseTemplate::new(201).insert_header("etag", "\"blob_etag_123\""))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path("/backend-api/public/plugins/workspace"))
        .and(header("authorization", "Bearer Access Token"))
        .and(header("chatgpt-account-id", "account_id"))
        .and(body_json(json!({
            "file_id": "file_123",
            "etag": "\"upload_etag_123\"",
            "discoverability": "UNLISTED",
            "share_targets": [
                {
                    "principal_type": "user",
                    "principal_id": "user-1",
                    "role": "reader",
                },
                {
                    "principal_type": "workspace",
                    "principal_id": "account_id",
                    "role": "reader",
                },
            ],
        })))
        .respond_with(ResponseTemplate::new(201).set_body_json(json!({
            "plugin_id": "plugins_123",
            "share_url": "https://chatgpt.example/plugins/share/share-key-1",
        })))
        .expect(1)
        .mount(&server)
        .await;

    let result = save_remote_plugin_share(
        &config,
        Some(&auth),
        codex_home.path(),
        &plugin_path,
        /*remote_plugin_id*/ None,
        RemotePluginShareAccessPolicy {
            discoverability: Some(RemotePluginShareDiscoverability::Unlisted),
            share_targets: Some(vec![RemotePluginShareTarget {
                principal_type: RemotePluginSharePrincipalType::User,
                principal_id: "user-1".to_string(),
                role: RemotePluginShareTargetRole::Reader,
            }]),
        },
    )
    .await
    .unwrap();

    assert_eq!(
        result,
        RemotePluginShareSaveResult {
            remote_plugin_id: "plugins_123".to_string(),
            share_url: Some("https://chatgpt.example/plugins/share/share-key-1".to_string()),
        }
    );
    assert_eq!(
        local_paths::load_plugin_share_local_paths(codex_home.path()).unwrap(),
        BTreeMap::from([("plugins_123".to_string(), plugin_path)])
    );

    let requests = server.received_requests().await.unwrap_or_default();
    let upload_request = requests
        .iter()
        .find(|request| request.method == "PUT" && request.url.path() == "/upload/file_123")
        .unwrap();
    let archive_files = archive_file_entries(&upload_request.body);
    assert_eq!(
        archive_files
            .get(".codex-plugin/plugin.json")
            .map(Vec::as_slice),
        Some(br#"{"name":"demo-plugin"}"#.as_slice())
    );
    assert_eq!(
        archive_files
            .get("skills/example/SKILL.md")
            .map(Vec::as_slice),
        Some(b"# Example\n\nA test skill.\n".as_slice())
    );
}

#[test]
fn archive_plugin_for_upload_rejects_archives_over_limit() {
    let temp_dir = TempDir::new().unwrap();
    let plugin_path = write_test_plugin(temp_dir.path(), "demo-plugin");
    write_file(
        &plugin_path.join("large.txt"),
        &"0123456789abcdef".repeat(1024),
    );

    let err = archive_plugin_for_upload_with_limit(&plugin_path, /*max_bytes*/ 16)
        .expect_err("oversized plugin archive should fail");

    assert!(matches!(
        err,
        RemotePluginCatalogError::ArchiveTooLarge { .. }
    ));
}

#[test]
fn archive_plugin_for_upload_places_manifest_at_archive_root() {
    let temp_dir = TempDir::new().unwrap();
    let plugin_path = write_test_plugin(temp_dir.path(), "demo-plugin");

    let archive_bytes = archive_plugin_for_upload(&plugin_path).unwrap();
    let archive_files = archive_file_entries(&archive_bytes);

    assert_eq!(
        archive_files.keys().cloned().collect::<Vec<_>>(),
        vec![
            ".codex-plugin/plugin.json".to_string(),
            "skills/example/SKILL.md".to_string()
        ]
    );
    assert_eq!(
        archive_files
            .get(".codex-plugin/plugin.json")
            .map(Vec::as_slice),
        Some(br#"{"name":"demo-plugin"}"#.as_slice())
    );
    assert_eq!(
        archive_files
            .get("skills/example/SKILL.md")
            .map(Vec::as_slice),
        Some(b"# Example\n\nA test skill.\n".as_slice())
    );
}

#[tokio::test]
async fn save_remote_plugin_share_updates_existing_workspace_plugin() {
    let codex_home = TempDir::new().unwrap();
    let temp_dir = TempDir::new().unwrap();
    let plugin_path =
        AbsolutePathBuf::try_from(write_test_plugin(temp_dir.path(), "demo-plugin")).unwrap();
    let archive_size = archive_plugin_for_upload(plugin_path.as_path())
        .unwrap()
        .len();
    let server = MockServer::start().await;
    let config = test_config(&server);
    let auth = test_auth();

    Mock::given(method("POST"))
        .and(path("/backend-api/public/plugins/workspace/upload-url"))
        .and(body_json(json!({
            "filename": "demo-plugin.tar.gz",
            "mime_type": "application/gzip",
            "size_bytes": archive_size,
            "plugin_id": "plugins_123",
        })))
        .respond_with(ResponseTemplate::new(201).set_body_json(json!({
            "file_id": "file_456",
            "upload_url": format!("{}/upload/file_456", server.uri()),
            "etag": "\"upload_etag_456\"",
        })))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("PUT"))
        .and(path("/upload/file_456"))
        .respond_with(ResponseTemplate::new(201).insert_header("etag", "\"blob_etag_456\""))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path("/backend-api/public/plugins/workspace/plugins_123"))
        .and(body_json(json!({
            "file_id": "file_456",
            "etag": "\"upload_etag_456\"",
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "plugin_id": "plugins_123",
        })))
        .expect(1)
        .mount(&server)
        .await;

    let result = save_remote_plugin_share(
        &config,
        Some(&auth),
        codex_home.path(),
        &plugin_path,
        Some("plugins_123"),
        RemotePluginShareAccessPolicy::default(),
    )
    .await
    .unwrap();

    assert_eq!(
        result,
        RemotePluginShareSaveResult {
            remote_plugin_id: "plugins_123".to_string(),
            share_url: None,
        }
    );
}

#[tokio::test]
async fn update_remote_plugin_share_targets_updates_targets() {
    let server = MockServer::start().await;
    let config = test_config(&server);
    let auth = test_auth();

    Mock::given(method("PUT"))
        .and(path("/backend-api/ps/plugins/plugins_123/shares"))
        .and(header("authorization", "Bearer Access Token"))
        .and(header("chatgpt-account-id", "account_id"))
        .and(body_json(json!({
            "discoverability": "UNLISTED",
            "targets": [
                {
                    "principal_type": "user",
                    "principal_id": "user-1",
                    "role": "editor",
                },
                {
                    "principal_type": "group",
                    "principal_id": "group-1",
                    "role": "reader",
                },
                {
                    "principal_type": "workspace",
                    "principal_id": "account_id",
                    "role": "reader",
                },
            ],
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "principals": [
                {
                    "principal_type": "user",
                    "principal_id": "user-1",
                    "role": "editor",
                    "name": "Gavin",
                },
                {
                    "principal_type": "group",
                    "principal_id": "group-1",
                    "role": "reader",
                    "name": "Engineering",
                },
            ],
            "discoverability": "UNLISTED",
        })))
        .expect(1)
        .mount(&server)
        .await;

    let result = update_remote_plugin_share_targets(
        &config,
        Some(&auth),
        "plugins_123",
        vec![
            RemotePluginShareTarget {
                principal_type: RemotePluginSharePrincipalType::User,
                principal_id: "user-1".to_string(),
                role: RemotePluginShareTargetRole::Editor,
            },
            RemotePluginShareTarget {
                principal_type: RemotePluginSharePrincipalType::Group,
                principal_id: "group-1".to_string(),
                role: RemotePluginShareTargetRole::Reader,
            },
        ],
        RemotePluginShareUpdateDiscoverability::Unlisted,
    )
    .await
    .unwrap();

    assert_eq!(
        result,
        RemotePluginShareUpdateTargetsResult {
            principals: vec![
                RemotePluginSharePrincipal {
                    principal_type: RemotePluginSharePrincipalType::User,
                    principal_id: "user-1".to_string(),
                    role: RemotePluginSharePrincipalRole::Editor,
                    name: "Gavin".to_string(),
                },
                RemotePluginSharePrincipal {
                    principal_type: RemotePluginSharePrincipalType::Group,
                    principal_id: "group-1".to_string(),
                    role: RemotePluginSharePrincipalRole::Reader,
                    name: "Engineering".to_string(),
                },
            ],
            discoverability: RemotePluginShareDiscoverability::Unlisted,
        }
    );
}

#[tokio::test]
async fn list_remote_plugin_shares_fetches_created_workspace_plugins() {
    let codex_home = TempDir::new().unwrap();
    let local_plugin_path =
        AbsolutePathBuf::try_from(codex_home.path().join("local-plugin")).unwrap();
    write_plugin_share_local_path_mapping(codex_home.path(), "plugins_123", &local_plugin_path);
    let server = MockServer::start().await;
    let config = test_config(&server);
    let auth = test_auth();

    Mock::given(method("GET"))
        .and(path("/backend-api/ps/plugins/workspace/created"))
        .and(header("authorization", "Bearer Access Token"))
        .and(header("chatgpt-account-id", "account_id"))
        .and(query_param(
            "limit",
            REMOTE_PLUGIN_LIST_PAGE_LIMIT.to_string(),
        ))
        .and(query_param_is_missing("pageToken"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "plugins": [remote_plugin_json_with_share_url_and_principals(
                "plugins_123",
                Some("https://chatgpt.example/plugins/share/share-key-1"),
                json!([
                    {
                        "principal_type": "user",
                        "principal_id": "user-owner",
                        "role": "owner",
                        "name": "Owner",
                    },
                    {
                        "principal_type": "user",
                        "principal_id": "user-reader",
                        "role": "reader",
                        "name": "Reader",
                    },
                ]),
            )],
            "pagination": {
                "next_page_token": "page-2"
            },
        })))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/backend-api/ps/plugins/workspace/created"))
        .and(header("authorization", "Bearer Access Token"))
        .and(header("chatgpt-account-id", "account_id"))
        .and(query_param(
            "limit",
            REMOTE_PLUGIN_LIST_PAGE_LIMIT.to_string(),
        ))
        .and(query_param("pageToken", "page-2"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "plugins": [remote_plugin_json_with_share_url_and_principals(
                "plugins_456",
                /*share_url*/ None,
                json!([
                    {
                        "principal_type": "user",
                        "principal_id": "user-owner",
                        "role": "owner",
                        "name": "Owner",
                    },
                    {
                        "principal_type": "user",
                        "principal_id": "user-editor",
                        "role": "editor",
                        "name": "Editor",
                    },
                ]),
            )],
            "pagination": empty_pagination_json(),
        })))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/backend-api/ps/plugins/installed"))
        .and(query_param("scope", "WORKSPACE"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "plugins": [installed_remote_plugin_json("plugins_456")],
            "pagination": empty_pagination_json(),
        })))
        .expect(1)
        .mount(&server)
        .await;

    let result = list_remote_plugin_shares(&config, Some(&auth), codex_home.path())
        .await
        .unwrap();

    assert_eq!(
        result,
        vec![
            RemotePluginShareSummary {
                summary: RemotePluginSummary {
                    id: "demo-plugin@workspace-shared-with-me".to_string(),
                    remote_plugin_id: "plugins_123".to_string(),
                    name: "demo-plugin".to_string(),
                    share_context: Some(RemotePluginShareContext {
                        remote_plugin_id: "plugins_123".to_string(),
                        remote_version: Some("0.1.0".to_string()),
                        discoverability: RemotePluginShareDiscoverability::Private,
                        share_url: Some(
                            "https://chatgpt.example/plugins/share/share-key-1".to_string(),
                        ),
                        creator_account_user_id: None,
                        creator_name: None,
                        share_principals: Some(vec![
                            RemotePluginSharePrincipal {
                                principal_type: RemotePluginSharePrincipalType::User,
                                principal_id: "user-owner".to_string(),
                                role: RemotePluginSharePrincipalRole::Owner,
                                name: "Owner".to_string(),
                            },
                            RemotePluginSharePrincipal {
                                principal_type: RemotePluginSharePrincipalType::User,
                                principal_id: "user-reader".to_string(),
                                role: RemotePluginSharePrincipalRole::Reader,
                                name: "Reader".to_string(),
                            },
                        ]),
                    }),
                    installed: false,
                    installed_at: None,
                    enabled: false,
                    install_policy: PluginInstallPolicy::Available,
                    auth_policy: PluginAuthPolicy::OnUse,
                    availability: PluginAvailability::Available,
                    disabled_reason: None,
                    eligible_plan_types: None,
                    interface: Some(expected_plugin_interface()),
                    keywords: Vec::new(),
                },
                local_plugin_path: Some(local_plugin_path),
            },
            RemotePluginShareSummary {
                summary: RemotePluginSummary {
                    id: "demo-plugin@workspace-shared-with-me".to_string(),
                    remote_plugin_id: "plugins_456".to_string(),
                    name: "demo-plugin".to_string(),
                    share_context: Some(RemotePluginShareContext {
                        remote_plugin_id: "plugins_456".to_string(),
                        remote_version: Some("0.1.0".to_string()),
                        discoverability: RemotePluginShareDiscoverability::Private,
                        share_url: None,
                        creator_account_user_id: None,
                        creator_name: None,
                        share_principals: Some(vec![
                            RemotePluginSharePrincipal {
                                principal_type: RemotePluginSharePrincipalType::User,
                                principal_id: "user-owner".to_string(),
                                role: RemotePluginSharePrincipalRole::Owner,
                                name: "Owner".to_string(),
                            },
                            RemotePluginSharePrincipal {
                                principal_type: RemotePluginSharePrincipalType::User,
                                principal_id: "user-editor".to_string(),
                                role: RemotePluginSharePrincipalRole::Editor,
                                name: "Editor".to_string(),
                            },
                        ]),
                    }),
                    installed: true,
                    installed_at: None,
                    enabled: true,
                    install_policy: PluginInstallPolicy::Available,
                    auth_policy: PluginAuthPolicy::OnUse,
                    availability: PluginAvailability::Available,
                    disabled_reason: None,
                    eligible_plan_types: None,
                    interface: Some(expected_plugin_interface()),
                    keywords: Vec::new(),
                },
                local_plugin_path: None,
            }
        ]
    );
}

#[tokio::test]
async fn delete_remote_plugin_share_deletes_workspace_plugin() {
    let codex_home = TempDir::new().unwrap();
    let local_plugin_path =
        AbsolutePathBuf::try_from(codex_home.path().join("local-plugin")).unwrap();
    write_plugin_share_local_path_mapping(codex_home.path(), "plugins_123", &local_plugin_path);
    let server = MockServer::start().await;
    let config = test_config(&server);
    let auth = test_auth();

    Mock::given(method("DELETE"))
        .and(path("/backend-api/public/plugins/workspace/plugins_123"))
        .and(header("authorization", "Bearer Access Token"))
        .and(header("chatgpt-account-id", "account_id"))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&server)
        .await;

    delete_remote_plugin_share(&config, Some(&auth), codex_home.path(), "plugins_123")
        .await
        .unwrap();
    assert_eq!(
        local_paths::load_plugin_share_local_paths(codex_home.path()).unwrap(),
        BTreeMap::new()
    );
}

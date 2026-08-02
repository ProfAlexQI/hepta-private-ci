use anyhow::Result;
use app_test_support::DEFAULT_CLIENT_NAME;
use app_test_support::McpProcess;
use app_test_support::to_response;
use codex_app_server_protocol::ClientInfo;
use codex_app_server_protocol::InitializeCapabilities;
use codex_app_server_protocol::JSONRPCMessage;
use codex_app_server_protocol::JSONRPCResponse;
use codex_app_server_protocol::PluginSearchParams;
use codex_app_server_protocol::PluginSearchResponse;
use codex_app_server_protocol::PluginSearchScope;
use codex_app_server_protocol::RequestId;
use pretty_assertions::assert_eq;
use std::time::Duration;
use tempfile::TempDir;
use tokio::time::timeout;

const DEFAULT_TIMEOUT: Duration = Duration::from_secs(20);

#[tokio::test]
async fn plugin_search_requires_experimental_api_capability() -> Result<()> {
    let codex_home = TempDir::new()?;
    let mut mcp = McpProcess::new(codex_home.path()).await?;
    let init = timeout(
        DEFAULT_TIMEOUT,
        mcp.initialize_with_capabilities(
            ClientInfo {
                name: DEFAULT_CLIENT_NAME.to_string(),
                title: None,
                version: "0.1.0".to_string(),
            },
            Some(InitializeCapabilities {
                experimental_api: false,
                request_attestation: false,
                opt_out_notification_methods: None,
            }),
        ),
    )
    .await??;
    let JSONRPCMessage::Response(_) = init else {
        anyhow::bail!("expected initialize response, got {init:?}");
    };

    let request_id = mcp
        .send_plugin_search_request(PluginSearchParams {
            search_term: "linear".into(),
            scope: None,
            cwds: None,
            cursor: None,
            limit: None,
        })
        .await?;
    let error = timeout(
        DEFAULT_TIMEOUT,
        mcp.read_stream_until_error_message(RequestId::Integer(request_id)),
    )
    .await??;
    assert_eq!(error.error.code, -32600);
    assert_eq!(
        error.error.message,
        "plugin/search requires experimentalApi capability"
    );
    assert_eq!(error.error.data, None);
    Ok(())
}

#[tokio::test]
async fn plugin_search_is_read_only_and_feature_gated() -> Result<()> {
    let codex_home = TempDir::new()?;
    let mut mcp = McpProcess::new(codex_home.path()).await?;
    timeout(DEFAULT_TIMEOUT, mcp.initialize()).await??;

    let response = search(
        &mut mcp,
        PluginSearchParams {
            search_term: "linear".into(),
            scope: None,
            cwds: None,
            cursor: None,
            limit: None,
        },
    )
    .await?;
    assert_eq!(
        response,
        PluginSearchResponse {
            data: Vec::new(),
            next_cursor: None,
        }
    );
    assert!(
        !codex_home
            .path()
            .join("hepta-plugin-mutation-journal.json")
            .exists()
    );
    Ok(())
}

#[tokio::test]
async fn plugin_search_denies_remote_scopes_when_remote_plugin_is_disabled() -> Result<()> {
    let codex_home = TempDir::new()?;
    std::fs::write(
        codex_home.path().join("config.toml"),
        r#"[features]
plugins = true
remote_plugin = false
remote_plugin_search = true
"#,
    )?;
    let mut mcp = McpProcess::new(codex_home.path()).await?;
    timeout(DEFAULT_TIMEOUT, mcp.initialize()).await??;

    for scope in [PluginSearchScope::Global, PluginSearchScope::Personal] {
        let response = search(
            &mut mcp,
            PluginSearchParams {
                search_term: "linear".into(),
                scope: Some(scope),
                cwds: None,
                cursor: None,
                limit: Some(16),
            },
        )
        .await?;
        assert!(response.data.is_empty());
        assert!(response.next_cursor.is_none());
    }
    assert!(
        !codex_home
            .path()
            .join("hepta-plugin-mutation-journal.json")
            .exists()
    );
    Ok(())
}

#[tokio::test]
async fn plugin_search_input_budgets_fail_closed_without_echoing_user_text() -> Result<()> {
    let codex_home = TempDir::new()?;
    let mut mcp = McpProcess::new(codex_home.path()).await?;
    timeout(DEFAULT_TIMEOUT, mcp.initialize()).await??;

    let secret = format!("private-marker-{}", "x".repeat(600));
    let request_id = mcp
        .send_plugin_search_request(PluginSearchParams {
            search_term: secret.clone(),
            scope: None,
            cwds: None,
            cursor: None,
            limit: Some(16),
        })
        .await?;
    let error = timeout(
        DEFAULT_TIMEOUT,
        mcp.read_stream_until_error_message(RequestId::Integer(request_id)),
    )
    .await??;
    assert_eq!(error.error.code, -32600);
    assert!(!error.error.message.contains(&secret));
    assert!(error.error.message.contains("searchTerm"));

    let request_id = mcp
        .send_plugin_search_request(PluginSearchParams {
            search_term: "linear".into(),
            scope: None,
            cwds: None,
            cursor: None,
            limit: Some(51),
        })
        .await?;
    let error = timeout(
        DEFAULT_TIMEOUT,
        mcp.read_stream_until_error_message(RequestId::Integer(request_id)),
    )
    .await??;
    assert_eq!(error.error.code, -32600);
    assert!(error.error.message.contains("result-count budget"));
    assert!(
        !codex_home
            .path()
            .join("hepta-plugin-mutation-journal.json")
            .exists()
    );
    Ok(())
}

async fn search(mcp: &mut McpProcess, params: PluginSearchParams) -> Result<PluginSearchResponse> {
    let request_id = mcp.send_plugin_search_request(params).await?;
    let response: JSONRPCResponse = timeout(
        DEFAULT_TIMEOUT,
        mcp.read_stream_until_response_message(RequestId::Integer(request_id)),
    )
    .await??;
    to_response(response)
}

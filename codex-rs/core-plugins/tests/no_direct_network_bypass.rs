use std::fs;
use std::path::Path;
use std::path::PathBuf;

fn rust_files_beneath(root: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    let mut pending = vec![root.to_path_buf()];
    while let Some(path) = pending.pop() {
        if path.is_dir() {
            for entry in fs::read_dir(&path).expect("read source directory") {
                pending.push(entry.expect("read source entry").path());
            }
        } else if path.extension().and_then(|extension| extension.to_str()) == Some("rs") {
            files.push(path);
        }
    }
    files
}

fn assert_forbidden(path: &Path, source: &str, needles: &[&str]) {
    for needle in needles {
        assert!(
            !source.contains(needle),
            "{} must not contain direct-network bypass `{needle}`",
            path.display()
        );
    }
}

#[test]
fn governed_network_callers_cannot_regress_to_direct_clients() {
    let crate_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let workspace = crate_root.parent().expect("workspace root");

    let mut remote_sources = vec![
        crate_root.join("src/remote.rs"),
        crate_root.join("src/remote_legacy.rs"),
        crate_root.join("src/remote_bundle.rs"),
    ];
    remote_sources.extend(rust_files_beneath(&crate_root.join("src/remote")));
    for path in remote_sources {
        let source = fs::read_to_string(&path).expect("read remote plugin source");
        assert_forbidden(
            &path,
            &source,
            &["build_reqwest_client", "reqwest::Client", "ClientBuilder"],
        );
    }

    let responses_ws = workspace.join("codex-api/src/endpoint/responses_websocket.rs");
    let responses_ws_source =
        fs::read_to_string(&responses_ws).expect("read Responses WebSocket source");
    assert_forbidden(
        &responses_ws,
        &responses_ws_source,
        &[
            "connect_async(",
            "connect_async_tls_with_config",
            "tokio_tungstenite::connect_async",
        ],
    );
    assert!(
        responses_ws_source.contains("WebSocketConnector::new"),
        "Responses WebSocket must use the configured connector"
    );

    let oauth_login = workspace.join("rmcp-client/src/perform_oauth_login.rs");
    let oauth_source = fs::read_to_string(&oauth_login).expect("read MCP OAuth source");
    assert_forbidden(
        &oauth_login,
        &oauth_source,
        &["ClientBuilder", "reqwest::Client::new"],
    );
    assert!(
        oauth_source.contains("OAuthHttpClientAdapter::new"),
        "MCP OAuth must use the runtime-selected HTTP adapter"
    );
}

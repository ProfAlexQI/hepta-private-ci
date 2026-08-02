use super::*;
use crate::remote::REMOTE_CREATED_BY_ME_MARKETPLACE_NAME;
use crate::remote::REMOTE_GLOBAL_MARKETPLACE_NAME;
use crate::remote::REMOTE_WORKSPACE_MARKETPLACE_NAME;
use codex_http_client::HttpClientFactory;
use codex_http_client::OutboundProxyPolicy;
use codex_login::CodexAuth;
use pretty_assertions::assert_eq;
use serde_json::json;

fn authorization() -> EgressAuthorization {
    EgressAuthorization::EffectBound {
        admission_hash: format!("sha256:{}", "1".repeat(64)),
        effect_plan_hash: format!("sha256:{}", "2".repeat(64)),
    }
}

fn http_client_factory() -> HttpClientFactory {
    HttpClientFactory::new(OutboundProxyPolicy::DirectOnly)
}

fn search<'a>(
    query: &'a str,
    scope: Option<RemotePluginScope>,
    limit: u32,
    page_token: Option<&'a str>,
) -> RemotePluginSearchRequest<'a> {
    RemotePluginSearchRequest {
        query,
        scope,
        limit,
        page_token,
    }
}

fn plugin(id: &str, name: &str, scope: &str) -> Value {
    json!({
        "id": id,
        "name": name,
        "scope": scope,
        "discoverability": (scope == "WORKSPACE").then_some("LISTED"),
        "installation_policy": "AVAILABLE",
        "authentication_policy": "ON_USE",
        "release": {
            "display_name": name,
            "description": format!("{name} description"),
            "interface": {},
        },
    })
}

#[test]
fn request_is_queryless_in_diagnostics_and_carries_bounded_sensitive_query_separately() {
    let config = RemotePluginServiceConfig::new(
        "https://chatgpt.com/backend-api/".into(),
        http_client_factory(),
    );
    let auth = CodexAuth::create_dummy_chatgpt_auth_for_testing();
    let request = build_remote_plugin_search_request(
        &config,
        Some(&auth),
        authorization(),
        search(
            "linear & docs/+",
            Some(RemotePluginScope::Global),
            16,
            Some("next page/+"),
        ),
    )
    .expect("search request should be admitted");

    assert_eq!(
        request.url,
        "https://chatgpt.com/backend-api/ps/plugins/search"
    );
    assert!(!request.url.contains("linear"));
    assert!(!request.url.contains("next page"));
    assert_eq!(
        request.query,
        vec![
            ("q".into(), "linear & docs/+".into()),
            ("limit".into(), "16".into()),
            ("scope".into(), "GLOBAL".into()),
            ("pageToken".into(), "next page/+".into()),
        ]
    );
    assert_eq!(request.capability, OutboundHttpCapability::OpenAiCodexApi);
    assert_eq!(request.method, EgressMethod::Get);
    assert_eq!(request.max_response_bytes, MAX_PLUGIN_SEARCH_RESPONSE_BYTES);
    assert_eq!(request.timeout, Duration::from_secs(15));
    assert!(request.body.is_none());
    assert!(
        request
            .headers
            .iter()
            .any(|(name, value)| name.eq_ignore_ascii_case("authorization")
                && value.starts_with("Bearer "))
    );
    let diagnostic = format!(
        "{:?}",
        search(
            "unique-query-secret",
            Some(RemotePluginScope::Global),
            16,
            Some("unique-cursor-secret"),
        )
    );
    assert!(!diagnostic.contains("unique-query-secret"));
    assert!(!diagnostic.contains("unique-cursor-secret"));
    assert!(diagnostic.contains("<redacted>"));
}

#[test]
fn request_requires_chatgpt_auth_before_egress() {
    let config = RemotePluginServiceConfig::new(
        "https://chatgpt.com/backend-api".into(),
        http_client_factory(),
    );
    let no_auth = build_remote_plugin_search_request(
        &config,
        None,
        authorization(),
        search("calendar", None, 16, None),
    );
    assert!(matches!(
        no_auth,
        Err(RemotePluginCatalogError::AuthRequired)
    ));

    let api_key = CodexAuth::from_api_key("secret");
    let api_key = build_remote_plugin_search_request(
        &config,
        Some(&api_key),
        authorization(),
        search("calendar", None, 16, None),
    );
    assert!(matches!(
        api_key,
        Err(RemotePluginCatalogError::UnsupportedAuthMode)
    ));
}

#[test]
fn request_bounds_query_cursor_and_page_size_fail_closed() {
    let config = RemotePluginServiceConfig::new(
        "https://chatgpt.com/backend-api".into(),
        http_client_factory(),
    );
    let auth = CodexAuth::create_dummy_chatgpt_auth_for_testing();
    for request in [
        search(" ", None, 16, None),
        search(
            &"q".repeat(MAX_PLUGIN_SEARCH_TERM_BYTES + 1),
            None,
            16,
            None,
        ),
        search(
            &"q".repeat(MAX_PLUGIN_SEARCH_TERM_CHARS + 1),
            None,
            16,
            None,
        ),
        search("q", None, 0, None),
        search("q", None, MAX_PLUGIN_SEARCH_LIMIT + 1, None),
        search("q", None, 16, Some("")),
        search(
            "q",
            None,
            16,
            Some(&"c".repeat(MAX_PLUGIN_SEARCH_CURSOR_BYTES + 1)),
        ),
        search(
            "q",
            None,
            16,
            Some(&"c".repeat(MAX_PLUGIN_SEARCH_CURSOR_CHARS + 1)),
        ),
    ] {
        assert!(
            build_remote_plugin_search_request(&config, Some(&auth), authorization(), request,)
                .is_err()
        );
    }
}

#[test]
fn response_preserves_order_and_maps_all_scopes_without_install_authority() {
    let body = serde_json::to_vec(&json!({
        "plugins": [
            plugin("global-id", "global", "GLOBAL"),
            plugin("personal-id", "personal", "USER"),
            plugin("workspace-id", "workspace", "WORKSPACE"),
        ],
        "pagination": {"next_page_token": "next"},
    }))
    .unwrap();
    let page = decode_remote_plugin_search_response(
        "https://chatgpt.com/backend-api/ps/plugins/search",
        3,
        TextEgressResponse { status: 200, body },
    )
    .expect("bounded search response should decode");

    assert_eq!(
        page.plugins
            .iter()
            .map(|plugin| plugin.id.as_str())
            .collect::<Vec<_>>(),
        vec![
            format!("global@{REMOTE_GLOBAL_MARKETPLACE_NAME}"),
            format!("personal@{REMOTE_CREATED_BY_ME_MARKETPLACE_NAME}"),
            format!("workspace@{REMOTE_WORKSPACE_MARKETPLACE_NAME}"),
        ]
    );
    assert!(
        page.plugins
            .iter()
            .all(|plugin| !plugin.installed && !plugin.enabled)
    );
    assert_eq!(page.next_page_token.as_deref(), Some("next"));
}

#[test]
fn response_allows_bounded_unknown_fields_for_forward_compatibility() {
    let mut item = plugin("global-id", "global", "GLOBAL");
    item["future_field"] = json!({"bounded": "value"});
    let body = serde_json::to_vec(&json!({
        "plugins": [item],
        "pagination": {"next_page_token": null},
        "future_top_level": true,
    }))
    .unwrap();
    let page = decode_remote_plugin_search_response(
        "https://chatgpt.com/backend-api/ps/plugins/search",
        1,
        TextEgressResponse { status: 200, body },
    )
    .expect("bounded unknown fields should be ignored by the typed decoder");
    assert_eq!(page.plugins.len(), 1);
}

#[test]
fn response_rejects_excessive_json_nesting_before_typed_decode() {
    let mut nested = Value::Null;
    for _ in 0..=MAX_PLUGIN_SEARCH_JSON_DEPTH {
        nested = json!({"nested": nested});
    }
    let body = serde_json::to_vec(&json!({
        "plugins": [],
        "pagination": {"next_page_token": null},
        "future": nested,
    }))
    .unwrap();
    assert!(matches!(
        decode_remote_plugin_search_response(
            "https://chatgpt.com/backend-api/ps/plugins/search",
            1,
            TextEgressResponse { status: 200, body }
        ),
        Err(RemotePluginCatalogError::SearchResponseBound("JSON depth"))
    ));
}

#[test]
fn response_rejects_excessive_json_nodes_and_aggregate_text_before_typed_decode() {
    let endpoint = "https://chatgpt.com/backend-api/ps/plugins/search";
    let too_many_nodes = serde_json::to_vec(&json!({
        "plugins": [],
        "pagination": {"next_page_token": null},
        "future": vec![Value::Null; MAX_PLUGIN_SEARCH_JSON_NODES],
    }))
    .unwrap();
    assert!(matches!(
        decode_remote_plugin_search_response(
            endpoint,
            1,
            TextEgressResponse {
                status: 200,
                body: too_many_nodes,
            }
        ),
        Err(RemotePluginCatalogError::SearchResponseBound(
            "JSON node count"
        ))
    ));

    let too_much_text = serde_json::to_vec(&json!({
        "plugins": [],
        "pagination": {"next_page_token": null},
        "future": vec!["x".repeat(MAX_PLUGIN_SEARCH_TEXT_FIELD_CHARS); 65],
    }))
    .unwrap();
    assert!(too_much_text.len() < MAX_PLUGIN_SEARCH_RESPONSE_BYTES);
    assert!(matches!(
        decode_remote_plugin_search_response(
            endpoint,
            1,
            TextEgressResponse {
                status: 200,
                body: too_much_text,
            }
        ),
        Err(RemotePluginCatalogError::SearchResponseBound(
            "aggregate text"
        ))
    ));
}

#[test]
fn response_count_text_cursor_and_byte_budgets_fail_closed() {
    let endpoint = "https://chatgpt.com/backend-api/ps/plugins/search";
    let too_many = serde_json::to_vec(&json!({
        "plugins": [plugin("one", "one", "GLOBAL"), plugin("two", "two", "GLOBAL")],
        "pagination": {"next_page_token": null},
    }))
    .unwrap();
    assert!(matches!(
        decode_remote_plugin_search_response(
            endpoint,
            1,
            TextEgressResponse {
                status: 200,
                body: too_many
            }
        ),
        Err(RemotePluginCatalogError::SearchResponseBound(
            "result count"
        ))
    ));

    let long_text = serde_json::to_vec(&json!({
        "plugins": [plugin("one", &"x".repeat(MAX_PLUGIN_SEARCH_TEXT_FIELD_BYTES + 1), "GLOBAL")],
        "pagination": {"next_page_token": null},
    }))
    .unwrap();
    assert!(matches!(
        decode_remote_plugin_search_response(
            endpoint,
            1,
            TextEgressResponse {
                status: 200,
                body: long_text
            }
        ),
        Err(RemotePluginCatalogError::SearchResponseBound(
            "individual text field"
        ))
    ));

    let long_cursor = serde_json::to_vec(&json!({
        "plugins": [],
        "pagination": {"next_page_token": "c".repeat(MAX_PLUGIN_SEARCH_CURSOR_BYTES + 1)},
    }))
    .unwrap();
    assert!(matches!(
        decode_remote_plugin_search_response(
            endpoint,
            1,
            TextEgressResponse {
                status: 200,
                body: long_cursor
            }
        ),
        Err(RemotePluginCatalogError::SearchResponseBound("cursor text"))
    ));

    assert!(matches!(
        decode_remote_plugin_search_response(
            endpoint,
            1,
            TextEgressResponse {
                status: 200,
                body: vec![b' '; MAX_PLUGIN_SEARCH_RESPONSE_BYTES + 1],
            }
        ),
        Err(RemotePluginCatalogError::SearchResponseBound("byte"))
    ));
}

#[test]
fn upstream_error_does_not_echo_response_or_sensitive_query_material() {
    let error = decode_remote_plugin_search_response(
        "https://chatgpt.com/backend-api/ps/plugins/search",
        16,
        TextEgressResponse {
            status: 503,
            body: b"sensitive search term and pagination token".to_vec(),
        },
    )
    .expect_err("upstream failure should fail closed");
    let message = error.to_string();
    assert!(!message.contains("sensitive search term"));
    assert!(!message.contains("pagination token"));
    assert!(!message.contains('?'));
}

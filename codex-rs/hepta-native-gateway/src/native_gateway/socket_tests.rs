use std::io::Read;
use std::io::Write;
use std::net::Shutdown;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use std::time::Instant;

pub(super) fn is_quarantined_control_ui_route(method: &str, path: &str) -> bool {
    CONTROL_UI_ROUTE_SPECS.iter().any(|route| {
        route.method == method
            && route.is_quarantined_transitive_effect()
            && control_ui_route_pattern_matches(route.pattern, path)
    })
}

fn route_over_real_socket(
    runtime: Arc<NativeGatewayRuntime>,
    options: NativeGatewayOptions,
    method: &str,
    path: &str,
) -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("socket parity listener");
    let address = listener.local_addr().expect("socket parity address");
    let server = thread::spawn(move || {
        let (mut stream, _) = listener.accept().expect("socket parity accept");
        handle_native_gateway_connection(
            &mut stream,
            Instant::now() + Duration::from_secs(30),
            &options,
            &runtime,
        )
        .expect("socket parity handler");
    });
    let mut client = TcpStream::connect(address).expect("socket parity client");
    client
        .set_read_timeout(Some(Duration::from_secs(30)))
        .expect("socket parity read timeout");
    write!(
        client,
        "{method} {path} HTTP/1.1\r\nHost: 127.0.0.1\r\nConnection: close\r\n\r\n"
    )
    .expect("socket parity request");
    client
        .shutdown(Shutdown::Write)
        .expect("socket parity request shutdown");
    let mut response = String::new();
    client
        .read_to_string(&mut response)
        .expect("socket parity response");
    server.join().expect("socket parity server");
    response
}

#[test]
fn all_registered_get_routes_return_structured_http_over_real_sockets() {
    let root = tempfile::tempdir().expect("runtime root");
    let runtime = Arc::new(
        NativeGatewayRuntime::bootstrap_with_anchor_for_test(root.path()).expect("keyed runtime"),
    );
    let options = test_gateway_options(false);
    for route in CONTROL_UI_ROUTE_SPECS
        .iter()
        .filter(|route| route.method == "GET")
    {
        let path = control_ui_sample_path(route.pattern);
        let response =
            route_over_real_socket(Arc::clone(&runtime), options.clone(), route.method, &path);
        let expected_status = if route.is_quarantined_transitive_effect() {
            "HTTP/1.1 410 Gone"
        } else {
            "HTTP/1.1 200 OK"
        };
        assert!(
            response.starts_with(expected_status),
            "{} {} returned {response:?}",
            route.method,
            route.pattern
        );
        let body = response
            .split_once("\r\n\r\n")
            .map(|(_, body)| body)
            .expect("socket parity response separator");
        assert!(
            !body.is_empty(),
            "{} {} returned an empty body",
            route.method,
            route.pattern
        );
        serde_json::from_str::<serde_json::Value>(body).unwrap_or_else(|error| {
            panic!(
                "{} {} returned invalid JSON: {error}: {body}",
                route.method, route.pattern
            )
        });
    }
}

#[test]
fn unknown_and_retired_routes_have_explicit_http_errors() {
    let root = tempfile::tempdir().expect("runtime root");
    let runtime = Arc::new(
        NativeGatewayRuntime::bootstrap_with_anchor_for_test(root.path()).expect("keyed runtime"),
    );
    let options = test_gateway_options(false);
    let unknown = route_over_real_socket(
        Arc::clone(&runtime),
        options.clone(),
        "GET",
        "/api/not-a-hepta-route",
    );
    assert!(unknown.starts_with("HTTP/1.1 404 Not Found"));
    assert!(unknown.contains(r#""error":"runtime_ingress.route_not_found""#));

    let retired = CONTROL_UI_ROUTE_SPECS
        .iter()
        .find(|route| route.is_quarantined_transitive_effect())
        .expect("retired route");
    let retired = route_over_real_socket(
        runtime,
        options,
        retired.method,
        &control_ui_sample_path(retired.pattern),
    );
    assert!(retired.starts_with("HTTP/1.1 410 Gone"));
    assert!(retired.contains(r#""error":"runtime_ingress.route_retired""#));
}

#[test]
fn telegram_live_soak_aliases_share_real_socket_behavior() {
    let root = tempfile::tempdir().expect("runtime root");
    let runtime = Arc::new(
        NativeGatewayRuntime::bootstrap_with_anchor_for_test(root.path()).expect("keyed runtime"),
    );
    let options = test_gateway_options(false);
    let responses = TELEGRAM_LIVE_SOAK_ROUTE
        .paths()
        .map(|path| {
            route_over_real_socket(Arc::clone(&runtime), options.clone(), "GET", path)
        })
        .collect::<Vec<_>>();
    for response in &responses {
        assert!(response.starts_with("HTTP/1.1 200 OK"));
        serde_json::from_str::<serde_json::Value>(
            response.split_once("\r\n\r\n").expect("response body").1,
        )
        .expect("Telegram live-soak JSON");
    }
    assert_eq!(
        responses
            .iter()
            .map(|response| response.split_once("\r\n\r\n").expect("response body").1)
            .collect::<Vec<_>>(),
        vec![
            responses[0].split_once("\r\n\r\n").expect("canonical body").1;
            TELEGRAM_LIVE_SOAK_ROUTE.paths().count()
        ]
    );
}

#[test]
fn compact_watchdog_projection_is_passthrough_over_a_real_socket() {
    let root = tempfile::tempdir().expect("runtime root");
    let runtime = Arc::new(
        NativeGatewayRuntime::bootstrap_with_anchor_for_test(root.path()).expect("keyed runtime"),
    );
    let response = route_over_real_socket(
        runtime,
        test_gateway_options(false),
        "GET",
        WATCHDOG_STATE_ENDPOINT,
    );
    assert!(response.starts_with("HTTP/1.1 200 OK"));
    let body = response.split_once("\r\n\r\n").expect("response body").1;
    assert!(body.len() < report_pagination::MAX_DEFAULT_REPORT_BYTES);
    let value: serde_json::Value = serde_json::from_str(body).expect("watchdog state JSON");
    assert_eq!(value["schema_version"], "hepta_watchdog_state_v1");
    assert_ne!(value["schema_version"], "hepta_report_summary_v2");
    assert_eq!(value["route"]["missing_route_count"], 0);
    assert_eq!(value["poll"]["external_network_read_by_status"], false);
    assert_eq!(value["poll"]["external_send_by_status"], false);
}

#[test]
fn typed_report_pagination_is_digest_bound_over_real_sockets() {
    let root = tempfile::tempdir().expect("runtime root");
    let runtime = Arc::new(
        NativeGatewayRuntime::bootstrap_with_anchor_for_test(root.path()).expect("keyed runtime"),
    );
    let options = test_gateway_options(false);
    let summary_response = route_over_real_socket(
        Arc::clone(&runtime),
        options.clone(),
        "GET",
        "/api/operator-security",
    );
    assert!(summary_response.starts_with("HTTP/1.1 200 OK"));
    let summary_body = summary_response
        .split_once("\r\n\r\n")
        .expect("summary body")
        .1;
    let summary: serde_json::Value =
        serde_json::from_str(summary_body).expect("report summary JSON");
    assert_eq!(summary["schema"], "hepta_report_summary_v2");
    assert_eq!(summary["status"], "attention");
    let snapshot = summary["content_sha256"]
        .as_str()
        .expect("summary snapshot");

    let page_response = route_over_real_socket(
        Arc::clone(&runtime),
        options.clone(),
        "GET",
        &format!(
            "/api/operator-security?detail=full&cursor=0&snapshot={snapshot}"
        ),
    );
    assert!(page_response.starts_with("HTTP/1.1 200 OK"));
    let page: serde_json::Value = serde_json::from_str(
        page_response
            .split_once("\r\n\r\n")
            .expect("page body")
            .1,
    )
    .expect("report page JSON");
    assert_eq!(page["schema"], "hepta_report_page_v2");
    assert_eq!(page["status"], "attention");
    assert_eq!(page["content_sha256"], snapshot);

    let conflict = route_over_real_socket(
        runtime,
        options,
        "GET",
        &format!(
            "/api/operator-security?detail=full&cursor=0&snapshot={}",
            "0".repeat(64)
        ),
    );
    assert!(conflict.starts_with("HTTP/1.1 409 Conflict"));
    assert!(conflict.contains("report snapshot changed"));
}

#[test]
fn canonical_evidence_route_serves_legacy_reports_over_real_sockets() {
    let root = tempfile::tempdir().expect("runtime root");
    let runtime = Arc::new(
        NativeGatewayRuntime::bootstrap_with_anchor_for_test(root.path()).expect("keyed runtime"),
    );
    let selected = CONTROL_UI_ROUTE_SPECS
        .iter()
        .filter(|route| {
            route.method == "GET"
                && route.pattern != EVIDENCE_INDEX_ENDPOINT
                && route.receipt_state().is_some()
                && !route.is_quarantined_transitive_effect()
                && !route.pattern.contains('<')
        })
        .min_by_key(|route| route.pattern.len())
        .expect("legacy evidence route");
    let response = route_over_real_socket(
        Arc::clone(&runtime),
        test_gateway_options(false),
        "GET",
        &format!("{EVIDENCE_INDEX_ENDPOINT}?route={}", selected.pattern),
    );
    assert!(response.starts_with("HTTP/1.1 200 OK"));
    let body = response.split_once("\r\n\r\n").expect("response body").1;
    let value: serde_json::Value = serde_json::from_str(body).expect("evidence JSON");
    assert_eq!(value["schema"], "hepta_evidence_document_v1");
    assert_eq!(value["selected_route"], selected.pattern);
    assert_eq!(value["evidence"]["legacy_compatibility_route"], true);
    assert_eq!(value["source_http_status"], "200 OK");

    let index = serde_json::to_value(evidence_api::evidence_index_report())
        .expect("evidence index JSON");
    let evidence_id = index["entries"]
        .as_array()
        .expect("evidence entries")
        .iter()
        .find(|entry| entry["route"] == selected.pattern)
        .and_then(|entry| entry["evidence_id"].as_str())
        .expect("stable evidence id");
    let by_id = route_over_real_socket(
        runtime,
        test_gateway_options(false),
        "GET",
        &format!("{EVIDENCE_INDEX_ENDPOINT}?id={evidence_id}"),
    );
    assert!(by_id.starts_with("HTTP/1.1 200 OK"));
    let body = by_id.split_once("\r\n\r\n").expect("response body").1;
    let value: serde_json::Value = serde_json::from_str(body).expect("evidence JSON");
    assert_eq!(value["selected_evidence_id"], evidence_id);
    assert_eq!(value["selected_route"], selected.pattern);
}

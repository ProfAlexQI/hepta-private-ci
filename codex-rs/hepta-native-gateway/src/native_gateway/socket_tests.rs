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
    route_over_real_socket_with_telemetry(
        runtime,
        options,
        method,
        path,
        "",
        None,
    )
}

fn route_over_real_socket_with_telemetry(
    runtime: Arc<NativeGatewayRuntime>,
    options: NativeGatewayOptions,
    method: &str,
    path: &str,
    request_headers: &str,
    telemetry_root: Option<PathBuf>,
) -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("socket parity listener");
    let address = listener.local_addr().expect("socket parity address");
    let server = thread::spawn(move || {
        let (mut stream, _) = listener.accept().expect("socket parity accept");
        let mut handle = || {
            handle_native_gateway_connection(
                &mut stream,
                Instant::now() + Duration::from_secs(30),
                &options,
                &runtime,
            )
        };
        match telemetry_root {
            Some(root) => legacy_route_usage::with_test_state_root(&root, handle),
            None => handle(),
        }
        .expect("socket parity handler");
    });
    let mut client = TcpStream::connect(address).expect("socket parity client");
    client
        .set_read_timeout(Some(Duration::from_secs(30)))
        .expect("socket parity read timeout");
    write!(
        client,
        "{method} {path} HTTP/1.1\r\nHost: 127.0.0.1\r\n{request_headers}Connection: close\r\n\r\n"
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
fn legacy_route_telemetry_covers_all_states_without_request_identifiers() {
    let runtime_root = tempfile::tempdir().expect("runtime root");
    let telemetry_root = tempfile::tempdir().expect("telemetry root");
    let runtime = Arc::new(
        NativeGatewayRuntime::bootstrap_with_anchor_for_test(runtime_root.path())
            .expect("keyed runtime"),
    );
    let options = test_gateway_options(false);
    let definitions = crate::route_definition::route_definition_registry();
    let legacy_200 = definitions
        .iter()
        .find(|definition| {
            definition.legacy_compatibility_route
                && definition.dispatch_handler == RouteDispatchHandler::NativeGateway
                && definition.lifecycle.source != "control_ui_transitive_effect_quarantine"
        })
        .expect("legacy 200 route");
    let canonical_only = definitions
        .iter()
        .find(|definition| {
            definition.legacy_compatibility_route
                && definition.dispatch_handler == RouteDispatchHandler::RetiredCompatibility
                && definition.lifecycle.source != "control_ui_transitive_effect_quarantine"
        })
        .expect("canonical-only route");
    let quarantine = definitions
        .iter()
        .find(|definition| {
            definition.legacy_compatibility_route
                && definition.lifecycle.source == "control_ui_transitive_effect_quarantine"
        })
        .expect("quarantine route");
    let cases = [
        (
            legacy_200,
            "Sec-Fetch-Mode: cors\r\nUser-Agent: secret-browser-agent\r\n",
            "HTTP/1.1 200 OK",
            "legacy_200",
            "accepted",
            "browser",
        ),
        (
            canonical_only,
            "Accept: application/json\r\nX-Secret: secret-query-body-ip-ua\r\n",
            "HTTP/1.1 410 Gone",
            "canonical_only_gone_410",
            "accepted",
            "json_client",
        ),
        (
            quarantine,
            "User-Agent: secret-quarantine-agent\r\n",
            "HTTP/1.1 410 Gone",
            "quarantine_preflight_410",
            "rejected",
            "unclassified",
        ),
    ];

    for (definition, headers, expected_status, _, _, _) in cases {
        let response = route_over_real_socket_with_telemetry(
            Arc::clone(&runtime),
            options.clone(),
            definition.lifecycle.method,
            &control_ui_sample_path(definition.lifecycle.path_pattern),
            headers,
            Some(telemetry_root.path().to_path_buf()),
        );
        assert!(response.starts_with(expected_status), "{response:?}");
    }

    let telemetry_path = telemetry_root
        .path()
        .join(legacy_route_usage::telemetry_relative_path());
    let all_events = fs::read_to_string(telemetry_path)
        .expect("legacy route telemetry JSONL")
        .lines()
        .map(|line| serde_json::from_str::<serde_json::Value>(line).expect("telemetry event"))
        .collect::<Vec<_>>();
    let events = all_events
        .iter()
        .filter(|event| event["event_type"] == "legacy_request")
        .collect::<Vec<_>>();
    assert_eq!(events.len(), cases.len());
    for (event, (definition, _, _, route_state, preflight, consumer_class)) in
        events.iter().zip(cases)
    {
        assert_eq!(event["route_key"], definition.lifecycle.path_pattern);
        assert_eq!(event["route_state"], route_state);
        assert_eq!(event["consumer_class"], consumer_class);
        assert_eq!(event["preflight"], preflight);
        assert_eq!(event["write_result"], "ok");
        assert_eq!(event["schema"], "hepta_control_ui_legacy_http_event_v1");
        assert_eq!(event["observation_complete"], true);
        assert!(event["sequence"].as_u64().is_some());
        assert_eq!(
            event["process_run_identifier_sha256"]
                .as_str()
                .map(str::len),
            Some(64)
        );
        assert!(event["time_unix_ms"].as_u64().is_some());
        assert_eq!(event["process_class"], "hepta_native_gateway");
        assert!(matches!(event["run_class"].as_str(), Some("operator" | "ci_test")));
        assert!(!event["head_sha"].as_str().unwrap_or_default().is_empty());
        assert_eq!(event["catalog_sha"].as_str().map(str::len), Some(64));
    }
    assert_eq!(events[0]["http_status"], 200);
    assert_eq!(events[1]["http_status"], 410);
    assert_eq!(events[2]["http_status"], 410);
    let jsonl = serde_json::to_string(&events).expect("telemetry events");
    for forbidden in [
        "secret-browser-agent",
        "secret-query-body-ip-ua",
        "secret-quarantine-agent",
        "user-agent",
        "127.0.0.1",
    ] {
        assert!(!jsonl.contains(forbidden), "persisted forbidden value {forbidden}");
    }
}

#[test]
fn rejection_writer_records_actual_503_without_route_state_fallback() {
    let telemetry_root = tempfile::tempdir().expect("telemetry root");
    let definition = crate::route_definition::route_definition_registry()
        .into_iter()
        .find(|definition| {
            definition.legacy_compatibility_route
                && definition.lifecycle.method == "GET"
                && definition.dispatch_handler == RouteDispatchHandler::NativeGateway
        })
        .expect("legacy route");
    let path = control_ui_sample_path(definition.lifecycle.path_pattern);
    let request = format!("GET {path} HTTP/1.1\r\nHost: 127.0.0.1\r\n\r\n");
    let listener = TcpListener::bind("127.0.0.1:0").expect("rejection listener");
    let address = listener.local_addr().expect("rejection address");
    let root = telemetry_root.path().to_path_buf();
    let server = thread::spawn(move || {
        let (mut stream, _) = listener.accept().expect("rejection accept");
        legacy_route_usage::with_test_state_root(&root, || {
            legacy_route_usage::begin_request(&request, "GET", &path);
            legacy_route_usage::record_preflight(legacy_route_usage::PreflightResult::Invalid);
            let result = http_rejections::response(
                &mut stream,
                "503 Service Unavailable",
                "application/json; charset=utf-8",
                br#"{"error":"runtime request preflight invalid"}"#,
            );
            legacy_route_usage::finish_request(&result);
            result
        })
        .expect("write rejection response");
    });
    let mut client = TcpStream::connect(address).expect("rejection client");
    client
        .shutdown(Shutdown::Write)
        .expect("rejection request shutdown");
    let mut response = String::new();
    client
        .read_to_string(&mut response)
        .expect("rejection response");
    server.join().expect("rejection server");
    assert!(response.starts_with("HTTP/1.1 503 Service Unavailable"));

    let telemetry_path = telemetry_root
        .path()
        .join(legacy_route_usage::telemetry_relative_path());
    let contents = fs::read_to_string(telemetry_path).expect("rejection telemetry");
    let event = contents
        .lines()
        .map(|line| serde_json::from_str::<serde_json::Value>(line).expect("telemetry event"))
        .find(|event| event["event_type"] == "legacy_request")
        .expect("rejection telemetry event");
    assert_eq!(event["http_status"], 503);
    assert_eq!(event["write_result"], "ok");
    assert_eq!(event["observation_complete"], true);
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
        let expected_status = if route_manifest_entry(route.method, &path)
            .is_some_and(|entry| entry.dispatch_handler == RouteDispatchHandler::RetiredCompatibility)
        {
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
fn ndu_h1_status_is_read_only_over_real_socket() {
    let root = tempfile::tempdir().expect("runtime root");
    let runtime = Arc::new(
        NativeGatewayRuntime::bootstrap_with_ndu_for_test(root.path()).expect("NDU runtime"),
    );
    let response = route_over_real_socket(
        runtime,
        test_gateway_options(false),
        "GET",
        crate::runtime_composition::NDU_H1_STATUS_ENDPOINT,
    );
    assert!(response.starts_with("HTTP/1.1 200 OK"));
    let body = response.split_once("\r\n\r\n").unwrap().1;
    let status: serde_json::Value = serde_json::from_str(body).unwrap();
    assert_eq!(status["enabled"], true);
    assert_eq!(status["ready"], true);
    assert_eq!(status["shadow_only"], true);
    assert_eq!(status["production_authority_granted"], false);
    assert_eq!(status["observed_event_count"], 0);
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
        Arc::clone(&runtime),
        options.clone(),
        retired.method,
        &control_ui_sample_path(retired.pattern),
    );
    assert!(retired.starts_with("HTTP/1.1 410 Gone"));
    assert!(retired.contains(r#""error":"runtime_ingress.route_retired""#));

    let canonical_only = CONTROL_UI_ROUTE_SPECS
        .iter()
        .find(|route| {
            !route.is_quarantined_transitive_effect()
                && route_manifest_entry(route.method, route.pattern).is_some_and(|entry| {
                    entry.dispatch_handler == RouteDispatchHandler::RetiredCompatibility
                })
        })
        .expect("canonical-only retired route");
    let canonical_only = route_over_real_socket(
        runtime,
        options,
        canonical_only.method,
        &control_ui_sample_path(canonical_only.pattern),
    );
    assert!(canonical_only.starts_with("HTTP/1.1 410 Gone"));
    assert!(canonical_only.contains(r#""error":"runtime_ingress.route_retired""#));
    assert!(canonical_only.contains(r#""reason":"legacy_evidence_route_is_canonical_only""#));
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

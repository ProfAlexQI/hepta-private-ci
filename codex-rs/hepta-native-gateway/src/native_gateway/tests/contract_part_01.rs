use super::*;

#[test]
fn parse_serve_ui_defaults_to_loopback() {
    let args = vec!["--serve-ui".to_string()];
    let options = parse_serve_ui_args(&args)
        .expect("parse")
        .expect("serve ui options");
    assert_eq!(options.bind_addr, DEFAULT_BIND_ADDR);
    assert!(!options.with_telegram_plugin);
    assert_eq!(options.telegram_plugin_poll_ms, DEFAULT_TELEGRAM_POLL_MS);
}

#[test]
fn parse_serve_ui_accepts_launchd_gateway_flags() {
    let args = vec![
        "--serve-ui".to_string(),
        "127.0.0.1:7777".to_string(),
        "--with-telegram-plugin".to_string(),
        "--telegram-plugin-poll-ms".to_string(),
        "250".to_string(),
    ];
    let options = parse_serve_ui_args(&args)
        .expect("parse")
        .expect("serve ui options");
    assert_eq!(options.bind_addr, "127.0.0.1:7777");
    assert!(options.with_telegram_plugin);
    assert_eq!(options.telegram_plugin_poll_ms, 500);
}

#[test]
fn parse_serve_ui_rejects_unknown_args() {
    let args = vec!["--serve-ui".to_string(), "--unknown".to_string()];
    let err = parse_serve_ui_args(&args).expect_err("unknown arg should fail");
    assert!(err.to_string().contains("unexpected --serve-ui argument"));
}

fn test_gateway_options(with_telegram_plugin: bool) -> NativeGatewayOptions {
    NativeGatewayOptions {
        bind_addr: DEFAULT_BIND_ADDR.to_string(),
        with_telegram_plugin,
        telegram_plugin_poll_ms: DEFAULT_TELEGRAM_POLL_MS,
    }
}

pub(super) fn quarantined_preflight() -> RuntimeRequestPreflightReceipt {
    RuntimeRequestPreflightReceipt {
        request_binding_hash: "quarantined-test-request-binding".into(),
        disposition: crate::runtime_composition::RuntimeRequestDisposition::PlanOnlyQuarantine,
        ingress_kind: crate::runtime_ingress::RuntimeIngressKind::MetadataRead,
        mutation_authorized: false,
        durable_intent_recorded: false,
        provider_effect_ack_recorded: false,
        terminal_receipt_recorded: false,
    }
}

#[test]
fn bounded_worker_pool_keeps_a_fast_rejection_responsive_during_a_slow_read() {
    use std::io::Read;
    use std::io::Write;
    use std::time::Duration;
    let options = test_gateway_options(false);
    let runtime_root = tempfile::tempdir().expect("runtime root");
    let runtime = Arc::new(
        NativeGatewayRuntime::bootstrap_with_anchor_for_test(runtime_root.path())
            .expect("keyed runtime"),
    );
    let pool = NativeGatewayConnectionPool::new(options, runtime, 2, 2).expect("worker pool");
    let listener = TcpListener::bind("127.0.0.1:0").expect("listener");
    let address = listener.local_addr().expect("address");
    let slow_client = TcpStream::connect(address).expect("slow client");
    let (slow_server, _) = listener.accept().expect("slow server");
    pool.dispatch(slow_server)
        .expect("dispatch slow connection");

    let mut fast_client = TcpStream::connect(address).expect("fast client");
    fast_client
        .set_read_timeout(Some(Duration::from_secs(1)))
        .expect("fast read timeout");
    let (fast_server, _) = listener.accept().expect("fast server");
    pool.dispatch(fast_server)
        .expect("dispatch fast connection");
    write!(
        fast_client,
        "POST / HTTP/1.1\r\ncontent-length: {}\r\n\r\n",
        MAX_HTTP_BODY_BYTES + 1
    )
    .expect("write oversized request");

    let mut response = String::new();
    fast_client
        .read_to_string(&mut response)
        .expect("fast response");
    assert!(response.starts_with("HTTP/1.1 413 Payload Too Large"));

    drop(slow_client);
    drop(pool);
}

#[test]
fn full_connection_queue_returns_503_under_the_short_overload_write_budget() {
    use std::io::Read;
    use std::time::Duration;
    use std::time::Instant;

    let (sender, _receiver) = mpsc::sync_channel(0);
    let pool = NativeGatewayConnectionPool { sender };
    let listener = TcpListener::bind("127.0.0.1:0").expect("listener");
    let mut client =
        TcpStream::connect(listener.local_addr().expect("address")).expect("overload client");
    client
        .set_read_timeout(Some(Duration::from_secs(1)))
        .expect("client read timeout");
    let (server, _) = listener.accept().expect("overload server");
    let started = Instant::now();

    pool.dispatch(server).expect("overload response");
    let mut response = String::new();
    client.read_to_string(&mut response).expect("503 response");

    assert!(response.starts_with("HTTP/1.1 503 Service Unavailable"));
    assert!(HTTP_OVERLOAD_WRITE_TIMEOUT <= Duration::from_millis(250));
    assert!(started.elapsed() < Duration::from_secs(1));
}

#[test]
fn control_ui_glass_asset_is_sha_bound_over_a_real_socket() {
    use std::io::Read;
    use std::io::Write;

    let options = test_gateway_options(false);
    let runtime_root = tempfile::tempdir().expect("runtime root");
    let runtime = Arc::new(
        NativeGatewayRuntime::bootstrap_with_anchor_for_test(runtime_root.path())
            .expect("keyed runtime"),
    );
    let pool = NativeGatewayConnectionPool::new(options, runtime, 1, 1).expect("worker pool");
    let listener = TcpListener::bind("127.0.0.1:0").expect("listener");
    let mut client =
        TcpStream::connect(listener.local_addr().expect("address")).expect("client");
    let (server, _) = listener.accept().expect("server");
    pool.dispatch(server).expect("dispatch asset request");
    write!(
        client,
        "GET /assets/k.png HTTP/1.1\r\nhost: {DEFAULT_BIND_ADDR}\r\n\r\n"
    )
    .expect("request");

    let mut response = Vec::new();
    client.read_to_end(&mut response).expect("asset response");
    let header_end = response
        .windows(4)
        .position(|window| window == b"\r\n\r\n")
        .map(|index| index + 4)
        .expect("HTTP header terminator");
    let headers = std::str::from_utf8(&response[..header_end]).expect("ASCII headers");
    let body = &response[header_end..];
    assert!(headers.starts_with("HTTP/1.1 200 OK\r\n"));
    assert!(headers.contains("content-type: image/png\r\n"));
    assert!(headers.contains("content-length: 2499731\r\n"));
    assert!(headers.contains("cache-control: public, max-age=3600, must-revalidate\r\n"));
    assert!(headers.contains(
        "etag: \"sha256-a54bc0d6352c3130d2d22b7df80f1fabaa94f5098fec12046e4f262e6d0d7c28\"\r\n"
    ));
    assert_eq!(body.len(), 2_499_731);
    assert_eq!(
        format!("{:x}", Sha256::digest(body)),
        hepta_core::control_ui::CONTROL_UI_GLASS_K_PNG_SHA256
    );
}

#[test]
fn overload_response_write_failure_is_connection_local() {
    use std::net::Shutdown;

    let (sender, _receiver) = mpsc::sync_channel(0);
    let pool = NativeGatewayConnectionPool { sender };
    let listener = TcpListener::bind("127.0.0.1:0").expect("listener");
    let client = TcpStream::connect(listener.local_addr().expect("address")).expect("client");
    let (server, _) = listener.accept().expect("server");
    server
        .shutdown(Shutdown::Write)
        .expect("disable server writes");

    pool.dispatch(server)
        .expect("one failed overload response must not terminate admission");
    drop(client);
}

#[test]
fn disconnected_worker_pool_remains_gateway_fatal() {
    let (sender, receiver) = mpsc::sync_channel(1);
    drop(receiver);
    let pool = NativeGatewayConnectionPool { sender };
    let listener = TcpListener::bind("127.0.0.1:0").expect("listener");
    let client = TcpStream::connect(listener.local_addr().expect("address")).expect("client");
    let (server, _) = listener.accept().expect("server");

    let error = pool
        .dispatch(server)
        .expect_err("a disconnected worker pool is process-fatal");
    assert!(error.to_string().contains("worker pool disconnected"));
    drop(client);
}

#[test]
fn gate_command_lists_the_declarative_registry_without_executing_reports() {
    let value: serde_json::Value = serde_json::from_str(
        &gate_command_json(&["--list".to_string()]).expect("gate registry json"),
    )
    .expect("gate registry value");

    assert_eq!(value["status"], "ready");
    assert_eq!(value["runner"], "hepta gate");
    assert_eq!(value["gate_count"], CONTROL_UI_ROUTE_SPECS.len());
    assert_eq!(
        value["ingress_lifecycle_registry_schema_version"],
        crate::runtime_ingress::RUNTIME_INGRESS_REGISTRY_SCHEMA_VERSION
    );
    assert_eq!(
        value["ingress_lifecycle_registry_sha256"]
            .as_str()
            .map(str::len),
        Some(64)
    );
    assert_eq!(value["quarantined_effect_route_count"], 28);
    assert_eq!(value["release_dispatch_ready"], false);
    assert_eq!(value["report_execution_performed"], false);
    assert_eq!(value["side_effect_free"], true);
    assert_eq!(
        value["receipt_state_machine"],
        serde_json::json!([
            "precondition",
            "denial",
            "receipt",
            "persistence",
            "retention",
            "terminal"
        ])
    );
}

#[test]
fn gate_command_resolves_capability_or_endpoint_id_as_registry_metadata() {
    let by_capability: serde_json::Value = serde_json::from_str(
        &gate_command_json(&["hepta-full-live-activation-closure-index".to_string()])
            .expect("gate spec by capability"),
    )
    .expect("gate spec value");
    let by_endpoint: serde_json::Value = serde_json::from_str(
        &gate_command_json(&[
            "hepta-memory-intelligence-kg-activation-truth-index".to_string(),
            "--json".to_string(),
        ])
        .expect("gate spec by endpoint id"),
    )
    .expect("endpoint gate spec value");

    assert_eq!(
        by_capability["pattern"],
        HEPTA_FULL_LIVE_ACTIVATION_CLOSURE_INDEX_ENDPOINT
    );
    assert_eq!(by_capability["receipt_state"], serde_json::Value::Null);
    assert_eq!(by_capability["report_execution_performed"], false);
    assert_eq!(
        by_endpoint["pattern"],
        HEPTA_MEMORY_INTELLIGENCE_KG_ACTIVATION_TRUTH_INDEX_ENDPOINT
    );
    assert_eq!(
        by_endpoint["registered_route_count"],
        CONTROL_UI_ROUTE_SPECS.len()
    );
}

#[test]
fn gate_command_resolves_a_migrated_shell_pair_from_the_declarative_specs() {
    let id = "hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-signing-final-ack-readback";
    let value: serde_json::Value = serde_json::from_str(
        &gate_command_json(&[id.to_string(), "--json".to_string()])
            .expect("migrated gate pair spec"),
    )
    .expect("migrated gate pair value");

    assert_eq!(value["mode"], "declarative_shell_pair_migration");
    assert_eq!(value["id"], id);
    assert_eq!(value["receipt_state"], "terminal");
    assert_eq!(value["report_execution_performed"], false);
    assert_eq!(value["side_effect_free"], true);
}

#[test]
fn gate_command_rejects_unknown_ids() {
    let err =
        gate_command_json(&["missing-gate".to_string()]).expect_err("unknown gate should fail");
    assert!(err.to_string().contains("unknown Hepta gate id"));
}

#[test]
fn native_gateway_readiness_exposes_pending_telegram_migration() {
    let options = test_gateway_options(true);
    let telegram_plugin =
        native_telegram::telegram_plugin_status(true, options.telegram_plugin_poll_ms);
    let body = native_gateway_json(&options, &telegram_plugin);
    assert!(body.contains(r#""runtime":"hepta""#));
    assert!(body.contains(r#""launchd_entrypoint_compatible":true"#));
    assert!(body.contains(r#""active_gateway_replacement_ready":false"#));
    assert!(body.contains(
        r#""gateway_replacement_readiness_endpoint":"/api/gateway-replacement-readiness""#
    ));
    assert!(body.contains(r#""gateway_route_core_status":{"source_crate":"hepta-gateway""#));
    assert!(body.contains(r#""route_core_ready":true"#));
    assert!(body.contains(
        r#""gateway_live_activation_plan_endpoint":"/api/gateway-live-activation-plan""#
    ));
    assert!(body.contains(r#""operator_approval_required":true"#));
    assert!(body.contains(r#""control_ui_route_parity_endpoint":"/api/control-ui-route-parity""#));
    assert!(body.contains(r#""control_ui_route_parity_ready":true"#));
    assert!(body.contains(r#""status":"blocked""#));
    assert!(body.contains(r#""active_install_allowed":false"#));
    assert!(body.contains(r#""telegram_model_runner_plan_ready""#));
    assert!(body.contains(r#""release_build_verified""#));
    assert!(body.contains(r#""control_ui_route_matrix_ready""#));
    assert!(body.contains(r#""control_ui_route_parity_verified""#));
    assert!(body.contains(r#""telegram_plugin_native_supervisor_ready":"#));
    assert!(body.contains(r#""telegram_receive_once_endpoint":"/api/telegram-receive-once""#));
    assert!(body.contains(r#""telegram_model_bridge_endpoint":"/api/telegram-model-bridge""#));
    assert!(body.contains(r#""telegram_send_plan_endpoint":"/api/telegram-send-plan""#));
    assert!(body.contains(r#""telegram_drain_once_endpoint":"/api/telegram-drain-once""#));
    assert!(body.contains(r#""telegram_poll_loop_endpoint":"/api/telegram-poll-loop""#));
    assert!(body.contains(r#""telegram_live_soak_endpoint":"/api/telegram-live-soak""#));
    assert!(
        body.contains(r#""telegram_live_soak_status_endpoint":"/api/telegram-live-soak-status""#)
    );
    assert!(body.contains(
        r#""telegram_production_readiness_endpoint":"/api/telegram-production-readiness""#
    ));
    assert!(
        body.contains(r#""telegram_delivery_ledger_endpoint":"/api/telegram-delivery-ledger""#)
    );
    assert!(body.contains(r#""telegram_owner_handoff_endpoint":"/api/telegram-owner-handoff""#));
    assert!(body.contains(r#""side_effect_free":true"#));
    assert!(body.contains(r#""production_guards""#));
    assert!(body.contains(r#""poll_loop_gate_env":"HEPTA_NATIVE_TELEGRAM_POLL_LOOP""#));
    assert!(body.contains(r#""worker_spawned_by_status":false"#));
    assert!(body.contains(r#""telegram_cursor_endpoint":"/api/telegram-cursor""#));
    assert!(body.contains(r#""telegram_readiness_summary_side_effect_free":true"#));
    assert!(body.contains(r#""readiness_summary_performs_live_read":false"#));
    assert!(body.contains(r#""readiness_summary_invokes_model":false"#));
    assert!(body.contains(r#""readiness_summary_sends_message":false"#));
    assert!(!body.contains("pending_migration"));
}

#[test]
fn telegram_live_soak_endpoint_is_side_effect_free() {
    let options = test_gateway_options(true);
    let mut canonical_body = None;
    for path in TELEGRAM_LIVE_SOAK_ROUTE.paths() {
        let (status, content_type, body) = route_native_gateway_request("GET", path, &options);
        assert_eq!(status, "200 OK", "live soak route failed: {path}");
        assert_eq!(content_type, "application/json; charset=utf-8");
        if let Some(canonical_body) = &canonical_body {
            assert_eq!(&body, canonical_body, "live soak alias drift: {path}");
        } else {
            canonical_body = Some(body.clone());
        }

        let value: serde_json::Value = serde_json::from_str(&body).expect("live soak json");
        assert_eq!(value["runtime"], "hepta");
        assert_eq!(value["side_effect_free"], true);
        assert_eq!(value["raw_update_payload_exposed"], false);
        assert_eq!(value["raw_prompt_text_exposed"], false);
        assert_eq!(value["raw_response_text_exposed"], false);
        assert_eq!(value["raw_token_exposed"], false);
        assert_eq!(value["poll_loop_status"]["worker_spawned_by_status"], false);
        assert_eq!(
            value["production_guards"]["retry_transient_send_errors"],
            true
        );
    }
}

#[test]
fn telegram_production_readiness_endpoint_is_side_effect_free() {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) =
        route_native_gateway_request("GET", TELEGRAM_PRODUCTION_READINESS_ENDPOINT, &options);
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body).expect("production readiness json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["side_effect_free"], true);
    assert_eq!(value["raw_update_payload_exposed"], false);
    assert_eq!(value["raw_prompt_text_exposed"], false);
    assert_eq!(value["raw_response_text_exposed"], false);
    assert_eq!(value["raw_token_exposed"], false);
    assert_eq!(
        value["min_poll_iterations_env"],
        "HEPTA_NATIVE_TELEGRAM_SOAK_MIN_POLLS"
    );
    assert_eq!(
        value["max_attention_count_env"],
        "HEPTA_NATIVE_TELEGRAM_SOAK_MAX_ATTENTION"
    );
}

#[test]
fn telegram_delivery_ledger_endpoint_is_read_only_and_redacted() {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) =
        route_native_gateway_request("GET", TELEGRAM_DELIVERY_LEDGER_ENDPOINT, &options);
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body).expect("delivery ledger json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["requested"], true);
    assert_eq!(
        value["ledger_path"],
        ".hepta/telegram/delivery-ledger.jsonl"
    );
    assert_eq!(value["raw_response_text_logged"], false);
    assert_eq!(value["raw_chat_id_logged"], false);
    assert_eq!(value["raw_message_id_logged"], false);
    assert_eq!(value["raw_token_logged"], false);
}

#[test]
fn telegram_owner_handoff_endpoint_is_side_effect_free() {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) =
        route_native_gateway_request("GET", TELEGRAM_OWNER_HANDOFF_ENDPOINT, &options);
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body).expect("owner handoff json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["endpoint"], TELEGRAM_OWNER_HANDOFF_ENDPOINT);
    assert_eq!(value["side_effect_free"], true);
    assert_eq!(value["raw_token_exposed"], false);
    assert_eq!(value["raw_update_payload_exposed"], false);
    assert_eq!(value["raw_prompt_text_exposed"], false);
    assert_eq!(value["raw_response_text_exposed"], false);
    assert!(value["takeover_blockers"].is_array());
}

#[test]
fn telegram_owner_handoff_detects_double_poller_risk() {
    let status = telegram_owner_handoff_status_from_inputs(NativeTelegramOwnerHandoffInputs {
        legacy_config_path: Some("/tmp/openclaw.json".to_string()),
        legacy_config_found: true,
        legacy_config_parse_ok: true,
        legacy_telegram_enabled: Some(true),
        legacy_token_fingerprint: Some("sha256:samebot00000000".to_string()),
        legacy_config_error: None,
        hepta_token_fingerprint: Some("sha256:samebot00000000".to_string()),
        hepta_telegram_requested: true,
        hepta_poll_loop_armed: true,
        hepta_poll_loop_gate_enabled: true,
        hepta_delivery_approval_gate_enabled: true,
    });

    assert_eq!(status.status, "conflict_risk");
    assert_eq!(status.active_owner, "conflict_risk");
    assert!(!status.ready);
    assert!(!status.conflict_free);
    assert!(!status.hepta_takeover_ready);
    assert!(!status.hepta_parallel_bot_ready);
    assert!(status.double_poller_risk);
    assert_eq!(status.bot_identity_match, Some(true));
    assert!(!status.parallel_bot_mode);
    assert!(
        status
            .takeover_blockers
            .contains(&"legacy_openclaw_telegram_enabled")
    );
    assert!(!status.raw_token_exposed);
}

#[test]
fn telegram_owner_handoff_allows_hepta_only_after_legacy_disabled() {
    let status = telegram_owner_handoff_status_from_inputs(NativeTelegramOwnerHandoffInputs {
        legacy_config_path: Some("/tmp/openclaw.json".to_string()),
        legacy_config_found: true,
        legacy_config_parse_ok: true,
        legacy_telegram_enabled: Some(false),
        legacy_token_fingerprint: Some("sha256:legacy00000000".to_string()),
        legacy_config_error: None,
        hepta_token_fingerprint: Some("sha256:hepta000000000".to_string()),
        hepta_telegram_requested: true,
        hepta_poll_loop_armed: true,
        hepta_poll_loop_gate_enabled: true,
        hepta_delivery_approval_gate_enabled: true,
    });

    assert_eq!(status.status, "hepta_takeover_ready");
    assert_eq!(status.active_owner, "hepta");
    assert!(status.ready);
    assert!(status.conflict_free);
    assert!(status.hepta_takeover_ready);
    assert!(!status.hepta_parallel_bot_ready);
    assert!(!status.double_poller_risk);
    assert!(status.takeover_blockers.is_empty());
}

#[test]
fn telegram_owner_handoff_allows_distinct_parallel_bots() {
    let status = telegram_owner_handoff_status_from_inputs(NativeTelegramOwnerHandoffInputs {
        legacy_config_path: Some("/tmp/openclaw.json".to_string()),
        legacy_config_found: true,
        legacy_config_parse_ok: true,
        legacy_telegram_enabled: Some(true),
        legacy_token_fingerprint: Some("sha256:legacy00000000".to_string()),
        legacy_config_error: None,
        hepta_token_fingerprint: Some("sha256:hepta000000000".to_string()),
        hepta_telegram_requested: true,
        hepta_poll_loop_armed: true,
        hepta_poll_loop_gate_enabled: true,
        hepta_delivery_approval_gate_enabled: true,
    });

    assert_eq!(status.status, "parallel_bot_ready");
    assert_eq!(status.active_owner, "parallel_bots");
    assert!(status.ready);
    assert!(status.conflict_free);
    assert!(!status.hepta_takeover_ready);
    assert!(status.hepta_parallel_bot_ready);
    assert!(!status.double_poller_risk);
    assert_eq!(status.bot_identity_match, Some(false));
    assert!(status.parallel_bot_mode);
    assert!(
        !status
            .takeover_blockers
            .contains(&"legacy_openclaw_telegram_enabled")
    );
    assert!(!status.raw_token_exposed);
}

#[test]
fn gateway_replacement_readiness_endpoint_reports_blockers_without_side_effects() {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) =
        route_native_gateway_request("GET", "/api/gateway-replacement-readiness", &options);
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body).expect("readiness json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["ready"], false);
    assert_eq!(value["active_install_allowed"], false);
    assert_eq!(value["side_effect_free"], true);
    assert_eq!(
        value["required_env_gates"]["live_read"]["env"],
        native_telegram::TELEGRAM_LIVE_READ_ENV
    );
    let blockers = value["blockers"]
        .as_array()
        .expect("blockers")
        .iter()
        .filter_map(|value| value.as_str())
        .collect::<Vec<_>>();
    assert!(!blockers.contains(&"in_process_model_runner_ready"));
    assert!(!blockers.contains(&"telegram_model_runner_plan_ready"));
    assert!(blockers.contains(&"release_build_verified"));
    assert!(blockers.contains(&"control_ui_route_parity_verified"));
    assert!(!blockers.contains(&"control_ui_route_matrix_ready"));
    assert!(value["telegram_owner_handoff_status"].is_object());
    assert_eq!(
        value["telegram_owner_handoff_endpoint"],
        TELEGRAM_OWNER_HANDOFF_ENDPOINT
    );
    assert_eq!(value["control_ui_route_parity"]["ready"], true);
    assert!(
        value["control_ui_route_parity"]["route_count"]
            .as_u64()
            .expect("route count")
            >= 40
    );
}

#[test]
fn gateway_live_activation_plan_is_side_effect_free_and_lists_operator_gates() {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) =
        route_native_gateway_request("GET", GATEWAY_LIVE_ACTIVATION_PLAN_ENDPOINT, &options);
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");
    let value: serde_json::Value = serde_json::from_str(&body).expect("activation plan json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["operator_approval_required"], true);
    assert_eq!(value["active_gateway_label"], ACTIVE_GATEWAY_LABEL);
    assert_eq!(
        value["current_legacy_binary"],
        HEPTA_CODEX_TRANSITION_BINARY
    );
    assert_eq!(value["replacement_binary"], HEPTA_ACTIVE_RELEASE_BINARY);
    assert_eq!(
        value["safety"]["status_probe_reads_telegram"], false,
        "activation planning must not read Telegram"
    );
    assert_eq!(value["safety"]["status_probe_invokes_model"], false);
    assert_eq!(value["safety"]["status_probe_sends_message"], false);
    assert_eq!(value["safety"]["status_probe_writes_cursor"], false);
    let envs = value["required_env_gates"]
        .as_array()
        .expect("required env gates")
        .iter()
        .filter_map(|item| item["env"].as_str())
        .collect::<Vec<_>>();
    assert!(envs.contains(&native_telegram::TELEGRAM_DELIVERY_APPROVED_ENV));
    assert!(envs.contains(&native_telegram::TELEGRAM_LIVE_READ_ENV));
    assert!(envs.contains(&native_telegram::TELEGRAM_MODEL_TURN_GATE_ENV));
    assert!(envs.contains(&native_telegram::TELEGRAM_SEND_GATE_ENV));
    assert!(envs.contains(&native_telegram::TELEGRAM_POLL_LOOP_ENV));
    assert!(envs.contains(&native_telegram::TELEGRAM_IN_PROCESS_MODEL_RUNNER_ENV));
    assert!(envs.contains(&RELEASE_BUILD_VERIFIED_ENV));
    assert!(envs.contains(&CONTROL_UI_PARITY_VERIFIED_ENV));
}

#[test]
fn control_ui_route_parity_report_covers_old_hepta_routes() {
    let report = control_ui_route_parity_report();
    assert!(report.ready);
    assert_eq!(
        report.evidence_scope,
        "typed route registration, compatibility-handler serialization, production ingress availability, and real-socket test coverage"
    );
    assert!(!report.live_product_complete);
    assert_eq!(report.missing_route_count, 0);
    assert_eq!(report.quarantined_route_count, 28);
    assert_eq!(
        report.production_dispatchable_route_count + report.quarantined_route_count,
        report.implemented_route_count
    );
    assert!(report.route_count >= 40);
    let routes = report
        .routes
        .iter()
        .map(|route| format!("{} {}", route.method, route.pattern))
        .collect::<Vec<_>>();
    assert!(routes.contains(&"GET /api/operator-console".to_string()));
    assert!(routes.contains(&"GET /api/query-transcript/<query>".to_string()));
    assert!(routes.contains(&"POST /api/commands/<id>".to_string()));
    assert!(routes.contains(&"POST /api/actions/<action>".to_string()));
    assert!(routes.contains(&"POST /api/chat".to_string()));
    assert!(routes.contains(&"GET /api/external-agent-benchmark".to_string()));
    assert!(routes.contains(&"GET /api/telegram-production-readiness".to_string()));
    assert!(routes.contains(&"GET /api/telegram-delivery-ledger".to_string()));
    assert!(routes.contains(&"GET /api/hepta-merge-completion".to_string()));
    assert!(routes.contains(&"GET /api/hepta-cli-command-inventory".to_string()));
    assert!(routes.contains(&"GET /api/hepta-provider-metadata-inventory".to_string()));
    assert!(routes.contains(&"GET /api/hepta-runtime-session-dry-run-inventory".to_string()));
    assert!(routes.contains(&"GET /api/hepta-channel-adapter-status-inventory".to_string()));
    assert!(routes.contains(&"GET /api/hepta-local-tooling-content-inventory".to_string()));
    assert!(routes.contains(&"GET /api/hepta-memory-capability-absorption-inventory".to_string()));
    assert!(routes.contains(
        &"GET /api/hepta-memory-intelligence-kg-full-enablement-runtime-readiness".to_string()
    ));
    assert!(routes.contains(&"GET /api/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-readiness".to_string()));
    assert!(routes.contains(&"GET /api/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-controlled".to_string()));
    assert!(routes.contains(&"GET /api/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-controlled-readback-receipt-no-persistence".to_string()));
    assert!(routes.contains(&"GET /api/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-controlled-readback-receipt-authority-denial".to_string()));
    assert!(routes.contains(&"GET /api/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-controlled-readback-receipt-trusted-operator-packet-separation".to_string()));
    assert!(routes.contains(&"GET /api/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-controlled-readback-receipt-trusted-operator-packet-intake-precondition".to_string()));
    assert!(routes.contains(&"GET /api/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-controlled-readback-receipt-trusted-operator-packet-partial-precondition-denial-matrix".to_string()));
    assert!(routes.contains(&"GET /api/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-controlled-readback-receipt-trusted-operator-packet-complete-precondition-authority-denial".to_string()));
    assert!(routes.contains(&"GET /api/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-controlled-readback-receipt-trusted-operator-packet-complete-precondition-operator-approval-lane-separation".to_string()));
    assert!(routes.contains(&"GET /api/hepta-memory-intelligence-kg-full-enablement-operator-approved-memory-live-mutation-durable-lane".to_string()));
    assert!(routes.contains(&"GET /api/hepta-memory-intelligence-kg-full-enablement-operator-approved-hepta-intelligence-context-attachment-lane".to_string()));
    assert!(routes.contains(&"GET /api/hepta-memory-intelligence-kg-full-enablement-operator-approved-kg-prompt-preview-read-only-adapter-lane".to_string()));
    assert!(routes.contains(&"GET /api/hepta-memory-intelligence-kg-full-enablement-operator-approved-kg-prompt-payload-materialization-lane".to_string()));
    assert!(routes.contains(&"GET /api/hepta-memory-intelligence-kg-full-enablement-operator-approved-kg-prompt-payload-acceptance-receipt-lane".to_string()));
    assert!(routes.contains(&"GET /api/hepta-memory-intelligence-kg-full-enablement-operator-approved-kg-prompt-payload-readback-audit-receipt-lane".to_string()));
    assert!(routes.contains(&"GET /api/hepta-memory-intelligence-kg-full-enablement-operator-approved-context-handoff-acceptance-lane".to_string()));
    assert!(routes.contains(&"GET /api/hepta-memory-intelligence-kg-full-enablement-operator-approved-context-handoff-receipt-audit-lane".to_string()));
    assert!(routes.contains(&"GET /api/hepta-release-hardening-status-gate".to_string()));
    assert!(routes.contains(&"GET /api/hepta-provider-channel-dry-run-plan".to_string()));
    assert!(routes.contains(&"GET /api/hepta-native-packaging-gate".to_string()));
    assert!(routes.contains(&"GET /api/hepta-legacy-compatibility-closure".to_string()));
    assert!(routes.contains(&"GET /api/hepta-public-ga-operator-approval-packet".to_string()));
    assert!(routes.contains(&"GET /api/hepta-public-ga-readiness".to_string()));
}

#[test]
fn hepta_merge_completion_endpoint_returns_machine_readable_audit() {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) =
        route_native_gateway_request("GET", HEPTA_MERGE_COMPLETION_ENDPOINT, &options);
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body).expect("merge completion json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["source_command"], "/hepta-merge-completion --json");
    assert_eq!(value["compatibility_mode"], "native_merge_completion_audit");
    assert_eq!(
        value["readiness_class"],
        "static_contract_ready_production_in_progress"
    );
    assert_eq!(value["contract_valid"], true);
    assert_eq!(value["locally_executable"], true);
    assert_eq!(value["integration_verified"], false);
    assert_eq!(value["live_enabled"], false);
    assert_eq!(value["release_provenance_verified"], false);
    assert_eq!(value["active_binary_consistency_verified"], false);
    assert_eq!(value["production_ready"], false);
    assert!(
        value["blockers"]
            .as_array()
            .expect("merge completion blockers")
            .iter()
            .any(|blocker| blocker == "release_provenance_not_verified")
    );
    assert!(
        value["blockers"]
            .as_array()
            .expect("merge completion blockers")
            .iter()
            .any(|blocker| blocker == "active_binary_consistency_not_verified")
    );
    assert_eq!(value["source_package_merge_percent"], 100);
    assert_eq!(value["local_deterministic_function_percent"], 100);
    assert_eq!(value["active_service_coexistence_percent"], 100);
    assert!(
        value["production_replacement_percent"]
            .as_u64()
            .expect("production replacement percent")
            < 100
    );
    assert_eq!(
        value["control_ui_product_status"],
        "static_contract_complete"
    );
    assert_eq!(value["control_ui_product_complete"], false);
    assert_eq!(value["control_ui_live_operator_surface_percent"], 0);
    assert_eq!(value["control_ui_evidence"]["schema_version"], 1);
    assert_eq!(
        value["control_ui_evidence"]["static_contract"]["status"],
        "verified"
    );
    assert_eq!(
        value["control_ui_evidence"]["static_contract"]["coverage_percent"],
        100
    );
    for layer in [
        "unit_state",
        "browser_behavior",
        "backend_mutation_readback",
        "live_adapter",
    ] {
        assert_eq!(
            value["control_ui_evidence"][layer]["status"], "not_bound_to_report",
            "{layer}"
        );
        assert_eq!(
            value["control_ui_evidence"][layer]["coverage_percent"], 0,
            "{layer}"
        );
    }
    assert_eq!(value["control_ui_evidence"]["overall_evidence_percent"], 20);
    assert_eq!(
        value["control_ui_evidence"]["all_required_layers_verified"],
        false
    );
    assert_eq!(value["old_hepta_script_total"], 20);
    assert_eq!(
        value["current_hepta_codex_script_total"],
        CURRENT_HEPTA_CODEX_SCRIPT_TOTAL
    );
    assert_eq!(value["old_hepta_ops_file_count"], 65);
    assert_eq!(value["old_hepta_rough_command_reference_count"], 574);
    assert_eq!(
        value["native_gateway_source_command_count"],
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    assert_eq!(value["route_matrix_ready"], true);
    assert_eq!(value["missing_route_count"], 0);
    assert_eq!(value["merge_completion_control_ui_surfaced"], true);
    assert_eq!(value["merge_completion_gateway_index_surfaced"], true);
    assert_eq!(value["browser_visual_smoke_ready"], false);
    assert_eq!(
        value["browser_visual_smoke_command"],
        "scripts/hepta-browser-visual-smoke.sh"
    );
    assert!(
        value["route_count"].as_u64().expect("route count") >= 59,
        "merge-completion route should be included in parity count"
    );
    assert_eq!(value["production_owner_handoff_required"], true);
    assert_eq!(value["telegram_live_send_enabled"], false);
    assert_eq!(value["native_post_real_activation_enabled"], false);
    assert_eq!(value["public_ga_claimed"], false);
    assert_eq!(value["safe_continue_internal_work"], true);
    assert_eq!(value["side_effects"]["external_side_effects"], false);
    assert_eq!(value["side_effects"]["telegram_read_performed"], false);
    assert_eq!(value["side_effects"]["message_sent"], false);
    let blockers = value["blockers"]
        .as_array()
        .expect("blockers")
        .iter()
        .filter_map(|item| item.as_str())
        .collect::<Vec<_>>();
    assert!(blockers.contains(&"telegram_owner_handoff_not_requested"));
    assert!(blockers.contains(&"control_ui_product_behavior_evidence_not_bound"));
    assert!(!blockers.contains(&"old_hepta_cli_command_breadth_not_fully_migrated"));
    assert!(!blockers.contains(&"browser_visual_e2e_not_run_in_this_audit"));
    let mut next_actions = value["next_actions"]
        .as_array()
        .expect("next actions")
        .iter()
        .filter_map(|item| item.as_str());
    assert!(
        next_actions.any(|action| action
            == "keep browser visual smoke, preflight, soak, and watchdog gates green")
    );
}

#[test]
fn hepta_native_packaging_gate_reports_local_packaging_without_distribution_side_effects() {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) =
        route_native_gateway_request("GET", HEPTA_NATIVE_PACKAGING_GATE_ENDPOINT, &options);
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body).expect("native packaging gate json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(
        value["source_command"],
        "/hepta-native-packaging-gate --json"
    );
    assert_eq!(
        value["compatibility_mode"],
        "native_app_packaging_readiness_gate"
    );
    assert_eq!(
        value["current_hepta_codex_script_total"],
        CURRENT_HEPTA_CODEX_SCRIPT_TOTAL
    );
    assert_eq!(
        value["native_gateway_source_command_count"],
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    assert_eq!(value["missing_route_count"], 0);
    assert_eq!(value["rust_source_file_count"], 125);
    assert_eq!(value["packaging_resource_file_count"], 111);
    assert_eq!(
        value["rust_source_file_count_policy"],
        "minimum_floor_from_reviewed_manifest"
    );
    assert_eq!(
        value["packaging_resource_file_count_policy"],
        "minimum_floor_from_reviewed_manifest"
    );
    assert_eq!(value["ui_iteration_file_count_flexible"], true);
    assert_eq!(value["required_metadata_file_count"], 9);
    assert_eq!(value["cargo_metadata_gate_ready"], true);
    assert_eq!(value["package_metadata_ready"], true);
    assert_eq!(value["icon_resource_matrix_ready"], true);
    assert_eq!(value["dmg_helper_script_ready"], true);
    assert_eq!(value["android_resource_matrix_ready"], true);
    assert_eq!(value["ios_icon_matrix_ready"], true);
    assert_eq!(value["local_bridge_fixture_smoke_ready"], true);
    assert_eq!(value["local_native_test_gate_ready"], true);
    assert_eq!(value["local_packaging_gate_ready"], true);
    assert_eq!(value["signing_notarization_deferred"], true);
    assert_eq!(value["public_distribution_artifact_written"], false);
    assert_eq!(value["side_effects"]["release_artifact_written"], false);
    assert_eq!(value["side_effects"]["app_signed"], false);
    assert_eq!(value["side_effects"]["app_notarized"], false);
    assert_eq!(value["side_effects"]["credential_read"], false);
    assert_eq!(
        value["side_effects"]["telegram_owner_handoff_performed"],
        false
    );
}

#[test]
fn hepta_legacy_compatibility_closure_covers_old_cli_scripts_without_live_execution() {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) =
        route_native_gateway_request("GET", HEPTA_LEGACY_COMPATIBILITY_CLOSURE_ENDPOINT, &options);
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value =
        serde_json::from_str(&body).expect("legacy compatibility closure json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(
        value["source_command"],
        "/hepta-legacy-compatibility-closure --json"
    );
    assert_eq!(
        value["compatibility_mode"],
        "native_legacy_cli_script_compatibility_closure"
    );
    assert_eq!(
        value["current_hepta_codex_script_total"],
        CURRENT_HEPTA_CODEX_SCRIPT_TOTAL
    );
    assert_eq!(
        value["native_gateway_source_command_count"],
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    assert_eq!(value["missing_route_count"], 0);
    assert_eq!(value["old_hepta_ops_file_count"], 65);
    assert_eq!(value["old_hepta_rough_command_reference_count"], 574);
    assert_eq!(value["old_hepta_script_total"], 20);
    assert_eq!(value["ops_file_family_covered_count"], 65);
    assert_eq!(value["release_hardening_script_family_count"], 12);
    assert_eq!(value["release_hardening_status_gate_ready_count"], 12);
    assert_eq!(value["local_route_script_coverage_ready"], true);
    assert_eq!(value["old_cli_command_breadth_fully_migrated"], true);
    assert_eq!(
        value["old_release_hardening_script_execution_compatibility_claimed"],
        true
    );
    assert_eq!(value["dangerous_live_execution_reenabled"], false);
    assert_eq!(value["credentialed_live_smoke_deferred"], true);
    assert_eq!(value["external_release_deferred"], true);
    assert_eq!(value["side_effects"]["process_spawned"], false);
    assert_eq!(value["side_effects"]["credential_read"], false);
    assert_eq!(value["side_effects"]["provider_invoked"], false);
    assert_eq!(value["side_effects"]["channel_send_performed"], false);
    assert_eq!(value["side_effects"]["telegram_send_performed"], false);
    assert_eq!(
        value["side_effects"]["native_post_mutation_performed"],
        false
    );
}

#[test]
fn hepta_public_ga_readiness_endpoint_blocks_public_claims_without_side_effects() {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) =
        route_native_gateway_request("GET", HEPTA_PUBLIC_GA_READINESS_ENDPOINT, &options);
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body).expect("public ga readiness json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["source_command"], "/hepta-public-ga-readiness --json");
    assert_eq!(
        value["compatibility_mode"],
        "native_public_ga_readiness_gate"
    );
    assert_eq!(value["side_effect_free"], true);
    assert_eq!(value["public_ga_ready"], false);
    assert_eq!(value["public_ga_claimed"], false);
    assert_eq!(value["external_public_release_performed"], false);
    assert_eq!(
        value["current_hepta_codex_script_total"],
        CURRENT_HEPTA_CODEX_SCRIPT_TOTAL
    );
    assert_eq!(
        value["native_gateway_source_command_count"],
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    assert_eq!(value["missing_route_count"], 0);
    assert_eq!(value["local_reports_synchronized"], true);
    assert_eq!(value["local_gate_matrix_ready"], true);
    assert_eq!(
        value["control_ui_product_status"],
        "static_contract_complete"
    );
    assert_eq!(value["control_ui_product_complete"], false);
    assert_eq!(value["control_ui_live_operator_surface_percent"], 0);
    assert_eq!(value["control_ui_overall_evidence_percent"], 20);
    assert!(
        value["production_replacement_percent"]
            .as_u64()
            .expect("production replacement percent")
            < 100
    );
    assert_eq!(value["native_post_dry_run_evidence_ready"], true);
    assert_eq!(value["native_post_real_activation_ready"], false);
    assert_eq!(value["credentialed_provider_smoke_ready"], false);
    assert_eq!(value["channel_live_delivery_ready"], false);
    assert_eq!(value["hepta_native_release_packaging_ready"], true);
    let endpoints = value["readiness_evidence_endpoints"]
        .as_array()
        .expect("evidence endpoints")
        .iter()
        .filter_map(|item| item.as_str())
        .collect::<Vec<_>>();
    assert!(endpoints.contains(&HEPTA_MERGE_COMPLETION_ENDPOINT));
    assert!(endpoints.contains(&HEPTA_PROVIDER_CHANNEL_DRY_RUN_PLAN_ENDPOINT));
    assert!(endpoints.contains(&HEPTA_NATIVE_PACKAGING_GATE_ENDPOINT));
    assert!(endpoints.contains(&HEPTA_LEGACY_COMPATIBILITY_CLOSURE_ENDPOINT));
    assert!(endpoints.contains(&NATIVE_POST_ACTIVATION_PLAN_ENDPOINT));
    let blockers = value["blockers"]
        .as_array()
        .expect("blockers")
        .iter()
        .filter_map(|item| item.as_str())
        .collect::<Vec<_>>();
    assert!(!blockers.contains(&"old_hepta_cli_command_breadth_not_fully_migrated"));
    assert!(
        !blockers.contains(&"old_release_hardening_script_execution_compatibility_not_claimed")
    );
    assert!(blockers.contains(&"native_post_real_activation_not_operator_approved"));
    assert!(blockers.contains(&"control_ui_product_behavior_evidence_not_bound"));
    assert!(!blockers.contains(&"hepta_native_release_packaging_not_complete"));
    assert_eq!(value["side_effects"]["public_release_published"], false);
    assert_eq!(value["side_effects"]["credential_read"], false);
    assert_eq!(value["side_effects"]["provider_invoked"], false);
    assert_eq!(value["side_effects"]["telegram_read_performed"], false);
    assert_eq!(value["side_effects"]["telegram_send_performed"], false);
    assert_eq!(
        value["side_effects"]["native_post_mutation_performed"],
        false
    );
}

#[test]
fn hepta_core_fusion_readiness_reports_hepta_root_without_side_effects() {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) =
        route_native_gateway_request("GET", HEPTA_CORE_FUSION_READINESS_ENDPOINT, &options);
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body).expect("core fusion readiness json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(
        value["source_command"],
        HEPTA_CORE_FUSION_READINESS_SOURCE_COMMAND
    );
    assert_eq!(
        value["compatibility_mode"],
        "hepta_root_ownership_inversion_with_engine_adapter_boundary"
    );
    assert_eq!(value["side_effect_free"], true);
    assert_eq!(value["phase"], "phase_5_engine_dependency_closure");
    assert_eq!(value["root_owner"], "hepta");
    assert_eq!(value["product_runtime_owner"], "hepta-runtime");
    assert_eq!(value["gateway_owner"], "hepta-gateway");
    assert_eq!(value["engine_adapter_owner"], "codex-engine-adapter");
    assert_eq!(value["codex_engine_role"], "internal_engine_adapter");
    assert_eq!(value["phase_1_root_ownership_inversion_ready"], true);
    assert_eq!(value["phase_2_engine_adapter_boundary_ready"], true);
    assert_eq!(value["phase_3_binary_package_inversion_ready"], true);
    assert_eq!(
        value["binary_package_inversion_gate"],
        "hepta_first_class_binary_package_inversion_gate"
    );
    assert_eq!(value["binary_package_inversion_gate_ready"], true);
    assert_eq!(
        value["binary_package_inversion_gate_status"],
        "ready_hepta_cli_release_package_ownership_active"
    );
    assert_eq!(value["active_binary_package"], "hepta-cli");
    assert_eq!(value["active_binary_target"], "hepta");
    assert_eq!(value["intended_binary_package"], "hepta-cli");
    assert_eq!(value["intended_binary_target"], "hepta");
    assert_eq!(
        value["binary_package_inversion_blockers"]
            .as_array()
            .expect("binary package inversion blockers")
            .len(),
        0
    );
    assert_eq!(
        value["phase_4_name_repository_closure_gate"],
        "hepta_name_repository_closure_gate"
    );
    assert_eq!(value["phase_4_name_repository_closure_gate_ready"], true);
    assert_eq!(
        value["phase_4_name_repository_closure_gate_status"],
        "ready_phase_4_transition_names_closed"
    );
    assert_eq!(
        value["phase_4_name_repository_closure_remaining_surface_count"]
            .as_u64()
            .expect("phase 4 remaining surface count"),
        0
    );
    assert!(
        value["phase_4_name_repository_closure_blockers"]
            .as_array()
            .expect("phase 4 blockers")
            .is_empty()
    );
    assert_eq!(value["phase_4_name_repository_closure_ready"], true);
    assert_eq!(
        value["phase_5_engine_dependency_closure_gate"],
        "hepta_engine_dependency_closure_gate"
    );
    assert_eq!(value["phase_5_engine_dependency_closure_gate_ready"], true);
    assert_eq!(
        value["phase_5_engine_dependency_closure_gate_status"],
        "ready_active_hepta_service_binary_direct_codex_dependencies_closed"
    );
    assert_eq!(
        value["phase_5_engine_dependency_closure_remaining_dependency_count"]
            .as_u64()
            .expect("phase 5 remaining dependency count"),
        0
    );
    assert!(
        value["phase_5_engine_dependency_closure_blockers"]
            .as_array()
            .expect("phase 5 blockers")
            .is_empty()
    );
    assert_eq!(value["full_fusion_complete"], true);
    let direct_dependencies = value["direct_codex_base_dependencies"]
        .as_array()
        .expect("direct dependencies")
        .iter()
        .filter_map(|item| item.as_str())
        .collect::<Vec<_>>();
    assert!(direct_dependencies.is_empty());
    assert_eq!(
        value["remaining_direct_codex_base_dependency_count"]
            .as_u64()
            .expect("remaining direct dependency count") as usize,
        direct_dependencies.len()
    );
    assert_eq!(
        value["forbidden_real_side_effects"]["public_ga_claimed"],
        false
    );
    assert_eq!(
        value["forbidden_real_side_effects"]["public_release_published"],
        false
    );
    assert_eq!(
        value["forbidden_real_side_effects"]["native_post_real_mutation_performed"],
        false
    );
    assert_eq!(
        value["forbidden_real_side_effects"]["task_publish_real_mutation_performed"],
        false
    );
    assert_eq!(
        value["forbidden_real_side_effects"]["credential_read"],
        false
    );
    assert_eq!(value["forbidden_real_side_effects"]["model_invoked"], false);
}

#[test]
fn hepta_name_repository_closure_reports_remaining_transition_surfaces_without_side_effects() {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) =
        route_native_gateway_request("GET", HEPTA_NAME_REPOSITORY_CLOSURE_ENDPOINT, &options);
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value =
        serde_json::from_str(&body).expect("name repository closure json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(
        value["source_command"],
        HEPTA_NAME_REPOSITORY_CLOSURE_SOURCE_COMMAND
    );
    assert_eq!(value["phase"], "phase_4_name_repository_closure");
    assert_eq!(value["root_owner"], "hepta");
    assert_eq!(value["closure_gate"], "hepta_name_repository_closure_gate");
    assert_eq!(value["closure_gate_ready"], true);
    assert_eq!(
        value["closure_gate_status"],
        "ready_phase_4_transition_names_closed"
    );
    assert_eq!(value["phase_4_name_repository_closure_ready"], true);
    assert_eq!(value["full_fusion_complete"], true);
    assert!(
        value["transition_surface_count"]
            .as_u64()
            .expect("transition surface count")
            >= 6
    );
    assert!(
        value["closed_transition_surface_count"]
            .as_u64()
            .expect("closed transition surface count")
            >= 6
    );
    assert_eq!(
        value["remaining_transition_surface_count"]
            .as_u64()
            .expect("remaining transition surface count"),
        0
    );
    let surfaces = value["surfaces"].as_array().expect("surfaces");
    assert!(surfaces.iter().any(|surface| {
        surface["surface_id"] == "active_release_binary_package"
            && surface["closure_state"] == "closed"
            && surface["blocks_full_fusion"] == false
    }));
    assert!(surfaces.iter().any(|surface| {
        surface["surface_id"] == "runtime_report_strings"
            && surface["current_name"] == "hepta"
            && surface["target_name"] == "hepta"
            && surface["closure_state"] == "closed"
            && surface["blocks_full_fusion"] == false
    }));
    assert!(surfaces.iter().any(|surface| {
        surface["surface_id"] == "engine_adapter_boundary_route"
            && surface["current_name"] == HEPTA_CODEX_ENGINE_ADAPTER_BOUNDARY_ENDPOINT
            && surface["target_name"] == HEPTA_ENGINE_ADAPTER_BOUNDARY_ENDPOINT
            && surface["closure_state"] == "alias_active"
            && surface["blocks_full_fusion"] == false
    }));
    assert!(surfaces.iter().any(|surface| {
        surface["surface_id"] == "release_gate_script_family"
            && surface["current_name"] == "scripts/hepta-codex-*.sh"
            && surface["target_name"] == "scripts/hepta-*.sh"
            && surface["closure_state"] == "alias_active"
            && surface["blocks_full_fusion"] == false
    }));
    assert!(surfaces.iter().any(|surface| {
        surface["surface_id"] == "core_fusion_route_document"
            && surface["target_name"] == "docs/architecture/HEPTA_CORE_FUSION_ROUTE.md"
            && surface["closure_state"] == "alias_active"
            && surface["blocks_full_fusion"] == false
    }));
    assert!(surfaces.iter().any(|surface| {
        surface["surface_id"] == "workspace_repository_directory"
            && surface["current_name"] == "/Users/qianqi/.openclaw/workspace/Hepta"
            && surface["target_name"] == "/Users/qianqi/.openclaw/workspace/Hepta"
            && surface["closure_state"] == "closed"
            && surface["blocks_full_fusion"] == false
    }));
    let blockers = value["blockers"]
        .as_array()
        .expect("blockers")
        .iter()
        .filter_map(|blocker| blocker.as_str())
        .collect::<Vec<_>>();
    assert!(blockers.is_empty());
    assert!(
        !blockers.contains(&"engine_adapter_boundary_route_still_uses_hepta_codex_transition_slug")
    );
    assert!(
        !blockers.contains(&"release_gate_script_family_still_uses_hepta_codex_transition_prefix")
    );
    assert_eq!(
        value["forbidden_real_side_effects"]["public_release_published"],
        false
    );
    assert_eq!(
        value["forbidden_real_side_effects"]["gateway_mutation_performed"],
        false
    );
    assert_eq!(
        value["forbidden_real_side_effects"]["credential_read"],
        false
    );
}

#[test]
fn hepta_engine_dependency_closure_reports_remaining_dependency_inventory() {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) =
        route_native_gateway_request("GET", HEPTA_ENGINE_DEPENDENCY_CLOSURE_ENDPOINT, &options);
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value =
        serde_json::from_str(&body).expect("engine dependency closure json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(
        value["source_command"],
        HEPTA_ENGINE_DEPENDENCY_CLOSURE_SOURCE_COMMAND
    );
    assert_eq!(value["phase"], "phase_5_engine_dependency_closure");
    assert_eq!(value["root_owner"], "hepta");
    assert_eq!(
        value["closure_gate"],
        "hepta_engine_dependency_closure_gate"
    );
    assert_eq!(value["closure_gate_ready"], true);
    assert_eq!(
        value["closure_gate_status"],
        "ready_active_hepta_service_binary_direct_codex_dependencies_closed"
    );
    assert_eq!(value["full_fusion_complete"], true);
    assert!(
        value["direct_dependency_count"]
            .as_u64()
            .expect("dependency count")
            >= 10
    );
    assert_eq!(value["adapter_retained_dependency_count"], 0);
    assert_eq!(value["remaining_direct_dependency_count"], 0);
    assert_eq!(
        value["closed_direct_dependency_count"],
        value["direct_dependency_count"]
    );

    let surfaces = value["surfaces"]
        .as_array()
        .expect("dependency closure surfaces");
    assert!(surfaces.iter().all(|surface| {
        surface["closure_state"] == "closed_active_hepta_service_binary_isolated"
            && surface["direct_dependency_retained"] == false
            && surface["compatibility_adapter_required"] == false
            && surface["typed_adapter_parity_ready"] == true
            && surface["blocks_full_fusion"] == false
    }));
    assert!(surfaces.iter().any(|surface| {
        surface["dependency_crate"] == "codex-core"
            && surface["adapter_surface_id"] == "tool_invocation"
            && surface["target_owner"] == "hepta-kernel"
    }));
    assert!(surfaces.iter().any(|surface| {
        surface["dependency_crate"] == "codex-tui"
            && surface["adapter_surface_id"] == "legacy_tui_cli"
            && surface["target_owner"] == "hepta-runtime"
    }));
    assert!(
        value["blockers"]
            .as_array()
            .expect("dependency closure blockers")
            .is_empty()
    );
    assert_eq!(
        value["forbidden_real_side_effects"]["public_release_published"],
        false
    );
    assert_eq!(
        value["forbidden_real_side_effects"]["gateway_mutation_performed"],
        false
    );
    assert_eq!(
        value["forbidden_real_side_effects"]["credential_read"],
        false
    );
    assert_eq!(value["forbidden_real_side_effects"]["model_invoked"], false);
}

#[test]
fn hepta_codex_engine_adapter_boundary_reports_surfaces_without_side_effects() {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) =
        route_native_gateway_request("GET", HEPTA_ENGINE_ADAPTER_BOUNDARY_ENDPOINT, &options);
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value =
        serde_json::from_str(&body).expect("engine adapter boundary json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(
        value["source_command"],
        HEPTA_ENGINE_ADAPTER_BOUNDARY_SOURCE_COMMAND
    );
    assert_eq!(
        value["canonical_endpoint"],
        HEPTA_ENGINE_ADAPTER_BOUNDARY_ENDPOINT
    );
    assert_eq!(
        value["canonical_source_command"],
        HEPTA_ENGINE_ADAPTER_BOUNDARY_SOURCE_COMMAND
    );
    assert_eq!(
        value["transition_alias_endpoint"],
        HEPTA_CODEX_ENGINE_ADAPTER_BOUNDARY_ENDPOINT
    );
    assert_eq!(
        value["transition_alias_source_command"],
        HEPTA_CODEX_ENGINE_ADAPTER_BOUNDARY_SOURCE_COMMAND
    );
    assert_eq!(value["hepta_named_route_alias_ready"], true);
    assert_eq!(value["transition_alias_retained"], true);
    assert_eq!(value["phase"], "phase_2_engine_adapter_boundary");
    assert_eq!(value["root_owner"], "hepta");
    assert_eq!(value["adapter_owner"], "codex-engine-adapter");
    assert_eq!(value["boundary_ready"], true);
    assert_eq!(value["adapter_parity_complete"], true);
    assert_eq!(value["adapter_parity_promotion_ready"], true);
    assert_eq!(
        value["adapter_parity_completion_gate"],
        "adapter_behavior_equivalence_to_parity_completion_gate"
    );
    assert_eq!(value["adapter_parity_completion_gate_ready"], true);
    assert_eq!(
        value["adapter_parity_completion_gate_status"],
        "ready_adapter_parity_promoted_active_hepta_service_dependency_closure_complete"
    );
    assert_eq!(
        value["adapter_parity_completion_gate_allows_promotion"],
        true
    );
    assert_eq!(value["full_fusion_complete"], true);
    assert!(
        value["adapter_parity_promotion_criteria"]
            .as_array()
            .expect("adapter parity criteria")
            .iter()
            .any(|item| item.as_str()
                == Some("all adapter surfaces expose typed request/response envelopes"))
    );
    assert!(
        value["adapter_parity_promotion_blockers"]
            .as_array()
            .expect("adapter parity blockers")
            .is_empty()
    );

    let surfaces = value["surfaces"].as_array().expect("surfaces array");
    assert!(surfaces.len() >= 6);
    assert_eq!(
        value["adapter_shadow_replay_required_surface_count"].as_u64(),
        Some(surfaces.len() as u64)
    );
    assert_eq!(
        value["adapter_shadow_replay_covered_surface_count"].as_u64(),
        Some(surfaces.len() as u64)
    );
    assert_eq!(
        value["adapter_shadow_replay_remaining_surface_count"].as_u64(),
        Some(0)
    );
    let parity_evidence = value["parity_evidence"]
        .as_array()
        .expect("parity evidence array");
    assert_eq!(parity_evidence.len(), surfaces.len());
    assert!(
        parity_evidence
            .iter()
            .all(|item| { item["evidence_ready"].as_bool().is_some_and(|ready| ready) })
    );
    assert!(parity_evidence.iter().all(|item| {
        item["compatibility_dispatch_checked"]
            .as_bool()
            .is_some_and(|checked| checked)
    }));
    assert!(parity_evidence.iter().all(|item| {
        item["behavior_equivalence_checked"]
            .as_bool()
            .is_some_and(|checked| checked)
    }));
    assert!(parity_evidence.iter().all(|item| {
        item["observable_behavior_preserved"]
            .as_bool()
            .is_some_and(|preserved| preserved)
    }));
    assert!(parity_evidence.iter().all(|item| {
        item["behavior_equivalence_check"]
            .as_str()
            .is_some_and(|check| check.contains("preserved"))
    }));
    assert_eq!(
        parity_evidence
            .iter()
            .filter(|item| {
                item["shadow_replay_checked"]
                    .as_bool()
                    .is_some_and(|checked| checked)
            })
            .count(),
        surfaces.len()
    );
    assert!(parity_evidence.iter().all(|item| {
        item["shadow_replay_observable_match"]
            .as_bool()
            .is_some_and(|matched| matched)
            && item["shadow_replay_side_effect_free"]
                .as_bool()
                .is_some_and(|free| free)
    }));
    assert!(surfaces.iter().all(|surface| {
        surface["live_mutation_allowed"]
            .as_bool()
            .is_some_and(|allowed| !allowed)
    }));
    assert!(surfaces.iter().all(|surface| {
        surface["typed_request_response_envelope_ready"]
            .as_bool()
            .is_some_and(|ready| ready)
    }));
    assert!(surfaces.iter().all(|surface| {
        surface["typed_adapter_parity_gate_ready"]
            .as_bool()
            .is_some_and(|ready| ready)
    }));
    let surface_ids = surfaces
        .iter()
        .filter_map(|surface| surface["surface_id"].as_str())
        .collect::<Vec<_>>();
    assert!(surface_ids.contains(&"model_provider_execution"));
    assert!(surface_ids.contains(&"session_thread_store"));
    assert!(surface_ids.contains(&"sandbox_exec"));

    assert_eq!(
        value["forbidden_real_side_effects"]["public_ga_claimed"],
        false
    );
    assert_eq!(
        value["forbidden_real_side_effects"]["credential_read"],
        false
    );
    assert_eq!(value["forbidden_real_side_effects"]["model_invoked"], false);
    assert_eq!(
        value["forbidden_real_side_effects"]["gateway_mutation_performed"],
        false
    );
}

#[test]
fn hepta_public_ga_operator_approval_packet_is_plan_only() {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
        "GET",
        HEPTA_PUBLIC_GA_OPERATOR_APPROVAL_PACKET_ENDPOINT,
        &options,
    );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value =
        serde_json::from_str(&body).expect("public ga approval packet json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(
        value["source_command"],
        "/hepta-public-ga-operator-approval-packet --json"
    );
    assert_eq!(
        value["compatibility_mode"],
        "native_public_ga_operator_approval_packet"
    );
    assert_eq!(value["status"], "ready");
    assert_eq!(value["approval_packet_ready"], true);
    assert_eq!(value["safe_default_mode"], "plan_only_no_live_mutation");
    assert_eq!(value["irreversible_actions_blocked_by_default"], true);
    assert_eq!(value["public_ga_ready"], false);
    assert_eq!(
        value["current_hepta_codex_script_total"],
        CURRENT_HEPTA_CODEX_SCRIPT_TOTAL
    );
    assert_eq!(
        value["native_gateway_source_command_count"],
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    assert_eq!(value["missing_route_count"], 0);
    assert_eq!(value["required_operator_approval_count"], 8);
    let blockers = value["blockers"]
        .as_array()
        .expect("blockers")
        .iter()
        .filter_map(|item| item.as_str())
        .collect::<Vec<_>>();
    assert!(blockers.contains(&"telegram_owner_handoff_not_operator_approved"));
    assert!(blockers.contains(&"external_public_release_not_operator_approved"));
    let endpoints = value["evidence_endpoints"]
        .as_array()
        .expect("evidence endpoints")
        .iter()
        .filter_map(|item| item.as_str())
        .collect::<Vec<_>>();
    assert!(endpoints.contains(&HEPTA_PUBLIC_GA_READINESS_ENDPOINT));
    assert!(endpoints.contains(&GATEWAY_LIVE_ACTIVATION_PLAN_ENDPOINT));
    assert_eq!(value["side_effects"]["launchd_mutated"], false);
    assert_eq!(value["side_effects"]["credential_read"], false);
    assert_eq!(value["side_effects"]["provider_invoked"], false);
    assert_eq!(value["side_effects"]["channel_send_performed"], false);
    assert_eq!(value["side_effects"]["telegram_send_performed"], false);
    assert_eq!(
        value["side_effects"]["native_post_mutation_performed"],
        false
    );
    assert_eq!(value["side_effects"]["external_send_performed"], false);
}

#[test]
fn hepta_cli_command_inventory_endpoint_returns_read_only_gap_report() {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) =
        route_native_gateway_request("GET", HEPTA_CLI_COMMAND_INVENTORY_ENDPOINT, &options);
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body).expect("cli command inventory json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(
        value["source_command"],
        "/hepta-cli-command-inventory --json"
    );
    assert_eq!(
        value["compatibility_mode"],
        "native_cli_command_breadth_inventory"
    );
    assert_eq!(value["side_effect_free"], true);
    assert_eq!(value["old_hepta_ops_file_count"], 65);
    assert_eq!(value["old_hepta_rough_command_reference_count"], 574);
    assert_eq!(value["old_hepta_script_total"], 20);
    assert_eq!(
        value["current_hepta_codex_script_total"],
        CURRENT_HEPTA_CODEX_SCRIPT_TOTAL
    );
    assert_eq!(
        value["native_gateway_source_command_count"],
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    assert_eq!(value["missing_route_count"], 0);
    assert_eq!(value["ops_family_count"], 5);
    assert_eq!(value["ops_file_family_covered_count"], 65);
    assert_eq!(value["old_cli_command_breadth_fully_migrated"], true);
    assert_eq!(value["safe_read_only_inventory_ready"], true);
    assert_eq!(
        value["script_inventory_script"],
        "scripts/hepta-cli-command-inventory.sh"
    );
    let families = value["ops_families"].as_array().expect("ops families");
    assert_eq!(families.len(), 5);
    assert_eq!(families[0]["name"], "provider_metadata_bridges");
    assert_eq!(families[0]["old_ops_file_count"], 15);
    assert_eq!(
        value["side_effects"]["provider_invoked"], false,
        "inventory must not invoke providers"
    );
    assert_eq!(value["side_effects"]["credential_read"], false);
    assert_eq!(value["side_effects"]["telegram_read_performed"], false);
    assert_eq!(value["side_effects"]["message_sent"], false);
    assert_eq!(
        value["side_effects"]["native_post_mutation_performed"],
        false
    );
    let blockers = value["blockers"]
        .as_array()
        .expect("blockers")
        .iter()
        .filter_map(|item| item.as_str())
        .collect::<Vec<_>>();
    assert!(!blockers.contains(&"old_hepta_cli_command_breadth_not_fully_migrated"));
    assert!(blockers.contains(&"channel_adapters_not_owner_handoff_approved"));
}

#[test]
fn hepta_provider_metadata_inventory_endpoint_is_metadata_only() {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) =
        route_native_gateway_request("GET", HEPTA_PROVIDER_METADATA_INVENTORY_ENDPOINT, &options);
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value =
        serde_json::from_str(&body).expect("provider metadata inventory json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(
        value["source_command"],
        "/hepta-provider-metadata-inventory --json"
    );
    assert_eq!(
        value["compatibility_mode"],
        "native_provider_metadata_inventory"
    );
    assert_eq!(value["old_provider_ops_file_count"], 15);
    assert_eq!(value["adjacent_search_ops_file_count"], 3);
    assert_eq!(
        value["current_hepta_codex_script_total"],
        CURRENT_HEPTA_CODEX_SCRIPT_TOTAL
    );
    assert_eq!(
        value["native_gateway_source_command_count"],
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    assert_eq!(value["missing_route_count"], 0);
    assert_eq!(value["provider_adapter_count"], 15);
    assert_eq!(value["adjacent_search_adapter_count"], 3);
    assert_eq!(value["metadata_inventory_ready"], true);
    assert_eq!(value["provider_live_invocation_enabled"], false);
    assert_eq!(value["credentialed_smoke_performed"], false);
    assert_eq!(
        value["script_inventory_script"],
        "scripts/hepta-provider-metadata-inventory.sh"
    );
    let providers = value["provider_adapters"]
        .as_array()
        .expect("provider adapters");
    assert_eq!(providers.len(), 15);
    assert_eq!(providers[0]["name"], "anthropic");
    assert_eq!(providers[10]["name"], "openai-codex");
    assert_eq!(
        value["side_effects"]["provider_invoked"], false,
        "inventory must not invoke providers"
    );
    assert_eq!(value["side_effects"]["credential_read"], false);
    assert_eq!(value["side_effects"]["external_network_read"], false);
    assert_eq!(value["side_effects"]["model_invoked"], false);
    assert_eq!(value["side_effects"]["message_sent"], false);
    let blockers = value["blockers"]
        .as_array()
        .expect("blockers")
        .iter()
        .filter_map(|item| item.as_str())
        .collect::<Vec<_>>();
    assert!(blockers.contains(&"provider_prompt_smoke_not_operator_approved"));
    assert!(blockers.contains(&"provider_credentials_not_read_by_inventory"));
}

#[test]
fn hepta_runtime_session_dry_run_inventory_endpoint_is_side_effect_free() {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
        "GET",
        HEPTA_RUNTIME_SESSION_DRY_RUN_INVENTORY_ENDPOINT,
        &options,
    );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value =
        serde_json::from_str(&body).expect("runtime session inventory json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(
        value["source_command"],
        "/hepta-runtime-session-dry-run-inventory --json"
    );
    assert_eq!(
        value["compatibility_mode"],
        "native_runtime_session_dry_run_inventory"
    );
    assert_eq!(value["old_runtime_ops_file_count"], 12);
    assert_eq!(
        value["current_hepta_codex_script_total"],
        CURRENT_HEPTA_CODEX_SCRIPT_TOTAL
    );
    assert_eq!(
        value["native_gateway_source_command_count"],
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    assert_eq!(value["missing_route_count"], 0);
    assert_eq!(value["dry_run_surface_count"], 12);
    assert_eq!(value["covered_old_ops_file_count"], 12);
    assert_eq!(value["planner_ready_count"], 12);
    assert_eq!(value["live_mutation_surface_count"], 0);
    assert_eq!(value["dry_run_inventory_ready"], true);
    assert_eq!(value["old_cli_invocation_compatibility_claimed"], false);
    assert_eq!(value["task_registry_mutation_enabled"], false);
    assert_eq!(value["session_store_mutation_enabled"], false);
    assert_eq!(value["gateway_event_enqueue_enabled"], false);
    assert_eq!(value["external_telemetry_push_enabled"], false);
    assert_eq!(
        value["script_inventory_script"],
        "scripts/hepta-runtime-session-dry-run-inventory.sh"
    );
    let surfaces = value["dry_run_surfaces"]
        .as_array()
        .expect("runtime dry-run surfaces");
    assert_eq!(surfaces.len(), 12);
    assert_eq!(surfaces[7]["name"], "runtime-event");
    assert_eq!(surfaces[8]["name"], "session-orchestration");
    assert_eq!(surfaces[9]["name"], "task-provenance");
    assert_eq!(value["side_effects"]["task_registry_mutated"], false);
    assert_eq!(value["side_effects"]["session_store_mutated"], false);
    assert_eq!(value["side_effects"]["gateway_event_enqueued"], false);
    assert_eq!(value["side_effects"]["hook_enqueued"], false);
    assert_eq!(value["side_effects"]["process_spawned"], false);
    assert_eq!(value["side_effects"]["provider_invoked"], false);
    assert_eq!(value["side_effects"]["model_invoked"], false);
    assert_eq!(value["side_effects"]["credential_read"], false);
    assert_eq!(value["side_effects"]["telegram_read_performed"], false);
    assert_eq!(value["side_effects"]["message_sent"], false);
    assert_eq!(
        value["side_effects"]["native_post_mutation_performed"],
        false
    );
    assert_eq!(value["side_effects"]["filesystem_written"], false);
    let blockers = value["blockers"]
        .as_array()
        .expect("blockers")
        .iter()
        .filter_map(|item| item.as_str())
        .collect::<Vec<_>>();
    assert!(blockers.contains(&"task_registry_live_mutation_not_operator_approved"));
    assert!(blockers.contains(&"gateway_event_enqueue_not_operator_approved"));
}

#[test]
fn hepta_context_recall_worker_scheduler_handoff_endpoint_is_plan_only_without_leaks() {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
        "GET",
        HEPTA_CONTEXT_RECALL_WORKER_SCHEDULER_HANDOFF_ENDPOINT,
        &options,
    );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");
    assert!(!body.contains("operator-ready-safe-context"));
    assert!(!body.contains("operator-due-safe-context"));
    assert!(!body.contains("worker-ready-source-id"));
    assert!(!body.contains("worker-due-source-id"));
    assert!(!body.contains("<selected_context_recall>"));
    assert!(!body.contains("[redacted-query]"));

    let value: serde_json::Value =
        serde_json::from_str(&body).expect("context recall worker scheduler handoff json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(
        value["source_command"],
        "/hepta-context-recall-worker-scheduler-handoff --dry-run --json"
    );
    assert_eq!(
        value["compatibility_mode"],
        "native_context_recall_worker_scheduler_handoff_dry_run"
    );
    assert_eq!(
        value["endpoint"],
        HEPTA_CONTEXT_RECALL_WORKER_SCHEDULER_HANDOFF_ENDPOINT
    );
    assert_eq!(
        value["operator_approval_env"],
        HEPTA_CONTEXT_RECALL_WORKER_SCHEDULER_HANDOFF_APPROVED_ENV
    );
    assert_eq!(value["side_effect_free"], true);
    assert_eq!(value["native_route"], true);
    assert_eq!(value["default_worker_policy"], "Disabled");
    assert_eq!(
        value["operator_approved_policy"],
        "ExperimentalOperatorApproved"
    );
    assert_eq!(value["route_executes_scheduler"], false);
    assert_eq!(value["route_runs_worker_task"], false);
    assert_eq!(value["route_invokes_model"], false);
    assert_eq!(value["route_injects_selected_snippets"], false);
    assert_eq!(value["ready_due_scheduler_variants_available"], true);
    assert_eq!(value["legacy_ready_due_scheduler_defaults_disabled"], true);
    assert_eq!(value["stable_schema_promoted"], false);
    assert_eq!(value["tui_exec_app_server_defaults_none"], true);
    assert_eq!(value["selected_snippet_text_exposed"], false);
    assert_eq!(value["source_ids_exposed"], false);
    assert_eq!(value["query_payload_exposed"], false);
    assert_eq!(value["side_effects"]["task_registry_mutated"], false);
    assert_eq!(value["side_effects"]["session_store_mutated"], false);
    assert_eq!(value["side_effects"]["worker_task_ran"], false);
    assert_eq!(value["side_effects"]["ready_scheduler_ran"], false);
    assert_eq!(value["side_effects"]["due_scheduler_ran"], false);
    assert_eq!(value["side_effects"]["provider_invoked"], false);
    assert_eq!(value["side_effects"]["model_invoked"], false);
    assert_eq!(value["side_effects"]["selected_snippets_injected"], false);
    assert_eq!(value["side_effects"]["stable_schema_mutated"], false);
    let entrypoints = value["allowed_runtime_entrypoints"]
        .as_array()
        .expect("runtime entrypoints")
        .iter()
        .filter_map(|item| item.as_str())
        .collect::<Vec<_>>();
    assert!(entrypoints.contains(&"run_ready_worker_tasks_with_context_recall_handoff"));
    assert!(entrypoints.contains(&"run_due_worker_tasks_with_context_recall_handoff"));
    let blockers = value["blockers"]
        .as_array()
        .expect("blockers")
        .iter()
        .filter_map(|item| item.as_str())
        .collect::<Vec<_>>();
    assert!(blockers.contains(&"native_gateway_route_is_plan_only_no_worker_execution"));
}

#[test]
fn hepta_channel_adapter_status_inventory_endpoint_is_disabled_and_gated() {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
        "GET",
        HEPTA_CHANNEL_ADAPTER_STATUS_INVENTORY_ENDPOINT,
        &options,
    );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value =
        serde_json::from_str(&body).expect("channel adapter inventory json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(
        value["source_command"],
        "/hepta-channel-adapter-status-inventory --json"
    );
    assert_eq!(
        value["compatibility_mode"],
        "native_channel_adapter_disabled_status_inventory"
    );
    assert_eq!(value["old_channel_ops_file_count"], 13);
    assert_eq!(
        value["current_hepta_codex_script_total"],
        CURRENT_HEPTA_CODEX_SCRIPT_TOTAL
    );
    assert_eq!(
        value["native_gateway_source_command_count"],
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    assert_eq!(value["missing_route_count"], 0);
    assert_eq!(value["adapter_count"], 13);
    assert_eq!(value["disabled_status_ready_count"], 13);
    assert_eq!(value["live_adapter_enabled_count"], 0);
    assert_eq!(value["channel_status_inventory_ready"], true);
    assert_eq!(value["old_cli_invocation_compatibility_claimed"], false);
    assert_eq!(value["live_channel_read_enabled"], false);
    assert_eq!(value["live_channel_send_enabled"], false);
    assert_eq!(value["owner_handoff_performed"], false);
    assert_eq!(
        value["script_inventory_script"],
        "scripts/hepta-channel-adapter-status-inventory.sh"
    );
    let adapters = value["channel_adapters"]
        .as_array()
        .expect("channel adapters");
    assert_eq!(adapters.len(), 13);
    assert_eq!(adapters[1]["name"], "discord");
    assert_eq!(adapters[9]["name"], "telegram");
    assert_eq!(adapters[12]["name"], "webhooks");
    assert_eq!(value["side_effects"]["channel_read_performed"], false);
    assert_eq!(value["side_effects"]["channel_send_performed"], false);
    assert_eq!(value["side_effects"]["credential_read"], false);
    assert_eq!(value["side_effects"]["external_network_read"], false);
    assert_eq!(value["side_effects"]["external_send_performed"], false);
    assert_eq!(
        value["side_effects"]["telegram_owner_handoff_performed"],
        false
    );
    assert_eq!(value["side_effects"]["telegram_read_performed"], false);
    assert_eq!(value["side_effects"]["telegram_send_performed"], false);
    assert_eq!(value["side_effects"]["voice_call_performed"], false);
    assert_eq!(value["side_effects"]["tts_audio_played"], false);
    assert_eq!(value["side_effects"]["webhook_delivered"], false);
    assert_eq!(value["side_effects"]["file_transfer_performed"], false);
    assert_eq!(
        value["side_effects"]["native_post_mutation_performed"],
        false
    );
    assert_eq!(value["side_effects"]["filesystem_written"], false);
    let blockers = value["blockers"]
        .as_array()
        .expect("blockers")
        .iter()
        .filter_map(|item| item.as_str())
        .collect::<Vec<_>>();
    assert!(blockers.contains(&"channel_live_read_not_operator_approved"));
    assert!(blockers.contains(&"telegram_owner_handoff_not_requested"));
}

#[test]
fn hepta_local_tooling_content_inventory_endpoint_is_plan_only() {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
        "GET",
        HEPTA_LOCAL_TOOLING_CONTENT_INVENTORY_ENDPOINT,
        &options,
    );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value =
        serde_json::from_str(&body).expect("local tooling content inventory json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(
        value["source_command"],
        "/hepta-local-tooling-content-inventory --json"
    );
    assert_eq!(
        value["compatibility_mode"],
        "native_local_tooling_content_planning_inventory"
    );
    assert_eq!(value["old_local_tooling_ops_file_count"], 11);
    assert_eq!(
        value["current_hepta_codex_script_total"],
        CURRENT_HEPTA_CODEX_SCRIPT_TOTAL
    );
    assert_eq!(
        value["native_gateway_source_command_count"],
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    assert_eq!(value["missing_route_count"], 0);
    assert_eq!(value["surface_count"], 11);
    assert_eq!(value["planner_ready_count"], 11);
    assert_eq!(value["live_process_enabled_count"], 0);
    assert_eq!(value["filesystem_touch_enabled_count"], 0);
    assert_eq!(value["network_read_enabled_count"], 0);
    assert_eq!(value["tool_invocation_enabled_count"], 0);
    assert_eq!(value["local_tooling_inventory_ready"], true);
    assert_eq!(value["old_cli_invocation_compatibility_claimed"], false);
    assert_eq!(value["process_execution_enabled"], false);
    assert_eq!(value["filesystem_read_enabled"], false);
    assert_eq!(value["filesystem_write_enabled"], false);
    assert_eq!(value["network_read_enabled"], false);
    assert_eq!(value["tool_invocation_enabled"], false);
    assert_eq!(
        value["script_inventory_script"],
        "scripts/hepta-local-tooling-content-inventory.sh"
    );
    let surfaces = value["local_tooling_surfaces"]
        .as_array()
        .expect("local tooling surfaces");
    assert_eq!(surfaces.len(), 11);
    assert_eq!(surfaces[0]["name"], "canvas");
    assert_eq!(surfaces[4]["name"], "filesystem");
    assert_eq!(surfaces[6]["name"], "process-execution");
    assert_eq!(surfaces[10]["name"], "wiki-tools");
    assert_eq!(value["side_effects"]["process_spawned"], false);
    assert_eq!(value["side_effects"]["filesystem_read"], false);
    assert_eq!(value["side_effects"]["filesystem_written"], false);
    assert_eq!(value["side_effects"]["external_network_read"], false);
    assert_eq!(value["side_effects"]["tool_invoked"], false);
    assert_eq!(value["side_effects"]["provider_invoked"], false);
    assert_eq!(value["side_effects"]["model_invoked"], false);
    assert_eq!(value["side_effects"]["credential_read"], false);
    assert_eq!(value["side_effects"]["channel_read_performed"], false);
    assert_eq!(value["side_effects"]["channel_send_performed"], false);
    assert_eq!(
        value["side_effects"]["native_post_mutation_performed"],
        false
    );
    let blockers = value["blockers"]
        .as_array()
        .expect("blockers")
        .iter()
        .filter_map(|item| item.as_str())
        .collect::<Vec<_>>();
    assert!(blockers.contains(&"process_execution_not_operator_approved"));
    assert!(blockers.contains(&"filesystem_write_not_operator_approved"));
    assert!(blockers.contains(&"tool_invocation_not_operator_approved"));
}

#[test]
fn hepta_systems_tool_registry_inventory_endpoint_is_read_only() {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
        "GET",
        HEPTA_SYSTEMS_TOOL_REGISTRY_INVENTORY_ENDPOINT,
        &options,
    );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value =
        serde_json::from_str(&body).expect("tool registry inventory json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(
        value["source_command"],
        "/hepta-systems-tool-registry-inventory --json"
    );
    assert_eq!(
        value["compatibility_mode"],
        "native_systems_tool_registry_inventory_report"
    );
    assert_eq!(
        value["endpoint"],
        HEPTA_SYSTEMS_TOOL_REGISTRY_INVENTORY_ENDPOINT
    );
    assert_eq!(
        value["native_gateway_source_command_count"],
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    assert_eq!(value["missing_route_count"], 0);
    assert_eq!(value["source_kind_count"], 5);
    assert_eq!(value["inventory_entry_field_count"], 11);
    assert_eq!(
        value["next_absorption_target"],
        "native_systems_cockpit_read_only_tool_registry_view"
    );
    assert_eq!(value["tool_registry_inventory_ready"], true);
    assert_eq!(value["tool_invocation_enabled"], false);
    assert_eq!(value["mcp_server_start_enabled"], false);
    assert_eq!(value["plugin_install_enabled"], false);
    assert_eq!(value["connector_install_enabled"], false);
    assert_eq!(value["side_effects"]["tool_invoked"], false);
    assert_eq!(value["side_effects"]["mcp_server_started"], false);
    assert_eq!(value["side_effects"]["plugin_installed"], false);
    assert_eq!(value["side_effects"]["connector_installed"], false);
    assert_eq!(value["side_effects"]["credential_read"], false);
    assert_eq!(value["side_effects"]["provider_invoked"], false);
    assert_eq!(value["side_effects"]["model_invoked"], false);
    assert_eq!(value["side_effects"]["channel_send_performed"], false);
    assert_eq!(value["side_effects"]["gateway_or_auth_mutated"], false);
    assert_eq!(
        value["side_effects"]["native_post_mutation_performed"],
        false
    );
}

#[test]
fn hepta_systems_workflow_definition_registry_endpoint_is_read_only() {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
        "GET",
        HEPTA_SYSTEMS_WORKFLOW_DEFINITION_REGISTRY_ENDPOINT,
        &options,
    );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value =
        serde_json::from_str(&body).expect("workflow definition registry json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(
        value["source_command"],
        "/hepta-systems-workflow-definition-registry --json"
    );
    assert_eq!(
        value["compatibility_mode"],
        "native_systems_workflow_definition_registry_report"
    );
    assert_eq!(
        value["endpoint"],
        HEPTA_SYSTEMS_WORKFLOW_DEFINITION_REGISTRY_ENDPOINT
    );
    assert_eq!(
        value["native_gateway_source_command_count"],
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    assert_eq!(value["missing_route_count"], 0);
    assert_eq!(value["step_kind_count"], 4);
    assert_eq!(value["definition_entry_field_count"], 5);
    assert_eq!(value["step_entry_field_count"], 6);
    assert_eq!(value["start_plan_field_count"], 5);
    assert_eq!(value["ready_to_append_start_event_requires_approval"], true);
    assert_eq!(value["start_plan_appends_event"], false);
    assert_eq!(value["step_projection_field_count"], 5);
    assert_eq!(value["step_projection_event_type_count"], 4);
    assert_eq!(value["step_projection_appends_events"], false);
    assert_eq!(value["pending_plan_field_count"], 6);
    assert_eq!(value["pending_plan_mutates_event_log"], false);
    assert_eq!(value["write_proposal_field_count"], 7);
    assert_eq!(value["write_proposal_commits_event_log"], false);
    assert_eq!(value["write_validation_field_count"], 6);
    assert_eq!(value["write_validation_commits_event_log"], false);
    assert_eq!(
        value["next_absorption_target"],
        "native_systems_cockpit_read_only_workflow_definition_registry"
    );
    assert_eq!(value["workflow_definition_registry_ready"], true);
    assert_eq!(value["workflow_activity_execution_enabled"], false);
    assert_eq!(value["tool_invocation_enabled"], false);
    assert_eq!(value["approval_resolution_enabled"], false);
    assert_eq!(value["delivery_send_enabled"], false);
    assert_eq!(value["ledger_mutation_enabled"], false);
    assert_eq!(value["side_effects"]["workflow_activity_executed"], false);
    assert_eq!(value["side_effects"]["tool_invoked"], false);
    assert_eq!(value["side_effects"]["approval_resolved"], false);
    assert_eq!(value["side_effects"]["delivery_send_performed"], false);
    assert_eq!(value["side_effects"]["ledger_mutated"], false);
    assert_eq!(value["side_effects"]["credential_read"], false);
    assert_eq!(value["side_effects"]["provider_invoked"], false);
    assert_eq!(value["side_effects"]["model_invoked"], false);
    assert_eq!(value["side_effects"]["channel_send_performed"], false);
    assert_eq!(value["side_effects"]["gateway_or_auth_mutated"], false);
    assert_eq!(
        value["side_effects"]["native_post_mutation_performed"],
        false
    );
}

#[test]
fn hepta_memory_capability_absorption_inventory_endpoint_is_gap_only() {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
        "GET",
        HEPTA_MEMORY_CAPABILITY_ABSORPTION_INVENTORY_ENDPOINT,
        &options,
    );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value =
        serde_json::from_str(&body).expect("memory capability inventory json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(
        value["source_command"],
        "/hepta-memory-capability-absorption-inventory --json"
    );
    assert_eq!(
        value["compatibility_mode"],
        "native_memory_capability_absorption_gap_inventory"
    );
    assert_eq!(value["old_memory_capability_ops_file_count"], 14);
    assert_eq!(
        value["current_hepta_codex_script_total"],
        CURRENT_HEPTA_CODEX_SCRIPT_TOTAL
    );
    assert_eq!(
        value["native_gateway_source_command_count"],
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    assert_eq!(value["missing_route_count"], 0);
    assert_eq!(value["surface_count"], 14);
    assert_eq!(value["absorbed_or_represented_count"], 14);
    assert_eq!(value["gap_report_ready_count"], 14);
    assert_eq!(value["live_mutation_enabled_count"], 0);
    assert_eq!(value["memory_capability_inventory_ready"], true);
    assert_eq!(value["old_cli_invocation_compatibility_claimed"], false);
    assert_eq!(value["memory_store_mutation_enabled"], false);
    assert_eq!(value["capability_registry_mutation_enabled"], false);
    assert_eq!(value["plugin_registry_mutation_enabled"], false);
    assert_eq!(value["coding_agent_spawn_enabled"], false);
    assert_eq!(value["search_provider_live_query_enabled"], false);
    assert_eq!(value["skill_workshop_write_enabled"], false);
    assert_eq!(
        value["script_inventory_script"],
        "scripts/hepta-memory-capability-inventory.sh"
    );
    let surfaces = value["memory_capability_surfaces"]
        .as_array()
        .expect("memory capability surfaces");
    assert_eq!(surfaces.len(), 14);
    assert_eq!(surfaces[0]["name"], "capability-surface");
    assert_eq!(surfaces[4]["name"], "memory-rem");
    assert_eq!(
        surfaces[4]["migration_status"],
        "represented_by_memory_rem_status_closure"
    );
    assert_eq!(surfaces[4]["absorbed_or_represented"], true);
    assert_eq!(surfaces[4]["live_mutation_enabled"], false);
    assert_eq!(surfaces[6]["name"], "memory-tools");
    assert_eq!(
        surfaces[6]["migration_status"],
        "represented_by_memory_tools_catalog_closure"
    );
    assert_eq!(surfaces[6]["absorbed_or_represented"], true);
    assert_eq!(surfaces[6]["live_mutation_enabled"], false);
    assert_eq!(surfaces[7]["name"], "native-coding-agent");
    assert_eq!(surfaces[9]["name"], "native-residual-runtime");
    assert_eq!(
        surfaces[9]["migration_status"],
        "represented_by_native_residual_runtime_status_closure"
    );
    assert_eq!(surfaces[9]["absorbed_or_represented"], true);
    assert_eq!(surfaces[9]["live_mutation_enabled"], false);
    assert_eq!(surfaces[11]["name"], "plugin-migration");
    assert_eq!(
        surfaces[11]["migration_status"],
        "represented_by_plugin_migration_plan_closure"
    );
    assert_eq!(surfaces[11]["absorbed_or_represented"], true);
    assert_eq!(surfaces[11]["live_mutation_enabled"], false);
    assert_eq!(surfaces[13]["name"], "skill-workshop");
    assert_eq!(
        surfaces[13]["migration_status"],
        "represented_by_skill_workshop_plan_closure"
    );
    assert_eq!(surfaces[13]["absorbed_or_represented"], true);
    assert_eq!(surfaces[13]["live_mutation_enabled"], false);
    assert_eq!(value["side_effects"]["memory_store_mutated"], false);
    assert_eq!(value["side_effects"]["capability_registry_mutated"], false);
    assert_eq!(value["side_effects"]["plugin_registry_mutated"], false);
    assert_eq!(value["side_effects"]["coding_agent_spawned"], false);
    assert_eq!(value["side_effects"]["skill_workshop_written"], false);
    assert_eq!(value["side_effects"]["filesystem_read"], false);
    assert_eq!(value["side_effects"]["filesystem_written"], false);
    assert_eq!(value["side_effects"]["external_network_read"], false);
    assert_eq!(value["side_effects"]["provider_invoked"], false);
    assert_eq!(value["side_effects"]["model_invoked"], false);
    assert_eq!(value["side_effects"]["credential_read"], false);
    assert_eq!(value["side_effects"]["channel_read_performed"], false);
    assert_eq!(value["side_effects"]["channel_send_performed"], false);
    assert_eq!(
        value["side_effects"]["native_post_mutation_performed"],
        false
    );
    let blockers = value["blockers"]
        .as_array()
        .expect("blockers")
        .iter()
        .filter_map(|item| item.as_str())
        .collect::<Vec<_>>();
    assert!(blockers.contains(&"memory_store_mutation_not_operator_approved"));
    assert!(blockers.contains(&"plugin_registry_mutation_not_operator_approved"));
    assert!(blockers.contains(&"coding_agent_spawn_not_operator_approved"));
}

#[test]
fn hepta_offline_route_parity_fixtures_match_native_reports() -> anyhow::Result<()> {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let mut native_reports = serde_json::Map::new();
    for (name, endpoint) in [
        (
            "memory_capability_absorption_inventory",
            "/api/hepta-memory-capability-absorption-inventory",
        ),
        ("core_fusion_readiness", "/api/hepta-core-fusion-readiness"),
        (
            "engine_dependency_closure",
            "/api/hepta-engine-dependency-closure",
        ),
        (
            "public_ga_operator_approval_packet",
            "/api/hepta-public-ga-operator-approval-packet",
        ),
        (
            "release_hardening_status_gate",
            "/api/hepta-release-hardening-status-gate",
        ),
        (
            "provider_channel_dry_run_plan",
            "/api/hepta-provider-channel-dry-run-plan",
        ),
        (
            "runtime_session_dry_run_inventory",
            "/api/hepta-runtime-session-dry-run-inventory",
        ),
        (
            "local_tooling_content_inventory",
            "/api/hepta-local-tooling-content-inventory",
        ),
    ] {
        let (status, _, body) = route_native_gateway_request("GET", endpoint, &options);
        assert_eq!(status, "200 OK");
        native_reports.insert(name.to_string(), serde_json::from_str(&body)?);
    }
    let fixture_path = std::env::current_dir()?
        .ancestors()
        .map(|ancestor| {
            ancestor
                .join("scripts/testdata/hepta-route-parity-native-report-fixture-bundle-v1.json")
        })
        .find(|candidate| candidate.is_file())
        .context("offline route parity Native report fixture bundle")?;
    let fixture: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&fixture_path)?)?;
    let normalized_reports = serde_json::to_string(&fixture["reports"])?;

    assert_eq!(
        fixture["reports"]["memory_capability_absorption_inventory"],
        serde_json::to_value(hepta_memory_capability_absorption_inventory_report())?
    );
    assert_eq!(
        (
            fixture["reports"].clone(),
            fixture["provenance"]["normalized_reports_sha256"].clone(),
        ),
        (
            serde_json::Value::Object(native_reports),
            serde_json::Value::String(format!(
                "{:x}",
                Sha256::digest(normalized_reports.as_bytes())
            )),
        )
    );
    Ok(())
}

#[test]
fn hepta_memory_intelligence_kg_full_enablement_runtime_readiness_endpoint_is_route_count_aware() {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
        "GET",
        HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_READINESS_ENDPOINT,
        &options,
    );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value =
        serde_json::from_str(&body).expect("full enablement runtime readiness json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-enablement-runtime-readiness --json"
    );
    assert_eq!(
        value["compatibility_mode"],
        "native_full_enablement_runtime_readiness_route_source_only"
    );
    assert_eq!(
        value["endpoint"],
        HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_READINESS_ENDPOINT
    );
    assert_eq!(
        value["native_gateway_source_command_count"],
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    assert_eq!(
        value["route_count"],
        serde_json::json!(NATIVE_GATEWAY_SOURCE_COMMAND_COUNT)
    );
    assert_eq!(value["route_count_cutover_floor"], 69);
    assert_eq!(value["route_count_floor_preserved"], true);
    assert_eq!(value["route_count_source_command_accepted"], true);
    assert_eq!(value["missing_route_count"], 0);
    assert_eq!(value["runtime_readiness_route_wired"], true);
    assert_eq!(
        value["runtime_readiness_route_active_install_performed_by_this_gate"],
        false
    );
    assert_eq!(value["full_enablement_activation_readiness_ready"], true);
    assert_eq!(
        value["full_enablement_activation_readiness_status"],
        "ready_for_operator_approved_activation_slicing"
    );
    assert_eq!(
        value["live_activation_status"],
        "not_performed_by_this_route"
    );
    assert_eq!(value["core_full_fusion_complete"], true);
    assert_eq!(value["active_binary_package"], "hepta-cli");
    assert_eq!(value["remaining_direct_codex_dependency_count"], 0);
    assert_eq!(value["memory_surface_count"], 14);
    assert_eq!(value["absorbed_or_represented_count"], 14);
    assert_eq!(value["gap_report_ready_count"], 14);
    assert_eq!(value["live_mutation_enabled_count"], 0);
    assert_eq!(value["memory_store_mutation_enabled"], false);
    assert_eq!(value["kg_source_gate_count"], 5);
    assert_eq!(value["kg_required_total_preflight_requirement_count"], 19);
    assert_eq!(value["kg_missing_total_preflight_requirement_count"], 19);
    assert_eq!(value["enablement_lane_count"], 6);
    assert_eq!(value["ready_enablement_lane_count"], 6);
    assert_eq!(value["current_live_enabled_lane_count"], 0);
    assert_eq!(value["rust_contract_reference_count"], 7);
    assert_eq!(value["rust_contract_compile_checked_count"], 7);
    assert_eq!(value["operator_approval_required_before_activation"], true);
    assert_eq!(value["operator_activation_receipt_required"], true);
    assert_eq!(value["rollback_kill_switch_required"], true);
    assert_eq!(value["long_soak_required_before_mutation"], true);
    assert_eq!(value["context_handoff_acceptance_required"], true);
    let blocked = value["blocked_activation_actions"]
        .as_array()
        .expect("blocked activation actions")
        .iter()
        .filter_map(|item| item.as_str())
        .collect::<Vec<_>>();
    assert!(blocked.contains(&"memory_store_mutation"));
    assert!(blocked.contains(&"kg_context_injection"));
    assert!(blocked.contains(&"live_kg_write"));
    assert!(blocked.contains(&"provider_model_invocation"));
    assert!(blocked.contains(&"credential_read"));
    assert_eq!(
        value["side_effects"]["full_live_enablement_performed"],
        false
    );
    assert_eq!(value["side_effects"]["memory_store_mutated"], false);
    assert_eq!(
        value["side_effects"]["hepta_intelligence_context_attached"],
        false
    );
    assert_eq!(value["side_effects"]["prompt_preview_rendered"], false);
    assert_eq!(value["side_effects"]["context_injection_performed"], false);
    assert_eq!(value["side_effects"]["model_invoked"], false);
    assert_eq!(value["side_effects"]["provider_invoked"], false);
    assert_eq!(
        value["side_effects"]["external_kg_adapter_read_performed"],
        false
    );
    assert_eq!(value["side_effects"]["credential_read"], false);
    assert_eq!(value["side_effects"]["channel_send_performed"], false);
    assert_eq!(value["side_effects"]["service_restart_performed"], false);
    assert_eq!(value["side_effects"]["release_artifact_written"], false);
    assert_eq!(value["side_effects"]["public_ga_claimed"], false);
}

#[test]
fn hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_shadow_execution_readiness_endpoint_is_source_route_only()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_SHADOW_EXECUTION_READINESS_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value =
        serde_json::from_str(&body).expect("shadow execution readiness json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-readiness --json"
    );
    assert_eq!(
        value["compatibility_mode"],
        "native_runtime_provider_router_shadow_context_activation_execution_readiness_route_source_only"
    );
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_SHADOW_EXECUTION_READINESS_ENDPOINT
        );
    assert_eq!(
        value["source_runtime_readiness_endpoint"],
        HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_READINESS_ENDPOINT
    );
    assert_eq!(
        value["source_runtime_execution_method"],
        "execute_memory_context_activation_shadow"
    );
    assert_eq!(
        value["native_gateway_source_command_count"],
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    assert_eq!(
        value["route_count"],
        serde_json::json!(NATIVE_GATEWAY_SOURCE_COMMAND_COUNT)
    );
    assert_eq!(value["missing_route_count"], 0);
    assert_eq!(value["route_count_cutover_floor"], 69);
    assert_eq!(value["route_count_source_command_accepted"], true);
    assert_eq!(value["source_route_wired"], true);
    assert_eq!(
        value["live_route_active_install_performed_by_this_gate"],
        false
    );
    assert_eq!(value["runtime_readiness_ready"], true);
    assert_eq!(value["runtime_readiness_status"], "ready");
    assert_eq!(
        value["operator_approved_shadow_context_activation_execution_report_ready"],
        true
    );
    assert_eq!(value["runtime_owned_execution_surface_present"], true);
    assert_eq!(value["release_gate_required"], true);
    assert_eq!(value["operator_release_approval_required"], true);
    assert_eq!(value["canary_telemetry_required"], true);
    assert_eq!(value["rollback_kill_switch_required"], true);
    assert_eq!(value["post_activation_watchdog_soak_plan_required"], true);
    assert_eq!(value["idempotency_required"], true);
    assert_eq!(value["traffic_percent_ppm_required"], 0);
    assert_eq!(value["context_handoff_acceptance_required"], true);
    assert_eq!(
        value["shadow_context_attachment_supported_by_runtime"],
        true
    );
    assert_eq!(value["execution_invoked_by_report_route"], false);
    assert_eq!(value["live_route_exposes_activation_command"], false);
    assert_eq!(value["provider_invocation_allowed"], false);
    assert_eq!(value["provider_invocation_performed"], false);
    assert_eq!(value["model_invocation_allowed"], false);
    assert_eq!(value["model_invocation_performed"], false);
    assert_eq!(value["auth_secret_read_allowed"], false);
    assert_eq!(value["auth_secret_read_performed"], false);
    assert_eq!(value["credential_read_allowed"], false);
    assert_eq!(value["credential_read_performed"], false);
    assert_eq!(value["external_network_call_allowed"], false);
    assert_eq!(value["external_network_call_performed"], false);
    assert_eq!(value["live_kg_write_allowed"], false);
    assert_eq!(value["live_kg_write_performed"], false);
    assert_eq!(value["live_mutation_enabled_count"], 0);
    assert_eq!(value["current_live_enabled_lane_count"], 0);
    let blocked = value["blocked_execution_actions"]
        .as_array()
        .expect("blocked execution actions")
        .iter()
        .filter_map(|item| item.as_str())
        .collect::<Vec<_>>();
    assert!(blocked.contains(&"invoke_shadow_execution_from_report_route"));
    assert!(blocked.contains(&"provider_model_invocation"));
    assert!(blocked.contains(&"credential_read"));
    assert!(blocked.contains(&"live_kg_write"));
    assert_eq!(
        value["side_effects"]["report_route_invoked_runtime_execution"],
        false
    );
    assert_eq!(
        value["side_effects"]["runtime_router_shadow_handoff_mutated_by_report_route"],
        false
    );
    assert_eq!(value["side_effects"]["provider_invoked"], false);
    assert_eq!(value["side_effects"]["model_invoked"], false);
    assert_eq!(value["side_effects"]["auth_secret_read"], false);
    assert_eq!(value["side_effects"]["credential_read"], false);
    assert_eq!(
        value["side_effects"]["external_network_call_performed"],
        false
    );
    assert_eq!(value["side_effects"]["live_kg_write_performed"], false);
    assert_eq!(value["side_effects"]["memory_store_mutated"], false);
    assert_eq!(value["side_effects"]["service_restarted"], false);
    assert_eq!(value["side_effects"]["active_binary_mutated"], false);
}

#[test]
fn hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_shadow_execution_controlled_endpoint_is_source_gate_only()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_SHADOW_EXECUTION_CONTROLLED_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value =
        serde_json::from_str(&body).expect("shadow execution controlled json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-controlled --json"
    );
    assert_eq!(
        value["compatibility_mode"],
        "native_runtime_provider_router_shadow_context_activation_execution_controlled_route_source_only"
    );
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_SHADOW_EXECUTION_CONTROLLED_ENDPOINT
        );
    assert_eq!(
            value["readiness_endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_SHADOW_EXECUTION_READINESS_ENDPOINT
        );
    assert_eq!(
        value["source_runtime_execution_method"],
        "execute_memory_context_activation_shadow"
    );
    assert_eq!(
        value["native_gateway_source_command_count"],
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    assert_eq!(
        value["route_count"],
        serde_json::json!(NATIVE_GATEWAY_SOURCE_COMMAND_COUNT)
    );
    assert_eq!(value["missing_route_count"], 0);
    assert_eq!(value["route_count_source_command_accepted"], true);
    assert_eq!(value["source_route_wired"], true);
    assert_eq!(value["readiness_route_ready"], true);
    assert_eq!(value["readiness_route_status"], "ready");
    assert_eq!(value["controlled_shadow_execution_report_ready"], true);
    assert_eq!(value["runtime_owned_execution_surface_present"], true);
    assert_eq!(
        value["controlled_execution_contract"],
        "hepta-runtime-provider-router-shadow-context-activation-controlled-report-v1"
    );
    assert_eq!(value["isolated_fixture_execution_required"], true);
    assert_eq!(
        value["isolated_fixture_execution_performed_by_source_gate"],
        true
    );
    assert_eq!(value["live_route_execution_invoked"], false);
    assert_eq!(value["report_route_exposes_activation_command"], false);
    assert_eq!(value["release_gate_required"], true);
    assert_eq!(value["operator_release_approval_required"], true);
    assert_eq!(value["canary_telemetry_required"], true);
    assert_eq!(value["rollback_kill_switch_required"], true);
    assert_eq!(value["post_activation_watchdog_soak_plan_required"], true);
    assert_eq!(value["idempotency_required"], true);
    assert_eq!(value["traffic_percent_ppm_required"], 0);
    assert_eq!(value["readback_receipt_required"], true);
    assert_eq!(value["audit_evidence_required"], true);
    assert_eq!(
        value["feature_flag_mutation_scope"],
        "isolated_source_fixture_only"
    );
    assert_eq!(
        value["context_attachment_scope"],
        "isolated_source_fixture_only"
    );
    assert_eq!(value["provider_invocation_allowed"], false);
    assert_eq!(value["provider_invocation_performed"], false);
    assert_eq!(value["model_invocation_allowed"], false);
    assert_eq!(value["model_invocation_performed"], false);
    assert_eq!(value["auth_secret_read_allowed"], false);
    assert_eq!(value["auth_secret_read_performed"], false);
    assert_eq!(value["credential_read_allowed"], false);
    assert_eq!(value["credential_read_performed"], false);
    assert_eq!(value["external_network_call_allowed"], false);
    assert_eq!(value["external_network_call_performed"], false);
    assert_eq!(value["live_kg_write_allowed"], false);
    assert_eq!(value["live_kg_write_performed"], false);
    assert_eq!(value["live_memory_write_allowed"], false);
    assert_eq!(value["live_memory_write_performed"], false);
    assert_eq!(value["live_mutation_enabled_count"], 0);
    assert_eq!(value["current_live_enabled_lane_count"], 0);
    let blocked = value["blocked_execution_actions"]
        .as_array()
        .expect("blocked execution actions")
        .iter()
        .filter_map(|item| item.as_str())
        .collect::<Vec<_>>();
    assert!(blocked.contains(&"invoke_shadow_execution_from_report_route"));
    assert!(blocked.contains(&"expose_live_activation_command"));
    assert!(blocked.contains(&"provider_model_invocation"));
    assert!(blocked.contains(&"credential_read"));
    assert!(blocked.contains(&"live_kg_write"));
    assert_eq!(
        value["side_effects"]["report_route_invoked_runtime_execution"],
        false
    );
    assert_eq!(
        value["side_effects"]["source_gate_invokes_isolated_fixture_execution"],
        true
    );
    assert_eq!(
        value["side_effects"]["isolated_fixture_router_mutated_by_source_gate"],
        true
    );
    assert_eq!(
        value["side_effects"]["live_7373_router_mutated_by_report_route"],
        false
    );
    assert_eq!(value["side_effects"]["provider_invoked"], false);
    assert_eq!(value["side_effects"]["model_invoked"], false);
    assert_eq!(value["side_effects"]["auth_secret_read"], false);
    assert_eq!(value["side_effects"]["credential_read"], false);
    assert_eq!(
        value["side_effects"]["external_network_call_performed"],
        false
    );
    assert_eq!(value["side_effects"]["live_kg_write_performed"], false);
    assert_eq!(value["side_effects"]["memory_store_mutated"], false);
    assert_eq!(value["side_effects"]["channel_send_performed"], false);
    assert_eq!(value["side_effects"]["service_restarted"], false);
    assert_eq!(value["side_effects"]["active_binary_mutated"], false);
}

#[test]
fn hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_shadow_execution_controlled_readback_receipt_no_persistence_endpoint_is_report_only()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_SHADOW_EXECUTION_CONTROLLED_READBACK_RECEIPT_NO_PERSISTENCE_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value =
        serde_json::from_str(&body).expect("controlled readback receipt no-persistence json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-controlled-readback-receipt-no-persistence --json"
    );
    assert_eq!(
        value["compatibility_mode"],
        "native_runtime_provider_router_shadow_context_activation_execution_controlled_readback_receipt_no_persistence_route_source_only"
    );
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_SHADOW_EXECUTION_CONTROLLED_READBACK_RECEIPT_NO_PERSISTENCE_ENDPOINT
        );
    assert_eq!(
            value["controlled_endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_SHADOW_EXECUTION_CONTROLLED_ENDPOINT
        );
    assert_eq!(
        value["native_gateway_source_command_count"],
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    assert_eq!(
        value["route_count"],
        serde_json::json!(NATIVE_GATEWAY_SOURCE_COMMAND_COUNT)
    );
    assert_eq!(value["missing_route_count"], 0);
    assert_eq!(value["route_count_source_command_accepted"], true);
    assert_eq!(value["source_route_wired"], true);
    assert_eq!(value["controlled_route_ready"], true);
    assert_eq!(value["controlled_route_status"], "ready");
    assert_eq!(value["controlled_shadow_execution_report_ready"], true);
    assert_eq!(value["readback_receipt_no_persistence_ready"], true);
    assert_eq!(value["readback_receipt_schema_declared"], true);
    assert_eq!(value["readback_receipt_requested"], true);
    assert_eq!(value["readback_receipt_allowed"], false);
    assert_eq!(value["readback_receipt_shape_accepted"], false);
    assert_eq!(value["readback_receipt_recorded"], false);
    assert_eq!(value["readback_receipt_persisted"], false);
    assert_eq!(value["readback_receipt_materialized"], false);
    assert_eq!(value["readback_receipt_filesystem_written"], false);
    assert_eq!(value["readback_receipt_ledger_written"], false);
    assert_eq!(value["readback_receipt_indexed"], false);
    assert_eq!(value["readback_receipt_enqueued"], false);
    assert_eq!(value["readback_receipt_delivered"], false);
    assert_eq!(value["readback_receipt_exported"], false);
    assert_eq!(value["readback_receipt_query_registered"], false);
    assert_eq!(value["readback_receipt_observability_recorded"], false);
    assert_eq!(value["readback_receipt_hash_bound"], false);
    assert_eq!(value["readback_receipt_signature_hash_recorded"], false);
    assert_eq!(value["readback_receipt_timestamp_recorded"], false);
    assert_eq!(value["readback_receipt_operator_identity_accepted"], false);
    assert_eq!(value["readback_receipt_status_accepted"], false);
    assert_eq!(value["completion_ack_recorded"], false);
    assert_eq!(value["completion_ack_persisted"], false);
    assert_eq!(value["completion_ack_accepted"], false);
    assert_eq!(value["operator_approval_from_receipt_accepted"], false);
    assert_eq!(value["activation_from_receipt_allowed"], false);
    assert_eq!(value["activation_authority_derived"], false);
    assert_eq!(value["public_claim_from_receipt_allowed"], false);
    assert_eq!(value["report_route_invokes_shadow_execution"], false);
    assert_eq!(value["report_route_exposes_activation_command"], false);
    assert_eq!(value["live_mutation_enabled_count"], 0);
    assert_eq!(value["current_live_enabled_lane_count"], 0);
    assert_eq!(value["readback_receipt_surface_count"], 10);
    assert_eq!(value["blocked_readback_receipt_fixture_count"], 10);
    assert_eq!(value["allowed_readback_receipt_fixture_count"], 0);
    let blocked = value["blocked_readback_receipt_actions"]
        .as_array()
        .expect("blocked readback receipt actions")
        .iter()
        .filter_map(|item| item.as_str())
        .collect::<Vec<_>>();
    assert!(blocked.contains(&"persist_controlled_readback_receipt"));
    assert!(blocked.contains(&"derive_operator_approval_from_readback_receipt"));
    assert!(blocked.contains(&"derive_activation_authority_from_readback_receipt"));
    assert!(blocked.contains(&"promote_readback_receipt_to_public_claim"));
    assert!(blocked.contains(&"live_kg_or_memory_write"));
    assert_eq!(
        value["side_effects"]["report_route_invoked_runtime_execution"],
        false
    );
    assert_eq!(
        value["side_effects"]["source_gate_invokes_isolated_fixture_execution"],
        true
    );
    assert_eq!(
        value["side_effects"]["live_7373_router_mutated_by_report_route"],
        false
    );
    assert_eq!(value["side_effects"]["readback_receipt_recorded"], false);
    assert_eq!(value["side_effects"]["readback_receipt_persisted"], false);
    assert_eq!(
        value["side_effects"]["readback_receipt_materialized"],
        false
    );
    assert_eq!(
        value["side_effects"]["readback_receipt_filesystem_written"],
        false
    );
    assert_eq!(
        value["side_effects"]["readback_receipt_observability_recorded"],
        false
    );
    assert_eq!(value["side_effects"]["completion_ack_recorded"], false);
    assert_eq!(
        value["side_effects"]["operator_approval_from_receipt_accepted"],
        false
    );
    assert_eq!(
        value["side_effects"]["activation_from_receipt_allowed"],
        false
    );
    assert_eq!(value["side_effects"]["activation_authority_derived"], false);
    assert_eq!(
        value["side_effects"]["public_claim_from_receipt_allowed"],
        false
    );
    assert_eq!(value["side_effects"]["provider_invoked"], false);
    assert_eq!(value["side_effects"]["model_invoked"], false);
    assert_eq!(value["side_effects"]["auth_secret_read"], false);
    assert_eq!(value["side_effects"]["credential_read"], false);
    assert_eq!(
        value["side_effects"]["external_network_call_performed"],
        false
    );
    assert_eq!(value["side_effects"]["live_kg_write_performed"], false);
    assert_eq!(value["side_effects"]["memory_store_mutated"], false);
    assert_eq!(value["side_effects"]["channel_send_performed"], false);
    assert_eq!(value["side_effects"]["external_send_performed"], false);
    assert_eq!(value["side_effects"]["service_restarted"], false);
    assert_eq!(value["side_effects"]["active_binary_mutated"], false);
    assert_eq!(value["side_effects"]["release_artifact_written"], false);
    assert_eq!(value["side_effects"]["public_release_claimed"], false);
}

#[test]
fn hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_shadow_execution_controlled_readback_receipt_authority_denial_endpoint_is_report_only()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_SHADOW_EXECUTION_CONTROLLED_READBACK_RECEIPT_AUTHORITY_DENIAL_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value =
        serde_json::from_str(&body).expect("controlled readback receipt authority denial json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-controlled-readback-receipt-authority-denial --json"
    );
    assert_eq!(
        value["compatibility_mode"],
        "native_runtime_provider_router_shadow_context_activation_execution_controlled_readback_receipt_authority_denial_route_source_only"
    );
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_SHADOW_EXECUTION_CONTROLLED_READBACK_RECEIPT_AUTHORITY_DENIAL_ENDPOINT
        );
    assert_eq!(
            value["no_persistence_endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_SHADOW_EXECUTION_CONTROLLED_READBACK_RECEIPT_NO_PERSISTENCE_ENDPOINT
        );
    assert_eq!(
        value["native_gateway_source_command_count"],
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    assert_eq!(
        value["route_count"],
        serde_json::json!(NATIVE_GATEWAY_SOURCE_COMMAND_COUNT)
    );
    assert_eq!(value["missing_route_count"], 0);
    assert_eq!(value["route_count_source_command_accepted"], true);
    assert_eq!(value["source_route_wired"], true);
    assert_eq!(value["no_persistence_route_ready"], true);
    assert_eq!(value["no_persistence_route_status"], "ready");
    assert_eq!(value["readback_receipt_no_persistence_ready"], true);
    assert_eq!(value["readback_receipt_authority_denial_ready"], true);
    assert_eq!(value["readback_receipt_authority_boundary_declared"], true);
    assert_eq!(value["readback_receipt_shape_observed"], true);
    assert_eq!(value["readback_receipt_shape_accepted"], false);
    assert_eq!(value["trusted_operator_acceptance_record_required"], true);
    assert_eq!(value["trusted_operator_acceptance_record_present"], false);
    assert_eq!(value["trusted_operator_acceptance_record_accepted"], false);
    assert_eq!(value["operator_identity_verified_from_receipt"], false);
    assert_eq!(value["operator_intent_confirmed_from_receipt"], false);
    assert_eq!(value["operator_approval_from_receipt_accepted"], false);
    assert_eq!(value["activation_authority_derived"], false);
    assert_eq!(value["activation_request_from_receipt_allowed"], false);
    assert_eq!(value["activation_command_from_receipt_exposed"], false);
    assert_eq!(value["live_mutation_from_receipt_allowed"], false);
    assert_eq!(value["public_claim_from_receipt_allowed"], false);
    assert_eq!(value["public_release_from_receipt_allowed"], false);
    assert_eq!(value["report_route_invokes_shadow_execution"], false);
    assert_eq!(value["report_route_exposes_activation_command"], false);
    assert_eq!(value["live_mutation_enabled_count"], 0);
    assert_eq!(value["current_live_enabled_lane_count"], 0);
    assert_eq!(value["receipt_authority_fixture_count"], 8);
    assert_eq!(value["blocked_receipt_authority_fixture_count"], 8);
    assert_eq!(value["allowed_receipt_authority_fixture_count"], 0);
    let blocked = value["blocked_receipt_authority_actions"]
        .as_array()
        .expect("blocked readback receipt authority actions")
        .iter()
        .filter_map(|item| item.as_str())
        .collect::<Vec<_>>();
    assert!(blocked.contains(&"accept_readback_receipt_as_trusted_operator_record"));
    assert!(blocked.contains(&"derive_operator_identity_from_readback_receipt"));
    assert!(blocked.contains(&"record_operator_approval_from_readback_receipt"));
    assert!(blocked.contains(&"derive_activation_authority_from_readback_receipt"));
    assert!(blocked.contains(&"expose_activation_command_from_readback_receipt"));
    assert!(blocked.contains(&"live_kg_or_memory_write"));
    assert_eq!(
        value["side_effects"]["report_route_invoked_runtime_execution"],
        false
    );
    assert_eq!(
        value["side_effects"]["source_gate_invokes_isolated_fixture_execution"],
        true
    );
    assert_eq!(
        value["side_effects"]["live_7373_router_mutated_by_report_route"],
        false
    );
    assert_eq!(value["side_effects"]["readback_receipt_recorded"], false);
    assert_eq!(value["side_effects"]["readback_receipt_persisted"], false);
    assert_eq!(
        value["side_effects"]["trusted_operator_acceptance_recorded"],
        false
    );
    assert_eq!(
        value["side_effects"]["trusted_operator_acceptance_record_persisted"],
        false
    );
    assert_eq!(value["side_effects"]["operator_identity_verified"], false);
    assert_eq!(value["side_effects"]["operator_intent_confirmed"], false);
    assert_eq!(value["side_effects"]["operator_approval_recorded"], false);
    assert_eq!(
        value["side_effects"]["activation_authority_recorded"],
        false
    );
    assert_eq!(value["side_effects"]["activation_request_enqueued"], false);
    assert_eq!(value["side_effects"]["activation_command_exposed"], false);
    assert_eq!(value["side_effects"]["public_claim_recorded"], false);
    assert_eq!(value["side_effects"]["provider_invoked"], false);
    assert_eq!(value["side_effects"]["model_invoked"], false);
    assert_eq!(value["side_effects"]["auth_secret_read"], false);
    assert_eq!(value["side_effects"]["credential_read"], false);
    assert_eq!(
        value["side_effects"]["external_network_call_performed"],
        false
    );
    assert_eq!(value["side_effects"]["live_kg_write_performed"], false);
    assert_eq!(value["side_effects"]["memory_store_mutated"], false);
    assert_eq!(value["side_effects"]["channel_send_performed"], false);
    assert_eq!(value["side_effects"]["external_send_performed"], false);
    assert_eq!(value["side_effects"]["service_restarted"], false);
    assert_eq!(value["side_effects"]["active_binary_mutated"], false);
    assert_eq!(value["side_effects"]["release_artifact_written"], false);
    assert_eq!(value["side_effects"]["public_release_claimed"], false);
}

#[test]
fn hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_shadow_execution_controlled_readback_receipt_trusted_operator_packet_separation_endpoint_is_report_only()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_SHADOW_EXECUTION_CONTROLLED_READBACK_RECEIPT_TRUSTED_OPERATOR_PACKET_SEPARATION_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body)
        .expect("controlled readback receipt trusted operator packet separation json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-controlled-readback-receipt-trusted-operator-packet-separation --json"
    );
    assert_eq!(
        value["compatibility_mode"],
        "native_runtime_provider_router_shadow_context_activation_execution_controlled_readback_receipt_trusted_operator_packet_separation_route_source_only"
    );
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_SHADOW_EXECUTION_CONTROLLED_READBACK_RECEIPT_TRUSTED_OPERATOR_PACKET_SEPARATION_ENDPOINT
        );
    assert_eq!(
            value["authority_denial_endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_SHADOW_EXECUTION_CONTROLLED_READBACK_RECEIPT_AUTHORITY_DENIAL_ENDPOINT
        );
    assert_eq!(
        value["native_gateway_source_command_count"],
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    assert_eq!(
        value["route_count"],
        serde_json::json!(NATIVE_GATEWAY_SOURCE_COMMAND_COUNT)
    );
    assert_eq!(value["missing_route_count"], 0);
    assert_eq!(value["route_count_source_command_accepted"], true);
    assert_eq!(value["source_route_wired"], true);
    assert_eq!(value["authority_denial_route_ready"], true);
    assert_eq!(value["authority_denial_route_status"], "ready");
    assert_eq!(value["readback_receipt_authority_denial_ready"], true);
    assert_eq!(value["trusted_operator_packet_separation_ready"], true);
    assert_eq!(value["readback_receipt_shape_observed"], true);
    assert_eq!(value["readback_receipt_shape_accepted"], false);
    assert_eq!(value["independent_trusted_operator_packet_required"], true);
    assert_eq!(value["independent_trusted_operator_packet_present"], false);
    assert_eq!(value["independent_trusted_operator_packet_accepted"], false);
    assert_eq!(
        value["readback_receipt_can_substitute_operator_packet"],
        false
    );
    assert_eq!(value["readback_receipt_can_bind_operator_packet"], false);
    assert_eq!(value["readback_receipt_can_extend_operator_packet"], false);
    assert_eq!(value["readback_receipt_can_refresh_operator_packet"], false);
    assert_eq!(value["readback_receipt_can_replay_operator_packet"], false);
    assert_eq!(
        value["readback_receipt_can_materialize_operator_packet"],
        false
    );
    assert_eq!(value["operator_packet_identity_required"], true);
    assert_eq!(value["operator_packet_intent_required"], true);
    assert_eq!(value["operator_packet_signature_required"], true);
    assert_eq!(value["operator_packet_session_required"], true);
    assert_eq!(value["operator_packet_freshness_required"], true);
    assert_eq!(value["operator_packet_scope_required"], true);
    assert_eq!(value["operator_identity_verified_from_packet"], false);
    assert_eq!(value["operator_intent_confirmed_from_packet"], false);
    assert_eq!(value["operator_approval_from_packet_accepted"], false);
    assert_eq!(value["activation_authority_from_packet_derived"], false);
    assert_eq!(value["activation_request_from_packet_allowed"], false);
    assert_eq!(value["activation_command_from_packet_exposed"], false);
    assert_eq!(value["live_mutation_from_packet_allowed"], false);
    assert_eq!(value["public_claim_from_packet_allowed"], false);
    assert_eq!(value["report_route_invokes_shadow_execution"], false);
    assert_eq!(value["report_route_exposes_activation_command"], false);
    assert_eq!(value["live_mutation_enabled_count"], 0);
    assert_eq!(value["current_live_enabled_lane_count"], 0);
    assert_eq!(value["packet_separation_fixture_count"], 9);
    assert_eq!(value["blocked_packet_substitution_fixture_count"], 9);
    assert_eq!(value["allowed_packet_substitution_fixture_count"], 0);
    let blocked = value["blocked_packet_separation_actions"]
        .as_array()
        .expect("blocked packet separation actions")
        .iter()
        .filter_map(|item| item.as_str())
        .collect::<Vec<_>>();
    assert!(blocked.contains(&"substitute_readback_receipt_for_trusted_operator_packet"));
    assert!(blocked.contains(&"bind_readback_receipt_to_operator_packet"));
    assert!(blocked.contains(&"materialize_trusted_operator_packet_from_readback_receipt"));
    assert!(blocked.contains(&"derive_activation_authority_from_receipt_payload"));
    assert!(blocked.contains(&"expose_activation_command_from_receipt_payload"));
    assert!(blocked.contains(&"live_kg_or_memory_write"));
    assert_eq!(
        value["side_effects"]["report_route_invoked_runtime_execution"],
        false
    );
    assert_eq!(
        value["side_effects"]["source_gate_invokes_isolated_fixture_execution"],
        true
    );
    assert_eq!(
        value["side_effects"]["live_7373_router_mutated_by_report_route"],
        false
    );
    assert_eq!(value["side_effects"]["readback_receipt_recorded"], false);
    assert_eq!(value["side_effects"]["readback_receipt_persisted"], false);
    assert_eq!(
        value["side_effects"]["trusted_operator_packet_recorded"],
        false
    );
    assert_eq!(
        value["side_effects"]["trusted_operator_packet_persisted"],
        false
    );
    assert_eq!(
        value["side_effects"]["trusted_operator_packet_materialized"],
        false
    );
    assert_eq!(
        value["side_effects"]["trusted_operator_packet_accepted"],
        false
    );
    assert_eq!(
        value["side_effects"]["receipt_substituted_operator_packet"],
        false
    );
    assert_eq!(
        value["side_effects"]["receipt_bound_operator_packet"],
        false
    );
    assert_eq!(
        value["side_effects"]["receipt_extended_operator_packet"],
        false
    );
    assert_eq!(
        value["side_effects"]["receipt_refreshed_operator_packet"],
        false
    );
    assert_eq!(
        value["side_effects"]["receipt_replayed_operator_packet"],
        false
    );
    assert_eq!(value["side_effects"]["operator_identity_verified"], false);
    assert_eq!(value["side_effects"]["operator_intent_confirmed"], false);
    assert_eq!(value["side_effects"]["operator_approval_recorded"], false);
    assert_eq!(
        value["side_effects"]["activation_authority_recorded"],
        false
    );
    assert_eq!(value["side_effects"]["activation_request_enqueued"], false);
    assert_eq!(value["side_effects"]["activation_command_exposed"], false);
    assert_eq!(value["side_effects"]["public_claim_recorded"], false);
    assert_eq!(value["side_effects"]["provider_invoked"], false);
    assert_eq!(value["side_effects"]["model_invoked"], false);
    assert_eq!(value["side_effects"]["auth_secret_read"], false);
    assert_eq!(value["side_effects"]["credential_read"], false);
    assert_eq!(
        value["side_effects"]["external_network_call_performed"],
        false
    );
    assert_eq!(value["side_effects"]["live_kg_write_performed"], false);
    assert_eq!(value["side_effects"]["memory_store_mutated"], false);
    assert_eq!(value["side_effects"]["channel_send_performed"], false);
    assert_eq!(value["side_effects"]["external_send_performed"], false);
    assert_eq!(value["side_effects"]["service_restarted"], false);
    assert_eq!(value["side_effects"]["active_binary_mutated"], false);
    assert_eq!(value["side_effects"]["release_artifact_written"], false);
    assert_eq!(value["side_effects"]["public_release_claimed"], false);
}

#[test]
fn hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_shadow_execution_controlled_readback_receipt_trusted_operator_packet_intake_precondition_endpoint_is_report_only()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_SHADOW_EXECUTION_CONTROLLED_READBACK_RECEIPT_TRUSTED_OPERATOR_PACKET_INTAKE_PRECONDITION_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body)
        .expect("controlled readback receipt trusted operator packet intake precondition json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-controlled-readback-receipt-trusted-operator-packet-intake-precondition --json"
    );
    assert_eq!(
        value["compatibility_mode"],
        "native_runtime_provider_router_shadow_context_activation_execution_controlled_readback_receipt_trusted_operator_packet_intake_precondition_route_source_only"
    );
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_SHADOW_EXECUTION_CONTROLLED_READBACK_RECEIPT_TRUSTED_OPERATOR_PACKET_INTAKE_PRECONDITION_ENDPOINT
        );
    assert_eq!(
            value["packet_separation_endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_SHADOW_EXECUTION_CONTROLLED_READBACK_RECEIPT_TRUSTED_OPERATOR_PACKET_SEPARATION_ENDPOINT
        );
    assert_eq!(
        value["native_gateway_source_command_count"],
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    assert_eq!(
        value["route_count"],
        serde_json::json!(NATIVE_GATEWAY_SOURCE_COMMAND_COUNT)
    );
    assert_eq!(value["missing_route_count"], 0);
    assert_eq!(value["route_count_source_command_accepted"], true);
    assert_eq!(value["source_route_wired"], true);
    assert_eq!(value["packet_separation_route_ready"], true);
    assert_eq!(value["packet_separation_route_status"], "ready");
    assert_eq!(value["trusted_operator_packet_separation_ready"], true);
    assert_eq!(
        value["trusted_operator_packet_intake_precondition_ready"],
        true
    );
    assert_eq!(value["independent_trusted_operator_packet_required"], true);
    assert_eq!(value["independent_trusted_operator_packet_present"], false);
    assert_eq!(
        value["independent_trusted_operator_packet_shape_declared"],
        true
    );
    assert_eq!(value["operator_packet_identity_required"], true);
    assert_eq!(value["operator_packet_intent_required"], true);
    assert_eq!(value["operator_packet_signature_required"], true);
    assert_eq!(value["operator_packet_session_required"], true);
    assert_eq!(value["operator_packet_freshness_required"], true);
    assert_eq!(value["operator_packet_scope_required"], true);
    assert_eq!(value["operator_packet_required_field_count"], 6);
    assert_eq!(value["operator_packet_verified_field_count"], 0);
    assert_eq!(value["operator_packet_missing_field_count"], 6);
    assert_eq!(value["operator_packet_identity_verified"], false);
    assert_eq!(value["operator_packet_intent_confirmed"], false);
    assert_eq!(value["operator_packet_signature_verified"], false);
    assert_eq!(value["operator_packet_session_bound"], false);
    assert_eq!(value["operator_packet_freshness_verified"], false);
    assert_eq!(value["operator_packet_scope_validated"], false);
    assert_eq!(
        value["operator_packet_acceptance_precondition_satisfied"],
        false
    );
    assert_eq!(value["operator_packet_recorded"], false);
    assert_eq!(value["operator_packet_persisted"], false);
    assert_eq!(value["operator_packet_accepted"], false);
    assert_eq!(value["operator_approval_from_packet_accepted"], false);
    assert_eq!(value["activation_authority_from_packet_derived"], false);
    assert_eq!(value["activation_request_from_packet_allowed"], false);
    assert_eq!(value["activation_command_from_packet_exposed"], false);
    assert_eq!(value["live_mutation_from_packet_allowed"], false);
    assert_eq!(value["public_claim_from_packet_allowed"], false);
    assert_eq!(value["report_route_invokes_shadow_execution"], false);
    assert_eq!(value["report_route_exposes_activation_command"], false);
    assert_eq!(value["live_mutation_enabled_count"], 0);
    assert_eq!(value["current_live_enabled_lane_count"], 0);
    assert_eq!(value["operator_packet_intake_fixture_count"], 6);
    assert_eq!(value["blocked_operator_packet_intake_fixture_count"], 6);
    assert_eq!(value["allowed_operator_packet_intake_fixture_count"], 0);
    let blocked = value["blocked_operator_packet_intake_actions"]
        .as_array()
        .expect("blocked operator packet intake actions")
        .iter()
        .filter_map(|item| item.as_str())
        .collect::<Vec<_>>();
    assert!(blocked.contains(&"accept_operator_packet_without_identity"));
    assert!(blocked.contains(&"accept_operator_packet_without_signature"));
    assert!(blocked.contains(&"accept_operator_packet_without_freshness"));
    assert!(blocked.contains(&"derive_activation_authority_from_unverified_packet"));
    assert!(blocked.contains(&"expose_activation_command_from_unverified_packet"));
    assert!(blocked.contains(&"live_kg_or_memory_write"));
    assert_eq!(
        value["side_effects"]["report_route_invoked_runtime_execution"],
        false
    );
    assert_eq!(
        value["side_effects"]["source_gate_invokes_isolated_fixture_execution"],
        true
    );
    assert_eq!(
        value["side_effects"]["live_7373_router_mutated_by_report_route"],
        false
    );
    assert_eq!(value["side_effects"]["operator_packet_recorded"], false);
    assert_eq!(value["side_effects"]["operator_packet_persisted"], false);
    assert_eq!(value["side_effects"]["operator_packet_materialized"], false);
    assert_eq!(value["side_effects"]["operator_packet_accepted"], false);
    assert_eq!(
        value["side_effects"]["operator_packet_identity_verified"],
        false
    );
    assert_eq!(
        value["side_effects"]["operator_packet_signature_verified"],
        false
    );
    assert_eq!(
        value["side_effects"]["operator_packet_freshness_verified"],
        false
    );
    assert_eq!(
        value["side_effects"]["operator_packet_scope_validated"],
        false
    );
    assert_eq!(value["side_effects"]["operator_approval_recorded"], false);
    assert_eq!(
        value["side_effects"]["activation_authority_recorded"],
        false
    );
    assert_eq!(value["side_effects"]["activation_request_enqueued"], false);
    assert_eq!(value["side_effects"]["activation_command_exposed"], false);
    assert_eq!(value["side_effects"]["provider_invoked"], false);
    assert_eq!(value["side_effects"]["model_invoked"], false);
    assert_eq!(value["side_effects"]["auth_secret_read"], false);
    assert_eq!(value["side_effects"]["credential_read"], false);
    assert_eq!(value["side_effects"]["live_kg_write_performed"], false);
    assert_eq!(value["side_effects"]["memory_store_mutated"], false);
    assert_eq!(value["side_effects"]["channel_send_performed"], false);
    assert_eq!(value["side_effects"]["external_send_performed"], false);
    assert_eq!(value["side_effects"]["service_restarted"], false);
    assert_eq!(value["side_effects"]["active_binary_mutated"], false);
    assert_eq!(value["side_effects"]["release_artifact_written"], false);
    assert_eq!(value["side_effects"]["public_release_claimed"], false);
}

#[test]
fn hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_shadow_execution_controlled_readback_receipt_trusted_operator_packet_partial_precondition_denial_matrix_endpoint_is_report_only()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_SHADOW_EXECUTION_CONTROLLED_READBACK_RECEIPT_TRUSTED_OPERATOR_PACKET_PARTIAL_PRECONDITION_DENIAL_MATRIX_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body)
            .expect("controlled readback receipt trusted operator packet partial precondition denial matrix json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-controlled-readback-receipt-trusted-operator-packet-partial-precondition-denial-matrix --json"
    );
    assert_eq!(
        value["compatibility_mode"],
        "native_runtime_provider_router_shadow_context_activation_execution_controlled_readback_receipt_trusted_operator_packet_partial_precondition_denial_matrix_route_source_only"
    );
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_SHADOW_EXECUTION_CONTROLLED_READBACK_RECEIPT_TRUSTED_OPERATOR_PACKET_PARTIAL_PRECONDITION_DENIAL_MATRIX_ENDPOINT
        );
    assert_eq!(
            value["intake_precondition_endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_SHADOW_EXECUTION_CONTROLLED_READBACK_RECEIPT_TRUSTED_OPERATOR_PACKET_INTAKE_PRECONDITION_ENDPOINT
        );
    assert_eq!(
        value["native_gateway_source_command_count"],
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    assert_eq!(
        value["route_count"],
        serde_json::json!(NATIVE_GATEWAY_SOURCE_COMMAND_COUNT)
    );
    assert_eq!(value["missing_route_count"], 0);
    assert_eq!(value["route_count_source_command_accepted"], true);
    assert_eq!(value["source_route_wired"], true);
    assert_eq!(value["intake_precondition_route_ready"], true);
    assert_eq!(value["intake_precondition_route_status"], "ready");
    assert_eq!(
        value["trusted_operator_packet_intake_precondition_ready"],
        true
    );
    assert_eq!(
        value["trusted_operator_packet_partial_precondition_denial_matrix_ready"],
        true
    );
    assert_eq!(value["independent_trusted_operator_packet_required"], true);
    assert_eq!(
        value["independent_trusted_operator_packet_shape_declared"],
        true
    );
    assert_eq!(value["operator_packet_required_field_count"], 6);
    assert_eq!(
        value["operator_packet_complete_verified_field_count_required"],
        6
    );
    assert_eq!(value["operator_packet_partial_fixture_count"], 6);
    assert_eq!(value["blocked_operator_packet_partial_fixture_count"], 6);
    assert_eq!(value["allowed_operator_packet_partial_fixture_count"], 0);
    assert_eq!(value["partial_packet_max_verified_field_count"], 5);
    assert_eq!(value["partial_packet_min_missing_field_count"], 1);
    assert_eq!(
        value["partial_packet_acceptance_precondition_satisfied_count"],
        0
    );
    assert_eq!(value["partial_packet_recorded_count"], 0);
    assert_eq!(value["partial_packet_persisted_count"], 0);
    assert_eq!(value["partial_packet_accepted_count"], 0);
    assert_eq!(value["partial_packet_activation_authority_count"], 0);
    assert_eq!(value["partial_packet_activation_command_exposed_count"], 0);
    assert_eq!(value["partial_packet_live_mutation_allowed_count"], 0);
    assert_eq!(value["partial_packet_public_claim_allowed_count"], 0);
    assert_eq!(value["missing_identity_fixture_blocked"], true);
    assert_eq!(value["missing_intent_fixture_blocked"], true);
    assert_eq!(value["missing_signature_fixture_blocked"], true);
    assert_eq!(value["missing_session_fixture_blocked"], true);
    assert_eq!(value["missing_freshness_fixture_blocked"], true);
    assert_eq!(value["missing_scope_fixture_blocked"], true);
    assert_eq!(value["report_route_invokes_shadow_execution"], false);
    assert_eq!(value["report_route_exposes_activation_command"], false);
    assert_eq!(value["live_mutation_enabled_count"], 0);
    assert_eq!(value["current_live_enabled_lane_count"], 0);

    let fixtures = value["partial_precondition_denial_fixtures"]
        .as_array()
        .expect("partial precondition denial fixtures");
    assert_eq!(fixtures.len(), 6);
    let missing_preconditions = fixtures
        .iter()
        .filter_map(|fixture| fixture["missing_precondition"].as_str())
        .collect::<Vec<_>>();
    for missing in [
        "identity",
        "intent",
        "signature",
        "session",
        "freshness",
        "scope",
    ] {
        assert!(missing_preconditions.contains(&missing));
    }
    for fixture in fixtures {
        assert_eq!(fixture["verified_field_count"], 5);
        assert_eq!(fixture["missing_field_count"], 1);
        assert_eq!(fixture["packet_recorded"], false);
        assert_eq!(fixture["packet_persisted"], false);
        assert_eq!(fixture["packet_accepted"], false);
        assert_eq!(fixture["operator_approval_recorded"], false);
        assert_eq!(fixture["activation_authority_derived"], false);
        assert_eq!(fixture["activation_request_allowed"], false);
        assert_eq!(fixture["activation_command_exposed"], false);
        assert_eq!(fixture["live_mutation_allowed"], false);
        assert_eq!(fixture["public_claim_allowed"], false);
    }

    let blocked = value["blocked_operator_packet_partial_precondition_actions"]
        .as_array()
        .expect("blocked partial packet actions")
        .iter()
        .filter_map(|item| item.as_str())
        .collect::<Vec<_>>();
    assert!(blocked.contains(&"accept_partial_operator_packet_missing_identity"));
    assert!(blocked.contains(&"accept_partial_operator_packet_missing_signature"));
    assert!(blocked.contains(&"accept_partial_operator_packet_missing_freshness"));
    assert!(blocked.contains(&"derive_activation_authority_from_partial_operator_packet"));
    assert!(blocked.contains(&"expose_activation_command_from_partial_operator_packet"));
    assert!(blocked.contains(&"live_kg_or_memory_write"));
    assert_eq!(
        value["side_effects"]["report_route_invoked_runtime_execution"],
        false
    );
    assert_eq!(
        value["side_effects"]["source_gate_invokes_isolated_fixture_execution"],
        true
    );
    assert_eq!(
        value["side_effects"]["live_7373_router_mutated_by_report_route"],
        false
    );
    assert_eq!(
        value["side_effects"]["partial_operator_packet_recorded"],
        false
    );
    assert_eq!(
        value["side_effects"]["partial_operator_packet_persisted"],
        false
    );
    assert_eq!(
        value["side_effects"]["partial_operator_packet_materialized"],
        false
    );
    assert_eq!(
        value["side_effects"]["partial_operator_packet_accepted"],
        false
    );
    assert_eq!(
        value["side_effects"]["partial_operator_packet_identity_verified"],
        false
    );
    assert_eq!(
        value["side_effects"]["partial_operator_packet_signature_verified"],
        false
    );
    assert_eq!(
        value["side_effects"]["partial_operator_packet_freshness_verified"],
        false
    );
    assert_eq!(
        value["side_effects"]["partial_operator_packet_scope_validated"],
        false
    );
    assert_eq!(
        value["side_effects"]["partial_operator_packet_activation_authority_recorded"],
        false
    );
    assert_eq!(
        value["side_effects"]["partial_operator_packet_activation_request_enqueued"],
        false
    );
    assert_eq!(
        value["side_effects"]["partial_operator_packet_activation_command_exposed"],
        false
    );
    assert_eq!(
        value["side_effects"]["partial_operator_packet_live_mutation_performed"],
        false
    );
    assert_eq!(
        value["side_effects"]["partial_operator_packet_public_claim_recorded"],
        false
    );
    assert_eq!(value["side_effects"]["provider_invoked"], false);
    assert_eq!(value["side_effects"]["model_invoked"], false);
    assert_eq!(value["side_effects"]["auth_secret_read"], false);
    assert_eq!(value["side_effects"]["credential_read"], false);
    assert_eq!(value["side_effects"]["live_kg_write_performed"], false);
    assert_eq!(value["side_effects"]["memory_store_mutated"], false);
    assert_eq!(value["side_effects"]["channel_send_performed"], false);
    assert_eq!(value["side_effects"]["external_send_performed"], false);
    assert_eq!(value["side_effects"]["service_restarted"], false);
    assert_eq!(value["side_effects"]["active_binary_mutated"], false);
    assert_eq!(value["side_effects"]["release_artifact_written"], false);
    assert_eq!(value["side_effects"]["public_release_claimed"], false);
}

#[test]
fn hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_shadow_execution_controlled_readback_receipt_trusted_operator_packet_complete_precondition_authority_denial_endpoint_is_report_only()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_SHADOW_EXECUTION_CONTROLLED_READBACK_RECEIPT_TRUSTED_OPERATOR_PACKET_COMPLETE_PRECONDITION_AUTHORITY_DENIAL_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body)
            .expect("controlled readback receipt trusted operator packet complete precondition authority denial json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-controlled-readback-receipt-trusted-operator-packet-complete-precondition-authority-denial --json"
    );
    assert_eq!(
        value["compatibility_mode"],
        "native_runtime_provider_router_shadow_context_activation_execution_controlled_readback_receipt_trusted_operator_packet_complete_precondition_authority_denial_route_source_only"
    );
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_SHADOW_EXECUTION_CONTROLLED_READBACK_RECEIPT_TRUSTED_OPERATOR_PACKET_COMPLETE_PRECONDITION_AUTHORITY_DENIAL_ENDPOINT
        );
    assert_eq!(
            value["partial_precondition_denial_matrix_endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_SHADOW_EXECUTION_CONTROLLED_READBACK_RECEIPT_TRUSTED_OPERATOR_PACKET_PARTIAL_PRECONDITION_DENIAL_MATRIX_ENDPOINT
        );
    assert_eq!(
        value["native_gateway_source_command_count"],
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    assert_eq!(
        value["route_count"],
        serde_json::json!(NATIVE_GATEWAY_SOURCE_COMMAND_COUNT)
    );
    assert_eq!(value["missing_route_count"], 0);
    assert_eq!(value["route_count_source_command_accepted"], true);
    assert_eq!(value["source_route_wired"], true);
    assert_eq!(
        value["partial_precondition_denial_matrix_route_ready"],
        true
    );
    assert_eq!(
        value["partial_precondition_denial_matrix_route_status"],
        "ready"
    );
    assert_eq!(
        value["trusted_operator_packet_partial_precondition_denial_matrix_ready"],
        true
    );
    assert_eq!(
        value["trusted_operator_packet_complete_precondition_authority_denial_ready"],
        true
    );
    assert_eq!(value["independent_trusted_operator_packet_required"], true);
    assert_eq!(
        value["independent_trusted_operator_packet_shape_declared"],
        true
    );
    assert_eq!(value["operator_packet_required_field_count"], 6);
    assert_eq!(value["operator_packet_verified_field_count"], 6);
    assert_eq!(value["operator_packet_missing_field_count"], 0);
    assert_eq!(value["operator_packet_identity_verified"], true);
    assert_eq!(value["operator_packet_intent_confirmed"], true);
    assert_eq!(value["operator_packet_signature_verified"], true);
    assert_eq!(value["operator_packet_session_bound"], true);
    assert_eq!(value["operator_packet_freshness_verified"], true);
    assert_eq!(value["operator_packet_scope_validated"], true);
    assert_eq!(
        value["operator_packet_acceptance_precondition_satisfied"],
        true
    );
    assert_eq!(value["operator_packet_recorded"], false);
    assert_eq!(value["operator_packet_persisted"], false);
    assert_eq!(value["operator_packet_accepted"], false);
    assert_eq!(value["operator_approval_from_packet_accepted"], false);
    assert_eq!(value["activation_authority_from_packet_derived"], false);
    assert_eq!(value["activation_request_from_packet_allowed"], false);
    assert_eq!(value["activation_command_from_packet_exposed"], false);
    assert_eq!(value["live_mutation_from_packet_allowed"], false);
    assert_eq!(value["public_claim_from_packet_allowed"], false);
    assert_eq!(value["report_route_invokes_shadow_execution"], false);
    assert_eq!(value["report_route_exposes_activation_command"], false);
    assert_eq!(value["live_mutation_enabled_count"], 0);
    assert_eq!(value["current_live_enabled_lane_count"], 0);
    assert_eq!(value["complete_precondition_fixture_count"], 1);
    assert_eq!(
        value["complete_precondition_authority_denied_fixture_count"],
        1
    );
    assert_eq!(
        value["complete_precondition_authority_allowed_fixture_count"],
        0
    );

    let fixtures = value["complete_precondition_authority_denial_fixtures"]
        .as_array()
        .expect("complete precondition authority denial fixtures");
    assert_eq!(fixtures.len(), 1);
    let fixture = &fixtures[0];
    assert_eq!(
        fixture["fixture_id"],
        "complete_operator_packet_all_preconditions_verified_authority_denied"
    );
    assert_eq!(fixture["verified_field_count"], 6);
    assert_eq!(fixture["missing_field_count"], 0);
    assert_eq!(fixture["identity_verified"], true);
    assert_eq!(fixture["intent_confirmed"], true);
    assert_eq!(fixture["signature_verified"], true);
    assert_eq!(fixture["session_bound"], true);
    assert_eq!(fixture["freshness_verified"], true);
    assert_eq!(fixture["scope_validated"], true);
    assert_eq!(fixture["acceptance_precondition_satisfied"], true);
    assert_eq!(fixture["packet_recorded"], false);
    assert_eq!(fixture["packet_persisted"], false);
    assert_eq!(fixture["packet_accepted"], false);
    assert_eq!(fixture["operator_approval_recorded"], false);
    assert_eq!(fixture["activation_authority_derived"], false);
    assert_eq!(fixture["activation_request_allowed"], false);
    assert_eq!(fixture["activation_command_exposed"], false);
    assert_eq!(fixture["live_mutation_allowed"], false);
    assert_eq!(fixture["public_claim_allowed"], false);

    let blocked = value["blocked_operator_packet_complete_precondition_authority_actions"]
        .as_array()
        .expect("blocked complete packet actions")
        .iter()
        .filter_map(|item| item.as_str())
        .collect::<Vec<_>>();
    assert!(blocked.contains(&"record_complete_operator_packet_from_report_route"));
    assert!(blocked.contains(&"accept_complete_operator_packet_from_report_route"));
    assert!(blocked.contains(&"derive_activation_authority_from_complete_operator_packet"));
    assert!(blocked.contains(&"expose_activation_command_from_complete_operator_packet"));
    assert!(blocked.contains(&"live_kg_or_memory_write"));
    assert_eq!(
        value["side_effects"]["report_route_invoked_runtime_execution"],
        false
    );
    assert_eq!(
        value["side_effects"]["source_gate_invokes_isolated_fixture_execution"],
        true
    );
    assert_eq!(
        value["side_effects"]["live_7373_router_mutated_by_report_route"],
        false
    );
    assert_eq!(
        value["side_effects"]["complete_operator_packet_recorded"],
        false
    );
    assert_eq!(
        value["side_effects"]["complete_operator_packet_persisted"],
        false
    );
    assert_eq!(
        value["side_effects"]["complete_operator_packet_materialized"],
        false
    );
    assert_eq!(
        value["side_effects"]["complete_operator_packet_accepted"],
        false
    );
    assert_eq!(
        value["side_effects"]["complete_operator_packet_operator_approval_recorded"],
        false
    );
    assert_eq!(
        value["side_effects"]["complete_operator_packet_activation_authority_recorded"],
        false
    );
    assert_eq!(
        value["side_effects"]["complete_operator_packet_activation_request_enqueued"],
        false
    );
    assert_eq!(
        value["side_effects"]["complete_operator_packet_activation_command_exposed"],
        false
    );
    assert_eq!(
        value["side_effects"]["complete_operator_packet_live_mutation_performed"],
        false
    );
    assert_eq!(
        value["side_effects"]["complete_operator_packet_public_claim_recorded"],
        false
    );
    assert_eq!(value["side_effects"]["provider_invoked"], false);
    assert_eq!(value["side_effects"]["model_invoked"], false);
    assert_eq!(value["side_effects"]["auth_secret_read"], false);
    assert_eq!(value["side_effects"]["credential_read"], false);
    assert_eq!(value["side_effects"]["live_kg_write_performed"], false);
    assert_eq!(value["side_effects"]["memory_store_mutated"], false);
    assert_eq!(value["side_effects"]["channel_send_performed"], false);
    assert_eq!(value["side_effects"]["external_send_performed"], false);
    assert_eq!(value["side_effects"]["service_restarted"], false);
    assert_eq!(value["side_effects"]["active_binary_mutated"], false);
    assert_eq!(value["side_effects"]["release_artifact_written"], false);
    assert_eq!(value["side_effects"]["public_release_claimed"], false);
}

#[test]
fn hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_shadow_execution_controlled_readback_receipt_trusted_operator_packet_complete_precondition_operator_approval_lane_separation_endpoint_is_report_only()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_SHADOW_EXECUTION_CONTROLLED_READBACK_RECEIPT_TRUSTED_OPERATOR_PACKET_COMPLETE_PRECONDITION_OPERATOR_APPROVAL_LANE_SEPARATION_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body)
            .expect("controlled readback receipt trusted operator packet complete precondition operator approval lane separation json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-controlled-readback-receipt-trusted-operator-packet-complete-precondition-operator-approval-lane-separation --json"
    );
    assert_eq!(
        value["compatibility_mode"],
        "native_runtime_provider_router_shadow_context_activation_execution_controlled_readback_receipt_trusted_operator_packet_complete_precondition_operator_approval_lane_separation_route_source_only"
    );
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_SHADOW_EXECUTION_CONTROLLED_READBACK_RECEIPT_TRUSTED_OPERATOR_PACKET_COMPLETE_PRECONDITION_OPERATOR_APPROVAL_LANE_SEPARATION_ENDPOINT
        );
    assert_eq!(
            value["complete_precondition_authority_denial_endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_SHADOW_EXECUTION_CONTROLLED_READBACK_RECEIPT_TRUSTED_OPERATOR_PACKET_COMPLETE_PRECONDITION_AUTHORITY_DENIAL_ENDPOINT
        );
    assert_eq!(
        value["native_gateway_source_command_count"],
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    assert_eq!(
        value["route_count"],
        serde_json::json!(NATIVE_GATEWAY_SOURCE_COMMAND_COUNT)
    );
    assert_eq!(value["missing_route_count"], 0);
    assert_eq!(value["route_count_source_command_accepted"], true);
    assert_eq!(value["source_route_wired"], true);
    assert_eq!(
        value["complete_precondition_authority_denial_route_ready"],
        true
    );
    assert_eq!(
        value["complete_precondition_authority_denial_route_status"],
        "ready"
    );
    assert_eq!(
        value["trusted_operator_packet_complete_precondition_authority_denial_ready"],
        true
    );
    assert_eq!(
        value["trusted_operator_packet_complete_precondition_operator_approval_lane_separation_ready"],
        true
    );
    assert_eq!(value["operator_packet_required_field_count"], 6);
    assert_eq!(value["operator_packet_verified_field_count"], 6);
    assert_eq!(value["operator_packet_missing_field_count"], 0);
    assert_eq!(
        value["operator_packet_acceptance_precondition_satisfied"],
        true
    );
    assert_eq!(value["operator_packet_accepted"], false);
    assert_eq!(value["operator_approval_from_packet_accepted"], false);
    assert_eq!(
        value["complete_precondition_can_substitute_operator_approval"],
        false
    );
    assert_eq!(
        value["complete_precondition_can_create_activation_lane"],
        false
    );
    assert_eq!(value["operator_approved_activation_lane_required"], true);
    assert_eq!(value["operator_approved_activation_lane_present"], false);
    assert_eq!(value["activation_lane_acceptance_allowed"], false);
    assert_eq!(value["activation_lane_recorded"], false);
    assert_eq!(value["activation_lane_persisted"], false);
    assert_eq!(value["activation_lane_enqueued"], false);
    assert_eq!(value["activation_lane_effective"], false);
    assert_eq!(value["activation_authority_from_packet_derived"], false);
    assert_eq!(value["activation_request_from_packet_allowed"], false);
    assert_eq!(value["activation_command_from_packet_exposed"], false);
    assert_eq!(value["live_mutation_from_packet_allowed"], false);
    assert_eq!(value["public_claim_from_packet_allowed"], false);
    assert_eq!(value["report_route_invokes_shadow_execution"], false);
    assert_eq!(value["report_route_exposes_activation_command"], false);
    assert_eq!(value["live_mutation_enabled_count"], 0);
    assert_eq!(value["current_live_enabled_lane_count"], 0);
    assert_eq!(value["operator_approval_receipt_required"], true);
    assert_eq!(value["rollback_kill_switch_required"], true);
    assert_eq!(value["post_activation_watchdog_soak_plan_required"], true);

    let blocked = value["blocked_operator_approval_lane_actions"]
        .as_array()
        .expect("blocked operator approval lane actions")
        .iter()
        .filter_map(|item| item.as_str())
        .collect::<Vec<_>>();
    assert!(blocked.contains(&"substitute_complete_packet_preconditions_for_operator_approval"));
    assert!(blocked.contains(&"create_operator_approval_lane_from_complete_packet_fixture"));
    assert!(blocked.contains(&"record_operator_approval_lane_from_report_route"));
    assert!(blocked.contains(&"derive_activation_authority_from_operator_approval_lane_report"));
    assert!(blocked.contains(&"live_kg_or_memory_write"));
    assert_eq!(
        value["side_effects"]["report_route_invoked_runtime_execution"],
        false
    );
    assert_eq!(
        value["side_effects"]["source_gate_invokes_isolated_fixture_execution"],
        true
    );
    assert_eq!(
        value["side_effects"]["live_7373_router_mutated_by_report_route"],
        false
    );
    assert_eq!(
        value["side_effects"]["complete_operator_packet_accepted"],
        false
    );
    assert_eq!(
        value["side_effects"]["operator_approval_lane_recorded"],
        false
    );
    assert_eq!(
        value["side_effects"]["operator_approval_lane_persisted"],
        false
    );
    assert_eq!(
        value["side_effects"]["operator_approval_lane_materialized"],
        false
    );
    assert_eq!(
        value["side_effects"]["operator_approval_lane_enqueued"],
        false
    );
    assert_eq!(
        value["side_effects"]["operator_approval_lane_effective"],
        false
    );
    assert_eq!(
        value["side_effects"]["activation_authority_recorded"],
        false
    );
    assert_eq!(value["side_effects"]["activation_request_enqueued"], false);
    assert_eq!(value["side_effects"]["activation_command_exposed"], false);
    assert_eq!(value["side_effects"]["live_mutation_performed"], false);
    assert_eq!(value["side_effects"]["public_claim_recorded"], false);
    assert_eq!(value["side_effects"]["provider_invoked"], false);
    assert_eq!(value["side_effects"]["model_invoked"], false);
    assert_eq!(value["side_effects"]["auth_secret_read"], false);
    assert_eq!(value["side_effects"]["credential_read"], false);
    assert_eq!(value["side_effects"]["live_kg_write_performed"], false);
    assert_eq!(value["side_effects"]["memory_store_mutated"], false);
    assert_eq!(value["side_effects"]["channel_send_performed"], false);
    assert_eq!(value["side_effects"]["external_send_performed"], false);
    assert_eq!(value["side_effects"]["service_restarted"], false);
    assert_eq!(value["side_effects"]["active_binary_mutated"], false);
    assert_eq!(value["side_effects"]["release_artifact_written"], false);
    assert_eq!(value["side_effects"]["public_release_claimed"], false);
}

#[test]
fn hepta_memory_intelligence_kg_full_enablement_operator_approved_memory_live_mutation_durable_lane_endpoint_enables_memory_lane_only()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_MEMORY_LIVE_MUTATION_DURABLE_LANE_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body)
        .expect("operator-approved memory live mutation durable lane json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-enablement-operator-approved-memory-live-mutation-durable-lane --json"
    );
    assert_eq!(
        value["compatibility_mode"],
        "native_full_enablement_operator_approved_memory_live_mutation_durable_lane_status"
    );
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_MEMORY_LIVE_MUTATION_DURABLE_LANE_ENDPOINT
        );
    assert_eq!(
            value["operator_approval_lane_separation_endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_RUNTIME_PROVIDER_ROUTER_SHADOW_EXECUTION_CONTROLLED_READBACK_RECEIPT_TRUSTED_OPERATOR_PACKET_COMPLETE_PRECONDITION_OPERATOR_APPROVAL_LANE_SEPARATION_ENDPOINT
        );
    assert_eq!(
        value["native_gateway_source_command_count"],
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    assert_eq!(
        value["route_count"],
        serde_json::json!(NATIVE_GATEWAY_SOURCE_COMMAND_COUNT)
    );
    assert_eq!(value["missing_route_count"], 0);
    assert_eq!(value["route_count_source_command_accepted"], true);
    assert_eq!(value["source_route_wired"], true);
    assert_eq!(value["operator_approval_lane_separation_route_ready"], true);
    assert_eq!(
        value["operator_approval_lane_separation_route_status"],
        "ready"
    );
    assert_eq!(value["operator_authorization_received"], true);
    assert_eq!(
        value["operator_authorization_scope"],
        "memory_durable_mutation_lane_only_no_kg_provider_model_channel_or_public_release"
    );
    assert_eq!(value["operator_approved_activation_lane_present"], true);
    assert_eq!(value["operator_approved_activation_lane_effective"], true);
    assert_eq!(
        value["operator_approval_receipt_required_for_write_execution"],
        true
    );
    assert_eq!(
        value["operator_approval_receipt_recorded_by_report_route"],
        false
    );
    assert_eq!(
        value["operator_approval_receipt_persisted_by_report_route"],
        false
    );
    assert_eq!(value["rollback_kill_switch_required"], true);
    assert_eq!(value["rollback_kill_switch_present"], true);
    assert_eq!(value["post_write_validation_required"], true);
    assert_eq!(value["post_write_validation_present"], true);
    assert_eq!(value["idempotency_required"], true);
    assert_eq!(value["idempotency_key_required_for_write_execution"], true);
    assert_eq!(value["memory_durable_mutation_lane_enabled"], true);
    assert_eq!(value["memory_store_write_path_enabled"], true);
    assert_eq!(value["memory_store_mutation_enabled"], true);
    assert_eq!(value["live_memory_write_allowed_by_lane"], true);
    assert_eq!(value["live_memory_write_performed_by_report_route"], false);
    assert_eq!(
        value["memory_write_execution_requires_explicit_command"],
        true
    );
    assert_eq!(
        value["memory_write_execution_command_exposed_by_report_route"],
        false
    );
    assert_eq!(value["memory_write_receipt_required"], true);
    assert_eq!(
        value["memory_write_receipt_recorded_by_report_route"],
        false
    );
    assert_eq!(value["kg_prompt_preview_lane_enabled"], false);
    assert_eq!(value["kg_external_adapter_read_lane_enabled"], false);
    assert_eq!(value["kg_live_write_lane_enabled"], false);
    assert_eq!(
        value["hepta_intelligence_context_attachment_lane_enabled"],
        false
    );
    assert_eq!(value["provider_model_invocation_lane_enabled"], false);
    assert_eq!(value["channel_delivery_lane_enabled"], false);
    assert_eq!(value["live_mutation_enabled_count"], 1);
    assert_eq!(value["current_live_enabled_lane_count"], 1);
    assert_eq!(value["enablement_lane_count"], 6);
    assert_eq!(value["ready_enablement_lane_count"], 6);

    let blocked = value["blocked_actions"]
        .as_array()
        .expect("blocked memory durable lane actions")
        .iter()
        .filter_map(|item| item.as_str())
        .collect::<Vec<_>>();
    assert!(blocked.contains(&"write_memory_from_report_route"));
    assert!(blocked.contains(&"invoke_provider_or_model"));
    assert!(blocked.contains(&"read_external_kg_adapter"));
    assert!(blocked.contains(&"write_live_kg"));
    assert!(blocked.contains(&"telegram_or_channel_delivery"));
    assert_eq!(
        value["side_effects"]["report_route_invoked_runtime_execution"],
        false
    );
    assert_eq!(
        value["side_effects"]["live_7373_router_mutated_by_report_route"],
        false
    );
    assert_eq!(
        value["side_effects"]["memory_store_write_path_enabled_by_report_route"],
        false
    );
    assert_eq!(value["side_effects"]["memory_store_mutated"], false);
    assert_eq!(value["side_effects"]["memory_store_write_performed"], false);
    assert_eq!(
        value["side_effects"]["memory_write_receipt_recorded"],
        false
    );
    assert_eq!(
        value["side_effects"]["post_write_validation_performed"],
        false
    );
    assert_eq!(
        value["side_effects"]["hepta_intelligence_context_attached"],
        false
    );
    assert_eq!(value["side_effects"]["prompt_preview_rendered"], false);
    assert_eq!(value["side_effects"]["context_injection_performed"], false);
    assert_eq!(value["side_effects"]["provider_invoked"], false);
    assert_eq!(value["side_effects"]["model_invoked"], false);
    assert_eq!(value["side_effects"]["auth_secret_read"], false);
    assert_eq!(value["side_effects"]["credential_read"], false);
    assert_eq!(
        value["side_effects"]["external_kg_adapter_read_performed"],
        false
    );
    assert_eq!(value["side_effects"]["live_kg_write_performed"], false);
    assert_eq!(value["side_effects"]["channel_send_performed"], false);
    assert_eq!(value["side_effects"]["external_send_performed"], false);
    assert_eq!(value["side_effects"]["service_restarted"], false);
    assert_eq!(value["side_effects"]["active_binary_mutated"], false);
    assert_eq!(value["side_effects"]["public_release_claimed"], false);
    assert_eq!(value["side_effects"]["public_ga_claimed"], false);
}

#[test]
fn hepta_memory_intelligence_kg_full_enablement_operator_approved_hepta_intelligence_context_attachment_lane_endpoint_enables_context_lane_only()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_HEPTA_INTELLIGENCE_CONTEXT_ATTACHMENT_LANE_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body)
        .expect("operator-approved Hepta Intelligence context attachment lane json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-enablement-operator-approved-hepta-intelligence-context-attachment-lane --json"
    );
    assert_eq!(
        value["compatibility_mode"],
        "native_full_enablement_operator_approved_hepta_intelligence_context_attachment_lane_status"
    );
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_HEPTA_INTELLIGENCE_CONTEXT_ATTACHMENT_LANE_ENDPOINT
        );
    assert_eq!(
            value["memory_live_mutation_durable_lane_endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_MEMORY_LIVE_MUTATION_DURABLE_LANE_ENDPOINT
        );
    assert_eq!(
        value["native_gateway_source_command_count"],
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    assert_eq!(
        value["route_count"],
        serde_json::json!(NATIVE_GATEWAY_SOURCE_COMMAND_COUNT)
    );
    assert_eq!(value["missing_route_count"], 0);
    assert_eq!(value["route_count_source_command_accepted"], true);
    assert_eq!(value["source_route_wired"], true);
    assert_eq!(value["memory_live_mutation_durable_lane_ready"], true);
    assert_eq!(value["memory_live_mutation_durable_lane_status"], "ready");
    assert_eq!(value["operator_authorization_received"], true);
    assert_eq!(
        value["operator_authorization_scope"],
        "hepta_intelligence_context_attachment_and_bounded_prompt_preview_lane_no_provider_model_kg_write_channel_or_public_release"
    );
    assert_eq!(value["operator_approved_activation_lane_present"], true);
    assert_eq!(value["operator_approved_activation_lane_effective"], true);
    assert_eq!(value["memory_durable_mutation_lane_enabled"], true);
    assert_eq!(value["memory_store_write_path_enabled"], true);
    assert_eq!(value["memory_store_mutation_enabled"], true);
    assert_eq!(value["live_memory_write_allowed_by_lane"], true);
    assert_eq!(value["live_memory_write_performed_by_report_route"], false);
    assert_eq!(
        value["hepta_intelligence_context_attachment_lane_enabled"],
        true
    );
    assert_eq!(
        value["hepta_intelligence_context_attachment_allowed_by_lane"],
        true
    );
    assert_eq!(
        value["hepta_intelligence_context_attached_by_report_route"],
        false
    );
    assert_eq!(value["bounded_prompt_preview_lane_enabled"], true);
    assert_eq!(value["bounded_prompt_preview_allowed_by_lane"], true);
    assert_eq!(value["prompt_preview_rendered_by_report_route"], false);
    assert_eq!(value["prompt_payload_materialized_by_report_route"], false);
    assert_eq!(value["context_handoff_acceptance_required"], true);
    assert_eq!(value["context_attachment_requires_explicit_command"], true);
    assert_eq!(value["prompt_preview_requires_explicit_command"], true);
    assert_eq!(value["context_injection_allowed_by_lane"], false);
    assert_eq!(value["context_injection_performed_by_report_route"], false);
    assert_eq!(value["kg_prompt_preview_lane_enabled"], false);
    assert_eq!(value["kg_external_adapter_read_lane_enabled"], false);
    assert_eq!(value["kg_live_write_lane_enabled"], false);
    assert_eq!(value["provider_model_invocation_lane_enabled"], false);
    assert_eq!(value["channel_delivery_lane_enabled"], false);
    assert_eq!(value["live_mutation_enabled_count"], 1);
    assert_eq!(value["current_live_enabled_lane_count"], 2);
    assert_eq!(value["enablement_lane_count"], 6);
    assert_eq!(value["ready_enablement_lane_count"], 6);

    let blocked = value["blocked_actions"]
        .as_array()
        .expect("blocked intelligence context attachment lane actions")
        .iter()
        .filter_map(|item| item.as_str())
        .collect::<Vec<_>>();
    assert!(blocked.contains(&"attach_unbounded_context"));
    assert!(blocked.contains(&"render_prompt_preview_from_report_route"));
    assert!(blocked.contains(&"inject_context_into_provider_prompt"));
    assert!(blocked.contains(&"invoke_provider_or_model"));
    assert!(blocked.contains(&"read_external_kg_adapter"));
    assert!(blocked.contains(&"write_live_kg"));
    assert!(blocked.contains(&"telegram_or_channel_delivery"));
    assert_eq!(
        value["side_effects"]["report_route_invoked_runtime_execution"],
        false
    );
    assert_eq!(value["side_effects"]["memory_store_mutated"], false);
    assert_eq!(value["side_effects"]["memory_store_write_performed"], false);
    assert_eq!(
        value["side_effects"]["hepta_intelligence_context_attached"],
        false
    );
    assert_eq!(value["side_effects"]["prompt_preview_rendered"], false);
    assert_eq!(value["side_effects"]["prompt_payload_materialized"], false);
    assert_eq!(value["side_effects"]["context_injection_performed"], false);
    assert_eq!(value["side_effects"]["provider_invoked"], false);
    assert_eq!(value["side_effects"]["model_invoked"], false);
    assert_eq!(value["side_effects"]["auth_secret_read"], false);
    assert_eq!(value["side_effects"]["credential_read"], false);
    assert_eq!(
        value["side_effects"]["external_kg_adapter_read_performed"],
        false
    );
    assert_eq!(value["side_effects"]["live_kg_write_performed"], false);
    assert_eq!(value["side_effects"]["channel_send_performed"], false);
    assert_eq!(value["side_effects"]["external_send_performed"], false);
    assert_eq!(value["side_effects"]["service_restarted"], false);
    assert_eq!(value["side_effects"]["active_binary_mutated"], false);
    assert_eq!(value["side_effects"]["public_release_claimed"], false);
    assert_eq!(value["side_effects"]["public_ga_claimed"], false);
}

#[test]
fn hepta_memory_intelligence_kg_full_enablement_operator_approved_kg_prompt_preview_read_only_adapter_lane_endpoint_enables_preview_adapter_lane_only()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
            "GET",
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_KG_PROMPT_PREVIEW_READ_ONLY_ADAPTER_LANE_ENDPOINT,
            &options,
        );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body)
        .expect("operator-approved KG prompt preview read-only adapter lane json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-full-enablement-operator-approved-kg-prompt-preview-read-only-adapter-lane --json"
    );
    assert_eq!(
        value["compatibility_mode"],
        "native_full_enablement_operator_approved_kg_prompt_preview_read_only_adapter_lane_status"
    );
    assert_eq!(
            value["endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_KG_PROMPT_PREVIEW_READ_ONLY_ADAPTER_LANE_ENDPOINT
        );
    assert_eq!(
            value["hepta_intelligence_context_attachment_lane_endpoint"],
            HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_HEPTA_INTELLIGENCE_CONTEXT_ATTACHMENT_LANE_ENDPOINT
        );
    assert_eq!(
        value["native_gateway_source_command_count"],
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    assert_eq!(
        value["route_count"],
        serde_json::json!(NATIVE_GATEWAY_SOURCE_COMMAND_COUNT)
    );
    assert_eq!(value["missing_route_count"], 0);
    assert_eq!(value["route_count_source_command_accepted"], true);
    assert_eq!(value["source_route_wired"], true);
    assert_eq!(
        value["hepta_intelligence_context_attachment_lane_ready"],
        true
    );
    assert_eq!(
        value["hepta_intelligence_context_attachment_lane_status"],
        "ready"
    );
    assert_eq!(value["operator_authorization_received"], true);
    assert_eq!(
        value["operator_authorization_scope"],
        "kg_prompt_preview_read_only_adapter_lane_no_kg_live_write_provider_model_channel_or_public_release"
    );
    assert_eq!(value["operator_approved_activation_lane_present"], true);
    assert_eq!(value["operator_approved_activation_lane_effective"], true);
    assert_eq!(value["memory_durable_mutation_lane_enabled"], true);
    assert_eq!(value["memory_store_write_path_enabled"], true);
    assert_eq!(value["memory_store_mutation_enabled"], true);
    assert_eq!(value["live_memory_write_allowed_by_lane"], true);
    assert_eq!(value["live_memory_write_performed_by_report_route"], false);
    assert_eq!(
        value["hepta_intelligence_context_attachment_lane_enabled"],
        true
    );
    assert_eq!(
        value["hepta_intelligence_context_attachment_allowed_by_lane"],
        true
    );
    assert_eq!(
        value["hepta_intelligence_context_attached_by_report_route"],
        false
    );
    assert_eq!(value["bounded_prompt_preview_lane_enabled"], true);
    assert_eq!(value["bounded_prompt_preview_allowed_by_lane"], true);
    assert_eq!(value["prompt_preview_rendered_by_report_route"], false);
    assert_eq!(value["prompt_payload_materialized_by_report_route"], false);
    assert_eq!(value["prompt_preview_requires_explicit_command"], true);
    assert_eq!(value["kg_prompt_preview_lane_enabled"], true);
    assert_eq!(value["kg_prompt_preview_allowed_by_lane"], true);
    assert_eq!(value["kg_prompt_preview_rendered_by_report_route"], false);
    assert_eq!(value["kg_external_adapter_read_lane_enabled"], true);
    assert_eq!(value["kg_external_adapter_read_allowed_by_lane"], true);
    assert_eq!(
        value["kg_external_adapter_read_performed_by_report_route"],
        false
    );
    assert_eq!(value["kg_external_adapter_requires_explicit_command"], true);
    assert_eq!(
        value["kg_external_adapter_credential_reference_required"],
        true
    );
    assert_eq!(
        value["kg_external_adapter_credential_read_allowed_by_lane"],
        false
    );
    assert_eq!(
        value["kg_external_adapter_credential_read_performed_by_report_route"],
        false
    );
    assert_eq!(value["supported_kg_adapter_count"], 3);
    let adapters = value["supported_kg_adapters"]
        .as_array()
        .expect("supported KG adapters")
        .iter()
        .filter_map(|item| item.as_str())
        .collect::<Vec<_>>();
    assert!(adapters.contains(&"graphiti"));
    assert!(adapters.contains(&"neo4j"));
    assert!(adapters.contains(&"cocoindex"));
    assert_eq!(value["context_handoff_acceptance_required"], true);
    assert_eq!(value["context_attachment_requires_explicit_command"], true);
    assert_eq!(value["context_injection_allowed_by_lane"], false);
    assert_eq!(value["context_injection_performed_by_report_route"], false);
    assert_eq!(value["kg_live_write_lane_enabled"], false);
    assert_eq!(value["kg_live_write_allowed_by_lane"], false);
    assert_eq!(value["kg_live_write_performed_by_report_route"], false);
    assert_eq!(value["provider_model_invocation_lane_enabled"], false);
    assert_eq!(value["provider_model_invocation_allowed_by_lane"], false);
    assert_eq!(value["channel_delivery_lane_enabled"], false);
    assert_eq!(value["live_mutation_enabled_count"], 1);
    assert_eq!(value["current_live_enabled_lane_count"], 3);
    assert_eq!(value["enablement_lane_count"], 6);
    assert_eq!(value["ready_enablement_lane_count"], 6);

    let blocked = value["blocked_actions"]
        .as_array()
        .expect("blocked KG prompt preview read-only adapter lane actions")
        .iter()
        .filter_map(|item| item.as_str())
        .collect::<Vec<_>>();
    assert!(blocked.contains(&"render_prompt_preview_from_report_route"));
    assert!(blocked.contains(&"read_kg_adapter_from_report_route"));
    assert!(blocked.contains(&"capture_kg_adapter_endpoint_or_credential_value"));
    assert!(blocked.contains(&"read_auth_secret_or_credential"));
    assert!(blocked.contains(&"write_live_kg"));
    assert!(blocked.contains(&"invoke_provider_or_model"));
    assert!(blocked.contains(&"telegram_or_channel_delivery"));
    assert_eq!(
        value["side_effects"]["report_route_invoked_runtime_execution"],
        false
    );
    assert_eq!(value["side_effects"]["memory_store_mutated"], false);
    assert_eq!(value["side_effects"]["memory_store_write_performed"], false);
    assert_eq!(
        value["side_effects"]["hepta_intelligence_context_attached"],
        false
    );
    assert_eq!(value["side_effects"]["prompt_preview_rendered"], false);
    assert_eq!(value["side_effects"]["prompt_payload_materialized"], false);
    assert_eq!(value["side_effects"]["context_injection_performed"], false);
    assert_eq!(value["side_effects"]["provider_invoked"], false);
    assert_eq!(value["side_effects"]["model_invoked"], false);
    assert_eq!(value["side_effects"]["auth_secret_read"], false);
    assert_eq!(value["side_effects"]["credential_read"], false);
    assert_eq!(
        value["side_effects"]["external_kg_adapter_read_performed"],
        false
    );
    assert_eq!(value["side_effects"]["live_kg_write_performed"], false);
    assert_eq!(value["side_effects"]["channel_send_performed"], false);
    assert_eq!(value["side_effects"]["external_send_performed"], false);
    assert_eq!(value["side_effects"]["service_restarted"], false);
    assert_eq!(value["side_effects"]["active_binary_mutated"], false);
    assert_eq!(value["side_effects"]["public_release_claimed"], false);
    assert_eq!(value["side_effects"]["public_ga_claimed"], false);
}

#[test]
fn hepta_memory_intelligence_kg_activation_truth_index_endpoint_separates_lane_readiness_from_full_live_activation()
 {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request(
        "GET",
        HEPTA_MEMORY_INTELLIGENCE_KG_ACTIVATION_TRUTH_INDEX_ENDPOINT,
        &options,
    );
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value =
        serde_json::from_str(&body).expect("Memory/Intelligence/KG activation truth index json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(
        value["source_command"],
        "/hepta-memory-intelligence-kg-activation-truth-index --json"
    );
    assert_eq!(
        value["compatibility_mode"],
        "native_memory_intelligence_kg_activation_truth_index_read_only"
    );
    assert_eq!(
        value["endpoint"],
        HEPTA_MEMORY_INTELLIGENCE_KG_ACTIVATION_TRUTH_INDEX_ENDPOINT
    );
    assert_eq!(
        value["native_gateway_source_command_count"],
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    assert_eq!(
        value["route_count"],
        serde_json::json!(NATIVE_GATEWAY_SOURCE_COMMAND_COUNT)
    );
    assert_eq!(
        value["implemented_route_count"],
        serde_json::json!(NATIVE_GATEWAY_SOURCE_COMMAND_COUNT)
    );
    assert_eq!(value["missing_route_count"], 0);
    assert_eq!(value["route_count_source_command_accepted"], true);
    assert_eq!(value["source_route_wired"], true);
    assert_eq!(value["hepta_core_connected"], true);
    assert_eq!(value["hepta_core_full_fusion_complete"], true);
    assert_eq!(value["active_binary_package"], "hepta-cli");
    assert_eq!(value["remaining_direct_codex_dependency_count"], 0);
    assert_eq!(value["memory_capability_inventory_ready"], true);
    assert_eq!(value["memory_surface_count"], 14);
    assert_eq!(value["absorbed_or_represented_count"], 14);
    assert_eq!(value["baseline_live_mutation_enabled_count"], 0);
    assert_eq!(value["baseline_memory_store_mutation_enabled"], false);
    assert_eq!(value["operator_approved_lanes_ready"], true);
    assert_eq!(value["operator_approved_lane_count"], 3);
    assert_eq!(value["ready_operator_approved_lane_count"], 3);
    assert_eq!(value["explicit_command_required_for_execution"], true);
    assert_eq!(value["report_only_boundaries_intact"], true);
    assert_eq!(value["full_live_activation_enabled"], false);
    assert_eq!(value["full_live_activation_status"], "blocked_report_only");
    assert_eq!(value["full_live_activation_blocked"], true);
    assert_eq!(value["live_activation_blocker_count"], 13);
    assert_eq!(value["replay_allowed"], false);
    assert_eq!(value["replay_accepted"], false);
    assert_eq!(value["readiness_index_side_effects_all_false"], true);

    assert_eq!(value["memory_lane"]["operator_approved_lane_ready"], true);
    assert_eq!(
        value["memory_lane"]["memory_durable_mutation_lane_enabled"],
        true
    );
    assert_eq!(
        value["memory_lane"]["live_memory_write_allowed_by_lane"],
        true
    );
    assert_eq!(
        value["memory_lane"]["execution_requires_explicit_command"],
        true
    );
    assert_eq!(value["memory_lane"]["report_route_write_performed"], false);
    assert_eq!(
        value["memory_lane"]["report_route_exposes_execution_command"],
        false
    );
    assert_eq!(
        value["memory_lane"]["side_effect_memory_store_mutated"],
        false
    );
    assert_eq!(
        value["hepta_intelligence_lane"]["operator_approved_lane_ready"],
        true
    );
    assert_eq!(
        value["hepta_intelligence_lane"]["context_attachment_lane_enabled"],
        true
    );
    assert_eq!(
        value["hepta_intelligence_lane"]["bounded_prompt_preview_lane_enabled"],
        true
    );
    assert_eq!(
        value["hepta_intelligence_lane"]["context_attachment_requires_explicit_command"],
        true
    );
    assert_eq!(
        value["hepta_intelligence_lane"]["report_route_context_attached"],
        false
    );
    assert_eq!(
        value["hepta_intelligence_lane"]["report_route_context_injection_performed"],
        false
    );
    assert_eq!(value["kg_lane"]["operator_approved_lane_ready"], true);
    assert_eq!(value["kg_lane"]["kg_prompt_preview_lane_enabled"], true);
    assert_eq!(
        value["kg_lane"]["kg_external_adapter_read_lane_enabled"],
        true
    );
    assert_eq!(
        value["kg_lane"]["kg_external_adapter_requires_explicit_command"],
        true
    );
    assert_eq!(
        value["kg_lane"]["kg_external_adapter_credential_reference_required"],
        true
    );
    assert_eq!(
        value["kg_lane"]["kg_external_adapter_credential_read_allowed_by_lane"],
        false
    );
    assert_eq!(value["kg_lane"]["supported_kg_adapter_count"], 3);
    assert_eq!(value["kg_lane"]["kg_live_write_lane_enabled"], false);
    assert_eq!(
        value["kg_lane"]["report_route_kg_adapter_read_performed"],
        false
    );
    assert_eq!(
        value["kg_lane"]["report_route_credential_read_performed"],
        false
    );
    assert_eq!(
        value["kg_lane"]["report_route_kg_live_write_performed"],
        false
    );

    let truth_matrix = value["truth_matrix"]
        .as_array()
        .expect("truth matrix entries");
    assert_eq!(truth_matrix.len(), 6);
    assert!(truth_matrix.iter().any(|entry| {
        entry["surface"] == "memory"
            && entry["operator_approved_lane_ready"] == true
            && entry["explicit_command_required"] == true
            && entry["report_route_execution_performed"] == false
            && entry["full_live_unrestricted"] == false
    }));
    assert!(truth_matrix.iter().any(|entry| {
        entry["surface"] == "kg"
            && entry["operator_approved_lane_ready"] == true
            && entry["explicit_command_required"] == true
            && entry["report_route_execution_performed"] == false
            && entry["full_live_unrestricted"] == false
    }));

    let blocked = value["blocked_actions"]
        .as_array()
        .expect("blocked truth-index actions")
        .iter()
        .filter_map(|item| item.as_str())
        .collect::<Vec<_>>();
    assert!(blocked.contains(&"treat_lane_ready_as_full_live_activation"));
    assert!(blocked.contains(&"write_memory_from_truth_index_report_route"));
    assert!(blocked.contains(&"write_live_kg_from_truth_index_report_route"));
    assert!(blocked.contains(&"invoke_provider_or_model_from_truth_index_report_route"));
    assert!(blocked.contains(&"release_public_claim_from_truth_index_report_route"));
    assert_eq!(
        value["allowed_next_actions"][0]["action"],
        "continue_release_artifact_publication_denial_chain"
    );
    assert_eq!(
        value["allowed_next_actions"][0]["claims_public_release"],
        false
    );

    let side_effects = value["side_effects"]
        .as_object()
        .expect("truth index side effects");
    assert!(
        side_effects
            .values()
            .all(|effect| effect.as_bool() == Some(false)),
        "truth index side effects must all be false: {side_effects:?}"
    );
}

#[test]
fn hepta_provider_channel_dry_run_plan_endpoint_is_side_effect_free() {
    let body = route_contract_body(HEPTA_PROVIDER_CHANNEL_DRY_RUN_PLAN_ENDPOINT);

    let value: serde_json::Value =
        serde_json::from_str(&body).expect("provider channel dry run plan json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(
        value["source_command"],
        "/hepta-provider-channel-dry-run-plan --json"
    );
    assert_eq!(
        value["compatibility_mode"],
        "native_provider_channel_runtime_dry_run_plan"
    );
    assert_eq!(value["side_effect_free"], true);
    assert_eq!(
        value["current_hepta_codex_script_total"],
        CURRENT_HEPTA_CODEX_SCRIPT_TOTAL
    );
    assert_eq!(
        value["native_gateway_source_command_count"],
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    assert_eq!(value["missing_route_count"], 0);
    assert_eq!(value["plan_family_count"], 5);
    assert_eq!(value["covered_old_ops_file_count"], 43);
    assert_eq!(value["covered_provider_ops_file_count"], 15);
    assert_eq!(value["covered_search_ops_file_count"], 3);
    assert_eq!(value["covered_channel_ops_file_count"], 13);
    assert_eq!(value["covered_runtime_ops_file_count"], 12);
    assert_eq!(value["dry_run_plan_ready_count"], 5);
    assert_eq!(value["isolated_fixture_contract_count"], 5);
    assert_eq!(value["live_invocation_enabled_count"], 0);
    assert_eq!(value["credential_read_required_count"], 0);
    assert_eq!(value["operator_approval_required_count"], 5);
    assert_eq!(value["provider_prompt_execution_enabled"], false);
    assert_eq!(value["search_network_query_enabled"], false);
    assert_eq!(value["channel_delivery_enabled"], false);
    assert_eq!(value["runtime_store_mutation_enabled"], false);
    assert_eq!(value["isolated_fixture_materialized"], false);
    assert_eq!(value["dry_run_plan_ready"], true);
    assert_eq!(
        value["script_inventory_script"],
        "scripts/hepta-provider-channel-dry-run-plan.sh"
    );
    let families = value["dry_run_families"]
        .as_array()
        .expect("dry run families");
    assert_eq!(families.len(), 5);
    assert_eq!(families[0]["name"], "provider-prompt-plan");
    assert_eq!(families[2]["name"], "search-readability-plan");
    assert_eq!(families[3]["name"], "channel-delivery-plan");
    assert_eq!(families[4]["name"], "runtime-session-plan");
    assert_eq!(value["side_effects"]["provider_invoked"], false);
    assert_eq!(value["side_effects"]["model_invoked"], false);
    assert_eq!(value["side_effects"]["credential_read"], false);
    assert_eq!(value["side_effects"]["external_network_read"], false);
    assert_eq!(value["side_effects"]["search_query_performed"], false);
    assert_eq!(value["side_effects"]["channel_read_performed"], false);
    assert_eq!(value["side_effects"]["channel_send_performed"], false);
    assert_eq!(
        value["side_effects"]["telegram_owner_handoff_performed"],
        false
    );
    assert_eq!(value["side_effects"]["telegram_read_performed"], false);
    assert_eq!(value["side_effects"]["telegram_send_performed"], false);
    assert_eq!(value["side_effects"]["process_spawned"], false);
    assert_eq!(value["side_effects"]["filesystem_read"], false);
    assert_eq!(value["side_effects"]["filesystem_written"], false);
    assert_eq!(value["side_effects"]["task_registry_mutated"], false);
    assert_eq!(value["side_effects"]["session_store_mutated"], false);
    assert_eq!(value["side_effects"]["gateway_event_enqueued"], false);
    assert_eq!(
        value["side_effects"]["native_post_mutation_performed"],
        false
    );
    assert_eq!(value["side_effects"]["gateway_mutation_performed"], false);
    assert_eq!(value["side_effects"]["external_send_performed"], false);
}

#[test]
fn control_ui_legacy_routes_are_reachable_without_side_effects() {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    for (method, path) in [
        ("GET", "/api/operator-console"),
        ("GET", "/api/query-transcript/sample"),
        ("GET", "/api/task/sample-task"),
        ("GET", "/api/live-events/0"),
        ("GET", "/api/external-agent-benchmark"),
        ("POST", "/api/actions/gateway-dispatch"),
        ("POST", "/api/commands/gateway-status"),
        ("POST", "/api/chat"),
    ] {
        let (status, content_type, body) = route_native_gateway_request(method, path, &options);
        assert_eq!(status, "200 OK", "{method} {path}");
        assert_eq!(content_type, "application/json; charset=utf-8");
        let value: serde_json::Value = serde_json::from_str(&body).expect("compat route json");
        assert_eq!(value["runtime"], "hepta");
        assert_eq!(value["external_side_effects"], false);
        assert_eq!(value["gateway_mutation_performed"], false);
        assert_eq!(value["telegram_read_performed"], false);
        assert_eq!(value["model_invoked"], false);
        assert_eq!(value["message_sent"], false);
        assert_eq!(value["cursor_written"], false);
    }
}

#[test]
fn operator_snapshot_returns_native_aggregate_without_side_effects() {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) =
        route_native_gateway_request("GET", "/api/operator-snapshot", &options);
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");

    let value: serde_json::Value = serde_json::from_str(&body).expect("operator snapshot json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["native_route"], true);
    assert_eq!(value["compatibility_mode"], "native_operator_snapshot");
    assert_eq!(value["side_effect_free"], true);
    assert_eq!(value["telegram_read_performed"], false);
    assert_eq!(value["model_invoked"], false);
    assert_eq!(value["message_sent"], false);
    assert_eq!(value["cursor_written"], false);
    assert_eq!(value["raw_token_exposed"], false);
    assert_eq!(value["health"]["status"], "ready");
    assert!(value["gateway_replacement_readiness"].is_object());
    assert!(value["control_ui_route_parity"].is_object());
    assert!(value["telegram_plugin"].is_object());
    assert!(value["telegram_live_soak_status"].is_object());
    assert_ne!(
        value["compatibility_mode"],
        "native_control_ui_route_parity_shell"
    );
}

#[test]
fn native_sessions_inventory_scans_metadata_without_transcript_text() {
    let temp = tempfile::tempdir().expect("tempdir");
    let sessions = temp.path().join("sessions/2026/05/18");
    std::fs::create_dir_all(&sessions).expect("create sessions dir");
    std::fs::write(
        sessions.join("rollout-2026-05-18T10-31-22-019e38e5-4a20-7000-a111-222222222222.jsonl"),
        r#"{"item":{"type":"message","text":"do-not-expose-transcript"}}"#,
    )
    .expect("write rollout");
    std::fs::write(sessions.join("ignored.jsonl"), "{}").expect("write ignored");

    let report = native_sessions_report(
        vec![NativeSessionRootCandidate {
            root: temp.path().join("sessions"),
            kind: "active",
        }],
        "/sessions --json",
        "native_sessions_inventory",
    );
    let body = serde_json::to_string(&report).expect("serialize sessions report");

    assert_eq!(report.status, "ready");
    assert_eq!(report.session_file_count, 1);
    assert_eq!(report.recent_session_count, 1);
    assert!(!report.raw_transcript_exposed);
    assert!(!report.transcript_text_exposed);
    assert_eq!(
        report.recent_sessions[0].session_id,
        "019e38e5-4a20-7000-a111-222222222222"
    );
    assert_eq!(
        report.recent_sessions[0].started_at_filename.as_deref(),
        Some("2026-05-18T10-31-22")
    );
    assert!(!body.contains("do-not-expose-transcript"));
}

#[test]
fn sessions_routes_return_native_inventory_without_side_effects() {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    for (path, mode) in [
        ("/api/sessions", "native_sessions_inventory"),
        ("/api/session-activity", "native_session_activity"),
    ] {
        let (status, content_type, body) = route_native_gateway_request("GET", path, &options);
        assert_eq!(status, "200 OK", "{path}");
        assert_eq!(content_type, "application/json; charset=utf-8");
        let value: serde_json::Value = serde_json::from_str(&body).expect("sessions route json");
        assert_eq!(value["runtime"], "hepta");
        assert_eq!(value["native_route"], true);
        assert_eq!(value["compatibility_mode"], mode);
        assert_eq!(value["side_effect_free"], true);
        assert_eq!(value["external_side_effects"], false);
        assert_eq!(value["gateway_mutation_performed"], false);
        assert_eq!(value["telegram_read_performed"], false);
        assert_eq!(value["model_invoked"], false);
        assert_eq!(value["message_sent"], false);
        assert_eq!(value["cursor_written"], false);
        assert_eq!(value["raw_transcript_exposed"], false);
        assert_eq!(value["transcript_text_exposed"], false);
        assert_ne!(
            value["compatibility_mode"],
            "native_control_ui_route_parity_shell"
        );
    }
}

#[test]
fn transcript_preview_redacts_text_and_query() {
    let temp = tempfile::tempdir().expect("tempdir");
    let sessions = temp.path().join("sessions/2026/05/18");
    std::fs::create_dir_all(&sessions).expect("create sessions dir");
    std::fs::write(
            sessions.join(
                "rollout-2026-05-18T11-12-03-019e38f3-1111-7000-a111-333333333333.jsonl",
            ),
            concat!(
                r#"{"timestamp":"2026-05-18T03:12:03Z","type":"response_item","payload":{"type":"message","role":"user","content":[{"type":"input_text","text":"super-secret-query-marker"}]}}"#,
                "\n",
                r#"{"timestamp":"2026-05-18T03:12:04Z","type":"event_msg","payload":{"type":"token_count","info":null}}"#,
                "\n",
            ),
        )
        .expect("write rollout");

    let report = native_transcript_report(
        vec![NativeSessionRootCandidate {
            root: temp.path().join("sessions"),
            kind: "active",
        }],
        Some("super-secret-query-marker"),
        5,
    );
    let body = serde_json::to_string(&report).expect("serialize transcript report");

    assert_eq!(report.status, "ready");
    assert!(report.query_present);
    assert!(report.query_redacted);
    assert_eq!(report.query_length, Some("super-secret-query-marker".len()));
    assert_eq!(report.matched_session_count, 1);
    assert_eq!(report.matched_line_count, 1);
    assert!(!report.raw_transcript_exposed);
    assert!(!report.transcript_text_exposed);
    assert!(!report.query_text_exposed);
    assert_eq!(report.sessions[0].line_count, 2);
    assert!(report.sessions[0].redacted_events[0].redacted);
    assert!(report.sessions[0].redacted_events[0].has_text_fields);
    assert!(!body.contains("super-secret-query-marker"));
    assert!(!body.contains("input_text"));
}

#[test]
fn transcript_routes_return_native_redacted_preview_without_side_effects() {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    for (path, mode, query_present) in [
        (
            "/api/transcript",
            "native_transcript_redacted_preview",
            false,
        ),
        (
            "/api/query-transcript/sample-secret-query",
            "native_query_transcript_redacted",
            true,
        ),
    ] {
        let (status, content_type, body) = route_native_gateway_request("GET", path, &options);
        assert_eq!(status, "200 OK", "{path}");
        assert_eq!(content_type, "application/json; charset=utf-8");
        assert!(!body.contains("sample-secret-query"));
        let value: serde_json::Value = serde_json::from_str(&body).expect("transcript route json");
        assert_eq!(value["runtime"], "hepta");
        assert_eq!(value["native_route"], true);
        assert_eq!(value["compatibility_mode"], mode);
        assert_eq!(value["side_effect_free"], true);
        assert_eq!(value["query_present"], query_present);
        assert_eq!(value["raw_transcript_exposed"], false);
        assert_eq!(value["transcript_text_exposed"], false);
        assert_eq!(value["query_text_exposed"], false);
        assert_eq!(value["external_side_effects"], false);
        assert_eq!(value["gateway_mutation_performed"], false);
        assert_eq!(value["telegram_read_performed"], false);
        assert_eq!(value["model_invoked"], false);
        assert_eq!(value["message_sent"], false);
        assert_eq!(value["cursor_written"], false);
        assert_ne!(
            value["compatibility_mode"],
            "native_control_ui_route_parity_shell"
        );
    }
}

#[test]
fn task_artifact_report_redacts_task_id_and_transcript_text() {
    let temp = tempfile::tempdir().expect("tempdir");
    let sessions = temp.path().join("sessions/2026/05/18");
    std::fs::create_dir_all(&sessions).expect("create sessions dir");
    std::fs::write(
            sessions.join(
                "rollout-2026-05-18T11-40-00-019e38f8-2222-7000-a111-444444444444.jsonl",
            ),
            concat!(
                r#"{"timestamp":"2026-05-18T03:40:00Z","type":"event_msg","payload":{"type":"agent_message","message":"task-secret-123 produced confidential patch text"}}"#,
                "\n",
            ),
        )
        .expect("write rollout");

    let transcript = native_transcript_report(
        vec![NativeSessionRootCandidate {
            root: temp.path().join("sessions"),
            kind: "active",
        }],
        Some("task-secret-123"),
        20,
    );
    let response = NativeTaskArtifactResponse {
        product: "Hepta",
        runtime: "hepta",
        status: "ready",
        source_command: "/task <task_id> --json",
        native_route: true,
        compatibility_mode: "native_task_drilldown_redacted",
        side_effect_free: true,
        artifact_kind: "task_drilldown",
        task_id_redacted: true,
        task_id_length: "task-secret-123".len(),
        evidence_found: transcript.matched_line_count > 0,
        matched_session_count: transcript.matched_session_count,
        matched_line_count: transcript.matched_line_count,
        evidence_search: transcript,
        raw_task_id_exposed: false,
        raw_transcript_exposed: false,
        transcript_text_exposed: false,
        model_invoked: false,
        external_side_effects: false,
        gateway_mutation_performed: false,
        telegram_read_performed: false,
        message_sent: false,
        cursor_written: false,
        next_migration_slice: "test",
    };
    let body = serde_json::to_string(&response).expect("serialize task response");

    assert!(response.evidence_found);
    assert_eq!(response.matched_line_count, 1);
    assert!(response.task_id_redacted);
    assert!(!response.raw_task_id_exposed);
    assert!(!response.transcript_text_exposed);
    assert!(!body.contains("task-secret-123"));
    assert!(!body.contains("confidential patch text"));
}

#[test]
fn task_artifact_routes_return_native_redacted_search_without_side_effects() {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    for (path, mode, artifact_kind) in [
        (
            "/api/task/sample-secret-task",
            "native_task_drilldown_redacted",
            "task_drilldown",
        ),
        (
            "/api/task-patches/sample-secret-task",
            "native_task_patches_redacted",
            "task_patches",
        ),
        (
            "/api/task-evidence/sample-secret-task",
            "native_task_evidence_redacted",
            "task_evidence",
        ),
        (
            "/api/task-replay/sample-secret-task",
            "native_task_replay_redacted",
            "task_replay",
        ),
        (
            "/api/promotion-ledger/sample-secret-task",
            "native_promotion_ledger_redacted",
            "promotion_ledger",
        ),
        (
            "/api/handoff-bundle/sample-secret-task",
            "native_handoff_bundle_redacted",
            "handoff_bundle",
        ),
    ] {
        let (status, content_type, body) = route_native_gateway_request("GET", path, &options);
        assert_eq!(status, "200 OK", "{path}");
        assert_eq!(content_type, "application/json; charset=utf-8");
        assert!(!body.contains("sample-secret-task"));
        let value: serde_json::Value =
            serde_json::from_str(&body).expect("task artifact route json");
        assert_eq!(value["runtime"], "hepta");
        assert_eq!(value["native_route"], true);
        assert_eq!(value["compatibility_mode"], mode);
        assert_eq!(value["artifact_kind"], artifact_kind);
        assert_eq!(value["side_effect_free"], true);
        assert_eq!(value["task_id_redacted"], true);
        assert_eq!(value["raw_task_id_exposed"], false);
        assert_eq!(value["raw_transcript_exposed"], false);
        assert_eq!(value["transcript_text_exposed"], false);
        assert_eq!(value["external_side_effects"], false);
        assert_eq!(value["gateway_mutation_performed"], false);
        assert_eq!(value["telegram_read_performed"], false);
        assert_eq!(value["model_invoked"], false);
        assert_eq!(value["message_sent"], false);
        assert_eq!(value["cursor_written"], false);
        assert_eq!(value["evidence_search"]["query_text_exposed"], false);
        assert_ne!(
            value["compatibility_mode"],
            "native_control_ui_route_parity_shell"
        );
    }
}

#[test]
fn event_report_redacts_cursor_and_transcript_text() {
    let temp = tempfile::tempdir().expect("tempdir");
    let sessions = temp.path().join("sessions/2026/05/18");
    std::fs::create_dir_all(&sessions).expect("create sessions dir");
    std::fs::write(
            sessions.join(
                "rollout-2026-05-18T12-10-00-019e3900-3333-7000-a111-555555555555.jsonl",
            ),
            concat!(
                r#"{"timestamp":"2026-05-18T04:10:00Z","type":"event_msg","payload":{"type":"agent_message","role":"assistant","message":"confidential event text"}}"#,
                "\n",
                r#"{"timestamp":"2026-05-18T04:10:01Z","type":"response_item","payload":{"type":"token_count","count":7}}"#,
                "\n",
            ),
        )
        .expect("write rollout");

    let report = native_events_report(
        vec![NativeSessionRootCandidate {
            root: temp.path().join("sessions"),
            kind: "active",
        }],
        NativeEventSurface::LiveEvents,
        Some("secret-live-cursor"),
    );
    let body = serde_json::to_string(&report).expect("serialize events report");

    assert_eq!(report.status, "ready");
    assert!(report.native_route);
    assert_eq!(report.compatibility_mode, "native_live_events_redacted");
    assert!(report.cursor_present);
    assert!(report.cursor_redacted);
    assert_eq!(report.cursor_length, Some("secret-live-cursor".len()));
    assert!(!report.raw_cursor_exposed);
    assert!(!report.cursor_text_exposed);
    assert!(!report.raw_transcript_exposed);
    assert!(!report.transcript_text_exposed);
    assert_eq!(report.total_line_count, 2);
    assert_eq!(report.parsed_json_line_count, 2);
    assert_eq!(report.recent_event_count, 2);
    assert!(
        report
            .event_type_counts
            .iter()
            .any(|count| count.event_type == "event_msg:agent_message" && count.count == 1)
    );
    assert!(!body.contains("secret-live-cursor"));
    assert!(!body.contains("confidential event text"));
}

#[test]
fn event_and_activity_routes_return_native_redacted_views_without_side_effects() {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    for (path, mode, surface, cursor_present) in [
        ("/api/events", "native_events_redacted", "events", false),
        (
            "/api/live-events/sample-secret-cursor",
            "native_live_events_redacted",
            "live_events",
            true,
        ),
        (
            "/api/events-report",
            "native_events_report_redacted",
            "events_report",
            false,
        ),
        (
            "/api/activity",
            "native_activity_redacted",
            "activity",
            false,
        ),
    ] {
        let (status, content_type, body) = route_native_gateway_request("GET", path, &options);
        assert_eq!(status, "200 OK", "{path}");
        assert_eq!(content_type, "application/json; charset=utf-8");
        assert!(!body.contains("sample-secret-cursor"));
        let value: serde_json::Value = serde_json::from_str(&body).expect("events route json");
        assert_eq!(value["runtime"], "hepta");
        assert_eq!(value["native_route"], true);
        assert_eq!(value["compatibility_mode"], mode);
        assert_eq!(value["event_surface"], surface);
        assert_eq!(value["side_effect_free"], true);
        assert_eq!(value["cursor_present"], cursor_present);
        assert_eq!(value["raw_cursor_exposed"], false);
        assert_eq!(value["cursor_text_exposed"], false);
        assert_eq!(value["raw_transcript_exposed"], false);
        assert_eq!(value["transcript_text_exposed"], false);
        assert_eq!(value["external_side_effects"], false);
        assert_eq!(value["gateway_mutation_performed"], false);
        assert_eq!(value["telegram_read_performed"], false);
        assert_eq!(value["model_invoked"], false);
        assert_eq!(value["message_sent"], false);
        assert_eq!(value["cursor_written"], false);
        assert_ne!(
            value["compatibility_mode"],
            "native_control_ui_route_parity_shell"
        );
        if path == "/api/activity" {
            assert!(value["activity_sessions"].is_object());
        } else {
            assert!(value["activity_sessions"].is_null());
        }
    }
}

#[test]
fn runtime_audit_report_counts_error_like_events_without_payloads() {
    let temp = tempfile::tempdir().expect("tempdir");
    let sessions = temp.path().join("sessions/2026/05/18");
    std::fs::create_dir_all(&sessions).expect("create sessions dir");
    std::fs::write(
            sessions.join(
                "rollout-2026-05-18T13-40-00-019e3900-3333-7000-a111-666666666666.jsonl",
            ),
            concat!(
                r#"{"timestamp":"2026-05-18T05:40:00Z","type":"event_msg","payload":{"type":"runtime_error","message":"super-secret-error-payload"}}"#,
                "\n",
                r#"{"timestamp":"2026-05-18T05:40:01Z","type":"event_msg","payload":{"type":"agent_message","role":"assistant","message":"subagent-secret-text"}}"#,
                "\n",
            ),
        )
        .expect("write rollout");

    let report = native_runtime_audit_report(
        vec![NativeSessionRootCandidate {
            root: temp.path().join("sessions"),
            kind: "active",
        }],
        NativeRuntimeAuditSurface::GatewayRetryDeadLetter,
    );
    let body = serde_json::to_string(&report).expect("serialize runtime audit report");

    assert_eq!(report.status, "ready");
    assert!(report.native_route);
    assert_eq!(
        report.compatibility_mode,
        "native_gateway_retry_dead_letter_redacted"
    );
    assert_eq!(report.audit_surface, "gateway_retry_dead_letter");
    assert_eq!(report.retry_or_error_event_count, 1);
    assert_eq!(report.subagent_event_count, 1);
    assert!(!report.redaction.raw_error_payload_exposed);
    assert!(!report.redaction.raw_agent_payload_exposed);
    assert!(!report.redaction.transcript_text_exposed);
    assert!(!report.side_effects.gateway_mutation_performed);
    assert!(!report.side_effects.telegram_read_performed);
    assert!(!report.side_effects.model_invoked);
    assert!(!report.side_effects.message_sent);
    assert!(!report.side_effects.cursor_written);
    assert!(!body.contains("super-secret-error-payload"));
    assert!(!body.contains("subagent-secret-text"));
}

#[test]
fn runtime_audit_routes_return_native_redacted_views_without_side_effects() {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    for (path, mode, audit_surface, agent_limit, message_limit) in [
        (
            "/api/subagent-observatory",
            "native_subagent_observatory_redacted",
            "subagent_observatory",
            None,
            None,
        ),
        (
            "/api/gateway-ledger",
            "native_gateway_ledger_redacted",
            "gateway_ledger",
            None,
            None,
        ),
        (
            "/api/gateway-retry-dead-letter",
            "native_gateway_retry_dead_letter_redacted",
            "gateway_retry_dead_letter",
            None,
            None,
        ),
        (
            "/api/multi-agent-runtime",
            "native_multi_agent_runtime_redacted",
            "multi_agent_runtime",
            Some(4),
            Some(8),
        ),
    ] {
        let (status, content_type, body) = route_native_gateway_request("GET", path, &options);
        assert_eq!(status, "200 OK", "{path}");
        assert_eq!(content_type, "application/json; charset=utf-8");
        let value: serde_json::Value =
            serde_json::from_str(&body).expect("runtime audit route json");

        assert_eq!(value["runtime"], "hepta");
        assert_eq!(value["native_route"], true);
        assert_eq!(value["compatibility_mode"], mode);
        assert_eq!(value["audit_surface"], audit_surface);
        assert_eq!(value["side_effect_free"], true);
        assert_eq!(value["sessions"]["native_route"], true);
        assert_eq!(value["events"]["native_route"], true);
        assert_eq!(value["redaction"]["raw_transcript_exposed"], false);
        assert_eq!(value["redaction"]["transcript_text_exposed"], false);
        assert_eq!(value["redaction"]["raw_agent_payload_exposed"], false);
        assert_eq!(value["redaction"]["raw_error_payload_exposed"], false);
        assert_eq!(
            value["redaction"]["raw_gateway_ledger_payload_exposed"],
            false
        );
        assert_eq!(value["side_effects"]["external_side_effects"], false);
        assert_eq!(value["side_effects"]["gateway_mutation_performed"], false);
        assert_eq!(value["side_effects"]["telegram_read_performed"], false);
        assert_eq!(value["side_effects"]["model_invoked"], false);
        assert_eq!(value["side_effects"]["message_sent"], false);
        assert_eq!(value["side_effects"]["cursor_written"], false);
        assert_eq!(
            value["agent_limit"],
            agent_limit
                .map(serde_json::Value::from)
                .unwrap_or(serde_json::Value::Null)
        );
        assert_eq!(
            value["message_limit"],
            message_limit
                .map(serde_json::Value::from)
                .unwrap_or(serde_json::Value::Null)
        );
        assert_ne!(
            value["compatibility_mode"],
            "native_control_ui_route_parity_shell"
        );
    }
}

#[test]
fn approvals_policy_and_config_routes_are_native_redacted_views() {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    for (path, mode) in [
        ("/api/approvals", "native_approvals_redacted"),
        ("/api/policy", "native_policy_snapshot"),
        ("/api/config", "native_config_surface_redacted"),
        ("/api/optional-configs", "native_optional_configs_redacted"),
    ] {
        let (status, content_type, body) = route_native_gateway_request("GET", path, &options);
        assert_eq!(status, "200 OK", "{path}");
        assert_eq!(content_type, "application/json; charset=utf-8");
        let value: serde_json::Value =
            serde_json::from_str(&body).expect("redacted config route json");
        assert_eq!(value["runtime"], "hepta");
        assert_eq!(value["native_route"], true);
        assert_eq!(value["compatibility_mode"], mode);
        assert_eq!(value["side_effect_free"], true);
        assert_eq!(value["external_side_effects"], false);
        assert_eq!(value["gateway_mutation_performed"], false);
        assert_eq!(value["telegram_read_performed"], false);
        assert_eq!(value["model_invoked"], false);
        assert_eq!(value["message_sent"], false);
        assert_eq!(value["cursor_written"], false);
        assert_ne!(
            value["compatibility_mode"],
            "native_control_ui_route_parity_shell"
        );
    }
}

#[test]
fn approvals_report_keeps_mutating_routes_guarded_without_payloads() {
    let report = native_approvals_report();
    let body = serde_json::to_string(&report).expect("serialize approvals report");

    assert_eq!(report.status, "ready");
    assert!(report.native_route);
    assert_eq!(report.pending_approval_count, 0);
    assert_eq!(report.approval_route_count, report.guarded_route_count);
    assert!(!report.raw_command_payload_exposed);
    assert!(!report.raw_approval_payload_exposed);
    assert!(report.approval_routes.iter().any(|route| {
        route.pattern == "/api/approvals/exec/apply"
            && route.guarded
            && route.confirmation_required_for_real_mutation
    }));
    assert!(!body.contains("secret-approval-payload"));
}

#[test]
fn optional_configs_report_exposes_metadata_not_contents() {
    let report = native_optional_configs_report();
    let body = serde_json::to_string(&report).expect("serialize optional configs report");

    assert!(report.native_route);
    assert_eq!(
        report.compatibility_mode,
        "native_optional_configs_redacted"
    );
    assert!(!report.config_content_exposed);
    assert!(!report.raw_config_value_exposed);
    assert!(
        report
            .configs
            .iter()
            .any(|config| { config.label == "agents" && !config.content_exposed })
    );
    assert!(!body.contains("Be genuinely helpful"));
    assert!(!body.contains("What to call them"));
}

#[test]
fn control_ui_audit_report_keeps_routes_guarded_without_dispatch() {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let telegram_plugin = native_telegram::telegram_plugin_status(true, 1500);
    let report = native_control_ui_audit_report(
        NativeControlUiAuditSurface::UiContractAudit,
        &options,
        &telegram_plugin,
    );

    assert_eq!(report.status, "static_contract_ready");
    assert!(report.native_route);
    assert_eq!(report.compatibility_mode, "native_ui_contract_audit");
    assert_eq!(report.control_ui_product_status, "static_contract_complete");
    assert!(!report.control_ui_product_complete);
    assert_eq!(report.control_ui_live_operator_surface_percent, 0);
    assert_eq!(report.control_ui_evidence.overall_evidence_percent, 20);
    assert!(!report.control_ui_evidence.all_required_layers_verified);
    assert_eq!(report.route_count, CONTROL_UI_ROUTE_SPECS.len());
    assert_eq!(
        report.get_route_count + report.post_route_count,
        CONTROL_UI_ROUTE_SPECS.len()
    );
    assert_eq!(report.post_route_count, report.guarded_post_route_count);
    assert!(!report.action_dispatched);
    assert!(!report.external_agent_spawned);
    assert!(!report.external_agent_benchmark_executed);
    assert!(!report.redaction.raw_action_payload_exposed);
    assert!(!report.redaction.raw_agent_payload_exposed);
    assert!(!report.side_effects.gateway_mutation_performed);
    assert!(!report.side_effects.telegram_read_performed);
    assert!(!report.side_effects.model_invoked);
    assert!(!report.side_effects.message_sent);
    assert!(!report.side_effects.cursor_written);
}

#[test]
fn control_ui_root_serves_rust_rendered_shell_and_assets() {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };

    let (status, content_type, body) = route_native_gateway_request("GET", "/", &options);
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "text/html; charset=utf-8");
    assert_eq!(body, hepta_core::control_ui::CONTROL_UI_INDEX_HTML);
    assert!(body.contains("data-rust-frontend-renderer=\"hepta-core::control_ui\""));
    assert!(body.contains("data-control-ui-product-first=\"true\""));
    assert!(body.contains("<script defer src=\"./control-ui.js\"></script>"));
    assert!(!body.contains("<script>"));

    let (status, content_type, legacy_body) =
        route_native_gateway_request("GET", "/gateway-status", &options);
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "text/html; charset=utf-8");
    assert!(legacy_body.contains("Hepta Control UI"));
    assert!(legacy_body.contains("/api/hepta-merge-completion"));
    assert!(legacy_body.contains("Control UI evidence"));
    assert!(legacy_body.contains("static_contract_complete"));
    assert!(legacy_body.contains("static 100%"));
    assert!(legacy_body.contains("live 0%"));
    assert!(!legacy_body.contains("100 / 100 / 100 / 100"));

    let (status, content_type, css) = route_native_gateway_request("GET", "/styles.css", &options);
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "text/css; charset=utf-8");
    assert!(css.contains(".tg-conversation-rail"));
    assert!(css.contains(".command-palette"));
    assert!(css.contains("safe-area-inset-bottom"));

    let logo = route_native_gateway_binary_asset("GET", "/assets/hepta-agent-logo.png")
        .expect("logo asset route");
    assert_eq!(logo.content_type, "image/png");
    assert_eq!(logo.cache_control, "public, max-age=3600, must-revalidate");
    assert_eq!(
        format!("{:x}", Sha256::digest(logo.body)),
        hepta_core::control_ui::CONTROL_UI_HEPTA_AGENT_LOGO_PNG_SHA256
    );
    assert!(logo.body.starts_with(b"\x89PNG\r\n\x1a\n"));
    assert!(logo.body.len() > 1024);

    let javascript = route_native_gateway_binary_asset("GET", "/control-ui.js")
        .expect("Control UI JavaScript asset route");
    assert_eq!(javascript.content_type, "text/javascript; charset=utf-8");
    assert_eq!(
        javascript.cache_control,
        "public, max-age=3600, must-revalidate"
    );
    assert_eq!(
        format!("{:x}", Sha256::digest(javascript.body)),
        hepta_core::control_ui::CONTROL_UI_JS_SHA256
    );
    let javascript_text = std::str::from_utf8(javascript.body).expect("JavaScript UTF-8");
    assert!(javascript_text.contains("const READ_ONLY_ROUTES = Object.freeze({"));
    assert!(javascript_text.contains("/api/operator-snapshot"));
    assert!(javascript_text.contains("new AbortController()"));
    assert!(javascript_text.contains("textContent"));
    assert!(!javascript_text.contains("innerHTML"));
    assert!(route_manifest_entry("GET", "/control-ui.js").is_some());

    assert!(route_native_gateway_binary_asset("GET", "/assets/k.png").is_none());
    assert!(route_manifest_entry("GET", "/assets/k.png").is_none());
    assert!(route_native_gateway_binary_asset("POST", "/assets/hepta-agent-logo.png").is_none());
    assert!(route_native_gateway_binary_asset("POST", "/control-ui.js").is_none());
    assert!(route_native_gateway_binary_asset("POST", "/assets/k.png").is_none());
}

#[test]
fn control_ui_shell_routes_return_native_plans_without_side_effects() {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    for (path, mode, surface, report_status, dry_run_only, read_only, plan_target) in [
        (
            "/api/control-ui",
            "native_control_ui_shell_snapshot",
            "control_ui",
            "static_contract_ready",
            false,
            true,
            None,
        ),
        (
            "/api/ui-contract-audit",
            "native_ui_contract_audit",
            "ui_contract_audit",
            "static_contract_ready",
            false,
            true,
            None,
        ),
        (
            "/api/gateway-dispatch",
            "native_gateway_dispatch_dry_run",
            "gateway_dispatch",
            "ready",
            true,
            false,
            Some("gateway-dispatch"),
        ),
        (
            "/api/ui-action-plan/gateway-dispatch",
            "native_ui_action_plan_gateway_dispatch",
            "ui_action_plan_gateway_dispatch",
            "ready",
            true,
            false,
            Some("gateway-dispatch"),
        ),
        (
            "/api/external-agent-benchmark",
            "native_external_agent_benchmark_redacted",
            "external_agent_benchmark",
            "ready",
            true,
            false,
            Some("external-agent-benchmark"),
        ),
    ] {
        let (status, content_type, body) = route_native_gateway_request("GET", path, &options);
        assert_eq!(status, "200 OK", "{path}");
        assert_eq!(content_type, "application/json; charset=utf-8");
        let value: serde_json::Value =
            serde_json::from_str(&body).expect("control ui audit route json");

        assert_eq!(value["runtime"], "hepta");
        assert_eq!(value["status"], report_status);
        assert_eq!(value["native_route"], true);
        assert_eq!(value["compatibility_mode"], mode);
        assert_eq!(value["control_surface"], surface);
        assert_eq!(value["side_effect_free"], true);
        assert_eq!(value["dry_run_only"], dry_run_only);
        assert_eq!(value["read_only"], read_only);
        assert_eq!(
            value["control_ui_product_status"],
            "static_contract_complete"
        );
        assert_eq!(value["control_ui_product_complete"], false);
        assert_eq!(value["control_ui_live_operator_surface_percent"], 0);
        assert_eq!(value["control_ui_evidence"]["overall_evidence_percent"], 20);
        assert_eq!(
            value["control_ui_evidence"]["all_required_layers_verified"],
            false
        );
        assert_eq!(value["confirmation_required_for_real_mutation"], false);
        assert_eq!(value["action_dispatched"], false);
        assert_eq!(value["external_agent_spawned"], false);
        assert_eq!(value["external_agent_benchmark_executed"], false);
        assert_eq!(value["redaction"]["raw_transcript_exposed"], false);
        assert_eq!(value["redaction"]["transcript_text_exposed"], false);
        assert_eq!(value["redaction"]["raw_token_exposed"], false);
        assert_eq!(value["redaction"]["raw_action_payload_exposed"], false);
        assert_eq!(value["redaction"]["raw_agent_payload_exposed"], false);
        assert_eq!(value["side_effects"]["external_side_effects"], false);
        assert_eq!(value["side_effects"]["gateway_mutation_performed"], false);
        assert_eq!(value["side_effects"]["telegram_read_performed"], false);
        assert_eq!(value["side_effects"]["model_invoked"], false);
        assert_eq!(value["side_effects"]["message_sent"], false);
        assert_eq!(value["side_effects"]["cursor_written"], false);
        assert_eq!(
            value["plan_target"],
            plan_target
                .map(serde_json::Value::from)
                .unwrap_or(serde_json::Value::Null)
        );
        assert_ne!(
            value["compatibility_mode"],
            "native_control_ui_route_parity_shell"
        );
    }
}

#[test]
fn post_plan_report_redacts_route_parameters_and_never_reads_body() {
    let spec = &native_post_plan_route_specs()[0];
    let report = native_post_plan_report(spec, Some("secret-action-payload"), None);
    let body = serde_json::to_string(&report).expect("serialize post plan report");

    assert_eq!(report.status, "dry_run_ready");
    assert!(report.native_route);
    assert_eq!(report.compatibility_mode, "native_action_post_dry_run");
    assert!(report.parameter_present);
    assert!(report.parameter_redacted);
    assert_eq!(report.parameter_length, Some("secret-action-payload".len()));
    assert!(!report.request_body_read);
    assert!(report.body_schema_ready);
    assert!(report.confirmation_contract_ready);
    assert!(report.rollback_contract_ready);
    assert!(report.idempotency_evidence_ready);
    assert!(report.audit_event_contract_ready);
    assert!(report.execution_admission_ready);
    assert_eq!(report.body_schema.schema_id, "hepta.post.ui_action.v1");
    assert!(!report.body_schema.body_read_during_plan);
    assert!(!report.body_schema.raw_body_exposed);
    assert!(report.body_admission_ready);
    assert_eq!(report.body_admission.admission_status, "not_required");
    assert!(!report.body_admission.body_received);
    assert!(!report.body_admission.request_body_read);
    assert!(!report.body_admission.raw_body_exposed);
    assert!(!report.body_admission.raw_field_values_exposed);
    assert!(
        report
            .body_schema
            .optional_fields
            .contains(&"action_payload")
    );
    assert!(
        !report
            .confirmation_contract
            .current_plan_requires_confirmation
    );
    assert_eq!(
        report.rollback_contract.current_plan_rollback_strategy,
        "noop_no_state_written"
    );
    assert!(!report.rollback_contract.state_written_by_plan);
    assert!(!report.idempotency_evidence.required);
    assert!(!report.idempotency_evidence.key_present);
    assert!(!report.idempotency_evidence.current_plan_store_written);
    assert!(!report.idempotency_evidence.raw_key_exposed);
    assert!(!report.audit_event_contract.required);
    assert_eq!(
        report.audit_event_contract.schema_id,
        "hepta.post.execution_audit.v1"
    );
    assert_eq!(report.audit_event_contract.event_kind, "ui_action");
    assert!(report.audit_event_contract.ready_for_real_handler);
    assert!(!report.audit_event_contract.current_plan_emits_audit_event);
    assert!(
        !report
            .audit_event_contract
            .current_plan_persists_audit_event
    );
    assert_eq!(report.execution_admission.admission_status, "blocked");
    assert!(
        !report
            .execution_admission
            .current_plan_executes_real_handler
    );
    assert!(!report.execution_admission.real_handler_currently_enabled);
    assert!(!report.execution_admission.real_handler_implemented);
    assert!(!report.execution_admission.allowlisted_for_real_handler);
    assert!(!report.execution_admission.enablement_gate_enabled);
    assert!(report.execution_admission.requires_dry_run_first);
    assert_eq!(report.execution_admission.blocked_reason, "plan_only_route");
    assert!(!report.raw_request_body_exposed);
    assert!(!report.raw_parameter_exposed);
    assert!(!report.action_dispatched);
    assert!(!report.gateway_mutation_performed);
    assert!(!report.message_sent);
    assert!(!body.contains("secret-action-payload"));
}

#[test]
fn post_routes_return_native_plans_without_side_effects() {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    for (path, mode, plan_kind, confirm_required, parameter_present) in [
        (
            "/api/actions/secret-action",
            "native_action_post_dry_run",
            "ui_action",
            false,
            true,
        ),
        (
            "/api/commands/secret-command",
            "native_readonly_command_plan",
            "readonly_command",
            false,
            true,
        ),
        (
            "/api/approvals/exec/apply",
            "native_approvals_exec_apply_dry_run",
            "approval_apply",
            true,
            false,
        ),
        (
            "/api/tasks/plan",
            "native_task_plan_dry_run",
            "task_plan",
            false,
            false,
        ),
        (
            "/api/tasks/publish",
            "native_task_publish_confirm_required",
            "task_publish",
            true,
            false,
        ),
        (
            "/api/chat/register",
            "native_chat_register_dry_run",
            "chat_register",
            false,
            false,
        ),
        (
            "/api/chat/archive",
            "native_chat_archive_dry_run",
            "chat_archive",
            false,
            false,
        ),
        (
            "/api/chat/unarchive",
            "native_chat_unarchive_dry_run",
            "chat_unarchive",
            false,
            false,
        ),
        (
            "/api/chat/delete",
            "native_chat_delete_dry_run",
            "chat_delete",
            false,
            false,
        ),
        (
            "/api/chat/plan",
            "native_chat_plan_dry_run",
            "chat_plan",
            false,
            false,
        ),
        (
            "/api/chat",
            "native_chat_send_confirm_required",
            "chat_send",
            true,
            false,
        ),
    ] {
        let (status, content_type, body) = route_native_gateway_request("POST", path, &options);
        assert_eq!(status, "200 OK", "{path}");
        assert_eq!(content_type, "application/json; charset=utf-8");
        assert!(!body.contains("secret-action"));
        assert!(!body.contains("secret-command"));
        let value: serde_json::Value = serde_json::from_str(&body).expect("post plan json");

        assert_eq!(value["runtime"], "hepta");
        assert_eq!(value["method"], "POST");
        assert_eq!(value["native_route"], true);
        assert_eq!(value["compatibility_mode"], mode);
        assert_eq!(value["plan_kind"], plan_kind);
        assert_eq!(value["side_effect_free"], true);
        assert_eq!(
            value["confirmation_required_for_real_mutation"],
            confirm_required
        );
        assert_eq!(value["parameter_present"], parameter_present);
        assert_eq!(value["parameter_redacted"], parameter_present);
        assert_eq!(value["request_body_read"], false);
        assert_eq!(value["request_body_redacted"], true);
        assert_eq!(value["body_schema_ready"], true);
        assert_eq!(value["body_admission_ready"], true);
        assert_eq!(value["confirmation_contract_ready"], true);
        assert_eq!(value["rollback_contract_ready"], true);
        assert_eq!(value["idempotency_evidence_ready"], true);
        assert_eq!(value["audit_event_contract_ready"], true);
        assert_eq!(value["execution_admission_ready"], true);
        assert_eq!(value["body_schema"]["content_type"], "application/json");
        assert_eq!(value["body_schema"]["body_read_during_plan"], false);
        assert_eq!(value["body_schema"]["raw_body_exposed"], false);
        assert_eq!(value["body_schema"]["raw_field_values_exposed"], false);
        assert_eq!(value["body_admission"]["request_body_read"], false);
        assert_eq!(value["body_admission"]["request_body_redacted"], true);
        assert_eq!(value["body_admission"]["raw_body_exposed"], false);
        assert_eq!(value["body_admission"]["raw_field_values_exposed"], false);
        assert_eq!(
            value["confirmation_contract"]["current_plan_requires_confirmation"],
            false
        );
        assert_eq!(
            value["confirmation_contract"]["real_mutation_requires_confirmation"],
            confirm_required
        );
        assert_eq!(
            value["confirmation_contract"]["operator_approval_required"],
            confirm_required
        );
        assert_eq!(
            value["confirmation_contract"]["raw_confirmation_payload_exposed"],
            false
        );
        assert_eq!(value["rollback_contract"]["current_plan_noop"], true);
        assert_eq!(value["rollback_contract"]["state_written_by_plan"], false);
        assert_eq!(
            value["rollback_contract"]["real_handler_requires_rollback_contract"],
            true
        );
        assert_eq!(
            value["rollback_contract"]["destructive_without_rollback"],
            false
        );
        assert_eq!(value["idempotency_evidence"]["required"], confirm_required);
        assert_eq!(value["idempotency_evidence"]["key_present"], false);
        assert_eq!(value["idempotency_evidence"]["key_redacted"], false);
        assert_eq!(
            value["idempotency_evidence"]["current_plan_lookup_performed"],
            false
        );
        assert_eq!(
            value["idempotency_evidence"]["current_plan_store_written"],
            false
        );
        assert_eq!(value["idempotency_evidence"]["raw_key_exposed"], false);
        assert_eq!(value["audit_event_contract"]["required"], confirm_required);
        assert_eq!(
            value["audit_event_contract"]["schema_id"],
            "hepta.post.execution_audit.v1"
        );
        assert_eq!(value["audit_event_contract"]["event_kind"], plan_kind);
        assert_eq!(
            value["audit_event_contract"]["current_plan_emits_audit_event"],
            false
        );
        assert_eq!(
            value["audit_event_contract"]["current_plan_persists_audit_event"],
            false
        );
        assert_eq!(
            value["audit_event_contract"]["raw_idempotency_key_exposed"],
            false
        );
        assert_eq!(value["execution_admission"]["admission_status"], "blocked");
        assert_eq!(
            value["execution_admission"]["current_plan_executes_real_handler"],
            false
        );
        assert_eq!(
            value["execution_admission"]["real_handler_currently_enabled"],
            false
        );
        assert_eq!(
            value["execution_admission"]["real_handler_implemented"],
            native_post_plan_kind_has_real_handler(plan_kind)
        );
        assert_eq!(
            value["execution_admission"]["allowlisted_for_real_handler"],
            confirm_required
        );
        assert_eq!(
            value["execution_admission"]["enablement_gate_env"],
            "HEPTA_NATIVE_POST_REAL_HANDLERS"
        );
        assert_eq!(
            value["execution_admission"]["enablement_gate_enabled"],
            false
        );
        assert_eq!(
            value["execution_admission"]["operator_approval_env"],
            "HEPTA_NATIVE_POST_REAL_HANDLER_APPROVED"
        );
        assert_eq!(
            value["execution_admission"]["operator_approval_enabled"],
            false
        );
        assert_eq!(
            value["execution_admission"]["request_body_admission_status"],
            value["body_admission"]["admission_status"]
        );
        assert_eq!(
            value["execution_admission"]["requires_body_schema"],
            confirm_required
        );
        assert_eq!(
            value["execution_admission"]["requires_confirmation_contract"],
            confirm_required
        );
        assert_eq!(
            value["execution_admission"]["requires_rollback_contract"],
            confirm_required
        );
        assert_eq!(
            value["execution_admission"]["requires_idempotency_key"],
            confirm_required
        );
        assert_eq!(
            value["execution_admission"]["idempotency_evidence_ready"],
            !confirm_required
        );
        assert_eq!(
            value["execution_admission"]["requires_audit_event"],
            confirm_required
        );
        assert_eq!(
            value["execution_admission"]["audit_event_contract_ready"],
            !confirm_required
        );
        assert_eq!(
            value["execution_admission"]["requires_rate_limit"],
            confirm_required
        );
        assert_eq!(value["execution_admission"]["requires_dry_run_first"], true);
        assert_eq!(
            value["execution_admission"]["external_side_effects_possible"],
            confirm_required
        );
        let expected_blocked_reason = if confirm_required {
            "body_admission_not_ready"
        } else {
            "plan_only_route"
        };
        assert_eq!(
            value["execution_admission"]["blocked_reason"],
            expected_blocked_reason
        );
        assert_eq!(value["real_handler_harness_ready"], true);
        let expected_harness_status = if !confirm_required {
            "plan_only_route"
        } else if native_post_plan_kind_has_real_handler(plan_kind) {
            "blocked"
        } else {
            "not_implemented"
        };
        assert_eq!(
            value["real_handler_harness"]["status"],
            expected_harness_status
        );
        assert_eq!(
            value["real_handler_harness"]["handler_implemented"],
            native_post_plan_kind_has_real_handler(plan_kind)
        );
        assert_eq!(value["real_handler_harness"]["dual_gate_satisfied"], false);
        assert_eq!(
            value["real_handler_harness"]["store_write_attempted"],
            false
        );
        assert_eq!(
            value["real_handler_harness"]["store_write_succeeded"],
            false
        );
        assert_eq!(value["real_handler_harness"]["task_published"], false);
        assert_eq!(
            value["real_handler_harness"]["external_side_effects"],
            false
        );
        assert_eq!(
            value["real_handler_harness"]["raw_idempotency_key_exposed"],
            false
        );
        assert_eq!(value["action_dispatched"], false);
        assert_eq!(value["command_executed"], false);
        assert_eq!(value["approval_applied"], false);
        assert_eq!(value["task_published"], false);
        assert_eq!(value["chat_mutated"], false);
        assert_eq!(value["raw_request_body_exposed"], false);
        assert_eq!(value["raw_parameter_exposed"], false);
        assert_eq!(value["external_side_effects"], false);
        assert_eq!(value["gateway_mutation_performed"], false);
        assert_eq!(value["telegram_read_performed"], false);
        assert_eq!(value["model_invoked"], false);
        assert_eq!(value["message_sent"], false);
        assert_eq!(value["cursor_written"], false);
        assert_ne!(
            value["compatibility_mode"],
            "native_control_ui_route_parity_shell"
        );
    }
}

#[test]
fn post_route_body_admission_reads_and_redacts_confirm_payload() {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let body = r#"{"task":"secret task text","confirm":true,"dry_run":true,"idempotency_key":"secret-idem"}"#;

    let (status, content_type, response_body) =
        route_native_gateway_request_with_body("POST", "/api/tasks/publish", &options, Some(body));

    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");
    assert!(!response_body.contains("secret task text"));
    assert!(!response_body.contains("secret-idem"));
    let value: serde_json::Value =
        serde_json::from_str(&response_body).expect("post body admission json");

    assert_eq!(value["plan_kind"], "task_publish");
    assert_eq!(value["request_body_read"], true);
    assert_eq!(value["request_body_redacted"], true);
    assert_eq!(value["body_schema"]["body_read_during_plan"], true);
    assert_eq!(value["body_admission_ready"], true);
    assert_eq!(
        value["body_admission"]["admission_status"],
        "ready_for_real_handler"
    );
    assert_eq!(value["body_admission"]["body_received"], true);
    assert_eq!(value["body_admission"]["request_body_read"], true);
    assert_eq!(value["body_admission"]["json_parse_attempted"], true);
    assert_eq!(value["body_admission"]["json_parse_ok"], true);
    assert_eq!(value["body_admission"]["json_object_present"], true);
    assert_eq!(value["body_admission"]["required_fields_present"], true);
    assert_eq!(
        value["body_admission"]["missing_required_fields"],
        serde_json::json!([])
    );
    assert_eq!(value["body_admission"]["confirm_field_truthy"], true);
    assert_eq!(value["body_admission"]["dry_run_first_satisfied"], true);
    assert_eq!(value["body_admission"]["idempotency_key_present"], true);
    assert_eq!(
        value["body_admission"]["ready_for_real_handler_input"],
        true
    );
    assert_eq!(value["body_admission"]["raw_body_exposed"], false);
    assert_eq!(value["body_admission"]["raw_field_values_exposed"], false);
    assert_eq!(value["idempotency_evidence"]["required"], true);
    assert_eq!(value["idempotency_evidence"]["key_present"], true);
    assert_eq!(value["idempotency_evidence"]["key_redacted"], true);
    assert_eq!(value["idempotency_evidence"]["key_shape_valid"], true);
    assert_eq!(
        value["idempotency_evidence"]["duplicate_suppression_required"],
        true
    );
    assert_eq!(
        value["idempotency_evidence"]["current_plan_store_written"],
        false
    );
    assert_eq!(value["idempotency_evidence"]["raw_key_exposed"], false);
    assert_eq!(value["audit_event_contract"]["required"], true);
    assert_eq!(
        value["audit_event_contract"]["body_schema_id"],
        "hepta.post.task_publish.v1"
    );
    assert_eq!(
        value["audit_event_contract"]["body_admission_status_recorded"],
        true
    );
    assert_eq!(
        value["audit_event_contract"]["idempotency_evidence_recorded"],
        true
    );
    assert_eq!(
        value["audit_event_contract"]["ready_for_real_handler"],
        true
    );
    assert_eq!(
        value["audit_event_contract"]["current_plan_emits_audit_event"],
        false
    );
    assert_eq!(
        value["audit_event_contract"]["raw_idempotency_key_exposed"],
        false
    );
    assert_eq!(
        value["execution_admission"]["request_body_admission_status"],
        "ready_for_real_handler"
    );
    assert_eq!(
        value["execution_admission"]["request_body_ready_for_real_handler"],
        true
    );
    assert_eq!(
        value["execution_admission"]["idempotency_evidence_ready"],
        true
    );
    assert_eq!(
        value["execution_admission"]["audit_event_contract_ready"],
        true
    );
    assert_eq!(
        value["execution_admission"]["real_handler_implemented"],
        false
    );
    assert_eq!(
        value["execution_admission"]["current_plan_executes_real_handler"],
        false
    );
    assert_eq!(
        value["execution_admission"]["operator_approval_enabled"],
        false
    );
    assert_eq!(
        value["execution_admission"]["blocked_reason"],
        "real_handler_not_wired"
    );
    assert_eq!(value["real_handler_harness_ready"], true);
    assert_eq!(value["real_handler_harness"]["status"], "not_implemented");
    assert_eq!(value["real_handler_harness"]["handler_implemented"], false);
    assert_eq!(
        value["real_handler_harness"]["store_write_attempted"],
        false
    );
    assert_eq!(value["real_handler_harness"]["task_published"], false);
    assert_eq!(
        value["real_handler_harness"]["raw_idempotency_key_exposed"],
        false
    );
    assert_eq!(value["task_published"], false);
    assert_eq!(value["external_side_effects"], false);
    assert_eq!(value["gateway_mutation_performed"], false);
    assert_eq!(value["message_sent"], false);
}

#[test]
fn native_post_execution_readiness_endpoint_is_side_effect_free() {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) =
        route_native_gateway_request("GET", NATIVE_POST_EXECUTION_READINESS_ENDPOINT, &options);
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");
    let value: serde_json::Value =
        serde_json::from_str(&body).expect("post execution readiness json");

    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(value["native_route"], true);
    assert_eq!(
        value["compatibility_mode"],
        "native_post_execution_readiness"
    );
    assert_eq!(value["side_effect_free"], true);
    assert_eq!(
        value["post_route_count"],
        serde_json::json!(native_post_plan_route_specs().len())
    );
    assert_eq!(value["real_handler_candidate_count"], 3);
    assert_eq!(
        value["evidence_contract_route_count"],
        value["post_route_count"]
    );
    assert_eq!(value["all_evidence_contracts_ready"], true);
    assert_eq!(value["real_handler_implemented_count"], 0);
    assert_eq!(value["real_handler_ready_count"], 0);
    assert_eq!(value["all_real_handlers_blocked"], true);
    assert_eq!(value["raw_request_body_exposed"], false);
    assert_eq!(value["raw_idempotency_key_exposed"], false);
    assert_eq!(value["raw_audit_payload_exposed"], false);
    assert_eq!(value["action_dispatched"], false);
    assert_eq!(value["task_published"], false);
    assert_eq!(value["chat_mutated"], false);
    assert_eq!(value["message_sent"], false);
    assert_eq!(value["cursor_written"], false);
    assert!(
        value["routes"]
            .as_array()
            .expect("routes array")
            .iter()
            .any(|route| route["pattern"] == "/api/tasks/publish"
                && route["allowlisted_for_real_handler"] == true
                && route["real_handler_implemented"] == false
                && route["blocked_reason"] == "real_handler_not_wired")
    );
}

#[test]
fn native_post_execution_stores_endpoint_is_read_only() {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let before = native_post_execution_stores_report();
    let (status, content_type, body) =
        route_native_gateway_request("GET", NATIVE_POST_EXECUTION_STORES_ENDPOINT, &options);
    let after = native_post_execution_stores_report();
    assert_eq!(after, before);
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");
    let value: serde_json::Value = serde_json::from_str(&body).expect("post stores json");
    assert_eq!(
        value,
        serde_json::to_value(&before).expect("expected post stores json")
    );

    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(value["native_route"], true);
    assert_eq!(value["compatibility_mode"], "native_post_execution_stores");
    assert_eq!(value["side_effect_free"], true);
    assert_eq!(value["store_root_env"], NATIVE_POST_EXECUTION_STORE_DIR_ENV);
    assert_eq!(value["store_file_count"], 4);
    assert_eq!(
        value["max_store_bytes_env"],
        NATIVE_POST_STORE_MAX_BYTES_ENV
    );
    assert_eq!(
        value["max_store_lines_env"],
        NATIVE_POST_STORE_MAX_LINES_ENV
    );
    assert_eq!(value["total_bytes"], before.total_bytes);
    assert_eq!(value["store_jsonl_valid"], true);
    assert_eq!(value["store_capacity_ok"], true);
    assert_eq!(value["total_line_count"], before.total_line_count);
    assert_eq!(value["valid_json_line_count"], before.valid_json_line_count);
    assert_eq!(
        value["invalid_json_line_count"],
        before.invalid_json_line_count
    );
    assert_eq!(value["persistence_implementation_ready"], true);
    assert_eq!(value["idempotency_store_ready"], true);
    assert_eq!(value["audit_store_ready"], true);
    assert_eq!(value["rollback_store_ready"], true);
    assert_eq!(value["rate_limit_store_ready"], true);
    assert_eq!(value["status_probe_creates_directory"], false);
    assert_eq!(value["status_probe_writes_files"], false);
    assert_eq!(value["current_plan_executes_real_handler"], false);
    assert_eq!(value["raw_request_body_exposed"], false);
    assert_eq!(value["raw_idempotency_key_exposed"], false);
    assert_eq!(value["raw_audit_payload_exposed"], false);
    assert_eq!(value["task_published"], false);
    assert_eq!(value["chat_mutated"], false);
    assert_eq!(value["message_sent"], false);
    assert_eq!(value["cursor_written"], false);
    assert!(
        value["stores"]
            .as_array()
            .expect("stores array")
            .iter()
            .any(|store| store["filename"] == "idempotency.jsonl"
                && store["append_only"] == true
                && store["jsonl_readable"] == true
                && store["jsonl_valid"] == true
                && store["line_count"].as_u64().is_some()
                && store["raw_idempotency_key_exposed"] == false)
    );
}

#[test]
fn native_post_activation_plan_reports_dual_gate_and_rollback_without_side_effects() {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) =
        route_native_gateway_request("GET", NATIVE_POST_ACTIVATION_PLAN_ENDPOINT, &options);
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");
    let value: serde_json::Value = serde_json::from_str(&body).expect("activation plan json");

    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "attention");
    assert_eq!(value["native_route"], true);
    assert_eq!(value["compatibility_mode"], "native_post_activation_plan");
    assert_eq!(value["side_effect_free"], true);
    assert_eq!(value["activation_preflight_ready"], false);
    assert_eq!(value["activation_currently_enabled"], false);
    assert_eq!(
        value["activation_blocked_reason"],
        "real_handler_not_implemented"
    );
    assert_eq!(value["handler_candidate_count"], 3);
    assert_eq!(value["handler_implemented_count"], 0);
    assert_eq!(value["all_handlers_implemented"], false);
    assert_eq!(
        value["handler_scope_env"],
        NATIVE_POST_REAL_HANDLER_SCOPE_ENV
    );
    assert_eq!(value["handler_scope"], serde_json::Value::Null);
    assert_eq!(value["handler_scope_configured"], false);
    assert_eq!(value["single_handler_scope_ready"], false);
    assert_eq!(value["selected_handler_count"], 0);
    assert_eq!(
        value["selected_handler_kinds"]
            .as_array()
            .expect("selected handler kinds")
            .len(),
        0
    );
    assert_eq!(value["execution_evidence_ready"], true);
    assert_eq!(value["store_contracts_ready"], true);
    assert_eq!(value["store_jsonl_valid"], true);
    assert_eq!(value["store_capacity_ok"], true);
    assert_eq!(value["rollback_ready"], false);
    assert_eq!(value["rollback_anchor_required"], true);
    assert_eq!(value["rollback_store_file"], "rollback.jsonl");
    assert_eq!(value["rollback_schema_id"], "hepta.post.rollback_anchor.v1");
    assert_eq!(value["dry_run_only"], true);
    assert_eq!(value["real_mutation_performed"], false);
    assert_eq!(value["store_write_attempted"], false);
    assert_eq!(value["approval_applied"], false);
    assert_eq!(value["task_published"], false);
    assert_eq!(value["chat_mutated"], false);
    assert_eq!(value["external_side_effects"], false);
    assert_eq!(value["telegram_read_performed"], false);
    assert_eq!(value["model_invoked"], false);
    assert_eq!(value["message_sent"], false);
    assert_eq!(value["cursor_written"], false);
    assert_eq!(value["raw_idempotency_key_exposed"], false);
    assert_eq!(value["raw_audit_payload_exposed"], false);
    let gates = value["required_gates"].as_array().expect("gates array");
    assert_eq!(gates.len(), 3);
    assert!(gates.iter().any(|gate| {
        gate["env"] == NATIVE_POST_REAL_HANDLERS_ENV
            && gate["enabled"] == false
            && gate["required_for_activation"] == true
    }));
    assert!(gates.iter().any(|gate| {
        gate["env"] == NATIVE_POST_REAL_HANDLER_APPROVAL_ENV
            && gate["enabled"] == false
            && gate["required_for_activation"] == true
    }));
    assert!(gates.iter().any(|gate| {
        gate["env"] == NATIVE_POST_REAL_HANDLER_SCOPE_ENV
            && gate["enabled"] == false
            && gate["required_for_activation"] == true
    }));
    assert!(
        value["rollback_actions"]
            .as_array()
            .expect("rollback actions")
            .iter()
            .any(|action| action
                .as_str()
                .expect("rollback action")
                .contains("launchctl kickstart"))
    );
}

#[test]
fn native_post_rollout_evidence_route_reports_without_side_effects() {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) =
        route_native_gateway_request("GET", NATIVE_POST_ROLLOUT_EVIDENCE_ENDPOINT, &options);
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");
    let value: serde_json::Value = serde_json::from_str(&body).expect("rollout evidence json");

    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "ready");
    assert_eq!(value["native_route"], true);
    assert_eq!(value["compatibility_mode"], "native_post_rollout_evidence");
    assert_eq!(value["side_effect_free"], true);
    assert_eq!(value["endpoint"], NATIVE_POST_ROLLOUT_EVIDENCE_ENDPOINT);
    assert_eq!(value["store_root_env"], NATIVE_POST_EXECUTION_STORE_DIR_ENV);
    assert_eq!(
        value["activation_scope_env"],
        NATIVE_POST_REAL_HANDLER_SCOPE_ENV
    );
    assert_eq!(value["jsonl_readable"], true);
    assert_eq!(value["read_error"], serde_json::Value::Null);
    assert_eq!(value["real_mutation_performed"], false);
    assert_eq!(value["approval_applied"], false);
    assert_eq!(value["task_published"], false);
    assert_eq!(value["chat_mutated"], false);
    assert_eq!(value["telegram_read_performed"], false);
    assert_eq!(value["model_invoked"], false);
    assert_eq!(value["message_sent"], false);
    assert_eq!(value["cursor_written"], false);
    assert_eq!(value["raw_request_body_exposed"], false);
    assert_eq!(value["raw_idempotency_key_exposed"], false);
    assert_eq!(value["raw_audit_payload_exposed"], false);
}

#[test]
fn native_post_gray_release_evidence_route_reports_staged_without_side_effects() {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) =
        route_native_gateway_request("GET", NATIVE_POST_GRAY_RELEASE_EVIDENCE_ENDPOINT, &options);
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");
    let value: serde_json::Value = serde_json::from_str(&body).expect("gray release evidence json");

    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["status"], "attention");
    assert_eq!(value["native_route"], true);
    assert_eq!(
        value["compatibility_mode"],
        "native_post_gray_release_evidence"
    );
    assert_eq!(value["side_effect_free"], true);
    assert_eq!(
        value["activation_plan_endpoint"],
        NATIVE_POST_ACTIVATION_PLAN_ENDPOINT
    );
    assert_eq!(
        value["rollout_evidence_endpoint"],
        NATIVE_POST_ROLLOUT_EVIDENCE_ENDPOINT
    );
    assert_eq!(value["handler_scope"], serde_json::Value::Null);
    assert_eq!(value["selected_handler_count"], 0);
    assert_eq!(value["single_handler_scope_ready"], false);
    assert_eq!(value["activation_preflight_ready"], false);
    assert_eq!(value["activation_currently_enabled"], false);
    assert_eq!(value["gray_release_ready"], false);
    assert_eq!(
        value["gray_release_phase"],
        "activation_preflight_not_ready"
    );
    assert_eq!(
        value["selected_handler_evidence"]["dry_run_record_present"],
        false
    );
    assert_eq!(
        value["selected_handler_evidence"]["rollback_anchor_present"],
        false
    );
    assert_eq!(value["store_write_attempted"], false);
    assert_eq!(value["task_published"], false);
    assert_eq!(value["chat_mutated"], false);
    assert_eq!(value["telegram_read_performed"], false);
    assert_eq!(value["model_invoked"], false);
    assert_eq!(value["message_sent"], false);
    assert_eq!(value["cursor_written"], false);
    assert_eq!(value["raw_request_body_exposed"], false);
    assert_eq!(value["raw_idempotency_key_exposed"], false);
    assert_eq!(value["raw_audit_payload_exposed"], false);
}

#[test]
fn native_post_execution_store_status_counts_jsonl_health() {
    let temp = tempfile::tempdir().expect("tempdir");
    let store_file = temp.path().join("idempotency.jsonl");
    std::fs::write(
        &store_file,
        concat!(
            r#"{"schema_id":"hepta.post.execution_store_record.v1","plan_kind":"task_publish"}"#,
            "\n",
            "not-json",
            "\n",
        ),
    )
    .expect("write store");

    let report = hepta_gateway::native_post_execution_stores_report(temp.path(), 1024, 10);
    let status = report
        .stores
        .iter()
        .find(|store| store.filename == "idempotency.jsonl")
        .expect("idempotency store status");

    assert!(status.exists);
    assert!(status.bytes_within_limit);
    assert!(status.jsonl_readable);
    assert!(!status.jsonl_valid);
    assert_eq!(status.line_count, 2);
    assert!(status.line_count_within_limit);
    assert_eq!(status.valid_json_line_count, 1);
    assert_eq!(status.invalid_json_line_count, 1);
    assert!(!status.raw_idempotency_key_exposed);
}

#[test]
fn native_post_execution_store_status_blocks_oversized_jsonl() {
    let temp = tempfile::tempdir().expect("tempdir");
    let store_file = temp.path().join("idempotency.jsonl");
    std::fs::write(
        &store_file,
        concat!(
            r#"{"schema_id":"hepta.post.execution_store_record.v1","plan_kind":"task_publish"}"#,
            "\n",
            r#"{"schema_id":"hepta.post.execution_store_record.v1","plan_kind":"chat_send"}"#,
            "\n",
        ),
    )
    .expect("write store");

    let report = hepta_gateway::native_post_execution_stores_report(temp.path(), 8, 1);
    let status = report
        .stores
        .iter()
        .find(|store| store.filename == "idempotency.jsonl")
        .expect("idempotency store status");

    assert!(status.exists);
    assert!(status.jsonl_valid);
    assert!(!status.bytes_within_limit);
    assert_eq!(status.line_count, 2);
    assert!(!status.line_count_within_limit);
    assert_eq!(status.invalid_json_line_count, 0);
}

#[test]
fn native_post_real_handler_harness_records_redacted_dry_run_under_dual_gate() {
    let spec = native_post_plan_route_specs()
        .iter()
        .find(|spec| spec.plan_kind == "task_publish")
        .expect("task publish spec");
    let body = r#"{"task":"secret task text","confirm":true,"dry_run":true,"idempotency_key":"secret-idem"}"#;
    let body_schema = native_post_body_schema(spec.plan_kind, true);
    let body_admission = native_post_body_admission(spec, &body_schema, Some(body));
    let idempotency_evidence = native_post_idempotency_evidence(spec, &body_admission);
    let audit_event_contract = native_post_audit_event_contract(
        spec,
        &body_schema,
        &body_admission,
        &idempotency_evidence,
    );
    let execution_admission = native_post_execution_admission_with_gates(
        spec,
        &body_admission,
        &idempotency_evidence,
        &audit_event_contract,
        true,
        true,
    );
    let temp = tempfile::tempdir().expect("tempdir");
    let fingerprint = idempotency_evidence
        .key_fingerprint
        .as_deref()
        .expect("idempotency fingerprint");
    assert!(fingerprint.starts_with("sha256:"));
    assert!(!fingerprint.contains("secret-idem"));

    let harness = native_post_real_handler_harness(
        spec,
        &body_schema,
        &body_admission,
        &idempotency_evidence,
        &audit_event_contract,
        &execution_admission,
        temp.path(),
    );

    assert_eq!(execution_admission.admission_status, "blocked");
    assert!(!execution_admission.current_plan_executes_real_handler);
    assert!(execution_admission.operator_approval_enabled);
    assert_eq!(execution_admission.blocked_reason, "real_handler_not_wired");
    assert_eq!(harness.status, "not_implemented");
    assert_eq!(harness.handler_kind, "task_publish");
    assert!(harness.dry_run_only);
    assert!(!harness.handler_implemented);
    assert!(harness.dual_gate_satisfied);
    assert!(!harness.capacity_check_performed);
    assert!(!harness.store_capacity_ok);
    assert!(!harness.store_write_attempted);
    assert!(!harness.store_write_succeeded);
    assert!(!harness.task_published);
    assert!(!harness.external_side_effects);
    assert!(!harness.raw_request_body_exposed);
    assert!(!harness.raw_idempotency_key_exposed);
    assert!(harness.store_write_report.is_none());
    assert!(!temp.path().join("idempotency.jsonl").exists());
}

#[test]
fn native_post_execution_store_capacity_blocks_projected_append_over_limits() {
    let spec = native_post_plan_route_specs()
        .iter()
        .find(|spec| spec.plan_kind == "task_publish")
        .expect("task publish spec");
    let body = r#"{"task":"secret capacity task","confirm":true,"dry_run":true,"idempotency_key":"secret-capacity-idem"}"#;
    let body_schema = native_post_body_schema(spec.plan_kind, true);
    let body_admission = native_post_body_admission(spec, &body_schema, Some(body));
    let idempotency_evidence = native_post_idempotency_evidence(spec, &body_admission);
    let audit_event_contract = native_post_audit_event_contract(
        spec,
        &body_schema,
        &body_admission,
        &idempotency_evidence,
    );
    let record = native_post_execution_store_record(
        spec,
        &body_schema,
        &body_admission,
        &idempotency_evidence,
        &audit_event_contract,
        true,
    );
    let temp = tempfile::tempdir().expect("tempdir");
    persist_native_post_execution_store_record(temp.path(), &record).expect("seed stores");

    assert!(
        native_post_execution_store_capacity_allows_append_with_limits(
            temp.path(),
            &record,
            1024 * 1024,
            2,
        )
        .expect("capacity check")
    );
    assert!(
        !native_post_execution_store_capacity_allows_append_with_limits(
            temp.path(),
            &record,
            1024 * 1024,
            1,
        )
        .expect("line capacity check")
    );
    assert!(
        !native_post_execution_store_capacity_allows_append_with_limits(
            temp.path(),
            &record,
            8,
            10,
        )
        .expect("byte capacity check")
    );
}

#[test]
fn native_post_real_handler_harness_suppresses_duplicate_idempotency_key() {
    let spec = native_post_plan_route_specs()
        .iter()
        .find(|spec| spec.plan_kind == "task_publish")
        .expect("task publish spec");
    let body = r#"{"task":"secret duplicate task","confirm":true,"dry_run":true,"idempotency_key":"secret-duplicate-idem"}"#;
    let body_schema = native_post_body_schema(spec.plan_kind, true);
    let body_admission = native_post_body_admission(spec, &body_schema, Some(body));
    let idempotency_evidence = native_post_idempotency_evidence(spec, &body_admission);
    let audit_event_contract = native_post_audit_event_contract(
        spec,
        &body_schema,
        &body_admission,
        &idempotency_evidence,
    );
    let execution_admission = native_post_execution_admission_with_gates(
        spec,
        &body_admission,
        &idempotency_evidence,
        &audit_event_contract,
        true,
        true,
    );
    let temp = tempfile::tempdir().expect("tempdir");

    let first = native_post_real_handler_harness(
        spec,
        &body_schema,
        &body_admission,
        &idempotency_evidence,
        &audit_event_contract,
        &execution_admission,
        temp.path(),
    );
    let second = native_post_real_handler_harness(
        spec,
        &body_schema,
        &body_admission,
        &idempotency_evidence,
        &audit_event_contract,
        &execution_admission,
        temp.path(),
    );

    assert_eq!(first.status, "not_implemented");
    assert!(!first.store_write_succeeded);
    assert_eq!(second.status, "not_implemented");
    assert!(!second.duplicate_check_performed);
    assert!(!second.duplicate_found);
    assert!(!second.duplicate_suppressed);
    assert!(!second.store_write_attempted);
    assert!(!second.store_write_succeeded);
    assert!(second.store_write_report.is_none());
    assert!(!temp.path().join("idempotency.jsonl").exists());
}

#[test]
fn native_post_real_handler_harness_rate_limits_recent_bucket() {
    let spec = native_post_plan_route_specs()
        .iter()
        .find(|spec| spec.plan_kind == "task_publish")
        .expect("task publish spec");
    let first_body = r#"{"task":"secret first task","confirm":true,"dry_run":true,"idempotency_key":"secret-first-idem"}"#;
    let second_body = r#"{"task":"secret second task","confirm":true,"dry_run":true,"idempotency_key":"secret-second-idem"}"#;
    let body_schema = native_post_body_schema(spec.plan_kind, true);
    let first_body_admission = native_post_body_admission(spec, &body_schema, Some(first_body));
    let first_idempotency_evidence = native_post_idempotency_evidence(spec, &first_body_admission);
    let first_audit_event_contract = native_post_audit_event_contract(
        spec,
        &body_schema,
        &first_body_admission,
        &first_idempotency_evidence,
    );
    let first_execution_admission = native_post_execution_admission_with_gates(
        spec,
        &first_body_admission,
        &first_idempotency_evidence,
        &first_audit_event_contract,
        true,
        true,
    );
    let second_body_admission = native_post_body_admission(spec, &body_schema, Some(second_body));
    let second_idempotency_evidence =
        native_post_idempotency_evidence(spec, &second_body_admission);
    let second_audit_event_contract = native_post_audit_event_contract(
        spec,
        &body_schema,
        &second_body_admission,
        &second_idempotency_evidence,
    );
    let second_execution_admission = native_post_execution_admission_with_gates(
        spec,
        &second_body_admission,
        &second_idempotency_evidence,
        &second_audit_event_contract,
        true,
        true,
    );
    let temp = tempfile::tempdir().expect("tempdir");

    let first = native_post_real_handler_harness(
        spec,
        &body_schema,
        &first_body_admission,
        &first_idempotency_evidence,
        &first_audit_event_contract,
        &first_execution_admission,
        temp.path(),
    );
    let second = native_post_real_handler_harness(
        spec,
        &body_schema,
        &second_body_admission,
        &second_idempotency_evidence,
        &second_audit_event_contract,
        &second_execution_admission,
        temp.path(),
    );

    assert_eq!(first.status, "not_implemented");
    assert!(!first.rate_limit_check_performed);
    assert!(!first.rate_limited);
    assert_eq!(second.status, "not_implemented");
    assert!(!second.duplicate_check_performed);
    assert!(!second.duplicate_found);
    assert!(!second.rate_limit_check_performed);
    assert!(!second.rate_limited);
    assert!(!second.rate_limit_suppressed);
    assert!(!second.store_write_attempted);
    assert!(!second.store_write_succeeded);
    assert!(second.store_write_report.is_none());
    assert!(!temp.path().join("rate-limit.jsonl").exists());
}

#[test]
fn native_post_real_handler_harness_covers_confirm_required_candidates() {
    let candidates = [
        (
            "approval_apply",
            r#"{"approval_id":"secret approval id","confirm":true,"dry_run":true,"idempotency_key":"secret-approval-idem"}"#,
            "secret approval id",
            "secret-approval-idem",
        ),
        (
            "task_publish",
            r#"{"task":"secret task body","confirm":true,"dry_run":true,"idempotency_key":"secret-task-idem"}"#,
            "secret task body",
            "secret-task-idem",
        ),
        (
            "chat_send",
            r#"{"chat_id":"secret chat id","message":"secret chat message","confirm":true,"dry_run":true,"idempotency_key":"secret-chat-idem"}"#,
            "secret chat message",
            "secret-chat-idem",
        ),
    ];
    let temp = tempfile::tempdir().expect("tempdir");

    for (plan_kind, body, _raw_secret, _raw_idempotency_key) in candidates {
        let spec = native_post_plan_route_specs()
            .iter()
            .find(|spec| spec.plan_kind == plan_kind)
            .expect("candidate spec");
        let body_schema = native_post_body_schema(spec.plan_kind, true);
        let body_admission = native_post_body_admission(spec, &body_schema, Some(body));
        let idempotency_evidence = native_post_idempotency_evidence(spec, &body_admission);
        let audit_event_contract = native_post_audit_event_contract(
            spec,
            &body_schema,
            &body_admission,
            &idempotency_evidence,
        );
        let execution_admission = native_post_execution_admission_with_gates(
            spec,
            &body_admission,
            &idempotency_evidence,
            &audit_event_contract,
            true,
            true,
        );

        let harness = native_post_real_handler_harness(
            spec,
            &body_schema,
            &body_admission,
            &idempotency_evidence,
            &audit_event_contract,
            &execution_admission,
            temp.path(),
        );

        assert_eq!(body_admission.admission_status, "ready_for_real_handler");
        assert!(!native_post_plan_kind_has_real_handler(plan_kind));
        assert_eq!(execution_admission.admission_status, "blocked");
        assert!(!execution_admission.current_plan_executes_real_handler);
        assert_eq!(execution_admission.blocked_reason, "real_handler_not_wired");
        assert_eq!(harness.status, "not_implemented");
        assert_eq!(harness.handler_kind, plan_kind);
        assert!(!harness.handler_implemented);
        assert!(harness.dry_run_only);
        assert!(!harness.store_write_attempted);
        assert!(!harness.store_write_succeeded);
        assert!(!harness.task_published);
        assert!(!harness.message_sent);
        assert!(!harness.external_side_effects);
        assert!(!harness.raw_request_body_exposed);
        assert!(!harness.raw_idempotency_key_exposed);
        assert!(harness.store_write_report.is_none());
    }

    assert!(!temp.path().join("idempotency.jsonl").exists());
}

#[test]
fn native_post_real_handler_harness_requires_operator_approval_gate() {
    let spec = native_post_plan_route_specs()
        .iter()
        .find(|spec| spec.plan_kind == "task_publish")
        .expect("task publish spec");
    let body = r#"{"task":"secret task text","confirm":true,"dry_run":true,"idempotency_key":"secret-idem"}"#;
    let body_schema = native_post_body_schema(spec.plan_kind, true);
    let body_admission = native_post_body_admission(spec, &body_schema, Some(body));
    let idempotency_evidence = native_post_idempotency_evidence(spec, &body_admission);
    let audit_event_contract = native_post_audit_event_contract(
        spec,
        &body_schema,
        &body_admission,
        &idempotency_evidence,
    );
    let execution_admission = native_post_execution_admission_with_gates(
        spec,
        &body_admission,
        &idempotency_evidence,
        &audit_event_contract,
        true,
        false,
    );
    let temp = tempfile::tempdir().expect("tempdir");

    let harness = native_post_real_handler_harness(
        spec,
        &body_schema,
        &body_admission,
        &idempotency_evidence,
        &audit_event_contract,
        &execution_admission,
        temp.path(),
    );

    assert_eq!(execution_admission.admission_status, "blocked");
    assert!(!execution_admission.current_plan_executes_real_handler);
    assert!(execution_admission.enablement_gate_enabled);
    assert!(!execution_admission.operator_approval_enabled);
    assert_eq!(execution_admission.blocked_reason, "real_handler_not_wired");
    assert_eq!(harness.status, "not_implemented");
    assert!(!harness.dual_gate_satisfied);
    assert!(!harness.store_write_attempted);
    assert!(!harness.store_write_succeeded);
    assert!(harness.store_write_report.is_none());
    assert!(!temp.path().join("idempotency.jsonl").exists());
}

#[test]
fn native_post_real_handler_harness_requires_matching_handler_scope() {
    let spec = native_post_plan_route_specs()
        .iter()
        .find(|spec| spec.plan_kind == "task_publish")
        .expect("task publish spec");
    let body = r#"{"task":"secret scoped task","confirm":true,"dry_run":true,"idempotency_key":"secret-scoped-idem"}"#;
    let body_schema = native_post_body_schema(spec.plan_kind, true);
    let body_admission = native_post_body_admission(spec, &body_schema, Some(body));
    let idempotency_evidence = native_post_idempotency_evidence(spec, &body_admission);
    let audit_event_contract = native_post_audit_event_contract(
        spec,
        &body_schema,
        &body_admission,
        &idempotency_evidence,
    );
    let mismatched_admission = native_post_execution_admission_with_scope(
        spec,
        &body_admission,
        &idempotency_evidence,
        &audit_event_contract,
        true,
        true,
        Some("chat_send"),
    );
    let matched_admission = native_post_execution_admission_with_scope(
        spec,
        &body_admission,
        &idempotency_evidence,
        &audit_event_contract,
        true,
        true,
        Some("task_publish"),
    );
    let temp = tempfile::tempdir().expect("tempdir");

    let mismatched_harness = native_post_real_handler_harness(
        spec,
        &body_schema,
        &body_admission,
        &idempotency_evidence,
        &audit_event_contract,
        &mismatched_admission,
        temp.path(),
    );

    assert_eq!(mismatched_admission.admission_status, "blocked");
    assert!(!mismatched_admission.current_plan_executes_real_handler);
    assert!(mismatched_admission.handler_scope_configured);
    assert!(!mismatched_admission.handler_scope_required);
    assert!(!mismatched_admission.handler_scope_matches);
    assert_eq!(
        mismatched_admission.blocked_reason,
        "real_handler_not_wired"
    );
    assert_eq!(mismatched_harness.status, "not_implemented");
    assert!(mismatched_harness.handler_scope_configured);
    assert!(!mismatched_harness.handler_scope_matches);
    assert!(!mismatched_harness.store_write_attempted);
    assert!(!temp.path().join("idempotency.jsonl").exists());

    let matched_harness = native_post_real_handler_harness(
        spec,
        &body_schema,
        &body_admission,
        &idempotency_evidence,
        &audit_event_contract,
        &matched_admission,
        temp.path(),
    );

    assert_eq!(matched_admission.admission_status, "blocked");
    assert!(matched_admission.handler_scope_matches);
    assert_eq!(matched_admission.blocked_reason, "real_handler_not_wired");
    assert_eq!(matched_harness.status, "not_implemented");
    assert!(matched_harness.handler_scope_matches);
    assert!(!matched_harness.store_write_attempted);
    assert!(!matched_harness.store_write_succeeded);
    assert!(!temp.path().join("idempotency.jsonl").exists());
}

#[test]
fn native_post_execution_store_writer_persists_redacted_records() {
    let spec = native_post_plan_route_specs()
        .iter()
        .find(|spec| spec.plan_kind == "task_publish")
        .expect("task publish spec");
    let body = r#"{"task":"secret task text","confirm":true,"dry_run":true,"idempotency_key":"secret-idem"}"#;
    let body_schema = native_post_body_schema(spec.plan_kind, true);
    let body_admission = native_post_body_admission(spec, &body_schema, Some(body));
    let idempotency_evidence = native_post_idempotency_evidence(spec, &body_admission);
    let audit_event_contract = native_post_audit_event_contract(
        spec,
        &body_schema,
        &body_admission,
        &idempotency_evidence,
    );
    let record = native_post_execution_store_record(
        spec,
        &body_schema,
        &body_admission,
        &idempotency_evidence,
        &audit_event_contract,
        true,
    );
    let temp = tempfile::tempdir().expect("tempdir");

    let report =
        persist_native_post_execution_store_record(temp.path(), &record).expect("write stores");

    assert_eq!(report.status, "written");
    assert_eq!(report.written_file_count, 4);
    assert!(!report.raw_request_body_exposed);
    assert!(!report.raw_idempotency_key_exposed);
    for file in report.written_files {
        let content = std::fs::read_to_string(&file).expect("read store file");
        assert!(content.contains("hepta.post.execution_store_record.v1"));
        assert!(content.contains("task_publish"));
        assert!(content.contains("idempotency_key_redacted"));
        assert!(!content.contains("secret task text"));
        assert!(!content.contains("secret-idem"));
    }
}

#[test]
fn native_post_rollout_evidence_summarizes_redacted_rollback_anchor() {
    let spec = native_post_plan_route_specs()
        .iter()
        .find(|spec| spec.plan_kind == "task_publish")
        .expect("task publish spec");
    let body = r#"{"task":"secret rollout task","confirm":true,"dry_run":true,"idempotency_key":"secret-rollout-idem"}"#;
    let body_schema = native_post_body_schema(spec.plan_kind, true);
    let body_admission = native_post_body_admission(spec, &body_schema, Some(body));
    let idempotency_evidence = native_post_idempotency_evidence(spec, &body_admission);
    let audit_event_contract = native_post_audit_event_contract(
        spec,
        &body_schema,
        &body_admission,
        &idempotency_evidence,
    );
    let record = native_post_execution_store_record(
        spec,
        &body_schema,
        &body_admission,
        &idempotency_evidence,
        &audit_event_contract,
        true,
    );
    let temp = tempfile::tempdir().expect("tempdir");

    let empty = hepta_gateway::native_post_rollout_evidence_report(
        temp.path(),
        DEFAULT_NATIVE_POST_STORE_MAX_BYTES,
        DEFAULT_NATIVE_POST_STORE_MAX_LINES,
        None,
    );
    assert_eq!(empty.status, "ready");
    assert_eq!(empty.record_count, 0);
    assert!(!empty.rollback_anchor_present);
    assert!(!empty.dry_run_record_present);
    assert!(empty.latest_record.is_none());

    persist_native_post_execution_store_record(temp.path(), &record).expect("write stores");
    let report = hepta_gateway::native_post_rollout_evidence_report(
        temp.path(),
        DEFAULT_NATIVE_POST_STORE_MAX_BYTES,
        DEFAULT_NATIVE_POST_STORE_MAX_LINES,
        None,
    );

    assert_eq!(report.status, "ready");
    assert!(report.rollout_evidence_ready);
    assert_eq!(report.record_count, 1);
    assert_eq!(report.dry_run_record_count, 1);
    assert_eq!(report.rollback_anchor_count, 1);
    assert!(report.rollback_anchor_present);
    assert!(report.dry_run_record_present);
    assert_eq!(report.invalid_json_line_count, 0);
    assert_eq!(report.plan_kind_counts.len(), 1);
    assert_eq!(report.plan_kind_counts[0].plan_kind, "task_publish");
    assert_eq!(report.plan_kind_counts[0].count, 1);
    assert!(!report.raw_request_body_exposed);
    assert!(!report.raw_idempotency_key_exposed);
    assert!(!report.task_published);
    assert!(!report.external_side_effects);
    let latest = report.latest_record.expect("latest record");
    assert_eq!(latest.plan_kind.as_deref(), Some("task_publish"));
    assert!(latest.current_plan_executes_real_handler);
    assert!(latest.idempotency_key_redacted);
    assert!(latest.idempotency_key_fingerprint_present);
    assert!(!latest.raw_request_body_exposed);
    assert!(!latest.raw_idempotency_key_exposed);
    let rollback_content =
        std::fs::read_to_string(temp.path().join("rollback.jsonl")).expect("rollback store");
    assert!(!rollback_content.contains("secret rollout task"));
    assert!(!rollback_content.contains("secret-rollout-idem"));
}

#[test]
fn native_post_gray_release_evidence_requires_scoped_rollback_anchor() {
    let spec = native_post_plan_route_specs()
        .iter()
        .find(|spec| spec.plan_kind == "task_publish")
        .expect("task publish spec");
    let body = r#"{"task":"secret gray task","confirm":true,"dry_run":true,"idempotency_key":"secret-gray-idem"}"#;
    let body_schema = native_post_body_schema(spec.plan_kind, true);
    let body_admission = native_post_body_admission(spec, &body_schema, Some(body));
    let idempotency_evidence = native_post_idempotency_evidence(spec, &body_admission);
    let audit_event_contract = native_post_audit_event_contract(
        spec,
        &body_schema,
        &body_admission,
        &idempotency_evidence,
    );
    let record = native_post_execution_store_record(
        spec,
        &body_schema,
        &body_admission,
        &idempotency_evidence,
        &audit_event_contract,
        true,
    );
    let temp = tempfile::tempdir().expect("tempdir");
    let before = hepta_gateway::native_post_gray_release_evidence_report(
        temp.path(),
        DEFAULT_NATIVE_POST_STORE_MAX_BYTES,
        DEFAULT_NATIVE_POST_STORE_MAX_LINES,
        Some("task_publish"),
        true,
        true,
    );
    assert_eq!(before.status, "attention");
    assert_eq!(before.gray_release_phase, "activation_preflight_not_ready");
    assert!(!before.gray_release_ready);
    assert!(!before.selected_handler_evidence_ready);

    persist_native_post_execution_store_record(temp.path(), &record).expect("write stores");
    let report = hepta_gateway::native_post_gray_release_evidence_report(
        temp.path(),
        DEFAULT_NATIVE_POST_STORE_MAX_BYTES,
        DEFAULT_NATIVE_POST_STORE_MAX_LINES,
        Some("task_publish"),
        true,
        true,
    );

    assert_eq!(report.status, "attention");
    assert_eq!(report.gray_release_phase, "activation_preflight_not_ready");
    assert!(!report.activation_currently_enabled);
    assert!(!report.single_handler_scope_ready);
    assert!(report.selected_handler_kind.is_none());
    assert!(!report.gray_release_evidence_ready);
    assert!(!report.selected_handler_evidence_ready);
    assert!(!report.gray_release_ready);
    assert_eq!(report.selected_handler_evidence.record_count, 0);
    assert_eq!(report.selected_handler_evidence.dry_run_record_count, 0);
    assert_eq!(report.selected_handler_evidence.rollback_anchor_count, 0);
    assert!(report.selected_handler_evidence.latest_record.is_none());
    assert!(!report.raw_request_body_exposed);
    assert!(!report.raw_idempotency_key_exposed);
    let rollback_content =
        std::fs::read_to_string(temp.path().join("rollback.jsonl")).expect("rollback store");
    assert!(!rollback_content.contains("secret gray task"));
    assert!(!rollback_content.contains("secret-gray-idem"));
}

#[test]
fn operator_console_returns_native_status_without_side_effects() {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) =
        route_native_gateway_request("GET", "/api/operator-console", &options);
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");
    let value: serde_json::Value = serde_json::from_str(&body).expect("operator console json");

    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["native_route"], true);
    assert_eq!(value["compatibility_mode"], "native_operator_console");
    assert_eq!(value["side_effect_free"], true);
    assert_eq!(value["sessions"]["native_route"], true);
    assert_eq!(
        value["sessions"]["compatibility_mode"],
        "native_sessions_inventory"
    );
    assert_eq!(value["external_side_effects"], false);
    assert_eq!(value["gateway_mutation_performed"], false);
    assert_eq!(value["telegram_read_performed"], false);
    assert_eq!(value["model_invoked"], false);
    assert_eq!(value["message_sent"], false);
    assert_eq!(value["cursor_written"], false);
    assert_eq!(value["raw_transcript_exposed"], false);
    assert_ne!(
        value["compatibility_mode"],
        "native_control_ui_route_parity_shell"
    );
}

#[test]
fn operator_security_returns_native_guard_matrix_without_side_effects() {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: true,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) =
        route_native_gateway_request("GET", "/api/operator-security", &options);
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");
    let value: serde_json::Value = serde_json::from_str(&body).expect("operator security json");

    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["native_route"], true);
    assert_eq!(value["compatibility_mode"], "native_operator_security");
    assert_eq!(value["side_effect_free"], true);
    let security_mode = value["security_mode"]
        .as_str()
        .expect("operator security mode");
    assert!(
        [
            "active_replacement_ready",
            "legacy_owner_coexistence_ready",
            "attention_required"
        ]
        .contains(&security_mode)
    );
    assert!(value["legacy_owner_coexistence_ready"].is_boolean());
    assert!(value["attention_reason"].is_string());
    assert_eq!(value["loopback_bind_required"], true);
    assert_eq!(value["loopback_bound"], true);
    assert_eq!(value["side_effects"]["external_side_effects"], false);
    assert_eq!(value["side_effects"]["gateway_mutation_performed"], false);
    assert_eq!(value["side_effects"]["telegram_read_performed"], false);
    assert_eq!(value["side_effects"]["model_invoked"], false);
    assert_eq!(value["side_effects"]["message_sent"], false);
    assert_eq!(value["side_effects"]["cursor_written"], false);
    assert_eq!(value["redaction"]["raw_transcript_exposed"], false);
    assert_eq!(value["redaction"]["raw_token_exposed"], false);
    assert_eq!(value["redaction"]["raw_idempotency_key_exposed"], false);
    assert_eq!(value["redaction"]["raw_audit_payload_exposed"], false);
    assert_eq!(
        value["post_execution_readiness_endpoint"],
        NATIVE_POST_EXECUTION_READINESS_ENDPOINT
    );
    assert_eq!(
        value["post_execution_stores_endpoint"],
        NATIVE_POST_EXECUTION_STORES_ENDPOINT
    );
    assert_eq!(
        value["post_activation_plan_endpoint"],
        NATIVE_POST_ACTIVATION_PLAN_ENDPOINT
    );
    assert_eq!(
        value["post_gray_release_evidence_endpoint"],
        NATIVE_POST_GRAY_RELEASE_EVIDENCE_ENDPOINT
    );
    assert_eq!(value["post_execution_readiness"]["status"], "ready");
    assert_eq!(
        value["post_execution_readiness"]["all_real_handlers_blocked"],
        true
    );
    assert_eq!(value["post_execution_stores_ready"], true);
    assert_eq!(value["post_execution_stores"]["status"], "ready");
    assert_eq!(value["post_activation_plan_ready"], true);
    assert_eq!(value["post_activation_plan"]["status"], "attention");
    assert_eq!(
        value["post_activation_plan"]["activation_preflight_ready"],
        false
    );
    assert_eq!(
        value["post_activation_plan"]["activation_currently_enabled"],
        false
    );
    assert_eq!(
        value["post_activation_plan"]["activation_blocked_reason"],
        "real_handler_not_implemented"
    );
    assert_eq!(value["post_activation_plan"]["rollback_ready"], false);
    assert_eq!(value["post_gray_release_evidence_ready"], true);
    assert_eq!(value["post_gray_release_evidence"]["status"], "attention");
    assert_eq!(
        value["post_gray_release_evidence"]["gray_release_ready"],
        false
    );
    assert_eq!(
        value["post_gray_release_evidence"]["store_write_attempted"],
        false
    );
    assert_eq!(
        value["post_execution_stores"]["status_probe_writes_files"],
        false
    );
    assert_eq!(
        value["post_execution_stores"]["raw_idempotency_key_exposed"],
        false
    );
    assert!(value["telegram_production_readiness_status"].is_object());
    assert_eq!(
        value["telegram_production_readiness_status"]["side_effect_free"],
        true
    );
    assert_eq!(
        value["telegram_production_readiness_status"]["raw_token_exposed"],
        false
    );
    assert_eq!(
        value["telegram_owner_handoff_endpoint"],
        TELEGRAM_OWNER_HANDOFF_ENDPOINT
    );
    assert_eq!(
        value["telegram_owner_handoff_status"]["side_effect_free"],
        true
    );
    assert_eq!(
        value["telegram_owner_handoff_status"]["raw_token_exposed"],
        false
    );
    assert_eq!(value["post_route_count"], value["guarded_post_route_count"]);
    assert!(
        value["dry_run_post_route_count"]
            .as_u64()
            .expect("dry-run count")
            <= value["post_route_count"].as_u64().expect("post count")
    );
    assert_ne!(
        value["compatibility_mode"],
        "native_control_ui_route_parity_shell"
    );
}

#[test]
fn control_ui_route_parity_endpoint_returns_ready_report() {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: false,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) =
        route_native_gateway_request("GET", CONTROL_UI_ROUTE_PARITY_ENDPOINT, &options);
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");
    let value: serde_json::Value = serde_json::from_str(&body).expect("parity json");
    assert_eq!(value["runtime"], "hepta");
    assert_eq!(value["ready"], true);
    assert_eq!(value["missing_route_count"], 0);
    assert_eq!(
        value["evidence_scope"],
        "typed route registration, compatibility-handler serialization, production ingress availability, and real-socket test coverage"
    );
    assert_eq!(value["live_product_complete"], false);
    assert!(
        value["legacy_source"]
            .as_str()
            .expect("legacy source")
            .contains("quarantined legacy GET effects")
    );
}

#[test]
fn route_health_returns_ready_json() {
    let options = NativeGatewayOptions {
        bind_addr: "127.0.0.1:7373".to_string(),
        with_telegram_plugin: false,
        telegram_plugin_poll_ms: 1500,
    };
    let (status, content_type, body) = route_native_gateway_request("GET", "/health", &options);
    assert_eq!(status, "200 OK");
    assert_eq!(content_type, "application/json; charset=utf-8");
    assert!(body.contains(r#""status":"ready""#));
}

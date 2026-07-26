use crate::runtime_composition::RuntimeRequestDisposition;

#[test]
fn plan_only_preflight_overrides_native_post_env_double_gate_inputs() {
    let receipt = RuntimeRequestPreflightReceipt {
        request_binding_hash: "bound-plan-request".into(),
        disposition: RuntimeRequestDisposition::PlanOnlyQuarantine,
        ingress_kind: crate::runtime_ingress::RuntimeIngressKind::MutationPlan,
        mutation_authorized: false,
        durable_intent_recorded: false,
        provider_effect_ack_recorded: false,
        terminal_receipt_recorded: false,
    };
    let gates = receipt.native_post_gate_inputs(true, true);
    let mut post_effect = receipt;
    post_effect.mutation_authorized = true;
    post_effect.durable_intent_recorded = true;
    post_effect.provider_effect_ack_recorded = true;
    let acked_gates = post_effect.native_post_gate_inputs(true, true);
    assert!(!acked_gates.real_handler_enabled);
    post_effect.provider_effect_ack_recorded = false;
    post_effect.terminal_receipt_recorded = true;
    let terminal_gates = post_effect.native_post_gate_inputs(true, true);
    assert!(!terminal_gates.real_handler_enabled);
    let store_parent = tempfile::tempdir().expect("store parent");
    let store_root = store_parent.path().join("native-post");
    let spec = native_post_plan_route_specs()
        .iter()
        .find(|spec| spec.plan_kind == "task_publish")
        .expect("task publish spec");
    let report = hepta_gateway::native_post_plan_report(
        spec,
        None,
        Some(r#"{"task":"secret","confirm":true,"dry_run":true,"idempotency_key":"secret-idem"}"#),
        gates.real_handler_enabled,
        gates.operator_approval_enabled,
        Some(spec.plan_kind),
        &store_root,
        NativePostExecutionStoreLimits {
            max_store_bytes: DEFAULT_NATIVE_POST_STORE_MAX_BYTES,
            max_store_lines: DEFAULT_NATIVE_POST_STORE_MAX_LINES,
            rate_limit_window_ms: DEFAULT_NATIVE_POST_RATE_LIMIT_WINDOW_MS,
        },
    );
    assert!(
        !report
            .execution_admission
            .current_plan_executes_real_handler
    );
    assert!(!report.real_handler_harness.store_write_attempted);
    assert!(!store_root.exists());
}

#[test]
fn telegram_receive_once_denies_before_config_token_cursor_or_network() {
    let runtime_root = tempfile::tempdir().expect("runtime root");
    let runtime = Arc::new(
        NativeGatewayRuntime::bootstrap_with_anchor_for_test(runtime_root.path())
            .expect("keyed runtime"),
    );
    let pool = NativeGatewayConnectionPool::new(test_gateway_options(true), runtime, 1, 1)
        .expect("worker pool");
    let listener = TcpListener::bind("127.0.0.1:0").expect("listener");
    let (response, value) = preference_http_round_trip(
        &pool,
        &listener,
        "/api/telegram-receive-once",
        &serde_json::json!({}),
    );
    assert!(response.starts_with("HTTP/1.1 503 Service Unavailable"));
    assert_eq!(
        value["error"],
        "telegram_runtime_admission.exact_read_authority_unavailable"
    );
    for field in [
        "config_observed",
        "token_observed",
        "cursor_observed",
        "external_network_read",
    ] {
        assert_eq!(value[field], false);
    }
}

#[test]
fn existing_action_surface_runs_read_only_runtime_kernel_canary_with_durable_receipt() {
    let runtime_root = tempfile::tempdir().expect("runtime root");
    let runtime = Arc::new(
        NativeGatewayRuntime::bootstrap_with_anchor_for_test(runtime_root.path())
            .expect("keyed runtime"),
    );
    let pool =
        NativeGatewayConnectionPool::new(test_gateway_options(false), Arc::clone(&runtime), 1, 3)
            .expect("worker pool");
    let listener = TcpListener::bind("127.0.0.1:0").expect("listener");
    let denied = preference_http_round_trip(
        &pool,
        &listener,
        crate::runtime_composition::RUNTIME_KERNEL_CANARY_ACTION_ENDPOINT,
        &serde_json::json!({"dry_run": false}),
    );
    assert!(denied.0.starts_with("HTTP/1.1 400 Bad Request"));
    assert_eq!(
        denied.1["error"],
        "runtime_kernel_canary_requires_exact_dry_run"
    );

    let first = preference_http_round_trip(
        &pool,
        &listener,
        crate::runtime_composition::RUNTIME_KERNEL_CANARY_ACTION_ENDPOINT,
        &serde_json::json!({"dry_run": true}),
    );
    assert!(first.0.starts_with("HTTP/1.1 200 OK"));
    assert_eq!(first.1["status"], "succeeded");
    assert_eq!(first.1["action"], "runtime-kernel-canary");
    assert_eq!(first.1["active_model_provider"], "demo");
    assert_eq!(first.1["active_model"], "demo-chat");
    assert_eq!(first.1["invoked_tool"], "echo");
    assert_eq!(
        first.1["provider_effect_ack_requirement"],
        "not_applicable_read_only_tool"
    );
    assert_eq!(
        first.1["execution_receipt"]["durable_intent_recorded"],
        true
    );
    assert_eq!(first.1["execution_receipt"]["effect_plan_recorded"], false);
    assert!(first.1["execution_receipt"]["provider_effect_ack_hash"].is_null());
    assert_eq!(first.1["execution_receipt"]["terminal_status"], "succeeded");
    assert_eq!(first.1["external_network_requested"], false);
    assert_eq!(first.1["external_side_effects"], false);
    assert_eq!(first.1["live_surface_expanded"], false);
    assert_eq!(first.1["raw_request_body_exposed"], false);
    let first_attempt = first.1["execution_receipt"]["attempt_id"]
        .as_str()
        .expect("attempt id");
    assert!(
        runtime
            .terminal_receipt_recorded_for_test(first_attempt)
            .expect("terminal receipt readback")
    );

    let second = preference_http_round_trip(
        &pool,
        &listener,
        crate::runtime_composition::RUNTIME_KERNEL_CANARY_ACTION_ENDPOINT,
        &serde_json::json!({"dry_run": true}),
    );
    assert!(second.0.starts_with("HTTP/1.1 200 OK"));
    assert_ne!(
        second.1["execution_receipt"]["attempt_id"],
        first.1["execution_receipt"]["attempt_id"]
    );
    assert_eq!(
        second.1["request_binding_hash"],
        first.1["request_binding_hash"]
    );
}

#[test]
fn authenticated_preference_http_denies_tamper_replay_and_noncanonical_proofs_before_write() {
    use hepta_contracts::ContentHash;
    use hepta_intelligence::PreferenceIngressAuthenticationKey;
    use hepta_intelligence::sign_preference_ingress_challenge;

    let runtime_root = tempfile::tempdir().expect("runtime root");
    let runtime = Arc::new(
        NativeGatewayRuntime::bootstrap_with_anchor_for_test(runtime_root.path())
            .expect("keyed runtime"),
    );
    let debug = format!("{runtime:?}");
    assert!(debug.contains("[REDACTED]"));
    assert!(!debug.contains("4041424344454647"));
    let pool =
        NativeGatewayConnectionPool::new(test_gateway_options(false), Arc::clone(&runtime), 1, 4)
            .expect("worker pool");
    let listener = TcpListener::bind("127.0.0.1:0").expect("listener");
    let (_, session_binding_hash) = runtime
        .preference_session_binding()
        .expect("runtime preference session binding");
    let challenge_request = serde_json::json!({
        "transition_id": "transition:http-live",
        "evidence_id": "evidence:http-live",
        "signal": "accepted",
        "receipt": {
            "id": "receipt:http-live",
            "hash": "sha256:receipt-http-live"
        },
        "session_binding_hash": session_binding_hash,
        "subject": "subject:http-live",
        "preference": "preference:http-live",
        "target": {
            "kind": "capability",
            "capability_id": "tool:http-live",
            "capability_revision": 7,
            "manifest_hash": "sha256:manifest-http-live",
            "catalog_revision": 11,
            "catalog_hash": "sha256:catalog-main"
        }
    });
    let mut authentication_key = [0_u8; 32];
    for (index, byte) in authentication_key.iter_mut().enumerate() {
        *byte = 0x40 + u8::try_from(index).expect("key index");
    }
    let challenge_envelope = crate::preference_ingress::authenticated_challenge_envelope_for_test(
        &challenge_request,
        authentication_key,
    )
    .expect("authenticated challenge envelope");
    let denied_plan = preference_http_round_trip(
        &pool,
        &listener,
        crate::preference_ingress::PREFERENCE_CHALLENGE_ENDPOINT,
        &serde_json::json!({
            "request": challenge_request,
            "proof": "0".repeat(64),
        }),
    );
    assert!(denied_plan.0.starts_with("HTTP/1.1 403 Forbidden"));
    assert_eq!(
        denied_plan.1["error"],
        "trusted_preference_ingress.authentication_denied"
    );
    assert!(denied_plan.1.get("commit").is_none());
    let mut tampered_plan = challenge_envelope.clone();
    tampered_plan["request"]["session_binding_hash"] =
        serde_json::json!("sha256:tampered-plan-session");
    let denied_plan_tamper = preference_http_round_trip(
        &pool,
        &listener,
        crate::preference_ingress::PREFERENCE_CHALLENGE_ENDPOINT,
        &tampered_plan,
    );
    assert!(denied_plan_tamper.0.starts_with("HTTP/1.1 403 Forbidden"));
    assert_eq!(
        denied_plan_tamper.1["error"],
        "trusted_preference_ingress.runtime_session_binding_mismatch"
    );
    let mut uppercase_plan = challenge_envelope.clone();
    let uppercase_planning_proof = uppercase_plan["proof"]
        .as_str()
        .expect("planning proof")
        .to_ascii_uppercase();
    uppercase_plan["proof"] = serde_json::json!(uppercase_planning_proof);
    let denied_plan_encoding = preference_http_round_trip(
        &pool,
        &listener,
        crate::preference_ingress::PREFERENCE_CHALLENGE_ENDPOINT,
        &uppercase_plan,
    );
    assert!(denied_plan_encoding.0.starts_with("HTTP/1.1 403 Forbidden"));
    assert_eq!(
        denied_plan_encoding.1["error"],
        "trusted_preference_ingress.plan_proof_encoding_invalid"
    );
    let challenge = preference_http_round_trip(
        &pool,
        &listener,
        crate::preference_ingress::PREFERENCE_CHALLENGE_ENDPOINT,
        &challenge_envelope,
    );
    assert!(challenge.0.starts_with("HTTP/1.1 200 OK"));
    assert_eq!(challenge.1["challenge_authenticated"], true);
    assert_eq!(
        challenge.1["network_binding_policy"],
        "strict_loopback_host_origin_csrf_json"
    );
    assert_eq!(challenge.1["transport_confidentiality_claimed"], false);
    assert_eq!(
        challenge.1["authority"],
        "hepta.intelligence.authenticated-preference-plan"
    );
    assert_eq!(
        challenge.1["commit_authority"],
        "hepta.memory.authenticated-preference-cas"
    );
    assert_eq!(challenge.1["runtime_preflight"], "plan_only_quarantine");
    assert_eq!(challenge.1["runtime_effect_authority_claimed"], false);
    let challenge_hash = challenge.1["commit"]["challenge_hash"]
        .as_str()
        .expect("challenge hash");
    let proof = sign_preference_ingress_challenge(
        &PreferenceIngressAuthenticationKey::from_bytes(authentication_key),
        &ContentHash::new(challenge_hash),
    )
    .expect("sign challenge")
    .to_hex();
    let prepared = challenge.1["commit"].clone();

    let uppercase = preference_commit_body(&prepared, &proof.to_ascii_uppercase());
    let denied_uppercase = preference_http_round_trip(
        &pool,
        &listener,
        crate::preference_ingress::PREFERENCE_COMMIT_ENDPOINT,
        &uppercase,
    );
    assert!(
        denied_uppercase.0.starts_with("HTTP/1.1 403 Forbidden"),
        "{}",
        denied_uppercase.0
    );
    assert_eq!(
        denied_uppercase.1["error"],
        "trusted_preference_ingress.proof_encoding_invalid"
    );

    let wrong_proof = preference_commit_body(&prepared, &"0".repeat(64));
    let denied_proof = preference_http_round_trip(
        &pool,
        &listener,
        crate::preference_ingress::PREFERENCE_COMMIT_ENDPOINT,
        &wrong_proof,
    );
    assert!(denied_proof.0.starts_with("HTTP/1.1 403 Forbidden"));
    assert_eq!(
        denied_proof.1["error"],
        "trusted_preference_ingress.authentication_denied"
    );
    assert!(!denied_proof.0.contains("4041424344454647"));
    assert!(!denied_proof.0.contains(&proof));

    let mut wrong_source = prepared.clone();
    wrong_source["source"]["identity"] = serde_json::json!("source:wrong");
    let denied_source = preference_http_round_trip(
        &pool,
        &listener,
        crate::preference_ingress::PREFERENCE_COMMIT_ENDPOINT,
        &preference_commit_body(&wrong_source, &proof),
    );
    assert!(denied_source.0.starts_with("HTTP/1.1 403 Forbidden"));
    assert_eq!(
        denied_source.1["error"],
        "trusted_preference_ingress.source_binding_mismatch"
    );

    let mut tampered = prepared.clone();
    tampered["request"]["session_binding_hash"] = serde_json::json!("sha256:tampered-session");
    let denied_tamper = preference_http_round_trip(
        &pool,
        &listener,
        crate::preference_ingress::PREFERENCE_COMMIT_ENDPOINT,
        &preference_commit_body(&tampered, &proof),
    );
    assert!(denied_tamper.0.starts_with("HTTP/1.1 403 Forbidden"));
    assert_eq!(
        denied_tamper.1["error"],
        "trusted_preference_ingress.runtime_session_binding_mismatch"
    );

    let committed = preference_http_round_trip(
        &pool,
        &listener,
        crate::preference_ingress::PREFERENCE_COMMIT_ENDPOINT,
        &preference_commit_body(&prepared, &proof),
    );
    assert!(committed.0.starts_with("HTTP/1.1 200 OK"));
    assert_eq!(committed.1["committed_now"], true);
    assert_eq!(committed.1["committed_next"]["revision"], 1);
    assert_eq!(committed.1["runtime_effect_authority_claimed"], false);
    let attached = runtime
        .authenticated_preference_context_for_test()
        .expect("attached preference lookup")
        .expect("authenticated preference context");
    assert_eq!(attached.revision().get(), 1);
    assert_eq!(
        attached.content_hash().as_str(),
        committed.1["committed_next"]["content_hash"]
            .as_str()
            .expect("committed preference hash")
    );
    let expected_attachment = attached.clone();

    let replay = preference_http_round_trip(
        &pool,
        &listener,
        crate::preference_ingress::PREFERENCE_COMMIT_ENDPOINT,
        &preference_commit_body(&prepared, &proof),
    );
    assert!(replay.0.starts_with("HTTP/1.1 409 Conflict"));
    assert_eq!(
        replay.1["error"],
        "trusted_preference_ingress.state_conflict"
    );
    drop(listener);
    drop(pool);
    drop(runtime);
    let reopened = NativeGatewayRuntime::open_existing_with_anchor_for_test(runtime_root.path())
        .expect("reopen runtime with authenticated preference hydration");
    assert_eq!(
        reopened
            .authenticated_preference_context_for_test()
            .expect("reopened preference lookup"),
        Some(expected_attachment)
    );
}

fn preference_commit_body(prepared: &serde_json::Value, proof: &str) -> serde_json::Value {
    serde_json::json!({
        "commit": prepared,
        "proof": proof,
    })
}

fn preference_http_round_trip(
    pool: &NativeGatewayConnectionPool,
    listener: &TcpListener,
    path: &str,
    body: &serde_json::Value,
) -> (String, serde_json::Value) {
    use std::io::Read;
    use std::io::Write;

    let body = serde_json::to_string(body).expect("request body");
    let mut client = TcpStream::connect(listener.local_addr().expect("address")).expect("client");
    let host = DEFAULT_BIND_ADDR;
    let (server, _) = listener.accept().expect("server");
    pool.dispatch(server).expect("dispatch");
    write!(
        client,
        "POST {path} HTTP/1.1\r\nhost: {host}\r\norigin: http://{host}\r\nx-hepta-csrf: 1\r\ncontent-type: application/json\r\ncontent-length: {}\r\n\r\n{body}",
        body.len(),
    )
    .expect("request");
    let mut response = String::new();
    client.read_to_string(&mut response).expect("response");
    let payload = response
        .split_once("\r\n\r\n")
        .map(|(_, body)| body)
        .expect("response body");
    let value = serde_json::from_str(payload).expect("response json");
    (response, value)
}

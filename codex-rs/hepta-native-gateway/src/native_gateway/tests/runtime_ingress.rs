#[test]
fn plan_only_preflight_overrides_native_post_env_double_gate_inputs() {
    let receipt = RuntimeRequestPreflightReceipt {
        request_binding_hash: "bound-plan-request".into(),
        disposition: RuntimeRequestDisposition::PlanOnlyQuarantine,
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
    assert!(!report.execution_admission.current_plan_executes_real_handler);
    assert!(!report.real_handler_harness.store_write_attempted);
    assert!(!store_root.exists());
}

#[test]
fn keyed_runtime_reaches_http_worker_dispatch() {
    use std::io::Read;
    use std::io::Write;

    let runtime_root = tempfile::tempdir().expect("runtime root");
    let runtime = Arc::new(
        NativeGatewayRuntime::bootstrap_for_test(runtime_root.path()).expect("keyed runtime"),
    );
    let pool = NativeGatewayConnectionPool::new(test_gateway_options(false), runtime, 1, 1)
        .expect("worker pool");
    let listener = TcpListener::bind("127.0.0.1:0").expect("listener");
    let mut client =
        TcpStream::connect(listener.local_addr().expect("address")).expect("client");
    let (server, _) = listener.accept().expect("server");
    pool.dispatch(server).expect("dispatch");
    write!(client, "GET /health HTTP/1.1\r\n\r\n").expect("request");
    let mut response = String::new();
    client.read_to_string(&mut response).expect("response");
    assert!(response.starts_with("HTTP/1.1 200 OK"));
    assert!(response.contains(r#""status":"ready""#));
}

#[test]
fn authenticated_preference_http_denies_tamper_replay_and_noncanonical_proofs_before_write() {
    use hepta_contracts::ContentHash;
    use hepta_intelligence::PreferenceIngressAuthenticationKey;
    use hepta_intelligence::sign_preference_ingress_challenge;

    let runtime_root = tempfile::tempdir().expect("runtime root");
    let runtime = Arc::new(
        NativeGatewayRuntime::bootstrap_for_test(runtime_root.path()).expect("keyed runtime"),
    );
    let debug = format!("{runtime:?}");
    assert!(debug.contains("[REDACTED]"));
    assert!(!debug.contains("4041424344454647"));
    let pool = NativeGatewayConnectionPool::new(
        test_gateway_options(false),
        Arc::clone(&runtime),
        1,
        4,
    )
    .expect("worker pool");
    let listener = TcpListener::bind("127.0.0.1:0").expect("listener");
    let challenge_request = serde_json::json!({
        "transition_id": "transition:http-live",
        "evidence_id": "evidence:http-live",
        "signal": "accepted",
        "receipt": {
            "id": "receipt:http-live",
            "hash": "sha256:receipt-http-live"
        },
        "session_binding_hash": "sha256:session-http-live",
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
    let challenge_envelope =
        crate::preference_ingress::authenticated_challenge_envelope_for_test(
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
    assert!(
        denied_plan_tamper
            .0
            .starts_with("HTTP/1.1 403 Forbidden")
    );
    assert_eq!(
        denied_plan_tamper.1["error"],
        "trusted_preference_ingress.authentication_denied"
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
    assert!(
        denied_plan_encoding
            .0
            .starts_with("HTTP/1.1 403 Forbidden")
    );
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
        "loopback_default_explicit_lab_override_possible"
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
        "trusted_preference_ingress.challenge_binding_mismatch"
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
}

fn preference_commit_body(
    prepared: &serde_json::Value,
    proof: &str,
) -> serde_json::Value {
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
    let mut client =
        TcpStream::connect(listener.local_addr().expect("address")).expect("client");
    let (server, _) = listener.accept().expect("server");
    pool.dispatch(server).expect("dispatch");
    write!(
        client,
        "POST {path} HTTP/1.1\r\ncontent-type: application/json\r\ncontent-length: {}\r\n\r\n{body}",
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

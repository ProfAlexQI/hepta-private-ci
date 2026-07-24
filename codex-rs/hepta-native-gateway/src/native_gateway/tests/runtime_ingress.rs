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

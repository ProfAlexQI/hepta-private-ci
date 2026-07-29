fn hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_post_write_readback_binding_boundary_report()
-> serde_json::Value {
    const POST_WRITE_READBACK_SURFACES: &[&str] = &[
        "source_minimal_scoped_memory_real_write_canary_wal_receipt_binding_required",
        "post_write_readback_plan_binding_required",
        "post_write_readback_result_identity_binding_required",
        "readback_receipt_linkage_binding_required",
        "readback_payload_digest_comparison_binding_required",
        "readback_namespace_store_scope_binding_required",
        "readback_redaction_secret_scan_binding_required",
        "readback_stale_guard_binding_required",
        "readback_phantom_guard_binding_required",
        "readback_operator_review_handoff_binding_required",
        "rollback_tombstone_handoff_binding_required",
        "memory_kg_provider_channel_public_release_install_active_binary_side_effects_forbidden",
    ];
    const POST_WRITE_READBACK_DENIALS: &[&str] = &[
        "source_minimal_scoped_memory_real_write_canary_wal_receipt_binding_boundary_required",
        "post_write_readback_plan_binding_required",
        "post_write_readback_result_identity_binding_required",
        "readback_receipt_linkage_binding_required",
        "readback_payload_digest_comparison_binding_required",
        "readback_namespace_store_scope_binding_required",
        "readback_redaction_secret_scan_binding_required",
        "readback_stale_guard_binding_required",
        "readback_phantom_guard_binding_required",
        "readback_operator_review_handoff_binding_required",
        "rollback_tombstone_handoff_binding_required",
        "nonce_consumption_report_route_denied",
        "explicit_command_dispatch_report_route_denied",
        "wal_write_denied",
        "wal_persistence_denied",
        "receipt_recording_denied",
        "receipt_persistence_denied",
        "receipt_materialization_denied",
        "receipt_delivery_denied",
        "post_write_readback_execution_denied",
        "readback_result_recording_denied",
        "readback_result_persistence_denied",
        "readback_acceptance_denied",
        "durable_memory_store_read_denied",
        "durable_memory_store_write_denied",
        "durable_memory_store_rollback_denied",
        "memory_store_mutation_denied",
        "rollback_tombstone_execution_denied",
        "kg_provider_credential_channel_public_release_side_effect_denied",
        "install_restart_active_binary_filesystem_mutation_denied",
    ];
    const FALSE_KEYS: &[&str] = &[
        "single_use_nonce_consumed",
        "explicit_command_dispatched",
        "wal_write_performed",
        "wal_recorded",
        "wal_persisted",
        "receipt_recorded",
        "receipt_persisted",
        "receipt_materialized",
        "receipt_delivered",
        "post_write_readback_performed",
        "readback_result_recorded",
        "readback_result_persisted",
        "readback_result_accepted",
        "rollback_executed",
        "rollback_result_recorded",
        "rollback_result_persisted",
        "rollback_result_accepted",
        "tombstone_written",
        "compensating_memory_write_performed",
        "activation_performed",
        "live_mutation_execution_performed",
        "memory_write_execution_performed",
        "memory_store_write_path_enabled",
        "memory_store_write_allowed",
        "memory_store_write_performed",
        "memory_store_mutation_allowed",
        "memory_store_mutated",
        "durable_memory_store_write_performed",
        "durable_memory_store_read_performed",
        "durable_memory_store_rollback_performed",
        "raw_payload_plaintext_recorded",
        "raw_payload_plaintext_persisted",
        "secret_material_read",
        "credential_read",
        "secret_file_read",
        "kg_adapter_read_performed",
        "live_kg_write_performed",
        "provider_invoked",
        "model_invoked",
        "telegram_send_performed",
        "channel_send_performed",
        "external_send_performed",
        "public_claim_promoted",
        "public_release_published",
        "public_ga_claimed",
        "release_artifact_written",
        "public_artifact_written",
        "install_executed",
        "launchd_mutated",
        "service_restarted",
        "service_restart_performed",
        "active_binary_mutated",
        "filesystem_written",
    ];

    fn post_write_readback_fixture(
        id: &str,
        status: &str,
        reason: &str,
        accepted: bool,
        extra: serde_json::Value,
    ) -> serde_json::Value {
        let mut base = serde_json::Map::new();
        macro_rules! insert_fixture_json {
            ($key:literal, $value:expr) => {
                base.insert($key.to_string(), serde_json::json!($value));
            };
        }
        insert_fixture_json!("id", id);
        insert_fixture_json!("fixture_id", id);
        insert_fixture_json!(
            "minimal_scoped_memory_real_write_canary_post_write_readback_binding_status",
            status
        );
        insert_fixture_json!("source_wal_receipt_binding_present", true);
        insert_fixture_json!("source_wal_receipt_binding_ready", true);
        insert_fixture_json!("reason", reason);
        insert_fixture_json!("post_write_readback_binding_noop_confirmed", true);
        for key in [
            "post_write_readback_plan_binding_requested",
            "post_write_readback_result_identity_binding_requested",
            "readback_receipt_linkage_binding_requested",
            "readback_payload_digest_comparison_binding_requested",
            "readback_namespace_store_scope_binding_requested",
            "readback_redaction_secret_scan_binding_requested",
            "readback_stale_guard_binding_requested",
            "readback_phantom_guard_binding_requested",
            "readback_operator_review_handoff_binding_requested",
            "rollback_tombstone_handoff_binding_requested",
            "single_use_nonce_consumption_requested",
            "explicit_command_dispatch_requested",
            "wal_write_requested",
            "wal_persistence_requested",
            "receipt_recording_requested",
            "receipt_persistence_requested",
            "receipt_materialization_requested",
            "receipt_delivery_requested",
            "post_write_readback_execution_requested",
            "readback_result_recording_requested",
            "readback_result_persistence_requested",
            "readback_acceptance_requested",
            "durable_memory_read_requested",
            "durable_memory_write_requested",
            "durable_memory_rollback_requested",
            "rollback_execution_requested",
            "tombstone_write_requested",
            "kg_live_write_requested",
            "provider_model_invocation_requested",
            "credential_read_requested",
            "channel_external_send_requested",
            "public_claim_requested",
            "release_artifact_write_requested",
            "install_restart_requested",
            "active_binary_mutation_requested",
        ] {
            base.insert(key.to_string(), serde_json::json!(false));
        }
        for key in [
            "minimal_scoped_memory_real_write_canary_post_write_readback_binding_accepted",
            "post_write_readback_plan_bound",
            "readback_result_identity_bound",
            "readback_receipt_linkage_bound",
            "readback_payload_digest_comparison_bound",
            "readback_namespace_store_scope_bound",
            "readback_redaction_secret_scan_bound",
            "readback_stale_guard_bound",
            "readback_phantom_guard_bound",
            "readback_operator_review_handoff_bound",
            "rollback_tombstone_handoff_bound",
        ] {
            base.insert(key.to_string(), serde_json::json!(accepted));
        }
        for &key in FALSE_KEYS {
            base.insert(key.to_string(), serde_json::json!(false));
        }
        if let Some(extra) = extra.as_object() {
            for (key, value) in extra {
                base.insert(key.clone(), value.clone());
            }
        }
        serde_json::Value::Object(base)
    }

    let route_matrix = control_ui_route_parity_report();
    let source = std::thread::Builder::new()
        .name("hepta-memory-minimal-canary-post-write-readback-source-report".to_string())
        .stack_size(8 * 1024 * 1024)
        .spawn(
            hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_wal_receipt_binding_boundary_report,
        )
        .ok()
        .and_then(|handle| handle.join().ok())
        .unwrap_or_else(|| {
            serde_json::json!({
                "status": "blocked",
                "memory_write_execution_minimal_scoped_memory_real_write_canary_wal_receipt_binding_boundary_ready": false,
                "minimal_scoped_memory_real_write_canary_wal_receipt_binding_ready": false,
                "source_minimal_scoped_memory_real_write_canary_post_write_readback_source_report_thread_failed": true
            })
        });

    let json_bool = |value: &serde_json::Value, key: &str| {
        value
            .get(key)
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
    };
    let json_u64 = |value: &serde_json::Value, key: &str| {
        value
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let side_effects_all_false = |value: &serde_json::Value| {
        value
            .get("side_effects")
            .and_then(serde_json::Value::as_object)
            .map(|effects| effects.values().all(|item| item.as_bool() == Some(false)))
            .unwrap_or(false)
    };
    let route_count_source_command_accepted = route_matrix.ready
        && route_matrix.route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let source_next_action_post_write_readback = source
        .get("allowed_next_actions")
        .and_then(serde_json::Value::as_array)
        .and_then(|items| items.get(1))
        .map(|item| {
            item.get("action").and_then(serde_json::Value::as_str)
                == Some(
                    "prepare_minimal_scoped_memory_real_write_canary_post_write_readback_binding_boundary",
                )
                && item
                    .get("requires_minimal_scoped_memory_real_write_canary_wal_receipt_binding")
                    .and_then(serde_json::Value::as_bool)
                    == Some(true)
        })
        .unwrap_or(false);
    let source_ready = source.get("status").and_then(serde_json::Value::as_str) == Some("ready")
        && json_bool(
            &source,
            "memory_write_execution_minimal_scoped_memory_real_write_canary_wal_receipt_binding_boundary_ready",
        )
        && json_bool(
            &source,
            "minimal_scoped_memory_real_write_canary_wal_receipt_binding_ready",
        )
        && json_bool(
            &source,
            "minimal_scoped_memory_real_write_canary_wal_receipt_binding_accepted_no_write",
        )
        && json_u64(
            &source,
            "accepted_minimal_scoped_memory_real_write_canary_wal_receipt_binding_fixture_count",
        ) == 1
        && json_u64(
            &source,
            "blocked_minimal_scoped_memory_real_write_canary_wal_receipt_binding_fixture_count",
        ) == 9
        && json_u64(&source, "wal_receipt_binding_authority_accepted_count") == 1
        && json_u64(&source, "post_write_readback_handoff_bound_count") == 1
        && json_u64(&source, "single_use_nonce_consumed_count") == 0
        && json_u64(&source, "explicit_command_dispatched_count") == 0
        && json_u64(&source, "wal_write_performed_count") == 0
        && json_u64(&source, "receipt_persisted_count") == 0
        && json_u64(&source, "post_write_readback_performed_count") == 0
        && json_u64(&source, "durable_memory_store_read_performed_count") == 0
        && json_u64(&source, "durable_memory_store_write_performed_count") == 0
        && json_u64(&source, "durable_memory_store_rollback_performed_count") == 0
        && json_u64(&source, "memory_store_write_performed_count") == 0
        && !json_bool(&source, "single_use_nonce_consumed")
        && !json_bool(&source, "explicit_command_dispatched")
        && !json_bool(&source, "wal_write_performed")
        && !json_bool(&source, "receipt_persisted")
        && !json_bool(&source, "post_write_readback_performed")
        && !json_bool(&source, "memory_write_execution_performed")
        && !json_bool(&source, "memory_store_write_performed")
        && !json_bool(&source, "durable_memory_store_write_performed")
        && !json_bool(&source, "durable_memory_store_read_performed")
        && !json_bool(&source, "durable_memory_store_rollback_performed")
        && !json_bool(&source, "live_kg_write_performed")
        && !json_bool(&source, "provider_invoked")
        && !json_bool(&source, "model_invoked")
        && !json_bool(&source, "credential_read")
        && !json_bool(&source, "channel_send_performed")
        && !json_bool(&source, "external_send_performed")
        && !json_bool(&source, "release_artifact_written")
        && !json_bool(&source, "install_executed")
        && !json_bool(&source, "active_binary_mutated")
        && source_next_action_post_write_readback
        && side_effects_all_false(&source);

    let fixtures = serde_json::Value::Array(vec![
        post_write_readback_fixture(
            "minimal-scoped-memory-real-write-canary-post-write-readback-binding-envelope",
            "accepted_post_write_readback_binding_no_read_or_write",
            "post_write_readback_binding_evidence_accepted_without_memory_read_or_write",
            true,
            serde_json::json!({
                "post_write_readback_plan_binding_requested": true,
                "post_write_readback_result_identity_binding_requested": true,
                "readback_receipt_linkage_binding_requested": true,
                "readback_payload_digest_comparison_binding_requested": true,
                "readback_namespace_store_scope_binding_requested": true,
                "readback_redaction_secret_scan_binding_requested": true,
                "readback_stale_guard_binding_requested": true,
                "readback_phantom_guard_binding_requested": true,
                "readback_operator_review_handoff_binding_requested": true,
                "rollback_tombstone_handoff_binding_requested": true
            }),
        ),
        post_write_readback_fixture(
            "minimal-scoped-memory-real-write-canary-post-write-readback-missing-wal-receipt-source",
            "blocked_source_noop",
            "source_minimal_scoped_memory_real_write_canary_wal_receipt_binding_boundary_required",
            false,
            serde_json::json!({
                "source_wal_receipt_binding_present": false,
                "source_wal_receipt_binding_ready": false,
                "post_write_readback_plan_binding_requested": true
            }),
        ),
        post_write_readback_fixture(
            "minimal-scoped-memory-real-write-canary-post-write-readback-plan-required",
            "blocked_readback_plan_noop",
            "post_write_readback_plan_binding_required",
            false,
            serde_json::json!({"post_write_readback_plan_binding_requested": true}),
        ),
        post_write_readback_fixture(
            "minimal-scoped-memory-real-write-canary-readback-result-identity-required",
            "blocked_readback_result_noop",
            "post_write_readback_result_identity_binding_required",
            false,
            serde_json::json!({"post_write_readback_result_identity_binding_requested": true}),
        ),
        post_write_readback_fixture(
            "minimal-scoped-memory-real-write-canary-readback-receipt-linkage-required",
            "blocked_readback_receipt_noop",
            "readback_receipt_linkage_binding_required",
            false,
            serde_json::json!({"readback_receipt_linkage_binding_requested": true}),
        ),
        post_write_readback_fixture(
            "minimal-scoped-memory-real-write-canary-readback-payload-digest-required",
            "blocked_readback_payload_digest_noop",
            "readback_payload_digest_comparison_binding_required",
            false,
            serde_json::json!({"readback_payload_digest_comparison_binding_requested": true}),
        ),
        post_write_readback_fixture(
            "minimal-scoped-memory-real-write-canary-readback-namespace-scope-required",
            "blocked_readback_namespace_scope_noop",
            "readback_namespace_store_scope_binding_required",
            false,
            serde_json::json!({"readback_namespace_store_scope_binding_requested": true}),
        ),
        post_write_readback_fixture(
            "minimal-scoped-memory-real-write-canary-readback-redaction-secret-scan-required",
            "blocked_readback_redaction_noop",
            "readback_redaction_secret_scan_binding_required",
            false,
            serde_json::json!({"readback_redaction_secret_scan_binding_requested": true}),
        ),
        post_write_readback_fixture(
            "minimal-scoped-memory-real-write-canary-readback-stale-phantom-guard-required",
            "blocked_readback_stale_phantom_noop",
            "readback_stale_and_phantom_guards_required",
            false,
            serde_json::json!({
                "readback_stale_guard_binding_requested": true,
                "readback_phantom_guard_binding_requested": true
            }),
        ),
        post_write_readback_fixture(
            "minimal-scoped-memory-real-write-canary-post-write-readback-direct-side-effect-attempt",
            "blocked_direct_side_effect_noop",
            "direct_readback_memory_and_external_side_effects_denied",
            false,
            serde_json::json!({
                "single_use_nonce_consumption_requested": true,
                "explicit_command_dispatch_requested": true,
                "wal_write_requested": true,
                "wal_persistence_requested": true,
                "receipt_recording_requested": true,
                "receipt_persistence_requested": true,
                "post_write_readback_execution_requested": true,
                "readback_result_recording_requested": true,
                "readback_result_persistence_requested": true,
                "readback_acceptance_requested": true,
                "durable_memory_read_requested": true,
                "durable_memory_write_requested": true,
                "durable_memory_rollback_requested": true,
                "rollback_execution_requested": true,
                "tombstone_write_requested": true,
                "kg_live_write_requested": true,
                "provider_model_invocation_requested": true,
                "credential_read_requested": true,
                "channel_external_send_requested": true,
                "release_artifact_write_requested": true,
                "install_restart_requested": true,
                "active_binary_mutation_requested": true
            }),
        ),
    ]);

    let fixture_count = fixtures.as_array().map(std::vec::Vec::len).unwrap_or(0);
    let accepted_fixture_count = fixtures
        .as_array()
        .map(|items| {
            items
                .iter()
                .filter(|fixture| {
                    fixture
                        .get("minimal_scoped_memory_real_write_canary_post_write_readback_binding_accepted")
                        .and_then(serde_json::Value::as_bool)
                        == Some(true)
                })
                .count()
        })
        .unwrap_or(0);
    let blocked_fixture_count = fixture_count.saturating_sub(accepted_fixture_count);
    let denials = POST_WRITE_READBACK_DENIALS
        .iter()
        .map(|reason| serde_json::json!(reason))
        .collect::<Vec<_>>();
    let denied_count = denials.len();
    let source_report_sha256 = sha256_text_value(&source.to_string());
    let boundary_hash_sha256 = sha256_text_value(&format!(
        "minimal-scoped-memory-real-write-canary-post-write-readback-binding-boundary:v1:source-ready={source_ready}:fixtures={fixture_count}:accepted={accepted_fixture_count}:denials={denied_count}:memory-reads=0:memory-writes=0"
    ));
    let policy_hash_sha256 = sha256_text_value(
        "minimal-scoped-memory-real-write-canary-post-write-readback-binding-policy:v1:no-nonce-consume:no-command-dispatch:no-wal-write:no-receipt-persist:no-memory-read-write",
    );
    let mut side_effects = serde_json::Map::new();
    for &key in FALSE_KEYS {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }
    let required_fields = serde_json::json!([
        "source_minimal_scoped_memory_real_write_canary_wal_receipt_binding_report_sha256",
        "wal_receipt_binding_fixture_id",
        "post_write_readback_plan_id",
        "readback_result_id",
        "readback_receipt_id",
        "wal_record_id",
        "receipt_id",
        "payload_digest_sha256",
        "readback_digest_comparison_id",
        "readback_namespace",
        "readback_store",
        "readback_scope",
        "redaction_secret_scan_id",
        "stale_read_guard_id",
        "phantom_read_guard_id",
        "operator_review_handoff_id",
        "rollback_tombstone_handoff_id",
        "active_binary_sha256",
        "route_count",
    ]);
    let allowed_next_actions = serde_json::json!([
        {
            "action": "run_minimal_scoped_memory_real_write_canary_post_write_readback_binding_boundary_require_live_gate",
            "status": "allowed_verification_only",
            "reads_memory": false,
            "writes_memory": false,
            "consumes_nonce": false,
            "dispatches_command": false,
            "writes_wal": false,
            "persists_receipt": false,
            "records_readback": false,
            "executes_rollback": false,
            "writes_tombstone": false,
            "writes_kg": false,
            "invokes_provider": false,
            "reads_credentials": false,
            "sends_externally": false,
            "publishes_artifacts": false,
            "installs_or_restarts": false,
            "mutates_active_binary": false
        },
        {
            "action": "prepare_minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_boundary",
            "status": "allowed_report_only_next_slice",
            "requires_minimal_scoped_memory_real_write_canary_post_write_readback_binding": true,
            "writes_memory": false,
            "reads_memory": false,
            "consumes_nonce": false,
            "dispatches_command": false,
            "writes_wal": false,
            "persists_receipt": false,
            "records_readback": false,
            "executes_rollback": false,
            "writes_tombstone": false,
            "writes_kg": false,
            "invokes_provider": false,
            "sends_externally": false
        }
    ]);
    let report_ready = route_count_source_command_accepted
        && source_ready
        && fixture_count == 10
        && accepted_fixture_count == 1
        && blocked_fixture_count == 9
        && denied_count == 30;

    let mut report = serde_json::Map::new();
    macro_rules! insert_report_json {
        ($key:literal, $value:expr) => {
            report.insert($key.to_string(), serde_json::json!($value));
        };
    }
    insert_report_json!("product", "Hepta");
    insert_report_json!("runtime", "hepta");
    insert_report_json!("status", if report_ready { "ready" } else { "blocked" });
    insert_report_json!("base_url", "http://127.0.0.1:7373");
    insert_report_json!(
        "gate",
        "hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_post_write_readback_binding_boundary_route"
    );
    insert_report_json!(
        "endpoint",
        HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_MINIMAL_SCOPED_MEMORY_REAL_WRITE_CANARY_POST_WRITE_READBACK_BINDING_BOUNDARY_ENDPOINT
    );
    insert_report_json!(
        "source_command",
        "/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-post-write-readback-binding-boundary --json"
    );
    insert_report_json!("native_route", true);
    insert_report_json!("side_effect_free", true);
    insert_report_json!("audit_date", "2026-07-03");
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_post_write_readback_binding_schema_version",
        "minimal_scoped_memory_real_write_canary_post_write_readback_binding_boundary_v1"
    );
    insert_report_json!(
        "scoped_memory_real_write_canary_mode",
        "minimal_scoped_memory_real_write_canary_post_write_readback_binding_no_read_or_write"
    );
    insert_report_json!(
        "native_gateway_source_command_count",
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    insert_report_json!("route_count", route_matrix.route_count);
    insert_report_json!(
        "implemented_route_count",
        route_matrix.implemented_route_count
    );
    insert_report_json!("missing_route_count", route_matrix.missing_route_count);
    insert_report_json!(
        "route_count_source_command_accepted",
        route_count_source_command_accepted
    );
    insert_report_json!(
        "memory_write_execution_minimal_scoped_memory_real_write_canary_post_write_readback_binding_boundary_ready",
        report_ready
    );
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_post_write_readback_binding_ready",
        report_ready
    );
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_post_write_readback_binding_accepted_no_read_or_write",
        report_ready
    );
    insert_report_json!(
        "source_minimal_scoped_memory_real_write_canary_wal_receipt_binding_boundary_ready",
        source_ready
    );
    insert_report_json!(
        "source_minimal_scoped_memory_real_write_canary_wal_receipt_binding_ready",
        json_bool(
            &source,
            "minimal_scoped_memory_real_write_canary_wal_receipt_binding_ready"
        )
    );
    insert_report_json!(
        "source_minimal_scoped_memory_real_write_canary_wal_receipt_binding_report_sha256",
        source_report_sha256
    );
    insert_report_json!(
        "source_accepted_minimal_scoped_memory_real_write_canary_wal_receipt_binding_fixture_count",
        json_u64(
            &source,
            "accepted_minimal_scoped_memory_real_write_canary_wal_receipt_binding_fixture_count"
        )
    );
    insert_report_json!(
        "source_blocked_minimal_scoped_memory_real_write_canary_wal_receipt_binding_fixture_count",
        json_u64(
            &source,
            "blocked_minimal_scoped_memory_real_write_canary_wal_receipt_binding_fixture_count"
        )
    );
    insert_report_json!(
        "source_wal_receipt_binding_authority_accepted_count",
        json_u64(&source, "wal_receipt_binding_authority_accepted_count")
    );
    insert_report_json!(
        "source_post_write_readback_handoff_bound_count",
        json_u64(&source, "post_write_readback_handoff_bound_count")
    );
    insert_report_json!(
        "source_denied_by_minimal_scoped_memory_real_write_canary_wal_receipt_binding_boundary_count",
        json_u64(
            &source,
            "denied_by_minimal_scoped_memory_real_write_canary_wal_receipt_binding_boundary_count"
        )
    );
    for key in [
        "single_use_nonce_consumed_count",
        "explicit_command_dispatched_count",
        "wal_write_performed_count",
        "receipt_persisted_count",
        "post_write_readback_performed_count",
        "durable_memory_store_read_performed_count",
        "durable_memory_store_write_performed_count",
        "durable_memory_store_rollback_performed_count",
        "memory_store_write_performed_count",
    ] {
        report.insert(
            format!("source_{key}"),
            serde_json::json!(json_u64(&source, key)),
        );
    }
    insert_report_json!("minimum_required_samples", 24);
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_post_write_readback_binding_boundary_hash_sha256",
        boundary_hash_sha256
    );
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_post_write_readback_binding_policy_hash_sha256",
        policy_hash_sha256
    );
    insert_report_json!(
        "required_minimal_scoped_memory_real_write_canary_post_write_readback_binding_surface_count",
        12
    );
    insert_report_json!(
        "ready_minimal_scoped_memory_real_write_canary_post_write_readback_binding_surface_count",
        12
    );
    insert_report_json!(
        "side_effect_free_minimal_scoped_memory_real_write_canary_post_write_readback_binding_surface_count",
        12
    );
    insert_report_json!(
        "required_minimal_scoped_memory_real_write_canary_post_write_readback_binding_fixture_count",
        10
    );
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_post_write_readback_binding_fixture_count",
        fixture_count
    );
    insert_report_json!(
        "accepted_minimal_scoped_memory_real_write_canary_post_write_readback_binding_fixture_count",
        accepted_fixture_count
    );
    insert_report_json!(
        "blocked_minimal_scoped_memory_real_write_canary_post_write_readback_binding_fixture_count",
        blocked_fixture_count
    );
    insert_report_json!(
        "noop_minimal_scoped_memory_real_write_canary_post_write_readback_binding_fixture_count",
        fixture_count
    );
    for key in [
        "post_write_readback_binding_authority_accepted_count",
        "post_write_readback_plan_bound_count",
        "readback_result_identity_bound_count",
        "readback_receipt_linkage_bound_count",
        "readback_payload_digest_comparison_bound_count",
        "readback_namespace_store_scope_bound_count",
        "readback_redaction_secret_scan_bound_count",
        "readback_stale_guard_bound_count",
        "readback_phantom_guard_bound_count",
        "readback_operator_review_handoff_bound_count",
        "rollback_tombstone_handoff_bound_count",
        "minimal_scoped_memory_real_write_canary_post_write_readback_binding_accepted_count",
    ] {
        report.insert(key.to_string(), serde_json::json!(1));
    }
    for key in [
        "single_use_nonce_consumed_count",
        "explicit_command_dispatched_count",
        "wal_write_performed_count",
        "wal_recorded_count",
        "wal_persisted_count",
        "receipt_recorded_count",
        "receipt_persisted_count",
        "receipt_materialized_count",
        "receipt_delivered_count",
        "post_write_readback_performed_count",
        "readback_result_recorded_count",
        "readback_result_persisted_count",
        "readback_result_accepted_count",
        "rollback_performed_count",
        "rollback_result_recorded_count",
        "rollback_result_persisted_count",
        "rollback_result_accepted_count",
        "tombstone_written_count",
        "compensating_memory_write_performed_count",
        "durable_memory_store_read_performed_count",
        "durable_memory_store_rollback_performed_count",
        "durable_memory_store_write_performed_count",
        "memory_store_write_performed_count",
    ] {
        report.insert(key.to_string(), serde_json::json!(0));
    }
    insert_report_json!(
        "required_before_minimal_scoped_memory_real_write_canary_post_write_readback_binding_count",
        19
    );
    report.insert(
        "required_minimal_scoped_memory_real_write_canary_post_write_readback_binding_fields"
            .to_string(),
        required_fields,
    );
    report.insert(
        "minimal_scoped_memory_real_write_canary_post_write_readback_binding_surfaces".to_string(),
        serde_json::json!(POST_WRITE_READBACK_SURFACES),
    );
    report.insert(
        "minimal_scoped_memory_real_write_canary_post_write_readback_binding_fixtures".to_string(),
        fixtures,
    );
    report.insert(
        "denied_by_minimal_scoped_memory_real_write_canary_post_write_readback_binding_boundary"
            .to_string(),
        serde_json::Value::Array(denials),
    );
    insert_report_json!(
        "denied_by_minimal_scoped_memory_real_write_canary_post_write_readback_binding_boundary_count",
        denied_count
    );
    for key in [
        "source_minimal_scoped_memory_real_write_canary_wal_receipt_binding_required",
        "minimal_scoped_memory_real_write_canary_post_write_readback_binding_accepted",
        "post_write_readback_plan_bound",
        "readback_result_identity_bound",
        "readback_receipt_linkage_bound",
        "readback_payload_digest_comparison_bound",
        "readback_namespace_store_scope_bound",
        "readback_redaction_secret_scan_bound",
        "readback_stale_guard_bound",
        "readback_phantom_guard_bound",
        "readback_operator_review_handoff_bound",
        "rollback_tombstone_handoff_bound",
        "nonce_consumption_forbidden_on_report_route",
        "explicit_command_dispatch_forbidden_on_report_route",
        "wal_write_forbidden",
        "wal_persistence_forbidden",
        "receipt_recording_forbidden",
        "receipt_persistence_forbidden",
        "receipt_materialization_forbidden",
        "receipt_delivery_forbidden",
        "post_write_readback_forbidden_on_report_route",
        "readback_result_recording_forbidden",
        "readback_result_persistence_forbidden",
        "readback_acceptance_forbidden",
        "durable_memory_read_forbidden",
        "durable_memory_write_forbidden",
        "durable_memory_rollback_forbidden",
        "memory_store_mutation_forbidden",
        "rollback_execution_forbidden",
        "tombstone_write_forbidden",
        "kg_live_write_forbidden",
        "provider_model_invocation_forbidden",
        "credential_channel_public_release_forbidden",
        "install_restart_active_binary_filesystem_mutation_forbidden",
    ] {
        report.insert(key.to_string(), serde_json::json!(true));
    }
    report.insert("allowed_next_actions".to_string(), allowed_next_actions);
    for &key in FALSE_KEYS {
        report.insert(key.to_string(), serde_json::json!(false));
    }
    report.insert(
        "side_effects".to_string(),
        serde_json::Value::Object(side_effects),
    );
    serde_json::Value::Object(report)
}
fn hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_boundary_report()
-> serde_json::Value {
    const PROOF_SURFACES: &[&str] = &[
        "source_minimal_scoped_memory_real_write_canary_post_write_readback_binding_required",
        "rollback_plan_proof_binding_required",
        "tombstone_plan_proof_binding_required",
        "rollback_target_proof_binding_required",
        "tombstone_target_proof_binding_required",
        "rollback_receipt_linkage_proof_binding_required",
        "tombstone_receipt_linkage_proof_binding_required",
        "rollback_idempotency_guard_proof_binding_required",
        "tombstone_idempotency_guard_proof_binding_required",
        "rollback_tombstone_audit_evidence_proof_binding_required",
        "operator_review_and_minimal_write_handoff_proof_binding_required",
        "memory_kg_provider_channel_public_release_install_active_binary_side_effects_forbidden",
    ];
    const PROOF_DENIALS: &[&str] = &[
        "source_minimal_scoped_memory_real_write_canary_post_write_readback_binding_boundary_required",
        "rollback_plan_proof_binding_required",
        "tombstone_plan_proof_binding_required",
        "rollback_target_proof_binding_required",
        "tombstone_target_proof_binding_required",
        "rollback_receipt_linkage_proof_binding_required",
        "tombstone_receipt_linkage_proof_binding_required",
        "rollback_idempotency_guard_proof_binding_required",
        "tombstone_idempotency_guard_proof_binding_required",
        "rollback_tombstone_audit_evidence_proof_binding_required",
        "operator_review_handoff_proof_binding_required",
        "minimal_real_write_canary_handoff_proof_binding_required",
        "nonce_consumption_report_route_denied",
        "explicit_command_dispatch_report_route_denied",
        "wal_write_denied",
        "wal_persistence_denied",
        "receipt_recording_denied",
        "receipt_persistence_denied",
        "receipt_materialization_denied",
        "receipt_delivery_denied",
        "post_write_readback_execution_denied",
        "readback_result_recording_denied",
        "readback_result_persistence_denied",
        "readback_acceptance_denied",
        "rollback_execution_denied",
        "rollback_result_recording_denied",
        "rollback_result_persistence_denied",
        "rollback_result_acceptance_denied",
        "tombstone_write_denied",
        "compensating_memory_write_denied",
        "durable_memory_store_read_denied",
        "durable_memory_store_write_denied",
        "durable_memory_store_rollback_denied",
        "memory_store_mutation_denied",
        "kg_provider_credential_channel_public_release_side_effect_denied",
        "install_restart_active_binary_filesystem_mutation_denied",
    ];
    const FALSE_KEYS: &[&str] = &[
        "single_use_nonce_consumed",
        "explicit_command_dispatched",
        "wal_write_performed",
        "wal_recorded",
        "wal_persisted",
        "receipt_recorded",
        "receipt_persisted",
        "receipt_materialized",
        "receipt_delivered",
        "post_write_readback_performed",
        "readback_result_recorded",
        "readback_result_persisted",
        "readback_result_accepted",
        "rollback_executed",
        "rollback_result_recorded",
        "rollback_result_persisted",
        "rollback_result_accepted",
        "tombstone_written",
        "compensating_memory_write_performed",
        "activation_performed",
        "live_mutation_execution_performed",
        "memory_write_execution_performed",
        "memory_store_write_path_enabled",
        "memory_store_write_allowed",
        "memory_store_write_performed",
        "memory_store_mutation_allowed",
        "memory_store_mutated",
        "durable_memory_store_write_performed",
        "durable_memory_store_read_performed",
        "durable_memory_store_rollback_performed",
        "raw_payload_plaintext_recorded",
        "raw_payload_plaintext_persisted",
        "secret_material_read",
        "credential_read",
        "secret_file_read",
        "kg_adapter_read_performed",
        "live_kg_write_performed",
        "provider_invoked",
        "model_invoked",
        "telegram_send_performed",
        "channel_send_performed",
        "external_send_performed",
        "public_claim_promoted",
        "public_release_published",
        "public_ga_claimed",
        "release_artifact_written",
        "public_artifact_written",
        "install_executed",
        "launchd_mutated",
        "service_restarted",
        "service_restart_performed",
        "active_binary_mutated",
        "filesystem_written",
    ];

    fn proof_fixture(
        id: &str,
        status: &str,
        reason: &str,
        accepted: bool,
        extra: serde_json::Value,
    ) -> serde_json::Value {
        let mut base = serde_json::Map::new();
        macro_rules! insert_fixture_json {
            ($key:literal, $value:expr) => {
                base.insert($key.to_string(), serde_json::json!($value));
            };
        }
        insert_fixture_json!("id", id);
        insert_fixture_json!("fixture_id", id);
        insert_fixture_json!(
            "minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_status",
            status
        );
        insert_fixture_json!("source_post_write_readback_binding_present", true);
        insert_fixture_json!("source_post_write_readback_binding_ready", true);
        insert_fixture_json!("reason", reason);
        insert_fixture_json!("rollback_tombstone_proof_noop_confirmed", true);
        for key in [
            "rollback_plan_proof_binding_requested",
            "tombstone_plan_proof_binding_requested",
            "rollback_target_proof_binding_requested",
            "tombstone_target_proof_binding_requested",
            "rollback_receipt_linkage_proof_binding_requested",
            "tombstone_receipt_linkage_proof_binding_requested",
            "rollback_idempotency_guard_proof_binding_requested",
            "tombstone_idempotency_guard_proof_binding_requested",
            "rollback_tombstone_audit_evidence_proof_binding_requested",
            "operator_review_handoff_proof_binding_requested",
            "minimal_real_write_canary_handoff_proof_binding_requested",
            "single_use_nonce_consumption_requested",
            "explicit_command_dispatch_requested",
            "wal_write_requested",
            "receipt_persistence_requested",
            "post_write_readback_execution_requested",
            "readback_result_recording_requested",
            "readback_result_persistence_requested",
            "readback_acceptance_requested",
            "rollback_execution_requested",
            "rollback_result_recording_requested",
            "rollback_result_persistence_requested",
            "rollback_result_acceptance_requested",
            "tombstone_write_requested",
            "compensating_memory_write_requested",
            "durable_memory_read_requested",
            "durable_memory_write_requested",
            "durable_memory_rollback_requested",
            "kg_live_write_requested",
            "provider_model_invocation_requested",
            "credential_read_requested",
            "channel_external_send_requested",
            "public_claim_requested",
            "release_artifact_write_requested",
            "install_restart_requested",
            "active_binary_mutation_requested",
        ] {
            base.insert(key.to_string(), serde_json::json!(false));
        }
        for key in [
            "minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_accepted",
            "rollback_plan_proof_bound",
            "tombstone_plan_proof_bound",
            "rollback_target_proof_bound",
            "tombstone_target_proof_bound",
            "rollback_receipt_linkage_proof_bound",
            "tombstone_receipt_linkage_proof_bound",
            "rollback_idempotency_guard_proof_bound",
            "tombstone_idempotency_guard_proof_bound",
            "rollback_tombstone_audit_evidence_proof_bound",
            "operator_review_handoff_proof_bound",
            "minimal_real_write_canary_handoff_proof_bound",
        ] {
            base.insert(key.to_string(), serde_json::json!(accepted));
        }
        for &key in FALSE_KEYS {
            base.insert(key.to_string(), serde_json::json!(false));
        }
        if let Some(extra) = extra.as_object() {
            for (key, value) in extra {
                base.insert(key.clone(), value.clone());
            }
        }
        serde_json::Value::Object(base)
    }

    let route_matrix = control_ui_route_parity_report();
    let source = std::thread::Builder::new()
        .name("hepta-memory-minimal-canary-rollback-tombstone-proof-source-report".to_string())
        .stack_size(8 * 1024 * 1024)
        .spawn(
            hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_post_write_readback_binding_boundary_report,
        )
        .ok()
        .and_then(|handle| handle.join().ok())
        .unwrap_or_else(|| {
            serde_json::json!({
                "status": "blocked",
                "memory_write_execution_minimal_scoped_memory_real_write_canary_post_write_readback_binding_boundary_ready": false,
                "minimal_scoped_memory_real_write_canary_post_write_readback_binding_ready": false,
                "source_minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_source_report_thread_failed": true
            })
        });

    let json_bool = |value: &serde_json::Value, key: &str| {
        value
            .get(key)
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
    };
    let json_u64 = |value: &serde_json::Value, key: &str| {
        value
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let side_effects_all_false = |value: &serde_json::Value| {
        value
            .get("side_effects")
            .and_then(serde_json::Value::as_object)
            .map(|effects| effects.values().all(|item| item.as_bool() == Some(false)))
            .unwrap_or(false)
    };
    let route_count_source_command_accepted = route_matrix.ready
        && route_matrix.route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let source_next_action_rollback_proof = source
        .get("allowed_next_actions")
        .and_then(serde_json::Value::as_array)
        .and_then(|items| items.get(1))
        .map(|item| {
            item.get("action").and_then(serde_json::Value::as_str)
                == Some("prepare_minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_boundary")
                && item
                    .get("requires_minimal_scoped_memory_real_write_canary_post_write_readback_binding")
                    .and_then(serde_json::Value::as_bool)
                    == Some(true)
        })
        .unwrap_or(false);
    let source_ready = source.get("status").and_then(serde_json::Value::as_str) == Some("ready")
        && json_bool(
            &source,
            "memory_write_execution_minimal_scoped_memory_real_write_canary_post_write_readback_binding_boundary_ready",
        )
        && json_bool(
            &source,
            "minimal_scoped_memory_real_write_canary_post_write_readback_binding_ready",
        )
        && json_bool(
            &source,
            "minimal_scoped_memory_real_write_canary_post_write_readback_binding_accepted_no_read_or_write",
        )
        && json_u64(
            &source,
            "accepted_minimal_scoped_memory_real_write_canary_post_write_readback_binding_fixture_count",
        ) == 1
        && json_u64(
            &source,
            "blocked_minimal_scoped_memory_real_write_canary_post_write_readback_binding_fixture_count",
        ) == 9
        && json_u64(
            &source,
            "post_write_readback_binding_authority_accepted_count",
        ) == 1
        && json_u64(&source, "rollback_tombstone_handoff_bound_count") == 1
        && json_u64(&source, "single_use_nonce_consumed_count") == 0
        && json_u64(&source, "explicit_command_dispatched_count") == 0
        && json_u64(&source, "wal_write_performed_count") == 0
        && json_u64(&source, "receipt_persisted_count") == 0
        && json_u64(&source, "post_write_readback_performed_count") == 0
        && json_u64(&source, "readback_result_recorded_count") == 0
        && json_u64(&source, "readback_result_persisted_count") == 0
        && json_u64(&source, "readback_result_accepted_count") == 0
        && json_u64(&source, "rollback_performed_count") == 0
        && json_u64(&source, "tombstone_written_count") == 0
        && json_u64(&source, "durable_memory_store_read_performed_count") == 0
        && json_u64(&source, "durable_memory_store_write_performed_count") == 0
        && json_u64(&source, "durable_memory_store_rollback_performed_count") == 0
        && json_u64(&source, "memory_store_write_performed_count") == 0
        && !json_bool(&source, "single_use_nonce_consumed")
        && !json_bool(&source, "explicit_command_dispatched")
        && !json_bool(&source, "wal_write_performed")
        && !json_bool(&source, "receipt_persisted")
        && !json_bool(&source, "post_write_readback_performed")
        && !json_bool(&source, "readback_result_accepted")
        && !json_bool(&source, "rollback_executed")
        && !json_bool(&source, "tombstone_written")
        && !json_bool(&source, "memory_write_execution_performed")
        && !json_bool(&source, "memory_store_write_performed")
        && !json_bool(&source, "durable_memory_store_write_performed")
        && !json_bool(&source, "durable_memory_store_read_performed")
        && !json_bool(&source, "durable_memory_store_rollback_performed")
        && !json_bool(&source, "live_kg_write_performed")
        && !json_bool(&source, "provider_invoked")
        && !json_bool(&source, "model_invoked")
        && !json_bool(&source, "credential_read")
        && !json_bool(&source, "channel_send_performed")
        && !json_bool(&source, "external_send_performed")
        && !json_bool(&source, "release_artifact_written")
        && !json_bool(&source, "install_executed")
        && !json_bool(&source, "active_binary_mutated")
        && source_next_action_rollback_proof
        && side_effects_all_false(&source);

    let fixtures = serde_json::Value::Array(vec![
        proof_fixture(
            "minimal-scoped-memory-real-write-canary-rollback-tombstone-proof-envelope",
            "accepted_rollback_tombstone_proof_no_rollback_or_write",
            "rollback_tombstone_proof_binding_evidence_accepted_without_rollback_or_write",
            true,
            serde_json::json!({
                "rollback_plan_proof_binding_requested": true,
                "tombstone_plan_proof_binding_requested": true,
                "rollback_target_proof_binding_requested": true,
                "tombstone_target_proof_binding_requested": true,
                "rollback_receipt_linkage_proof_binding_requested": true,
                "tombstone_receipt_linkage_proof_binding_requested": true,
                "rollback_idempotency_guard_proof_binding_requested": true,
                "tombstone_idempotency_guard_proof_binding_requested": true,
                "rollback_tombstone_audit_evidence_proof_binding_requested": true,
                "operator_review_handoff_proof_binding_requested": true,
                "minimal_real_write_canary_handoff_proof_binding_requested": true
            }),
        ),
        proof_fixture(
            "minimal-scoped-memory-real-write-canary-rollback-proof-missing-post-write-readback-source",
            "blocked_source_noop",
            "source_minimal_scoped_memory_real_write_canary_post_write_readback_binding_boundary_required",
            false,
            serde_json::json!({
                "source_post_write_readback_binding_present": false,
                "source_post_write_readback_binding_ready": false,
                "rollback_plan_proof_binding_requested": true
            }),
        ),
        proof_fixture(
            "minimal-scoped-memory-real-write-canary-rollback-plan-proof-required",
            "blocked_rollback_plan_noop",
            "rollback_plan_proof_binding_required",
            false,
            serde_json::json!({"rollback_plan_proof_binding_requested": true}),
        ),
        proof_fixture(
            "minimal-scoped-memory-real-write-canary-tombstone-plan-proof-required",
            "blocked_tombstone_plan_noop",
            "tombstone_plan_proof_binding_required",
            false,
            serde_json::json!({"tombstone_plan_proof_binding_requested": true}),
        ),
        proof_fixture(
            "minimal-scoped-memory-real-write-canary-rollback-target-proof-required",
            "blocked_rollback_target_noop",
            "rollback_target_proof_binding_required",
            false,
            serde_json::json!({"rollback_target_proof_binding_requested": true}),
        ),
        proof_fixture(
            "minimal-scoped-memory-real-write-canary-tombstone-target-proof-required",
            "blocked_tombstone_target_noop",
            "tombstone_target_proof_binding_required",
            false,
            serde_json::json!({"tombstone_target_proof_binding_requested": true}),
        ),
        proof_fixture(
            "minimal-scoped-memory-real-write-canary-rollback-receipt-proof-required",
            "blocked_rollback_receipt_noop",
            "rollback_receipt_linkage_proof_binding_required",
            false,
            serde_json::json!({"rollback_receipt_linkage_proof_binding_requested": true}),
        ),
        proof_fixture(
            "minimal-scoped-memory-real-write-canary-tombstone-receipt-proof-required",
            "blocked_tombstone_receipt_noop",
            "tombstone_receipt_linkage_proof_binding_required",
            false,
            serde_json::json!({"tombstone_receipt_linkage_proof_binding_requested": true}),
        ),
        proof_fixture(
            "minimal-scoped-memory-real-write-canary-rollback-tombstone-guards-audit-handoff-required",
            "blocked_guards_audit_handoff_noop",
            "rollback_tombstone_guards_audit_and_handoff_proof_required",
            false,
            serde_json::json!({
                "rollback_idempotency_guard_proof_binding_requested": true,
                "tombstone_idempotency_guard_proof_binding_requested": true,
                "rollback_tombstone_audit_evidence_proof_binding_requested": true,
                "operator_review_handoff_proof_binding_requested": true,
                "minimal_real_write_canary_handoff_proof_binding_requested": true
            }),
        ),
        proof_fixture(
            "minimal-scoped-memory-real-write-canary-rollback-tombstone-proof-direct-side-effect-attempt",
            "blocked_direct_side_effect_noop",
            "direct_rollback_tombstone_memory_and_external_side_effects_denied",
            false,
            serde_json::json!({
                "single_use_nonce_consumption_requested": true,
                "explicit_command_dispatch_requested": true,
                "wal_write_requested": true,
                "receipt_persistence_requested": true,
                "post_write_readback_execution_requested": true,
                "readback_result_recording_requested": true,
                "readback_result_persistence_requested": true,
                "readback_acceptance_requested": true,
                "rollback_execution_requested": true,
                "rollback_result_recording_requested": true,
                "rollback_result_persistence_requested": true,
                "rollback_result_acceptance_requested": true,
                "tombstone_write_requested": true,
                "compensating_memory_write_requested": true,
                "durable_memory_read_requested": true,
                "durable_memory_write_requested": true,
                "durable_memory_rollback_requested": true,
                "kg_live_write_requested": true,
                "provider_model_invocation_requested": true,
                "credential_read_requested": true,
                "channel_external_send_requested": true,
                "release_artifact_write_requested": true,
                "install_restart_requested": true,
                "active_binary_mutation_requested": true
            }),
        ),
    ]);

    let fixture_count = fixtures.as_array().map(std::vec::Vec::len).unwrap_or(0);
    let accepted_fixture_count = fixtures
        .as_array()
        .map(|items| {
            items
                .iter()
                .filter(|fixture| {
                    fixture
                        .get("minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_accepted")
                        .and_then(serde_json::Value::as_bool)
                        == Some(true)
                })
                .count()
        })
        .unwrap_or(0);
    let blocked_fixture_count = fixture_count.saturating_sub(accepted_fixture_count);
    let denials = PROOF_DENIALS
        .iter()
        .map(|reason| serde_json::json!(reason))
        .collect::<Vec<_>>();
    let denied_count = denials.len();
    let source_report_sha256 = sha256_text_value(&source.to_string());
    let boundary_hash_sha256 = sha256_text_value(&format!(
        "minimal-scoped-memory-real-write-canary-rollback-tombstone-proof-boundary:v1:source-ready={source_ready}:fixtures={fixture_count}:accepted={accepted_fixture_count}:denials={denied_count}:rollback=0:tombstone=0:memory-writes=0"
    ));
    let policy_hash_sha256 = sha256_text_value(
        "minimal-scoped-memory-real-write-canary-rollback-tombstone-proof-policy:v1:no-nonce-consume:no-command-dispatch:no-wal-write:no-receipt-persist:no-memory-read-write:no-rollback:no-tombstone",
    );
    let mut side_effects = serde_json::Map::new();
    for &key in FALSE_KEYS {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }
    let required_fields = serde_json::json!([
        "source_minimal_scoped_memory_real_write_canary_post_write_readback_binding_report_sha256",
        "post_write_readback_binding_fixture_id",
        "rollback_plan_proof_id",
        "tombstone_plan_proof_id",
        "rollback_target_proof_id",
        "tombstone_target_proof_id",
        "rollback_receipt_id",
        "tombstone_receipt_id",
        "rollback_idempotency_guard_id",
        "tombstone_idempotency_guard_id",
        "rollback_tombstone_audit_evidence_id",
        "operator_review_handoff_id",
        "minimal_real_write_canary_handoff_id",
        "active_binary_sha256",
        "route_count",
    ]);
    let allowed_next_actions = serde_json::json!([
        {
            "action": "run_minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_boundary_require_live_gate",
            "status": "allowed_verification_only",
            "reads_memory": false,
            "writes_memory": false,
            "consumes_nonce": false,
            "dispatches_command": false,
            "writes_wal": false,
            "persists_receipt": false,
            "records_readback": false,
            "executes_rollback": false,
            "writes_tombstone": false,
            "writes_kg": false,
            "invokes_provider": false,
            "reads_credentials": false,
            "sends_externally": false,
            "publishes_artifacts": false,
            "installs_or_restarts": false,
            "mutates_active_binary": false
        },
        {
            "action": "prepare_minimal_scoped_memory_real_write_canary_execution_boundary",
            "status": "allowed_report_only_next_slice",
            "requires_minimal_scoped_memory_real_write_canary_rollback_tombstone_proof": true,
            "writes_memory": false,
            "reads_memory": false,
            "consumes_nonce": false,
            "dispatches_command": false,
            "writes_wal": false,
            "persists_receipt": false,
            "records_readback": false,
            "executes_rollback": false,
            "writes_tombstone": false,
            "writes_kg": false,
            "invokes_provider": false,
            "sends_externally": false
        }
    ]);
    let report_ready = route_count_source_command_accepted
        && source_ready
        && fixture_count == 10
        && accepted_fixture_count == 1
        && blocked_fixture_count == 9
        && denied_count == 36;

    let mut report = serde_json::Map::new();
    macro_rules! insert_report_json {
        ($key:literal, $value:expr) => {
            report.insert($key.to_string(), serde_json::json!($value));
        };
    }
    insert_report_json!("product", "Hepta");
    insert_report_json!("runtime", "hepta");
    insert_report_json!("status", if report_ready { "ready" } else { "blocked" });
    insert_report_json!("base_url", "http://127.0.0.1:7373");
    insert_report_json!(
        "gate",
        "hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_boundary_route"
    );
    insert_report_json!(
        "endpoint",
        HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_MINIMAL_SCOPED_MEMORY_REAL_WRITE_CANARY_ROLLBACK_TOMBSTONE_PROOF_BOUNDARY_ENDPOINT
    );
    insert_report_json!(
        "source_command",
        "/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-rollback-tombstone-proof-boundary --json"
    );
    insert_report_json!("native_route", true);
    insert_report_json!("side_effect_free", true);
    insert_report_json!("audit_date", "2026-07-04");
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_schema_version",
        "minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_boundary_v1"
    );
    insert_report_json!(
        "scoped_memory_real_write_canary_mode",
        "minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_no_rollback_or_write"
    );
    insert_report_json!(
        "native_gateway_source_command_count",
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    insert_report_json!("route_count", route_matrix.route_count);
    insert_report_json!(
        "implemented_route_count",
        route_matrix.implemented_route_count
    );
    insert_report_json!("missing_route_count", route_matrix.missing_route_count);
    insert_report_json!(
        "route_count_source_command_accepted",
        route_count_source_command_accepted
    );
    insert_report_json!(
        "memory_write_execution_minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_boundary_ready",
        report_ready
    );
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_ready",
        report_ready
    );
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_accepted_no_rollback_or_write",
        report_ready
    );
    insert_report_json!(
        "source_minimal_scoped_memory_real_write_canary_post_write_readback_binding_boundary_ready",
        source_ready
    );
    insert_report_json!(
        "source_minimal_scoped_memory_real_write_canary_post_write_readback_binding_ready",
        json_bool(
            &source,
            "minimal_scoped_memory_real_write_canary_post_write_readback_binding_ready"
        )
    );
    insert_report_json!(
        "source_minimal_scoped_memory_real_write_canary_post_write_readback_binding_report_sha256",
        source_report_sha256
    );
    insert_report_json!(
        "source_accepted_minimal_scoped_memory_real_write_canary_post_write_readback_binding_fixture_count",
        json_u64(
            &source,
            "accepted_minimal_scoped_memory_real_write_canary_post_write_readback_binding_fixture_count"
        )
    );
    insert_report_json!(
        "source_blocked_minimal_scoped_memory_real_write_canary_post_write_readback_binding_fixture_count",
        json_u64(
            &source,
            "blocked_minimal_scoped_memory_real_write_canary_post_write_readback_binding_fixture_count"
        )
    );
    insert_report_json!(
        "source_post_write_readback_binding_authority_accepted_count",
        json_u64(
            &source,
            "post_write_readback_binding_authority_accepted_count"
        )
    );
    insert_report_json!(
        "source_rollback_tombstone_handoff_bound_count",
        json_u64(&source, "rollback_tombstone_handoff_bound_count")
    );
    insert_report_json!(
        "source_denied_by_minimal_scoped_memory_real_write_canary_post_write_readback_binding_boundary_count",
        json_u64(
            &source,
            "denied_by_minimal_scoped_memory_real_write_canary_post_write_readback_binding_boundary_count"
        )
    );
    for key in [
        "single_use_nonce_consumed_count",
        "explicit_command_dispatched_count",
        "wal_write_performed_count",
        "receipt_persisted_count",
        "post_write_readback_performed_count",
        "readback_result_recorded_count",
        "readback_result_persisted_count",
        "readback_result_accepted_count",
        "rollback_performed_count",
        "tombstone_written_count",
        "durable_memory_store_read_performed_count",
        "durable_memory_store_write_performed_count",
        "durable_memory_store_rollback_performed_count",
        "memory_store_write_performed_count",
    ] {
        report.insert(
            format!("source_{key}"),
            serde_json::json!(json_u64(&source, key)),
        );
    }
    insert_report_json!("minimum_required_samples", 24);
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_boundary_hash_sha256",
        boundary_hash_sha256
    );
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_policy_hash_sha256",
        policy_hash_sha256
    );
    insert_report_json!(
        "required_minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_surface_count",
        12
    );
    insert_report_json!(
        "ready_minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_surface_count",
        12
    );
    insert_report_json!(
        "side_effect_free_minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_surface_count",
        12
    );
    insert_report_json!(
        "required_minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_fixture_count",
        10
    );
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_fixture_count",
        fixture_count
    );
    insert_report_json!(
        "accepted_minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_fixture_count",
        accepted_fixture_count
    );
    insert_report_json!(
        "blocked_minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_fixture_count",
        blocked_fixture_count
    );
    insert_report_json!(
        "noop_minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_fixture_count",
        fixture_count
    );
    for key in [
        "rollback_tombstone_proof_authority_accepted_count",
        "rollback_plan_proof_bound_count",
        "tombstone_plan_proof_bound_count",
        "rollback_target_proof_bound_count",
        "tombstone_target_proof_bound_count",
        "rollback_receipt_linkage_proof_bound_count",
        "tombstone_receipt_linkage_proof_bound_count",
        "rollback_idempotency_guard_proof_bound_count",
        "tombstone_idempotency_guard_proof_bound_count",
        "rollback_tombstone_audit_evidence_proof_bound_count",
        "operator_review_handoff_proof_bound_count",
        "minimal_real_write_canary_handoff_proof_bound_count",
        "minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_accepted_count",
    ] {
        report.insert(key.to_string(), serde_json::json!(1));
    }
    for key in [
        "single_use_nonce_consumed_count",
        "explicit_command_dispatched_count",
        "wal_write_performed_count",
        "wal_recorded_count",
        "wal_persisted_count",
        "receipt_recorded_count",
        "receipt_persisted_count",
        "receipt_materialized_count",
        "receipt_delivered_count",
        "post_write_readback_performed_count",
        "readback_result_recorded_count",
        "readback_result_persisted_count",
        "readback_result_accepted_count",
        "rollback_performed_count",
        "rollback_result_recorded_count",
        "rollback_result_persisted_count",
        "rollback_result_accepted_count",
        "tombstone_written_count",
        "compensating_memory_write_performed_count",
        "durable_memory_store_read_performed_count",
        "durable_memory_store_rollback_performed_count",
        "durable_memory_store_write_performed_count",
        "memory_store_write_performed_count",
    ] {
        report.insert(key.to_string(), serde_json::json!(0));
    }
    insert_report_json!(
        "required_before_minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_count",
        15
    );
    report.insert(
        "required_minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_fields"
            .to_string(),
        required_fields,
    );
    report.insert(
        "minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_surfaces".to_string(),
        serde_json::json!(PROOF_SURFACES),
    );
    report.insert(
        "minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_fixtures".to_string(),
        fixtures,
    );
    report.insert(
        "denied_by_minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_boundary"
            .to_string(),
        serde_json::Value::Array(denials),
    );
    insert_report_json!(
        "denied_by_minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_boundary_count",
        denied_count
    );
    for key in [
        "source_minimal_scoped_memory_real_write_canary_post_write_readback_binding_required",
        "minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_accepted",
        "rollback_plan_proof_bound",
        "tombstone_plan_proof_bound",
        "rollback_target_proof_bound",
        "tombstone_target_proof_bound",
        "rollback_receipt_linkage_proof_bound",
        "tombstone_receipt_linkage_proof_bound",
        "rollback_idempotency_guard_proof_bound",
        "tombstone_idempotency_guard_proof_bound",
        "rollback_tombstone_audit_evidence_proof_bound",
        "operator_review_handoff_proof_bound",
        "minimal_real_write_canary_handoff_proof_bound",
        "nonce_consumption_forbidden_on_report_route",
        "explicit_command_dispatch_forbidden_on_report_route",
        "wal_write_forbidden",
        "wal_persistence_forbidden",
        "receipt_recording_forbidden",
        "receipt_persistence_forbidden",
        "receipt_materialization_forbidden",
        "receipt_delivery_forbidden",
        "post_write_readback_forbidden_on_report_route",
        "readback_result_recording_forbidden",
        "readback_result_persistence_forbidden",
        "readback_acceptance_forbidden",
        "rollback_execution_forbidden",
        "tombstone_write_forbidden",
        "durable_memory_read_forbidden",
        "durable_memory_write_forbidden",
        "durable_memory_rollback_forbidden",
        "memory_store_mutation_forbidden",
        "kg_live_write_forbidden",
        "provider_model_invocation_forbidden",
        "credential_channel_public_release_forbidden",
        "install_restart_active_binary_filesystem_mutation_forbidden",
    ] {
        report.insert(key.to_string(), serde_json::json!(true));
    }
    report.insert("allowed_next_actions".to_string(), allowed_next_actions);
    for &key in FALSE_KEYS {
        report.insert(key.to_string(), serde_json::json!(false));
    }
    report.insert(
        "side_effects".to_string(),
        serde_json::Value::Object(side_effects),
    );
    serde_json::Value::Object(report)
}

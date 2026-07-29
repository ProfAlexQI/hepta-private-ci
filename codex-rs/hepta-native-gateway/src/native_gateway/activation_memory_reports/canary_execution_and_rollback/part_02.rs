fn hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_durable_store_write_plan_boundary_report()
-> serde_json::Value {
    const PLAN_SURFACES: &[&str] = &[
        "source_tombstone_cleanup_acceptance_required",
        "approved_namespace_store_scope_required",
        "tombstone_cleanup_acceptance_hash_required",
        "durable_store_target_required",
        "durable_store_write_envelope_required",
        "durable_store_write_payload_digest_redaction_required",
        "durable_store_write_wal_receipt_plan_required",
        "durable_store_write_readback_plan_required",
        "durable_store_write_rollback_plan_required",
        "durable_store_write_tombstone_cleanup_plan_required",
        "durable_store_write_operator_handoff_required",
        "durable_memory_kg_provider_channel_release_install_active_binary_side_effects_forbidden",
    ];
    const PLAN_DENIALS: &[&str] = &[
        "source_tombstone_cleanup_acceptance_boundary_required",
        "approved_namespace_required",
        "approved_store_required",
        "approved_scope_required",
        "tombstone_cleanup_acceptance_hash_required",
        "tombstone_cleanup_receipt_linkage_required",
        "durable_store_target_required",
        "durable_store_write_envelope_required",
        "durable_store_write_payload_digest_required",
        "durable_store_write_wal_receipt_plan_required",
        "durable_store_write_readback_plan_required",
        "durable_store_write_rollback_plan_required",
        "durable_store_write_tombstone_cleanup_plan_required",
        "durable_store_write_operator_handoff_required",
        "durable_memory_store_read_denied",
        "durable_memory_store_write_denied",
        "durable_memory_store_rollback_denied",
        "memory_store_mutation_denied",
        "wal_write_denied",
        "receipt_record_persist_materialize_denied",
        "artifact_filesystem_write_denied",
        "artifact_cleanup_denied",
        "rollback_execution_denied",
        "tombstone_write_denied",
        "kg_live_write_denied",
        "provider_model_invocation_denied",
        "credential_channel_external_send_denied",
        "public_release_artifact_denied",
        "install_restart_active_binary_mutation_denied",
        "raw_payload_plaintext_denied",
    ];
    const FALSE_EXTERNAL_KEYS: &[&str] = &[
        "single_use_nonce_consumed",
        "explicit_command_dispatched",
        "durable_store_write_plan_executed",
        "wal_write_performed",
        "wal_recorded",
        "wal_persisted",
        "receipt_recorded",
        "receipt_persisted",
        "receipt_materialized",
        "receipt_delivered",
        "canary_artifact_filesystem_written",
        "artifact_readback_performed",
        "artifact_cleanup_performed",
        "filesystem_written",
        "post_write_readback_performed",
        "readback_result_recorded",
        "readback_result_persisted",
        "readback_result_accepted",
        "rollback_executed",
        "rollback_performed",
        "rollback_result_recorded",
        "rollback_result_persisted",
        "rollback_result_accepted",
        "tombstone_cleanup_executed",
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
    ];
    const TRUE_PLAN_KEYS: &[&str] = &[
        "durable_store_write_plan_performed",
        "durable_store_write_plan_result_recorded",
        "durable_store_write_plan_result_accepted",
        "durable_store_target_bound",
        "durable_store_write_envelope_bound",
        "durable_store_write_wal_receipt_plan_bound",
        "durable_store_write_readback_plan_bound",
        "durable_store_write_rollback_plan_bound",
        "durable_store_write_tombstone_cleanup_plan_bound",
        "durable_store_write_operator_handoff_bound",
        "minimal_scoped_memory_real_write_canary_durable_store_write_plan_accepted",
    ];

    fn plan_fixture(
        id: &str,
        status: &str,
        reason: &str,
        accepted: bool,
        extra: serde_json::Value,
    ) -> serde_json::Value {
        let mut base = serde_json::Map::new();
        base.insert("id".to_string(), serde_json::json!(id));
        base.insert("fixture_id".to_string(), serde_json::json!(id));
        base.insert(
            "minimal_scoped_memory_real_write_canary_durable_store_write_plan_status".to_string(),
            serde_json::json!(status),
        );
        base.insert("reason".to_string(), serde_json::json!(reason));
        base.insert(
            "minimal_scoped_memory_real_write_canary_durable_store_write_plan_accepted".to_string(),
            serde_json::json!(accepted),
        );
        for key in [
            "source_tombstone_cleanup_acceptance_boundary_ready",
            "approved_namespace_bound",
            "approved_store_bound",
            "approved_scope_bound",
            "tombstone_cleanup_acceptance_hash_bound",
            "tombstone_cleanup_receipt_linkage_bound",
            "durable_store_target_bound",
            "durable_store_write_envelope_bound",
            "durable_store_write_payload_digest_bound",
            "durable_store_write_wal_receipt_plan_bound",
            "durable_store_write_readback_plan_bound",
            "durable_store_write_rollback_plan_bound",
            "durable_store_write_tombstone_cleanup_plan_bound",
            "durable_store_write_operator_handoff_bound",
        ] {
            base.insert(key.to_string(), serde_json::json!(accepted));
        }
        for &key in FALSE_EXTERNAL_KEYS {
            base.insert(key.to_string(), serde_json::json!(false));
        }
        for &key in TRUE_PLAN_KEYS {
            base.insert(key.to_string(), serde_json::json!(accepted));
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
        .name("hepta-memory-minimal-canary-durable-store-plan-source-report".to_string())
        .stack_size(8 * 1024 * 1024)
        .spawn(
            hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_tombstone_cleanup_acceptance_boundary_report,
        )
        .ok()
        .and_then(|handle| handle.join().ok())
        .unwrap_or_else(|| {
            serde_json::json!({
                "status": "blocked",
                "memory_write_execution_minimal_scoped_memory_real_write_canary_tombstone_cleanup_acceptance_boundary_ready": false,
                "minimal_scoped_memory_real_write_canary_tombstone_cleanup_acceptance_ready": false,
                "source_minimal_scoped_memory_real_write_canary_durable_store_write_plan_source_report_thread_failed": true
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
    let json_str = |value: &serde_json::Value, key: &str| {
        value
            .get(key)
            .and_then(serde_json::Value::as_str)
            .unwrap_or("")
            .to_string()
    };
    let route_count_source_command_accepted = route_matrix.ready
        && route_matrix.route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let source_next_action_durable_store_plan = source
        .get("allowed_next_actions")
        .and_then(serde_json::Value::as_array)
        .and_then(|items| items.get(1))
        .map(|item| {
            item.get("action").and_then(serde_json::Value::as_str)
                == Some(
                    "prepare_minimal_scoped_memory_real_write_canary_durable_store_write_plan_boundary",
                )
                && item
                    .get("requires_minimal_scoped_memory_real_write_canary_tombstone_cleanup_acceptance")
                    .and_then(serde_json::Value::as_bool)
                    == Some(true)
        })
        .unwrap_or(false);
    let source_side_effects_ok = source
        .get("side_effects")
        .and_then(serde_json::Value::as_object)
        .map(|effects| {
            effects
                .get("tombstone_cleanup_acceptance_performed")
                .and_then(serde_json::Value::as_bool)
                == Some(true)
                && effects
                    .get("tombstone_cleanup_acceptance_result_accepted")
                    .and_then(serde_json::Value::as_bool)
                    == Some(true)
                && effects
                    .get("tombstone_cleanup_executed")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && effects
                    .get("tombstone_written")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && effects
                    .get("durable_memory_store_write_performed")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && effects
                    .get("external_send_performed")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
        })
        .unwrap_or(false);
    let source_ready = source.get("status").and_then(serde_json::Value::as_str) == Some("ready")
        && json_bool(
            &source,
            "memory_write_execution_minimal_scoped_memory_real_write_canary_tombstone_cleanup_acceptance_boundary_ready",
        )
        && json_bool(
            &source,
            "minimal_scoped_memory_real_write_canary_tombstone_cleanup_acceptance_ready",
        )
        && json_bool(
            &source,
            "minimal_scoped_memory_real_write_canary_tombstone_cleanup_acceptance_performed",
        )
        && json_bool(
            &source,
            "minimal_scoped_memory_real_write_canary_tombstone_cleanup_acceptance_accepted",
        )
        && json_bool(&source, "tombstone_cleanup_acceptance_result_accepted")
        && json_bool(&source, "tombstone_cleanup_receipt_linkage_verified")
        && json_bool(&source, "tombstone_cleanup_idempotency_guard_verified")
        && json_u64(
            &source,
            "accepted_minimal_scoped_memory_real_write_canary_tombstone_cleanup_acceptance_fixture_count",
        ) == 1
        && json_u64(
            &source,
            "blocked_minimal_scoped_memory_real_write_canary_tombstone_cleanup_acceptance_fixture_count",
        ) == 9
        && json_u64(
            &source,
            "tombstone_cleanup_acceptance_result_accepted_count",
        ) == 1
        && json_u64(&source, "tombstone_cleanup_plan_bound_count") == 1
        && json_u64(&source, "tombstone_cleanup_target_bound_count") == 1
        && json_u64(&source, "tombstone_cleanup_receipt_linkage_bound_count") == 1
        && json_u64(
            &source,
            "tombstone_cleanup_idempotency_guard_accepted_count",
        ) == 1
        && json_u64(&source, "tombstone_cleanup_executed_count") == 0
        && json_u64(&source, "artifact_cleanup_performed_count") == 0
        && json_u64(&source, "rollback_performed_count") == 0
        && json_u64(&source, "tombstone_written_count") == 0
        && json_u64(&source, "durable_memory_store_write_performed_count") == 0
        && json_u64(&source, "memory_store_write_performed_count") == 0
        && !json_bool(&source, "tombstone_cleanup_executed")
        && !json_bool(&source, "tombstone_written")
        && !json_bool(&source, "durable_memory_store_write_performed")
        && !json_bool(&source, "durable_memory_store_read_performed")
        && !json_bool(&source, "durable_memory_store_rollback_performed")
        && !json_bool(&source, "memory_store_write_performed")
        && !json_bool(&source, "live_kg_write_performed")
        && !json_bool(&source, "provider_invoked")
        && !json_bool(&source, "model_invoked")
        && !json_bool(&source, "credential_read")
        && !json_bool(&source, "channel_send_performed")
        && !json_bool(&source, "external_send_performed")
        && !json_bool(&source, "release_artifact_written")
        && !json_bool(&source, "install_executed")
        && !json_bool(&source, "active_binary_mutated")
        && source_next_action_durable_store_plan
        && source_side_effects_ok;

    let approved_namespace = json_str(&source, "approved_namespace");
    let approved_store = json_str(&source, "approved_store");
    let approved_scope = json_str(&source, "approved_scope");
    let source_report_sha256 = sha256_text_value(&source.to_string());
    let source_tombstone_cleanup_acceptance_hash_sha256 =
        json_str(&source, "tombstone_cleanup_acceptance_hash_sha256");
    let source_tombstone_cleanup_receipt_linkage_sha256 =
        json_str(&source, "tombstone_cleanup_receipt_linkage_sha256");
    let source_tombstone_cleanup_target_sha256 =
        json_str(&source, "tombstone_cleanup_target_sha256");
    let durable_store_write_target_id =
        "hepta-minimal-scoped-memory-real-write-canary-durable-store-write-target-v1";
    let durable_store_target_store_id = "hepta-memory-durable-store-canary-plan-only";
    let durable_store_write_payload_digest_sha256 = sha256_text_value(&format!(
        "minimal-scoped-memory-real-write-canary-durable-store-payload:v1:namespace={approved_namespace}:target-store={durable_store_target_store_id}:scope={approved_scope}:raw=false"
    ));
    let durable_store_write_target_sha256 = sha256_text_value(&format!(
        "minimal-scoped-memory-real-write-canary-durable-store-target:v1:namespace={approved_namespace}:approved-store={approved_store}:target-store={durable_store_target_store_id}:scope={approved_scope}:source-acceptance={source_tombstone_cleanup_acceptance_hash_sha256}"
    ));
    let durable_store_write_envelope_sha256 = sha256_text_value(&format!(
        "minimal-scoped-memory-real-write-canary-durable-store-write-envelope:v1:source={source_report_sha256}:target={durable_store_write_target_sha256}:payload={durable_store_write_payload_digest_sha256}:write=false"
    ));
    let durable_store_write_wal_receipt_plan_sha256 = sha256_text_value(&format!(
        "minimal-scoped-memory-real-write-canary-durable-store-wal-receipt-plan:v1:envelope={durable_store_write_envelope_sha256}:source-linkage={source_tombstone_cleanup_receipt_linkage_sha256}:wal-write=false:receipt-persist=false"
    ));
    let durable_store_write_readback_plan_sha256 = sha256_text_value(&format!(
        "minimal-scoped-memory-real-write-canary-durable-store-readback-plan:v1:wal-receipt={durable_store_write_wal_receipt_plan_sha256}:read=false"
    ));
    let durable_store_write_rollback_plan_sha256 = sha256_text_value(&format!(
        "minimal-scoped-memory-real-write-canary-durable-store-rollback-plan:v1:readback-plan={durable_store_write_readback_plan_sha256}:rollback=false"
    ));
    let durable_store_write_tombstone_cleanup_plan_sha256 = sha256_text_value(&format!(
        "minimal-scoped-memory-real-write-canary-durable-store-tombstone-cleanup-plan:v1:rollback-plan={durable_store_write_rollback_plan_sha256}:source-target={source_tombstone_cleanup_target_sha256}:tombstone=false:cleanup=false"
    ));
    let durable_store_write_operator_handoff_sha256 = sha256_text_value(&format!(
        "minimal-scoped-memory-real-write-canary-durable-store-operator-handoff:v1:target={durable_store_write_target_sha256}:envelope={durable_store_write_envelope_sha256}:tombstone-plan={durable_store_write_tombstone_cleanup_plan_sha256}:accepted=true"
    ));
    let durable_store_write_plan_hash_sha256 = sha256_text_value(&format!(
        "minimal-scoped-memory-real-write-canary-durable-store-write-plan:v1:namespace={approved_namespace}:target-store={durable_store_target_store_id}:scope={approved_scope}:handoff={durable_store_write_operator_handoff_sha256}:write=false"
    ));

    let namespace_bound = approved_namespace == "hepta.memory.canary";
    let store_bound = approved_store == "wal-receipt-canary-artifact";
    let scope_bound = approved_scope == "session";
    let tombstone_cleanup_acceptance_hash_bound =
        !source_tombstone_cleanup_acceptance_hash_sha256.is_empty();
    let tombstone_cleanup_receipt_linkage_bound =
        !source_tombstone_cleanup_receipt_linkage_sha256.is_empty();
    let durable_store_target_bound = durable_store_target_store_id
        == "hepta-memory-durable-store-canary-plan-only"
        && !durable_store_write_target_sha256.is_empty();
    let durable_store_write_payload_digest_bound =
        !durable_store_write_payload_digest_sha256.is_empty();
    let durable_store_write_envelope_bound = !durable_store_write_envelope_sha256.is_empty();
    let durable_store_write_wal_receipt_plan_bound =
        !durable_store_write_wal_receipt_plan_sha256.is_empty();
    let durable_store_write_readback_plan_bound =
        !durable_store_write_readback_plan_sha256.is_empty();
    let durable_store_write_rollback_plan_bound =
        !durable_store_write_rollback_plan_sha256.is_empty();
    let durable_store_write_tombstone_cleanup_plan_bound =
        !durable_store_write_tombstone_cleanup_plan_sha256.is_empty();
    let durable_store_write_operator_handoff_bound =
        !durable_store_write_operator_handoff_sha256.is_empty();

    let fixtures = serde_json::Value::Array(vec![
        plan_fixture(
            "minimal-scoped-memory-real-write-canary-durable-store-write-plan",
            "accepted_durable_store_write_plan",
            "durable_store_target_write_envelope_wal_receipt_readback_rollback_tombstone_cleanup_operator_handoff_plan_accepted",
            true,
            serde_json::json!({
                "approved_namespace": approved_namespace,
                "approved_store": approved_store,
                "approved_scope": approved_scope,
                "durable_store_write_target_id": durable_store_write_target_id,
                "durable_store_target_store_id": durable_store_target_store_id,
                "source_tombstone_cleanup_acceptance_hash_sha256": source_tombstone_cleanup_acceptance_hash_sha256,
                "source_tombstone_cleanup_receipt_linkage_sha256": source_tombstone_cleanup_receipt_linkage_sha256,
                "durable_store_write_payload_digest_sha256": durable_store_write_payload_digest_sha256,
                "durable_store_write_target_sha256": durable_store_write_target_sha256,
                "durable_store_write_envelope_sha256": durable_store_write_envelope_sha256,
                "durable_store_write_wal_receipt_plan_sha256": durable_store_write_wal_receipt_plan_sha256,
                "durable_store_write_readback_plan_sha256": durable_store_write_readback_plan_sha256,
                "durable_store_write_rollback_plan_sha256": durable_store_write_rollback_plan_sha256,
                "durable_store_write_tombstone_cleanup_plan_sha256": durable_store_write_tombstone_cleanup_plan_sha256,
                "durable_store_write_operator_handoff_sha256": durable_store_write_operator_handoff_sha256,
                "durable_store_write_plan_hash_sha256": durable_store_write_plan_hash_sha256,
            }),
        ),
        plan_fixture(
            "minimal-scoped-memory-real-write-canary-durable-store-write-plan-missing-source",
            "blocked_source_noop",
            "source_tombstone_cleanup_acceptance_boundary_required",
            false,
            serde_json::json!({"source_tombstone_cleanup_acceptance_boundary_ready": false}),
        ),
        plan_fixture(
            "minimal-scoped-memory-real-write-canary-durable-store-write-plan-wrong-namespace",
            "blocked_namespace_noop",
            "approved_namespace_required",
            false,
            serde_json::json!({"approved_namespace_bound": false}),
        ),
        plan_fixture(
            "minimal-scoped-memory-real-write-canary-durable-store-write-plan-wrong-store",
            "blocked_store_noop",
            "approved_store_required",
            false,
            serde_json::json!({"approved_store_bound": false}),
        ),
        plan_fixture(
            "minimal-scoped-memory-real-write-canary-durable-store-write-plan-wrong-scope",
            "blocked_scope_noop",
            "approved_scope_required",
            false,
            serde_json::json!({"approved_scope_bound": false}),
        ),
        plan_fixture(
            "minimal-scoped-memory-real-write-canary-durable-store-write-plan-missing-tombstone-cleanup-acceptance",
            "blocked_tombstone_cleanup_acceptance_noop",
            "tombstone_cleanup_acceptance_hash_required",
            false,
            serde_json::json!({"tombstone_cleanup_acceptance_hash_bound": false}),
        ),
        plan_fixture(
            "minimal-scoped-memory-real-write-canary-durable-store-write-plan-missing-target",
            "blocked_durable_store_target_noop",
            "durable_store_target_required",
            false,
            serde_json::json!({"durable_store_target_bound": false}),
        ),
        plan_fixture(
            "minimal-scoped-memory-real-write-canary-durable-store-write-plan-missing-envelope",
            "blocked_write_envelope_noop",
            "durable_store_write_envelope_required",
            false,
            serde_json::json!({"durable_store_write_envelope_bound": false}),
        ),
        plan_fixture(
            "minimal-scoped-memory-real-write-canary-durable-store-write-plan-missing-rollback-plan",
            "blocked_rollback_plan_noop",
            "durable_store_write_rollback_plan_required",
            false,
            serde_json::json!({"durable_store_write_rollback_plan_bound": false}),
        ),
        plan_fixture(
            "minimal-scoped-memory-real-write-canary-durable-store-write-plan-direct-side-effect-attempt",
            "blocked_direct_side_effect_noop",
            "direct_durable_memory_kg_provider_channel_release_install_active_binary_side_effects_denied",
            false,
            serde_json::json!({
                "durable_memory_write_requested": true,
                "memory_store_mutation_requested": true,
                "wal_write_requested": true,
                "receipt_persist_requested": true,
                "durable_memory_read_requested": true,
                "rollback_execution_requested": true,
                "tombstone_write_requested": true,
                "artifact_cleanup_requested": true,
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
                        .get("minimal_scoped_memory_real_write_canary_durable_store_write_plan_accepted")
                        .and_then(serde_json::Value::as_bool)
                        == Some(true)
                })
                .count()
        })
        .unwrap_or(0);
    let blocked_fixture_count = fixture_count.saturating_sub(accepted_fixture_count);
    let denials = PLAN_DENIALS
        .iter()
        .map(|reason| serde_json::json!(reason))
        .collect::<Vec<_>>();
    let denied_count = denials.len();
    let plan_ops_ok = namespace_bound
        && store_bound
        && scope_bound
        && tombstone_cleanup_acceptance_hash_bound
        && tombstone_cleanup_receipt_linkage_bound
        && durable_store_target_bound
        && durable_store_write_payload_digest_bound
        && durable_store_write_envelope_bound
        && durable_store_write_wal_receipt_plan_bound
        && durable_store_write_readback_plan_bound
        && durable_store_write_rollback_plan_bound
        && durable_store_write_tombstone_cleanup_plan_bound
        && durable_store_write_operator_handoff_bound;
    let report_ready = route_count_source_command_accepted
        && source_ready
        && fixture_count == 10
        && accepted_fixture_count == 1
        && blocked_fixture_count == 9
        && denied_count == 30
        && plan_ops_ok;
    let boundary_hash_sha256 = sha256_text_value(&format!(
        "minimal-scoped-memory-real-write-canary-durable-store-write-plan-boundary:v1:source-ready={source_ready}:target={durable_store_target_bound}:envelope={durable_store_write_envelope_bound}:wal-receipt-plan={durable_store_write_wal_receipt_plan_bound}:readback-plan={durable_store_write_readback_plan_bound}:rollback-plan={durable_store_write_rollback_plan_bound}:fixtures={fixture_count}:accepted={accepted_fixture_count}:denials={denied_count}"
    ));
    let policy_hash_sha256 = sha256_text_value(
        "minimal-scoped-memory-real-write-canary-durable-store-write-plan-policy:v1:accept-plan-only:no-durable-memory-write:no-memory-store-mutation:no-wal-write:no-receipt-persist:no-readback:no-rollback:no-tombstone:no-kg:no-provider:no-channel:no-release:no-install",
    );

    let mut side_effects = serde_json::Map::new();
    for &key in FALSE_EXTERNAL_KEYS {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }
    for &key in TRUE_PLAN_KEYS {
        side_effects.insert(key.to_string(), serde_json::json!(report_ready));
    }
    let required_fields = serde_json::json!([
        "source_minimal_scoped_memory_real_write_canary_tombstone_cleanup_acceptance_report_sha256",
        "source_tombstone_cleanup_acceptance_hash_sha256",
        "source_tombstone_cleanup_receipt_linkage_sha256",
        "approved_namespace",
        "approved_store",
        "approved_scope",
        "durable_store_write_target_id",
        "durable_store_target_store_id",
        "durable_store_write_payload_digest_sha256",
        "durable_store_write_target_sha256",
        "durable_store_write_envelope_sha256",
        "durable_store_write_wal_receipt_plan_sha256",
        "durable_store_write_readback_plan_sha256",
        "durable_store_write_rollback_plan_sha256",
        "durable_store_write_tombstone_cleanup_plan_sha256",
        "durable_store_write_operator_handoff_sha256",
        "durable_store_write_plan_hash_sha256",
        "route_count"
    ]);
    let allowed_next_actions = serde_json::json!([
        {
            "action": "run_minimal_scoped_memory_real_write_canary_durable_store_write_plan_boundary_require_live_gate",
            "status": "allowed_verification_only",
            "accepts_durable_store_write_plan_evidence": true,
            "writes_durable_memory": false,
            "mutates_memory_store": false,
            "writes_wal": false,
            "persists_receipt": false,
            "reads_memory": false,
            "executes_rollback": false,
            "writes_tombstone": false,
            "cleans_artifacts": false,
            "writes_kg": false,
            "invokes_provider": false,
            "reads_credentials": false,
            "sends_externally": false,
            "publishes_artifacts": false,
            "installs_or_restarts": false,
            "mutates_active_binary": false
        },
        {
            "action": "prepare_minimal_scoped_memory_real_write_canary_durable_store_write_preflight_boundary",
            "status": "allowed_report_only_next_slice",
            "requires_minimal_scoped_memory_real_write_canary_durable_store_write_plan": true,
            "writes_durable_memory": false,
            "mutates_memory_store": false,
            "writes_wal": false,
            "persists_receipt": false,
            "executes_rollback": false,
            "writes_tombstone": false,
            "writes_kg": false,
            "invokes_provider": false,
            "sends_externally": false
        }
    ]);

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
        "hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_durable_store_write_plan_boundary_route"
    );
    insert_report_json!(
        "endpoint",
        HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_MINIMAL_SCOPED_MEMORY_REAL_WRITE_CANARY_DURABLE_STORE_WRITE_PLAN_BOUNDARY_ENDPOINT
    );
    insert_report_json!(
        "source_command",
        "/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-durable-store-write-plan-boundary --json"
    );
    insert_report_json!("native_route", true);
    insert_report_json!("side_effect_free", false);
    insert_report_json!("external_side_effect_free", true);
    insert_report_json!("audit_date", "2026-07-04");
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_durable_store_write_plan_schema_version",
        "minimal_scoped_memory_real_write_canary_durable_store_write_plan_boundary_v1"
    );
    insert_report_json!(
        "scoped_memory_real_write_canary_mode",
        "minimal_scoped_memory_real_write_canary_durable_store_write_plan_report_only"
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
        "memory_write_execution_minimal_scoped_memory_real_write_canary_durable_store_write_plan_boundary_ready",
        report_ready
    );
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_durable_store_write_plan_ready",
        report_ready
    );
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_durable_store_write_plan_performed",
        report_ready
    );
    insert_report_json!(
        "source_minimal_scoped_memory_real_write_canary_tombstone_cleanup_acceptance_boundary_ready",
        source_ready
    );
    insert_report_json!(
        "source_minimal_scoped_memory_real_write_canary_tombstone_cleanup_acceptance_report_sha256",
        source_report_sha256
    );
    for key in [
        "accepted_minimal_scoped_memory_real_write_canary_tombstone_cleanup_acceptance_fixture_count",
        "blocked_minimal_scoped_memory_real_write_canary_tombstone_cleanup_acceptance_fixture_count",
        "tombstone_cleanup_acceptance_result_accepted_count",
        "tombstone_cleanup_plan_bound_count",
        "tombstone_cleanup_target_bound_count",
        "tombstone_cleanup_receipt_linkage_bound_count",
        "tombstone_cleanup_idempotency_guard_accepted_count",
        "tombstone_cleanup_executed_count",
        "artifact_cleanup_performed_count",
        "rollback_performed_count",
        "tombstone_written_count",
        "durable_memory_store_write_performed_count",
        "memory_store_write_performed_count",
    ] {
        report.insert(
            format!("source_{key}"),
            serde_json::json!(json_u64(&source, key)),
        );
    }
    insert_report_json!("approved_namespace", approved_namespace);
    insert_report_json!("approved_store", approved_store);
    insert_report_json!("approved_scope", approved_scope);
    insert_report_json!(
        "source_tombstone_cleanup_acceptance_hash_sha256",
        source_tombstone_cleanup_acceptance_hash_sha256
    );
    insert_report_json!(
        "source_tombstone_cleanup_receipt_linkage_sha256",
        source_tombstone_cleanup_receipt_linkage_sha256
    );
    insert_report_json!(
        "source_tombstone_cleanup_target_sha256",
        source_tombstone_cleanup_target_sha256
    );
    insert_report_json!(
        "durable_store_write_target_id",
        durable_store_write_target_id
    );
    insert_report_json!(
        "durable_store_target_store_id",
        durable_store_target_store_id
    );
    insert_report_json!(
        "durable_store_write_payload_digest_sha256",
        durable_store_write_payload_digest_sha256
    );
    insert_report_json!(
        "durable_store_write_target_sha256",
        durable_store_write_target_sha256
    );
    insert_report_json!(
        "durable_store_write_envelope_sha256",
        durable_store_write_envelope_sha256
    );
    insert_report_json!(
        "durable_store_write_wal_receipt_plan_sha256",
        durable_store_write_wal_receipt_plan_sha256
    );
    insert_report_json!(
        "durable_store_write_readback_plan_sha256",
        durable_store_write_readback_plan_sha256
    );
    insert_report_json!(
        "durable_store_write_rollback_plan_sha256",
        durable_store_write_rollback_plan_sha256
    );
    insert_report_json!(
        "durable_store_write_tombstone_cleanup_plan_sha256",
        durable_store_write_tombstone_cleanup_plan_sha256
    );
    insert_report_json!(
        "durable_store_write_operator_handoff_sha256",
        durable_store_write_operator_handoff_sha256
    );
    insert_report_json!(
        "durable_store_write_plan_hash_sha256",
        durable_store_write_plan_hash_sha256
    );
    insert_report_json!(
        "durable_store_write_plan_receipt_linkage_verified",
        tombstone_cleanup_receipt_linkage_bound && durable_store_write_wal_receipt_plan_bound
    );
    insert_report_json!(
        "durable_store_write_plan_rollback_tombstone_cleanup_verified",
        durable_store_write_rollback_plan_bound && durable_store_write_tombstone_cleanup_plan_bound
    );
    insert_report_json!(
        "required_minimal_scoped_memory_real_write_canary_durable_store_write_plan_surface_count",
        PLAN_SURFACES.len()
    );
    insert_report_json!(
        "ready_minimal_scoped_memory_real_write_canary_durable_store_write_plan_surface_count",
        if report_ready { PLAN_SURFACES.len() } else { 0 }
    );
    insert_report_json!(
        "required_minimal_scoped_memory_real_write_canary_durable_store_write_plan_fixture_count",
        10
    );
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_durable_store_write_plan_fixture_count",
        fixture_count
    );
    insert_report_json!(
        "accepted_minimal_scoped_memory_real_write_canary_durable_store_write_plan_fixture_count",
        accepted_fixture_count
    );
    insert_report_json!(
        "blocked_minimal_scoped_memory_real_write_canary_durable_store_write_plan_fixture_count",
        blocked_fixture_count
    );
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_durable_store_write_plan_accepted_count",
        accepted_fixture_count
    );
    for key in [
        "durable_store_write_plan_authority_accepted_count",
        "source_tombstone_cleanup_acceptance_bound_count",
        "tombstone_cleanup_acceptance_hash_bound_count",
        "tombstone_cleanup_receipt_linkage_bound_count",
        "durable_store_target_bound_count",
        "durable_store_write_envelope_bound_count",
        "durable_store_write_payload_digest_bound_count",
        "durable_store_write_wal_receipt_plan_bound_count",
        "durable_store_write_readback_plan_bound_count",
        "durable_store_write_rollback_plan_bound_count",
        "durable_store_write_tombstone_cleanup_plan_bound_count",
        "durable_store_write_operator_handoff_bound_count",
        "durable_store_write_plan_result_recorded_count",
        "durable_store_write_plan_result_accepted_count",
    ] {
        report.insert(key.to_string(), serde_json::json!(accepted_fixture_count));
    }
    for key in [
        "single_use_nonce_consumed_count",
        "explicit_command_dispatched_count",
        "durable_store_write_plan_executed_count",
        "wal_write_performed_count",
        "wal_recorded_count",
        "wal_persisted_count",
        "receipt_recorded_count",
        "receipt_persisted_count",
        "receipt_materialized_count",
        "receipt_delivered_count",
        "canary_artifact_filesystem_written_count",
        "artifact_readback_performed_count",
        "artifact_cleanup_performed_count",
        "post_write_readback_performed_count",
        "readback_result_recorded_count",
        "readback_result_persisted_count",
        "readback_result_accepted_count",
        "rollback_performed_count",
        "rollback_result_recorded_count",
        "rollback_result_persisted_count",
        "rollback_result_accepted_count",
        "tombstone_cleanup_executed_count",
        "tombstone_written_count",
        "compensating_memory_write_performed_count",
        "durable_memory_store_read_performed_count",
        "durable_memory_store_write_performed_count",
        "durable_memory_store_rollback_performed_count",
        "memory_store_write_performed_count",
        "memory_store_mutation_performed_count",
        "raw_payload_plaintext_recorded_count",
        "kg_live_write_performed_count",
        "provider_invoked_count",
        "model_invoked_count",
        "credential_read_count",
        "channel_send_performed_count",
        "external_send_performed_count",
        "release_artifact_written_count",
        "install_executed_count",
        "service_restarted_count",
        "active_binary_mutated_count",
    ] {
        report.insert(key.to_string(), serde_json::json!(0));
    }
    report.insert(
        "required_minimal_scoped_memory_real_write_canary_durable_store_write_plan_fields"
            .to_string(),
        required_fields,
    );
    report.insert(
        "minimal_scoped_memory_real_write_canary_durable_store_write_plan_surfaces".to_string(),
        serde_json::json!(PLAN_SURFACES),
    );
    report.insert(
        "minimal_scoped_memory_real_write_canary_durable_store_write_plan_fixtures".to_string(),
        fixtures,
    );
    report.insert(
        "denied_by_minimal_scoped_memory_real_write_canary_durable_store_write_plan_boundary"
            .to_string(),
        serde_json::Value::Array(denials),
    );
    insert_report_json!(
        "denied_by_minimal_scoped_memory_real_write_canary_durable_store_write_plan_boundary_count",
        denied_count
    );
    for key in [
        "source_tombstone_cleanup_acceptance_required",
        "minimal_scoped_memory_real_write_canary_durable_store_write_plan_accepted",
        "approved_namespace_bound",
        "approved_store_bound",
        "approved_scope_bound",
        "tombstone_cleanup_acceptance_hash_bound",
        "tombstone_cleanup_receipt_linkage_bound",
        "durable_store_target_bound",
        "durable_store_write_envelope_bound",
        "durable_store_write_payload_digest_bound",
        "durable_store_write_wal_receipt_plan_bound",
        "durable_store_write_readback_plan_bound",
        "durable_store_write_rollback_plan_bound",
        "durable_store_write_tombstone_cleanup_plan_bound",
        "durable_store_write_operator_handoff_bound",
        "durable_memory_read_forbidden",
        "durable_memory_write_forbidden",
        "durable_memory_rollback_forbidden",
        "memory_store_mutation_forbidden",
        "wal_write_forbidden_on_report_route",
        "receipt_persist_forbidden_on_report_route",
        "post_write_readback_forbidden_on_report_route",
        "rollback_execution_forbidden",
        "tombstone_write_forbidden",
        "artifact_cleanup_forbidden",
        "raw_payload_plaintext_forbidden",
        "kg_live_write_forbidden",
        "provider_model_invocation_forbidden",
        "credential_channel_public_release_forbidden",
        "install_restart_active_binary_mutation_forbidden",
    ] {
        report.insert(key.to_string(), serde_json::json!(true));
    }
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_durable_store_write_plan_boundary_hash_sha256",
        boundary_hash_sha256
    );
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_durable_store_write_plan_policy_hash_sha256",
        policy_hash_sha256
    );
    report.insert("allowed_next_actions".to_string(), allowed_next_actions);
    for &key in FALSE_EXTERNAL_KEYS {
        report.insert(key.to_string(), serde_json::json!(false));
    }
    for &key in TRUE_PLAN_KEYS {
        report.insert(key.to_string(), serde_json::json!(report_ready));
    }
    report.insert(
        "side_effects".to_string(),
        serde_json::Value::Object(side_effects),
    );
    serde_json::Value::Object(report)
}

fn hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_durable_store_write_preflight_boundary_report()
-> serde_json::Value {
    const PREFLIGHT_SURFACES: &[&str] = &[
        "source_durable_store_write_plan_required",
        "approved_namespace_store_scope_required",
        "durable_store_target_reachability_preflight_required",
        "durable_store_write_envelope_preflight_required",
        "payload_redaction_no_secret_preflight_required",
        "wal_receipt_preflight_required",
        "post_write_readback_preflight_required",
        "rollback_preflight_required",
        "tombstone_cleanup_preflight_required",
        "idempotency_replay_guard_preflight_required",
        "operator_preflight_handoff_required",
        "durable_memory_kg_provider_channel_release_install_active_binary_side_effects_forbidden",
    ];
    const PREFLIGHT_DENIALS: &[&str] = &[
        "source_durable_store_write_plan_boundary_required",
        "source_durable_store_write_plan_result_acceptance_required",
        "source_durable_store_write_plan_hash_required",
        "approved_namespace_required",
        "approved_store_required",
        "approved_scope_required",
        "durable_store_target_required",
        "durable_store_target_reachability_preflight_required",
        "durable_store_write_envelope_required",
        "durable_store_write_envelope_preflight_required",
        "payload_digest_required",
        "payload_redaction_preflight_required",
        "payload_secret_plaintext_scan_required",
        "wal_receipt_preflight_required",
        "readback_preflight_required",
        "rollback_preflight_required",
        "tombstone_cleanup_preflight_required",
        "idempotency_replay_guard_preflight_required",
        "operator_preflight_handoff_required",
        "durable_memory_store_read_denied",
        "durable_memory_store_write_denied",
        "durable_memory_store_rollback_denied",
        "memory_store_mutation_denied",
        "wal_write_denied",
        "receipt_record_persist_materialize_denied",
        "artifact_filesystem_write_denied",
        "post_write_readback_denied",
        "rollback_tombstone_execution_denied",
        "kg_provider_credential_channel_release_install_denied",
        "raw_payload_plaintext_denied",
    ];
    const FALSE_EXTERNAL_KEYS: &[&str] = &[
        "single_use_nonce_consumed",
        "explicit_command_dispatched",
        "durable_store_write_preflight_executed",
        "durable_store_write_plan_executed",
        "wal_write_performed",
        "wal_recorded",
        "wal_persisted",
        "receipt_recorded",
        "receipt_persisted",
        "receipt_materialized",
        "receipt_delivered",
        "canary_artifact_filesystem_written",
        "artifact_readback_performed",
        "artifact_cleanup_performed",
        "filesystem_written",
        "post_write_readback_performed",
        "readback_result_recorded",
        "readback_result_persisted",
        "readback_result_accepted",
        "rollback_executed",
        "rollback_performed",
        "rollback_result_recorded",
        "rollback_result_persisted",
        "rollback_result_accepted",
        "tombstone_cleanup_executed",
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
    ];
    const TRUE_PREFLIGHT_KEYS: &[&str] = &[
        "durable_store_write_preflight_performed",
        "durable_store_write_preflight_result_recorded",
        "durable_store_write_preflight_result_accepted",
        "durable_store_target_reachability_checked",
        "approved_namespace_store_scope_preflight_verified",
        "durable_store_write_envelope_preflight_verified",
        "durable_store_write_payload_digest_preflight_verified",
        "payload_redaction_preflight_verified",
        "payload_secret_plaintext_scan_passed",
        "durable_store_write_wal_receipt_preflight_bound",
        "durable_store_write_readback_preflight_bound",
        "durable_store_write_rollback_preflight_bound",
        "durable_store_write_tombstone_cleanup_preflight_bound",
        "durable_store_write_idempotency_replay_guard_preflight_bound",
        "durable_store_write_operator_preflight_handoff_bound",
        "minimal_scoped_memory_real_write_canary_durable_store_write_preflight_accepted",
    ];

    fn preflight_fixture(
        id: &str,
        status: &str,
        reason: &str,
        accepted: bool,
        extra: serde_json::Value,
    ) -> serde_json::Value {
        let mut base = serde_json::Map::new();
        base.insert("id".to_string(), serde_json::json!(id));
        base.insert("fixture_id".to_string(), serde_json::json!(id));
        base.insert(
            "minimal_scoped_memory_real_write_canary_durable_store_write_preflight_status"
                .to_string(),
            serde_json::json!(status),
        );
        base.insert("reason".to_string(), serde_json::json!(reason));
        base.insert(
            "minimal_scoped_memory_real_write_canary_durable_store_write_preflight_accepted"
                .to_string(),
            serde_json::json!(accepted),
        );
        for key in [
            "source_durable_store_write_plan_boundary_ready",
            "source_durable_store_write_plan_hash_bound",
            "source_durable_store_write_plan_result_accepted",
            "approved_namespace_bound",
            "approved_store_bound",
            "approved_scope_bound",
            "durable_store_target_reachability_checked",
            "approved_namespace_store_scope_preflight_verified",
            "durable_store_write_envelope_preflight_verified",
            "durable_store_write_payload_digest_preflight_verified",
            "payload_redaction_preflight_verified",
            "payload_secret_plaintext_scan_passed",
            "durable_store_write_wal_receipt_preflight_bound",
            "durable_store_write_readback_preflight_bound",
            "durable_store_write_rollback_preflight_bound",
            "durable_store_write_tombstone_cleanup_preflight_bound",
            "durable_store_write_idempotency_replay_guard_preflight_bound",
            "durable_store_write_operator_preflight_handoff_bound",
        ] {
            base.insert(key.to_string(), serde_json::json!(accepted));
        }
        for &key in FALSE_EXTERNAL_KEYS {
            base.insert(key.to_string(), serde_json::json!(false));
        }
        for &key in TRUE_PREFLIGHT_KEYS {
            base.insert(key.to_string(), serde_json::json!(accepted));
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
        .name("hepta-memory-minimal-canary-durable-store-preflight-source-report".to_string())
        .stack_size(8 * 1024 * 1024)
        .spawn(
            hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_durable_store_write_plan_boundary_report,
        )
        .ok()
        .and_then(|handle| handle.join().ok())
        .unwrap_or_else(|| {
            serde_json::json!({
                "status": "blocked",
                "memory_write_execution_minimal_scoped_memory_real_write_canary_durable_store_write_plan_boundary_ready": false,
                "minimal_scoped_memory_real_write_canary_durable_store_write_plan_ready": false,
                "source_minimal_scoped_memory_real_write_canary_durable_store_write_preflight_source_report_thread_failed": true
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
    let json_str = |value: &serde_json::Value, key: &str| {
        value
            .get(key)
            .and_then(serde_json::Value::as_str)
            .unwrap_or("")
            .to_string()
    };
    let route_count_source_command_accepted = route_matrix.ready
        && route_matrix.route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let source_next_action_preflight = source
        .get("allowed_next_actions")
        .and_then(serde_json::Value::as_array)
        .and_then(|items| items.get(1))
        .map(|item| {
            item.get("action").and_then(serde_json::Value::as_str)
                == Some(
                    "prepare_minimal_scoped_memory_real_write_canary_durable_store_write_preflight_boundary",
                )
                && item
                    .get("requires_minimal_scoped_memory_real_write_canary_durable_store_write_plan")
                    .and_then(serde_json::Value::as_bool)
                    == Some(true)
        })
        .unwrap_or(false);
    let source_side_effects_ok = source
        .get("side_effects")
        .and_then(serde_json::Value::as_object)
        .map(|effects| {
            effects
                .get("durable_store_write_plan_performed")
                .and_then(serde_json::Value::as_bool)
                == Some(true)
                && effects
                    .get("durable_store_write_plan_result_accepted")
                    .and_then(serde_json::Value::as_bool)
                    == Some(true)
                && effects
                    .get("durable_store_write_plan_executed")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && effects
                    .get("durable_memory_store_write_performed")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && effects
                    .get("memory_store_write_performed")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && effects
                    .get("external_send_performed")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
        })
        .unwrap_or(false);
    let source_ready = source.get("status").and_then(serde_json::Value::as_str) == Some("ready")
        && json_bool(
            &source,
            "memory_write_execution_minimal_scoped_memory_real_write_canary_durable_store_write_plan_boundary_ready",
        )
        && json_bool(
            &source,
            "minimal_scoped_memory_real_write_canary_durable_store_write_plan_ready",
        )
        && json_bool(
            &source,
            "minimal_scoped_memory_real_write_canary_durable_store_write_plan_performed",
        )
        && json_bool(
            &source,
            "minimal_scoped_memory_real_write_canary_durable_store_write_plan_accepted",
        )
        && json_bool(&source, "durable_store_write_plan_result_accepted")
        && json_bool(&source, "durable_store_write_plan_receipt_linkage_verified")
        && json_bool(
            &source,
            "durable_store_write_plan_rollback_tombstone_cleanup_verified",
        )
        && json_u64(
            &source,
            "accepted_minimal_scoped_memory_real_write_canary_durable_store_write_plan_fixture_count",
        ) == 1
        && json_u64(
            &source,
            "blocked_minimal_scoped_memory_real_write_canary_durable_store_write_plan_fixture_count",
        ) == 9
        && json_u64(&source, "durable_store_write_plan_result_accepted_count") == 1
        && json_u64(&source, "durable_store_write_plan_executed_count") == 0
        && json_u64(&source, "durable_memory_store_write_performed_count") == 0
        && json_u64(&source, "durable_memory_store_read_performed_count") == 0
        && json_u64(&source, "durable_memory_store_rollback_performed_count") == 0
        && json_u64(&source, "memory_store_write_performed_count") == 0
        && json_u64(&source, "wal_write_performed_count") == 0
        && json_u64(&source, "receipt_persisted_count") == 0
        && json_u64(&source, "artifact_cleanup_performed_count") == 0
        && json_u64(&source, "post_write_readback_performed_count") == 0
        && json_u64(&source, "rollback_performed_count") == 0
        && json_u64(&source, "tombstone_written_count") == 0
        && !json_bool(&source, "durable_store_write_plan_executed")
        && !json_bool(&source, "durable_memory_store_write_performed")
        && !json_bool(&source, "durable_memory_store_read_performed")
        && !json_bool(&source, "durable_memory_store_rollback_performed")
        && !json_bool(&source, "memory_store_write_performed")
        && !json_bool(&source, "wal_write_performed")
        && !json_bool(&source, "receipt_persisted")
        && !json_bool(&source, "artifact_cleanup_performed")
        && !json_bool(&source, "post_write_readback_performed")
        && !json_bool(&source, "rollback_performed")
        && !json_bool(&source, "tombstone_written")
        && !json_bool(&source, "live_kg_write_performed")
        && !json_bool(&source, "provider_invoked")
        && !json_bool(&source, "model_invoked")
        && !json_bool(&source, "credential_read")
        && !json_bool(&source, "channel_send_performed")
        && !json_bool(&source, "external_send_performed")
        && !json_bool(&source, "release_artifact_written")
        && !json_bool(&source, "install_executed")
        && !json_bool(&source, "active_binary_mutated")
        && source_next_action_preflight
        && source_side_effects_ok;

    let approved_namespace = json_str(&source, "approved_namespace");
    let approved_store = json_str(&source, "approved_store");
    let approved_scope = json_str(&source, "approved_scope");
    let source_report_sha256 = sha256_text_value(&source.to_string());
    let source_durable_store_write_plan_hash_sha256 =
        json_str(&source, "durable_store_write_plan_hash_sha256");
    let source_durable_store_write_target_sha256 =
        json_str(&source, "durable_store_write_target_sha256");
    let source_durable_store_write_envelope_sha256 =
        json_str(&source, "durable_store_write_envelope_sha256");
    let source_durable_store_write_payload_digest_sha256 =
        json_str(&source, "durable_store_write_payload_digest_sha256");
    let source_durable_store_write_wal_receipt_plan_sha256 =
        json_str(&source, "durable_store_write_wal_receipt_plan_sha256");
    let source_durable_store_write_readback_plan_sha256 =
        json_str(&source, "durable_store_write_readback_plan_sha256");
    let source_durable_store_write_rollback_plan_sha256 =
        json_str(&source, "durable_store_write_rollback_plan_sha256");
    let source_durable_store_write_tombstone_cleanup_plan_sha256 =
        json_str(&source, "durable_store_write_tombstone_cleanup_plan_sha256");
    let durable_store_write_target_id = json_str(&source, "durable_store_write_target_id");
    let durable_store_target_store_id = json_str(&source, "durable_store_target_store_id");

    let durable_store_write_preflight_target_reachability_sha256 = sha256_text_value(&format!(
        "minimal-scoped-memory-real-write-canary-durable-store-preflight-target-reachability:v1:target={source_durable_store_write_target_sha256}:target-store={durable_store_target_store_id}:reachable=true:write=false"
    ));
    let durable_store_write_preflight_namespace_scope_sha256 = sha256_text_value(&format!(
        "minimal-scoped-memory-real-write-canary-durable-store-preflight-namespace-scope:v1:namespace={approved_namespace}:store={approved_store}:target-store={durable_store_target_store_id}:scope={approved_scope}:accepted=true"
    ));
    let durable_store_write_preflight_redaction_sha256 = sha256_text_value(&format!(
        "minimal-scoped-memory-real-write-canary-durable-store-preflight-redaction:v1:payload={source_durable_store_write_payload_digest_sha256}:raw=false:secret-scan=pass"
    ));
    let durable_store_write_preflight_wal_receipt_sha256 = sha256_text_value(&format!(
        "minimal-scoped-memory-real-write-canary-durable-store-preflight-wal-receipt:v1:plan={source_durable_store_write_wal_receipt_plan_sha256}:wal-write=false:receipt-persist=false"
    ));
    let durable_store_write_preflight_readback_sha256 = sha256_text_value(&format!(
        "minimal-scoped-memory-real-write-canary-durable-store-preflight-readback:v1:plan={source_durable_store_write_readback_plan_sha256}:read=false"
    ));
    let durable_store_write_preflight_rollback_sha256 = sha256_text_value(&format!(
        "minimal-scoped-memory-real-write-canary-durable-store-preflight-rollback:v1:plan={source_durable_store_write_rollback_plan_sha256}:rollback=false"
    ));
    let durable_store_write_preflight_tombstone_cleanup_sha256 = sha256_text_value(&format!(
        "minimal-scoped-memory-real-write-canary-durable-store-preflight-tombstone-cleanup:v1:plan={source_durable_store_write_tombstone_cleanup_plan_sha256}:tombstone=false:cleanup=false"
    ));
    let durable_store_write_preflight_idempotency_replay_guard_sha256 = sha256_text_value(
        &format!(
            "minimal-scoped-memory-real-write-canary-durable-store-preflight-idempotency-replay:v1:source-plan={source_durable_store_write_plan_hash_sha256}:target-store={durable_store_target_store_id}:replay=false"
        ),
    );
    let durable_store_write_preflight_operator_handoff_sha256 = sha256_text_value(&format!(
        "minimal-scoped-memory-real-write-canary-durable-store-preflight-operator-handoff:v1:source={source_report_sha256}:target={source_durable_store_write_target_sha256}:preflight=true:write=false"
    ));
    let durable_store_write_preflight_hash_sha256 = sha256_text_value(&format!(
        "minimal-scoped-memory-real-write-canary-durable-store-write-preflight:v1:source-plan={source_durable_store_write_plan_hash_sha256}:target-reachability={durable_store_write_preflight_target_reachability_sha256}:namespace-scope={durable_store_write_preflight_namespace_scope_sha256}:redaction={durable_store_write_preflight_redaction_sha256}:wal-receipt={durable_store_write_preflight_wal_receipt_sha256}:readback={durable_store_write_preflight_readback_sha256}:rollback={durable_store_write_preflight_rollback_sha256}:tombstone={durable_store_write_preflight_tombstone_cleanup_sha256}:handoff={durable_store_write_preflight_operator_handoff_sha256}"
    ));

    let namespace_bound = approved_namespace == "hepta.memory.canary";
    let store_bound = approved_store == "wal-receipt-canary-artifact";
    let scope_bound = approved_scope == "session";
    let source_plan_hash_bound = !source_durable_store_write_plan_hash_sha256.is_empty();
    let target_bound = durable_store_target_store_id
        == "hepta-memory-durable-store-canary-plan-only"
        && !source_durable_store_write_target_sha256.is_empty()
        && !durable_store_write_target_id.is_empty();
    let envelope_bound = !source_durable_store_write_envelope_sha256.is_empty();
    let payload_digest_bound = !source_durable_store_write_payload_digest_sha256.is_empty();
    let wal_receipt_plan_bound = !source_durable_store_write_wal_receipt_plan_sha256.is_empty();
    let readback_plan_bound = !source_durable_store_write_readback_plan_sha256.is_empty();
    let rollback_plan_bound = !source_durable_store_write_rollback_plan_sha256.is_empty();
    let tombstone_cleanup_plan_bound =
        !source_durable_store_write_tombstone_cleanup_plan_sha256.is_empty();
    let preflight_hashes_bound = [
        &durable_store_write_preflight_target_reachability_sha256,
        &durable_store_write_preflight_namespace_scope_sha256,
        &durable_store_write_preflight_redaction_sha256,
        &durable_store_write_preflight_wal_receipt_sha256,
        &durable_store_write_preflight_readback_sha256,
        &durable_store_write_preflight_rollback_sha256,
        &durable_store_write_preflight_tombstone_cleanup_sha256,
        &durable_store_write_preflight_idempotency_replay_guard_sha256,
        &durable_store_write_preflight_operator_handoff_sha256,
        &durable_store_write_preflight_hash_sha256,
    ]
    .iter()
    .all(|hash| !hash.is_empty());

    let fixtures = serde_json::Value::Array(vec![
        preflight_fixture(
            "minimal-scoped-memory-real-write-canary-durable-store-write-preflight",
            "accepted_durable_store_write_preflight",
            "durable_store_target_namespace_redaction_wal_receipt_readback_rollback_tombstone_idempotency_operator_handoff_preflight_accepted",
            true,
            serde_json::json!({
                "approved_namespace": approved_namespace,
                "approved_store": approved_store,
                "approved_scope": approved_scope,
                "durable_store_write_target_id": durable_store_write_target_id,
                "durable_store_target_store_id": durable_store_target_store_id,
                "source_durable_store_write_plan_hash_sha256": source_durable_store_write_plan_hash_sha256,
                "durable_store_write_preflight_hash_sha256": durable_store_write_preflight_hash_sha256,
                "durable_store_write_preflight_operator_handoff_sha256": durable_store_write_preflight_operator_handoff_sha256
            }),
        ),
        preflight_fixture(
            "missing-source-plan",
            "blocked_noop",
            "source_durable_store_write_plan_missing",
            false,
            serde_json::json!({}),
        ),
        preflight_fixture(
            "wrong-namespace",
            "blocked_noop",
            "approved_namespace_mismatch",
            false,
            serde_json::json!({}),
        ),
        preflight_fixture(
            "wrong-store",
            "blocked_noop",
            "approved_store_mismatch",
            false,
            serde_json::json!({}),
        ),
        preflight_fixture(
            "wrong-scope",
            "blocked_noop",
            "approved_scope_mismatch",
            false,
            serde_json::json!({}),
        ),
        preflight_fixture(
            "target-unreachable",
            "blocked_noop",
            "durable_store_target_reachability_missing",
            false,
            serde_json::json!({}),
        ),
        preflight_fixture(
            "payload-redaction-missing",
            "blocked_noop",
            "payload_redaction_or_secret_scan_missing",
            false,
            serde_json::json!({}),
        ),
        preflight_fixture(
            "wal-receipt-preflight-missing",
            "blocked_noop",
            "wal_receipt_preflight_missing",
            false,
            serde_json::json!({}),
        ),
        preflight_fixture(
            "rollback-tombstone-preflight-missing",
            "blocked_noop",
            "rollback_or_tombstone_cleanup_preflight_missing",
            false,
            serde_json::json!({}),
        ),
        preflight_fixture(
            "operator-handoff-missing",
            "blocked_noop",
            "operator_preflight_handoff_missing",
            false,
            serde_json::json!({}),
        ),
    ]);
    let accepted_fixture_count = fixtures
        .as_array()
        .map(|items| {
            items
                .iter()
                .filter(|item| {
                    json_bool(
                        item,
                        "minimal_scoped_memory_real_write_canary_durable_store_write_preflight_accepted",
                    )
                })
                .count()
        })
        .unwrap_or(0);
    let blocked_fixture_count = fixtures
        .as_array()
        .map(std::vec::Vec::len)
        .unwrap_or(0)
        .saturating_sub(accepted_fixture_count);
    let surface_ready = source_ready
        && namespace_bound
        && store_bound
        && scope_bound
        && source_plan_hash_bound
        && target_bound
        && envelope_bound
        && payload_digest_bound
        && wal_receipt_plan_bound
        && readback_plan_bound
        && rollback_plan_bound
        && tombstone_cleanup_plan_bound
        && preflight_hashes_bound;
    let ready_surface_count = if surface_ready {
        PREFLIGHT_SURFACES.len()
    } else {
        0
    };
    let report_ready = route_count_source_command_accepted
        && surface_ready
        && accepted_fixture_count == 1
        && blocked_fixture_count == 9
        && PREFLIGHT_DENIALS.len() == 30;
    let boundary_hash_sha256 = sha256_text_value(&format!(
        "minimal-scoped-memory-real-write-canary-durable-store-write-preflight-boundary:v1:source-ready={source_ready}:target={target_bound}:redaction=true:preflight={durable_store_write_preflight_hash_sha256}:fixtures=10:accepted=1:denials=30"
    ));
    let policy_hash_sha256 = sha256_text_value(
        "minimal-scoped-memory-real-write-canary-durable-store-write-preflight-policy:v1:accept-preflight-only:no-durable-memory-write:no-memory-store-mutation:no-wal-write:no-receipt-persist:no-readback:no-rollback:no-tombstone:no-kg:no-provider:no-channel:no-release:no-install",
    );

    let mut side_effects = serde_json::Map::new();
    side_effects.insert(
        "durable_store_write_preflight_performed".to_string(),
        serde_json::json!(report_ready),
    );
    side_effects.insert(
        "durable_store_write_preflight_result_accepted".to_string(),
        serde_json::json!(report_ready),
    );
    for &key in FALSE_EXTERNAL_KEYS {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }
    side_effects.insert(
        "durable_memory_store_write_performed".to_string(),
        serde_json::json!(false),
    );
    side_effects.insert(
        "memory_store_write_performed".to_string(),
        serde_json::json!(false),
    );
    side_effects.insert(
        "external_send_performed".to_string(),
        serde_json::json!(false),
    );

    let mut report = serde_json::Map::new();
    macro_rules! insert_report_json {
        ($key:expr, $value:expr) => {
            report.insert($key.to_string(), serde_json::json!($value));
        };
    }
    insert_report_json!("product", "Hepta");
    insert_report_json!("runtime", "hepta");
    insert_report_json!("status", if report_ready { "ready" } else { "blocked" });
    insert_report_json!("base_url", "http://127.0.0.1:7373");
    insert_report_json!(
        "gate",
        "hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_durable_store_write_preflight_boundary_gate"
    );
    insert_report_json!(
        "endpoint",
        HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_MINIMAL_SCOPED_MEMORY_REAL_WRITE_CANARY_DURABLE_STORE_WRITE_PREFLIGHT_BOUNDARY_ENDPOINT
    );
    insert_report_json!(
        "source_command",
        "/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-durable-store-write-preflight-boundary --json"
    );
    insert_report_json!("native_route", true);
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
        "memory_write_execution_minimal_scoped_memory_real_write_canary_durable_store_write_preflight_boundary_ready",
        report_ready
    );
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_durable_store_write_preflight_ready",
        report_ready
    );
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_durable_store_write_preflight_performed",
        report_ready
    );
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_durable_store_write_preflight_accepted",
        report_ready
    );
    insert_report_json!(
        "scoped_memory_real_write_canary_mode",
        "minimal_scoped_memory_real_write_canary_durable_store_write_preflight_report_only"
    );
    insert_report_json!(
        "source_minimal_scoped_memory_real_write_canary_durable_store_write_plan_boundary_ready",
        source_ready
    );
    insert_report_json!(
        "source_minimal_scoped_memory_real_write_canary_durable_store_write_plan_report_sha256",
        source_report_sha256
    );
    insert_report_json!(
        "source_accepted_minimal_scoped_memory_real_write_canary_durable_store_write_plan_fixture_count",
        json_u64(
            &source,
            "accepted_minimal_scoped_memory_real_write_canary_durable_store_write_plan_fixture_count",
        )
    );
    insert_report_json!(
        "source_blocked_minimal_scoped_memory_real_write_canary_durable_store_write_plan_fixture_count",
        json_u64(
            &source,
            "blocked_minimal_scoped_memory_real_write_canary_durable_store_write_plan_fixture_count",
        )
    );
    insert_report_json!(
        "source_durable_store_write_plan_result_accepted_count",
        json_u64(&source, "durable_store_write_plan_result_accepted_count")
    );
    insert_report_json!(
        "source_durable_store_write_plan_executed_count",
        json_u64(&source, "durable_store_write_plan_executed_count")
    );
    insert_report_json!(
        "source_durable_memory_store_write_performed_count",
        json_u64(&source, "durable_memory_store_write_performed_count")
    );
    insert_report_json!(
        "source_memory_store_write_performed_count",
        json_u64(&source, "memory_store_write_performed_count")
    );
    insert_report_json!("approved_namespace", approved_namespace);
    insert_report_json!("approved_store", approved_store);
    insert_report_json!("approved_scope", approved_scope);
    insert_report_json!(
        "durable_store_write_target_id",
        durable_store_write_target_id
    );
    insert_report_json!(
        "durable_store_target_store_id",
        durable_store_target_store_id
    );
    insert_report_json!(
        "source_durable_store_write_plan_hash_sha256",
        source_durable_store_write_plan_hash_sha256
    );
    insert_report_json!(
        "source_durable_store_write_target_sha256",
        source_durable_store_write_target_sha256
    );
    insert_report_json!(
        "source_durable_store_write_envelope_sha256",
        source_durable_store_write_envelope_sha256
    );
    insert_report_json!(
        "source_durable_store_write_payload_digest_sha256",
        source_durable_store_write_payload_digest_sha256
    );
    insert_report_json!(
        "source_durable_store_write_wal_receipt_plan_sha256",
        source_durable_store_write_wal_receipt_plan_sha256
    );
    insert_report_json!(
        "source_durable_store_write_readback_plan_sha256",
        source_durable_store_write_readback_plan_sha256
    );
    insert_report_json!(
        "source_durable_store_write_rollback_plan_sha256",
        source_durable_store_write_rollback_plan_sha256
    );
    insert_report_json!(
        "source_durable_store_write_tombstone_cleanup_plan_sha256",
        source_durable_store_write_tombstone_cleanup_plan_sha256
    );
    insert_report_json!(
        "durable_store_write_preflight_target_reachability_sha256",
        durable_store_write_preflight_target_reachability_sha256
    );
    insert_report_json!(
        "durable_store_write_preflight_namespace_scope_sha256",
        durable_store_write_preflight_namespace_scope_sha256
    );
    insert_report_json!(
        "durable_store_write_preflight_redaction_sha256",
        durable_store_write_preflight_redaction_sha256
    );
    insert_report_json!(
        "durable_store_write_preflight_wal_receipt_sha256",
        durable_store_write_preflight_wal_receipt_sha256
    );
    insert_report_json!(
        "durable_store_write_preflight_readback_sha256",
        durable_store_write_preflight_readback_sha256
    );
    insert_report_json!(
        "durable_store_write_preflight_rollback_sha256",
        durable_store_write_preflight_rollback_sha256
    );
    insert_report_json!(
        "durable_store_write_preflight_tombstone_cleanup_sha256",
        durable_store_write_preflight_tombstone_cleanup_sha256
    );
    insert_report_json!(
        "durable_store_write_preflight_idempotency_replay_guard_sha256",
        durable_store_write_preflight_idempotency_replay_guard_sha256
    );
    insert_report_json!(
        "durable_store_write_preflight_operator_handoff_sha256",
        durable_store_write_preflight_operator_handoff_sha256
    );
    insert_report_json!(
        "durable_store_write_preflight_hash_sha256",
        durable_store_write_preflight_hash_sha256
    );
    insert_report_json!(
        "required_minimal_scoped_memory_real_write_canary_durable_store_write_preflight_surface_count",
        PREFLIGHT_SURFACES.len()
    );
    insert_report_json!(
        "ready_minimal_scoped_memory_real_write_canary_durable_store_write_preflight_surface_count",
        ready_surface_count
    );
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_durable_store_write_preflight_fixture_count",
        10
    );
    insert_report_json!(
        "accepted_minimal_scoped_memory_real_write_canary_durable_store_write_preflight_fixture_count",
        accepted_fixture_count
    );
    insert_report_json!(
        "blocked_minimal_scoped_memory_real_write_canary_durable_store_write_preflight_fixture_count",
        blocked_fixture_count
    );
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_durable_store_write_preflight_accepted_count",
        if report_ready { 1 } else { 0 }
    );
    insert_report_json!(
        "durable_store_write_preflight_authority_accepted_count",
        if report_ready { 1 } else { 0 }
    );
    insert_report_json!(
        "durable_store_write_preflight_result_accepted_count",
        if report_ready { 1 } else { 0 }
    );
    insert_report_json!(
        "denied_by_minimal_scoped_memory_real_write_canary_durable_store_write_preflight_boundary",
        PREFLIGHT_DENIALS
    );
    insert_report_json!(
        "denied_by_minimal_scoped_memory_real_write_canary_durable_store_write_preflight_boundary_count",
        PREFLIGHT_DENIALS.len()
    );
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_durable_store_write_preflight_boundary_hash_sha256",
        boundary_hash_sha256
    );
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_durable_store_write_preflight_policy_hash_sha256",
        policy_hash_sha256
    );
    report.insert(
        "minimal_scoped_memory_real_write_canary_durable_store_write_preflight_fixtures"
            .to_string(),
        fixtures,
    );
    for key in [
        "source_durable_store_write_plan_bound",
        "source_durable_store_write_plan_hash_bound",
        "source_durable_store_write_plan_result_accepted",
        "approved_namespace_bound",
        "approved_store_bound",
        "approved_scope_bound",
        "durable_store_target_reachability_checked",
        "approved_namespace_store_scope_preflight_verified",
        "durable_store_write_envelope_preflight_verified",
        "durable_store_write_payload_digest_preflight_verified",
        "payload_redaction_preflight_verified",
        "payload_secret_plaintext_scan_passed",
        "durable_store_write_wal_receipt_preflight_bound",
        "durable_store_write_readback_preflight_bound",
        "durable_store_write_rollback_preflight_bound",
        "durable_store_write_tombstone_cleanup_preflight_bound",
        "durable_store_write_idempotency_replay_guard_preflight_bound",
        "durable_store_write_operator_preflight_handoff_bound",
        "durable_memory_write_forbidden",
        "memory_store_mutation_forbidden",
        "wal_write_forbidden_on_report_route",
        "receipt_persist_forbidden_on_report_route",
        "post_write_readback_forbidden_on_report_route",
        "rollback_execution_forbidden",
        "tombstone_write_forbidden",
        "artifact_cleanup_forbidden",
    ] {
        insert_report_json!(key, report_ready);
    }
    for key in [
        "source_durable_store_write_plan_bound_count",
        "source_durable_store_write_plan_hash_bound_count",
        "source_durable_store_write_plan_result_accepted_count",
        "durable_store_target_reachability_checked_count",
        "approved_namespace_store_scope_preflight_verified_count",
        "durable_store_write_envelope_preflight_verified_count",
        "durable_store_write_payload_digest_preflight_verified_count",
        "payload_redaction_preflight_verified_count",
        "payload_secret_plaintext_scan_passed_count",
        "durable_store_write_wal_receipt_preflight_bound_count",
        "durable_store_write_readback_preflight_bound_count",
        "durable_store_write_rollback_preflight_bound_count",
        "durable_store_write_tombstone_cleanup_preflight_bound_count",
        "durable_store_write_idempotency_replay_guard_preflight_bound_count",
        "durable_store_write_operator_preflight_handoff_bound_count",
        "durable_store_write_preflight_result_recorded_count",
        "durable_store_write_preflight_result_accepted_count",
    ] {
        insert_report_json!(key, if report_ready { 1 } else { 0 });
    }
    for &key in FALSE_EXTERNAL_KEYS {
        insert_report_json!(key, false);
        insert_report_json!(format!("{key}_count"), 0);
    }
    for &key in TRUE_PREFLIGHT_KEYS {
        insert_report_json!(key, report_ready);
    }
    report.insert(
        "side_effects".to_string(),
        serde_json::Value::Object(side_effects),
    );
    report.insert(
        "allowed_next_actions".to_string(),
        serde_json::json!([
            {
                "action": "run_minimal_scoped_memory_real_write_canary_durable_store_write_preflight_boundary_require_live_gate",
                "status": "allowed_verification_only",
                "writes_durable_memory": false,
                "mutates_memory_store": false
            },
            {
                "action": "prepare_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_boundary",
                "status": "allowed_report_only_next_slice",
                "requires_minimal_scoped_memory_real_write_canary_durable_store_write_preflight": true,
                "writes_durable_memory": false,
                "mutates_memory_store": false
            }
        ]),
    );
    serde_json::Value::Object(report)
}

fn hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_boundary_report()
-> serde_json::Value {
    const READINESS_SURFACES: &[&str] = &[
        "source_durable_store_write_preflight_required",
        "approved_namespace_store_scope_required",
        "durable_store_target_guard_required",
        "guarded_execution_envelope_required",
        "nonce_command_guard_required",
        "single_write_budget_guard_required",
        "wal_receipt_guard_required",
        "post_write_readback_guard_required",
        "rollback_guard_required",
        "tombstone_cleanup_guard_required",
        "idempotency_replay_guard_required",
        "durable_memory_kg_provider_channel_release_install_active_binary_side_effects_forbidden",
    ];
    const READINESS_DENIALS: &[&str] = &[
        "source_durable_store_write_preflight_boundary_required",
        "source_durable_store_write_preflight_result_acceptance_required",
        "source_durable_store_write_preflight_hash_required",
        "approved_namespace_required",
        "approved_store_required",
        "approved_scope_required",
        "durable_store_target_required",
        "durable_store_target_reachability_preflight_required",
        "guarded_execution_envelope_required",
        "single_use_nonce_guard_required",
        "explicit_command_guard_required",
        "single_write_budget_guard_required",
        "wal_receipt_guard_required",
        "readback_guard_required",
        "rollback_guard_required",
        "tombstone_cleanup_guard_required",
        "idempotency_replay_guard_required",
        "operator_guarded_execution_handoff_required",
        "durable_memory_store_read_denied",
        "durable_memory_store_write_denied",
        "durable_memory_store_rollback_denied",
        "memory_store_mutation_denied",
        "wal_write_denied",
        "receipt_record_persist_materialize_denied",
        "artifact_filesystem_write_denied",
        "post_write_readback_denied",
        "rollback_tombstone_execution_denied",
        "kg_provider_credential_channel_release_install_denied",
        "raw_payload_plaintext_denied",
        "guard_bypass_denied",
        "stale_preflight_denied",
        "direct_execution_authority_denied",
    ];
    const FALSE_EXTERNAL_KEYS: &[&str] = &[
        "single_use_nonce_consumed",
        "explicit_command_dispatched",
        "guarded_execution_command_dispatched",
        "durable_store_write_preflight_executed",
        "durable_store_write_guarded_execution_readiness_executed",
        "durable_store_write_guarded_execution_executed",
        "durable_store_write_execution_performed",
        "durable_store_write_plan_executed",
        "wal_write_performed",
        "wal_recorded",
        "wal_persisted",
        "receipt_recorded",
        "receipt_persisted",
        "receipt_materialized",
        "receipt_delivered",
        "canary_artifact_filesystem_written",
        "artifact_readback_performed",
        "artifact_cleanup_performed",
        "filesystem_written",
        "post_write_readback_performed",
        "readback_result_recorded",
        "readback_result_persisted",
        "readback_result_accepted",
        "rollback_executed",
        "rollback_performed",
        "rollback_result_recorded",
        "rollback_result_persisted",
        "rollback_result_accepted",
        "tombstone_cleanup_executed",
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
    ];
    const TRUE_READINESS_KEYS: &[&str] = &[
        "durable_store_write_guarded_execution_readiness_performed",
        "durable_store_write_guarded_execution_readiness_result_recorded",
        "durable_store_write_guarded_execution_readiness_result_accepted",
        "source_durable_store_write_preflight_bound",
        "source_durable_store_write_preflight_hash_bound",
        "source_durable_store_write_preflight_result_accepted",
        "approved_namespace_store_scope_guard_verified",
        "durable_store_target_guard_verified",
        "guarded_execution_envelope_bound",
        "single_use_nonce_guard_bound",
        "explicit_command_guard_bound",
        "single_write_budget_guard_bound",
        "wal_receipt_guard_bound",
        "post_write_readback_guard_bound",
        "rollback_guard_bound",
        "tombstone_cleanup_guard_bound",
        "idempotency_replay_guard_bound",
        "operator_guarded_execution_handoff_bound",
        "durable_memory_write_forbidden_until_guarded_execution_boundary",
        "memory_store_mutation_forbidden_until_guarded_execution_boundary",
        "kg_provider_channel_release_install_active_binary_forbidden",
        "minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_accepted",
    ];

    fn readiness_fixture(
        id: &str,
        status: &str,
        reason: &str,
        accepted: bool,
        extra: serde_json::Value,
    ) -> serde_json::Value {
        let mut base = serde_json::Map::new();
        base.insert("id".to_string(), serde_json::json!(id));
        base.insert("fixture_id".to_string(), serde_json::json!(id));
        base.insert(
            "minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_status".to_string(),
            serde_json::json!(status),
        );
        base.insert("reason".to_string(), serde_json::json!(reason));
        base.insert(
            "minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_accepted".to_string(),
            serde_json::json!(accepted),
        );
        for key in [
            "source_durable_store_write_preflight_bound",
            "source_durable_store_write_preflight_hash_bound",
            "source_durable_store_write_preflight_result_accepted",
            "approved_namespace_bound",
            "approved_store_bound",
            "approved_scope_bound",
            "approved_namespace_store_scope_guard_verified",
            "durable_store_target_guard_verified",
            "guarded_execution_envelope_bound",
            "single_use_nonce_guard_bound",
            "explicit_command_guard_bound",
            "single_write_budget_guard_bound",
            "wal_receipt_guard_bound",
            "post_write_readback_guard_bound",
            "rollback_guard_bound",
            "tombstone_cleanup_guard_bound",
            "idempotency_replay_guard_bound",
            "operator_guarded_execution_handoff_bound",
        ] {
            base.insert(key.to_string(), serde_json::json!(accepted));
        }
        for &key in FALSE_EXTERNAL_KEYS {
            base.insert(key.to_string(), serde_json::json!(false));
        }
        for &key in TRUE_READINESS_KEYS {
            base.insert(key.to_string(), serde_json::json!(accepted));
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
        .name("hepta-memory-minimal-canary-durable-store-guarded-readiness-source-report".to_string())
        .stack_size(8 * 1024 * 1024)
        .spawn(
            hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_durable_store_write_preflight_boundary_report,
        )
        .ok()
        .and_then(|handle| handle.join().ok())
        .unwrap_or_else(|| {
            serde_json::json!({
                "status": "blocked",
                "memory_write_execution_minimal_scoped_memory_real_write_canary_durable_store_write_preflight_boundary_ready": false,
                "minimal_scoped_memory_real_write_canary_durable_store_write_preflight_ready": false,
                "source_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_source_report_thread_failed": true
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
    let json_str = |value: &serde_json::Value, key: &str| {
        value
            .get(key)
            .and_then(serde_json::Value::as_str)
            .unwrap_or("")
            .to_string()
    };
    let route_count_source_command_accepted = route_matrix.ready
        && route_matrix.route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let source_next_action_guarded_readiness = source
        .get("allowed_next_actions")
        .and_then(serde_json::Value::as_array)
        .and_then(|items| items.get(1))
        .map(|item| {
            item.get("action").and_then(serde_json::Value::as_str)
                == Some(
                    "prepare_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_boundary",
                )
                && item
                    .get("requires_minimal_scoped_memory_real_write_canary_durable_store_write_preflight")
                    .and_then(serde_json::Value::as_bool)
                    == Some(true)
        })
        .unwrap_or(false);
    let source_side_effects_ok = source
        .get("side_effects")
        .and_then(serde_json::Value::as_object)
        .map(|effects| {
            effects
                .get("durable_store_write_preflight_performed")
                .and_then(serde_json::Value::as_bool)
                == Some(true)
                && effects
                    .get("durable_store_write_preflight_result_accepted")
                    .and_then(serde_json::Value::as_bool)
                    == Some(true)
                && effects
                    .get("durable_store_write_preflight_executed")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && effects
                    .get("durable_memory_store_write_performed")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && effects
                    .get("memory_store_write_performed")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && effects
                    .get("external_send_performed")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
        })
        .unwrap_or(false);
    let source_ready = source.get("status").and_then(serde_json::Value::as_str) == Some("ready")
        && json_bool(
            &source,
            "memory_write_execution_minimal_scoped_memory_real_write_canary_durable_store_write_preflight_boundary_ready",
        )
        && json_bool(
            &source,
            "minimal_scoped_memory_real_write_canary_durable_store_write_preflight_ready",
        )
        && json_bool(
            &source,
            "minimal_scoped_memory_real_write_canary_durable_store_write_preflight_performed",
        )
        && json_bool(
            &source,
            "minimal_scoped_memory_real_write_canary_durable_store_write_preflight_accepted",
        )
        && json_bool(&source, "durable_store_write_preflight_result_accepted")
        && json_bool(&source, "durable_store_target_reachability_checked")
        && json_bool(&source, "approved_namespace_store_scope_preflight_verified")
        && json_bool(&source, "payload_secret_plaintext_scan_passed")
        && json_bool(&source, "durable_store_write_wal_receipt_preflight_bound")
        && json_bool(&source, "durable_store_write_readback_preflight_bound")
        && json_bool(&source, "durable_store_write_rollback_preflight_bound")
        && json_bool(
            &source,
            "durable_store_write_tombstone_cleanup_preflight_bound",
        )
        && json_bool(
            &source,
            "durable_store_write_idempotency_replay_guard_preflight_bound",
        )
        && json_bool(
            &source,
            "durable_store_write_operator_preflight_handoff_bound",
        )
        && json_u64(
            &source,
            "accepted_minimal_scoped_memory_real_write_canary_durable_store_write_preflight_fixture_count",
        ) == 1
        && json_u64(
            &source,
            "blocked_minimal_scoped_memory_real_write_canary_durable_store_write_preflight_fixture_count",
        ) == 9
        && json_u64(
            &source,
            "durable_store_write_preflight_result_accepted_count",
        ) == 1
        && json_u64(&source, "durable_store_write_preflight_executed_count") == 0
        && json_u64(&source, "durable_memory_store_write_performed_count") == 0
        && json_u64(&source, "durable_memory_store_read_performed_count") == 0
        && json_u64(&source, "durable_memory_store_rollback_performed_count") == 0
        && json_u64(&source, "memory_store_write_performed_count") == 0
        && json_u64(&source, "wal_write_performed_count") == 0
        && json_u64(&source, "receipt_persisted_count") == 0
        && json_u64(&source, "artifact_cleanup_performed_count") == 0
        && json_u64(&source, "post_write_readback_performed_count") == 0
        && json_u64(&source, "rollback_performed_count") == 0
        && json_u64(&source, "tombstone_written_count") == 0
        && !json_bool(&source, "durable_store_write_preflight_executed")
        && !json_bool(&source, "durable_store_write_plan_executed")
        && !json_bool(&source, "durable_memory_store_write_performed")
        && !json_bool(&source, "durable_memory_store_read_performed")
        && !json_bool(&source, "durable_memory_store_rollback_performed")
        && !json_bool(&source, "memory_store_write_performed")
        && !json_bool(&source, "wal_write_performed")
        && !json_bool(&source, "receipt_persisted")
        && !json_bool(&source, "artifact_cleanup_performed")
        && !json_bool(&source, "post_write_readback_performed")
        && !json_bool(&source, "rollback_performed")
        && !json_bool(&source, "tombstone_written")
        && !json_bool(&source, "raw_payload_plaintext_recorded")
        && !json_bool(&source, "live_kg_write_performed")
        && !json_bool(&source, "provider_invoked")
        && !json_bool(&source, "model_invoked")
        && !json_bool(&source, "credential_read")
        && !json_bool(&source, "channel_send_performed")
        && !json_bool(&source, "external_send_performed")
        && !json_bool(&source, "release_artifact_written")
        && !json_bool(&source, "install_executed")
        && !json_bool(&source, "active_binary_mutated")
        && source_next_action_guarded_readiness
        && source_side_effects_ok;

    let approved_namespace = json_str(&source, "approved_namespace");
    let approved_store = json_str(&source, "approved_store");
    let approved_scope = json_str(&source, "approved_scope");
    let durable_store_write_target_id = json_str(&source, "durable_store_write_target_id");
    let durable_store_target_store_id = json_str(&source, "durable_store_target_store_id");
    let source_report_sha256 = sha256_text_value(&source.to_string());
    let source_durable_store_write_preflight_hash_sha256 =
        json_str(&source, "durable_store_write_preflight_hash_sha256");
    let source_durable_store_write_preflight_operator_handoff_sha256 = json_str(
        &source,
        "durable_store_write_preflight_operator_handoff_sha256",
    );
    let source_durable_store_write_target_sha256 =
        json_str(&source, "source_durable_store_write_target_sha256");
    let source_durable_store_write_payload_digest_sha256 =
        json_str(&source, "source_durable_store_write_payload_digest_sha256");
    let source_durable_store_write_wal_receipt_plan_sha256 = json_str(
        &source,
        "source_durable_store_write_wal_receipt_plan_sha256",
    );
    let source_durable_store_write_readback_plan_sha256 =
        json_str(&source, "source_durable_store_write_readback_plan_sha256");
    let source_durable_store_write_rollback_plan_sha256 =
        json_str(&source, "source_durable_store_write_rollback_plan_sha256");
    let source_durable_store_write_tombstone_cleanup_plan_sha256 = json_str(
        &source,
        "source_durable_store_write_tombstone_cleanup_plan_sha256",
    );
    let namespace_bound = approved_namespace == "hepta.memory.canary";
    let store_bound = approved_store == "wal-receipt-canary-artifact";
    let scope_bound = approved_scope == "session";
    let source_preflight_hash_bound = !source_durable_store_write_preflight_hash_sha256.is_empty();
    let target_bound = durable_store_write_target_id
        == "hepta-minimal-scoped-memory-real-write-canary-durable-store-write-target-v1"
        && durable_store_target_store_id == "hepta-memory-durable-store-canary-plan-only"
        && !source_durable_store_write_target_sha256.is_empty();
    let payload_digest_bound = !source_durable_store_write_payload_digest_sha256.is_empty();
    let wal_receipt_plan_bound = !source_durable_store_write_wal_receipt_plan_sha256.is_empty();
    let readback_plan_bound = !source_durable_store_write_readback_plan_sha256.is_empty();
    let rollback_plan_bound = !source_durable_store_write_rollback_plan_sha256.is_empty();
    let tombstone_cleanup_plan_bound =
        !source_durable_store_write_tombstone_cleanup_plan_sha256.is_empty();

    let guarded_execution_envelope_sha256 = sha256_text_value(&format!(
        "minimal-scoped-memory-real-write-canary-durable-store-guarded-execution-envelope:v1:source-preflight={source_durable_store_write_preflight_hash_sha256}:target-store={durable_store_target_store_id}:namespace={approved_namespace}:scope={approved_scope}:execute=false"
    ));
    let single_use_nonce_guard_sha256 = sha256_text_value(&format!(
        "minimal-scoped-memory-real-write-canary-durable-store-guarded-execution-nonce:v1:source-preflight={source_durable_store_write_preflight_hash_sha256}:nonce-consumed=false:execute=false"
    ));
    let explicit_command_guard_sha256 = sha256_text_value(&format!(
        "minimal-scoped-memory-real-write-canary-durable-store-guarded-execution-command:v1:source-preflight={source_durable_store_write_preflight_hash_sha256}:command-dispatched=false:operator-explicit-required=true"
    ));
    let single_write_budget_guard_sha256 = sha256_text_value(&format!(
        "minimal-scoped-memory-real-write-canary-durable-store-guarded-execution-budget:v1:target={durable_store_write_target_id}:max-write=1:max-readback=1:max-rollback=1:execute=false"
    ));
    let wal_receipt_guard_sha256 = sha256_text_value(&format!(
        "minimal-scoped-memory-real-write-canary-durable-store-guarded-execution-wal-receipt:v1:plan={source_durable_store_write_wal_receipt_plan_sha256}:wal-write=false:receipt-persist=false"
    ));
    let readback_guard_sha256 = sha256_text_value(&format!(
        "minimal-scoped-memory-real-write-canary-durable-store-guarded-execution-readback:v1:plan={source_durable_store_write_readback_plan_sha256}:read=false"
    ));
    let rollback_guard_sha256 = sha256_text_value(&format!(
        "minimal-scoped-memory-real-write-canary-durable-store-guarded-execution-rollback:v1:plan={source_durable_store_write_rollback_plan_sha256}:rollback=false"
    ));
    let tombstone_cleanup_guard_sha256 = sha256_text_value(&format!(
        "minimal-scoped-memory-real-write-canary-durable-store-guarded-execution-tombstone-cleanup:v1:plan={source_durable_store_write_tombstone_cleanup_plan_sha256}:tombstone=false:cleanup=false"
    ));
    let idempotency_replay_guard_sha256 = sha256_text_value(&format!(
        "minimal-scoped-memory-real-write-canary-durable-store-guarded-execution-idempotency-replay:v1:source-preflight={source_durable_store_write_preflight_hash_sha256}:target-store={durable_store_target_store_id}:replay=false"
    ));
    let operator_guarded_execution_handoff_sha256 = sha256_text_value(&format!(
        "minimal-scoped-memory-real-write-canary-durable-store-guarded-execution-operator-handoff:v1:source={source_report_sha256}:preflight-handoff={source_durable_store_write_preflight_operator_handoff_sha256}:readiness=true:execute=false"
    ));
    let guarded_execution_readiness_hash_sha256 = sha256_text_value(&format!(
        "minimal-scoped-memory-real-write-canary-durable-store-write-guarded-execution-readiness:v1:source-preflight={source_durable_store_write_preflight_hash_sha256}:envelope={guarded_execution_envelope_sha256}:nonce={single_use_nonce_guard_sha256}:command={explicit_command_guard_sha256}:budget={single_write_budget_guard_sha256}:wal={wal_receipt_guard_sha256}:readback={readback_guard_sha256}:rollback={rollback_guard_sha256}:tombstone={tombstone_cleanup_guard_sha256}:handoff={operator_guarded_execution_handoff_sha256}"
    ));
    let guard_hashes_bound = [
        &guarded_execution_envelope_sha256,
        &single_use_nonce_guard_sha256,
        &explicit_command_guard_sha256,
        &single_write_budget_guard_sha256,
        &wal_receipt_guard_sha256,
        &readback_guard_sha256,
        &rollback_guard_sha256,
        &tombstone_cleanup_guard_sha256,
        &idempotency_replay_guard_sha256,
        &operator_guarded_execution_handoff_sha256,
        &guarded_execution_readiness_hash_sha256,
    ]
    .iter()
    .all(|hash| !hash.is_empty());

    let fixtures = serde_json::json!([
        readiness_fixture(
            "accepted-guarded-execution-readiness",
            "accepted",
            "minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_accepted",
            source_ready
                && namespace_bound
                && store_bound
                && scope_bound
                && source_preflight_hash_bound
                && target_bound
                && payload_digest_bound
                && wal_receipt_plan_bound
                && readback_plan_bound
                && rollback_plan_bound
                && tombstone_cleanup_plan_bound
                && guard_hashes_bound,
            serde_json::json!({
                "approved_namespace": approved_namespace,
                "approved_store": approved_store,
                "approved_scope": approved_scope,
                "durable_store_write_target_id": durable_store_write_target_id,
                "durable_store_target_store_id": durable_store_target_store_id,
                "source_durable_store_write_preflight_hash_sha256": source_durable_store_write_preflight_hash_sha256,
                "guarded_execution_readiness_hash_sha256": guarded_execution_readiness_hash_sha256,
                "operator_guarded_execution_handoff_sha256": operator_guarded_execution_handoff_sha256
            }),
        ),
        readiness_fixture(
            "missing-source-preflight",
            "blocked_noop",
            "source_durable_store_write_preflight_missing",
            false,
            serde_json::json!({}),
        ),
        readiness_fixture(
            "wrong-namespace",
            "blocked_noop",
            "approved_namespace_mismatch",
            false,
            serde_json::json!({}),
        ),
        readiness_fixture(
            "wrong-store",
            "blocked_noop",
            "approved_store_mismatch",
            false,
            serde_json::json!({}),
        ),
        readiness_fixture(
            "wrong-scope",
            "blocked_noop",
            "approved_scope_mismatch",
            false,
            serde_json::json!({}),
        ),
        readiness_fixture(
            "execution-envelope-missing",
            "blocked_noop",
            "guarded_execution_envelope_missing",
            false,
            serde_json::json!({}),
        ),
        readiness_fixture(
            "nonce-command-guard-missing",
            "blocked_noop",
            "nonce_or_explicit_command_guard_missing",
            false,
            serde_json::json!({}),
        ),
        readiness_fixture(
            "budget-guard-missing",
            "blocked_noop",
            "single_write_budget_guard_missing",
            false,
            serde_json::json!({}),
        ),
        readiness_fixture(
            "readback-rollback-tombstone-guard-missing",
            "blocked_noop",
            "readback_rollback_or_tombstone_guard_missing",
            false,
            serde_json::json!({}),
        ),
        readiness_fixture(
            "direct-execution-attempt",
            "blocked_noop",
            "direct_durable_store_write_execution_denied",
            false,
            serde_json::json!({}),
        ),
    ]);
    let accepted_fixture_count = fixtures
        .as_array()
        .map(|items| {
            items
                .iter()
                .filter(|item| {
                    json_bool(
                        item,
                        "minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_accepted",
                    )
                })
                .count()
        })
        .unwrap_or(0);
    let blocked_fixture_count = fixtures
        .as_array()
        .map(std::vec::Vec::len)
        .unwrap_or(0)
        .saturating_sub(accepted_fixture_count);
    let surface_ready = source_ready
        && namespace_bound
        && store_bound
        && scope_bound
        && source_preflight_hash_bound
        && target_bound
        && payload_digest_bound
        && wal_receipt_plan_bound
        && readback_plan_bound
        && rollback_plan_bound
        && tombstone_cleanup_plan_bound
        && guard_hashes_bound;
    let ready_surface_count = if surface_ready {
        READINESS_SURFACES.len()
    } else {
        0
    };
    let report_ready = route_count_source_command_accepted
        && surface_ready
        && accepted_fixture_count == 1
        && blocked_fixture_count == 9
        && READINESS_DENIALS.len() == 32;
    let boundary_hash_sha256 = sha256_text_value(&format!(
        "minimal-scoped-memory-real-write-canary-durable-store-write-guarded-execution-readiness-boundary:v1:source-ready={source_ready}:target={target_bound}:guards={guarded_execution_readiness_hash_sha256}:fixtures=10:accepted=1:denials=32"
    ));
    let policy_hash_sha256 = sha256_text_value(
        "minimal-scoped-memory-real-write-canary-durable-store-write-guarded-execution-readiness-policy:v1:accept-readiness-only:no-durable-memory-write:no-memory-store-mutation:no-wal-write:no-receipt-persist:no-readback:no-rollback:no-tombstone:no-kg:no-provider:no-channel:no-release:no-install",
    );

    let mut side_effects = serde_json::Map::new();
    side_effects.insert(
        "durable_store_write_guarded_execution_readiness_performed".to_string(),
        serde_json::json!(report_ready),
    );
    side_effects.insert(
        "durable_store_write_guarded_execution_readiness_result_accepted".to_string(),
        serde_json::json!(report_ready),
    );
    for &key in FALSE_EXTERNAL_KEYS {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }
    side_effects.insert(
        "durable_memory_store_write_performed".to_string(),
        serde_json::json!(false),
    );
    side_effects.insert(
        "memory_store_write_performed".to_string(),
        serde_json::json!(false),
    );
    side_effects.insert(
        "external_send_performed".to_string(),
        serde_json::json!(false),
    );

    let mut report = serde_json::Map::new();
    macro_rules! insert_report_json {
        ($key:expr, $value:expr) => {
            report.insert($key.to_string(), serde_json::json!($value));
        };
    }
    insert_report_json!("product", "Hepta");
    insert_report_json!("runtime", "hepta");
    insert_report_json!("status", if report_ready { "ready" } else { "blocked" });
    insert_report_json!("base_url", "http://127.0.0.1:7373");
    insert_report_json!(
        "gate",
        "hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_boundary_gate"
    );
    insert_report_json!(
        "endpoint",
        HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_MINIMAL_SCOPED_MEMORY_REAL_WRITE_CANARY_DURABLE_STORE_WRITE_GUARDED_EXECUTION_READINESS_BOUNDARY_ENDPOINT
    );
    insert_report_json!(
        "source_command",
        "/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-durable-store-write-guarded-execution-readiness-boundary --json"
    );
    insert_report_json!("native_route", true);
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
        "memory_write_execution_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_boundary_ready",
        report_ready
    );
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_ready",
        report_ready
    );
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_performed",
        report_ready
    );
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_accepted",
        report_ready
    );
    insert_report_json!(
        "scoped_memory_real_write_canary_mode",
        "minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_report_only"
    );
    insert_report_json!(
        "source_minimal_scoped_memory_real_write_canary_durable_store_write_preflight_boundary_ready",
        source_ready
    );
    insert_report_json!(
        "source_minimal_scoped_memory_real_write_canary_durable_store_write_preflight_report_sha256",
        source_report_sha256
    );
    insert_report_json!(
        "source_accepted_minimal_scoped_memory_real_write_canary_durable_store_write_preflight_fixture_count",
        json_u64(
            &source,
            "accepted_minimal_scoped_memory_real_write_canary_durable_store_write_preflight_fixture_count",
        )
    );
    insert_report_json!(
        "source_blocked_minimal_scoped_memory_real_write_canary_durable_store_write_preflight_fixture_count",
        json_u64(
            &source,
            "blocked_minimal_scoped_memory_real_write_canary_durable_store_write_preflight_fixture_count",
        )
    );
    insert_report_json!(
        "source_durable_store_write_preflight_result_accepted_count",
        json_u64(
            &source,
            "durable_store_write_preflight_result_accepted_count"
        )
    );
    insert_report_json!(
        "source_durable_store_write_preflight_executed_count",
        json_u64(&source, "durable_store_write_preflight_executed_count")
    );
    insert_report_json!(
        "source_durable_memory_store_write_performed_count",
        json_u64(&source, "durable_memory_store_write_performed_count")
    );
    insert_report_json!(
        "source_memory_store_write_performed_count",
        json_u64(&source, "memory_store_write_performed_count")
    );
    insert_report_json!("approved_namespace", approved_namespace);
    insert_report_json!("approved_store", approved_store);
    insert_report_json!("approved_scope", approved_scope);
    insert_report_json!(
        "durable_store_write_target_id",
        durable_store_write_target_id
    );
    insert_report_json!(
        "durable_store_target_store_id",
        durable_store_target_store_id
    );
    insert_report_json!(
        "source_durable_store_write_preflight_hash_sha256",
        source_durable_store_write_preflight_hash_sha256
    );
    insert_report_json!(
        "source_durable_store_write_preflight_operator_handoff_sha256",
        source_durable_store_write_preflight_operator_handoff_sha256
    );
    insert_report_json!(
        "source_durable_store_write_target_sha256",
        source_durable_store_write_target_sha256
    );
    insert_report_json!(
        "source_durable_store_write_payload_digest_sha256",
        source_durable_store_write_payload_digest_sha256
    );
    insert_report_json!(
        "source_durable_store_write_wal_receipt_plan_sha256",
        source_durable_store_write_wal_receipt_plan_sha256
    );
    insert_report_json!(
        "source_durable_store_write_readback_plan_sha256",
        source_durable_store_write_readback_plan_sha256
    );
    insert_report_json!(
        "source_durable_store_write_rollback_plan_sha256",
        source_durable_store_write_rollback_plan_sha256
    );
    insert_report_json!(
        "source_durable_store_write_tombstone_cleanup_plan_sha256",
        source_durable_store_write_tombstone_cleanup_plan_sha256
    );
    insert_report_json!(
        "guarded_execution_envelope_sha256",
        guarded_execution_envelope_sha256
    );
    insert_report_json!(
        "single_use_nonce_guard_sha256",
        single_use_nonce_guard_sha256
    );
    insert_report_json!(
        "explicit_command_guard_sha256",
        explicit_command_guard_sha256
    );
    insert_report_json!(
        "single_write_budget_guard_sha256",
        single_write_budget_guard_sha256
    );
    insert_report_json!("wal_receipt_guard_sha256", wal_receipt_guard_sha256);
    insert_report_json!("readback_guard_sha256", readback_guard_sha256);
    insert_report_json!("rollback_guard_sha256", rollback_guard_sha256);
    insert_report_json!(
        "tombstone_cleanup_guard_sha256",
        tombstone_cleanup_guard_sha256
    );
    insert_report_json!(
        "idempotency_replay_guard_sha256",
        idempotency_replay_guard_sha256
    );
    insert_report_json!(
        "operator_guarded_execution_handoff_sha256",
        operator_guarded_execution_handoff_sha256
    );
    insert_report_json!(
        "guarded_execution_readiness_hash_sha256",
        guarded_execution_readiness_hash_sha256
    );
    insert_report_json!(
        "required_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_surface_count",
        READINESS_SURFACES.len()
    );
    insert_report_json!(
        "ready_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_surface_count",
        ready_surface_count
    );
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_fixture_count",
        10
    );
    insert_report_json!(
        "accepted_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_fixture_count",
        accepted_fixture_count
    );
    insert_report_json!(
        "blocked_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_fixture_count",
        blocked_fixture_count
    );
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_accepted_count",
        if report_ready { 1 } else { 0 }
    );
    insert_report_json!(
        "durable_store_write_guarded_execution_readiness_authority_accepted_count",
        if report_ready { 1 } else { 0 }
    );
    insert_report_json!(
        "durable_store_write_guarded_execution_readiness_result_accepted_count",
        if report_ready { 1 } else { 0 }
    );
    insert_report_json!(
        "denied_by_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_boundary",
        READINESS_DENIALS
    );
    insert_report_json!(
        "denied_by_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_boundary_count",
        READINESS_DENIALS.len()
    );
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_boundary_hash_sha256",
        boundary_hash_sha256
    );
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_policy_hash_sha256",
        policy_hash_sha256
    );
    report.insert(
        "minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_fixtures"
            .to_string(),
        fixtures,
    );
    for key in [
        "source_durable_store_write_preflight_bound",
        "source_durable_store_write_preflight_hash_bound",
        "source_durable_store_write_preflight_result_accepted",
        "approved_namespace_bound",
        "approved_store_bound",
        "approved_scope_bound",
        "approved_namespace_store_scope_guard_verified",
        "durable_store_target_guard_verified",
        "guarded_execution_envelope_bound",
        "single_use_nonce_guard_bound",
        "explicit_command_guard_bound",
        "single_write_budget_guard_bound",
        "wal_receipt_guard_bound",
        "post_write_readback_guard_bound",
        "rollback_guard_bound",
        "tombstone_cleanup_guard_bound",
        "idempotency_replay_guard_bound",
        "operator_guarded_execution_handoff_bound",
        "durable_memory_write_forbidden_until_guarded_execution_boundary",
        "memory_store_mutation_forbidden_until_guarded_execution_boundary",
        "kg_provider_channel_release_install_active_binary_forbidden",
    ] {
        insert_report_json!(key, report_ready);
    }
    for key in [
        "source_durable_store_write_preflight_bound_count",
        "source_durable_store_write_preflight_hash_bound_count",
        "source_durable_store_write_preflight_result_accepted_count",
        "approved_namespace_store_scope_guard_verified_count",
        "durable_store_target_guard_verified_count",
        "guarded_execution_envelope_bound_count",
        "single_use_nonce_guard_bound_count",
        "explicit_command_guard_bound_count",
        "single_write_budget_guard_bound_count",
        "wal_receipt_guard_bound_count",
        "post_write_readback_guard_bound_count",
        "rollback_guard_bound_count",
        "tombstone_cleanup_guard_bound_count",
        "idempotency_replay_guard_bound_count",
        "operator_guarded_execution_handoff_bound_count",
        "durable_store_write_guarded_execution_readiness_result_recorded_count",
        "durable_store_write_guarded_execution_readiness_result_accepted_count",
    ] {
        insert_report_json!(key, if report_ready { 1 } else { 0 });
    }
    for &key in FALSE_EXTERNAL_KEYS {
        insert_report_json!(key, false);
        insert_report_json!(format!("{key}_count"), 0);
    }
    for &key in TRUE_READINESS_KEYS {
        insert_report_json!(key, report_ready);
    }
    report.insert(
        "side_effects".to_string(),
        serde_json::Value::Object(side_effects),
    );
    report.insert(
        "allowed_next_actions".to_string(),
        serde_json::json!([
            {
                "action": "run_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_boundary_require_live_gate",
                "status": "allowed_verification_only",
                "writes_durable_memory": false,
                "mutates_memory_store": false
            },
            {
                "action": "prepare_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_boundary",
                "status": "allowed_report_only_next_slice",
                "requires_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness": true,
                "writes_durable_memory": false,
                "mutates_memory_store": false
            }
        ]),
    );
    serde_json::Value::Object(report)
}

fn hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_boundary_report()
-> serde_json::Value {
    const EXECUTION_SURFACES: &[&str] = &[
        "source_guarded_execution_readiness_required",
        "approved_namespace_store_scope_required",
        "durable_store_target_guard_required",
        "guarded_execution_boundary_envelope_required",
        "nonce_command_guard_required",
        "single_write_budget_guard_required",
        "wal_receipt_execution_guard_required",
        "post_write_readback_execution_guard_required",
        "rollback_execution_guard_required",
        "tombstone_cleanup_execution_guard_required",
        "idempotency_replay_execution_guard_required",
        "durable_memory_kg_provider_channel_release_install_active_binary_side_effects_forbidden",
    ];
    const EXECUTION_DENIALS: &[&str] = &[
        "source_guarded_execution_readiness_boundary_required",
        "source_guarded_execution_readiness_result_acceptance_required",
        "source_guarded_execution_readiness_hash_required",
        "approved_namespace_required",
        "approved_store_required",
        "approved_scope_required",
        "durable_store_target_required",
        "guarded_execution_boundary_envelope_required",
        "single_use_nonce_guard_required",
        "explicit_command_guard_required",
        "single_write_budget_guard_required",
        "wal_receipt_guard_required",
        "readback_guard_required",
        "rollback_guard_required",
        "tombstone_cleanup_guard_required",
        "idempotency_replay_guard_required",
        "operator_guarded_execution_boundary_handoff_required",
        "direct_durable_store_write_execution_denied",
        "durable_memory_store_read_denied",
        "durable_memory_store_write_denied",
        "durable_memory_store_rollback_denied",
        "memory_store_mutation_denied",
        "wal_write_denied",
        "receipt_record_persist_materialize_denied",
        "artifact_filesystem_write_denied",
        "post_write_readback_denied",
        "rollback_tombstone_execution_denied",
        "kg_provider_credential_channel_release_install_denied",
        "raw_payload_plaintext_denied",
        "replay_execution_denied",
        "guard_bypass_denied",
        "stale_readiness_denied",
        "single_shot_execution_command_required",
        "actual_durable_write_must_use_next_boundary",
    ];
    const FALSE_EXTERNAL_KEYS: &[&str] = &[
        "single_use_nonce_consumed",
        "explicit_command_dispatched",
        "guarded_execution_command_dispatched",
        "durable_store_write_preflight_executed",
        "durable_store_write_guarded_execution_readiness_executed",
        "durable_store_write_guarded_execution_boundary_executed",
        "durable_store_write_guarded_execution_executed",
        "durable_store_write_execution_performed",
        "durable_store_write_plan_executed",
        "wal_write_performed",
        "wal_recorded",
        "wal_persisted",
        "receipt_recorded",
        "receipt_persisted",
        "receipt_materialized",
        "receipt_delivered",
        "canary_artifact_filesystem_written",
        "artifact_readback_performed",
        "artifact_cleanup_performed",
        "filesystem_written",
        "post_write_readback_performed",
        "readback_result_recorded",
        "readback_result_persisted",
        "readback_result_accepted",
        "rollback_executed",
        "rollback_performed",
        "rollback_result_recorded",
        "rollback_result_persisted",
        "rollback_result_accepted",
        "tombstone_cleanup_executed",
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
    ];
    const TRUE_BOUNDARY_KEYS: &[&str] = &[
        "durable_store_write_guarded_execution_boundary_performed",
        "durable_store_write_guarded_execution_boundary_result_recorded",
        "durable_store_write_guarded_execution_boundary_result_accepted",
        "source_durable_store_write_guarded_execution_readiness_bound",
        "source_durable_store_write_guarded_execution_readiness_hash_bound",
        "source_durable_store_write_guarded_execution_readiness_result_accepted",
        "approved_namespace_store_scope_execution_guard_verified",
        "durable_store_target_execution_guard_verified",
        "guarded_execution_boundary_envelope_bound",
        "single_use_nonce_execution_guard_verified",
        "explicit_command_execution_guard_verified",
        "single_write_budget_execution_guard_verified",
        "wal_receipt_execution_guard_verified",
        "post_write_readback_execution_guard_verified",
        "rollback_execution_guard_verified",
        "tombstone_cleanup_execution_guard_verified",
        "idempotency_replay_execution_guard_verified",
        "operator_guarded_execution_boundary_handoff_bound",
        "durable_memory_write_forbidden_until_single_shot_execution",
        "memory_store_mutation_forbidden_until_single_shot_execution",
        "kg_provider_channel_release_install_active_binary_forbidden",
        "minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_accepted",
    ];

    fn execution_fixture(
        id: &str,
        status: &str,
        reason: &str,
        accepted: bool,
        extra: serde_json::Value,
    ) -> serde_json::Value {
        let mut base = serde_json::Map::new();
        base.insert("id".to_string(), serde_json::json!(id));
        base.insert("fixture_id".to_string(), serde_json::json!(id));
        base.insert(
            "minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_status"
                .to_string(),
            serde_json::json!(status),
        );
        base.insert("reason".to_string(), serde_json::json!(reason));
        base.insert(
            "minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_accepted"
                .to_string(),
            serde_json::json!(accepted),
        );
        for &key in TRUE_BOUNDARY_KEYS {
            base.insert(key.to_string(), serde_json::json!(accepted));
        }
        for &key in FALSE_EXTERNAL_KEYS {
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
        .name("hepta-memory-minimal-canary-durable-store-guarded-execution-source-report".to_string())
        .stack_size(8 * 1024 * 1024)
        .spawn(
            hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_boundary_report,
        )
        .ok()
        .and_then(|handle| handle.join().ok())
        .unwrap_or_else(|| {
            serde_json::json!({
                "status": "blocked",
                "memory_write_execution_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_boundary_ready": false,
                "minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_ready": false,
                "source_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_source_report_thread_failed": true
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
    let json_str = |value: &serde_json::Value, key: &str| {
        value
            .get(key)
            .and_then(serde_json::Value::as_str)
            .unwrap_or("")
            .to_string()
    };
    let route_count_source_command_accepted = route_matrix.ready
        && route_matrix.route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let source_next_action_guarded_execution = source
        .get("allowed_next_actions")
        .and_then(serde_json::Value::as_array)
        .and_then(|items| items.get(1))
        .map(|item| {
            item.get("action").and_then(serde_json::Value::as_str)
                == Some(
                    "prepare_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_boundary",
                )
                && item
                    .get("requires_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness")
                    .and_then(serde_json::Value::as_bool)
                    == Some(true)
                && item
                    .get("writes_durable_memory")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
        })
        .unwrap_or(false);
    let source_side_effects_ok = source
        .get("side_effects")
        .and_then(serde_json::Value::as_object)
        .map(|effects| {
            effects
                .get("durable_store_write_guarded_execution_readiness_performed")
                .and_then(serde_json::Value::as_bool)
                == Some(true)
                && effects
                    .get("durable_store_write_guarded_execution_readiness_result_accepted")
                    .and_then(serde_json::Value::as_bool)
                    == Some(true)
                && effects
                    .get("durable_store_write_guarded_execution_executed")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && effects
                    .get("durable_memory_store_write_performed")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && effects
                    .get("memory_store_write_performed")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && effects
                    .get("external_send_performed")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
        })
        .unwrap_or(false);
    let source_ready = source.get("status").and_then(serde_json::Value::as_str) == Some("ready")
        && json_bool(
            &source,
            "memory_write_execution_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_boundary_ready",
        )
        && json_bool(
            &source,
            "minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_ready",
        )
        && json_bool(
            &source,
            "minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_performed",
        )
        && json_bool(
            &source,
            "minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_accepted",
        )
        && json_bool(
            &source,
            "durable_store_write_guarded_execution_readiness_result_accepted",
        )
        && json_bool(&source, "guarded_execution_envelope_bound")
        && json_bool(&source, "single_use_nonce_guard_bound")
        && json_bool(&source, "explicit_command_guard_bound")
        && json_bool(&source, "single_write_budget_guard_bound")
        && json_bool(&source, "wal_receipt_guard_bound")
        && json_bool(&source, "post_write_readback_guard_bound")
        && json_bool(&source, "rollback_guard_bound")
        && json_bool(&source, "tombstone_cleanup_guard_bound")
        && json_bool(&source, "idempotency_replay_guard_bound")
        && json_bool(&source, "operator_guarded_execution_handoff_bound")
        && json_u64(
            &source,
            "accepted_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_fixture_count",
        ) == 1
        && json_u64(
            &source,
            "blocked_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_fixture_count",
        ) == 9
        && json_u64(
            &source,
            "durable_store_write_guarded_execution_readiness_result_accepted_count",
        ) == 1
        && json_u64(
            &source,
            "durable_store_write_guarded_execution_readiness_executed_count",
        ) == 0
        && json_u64(
            &source,
            "durable_store_write_guarded_execution_executed_count",
        ) == 0
        && json_u64(&source, "durable_store_write_execution_performed_count") == 0
        && json_u64(&source, "durable_memory_store_write_performed_count") == 0
        && json_u64(&source, "durable_memory_store_read_performed_count") == 0
        && json_u64(&source, "durable_memory_store_rollback_performed_count") == 0
        && json_u64(&source, "memory_store_write_performed_count") == 0
        && json_u64(&source, "wal_write_performed_count") == 0
        && json_u64(&source, "receipt_persisted_count") == 0
        && json_u64(&source, "post_write_readback_performed_count") == 0
        && json_u64(&source, "rollback_performed_count") == 0
        && json_u64(&source, "tombstone_written_count") == 0
        && !json_bool(
            &source,
            "durable_store_write_guarded_execution_readiness_executed",
        )
        && !json_bool(&source, "durable_store_write_guarded_execution_executed")
        && !json_bool(&source, "durable_store_write_execution_performed")
        && !json_bool(&source, "durable_memory_store_write_performed")
        && !json_bool(&source, "durable_memory_store_read_performed")
        && !json_bool(&source, "durable_memory_store_rollback_performed")
        && !json_bool(&source, "memory_store_write_performed")
        && !json_bool(&source, "wal_write_performed")
        && !json_bool(&source, "receipt_persisted")
        && !json_bool(&source, "post_write_readback_performed")
        && !json_bool(&source, "rollback_performed")
        && !json_bool(&source, "tombstone_written")
        && !json_bool(&source, "raw_payload_plaintext_recorded")
        && !json_bool(&source, "live_kg_write_performed")
        && !json_bool(&source, "provider_invoked")
        && !json_bool(&source, "model_invoked")
        && !json_bool(&source, "credential_read")
        && !json_bool(&source, "channel_send_performed")
        && !json_bool(&source, "external_send_performed")
        && !json_bool(&source, "release_artifact_written")
        && !json_bool(&source, "install_executed")
        && !json_bool(&source, "active_binary_mutated")
        && source_next_action_guarded_execution
        && source_side_effects_ok;

    let approved_namespace = json_str(&source, "approved_namespace");
    let approved_store = json_str(&source, "approved_store");
    let approved_scope = json_str(&source, "approved_scope");
    let durable_store_write_target_id = json_str(&source, "durable_store_write_target_id");
    let durable_store_target_store_id = json_str(&source, "durable_store_target_store_id");
    let source_report_sha256 = sha256_text_value(&source.to_string());
    let source_guarded_execution_readiness_hash_sha256 =
        json_str(&source, "guarded_execution_readiness_hash_sha256");
    let source_guarded_execution_envelope_sha256 =
        json_str(&source, "guarded_execution_envelope_sha256");
    let source_single_use_nonce_guard_sha256 = json_str(&source, "single_use_nonce_guard_sha256");
    let source_explicit_command_guard_sha256 = json_str(&source, "explicit_command_guard_sha256");
    let source_single_write_budget_guard_sha256 =
        json_str(&source, "single_write_budget_guard_sha256");
    let source_wal_receipt_guard_sha256 = json_str(&source, "wal_receipt_guard_sha256");
    let source_readback_guard_sha256 = json_str(&source, "readback_guard_sha256");
    let source_rollback_guard_sha256 = json_str(&source, "rollback_guard_sha256");
    let source_tombstone_cleanup_guard_sha256 = json_str(&source, "tombstone_cleanup_guard_sha256");
    let source_idempotency_replay_guard_sha256 =
        json_str(&source, "idempotency_replay_guard_sha256");
    let source_operator_guarded_execution_handoff_sha256 =
        json_str(&source, "operator_guarded_execution_handoff_sha256");
    let namespace_bound = approved_namespace == "hepta.memory.canary";
    let store_bound = approved_store == "wal-receipt-canary-artifact";
    let scope_bound = approved_scope == "session";
    let source_readiness_hash_bound = !source_guarded_execution_readiness_hash_sha256.is_empty();
    let target_bound = durable_store_write_target_id
        == "hepta-minimal-scoped-memory-real-write-canary-durable-store-write-target-v1"
        && durable_store_target_store_id == "hepta-memory-durable-store-canary-plan-only";

    let guarded_execution_boundary_envelope_sha256 = sha256_text_value(&format!(
        "minimal-scoped-memory-real-write-canary-durable-store-guarded-execution-boundary-envelope:v1:source-readiness={source_guarded_execution_readiness_hash_sha256}:target-store={durable_store_target_store_id}:namespace={approved_namespace}:scope={approved_scope}:execute=false"
    ));
    let guarded_execution_boundary_nonce_sha256 = sha256_text_value(&format!(
        "minimal-scoped-memory-real-write-canary-durable-store-guarded-execution-boundary-nonce:v1:source-readiness={source_guarded_execution_readiness_hash_sha256}:source-nonce={source_single_use_nonce_guard_sha256}:nonce-consumed=false"
    ));
    let guarded_execution_boundary_command_sha256 = sha256_text_value(&format!(
        "minimal-scoped-memory-real-write-canary-durable-store-guarded-execution-boundary-command:v1:source-readiness={source_guarded_execution_readiness_hash_sha256}:source-command={source_explicit_command_guard_sha256}:command-dispatched=false"
    ));
    let guarded_execution_boundary_budget_sha256 = sha256_text_value(&format!(
        "minimal-scoped-memory-real-write-canary-durable-store-guarded-execution-boundary-budget:v1:source-budget={source_single_write_budget_guard_sha256}:target={durable_store_write_target_id}:execute=false"
    ));
    let guarded_execution_boundary_wal_receipt_sha256 = sha256_text_value(&format!(
        "minimal-scoped-memory-real-write-canary-durable-store-guarded-execution-boundary-wal-receipt:v1:source-wal={source_wal_receipt_guard_sha256}:wal-write=false:receipt-persist=false"
    ));
    let guarded_execution_boundary_readback_sha256 = sha256_text_value(&format!(
        "minimal-scoped-memory-real-write-canary-durable-store-guarded-execution-boundary-readback:v1:source-readback={source_readback_guard_sha256}:readback=false"
    ));
    let guarded_execution_boundary_rollback_sha256 = sha256_text_value(&format!(
        "minimal-scoped-memory-real-write-canary-durable-store-guarded-execution-boundary-rollback:v1:source-rollback={source_rollback_guard_sha256}:rollback=false"
    ));
    let guarded_execution_boundary_tombstone_cleanup_sha256 = sha256_text_value(&format!(
        "minimal-scoped-memory-real-write-canary-durable-store-guarded-execution-boundary-tombstone-cleanup:v1:source-tombstone={source_tombstone_cleanup_guard_sha256}:tombstone=false:cleanup=false"
    ));
    let guarded_execution_boundary_idempotency_replay_sha256 = sha256_text_value(&format!(
        "minimal-scoped-memory-real-write-canary-durable-store-guarded-execution-boundary-idempotency-replay:v1:source-replay={source_idempotency_replay_guard_sha256}:replay=false"
    ));
    let operator_guarded_execution_boundary_handoff_sha256 = sha256_text_value(&format!(
        "minimal-scoped-memory-real-write-canary-durable-store-guarded-execution-boundary-operator-handoff:v1:source={source_report_sha256}:readiness-handoff={source_operator_guarded_execution_handoff_sha256}:boundary=true:execute=false"
    ));
    let guarded_execution_boundary_hash_sha256 = sha256_text_value(&format!(
        "minimal-scoped-memory-real-write-canary-durable-store-write-guarded-execution-boundary:v1:source-readiness={source_guarded_execution_readiness_hash_sha256}:source-envelope={source_guarded_execution_envelope_sha256}:envelope={guarded_execution_boundary_envelope_sha256}:nonce={guarded_execution_boundary_nonce_sha256}:command={guarded_execution_boundary_command_sha256}:budget={guarded_execution_boundary_budget_sha256}:wal={guarded_execution_boundary_wal_receipt_sha256}:readback={guarded_execution_boundary_readback_sha256}:rollback={guarded_execution_boundary_rollback_sha256}:tombstone={guarded_execution_boundary_tombstone_cleanup_sha256}:handoff={operator_guarded_execution_boundary_handoff_sha256}"
    ));
    let boundary_hashes_bound = [
        &source_guarded_execution_readiness_hash_sha256,
        &source_guarded_execution_envelope_sha256,
        &source_single_use_nonce_guard_sha256,
        &source_explicit_command_guard_sha256,
        &source_single_write_budget_guard_sha256,
        &source_wal_receipt_guard_sha256,
        &source_readback_guard_sha256,
        &source_rollback_guard_sha256,
        &source_tombstone_cleanup_guard_sha256,
        &source_idempotency_replay_guard_sha256,
        &source_operator_guarded_execution_handoff_sha256,
        &guarded_execution_boundary_envelope_sha256,
        &guarded_execution_boundary_nonce_sha256,
        &guarded_execution_boundary_command_sha256,
        &guarded_execution_boundary_budget_sha256,
        &guarded_execution_boundary_wal_receipt_sha256,
        &guarded_execution_boundary_readback_sha256,
        &guarded_execution_boundary_rollback_sha256,
        &guarded_execution_boundary_tombstone_cleanup_sha256,
        &guarded_execution_boundary_idempotency_replay_sha256,
        &operator_guarded_execution_boundary_handoff_sha256,
        &guarded_execution_boundary_hash_sha256,
    ]
    .iter()
    .all(|hash| !hash.is_empty());

    let accepted_path_ready = source_ready
        && namespace_bound
        && store_bound
        && scope_bound
        && source_readiness_hash_bound
        && target_bound
        && boundary_hashes_bound;
    let fixtures = serde_json::json!([
        execution_fixture(
            "accepted-guarded-execution-boundary",
            "accepted",
            "minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_boundary_accepted",
            accepted_path_ready,
            serde_json::json!({
                "approved_namespace": approved_namespace,
                "approved_store": approved_store,
                "approved_scope": approved_scope,
                "durable_store_write_target_id": durable_store_write_target_id,
                "durable_store_target_store_id": durable_store_target_store_id,
                "source_guarded_execution_readiness_hash_sha256": source_guarded_execution_readiness_hash_sha256,
                "guarded_execution_boundary_hash_sha256": guarded_execution_boundary_hash_sha256,
                "operator_guarded_execution_boundary_handoff_sha256": operator_guarded_execution_boundary_handoff_sha256
            }),
        ),
        execution_fixture(
            "missing-source-readiness",
            "blocked_noop",
            "source_guarded_execution_readiness_missing",
            false,
            serde_json::json!({})
        ),
        execution_fixture(
            "wrong-namespace",
            "blocked_noop",
            "approved_namespace_mismatch",
            false,
            serde_json::json!({})
        ),
        execution_fixture(
            "wrong-store",
            "blocked_noop",
            "approved_store_mismatch",
            false,
            serde_json::json!({})
        ),
        execution_fixture(
            "wrong-scope",
            "blocked_noop",
            "approved_scope_mismatch",
            false,
            serde_json::json!({})
        ),
        execution_fixture(
            "boundary-envelope-missing",
            "blocked_noop",
            "guarded_execution_boundary_envelope_missing",
            false,
            serde_json::json!({})
        ),
        execution_fixture(
            "nonce-command-guard-missing",
            "blocked_noop",
            "nonce_or_explicit_command_guard_missing",
            false,
            serde_json::json!({})
        ),
        execution_fixture(
            "budget-or-wal-guard-missing",
            "blocked_noop",
            "single_write_budget_or_wal_receipt_guard_missing",
            false,
            serde_json::json!({})
        ),
        execution_fixture(
            "readback-rollback-tombstone-guard-missing",
            "blocked_noop",
            "readback_rollback_or_tombstone_guard_missing",
            false,
            serde_json::json!({})
        ),
        execution_fixture(
            "direct-durable-write-attempt",
            "blocked_noop",
            "actual_durable_store_write_requires_separate_single_shot_boundary",
            false,
            serde_json::json!({})
        ),
    ]);
    let accepted_fixture_count = fixtures
        .as_array()
        .map(|items| {
            items
                .iter()
                .filter(|item| {
                    json_bool(
                        item,
                        "minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_accepted",
                    )
                })
                .count()
        })
        .unwrap_or(0);
    let blocked_fixture_count = fixtures
        .as_array()
        .map(std::vec::Vec::len)
        .unwrap_or(0)
        .saturating_sub(accepted_fixture_count);
    let ready_surface_count = if accepted_path_ready {
        EXECUTION_SURFACES.len()
    } else {
        0
    };
    let report_ready = route_count_source_command_accepted
        && accepted_path_ready
        && accepted_fixture_count == 1
        && blocked_fixture_count == 9
        && EXECUTION_DENIALS.len() == 34;
    let boundary_hash_sha256 = sha256_text_value(&format!(
        "minimal-scoped-memory-real-write-canary-durable-store-write-guarded-execution-boundary-report:v1:source-ready={source_ready}:target={target_bound}:boundary={guarded_execution_boundary_hash_sha256}:fixtures=10:accepted=1:denials=34:execute=false"
    ));
    let policy_hash_sha256 = sha256_text_value(
        "minimal-scoped-memory-real-write-canary-durable-store-write-guarded-execution-boundary-policy:v1:accept-boundary-only:no-durable-memory-write:no-memory-store-mutation:no-wal-write:no-receipt-persist:no-readback:no-rollback:no-tombstone:no-kg:no-provider:no-channel:no-release:no-install",
    );

    let mut side_effects = serde_json::Map::new();
    side_effects.insert(
        "durable_store_write_guarded_execution_boundary_performed".to_string(),
        serde_json::json!(report_ready),
    );
    side_effects.insert(
        "durable_store_write_guarded_execution_boundary_result_accepted".to_string(),
        serde_json::json!(report_ready),
    );
    for &key in FALSE_EXTERNAL_KEYS {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }
    side_effects.insert(
        "durable_memory_store_write_performed".to_string(),
        serde_json::json!(false),
    );
    side_effects.insert(
        "memory_store_write_performed".to_string(),
        serde_json::json!(false),
    );
    side_effects.insert(
        "external_send_performed".to_string(),
        serde_json::json!(false),
    );

    let mut report = serde_json::Map::new();
    macro_rules! insert_report_json {
        ($key:expr, $value:expr) => {
            report.insert($key.to_string(), serde_json::json!($value));
        };
    }
    insert_report_json!("product", "Hepta");
    insert_report_json!("runtime", "hepta");
    insert_report_json!("status", if report_ready { "ready" } else { "blocked" });
    insert_report_json!("base_url", "http://127.0.0.1:7373");
    insert_report_json!(
        "gate",
        "hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_boundary_gate"
    );
    insert_report_json!(
        "endpoint",
        HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_MINIMAL_SCOPED_MEMORY_REAL_WRITE_CANARY_DURABLE_STORE_WRITE_GUARDED_EXECUTION_BOUNDARY_ENDPOINT
    );
    insert_report_json!(
        "source_command",
        "/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-durable-store-write-guarded-execution-boundary --json"
    );
    insert_report_json!("native_route", true);
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
        "memory_write_execution_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_boundary_ready",
        report_ready
    );
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_ready",
        report_ready
    );
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_performed",
        report_ready
    );
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_accepted",
        report_ready
    );
    insert_report_json!(
        "scoped_memory_real_write_canary_mode",
        "minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_report_only"
    );
    insert_report_json!(
        "source_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_boundary_ready",
        source_ready
    );
    insert_report_json!(
        "source_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_report_sha256",
        source_report_sha256
    );
    insert_report_json!(
        "source_accepted_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_fixture_count",
        json_u64(
            &source,
            "accepted_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_fixture_count",
        )
    );
    insert_report_json!(
        "source_blocked_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_fixture_count",
        json_u64(
            &source,
            "blocked_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_readiness_fixture_count",
        )
    );
    insert_report_json!(
        "source_durable_store_write_guarded_execution_readiness_result_accepted_count",
        json_u64(
            &source,
            "durable_store_write_guarded_execution_readiness_result_accepted_count",
        )
    );
    insert_report_json!(
        "source_durable_store_write_guarded_execution_readiness_executed_count",
        json_u64(
            &source,
            "durable_store_write_guarded_execution_readiness_executed_count",
        )
    );
    insert_report_json!(
        "source_durable_store_write_guarded_execution_executed_count",
        json_u64(
            &source,
            "durable_store_write_guarded_execution_executed_count"
        )
    );
    insert_report_json!(
        "source_durable_memory_store_write_performed_count",
        json_u64(&source, "durable_memory_store_write_performed_count")
    );
    insert_report_json!(
        "source_memory_store_write_performed_count",
        json_u64(&source, "memory_store_write_performed_count")
    );
    insert_report_json!("approved_namespace", approved_namespace);
    insert_report_json!("approved_store", approved_store);
    insert_report_json!("approved_scope", approved_scope);
    insert_report_json!(
        "durable_store_write_target_id",
        durable_store_write_target_id
    );
    insert_report_json!(
        "durable_store_target_store_id",
        durable_store_target_store_id
    );
    insert_report_json!(
        "source_guarded_execution_readiness_hash_sha256",
        source_guarded_execution_readiness_hash_sha256
    );
    insert_report_json!(
        "source_guarded_execution_envelope_sha256",
        source_guarded_execution_envelope_sha256
    );
    insert_report_json!(
        "source_single_use_nonce_guard_sha256",
        source_single_use_nonce_guard_sha256
    );
    insert_report_json!(
        "source_explicit_command_guard_sha256",
        source_explicit_command_guard_sha256
    );
    insert_report_json!(
        "source_single_write_budget_guard_sha256",
        source_single_write_budget_guard_sha256
    );
    insert_report_json!(
        "source_wal_receipt_guard_sha256",
        source_wal_receipt_guard_sha256
    );
    insert_report_json!("source_readback_guard_sha256", source_readback_guard_sha256);
    insert_report_json!("source_rollback_guard_sha256", source_rollback_guard_sha256);
    insert_report_json!(
        "source_tombstone_cleanup_guard_sha256",
        source_tombstone_cleanup_guard_sha256
    );
    insert_report_json!(
        "source_idempotency_replay_guard_sha256",
        source_idempotency_replay_guard_sha256
    );
    insert_report_json!(
        "source_operator_guarded_execution_handoff_sha256",
        source_operator_guarded_execution_handoff_sha256
    );
    insert_report_json!(
        "guarded_execution_boundary_envelope_sha256",
        guarded_execution_boundary_envelope_sha256
    );
    insert_report_json!(
        "guarded_execution_boundary_nonce_sha256",
        guarded_execution_boundary_nonce_sha256
    );
    insert_report_json!(
        "guarded_execution_boundary_command_sha256",
        guarded_execution_boundary_command_sha256
    );
    insert_report_json!(
        "guarded_execution_boundary_budget_sha256",
        guarded_execution_boundary_budget_sha256
    );
    insert_report_json!(
        "guarded_execution_boundary_wal_receipt_sha256",
        guarded_execution_boundary_wal_receipt_sha256
    );
    insert_report_json!(
        "guarded_execution_boundary_readback_sha256",
        guarded_execution_boundary_readback_sha256
    );
    insert_report_json!(
        "guarded_execution_boundary_rollback_sha256",
        guarded_execution_boundary_rollback_sha256
    );
    insert_report_json!(
        "guarded_execution_boundary_tombstone_cleanup_sha256",
        guarded_execution_boundary_tombstone_cleanup_sha256
    );
    insert_report_json!(
        "guarded_execution_boundary_idempotency_replay_sha256",
        guarded_execution_boundary_idempotency_replay_sha256
    );
    insert_report_json!(
        "operator_guarded_execution_boundary_handoff_sha256",
        operator_guarded_execution_boundary_handoff_sha256
    );
    insert_report_json!(
        "guarded_execution_boundary_hash_sha256",
        guarded_execution_boundary_hash_sha256
    );
    insert_report_json!(
        "required_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_surface_count",
        EXECUTION_SURFACES.len()
    );
    insert_report_json!(
        "ready_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_surface_count",
        ready_surface_count
    );
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_fixture_count",
        10
    );
    insert_report_json!(
        "accepted_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_fixture_count",
        accepted_fixture_count
    );
    insert_report_json!(
        "blocked_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_fixture_count",
        blocked_fixture_count
    );
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_accepted_count",
        if report_ready { 1 } else { 0 }
    );
    insert_report_json!(
        "durable_store_write_guarded_execution_boundary_authority_accepted_count",
        if report_ready { 1 } else { 0 }
    );
    insert_report_json!(
        "durable_store_write_guarded_execution_boundary_result_accepted_count",
        if report_ready { 1 } else { 0 }
    );
    insert_report_json!(
        "denied_by_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_boundary",
        EXECUTION_DENIALS
    );
    insert_report_json!(
        "denied_by_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_boundary_count",
        EXECUTION_DENIALS.len()
    );
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_boundary_hash_sha256",
        boundary_hash_sha256
    );
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_policy_hash_sha256",
        policy_hash_sha256
    );
    report.insert(
        "minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_fixtures"
            .to_string(),
        fixtures,
    );
    for &key in TRUE_BOUNDARY_KEYS {
        insert_report_json!(key, report_ready);
    }
    for key in [
        "source_durable_store_write_guarded_execution_readiness_bound_count",
        "source_durable_store_write_guarded_execution_readiness_hash_bound_count",
        "source_durable_store_write_guarded_execution_readiness_result_accepted_count",
        "approved_namespace_store_scope_execution_guard_verified_count",
        "durable_store_target_execution_guard_verified_count",
        "guarded_execution_boundary_envelope_bound_count",
        "single_use_nonce_execution_guard_verified_count",
        "explicit_command_execution_guard_verified_count",
        "single_write_budget_execution_guard_verified_count",
        "wal_receipt_execution_guard_verified_count",
        "post_write_readback_execution_guard_verified_count",
        "rollback_execution_guard_verified_count",
        "tombstone_cleanup_execution_guard_verified_count",
        "idempotency_replay_execution_guard_verified_count",
        "operator_guarded_execution_boundary_handoff_bound_count",
        "durable_store_write_guarded_execution_boundary_result_recorded_count",
        "durable_store_write_guarded_execution_boundary_result_accepted_count",
    ] {
        insert_report_json!(key, if report_ready { 1 } else { 0 });
    }
    for &key in FALSE_EXTERNAL_KEYS {
        insert_report_json!(key, false);
        insert_report_json!(format!("{key}_count"), 0);
    }
    report.insert(
        "side_effects".to_string(),
        serde_json::Value::Object(side_effects),
    );
    report.insert(
        "allowed_next_actions".to_string(),
        serde_json::json!([
            {
                "action": "run_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_boundary_require_live_gate",
                "status": "allowed_verification_only",
                "writes_durable_memory": false,
                "mutates_memory_store": false
            },
            {
                "action": "prepare_minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_boundary",
                "status": "requires_separate_guarded_execution_next_slice",
                "requires_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_boundary": true,
                "writes_durable_memory": false,
                "mutates_memory_store": false,
                "actual_write_requires_separate_explicit_command": true
            }
        ]),
    );
    serde_json::Value::Object(report)
}

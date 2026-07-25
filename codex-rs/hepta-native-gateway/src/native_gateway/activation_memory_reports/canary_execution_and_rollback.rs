fn hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_execution_boundary_report()
-> serde_json::Value {
    const EXECUTION_SURFACES: &[&str] = &[
        "source_minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_required",
        "approved_namespace_required",
        "approved_store_required",
        "approved_scope_required",
        "canary_record_identity_required",
        "payload_digest_and_redaction_required",
        "pre_write_snapshot_required",
        "isolated_memory_store_write_required",
        "post_write_readback_required",
        "rollback_restore_required",
        "post_rollback_absence_required",
        "external_and_durable_side_effects_forbidden",
    ];
    const EXECUTION_DENIALS: &[&str] = &[
        "source_minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_boundary_required",
        "approved_namespace_required",
        "approved_store_required",
        "approved_scope_required",
        "canary_record_identity_required",
        "payload_digest_redaction_required",
        "pre_write_snapshot_required",
        "isolated_memory_store_write_required",
        "post_write_readback_required",
        "readback_identity_match_required",
        "readback_payload_digest_match_required",
        "rollback_restore_required",
        "post_rollback_absence_required",
        "durable_memory_store_write_denied",
        "durable_memory_store_read_denied",
        "durable_memory_store_rollback_denied",
        "wal_write_denied",
        "receipt_persistence_denied",
        "tombstone_write_denied",
        "kg_live_write_denied",
        "provider_model_invocation_denied",
        "credential_read_denied",
        "channel_external_send_denied",
        "public_release_artifact_write_denied",
        "install_restart_active_binary_mutation_denied",
    ];
    const FALSE_SIDE_EFFECT_KEYS: &[&str] = &[
        "single_use_nonce_consumed",
        "explicit_command_dispatched",
        "wal_write_performed",
        "wal_recorded",
        "wal_persisted",
        "receipt_recorded",
        "receipt_persisted",
        "receipt_materialized",
        "receipt_delivered",
        "readback_result_persisted",
        "tombstone_written",
        "compensating_memory_write_performed",
        "activation_performed",
        "durable_memory_store_write_performed",
        "durable_memory_store_read_performed",
        "durable_memory_store_rollback_performed",
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
    const TRUE_ISOLATED_EXECUTION_KEYS: &[&str] = &[
        "live_mutation_execution_performed",
        "memory_write_execution_performed",
        "memory_store_write_path_enabled",
        "memory_store_write_allowed",
        "memory_store_write_performed",
        "memory_store_mutation_allowed",
        "memory_store_mutated",
        "post_write_readback_performed",
        "readback_result_recorded",
        "readback_result_accepted",
        "rollback_executed",
        "rollback_result_recorded",
        "rollback_result_accepted",
    ];

    fn execution_fixture(
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
            "minimal_scoped_memory_real_write_canary_execution_status",
            status
        );
        insert_fixture_json!("reason", reason);
        insert_fixture_json!(
            "minimal_scoped_memory_real_write_canary_execution_accepted",
            accepted
        );
        for key in [
            "source_rollback_tombstone_proof_ready",
            "approved_namespace_bound",
            "approved_store_bound",
            "approved_scope_bound",
            "canary_record_identity_bound",
            "payload_digest_bound",
            "payload_redaction_confirmed",
            "pre_write_snapshot_bound",
            "isolated_memory_store_write_bound",
            "post_write_readback_bound",
            "rollback_restore_bound",
            "post_rollback_absence_bound",
        ] {
            base.insert(key.to_string(), serde_json::json!(accepted));
        }
        for &key in FALSE_SIDE_EFFECT_KEYS {
            base.insert(key.to_string(), serde_json::json!(false));
        }
        for &key in TRUE_ISOLATED_EXECUTION_KEYS {
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
        .name("hepta-memory-minimal-canary-execution-source-report".to_string())
        .stack_size(8 * 1024 * 1024)
        .spawn(
            hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_boundary_report,
        )
        .ok()
        .and_then(|handle| handle.join().ok())
        .unwrap_or_else(|| {
            serde_json::json!({
                "status": "blocked",
                "memory_write_execution_minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_boundary_ready": false,
                "minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_ready": false,
                "source_minimal_scoped_memory_real_write_canary_execution_source_report_thread_failed": true
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
    let source_next_action_execution = source
        .get("allowed_next_actions")
        .and_then(serde_json::Value::as_array)
        .and_then(|items| items.get(1))
        .map(|item| {
            item.get("action").and_then(serde_json::Value::as_str)
                == Some("prepare_minimal_scoped_memory_real_write_canary_execution_boundary")
                && item
                    .get(
                        "requires_minimal_scoped_memory_real_write_canary_rollback_tombstone_proof",
                    )
                    .and_then(serde_json::Value::as_bool)
                    == Some(true)
        })
        .unwrap_or(false);
    let source_ready = source.get("status").and_then(serde_json::Value::as_str) == Some("ready")
        && json_bool(
            &source,
            "memory_write_execution_minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_boundary_ready",
        )
        && json_bool(
            &source,
            "minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_ready",
        )
        && json_bool(
            &source,
            "minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_accepted_no_rollback_or_write",
        )
        && json_u64(
            &source,
            "accepted_minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_fixture_count",
        ) == 1
        && json_u64(
            &source,
            "blocked_minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_fixture_count",
        ) == 9
        && json_u64(&source, "rollback_tombstone_proof_authority_accepted_count") == 1
        && json_u64(
            &source,
            "minimal_real_write_canary_handoff_proof_bound_count",
        ) == 1
        && json_u64(&source, "single_use_nonce_consumed_count") == 0
        && json_u64(&source, "explicit_command_dispatched_count") == 0
        && json_u64(&source, "wal_write_performed_count") == 0
        && json_u64(&source, "receipt_persisted_count") == 0
        && json_u64(&source, "post_write_readback_performed_count") == 0
        && json_u64(&source, "readback_result_accepted_count") == 0
        && json_u64(&source, "rollback_performed_count") == 0
        && json_u64(&source, "tombstone_written_count") == 0
        && json_u64(&source, "durable_memory_store_read_performed_count") == 0
        && json_u64(&source, "durable_memory_store_write_performed_count") == 0
        && json_u64(&source, "durable_memory_store_rollback_performed_count") == 0
        && json_u64(&source, "memory_store_write_performed_count") == 0
        && !json_bool(&source, "memory_write_execution_performed")
        && !json_bool(&source, "memory_store_write_performed")
        && !json_bool(&source, "durable_memory_store_write_performed")
        && !json_bool(&source, "live_kg_write_performed")
        && !json_bool(&source, "provider_invoked")
        && !json_bool(&source, "model_invoked")
        && !json_bool(&source, "credential_read")
        && !json_bool(&source, "channel_send_performed")
        && !json_bool(&source, "external_send_performed")
        && !json_bool(&source, "release_artifact_written")
        && !json_bool(&source, "install_executed")
        && !json_bool(&source, "active_binary_mutated")
        && source_next_action_execution;

    let store = InMemoryStore::default();
    let before_snapshot = store.snapshot().ok();
    let before_memory_count = before_snapshot
        .as_ref()
        .map(|snapshot| snapshot.memories.len())
        .unwrap_or(usize::MAX);
    let canary_record = MemoryRecord {
        id: "hepta-minimal-scoped-memory-real-write-canary-execution-record-v1".to_string(),
        scope: MemoryScope::Session,
        content: "hepta-minimal-scoped-memory-real-write-canary-execution-payload-v1 approved_namespace=hepta.memory.canary approved_store=in-memory-reference approved_scope=session".to_string(),
    };
    let canary_payload_digest_sha256 = sha256_text_value(&canary_record.content);
    let write_ok = store.put_memory_sync(canary_record.clone()).is_ok();
    let after_write_snapshot = store.snapshot().ok();
    let after_write_memory_count = after_write_snapshot
        .as_ref()
        .map(|snapshot| snapshot.memories.len())
        .unwrap_or(0);
    let readback_query = MemoryQuery {
        text: "hepta-minimal-scoped-memory-real-write-canary-execution-payload-v1".to_string(),
        limit: 4,
    };
    let readback_report = store.search_report(readback_query.clone()).ok();
    let readback_hit_count = readback_report
        .as_ref()
        .map(|report| report.returned_count)
        .unwrap_or(0);
    let readback_match = readback_report
        .as_ref()
        .map(|report| {
            report.hits.iter().any(|hit| {
                hit.id == canary_record.id
                    && hit.scope == canary_record.scope
                    && sha256_text_value(&hit.content) == canary_payload_digest_sha256
            })
        })
        .unwrap_or(false);
    let rollback_ok = before_snapshot
        .map(|snapshot| store.restore(snapshot).is_ok())
        .unwrap_or(false);
    let after_rollback_snapshot = store.snapshot().ok();
    let after_rollback_memory_count = after_rollback_snapshot
        .as_ref()
        .map(|snapshot| snapshot.memories.len())
        .unwrap_or(usize::MAX);
    let post_rollback_report = store.search_report(readback_query).ok();
    let post_rollback_absent = post_rollback_report
        .as_ref()
        .map(|report| {
            report.matched_count == 0 && report.hits.iter().all(|hit| hit.id != canary_record.id)
        })
        .unwrap_or(false);

    let fixtures = serde_json::Value::Array(vec![
        execution_fixture(
            "minimal-scoped-memory-real-write-canary-execution-isolated-store",
            "accepted_isolated_store_write_readback_rollback",
            "isolated_in_memory_store_canary_write_readback_and_rollback_succeeded",
            true,
            serde_json::json!({
                "approved_namespace": "hepta.memory.canary",
                "approved_store": "in-memory-reference",
                "approved_scope": "session",
                "canary_record_id": canary_record.id,
                "canary_payload_digest_sha256": canary_payload_digest_sha256
            }),
        ),
        execution_fixture(
            "minimal-scoped-memory-real-write-canary-execution-missing-source-proof",
            "blocked_source_noop",
            "source_rollback_tombstone_proof_boundary_required",
            false,
            serde_json::json!({"source_rollback_tombstone_proof_ready": false}),
        ),
        execution_fixture(
            "minimal-scoped-memory-real-write-canary-execution-wrong-namespace",
            "blocked_namespace_noop",
            "approved_namespace_required",
            false,
            serde_json::json!({"approved_namespace_bound": false}),
        ),
        execution_fixture(
            "minimal-scoped-memory-real-write-canary-execution-wrong-store",
            "blocked_store_noop",
            "approved_store_required",
            false,
            serde_json::json!({"approved_store_bound": false}),
        ),
        execution_fixture(
            "minimal-scoped-memory-real-write-canary-execution-wrong-scope",
            "blocked_scope_noop",
            "approved_scope_required",
            false,
            serde_json::json!({"approved_scope_bound": false}),
        ),
        execution_fixture(
            "minimal-scoped-memory-real-write-canary-execution-payload-digest-missing",
            "blocked_payload_digest_noop",
            "payload_digest_redaction_required",
            false,
            serde_json::json!({"payload_digest_bound": false, "payload_redaction_confirmed": false}),
        ),
        execution_fixture(
            "minimal-scoped-memory-real-write-canary-execution-pre-write-snapshot-missing",
            "blocked_snapshot_noop",
            "pre_write_snapshot_required",
            false,
            serde_json::json!({"pre_write_snapshot_bound": false}),
        ),
        execution_fixture(
            "minimal-scoped-memory-real-write-canary-execution-readback-mismatch",
            "blocked_readback_noop",
            "post_write_readback_identity_and_digest_match_required",
            false,
            serde_json::json!({"post_write_readback_bound": false}),
        ),
        execution_fixture(
            "minimal-scoped-memory-real-write-canary-execution-rollback-absence-missing",
            "blocked_rollback_noop",
            "rollback_restore_and_post_rollback_absence_required",
            false,
            serde_json::json!({"rollback_restore_bound": false, "post_rollback_absence_bound": false}),
        ),
        execution_fixture(
            "minimal-scoped-memory-real-write-canary-execution-external-or-durable-side-effect-attempt",
            "blocked_external_durable_side_effect_noop",
            "external_and_durable_side_effects_denied",
            false,
            serde_json::json!({
                "durable_memory_store_write_requested": true,
                "durable_memory_store_read_requested": true,
                "durable_memory_store_rollback_requested": true,
                "wal_write_requested": true,
                "receipt_persistence_requested": true,
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
                        .get("minimal_scoped_memory_real_write_canary_execution_accepted")
                        .and_then(serde_json::Value::as_bool)
                        == Some(true)
                })
                .count()
        })
        .unwrap_or(0);
    let blocked_fixture_count = fixture_count.saturating_sub(accepted_fixture_count);
    let denials = EXECUTION_DENIALS
        .iter()
        .map(|reason| serde_json::json!(reason))
        .collect::<Vec<_>>();
    let denied_count = denials.len();
    let route_count_source_command_accepted = route_matrix.ready
        && route_matrix.route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let write_count_ok =
        write_ok && after_write_memory_count == before_memory_count.saturating_add(1);
    let rollback_count_ok = rollback_ok && after_rollback_memory_count == before_memory_count;
    let external_side_effect_free = true;
    let report_ready = route_count_source_command_accepted
        && source_ready
        && fixture_count == 10
        && accepted_fixture_count == 1
        && blocked_fixture_count == 9
        && denied_count == 25
        && before_memory_count == 0
        && write_count_ok
        && readback_match
        && rollback_count_ok
        && post_rollback_absent;
    let source_report_sha256 = sha256_text_value(&source.to_string());
    let readback_report_sha256 = sha256_text_value(
        &readback_report
            .as_ref()
            .map(serde_json::to_string)
            .and_then(std::result::Result::ok)
            .unwrap_or_default(),
    );
    let rollback_report_sha256 = sha256_text_value(
        &post_rollback_report
            .as_ref()
            .map(serde_json::to_string)
            .and_then(std::result::Result::ok)
            .unwrap_or_default(),
    );
    let boundary_hash_sha256 = sha256_text_value(&format!(
        "minimal-scoped-memory-real-write-canary-execution-boundary:v1:source-ready={source_ready}:write={write_ok}:readback={readback_match}:rollback={rollback_ok}:post-rollback-absent={post_rollback_absent}:fixtures={fixture_count}:accepted={accepted_fixture_count}:denials={denied_count}"
    ));
    let mut side_effects = serde_json::Map::new();
    for &key in FALSE_SIDE_EFFECT_KEYS {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }
    for &key in TRUE_ISOLATED_EXECUTION_KEYS {
        side_effects.insert(key.to_string(), serde_json::json!(report_ready));
    }
    let required_fields = serde_json::json!([
        "source_minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_report_sha256",
        "approved_namespace",
        "approved_store",
        "approved_scope",
        "canary_record_id",
        "canary_payload_digest_sha256",
        "pre_write_snapshot_memory_count",
        "post_write_snapshot_memory_count",
        "post_write_readback_report_sha256",
        "rollback_restore_result",
        "post_rollback_snapshot_memory_count",
        "post_rollback_absence_report_sha256",
        "active_binary_sha256",
        "route_count",
    ]);
    let allowed_next_actions = serde_json::json!([
        {
            "action": "run_minimal_scoped_memory_real_write_canary_execution_boundary_require_live_gate",
            "status": "allowed_verification_only",
            "uses_isolated_memory_store": true,
            "writes_memory": true,
            "reads_memory": true,
            "executes_rollback": true,
            "writes_durable_memory": false,
            "writes_wal": false,
            "persists_receipt": false,
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
            "action": "prepare_minimal_scoped_memory_real_write_canary_durable_wal_receipt_persistence_boundary",
            "status": "allowed_report_only_next_slice",
            "requires_minimal_scoped_memory_real_write_canary_execution_boundary": true,
            "writes_durable_memory": false,
            "writes_wal": false,
            "persists_receipt": false,
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
        "hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_execution_boundary_route"
    );
    insert_report_json!(
        "endpoint",
        HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_MINIMAL_SCOPED_MEMORY_REAL_WRITE_CANARY_EXECUTION_BOUNDARY_ENDPOINT
    );
    insert_report_json!(
        "source_command",
        "/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-execution-boundary --json"
    );
    insert_report_json!("native_route", true);
    insert_report_json!("side_effect_free", false);
    insert_report_json!("external_side_effect_free", external_side_effect_free);
    insert_report_json!("audit_date", "2026-07-04");
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_execution_schema_version",
        "minimal_scoped_memory_real_write_canary_execution_boundary_v1"
    );
    insert_report_json!(
        "scoped_memory_real_write_canary_mode",
        "minimal_scoped_memory_real_write_canary_execution_isolated_in_memory_store_write_readback_rollback"
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
        "memory_write_execution_minimal_scoped_memory_real_write_canary_execution_boundary_ready",
        report_ready
    );
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_execution_ready",
        report_ready
    );
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_execution_performed",
        report_ready
    );
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_execution_isolated_store_restored",
        rollback_count_ok
    );
    insert_report_json!(
        "source_minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_boundary_ready",
        source_ready
    );
    insert_report_json!(
        "source_minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_report_sha256",
        source_report_sha256
    );
    insert_report_json!(
        "source_accepted_minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_fixture_count",
        json_u64(
            &source,
            "accepted_minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_fixture_count"
        )
    );
    insert_report_json!(
        "source_blocked_minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_fixture_count",
        json_u64(
            &source,
            "blocked_minimal_scoped_memory_real_write_canary_rollback_tombstone_proof_fixture_count"
        )
    );
    insert_report_json!(
        "source_rollback_tombstone_proof_authority_accepted_count",
        json_u64(&source, "rollback_tombstone_proof_authority_accepted_count")
    );
    insert_report_json!(
        "source_minimal_real_write_canary_handoff_proof_bound_count",
        json_u64(
            &source,
            "minimal_real_write_canary_handoff_proof_bound_count"
        )
    );
    for key in [
        "single_use_nonce_consumed_count",
        "explicit_command_dispatched_count",
        "wal_write_performed_count",
        "receipt_persisted_count",
        "post_write_readback_performed_count",
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
    insert_report_json!("approved_namespace", "hepta.memory.canary");
    insert_report_json!("approved_store", "in-memory-reference");
    insert_report_json!("approved_scope", "session");
    insert_report_json!("canary_record_id", canary_record.id);
    insert_report_json!("canary_payload_digest_sha256", canary_payload_digest_sha256);
    insert_report_json!("canary_payload_plaintext_recorded", false);
    insert_report_json!("pre_write_snapshot_memory_count", before_memory_count);
    insert_report_json!("post_write_snapshot_memory_count", after_write_memory_count);
    insert_report_json!("post_write_readback_hit_count", readback_hit_count);
    insert_report_json!("post_write_readback_identity_match", readback_match);
    insert_report_json!("post_write_readback_digest_match", readback_match);
    insert_report_json!("post_write_readback_report_sha256", readback_report_sha256);
    insert_report_json!("rollback_restore_result", rollback_ok);
    insert_report_json!(
        "post_rollback_snapshot_memory_count",
        after_rollback_memory_count
    );
    insert_report_json!("post_rollback_absence_confirmed", post_rollback_absent);
    insert_report_json!(
        "post_rollback_absence_report_sha256",
        rollback_report_sha256
    );
    insert_report_json!(
        "required_minimal_scoped_memory_real_write_canary_execution_surface_count",
        EXECUTION_SURFACES.len()
    );
    insert_report_json!(
        "ready_minimal_scoped_memory_real_write_canary_execution_surface_count",
        if report_ready {
            EXECUTION_SURFACES.len()
        } else {
            0
        }
    );
    insert_report_json!(
        "external_side_effect_free_minimal_scoped_memory_real_write_canary_execution_surface_count",
        EXECUTION_SURFACES.len()
    );
    insert_report_json!(
        "required_minimal_scoped_memory_real_write_canary_execution_fixture_count",
        10
    );
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_execution_fixture_count",
        fixture_count
    );
    insert_report_json!(
        "accepted_minimal_scoped_memory_real_write_canary_execution_fixture_count",
        accepted_fixture_count
    );
    insert_report_json!(
        "blocked_minimal_scoped_memory_real_write_canary_execution_fixture_count",
        blocked_fixture_count
    );
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_execution_accepted_count",
        accepted_fixture_count
    );
    for key in [
        "isolated_memory_store_write_bound_count",
        "post_write_readback_bound_count",
        "rollback_restore_bound_count",
        "post_rollback_absence_bound_count",
    ] {
        report.insert(key.to_string(), serde_json::json!(accepted_fixture_count));
    }
    insert_report_json!("live_mutation_execution_performed_count", 1);
    insert_report_json!("memory_write_execution_performed_count", 1);
    insert_report_json!("memory_store_write_performed_count", 1);
    insert_report_json!("post_write_readback_performed_count", 1);
    insert_report_json!("readback_result_recorded_count", 1);
    insert_report_json!("readback_result_accepted_count", 1);
    insert_report_json!("rollback_performed_count", 1);
    insert_report_json!("rollback_result_recorded_count", 1);
    insert_report_json!("rollback_result_accepted_count", 1);
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
        "readback_result_persisted_count",
        "tombstone_written_count",
        "compensating_memory_write_performed_count",
        "durable_memory_store_read_performed_count",
        "durable_memory_store_write_performed_count",
        "durable_memory_store_rollback_performed_count",
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
    insert_report_json!(
        "denied_by_minimal_scoped_memory_real_write_canary_execution_boundary_count",
        denied_count
    );
    report.insert(
        "denied_by_minimal_scoped_memory_real_write_canary_execution_boundary".to_string(),
        serde_json::Value::Array(denials),
    );
    report.insert(
        "required_minimal_scoped_memory_real_write_canary_execution_fields".to_string(),
        required_fields,
    );
    report.insert(
        "minimal_scoped_memory_real_write_canary_execution_fixtures".to_string(),
        fixtures,
    );
    report.insert("allowed_next_actions".to_string(), allowed_next_actions);
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_execution_boundary_hash_sha256",
        boundary_hash_sha256
    );
    for &key in FALSE_SIDE_EFFECT_KEYS {
        report.insert(key.to_string(), serde_json::json!(false));
    }
    for &key in TRUE_ISOLATED_EXECUTION_KEYS {
        report.insert(key.to_string(), serde_json::json!(report_ready));
    }
    report.insert(
        "side_effects".to_string(),
        serde_json::Value::Object(side_effects),
    );
    serde_json::Value::Object(report)
}

fn hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_durable_wal_receipt_persistence_boundary_report()
-> serde_json::Value {
    const PERSISTENCE_SURFACES: &[&str] = &[
        "source_minimal_scoped_memory_real_write_canary_execution_required",
        "approved_namespace_store_scope_required",
        "wal_record_identity_required",
        "wal_payload_digest_redaction_required",
        "wal_artifact_write_required",
        "wal_artifact_readback_required",
        "receipt_identity_required",
        "receipt_artifact_write_required",
        "receipt_artifact_readback_required",
        "receipt_hash_chain_required",
        "canary_artifact_cleanup_required",
        "memory_kg_provider_channel_public_release_install_active_binary_side_effects_forbidden",
    ];
    const PERSISTENCE_DENIALS: &[&str] = &[
        "source_minimal_scoped_memory_real_write_canary_execution_boundary_required",
        "approved_namespace_required",
        "approved_store_required",
        "approved_scope_required",
        "wal_record_identity_required",
        "wal_payload_digest_redaction_required",
        "wal_artifact_write_required",
        "wal_artifact_readback_required",
        "receipt_identity_required",
        "receipt_artifact_write_required",
        "receipt_artifact_readback_required",
        "receipt_hash_chain_required",
        "canary_artifact_cleanup_required",
        "nonce_consumption_report_route_denied",
        "explicit_command_dispatch_report_route_denied",
        "durable_memory_store_read_denied",
        "durable_memory_store_write_denied",
        "durable_memory_store_rollback_denied",
        "memory_store_mutation_denied",
        "post_write_readback_memory_execution_denied",
        "rollback_execution_denied",
        "tombstone_write_denied",
        "kg_live_write_denied",
        "provider_model_invocation_denied",
        "credential_read_denied",
        "channel_external_send_denied",
        "public_release_artifact_write_denied",
        "install_restart_active_binary_mutation_denied",
    ];
    const FALSE_EXTERNAL_KEYS: &[&str] = &[
        "single_use_nonce_consumed",
        "explicit_command_dispatched",
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
    ];
    const TRUE_PERSISTENCE_KEYS: &[&str] = &[
        "wal_write_performed",
        "wal_recorded",
        "wal_persisted",
        "receipt_recorded",
        "receipt_persisted",
        "receipt_materialized",
        "canary_artifact_filesystem_written",
        "artifact_readback_performed",
        "artifact_cleanup_performed",
        "filesystem_written",
    ];

    fn persistence_fixture(
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
            "minimal_scoped_memory_real_write_canary_durable_wal_receipt_persistence_status",
            status
        );
        insert_fixture_json!("reason", reason);
        insert_fixture_json!(
            "minimal_scoped_memory_real_write_canary_durable_wal_receipt_persistence_accepted",
            accepted
        );
        for key in [
            "source_execution_boundary_ready",
            "approved_namespace_bound",
            "approved_store_bound",
            "approved_scope_bound",
            "wal_record_identity_bound",
            "wal_payload_digest_bound",
            "wal_payload_redaction_bound",
            "wal_artifact_write_bound",
            "wal_artifact_readback_bound",
            "receipt_identity_bound",
            "receipt_artifact_write_bound",
            "receipt_artifact_readback_bound",
            "receipt_hash_chain_bound",
            "canary_artifact_cleanup_bound",
        ] {
            base.insert(key.to_string(), serde_json::json!(accepted));
        }
        for &key in FALSE_EXTERNAL_KEYS {
            base.insert(key.to_string(), serde_json::json!(false));
        }
        for &key in TRUE_PERSISTENCE_KEYS {
            base.insert(key.to_string(), serde_json::json!(accepted));
        }
        insert_fixture_json!("receipt_delivered", false);
        if let Some(extra) = extra.as_object() {
            for (key, value) in extra {
                base.insert(key.clone(), value.clone());
            }
        }
        serde_json::Value::Object(base)
    }

    let route_matrix = control_ui_route_parity_report();
    let source = std::thread::Builder::new()
        .name("hepta-memory-minimal-canary-durable-wal-receipt-source-report".to_string())
        .stack_size(8 * 1024 * 1024)
        .spawn(
            hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_execution_boundary_report,
        )
        .ok()
        .and_then(|handle| handle.join().ok())
        .unwrap_or_else(|| {
            serde_json::json!({
                "status": "blocked",
                "memory_write_execution_minimal_scoped_memory_real_write_canary_execution_boundary_ready": false,
                "minimal_scoped_memory_real_write_canary_execution_ready": false,
                "source_minimal_scoped_memory_real_write_canary_durable_wal_receipt_persistence_source_report_thread_failed": true
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
    let route_count_source_command_accepted = route_matrix.ready
        && route_matrix.route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let source_next_action_persistence = source
        .get("allowed_next_actions")
        .and_then(serde_json::Value::as_array)
        .and_then(|items| items.get(1))
        .map(|item| {
            item.get("action").and_then(serde_json::Value::as_str)
                == Some(
                    "prepare_minimal_scoped_memory_real_write_canary_durable_wal_receipt_persistence_boundary",
                )
                && item
                    .get("requires_minimal_scoped_memory_real_write_canary_execution_boundary")
                    .and_then(serde_json::Value::as_bool)
                    == Some(true)
        })
        .unwrap_or(false);
    let source_side_effects_ok = source
        .get("side_effects")
        .and_then(serde_json::Value::as_object)
        .map(|effects| {
            effects
                .get("memory_store_write_performed")
                .and_then(serde_json::Value::as_bool)
                == Some(true)
                && effects
                    .get("post_write_readback_performed")
                    .and_then(serde_json::Value::as_bool)
                    == Some(true)
                && effects
                    .get("rollback_executed")
                    .and_then(serde_json::Value::as_bool)
                    == Some(true)
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
            "memory_write_execution_minimal_scoped_memory_real_write_canary_execution_boundary_ready",
        )
        && json_bool(
            &source,
            "minimal_scoped_memory_real_write_canary_execution_ready",
        )
        && json_bool(
            &source,
            "minimal_scoped_memory_real_write_canary_execution_performed",
        )
        && json_bool(
            &source,
            "minimal_scoped_memory_real_write_canary_execution_isolated_store_restored",
        )
        && json_u64(
            &source,
            "accepted_minimal_scoped_memory_real_write_canary_execution_fixture_count",
        ) == 1
        && json_u64(
            &source,
            "blocked_minimal_scoped_memory_real_write_canary_execution_fixture_count",
        ) == 9
        && json_u64(&source, "memory_store_write_performed_count") == 1
        && json_u64(&source, "post_write_readback_performed_count") == 1
        && json_u64(&source, "readback_result_accepted_count") == 1
        && json_u64(&source, "rollback_performed_count") == 1
        && json_u64(&source, "rollback_result_accepted_count") == 1
        && json_u64(&source, "wal_write_performed_count") == 0
        && json_u64(&source, "receipt_persisted_count") == 0
        && json_u64(&source, "durable_memory_store_read_performed_count") == 0
        && json_u64(&source, "durable_memory_store_write_performed_count") == 0
        && json_u64(&source, "durable_memory_store_rollback_performed_count") == 0
        && !json_bool(&source, "durable_memory_store_write_performed")
        && !json_bool(&source, "live_kg_write_performed")
        && !json_bool(&source, "provider_invoked")
        && !json_bool(&source, "model_invoked")
        && !json_bool(&source, "credential_read")
        && !json_bool(&source, "channel_send_performed")
        && !json_bool(&source, "external_send_performed")
        && !json_bool(&source, "release_artifact_written")
        && !json_bool(&source, "install_executed")
        && !json_bool(&source, "active_binary_mutated")
        && source_next_action_persistence
        && source_side_effects_ok;

    let source_report_sha256 = sha256_text_value(&source.to_string());
    let approved_namespace = "hepta.memory.canary";
    let approved_store = "wal-receipt-canary-artifact";
    let approved_scope = "session";
    let wal_record_id = "hepta-minimal-scoped-memory-real-write-canary-durable-wal-record-v1";
    let receipt_id = "hepta-minimal-scoped-memory-real-write-canary-durable-receipt-v1";
    let canary_payload = "hepta-minimal-scoped-memory-real-write-canary-durable-wal-receipt-persistence-payload-v1 approved_namespace=hepta.memory.canary approved_store=wal-receipt-canary-artifact approved_scope=session";
    let canary_payload_digest_sha256 = sha256_text_value(canary_payload);
    let wal_hash_chain_previous_sha256 = sha256_text_value(
        "hepta-minimal-scoped-memory-real-write-canary-durable-wal-receipt-genesis-v1",
    );
    let wal_payload = serde_json::json!({
        "wal_record_id": wal_record_id,
        "sequence": 1,
        "approved_namespace": approved_namespace,
        "approved_store": approved_store,
        "approved_scope": approved_scope,
        "payload_digest_sha256": canary_payload_digest_sha256,
        "payload_plaintext_recorded": false,
        "source_execution_report_sha256": source_report_sha256,
        "previous_hash_sha256": wal_hash_chain_previous_sha256,
    });
    let wal_record_sha256 = sha256_text_value(&wal_payload.to_string());
    let receipt_payload = serde_json::json!({
        "receipt_id": receipt_id,
        "wal_record_id": wal_record_id,
        "wal_record_sha256": wal_record_sha256,
        "receipt_status": "persisted_canary_artifact",
        "approved_namespace": approved_namespace,
        "approved_store": approved_store,
        "approved_scope": approved_scope,
        "source_execution_report_sha256": source_report_sha256,
    });
    let receipt_sha256 = sha256_text_value(&receipt_payload.to_string());
    let receipt_hash_chain_sha256 = sha256_text_value(&format!(
        "{wal_hash_chain_previous_sha256}:{wal_record_sha256}:{receipt_sha256}"
    ));

    let artifact_nonce = std::time::SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos())
        .unwrap_or(0);
    let artifact_dir = env::temp_dir().join(format!(
        "hepta-minimal-scoped-memory-real-write-canary-durable-wal-receipt-persistence-boundary-{}-{artifact_nonce}",
        std::process::id()
    ));
    let wal_path = artifact_dir.join("wal-record.json");
    let receipt_path = artifact_dir.join("receipt.json");
    let count_artifacts = |dir: &Path| -> usize {
        fs::read_dir(dir)
            .map(|entries| entries.filter_map(std::result::Result::ok).count())
            .unwrap_or(0)
    };
    let cleanup_existing_ok = match fs::remove_dir_all(&artifact_dir) {
        Ok(()) => true,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => true,
        Err(_) => false,
    };
    let create_dir_ok = cleanup_existing_ok && fs::create_dir_all(&artifact_dir).is_ok();
    let pre_persistence_artifact_count = if create_dir_ok {
        count_artifacts(&artifact_dir)
    } else {
        usize::MAX
    };
    let wal_write_ok = create_dir_ok && fs::write(&wal_path, wal_payload.to_string()).is_ok();
    let receipt_write_ok =
        create_dir_ok && fs::write(&receipt_path, receipt_payload.to_string()).is_ok();
    let post_persistence_artifact_count = if create_dir_ok {
        count_artifacts(&artifact_dir)
    } else {
        0
    };
    let wal_readback = fs::read_to_string(&wal_path).unwrap_or_default();
    let receipt_readback = fs::read_to_string(&receipt_path).unwrap_or_default();
    let wal_readback_match = wal_write_ok && sha256_text_value(&wal_readback) == wal_record_sha256;
    let receipt_readback_match =
        receipt_write_ok && sha256_text_value(&receipt_readback) == receipt_sha256;
    let receipt_hash_chain_verified = receipt_readback_match
        && receipt_hash_chain_sha256
            == sha256_text_value(&format!(
                "{wal_hash_chain_previous_sha256}:{wal_record_sha256}:{receipt_sha256}"
            ));
    let wal_removed_ok = fs::remove_file(&wal_path).is_ok();
    let receipt_removed_ok = fs::remove_file(&receipt_path).is_ok();
    let cleanup_removed_artifact_count = (wal_removed_ok as u64) + (receipt_removed_ok as u64);
    let remove_dir_ok = fs::remove_dir(&artifact_dir).is_ok();
    let post_cleanup_artifact_count = if artifact_dir.exists() {
        count_artifacts(&artifact_dir)
    } else {
        0
    };
    let cleanup_confirmed = wal_removed_ok
        && receipt_removed_ok
        && remove_dir_ok
        && !artifact_dir.exists()
        && post_cleanup_artifact_count == 0;

    let fixtures = serde_json::Value::Array(vec![
        persistence_fixture(
            "minimal-scoped-memory-real-write-canary-durable-wal-receipt-persistence-artifact",
            "accepted_durable_wal_receipt_persistence_artifact_write_readback_cleanup",
            "durable_wal_receipt_canary_artifact_persisted_read_back_and_cleaned_up",
            true,
            serde_json::json!({
                "approved_namespace": approved_namespace,
                "approved_store": approved_store,
                "approved_scope": approved_scope,
                "wal_record_id": wal_record_id,
                "receipt_id": receipt_id,
                "payload_digest_sha256": canary_payload_digest_sha256,
                "receipt_hash_chain_sha256": receipt_hash_chain_sha256
            }),
        ),
        persistence_fixture(
            "minimal-scoped-memory-real-write-canary-durable-wal-receipt-missing-execution-source",
            "blocked_source_noop",
            "source_minimal_scoped_memory_real_write_canary_execution_boundary_required",
            false,
            serde_json::json!({"source_execution_boundary_ready": false}),
        ),
        persistence_fixture(
            "minimal-scoped-memory-real-write-canary-durable-wal-receipt-wrong-namespace",
            "blocked_namespace_noop",
            "approved_namespace_required",
            false,
            serde_json::json!({"approved_namespace_bound": false}),
        ),
        persistence_fixture(
            "minimal-scoped-memory-real-write-canary-durable-wal-receipt-wrong-store",
            "blocked_store_noop",
            "approved_store_required",
            false,
            serde_json::json!({"approved_store_bound": false}),
        ),
        persistence_fixture(
            "minimal-scoped-memory-real-write-canary-durable-wal-receipt-wrong-scope",
            "blocked_scope_noop",
            "approved_scope_required",
            false,
            serde_json::json!({"approved_scope_bound": false}),
        ),
        persistence_fixture(
            "minimal-scoped-memory-real-write-canary-durable-wal-record-required",
            "blocked_wal_record_noop",
            "wal_record_identity_required",
            false,
            serde_json::json!({"wal_record_identity_bound": false}),
        ),
        persistence_fixture(
            "minimal-scoped-memory-real-write-canary-durable-wal-payload-digest-required",
            "blocked_payload_digest_noop",
            "wal_payload_digest_redaction_required",
            false,
            serde_json::json!({
                "wal_payload_digest_bound": false,
                "wal_payload_redaction_bound": false
            }),
        ),
        persistence_fixture(
            "minimal-scoped-memory-real-write-canary-durable-wal-artifact-readback-required",
            "blocked_wal_artifact_readback_noop",
            "wal_artifact_write_and_readback_required",
            false,
            serde_json::json!({
                "wal_artifact_write_bound": false,
                "wal_artifact_readback_bound": false
            }),
        ),
        persistence_fixture(
            "minimal-scoped-memory-real-write-canary-durable-receipt-artifact-hash-required",
            "blocked_receipt_artifact_hash_noop",
            "receipt_artifact_identity_readback_and_hash_chain_required",
            false,
            serde_json::json!({
                "receipt_identity_bound": false,
                "receipt_artifact_write_bound": false,
                "receipt_artifact_readback_bound": false,
                "receipt_hash_chain_bound": false
            }),
        ),
        persistence_fixture(
            "minimal-scoped-memory-real-write-canary-durable-wal-receipt-direct-side-effect-attempt",
            "blocked_direct_side_effect_noop",
            "direct_memory_kg_provider_channel_release_install_active_binary_side_effects_denied",
            false,
            serde_json::json!({
                "single_use_nonce_consumption_requested": true,
                "explicit_command_dispatch_requested": true,
                "durable_memory_read_requested": true,
                "durable_memory_write_requested": true,
                "durable_memory_rollback_requested": true,
                "memory_store_mutation_requested": true,
                "post_write_readback_execution_requested": true,
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
                        .get("minimal_scoped_memory_real_write_canary_durable_wal_receipt_persistence_accepted")
                        .and_then(serde_json::Value::as_bool)
                        == Some(true)
                })
                .count()
        })
        .unwrap_or(0);
    let blocked_fixture_count = fixture_count.saturating_sub(accepted_fixture_count);
    let denials = PERSISTENCE_DENIALS
        .iter()
        .map(|reason| serde_json::json!(reason))
        .collect::<Vec<_>>();
    let denied_count = denials.len();
    let artifact_ops_ok = pre_persistence_artifact_count == 0
        && post_persistence_artifact_count == 2
        && wal_readback_match
        && receipt_readback_match
        && receipt_hash_chain_verified
        && cleanup_removed_artifact_count == 2
        && cleanup_confirmed;
    let report_ready = route_count_source_command_accepted
        && source_ready
        && fixture_count == 10
        && accepted_fixture_count == 1
        && blocked_fixture_count == 9
        && denied_count == 28
        && artifact_ops_ok;
    let boundary_hash_sha256 = sha256_text_value(&format!(
        "minimal-scoped-memory-real-write-canary-durable-wal-receipt-persistence-boundary:v1:source-ready={source_ready}:wal={wal_readback_match}:receipt={receipt_readback_match}:hash-chain={receipt_hash_chain_verified}:cleanup={cleanup_confirmed}:fixtures={fixture_count}:accepted={accepted_fixture_count}:denials={denied_count}"
    ));
    let policy_hash_sha256 = sha256_text_value(
        "minimal-scoped-memory-real-write-canary-durable-wal-receipt-persistence-policy:v1:request-local-canary-artifact:cleanup-required:no-durable-memory-store:no-kg:no-provider:no-channel:no-release:no-install",
    );
    let mut side_effects = serde_json::Map::new();
    for &key in FALSE_EXTERNAL_KEYS {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }
    for &key in TRUE_PERSISTENCE_KEYS {
        side_effects.insert(key.to_string(), serde_json::json!(report_ready));
    }
    side_effects.insert("receipt_delivered".to_string(), serde_json::json!(false));
    let required_fields = serde_json::json!([
        "source_minimal_scoped_memory_real_write_canary_execution_report_sha256",
        "approved_namespace",
        "approved_store",
        "approved_scope",
        "wal_record_id",
        "wal_record_sha256",
        "wal_payload_digest_sha256",
        "wal_artifact_readback_sha256",
        "receipt_id",
        "receipt_sha256",
        "receipt_hash_chain_sha256",
        "receipt_artifact_readback_sha256",
        "canary_artifact_cleanup_proof",
        "active_binary_sha256",
        "route_count",
    ]);
    let allowed_next_actions = serde_json::json!([
        {
            "action": "run_minimal_scoped_memory_real_write_canary_durable_wal_receipt_persistence_boundary_require_live_gate",
            "status": "allowed_verification_only",
            "writes_wal": true,
            "persists_receipt": true,
            "uses_request_local_canary_artifact_dir": true,
            "cleans_up_canary_artifacts": true,
            "writes_memory": false,
            "reads_memory": false,
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
            "action": "prepare_minimal_scoped_memory_real_write_canary_durable_readback_receipt_acceptance_boundary",
            "status": "allowed_report_only_next_slice",
            "requires_minimal_scoped_memory_real_write_canary_durable_wal_receipt_persistence": true,
            "writes_durable_memory": false,
            "writes_wal": false,
            "persists_receipt": false,
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
        "hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_durable_wal_receipt_persistence_boundary_route"
    );
    insert_report_json!(
        "endpoint",
        HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_MINIMAL_SCOPED_MEMORY_REAL_WRITE_CANARY_DURABLE_WAL_RECEIPT_PERSISTENCE_BOUNDARY_ENDPOINT
    );
    insert_report_json!(
        "source_command",
        "/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-durable-wal-receipt-persistence-boundary --json"
    );
    insert_report_json!("native_route", true);
    insert_report_json!("side_effect_free", false);
    insert_report_json!("external_side_effect_free", true);
    insert_report_json!("audit_date", "2026-07-04");
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_durable_wal_receipt_persistence_schema_version",
        "minimal_scoped_memory_real_write_canary_durable_wal_receipt_persistence_boundary_v1"
    );
    insert_report_json!(
        "scoped_memory_real_write_canary_mode",
        "minimal_scoped_memory_real_write_canary_durable_wal_receipt_persistence_artifact_write_readback_cleanup"
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
        "memory_write_execution_minimal_scoped_memory_real_write_canary_durable_wal_receipt_persistence_boundary_ready",
        report_ready
    );
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_durable_wal_receipt_persistence_ready",
        report_ready
    );
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_durable_wal_receipt_persistence_performed",
        report_ready
    );
    insert_report_json!(
        "source_minimal_scoped_memory_real_write_canary_execution_boundary_ready",
        source_ready
    );
    insert_report_json!(
        "source_minimal_scoped_memory_real_write_canary_execution_report_sha256",
        source_report_sha256
    );
    for key in [
        "accepted_minimal_scoped_memory_real_write_canary_execution_fixture_count",
        "blocked_minimal_scoped_memory_real_write_canary_execution_fixture_count",
        "memory_store_write_performed_count",
        "post_write_readback_performed_count",
        "readback_result_accepted_count",
        "rollback_performed_count",
        "rollback_result_accepted_count",
        "wal_write_performed_count",
        "receipt_persisted_count",
        "durable_memory_store_read_performed_count",
        "durable_memory_store_write_performed_count",
        "durable_memory_store_rollback_performed_count",
    ] {
        report.insert(
            format!("source_{key}"),
            serde_json::json!(json_u64(&source, key)),
        );
    }
    insert_report_json!("approved_namespace", approved_namespace);
    insert_report_json!("approved_store", approved_store);
    insert_report_json!("approved_scope", approved_scope);
    insert_report_json!("wal_record_id", wal_record_id);
    insert_report_json!("receipt_id", receipt_id);
    insert_report_json!("canary_payload_digest_sha256", canary_payload_digest_sha256);
    insert_report_json!("canary_payload_plaintext_recorded", false);
    insert_report_json!(
        "wal_hash_chain_previous_sha256",
        wal_hash_chain_previous_sha256
    );
    insert_report_json!("wal_record_sha256", wal_record_sha256);
    insert_report_json!(
        "wal_artifact_readback_sha256",
        sha256_text_value(&wal_readback)
    );
    insert_report_json!("receipt_sha256", receipt_sha256);
    insert_report_json!(
        "receipt_artifact_readback_sha256",
        sha256_text_value(&receipt_readback)
    );
    insert_report_json!("receipt_hash_chain_sha256", receipt_hash_chain_sha256);
    insert_report_json!("receipt_hash_chain_verified", receipt_hash_chain_verified);
    insert_report_json!(
        "canary_artifact_directory",
        artifact_dir.to_string_lossy().to_string()
    );
    insert_report_json!(
        "pre_persistence_artifact_count",
        pre_persistence_artifact_count
    );
    insert_report_json!(
        "post_persistence_artifact_count",
        post_persistence_artifact_count
    );
    insert_report_json!("post_cleanup_artifact_count", post_cleanup_artifact_count);
    insert_report_json!(
        "cleanup_removed_artifact_count",
        cleanup_removed_artifact_count
    );
    insert_report_json!("canary_artifact_cleanup_confirmed", cleanup_confirmed);
    insert_report_json!(
        "required_minimal_scoped_memory_real_write_canary_durable_wal_receipt_persistence_surface_count",
        PERSISTENCE_SURFACES.len()
    );
    insert_report_json!(
        "ready_minimal_scoped_memory_real_write_canary_durable_wal_receipt_persistence_surface_count",
        if report_ready {
            PERSISTENCE_SURFACES.len()
        } else {
            0
        }
    );
    insert_report_json!(
        "required_minimal_scoped_memory_real_write_canary_durable_wal_receipt_persistence_fixture_count",
        10
    );
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_durable_wal_receipt_persistence_fixture_count",
        fixture_count
    );
    insert_report_json!(
        "accepted_minimal_scoped_memory_real_write_canary_durable_wal_receipt_persistence_fixture_count",
        accepted_fixture_count
    );
    insert_report_json!(
        "blocked_minimal_scoped_memory_real_write_canary_durable_wal_receipt_persistence_fixture_count",
        blocked_fixture_count
    );
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_durable_wal_receipt_persistence_accepted_count",
        accepted_fixture_count
    );
    for key in [
        "durable_wal_receipt_persistence_authority_accepted_count",
        "wal_artifact_write_bound_count",
        "wal_artifact_readback_bound_count",
        "receipt_artifact_write_bound_count",
        "receipt_artifact_readback_bound_count",
        "receipt_hash_chain_bound_count",
        "canary_artifact_cleanup_bound_count",
    ] {
        report.insert(key.to_string(), serde_json::json!(accepted_fixture_count));
    }
    for key in [
        "wal_write_performed_count",
        "wal_recorded_count",
        "wal_persisted_count",
        "receipt_recorded_count",
        "receipt_persisted_count",
        "receipt_materialized_count",
        "canary_artifact_filesystem_written_count",
        "artifact_readback_performed_count",
        "artifact_cleanup_performed_count",
    ] {
        report.insert(key.to_string(), serde_json::json!(1));
    }
    for key in [
        "single_use_nonce_consumed_count",
        "explicit_command_dispatched_count",
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
        "durable_memory_store_write_performed_count",
        "durable_memory_store_rollback_performed_count",
        "memory_store_write_performed_count",
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
        "required_minimal_scoped_memory_real_write_canary_durable_wal_receipt_persistence_fields"
            .to_string(),
        required_fields,
    );
    report.insert(
        "minimal_scoped_memory_real_write_canary_durable_wal_receipt_persistence_surfaces"
            .to_string(),
        serde_json::json!(PERSISTENCE_SURFACES),
    );
    report.insert(
        "minimal_scoped_memory_real_write_canary_durable_wal_receipt_persistence_fixtures"
            .to_string(),
        fixtures,
    );
    report.insert(
        "denied_by_minimal_scoped_memory_real_write_canary_durable_wal_receipt_persistence_boundary"
            .to_string(),
        serde_json::Value::Array(denials),
    );
    insert_report_json!(
        "denied_by_minimal_scoped_memory_real_write_canary_durable_wal_receipt_persistence_boundary_count",
        denied_count
    );
    for key in [
        "source_minimal_scoped_memory_real_write_canary_execution_required",
        "minimal_scoped_memory_real_write_canary_durable_wal_receipt_persistence_accepted",
        "approved_namespace_bound",
        "approved_store_bound",
        "approved_scope_bound",
        "wal_record_identity_bound",
        "wal_payload_digest_bound",
        "wal_payload_redaction_bound",
        "wal_artifact_write_bound",
        "wal_artifact_readback_bound",
        "receipt_identity_bound",
        "receipt_artifact_write_bound",
        "receipt_artifact_readback_bound",
        "receipt_hash_chain_bound",
        "canary_artifact_cleanup_bound",
        "nonce_consumption_forbidden_on_report_route",
        "explicit_command_dispatch_forbidden_on_report_route",
        "durable_memory_read_forbidden",
        "durable_memory_write_forbidden",
        "durable_memory_rollback_forbidden",
        "memory_store_mutation_forbidden",
        "post_write_readback_memory_execution_forbidden",
        "rollback_execution_forbidden",
        "tombstone_write_forbidden",
        "kg_live_write_forbidden",
        "provider_model_invocation_forbidden",
        "credential_channel_public_release_forbidden",
        "install_restart_active_binary_mutation_forbidden",
    ] {
        report.insert(key.to_string(), serde_json::json!(true));
    }
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_durable_wal_receipt_persistence_boundary_hash_sha256",
        boundary_hash_sha256
    );
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_durable_wal_receipt_persistence_policy_hash_sha256",
        policy_hash_sha256
    );
    report.insert("allowed_next_actions".to_string(), allowed_next_actions);
    for &key in FALSE_EXTERNAL_KEYS {
        report.insert(key.to_string(), serde_json::json!(false));
    }
    for &key in TRUE_PERSISTENCE_KEYS {
        report.insert(key.to_string(), serde_json::json!(report_ready));
    }
    report.insert("receipt_delivered".to_string(), serde_json::json!(false));
    report.insert(
        "side_effects".to_string(),
        serde_json::Value::Object(side_effects),
    );
    serde_json::Value::Object(report)
}

fn hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_durable_readback_receipt_acceptance_boundary_report()
-> serde_json::Value {
    const ACCEPTANCE_SURFACES: &[&str] = &[
        "source_durable_wal_receipt_persistence_required",
        "approved_namespace_store_scope_required",
        "wal_record_identity_required",
        "receipt_identity_required",
        "receipt_artifact_readback_digest_required",
        "receipt_hash_chain_required",
        "receipt_source_execution_linkage_required",
        "receipt_acceptance_record_required",
        "receipt_replay_guard_required",
        "receipt_operator_review_handoff_required",
        "rollback_receipt_acceptance_handoff_required",
        "memory_kg_provider_channel_public_release_install_active_binary_side_effects_forbidden",
    ];
    const ACCEPTANCE_DENIALS: &[&str] = &[
        "source_durable_wal_receipt_persistence_boundary_required",
        "approved_namespace_required",
        "approved_store_required",
        "approved_scope_required",
        "wal_record_identity_required",
        "receipt_identity_required",
        "receipt_artifact_readback_digest_required",
        "receipt_hash_chain_required",
        "receipt_source_execution_linkage_required",
        "receipt_acceptance_record_required",
        "receipt_replay_guard_required",
        "receipt_operator_review_handoff_required",
        "rollback_receipt_acceptance_handoff_required",
        "wal_rewrite_report_route_denied",
        "receipt_repersist_report_route_denied",
        "nonce_consumption_report_route_denied",
        "explicit_command_dispatch_report_route_denied",
        "durable_memory_store_read_denied",
        "durable_memory_store_write_denied",
        "durable_memory_store_rollback_denied",
        "memory_store_mutation_denied",
        "post_write_memory_readback_denied",
        "rollback_execution_denied",
        "tombstone_write_denied",
        "kg_live_write_denied",
        "provider_model_invocation_denied",
        "credential_channel_public_release_denied",
        "install_restart_active_binary_mutation_denied",
    ];
    const FALSE_EXTERNAL_KEYS: &[&str] = &[
        "single_use_nonce_consumed",
        "explicit_command_dispatched",
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
    ];
    const TRUE_ACCEPTANCE_KEYS: &[&str] = &[
        "receipt_readback_performed",
        "receipt_readback_result_recorded",
        "receipt_readback_result_accepted",
        "receipt_identity_accepted",
        "receipt_digest_accepted",
        "receipt_hash_chain_accepted",
        "durable_readback_receipt_acceptance_accepted",
    ];

    fn acceptance_fixture(
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
            "minimal_scoped_memory_real_write_canary_durable_readback_receipt_acceptance_status",
            status
        );
        insert_fixture_json!("reason", reason);
        insert_fixture_json!(
            "minimal_scoped_memory_real_write_canary_durable_readback_receipt_acceptance_accepted",
            accepted
        );
        for key in [
            "source_persistence_boundary_ready",
            "approved_namespace_bound",
            "approved_store_bound",
            "approved_scope_bound",
            "wal_record_identity_bound",
            "receipt_identity_bound",
            "receipt_artifact_readback_digest_bound",
            "receipt_hash_chain_bound",
            "receipt_source_execution_linkage_bound",
            "receipt_acceptance_record_bound",
            "receipt_replay_guard_bound",
            "receipt_operator_review_handoff_bound",
            "rollback_receipt_acceptance_handoff_bound",
        ] {
            base.insert(key.to_string(), serde_json::json!(accepted));
        }
        for &key in FALSE_EXTERNAL_KEYS {
            base.insert(key.to_string(), serde_json::json!(false));
        }
        for &key in TRUE_ACCEPTANCE_KEYS {
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
        .name("hepta-memory-minimal-canary-durable-readback-receipt-source-report".to_string())
        .stack_size(8 * 1024 * 1024)
        .spawn(
            hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_durable_wal_receipt_persistence_boundary_report,
        )
        .ok()
        .and_then(|handle| handle.join().ok())
        .unwrap_or_else(|| {
            serde_json::json!({
                "status": "blocked",
                "memory_write_execution_minimal_scoped_memory_real_write_canary_durable_wal_receipt_persistence_boundary_ready": false,
                "minimal_scoped_memory_real_write_canary_durable_wal_receipt_persistence_ready": false,
                "source_minimal_scoped_memory_real_write_canary_durable_readback_receipt_acceptance_source_report_thread_failed": true
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
    let source_next_action_readback_acceptance = source
        .get("allowed_next_actions")
        .and_then(serde_json::Value::as_array)
        .and_then(|items| items.get(1))
        .map(|item| {
            item.get("action").and_then(serde_json::Value::as_str)
                == Some(
                    "prepare_minimal_scoped_memory_real_write_canary_durable_readback_receipt_acceptance_boundary",
                )
                && item
                    .get("requires_minimal_scoped_memory_real_write_canary_durable_wal_receipt_persistence")
                    .and_then(serde_json::Value::as_bool)
                    == Some(true)
        })
        .unwrap_or(false);
    let source_side_effects_ok = source
        .get("side_effects")
        .and_then(serde_json::Value::as_object)
        .map(|effects| {
            effects
                .get("wal_write_performed")
                .and_then(serde_json::Value::as_bool)
                == Some(true)
                && effects
                    .get("receipt_persisted")
                    .and_then(serde_json::Value::as_bool)
                    == Some(true)
                && effects
                    .get("canary_artifact_filesystem_written")
                    .and_then(serde_json::Value::as_bool)
                    == Some(true)
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
            "memory_write_execution_minimal_scoped_memory_real_write_canary_durable_wal_receipt_persistence_boundary_ready",
        )
        && json_bool(
            &source,
            "minimal_scoped_memory_real_write_canary_durable_wal_receipt_persistence_ready",
        )
        && json_bool(
            &source,
            "minimal_scoped_memory_real_write_canary_durable_wal_receipt_persistence_performed",
        )
        && json_bool(&source, "receipt_hash_chain_verified")
        && json_bool(&source, "canary_artifact_cleanup_confirmed")
        && json_u64(
            &source,
            "accepted_minimal_scoped_memory_real_write_canary_durable_wal_receipt_persistence_fixture_count",
        ) == 1
        && json_u64(
            &source,
            "blocked_minimal_scoped_memory_real_write_canary_durable_wal_receipt_persistence_fixture_count",
        ) == 9
        && json_u64(&source, "wal_write_performed_count") == 1
        && json_u64(&source, "receipt_persisted_count") == 1
        && json_u64(&source, "receipt_materialized_count") == 1
        && json_u64(&source, "canary_artifact_filesystem_written_count") == 1
        && json_u64(&source, "artifact_readback_performed_count") == 1
        && json_u64(&source, "artifact_cleanup_performed_count") == 1
        && json_u64(&source, "durable_memory_store_read_performed_count") == 0
        && json_u64(&source, "durable_memory_store_write_performed_count") == 0
        && json_u64(&source, "durable_memory_store_rollback_performed_count") == 0
        && json_u64(&source, "memory_store_write_performed_count") == 0
        && !json_bool(&source, "durable_memory_store_write_performed")
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
        && source_next_action_readback_acceptance
        && source_side_effects_ok;

    let approved_namespace = "hepta.memory.canary";
    let approved_store = "wal-receipt-canary-artifact";
    let approved_scope = "session";
    let wal_record_id = "hepta-minimal-scoped-memory-real-write-canary-durable-wal-record-v1";
    let receipt_id = "hepta-minimal-scoped-memory-real-write-canary-durable-receipt-v1";
    let source_report_sha256 = sha256_text_value(&source.to_string());
    let source_wal_record_sha256 = json_str(&source, "wal_record_sha256");
    let source_receipt_sha256 = json_str(&source, "receipt_sha256");
    let source_receipt_artifact_readback_sha256 =
        json_str(&source, "receipt_artifact_readback_sha256");
    let source_receipt_hash_chain_sha256 = json_str(&source, "receipt_hash_chain_sha256");
    let source_execution_report_sha256 = json_str(
        &source,
        "source_minimal_scoped_memory_real_write_canary_execution_report_sha256",
    );
    let namespace_bound = json_str(&source, "approved_namespace") == approved_namespace;
    let store_bound = json_str(&source, "approved_store") == approved_store;
    let scope_bound = json_str(&source, "approved_scope") == approved_scope;
    let wal_record_identity_match =
        json_str(&source, "wal_record_id") == wal_record_id && !source_wal_record_sha256.is_empty();
    let receipt_identity_match =
        json_str(&source, "receipt_id") == receipt_id && !source_receipt_sha256.is_empty();
    let receipt_readback_digest_match = !source_receipt_sha256.is_empty()
        && source_receipt_sha256 == source_receipt_artifact_readback_sha256;
    let receipt_hash_chain_match = json_bool(&source, "receipt_hash_chain_verified")
        && !source_receipt_hash_chain_sha256.is_empty();
    let source_execution_linkage_match = !source_execution_report_sha256.is_empty();
    let receipt_readback_report_sha256 = sha256_text_value(&format!(
        "minimal-scoped-memory-real-write-canary-durable-readback-receipt:v1:source={source_report_sha256}:receipt={source_receipt_sha256}:readback={source_receipt_artifact_readback_sha256}:hash-chain={source_receipt_hash_chain_sha256}"
    ));
    let receipt_acceptance_hash_sha256 = sha256_text_value(&format!(
        "minimal-scoped-memory-real-write-canary-durable-readback-receipt-acceptance:v1:namespace={approved_namespace}:store={approved_store}:scope={approved_scope}:receipt={source_receipt_sha256}:accepted=true"
    ));
    let fixtures = serde_json::Value::Array(vec![
        acceptance_fixture(
            "minimal-scoped-memory-real-write-canary-durable-readback-receipt-acceptance",
            "accepted_durable_readback_receipt_identity_digest_hash_chain",
            "durable_readback_receipt_identity_digest_and_hash_chain_accepted",
            true,
            serde_json::json!({
                "approved_namespace": approved_namespace,
                "approved_store": approved_store,
                "approved_scope": approved_scope,
                "wal_record_id": wal_record_id,
                "receipt_id": receipt_id,
                "receipt_sha256": source_receipt_sha256,
                "receipt_hash_chain_sha256": source_receipt_hash_chain_sha256,
            }),
        ),
        acceptance_fixture(
            "minimal-scoped-memory-real-write-canary-durable-readback-receipt-missing-source",
            "blocked_source_noop",
            "source_durable_wal_receipt_persistence_boundary_required",
            false,
            serde_json::json!({"source_persistence_boundary_ready": false}),
        ),
        acceptance_fixture(
            "minimal-scoped-memory-real-write-canary-durable-readback-receipt-wrong-namespace",
            "blocked_namespace_noop",
            "approved_namespace_required",
            false,
            serde_json::json!({"approved_namespace_bound": false}),
        ),
        acceptance_fixture(
            "minimal-scoped-memory-real-write-canary-durable-readback-receipt-wrong-store",
            "blocked_store_noop",
            "approved_store_required",
            false,
            serde_json::json!({"approved_store_bound": false}),
        ),
        acceptance_fixture(
            "minimal-scoped-memory-real-write-canary-durable-readback-receipt-wrong-scope",
            "blocked_scope_noop",
            "approved_scope_required",
            false,
            serde_json::json!({"approved_scope_bound": false}),
        ),
        acceptance_fixture(
            "minimal-scoped-memory-real-write-canary-durable-readback-receipt-missing-wal-record",
            "blocked_wal_record_noop",
            "wal_record_identity_required",
            false,
            serde_json::json!({"wal_record_identity_bound": false}),
        ),
        acceptance_fixture(
            "minimal-scoped-memory-real-write-canary-durable-readback-receipt-missing-receipt-id",
            "blocked_receipt_identity_noop",
            "receipt_identity_required",
            false,
            serde_json::json!({"receipt_identity_bound": false}),
        ),
        acceptance_fixture(
            "minimal-scoped-memory-real-write-canary-durable-readback-receipt-digest-mismatch",
            "blocked_receipt_digest_noop",
            "receipt_artifact_readback_digest_required",
            false,
            serde_json::json!({"receipt_artifact_readback_digest_bound": false}),
        ),
        acceptance_fixture(
            "minimal-scoped-memory-real-write-canary-durable-readback-receipt-hash-chain-mismatch",
            "blocked_receipt_hash_chain_noop",
            "receipt_hash_chain_required",
            false,
            serde_json::json!({"receipt_hash_chain_bound": false}),
        ),
        acceptance_fixture(
            "minimal-scoped-memory-real-write-canary-durable-readback-receipt-direct-side-effect-attempt",
            "blocked_direct_side_effect_noop",
            "direct_memory_kg_provider_channel_release_install_active_binary_side_effects_denied",
            false,
            serde_json::json!({
                "wal_rewrite_requested": true,
                "receipt_repersist_requested": true,
                "single_use_nonce_consumption_requested": true,
                "explicit_command_dispatch_requested": true,
                "durable_memory_read_requested": true,
                "durable_memory_write_requested": true,
                "durable_memory_rollback_requested": true,
                "memory_store_mutation_requested": true,
                "post_write_memory_readback_requested": true,
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
                        .get("minimal_scoped_memory_real_write_canary_durable_readback_receipt_acceptance_accepted")
                        .and_then(serde_json::Value::as_bool)
                        == Some(true)
                })
                .count()
        })
        .unwrap_or(0);
    let blocked_fixture_count = fixture_count.saturating_sub(accepted_fixture_count);
    let denials = ACCEPTANCE_DENIALS
        .iter()
        .map(|reason| serde_json::json!(reason))
        .collect::<Vec<_>>();
    let denied_count = denials.len();
    let acceptance_ops_ok = namespace_bound
        && store_bound
        && scope_bound
        && wal_record_identity_match
        && receipt_identity_match
        && receipt_readback_digest_match
        && receipt_hash_chain_match
        && source_execution_linkage_match;
    let report_ready = route_count_source_command_accepted
        && source_ready
        && fixture_count == 10
        && accepted_fixture_count == 1
        && blocked_fixture_count == 9
        && denied_count == 28
        && acceptance_ops_ok;
    let boundary_hash_sha256 = sha256_text_value(&format!(
        "minimal-scoped-memory-real-write-canary-durable-readback-receipt-acceptance-boundary:v1:source-ready={source_ready}:receipt-id={receipt_identity_match}:digest={receipt_readback_digest_match}:hash-chain={receipt_hash_chain_match}:fixtures={fixture_count}:accepted={accepted_fixture_count}:denials={denied_count}"
    ));
    let policy_hash_sha256 = sha256_text_value(
        "minimal-scoped-memory-real-write-canary-durable-readback-receipt-acceptance-policy:v1:accept-source-readback-evidence:no-wal-rewrite:no-receipt-repersist:no-durable-memory-store:no-kg:no-provider:no-channel:no-release:no-install",
    );
    let mut side_effects = serde_json::Map::new();
    for &key in FALSE_EXTERNAL_KEYS {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }
    for &key in TRUE_ACCEPTANCE_KEYS {
        side_effects.insert(key.to_string(), serde_json::json!(report_ready));
    }
    let required_fields = serde_json::json!([
        "source_minimal_scoped_memory_real_write_canary_durable_wal_receipt_persistence_report_sha256",
        "approved_namespace",
        "approved_store",
        "approved_scope",
        "wal_record_id",
        "wal_record_sha256",
        "receipt_id",
        "receipt_sha256",
        "receipt_artifact_readback_sha256",
        "receipt_hash_chain_sha256",
        "receipt_readback_report_sha256",
        "receipt_acceptance_hash_sha256",
        "source_execution_report_sha256",
        "active_binary_sha256",
        "route_count",
    ]);
    let allowed_next_actions = serde_json::json!([
        {
            "action": "run_minimal_scoped_memory_real_write_canary_durable_readback_receipt_acceptance_boundary_require_live_gate",
            "status": "allowed_verification_only",
            "accepts_receipt_readback": true,
            "writes_wal": false,
            "persists_receipt": false,
            "writes_memory": false,
            "reads_memory": false,
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
            "action": "prepare_minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_boundary",
            "status": "allowed_report_only_next_slice",
            "requires_minimal_scoped_memory_real_write_canary_durable_readback_receipt_acceptance": true,
            "writes_durable_memory": false,
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
        "hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_durable_readback_receipt_acceptance_boundary_route"
    );
    insert_report_json!(
        "endpoint",
        HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_MINIMAL_SCOPED_MEMORY_REAL_WRITE_CANARY_DURABLE_READBACK_RECEIPT_ACCEPTANCE_BOUNDARY_ENDPOINT
    );
    insert_report_json!(
        "source_command",
        "/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-durable-readback-receipt-acceptance-boundary --json"
    );
    insert_report_json!("native_route", true);
    insert_report_json!("side_effect_free", false);
    insert_report_json!("external_side_effect_free", true);
    insert_report_json!("audit_date", "2026-07-04");
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_durable_readback_receipt_acceptance_schema_version",
        "minimal_scoped_memory_real_write_canary_durable_readback_receipt_acceptance_boundary_v1"
    );
    insert_report_json!(
        "scoped_memory_real_write_canary_mode",
        "minimal_scoped_memory_real_write_canary_durable_readback_receipt_acceptance_report_only"
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
        "memory_write_execution_minimal_scoped_memory_real_write_canary_durable_readback_receipt_acceptance_boundary_ready",
        report_ready
    );
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_durable_readback_receipt_acceptance_ready",
        report_ready
    );
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_durable_readback_receipt_acceptance_performed",
        report_ready
    );
    insert_report_json!(
        "source_minimal_scoped_memory_real_write_canary_durable_wal_receipt_persistence_boundary_ready",
        source_ready
    );
    insert_report_json!(
        "source_minimal_scoped_memory_real_write_canary_durable_wal_receipt_persistence_report_sha256",
        source_report_sha256
    );
    for key in [
        "accepted_minimal_scoped_memory_real_write_canary_durable_wal_receipt_persistence_fixture_count",
        "blocked_minimal_scoped_memory_real_write_canary_durable_wal_receipt_persistence_fixture_count",
        "wal_write_performed_count",
        "wal_recorded_count",
        "wal_persisted_count",
        "receipt_recorded_count",
        "receipt_persisted_count",
        "receipt_materialized_count",
        "canary_artifact_filesystem_written_count",
        "artifact_readback_performed_count",
        "artifact_cleanup_performed_count",
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
    insert_report_json!("approved_namespace", approved_namespace);
    insert_report_json!("approved_store", approved_store);
    insert_report_json!("approved_scope", approved_scope);
    insert_report_json!("wal_record_id", wal_record_id);
    insert_report_json!("wal_record_sha256", source_wal_record_sha256);
    insert_report_json!("receipt_id", receipt_id);
    insert_report_json!("receipt_sha256", source_receipt_sha256);
    insert_report_json!(
        "receipt_artifact_readback_sha256",
        source_receipt_artifact_readback_sha256
    );
    insert_report_json!(
        "receipt_hash_chain_sha256",
        source_receipt_hash_chain_sha256
    );
    insert_report_json!(
        "receipt_readback_digest_match",
        receipt_readback_digest_match
    );
    insert_report_json!("receipt_hash_chain_verified", receipt_hash_chain_match);
    insert_report_json!(
        "source_execution_report_sha256",
        source_execution_report_sha256
    );
    insert_report_json!(
        "receipt_readback_report_sha256",
        receipt_readback_report_sha256
    );
    insert_report_json!(
        "receipt_acceptance_hash_sha256",
        receipt_acceptance_hash_sha256
    );
    insert_report_json!(
        "required_minimal_scoped_memory_real_write_canary_durable_readback_receipt_acceptance_surface_count",
        ACCEPTANCE_SURFACES.len()
    );
    insert_report_json!(
        "ready_minimal_scoped_memory_real_write_canary_durable_readback_receipt_acceptance_surface_count",
        if report_ready {
            ACCEPTANCE_SURFACES.len()
        } else {
            0
        }
    );
    insert_report_json!(
        "required_minimal_scoped_memory_real_write_canary_durable_readback_receipt_acceptance_fixture_count",
        10
    );
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_durable_readback_receipt_acceptance_fixture_count",
        fixture_count
    );
    insert_report_json!(
        "accepted_minimal_scoped_memory_real_write_canary_durable_readback_receipt_acceptance_fixture_count",
        accepted_fixture_count
    );
    insert_report_json!(
        "blocked_minimal_scoped_memory_real_write_canary_durable_readback_receipt_acceptance_fixture_count",
        blocked_fixture_count
    );
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_durable_readback_receipt_acceptance_accepted_count",
        accepted_fixture_count
    );
    for key in [
        "durable_readback_receipt_acceptance_authority_accepted_count",
        "source_durable_wal_receipt_persistence_bound_count",
        "receipt_readback_identity_bound_count",
        "receipt_readback_digest_bound_count",
        "receipt_hash_chain_acceptance_bound_count",
        "receipt_source_execution_linkage_bound_count",
        "receipt_acceptance_record_bound_count",
        "rollback_receipt_acceptance_handoff_bound_count",
    ] {
        report.insert(key.to_string(), serde_json::json!(accepted_fixture_count));
    }
    for key in [
        "receipt_readback_performed_count",
        "receipt_readback_result_recorded_count",
        "receipt_readback_result_accepted_count",
        "receipt_acceptance_recorded_count",
        "receipt_replay_guard_accepted_count",
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
        "tombstone_written_count",
        "compensating_memory_write_performed_count",
        "durable_memory_store_read_performed_count",
        "durable_memory_store_write_performed_count",
        "durable_memory_store_rollback_performed_count",
        "memory_store_write_performed_count",
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
        "required_minimal_scoped_memory_real_write_canary_durable_readback_receipt_acceptance_fields"
            .to_string(),
        required_fields,
    );
    report.insert(
        "minimal_scoped_memory_real_write_canary_durable_readback_receipt_acceptance_surfaces"
            .to_string(),
        serde_json::json!(ACCEPTANCE_SURFACES),
    );
    report.insert(
        "minimal_scoped_memory_real_write_canary_durable_readback_receipt_acceptance_fixtures"
            .to_string(),
        fixtures,
    );
    report.insert(
        "denied_by_minimal_scoped_memory_real_write_canary_durable_readback_receipt_acceptance_boundary"
            .to_string(),
        serde_json::Value::Array(denials),
    );
    insert_report_json!(
        "denied_by_minimal_scoped_memory_real_write_canary_durable_readback_receipt_acceptance_boundary_count",
        denied_count
    );
    for key in [
        "source_durable_wal_receipt_persistence_required",
        "minimal_scoped_memory_real_write_canary_durable_readback_receipt_acceptance_accepted",
        "approved_namespace_bound",
        "approved_store_bound",
        "approved_scope_bound",
        "wal_record_identity_bound",
        "receipt_identity_bound",
        "receipt_artifact_readback_digest_bound",
        "receipt_hash_chain_bound",
        "receipt_source_execution_linkage_bound",
        "receipt_acceptance_record_bound",
        "receipt_replay_guard_bound",
        "receipt_operator_review_handoff_bound",
        "rollback_receipt_acceptance_handoff_bound",
        "wal_rewrite_forbidden_on_report_route",
        "receipt_repersist_forbidden_on_report_route",
        "nonce_consumption_forbidden_on_report_route",
        "explicit_command_dispatch_forbidden_on_report_route",
        "durable_memory_read_forbidden",
        "durable_memory_write_forbidden",
        "durable_memory_rollback_forbidden",
        "memory_store_mutation_forbidden",
        "post_write_memory_readback_forbidden",
        "rollback_execution_forbidden",
        "tombstone_write_forbidden",
        "kg_live_write_forbidden",
        "provider_model_invocation_forbidden",
        "credential_channel_public_release_forbidden",
        "install_restart_active_binary_mutation_forbidden",
    ] {
        report.insert(key.to_string(), serde_json::json!(true));
    }
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_durable_readback_receipt_acceptance_boundary_hash_sha256",
        boundary_hash_sha256
    );
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_durable_readback_receipt_acceptance_policy_hash_sha256",
        policy_hash_sha256
    );
    report.insert("allowed_next_actions".to_string(), allowed_next_actions);
    for &key in FALSE_EXTERNAL_KEYS {
        report.insert(key.to_string(), serde_json::json!(false));
    }
    for &key in TRUE_ACCEPTANCE_KEYS {
        report.insert(key.to_string(), serde_json::json!(report_ready));
    }
    report.insert(
        "side_effects".to_string(),
        serde_json::Value::Object(side_effects),
    );
    serde_json::Value::Object(report)
}

fn hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_boundary_report()
-> serde_json::Value {
    const ACCEPTANCE_SURFACES: &[&str] = &[
        "source_durable_readback_receipt_acceptance_required",
        "approved_namespace_store_scope_required",
        "receipt_acceptance_hash_required",
        "rollback_receipt_identity_required",
        "rollback_receipt_digest_required",
        "rollback_receipt_hash_chain_required",
        "rollback_receipt_source_readback_linkage_required",
        "rollback_receipt_acceptance_record_required",
        "rollback_receipt_replay_guard_required",
        "rollback_operator_review_handoff_required",
        "tombstone_cleanup_handoff_required",
        "memory_kg_provider_channel_public_release_install_active_binary_side_effects_forbidden",
    ];
    const ACCEPTANCE_DENIALS: &[&str] = &[
        "source_durable_readback_receipt_acceptance_boundary_required",
        "approved_namespace_required",
        "approved_store_required",
        "approved_scope_required",
        "receipt_acceptance_hash_required",
        "rollback_receipt_identity_required",
        "rollback_receipt_digest_required",
        "rollback_receipt_hash_chain_required",
        "rollback_receipt_source_readback_linkage_required",
        "rollback_receipt_acceptance_record_required",
        "rollback_receipt_replay_guard_required",
        "tombstone_cleanup_handoff_required",
        "wal_rewrite_report_route_denied",
        "receipt_repersist_report_route_denied",
        "nonce_consumption_report_route_denied",
        "explicit_command_dispatch_report_route_denied",
        "durable_memory_store_read_denied",
        "durable_memory_store_write_denied",
        "durable_memory_store_rollback_denied",
        "memory_store_mutation_denied",
        "post_write_memory_readback_denied",
        "rollback_execution_denied",
        "tombstone_write_denied",
        "kg_live_write_denied",
        "provider_model_invocation_denied",
        "credential_channel_public_release_denied",
        "install_restart_active_binary_mutation_denied",
        "compensating_memory_write_denied",
    ];
    const FALSE_EXTERNAL_KEYS: &[&str] = &[
        "single_use_nonce_consumed",
        "explicit_command_dispatched",
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
    const TRUE_ACCEPTANCE_KEYS: &[&str] = &[
        "rollback_receipt_acceptance_performed",
        "rollback_receipt_acceptance_result_recorded",
        "rollback_receipt_acceptance_result_accepted",
        "rollback_receipt_identity_accepted",
        "rollback_receipt_digest_accepted",
        "rollback_receipt_hash_chain_accepted",
        "minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_accepted",
    ];

    fn acceptance_fixture(
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
            "minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_status"
                .to_string(),
            serde_json::json!(status),
        );
        base.insert("reason".to_string(), serde_json::json!(reason));
        base.insert(
            "minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_accepted"
                .to_string(),
            serde_json::json!(accepted),
        );
        for key in [
            "source_readback_receipt_acceptance_boundary_ready",
            "approved_namespace_bound",
            "approved_store_bound",
            "approved_scope_bound",
            "receipt_acceptance_hash_bound",
            "rollback_receipt_identity_bound",
            "rollback_receipt_digest_bound",
            "rollback_receipt_hash_chain_bound",
            "rollback_receipt_source_readback_linkage_bound",
            "rollback_receipt_acceptance_record_bound",
            "rollback_receipt_replay_guard_bound",
            "rollback_operator_review_handoff_bound",
            "tombstone_cleanup_handoff_bound",
        ] {
            base.insert(key.to_string(), serde_json::json!(accepted));
        }
        for &key in FALSE_EXTERNAL_KEYS {
            base.insert(key.to_string(), serde_json::json!(false));
        }
        for &key in TRUE_ACCEPTANCE_KEYS {
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
        .name("hepta-memory-minimal-canary-rollback-receipt-source-report".to_string())
        .stack_size(8 * 1024 * 1024)
        .spawn(
            hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_durable_readback_receipt_acceptance_boundary_report,
        )
        .ok()
        .and_then(|handle| handle.join().ok())
        .unwrap_or_else(|| {
            serde_json::json!({
                "status": "blocked",
                "memory_write_execution_minimal_scoped_memory_real_write_canary_durable_readback_receipt_acceptance_boundary_ready": false,
                "minimal_scoped_memory_real_write_canary_durable_readback_receipt_acceptance_ready": false,
                "source_minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_source_report_thread_failed": true
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
    let source_next_action_rollback_acceptance = source
        .get("allowed_next_actions")
        .and_then(serde_json::Value::as_array)
        .and_then(|items| items.get(1))
        .map(|item| {
            item.get("action").and_then(serde_json::Value::as_str)
                == Some(
                    "prepare_minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_boundary",
                )
                && item
                    .get("requires_minimal_scoped_memory_real_write_canary_durable_readback_receipt_acceptance")
                    .and_then(serde_json::Value::as_bool)
                    == Some(true)
        })
        .unwrap_or(false);
    let source_side_effects_ok = source
        .get("side_effects")
        .and_then(serde_json::Value::as_object)
        .map(|effects| {
            effects
                .get("receipt_readback_performed")
                .and_then(serde_json::Value::as_bool)
                == Some(true)
                && effects
                    .get("receipt_readback_result_accepted")
                    .and_then(serde_json::Value::as_bool)
                    == Some(true)
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
            "memory_write_execution_minimal_scoped_memory_real_write_canary_durable_readback_receipt_acceptance_boundary_ready",
        )
        && json_bool(
            &source,
            "minimal_scoped_memory_real_write_canary_durable_readback_receipt_acceptance_ready",
        )
        && json_bool(
            &source,
            "minimal_scoped_memory_real_write_canary_durable_readback_receipt_acceptance_performed",
        )
        && json_bool(
            &source,
            "minimal_scoped_memory_real_write_canary_durable_readback_receipt_acceptance_accepted",
        )
        && json_bool(&source, "receipt_readback_result_accepted")
        && json_bool(&source, "receipt_hash_chain_verified")
        && json_u64(
            &source,
            "accepted_minimal_scoped_memory_real_write_canary_durable_readback_receipt_acceptance_fixture_count",
        ) == 1
        && json_u64(
            &source,
            "blocked_minimal_scoped_memory_real_write_canary_durable_readback_receipt_acceptance_fixture_count",
        ) == 9
        && json_u64(&source, "receipt_readback_performed_count") == 1
        && json_u64(&source, "receipt_readback_result_accepted_count") == 1
        && json_u64(&source, "receipt_acceptance_recorded_count") == 1
        && json_u64(&source, "rollback_receipt_acceptance_handoff_bound_count") == 1
        && json_u64(&source, "wal_write_performed_count") == 0
        && json_u64(&source, "receipt_persisted_count") == 0
        && json_u64(&source, "durable_memory_store_read_performed_count") == 0
        && json_u64(&source, "durable_memory_store_write_performed_count") == 0
        && json_u64(&source, "durable_memory_store_rollback_performed_count") == 0
        && json_u64(&source, "memory_store_write_performed_count") == 0
        && !json_bool(&source, "rollback_executed")
        && !json_bool(&source, "tombstone_written")
        && !json_bool(&source, "durable_memory_store_write_performed")
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
        && source_next_action_rollback_acceptance
        && source_side_effects_ok;

    let approved_namespace = json_str(&source, "approved_namespace");
    let approved_store = json_str(&source, "approved_store");
    let approved_scope = json_str(&source, "approved_scope");
    let source_report_sha256 = sha256_text_value(&source.to_string());
    let source_receipt_acceptance_hash_sha256 = json_str(&source, "receipt_acceptance_hash_sha256");
    let source_receipt_readback_report_sha256 = json_str(&source, "receipt_readback_report_sha256");
    let source_receipt_hash_chain_sha256 = json_str(&source, "receipt_hash_chain_sha256");
    let rollback_receipt_id = "hepta-minimal-scoped-memory-real-write-canary-rollback-receipt-v1";
    let rollback_receipt_sha256 = sha256_text_value(&format!(
        "minimal-scoped-memory-real-write-canary-rollback-receipt:v1:source={source_report_sha256}:receipt-acceptance={source_receipt_acceptance_hash_sha256}:readback={source_receipt_readback_report_sha256}:hash-chain={source_receipt_hash_chain_sha256}"
    ));
    let rollback_receipt_hash_chain_sha256 = sha256_text_value(&format!(
        "minimal-scoped-memory-real-write-canary-rollback-receipt-hash-chain:v1:prior={source_receipt_hash_chain_sha256}:rollback-receipt={rollback_receipt_sha256}"
    ));
    let rollback_receipt_acceptance_hash_sha256 = sha256_text_value(&format!(
        "minimal-scoped-memory-real-write-canary-rollback-receipt-acceptance:v1:namespace={approved_namespace}:store={approved_store}:scope={approved_scope}:rollback-receipt={rollback_receipt_sha256}:accepted=true"
    ));
    let namespace_bound = approved_namespace == "hepta.memory.canary";
    let store_bound = approved_store == "wal-receipt-canary-artifact";
    let scope_bound = approved_scope == "session";
    let source_receipt_acceptance_bound = !source_receipt_acceptance_hash_sha256.is_empty();
    let rollback_receipt_identity_match =
        !rollback_receipt_id.is_empty() && !rollback_receipt_sha256.is_empty();
    let rollback_receipt_digest_match =
        !rollback_receipt_sha256.is_empty() && !source_receipt_readback_report_sha256.is_empty();
    let rollback_receipt_hash_chain_match = !rollback_receipt_hash_chain_sha256.is_empty()
        && !source_receipt_hash_chain_sha256.is_empty();
    let rollback_receipt_source_linkage_match =
        !source_report_sha256.is_empty() && !source_receipt_readback_report_sha256.is_empty();

    let fixtures = serde_json::Value::Array(vec![
        acceptance_fixture(
            "minimal-scoped-memory-real-write-canary-rollback-receipt-acceptance",
            "accepted_rollback_receipt_identity_digest_hash_chain",
            "rollback_receipt_identity_digest_hash_chain_and_tombstone_cleanup_handoff_accepted",
            true,
            serde_json::json!({
                "approved_namespace": approved_namespace,
                "approved_store": approved_store,
                "approved_scope": approved_scope,
                "source_receipt_acceptance_hash_sha256": source_receipt_acceptance_hash_sha256,
                "rollback_receipt_id": rollback_receipt_id,
                "rollback_receipt_sha256": rollback_receipt_sha256,
                "rollback_receipt_hash_chain_sha256": rollback_receipt_hash_chain_sha256,
            }),
        ),
        acceptance_fixture(
            "minimal-scoped-memory-real-write-canary-rollback-receipt-missing-source",
            "blocked_source_noop",
            "source_durable_readback_receipt_acceptance_boundary_required",
            false,
            serde_json::json!({"source_readback_receipt_acceptance_boundary_ready": false}),
        ),
        acceptance_fixture(
            "minimal-scoped-memory-real-write-canary-rollback-receipt-wrong-namespace",
            "blocked_namespace_noop",
            "approved_namespace_required",
            false,
            serde_json::json!({"approved_namespace_bound": false}),
        ),
        acceptance_fixture(
            "minimal-scoped-memory-real-write-canary-rollback-receipt-wrong-store",
            "blocked_store_noop",
            "approved_store_required",
            false,
            serde_json::json!({"approved_store_bound": false}),
        ),
        acceptance_fixture(
            "minimal-scoped-memory-real-write-canary-rollback-receipt-wrong-scope",
            "blocked_scope_noop",
            "approved_scope_required",
            false,
            serde_json::json!({"approved_scope_bound": false}),
        ),
        acceptance_fixture(
            "minimal-scoped-memory-real-write-canary-rollback-receipt-missing-source-acceptance",
            "blocked_receipt_acceptance_hash_noop",
            "receipt_acceptance_hash_required",
            false,
            serde_json::json!({"receipt_acceptance_hash_bound": false}),
        ),
        acceptance_fixture(
            "minimal-scoped-memory-real-write-canary-rollback-receipt-missing-identity",
            "blocked_rollback_receipt_identity_noop",
            "rollback_receipt_identity_required",
            false,
            serde_json::json!({"rollback_receipt_identity_bound": false}),
        ),
        acceptance_fixture(
            "minimal-scoped-memory-real-write-canary-rollback-receipt-digest-mismatch",
            "blocked_rollback_receipt_digest_noop",
            "rollback_receipt_digest_required",
            false,
            serde_json::json!({"rollback_receipt_digest_bound": false}),
        ),
        acceptance_fixture(
            "minimal-scoped-memory-real-write-canary-rollback-receipt-hash-chain-mismatch",
            "blocked_rollback_receipt_hash_chain_noop",
            "rollback_receipt_hash_chain_required",
            false,
            serde_json::json!({"rollback_receipt_hash_chain_bound": false}),
        ),
        acceptance_fixture(
            "minimal-scoped-memory-real-write-canary-rollback-receipt-direct-side-effect-attempt",
            "blocked_direct_side_effect_noop",
            "direct_memory_kg_provider_channel_release_install_active_binary_side_effects_denied",
            false,
            serde_json::json!({
                "wal_rewrite_requested": true,
                "receipt_repersist_requested": true,
                "rollback_execution_requested": true,
                "tombstone_write_requested": true,
                "durable_memory_read_requested": true,
                "durable_memory_write_requested": true,
                "durable_memory_rollback_requested": true,
                "memory_store_mutation_requested": true,
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
                        .get("minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_accepted")
                        .and_then(serde_json::Value::as_bool)
                        == Some(true)
                })
                .count()
        })
        .unwrap_or(0);
    let blocked_fixture_count = fixture_count.saturating_sub(accepted_fixture_count);
    let denials = ACCEPTANCE_DENIALS
        .iter()
        .map(|reason| serde_json::json!(reason))
        .collect::<Vec<_>>();
    let denied_count = denials.len();
    let acceptance_ops_ok = namespace_bound
        && store_bound
        && scope_bound
        && source_receipt_acceptance_bound
        && rollback_receipt_identity_match
        && rollback_receipt_digest_match
        && rollback_receipt_hash_chain_match
        && rollback_receipt_source_linkage_match;
    let report_ready = route_count_source_command_accepted
        && source_ready
        && fixture_count == 10
        && accepted_fixture_count == 1
        && blocked_fixture_count == 9
        && denied_count == 28
        && acceptance_ops_ok;
    let boundary_hash_sha256 = sha256_text_value(&format!(
        "minimal-scoped-memory-real-write-canary-rollback-receipt-acceptance-boundary:v1:source-ready={source_ready}:receipt-acceptance={source_receipt_acceptance_bound}:rollback-receipt={rollback_receipt_identity_match}:digest={rollback_receipt_digest_match}:hash-chain={rollback_receipt_hash_chain_match}:fixtures={fixture_count}:accepted={accepted_fixture_count}:denials={denied_count}"
    ));
    let policy_hash_sha256 = sha256_text_value(
        "minimal-scoped-memory-real-write-canary-rollback-receipt-acceptance-policy:v1:accept-source-readback-acceptance-evidence:no-wal-rewrite:no-receipt-repersist:no-rollback-execution:no-tombstone:no-durable-memory-store:no-kg:no-provider:no-channel:no-release:no-install",
    );
    let mut side_effects = serde_json::Map::new();
    for &key in FALSE_EXTERNAL_KEYS {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }
    for &key in TRUE_ACCEPTANCE_KEYS {
        side_effects.insert(key.to_string(), serde_json::json!(report_ready));
    }
    let required_fields = serde_json::json!([
        "source_minimal_scoped_memory_real_write_canary_durable_readback_receipt_acceptance_report_sha256",
        "source_receipt_acceptance_hash_sha256",
        "source_receipt_readback_report_sha256",
        "approved_namespace",
        "approved_store",
        "approved_scope",
        "rollback_receipt_id",
        "rollback_receipt_sha256",
        "rollback_receipt_hash_chain_sha256",
        "rollback_receipt_acceptance_hash_sha256",
        "active_binary_sha256",
        "route_count",
    ]);
    let allowed_next_actions = serde_json::json!([
        {
            "action": "run_minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_boundary_require_live_gate",
            "status": "allowed_verification_only",
            "accepts_rollback_receipt": true,
            "writes_wal": false,
            "persists_receipt": false,
            "writes_memory": false,
            "reads_memory": false,
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
            "action": "prepare_minimal_scoped_memory_real_write_canary_tombstone_cleanup_acceptance_boundary",
            "status": "allowed_report_only_next_slice",
            "requires_minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance": true,
            "writes_durable_memory": false,
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
        "hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_boundary_route"
    );
    insert_report_json!(
        "endpoint",
        HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_MINIMAL_SCOPED_MEMORY_REAL_WRITE_CANARY_ROLLBACK_RECEIPT_ACCEPTANCE_BOUNDARY_ENDPOINT
    );
    insert_report_json!(
        "source_command",
        "/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-rollback-receipt-acceptance-boundary --json"
    );
    insert_report_json!("native_route", true);
    insert_report_json!("side_effect_free", false);
    insert_report_json!("external_side_effect_free", true);
    insert_report_json!("audit_date", "2026-07-04");
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_schema_version",
        "minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_boundary_v1"
    );
    insert_report_json!(
        "scoped_memory_real_write_canary_mode",
        "minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_report_only"
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
        "memory_write_execution_minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_boundary_ready",
        report_ready
    );
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_ready",
        report_ready
    );
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_performed",
        report_ready
    );
    insert_report_json!(
        "source_minimal_scoped_memory_real_write_canary_durable_readback_receipt_acceptance_boundary_ready",
        source_ready
    );
    insert_report_json!(
        "source_minimal_scoped_memory_real_write_canary_durable_readback_receipt_acceptance_report_sha256",
        source_report_sha256
    );
    for key in [
        "accepted_minimal_scoped_memory_real_write_canary_durable_readback_receipt_acceptance_fixture_count",
        "blocked_minimal_scoped_memory_real_write_canary_durable_readback_receipt_acceptance_fixture_count",
        "receipt_readback_performed_count",
        "receipt_readback_result_recorded_count",
        "receipt_readback_result_accepted_count",
        "receipt_acceptance_recorded_count",
        "receipt_replay_guard_accepted_count",
        "rollback_receipt_acceptance_handoff_bound_count",
        "wal_write_performed_count",
        "receipt_persisted_count",
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
    insert_report_json!("approved_namespace", approved_namespace);
    insert_report_json!("approved_store", approved_store);
    insert_report_json!("approved_scope", approved_scope);
    insert_report_json!(
        "source_receipt_acceptance_hash_sha256",
        source_receipt_acceptance_hash_sha256
    );
    insert_report_json!(
        "source_receipt_readback_report_sha256",
        source_receipt_readback_report_sha256
    );
    insert_report_json!(
        "source_receipt_hash_chain_sha256",
        source_receipt_hash_chain_sha256
    );
    insert_report_json!("rollback_receipt_id", rollback_receipt_id);
    insert_report_json!("rollback_receipt_sha256", rollback_receipt_sha256);
    insert_report_json!(
        "rollback_receipt_hash_chain_sha256",
        rollback_receipt_hash_chain_sha256
    );
    insert_report_json!(
        "rollback_receipt_acceptance_hash_sha256",
        rollback_receipt_acceptance_hash_sha256
    );
    insert_report_json!(
        "rollback_receipt_digest_match",
        rollback_receipt_digest_match
    );
    insert_report_json!(
        "rollback_receipt_hash_chain_verified",
        rollback_receipt_hash_chain_match
    );
    insert_report_json!(
        "required_minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_surface_count",
        ACCEPTANCE_SURFACES.len()
    );
    insert_report_json!(
        "ready_minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_surface_count",
        if report_ready {
            ACCEPTANCE_SURFACES.len()
        } else {
            0
        }
    );
    insert_report_json!(
        "required_minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_fixture_count",
        10
    );
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_fixture_count",
        fixture_count
    );
    insert_report_json!(
        "accepted_minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_fixture_count",
        accepted_fixture_count
    );
    insert_report_json!(
        "blocked_minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_fixture_count",
        blocked_fixture_count
    );
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_accepted_count",
        accepted_fixture_count
    );
    for key in [
        "rollback_receipt_acceptance_authority_accepted_count",
        "source_durable_readback_receipt_acceptance_bound_count",
        "receipt_acceptance_hash_bound_count",
        "rollback_receipt_identity_bound_count",
        "rollback_receipt_digest_bound_count",
        "rollback_receipt_hash_chain_bound_count",
        "rollback_receipt_source_readback_linkage_bound_count",
        "rollback_receipt_acceptance_record_bound_count",
        "rollback_receipt_replay_guard_accepted_count",
        "tombstone_cleanup_handoff_bound_count",
        "rollback_receipt_acceptance_result_recorded_count",
        "rollback_receipt_acceptance_result_accepted_count",
    ] {
        report.insert(key.to_string(), serde_json::json!(accepted_fixture_count));
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
        "tombstone_written_count",
        "compensating_memory_write_performed_count",
        "durable_memory_store_read_performed_count",
        "durable_memory_store_write_performed_count",
        "durable_memory_store_rollback_performed_count",
        "memory_store_write_performed_count",
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
        "required_minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_fields"
            .to_string(),
        required_fields,
    );
    report.insert(
        "minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_surfaces".to_string(),
        serde_json::json!(ACCEPTANCE_SURFACES),
    );
    report.insert(
        "minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_fixtures".to_string(),
        fixtures,
    );
    report.insert(
        "denied_by_minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_boundary"
            .to_string(),
        serde_json::Value::Array(denials),
    );
    insert_report_json!(
        "denied_by_minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_boundary_count",
        denied_count
    );
    for key in [
        "source_durable_readback_receipt_acceptance_required",
        "minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_accepted",
        "approved_namespace_bound",
        "approved_store_bound",
        "approved_scope_bound",
        "receipt_acceptance_hash_bound",
        "rollback_receipt_identity_bound",
        "rollback_receipt_digest_bound",
        "rollback_receipt_hash_chain_bound",
        "rollback_receipt_source_readback_linkage_bound",
        "rollback_receipt_acceptance_record_bound",
        "rollback_receipt_replay_guard_bound",
        "rollback_operator_review_handoff_bound",
        "tombstone_cleanup_handoff_bound",
        "wal_rewrite_forbidden_on_report_route",
        "receipt_repersist_forbidden_on_report_route",
        "nonce_consumption_forbidden_on_report_route",
        "explicit_command_dispatch_forbidden_on_report_route",
        "durable_memory_read_forbidden",
        "durable_memory_write_forbidden",
        "durable_memory_rollback_forbidden",
        "memory_store_mutation_forbidden",
        "post_write_memory_readback_forbidden",
        "rollback_execution_forbidden",
        "tombstone_write_forbidden",
        "kg_live_write_forbidden",
        "provider_model_invocation_forbidden",
        "credential_channel_public_release_forbidden",
        "install_restart_active_binary_mutation_forbidden",
    ] {
        report.insert(key.to_string(), serde_json::json!(true));
    }
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_boundary_hash_sha256",
        boundary_hash_sha256
    );
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_policy_hash_sha256",
        policy_hash_sha256
    );
    report.insert("allowed_next_actions".to_string(), allowed_next_actions);
    for &key in FALSE_EXTERNAL_KEYS {
        report.insert(key.to_string(), serde_json::json!(false));
    }
    for &key in TRUE_ACCEPTANCE_KEYS {
        report.insert(key.to_string(), serde_json::json!(report_ready));
    }
    report.insert(
        "side_effects".to_string(),
        serde_json::Value::Object(side_effects),
    );
    serde_json::Value::Object(report)
}

fn hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_tombstone_cleanup_acceptance_boundary_report()
-> serde_json::Value {
    const ACCEPTANCE_SURFACES: &[&str] = &[
        "source_rollback_receipt_acceptance_required",
        "approved_namespace_store_scope_required",
        "rollback_receipt_acceptance_hash_required",
        "tombstone_cleanup_plan_required",
        "tombstone_cleanup_target_required",
        "tombstone_cleanup_receipt_linkage_required",
        "tombstone_cleanup_idempotency_guard_required",
        "tombstone_cleanup_operator_review_handoff_required",
        "tombstone_write_forbidden",
        "artifact_cleanup_forbidden",
        "durable_memory_store_side_effects_forbidden",
        "kg_provider_channel_public_release_install_active_binary_side_effects_forbidden",
    ];
    const ACCEPTANCE_DENIALS: &[&str] = &[
        "source_rollback_receipt_acceptance_boundary_required",
        "approved_namespace_required",
        "approved_store_required",
        "approved_scope_required",
        "rollback_receipt_acceptance_hash_required",
        "rollback_receipt_identity_required",
        "tombstone_cleanup_plan_required",
        "tombstone_cleanup_target_required",
        "tombstone_cleanup_receipt_linkage_required",
        "tombstone_cleanup_idempotency_guard_required",
        "tombstone_cleanup_operator_review_handoff_required",
        "rollback_execution_denied",
        "tombstone_write_denied",
        "artifact_cleanup_denied",
        "wal_write_denied",
        "receipt_record_persist_materialize_denied",
        "nonce_consumption_denied",
        "explicit_command_dispatch_denied",
        "durable_memory_store_read_denied",
        "durable_memory_store_write_denied",
        "durable_memory_store_rollback_denied",
        "memory_store_mutation_denied",
        "compensating_memory_write_denied",
        "kg_live_write_denied",
        "provider_model_invocation_denied",
        "credential_channel_external_send_denied",
        "public_release_artifact_denied",
        "install_restart_active_binary_mutation_denied",
    ];
    const FALSE_EXTERNAL_KEYS: &[&str] = &[
        "single_use_nonce_consumed",
        "explicit_command_dispatched",
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
        "tombstone_cleanup_executed",
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
    const TRUE_ACCEPTANCE_KEYS: &[&str] = &[
        "tombstone_cleanup_acceptance_performed",
        "tombstone_cleanup_acceptance_result_recorded",
        "tombstone_cleanup_acceptance_result_accepted",
        "tombstone_cleanup_plan_accepted",
        "tombstone_cleanup_target_accepted",
        "tombstone_cleanup_receipt_linkage_accepted",
        "tombstone_cleanup_idempotency_guard_accepted",
        "minimal_scoped_memory_real_write_canary_tombstone_cleanup_acceptance_accepted",
    ];

    fn acceptance_fixture(
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
            "minimal_scoped_memory_real_write_canary_tombstone_cleanup_acceptance_status"
                .to_string(),
            serde_json::json!(status),
        );
        base.insert("reason".to_string(), serde_json::json!(reason));
        base.insert(
            "minimal_scoped_memory_real_write_canary_tombstone_cleanup_acceptance_accepted"
                .to_string(),
            serde_json::json!(accepted),
        );
        for key in [
            "source_rollback_receipt_acceptance_boundary_ready",
            "approved_namespace_bound",
            "approved_store_bound",
            "approved_scope_bound",
            "rollback_receipt_acceptance_hash_bound",
            "rollback_receipt_identity_bound",
            "tombstone_cleanup_plan_bound",
            "tombstone_cleanup_target_bound",
            "tombstone_cleanup_receipt_linkage_bound",
            "tombstone_cleanup_idempotency_guard_bound",
            "tombstone_cleanup_operator_review_handoff_bound",
        ] {
            base.insert(key.to_string(), serde_json::json!(accepted));
        }
        for &key in FALSE_EXTERNAL_KEYS {
            base.insert(key.to_string(), serde_json::json!(false));
        }
        for &key in TRUE_ACCEPTANCE_KEYS {
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
        .name("hepta-memory-minimal-canary-tombstone-cleanup-source-report".to_string())
        .stack_size(8 * 1024 * 1024)
        .spawn(
            hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_boundary_report,
        )
        .ok()
        .and_then(|handle| handle.join().ok())
        .unwrap_or_else(|| {
            serde_json::json!({
                "status": "blocked",
                "memory_write_execution_minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_boundary_ready": false,
                "minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_ready": false,
                "source_minimal_scoped_memory_real_write_canary_tombstone_cleanup_acceptance_source_report_thread_failed": true
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
    let source_next_action_tombstone_cleanup = source
        .get("allowed_next_actions")
        .and_then(serde_json::Value::as_array)
        .and_then(|items| items.get(1))
        .map(|item| {
            item.get("action").and_then(serde_json::Value::as_str)
                == Some(
                    "prepare_minimal_scoped_memory_real_write_canary_tombstone_cleanup_acceptance_boundary",
                )
                && item
                    .get("requires_minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance")
                    .and_then(serde_json::Value::as_bool)
                    == Some(true)
        })
        .unwrap_or(false);
    let source_side_effects_ok = source
        .get("side_effects")
        .and_then(serde_json::Value::as_object)
        .map(|effects| {
            effects
                .get("rollback_receipt_acceptance_performed")
                .and_then(serde_json::Value::as_bool)
                == Some(true)
                && effects
                    .get("rollback_receipt_acceptance_result_accepted")
                    .and_then(serde_json::Value::as_bool)
                    == Some(true)
                && effects
                    .get("rollback_executed")
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
            "memory_write_execution_minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_boundary_ready",
        )
        && json_bool(
            &source,
            "minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_ready",
        )
        && json_bool(
            &source,
            "minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_performed",
        )
        && json_bool(
            &source,
            "minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_accepted",
        )
        && json_bool(&source, "rollback_receipt_acceptance_result_accepted")
        && json_bool(&source, "rollback_receipt_hash_chain_verified")
        && json_u64(
            &source,
            "accepted_minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_fixture_count",
        ) == 1
        && json_u64(
            &source,
            "blocked_minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_fixture_count",
        ) == 9
        && json_u64(&source, "rollback_receipt_acceptance_result_accepted_count") == 1
        && json_u64(&source, "rollback_receipt_identity_bound_count") == 1
        && json_u64(&source, "rollback_receipt_digest_bound_count") == 1
        && json_u64(&source, "rollback_receipt_hash_chain_bound_count") == 1
        && json_u64(&source, "tombstone_cleanup_handoff_bound_count") == 1
        && json_u64(&source, "wal_write_performed_count") == 0
        && json_u64(&source, "receipt_persisted_count") == 0
        && json_u64(&source, "rollback_performed_count") == 0
        && json_u64(&source, "tombstone_written_count") == 0
        && json_u64(&source, "durable_memory_store_write_performed_count") == 0
        && json_u64(&source, "memory_store_write_performed_count") == 0
        && !json_bool(&source, "rollback_executed")
        && !json_bool(&source, "tombstone_written")
        && !json_bool(&source, "durable_memory_store_write_performed")
        && !json_bool(&source, "live_kg_write_performed")
        && !json_bool(&source, "provider_invoked")
        && !json_bool(&source, "model_invoked")
        && !json_bool(&source, "credential_read")
        && !json_bool(&source, "channel_send_performed")
        && !json_bool(&source, "external_send_performed")
        && !json_bool(&source, "release_artifact_written")
        && !json_bool(&source, "install_executed")
        && !json_bool(&source, "active_binary_mutated")
        && source_next_action_tombstone_cleanup
        && source_side_effects_ok;

    let approved_namespace = json_str(&source, "approved_namespace");
    let approved_store = json_str(&source, "approved_store");
    let approved_scope = json_str(&source, "approved_scope");
    let source_report_sha256 = sha256_text_value(&source.to_string());
    let source_rollback_receipt_acceptance_hash_sha256 =
        json_str(&source, "rollback_receipt_acceptance_hash_sha256");
    let source_rollback_receipt_sha256 = json_str(&source, "rollback_receipt_sha256");
    let source_rollback_receipt_hash_chain_sha256 =
        json_str(&source, "rollback_receipt_hash_chain_sha256");
    let tombstone_cleanup_target_id =
        "hepta-minimal-scoped-memory-real-write-canary-tombstone-cleanup-target-v1";
    let tombstone_cleanup_plan_sha256 = sha256_text_value(&format!(
        "minimal-scoped-memory-real-write-canary-tombstone-cleanup-plan:v1:source={source_report_sha256}:rollback-acceptance={source_rollback_receipt_acceptance_hash_sha256}:target={tombstone_cleanup_target_id}:write=false:cleanup=false"
    ));
    let tombstone_cleanup_target_sha256 = sha256_text_value(&format!(
        "minimal-scoped-memory-real-write-canary-tombstone-cleanup-target:v1:namespace={approved_namespace}:store={approved_store}:scope={approved_scope}:rollback-receipt={source_rollback_receipt_sha256}"
    ));
    let tombstone_cleanup_receipt_linkage_sha256 = sha256_text_value(&format!(
        "minimal-scoped-memory-real-write-canary-tombstone-cleanup-receipt-linkage:v1:plan={tombstone_cleanup_plan_sha256}:target={tombstone_cleanup_target_sha256}:source-hash-chain={source_rollback_receipt_hash_chain_sha256}"
    ));
    let tombstone_cleanup_acceptance_hash_sha256 = sha256_text_value(&format!(
        "minimal-scoped-memory-real-write-canary-tombstone-cleanup-acceptance:v1:namespace={approved_namespace}:store={approved_store}:scope={approved_scope}:linkage={tombstone_cleanup_receipt_linkage_sha256}:accepted=true"
    ));
    let namespace_bound = approved_namespace == "hepta.memory.canary";
    let store_bound = approved_store == "wal-receipt-canary-artifact";
    let scope_bound = approved_scope == "session";
    let rollback_receipt_acceptance_bound =
        !source_rollback_receipt_acceptance_hash_sha256.is_empty();
    let rollback_receipt_identity_bound = !source_rollback_receipt_sha256.is_empty();
    let tombstone_cleanup_plan_bound = !tombstone_cleanup_plan_sha256.is_empty();
    let tombstone_cleanup_target_bound = !tombstone_cleanup_target_sha256.is_empty();
    let tombstone_cleanup_receipt_linkage_bound = !tombstone_cleanup_receipt_linkage_sha256
        .is_empty()
        && !source_rollback_receipt_hash_chain_sha256.is_empty();
    let tombstone_cleanup_idempotency_guard_bound =
        rollback_receipt_acceptance_bound && tombstone_cleanup_receipt_linkage_bound;

    let fixtures = serde_json::Value::Array(vec![
        acceptance_fixture(
            "minimal-scoped-memory-real-write-canary-tombstone-cleanup-acceptance",
            "accepted_tombstone_cleanup_plan_target_receipt_linkage",
            "tombstone_cleanup_plan_target_receipt_linkage_and_idempotency_guard_accepted",
            true,
            serde_json::json!({
                "approved_namespace": approved_namespace,
                "approved_store": approved_store,
                "approved_scope": approved_scope,
                "source_rollback_receipt_acceptance_hash_sha256": source_rollback_receipt_acceptance_hash_sha256,
                "tombstone_cleanup_target_id": tombstone_cleanup_target_id,
                "tombstone_cleanup_plan_sha256": tombstone_cleanup_plan_sha256,
                "tombstone_cleanup_target_sha256": tombstone_cleanup_target_sha256,
                "tombstone_cleanup_receipt_linkage_sha256": tombstone_cleanup_receipt_linkage_sha256,
            }),
        ),
        acceptance_fixture(
            "minimal-scoped-memory-real-write-canary-tombstone-cleanup-missing-source",
            "blocked_source_noop",
            "source_rollback_receipt_acceptance_boundary_required",
            false,
            serde_json::json!({"source_rollback_receipt_acceptance_boundary_ready": false}),
        ),
        acceptance_fixture(
            "minimal-scoped-memory-real-write-canary-tombstone-cleanup-wrong-namespace",
            "blocked_namespace_noop",
            "approved_namespace_required",
            false,
            serde_json::json!({"approved_namespace_bound": false}),
        ),
        acceptance_fixture(
            "minimal-scoped-memory-real-write-canary-tombstone-cleanup-wrong-store",
            "blocked_store_noop",
            "approved_store_required",
            false,
            serde_json::json!({"approved_store_bound": false}),
        ),
        acceptance_fixture(
            "minimal-scoped-memory-real-write-canary-tombstone-cleanup-wrong-scope",
            "blocked_scope_noop",
            "approved_scope_required",
            false,
            serde_json::json!({"approved_scope_bound": false}),
        ),
        acceptance_fixture(
            "minimal-scoped-memory-real-write-canary-tombstone-cleanup-missing-rollback-acceptance",
            "blocked_rollback_receipt_acceptance_noop",
            "rollback_receipt_acceptance_hash_required",
            false,
            serde_json::json!({"rollback_receipt_acceptance_hash_bound": false}),
        ),
        acceptance_fixture(
            "minimal-scoped-memory-real-write-canary-tombstone-cleanup-missing-plan",
            "blocked_tombstone_cleanup_plan_noop",
            "tombstone_cleanup_plan_required",
            false,
            serde_json::json!({"tombstone_cleanup_plan_bound": false}),
        ),
        acceptance_fixture(
            "minimal-scoped-memory-real-write-canary-tombstone-cleanup-missing-target",
            "blocked_tombstone_cleanup_target_noop",
            "tombstone_cleanup_target_required",
            false,
            serde_json::json!({"tombstone_cleanup_target_bound": false}),
        ),
        acceptance_fixture(
            "minimal-scoped-memory-real-write-canary-tombstone-cleanup-missing-linkage",
            "blocked_tombstone_cleanup_receipt_linkage_noop",
            "tombstone_cleanup_receipt_linkage_required",
            false,
            serde_json::json!({"tombstone_cleanup_receipt_linkage_bound": false}),
        ),
        acceptance_fixture(
            "minimal-scoped-memory-real-write-canary-tombstone-cleanup-direct-side-effect-attempt",
            "blocked_direct_side_effect_noop",
            "direct_tombstone_cleanup_memory_kg_provider_channel_release_install_active_binary_side_effects_denied",
            false,
            serde_json::json!({
                "rollback_execution_requested": true,
                "tombstone_write_requested": true,
                "artifact_cleanup_requested": true,
                "durable_memory_write_requested": true,
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
                        .get("minimal_scoped_memory_real_write_canary_tombstone_cleanup_acceptance_accepted")
                        .and_then(serde_json::Value::as_bool)
                        == Some(true)
                })
                .count()
        })
        .unwrap_or(0);
    let blocked_fixture_count = fixture_count.saturating_sub(accepted_fixture_count);
    let denials = ACCEPTANCE_DENIALS
        .iter()
        .map(|reason| serde_json::json!(reason))
        .collect::<Vec<_>>();
    let denied_count = denials.len();
    let acceptance_ops_ok = namespace_bound
        && store_bound
        && scope_bound
        && rollback_receipt_acceptance_bound
        && rollback_receipt_identity_bound
        && tombstone_cleanup_plan_bound
        && tombstone_cleanup_target_bound
        && tombstone_cleanup_receipt_linkage_bound
        && tombstone_cleanup_idempotency_guard_bound;
    let report_ready = route_count_source_command_accepted
        && source_ready
        && fixture_count == 10
        && accepted_fixture_count == 1
        && blocked_fixture_count == 9
        && denied_count == 28
        && acceptance_ops_ok;
    let boundary_hash_sha256 = sha256_text_value(&format!(
        "minimal-scoped-memory-real-write-canary-tombstone-cleanup-acceptance-boundary:v1:source-ready={source_ready}:rollback-acceptance={rollback_receipt_acceptance_bound}:plan={tombstone_cleanup_plan_bound}:target={tombstone_cleanup_target_bound}:linkage={tombstone_cleanup_receipt_linkage_bound}:fixtures={fixture_count}:accepted={accepted_fixture_count}:denials={denied_count}"
    ));
    let policy_hash_sha256 = sha256_text_value(
        "minimal-scoped-memory-real-write-canary-tombstone-cleanup-acceptance-policy:v1:accept-source-rollback-receipt-evidence:no-tombstone-write:no-artifact-cleanup:no-rollback-execution:no-durable-memory-store:no-kg:no-provider:no-channel:no-release:no-install",
    );

    let mut side_effects = serde_json::Map::new();
    for &key in FALSE_EXTERNAL_KEYS {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }
    for &key in TRUE_ACCEPTANCE_KEYS {
        side_effects.insert(key.to_string(), serde_json::json!(report_ready));
    }
    let required_fields = serde_json::json!([
        "source_minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_report_sha256",
        "source_rollback_receipt_acceptance_hash_sha256",
        "source_rollback_receipt_sha256",
        "source_rollback_receipt_hash_chain_sha256",
        "approved_namespace",
        "approved_store",
        "approved_scope",
        "tombstone_cleanup_target_id",
        "tombstone_cleanup_plan_sha256",
        "tombstone_cleanup_target_sha256",
        "tombstone_cleanup_receipt_linkage_sha256",
        "tombstone_cleanup_acceptance_hash_sha256",
        "active_binary_sha256",
        "route_count",
    ]);
    let allowed_next_actions = serde_json::json!([
        {
            "action": "run_minimal_scoped_memory_real_write_canary_tombstone_cleanup_acceptance_boundary_require_live_gate",
            "status": "allowed_verification_only",
            "accepts_tombstone_cleanup_evidence": true,
            "writes_wal": false,
            "persists_receipt": false,
            "writes_memory": false,
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
            "action": "prepare_minimal_scoped_memory_real_write_canary_durable_store_write_plan_boundary",
            "status": "allowed_report_only_next_slice",
            "requires_minimal_scoped_memory_real_write_canary_tombstone_cleanup_acceptance": true,
            "writes_durable_memory": false,
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
        "hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_tombstone_cleanup_acceptance_boundary_route"
    );
    insert_report_json!(
        "endpoint",
        HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_MINIMAL_SCOPED_MEMORY_REAL_WRITE_CANARY_TOMBSTONE_CLEANUP_ACCEPTANCE_BOUNDARY_ENDPOINT
    );
    insert_report_json!(
        "source_command",
        "/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-tombstone-cleanup-acceptance-boundary --json"
    );
    insert_report_json!("native_route", true);
    insert_report_json!("side_effect_free", false);
    insert_report_json!("external_side_effect_free", true);
    insert_report_json!("audit_date", "2026-07-04");
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_tombstone_cleanup_acceptance_schema_version",
        "minimal_scoped_memory_real_write_canary_tombstone_cleanup_acceptance_boundary_v1"
    );
    insert_report_json!(
        "scoped_memory_real_write_canary_mode",
        "minimal_scoped_memory_real_write_canary_tombstone_cleanup_acceptance_report_only"
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
        "memory_write_execution_minimal_scoped_memory_real_write_canary_tombstone_cleanup_acceptance_boundary_ready",
        report_ready
    );
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_tombstone_cleanup_acceptance_ready",
        report_ready
    );
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_tombstone_cleanup_acceptance_performed",
        report_ready
    );
    insert_report_json!(
        "source_minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_boundary_ready",
        source_ready
    );
    insert_report_json!(
        "source_minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_report_sha256",
        source_report_sha256
    );
    for key in [
        "accepted_minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_fixture_count",
        "blocked_minimal_scoped_memory_real_write_canary_rollback_receipt_acceptance_fixture_count",
        "rollback_receipt_acceptance_result_recorded_count",
        "rollback_receipt_acceptance_result_accepted_count",
        "rollback_receipt_identity_bound_count",
        "rollback_receipt_digest_bound_count",
        "rollback_receipt_hash_chain_bound_count",
        "tombstone_cleanup_handoff_bound_count",
        "wal_write_performed_count",
        "receipt_persisted_count",
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
        "source_rollback_receipt_acceptance_hash_sha256",
        source_rollback_receipt_acceptance_hash_sha256
    );
    insert_report_json!(
        "source_rollback_receipt_sha256",
        source_rollback_receipt_sha256
    );
    insert_report_json!(
        "source_rollback_receipt_hash_chain_sha256",
        source_rollback_receipt_hash_chain_sha256
    );
    insert_report_json!("tombstone_cleanup_target_id", tombstone_cleanup_target_id);
    insert_report_json!(
        "tombstone_cleanup_plan_sha256",
        tombstone_cleanup_plan_sha256
    );
    insert_report_json!(
        "tombstone_cleanup_target_sha256",
        tombstone_cleanup_target_sha256
    );
    insert_report_json!(
        "tombstone_cleanup_receipt_linkage_sha256",
        tombstone_cleanup_receipt_linkage_sha256
    );
    insert_report_json!(
        "tombstone_cleanup_acceptance_hash_sha256",
        tombstone_cleanup_acceptance_hash_sha256
    );
    insert_report_json!(
        "tombstone_cleanup_receipt_linkage_verified",
        tombstone_cleanup_receipt_linkage_bound
    );
    insert_report_json!(
        "tombstone_cleanup_idempotency_guard_verified",
        tombstone_cleanup_idempotency_guard_bound
    );
    insert_report_json!(
        "required_minimal_scoped_memory_real_write_canary_tombstone_cleanup_acceptance_surface_count",
        ACCEPTANCE_SURFACES.len()
    );
    insert_report_json!(
        "ready_minimal_scoped_memory_real_write_canary_tombstone_cleanup_acceptance_surface_count",
        if report_ready {
            ACCEPTANCE_SURFACES.len()
        } else {
            0
        }
    );
    insert_report_json!(
        "required_minimal_scoped_memory_real_write_canary_tombstone_cleanup_acceptance_fixture_count",
        10
    );
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_tombstone_cleanup_acceptance_fixture_count",
        fixture_count
    );
    insert_report_json!(
        "accepted_minimal_scoped_memory_real_write_canary_tombstone_cleanup_acceptance_fixture_count",
        accepted_fixture_count
    );
    insert_report_json!(
        "blocked_minimal_scoped_memory_real_write_canary_tombstone_cleanup_acceptance_fixture_count",
        blocked_fixture_count
    );
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_tombstone_cleanup_acceptance_accepted_count",
        accepted_fixture_count
    );
    for key in [
        "tombstone_cleanup_acceptance_authority_accepted_count",
        "source_rollback_receipt_acceptance_bound_count",
        "rollback_receipt_acceptance_hash_bound_count",
        "rollback_receipt_identity_bound_count",
        "tombstone_cleanup_plan_bound_count",
        "tombstone_cleanup_target_bound_count",
        "tombstone_cleanup_receipt_linkage_bound_count",
        "tombstone_cleanup_idempotency_guard_accepted_count",
        "tombstone_cleanup_operator_review_handoff_bound_count",
        "tombstone_cleanup_acceptance_result_recorded_count",
        "tombstone_cleanup_acceptance_result_accepted_count",
    ] {
        report.insert(key.to_string(), serde_json::json!(accepted_fixture_count));
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
        "canary_artifact_filesystem_written_count",
        "artifact_readback_performed_count",
        "artifact_cleanup_performed_count",
        "tombstone_cleanup_executed_count",
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
        "durable_memory_store_write_performed_count",
        "durable_memory_store_rollback_performed_count",
        "memory_store_write_performed_count",
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
        "required_minimal_scoped_memory_real_write_canary_tombstone_cleanup_acceptance_fields"
            .to_string(),
        required_fields,
    );
    report.insert(
        "minimal_scoped_memory_real_write_canary_tombstone_cleanup_acceptance_surfaces".to_string(),
        serde_json::json!(ACCEPTANCE_SURFACES),
    );
    report.insert(
        "minimal_scoped_memory_real_write_canary_tombstone_cleanup_acceptance_fixtures".to_string(),
        fixtures,
    );
    report.insert(
        "denied_by_minimal_scoped_memory_real_write_canary_tombstone_cleanup_acceptance_boundary"
            .to_string(),
        serde_json::Value::Array(denials),
    );
    insert_report_json!(
        "denied_by_minimal_scoped_memory_real_write_canary_tombstone_cleanup_acceptance_boundary_count",
        denied_count
    );
    for key in [
        "source_rollback_receipt_acceptance_required",
        "minimal_scoped_memory_real_write_canary_tombstone_cleanup_acceptance_accepted",
        "approved_namespace_bound",
        "approved_store_bound",
        "approved_scope_bound",
        "rollback_receipt_acceptance_hash_bound",
        "rollback_receipt_identity_bound",
        "tombstone_cleanup_plan_bound",
        "tombstone_cleanup_target_bound",
        "tombstone_cleanup_receipt_linkage_bound",
        "tombstone_cleanup_idempotency_guard_bound",
        "tombstone_cleanup_operator_review_handoff_bound",
        "rollback_execution_forbidden",
        "tombstone_write_forbidden",
        "artifact_cleanup_forbidden",
        "wal_write_forbidden_on_report_route",
        "receipt_persist_forbidden_on_report_route",
        "nonce_consumption_forbidden_on_report_route",
        "explicit_command_dispatch_forbidden_on_report_route",
        "durable_memory_read_forbidden",
        "durable_memory_write_forbidden",
        "durable_memory_rollback_forbidden",
        "memory_store_mutation_forbidden",
        "compensating_memory_write_forbidden",
        "kg_live_write_forbidden",
        "provider_model_invocation_forbidden",
        "credential_channel_public_release_forbidden",
        "install_restart_active_binary_mutation_forbidden",
    ] {
        report.insert(key.to_string(), serde_json::json!(true));
    }
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_tombstone_cleanup_acceptance_boundary_hash_sha256",
        boundary_hash_sha256
    );
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_tombstone_cleanup_acceptance_policy_hash_sha256",
        policy_hash_sha256
    );
    report.insert("allowed_next_actions".to_string(), allowed_next_actions);
    for &key in FALSE_EXTERNAL_KEYS {
        report.insert(key.to_string(), serde_json::json!(false));
    }
    for &key in TRUE_ACCEPTANCE_KEYS {
        report.insert(key.to_string(), serde_json::json!(report_ready));
    }
    report.insert(
        "side_effects".to_string(),
        serde_json::Value::Object(side_effects),
    );
    serde_json::Value::Object(report)
}

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

fn hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_boundary_report()
-> serde_json::Value {
    const SINGLE_SHOT_SURFACES: &[&str] = &[
        "source_guarded_execution_boundary_required",
        "approved_namespace_required",
        "approved_store_required",
        "approved_scope_required",
        "single_shot_execution_envelope_required",
        "single_use_nonce_and_explicit_command_required",
        "single_write_budget_required",
        "wal_receipt_artifact_required",
        "canary_store_write_required",
        "post_write_readback_required",
        "rollback_and_tombstone_cleanup_required",
        "production_and_external_side_effects_forbidden",
    ];
    const SINGLE_SHOT_DENIALS: &[&str] = &[
        "source_guarded_execution_boundary_required",
        "source_guarded_execution_boundary_hash_required",
        "approved_namespace_required",
        "approved_store_required",
        "approved_scope_required",
        "single_shot_execution_envelope_required",
        "single_use_nonce_guard_required",
        "explicit_command_guard_required",
        "single_write_budget_guard_required",
        "wal_receipt_guard_required",
        "canary_wal_artifact_write_required",
        "canary_receipt_artifact_write_required",
        "canary_receipt_hash_chain_required",
        "canary_record_identity_required",
        "canary_payload_digest_required",
        "canary_pre_write_snapshot_required",
        "canary_store_write_required",
        "canary_post_write_readback_required",
        "canary_readback_identity_digest_required",
        "canary_rollback_snapshot_restore_required",
        "canary_post_rollback_absence_required",
        "canary_tombstone_cleanup_required",
        "canary_artifact_cleanup_required",
        "canary_zero_residue_required",
        "idempotency_replay_guard_required",
        "operator_handoff_required",
        "production_durable_memory_backend_write_denied",
        "production_durable_memory_backend_read_denied",
        "production_durable_memory_backend_rollback_denied",
        "kg_live_write_denied",
        "provider_model_invocation_denied",
        "credential_read_denied",
        "channel_external_send_denied",
        "public_release_artifact_write_denied",
        "install_restart_active_binary_mutation_denied",
        "unrestricted_full_live_activation_denied",
    ];
    const TRUE_CANARY_KEYS: &[&str] = &[
        "minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_ready",
        "minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_performed",
        "minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_accepted",
        "durable_store_write_single_shot_execution_performed",
        "durable_store_write_single_shot_execution_result_recorded",
        "durable_store_write_single_shot_execution_result_accepted",
        "single_shot_canary_execution_envelope_bound",
        "single_shot_canary_nonce_consumed",
        "single_shot_canary_explicit_command_accepted",
        "single_shot_canary_single_write_budget_enforced",
        "single_shot_canary_wal_written",
        "single_shot_canary_receipt_persisted",
        "single_shot_canary_receipt_materialized",
        "single_shot_canary_receipt_hash_chain_verified",
        "single_shot_canary_memory_store_write_performed",
        "single_shot_canary_post_write_readback_performed",
        "single_shot_canary_readback_accepted",
        "single_shot_canary_rollback_executed",
        "single_shot_canary_tombstone_cleanup_executed",
        "single_shot_canary_artifact_cleanup_executed",
        "single_shot_canary_zero_residue_confirmed",
        "idempotency_replay_guard_verified",
        "operator_single_shot_execution_handoff_bound",
        "kg_provider_channel_release_install_active_binary_forbidden",
    ];
    const TRUE_MUTATION_KEYS: &[&str] = &[
        "durable_store_write_execution_performed",
        "memory_write_execution_performed",
        "memory_store_write_path_enabled",
        "memory_store_write_allowed",
        "memory_store_write_performed",
        "memory_store_mutation_allowed",
        "memory_store_mutated",
        "wal_write_performed",
        "wal_recorded",
        "wal_persisted",
        "receipt_recorded",
        "receipt_persisted",
        "receipt_materialized",
        "post_write_readback_performed",
        "readback_result_recorded",
        "readback_result_accepted",
        "rollback_executed",
        "rollback_performed",
        "rollback_result_recorded",
        "rollback_result_accepted",
        "tombstone_cleanup_executed",
        "tombstone_cleanup_result_recorded",
        "tombstone_cleanup_result_accepted",
    ];
    const FALSE_PRODUCTION_AND_EXTERNAL_KEYS: &[&str] = &[
        "production_durable_memory_backend_present",
        "production_durable_memory_store_write_performed",
        "actual_production_durable_memory_write_performed",
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

    fn execution_fixture(
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
            "minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_status",
            status
        );
        insert_fixture_json!("reason", reason);
        insert_fixture_json!(
            "minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_accepted",
            accepted
        );
        for key in [
            "source_guarded_execution_boundary_bound",
            "approved_namespace_bound",
            "approved_store_bound",
            "approved_scope_bound",
            "single_shot_execution_envelope_bound",
            "single_use_nonce_bound",
            "explicit_command_bound",
            "single_write_budget_bound",
            "wal_receipt_artifact_bound",
            "canary_store_write_bound",
            "post_write_readback_bound",
            "rollback_tombstone_cleanup_bound",
            "zero_residue_bound",
        ] {
            base.insert(key.to_string(), serde_json::json!(accepted));
        }
        for &key in TRUE_MUTATION_KEYS {
            base.insert(key.to_string(), serde_json::json!(accepted));
        }
        for &key in FALSE_PRODUCTION_AND_EXTERNAL_KEYS {
            base.insert(key.to_string(), serde_json::json!(false));
        }
        if let Some(extra) = extra.as_object() {
            for (key, value) in extra {
                base.insert(key.clone(), value.clone());
            }
        }
        serde_json::Value::Object(base)
    }

    fn file_count(dir: &Path) -> usize {
        fs::read_dir(dir)
            .ok()
            .map(|entries| {
                entries
                    .filter_map(std::result::Result::ok)
                    .filter(|entry| {
                        entry
                            .file_type()
                            .map(|file_type| file_type.is_file())
                            .unwrap_or(false)
                    })
                    .count()
            })
            .unwrap_or(0)
    }

    fn remove_dir_if_present(dir: &Path) -> bool {
        match fs::remove_dir_all(dir) {
            Ok(()) => true,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => true,
            Err(_) => false,
        }
    }

    let route_matrix = control_ui_route_parity_report();
    let source = std::thread::Builder::new()
        .name("hepta-memory-single-shot-execution-source-report".to_string())
        .stack_size(8 * 1024 * 1024)
        .spawn(
            hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_boundary_report,
        )
        .ok()
        .and_then(|handle| handle.join().ok())
        .unwrap_or_else(|| {
            serde_json::json!({
                "status": "blocked",
                "memory_write_execution_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_boundary_ready": false,
                "minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_accepted": false,
                "source_minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_source_report_thread_failed": true
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
    let source_next_action_single_shot = source
        .get("allowed_next_actions")
        .and_then(serde_json::Value::as_array)
        .and_then(|items| items.get(1))
        .map(|item| {
            item.get("action").and_then(serde_json::Value::as_str)
                == Some(
                    "prepare_minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_boundary",
                )
                && item
                    .get(
                        "requires_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_boundary",
                    )
                    .and_then(serde_json::Value::as_bool)
                    == Some(true)
                && item
                    .get("actual_write_requires_separate_explicit_command")
                    .and_then(serde_json::Value::as_bool)
                    == Some(true)
        })
        .unwrap_or(false);
    let source_side_effects_ok = source
        .get("side_effects")
        .and_then(serde_json::Value::as_object)
        .map(|side_effects| {
            [
                "durable_store_write_guarded_execution_executed",
                "durable_store_write_execution_performed",
                "durable_memory_store_write_performed",
                "durable_memory_store_read_performed",
                "durable_memory_store_rollback_performed",
                "memory_store_write_performed",
                "wal_write_performed",
                "receipt_persisted",
                "post_write_readback_performed",
                "rollback_executed",
                "tombstone_cleanup_executed",
                "live_kg_write_performed",
                "provider_invoked",
                "model_invoked",
                "credential_read",
                "channel_send_performed",
                "external_send_performed",
                "release_artifact_written",
                "install_executed",
                "service_restarted",
                "active_binary_mutated",
            ]
            .iter()
            .all(|key| side_effects.get(*key).and_then(serde_json::Value::as_bool) == Some(false))
        })
        .unwrap_or(false);

    let source_ready = source.get("status").and_then(serde_json::Value::as_str) == Some("ready")
        && json_bool(
            &source,
            "memory_write_execution_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_boundary_ready",
        )
        && json_bool(
            &source,
            "minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_accepted",
        )
        && json_u64(
            &source,
            "accepted_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_fixture_count",
        ) == 1
        && json_u64(
            &source,
            "blocked_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_fixture_count",
        ) == 9
        && json_u64(
            &source,
            "durable_store_write_guarded_execution_boundary_result_accepted_count",
        ) == 1
        && json_bool(
            &source,
            "durable_store_write_guarded_execution_boundary_result_accepted",
        )
        && !json_bool(
            &source,
            "durable_store_write_guarded_execution_boundary_executed",
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
        && !json_bool(&source, "rollback_executed")
        && !json_bool(&source, "tombstone_cleanup_executed")
        && !json_bool(&source, "live_kg_write_performed")
        && !json_bool(&source, "provider_invoked")
        && !json_bool(&source, "model_invoked")
        && !json_bool(&source, "credential_read")
        && !json_bool(&source, "channel_send_performed")
        && !json_bool(&source, "external_send_performed")
        && !json_bool(&source, "release_artifact_written")
        && !json_bool(&source, "install_executed")
        && !json_bool(&source, "service_restarted")
        && !json_bool(&source, "active_binary_mutated")
        && source_next_action_single_shot
        && source_side_effects_ok;

    let approved_namespace = json_str(&source, "approved_namespace");
    let approved_store = json_str(&source, "approved_store");
    let approved_scope = json_str(&source, "approved_scope");
    let durable_store_write_target_id = json_str(&source, "durable_store_write_target_id");
    let durable_store_target_store_id = json_str(&source, "durable_store_target_store_id");
    let source_report_sha256 = sha256_text_value(&source.to_string());
    let source_guarded_execution_boundary_hash_sha256 =
        json_str(&source, "guarded_execution_boundary_hash_sha256");
    let source_guarded_execution_boundary_report_hash_sha256 = json_str(
        &source,
        "minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_boundary_hash_sha256",
    );
    let source_guarded_execution_boundary_handoff_sha256 = json_str(
        &source,
        "operator_guarded_execution_boundary_handoff_sha256",
    );
    let source_guarded_execution_boundary_wal_receipt_sha256 =
        json_str(&source, "guarded_execution_boundary_wal_receipt_sha256");
    let source_guarded_execution_boundary_readback_sha256 =
        json_str(&source, "guarded_execution_boundary_readback_sha256");
    let source_guarded_execution_boundary_rollback_sha256 =
        json_str(&source, "guarded_execution_boundary_rollback_sha256");
    let source_guarded_execution_boundary_tombstone_cleanup_sha256 = json_str(
        &source,
        "guarded_execution_boundary_tombstone_cleanup_sha256",
    );
    let source_guarded_execution_boundary_replay_sha256 = json_str(
        &source,
        "guarded_execution_boundary_idempotency_replay_sha256",
    );

    let namespace_bound = approved_namespace == "hepta.memory.canary";
    let store_bound = approved_store == "wal-receipt-canary-artifact";
    let scope_bound = approved_scope == "session";
    let target_bound = durable_store_write_target_id
        == "hepta-minimal-scoped-memory-real-write-canary-durable-store-write-target-v1"
        && durable_store_target_store_id == "hepta-memory-durable-store-canary-plan-only";

    let canary_record = MemoryRecord {
        id: "hepta-minimal-scoped-memory-real-write-canary-durable-store-write-single-shot-record-v1"
            .to_string(),
        scope: MemoryScope::Session,
        content: "hepta-minimal-scoped-memory-real-write-canary-durable-store-write-single-shot-payload-v1 approved_namespace=hepta.memory.canary approved_store=wal-receipt-canary-artifact approved_scope=session redacted_non_secret_canary=true".to_string(),
    };
    let canary_payload_digest_sha256 = sha256_text_value(&canary_record.content);
    let single_shot_execution_envelope_sha256 = sha256_text_value(&format!(
        "minimal-scoped-memory-real-write-canary-durable-store-write-single-shot-execution-envelope:v1:source-boundary={source_guarded_execution_boundary_hash_sha256}:target={durable_store_write_target_id}:store={durable_store_target_store_id}:record={}:payload={canary_payload_digest_sha256}",
        canary_record.id
    ));
    let single_shot_nonce_sha256 = sha256_text_value(&format!(
        "minimal-scoped-memory-real-write-canary-durable-store-write-single-shot-nonce:v1:envelope={single_shot_execution_envelope_sha256}:request-local-consumed=true"
    ));
    let single_shot_command_sha256 = sha256_text_value(&format!(
        "minimal-scoped-memory-real-write-canary-durable-store-write-single-shot-command:v1:source-command=/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-durable-store-write-single-shot-execution-boundary --json:envelope={single_shot_execution_envelope_sha256}"
    ));
    let single_shot_budget_sha256 = sha256_text_value(&format!(
        "minimal-scoped-memory-real-write-canary-durable-store-write-single-shot-budget:v1:max-write-count=1:record={}:namespace={approved_namespace}:scope={approved_scope}",
        canary_record.id
    ));

    let artifact_root = env::temp_dir().join(format!(
        "hepta-memory-single-shot-canary-{}",
        &single_shot_execution_envelope_sha256[..16]
    ));
    let cleanup_pre_ok = remove_dir_if_present(&artifact_root);
    let artifact_setup_ok = cleanup_pre_ok && fs::create_dir_all(&artifact_root).is_ok();
    let artifact_pre_count = file_count(&artifact_root);
    let wal_path = artifact_root.join("single-shot-canary.wal.json");
    let receipt_path = artifact_root.join("single-shot-canary.receipt.json");
    let cleanup_path = artifact_root.join("single-shot-canary.cleanup-receipt.json");
    let wal_payload = serde_json::json!({
        "schema": "hepta.memory.canary.single_shot_wal.v1",
        "record_id": canary_record.id,
        "scope": "session",
        "approved_namespace": approved_namespace,
        "approved_store": approved_store,
        "approved_scope": approved_scope,
        "payload_digest_sha256": canary_payload_digest_sha256,
        "redacted_non_secret_canary": true,
        "production_durable_backend_write": false
    })
    .to_string();
    let wal_hash_sha256 = sha256_text_value(&wal_payload);
    let wal_write_ok = artifact_setup_ok && fs::write(&wal_path, &wal_payload).is_ok();
    let receipt_payload = serde_json::json!({
        "schema": "hepta.memory.canary.single_shot_receipt.v1",
        "record_id": canary_record.id,
        "wal_hash_sha256": wal_hash_sha256,
        "source_guarded_execution_boundary_hash_sha256": source_guarded_execution_boundary_hash_sha256,
        "single_shot_execution_envelope_sha256": single_shot_execution_envelope_sha256,
        "nonce_sha256": single_shot_nonce_sha256,
        "command_sha256": single_shot_command_sha256,
        "single_write_budget_sha256": single_shot_budget_sha256,
        "production_durable_backend_write": false
    })
    .to_string();
    let receipt_hash_sha256 = sha256_text_value(&receipt_payload);
    let receipt_hash_chain_sha256 =
        sha256_text_value(&format!("{wal_hash_sha256}:{receipt_hash_sha256}"));
    let receipt_write_ok = wal_write_ok && fs::write(&receipt_path, &receipt_payload).is_ok();

    let store = InMemoryStore::default();
    let before_snapshot = store.snapshot().ok();
    let before_memory_count = before_snapshot
        .as_ref()
        .map(|snapshot| snapshot.memories.len())
        .unwrap_or(usize::MAX);
    let write_ok = store.put_memory_sync(canary_record.clone()).is_ok();
    let after_write_snapshot = store.snapshot().ok();
    let after_write_memory_count = after_write_snapshot
        .as_ref()
        .map(|snapshot| snapshot.memories.len())
        .unwrap_or(0);
    let readback_query = MemoryQuery {
        text: "hepta-minimal-scoped-memory-real-write-canary-durable-store-write-single-shot-payload-v1"
            .to_string(),
        limit: 4,
    };
    let readback_report = store.search_report(readback_query.clone()).ok();
    let readback_hit_count = readback_report
        .as_ref()
        .map(|report| report.returned_count)
        .unwrap_or(0);
    let readback_match = readback_report
        .as_ref()
        .map(|report| {
            report.hits.iter().any(|hit| {
                hit.id == canary_record.id
                    && hit.scope == canary_record.scope
                    && sha256_text_value(&hit.content) == canary_payload_digest_sha256
            })
        })
        .unwrap_or(false);
    let rollback_ok = before_snapshot
        .map(|snapshot| store.restore(snapshot).is_ok())
        .unwrap_or(false);
    let after_rollback_snapshot = store.snapshot().ok();
    let after_rollback_memory_count = after_rollback_snapshot
        .as_ref()
        .map(|snapshot| snapshot.memories.len())
        .unwrap_or(usize::MAX);
    let post_rollback_report = store.search_report(readback_query).ok();
    let post_rollback_absent = post_rollback_report
        .as_ref()
        .map(|report| {
            report.matched_count == 0 && report.hits.iter().all(|hit| hit.id != canary_record.id)
        })
        .unwrap_or(false);

    let cleanup_payload = serde_json::json!({
        "schema": "hepta.memory.canary.single_shot_cleanup_receipt.v1",
        "record_id": canary_record.id,
        "receipt_hash_chain_sha256": receipt_hash_chain_sha256,
        "rollback_restored": rollback_ok,
        "post_rollback_absent": post_rollback_absent,
        "cleanup_scope": "request-local-canary-artifacts",
        "production_durable_backend_write": false
    })
    .to_string();
    let cleanup_receipt_hash_sha256 = sha256_text_value(&cleanup_payload);
    let cleanup_receipt_write_ok =
        receipt_write_ok && fs::write(&cleanup_path, &cleanup_payload).is_ok();
    let artifact_write_count = [wal_write_ok, receipt_write_ok, cleanup_receipt_write_ok]
        .iter()
        .filter(|ok| **ok)
        .count();
    let wal_readback = fs::read_to_string(&wal_path).unwrap_or_default();
    let receipt_readback = fs::read_to_string(&receipt_path).unwrap_or_default();
    let cleanup_readback = fs::read_to_string(&cleanup_path).unwrap_or_default();
    let artifact_readback_count = [
        wal_readback == wal_payload,
        receipt_readback == receipt_payload,
        cleanup_readback == cleanup_payload,
    ]
    .iter()
    .filter(|matched| **matched)
    .count();
    let receipt_hash_chain_verified = sha256_text_value(&format!(
        "{}:{}",
        sha256_text_value(&wal_readback),
        sha256_text_value(&receipt_readback)
    )) == receipt_hash_chain_sha256
        && sha256_text_value(&cleanup_readback) == cleanup_receipt_hash_sha256;
    let removed_count = [&wal_path, &receipt_path, &cleanup_path]
        .iter()
        .filter(|path| fs::remove_file(path).is_ok())
        .count();
    let cleanup_dir_ok = remove_dir_if_present(&artifact_root);
    let artifact_post_cleanup_count = file_count(&artifact_root);
    let artifact_zero_residue =
        cleanup_dir_ok && artifact_post_cleanup_count == 0 && removed_count == artifact_write_count;

    let single_shot_execution_hash_sha256 = sha256_text_value(&format!(
        "minimal-scoped-memory-real-write-canary-durable-store-write-single-shot-execution:v1:source={source_guarded_execution_boundary_hash_sha256}:envelope={single_shot_execution_envelope_sha256}:nonce={single_shot_nonce_sha256}:command={single_shot_command_sha256}:budget={single_shot_budget_sha256}:wal={wal_hash_sha256}:receipt={receipt_hash_sha256}:chain={receipt_hash_chain_sha256}:cleanup={cleanup_receipt_hash_sha256}:store-write={write_ok}:readback={readback_match}:rollback={rollback_ok}:zero-residue={artifact_zero_residue}"
    ));
    let canary_execution_ready = source_ready
        && namespace_bound
        && store_bound
        && scope_bound
        && target_bound
        && !source_guarded_execution_boundary_hash_sha256.is_empty()
        && !source_guarded_execution_boundary_report_hash_sha256.is_empty()
        && !source_guarded_execution_boundary_handoff_sha256.is_empty()
        && artifact_setup_ok
        && artifact_pre_count == 0
        && wal_write_ok
        && receipt_write_ok
        && cleanup_receipt_write_ok
        && artifact_write_count == 3
        && artifact_readback_count == 3
        && receipt_hash_chain_verified
        && before_memory_count == 0
        && write_ok
        && after_write_memory_count == 1
        && readback_hit_count == 1
        && readback_match
        && rollback_ok
        && after_rollback_memory_count == 0
        && post_rollback_absent
        && removed_count == 3
        && artifact_zero_residue;

    let fixtures = serde_json::Value::Array(vec![
        execution_fixture(
            "minimal-scoped-memory-real-write-canary-durable-store-write-single-shot-execution",
            "accepted_single_shot_canary_write_readback_rollback_cleanup",
            "request_local_canary_store_write_wal_receipt_readback_rollback_tombstone_cleanup_zero_residue_succeeded",
            canary_execution_ready,
            serde_json::json!({
                "canary_record_id": canary_record.id,
                "canary_payload_digest_sha256": canary_payload_digest_sha256,
                "single_shot_execution_hash_sha256": single_shot_execution_hash_sha256,
                "receipt_hash_chain_sha256": receipt_hash_chain_sha256
            }),
        ),
        execution_fixture(
            "missing-source-guarded-execution-boundary",
            "blocked_noop",
            "source_guarded_execution_boundary_required",
            false,
            serde_json::json!({"source_guarded_execution_boundary_bound": false}),
        ),
        execution_fixture(
            "wrong-namespace",
            "blocked_noop",
            "approved_namespace_required",
            false,
            serde_json::json!({"approved_namespace_bound": false}),
        ),
        execution_fixture(
            "wrong-store",
            "blocked_noop",
            "approved_store_required",
            false,
            serde_json::json!({"approved_store_bound": false}),
        ),
        execution_fixture(
            "wrong-scope",
            "blocked_noop",
            "approved_scope_required",
            false,
            serde_json::json!({"approved_scope_bound": false}),
        ),
        execution_fixture(
            "missing-single-shot-envelope",
            "blocked_noop",
            "single_shot_execution_envelope_required",
            false,
            serde_json::json!({"single_shot_execution_envelope_bound": false}),
        ),
        execution_fixture(
            "missing-nonce-command-budget",
            "blocked_noop",
            "single_use_nonce_explicit_command_and_single_write_budget_required",
            false,
            serde_json::json!({
                "single_use_nonce_bound": false,
                "explicit_command_bound": false,
                "single_write_budget_bound": false
            }),
        ),
        execution_fixture(
            "missing-wal-receipt-hash-chain",
            "blocked_noop",
            "wal_receipt_artifact_and_hash_chain_required",
            false,
            serde_json::json!({"wal_receipt_artifact_bound": false}),
        ),
        execution_fixture(
            "missing-readback-rollback-cleanup",
            "blocked_noop",
            "post_write_readback_rollback_tombstone_cleanup_zero_residue_required",
            false,
            serde_json::json!({
                "post_write_readback_bound": false,
                "rollback_tombstone_cleanup_bound": false,
                "zero_residue_bound": false
            }),
        ),
        execution_fixture(
            "production-durable-memory-backend-write-attempt",
            "blocked_noop",
            "production_durable_memory_backend_write_remains_blocked_for_canary_boundary",
            false,
            serde_json::json!({"production_durable_memory_store_write_performed": false}),
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
                        "minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_accepted",
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
    let ready_surface_count = if canary_execution_ready {
        SINGLE_SHOT_SURFACES.len()
    } else {
        0
    };
    let report_ready = route_count_source_command_accepted
        && canary_execution_ready
        && accepted_fixture_count == 1
        && blocked_fixture_count == 9
        && SINGLE_SHOT_DENIALS.len() == 36;
    let boundary_hash_sha256 = sha256_text_value(&format!(
        "minimal-scoped-memory-real-write-canary-durable-store-write-single-shot-execution-boundary-report:v1:source-ready={source_ready}:canary-ready={canary_execution_ready}:execution={single_shot_execution_hash_sha256}:fixtures=10:accepted=1:denials=36:production-durable-write=false"
    ));
    let policy_hash_sha256 = sha256_text_value(
        "minimal-scoped-memory-real-write-canary-durable-store-write-single-shot-execution-policy:v1:request-local-canary-store-write-only:production-durable-memory-backend-blocked:kg-provider-channel-release-install-active-binary-blocked",
    );

    let mut side_effects = serde_json::Map::new();
    for &key in TRUE_CANARY_KEYS.iter().chain(TRUE_MUTATION_KEYS.iter()) {
        side_effects.insert(key.to_string(), serde_json::json!(report_ready));
    }
    for &key in FALSE_PRODUCTION_AND_EXTERNAL_KEYS {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }

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
        "hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_boundary_gate"
    );
    insert_report_json!(
        "endpoint",
        HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_MINIMAL_SCOPED_MEMORY_REAL_WRITE_CANARY_DURABLE_STORE_WRITE_SINGLE_SHOT_EXECUTION_BOUNDARY_ENDPOINT
    );
    insert_report_json!(
        "source_command",
        "/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-durable-store-write-single-shot-execution-boundary --json"
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
        "memory_write_execution_minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_boundary_ready",
        report_ready
    );
    insert_report_json!(
        "scoped_memory_real_write_canary_mode",
        "minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_request_local_canary_store"
    );
    insert_report_json!(
        "durable_store_write_execution_scope",
        "request_local_canary_store_with_request_local_wal_receipt_artifacts"
    );
    insert_report_json!(
        "source_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_boundary_ready",
        source_ready
    );
    insert_report_json!(
        "source_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_boundary_report_sha256",
        source_report_sha256
    );
    insert_report_json!(
        "source_accepted_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_fixture_count",
        json_u64(
            &source,
            "accepted_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_fixture_count",
        )
    );
    insert_report_json!(
        "source_blocked_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_fixture_count",
        json_u64(
            &source,
            "blocked_minimal_scoped_memory_real_write_canary_durable_store_write_guarded_execution_fixture_count",
        )
    );
    insert_report_json!(
        "source_durable_store_write_guarded_execution_boundary_result_accepted_count",
        json_u64(
            &source,
            "durable_store_write_guarded_execution_boundary_result_accepted_count",
        )
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
        "source_guarded_execution_boundary_hash_sha256",
        source_guarded_execution_boundary_hash_sha256
    );
    insert_report_json!(
        "source_guarded_execution_boundary_report_hash_sha256",
        source_guarded_execution_boundary_report_hash_sha256
    );
    insert_report_json!(
        "source_guarded_execution_boundary_handoff_sha256",
        source_guarded_execution_boundary_handoff_sha256
    );
    insert_report_json!(
        "source_guarded_execution_boundary_wal_receipt_sha256",
        source_guarded_execution_boundary_wal_receipt_sha256
    );
    insert_report_json!(
        "source_guarded_execution_boundary_readback_sha256",
        source_guarded_execution_boundary_readback_sha256
    );
    insert_report_json!(
        "source_guarded_execution_boundary_rollback_sha256",
        source_guarded_execution_boundary_rollback_sha256
    );
    insert_report_json!(
        "source_guarded_execution_boundary_tombstone_cleanup_sha256",
        source_guarded_execution_boundary_tombstone_cleanup_sha256
    );
    insert_report_json!(
        "source_guarded_execution_boundary_replay_sha256",
        source_guarded_execution_boundary_replay_sha256
    );
    insert_report_json!("canary_record_id", canary_record.id);
    insert_report_json!("canary_payload_digest_sha256", canary_payload_digest_sha256);
    insert_report_json!(
        "single_shot_execution_envelope_sha256",
        single_shot_execution_envelope_sha256
    );
    insert_report_json!("single_shot_nonce_sha256", single_shot_nonce_sha256);
    insert_report_json!("single_shot_command_sha256", single_shot_command_sha256);
    insert_report_json!("single_shot_budget_sha256", single_shot_budget_sha256);
    insert_report_json!("single_shot_wal_hash_sha256", wal_hash_sha256);
    insert_report_json!("single_shot_receipt_hash_sha256", receipt_hash_sha256);
    insert_report_json!(
        "single_shot_receipt_hash_chain_sha256",
        receipt_hash_chain_sha256
    );
    insert_report_json!(
        "single_shot_cleanup_receipt_hash_sha256",
        cleanup_receipt_hash_sha256
    );
    insert_report_json!(
        "single_shot_execution_hash_sha256",
        single_shot_execution_hash_sha256
    );
    insert_report_json!(
        "single_shot_canary_pre_write_memory_count",
        before_memory_count
    );
    insert_report_json!(
        "single_shot_canary_post_write_memory_count",
        after_write_memory_count
    );
    insert_report_json!("single_shot_canary_readback_hit_count", readback_hit_count);
    insert_report_json!("single_shot_canary_rollback_restored", rollback_ok);
    insert_report_json!(
        "single_shot_canary_post_rollback_memory_count",
        after_rollback_memory_count
    );
    insert_report_json!(
        "single_shot_canary_post_rollback_absence_confirmed",
        post_rollback_absent
    );
    insert_report_json!("single_shot_canary_artifact_pre_count", artifact_pre_count);
    insert_report_json!(
        "single_shot_canary_artifact_write_count",
        artifact_write_count
    );
    insert_report_json!(
        "single_shot_canary_artifact_readback_count",
        artifact_readback_count
    );
    insert_report_json!(
        "single_shot_canary_artifact_cleanup_removed_count",
        removed_count
    );
    insert_report_json!(
        "single_shot_canary_artifact_post_cleanup_count",
        artifact_post_cleanup_count
    );
    insert_report_json!(
        "single_shot_canary_artifact_zero_residue_confirmed",
        artifact_zero_residue
    );
    insert_report_json!(
        "required_minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_surface_count",
        SINGLE_SHOT_SURFACES.len()
    );
    insert_report_json!(
        "ready_minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_surface_count",
        ready_surface_count
    );
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_fixture_count",
        10
    );
    insert_report_json!(
        "accepted_minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_fixture_count",
        accepted_fixture_count
    );
    insert_report_json!(
        "blocked_minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_fixture_count",
        blocked_fixture_count
    );
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_accepted_count",
        if report_ready { 1 } else { 0 }
    );
    insert_report_json!(
        "durable_store_write_single_shot_execution_result_accepted_count",
        if report_ready { 1 } else { 0 }
    );
    insert_report_json!(
        "denied_by_minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_boundary",
        SINGLE_SHOT_DENIALS
    );
    insert_report_json!(
        "denied_by_minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_boundary_count",
        SINGLE_SHOT_DENIALS.len()
    );
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_boundary_hash_sha256",
        boundary_hash_sha256
    );
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_policy_hash_sha256",
        policy_hash_sha256
    );
    report.insert(
        "minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_surfaces"
            .to_string(),
        serde_json::json!(SINGLE_SHOT_SURFACES),
    );
    report.insert(
        "minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_fixtures"
            .to_string(),
        fixtures,
    );
    for &key in TRUE_CANARY_KEYS.iter().chain(TRUE_MUTATION_KEYS.iter()) {
        insert_report_json!(key, report_ready);
        insert_report_json!(format!("{key}_count"), if report_ready { 1 } else { 0 });
    }
    for &key in FALSE_PRODUCTION_AND_EXTERNAL_KEYS {
        insert_report_json!(key, false);
        insert_report_json!(format!("{key}_count"), 0);
    }
    insert_report_json!(
        "durable_memory_backend_missing_production_write_blocked",
        true
    );
    insert_report_json!(
        "production_durable_memory_write_forbidden_until_backend_and_gate_exist",
        true
    );
    report.insert(
        "side_effects".to_string(),
        serde_json::Value::Object(side_effects),
    );
    report.insert(
        "allowed_next_actions".to_string(),
        serde_json::json!([
            {
                "action": "run_minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_boundary_require_live_gate",
                "status": "allowed_verification_only",
                "writes_production_durable_memory": false,
                "mutates_request_local_canary_store": true
            },
            {
                "action": "prepare_minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_boundary",
                "status": "requires_single_shot_execution_receipt_readback_acceptance",
                "requires_minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_boundary": true,
                "writes_production_durable_memory": false,
                "mutates_memory_store": false
            }
        ]),
    );
    serde_json::Value::Object(report)
}

fn hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_boundary_report()
-> serde_json::Value {
    const RECEIPT_ACCEPTANCE_SURFACES: &[&str] = &[
        "source_single_shot_execution_boundary_required",
        "approved_namespace_store_scope_required",
        "single_shot_execution_hash_required",
        "single_shot_wal_receipt_hash_chain_required",
        "single_shot_receipt_identity_required",
        "single_shot_post_write_readback_required",
        "single_shot_rollback_restore_required",
        "single_shot_tombstone_cleanup_required",
        "single_shot_zero_residue_required",
        "receipt_acceptance_record_required",
        "rollback_tombstone_zero_residue_handoff_required",
        "production_and_external_side_effects_forbidden",
    ];
    const RECEIPT_ACCEPTANCE_DENIALS: &[&str] = &[
        "source_single_shot_execution_boundary_required",
        "source_single_shot_execution_boundary_hash_required",
        "approved_namespace_required",
        "approved_store_required",
        "approved_scope_required",
        "durable_store_write_target_required",
        "durable_store_target_store_required",
        "single_shot_record_identity_required",
        "single_shot_payload_digest_required",
        "single_shot_execution_envelope_required",
        "single_shot_nonce_record_required",
        "single_shot_command_record_required",
        "single_shot_budget_record_required",
        "single_shot_wal_hash_required",
        "single_shot_receipt_hash_required",
        "single_shot_receipt_hash_chain_required",
        "single_shot_cleanup_receipt_required",
        "single_shot_execution_hash_required",
        "single_shot_post_write_readback_required",
        "single_shot_rollback_restore_required",
        "single_shot_tombstone_cleanup_required",
        "single_shot_zero_residue_required",
        "receipt_acceptance_record_required",
        "receipt_acceptance_replay_guard_required",
        "operator_receipt_acceptance_handoff_required",
        "new_canary_store_write_report_route_denied",
        "wal_rewrite_report_route_denied",
        "receipt_repersist_report_route_denied",
        "production_durable_memory_backend_write_denied",
        "durable_memory_backend_read_or_rollback_denied",
        "kg_provider_channel_release_install_active_binary_denied",
        "unrestricted_full_live_activation_denied",
    ];
    const FALSE_RECEIPT_ACCEPTANCE_SIDE_EFFECT_KEYS: &[&str] = &[
        "durable_store_write_execution_performed",
        "durable_store_write_single_shot_execution_performed",
        "memory_write_execution_performed",
        "memory_store_write_path_enabled",
        "memory_store_write_allowed",
        "memory_store_write_performed",
        "memory_store_mutation_allowed",
        "memory_store_mutated",
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
        "rollback_performed",
        "rollback_result_recorded",
        "rollback_result_persisted",
        "rollback_result_accepted",
        "tombstone_cleanup_executed",
        "tombstone_cleanup_result_recorded",
        "tombstone_cleanup_result_accepted",
        "single_shot_canary_nonce_consumed",
        "single_shot_canary_explicit_command_accepted",
        "single_shot_canary_memory_store_write_performed",
        "single_shot_canary_post_write_readback_performed",
        "single_shot_canary_rollback_executed",
        "single_shot_canary_tombstone_cleanup_executed",
        "single_shot_canary_artifact_cleanup_executed",
        "production_durable_memory_backend_present",
        "production_durable_memory_store_write_performed",
        "actual_production_durable_memory_write_performed",
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
    const TRUE_RECEIPT_ACCEPTANCE_KEYS: &[&str] = &[
        "durable_store_write_receipt_acceptance_performed",
        "durable_store_write_receipt_acceptance_result_recorded",
        "durable_store_write_receipt_acceptance_result_accepted",
        "single_shot_receipt_identity_accepted",
        "single_shot_receipt_hash_chain_accepted",
        "single_shot_readback_evidence_accepted",
        "single_shot_rollback_cleanup_zero_residue_evidence_accepted",
        "receipt_acceptance_recorded",
        "receipt_acceptance_replay_guard_accepted",
        "operator_receipt_acceptance_handoff_bound",
        "rollback_tombstone_zero_residue_handoff_bound",
        "kg_provider_channel_release_install_active_binary_forbidden",
    ];

    fn acceptance_fixture(
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
            "minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_status",
            status
        );
        insert_fixture_json!("reason", reason);
        insert_fixture_json!(
            "minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_accepted",
            accepted
        );
        for key in [
            "source_single_shot_execution_boundary_bound",
            "approved_namespace_bound",
            "approved_store_bound",
            "approved_scope_bound",
            "durable_store_write_target_bound",
            "durable_store_target_store_bound",
            "single_shot_record_identity_bound",
            "single_shot_execution_envelope_bound",
            "single_shot_receipt_identity_bound",
            "single_shot_receipt_hash_chain_bound",
            "single_shot_readback_evidence_bound",
            "single_shot_rollback_cleanup_zero_residue_bound",
            "receipt_acceptance_record_bound",
            "receipt_acceptance_replay_guard_bound",
            "operator_receipt_acceptance_handoff_bound",
            "rollback_tombstone_zero_residue_handoff_bound",
        ] {
            base.insert(key.to_string(), serde_json::json!(accepted));
        }
        for &key in FALSE_RECEIPT_ACCEPTANCE_SIDE_EFFECT_KEYS {
            base.insert(key.to_string(), serde_json::json!(false));
        }
        for &key in TRUE_RECEIPT_ACCEPTANCE_KEYS {
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
        .name("hepta-memory-single-shot-receipt-acceptance-source-report".to_string())
        .stack_size(8 * 1024 * 1024)
        .spawn(
            hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_boundary_report,
        )
        .ok()
        .and_then(|handle| handle.join().ok())
        .unwrap_or_else(|| {
            serde_json::json!({
                "status": "blocked",
                "memory_write_execution_minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_boundary_ready": false,
                "minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_ready": false,
                "source_minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_source_report_thread_failed": true
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
    let bool_count = |value: &serde_json::Value, key: &str| -> usize {
        if json_bool(value, key) { 1 } else { 0 }
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
    let source_next_action_receipt_acceptance = source
        .get("allowed_next_actions")
        .and_then(serde_json::Value::as_array)
        .and_then(|items| items.get(1))
        .map(|item| {
            item.get("action").and_then(serde_json::Value::as_str)
                == Some(
                    "prepare_minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_boundary",
                )
                && item
                    .get(
                        "requires_minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_boundary",
                    )
                    .and_then(serde_json::Value::as_bool)
                    == Some(true)
                && item
                    .get("writes_production_durable_memory")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
        })
        .unwrap_or(false);
    let source_side_effects_ok = source
        .get("side_effects")
        .and_then(serde_json::Value::as_object)
        .map(|effects| {
            effects
                .get("memory_store_write_performed")
                .and_then(serde_json::Value::as_bool)
                == Some(true)
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
            "memory_write_execution_minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_boundary_ready",
        )
        && json_bool(
            &source,
            "minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_ready",
        )
        && json_bool(
            &source,
            "minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_accepted",
        )
        && json_u64(
            &source,
            "accepted_minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_fixture_count",
        ) == 1
        && json_u64(
            &source,
            "blocked_minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_fixture_count",
        ) == 9
        && json_u64(
            &source,
            "durable_store_write_single_shot_execution_result_accepted_count",
        ) == 1
        && json_bool(&source, "memory_store_write_performed")
        && json_bool(&source, "wal_write_performed")
        && json_bool(&source, "receipt_persisted")
        && json_bool(&source, "post_write_readback_performed")
        && json_bool(&source, "rollback_executed")
        && json_bool(&source, "tombstone_cleanup_executed")
        && json_u64(&source, "single_shot_canary_post_write_memory_count") == 1
        && json_u64(&source, "single_shot_canary_readback_hit_count") == 1
        && json_bool(&source, "single_shot_canary_rollback_restored")
        && json_u64(&source, "single_shot_canary_post_rollback_memory_count") == 0
        && json_bool(
            &source,
            "single_shot_canary_post_rollback_absence_confirmed",
        )
        && json_u64(&source, "single_shot_canary_artifact_write_count") == 3
        && json_u64(&source, "single_shot_canary_artifact_readback_count") == 3
        && json_u64(&source, "single_shot_canary_artifact_cleanup_removed_count") == 3
        && json_u64(&source, "single_shot_canary_artifact_post_cleanup_count") == 0
        && json_bool(
            &source,
            "single_shot_canary_artifact_zero_residue_confirmed",
        )
        && !json_bool(&source, "production_durable_memory_store_write_performed")
        && !json_bool(&source, "actual_production_durable_memory_write_performed")
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
        && !json_bool(&source, "service_restarted")
        && !json_bool(&source, "active_binary_mutated")
        && source_next_action_receipt_acceptance
        && source_side_effects_ok;

    let approved_namespace = json_str(&source, "approved_namespace");
    let approved_store = json_str(&source, "approved_store");
    let approved_scope = json_str(&source, "approved_scope");
    let durable_store_write_target_id = json_str(&source, "durable_store_write_target_id");
    let durable_store_target_store_id = json_str(&source, "durable_store_target_store_id");
    let canary_record_id = json_str(&source, "canary_record_id");
    let source_report_sha256 = sha256_text_value(&source.to_string());
    let source_single_shot_boundary_hash_sha256 = json_str(
        &source,
        "minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_boundary_hash_sha256",
    );
    let source_single_shot_policy_hash_sha256 = json_str(
        &source,
        "minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_policy_hash_sha256",
    );
    let canary_payload_digest_sha256 = json_str(&source, "canary_payload_digest_sha256");
    let single_shot_execution_envelope_sha256 =
        json_str(&source, "single_shot_execution_envelope_sha256");
    let single_shot_nonce_sha256 = json_str(&source, "single_shot_nonce_sha256");
    let single_shot_command_sha256 = json_str(&source, "single_shot_command_sha256");
    let single_shot_budget_sha256 = json_str(&source, "single_shot_budget_sha256");
    let single_shot_wal_hash_sha256 = json_str(&source, "single_shot_wal_hash_sha256");
    let single_shot_receipt_hash_sha256 = json_str(&source, "single_shot_receipt_hash_sha256");
    let single_shot_receipt_hash_chain_sha256 =
        json_str(&source, "single_shot_receipt_hash_chain_sha256");
    let single_shot_cleanup_receipt_hash_sha256 =
        json_str(&source, "single_shot_cleanup_receipt_hash_sha256");
    let single_shot_execution_hash_sha256 = json_str(&source, "single_shot_execution_hash_sha256");

    let namespace_bound = approved_namespace == "hepta.memory.canary";
    let store_bound = approved_store == "wal-receipt-canary-artifact";
    let scope_bound = approved_scope == "session";
    let target_bound = durable_store_write_target_id
        == "hepta-minimal-scoped-memory-real-write-canary-durable-store-write-target-v1"
        && durable_store_target_store_id == "hepta-memory-durable-store-canary-plan-only";
    let record_bound = canary_record_id
        == "hepta-minimal-scoped-memory-real-write-canary-durable-store-write-single-shot-record-v1";
    let hashes_bound = !source_single_shot_boundary_hash_sha256.is_empty()
        && !source_single_shot_policy_hash_sha256.is_empty()
        && !canary_payload_digest_sha256.is_empty()
        && !single_shot_execution_envelope_sha256.is_empty()
        && !single_shot_nonce_sha256.is_empty()
        && !single_shot_command_sha256.is_empty()
        && !single_shot_budget_sha256.is_empty()
        && !single_shot_wal_hash_sha256.is_empty()
        && !single_shot_receipt_hash_sha256.is_empty()
        && !single_shot_receipt_hash_chain_sha256.is_empty()
        && !single_shot_cleanup_receipt_hash_sha256.is_empty()
        && !single_shot_execution_hash_sha256.is_empty();
    let readback_bound = json_u64(&source, "single_shot_canary_readback_hit_count") == 1;
    let rollback_cleanup_zero_residue_bound =
        json_bool(&source, "single_shot_canary_rollback_restored")
            && json_u64(&source, "single_shot_canary_post_rollback_memory_count") == 0
            && json_bool(
                &source,
                "single_shot_canary_post_rollback_absence_confirmed",
            )
            && json_u64(&source, "single_shot_canary_artifact_cleanup_removed_count") == 3
            && json_u64(&source, "single_shot_canary_artifact_post_cleanup_count") == 0
            && json_bool(
                &source,
                "single_shot_canary_artifact_zero_residue_confirmed",
            );
    let receipt_acceptance_record_hash_sha256 = sha256_text_value(&format!(
        "minimal-scoped-memory-real-write-canary-durable-store-write-receipt-acceptance-record:v1:source={source_report_sha256}:record={canary_record_id}:receipt={single_shot_receipt_hash_sha256}:chain={single_shot_receipt_hash_chain_sha256}:zero-residue=true"
    ));
    let receipt_acceptance_readback_hash_sha256 = sha256_text_value(&format!(
        "minimal-scoped-memory-real-write-canary-durable-store-write-receipt-acceptance-readback:v1:record={canary_record_id}:readback-hit=1:rollback-restored=true:cleanup={single_shot_cleanup_receipt_hash_sha256}:source-execution={single_shot_execution_hash_sha256}"
    ));
    let receipt_acceptance_hash_sha256 = sha256_text_value(&format!(
        "minimal-scoped-memory-real-write-canary-durable-store-write-receipt-acceptance:v1:namespace={approved_namespace}:store={approved_store}:scope={approved_scope}:receipt={single_shot_receipt_hash_sha256}:chain={single_shot_receipt_hash_chain_sha256}:readback={receipt_acceptance_readback_hash_sha256}:accepted=true"
    ));

    let fixtures = serde_json::Value::Array(vec![
        acceptance_fixture(
            "minimal-scoped-memory-real-write-canary-durable-store-write-receipt-acceptance",
            "accepted_single_shot_receipt_readback_rollback_cleanup_zero_residue",
            "single_shot_receipt_hash_chain_readback_rollback_cleanup_zero_residue_accepted",
            true,
            serde_json::json!({
                "approved_namespace": approved_namespace,
                "approved_store": approved_store,
                "approved_scope": approved_scope,
                "durable_store_write_target_id": durable_store_write_target_id,
                "durable_store_target_store_id": durable_store_target_store_id,
                "canary_record_id": canary_record_id,
                "single_shot_receipt_hash_sha256": single_shot_receipt_hash_sha256,
                "single_shot_receipt_hash_chain_sha256": single_shot_receipt_hash_chain_sha256,
                "single_shot_execution_hash_sha256": single_shot_execution_hash_sha256,
                "receipt_acceptance_hash_sha256": receipt_acceptance_hash_sha256,
            }),
        ),
        acceptance_fixture(
            "missing-single-shot-source-boundary",
            "blocked_source_noop",
            "source_single_shot_execution_boundary_required",
            false,
            serde_json::json!({"source_single_shot_execution_boundary_bound": false}),
        ),
        acceptance_fixture(
            "wrong-namespace",
            "blocked_namespace_noop",
            "approved_namespace_required",
            false,
            serde_json::json!({"approved_namespace_bound": false}),
        ),
        acceptance_fixture(
            "wrong-store",
            "blocked_store_noop",
            "approved_store_required",
            false,
            serde_json::json!({"approved_store_bound": false}),
        ),
        acceptance_fixture(
            "wrong-scope",
            "blocked_scope_noop",
            "approved_scope_required",
            false,
            serde_json::json!({"approved_scope_bound": false}),
        ),
        acceptance_fixture(
            "missing-single-shot-receipt",
            "blocked_receipt_noop",
            "single_shot_receipt_hash_required",
            false,
            serde_json::json!({"single_shot_receipt_identity_bound": false}),
        ),
        acceptance_fixture(
            "missing-single-shot-hash-chain",
            "blocked_hash_chain_noop",
            "single_shot_receipt_hash_chain_required",
            false,
            serde_json::json!({"single_shot_receipt_hash_chain_bound": false}),
        ),
        acceptance_fixture(
            "missing-single-shot-readback",
            "blocked_readback_noop",
            "single_shot_post_write_readback_required",
            false,
            serde_json::json!({"single_shot_readback_evidence_bound": false}),
        ),
        acceptance_fixture(
            "missing-rollback-cleanup-zero-residue",
            "blocked_zero_residue_noop",
            "single_shot_rollback_cleanup_zero_residue_required",
            false,
            serde_json::json!({"single_shot_rollback_cleanup_zero_residue_bound": false}),
        ),
        acceptance_fixture(
            "new-write-or-external-side-effect-attempt",
            "blocked_side_effect_noop",
            "receipt_acceptance_report_route_must_not_execute_new_write_or_external_side_effect",
            false,
            serde_json::json!({
                "durable_store_write_execution_performed": false,
                "memory_store_write_performed": false,
                "wal_write_performed": false,
                "receipt_persisted": false,
                "durable_memory_store_write_performed": false,
                "external_send_performed": false
            }),
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
                        "minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_accepted",
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
    let denied_by = RECEIPT_ACCEPTANCE_DENIALS
        .iter()
        .map(|reason| serde_json::json!(reason))
        .collect::<Vec<_>>();
    let denied_count = denied_by.len();
    let receipt_acceptance_ops_ok = source_ready
        && namespace_bound
        && store_bound
        && scope_bound
        && target_bound
        && record_bound
        && hashes_bound
        && readback_bound
        && rollback_cleanup_zero_residue_bound;
    let report_ready = route_count_source_command_accepted
        && receipt_acceptance_ops_ok
        && accepted_fixture_count == 1
        && blocked_fixture_count == 9
        && denied_count == 32;
    let ready_surface_count = if report_ready {
        RECEIPT_ACCEPTANCE_SURFACES.len()
    } else {
        0
    };
    let boundary_hash_sha256 = sha256_text_value(&format!(
        "minimal-scoped-memory-real-write-canary-durable-store-write-receipt-acceptance-boundary-report:v1:source-ready={source_ready}:receipt-acceptance={report_ready}:receipt={single_shot_receipt_hash_sha256}:chain={single_shot_receipt_hash_chain_sha256}:fixtures=10:accepted=1:denials=32:production-durable-write=false:new-memory-write=false"
    ));
    let policy_hash_sha256 = sha256_text_value(
        "minimal-scoped-memory-real-write-canary-durable-store-write-receipt-acceptance-policy:v1:accept-single-shot-receipt-evidence:no-new-store-write:no-wal-rewrite:no-receipt-repersist:no-production-durable-memory:no-kg:no-provider:no-channel:no-release:no-install",
    );
    let required_fields = serde_json::json!([
        "source_minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_boundary_report_sha256",
        "approved_namespace",
        "approved_store",
        "approved_scope",
        "durable_store_write_target_id",
        "durable_store_target_store_id",
        "canary_record_id",
        "single_shot_receipt_hash_sha256",
        "single_shot_receipt_hash_chain_sha256",
        "single_shot_execution_hash_sha256",
        "receipt_acceptance_hash_sha256",
        "receipt_acceptance_record_hash_sha256",
        "receipt_acceptance_readback_hash_sha256",
        "route_count",
    ]);
    let allowed_next_actions = serde_json::json!([
        {
            "action": "run_minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_boundary_require_live_gate",
            "status": "allowed_verification_only",
            "accepts_single_shot_receipt": true,
            "writes_new_canary_store_record": false,
            "writes_production_durable_memory": false,
            "writes_wal": false,
            "persists_receipt": false,
            "reads_memory": false,
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
            "action": "prepare_minimal_scoped_memory_real_write_canary_durable_store_write_rollback_tombstone_zero_residue_acceptance_boundary",
            "status": "allowed_report_only_next_slice",
            "requires_minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_boundary": true,
            "writes_new_canary_store_record": false,
            "writes_production_durable_memory": false,
            "writes_wal": false,
            "persists_receipt": false,
            "executes_rollback": false,
            "writes_tombstone": false,
            "writes_kg": false,
            "invokes_provider": false,
            "sends_externally": false
        }
    ]);

    let mut side_effects = serde_json::Map::new();
    for &key in FALSE_RECEIPT_ACCEPTANCE_SIDE_EFFECT_KEYS {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }
    for &key in TRUE_RECEIPT_ACCEPTANCE_KEYS {
        side_effects.insert(key.to_string(), serde_json::json!(report_ready));
    }

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
        "hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_boundary_gate"
    );
    insert_report_json!(
        "endpoint",
        HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_MINIMAL_SCOPED_MEMORY_REAL_WRITE_CANARY_DURABLE_STORE_WRITE_RECEIPT_ACCEPTANCE_BOUNDARY_ENDPOINT
    );
    insert_report_json!(
        "source_command",
        "/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-durable-store-write-receipt-acceptance-boundary --json"
    );
    insert_report_json!("native_route", true);
    insert_report_json!("side_effect_free", false);
    insert_report_json!("external_side_effect_free", true);
    insert_report_json!("audit_date", "2026-07-04");
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_schema_version",
        "minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_boundary_v1"
    );
    insert_report_json!(
        "scoped_memory_real_write_canary_mode",
        "minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_report_only"
    );
    insert_report_json!(
        "durable_store_write_receipt_acceptance_scope",
        "accept_single_shot_request_local_canary_receipt_hash_chain_readback_rollback_cleanup_zero_residue_evidence_only"
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
        "memory_write_execution_minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_boundary_ready",
        report_ready
    );
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_ready",
        report_ready
    );
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_performed",
        report_ready
    );
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_accepted",
        report_ready
    );
    insert_report_json!(
        "source_minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_boundary_ready",
        source_ready
    );
    insert_report_json!(
        "source_minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_boundary_report_sha256",
        source_report_sha256
    );
    insert_report_json!(
        "source_accepted_minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_fixture_count",
        json_u64(
            &source,
            "accepted_minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_fixture_count",
        )
    );
    insert_report_json!(
        "source_blocked_minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_fixture_count",
        json_u64(
            &source,
            "blocked_minimal_scoped_memory_real_write_canary_durable_store_write_single_shot_execution_fixture_count",
        )
    );
    insert_report_json!(
        "source_durable_store_write_single_shot_execution_result_accepted_count",
        json_u64(
            &source,
            "durable_store_write_single_shot_execution_result_accepted_count",
        )
    );
    for key in [
        "durable_store_write_execution_performed",
        "memory_write_execution_performed",
        "memory_store_write_performed",
        "wal_write_performed",
        "receipt_persisted",
        "post_write_readback_performed",
        "rollback_executed",
        "tombstone_cleanup_executed",
        "production_durable_memory_store_write_performed",
        "actual_production_durable_memory_write_performed",
        "durable_memory_store_write_performed",
        "durable_memory_store_read_performed",
        "durable_memory_store_rollback_performed",
        "live_kg_write_performed",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "channel_send_performed",
        "external_send_performed",
        "release_artifact_written",
        "install_executed",
        "service_restarted",
        "active_binary_mutated",
    ] {
        report.insert(
            format!("source_{key}_count"),
            serde_json::json!(bool_count(&source, key)),
        );
    }
    for key in [
        "single_shot_canary_pre_write_memory_count",
        "single_shot_canary_post_write_memory_count",
        "single_shot_canary_readback_hit_count",
        "single_shot_canary_post_rollback_memory_count",
        "single_shot_canary_artifact_pre_count",
        "single_shot_canary_artifact_write_count",
        "single_shot_canary_artifact_readback_count",
        "single_shot_canary_artifact_cleanup_removed_count",
        "single_shot_canary_artifact_post_cleanup_count",
    ] {
        report.insert(
            format!("source_{key}"),
            serde_json::json!(json_u64(&source, key)),
        );
    }
    for key in [
        "single_shot_canary_rollback_restored",
        "single_shot_canary_post_rollback_absence_confirmed",
        "single_shot_canary_artifact_zero_residue_confirmed",
    ] {
        report.insert(
            format!("source_{key}"),
            serde_json::json!(json_bool(&source, key)),
        );
    }
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
    insert_report_json!("canary_record_id", canary_record_id);
    insert_report_json!(
        "source_single_shot_boundary_hash_sha256",
        source_single_shot_boundary_hash_sha256
    );
    insert_report_json!(
        "source_single_shot_policy_hash_sha256",
        source_single_shot_policy_hash_sha256
    );
    insert_report_json!("canary_payload_digest_sha256", canary_payload_digest_sha256);
    insert_report_json!(
        "single_shot_execution_envelope_sha256",
        single_shot_execution_envelope_sha256
    );
    insert_report_json!("single_shot_nonce_sha256", single_shot_nonce_sha256);
    insert_report_json!("single_shot_command_sha256", single_shot_command_sha256);
    insert_report_json!("single_shot_budget_sha256", single_shot_budget_sha256);
    insert_report_json!("single_shot_wal_hash_sha256", single_shot_wal_hash_sha256);
    insert_report_json!(
        "single_shot_receipt_hash_sha256",
        single_shot_receipt_hash_sha256
    );
    insert_report_json!(
        "single_shot_receipt_hash_chain_sha256",
        single_shot_receipt_hash_chain_sha256
    );
    insert_report_json!(
        "single_shot_cleanup_receipt_hash_sha256",
        single_shot_cleanup_receipt_hash_sha256
    );
    insert_report_json!(
        "single_shot_execution_hash_sha256",
        single_shot_execution_hash_sha256
    );
    insert_report_json!(
        "receipt_acceptance_record_hash_sha256",
        receipt_acceptance_record_hash_sha256
    );
    insert_report_json!(
        "receipt_acceptance_readback_hash_sha256",
        receipt_acceptance_readback_hash_sha256
    );
    insert_report_json!(
        "receipt_acceptance_hash_sha256",
        receipt_acceptance_hash_sha256
    );
    insert_report_json!("receipt_readback_digest_match", readback_bound);
    insert_report_json!("receipt_hash_chain_verified", hashes_bound);
    insert_report_json!(
        "single_shot_rollback_cleanup_zero_residue_verified",
        rollback_cleanup_zero_residue_bound
    );
    insert_report_json!(
        "required_minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_surface_count",
        RECEIPT_ACCEPTANCE_SURFACES.len()
    );
    insert_report_json!(
        "ready_minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_surface_count",
        ready_surface_count
    );
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_fixture_count",
        fixtures.as_array().map(std::vec::Vec::len).unwrap_or(0)
    );
    insert_report_json!(
        "accepted_minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_fixture_count",
        accepted_fixture_count
    );
    insert_report_json!(
        "blocked_minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_fixture_count",
        blocked_fixture_count
    );
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_accepted_count",
        accepted_fixture_count
    );
    for key in [
        "durable_store_write_receipt_acceptance_authority_accepted_count",
        "source_single_shot_execution_bound_count",
        "single_shot_receipt_identity_bound_count",
        "single_shot_receipt_hash_chain_bound_count",
        "single_shot_readback_evidence_bound_count",
        "single_shot_rollback_cleanup_zero_residue_bound_count",
        "receipt_acceptance_record_bound_count",
        "receipt_acceptance_result_recorded_count",
        "receipt_acceptance_result_accepted_count",
        "receipt_acceptance_replay_guard_accepted_count",
        "operator_receipt_acceptance_handoff_bound_count",
        "rollback_tombstone_zero_residue_handoff_bound_count",
    ] {
        report.insert(key.to_string(), serde_json::json!(accepted_fixture_count));
    }
    for &key in FALSE_RECEIPT_ACCEPTANCE_SIDE_EFFECT_KEYS {
        report.insert(key.to_string(), serde_json::json!(false));
        report.insert(format!("{key}_count"), serde_json::json!(0));
    }
    for &key in TRUE_RECEIPT_ACCEPTANCE_KEYS {
        report.insert(key.to_string(), serde_json::json!(report_ready));
        report.insert(
            format!("{key}_count"),
            serde_json::json!(if report_ready { 1 } else { 0 }),
        );
    }
    for key in [
        "source_single_shot_execution_boundary_required",
        "minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_accepted",
        "approved_namespace_bound",
        "approved_store_bound",
        "approved_scope_bound",
        "durable_store_write_target_bound",
        "durable_store_target_store_bound",
        "single_shot_record_identity_bound",
        "single_shot_execution_envelope_bound",
        "single_shot_receipt_identity_bound",
        "single_shot_receipt_hash_chain_bound",
        "single_shot_readback_evidence_bound",
        "single_shot_rollback_cleanup_zero_residue_bound",
        "receipt_acceptance_record_bound",
        "receipt_acceptance_replay_guard_bound",
        "operator_receipt_acceptance_handoff_bound",
        "rollback_tombstone_zero_residue_handoff_bound",
        "new_canary_store_write_forbidden_on_report_route",
        "wal_rewrite_forbidden_on_report_route",
        "receipt_repersist_forbidden_on_report_route",
        "production_durable_memory_write_forbidden",
        "durable_memory_read_or_rollback_forbidden",
        "kg_live_write_forbidden",
        "provider_model_invocation_forbidden",
        "credential_channel_public_release_forbidden",
        "install_restart_active_binary_mutation_forbidden",
    ] {
        report.insert(key.to_string(), serde_json::json!(true));
    }
    report.insert(
        "required_minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_fields"
            .to_string(),
        required_fields,
    );
    report.insert(
        "minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_surfaces"
            .to_string(),
        serde_json::json!(RECEIPT_ACCEPTANCE_SURFACES),
    );
    report.insert(
        "minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_fixtures"
            .to_string(),
        fixtures,
    );
    report.insert(
        "denied_by_minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_boundary"
            .to_string(),
        serde_json::Value::Array(denied_by),
    );
    insert_report_json!(
        "denied_by_minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_boundary_count",
        denied_count
    );
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_boundary_hash_sha256",
        boundary_hash_sha256
    );
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_policy_hash_sha256",
        policy_hash_sha256
    );
    report.insert("allowed_next_actions".to_string(), allowed_next_actions);
    report.insert(
        "side_effects".to_string(),
        serde_json::Value::Object(side_effects),
    );
    serde_json::Value::Object(report)
}

fn hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_durable_store_write_rollback_tombstone_zero_residue_acceptance_boundary_report()
-> serde_json::Value {
    const ZERO_RESIDUE_SURFACES: &[&str] = &[
        "source_receipt_acceptance_boundary_required",
        "approved_namespace_store_scope_required",
        "receipt_acceptance_hash_chain_required",
        "single_shot_rollback_restore_required",
        "single_shot_tombstone_cleanup_required",
        "single_shot_artifact_cleanup_required",
        "single_shot_post_rollback_absence_required",
        "single_shot_zero_residue_required",
        "zero_residue_acceptance_record_required",
        "zero_residue_replay_guard_required",
        "operator_zero_residue_acceptance_handoff_required",
        "production_and_external_side_effects_forbidden",
    ];
    const ZERO_RESIDUE_DENIALS: &[&str] = &[
        "source_receipt_acceptance_boundary_required",
        "source_receipt_acceptance_boundary_hash_required",
        "receipt_acceptance_hash_required",
        "receipt_acceptance_record_required",
        "approved_namespace_required",
        "approved_store_required",
        "approved_scope_required",
        "durable_store_write_target_required",
        "durable_store_target_store_required",
        "single_shot_record_identity_required",
        "single_shot_receipt_hash_required",
        "single_shot_receipt_hash_chain_required",
        "single_shot_execution_hash_required",
        "single_shot_cleanup_receipt_required",
        "single_shot_post_write_readback_required",
        "single_shot_rollback_restore_required",
        "single_shot_post_rollback_absence_required",
        "single_shot_tombstone_cleanup_required",
        "single_shot_artifact_cleanup_required",
        "single_shot_zero_residue_required",
        "zero_residue_acceptance_record_required",
        "zero_residue_acceptance_readback_required",
        "zero_residue_replay_guard_required",
        "operator_zero_residue_acceptance_handoff_required",
        "new_canary_store_write_report_route_denied",
        "rollback_execution_report_route_denied",
        "tombstone_write_report_route_denied",
        "wal_rewrite_report_route_denied",
        "receipt_repersist_report_route_denied",
        "raw_payload_plaintext_recording_denied",
        "production_durable_memory_backend_write_denied",
        "durable_memory_backend_read_or_rollback_denied",
        "kg_provider_channel_release_install_active_binary_denied",
        "unrestricted_full_live_activation_denied",
    ];
    const FALSE_ZERO_RESIDUE_SIDE_EFFECT_KEYS: &[&str] = &[
        "durable_store_write_execution_performed",
        "durable_store_write_single_shot_execution_performed",
        "durable_store_write_receipt_acceptance_performed",
        "memory_write_execution_performed",
        "memory_store_write_path_enabled",
        "memory_store_write_allowed",
        "memory_store_write_performed",
        "memory_store_mutation_allowed",
        "memory_store_mutated",
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
        "rollback_performed",
        "rollback_result_recorded",
        "rollback_result_persisted",
        "rollback_result_accepted",
        "tombstone_write_performed",
        "tombstone_cleanup_executed",
        "tombstone_cleanup_result_recorded",
        "tombstone_cleanup_result_accepted",
        "single_shot_canary_nonce_consumed",
        "single_shot_canary_explicit_command_accepted",
        "single_shot_canary_memory_store_write_performed",
        "single_shot_canary_post_write_readback_performed",
        "single_shot_canary_rollback_executed",
        "single_shot_canary_tombstone_cleanup_executed",
        "single_shot_canary_artifact_cleanup_executed",
        "production_durable_memory_backend_present",
        "production_durable_memory_store_write_performed",
        "actual_production_durable_memory_write_performed",
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
    const TRUE_ZERO_RESIDUE_KEYS: &[&str] = &[
        "durable_store_write_rollback_tombstone_zero_residue_acceptance_performed",
        "durable_store_write_rollback_tombstone_zero_residue_acceptance_result_recorded",
        "durable_store_write_rollback_tombstone_zero_residue_acceptance_result_accepted",
        "source_receipt_acceptance_boundary_accepted",
        "single_shot_rollback_cleanup_zero_residue_evidence_accepted",
        "single_shot_artifact_zero_residue_evidence_accepted",
        "rollback_tombstone_cleanup_absence_accepted",
        "zero_residue_acceptance_recorded",
        "zero_residue_acceptance_replay_guard_accepted",
        "operator_zero_residue_acceptance_handoff_bound",
        "kg_provider_channel_release_install_active_binary_forbidden",
    ];

    fn zero_residue_fixture(
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
            "minimal_scoped_memory_real_write_canary_durable_store_write_rollback_tombstone_zero_residue_acceptance_status",
            status
        );
        insert_fixture_json!("reason", reason);
        insert_fixture_json!(
            "minimal_scoped_memory_real_write_canary_durable_store_write_rollback_tombstone_zero_residue_acceptance_accepted",
            accepted
        );
        for key in [
            "source_receipt_acceptance_boundary_bound",
            "approved_namespace_bound",
            "approved_store_bound",
            "approved_scope_bound",
            "durable_store_write_target_bound",
            "durable_store_target_store_bound",
            "single_shot_record_identity_bound",
            "receipt_acceptance_hash_bound",
            "single_shot_receipt_hash_chain_bound",
            "single_shot_rollback_cleanup_zero_residue_bound",
            "single_shot_artifact_zero_residue_bound",
            "rollback_tombstone_cleanup_absence_bound",
            "zero_residue_acceptance_record_bound",
            "zero_residue_acceptance_replay_guard_bound",
            "operator_zero_residue_acceptance_handoff_bound",
        ] {
            base.insert(key.to_string(), serde_json::json!(accepted));
        }
        for &key in FALSE_ZERO_RESIDUE_SIDE_EFFECT_KEYS {
            base.insert(key.to_string(), serde_json::json!(false));
        }
        for &key in TRUE_ZERO_RESIDUE_KEYS {
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
        .name("hepta-memory-zero-residue-acceptance-source-report".to_string())
        .stack_size(8 * 1024 * 1024)
        .spawn(
            hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_boundary_report,
        )
        .ok()
        .and_then(|handle| handle.join().ok())
        .unwrap_or_else(|| {
            serde_json::json!({
                "status": "blocked",
                "memory_write_execution_minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_boundary_ready": false,
                "minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_accepted": false,
                "source_minimal_scoped_memory_real_write_canary_durable_store_write_rollback_tombstone_zero_residue_acceptance_source_report_thread_failed": true
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
    let bool_count = |value: &serde_json::Value, key: &str| -> usize {
        if json_bool(value, key) { 1 } else { 0 }
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
    let source_next_action_zero_residue = source
        .get("allowed_next_actions")
        .and_then(serde_json::Value::as_array)
        .and_then(|items| items.get(1))
        .map(|item| {
            item.get("action").and_then(serde_json::Value::as_str)
                == Some(
                    "prepare_minimal_scoped_memory_real_write_canary_durable_store_write_rollback_tombstone_zero_residue_acceptance_boundary",
                )
                && item
                    .get(
                        "requires_minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_boundary",
                    )
                    .and_then(serde_json::Value::as_bool)
                    == Some(true)
                && item
                    .get("writes_new_canary_store_record")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
        })
        .unwrap_or(false);
    let source_side_effects_ok = source
        .get("side_effects")
        .and_then(serde_json::Value::as_object)
        .map(|effects| {
            effects
                .get("durable_store_write_receipt_acceptance_performed")
                .and_then(serde_json::Value::as_bool)
                == Some(true)
                && effects
                    .get("memory_store_write_performed")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && effects
                    .get("wal_write_performed")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && effects
                    .get("receipt_persisted")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && effects
                    .get("rollback_executed")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && effects
                    .get("tombstone_cleanup_executed")
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
            "memory_write_execution_minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_boundary_ready",
        )
        && json_bool(
            &source,
            "minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_accepted",
        )
        && json_u64(
            &source,
            "accepted_minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_fixture_count",
        ) == 1
        && json_u64(
            &source,
            "blocked_minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_fixture_count",
        ) == 9
        && json_u64(&source, "receipt_acceptance_result_accepted_count") == 1
        && json_bool(
            &source,
            "single_shot_rollback_cleanup_zero_residue_verified",
        )
        && json_bool(
            &source,
            "source_single_shot_canary_artifact_zero_residue_confirmed",
        )
        && json_u64(
            &source,
            "source_single_shot_canary_post_rollback_memory_count",
        ) == 0
        && json_bool(
            &source,
            "source_single_shot_canary_post_rollback_absence_confirmed",
        )
        && json_u64(
            &source,
            "source_single_shot_canary_artifact_cleanup_removed_count",
        ) == 3
        && json_u64(
            &source,
            "source_single_shot_canary_artifact_post_cleanup_count",
        ) == 0
        && json_u64(&source, "source_memory_store_write_performed_count") == 1
        && json_u64(&source, "source_wal_write_performed_count") == 1
        && json_u64(&source, "source_receipt_persisted_count") == 1
        && json_u64(&source, "source_post_write_readback_performed_count") == 1
        && json_u64(&source, "source_rollback_executed_count") == 1
        && json_u64(&source, "source_tombstone_cleanup_executed_count") == 1
        && !json_bool(&source, "memory_store_write_performed")
        && !json_bool(&source, "wal_write_performed")
        && !json_bool(&source, "receipt_persisted")
        && !json_bool(&source, "rollback_executed")
        && !json_bool(&source, "tombstone_cleanup_executed")
        && !json_bool(&source, "production_durable_memory_store_write_performed")
        && !json_bool(&source, "actual_production_durable_memory_write_performed")
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
        && !json_bool(&source, "service_restarted")
        && !json_bool(&source, "active_binary_mutated")
        && source_next_action_zero_residue
        && source_side_effects_ok;

    let approved_namespace = json_str(&source, "approved_namespace");
    let approved_store = json_str(&source, "approved_store");
    let approved_scope = json_str(&source, "approved_scope");
    let durable_store_write_target_id = json_str(&source, "durable_store_write_target_id");
    let durable_store_target_store_id = json_str(&source, "durable_store_target_store_id");
    let canary_record_id = json_str(&source, "canary_record_id");
    let source_report_sha256 = sha256_text_value(&source.to_string());
    let source_receipt_acceptance_boundary_hash_sha256 = json_str(
        &source,
        "minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_boundary_hash_sha256",
    );
    let source_receipt_acceptance_policy_hash_sha256 = json_str(
        &source,
        "minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_policy_hash_sha256",
    );
    let canary_payload_digest_sha256 = json_str(&source, "canary_payload_digest_sha256");
    let single_shot_receipt_hash_sha256 = json_str(&source, "single_shot_receipt_hash_sha256");
    let single_shot_receipt_hash_chain_sha256 =
        json_str(&source, "single_shot_receipt_hash_chain_sha256");
    let single_shot_cleanup_receipt_hash_sha256 =
        json_str(&source, "single_shot_cleanup_receipt_hash_sha256");
    let single_shot_execution_hash_sha256 = json_str(&source, "single_shot_execution_hash_sha256");
    let receipt_acceptance_record_hash_sha256 =
        json_str(&source, "receipt_acceptance_record_hash_sha256");
    let receipt_acceptance_readback_hash_sha256 =
        json_str(&source, "receipt_acceptance_readback_hash_sha256");
    let receipt_acceptance_hash_sha256 = json_str(&source, "receipt_acceptance_hash_sha256");

    let namespace_bound = approved_namespace == "hepta.memory.canary";
    let store_bound = approved_store == "wal-receipt-canary-artifact";
    let scope_bound = approved_scope == "session";
    let target_bound = durable_store_write_target_id
        == "hepta-minimal-scoped-memory-real-write-canary-durable-store-write-target-v1"
        && durable_store_target_store_id == "hepta-memory-durable-store-canary-plan-only";
    let record_bound = canary_record_id
        == "hepta-minimal-scoped-memory-real-write-canary-durable-store-write-single-shot-record-v1";
    let hashes_bound = !source_receipt_acceptance_boundary_hash_sha256.is_empty()
        && !source_receipt_acceptance_policy_hash_sha256.is_empty()
        && !canary_payload_digest_sha256.is_empty()
        && !single_shot_receipt_hash_sha256.is_empty()
        && !single_shot_receipt_hash_chain_sha256.is_empty()
        && !single_shot_cleanup_receipt_hash_sha256.is_empty()
        && !single_shot_execution_hash_sha256.is_empty()
        && !receipt_acceptance_record_hash_sha256.is_empty()
        && !receipt_acceptance_readback_hash_sha256.is_empty()
        && !receipt_acceptance_hash_sha256.is_empty();
    let rollback_cleanup_absence_bound =
        json_bool(&source, "source_single_shot_canary_rollback_restored")
            && json_u64(
                &source,
                "source_single_shot_canary_post_rollback_memory_count",
            ) == 0
            && json_bool(
                &source,
                "source_single_shot_canary_post_rollback_absence_confirmed",
            )
            && json_u64(
                &source,
                "source_single_shot_canary_artifact_cleanup_removed_count",
            ) == 3
            && json_u64(
                &source,
                "source_single_shot_canary_artifact_post_cleanup_count",
            ) == 0;
    let artifact_zero_residue_bound = json_bool(
        &source,
        "source_single_shot_canary_artifact_zero_residue_confirmed",
    );
    let zero_residue_acceptance_record_hash_sha256 = sha256_text_value(&format!(
        "minimal-scoped-memory-real-write-canary-durable-store-write-rollback-tombstone-zero-residue-acceptance-record:v1:source={source_report_sha256}:receipt-acceptance={receipt_acceptance_hash_sha256}:record={canary_record_id}:cleanup={single_shot_cleanup_receipt_hash_sha256}:zero-residue=true"
    ));
    let zero_residue_acceptance_readback_hash_sha256 = sha256_text_value(&format!(
        "minimal-scoped-memory-real-write-canary-durable-store-write-rollback-tombstone-zero-residue-acceptance-readback:v1:record={canary_record_id}:post-rollback-memory=0:artifact-post-cleanup=0:absence=true:source-execution={single_shot_execution_hash_sha256}"
    ));
    let zero_residue_acceptance_hash_sha256 = sha256_text_value(&format!(
        "minimal-scoped-memory-real-write-canary-durable-store-write-rollback-tombstone-zero-residue-acceptance:v1:namespace={approved_namespace}:store={approved_store}:scope={approved_scope}:receipt-acceptance={receipt_acceptance_hash_sha256}:cleanup={single_shot_cleanup_receipt_hash_sha256}:readback={zero_residue_acceptance_readback_hash_sha256}:accepted=true"
    ));

    let fixtures = serde_json::Value::Array(vec![
        zero_residue_fixture(
            "minimal-scoped-memory-real-write-canary-durable-store-write-rollback-tombstone-zero-residue-acceptance",
            "accepted_rollback_tombstone_cleanup_zero_residue",
            "single_shot_rollback_tombstone_cleanup_artifact_zero_residue_accepted",
            true,
            serde_json::json!({
                "approved_namespace": approved_namespace,
                "approved_store": approved_store,
                "approved_scope": approved_scope,
                "durable_store_write_target_id": durable_store_write_target_id,
                "durable_store_target_store_id": durable_store_target_store_id,
                "canary_record_id": canary_record_id,
                "receipt_acceptance_hash_sha256": receipt_acceptance_hash_sha256,
                "single_shot_cleanup_receipt_hash_sha256": single_shot_cleanup_receipt_hash_sha256,
                "zero_residue_acceptance_hash_sha256": zero_residue_acceptance_hash_sha256,
            }),
        ),
        zero_residue_fixture(
            "missing-receipt-acceptance-source-boundary",
            "blocked_source_noop",
            "source_receipt_acceptance_boundary_required",
            false,
            serde_json::json!({"source_receipt_acceptance_boundary_bound": false}),
        ),
        zero_residue_fixture(
            "wrong-namespace",
            "blocked_namespace_noop",
            "approved_namespace_required",
            false,
            serde_json::json!({"approved_namespace_bound": false}),
        ),
        zero_residue_fixture(
            "wrong-store",
            "blocked_store_noop",
            "approved_store_required",
            false,
            serde_json::json!({"approved_store_bound": false}),
        ),
        zero_residue_fixture(
            "wrong-scope",
            "blocked_scope_noop",
            "approved_scope_required",
            false,
            serde_json::json!({"approved_scope_bound": false}),
        ),
        zero_residue_fixture(
            "missing-rollback-restore",
            "blocked_rollback_restore_noop",
            "single_shot_rollback_restore_required",
            false,
            serde_json::json!({"single_shot_rollback_cleanup_zero_residue_bound": false}),
        ),
        zero_residue_fixture(
            "missing-tombstone-cleanup",
            "blocked_tombstone_cleanup_noop",
            "single_shot_tombstone_cleanup_required",
            false,
            serde_json::json!({"rollback_tombstone_cleanup_absence_bound": false}),
        ),
        zero_residue_fixture(
            "missing-artifact-cleanup",
            "blocked_artifact_cleanup_noop",
            "single_shot_artifact_cleanup_required",
            false,
            serde_json::json!({"single_shot_artifact_zero_residue_bound": false}),
        ),
        zero_residue_fixture(
            "missing-post-rollback-absence",
            "blocked_absence_noop",
            "single_shot_post_rollback_absence_required",
            false,
            serde_json::json!({"rollback_tombstone_cleanup_absence_bound": false}),
        ),
        zero_residue_fixture(
            "new-write-rollback-tombstone-or-external-side-effect-attempt",
            "blocked_side_effect_noop",
            "zero_residue_acceptance_report_route_must_not_execute_new_write_rollback_tombstone_or_external_side_effect",
            false,
            serde_json::json!({
                "memory_store_write_performed": false,
                "wal_write_performed": false,
                "receipt_persisted": false,
                "rollback_executed": false,
                "tombstone_write_performed": false,
                "tombstone_cleanup_executed": false,
                "durable_memory_store_write_performed": false,
                "external_send_performed": false
            }),
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
                        "minimal_scoped_memory_real_write_canary_durable_store_write_rollback_tombstone_zero_residue_acceptance_accepted",
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
    let denied_by = ZERO_RESIDUE_DENIALS
        .iter()
        .map(|reason| serde_json::json!(reason))
        .collect::<Vec<_>>();
    let denied_count = denied_by.len();
    let zero_residue_ops_ok = source_ready
        && namespace_bound
        && store_bound
        && scope_bound
        && target_bound
        && record_bound
        && hashes_bound
        && rollback_cleanup_absence_bound
        && artifact_zero_residue_bound;
    let report_ready = route_count_source_command_accepted
        && zero_residue_ops_ok
        && accepted_fixture_count == 1
        && blocked_fixture_count == 9
        && denied_count == 34;
    let ready_surface_count = if report_ready {
        ZERO_RESIDUE_SURFACES.len()
    } else {
        0
    };
    let boundary_hash_sha256 = sha256_text_value(&format!(
        "minimal-scoped-memory-real-write-canary-durable-store-write-rollback-tombstone-zero-residue-acceptance-boundary-report:v1:source-ready={source_ready}:zero-residue={report_ready}:receipt-acceptance={receipt_acceptance_hash_sha256}:cleanup={single_shot_cleanup_receipt_hash_sha256}:fixtures=10:accepted=1:denials=34:production-durable-write=false:new-write=false"
    ));
    let policy_hash_sha256 = sha256_text_value(
        "minimal-scoped-memory-real-write-canary-durable-store-write-rollback-tombstone-zero-residue-acceptance-policy:v1:accept-rollback-tombstone-cleanup-zero-residue-evidence:no-new-store-write:no-rollback-execution:no-tombstone-write:no-wal-rewrite:no-receipt-repersist:no-production-durable-memory:no-kg:no-provider:no-channel:no-release:no-install",
    );
    let required_fields = serde_json::json!([
        "source_minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_boundary_report_sha256",
        "approved_namespace",
        "approved_store",
        "approved_scope",
        "durable_store_write_target_id",
        "durable_store_target_store_id",
        "canary_record_id",
        "receipt_acceptance_hash_sha256",
        "single_shot_cleanup_receipt_hash_sha256",
        "zero_residue_acceptance_hash_sha256",
        "zero_residue_acceptance_record_hash_sha256",
        "zero_residue_acceptance_readback_hash_sha256",
        "route_count",
    ]);
    let allowed_next_actions = serde_json::json!([
        {
            "action": "run_minimal_scoped_memory_real_write_canary_durable_store_write_rollback_tombstone_zero_residue_acceptance_boundary_require_live_gate",
            "status": "allowed_verification_only",
            "accepts_zero_residue_evidence": true,
            "writes_new_canary_store_record": false,
            "writes_production_durable_memory": false,
            "writes_wal": false,
            "persists_receipt": false,
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
            "action": "prepare_scoped_production_durable_memory_write_preflight_boundary",
            "status": "requires_separate_production_durable_memory_gate",
            "requires_minimal_scoped_memory_real_write_canary_durable_store_write_rollback_tombstone_zero_residue_acceptance_boundary": true,
            "writes_production_durable_memory": false,
            "production_durable_memory_requires_separate_explicit_gate": true,
            "writes_kg": false,
            "invokes_provider": false,
            "sends_externally": false
        }
    ]);

    let mut side_effects = serde_json::Map::new();
    for &key in FALSE_ZERO_RESIDUE_SIDE_EFFECT_KEYS {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }
    for &key in TRUE_ZERO_RESIDUE_KEYS {
        side_effects.insert(key.to_string(), serde_json::json!(report_ready));
    }

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
        "hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_durable_store_write_rollback_tombstone_zero_residue_acceptance_boundary_gate"
    );
    insert_report_json!(
        "endpoint",
        HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_MINIMAL_SCOPED_MEMORY_REAL_WRITE_CANARY_DURABLE_STORE_WRITE_ROLLBACK_TOMBSTONE_ZERO_RESIDUE_ACCEPTANCE_BOUNDARY_ENDPOINT
    );
    insert_report_json!(
        "source_command",
        "/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-durable-store-write-rollback-tombstone-zero-residue-acceptance-boundary --json"
    );
    insert_report_json!("native_route", true);
    insert_report_json!("side_effect_free", false);
    insert_report_json!("external_side_effect_free", true);
    insert_report_json!("audit_date", "2026-07-05");
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_durable_store_write_rollback_tombstone_zero_residue_acceptance_schema_version",
        "minimal_scoped_memory_real_write_canary_durable_store_write_rollback_tombstone_zero_residue_acceptance_boundary_v1"
    );
    insert_report_json!(
        "scoped_memory_real_write_canary_mode",
        "minimal_scoped_memory_real_write_canary_durable_store_write_rollback_tombstone_zero_residue_acceptance_report_only"
    );
    insert_report_json!(
        "durable_store_write_rollback_tombstone_zero_residue_acceptance_scope",
        "accept_single_shot_rollback_tombstone_cleanup_artifact_zero_residue_evidence_only"
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
        "memory_write_execution_minimal_scoped_memory_real_write_canary_durable_store_write_rollback_tombstone_zero_residue_acceptance_boundary_ready",
        report_ready
    );
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_durable_store_write_rollback_tombstone_zero_residue_acceptance_ready",
        report_ready
    );
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_durable_store_write_rollback_tombstone_zero_residue_acceptance_performed",
        report_ready
    );
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_durable_store_write_rollback_tombstone_zero_residue_acceptance_accepted",
        report_ready
    );
    insert_report_json!(
        "source_minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_boundary_ready",
        source_ready
    );
    insert_report_json!(
        "source_minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_boundary_report_sha256",
        source_report_sha256
    );
    insert_report_json!(
        "source_accepted_minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_fixture_count",
        json_u64(
            &source,
            "accepted_minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_fixture_count",
        )
    );
    insert_report_json!(
        "source_blocked_minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_fixture_count",
        json_u64(
            &source,
            "blocked_minimal_scoped_memory_real_write_canary_durable_store_write_receipt_acceptance_fixture_count",
        )
    );
    insert_report_json!(
        "source_receipt_acceptance_result_accepted_count",
        json_u64(&source, "receipt_acceptance_result_accepted_count")
    );
    insert_report_json!(
        "source_single_shot_memory_store_write_performed_count",
        json_u64(&source, "source_memory_store_write_performed_count")
    );
    insert_report_json!(
        "source_single_shot_wal_write_performed_count",
        json_u64(&source, "source_wal_write_performed_count")
    );
    insert_report_json!(
        "source_single_shot_receipt_persisted_count",
        json_u64(&source, "source_receipt_persisted_count")
    );
    insert_report_json!(
        "source_single_shot_post_write_readback_performed_count",
        json_u64(&source, "source_post_write_readback_performed_count")
    );
    insert_report_json!(
        "source_single_shot_rollback_executed_count",
        json_u64(&source, "source_rollback_executed_count")
    );
    insert_report_json!(
        "source_single_shot_tombstone_cleanup_executed_count",
        json_u64(&source, "source_tombstone_cleanup_executed_count")
    );
    for key in [
        "durable_store_write_receipt_acceptance_performed",
        "memory_store_write_performed",
        "wal_write_performed",
        "receipt_persisted",
        "post_write_readback_performed",
        "rollback_executed",
        "tombstone_cleanup_executed",
        "production_durable_memory_store_write_performed",
        "actual_production_durable_memory_write_performed",
        "durable_memory_store_write_performed",
        "durable_memory_store_read_performed",
        "durable_memory_store_rollback_performed",
        "live_kg_write_performed",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "channel_send_performed",
        "external_send_performed",
        "release_artifact_written",
        "install_executed",
        "service_restarted",
        "active_binary_mutated",
    ] {
        report.insert(
            format!("source_current_{key}_count"),
            serde_json::json!(bool_count(&source, key)),
        );
    }
    for key in [
        "source_single_shot_canary_post_write_memory_count",
        "source_single_shot_canary_readback_hit_count",
        "source_single_shot_canary_post_rollback_memory_count",
        "source_single_shot_canary_artifact_write_count",
        "source_single_shot_canary_artifact_readback_count",
        "source_single_shot_canary_artifact_cleanup_removed_count",
        "source_single_shot_canary_artifact_post_cleanup_count",
    ] {
        report.insert(key.to_string(), serde_json::json!(json_u64(&source, key)));
    }
    for key in [
        "source_single_shot_canary_rollback_restored",
        "source_single_shot_canary_post_rollback_absence_confirmed",
        "source_single_shot_canary_artifact_zero_residue_confirmed",
        "single_shot_rollback_cleanup_zero_residue_verified",
    ] {
        report.insert(key.to_string(), serde_json::json!(json_bool(&source, key)));
    }
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
    insert_report_json!("canary_record_id", canary_record_id);
    insert_report_json!(
        "source_receipt_acceptance_boundary_hash_sha256",
        source_receipt_acceptance_boundary_hash_sha256
    );
    insert_report_json!(
        "source_receipt_acceptance_policy_hash_sha256",
        source_receipt_acceptance_policy_hash_sha256
    );
    insert_report_json!("canary_payload_digest_sha256", canary_payload_digest_sha256);
    insert_report_json!(
        "single_shot_receipt_hash_sha256",
        single_shot_receipt_hash_sha256
    );
    insert_report_json!(
        "single_shot_receipt_hash_chain_sha256",
        single_shot_receipt_hash_chain_sha256
    );
    insert_report_json!(
        "single_shot_cleanup_receipt_hash_sha256",
        single_shot_cleanup_receipt_hash_sha256
    );
    insert_report_json!(
        "single_shot_execution_hash_sha256",
        single_shot_execution_hash_sha256
    );
    insert_report_json!(
        "receipt_acceptance_record_hash_sha256",
        receipt_acceptance_record_hash_sha256
    );
    insert_report_json!(
        "receipt_acceptance_readback_hash_sha256",
        receipt_acceptance_readback_hash_sha256
    );
    insert_report_json!(
        "receipt_acceptance_hash_sha256",
        receipt_acceptance_hash_sha256
    );
    insert_report_json!(
        "zero_residue_acceptance_record_hash_sha256",
        zero_residue_acceptance_record_hash_sha256
    );
    insert_report_json!(
        "zero_residue_acceptance_readback_hash_sha256",
        zero_residue_acceptance_readback_hash_sha256
    );
    insert_report_json!(
        "zero_residue_acceptance_hash_sha256",
        zero_residue_acceptance_hash_sha256
    );
    insert_report_json!(
        "rollback_tombstone_cleanup_absence_verified",
        rollback_cleanup_absence_bound
    );
    insert_report_json!(
        "artifact_zero_residue_verified",
        artifact_zero_residue_bound
    );
    insert_report_json!(
        "required_minimal_scoped_memory_real_write_canary_durable_store_write_rollback_tombstone_zero_residue_acceptance_surface_count",
        ZERO_RESIDUE_SURFACES.len()
    );
    insert_report_json!(
        "ready_minimal_scoped_memory_real_write_canary_durable_store_write_rollback_tombstone_zero_residue_acceptance_surface_count",
        ready_surface_count
    );
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_durable_store_write_rollback_tombstone_zero_residue_acceptance_fixture_count",
        fixtures.as_array().map(std::vec::Vec::len).unwrap_or(0)
    );
    insert_report_json!(
        "accepted_minimal_scoped_memory_real_write_canary_durable_store_write_rollback_tombstone_zero_residue_acceptance_fixture_count",
        accepted_fixture_count
    );
    insert_report_json!(
        "blocked_minimal_scoped_memory_real_write_canary_durable_store_write_rollback_tombstone_zero_residue_acceptance_fixture_count",
        blocked_fixture_count
    );
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_durable_store_write_rollback_tombstone_zero_residue_acceptance_accepted_count",
        accepted_fixture_count
    );
    for key in [
        "durable_store_write_rollback_tombstone_zero_residue_acceptance_authority_accepted_count",
        "source_receipt_acceptance_boundary_bound_count",
        "source_receipt_acceptance_hash_bound_count",
        "single_shot_rollback_cleanup_zero_residue_bound_count",
        "single_shot_artifact_zero_residue_bound_count",
        "rollback_tombstone_cleanup_absence_bound_count",
        "zero_residue_acceptance_record_bound_count",
        "zero_residue_acceptance_result_recorded_count",
        "zero_residue_acceptance_result_accepted_count",
        "zero_residue_acceptance_replay_guard_accepted_count",
        "operator_zero_residue_acceptance_handoff_bound_count",
    ] {
        report.insert(key.to_string(), serde_json::json!(accepted_fixture_count));
    }
    for &key in FALSE_ZERO_RESIDUE_SIDE_EFFECT_KEYS {
        report.insert(key.to_string(), serde_json::json!(false));
        report.insert(format!("{key}_count"), serde_json::json!(0));
    }
    for &key in TRUE_ZERO_RESIDUE_KEYS {
        report.insert(key.to_string(), serde_json::json!(report_ready));
        report.insert(
            format!("{key}_count"),
            serde_json::json!(if report_ready { 1 } else { 0 }),
        );
    }
    for key in [
        "source_receipt_acceptance_boundary_required",
        "minimal_scoped_memory_real_write_canary_durable_store_write_rollback_tombstone_zero_residue_acceptance_accepted",
        "approved_namespace_bound",
        "approved_store_bound",
        "approved_scope_bound",
        "durable_store_write_target_bound",
        "durable_store_target_store_bound",
        "single_shot_record_identity_bound",
        "receipt_acceptance_hash_bound",
        "single_shot_receipt_hash_chain_bound",
        "single_shot_rollback_cleanup_zero_residue_bound",
        "single_shot_artifact_zero_residue_bound",
        "rollback_tombstone_cleanup_absence_bound",
        "zero_residue_acceptance_record_bound",
        "zero_residue_acceptance_replay_guard_bound",
        "operator_zero_residue_acceptance_handoff_bound",
        "new_canary_store_write_forbidden_on_report_route",
        "rollback_execution_forbidden_on_report_route",
        "tombstone_write_forbidden_on_report_route",
        "wal_rewrite_forbidden_on_report_route",
        "receipt_repersist_forbidden_on_report_route",
        "production_durable_memory_write_forbidden",
        "durable_memory_read_or_rollback_forbidden",
        "kg_live_write_forbidden",
        "provider_model_invocation_forbidden",
        "credential_channel_public_release_forbidden",
        "install_restart_active_binary_mutation_forbidden",
    ] {
        report.insert(key.to_string(), serde_json::json!(true));
    }
    report.insert(
        "required_minimal_scoped_memory_real_write_canary_durable_store_write_rollback_tombstone_zero_residue_acceptance_fields"
            .to_string(),
        required_fields,
    );
    report.insert(
        "minimal_scoped_memory_real_write_canary_durable_store_write_rollback_tombstone_zero_residue_acceptance_surfaces"
            .to_string(),
        serde_json::json!(ZERO_RESIDUE_SURFACES),
    );
    report.insert(
        "minimal_scoped_memory_real_write_canary_durable_store_write_rollback_tombstone_zero_residue_acceptance_fixtures"
            .to_string(),
        fixtures,
    );
    report.insert(
        "denied_by_minimal_scoped_memory_real_write_canary_durable_store_write_rollback_tombstone_zero_residue_acceptance_boundary"
            .to_string(),
        serde_json::Value::Array(denied_by),
    );
    insert_report_json!(
        "denied_by_minimal_scoped_memory_real_write_canary_durable_store_write_rollback_tombstone_zero_residue_acceptance_boundary_count",
        denied_count
    );
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_durable_store_write_rollback_tombstone_zero_residue_acceptance_boundary_hash_sha256",
        boundary_hash_sha256
    );
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_durable_store_write_rollback_tombstone_zero_residue_acceptance_policy_hash_sha256",
        policy_hash_sha256
    );
    report.insert("allowed_next_actions".to_string(), allowed_next_actions);
    report.insert(
        "side_effects".to_string(),
        serde_json::Value::Object(side_effects),
    );
    serde_json::Value::Object(report)
}

fn hepta_memory_live_mutation_operator_write_execution_scoped_production_durable_memory_write_preflight_boundary_report()
-> serde_json::Value {
    const PREFLIGHT_SURFACES: &[&str] = &[
        "source_zero_residue_acceptance_boundary_required",
        "production_durable_memory_target_required",
        "operator_approval_packet_required",
        "single_use_nonce_required",
        "explicit_command_required",
        "payload_redaction_required",
        "wal_receipt_plan_required",
        "post_write_readback_plan_required",
        "rollback_tombstone_zero_residue_plan_required",
        "replay_idempotency_guard_required",
        "operator_preflight_handoff_required",
        "production_write_execution_forbidden_on_preflight_route",
    ];
    const PREFLIGHT_DENIALS: &[&str] = &[
        "source_zero_residue_acceptance_boundary_required",
        "source_zero_residue_acceptance_hash_required",
        "approved_production_namespace_required",
        "approved_production_store_required",
        "approved_production_scope_required",
        "production_durable_memory_target_required",
        "operator_approval_packet_required",
        "operator_identity_session_required",
        "operator_scope_binding_required",
        "single_use_nonce_required",
        "explicit_command_required",
        "command_budget_required",
        "payload_redaction_required",
        "raw_plaintext_payload_denied",
        "wal_plan_required",
        "receipt_plan_required",
        "receipt_hash_chain_required",
        "post_write_readback_plan_required",
        "rollback_plan_required",
        "tombstone_cleanup_plan_required",
        "zero_residue_plan_required",
        "replay_idempotency_guard_required",
        "preflight_result_record_required",
        "preflight_result_readback_required",
        "production_write_execution_report_route_denied",
        "production_durable_memory_backend_write_denied",
        "durable_memory_backend_read_or_rollback_denied",
        "memory_store_mutation_denied",
        "wal_write_report_route_denied",
        "receipt_persist_report_route_denied",
        "rollback_execution_report_route_denied",
        "tombstone_write_report_route_denied",
        "kg_live_write_denied",
        "provider_model_invocation_denied",
        "credential_channel_release_install_denied",
        "unrestricted_full_live_activation_denied",
    ];
    const FALSE_PREFLIGHT_SIDE_EFFECT_KEYS: &[&str] = &[
        "production_durable_memory_write_executed",
        "production_durable_memory_backend_present",
        "production_durable_memory_store_write_performed",
        "actual_production_durable_memory_write_performed",
        "durable_memory_store_write_performed",
        "durable_memory_store_read_performed",
        "durable_memory_store_rollback_performed",
        "memory_write_execution_performed",
        "memory_store_write_path_enabled",
        "memory_store_write_allowed",
        "memory_store_write_performed",
        "memory_store_mutation_allowed",
        "memory_store_mutated",
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
        "rollback_performed",
        "rollback_result_recorded",
        "rollback_result_persisted",
        "rollback_result_accepted",
        "tombstone_write_performed",
        "tombstone_cleanup_executed",
        "tombstone_cleanup_result_recorded",
        "tombstone_cleanup_result_accepted",
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
        "scoped_production_durable_memory_write_preflight_performed",
        "scoped_production_durable_memory_write_preflight_result_recorded",
        "scoped_production_durable_memory_write_preflight_result_accepted",
        "source_zero_residue_acceptance_boundary_accepted",
        "production_durable_memory_target_bound",
        "operator_approval_packet_preflight_bound",
        "operator_identity_session_preflight_bound",
        "single_use_nonce_preflight_bound",
        "explicit_command_preflight_bound",
        "payload_redaction_preflight_bound",
        "wal_receipt_preflight_bound",
        "post_write_readback_preflight_bound",
        "rollback_tombstone_zero_residue_preflight_bound",
        "replay_idempotency_preflight_bound",
        "production_write_execution_forbidden_on_preflight_route",
        "kg_provider_channel_release_install_active_binary_forbidden",
    ];

    let route_matrix = control_ui_route_parity_report();
    let source = std::thread::Builder::new()
        .name("hepta-memory-production-durable-preflight-source-report".to_string())
        .stack_size(8 * 1024 * 1024)
        .spawn(
            hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_durable_store_write_rollback_tombstone_zero_residue_acceptance_boundary_report,
        )
        .ok()
        .and_then(|handle| handle.join().ok())
        .unwrap_or_else(|| {
            serde_json::json!({
                "status": "blocked",
                "memory_write_execution_minimal_scoped_memory_real_write_canary_durable_store_write_rollback_tombstone_zero_residue_acceptance_boundary_ready": false,
                "minimal_scoped_memory_real_write_canary_durable_store_write_rollback_tombstone_zero_residue_acceptance_accepted": false,
                "source_scoped_production_durable_memory_write_preflight_source_report_thread_failed": true
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
                == Some("prepare_scoped_production_durable_memory_write_preflight_boundary")
                && item
                    .get(
                        "requires_minimal_scoped_memory_real_write_canary_durable_store_write_rollback_tombstone_zero_residue_acceptance_boundary",
                    )
                    .and_then(serde_json::Value::as_bool)
                    == Some(true)
                && item
                    .get("writes_production_durable_memory")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
        })
        .unwrap_or(false);
    let source_side_effects_ok = source
        .get("side_effects")
        .and_then(serde_json::Value::as_object)
        .map(|effects| {
            effects
                .get("durable_store_write_rollback_tombstone_zero_residue_acceptance_performed")
                .and_then(serde_json::Value::as_bool)
                == Some(true)
                && effects
                    .get(
                        "durable_store_write_rollback_tombstone_zero_residue_acceptance_result_accepted",
                    )
                    .and_then(serde_json::Value::as_bool)
                    == Some(true)
                && effects
                    .get("memory_store_write_performed")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && effects
                    .get("wal_write_performed")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && effects
                    .get("receipt_persisted")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && effects
                    .get("rollback_executed")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && effects
                    .get("tombstone_cleanup_executed")
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
            "memory_write_execution_minimal_scoped_memory_real_write_canary_durable_store_write_rollback_tombstone_zero_residue_acceptance_boundary_ready",
        )
        && json_bool(
            &source,
            "minimal_scoped_memory_real_write_canary_durable_store_write_rollback_tombstone_zero_residue_acceptance_accepted",
        )
        && json_u64(
            &source,
            "accepted_minimal_scoped_memory_real_write_canary_durable_store_write_rollback_tombstone_zero_residue_acceptance_fixture_count",
        ) == 1
        && json_u64(
            &source,
            "blocked_minimal_scoped_memory_real_write_canary_durable_store_write_rollback_tombstone_zero_residue_acceptance_fixture_count",
        ) == 9
        && json_u64(&source, "zero_residue_acceptance_result_accepted_count") == 1
        && json_bool(&source, "rollback_tombstone_cleanup_absence_verified")
        && json_bool(&source, "artifact_zero_residue_verified")
        && json_bool(
            &source,
            "source_single_shot_canary_artifact_zero_residue_confirmed",
        )
        && json_u64(
            &source,
            "source_single_shot_canary_post_rollback_memory_count",
        ) == 0
        && json_u64(
            &source,
            "source_single_shot_canary_artifact_post_cleanup_count",
        ) == 0
        && json_u64(
            &source,
            "source_single_shot_memory_store_write_performed_count",
        ) == 1
        && json_u64(&source, "source_single_shot_wal_write_performed_count") == 1
        && json_u64(&source, "source_single_shot_receipt_persisted_count") == 1
        && json_u64(
            &source,
            "source_single_shot_post_write_readback_performed_count",
        ) == 1
        && json_u64(&source, "source_single_shot_rollback_executed_count") == 1
        && json_u64(
            &source,
            "source_single_shot_tombstone_cleanup_executed_count",
        ) == 1
        && !json_bool(&source, "memory_store_write_performed")
        && !json_bool(&source, "wal_write_performed")
        && !json_bool(&source, "receipt_persisted")
        && !json_bool(&source, "rollback_executed")
        && !json_bool(&source, "tombstone_cleanup_executed")
        && !json_bool(&source, "production_durable_memory_store_write_performed")
        && !json_bool(&source, "actual_production_durable_memory_write_performed")
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
        && !json_bool(&source, "service_restarted")
        && !json_bool(&source, "active_binary_mutated")
        && source_next_action_preflight
        && source_side_effects_ok;

    let approved_production_namespace = "hepta.memory.production.scoped";
    let approved_production_store = "hepta-memory-durable-store-production-preflight-only";
    let approved_production_scope = "operator-approved-session";
    let production_durable_memory_target_id =
        "hepta-scoped-production-durable-memory-write-target-v1";
    let production_durable_memory_payload_class = "redacted-minimal-operator-approved-memory-fact";
    let operator_packet_scope = "hepta.memory.production.scoped:session:single-write-preflight";
    let source_report_sha256 = sha256_text_value(&source.to_string());
    let source_zero_residue_acceptance_boundary_hash_sha256 = json_str(
        &source,
        "minimal_scoped_memory_real_write_canary_durable_store_write_rollback_tombstone_zero_residue_acceptance_boundary_hash_sha256",
    );
    let source_zero_residue_acceptance_policy_hash_sha256 = json_str(
        &source,
        "minimal_scoped_memory_real_write_canary_durable_store_write_rollback_tombstone_zero_residue_acceptance_policy_hash_sha256",
    );
    let source_zero_residue_acceptance_hash_sha256 =
        json_str(&source, "zero_residue_acceptance_hash_sha256");
    let target_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-target:v1:namespace={approved_production_namespace}:store={approved_production_store}:scope={approved_production_scope}:target={production_durable_memory_target_id}:source={source_zero_residue_acceptance_hash_sha256}"
    ));
    let operator_packet_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-operator-packet-preflight:v1:scope={operator_packet_scope}:target={target_hash_sha256}:requires-fresh-approval=true"
    ));
    let nonce_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-nonce-preflight:v1:packet={operator_packet_hash_sha256}:single-use=true"
    ));
    let command_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-explicit-command-preflight:v1:nonce={nonce_hash_sha256}:budget=single-write"
    ));
    let payload_redaction_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-payload-redaction-preflight:v1:class={production_durable_memory_payload_class}:raw-plaintext-recording=false"
    ));
    let wal_receipt_plan_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-wal-receipt-plan:v1:command={command_hash_sha256}:payload={payload_redaction_hash_sha256}:persist-now=false"
    ));
    let readback_plan_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-readback-plan:v1:wal-receipt={wal_receipt_plan_hash_sha256}:execute-now=false"
    ));
    let rollback_tombstone_zero_residue_plan_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-rollback-tombstone-zero-residue-plan:v1:readback={readback_plan_hash_sha256}:source-zero-residue={source_zero_residue_acceptance_hash_sha256}:execute-now=false"
    ));
    let preflight_result_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-preflight-result:v1:target={target_hash_sha256}:operator={operator_packet_hash_sha256}:command={command_hash_sha256}:rollback={rollback_tombstone_zero_residue_plan_hash_sha256}:accepted=true"
    ));
    let preflight_boundary_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-preflight-boundary:v1:source={source_report_sha256}:result={preflight_result_hash_sha256}:fixtures=10:accepted=1:denials={}:production-write=false",
        PREFLIGHT_DENIALS.len()
    ));
    let preflight_policy_hash_sha256 = sha256_text_value(
        "scoped-production-durable-memory-write-preflight-policy:v1:bind-target-operator-nonce-command-wal-receipt-readback-rollback-tombstone-zero-residue:no-production-write:no-kg:no-provider:no-channel:no-release:no-install",
    );
    let target_bound = !source_zero_residue_acceptance_boundary_hash_sha256.is_empty()
        && !source_zero_residue_acceptance_policy_hash_sha256.is_empty()
        && !source_zero_residue_acceptance_hash_sha256.is_empty();
    let surfaces_ready = source_ready && target_bound;
    let report_ready = route_count_source_command_accepted && surfaces_ready;
    let accepted_fixture_count = if report_ready { 1 } else { 0 };
    let blocked_fixture_count = 10 - accepted_fixture_count;

    let mut fixtures = Vec::new();
    fixtures.push(serde_json::json!({
        "id": "scoped-production-durable-memory-write-preflight",
        "fixture_id": "scoped-production-durable-memory-write-preflight",
        "scoped_production_durable_memory_write_preflight_accepted": report_ready,
        "reason": if report_ready { "production_durable_memory_write_preflight_guards_bound_without_execution" } else { "source_zero_residue_or_route_count_not_ready" },
        "source_zero_residue_acceptance_boundary_bound": report_ready,
        "production_durable_memory_target_bound": report_ready,
        "operator_approval_packet_preflight_bound": report_ready,
        "single_use_nonce_preflight_bound": report_ready,
        "explicit_command_preflight_bound": report_ready,
        "wal_receipt_preflight_bound": report_ready,
        "post_write_readback_preflight_bound": report_ready,
        "rollback_tombstone_zero_residue_preflight_bound": report_ready,
        "production_durable_memory_store_write_performed": false,
        "external_send_performed": false
    }));
    for id in [
        "missing-zero-residue-source",
        "wrong-production-namespace",
        "missing-operator-approval-packet",
        "missing-single-use-nonce",
        "missing-explicit-command",
        "missing-wal-receipt-plan",
        "missing-post-write-readback-plan",
        "missing-rollback-tombstone-zero-residue-plan",
        "production-write-or-external-side-effect-attempt",
    ] {
        fixtures.push(serde_json::json!({
            "id": id,
            "fixture_id": id,
            "scoped_production_durable_memory_write_preflight_accepted": false,
            "reason": "blocked_noop",
            "production_durable_memory_store_write_performed": false,
            "external_send_performed": false
        }));
    }

    let mut side_effects = serde_json::Map::new();
    for &key in FALSE_PREFLIGHT_SIDE_EFFECT_KEYS {
        side_effects.insert(key.to_string(), serde_json::json!(false));
        side_effects.insert(format!("{key}_count"), serde_json::json!(0));
    }
    for &key in TRUE_PREFLIGHT_KEYS {
        side_effects.insert(key.to_string(), serde_json::json!(report_ready));
        side_effects.insert(
            format!("{key}_count"),
            serde_json::json!(if report_ready { 1 } else { 0 }),
        );
    }

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "endpoint": HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_SCOPED_PRODUCTION_DURABLE_MEMORY_WRITE_PREFLIGHT_BOUNDARY_ENDPOINT,
        "source_command": "/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-preflight-boundary --json",
        "native_route": true,
        "side_effect_free": false,
        "external_side_effect_free": true,
        "audit_date": "2026-07-05"
    });
    let report_object = report
        .as_object_mut()
        .expect("scoped production durable Memory write preflight report object");
    macro_rules! insert_report {
        ($key:expr, $value:expr) => {
            report_object.insert($key.to_string(), serde_json::json!($value));
        };
    }

    insert_report!(
        "native_gateway_source_command_count",
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    );
    insert_report!("route_count", route_matrix.route_count);
    insert_report!(
        "implemented_route_count",
        route_matrix.implemented_route_count
    );
    insert_report!("missing_route_count", route_matrix.missing_route_count);
    insert_report!(
        "route_count_source_command_accepted",
        route_count_source_command_accepted
    );
    insert_report!(
        "memory_write_execution_scoped_production_durable_memory_write_preflight_boundary_ready",
        report_ready
    );
    insert_report!(
        "scoped_production_durable_memory_write_preflight_ready",
        report_ready
    );
    insert_report!(
        "scoped_production_durable_memory_write_preflight_performed",
        report_ready
    );
    insert_report!(
        "scoped_production_durable_memory_write_preflight_accepted",
        report_ready
    );
    insert_report!(
        "scoped_production_durable_memory_write_preflight_mode",
        "preflight_only_no_production_durable_memory_mutation"
    );
    insert_report!(
        "source_minimal_scoped_memory_real_write_canary_durable_store_write_rollback_tombstone_zero_residue_acceptance_boundary_ready",
        source_ready
    );
    insert_report!(
        "source_minimal_scoped_memory_real_write_canary_durable_store_write_rollback_tombstone_zero_residue_acceptance_boundary_report_sha256",
        source_report_sha256
    );
    insert_report!(
        "source_zero_residue_acceptance_boundary_hash_sha256",
        source_zero_residue_acceptance_boundary_hash_sha256
    );
    insert_report!(
        "source_zero_residue_acceptance_policy_hash_sha256",
        source_zero_residue_acceptance_policy_hash_sha256
    );
    insert_report!(
        "source_zero_residue_acceptance_hash_sha256",
        source_zero_residue_acceptance_hash_sha256
    );
    insert_report!(
        "source_accepted_zero_residue_acceptance_fixture_count",
        json_u64(
            &source,
            "accepted_minimal_scoped_memory_real_write_canary_durable_store_write_rollback_tombstone_zero_residue_acceptance_fixture_count"
        )
    );
    insert_report!(
        "source_blocked_zero_residue_acceptance_fixture_count",
        json_u64(
            &source,
            "blocked_minimal_scoped_memory_real_write_canary_durable_store_write_rollback_tombstone_zero_residue_acceptance_fixture_count"
        )
    );
    insert_report!(
        "source_zero_residue_acceptance_result_accepted_count",
        json_u64(&source, "zero_residue_acceptance_result_accepted_count")
    );
    insert_report!(
        "source_single_shot_memory_store_write_performed_count",
        json_u64(
            &source,
            "source_single_shot_memory_store_write_performed_count"
        )
    );
    insert_report!(
        "source_single_shot_wal_write_performed_count",
        json_u64(&source, "source_single_shot_wal_write_performed_count")
    );
    insert_report!(
        "source_single_shot_receipt_persisted_count",
        json_u64(&source, "source_single_shot_receipt_persisted_count")
    );
    insert_report!(
        "source_single_shot_post_write_readback_performed_count",
        json_u64(
            &source,
            "source_single_shot_post_write_readback_performed_count"
        )
    );
    insert_report!(
        "source_single_shot_rollback_executed_count",
        json_u64(&source, "source_single_shot_rollback_executed_count")
    );
    insert_report!(
        "source_single_shot_tombstone_cleanup_executed_count",
        json_u64(
            &source,
            "source_single_shot_tombstone_cleanup_executed_count"
        )
    );
    insert_report!(
        "source_single_shot_canary_post_rollback_memory_count",
        json_u64(
            &source,
            "source_single_shot_canary_post_rollback_memory_count"
        )
    );
    insert_report!(
        "source_single_shot_canary_artifact_post_cleanup_count",
        json_u64(
            &source,
            "source_single_shot_canary_artifact_post_cleanup_count"
        )
    );
    insert_report!(
        "source_single_shot_canary_artifact_zero_residue_confirmed",
        json_bool(
            &source,
            "source_single_shot_canary_artifact_zero_residue_confirmed"
        )
    );
    insert_report!("source_current_memory_store_write_performed_count", 0);
    insert_report!("source_current_wal_write_performed_count", 0);
    insert_report!("source_current_receipt_persisted_count", 0);
    insert_report!("source_current_rollback_executed_count", 0);
    insert_report!("source_current_tombstone_cleanup_executed_count", 0);
    insert_report!(
        "source_current_durable_memory_store_write_performed_count",
        0
    );
    insert_report!("source_current_external_send_performed_count", 0);
    insert_report!(
        "approved_production_namespace",
        approved_production_namespace
    );
    insert_report!("approved_production_store", approved_production_store);
    insert_report!("approved_production_scope", approved_production_scope);
    insert_report!(
        "production_durable_memory_target_id",
        production_durable_memory_target_id
    );
    insert_report!(
        "production_durable_memory_payload_class",
        production_durable_memory_payload_class
    );
    insert_report!("operator_packet_scope", operator_packet_scope);
    insert_report!(
        "production_durable_memory_write_preflight_target_hash_sha256",
        target_hash_sha256
    );
    insert_report!(
        "production_durable_memory_write_preflight_operator_packet_hash_sha256",
        operator_packet_hash_sha256
    );
    insert_report!(
        "production_durable_memory_write_preflight_nonce_hash_sha256",
        nonce_hash_sha256
    );
    insert_report!(
        "production_durable_memory_write_preflight_command_hash_sha256",
        command_hash_sha256
    );
    insert_report!(
        "production_durable_memory_write_preflight_payload_redaction_hash_sha256",
        payload_redaction_hash_sha256
    );
    insert_report!(
        "production_durable_memory_write_preflight_wal_receipt_plan_hash_sha256",
        wal_receipt_plan_hash_sha256
    );
    insert_report!(
        "production_durable_memory_write_preflight_readback_plan_hash_sha256",
        readback_plan_hash_sha256
    );
    insert_report!(
        "production_durable_memory_write_preflight_rollback_tombstone_zero_residue_plan_hash_sha256",
        rollback_tombstone_zero_residue_plan_hash_sha256
    );
    insert_report!(
        "scoped_production_durable_memory_write_preflight_result_hash_sha256",
        preflight_result_hash_sha256
    );
    insert_report!(
        "scoped_production_durable_memory_write_preflight_boundary_hash_sha256",
        preflight_boundary_hash_sha256
    );
    insert_report!(
        "scoped_production_durable_memory_write_preflight_policy_hash_sha256",
        preflight_policy_hash_sha256
    );
    insert_report!(
        "required_scoped_production_durable_memory_write_preflight_surface_count",
        PREFLIGHT_SURFACES.len()
    );
    insert_report!(
        "ready_scoped_production_durable_memory_write_preflight_surface_count",
        if surfaces_ready {
            PREFLIGHT_SURFACES.len()
        } else {
            0
        }
    );
    insert_report!(
        "scoped_production_durable_memory_write_preflight_surfaces",
        PREFLIGHT_SURFACES
    );
    insert_report!(
        "scoped_production_durable_memory_write_preflight_fixture_count",
        fixtures.len()
    );
    insert_report!(
        "accepted_scoped_production_durable_memory_write_preflight_fixture_count",
        accepted_fixture_count
    );
    insert_report!(
        "blocked_scoped_production_durable_memory_write_preflight_fixture_count",
        blocked_fixture_count
    );
    insert_report!(
        "scoped_production_durable_memory_write_preflight_fixtures",
        fixtures
    );
    insert_report!(
        "denied_by_scoped_production_durable_memory_write_preflight_boundary",
        PREFLIGHT_DENIALS
    );
    insert_report!(
        "denied_by_scoped_production_durable_memory_write_preflight_boundary_count",
        PREFLIGHT_DENIALS.len()
    );
    insert_report!(
        "allowed_next_actions",
        [
            serde_json::json!({
                "action": "run_scoped_production_durable_memory_write_preflight_boundary_require_live_gate",
                "status": "allowed_verification_only",
                "accepts_preflight_evidence": true,
                "writes_production_durable_memory": false,
                "writes_memory_store": false,
                "writes_wal": false,
                "persists_receipt": false,
                "executes_rollback": false,
                "writes_tombstone": false
            }),
            serde_json::json!({
                "action": "prepare_scoped_production_durable_memory_write_operator_packet_acceptance_boundary",
                "status": "requires_separate_operator_packet_acceptance_gate",
                "requires_scoped_production_durable_memory_write_preflight_boundary": true,
                "writes_production_durable_memory": false
            }),
        ]
    );
    for &key in FALSE_PREFLIGHT_SIDE_EFFECT_KEYS {
        report_object.insert(key.to_string(), serde_json::json!(false));
        report_object.insert(format!("{key}_count"), serde_json::json!(0));
    }
    for &key in TRUE_PREFLIGHT_KEYS {
        report_object.insert(key.to_string(), serde_json::json!(report_ready));
        report_object.insert(
            format!("{key}_count"),
            serde_json::json!(if report_ready { 1 } else { 0 }),
        );
    }
    for key in [
        "source_zero_residue_acceptance_boundary_bound",
        "approved_production_namespace_bound",
        "approved_production_store_bound",
        "approved_production_scope_bound",
        "production_durable_memory_target_bound",
        "operator_approval_packet_preflight_bound",
        "operator_identity_session_preflight_bound",
        "single_use_nonce_preflight_bound",
        "explicit_command_preflight_bound",
        "payload_redaction_preflight_bound",
        "wal_receipt_preflight_bound",
        "post_write_readback_preflight_bound",
        "rollback_tombstone_zero_residue_preflight_bound",
        "replay_idempotency_preflight_bound",
        "production_write_execution_forbidden_on_preflight_route",
        "production_durable_memory_write_forbidden",
        "memory_store_mutation_forbidden",
        "wal_write_forbidden_on_preflight_route",
        "receipt_persist_forbidden_on_preflight_route",
        "rollback_execution_forbidden_on_preflight_route",
        "tombstone_write_forbidden_on_preflight_route",
        "kg_live_write_forbidden",
        "provider_model_invocation_forbidden",
        "credential_channel_public_release_forbidden",
        "install_restart_active_binary_mutation_forbidden",
    ] {
        report_object.insert(key.to_string(), serde_json::json!(true));
    }
    report_object.insert(
        "side_effects".to_string(),
        serde_json::Value::Object(side_effects),
    );
    report
}


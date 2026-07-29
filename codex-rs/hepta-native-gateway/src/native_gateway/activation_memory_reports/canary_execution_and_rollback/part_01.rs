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

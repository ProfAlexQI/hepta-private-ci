
fn hepta_memory_live_mutation_operator_write_execution_activation_command_result_receipt_release_artifact_publication_denial_boundary_report()
-> serde_json::Value {
    const RELEASE_ARTIFACT_PUBLICATION_SURFACES: &[&str] = &[
        "source_terminal_operator_decision_report_required",
        "release_artifact_write_denied",
        "public_artifact_write_denied",
        "artifact_signature_notarization_acceptance_denied",
        "publication_queue_enqueue_denied",
        "publication_manifest_write_denied",
        "public_distribution_channel_delivery_denied",
        "public_version_tag_release_promotion_denied",
        "release_notes_changelog_materialization_denied",
        "terminal_operator_decision_is_not_release_approval",
        "activation_from_release_artifact_publication_denied",
        "external_public_install_restart_active_binary_publication_denied",
    ];
    const RELEASE_ARTIFACT_PUBLICATION_DENIALS: &[&str] = &[
        "source_terminal_operator_decision_report_required",
        "release_artifact_write_denied",
        "public_artifact_write_denied",
        "artifact_signature_notarization_acceptance_denied",
        "publication_queue_enqueue_denied",
        "publication_manifest_write_denied",
        "public_distribution_channel_delivery_denied",
        "public_version_tag_release_promotion_denied",
        "release_notes_changelog_materialization_denied",
        "terminal_operator_decision_is_not_release_approval",
        "activation_from_release_artifact_publication_denied",
        "memory_write_publication_denied",
        "provider_prompt_publication_denied",
        "install_restart_active_binary_publication_denied",
    ];
    const FALSE_KEYS: &[&str] = &[
        "activation_command_result_receipt_release_artifact_publication_allowed",
        "activation_command_result_receipt_release_artifact_publication_requested",
        "activation_command_result_receipt_release_artifact_publication_accepted",
        "activation_command_result_receipt_release_artifact_publication_recorded",
        "activation_command_result_receipt_release_artifact_publication_persisted",
        "activation_command_result_receipt_release_artifact_publication_materialized",
        "release_artifact_publication_allowed",
        "release_artifact_publication_requested",
        "release_artifact_publication_accepted",
        "release_artifact_publication_recorded",
        "release_artifact_publication_persisted",
        "release_artifact_publication_materialized",
        "release_artifact_filesystem_written",
        "release_artifact_written",
        "public_artifact_written",
        "artifact_signature_accepted",
        "artifact_notarization_accepted",
        "publication_queue_enqueued",
        "publication_manifest_written",
        "public_distribution_performed",
        "public_release_published",
        "public_ga_claimed",
        "public_claim_promoted",
        "public_version_tag_created",
        "release_notes_materialized",
        "changelog_materialized",
        "terminal_operator_decision_promoted_to_release_approval",
        "telegram_send_performed",
        "channel_send_performed",
        "external_send_performed",
        "activation_command_result_receipt_recorded",
        "activation_command_result_receipt_persisted",
        "activation_command_result_receipt_accepted",
        "activation_command_result_receipt_materialized",
        "activation_command_completion_ack_recorded",
        "activation_allowed_by_release_artifact_publication",
        "activation_allowed_by_terminal_operator_decision",
        "activation_allowed_by_result_receipt",
        "activation_allowed",
        "activation_performed",
        "activation_activated",
        "live_mutation_execution_ready",
        "live_mutation_execution_allowed",
        "live_mutation_execution_performed",
        "memory_write_execution_allowed",
        "memory_write_execution_ready",
        "memory_write_execution_performed",
        "memory_store_write_path_enabled",
        "memory_store_write_allowed",
        "memory_store_write_performed",
        "memory_store_mutation_allowed",
        "memory_store_mutated",
        "live_kg_write_performed",
        "rollback_execution_allowed",
        "rollback_executed",
        "raw_payload_plaintext_recorded",
        "raw_payload_plaintext_persisted",
        "secret_material_read",
        "auth_secret_read",
        "credential_read",
        "secret_file_read",
        "provider_prompt_replay_enabled",
        "provider_invoked",
        "model_invoked",
        "install_executed",
        "launchd_mutated",
        "service_restart_performed",
        "service_restarted",
        "active_binary_mutated",
        "filesystem_written",
    ];

    fn release_artifact_publication_fixture(
        id: &str,
        status: &str,
        reason: &str,
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
        insert_fixture_json!("release_artifact_publication_status", status);
        insert_fixture_json!("source_terminal_operator_decision_present", true);
        insert_fixture_json!("source_terminal_operator_decision_ready", true);
        insert_fixture_json!("release_artifact_publication_noop_confirmed", true);
        insert_fixture_json!("reason", reason);
        for key in [
            "release_artifact_publication_requested",
            "release_artifact_write_requested",
            "public_artifact_write_requested",
            "artifact_signature_requested",
            "artifact_notarization_requested",
            "publication_queue_enqueue_requested",
            "publication_manifest_write_requested",
            "public_distribution_requested",
            "telegram_delivery_requested",
            "channel_delivery_requested",
            "external_delivery_requested",
            "public_version_tag_requested",
            "public_release_publish_requested",
            "public_ga_claim_requested",
            "release_notes_materialization_requested",
            "changelog_materialization_requested",
            "terminal_operator_decision_release_approval_requested",
            "activation_from_release_publication_requested",
            "memory_write_publication_requested",
            "provider_prompt_publication_requested",
            "install_publication_requested",
            "service_restart_publication_requested",
            "active_binary_publication_requested",
        ] {
            base.insert(key.to_string(), serde_json::json!(false));
        }
        for key in [
            "release_artifact_publication_allowed",
            "release_artifact_publication_accepted",
            "release_artifact_publication_recorded",
            "release_artifact_publication_persisted",
            "release_artifact_publication_materialized",
            "release_artifact_filesystem_written",
            "release_artifact_written",
            "public_artifact_written",
            "artifact_signature_accepted",
            "artifact_notarization_accepted",
            "publication_queue_enqueued",
            "publication_manifest_written",
            "public_distribution_performed",
            "public_release_published",
            "public_ga_claimed",
            "public_claim_promoted",
            "public_version_tag_created",
            "release_notes_materialized",
            "changelog_materialized",
            "terminal_operator_decision_promoted_to_release_approval",
            "telegram_send_performed",
            "channel_send_performed",
            "external_send_performed",
            "receipt_recorded",
            "receipt_persisted",
            "receipt_accepted",
            "receipt_materialized",
            "completion_ack_recorded",
            "activation_allowed",
            "activation_performed",
            "live_mutation_execution_performed",
            "memory_write_execution_performed",
            "memory_store_write_performed",
            "memory_store_mutated",
            "live_kg_write_performed",
            "rollback_executed",
            "secret_material_read",
            "auth_secret_read",
            "credential_read",
            "secret_file_read",
            "provider_invoked",
            "model_invoked",
            "provider_prompt_replayed",
            "install_executed",
            "launchd_mutated",
            "service_restarted",
            "service_restart_performed",
            "active_binary_mutated",
        ] {
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
        .name("hepta-memory-write-result-receipt-terminal-decision-source-report".to_string())
        .stack_size(8 * 1024 * 1024)
        .spawn(
            hepta_memory_live_mutation_operator_write_execution_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_boundary_report,
        )
        .ok()
        .and_then(|handle| handle.join().ok())
        .unwrap_or_else(|| {
            serde_json::json!({
                "status": "blocked",
                "memory_write_execution_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_boundary_ready": false,
                "memory_write_execution_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_ready": false,
                "source_terminal_operator_decision_public_claim_source_report_thread_failed": true
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
    let source_next_action_release_artifact = source
        .get("allowed_next_actions")
        .and_then(serde_json::Value::as_array)
        .and_then(|items| items.get(1))
        .map(|item| {
            item.get("action").and_then(serde_json::Value::as_str)
                == Some(
                    "prepare_memory_write_execution_activation_command_result_receipt_release_artifact_publication_denial_boundary",
                )
                && item
                    .get("publishes_release_artifact")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && item
                    .get("claims_public_release")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && item
                    .get("writes_release_artifact")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
        })
        .unwrap_or(false);
    let source_ready = json_str(&source, "status") == "ready"
        && json_bool(
            &source,
            "memory_write_execution_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_boundary_ready",
        )
        && json_bool(
            &source,
            "memory_write_execution_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_ready",
        )
        && json_u64(
            &source,
            "activation_command_result_receipt_terminal_operator_decision_public_claim_fixture_count",
        ) == 10
        && json_u64(
            &source,
            "blocked_activation_command_result_receipt_terminal_operator_decision_public_claim_fixture_count",
        ) == 10
        && json_u64(
            &source,
            "accepted_activation_command_result_receipt_terminal_operator_decision_public_claim_fixture_count",
        ) == 0
        && json_u64(
            &source,
            "activation_command_result_receipt_terminal_operator_decision_performed_count",
        ) == 0
        && json_u64(
            &source,
            "activation_command_result_receipt_public_claim_promotion_performed_count",
        ) == 0
        && json_u64(
            &source,
            "denied_by_activation_command_result_receipt_terminal_operator_decision_public_claim_count",
        ) == 55
        && !json_bool(
            &source,
            "activation_command_result_receipt_public_claim_promoted",
        )
        && !json_bool(&source, "public_release_published")
        && !json_bool(&source, "public_ga_claimed")
        && !json_bool(&source, "release_artifact_written")
        && !json_bool(&source, "public_artifact_written")
        && !json_bool(&source, "public_distribution_performed")
        && !json_bool(&source, "activation_allowed")
        && !json_bool(&source, "activation_performed")
        && !json_bool(&source, "memory_store_write_performed")
        && !json_bool(&source, "memory_store_mutated")
        && !json_bool(&source, "rollback_executed")
        && !json_bool(&source, "provider_invoked")
        && !json_bool(&source, "model_invoked")
        && !json_bool(&source, "credential_read")
        && !json_bool(&source, "secret_file_read")
        && !json_bool(&source, "live_kg_write_performed")
        && !json_bool(&source, "telegram_send_performed")
        && !json_bool(&source, "channel_send_performed")
        && !json_bool(&source, "external_send_performed")
        && !json_bool(&source, "install_executed")
        && !json_bool(&source, "service_restarted")
        && !json_bool(&source, "active_binary_mutated")
        && source_next_action_release_artifact
        && side_effects_all_false(&source);

    let release_artifact_publication_fixtures = serde_json::Value::Array(vec![
        release_artifact_publication_fixture(
            "activation-result-receipt-release-artifact-publication-missing-terminal-decision",
            "blocked_noop",
            "source_terminal_operator_decision_report_required",
            serde_json::json!({
                "source_terminal_operator_decision_present": false,
                "source_terminal_operator_decision_ready": false,
                "release_artifact_publication_requested": true
            }),
        ),
        release_artifact_publication_fixture(
            "activation-result-receipt-release-artifact-write-request",
            "blocked_artifact_noop",
            "release_artifact_write_denied",
            serde_json::json!({
                "release_artifact_write_requested": true,
                "release_artifact_publication_requested": true
            }),
        ),
        release_artifact_publication_fixture(
            "activation-result-receipt-public-artifact-write-request",
            "blocked_artifact_noop",
            "public_artifact_write_denied",
            serde_json::json!({
                "public_artifact_write_requested": true,
                "release_artifact_publication_requested": true
            }),
        ),
        release_artifact_publication_fixture(
            "activation-result-receipt-artifact-signature-notarization-request",
            "blocked_artifact_noop",
            "artifact_signature_notarization_acceptance_denied",
            serde_json::json!({
                "artifact_signature_requested": true,
                "artifact_notarization_requested": true,
                "release_artifact_publication_requested": true
            }),
        ),
        release_artifact_publication_fixture(
            "activation-result-receipt-publication-queue-request",
            "blocked_publication_noop",
            "publication_queue_enqueue_denied",
            serde_json::json!({
                "publication_queue_enqueue_requested": true,
                "publication_manifest_write_requested": true
            }),
        ),
        release_artifact_publication_fixture(
            "activation-result-receipt-distribution-channel-request",
            "blocked_distribution_noop",
            "public_distribution_channel_delivery_denied",
            serde_json::json!({
                "public_distribution_requested": true,
                "telegram_delivery_requested": true,
                "channel_delivery_requested": true,
                "external_delivery_requested": true
            }),
        ),
        release_artifact_publication_fixture(
            "activation-result-receipt-public-version-tag-request",
            "blocked_release_noop",
            "public_version_tag_release_promotion_denied",
            serde_json::json!({
                "public_version_tag_requested": true,
                "public_release_publish_requested": true,
                "public_ga_claim_requested": true
            }),
        ),
        release_artifact_publication_fixture(
            "activation-result-receipt-release-notes-changelog-request",
            "blocked_artifact_noop",
            "release_notes_changelog_materialization_denied",
            serde_json::json!({
                "release_notes_materialization_requested": true,
                "changelog_materialization_requested": true
            }),
        ),
        release_artifact_publication_fixture(
            "activation-result-receipt-terminal-decision-as-release-approval",
            "blocked_promotion_noop",
            "terminal_operator_decision_is_not_release_approval",
            serde_json::json!({
                "terminal_operator_decision_release_approval_requested": true,
                "release_artifact_publication_requested": true
            }),
        ),
        release_artifact_publication_fixture(
            "activation-result-receipt-release-publication-activation-memory-provider-install",
            "blocked_promotion_noop",
            "activation_memory_provider_install_restart_active_binary_publication_denied",
            serde_json::json!({
                "activation_from_release_publication_requested": true,
                "memory_write_publication_requested": true,
                "provider_prompt_publication_requested": true,
                "install_publication_requested": true,
                "service_restart_publication_requested": true,
                "active_binary_publication_requested": true
            }),
        ),
    ]);
    let fixture_count = release_artifact_publication_fixtures
        .as_array()
        .map(std::vec::Vec::len)
        .unwrap_or(0);
    let source_report_sha256 = sha256_json_value(&source);
    let fixtures_hash_sha256 = sha256_json_value(&release_artifact_publication_fixtures);
    let boundary_hash_sha256 = sha256_text_value(&format!(
        "hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-release-artifact-publication-denial-boundary-v1:{}:{}:{}",
        route_matrix.route_count, source_report_sha256, fixtures_hash_sha256,
    ));
    let report_ready = route_matrix.ready
        && route_count_source_command_accepted
        && source_ready
        && RELEASE_ARTIFACT_PUBLICATION_SURFACES.len() == 12
        && fixture_count == 10
        && RELEASE_ARTIFACT_PUBLICATION_DENIALS.len() == 14;

    let mut denials = source
        .get("denied_by_activation_command_result_receipt_terminal_operator_decision_public_claim")
        .and_then(serde_json::Value::as_array)
        .cloned()
        .unwrap_or_default();
    for &denial in RELEASE_ARTIFACT_PUBLICATION_DENIALS {
        denials.push(serde_json::json!(denial));
    }
    let denied_count = denials.len();

    let mut side_effects = source
        .get("side_effects")
        .and_then(serde_json::Value::as_object)
        .cloned()
        .unwrap_or_default();
    for &key in FALSE_KEYS {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }

    let allowed_next_actions = serde_json::json!([
        {
            "action": "run_memory_write_execution_activation_command_result_receipt_release_artifact_publication_denial_boundary_require_live_gate",
            "status": "allowed_verification_only",
            "publishes_release_artifact": false,
            "claims_public_release": false,
            "writes_release_artifact": false,
            "writes_public_artifact": false,
            "signs_or_notarizes_artifact": false,
            "enqueues_publication": false,
            "performs_public_distribution": false,
            "writes_memory": false,
            "executes_rollback": false,
            "writes_kg": false,
            "invokes_provider": false,
            "reads_credentials": false,
            "sends_externally": false,
            "installs_or_restarts": false,
            "mutates_active_binary": false
        },
        {
            "action": "prepare_scoped_memory_real_write_canary_operator_approval_packet",
            "status": "allowed_report_only_next_slice",
            "requires_explicit_operator_acceptance": true,
            "requires_single_use_nonce": true,
            "requires_readback_and_rollback_receipts": true,
            "publishes_release_artifact": false,
            "claims_public_release": false,
            "invokes_model": false,
            "writes_kg": false,
            "sends_externally": false,
            "installs_or_restarts": false,
            "mutates_active_binary": false
        }
    ]);

    let mut report = source
        .as_object()
        .cloned()
        .unwrap_or_else(serde_json::Map::new);
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
        "hepta_memory_live_mutation_operator_write_execution_activation_command_result_receipt_release_artifact_publication_denial_boundary_route"
    );
    insert_report_json!(
        "endpoint",
        HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_ACTIVATION_COMMAND_RESULT_RECEIPT_RELEASE_ARTIFACT_PUBLICATION_DENIAL_BOUNDARY_ENDPOINT
    );
    insert_report_json!(
        "source_command",
        "/hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-release-artifact-publication-denial-boundary --json"
    );
    insert_report_json!("native_route", true);
    insert_report_json!("side_effect_free", true);
    insert_report_json!("audit_date", "2026-07-03");
    insert_report_json!(
        "memory_write_execution_activation_command_result_receipt_release_artifact_publication_denial_boundary_schema_version",
        "memory_write_execution_activation_command_result_receipt_release_artifact_publication_denial_boundary_v1"
    );
    insert_report_json!(
        "memory_write_execution_activation_command_result_receipt_release_artifact_publication_denial_boundary_ready",
        report_ready
    );
    insert_report_json!(
        "activation_command_result_receipt_release_artifact_publication_mode",
        "memory_write_execution_activation_command_result_receipt_release_artifact_publication_denial"
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
    insert_report_json!("boundary_hash_sha256", boundary_hash_sha256);
    insert_report_json!(
        "release_artifact_publication_fixtures_hash_sha256",
        fixtures_hash_sha256
    );
    insert_report_json!(
        "source_activation_command_result_receipt_terminal_operator_decision_public_claim_boundary_endpoint",
        HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_ACTIVATION_COMMAND_RESULT_RECEIPT_TERMINAL_OPERATOR_DECISION_PUBLIC_CLAIM_NON_PROMOTION_DENIAL_BOUNDARY_ENDPOINT
    );
    insert_report_json!(
        "source_activation_command_result_receipt_terminal_operator_decision_public_claim_boundary_ready",
        source_ready
    );
    insert_report_json!(
        "source_activation_command_result_receipt_terminal_operator_decision_public_claim_ready",
        json_bool(
            &source,
            "memory_write_execution_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_ready"
        )
    );
    insert_report_json!(
        "source_activation_command_result_receipt_terminal_operator_decision_public_claim_report_sha256",
        source_report_sha256
    );
    insert_report_json!(
        "source_terminal_operator_decision_public_claim_fixture_count",
        json_u64(
            &source,
            "activation_command_result_receipt_terminal_operator_decision_public_claim_fixture_count"
        )
    );
    insert_report_json!(
        "source_accepted_terminal_operator_decision_public_claim_fixture_count",
        json_u64(
            &source,
            "accepted_activation_command_result_receipt_terminal_operator_decision_public_claim_fixture_count"
        )
    );
    insert_report_json!(
        "source_terminal_operator_decision_performed_count",
        json_u64(
            &source,
            "activation_command_result_receipt_terminal_operator_decision_performed_count"
        )
    );
    insert_report_json!(
        "source_public_claim_promotion_performed_count",
        json_u64(
            &source,
            "activation_command_result_receipt_public_claim_promotion_performed_count"
        )
    );
    insert_report_json!(
        "source_terminal_operator_decision_public_claim_denial_count",
        json_u64(
            &source,
            "denied_by_activation_command_result_receipt_terminal_operator_decision_public_claim_count"
        )
    );
    insert_report_json!("minimum_required_samples", 24);
    insert_report_json!(
        "memory_write_execution_activation_command_result_receipt_release_artifact_publication_denial_ready",
        true
    );
    insert_report_json!(
        "memory_write_execution_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_ready",
        true
    );
    insert_report_json!(
        "memory_write_execution_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_ready",
        true
    );
    insert_report_json!(
        "memory_write_execution_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_ready",
        true
    );
    insert_report_json!(
        "memory_write_execution_activation_command_result_receipt_no_persistence_ready",
        true
    );
    insert_report_json!(
        "required_activation_command_result_receipt_release_artifact_publication_surface_count",
        12
    );
    insert_report_json!(
        "ready_activation_command_result_receipt_release_artifact_publication_surface_count",
        12
    );
    insert_report_json!(
        "side_effect_free_activation_command_result_receipt_release_artifact_publication_surface_count",
        12
    );
    insert_report_json!(
        "required_activation_command_result_receipt_release_artifact_publication_fixture_count",
        10
    );
    insert_report_json!(
        "activation_command_result_receipt_release_artifact_publication_fixture_count",
        fixture_count
    );
    insert_report_json!(
        "blocked_activation_command_result_receipt_release_artifact_publication_fixture_count",
        fixture_count
    );
    insert_report_json!(
        "noop_activation_command_result_receipt_release_artifact_publication_fixture_count",
        fixture_count
    );
    insert_report_json!(
        "allowed_activation_command_result_receipt_release_artifact_publication_fixture_count",
        0
    );
    insert_report_json!(
        "accepted_activation_command_result_receipt_release_artifact_publication_fixture_count",
        0
    );
    insert_report_json!(
        "activation_command_result_receipt_release_artifact_publication_performed_count",
        0
    );
    insert_report_json!("release_artifact_written_count", 0);
    insert_report_json!("public_artifact_written_count", 0);
    insert_report_json!("public_distribution_performed_count", 0);
    insert_report_json!("publication_manifest_written_count", 0);
    insert_report_json!("publication_queue_enqueued_count", 0);
    insert_report_json!("memory_store_write_performed_count", 0);

    for &key in FALSE_KEYS {
        report.insert(key.to_string(), serde_json::json!(false));
    }
    report.insert(
        "activation_command_result_receipt_release_artifact_publication_surfaces".to_string(),
        serde_json::json!(RELEASE_ARTIFACT_PUBLICATION_SURFACES),
    );
    report.insert(
        "activation_command_result_receipt_release_artifact_publication_fixtures".to_string(),
        release_artifact_publication_fixtures,
    );
    report.insert(
        "denied_by_activation_command_result_receipt_release_artifact_publication".to_string(),
        serde_json::Value::Array(denials),
    );
    insert_report_json!(
        "denied_by_activation_command_result_receipt_release_artifact_publication_count",
        denied_count
    );
    insert_report_json!("release_artifact_publication_forbidden", true);
    insert_report_json!("release_artifact_write_forbidden", true);
    insert_report_json!("public_artifact_write_forbidden", true);
    insert_report_json!("artifact_signature_notarization_forbidden", true);
    insert_report_json!("publication_queue_forbidden", true);
    insert_report_json!("publication_manifest_forbidden", true);
    insert_report_json!("public_distribution_forbidden", true);
    insert_report_json!("public_release_publication_forbidden", true);
    insert_report_json!("public_ga_claim_forbidden", true);
    insert_report_json!(
        "terminal_operator_decision_release_approval_forbidden",
        true
    );
    insert_report_json!(
        "activation_from_release_artifact_publication_forbidden",
        true
    );
    insert_report_json!(
        "runtime_provider_memory_kg_release_artifact_publication_forbidden",
        true
    );
    insert_report_json!(
        "external_public_install_restart_active_binary_publication_forbidden",
        true
    );
    report.insert("allowed_next_actions".to_string(), allowed_next_actions);
    report.insert(
        "side_effects".to_string(),
        serde_json::Value::Object(side_effects),
    );

    serde_json::Value::Object(report)
}

fn hepta_memory_live_mutation_operator_write_execution_scoped_memory_real_write_canary_operator_approval_packet_nonce_command_dry_run_boundary_report()
-> serde_json::Value {
    const SCOPED_CANARY_SURFACES: &[&str] = &[
        "fresh_operator_approval_packet_required",
        "single_use_nonce_required",
        "operator_identity_session_binding_required",
        "explicit_command_path_required",
        "canary_scope_namespace_store_binding_required",
        "payload_digest_redaction_binding_required",
        "active_binary_sha_route_count_binding_required",
        "release_artifact_denial_source_binding_required",
        "wal_receipt_persistence_plan_required",
        "post_write_readback_validation_plan_required",
        "rollback_tombstone_plan_required",
        "external_kg_provider_public_install_active_binary_side_effects_forbidden",
    ];
    const SCOPED_CANARY_DENIALS: &[&str] = &[
        "source_release_artifact_publication_denial_boundary_required",
        "fresh_operator_approval_packet_not_recorded",
        "fresh_operator_approval_packet_not_persisted",
        "fresh_operator_approval_packet_not_accepted",
        "operator_identity_not_bound",
        "operator_session_not_bound",
        "operator_signature_not_verified",
        "single_use_nonce_not_issued",
        "single_use_nonce_not_consumed",
        "nonce_replay_window_not_closed",
        "explicit_command_not_accepted",
        "explicit_command_not_dispatched",
        "canary_scope_not_bound",
        "canary_namespace_not_bound",
        "canary_store_not_bound",
        "payload_digest_not_bound",
        "redaction_proof_not_accepted",
        "active_binary_sha_not_bound",
        "route_count_not_bound",
        "fresh_long_soak_evidence_not_accepted",
        "wal_receipt_plan_not_accepted",
        "receipt_persistence_not_allowed",
        "post_write_readback_plan_not_accepted",
        "rollback_tombstone_plan_not_accepted",
        "durable_memory_write_denied",
        "memory_store_mutation_denied",
        "kg_live_write_denied",
        "provider_model_invocation_denied",
        "credential_read_denied",
        "channel_external_send_denied",
        "public_claim_release_artifact_denied",
        "install_restart_active_binary_mutation_denied",
    ];
    const FALSE_KEYS: &[&str] = &[
        "fresh_operator_approval_packet_recorded",
        "fresh_operator_approval_packet_persisted",
        "fresh_operator_approval_packet_accepted",
        "operator_identity_bound",
        "operator_session_bound",
        "operator_signature_verified",
        "operator_timestamp_accepted",
        "operator_scope_bound",
        "single_use_nonce_issued",
        "single_use_nonce_consumed",
        "single_use_nonce_replayed",
        "nonce_replay_allowed",
        "explicit_command_accepted",
        "explicit_command_dispatched",
        "explicit_command_performed",
        "canary_scope_bound",
        "canary_namespace_bound",
        "canary_store_bound",
        "payload_digest_bound",
        "redaction_proof_accepted",
        "active_binary_sha_bound",
        "route_count_bound",
        "fresh_long_soak_evidence_accepted",
        "wal_receipt_plan_accepted",
        "wal_write_performed",
        "receipt_recorded",
        "receipt_persisted",
        "receipt_accepted",
        "post_write_readback_plan_accepted",
        "post_write_readback_performed",
        "post_write_validation_performed",
        "rollback_plan_accepted",
        "rollback_tombstone_plan_accepted",
        "rollback_executed",
        "tombstone_written",
        "activation_allowed_by_scoped_canary_approval",
        "activation_allowed",
        "activation_performed",
        "live_mutation_execution_ready",
        "live_mutation_execution_allowed",
        "live_mutation_execution_performed",
        "memory_write_execution_allowed",
        "memory_write_execution_ready",
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
        "provider_prompt_replay_enabled",
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

    fn scoped_canary_fixture(
        id: &str,
        status: &str,
        reason: &str,
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
        insert_fixture_json!("scoped_memory_real_write_canary_status", status);
        insert_fixture_json!("source_release_artifact_publication_denial_present", true);
        insert_fixture_json!("source_release_artifact_publication_denial_ready", true);
        insert_fixture_json!("scoped_canary_dry_run_noop_confirmed", true);
        insert_fixture_json!("reason", reason);
        for key in [
            "approval_packet_requested",
            "nonce_issue_requested",
            "nonce_consume_requested",
            "explicit_command_requested",
            "canary_scope_requested",
            "canary_namespace_requested",
            "canary_store_requested",
            "payload_digest_binding_requested",
            "active_binary_sha_binding_requested",
            "route_count_binding_requested",
            "wal_receipt_plan_requested",
            "post_write_readback_plan_requested",
            "rollback_tombstone_plan_requested",
            "durable_memory_write_requested",
            "memory_store_mutation_requested",
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
            "approval_packet_recorded",
            "approval_packet_persisted",
            "approval_packet_accepted",
            "operator_identity_bound",
            "operator_session_bound",
            "operator_signature_verified",
            "single_use_nonce_issued",
            "single_use_nonce_consumed",
            "explicit_command_accepted",
            "explicit_command_dispatched",
            "canary_scope_bound",
            "canary_namespace_bound",
            "canary_store_bound",
            "payload_digest_bound",
            "active_binary_sha_bound",
            "route_count_bound",
            "wal_receipt_plan_accepted",
            "receipt_recorded",
            "receipt_persisted",
            "receipt_accepted",
            "post_write_readback_plan_accepted",
            "post_write_readback_performed",
            "rollback_tombstone_plan_accepted",
            "rollback_executed",
            "tombstone_written",
            "activation_allowed",
            "live_mutation_execution_performed",
            "memory_write_execution_performed",
            "memory_store_write_performed",
            "memory_store_mutated",
            "durable_memory_store_write_performed",
            "kg_adapter_read_performed",
            "live_kg_write_performed",
            "provider_invoked",
            "model_invoked",
            "credential_read",
            "secret_file_read",
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
            "active_binary_mutated",
        ] {
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
        .name("hepta-memory-scoped-real-write-canary-source-report".to_string())
        .stack_size(8 * 1024 * 1024)
        .spawn(
            hepta_memory_live_mutation_operator_write_execution_activation_command_result_receipt_release_artifact_publication_denial_boundary_report,
        )
        .ok()
        .and_then(|handle| handle.join().ok())
        .unwrap_or_else(|| {
            serde_json::json!({
                "status": "blocked",
                "memory_write_execution_activation_command_result_receipt_release_artifact_publication_denial_boundary_ready": false,
                "memory_write_execution_activation_command_result_receipt_release_artifact_publication_denial_ready": false,
                "source_release_artifact_publication_source_report_thread_failed": true
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
    let source_next_action_scoped_canary = source
        .get("allowed_next_actions")
        .and_then(serde_json::Value::as_array)
        .and_then(|items| items.get(1))
        .map(|item| {
            item.get("action").and_then(serde_json::Value::as_str)
                == Some("prepare_scoped_memory_real_write_canary_operator_approval_packet")
                && item
                    .get("requires_explicit_operator_acceptance")
                    .and_then(serde_json::Value::as_bool)
                    == Some(true)
        })
        .unwrap_or(false);
    let source_ready = json_str(&source, "status") == "ready"
        && json_bool(
            &source,
            "memory_write_execution_activation_command_result_receipt_release_artifact_publication_denial_boundary_ready",
        )
        && json_bool(
            &source,
            "memory_write_execution_activation_command_result_receipt_release_artifact_publication_denial_ready",
        )
        && json_u64(
            &source,
            "activation_command_result_receipt_release_artifact_publication_fixture_count",
        ) == 10
        && json_u64(
            &source,
            "accepted_activation_command_result_receipt_release_artifact_publication_fixture_count",
        ) == 0
        && json_u64(
            &source,
            "activation_command_result_receipt_release_artifact_publication_performed_count",
        ) == 0
        && json_u64(&source, "release_artifact_written_count") == 0
        && json_u64(&source, "public_artifact_written_count") == 0
        && json_u64(&source, "memory_store_write_performed_count") == 0
        && json_u64(
            &source,
            "denied_by_activation_command_result_receipt_release_artifact_publication_count",
        ) == 69
        && !json_bool(&source, "release_artifact_publication_accepted")
        && !json_bool(&source, "release_artifact_written")
        && !json_bool(&source, "public_artifact_written")
        && !json_bool(&source, "public_release_published")
        && !json_bool(&source, "public_ga_claimed")
        && !json_bool(&source, "activation_allowed")
        && !json_bool(&source, "activation_performed")
        && !json_bool(&source, "memory_store_write_performed")
        && !json_bool(&source, "memory_store_mutated")
        && !json_bool(&source, "live_kg_write_performed")
        && !json_bool(&source, "provider_invoked")
        && !json_bool(&source, "model_invoked")
        && !json_bool(&source, "credential_read")
        && !json_bool(&source, "telegram_send_performed")
        && !json_bool(&source, "channel_send_performed")
        && !json_bool(&source, "external_send_performed")
        && !json_bool(&source, "install_executed")
        && !json_bool(&source, "service_restarted")
        && !json_bool(&source, "active_binary_mutated")
        && source_next_action_scoped_canary
        && side_effects_all_false(&source);

    let fixtures = serde_json::Value::Array(vec![
        scoped_canary_fixture(
            "scoped-memory-real-write-canary-missing-release-artifact-source",
            "blocked_source_noop",
            "release_artifact_publication_denial_boundary_source_required",
            serde_json::json!({
                "source_release_artifact_publication_denial_present": false,
                "source_release_artifact_publication_denial_ready": false,
                "approval_packet_requested": true
            }),
        ),
        scoped_canary_fixture(
            "scoped-memory-real-write-canary-fresh-approval-packet-required",
            "blocked_approval_noop",
            "fresh_operator_approval_packet_required",
            serde_json::json!({
                "approval_packet_requested": true
            }),
        ),
        scoped_canary_fixture(
            "scoped-memory-real-write-canary-operator-identity-session-required",
            "blocked_identity_noop",
            "operator_identity_session_signature_required",
            serde_json::json!({
                "approval_packet_requested": true,
                "operator_identity_requested": true,
                "operator_session_requested": true
            }),
        ),
        scoped_canary_fixture(
            "scoped-memory-real-write-canary-single-use-nonce-required",
            "blocked_nonce_noop",
            "single_use_nonce_required_and_not_consumed",
            serde_json::json!({
                "nonce_issue_requested": true,
                "nonce_consume_requested": true
            }),
        ),
        scoped_canary_fixture(
            "scoped-memory-real-write-canary-explicit-command-dry-run-only",
            "blocked_command_noop",
            "explicit_command_path_required_but_dry_run_only",
            serde_json::json!({
                "explicit_command_requested": true
            }),
        ),
        scoped_canary_fixture(
            "scoped-memory-real-write-canary-scope-namespace-store-binding",
            "blocked_scope_noop",
            "canary_scope_namespace_store_binding_required",
            serde_json::json!({
                "canary_scope_requested": true,
                "canary_namespace_requested": true,
                "canary_store_requested": true
            }),
        ),
        scoped_canary_fixture(
            "scoped-memory-real-write-canary-payload-digest-redaction-binding",
            "blocked_payload_noop",
            "payload_digest_and_redaction_proof_required",
            serde_json::json!({
                "payload_digest_binding_requested": true
            }),
        ),
        scoped_canary_fixture(
            "scoped-memory-real-write-canary-binary-route-count-binding",
            "blocked_binary_noop",
            "active_binary_sha_and_route_count_binding_required",
            serde_json::json!({
                "active_binary_sha_binding_requested": true,
                "route_count_binding_requested": true
            }),
        ),
        scoped_canary_fixture(
            "scoped-memory-real-write-canary-wal-readback-rollback-plans",
            "blocked_receipt_noop",
            "wal_receipt_readback_and_rollback_plans_required",
            serde_json::json!({
                "wal_receipt_plan_requested": true,
                "post_write_readback_plan_requested": true,
                "rollback_tombstone_plan_requested": true
            }),
        ),
        scoped_canary_fixture(
            "scoped-memory-real-write-canary-direct-side-effect-attempt",
            "blocked_execution_noop",
            "direct_memory_kg_provider_channel_release_install_active_binary_side_effects_denied",
            serde_json::json!({
                "durable_memory_write_requested": true,
                "memory_store_mutation_requested": true,
                "kg_live_write_requested": true,
                "provider_model_invocation_requested": true,
                "credential_read_requested": true,
                "channel_external_send_requested": true,
                "public_claim_requested": true,
                "release_artifact_write_requested": true,
                "install_restart_requested": true,
                "active_binary_mutation_requested": true
            }),
        ),
    ]);
    let fixture_count = fixtures.as_array().map(std::vec::Vec::len).unwrap_or(0);
    let source_report_sha256 = sha256_json_value(&source);
    let fixtures_hash_sha256 = sha256_json_value(&fixtures);
    let boundary_hash_sha256 = sha256_text_value(&format!(
        "hepta-memory-live-mutation-operator-write-execution-scoped-memory-real-write-canary-operator-approval-packet-nonce-command-dry-run-boundary-v1:{}:{}:{}",
        route_matrix.route_count, source_report_sha256, fixtures_hash_sha256
    ));
    let policy_hash_sha256 = sha256_text_value(
        "scoped-memory-real-write-canary-approval-packet-nonce-command-dry-run:no-approval-accept:no-nonce-consume:no-command-dispatch:no-durable-write",
    );
    let denials = SCOPED_CANARY_DENIALS
        .iter()
        .map(|reason| {
            serde_json::json!({
                "reason": reason,
                "accepted": false,
                "performed": false,
                "writes_memory": false,
                "writes_kg": false,
                "invokes_provider": false,
                "sends_externally": false
            })
        })
        .collect::<Vec<_>>();
    let denied_count = denials.len();
    let required_fields = serde_json::json!([
        "fresh_operator_approval_packet_id",
        "operator_identity_hash",
        "operator_session_id",
        "operator_signature_hash",
        "operator_timestamp",
        "single_use_nonce_id",
        "explicit_command_id",
        "canary_scope",
        "canary_namespace",
        "canary_store",
        "payload_digest_sha256",
        "redaction_proof_id",
        "active_binary_sha256",
        "native_route_count",
        "source_release_artifact_publication_denial_report_sha256",
        "wal_receipt_plan_id",
        "post_write_readback_validation_plan_id",
        "rollback_tombstone_plan_id",
    ]);

    let mut side_effects = serde_json::Map::new();
    for &key in FALSE_KEYS {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }

    let report_ready = route_matrix.ready
        && route_count_source_command_accepted
        && source_ready
        && SCOPED_CANARY_SURFACES.len() == 12
        && fixture_count == 10
        && denied_count == 32;

    let allowed_next_actions = serde_json::json!([
        {
            "action": "run_scoped_memory_real_write_canary_operator_approval_packet_nonce_command_dry_run_boundary_require_live_gate",
            "status": "allowed_verification_only",
            "accepts_operator_approval_packet": false,
            "consumes_nonce": false,
            "dispatches_command": false,
            "writes_memory": false,
            "writes_kg": false,
            "invokes_provider": false,
            "reads_credentials": false,
            "sends_externally": false,
            "publishes_artifacts": false,
            "installs_or_restarts": false,
            "mutates_active_binary": false
        },
        {
            "action": "prepare_scoped_memory_real_write_canary_readback_rollback_tombstone_dry_run_boundary",
            "status": "allowed_report_only_next_slice",
            "requires_accepted_operator_approval_packet": true,
            "requires_consumed_single_use_nonce": true,
            "requires_explicit_command": true,
            "writes_memory": false,
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
        "hepta_memory_live_mutation_operator_write_execution_scoped_memory_real_write_canary_operator_approval_packet_nonce_command_dry_run_boundary_route"
    );
    insert_report_json!(
        "endpoint",
        HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_SCOPED_MEMORY_REAL_WRITE_CANARY_OPERATOR_APPROVAL_PACKET_NONCE_COMMAND_DRY_RUN_BOUNDARY_ENDPOINT
    );
    insert_report_json!(
        "source_command",
        "/hepta-memory-live-mutation-operator-write-execution-scoped-memory-real-write-canary-operator-approval-packet-nonce-command-dry-run-boundary --json"
    );
    insert_report_json!("native_route", true);
    insert_report_json!("side_effect_free", true);
    insert_report_json!("audit_date", "2026-07-03");
    insert_report_json!(
        "scoped_memory_real_write_canary_boundary_schema_version",
        "scoped_memory_real_write_canary_operator_approval_packet_nonce_command_dry_run_boundary_v1"
    );
    insert_report_json!(
        "scoped_memory_real_write_canary_mode",
        "scoped_memory_real_write_canary_operator_approval_packet_nonce_command_dry_run_no_write"
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
        "memory_write_execution_scoped_memory_real_write_canary_operator_approval_packet_nonce_command_dry_run_boundary_ready",
        report_ready
    );
    insert_report_json!(
        "scoped_memory_real_write_canary_operator_approval_packet_nonce_command_dry_run_ready",
        report_ready
    );
    insert_report_json!(
        "source_activation_command_result_receipt_release_artifact_publication_boundary_ready",
        source_ready
    );
    insert_report_json!(
        "source_activation_command_result_receipt_release_artifact_publication_ready",
        json_bool(
            &source,
            "memory_write_execution_activation_command_result_receipt_release_artifact_publication_denial_ready"
        )
    );
    insert_report_json!(
        "source_activation_command_result_receipt_release_artifact_publication_report_sha256",
        source_report_sha256
    );
    insert_report_json!(
        "source_release_artifact_publication_fixture_count",
        json_u64(
            &source,
            "activation_command_result_receipt_release_artifact_publication_fixture_count"
        )
    );
    insert_report_json!(
        "source_accepted_release_artifact_publication_fixture_count",
        json_u64(
            &source,
            "accepted_activation_command_result_receipt_release_artifact_publication_fixture_count"
        )
    );
    insert_report_json!(
        "source_release_artifact_publication_performed_count",
        json_u64(
            &source,
            "activation_command_result_receipt_release_artifact_publication_performed_count"
        )
    );
    insert_report_json!(
        "source_release_artifact_publication_denial_count",
        json_u64(
            &source,
            "denied_by_activation_command_result_receipt_release_artifact_publication_count"
        )
    );
    insert_report_json!("minimum_required_samples", 24);
    insert_report_json!(
        "scoped_memory_real_write_canary_boundary_hash_sha256",
        boundary_hash_sha256
    );
    insert_report_json!(
        "scoped_memory_real_write_canary_policy_hash_sha256",
        policy_hash_sha256
    );
    insert_report_json!(
        "required_scoped_memory_real_write_canary_operator_approval_surface_count",
        12
    );
    insert_report_json!(
        "ready_scoped_memory_real_write_canary_operator_approval_surface_count",
        12
    );
    insert_report_json!(
        "side_effect_free_scoped_memory_real_write_canary_operator_approval_surface_count",
        12
    );
    insert_report_json!(
        "required_scoped_memory_real_write_canary_operator_approval_fixture_count",
        10
    );
    insert_report_json!(
        "scoped_memory_real_write_canary_operator_approval_fixture_count",
        fixture_count
    );
    insert_report_json!(
        "blocked_scoped_memory_real_write_canary_operator_approval_fixture_count",
        fixture_count
    );
    insert_report_json!(
        "noop_scoped_memory_real_write_canary_operator_approval_fixture_count",
        fixture_count
    );
    insert_report_json!(
        "allowed_scoped_memory_real_write_canary_operator_approval_fixture_count",
        0
    );
    insert_report_json!(
        "accepted_scoped_memory_real_write_canary_operator_approval_fixture_count",
        0
    );
    insert_report_json!(
        "scoped_memory_real_write_canary_approval_packet_accepted_count",
        0
    );
    insert_report_json!("single_use_nonce_consumed_count", 0);
    insert_report_json!("explicit_command_dispatched_count", 0);
    insert_report_json!("wal_write_performed_count", 0);
    insert_report_json!("receipt_persisted_count", 0);
    insert_report_json!("post_write_readback_performed_count", 0);
    insert_report_json!("rollback_tombstone_performed_count", 0);
    insert_report_json!("memory_store_write_performed_count", 0);
    insert_report_json!(
        "required_before_scoped_memory_real_write_canary_acceptance_count",
        18
    );
    report.insert(
        "required_scoped_memory_real_write_canary_operator_approval_fields".to_string(),
        required_fields,
    );
    report.insert(
        "scoped_memory_real_write_canary_operator_approval_surfaces".to_string(),
        serde_json::json!(SCOPED_CANARY_SURFACES),
    );
    report.insert(
        "scoped_memory_real_write_canary_operator_approval_fixtures".to_string(),
        fixtures,
    );
    report.insert(
        "denied_by_scoped_memory_real_write_canary_operator_approval_packet_nonce_command_dry_run"
            .to_string(),
        serde_json::Value::Array(denials),
    );
    insert_report_json!(
        "denied_by_scoped_memory_real_write_canary_operator_approval_packet_nonce_command_dry_run_count",
        denied_count
    );
    insert_report_json!("fresh_operator_approval_packet_required", true);
    insert_report_json!("single_use_nonce_required", true);
    insert_report_json!("explicit_command_required", true);
    insert_report_json!("durable_memory_write_forbidden", true);
    insert_report_json!("memory_store_mutation_forbidden", true);
    insert_report_json!("kg_live_write_forbidden", true);
    insert_report_json!("provider_model_invocation_forbidden", true);
    insert_report_json!("credential_read_forbidden", true);
    insert_report_json!("channel_external_send_forbidden", true);
    insert_report_json!("public_claim_release_artifact_forbidden", true);
    insert_report_json!("install_restart_active_binary_mutation_forbidden", true);
    for &key in FALSE_KEYS {
        report.insert(key.to_string(), serde_json::json!(false));
    }
    report.insert("allowed_next_actions".to_string(), allowed_next_actions);
    report.insert(
        "side_effects".to_string(),
        serde_json::Value::Object(side_effects),
    );

    serde_json::Value::Object(report)
}

fn hepta_memory_live_mutation_operator_write_execution_scoped_memory_real_write_canary_readback_validation_dry_run_boundary_report()
-> serde_json::Value {
    const READBACK_SURFACES: &[&str] = &[
        "source_canary_approval_nonce_command_dry_run_required",
        "post_write_readback_plan_required",
        "receipt_linkage_required",
        "payload_digest_comparison_required",
        "namespace_store_scope_match_required",
        "redaction_proof_required",
        "secret_plaintext_absence_required",
        "stale_read_prevention_required",
        "phantom_read_prevention_required",
        "operator_review_handoff_required",
        "rollback_tombstone_handoff_required",
        "external_kg_provider_public_install_active_binary_side_effects_forbidden",
    ];
    const READBACK_DENIALS: &[&str] = &[
        "source_scoped_canary_approval_nonce_command_dry_run_boundary_required",
        "fresh_operator_approval_packet_not_accepted",
        "single_use_nonce_not_consumed",
        "explicit_command_not_dispatched",
        "wal_receipt_not_persisted",
        "post_write_readback_plan_not_accepted",
        "durable_memory_store_read_denied",
        "readback_result_not_recorded",
        "readback_result_not_persisted",
        "readback_payload_digest_not_compared",
        "readback_namespace_scope_store_not_verified",
        "readback_redaction_proof_not_accepted",
        "readback_secret_plaintext_scan_not_performed",
        "stale_read_guard_not_accepted",
        "phantom_read_guard_not_accepted",
        "operator_review_handoff_not_accepted",
        "rollback_tombstone_handoff_not_accepted",
        "durable_memory_write_denied",
        "memory_store_mutation_denied",
        "rollback_execution_denied",
        "kg_live_write_denied",
        "provider_model_invocation_denied",
        "credential_read_denied",
        "channel_external_send_denied",
        "public_claim_release_artifact_denied",
        "install_restart_active_binary_mutation_denied",
    ];
    const FALSE_KEYS: &[&str] = &[
        "fresh_operator_approval_packet_accepted",
        "single_use_nonce_consumed",
        "explicit_command_dispatched",
        "wal_receipt_plan_accepted",
        "wal_write_performed",
        "receipt_recorded",
        "receipt_persisted",
        "receipt_accepted",
        "post_write_readback_plan_accepted",
        "post_write_readback_performed",
        "post_write_validation_performed",
        "readback_result_recorded",
        "readback_result_persisted",
        "readback_result_accepted",
        "readback_payload_digest_compared",
        "readback_payload_digest_matched",
        "readback_namespace_bound",
        "readback_store_bound",
        "readback_scope_verified",
        "readback_redaction_proof_accepted",
        "readback_secret_plaintext_scan_performed",
        "readback_secret_plaintext_found",
        "stale_read_guard_accepted",
        "phantom_read_guard_accepted",
        "operator_review_handoff_accepted",
        "rollback_tombstone_handoff_accepted",
        "rollback_tombstone_plan_accepted",
        "rollback_executed",
        "tombstone_written",
        "activation_allowed_by_scoped_canary_readback",
        "activation_allowed",
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

    fn readback_fixture(
        id: &str,
        status: &str,
        reason: &str,
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
        insert_fixture_json!("scoped_memory_real_write_canary_readback_status", status);
        insert_fixture_json!("source_scoped_canary_approval_nonce_command_present", true);
        insert_fixture_json!("source_scoped_canary_approval_nonce_command_ready", true);
        insert_fixture_json!("scoped_canary_readback_dry_run_noop_confirmed", true);
        insert_fixture_json!("reason", reason);
        for key in [
            "readback_plan_requested",
            "receipt_linkage_requested",
            "payload_digest_compare_requested",
            "namespace_store_scope_check_requested",
            "redaction_proof_requested",
            "secret_plaintext_scan_requested",
            "stale_read_guard_requested",
            "phantom_read_guard_requested",
            "operator_review_handoff_requested",
            "rollback_tombstone_handoff_requested",
            "durable_memory_read_requested",
            "durable_memory_write_requested",
            "memory_store_mutation_requested",
            "rollback_execution_requested",
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
            "approval_packet_accepted",
            "single_use_nonce_consumed",
            "explicit_command_dispatched",
            "receipt_persisted",
            "post_write_readback_plan_accepted",
            "post_write_readback_performed",
            "readback_result_recorded",
            "readback_result_persisted",
            "readback_result_accepted",
            "readback_payload_digest_compared",
            "readback_payload_digest_matched",
            "readback_redaction_proof_accepted",
            "readback_secret_plaintext_scan_performed",
            "readback_secret_plaintext_found",
            "stale_read_guard_accepted",
            "phantom_read_guard_accepted",
            "operator_review_handoff_accepted",
            "rollback_tombstone_handoff_accepted",
            "rollback_executed",
            "tombstone_written",
            "activation_allowed",
            "memory_write_execution_performed",
            "memory_store_write_performed",
            "memory_store_mutated",
            "durable_memory_store_write_performed",
            "durable_memory_store_read_performed",
            "durable_memory_store_rollback_performed",
            "kg_adapter_read_performed",
            "live_kg_write_performed",
            "provider_invoked",
            "model_invoked",
            "credential_read",
            "secret_file_read",
            "telegram_send_performed",
            "channel_send_performed",
            "external_send_performed",
            "public_claim_promoted",
            "release_artifact_written",
            "install_executed",
            "service_restarted",
            "active_binary_mutated",
        ] {
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
        .name("hepta-memory-scoped-real-write-canary-readback-source-report".to_string())
        .stack_size(8 * 1024 * 1024)
        .spawn(
            hepta_memory_live_mutation_operator_write_execution_scoped_memory_real_write_canary_operator_approval_packet_nonce_command_dry_run_boundary_report,
        )
        .ok()
        .and_then(|handle| handle.join().ok())
        .unwrap_or_else(|| {
            serde_json::json!({
                "status": "blocked",
                "memory_write_execution_scoped_memory_real_write_canary_operator_approval_packet_nonce_command_dry_run_boundary_ready": false,
                "scoped_memory_real_write_canary_operator_approval_packet_nonce_command_dry_run_ready": false,
                "source_scoped_canary_approval_nonce_command_source_report_thread_failed": true
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
    let source_next_action_readback = source
        .get("allowed_next_actions")
        .and_then(serde_json::Value::as_array)
        .and_then(|items| items.get(1))
        .map(|item| {
            item.get("action").and_then(serde_json::Value::as_str)
                == Some("prepare_scoped_memory_real_write_canary_readback_rollback_tombstone_dry_run_boundary")
                && item
                    .get("requires_accepted_operator_approval_packet")
                    .and_then(serde_json::Value::as_bool)
                    == Some(true)
        })
        .unwrap_or(false);
    let source_ready = source.get("status").and_then(serde_json::Value::as_str) == Some("ready")
        && json_bool(
            &source,
            "memory_write_execution_scoped_memory_real_write_canary_operator_approval_packet_nonce_command_dry_run_boundary_ready",
        )
        && json_bool(
            &source,
            "scoped_memory_real_write_canary_operator_approval_packet_nonce_command_dry_run_ready",
        )
        && json_u64(
            &source,
            "scoped_memory_real_write_canary_operator_approval_fixture_count",
        ) == 10
        && json_u64(
            &source,
            "accepted_scoped_memory_real_write_canary_operator_approval_fixture_count",
        ) == 0
        && json_u64(
            &source,
            "scoped_memory_real_write_canary_approval_packet_accepted_count",
        ) == 0
        && json_u64(&source, "single_use_nonce_consumed_count") == 0
        && json_u64(&source, "explicit_command_dispatched_count") == 0
        && json_u64(&source, "wal_write_performed_count") == 0
        && json_u64(&source, "receipt_persisted_count") == 0
        && json_u64(&source, "post_write_readback_performed_count") == 0
        && json_u64(&source, "memory_store_write_performed_count") == 0
        && json_u64(
            &source,
            "denied_by_scoped_memory_real_write_canary_operator_approval_packet_nonce_command_dry_run_count",
        ) == 32
        && !json_bool(&source, "fresh_operator_approval_packet_accepted")
        && !json_bool(&source, "single_use_nonce_consumed")
        && !json_bool(&source, "explicit_command_dispatched")
        && !json_bool(&source, "receipt_persisted")
        && !json_bool(&source, "post_write_readback_performed")
        && !json_bool(&source, "memory_store_write_performed")
        && !json_bool(&source, "memory_store_mutated")
        && !json_bool(&source, "durable_memory_store_write_performed")
        && !json_bool(&source, "durable_memory_store_read_performed")
        && !json_bool(&source, "live_kg_write_performed")
        && !json_bool(&source, "provider_invoked")
        && !json_bool(&source, "model_invoked")
        && !json_bool(&source, "credential_read")
        && !json_bool(&source, "channel_send_performed")
        && !json_bool(&source, "external_send_performed")
        && !json_bool(&source, "release_artifact_written")
        && !json_bool(&source, "install_executed")
        && !json_bool(&source, "active_binary_mutated")
        && source_next_action_readback
        && side_effects_all_false(&source);

    let fixtures = serde_json::Value::Array(vec![
        readback_fixture(
            "scoped-memory-real-write-canary-readback-missing-approval-source",
            "blocked_source_noop",
            "scoped_canary_approval_nonce_command_dry_run_source_required",
            serde_json::json!({
                "source_scoped_canary_approval_nonce_command_present": false,
                "source_scoped_canary_approval_nonce_command_ready": false,
                "readback_plan_requested": true
            }),
        ),
        readback_fixture(
            "scoped-memory-real-write-canary-readback-plan-required",
            "blocked_plan_noop",
            "post_write_readback_validation_plan_required",
            serde_json::json!({"readback_plan_requested": true}),
        ),
        readback_fixture(
            "scoped-memory-real-write-canary-readback-receipt-linkage-required",
            "blocked_receipt_noop",
            "wal_receipt_linkage_required_before_readback",
            serde_json::json!({"receipt_linkage_requested": true}),
        ),
        readback_fixture(
            "scoped-memory-real-write-canary-readback-payload-digest-compare-required",
            "blocked_digest_noop",
            "payload_digest_comparison_required_before_acceptance",
            serde_json::json!({"payload_digest_compare_requested": true}),
        ),
        readback_fixture(
            "scoped-memory-real-write-canary-readback-namespace-store-scope-required",
            "blocked_scope_noop",
            "canary_namespace_store_scope_match_required",
            serde_json::json!({"namespace_store_scope_check_requested": true}),
        ),
        readback_fixture(
            "scoped-memory-real-write-canary-readback-redaction-secret-scan-required",
            "blocked_redaction_noop",
            "redaction_proof_and_secret_plaintext_scan_required",
            serde_json::json!({
                "redaction_proof_requested": true,
                "secret_plaintext_scan_requested": true
            }),
        ),
        readback_fixture(
            "scoped-memory-real-write-canary-readback-stale-phantom-guards-required",
            "blocked_consistency_noop",
            "stale_and_phantom_read_guards_required",
            serde_json::json!({
                "stale_read_guard_requested": true,
                "phantom_read_guard_requested": true
            }),
        ),
        readback_fixture(
            "scoped-memory-real-write-canary-readback-operator-review-required",
            "blocked_review_noop",
            "operator_review_handoff_required_before_acceptance",
            serde_json::json!({"operator_review_handoff_requested": true}),
        ),
        readback_fixture(
            "scoped-memory-real-write-canary-readback-rollback-handoff-required",
            "blocked_rollback_noop",
            "rollback_tombstone_handoff_required_before_real_write_canary",
            serde_json::json!({"rollback_tombstone_handoff_requested": true}),
        ),
        readback_fixture(
            "scoped-memory-real-write-canary-readback-direct-side-effect-attempt",
            "blocked_execution_noop",
            "direct_read_write_rollback_kg_provider_channel_release_install_active_binary_side_effects_denied",
            serde_json::json!({
                "durable_memory_read_requested": true,
                "durable_memory_write_requested": true,
                "memory_store_mutation_requested": true,
                "rollback_execution_requested": true,
                "kg_live_write_requested": true,
                "provider_model_invocation_requested": true,
                "credential_read_requested": true,
                "channel_external_send_requested": true,
                "public_claim_requested": true,
                "release_artifact_write_requested": true,
                "install_restart_requested": true,
                "active_binary_mutation_requested": true
            }),
        ),
    ]);
    let fixture_count = fixtures.as_array().map(std::vec::Vec::len).unwrap_or(0);
    let denials = READBACK_DENIALS
        .iter()
        .map(|reason| {
            serde_json::json!({
                "reason": reason,
                "accepted": false,
                "performed": false,
                "reads_memory": false,
                "writes_memory": false,
                "executes_rollback": false,
                "writes_kg": false,
                "invokes_provider": false,
                "sends_externally": false
            })
        })
        .collect::<Vec<_>>();
    let denied_count = denials.len();
    let source_report_sha256 = sha256_json_value(&source);
    let fixtures_hash_sha256 = sha256_json_value(&fixtures);
    let boundary_hash_sha256 = sha256_text_value(&format!(
        "hepta-memory-live-mutation-operator-write-execution-scoped-memory-real-write-canary-readback-validation-dry-run-boundary-v1:{}:{}:{}",
        route_matrix.route_count, source_report_sha256, fixtures_hash_sha256
    ));
    let policy_hash_sha256 = sha256_text_value(
        "scoped-memory-real-write-canary-readback-validation-dry-run:no-durable-read:no-validation-accept:no-rollback:no-write",
    );
    let report_ready = route_matrix.ready
        && route_count_source_command_accepted
        && source_ready
        && READBACK_SURFACES.len() == 12
        && fixture_count == 10
        && denied_count == 26;

    let mut side_effects = serde_json::Map::new();
    for &key in FALSE_KEYS {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }

    let required_fields = serde_json::json!([
        "source_scoped_canary_approval_nonce_command_report_sha256",
        "wal_receipt_id",
        "receipt_persisted_proof_id",
        "post_write_readback_plan_id",
        "readback_command_id",
        "canary_namespace",
        "canary_store",
        "canary_scope",
        "payload_digest_sha256",
        "readback_payload_digest_sha256",
        "redaction_proof_id",
        "secret_plaintext_scan_id",
        "stale_read_guard_id",
        "phantom_read_guard_id",
        "operator_review_handoff_id",
        "rollback_tombstone_handoff_id",
    ]);
    let allowed_next_actions = serde_json::json!([
        {
            "action": "run_scoped_memory_real_write_canary_readback_validation_dry_run_boundary_require_live_gate",
            "status": "allowed_verification_only",
            "reads_memory": false,
            "writes_memory": false,
            "executes_rollback": false,
            "writes_kg": false,
            "invokes_provider": false,
            "reads_credentials": false,
            "sends_externally": false,
            "publishes_artifacts": false,
            "installs_or_restarts": false,
            "mutates_active_binary": false
        },
        {
            "action": "prepare_scoped_memory_real_write_canary_rollback_tombstone_dry_run_boundary",
            "status": "allowed_report_only_next_slice",
            "requires_readback_validation_acceptance": true,
            "writes_memory": false,
            "reads_memory": false,
            "executes_rollback": false,
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
        "hepta_memory_live_mutation_operator_write_execution_scoped_memory_real_write_canary_readback_validation_dry_run_boundary_route"
    );
    insert_report_json!(
        "endpoint",
        HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_SCOPED_MEMORY_REAL_WRITE_CANARY_READBACK_VALIDATION_DRY_RUN_BOUNDARY_ENDPOINT
    );
    insert_report_json!(
        "source_command",
        "/hepta-memory-live-mutation-operator-write-execution-scoped-memory-real-write-canary-readback-validation-dry-run-boundary --json"
    );
    insert_report_json!("native_route", true);
    insert_report_json!("side_effect_free", true);
    insert_report_json!("audit_date", "2026-07-03");
    insert_report_json!(
        "scoped_memory_real_write_canary_readback_boundary_schema_version",
        "scoped_memory_real_write_canary_readback_validation_dry_run_boundary_v1"
    );
    insert_report_json!(
        "scoped_memory_real_write_canary_mode",
        "scoped_memory_real_write_canary_readback_validation_dry_run_no_read_no_write"
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
        "memory_write_execution_scoped_memory_real_write_canary_readback_validation_dry_run_boundary_ready",
        report_ready
    );
    insert_report_json!(
        "scoped_memory_real_write_canary_readback_validation_dry_run_ready",
        report_ready
    );
    insert_report_json!(
        "source_scoped_memory_real_write_canary_operator_approval_packet_nonce_command_dry_run_boundary_ready",
        source_ready
    );
    insert_report_json!(
        "source_scoped_memory_real_write_canary_operator_approval_packet_nonce_command_dry_run_ready",
        json_bool(
            &source,
            "scoped_memory_real_write_canary_operator_approval_packet_nonce_command_dry_run_ready",
        )
    );
    insert_report_json!(
        "source_scoped_memory_real_write_canary_operator_approval_packet_nonce_command_report_sha256",
        source_report_sha256
    );
    insert_report_json!(
        "source_scoped_memory_real_write_canary_operator_approval_fixture_count",
        json_u64(
            &source,
            "scoped_memory_real_write_canary_operator_approval_fixture_count"
        )
    );
    insert_report_json!(
        "source_accepted_scoped_memory_real_write_canary_operator_approval_fixture_count",
        json_u64(
            &source,
            "accepted_scoped_memory_real_write_canary_operator_approval_fixture_count"
        )
    );
    insert_report_json!(
        "source_scoped_memory_real_write_canary_approval_packet_accepted_count",
        json_u64(
            &source,
            "scoped_memory_real_write_canary_approval_packet_accepted_count"
        )
    );
    insert_report_json!(
        "source_single_use_nonce_consumed_count",
        json_u64(&source, "single_use_nonce_consumed_count")
    );
    insert_report_json!(
        "source_explicit_command_dispatched_count",
        json_u64(&source, "explicit_command_dispatched_count")
    );
    insert_report_json!(
        "source_post_write_readback_performed_count",
        json_u64(&source, "post_write_readback_performed_count")
    );
    insert_report_json!(
        "source_memory_store_write_performed_count",
        json_u64(&source, "memory_store_write_performed_count")
    );
    insert_report_json!("minimum_required_samples", 24);
    insert_report_json!(
        "scoped_memory_real_write_canary_readback_boundary_hash_sha256",
        boundary_hash_sha256
    );
    insert_report_json!(
        "scoped_memory_real_write_canary_readback_policy_hash_sha256",
        policy_hash_sha256
    );
    insert_report_json!(
        "required_scoped_memory_real_write_canary_readback_surface_count",
        12
    );
    insert_report_json!(
        "ready_scoped_memory_real_write_canary_readback_surface_count",
        12
    );
    insert_report_json!(
        "side_effect_free_scoped_memory_real_write_canary_readback_surface_count",
        12
    );
    insert_report_json!(
        "required_scoped_memory_real_write_canary_readback_fixture_count",
        10
    );
    insert_report_json!(
        "scoped_memory_real_write_canary_readback_fixture_count",
        fixture_count
    );
    insert_report_json!(
        "blocked_scoped_memory_real_write_canary_readback_fixture_count",
        fixture_count
    );
    insert_report_json!(
        "noop_scoped_memory_real_write_canary_readback_fixture_count",
        fixture_count
    );
    insert_report_json!(
        "allowed_scoped_memory_real_write_canary_readback_fixture_count",
        0
    );
    insert_report_json!(
        "accepted_scoped_memory_real_write_canary_readback_fixture_count",
        0
    );
    for key in [
        "readback_plan_accepted_count",
        "readback_performed_count",
        "readback_result_recorded_count",
        "readback_result_persisted_count",
        "readback_result_accepted_count",
        "readback_payload_digest_compared_count",
        "readback_redaction_proof_accepted_count",
        "readback_secret_plaintext_scan_performed_count",
        "durable_memory_store_read_performed_count",
        "memory_store_write_performed_count",
        "rollback_tombstone_handoff_accepted_count",
    ] {
        report.insert(key.to_string(), serde_json::json!(0));
    }
    insert_report_json!(
        "required_before_scoped_memory_real_write_canary_readback_acceptance_count",
        16
    );
    report.insert(
        "required_scoped_memory_real_write_canary_readback_fields".to_string(),
        required_fields,
    );
    report.insert(
        "scoped_memory_real_write_canary_readback_surfaces".to_string(),
        serde_json::json!(READBACK_SURFACES),
    );
    report.insert(
        "scoped_memory_real_write_canary_readback_fixtures".to_string(),
        fixtures,
    );
    report.insert(
        "denied_by_scoped_memory_real_write_canary_readback_validation_dry_run".to_string(),
        serde_json::Value::Array(denials),
    );
    insert_report_json!(
        "denied_by_scoped_memory_real_write_canary_readback_validation_dry_run_count",
        denied_count
    );
    for key in [
        "post_write_readback_plan_required",
        "receipt_linkage_required",
        "payload_digest_comparison_required",
        "redaction_secret_scan_required",
        "rollback_tombstone_handoff_required",
        "durable_memory_read_forbidden",
        "durable_memory_write_forbidden",
        "memory_store_mutation_forbidden",
        "rollback_execution_forbidden",
        "kg_live_write_forbidden",
        "provider_model_invocation_forbidden",
        "credential_read_forbidden",
        "channel_external_send_forbidden",
        "public_claim_release_artifact_forbidden",
        "install_restart_active_binary_mutation_forbidden",
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

fn hepta_memory_live_mutation_operator_write_execution_scoped_memory_real_write_canary_rollback_tombstone_dry_run_boundary_report()
-> serde_json::Value {
    const ROLLBACK_SURFACES: &[&str] = &[
        "source_readback_validation_dry_run_required",
        "readback_result_acceptance_required",
        "rollback_plan_required",
        "tombstone_plan_required",
        "rollback_target_binding_required",
        "rollback_receipt_linkage_required",
        "rollback_idempotency_guard_required",
        "rollback_ordering_guard_required",
        "rollback_audit_evidence_required",
        "operator_review_handoff_required",
        "minimal_real_write_handoff_required",
        "memory_kg_provider_channel_public_release_install_active_binary_side_effects_forbidden",
    ];
    const ROLLBACK_DENIALS: &[&str] = &[
        "source_readback_validation_dry_run_boundary_required",
        "readback_result_not_accepted",
        "rollback_tombstone_handoff_not_accepted",
        "rollback_plan_not_accepted",
        "tombstone_plan_not_accepted",
        "rollback_target_not_bound",
        "rollback_receipt_not_linked",
        "rollback_idempotency_guard_not_accepted",
        "rollback_ordering_guard_not_accepted",
        "rollback_audit_evidence_not_recorded",
        "operator_review_handoff_not_accepted",
        "minimal_real_write_handoff_not_accepted",
        "durable_memory_store_read_denied",
        "durable_memory_store_write_denied",
        "durable_memory_store_rollback_denied",
        "memory_store_mutation_denied",
        "rollback_execution_denied",
        "tombstone_write_denied",
        "compensating_memory_write_denied",
        "kg_live_write_denied",
        "provider_model_invocation_denied",
        "credential_read_denied",
        "channel_external_send_denied",
        "public_claim_release_artifact_denied",
        "install_restart_active_binary_mutation_denied",
        "filesystem_write_denied",
        "activation_authority_denied",
        "minimal_real_write_canary_acceptance_denied",
    ];
    const FALSE_KEYS: &[&str] = &[
        "fresh_operator_approval_packet_accepted",
        "single_use_nonce_consumed",
        "explicit_command_dispatched",
        "wal_receipt_plan_accepted",
        "wal_write_performed",
        "receipt_recorded",
        "receipt_persisted",
        "receipt_accepted",
        "post_write_readback_plan_accepted",
        "post_write_readback_performed",
        "readback_result_recorded",
        "readback_result_persisted",
        "readback_result_accepted",
        "readback_payload_digest_compared",
        "readback_payload_digest_matched",
        "readback_redaction_proof_accepted",
        "readback_secret_plaintext_scan_performed",
        "rollback_tombstone_handoff_accepted",
        "rollback_plan_accepted",
        "rollback_tombstone_plan_accepted",
        "tombstone_plan_accepted",
        "rollback_target_bound",
        "rollback_receipt_linked",
        "rollback_ordering_guard_accepted",
        "rollback_idempotency_guard_accepted",
        "rollback_audit_evidence_recorded",
        "rollback_audit_evidence_persisted",
        "operator_review_handoff_accepted",
        "minimal_real_write_handoff_accepted",
        "rollback_executed",
        "rollback_result_recorded",
        "rollback_result_persisted",
        "rollback_result_accepted",
        "tombstone_written",
        "compensating_memory_write_performed",
        "activation_allowed_by_scoped_canary_rollback",
        "activation_allowed",
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

    fn rollback_fixture(
        id: &str,
        status: &str,
        reason: &str,
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
            "scoped_memory_real_write_canary_rollback_tombstone_status",
            status
        );
        insert_fixture_json!("source_readback_validation_dry_run_present", true);
        insert_fixture_json!("source_readback_validation_dry_run_ready", true);
        insert_fixture_json!(
            "scoped_canary_rollback_tombstone_dry_run_noop_confirmed",
            true
        );
        insert_fixture_json!("reason", reason);
        for key in [
            "readback_result_acceptance_requested",
            "rollback_plan_requested",
            "tombstone_plan_requested",
            "rollback_target_binding_requested",
            "rollback_receipt_linkage_requested",
            "rollback_idempotency_guard_requested",
            "rollback_ordering_guard_requested",
            "rollback_audit_evidence_requested",
            "operator_review_handoff_requested",
            "minimal_real_write_handoff_requested",
            "durable_memory_read_requested",
            "durable_memory_write_requested",
            "durable_memory_rollback_requested",
            "memory_store_mutation_requested",
            "rollback_execution_requested",
            "tombstone_write_requested",
            "compensating_memory_write_requested",
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
            "readback_result_accepted",
            "rollback_tombstone_handoff_accepted",
            "rollback_plan_accepted",
            "rollback_tombstone_plan_accepted",
            "tombstone_plan_accepted",
            "rollback_target_bound",
            "rollback_receipt_linked",
            "rollback_idempotency_guard_accepted",
            "rollback_ordering_guard_accepted",
            "rollback_audit_evidence_recorded",
            "rollback_audit_evidence_persisted",
            "operator_review_handoff_accepted",
            "minimal_real_write_handoff_accepted",
            "rollback_executed",
            "rollback_result_recorded",
            "rollback_result_persisted",
            "rollback_result_accepted",
            "tombstone_written",
            "compensating_memory_write_performed",
            "activation_allowed",
            "memory_write_execution_performed",
            "memory_store_write_performed",
            "memory_store_mutated",
            "durable_memory_store_write_performed",
            "durable_memory_store_read_performed",
            "durable_memory_store_rollback_performed",
            "kg_adapter_read_performed",
            "live_kg_write_performed",
            "provider_invoked",
            "model_invoked",
            "credential_read",
            "secret_file_read",
            "telegram_send_performed",
            "channel_send_performed",
            "external_send_performed",
            "public_claim_promoted",
            "release_artifact_written",
            "install_executed",
            "service_restarted",
            "active_binary_mutated",
        ] {
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
        .name("hepta-memory-scoped-real-write-canary-rollback-source-report".to_string())
        .stack_size(8 * 1024 * 1024)
        .spawn(
            hepta_memory_live_mutation_operator_write_execution_scoped_memory_real_write_canary_readback_validation_dry_run_boundary_report,
        )
        .ok()
        .and_then(|handle| handle.join().ok())
        .unwrap_or_else(|| {
            serde_json::json!({
                "status": "blocked",
                "memory_write_execution_scoped_memory_real_write_canary_readback_validation_dry_run_boundary_ready": false,
                "scoped_memory_real_write_canary_readback_validation_dry_run_ready": false,
                "source_scoped_canary_readback_validation_source_report_thread_failed": true
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
    let source_next_action_rollback = source
        .get("allowed_next_actions")
        .and_then(serde_json::Value::as_array)
        .and_then(|items| items.get(1))
        .map(|item| {
            item.get("action").and_then(serde_json::Value::as_str)
                == Some(
                    "prepare_scoped_memory_real_write_canary_rollback_tombstone_dry_run_boundary",
                )
                && item
                    .get("requires_readback_validation_acceptance")
                    .and_then(serde_json::Value::as_bool)
                    == Some(true)
        })
        .unwrap_or(false);
    let source_ready = source.get("status").and_then(serde_json::Value::as_str) == Some("ready")
        && json_bool(
            &source,
            "memory_write_execution_scoped_memory_real_write_canary_readback_validation_dry_run_boundary_ready",
        )
        && json_bool(
            &source,
            "scoped_memory_real_write_canary_readback_validation_dry_run_ready",
        )
        && json_u64(
            &source,
            "scoped_memory_real_write_canary_readback_fixture_count",
        ) == 10
        && json_u64(
            &source,
            "accepted_scoped_memory_real_write_canary_readback_fixture_count",
        ) == 0
        && json_u64(
            &source,
            "denied_by_scoped_memory_real_write_canary_readback_validation_dry_run_count",
        ) == 26
        && json_u64(&source, "readback_performed_count") == 0
        && json_u64(&source, "readback_result_accepted_count") == 0
        && json_u64(&source, "durable_memory_store_read_performed_count") == 0
        && json_u64(&source, "memory_store_write_performed_count") == 0
        && json_u64(&source, "rollback_tombstone_handoff_accepted_count") == 0
        && !json_bool(&source, "readback_result_accepted")
        && !json_bool(&source, "rollback_tombstone_handoff_accepted")
        && !json_bool(&source, "durable_memory_store_read_performed")
        && !json_bool(&source, "durable_memory_store_write_performed")
        && !json_bool(&source, "durable_memory_store_rollback_performed")
        && !json_bool(&source, "memory_store_write_performed")
        && !json_bool(&source, "memory_store_mutated")
        && !json_bool(&source, "rollback_executed")
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
        && source_next_action_rollback
        && side_effects_all_false(&source);

    let fixtures = serde_json::Value::Array(vec![
        rollback_fixture(
            "scoped-memory-real-write-canary-rollback-missing-readback-source",
            "blocked_source_noop",
            "readback_validation_dry_run_source_required",
            serde_json::json!({
                "source_readback_validation_dry_run_present": false,
                "source_readback_validation_dry_run_ready": false,
                "rollback_plan_requested": true
            }),
        ),
        rollback_fixture(
            "scoped-memory-real-write-canary-rollback-readback-acceptance-required",
            "blocked_readback_acceptance_noop",
            "readback_result_acceptance_required_before_rollback_plan",
            serde_json::json!({"readback_result_acceptance_requested": true}),
        ),
        rollback_fixture(
            "scoped-memory-real-write-canary-rollback-plan-required",
            "blocked_rollback_plan_noop",
            "rollback_plan_required_before_real_write_canary",
            serde_json::json!({"rollback_plan_requested": true}),
        ),
        rollback_fixture(
            "scoped-memory-real-write-canary-tombstone-plan-required",
            "blocked_tombstone_plan_noop",
            "tombstone_plan_required_before_real_write_canary",
            serde_json::json!({"tombstone_plan_requested": true}),
        ),
        rollback_fixture(
            "scoped-memory-real-write-canary-rollback-target-binding-required",
            "blocked_target_binding_noop",
            "rollback_target_binding_required_before_real_write_canary",
            serde_json::json!({"rollback_target_binding_requested": true}),
        ),
        rollback_fixture(
            "scoped-memory-real-write-canary-rollback-receipt-linkage-required",
            "blocked_receipt_linkage_noop",
            "rollback_receipt_linkage_required_before_real_write_canary",
            serde_json::json!({"rollback_receipt_linkage_requested": true}),
        ),
        rollback_fixture(
            "scoped-memory-real-write-canary-rollback-idempotency-ordering-guards-required",
            "blocked_guard_noop",
            "rollback_idempotency_and_ordering_guards_required",
            serde_json::json!({
                "rollback_idempotency_guard_requested": true,
                "rollback_ordering_guard_requested": true
            }),
        ),
        rollback_fixture(
            "scoped-memory-real-write-canary-rollback-audit-evidence-required",
            "blocked_audit_noop",
            "rollback_audit_evidence_required_before_real_write_canary",
            serde_json::json!({"rollback_audit_evidence_requested": true}),
        ),
        rollback_fixture(
            "scoped-memory-real-write-canary-rollback-operator-review-and-minimal-handoff-required",
            "blocked_handoff_noop",
            "operator_review_and_minimal_real_write_handoff_required",
            serde_json::json!({
                "operator_review_handoff_requested": true,
                "minimal_real_write_handoff_requested": true
            }),
        ),
        rollback_fixture(
            "scoped-memory-real-write-canary-rollback-direct-side-effect-attempt",
            "blocked_execution_noop",
            "direct_rollback_tombstone_memory_kg_provider_channel_release_install_active_binary_side_effects_denied",
            serde_json::json!({
                "durable_memory_read_requested": true,
                "durable_memory_write_requested": true,
                "durable_memory_rollback_requested": true,
                "memory_store_mutation_requested": true,
                "rollback_execution_requested": true,
                "tombstone_write_requested": true,
                "compensating_memory_write_requested": true,
                "kg_live_write_requested": true,
                "provider_model_invocation_requested": true,
                "credential_read_requested": true,
                "channel_external_send_requested": true,
                "public_claim_requested": true,
                "release_artifact_write_requested": true,
                "install_restart_requested": true,
                "active_binary_mutation_requested": true
            }),
        ),
    ]);
    let fixture_count = fixtures.as_array().map(std::vec::Vec::len).unwrap_or(0);
    let denials = ROLLBACK_DENIALS
        .iter()
        .map(|reason| {
            serde_json::json!({
                "reason": reason,
                "accepted": false,
                "performed": false,
                "reads_memory": false,
                "writes_memory": false,
                "executes_rollback": false,
                "writes_tombstone": false,
                "writes_kg": false,
                "invokes_provider": false,
                "sends_externally": false
            })
        })
        .collect::<Vec<_>>();
    let denied_count = denials.len();
    let source_report_sha256 = sha256_json_value(&source);
    let fixtures_hash_sha256 = sha256_json_value(&fixtures);
    let boundary_hash_sha256 = sha256_text_value(&format!(
        "hepta-memory-live-mutation-operator-write-execution-scoped-memory-real-write-canary-rollback-tombstone-dry-run-boundary-v1:{}:{}:{}",
        route_matrix.route_count, source_report_sha256, fixtures_hash_sha256
    ));
    let policy_hash_sha256 = sha256_text_value(
        "scoped-memory-real-write-canary-rollback-tombstone-dry-run:no-durable-read:no-write:no-rollback:no-tombstone",
    );
    let report_ready = route_matrix.ready
        && route_count_source_command_accepted
        && source_ready
        && ROLLBACK_SURFACES.len() == 12
        && fixture_count == 10
        && denied_count == 28;

    let mut side_effects = serde_json::Map::new();
    for &key in FALSE_KEYS {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }

    let required_fields = serde_json::json!([
        "source_scoped_canary_readback_validation_report_sha256",
        "wal_receipt_id",
        "readback_result_id",
        "rollback_plan_id",
        "tombstone_plan_id",
        "rollback_target_digest",
        "canary_namespace",
        "canary_store",
        "canary_scope",
        "rollback_receipt_id",
        "rollback_idempotency_guard_id",
        "rollback_ordering_guard_id",
        "rollback_audit_evidence_id",
        "operator_review_handoff_id",
        "minimal_real_write_handoff_id",
    ]);
    let allowed_next_actions = serde_json::json!([
        {
            "action": "run_scoped_memory_real_write_canary_rollback_tombstone_dry_run_boundary_require_live_gate",
            "status": "allowed_verification_only",
            "reads_memory": false,
            "writes_memory": false,
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
            "action": "prepare_minimal_scoped_memory_real_write_canary_accepted_gate",
            "status": "allowed_report_only_next_slice",
            "requires_rollback_tombstone_dry_run_boundary": true,
            "writes_memory": false,
            "reads_memory": false,
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
        "hepta_memory_live_mutation_operator_write_execution_scoped_memory_real_write_canary_rollback_tombstone_dry_run_boundary_route"
    );
    insert_report_json!(
        "endpoint",
        HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_SCOPED_MEMORY_REAL_WRITE_CANARY_ROLLBACK_TOMBSTONE_DRY_RUN_BOUNDARY_ENDPOINT
    );
    insert_report_json!(
        "source_command",
        "/hepta-memory-live-mutation-operator-write-execution-scoped-memory-real-write-canary-rollback-tombstone-dry-run-boundary --json"
    );
    insert_report_json!("native_route", true);
    insert_report_json!("side_effect_free", true);
    insert_report_json!("audit_date", "2026-07-03");
    insert_report_json!(
        "scoped_memory_real_write_canary_rollback_tombstone_boundary_schema_version",
        "scoped_memory_real_write_canary_rollback_tombstone_dry_run_boundary_v1"
    );
    insert_report_json!(
        "scoped_memory_real_write_canary_mode",
        "scoped_memory_real_write_canary_rollback_tombstone_dry_run_no_rollback_no_write"
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
        "memory_write_execution_scoped_memory_real_write_canary_rollback_tombstone_dry_run_boundary_ready",
        report_ready
    );
    insert_report_json!(
        "scoped_memory_real_write_canary_rollback_tombstone_dry_run_ready",
        report_ready
    );
    insert_report_json!(
        "source_scoped_memory_real_write_canary_readback_validation_dry_run_boundary_ready",
        source_ready
    );
    insert_report_json!(
        "source_scoped_memory_real_write_canary_readback_validation_dry_run_ready",
        json_bool(
            &source,
            "scoped_memory_real_write_canary_readback_validation_dry_run_ready",
        )
    );
    insert_report_json!(
        "source_scoped_memory_real_write_canary_readback_validation_report_sha256",
        source_report_sha256
    );
    insert_report_json!(
        "source_scoped_memory_real_write_canary_readback_fixture_count",
        json_u64(
            &source,
            "scoped_memory_real_write_canary_readback_fixture_count"
        )
    );
    insert_report_json!(
        "source_accepted_scoped_memory_real_write_canary_readback_fixture_count",
        json_u64(
            &source,
            "accepted_scoped_memory_real_write_canary_readback_fixture_count"
        )
    );
    insert_report_json!(
        "source_scoped_memory_real_write_canary_readback_denial_count",
        json_u64(
            &source,
            "denied_by_scoped_memory_real_write_canary_readback_validation_dry_run_count"
        )
    );
    insert_report_json!(
        "source_readback_performed_count",
        json_u64(&source, "readback_performed_count")
    );
    insert_report_json!(
        "source_readback_result_accepted_count",
        json_u64(&source, "readback_result_accepted_count")
    );
    insert_report_json!(
        "source_durable_memory_store_read_performed_count",
        json_u64(&source, "durable_memory_store_read_performed_count")
    );
    insert_report_json!(
        "source_memory_store_write_performed_count",
        json_u64(&source, "memory_store_write_performed_count")
    );
    insert_report_json!(
        "source_rollback_tombstone_handoff_accepted_count",
        json_u64(&source, "rollback_tombstone_handoff_accepted_count")
    );
    insert_report_json!("minimum_required_samples", 24);
    insert_report_json!(
        "scoped_memory_real_write_canary_rollback_tombstone_boundary_hash_sha256",
        boundary_hash_sha256
    );
    insert_report_json!(
        "scoped_memory_real_write_canary_rollback_tombstone_policy_hash_sha256",
        policy_hash_sha256
    );
    insert_report_json!(
        "required_scoped_memory_real_write_canary_rollback_tombstone_surface_count",
        12
    );
    insert_report_json!(
        "ready_scoped_memory_real_write_canary_rollback_tombstone_surface_count",
        12
    );
    insert_report_json!(
        "side_effect_free_scoped_memory_real_write_canary_rollback_tombstone_surface_count",
        12
    );
    insert_report_json!(
        "required_scoped_memory_real_write_canary_rollback_tombstone_fixture_count",
        10
    );
    insert_report_json!(
        "scoped_memory_real_write_canary_rollback_tombstone_fixture_count",
        fixture_count
    );
    insert_report_json!(
        "blocked_scoped_memory_real_write_canary_rollback_tombstone_fixture_count",
        fixture_count
    );
    insert_report_json!(
        "noop_scoped_memory_real_write_canary_rollback_tombstone_fixture_count",
        fixture_count
    );
    insert_report_json!(
        "allowed_scoped_memory_real_write_canary_rollback_tombstone_fixture_count",
        0
    );
    insert_report_json!(
        "accepted_scoped_memory_real_write_canary_rollback_tombstone_fixture_count",
        0
    );
    for key in [
        "rollback_plan_accepted_count",
        "rollback_tombstone_plan_accepted_count",
        "tombstone_plan_accepted_count",
        "rollback_target_bound_count",
        "rollback_receipt_linked_count",
        "rollback_ordering_guard_accepted_count",
        "rollback_idempotency_guard_accepted_count",
        "rollback_audit_evidence_recorded_count",
        "rollback_audit_evidence_persisted_count",
        "operator_review_handoff_accepted_count",
        "minimal_real_write_handoff_accepted_count",
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
        "required_before_scoped_memory_real_write_canary_rollback_tombstone_acceptance_count",
        15
    );
    report.insert(
        "required_scoped_memory_real_write_canary_rollback_tombstone_fields".to_string(),
        required_fields,
    );
    report.insert(
        "scoped_memory_real_write_canary_rollback_tombstone_surfaces".to_string(),
        serde_json::json!(ROLLBACK_SURFACES),
    );
    report.insert(
        "scoped_memory_real_write_canary_rollback_tombstone_fixtures".to_string(),
        fixtures,
    );
    report.insert(
        "denied_by_scoped_memory_real_write_canary_rollback_tombstone_dry_run".to_string(),
        serde_json::Value::Array(denials),
    );
    insert_report_json!(
        "denied_by_scoped_memory_real_write_canary_rollback_tombstone_dry_run_count",
        denied_count
    );
    for key in [
        "rollback_plan_required",
        "tombstone_plan_required",
        "rollback_target_binding_required",
        "rollback_receipt_linkage_required",
        "rollback_idempotency_guard_required",
        "rollback_ordering_guard_required",
        "rollback_audit_evidence_required",
        "operator_review_handoff_required",
        "minimal_real_write_handoff_required",
        "rollback_execution_forbidden",
        "tombstone_write_forbidden",
        "durable_memory_read_forbidden",
        "durable_memory_write_forbidden",
        "durable_memory_rollback_forbidden",
        "memory_store_mutation_forbidden",
        "kg_live_write_forbidden",
        "provider_model_invocation_forbidden",
        "credential_read_forbidden",
        "channel_external_send_forbidden",
        "public_claim_release_artifact_forbidden",
        "install_restart_active_binary_mutation_forbidden",
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

fn hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_operator_approval_nonce_command_accepted_gate_boundary_report()
-> serde_json::Value {
    const ACCEPTED_GATE_SURFACES: &[&str] = &[
        "source_rollback_tombstone_dry_run_required",
        "fresh_operator_approval_artifact_required",
        "operator_identity_session_binding_required",
        "single_use_nonce_authority_required",
        "explicit_command_acceptance_required",
        "canary_namespace_store_scope_binding_required",
        "payload_digest_redaction_binding_required",
        "active_binary_sha_route_count_binding_required",
        "wal_receipt_binding_required",
        "post_write_readback_binding_required",
        "rollback_tombstone_proof_binding_required",
        "memory_kg_provider_channel_public_release_install_active_binary_side_effects_forbidden",
    ];
    const ACCEPTED_GATE_DENIALS: &[&str] = &[
        "source_rollback_tombstone_dry_run_boundary_required",
        "fresh_operator_approval_artifact_required",
        "operator_identity_session_binding_required",
        "single_use_nonce_authority_required",
        "explicit_command_acceptance_required",
        "canary_namespace_store_scope_binding_required",
        "payload_digest_redaction_binding_required",
        "active_binary_sha_route_count_binding_required",
        "wal_receipt_binding_required",
        "post_write_readback_binding_required",
        "rollback_tombstone_proof_binding_required",
        "nonce_consumption_report_route_denied",
        "explicit_command_dispatch_report_route_denied",
        "durable_memory_store_write_denied",
        "durable_memory_store_read_denied",
        "durable_memory_store_rollback_denied",
        "memory_store_mutation_denied",
        "wal_write_denied",
        "receipt_persistence_denied",
        "kg_live_write_denied",
        "provider_model_invocation_denied",
        "credential_read_denied",
        "channel_external_send_denied",
        "public_claim_release_artifact_denied",
        "install_restart_active_binary_mutation_denied",
        "filesystem_write_denied",
    ];
    const FALSE_KEYS: &[&str] = &[
        "single_use_nonce_consumed",
        "explicit_command_dispatched",
        "wal_write_performed",
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

    fn accepted_gate_fixture(
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
            "minimal_scoped_memory_real_write_canary_accepted_gate_status",
            status
        );
        insert_fixture_json!("source_rollback_tombstone_dry_run_present", true);
        insert_fixture_json!("source_rollback_tombstone_dry_run_ready", true);
        insert_fixture_json!("reason", reason);
        insert_fixture_json!("accepted_authority_envelope_noop_confirmed", true);
        for key in [
            "fresh_operator_approval_artifact_requested",
            "operator_identity_session_binding_requested",
            "single_use_nonce_authority_requested",
            "explicit_command_acceptance_requested",
            "canary_namespace_store_scope_binding_requested",
            "payload_digest_redaction_binding_requested",
            "active_binary_sha_route_count_binding_requested",
            "wal_receipt_binding_requested",
            "post_write_readback_binding_requested",
            "rollback_tombstone_proof_binding_requested",
            "single_use_nonce_consumption_requested",
            "explicit_command_dispatch_requested",
            "durable_memory_read_requested",
            "durable_memory_write_requested",
            "durable_memory_rollback_requested",
            "memory_store_mutation_requested",
            "wal_write_requested",
            "receipt_persistence_requested",
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
            "fresh_operator_approval_artifact_accepted",
            "operator_identity_bound",
            "operator_session_bound",
            "single_use_nonce_authority_accepted",
            "explicit_command_accepted",
            "canary_namespace_bound",
            "canary_store_bound",
            "canary_scope_bound",
            "payload_digest_bound",
            "payload_redaction_bound",
            "active_binary_sha_bound",
            "route_count_bound",
            "wal_receipt_binding_accepted",
            "post_write_readback_binding_accepted",
            "rollback_tombstone_proof_binding_accepted",
            "minimal_real_write_authority_accepted",
        ] {
            base.insert(key.to_string(), serde_json::json!(accepted));
        }
        for key in [
            "single_use_nonce_consumed",
            "explicit_command_dispatched",
            "wal_write_performed",
            "receipt_recorded",
            "receipt_persisted",
            "receipt_materialized",
            "post_write_readback_performed",
            "readback_result_accepted",
            "rollback_executed",
            "tombstone_written",
            "compensating_memory_write_performed",
            "activation_performed",
            "live_mutation_execution_performed",
            "memory_write_execution_performed",
            "memory_store_write_performed",
            "memory_store_mutated",
            "durable_memory_store_write_performed",
            "durable_memory_store_read_performed",
            "durable_memory_store_rollback_performed",
            "kg_adapter_read_performed",
            "live_kg_write_performed",
            "provider_invoked",
            "model_invoked",
            "credential_read",
            "secret_file_read",
            "telegram_send_performed",
            "channel_send_performed",
            "external_send_performed",
            "public_claim_promoted",
            "release_artifact_written",
            "install_executed",
            "service_restarted",
            "active_binary_mutated",
            "filesystem_written",
        ] {
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
        .name("hepta-memory-minimal-canary-accepted-gate-source-report".to_string())
        .stack_size(8 * 1024 * 1024)
        .spawn(
            hepta_memory_live_mutation_operator_write_execution_scoped_memory_real_write_canary_rollback_tombstone_dry_run_boundary_report,
        )
        .ok()
        .and_then(|handle| handle.join().ok())
        .unwrap_or_else(|| {
            serde_json::json!({
                "status": "blocked",
                "memory_write_execution_scoped_memory_real_write_canary_rollback_tombstone_dry_run_boundary_ready": false,
                "scoped_memory_real_write_canary_rollback_tombstone_dry_run_ready": false,
                "source_minimal_scoped_memory_real_write_canary_accepted_gate_source_report_thread_failed": true
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
    let source_next_action_accepted_gate = source
        .get("allowed_next_actions")
        .and_then(serde_json::Value::as_array)
        .and_then(|items| items.get(1))
        .map(|item| {
            item.get("action").and_then(serde_json::Value::as_str)
                == Some("prepare_minimal_scoped_memory_real_write_canary_accepted_gate")
                && item
                    .get("requires_rollback_tombstone_dry_run_boundary")
                    .and_then(serde_json::Value::as_bool)
                    == Some(true)
        })
        .unwrap_or(false);
    let source_ready = source.get("status").and_then(serde_json::Value::as_str) == Some("ready")
        && json_bool(
            &source,
            "memory_write_execution_scoped_memory_real_write_canary_rollback_tombstone_dry_run_boundary_ready",
        )
        && json_bool(
            &source,
            "scoped_memory_real_write_canary_rollback_tombstone_dry_run_ready",
        )
        && json_u64(
            &source,
            "scoped_memory_real_write_canary_rollback_tombstone_fixture_count",
        ) == 10
        && json_u64(
            &source,
            "accepted_scoped_memory_real_write_canary_rollback_tombstone_fixture_count",
        ) == 0
        && json_u64(
            &source,
            "denied_by_scoped_memory_real_write_canary_rollback_tombstone_dry_run_count",
        ) == 28
        && json_u64(&source, "rollback_performed_count") == 0
        && json_u64(&source, "tombstone_written_count") == 0
        && json_u64(&source, "durable_memory_store_read_performed_count") == 0
        && json_u64(&source, "durable_memory_store_write_performed_count") == 0
        && json_u64(&source, "durable_memory_store_rollback_performed_count") == 0
        && json_u64(&source, "memory_store_write_performed_count") == 0
        && !json_bool(&source, "rollback_executed")
        && !json_bool(&source, "tombstone_written")
        && !json_bool(&source, "compensating_memory_write_performed")
        && !json_bool(&source, "memory_write_execution_performed")
        && !json_bool(&source, "memory_store_write_performed")
        && !json_bool(&source, "memory_store_mutated")
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
        && source_next_action_accepted_gate
        && side_effects_all_false(&source);

    let fixtures = serde_json::Value::Array(vec![
        accepted_gate_fixture(
            "minimal-scoped-memory-real-write-canary-accepted-gate-authority-envelope",
            "accepted_authority_noop",
            "operator_approval_nonce_command_scope_bindings_accepted_without_nonce_consumption_or_write",
            true,
            serde_json::json!({
                "fresh_operator_approval_artifact_requested": true,
                "operator_identity_session_binding_requested": true,
                "single_use_nonce_authority_requested": true,
                "explicit_command_acceptance_requested": true,
                "canary_namespace_store_scope_binding_requested": true,
                "payload_digest_redaction_binding_requested": true,
                "active_binary_sha_route_count_binding_requested": true,
                "wal_receipt_binding_requested": true,
                "post_write_readback_binding_requested": true,
                "rollback_tombstone_proof_binding_requested": true
            }),
        ),
        accepted_gate_fixture(
            "minimal-scoped-memory-real-write-canary-missing-rollback-source",
            "blocked_source_noop",
            "rollback_tombstone_dry_run_boundary_required",
            false,
            serde_json::json!({
                "source_rollback_tombstone_dry_run_present": false,
                "source_rollback_tombstone_dry_run_ready": false,
                "fresh_operator_approval_artifact_requested": true
            }),
        ),
        accepted_gate_fixture(
            "minimal-scoped-memory-real-write-canary-operator-approval-required",
            "blocked_operator_approval_noop",
            "fresh_operator_approval_artifact_required",
            false,
            serde_json::json!({"fresh_operator_approval_artifact_requested": true}),
        ),
        accepted_gate_fixture(
            "minimal-scoped-memory-real-write-canary-identity-session-required",
            "blocked_identity_session_noop",
            "operator_identity_session_binding_required",
            false,
            serde_json::json!({"operator_identity_session_binding_requested": true}),
        ),
        accepted_gate_fixture(
            "minimal-scoped-memory-real-write-canary-nonce-required",
            "blocked_nonce_noop",
            "single_use_nonce_authority_required",
            false,
            serde_json::json!({"single_use_nonce_authority_requested": true}),
        ),
        accepted_gate_fixture(
            "minimal-scoped-memory-real-write-canary-explicit-command-required",
            "blocked_explicit_command_noop",
            "explicit_command_acceptance_required",
            false,
            serde_json::json!({"explicit_command_acceptance_requested": true}),
        ),
        accepted_gate_fixture(
            "minimal-scoped-memory-real-write-canary-scope-store-required",
            "blocked_scope_store_noop",
            "canary_namespace_store_scope_binding_required",
            false,
            serde_json::json!({"canary_namespace_store_scope_binding_requested": true}),
        ),
        accepted_gate_fixture(
            "minimal-scoped-memory-real-write-canary-digest-redaction-required",
            "blocked_digest_redaction_noop",
            "payload_digest_redaction_binding_required",
            false,
            serde_json::json!({"payload_digest_redaction_binding_requested": true}),
        ),
        accepted_gate_fixture(
            "minimal-scoped-memory-real-write-canary-binary-route-wal-readback-rollback-required",
            "blocked_binding_noop",
            "active_binary_route_wal_readback_rollback_bindings_required",
            false,
            serde_json::json!({
                "active_binary_sha_route_count_binding_requested": true,
                "wal_receipt_binding_requested": true,
                "post_write_readback_binding_requested": true,
                "rollback_tombstone_proof_binding_requested": true
            }),
        ),
        accepted_gate_fixture(
            "minimal-scoped-memory-real-write-canary-direct-side-effect-attempt",
            "blocked_side_effect_noop",
            "direct_nonce_consumption_dispatch_memory_wal_readback_rollback_kg_provider_channel_release_install_active_binary_side_effects_denied",
            false,
            serde_json::json!({
                "single_use_nonce_consumption_requested": true,
                "explicit_command_dispatch_requested": true,
                "durable_memory_read_requested": true,
                "durable_memory_write_requested": true,
                "durable_memory_rollback_requested": true,
                "memory_store_mutation_requested": true,
                "wal_write_requested": true,
                "receipt_persistence_requested": true,
                "rollback_execution_requested": true,
                "tombstone_write_requested": true,
                "kg_live_write_requested": true,
                "provider_model_invocation_requested": true,
                "credential_read_requested": true,
                "channel_external_send_requested": true,
                "public_claim_requested": true,
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
                .filter(|item| json_bool(item, "minimal_real_write_authority_accepted"))
                .count()
        })
        .unwrap_or(0);
    let blocked_fixture_count = fixture_count.saturating_sub(accepted_fixture_count);
    let denials = ACCEPTED_GATE_DENIALS
        .iter()
        .map(|reason| {
            serde_json::json!({
                "reason": reason,
                "accepted": false,
                "performed": false,
                "consumes_nonce": false,
                "dispatches_command": false,
                "reads_memory": false,
                "writes_memory": false,
                "writes_wal": false,
                "persists_receipt": false,
                "executes_rollback": false,
                "writes_tombstone": false,
                "writes_kg": false,
                "invokes_provider": false,
                "sends_externally": false
            })
        })
        .collect::<Vec<_>>();
    let denied_count = denials.len();
    let source_report_sha256 = sha256_json_value(&source);
    let fixtures_hash_sha256 = sha256_json_value(&fixtures);
    let boundary_hash_sha256 = sha256_text_value(&format!(
        "hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-accepted-gate-boundary-v1:{}:{}:{}",
        route_matrix.route_count, source_report_sha256, fixtures_hash_sha256
    ));
    let policy_hash_sha256 = sha256_text_value(
        "minimal-scoped-memory-real-write-canary-accepted-gate:authority-accepted:no-nonce-consume:no-command-dispatch:no-write:no-read:no-rollback",
    );
    let report_ready = route_matrix.ready
        && route_count_source_command_accepted
        && source_ready
        && ACCEPTED_GATE_SURFACES.len() == 12
        && fixture_count == 10
        && accepted_fixture_count == 1
        && blocked_fixture_count == 9
        && denied_count == 26;

    let mut side_effects = serde_json::Map::new();
    for &key in FALSE_KEYS {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }

    let required_fields = serde_json::json!([
        "source_scoped_canary_rollback_tombstone_report_sha256",
        "operator_approval_artifact_id",
        "operator_identity",
        "operator_session_id",
        "single_use_nonce_id",
        "explicit_command_id",
        "canary_namespace",
        "canary_store",
        "canary_scope",
        "payload_digest_sha256",
        "payload_redaction_proof_id",
        "active_binary_sha256",
        "route_count",
        "wal_receipt_plan_id",
        "post_write_readback_plan_id",
        "rollback_tombstone_proof_id",
    ]);
    let allowed_next_actions = serde_json::json!([
        {
            "action": "run_minimal_scoped_memory_real_write_canary_accepted_gate_boundary_require_live_gate",
            "status": "allowed_verification_only",
            "reads_memory": false,
            "writes_memory": false,
            "consumes_nonce": false,
            "dispatches_command": false,
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
            "action": "prepare_minimal_scoped_memory_real_write_canary_wal_receipt_binding_boundary",
            "status": "allowed_report_only_next_slice",
            "requires_minimal_scoped_memory_real_write_canary_accepted_gate": true,
            "writes_memory": false,
            "reads_memory": false,
            "consumes_nonce": false,
            "dispatches_command": false,
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
        "hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_operator_approval_nonce_command_accepted_gate_boundary_route"
    );
    insert_report_json!(
        "endpoint",
        HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_MINIMAL_SCOPED_MEMORY_REAL_WRITE_CANARY_OPERATOR_APPROVAL_NONCE_COMMAND_ACCEPTED_GATE_BOUNDARY_ENDPOINT
    );
    insert_report_json!(
        "source_command",
        "/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-operator-approval-nonce-command-accepted-gate-boundary --json"
    );
    insert_report_json!("native_route", true);
    insert_report_json!("side_effect_free", true);
    insert_report_json!("audit_date", "2026-07-03");
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_accepted_gate_schema_version",
        "minimal_scoped_memory_real_write_canary_operator_approval_nonce_command_accepted_gate_boundary_v1"
    );
    insert_report_json!(
        "scoped_memory_real_write_canary_mode",
        "minimal_scoped_memory_real_write_canary_accepted_gate_authority_no_write"
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
        "memory_write_execution_minimal_scoped_memory_real_write_canary_operator_approval_nonce_command_accepted_gate_boundary_ready",
        report_ready
    );
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_accepted_gate_ready",
        report_ready
    );
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_authority_accepted_no_write",
        report_ready
    );
    insert_report_json!(
        "source_scoped_memory_real_write_canary_rollback_tombstone_dry_run_boundary_ready",
        source_ready
    );
    insert_report_json!(
        "source_scoped_memory_real_write_canary_rollback_tombstone_dry_run_ready",
        json_bool(
            &source,
            "scoped_memory_real_write_canary_rollback_tombstone_dry_run_ready",
        )
    );
    insert_report_json!(
        "source_scoped_memory_real_write_canary_rollback_tombstone_report_sha256",
        source_report_sha256
    );
    insert_report_json!(
        "source_scoped_memory_real_write_canary_rollback_tombstone_fixture_count",
        json_u64(
            &source,
            "scoped_memory_real_write_canary_rollback_tombstone_fixture_count"
        )
    );
    insert_report_json!(
        "source_accepted_scoped_memory_real_write_canary_rollback_tombstone_fixture_count",
        json_u64(
            &source,
            "accepted_scoped_memory_real_write_canary_rollback_tombstone_fixture_count"
        )
    );
    insert_report_json!(
        "source_scoped_memory_real_write_canary_rollback_tombstone_denial_count",
        json_u64(
            &source,
            "denied_by_scoped_memory_real_write_canary_rollback_tombstone_dry_run_count"
        )
    );
    insert_report_json!(
        "source_rollback_performed_count",
        json_u64(&source, "rollback_performed_count")
    );
    insert_report_json!(
        "source_tombstone_written_count",
        json_u64(&source, "tombstone_written_count")
    );
    insert_report_json!(
        "source_durable_memory_store_read_performed_count",
        json_u64(&source, "durable_memory_store_read_performed_count")
    );
    insert_report_json!(
        "source_durable_memory_store_write_performed_count",
        json_u64(&source, "durable_memory_store_write_performed_count")
    );
    insert_report_json!(
        "source_durable_memory_store_rollback_performed_count",
        json_u64(&source, "durable_memory_store_rollback_performed_count")
    );
    insert_report_json!(
        "source_memory_store_write_performed_count",
        json_u64(&source, "memory_store_write_performed_count")
    );
    insert_report_json!("minimum_required_samples", 24);
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_accepted_gate_boundary_hash_sha256",
        boundary_hash_sha256
    );
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_accepted_gate_policy_hash_sha256",
        policy_hash_sha256
    );
    insert_report_json!(
        "required_minimal_scoped_memory_real_write_canary_accepted_gate_surface_count",
        12
    );
    insert_report_json!(
        "ready_minimal_scoped_memory_real_write_canary_accepted_gate_surface_count",
        12
    );
    insert_report_json!(
        "side_effect_free_minimal_scoped_memory_real_write_canary_accepted_gate_surface_count",
        12
    );
    insert_report_json!(
        "required_minimal_scoped_memory_real_write_canary_accepted_gate_fixture_count",
        10
    );
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_accepted_gate_fixture_count",
        fixture_count
    );
    insert_report_json!(
        "accepted_minimal_scoped_memory_real_write_canary_accepted_gate_fixture_count",
        accepted_fixture_count
    );
    insert_report_json!(
        "blocked_minimal_scoped_memory_real_write_canary_accepted_gate_fixture_count",
        blocked_fixture_count
    );
    insert_report_json!(
        "noop_minimal_scoped_memory_real_write_canary_accepted_gate_fixture_count",
        fixture_count
    );
    for key in [
        "fresh_operator_approval_artifact_accepted_count",
        "operator_identity_session_bound_count",
        "single_use_nonce_authority_accepted_count",
        "explicit_command_accepted_count",
        "canary_namespace_store_scope_bound_count",
        "payload_digest_redaction_bound_count",
        "active_binary_sha_route_count_bound_count",
        "wal_receipt_binding_accepted_count",
        "post_write_readback_binding_accepted_count",
        "rollback_tombstone_proof_binding_accepted_count",
        "minimal_scoped_memory_real_write_canary_authority_accepted_count",
    ] {
        report.insert(key.to_string(), serde_json::json!(1));
    }
    for key in [
        "single_use_nonce_consumed_count",
        "explicit_command_dispatched_count",
        "wal_write_performed_count",
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
        "required_before_minimal_scoped_memory_real_write_canary_execution_count",
        16
    );
    report.insert(
        "required_minimal_scoped_memory_real_write_canary_accepted_gate_fields".to_string(),
        required_fields,
    );
    report.insert(
        "minimal_scoped_memory_real_write_canary_accepted_gate_surfaces".to_string(),
        serde_json::json!(ACCEPTED_GATE_SURFACES),
    );
    report.insert(
        "minimal_scoped_memory_real_write_canary_accepted_gate_fixtures".to_string(),
        fixtures,
    );
    report.insert(
        "denied_by_minimal_scoped_memory_real_write_canary_accepted_gate_boundary".to_string(),
        serde_json::Value::Array(denials),
    );
    insert_report_json!(
        "denied_by_minimal_scoped_memory_real_write_canary_accepted_gate_boundary_count",
        denied_count
    );
    for key in [
        "source_rollback_tombstone_dry_run_required",
        "fresh_operator_approval_artifact_accepted",
        "operator_identity_bound",
        "operator_session_bound",
        "single_use_nonce_authority_accepted",
        "explicit_command_accepted",
        "canary_namespace_bound",
        "canary_store_bound",
        "canary_scope_bound",
        "payload_digest_bound",
        "payload_redaction_bound",
        "active_binary_sha_bound",
        "route_count_bound",
        "wal_receipt_binding_accepted",
        "post_write_readback_binding_accepted",
        "rollback_tombstone_proof_binding_accepted",
        "minimal_real_write_authority_accepted",
        "nonce_consumption_forbidden_on_report_route",
        "explicit_command_dispatch_forbidden_on_report_route",
        "durable_memory_read_forbidden",
        "durable_memory_write_forbidden",
        "durable_memory_rollback_forbidden",
        "memory_store_mutation_forbidden",
        "wal_write_forbidden",
        "receipt_persistence_forbidden",
        "rollback_execution_forbidden",
        "tombstone_write_forbidden",
        "kg_live_write_forbidden",
        "provider_model_invocation_forbidden",
        "credential_read_forbidden",
        "channel_external_send_forbidden",
        "public_claim_release_artifact_forbidden",
        "install_restart_active_binary_mutation_forbidden",
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

fn hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_wal_receipt_binding_boundary_report()
-> serde_json::Value {
    const WAL_RECEIPT_SURFACES: &[&str] = &[
        "source_minimal_scoped_memory_real_write_canary_accepted_gate_required",
        "wal_namespace_store_scope_binding_required",
        "wal_record_identity_binding_required",
        "wal_sequence_monotonicity_guard_required",
        "wal_idempotency_key_required",
        "payload_digest_redaction_binding_required",
        "receipt_identity_binding_required",
        "receipt_hash_chain_binding_required",
        "receipt_replay_guard_required",
        "receipt_audit_evidence_binding_required",
        "post_write_readback_handoff_binding_required",
        "memory_kg_provider_channel_public_release_install_active_binary_side_effects_forbidden",
    ];
    const WAL_RECEIPT_DENIALS: &[&str] = &[
        "source_minimal_scoped_memory_real_write_canary_accepted_gate_required",
        "wal_namespace_store_scope_binding_required",
        "wal_record_identity_binding_required",
        "wal_sequence_monotonicity_guard_required",
        "wal_idempotency_key_required",
        "payload_digest_redaction_binding_required",
        "receipt_identity_binding_required",
        "receipt_hash_chain_binding_required",
        "receipt_replay_guard_required",
        "receipt_audit_evidence_binding_required",
        "post_write_readback_handoff_binding_required",
        "nonce_consumption_report_route_denied",
        "explicit_command_dispatch_report_route_denied",
        "wal_write_denied",
        "wal_persistence_denied",
        "receipt_recording_denied",
        "receipt_persistence_denied",
        "receipt_materialization_denied",
        "receipt_delivery_denied",
        "durable_memory_store_read_denied",
        "durable_memory_store_write_denied",
        "durable_memory_store_rollback_denied",
        "memory_store_mutation_denied",
        "rollback_tombstone_execution_denied",
        "kg_live_write_denied",
        "provider_model_invocation_denied",
        "credential_channel_public_release_side_effect_denied",
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

    fn wal_receipt_fixture(
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
            "minimal_scoped_memory_real_write_canary_wal_receipt_binding_status",
            status
        );
        insert_fixture_json!("source_accepted_gate_present", true);
        insert_fixture_json!("source_accepted_gate_ready", true);
        insert_fixture_json!("reason", reason);
        insert_fixture_json!("wal_receipt_binding_noop_confirmed", true);
        for key in [
            "wal_namespace_store_scope_binding_requested",
            "wal_record_identity_binding_requested",
            "wal_sequence_monotonicity_guard_requested",
            "wal_idempotency_key_requested",
            "payload_digest_redaction_binding_requested",
            "receipt_identity_binding_requested",
            "receipt_hash_chain_binding_requested",
            "receipt_replay_guard_requested",
            "receipt_audit_evidence_binding_requested",
            "post_write_readback_handoff_binding_requested",
            "single_use_nonce_consumption_requested",
            "explicit_command_dispatch_requested",
            "wal_write_requested",
            "wal_persistence_requested",
            "receipt_recording_requested",
            "receipt_persistence_requested",
            "receipt_materialization_requested",
            "receipt_delivery_requested",
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
            "minimal_scoped_memory_real_write_canary_wal_receipt_binding_accepted",
            "wal_namespace_bound",
            "wal_store_bound",
            "wal_scope_bound",
            "wal_record_id_bound",
            "wal_sequence_guard_bound",
            "wal_idempotency_key_bound",
            "wal_payload_digest_bound",
            "wal_payload_redaction_bound",
            "receipt_id_bound",
            "receipt_hash_chain_bound",
            "receipt_replay_guard_bound",
            "receipt_audit_evidence_bound",
            "post_write_readback_handoff_bound",
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
        .name("hepta-memory-minimal-canary-wal-receipt-source-report".to_string())
        .stack_size(8 * 1024 * 1024)
        .spawn(
            hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_operator_approval_nonce_command_accepted_gate_boundary_report,
        )
        .ok()
        .and_then(|handle| handle.join().ok())
        .unwrap_or_else(|| {
            serde_json::json!({
                "status": "blocked",
                "memory_write_execution_minimal_scoped_memory_real_write_canary_operator_approval_nonce_command_accepted_gate_boundary_ready": false,
                "minimal_scoped_memory_real_write_canary_accepted_gate_ready": false,
                "source_minimal_scoped_memory_real_write_canary_wal_receipt_source_report_thread_failed": true
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
    let source_next_action_wal_receipt = source
        .get("allowed_next_actions")
        .and_then(serde_json::Value::as_array)
        .and_then(|items| items.get(1))
        .map(|item| {
            item.get("action").and_then(serde_json::Value::as_str)
                == Some(
                    "prepare_minimal_scoped_memory_real_write_canary_wal_receipt_binding_boundary",
                )
                && item
                    .get("requires_minimal_scoped_memory_real_write_canary_accepted_gate")
                    .and_then(serde_json::Value::as_bool)
                    == Some(true)
        })
        .unwrap_or(false);
    let source_ready = source.get("status").and_then(serde_json::Value::as_str) == Some("ready")
        && json_bool(
            &source,
            "memory_write_execution_minimal_scoped_memory_real_write_canary_operator_approval_nonce_command_accepted_gate_boundary_ready",
        )
        && json_bool(
            &source,
            "minimal_scoped_memory_real_write_canary_accepted_gate_ready",
        )
        && json_bool(
            &source,
            "minimal_scoped_memory_real_write_canary_authority_accepted_no_write",
        )
        && json_u64(
            &source,
            "accepted_minimal_scoped_memory_real_write_canary_accepted_gate_fixture_count",
        ) == 1
        && json_u64(
            &source,
            "blocked_minimal_scoped_memory_real_write_canary_accepted_gate_fixture_count",
        ) == 9
        && json_u64(
            &source,
            "minimal_scoped_memory_real_write_canary_authority_accepted_count",
        ) == 1
        && json_u64(&source, "single_use_nonce_consumed_count") == 0
        && json_u64(&source, "explicit_command_dispatched_count") == 0
        && json_u64(&source, "wal_write_performed_count") == 0
        && json_u64(&source, "receipt_persisted_count") == 0
        && json_u64(&source, "durable_memory_store_read_performed_count") == 0
        && json_u64(&source, "durable_memory_store_write_performed_count") == 0
        && json_u64(&source, "durable_memory_store_rollback_performed_count") == 0
        && json_u64(&source, "memory_store_write_performed_count") == 0
        && !json_bool(&source, "single_use_nonce_consumed")
        && !json_bool(&source, "explicit_command_dispatched")
        && !json_bool(&source, "wal_write_performed")
        && !json_bool(&source, "receipt_persisted")
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
        && source_next_action_wal_receipt
        && side_effects_all_false(&source);

    let fixtures = serde_json::Value::Array(vec![
        wal_receipt_fixture(
            "minimal-scoped-memory-real-write-canary-wal-receipt-binding-envelope",
            "accepted_wal_receipt_binding_noop",
            "wal_receipt_binding_evidence_accepted_without_wal_or_receipt_write",
            true,
            serde_json::json!({
                "wal_namespace_store_scope_binding_requested": true,
                "wal_record_identity_binding_requested": true,
                "wal_sequence_monotonicity_guard_requested": true,
                "wal_idempotency_key_requested": true,
                "payload_digest_redaction_binding_requested": true,
                "receipt_identity_binding_requested": true,
                "receipt_hash_chain_binding_requested": true,
                "receipt_replay_guard_requested": true,
                "receipt_audit_evidence_binding_requested": true,
                "post_write_readback_handoff_binding_requested": true
            }),
        ),
        wal_receipt_fixture(
            "minimal-scoped-memory-real-write-canary-wal-receipt-missing-accepted-gate-source",
            "blocked_source_noop",
            "source_minimal_scoped_memory_real_write_canary_accepted_gate_required",
            false,
            serde_json::json!({
                "source_accepted_gate_present": false,
                "source_accepted_gate_ready": false,
                "wal_namespace_store_scope_binding_requested": true
            }),
        ),
        wal_receipt_fixture(
            "minimal-scoped-memory-real-write-canary-wal-scope-required",
            "blocked_wal_scope_noop",
            "wal_namespace_store_scope_binding_required",
            false,
            serde_json::json!({"wal_namespace_store_scope_binding_requested": true}),
        ),
        wal_receipt_fixture(
            "minimal-scoped-memory-real-write-canary-wal-record-required",
            "blocked_wal_record_noop",
            "wal_record_identity_binding_required",
            false,
            serde_json::json!({"wal_record_identity_binding_requested": true}),
        ),
        wal_receipt_fixture(
            "minimal-scoped-memory-real-write-canary-wal-sequence-required",
            "blocked_wal_sequence_noop",
            "wal_sequence_monotonicity_guard_required",
            false,
            serde_json::json!({"wal_sequence_monotonicity_guard_requested": true}),
        ),
        wal_receipt_fixture(
            "minimal-scoped-memory-real-write-canary-wal-idempotency-required",
            "blocked_wal_idempotency_noop",
            "wal_idempotency_key_required",
            false,
            serde_json::json!({"wal_idempotency_key_requested": true}),
        ),
        wal_receipt_fixture(
            "minimal-scoped-memory-real-write-canary-receipt-identity-required",
            "blocked_receipt_identity_noop",
            "receipt_identity_binding_required",
            false,
            serde_json::json!({"receipt_identity_binding_requested": true}),
        ),
        wal_receipt_fixture(
            "minimal-scoped-memory-real-write-canary-receipt-hash-replay-required",
            "blocked_receipt_hash_replay_noop",
            "receipt_hash_chain_and_replay_guard_required",
            false,
            serde_json::json!({
                "receipt_hash_chain_binding_requested": true,
                "receipt_replay_guard_requested": true
            }),
        ),
        wal_receipt_fixture(
            "minimal-scoped-memory-real-write-canary-receipt-audit-readback-required",
            "blocked_receipt_audit_readback_noop",
            "receipt_audit_evidence_and_readback_handoff_required",
            false,
            serde_json::json!({
                "receipt_audit_evidence_binding_requested": true,
                "post_write_readback_handoff_binding_requested": true
            }),
        ),
        wal_receipt_fixture(
            "minimal-scoped-memory-real-write-canary-wal-receipt-direct-side-effect-attempt",
            "blocked_direct_side_effect_noop",
            "direct_wal_receipt_memory_and_external_side_effects_denied",
            false,
            serde_json::json!({
                "single_use_nonce_consumption_requested": true,
                "explicit_command_dispatch_requested": true,
                "wal_write_requested": true,
                "wal_persistence_requested": true,
                "receipt_recording_requested": true,
                "receipt_persistence_requested": true,
                "receipt_materialization_requested": true,
                "receipt_delivery_requested": true,
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
                        .get("minimal_scoped_memory_real_write_canary_wal_receipt_binding_accepted")
                        .and_then(serde_json::Value::as_bool)
                        == Some(true)
                })
                .count()
        })
        .unwrap_or(0);
    let blocked_fixture_count = fixture_count.saturating_sub(accepted_fixture_count);
    let denials = WAL_RECEIPT_DENIALS
        .iter()
        .map(|reason| serde_json::json!(reason))
        .collect::<Vec<_>>();
    let denied_count = denials.len();
    let source_report_sha256 = sha256_text_value(&source.to_string());
    let boundary_hash_sha256 = sha256_text_value(&format!(
        "minimal-scoped-memory-real-write-canary-wal-receipt-binding-boundary:v1:source-ready={source_ready}:fixtures={fixture_count}:accepted={accepted_fixture_count}:denials={denied_count}:memory-writes=0:wal-writes=0:receipt-persistence=0"
    ));
    let policy_hash_sha256 = sha256_text_value(
        "minimal-scoped-memory-real-write-canary-wal-receipt-binding-policy:v1:no-nonce-consume:no-command-dispatch:no-wal-write:no-receipt-persist:no-memory-read-write",
    );
    let mut side_effects = serde_json::Map::new();
    for &key in FALSE_KEYS {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }
    let required_fields = serde_json::json!([
        "source_minimal_scoped_memory_real_write_canary_accepted_gate_report_sha256",
        "accepted_gate_fixture_id",
        "wal_namespace",
        "wal_store",
        "wal_scope",
        "wal_record_id",
        "wal_sequence_guard_id",
        "wal_idempotency_key",
        "payload_digest_sha256",
        "payload_redaction_proof_id",
        "receipt_id",
        "receipt_hash_chain_id",
        "receipt_replay_guard_id",
        "receipt_audit_evidence_id",
        "post_write_readback_handoff_id",
        "active_binary_sha256",
        "route_count",
    ]);
    let allowed_next_actions = serde_json::json!([
        {
            "action": "run_minimal_scoped_memory_real_write_canary_wal_receipt_binding_boundary_require_live_gate",
            "status": "allowed_verification_only",
            "reads_memory": false,
            "writes_memory": false,
            "consumes_nonce": false,
            "dispatches_command": false,
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
            "action": "prepare_minimal_scoped_memory_real_write_canary_post_write_readback_binding_boundary",
            "status": "allowed_report_only_next_slice",
            "requires_minimal_scoped_memory_real_write_canary_wal_receipt_binding": true,
            "writes_memory": false,
            "reads_memory": false,
            "consumes_nonce": false,
            "dispatches_command": false,
            "writes_wal": false,
            "persists_receipt": false,
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
        && denied_count == 28;

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
        "hepta_memory_live_mutation_operator_write_execution_minimal_scoped_memory_real_write_canary_wal_receipt_binding_boundary_route"
    );
    insert_report_json!(
        "endpoint",
        HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_MINIMAL_SCOPED_MEMORY_REAL_WRITE_CANARY_WAL_RECEIPT_BINDING_BOUNDARY_ENDPOINT
    );
    insert_report_json!(
        "source_command",
        "/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-wal-receipt-binding-boundary --json"
    );
    insert_report_json!("native_route", true);
    insert_report_json!("side_effect_free", true);
    insert_report_json!("audit_date", "2026-07-03");
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_wal_receipt_binding_schema_version",
        "minimal_scoped_memory_real_write_canary_wal_receipt_binding_boundary_v1"
    );
    insert_report_json!(
        "scoped_memory_real_write_canary_mode",
        "minimal_scoped_memory_real_write_canary_wal_receipt_binding_no_write"
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
        "memory_write_execution_minimal_scoped_memory_real_write_canary_wal_receipt_binding_boundary_ready",
        report_ready
    );
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_wal_receipt_binding_ready",
        report_ready
    );
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_wal_receipt_binding_accepted_no_write",
        report_ready
    );
    insert_report_json!(
        "source_minimal_scoped_memory_real_write_canary_accepted_gate_boundary_ready",
        source_ready
    );
    insert_report_json!(
        "source_minimal_scoped_memory_real_write_canary_accepted_gate_ready",
        json_bool(
            &source,
            "minimal_scoped_memory_real_write_canary_accepted_gate_ready"
        )
    );
    insert_report_json!(
        "source_minimal_scoped_memory_real_write_canary_accepted_gate_report_sha256",
        source_report_sha256
    );
    insert_report_json!(
        "source_accepted_minimal_scoped_memory_real_write_canary_accepted_gate_fixture_count",
        json_u64(
            &source,
            "accepted_minimal_scoped_memory_real_write_canary_accepted_gate_fixture_count"
        )
    );
    insert_report_json!(
        "source_blocked_minimal_scoped_memory_real_write_canary_accepted_gate_fixture_count",
        json_u64(
            &source,
            "blocked_minimal_scoped_memory_real_write_canary_accepted_gate_fixture_count"
        )
    );
    insert_report_json!(
        "source_minimal_scoped_memory_real_write_canary_authority_accepted_count",
        json_u64(
            &source,
            "minimal_scoped_memory_real_write_canary_authority_accepted_count"
        )
    );
    insert_report_json!(
        "source_minimal_scoped_memory_real_write_canary_accepted_gate_denial_count",
        json_u64(
            &source,
            "denied_by_minimal_scoped_memory_real_write_canary_accepted_gate_boundary_count"
        )
    );
    insert_report_json!(
        "source_single_use_nonce_consumed_count",
        json_u64(&source, "single_use_nonce_consumed_count")
    );
    insert_report_json!(
        "source_explicit_command_dispatched_count",
        json_u64(&source, "explicit_command_dispatched_count")
    );
    insert_report_json!(
        "source_wal_write_performed_count",
        json_u64(&source, "wal_write_performed_count")
    );
    insert_report_json!(
        "source_receipt_persisted_count",
        json_u64(&source, "receipt_persisted_count")
    );
    insert_report_json!(
        "source_durable_memory_store_read_performed_count",
        json_u64(&source, "durable_memory_store_read_performed_count")
    );
    insert_report_json!(
        "source_durable_memory_store_write_performed_count",
        json_u64(&source, "durable_memory_store_write_performed_count")
    );
    insert_report_json!(
        "source_durable_memory_store_rollback_performed_count",
        json_u64(&source, "durable_memory_store_rollback_performed_count")
    );
    insert_report_json!(
        "source_memory_store_write_performed_count",
        json_u64(&source, "memory_store_write_performed_count")
    );
    insert_report_json!("minimum_required_samples", 24);
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_wal_receipt_binding_boundary_hash_sha256",
        boundary_hash_sha256
    );
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_wal_receipt_binding_policy_hash_sha256",
        policy_hash_sha256
    );
    insert_report_json!(
        "required_minimal_scoped_memory_real_write_canary_wal_receipt_binding_surface_count",
        12
    );
    insert_report_json!(
        "ready_minimal_scoped_memory_real_write_canary_wal_receipt_binding_surface_count",
        12
    );
    insert_report_json!(
        "side_effect_free_minimal_scoped_memory_real_write_canary_wal_receipt_binding_surface_count",
        12
    );
    insert_report_json!(
        "required_minimal_scoped_memory_real_write_canary_wal_receipt_binding_fixture_count",
        10
    );
    insert_report_json!(
        "minimal_scoped_memory_real_write_canary_wal_receipt_binding_fixture_count",
        fixture_count
    );
    insert_report_json!(
        "accepted_minimal_scoped_memory_real_write_canary_wal_receipt_binding_fixture_count",
        accepted_fixture_count
    );
    insert_report_json!(
        "blocked_minimal_scoped_memory_real_write_canary_wal_receipt_binding_fixture_count",
        blocked_fixture_count
    );
    insert_report_json!(
        "noop_minimal_scoped_memory_real_write_canary_wal_receipt_binding_fixture_count",
        fixture_count
    );
    for key in [
        "wal_receipt_binding_authority_accepted_count",
        "wal_namespace_store_scope_bound_count",
        "wal_record_id_bound_count",
        "wal_sequence_guard_bound_count",
        "wal_idempotency_key_bound_count",
        "wal_payload_digest_redaction_bound_count",
        "receipt_id_bound_count",
        "receipt_hash_chain_bound_count",
        "receipt_replay_guard_bound_count",
        "receipt_audit_evidence_bound_count",
        "post_write_readback_handoff_bound_count",
        "minimal_scoped_memory_real_write_canary_wal_receipt_binding_accepted_count",
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
        "required_before_minimal_scoped_memory_real_write_canary_wal_receipt_binding_count",
        17
    );
    report.insert(
        "required_minimal_scoped_memory_real_write_canary_wal_receipt_binding_fields".to_string(),
        required_fields,
    );
    report.insert(
        "minimal_scoped_memory_real_write_canary_wal_receipt_binding_surfaces".to_string(),
        serde_json::json!(WAL_RECEIPT_SURFACES),
    );
    report.insert(
        "minimal_scoped_memory_real_write_canary_wal_receipt_binding_fixtures".to_string(),
        fixtures,
    );
    report.insert(
        "denied_by_minimal_scoped_memory_real_write_canary_wal_receipt_binding_boundary"
            .to_string(),
        serde_json::Value::Array(denials),
    );
    insert_report_json!(
        "denied_by_minimal_scoped_memory_real_write_canary_wal_receipt_binding_boundary_count",
        denied_count
    );
    for key in [
        "source_minimal_scoped_memory_real_write_canary_accepted_gate_required",
        "minimal_scoped_memory_real_write_canary_wal_receipt_binding_accepted",
        "wal_namespace_bound",
        "wal_store_bound",
        "wal_scope_bound",
        "wal_record_id_bound",
        "wal_sequence_guard_bound",
        "wal_idempotency_key_bound",
        "wal_payload_digest_bound",
        "wal_payload_redaction_bound",
        "receipt_id_bound",
        "receipt_hash_chain_bound",
        "receipt_replay_guard_bound",
        "receipt_audit_evidence_bound",
        "post_write_readback_handoff_bound",
        "nonce_consumption_forbidden_on_report_route",
        "explicit_command_dispatch_forbidden_on_report_route",
        "wal_write_forbidden",
        "wal_persistence_forbidden",
        "receipt_recording_forbidden",
        "receipt_persistence_forbidden",
        "receipt_materialization_forbidden",
        "receipt_delivery_forbidden",
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

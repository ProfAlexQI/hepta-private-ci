fn hepta_memory_live_mutation_operator_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_boundary_report()
-> serde_json::Value {
    const RELEASE_ARTIFACT_PUBLICATION_SURFACES: &[&str] = &[
        "source_terminal_operator_decision_public_claim_report_required",
        "release_artifact_publication_denied",
        "release_artifact_write_denied",
        "public_artifact_write_denied",
        "artifact_signature_notarization_denied",
        "publication_queue_enqueue_denied",
        "publication_manifest_write_denied",
        "public_distribution_channel_delivery_denied",
        "public_version_tag_release_promotion_denied",
        "release_notes_changelog_materialization_denied",
        "terminal_operator_decision_is_not_release_approval",
        "activation_from_release_artifact_publication_denied",
        "dry_run_production_memory_release_publication_denied",
        "external_public_install_restart_active_binary_publication_denied",
    ];
    const RELEASE_ARTIFACT_PUBLICATION_DENIALS: &[&str] = &[
        "source_terminal_operator_decision_public_claim_report_required",
        "release_artifact_publication_denied",
        "release_artifact_write_denied",
        "public_artifact_write_denied",
        "artifact_signature_notarization_denied",
        "publication_queue_enqueue_denied",
        "publication_manifest_write_denied",
        "public_distribution_channel_delivery_denied",
        "public_version_tag_release_promotion_denied",
        "release_notes_changelog_materialization_denied",
        "terminal_operator_decision_is_not_release_approval",
        "terminal_operator_decision_release_approval_promotion_denied",
        "activation_from_release_artifact_publication_denied",
        "dry_run_execution_release_publication_denied",
        "production_memory_write_release_publication_denied",
        "memory_write_publication_denied",
        "kg_write_publication_denied",
        "provider_prompt_publication_denied",
        "channel_delivery_publication_denied",
        "install_restart_active_binary_publication_denied",
    ];

    fn release_artifact_publication_fixture(
        id: &str,
        status: &str,
        accepted: bool,
        reason: &str,
        extra: serde_json::Value,
    ) -> serde_json::Value {
        let mut fixture = serde_json::Map::new();
        macro_rules! insert_fixture_json {
            ($key:literal, $value:expr) => {
                fixture.insert($key.to_string(), serde_json::json!($value));
            };
        }
        insert_fixture_json!("id", id);
        insert_fixture_json!("fixture_id", id);
        insert_fixture_json!(
            "scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_status",
            status
        );
        insert_fixture_json!(
            "scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_accepted",
            accepted
        );
        insert_fixture_json!(
            "source_terminal_operator_decision_public_claim_present",
            true
        );
        insert_fixture_json!("source_terminal_operator_decision_public_claim_ready", true);
        insert_fixture_json!("release_artifact_publication_denial_noop_confirmed", true);
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
            "dry_run_execution_publication_requested",
            "production_memory_write_publication_requested",
            "memory_write_publication_requested",
            "kg_write_publication_requested",
            "provider_prompt_publication_requested",
            "credential_publication_requested",
            "install_publication_requested",
            "service_restart_publication_requested",
            "active_binary_publication_requested",
        ] {
            fixture.insert(key.to_string(), serde_json::json!(false));
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
            "activation_allowed",
            "activation_performed",
            "dry_run_execution_executed",
            "production_durable_memory_write_executed",
            "production_durable_memory_store_write_performed",
            "memory_write_execution_performed",
            "memory_store_write_performed",
            "memory_store_mutated",
            "wal_write_performed",
            "rollback_executed",
            "live_kg_write_performed",
            "provider_invoked",
            "model_invoked",
            "credential_read",
            "secret_file_read",
            "install_executed",
            "launchd_mutated",
            "service_restarted",
            "service_restart_performed",
            "active_binary_mutated",
        ] {
            fixture.insert(key.to_string(), serde_json::json!(false));
        }
        if let Some(extra) = extra.as_object() {
            for (key, value) in extra {
                fixture.insert(key.clone(), value.clone());
            }
        }
        serde_json::Value::Object(fixture)
    }

    let route_matrix = control_ui_route_parity_report();
    let source = std::thread::Builder::new()
        .name("hepta-scoped-production-memory-terminal-decision-source-report".to_string())
        .stack_size(8 * 1024 * 1024)
        .spawn(hepta_memory_live_mutation_operator_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_boundary_report)
        .ok()
        .and_then(|handle| handle.join().ok())
        .unwrap_or_else(|| {
            serde_json::json!({
                "status": "blocked",
                "memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_boundary_ready": false,
                "source_terminal_operator_decision_public_claim_report_thread_failed": true
            })
        });

    let source_bool = |key: &str| {
        source
            .get(key)
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
    };
    let source_u64 = |key: &str| {
        source
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let source_str = |key: &str| {
        source
            .get(key)
            .and_then(serde_json::Value::as_str)
            .unwrap_or("")
            .to_string()
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
                    "prepare_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_boundary",
                )
                && item
                    .get("writes_release_artifact")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && item
                    .get("claims_public_release")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
        })
        .unwrap_or(false);
    let source_ready = source_str("status") == "ready"
        && source_bool(
            "memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_boundary_ready",
        )
        && source_bool(
            "scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_accepted",
        )
        && source_u64(
            "accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_fixture_count",
        ) == 1
        && source_u64(
            "blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_fixture_count",
        ) == 9
        && source_u64(
            "denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_boundary_count",
        ) >= 80
        && !source_bool("dry_run_execution_result_receipt_terminal_operator_decision_recorded")
        && !source_bool("dry_run_execution_result_receipt_terminal_operator_decision_accepted")
        && !source_bool("dry_run_execution_result_receipt_terminal_operator_status_recorded")
        && !source_bool("dry_run_execution_result_receipt_public_claim_promoted")
        && !source_bool("dry_run_execution_result_receipt_public_release_published")
        && !source_bool("dry_run_execution_result_receipt_release_artifact_written")
        && !source_bool(
            "dry_run_execution_result_receipt_authority_promoted_from_terminal_operator_decision",
        )
        && !source_bool("dry_run_execution_executed")
        && !source_bool("production_durable_memory_store_write_performed")
        && !source_bool("memory_store_write_performed")
        && !source_bool("wal_write_performed")
        && !source_bool("receipt_persisted")
        && !source_bool("live_kg_write_performed")
        && !source_bool("provider_invoked")
        && !source_bool("model_invoked")
        && !source_bool("credential_read")
        && !source_bool("channel_send_performed")
        && !source_bool("external_send_performed")
        && !source_bool("install_executed")
        && !source_bool("service_restarted")
        && !source_bool("active_binary_mutated")
        && source_next_action_release_artifact;

    let fixtures = serde_json::Value::Array(vec![
        release_artifact_publication_fixture(
            "scoped-production-durable-memory-write-dry-run-result-receipt-release-artifact-publication-report-only-binding",
            "accepted_report_only",
            true,
            "release_artifact_publication_denial_matrix_bound_without_publication_or_release_artifact",
            serde_json::json!({}),
        ),
        release_artifact_publication_fixture(
            "scoped-production-durable-memory-write-dry-run-result-receipt-release-artifact-publication-missing-terminal-decision-public-claim",
            "blocked_noop",
            false,
            "source_terminal_operator_decision_public_claim_report_required",
            serde_json::json!({
                "source_terminal_operator_decision_public_claim_present": false,
                "source_terminal_operator_decision_public_claim_ready": false,
                "release_artifact_publication_requested": true
            }),
        ),
        release_artifact_publication_fixture(
            "scoped-production-durable-memory-write-dry-run-result-receipt-release-artifact-write-request",
            "blocked_artifact_noop",
            false,
            "release_artifact_write_denied",
            serde_json::json!({
                "release_artifact_write_requested": true,
                "release_artifact_publication_requested": true
            }),
        ),
        release_artifact_publication_fixture(
            "scoped-production-durable-memory-write-dry-run-result-receipt-public-artifact-write-request",
            "blocked_artifact_noop",
            false,
            "public_artifact_write_denied",
            serde_json::json!({
                "public_artifact_write_requested": true,
                "release_artifact_publication_requested": true
            }),
        ),
        release_artifact_publication_fixture(
            "scoped-production-durable-memory-write-dry-run-result-receipt-artifact-signature-notarization-request",
            "blocked_artifact_noop",
            false,
            "artifact_signature_notarization_denied",
            serde_json::json!({
                "artifact_signature_requested": true,
                "artifact_notarization_requested": true,
                "release_artifact_publication_requested": true
            }),
        ),
        release_artifact_publication_fixture(
            "scoped-production-durable-memory-write-dry-run-result-receipt-publication-queue-manifest-request",
            "blocked_publication_noop",
            false,
            "publication_queue_manifest_denied",
            serde_json::json!({
                "publication_queue_enqueue_requested": true,
                "publication_manifest_write_requested": true
            }),
        ),
        release_artifact_publication_fixture(
            "scoped-production-durable-memory-write-dry-run-result-receipt-distribution-channel-request",
            "blocked_distribution_noop",
            false,
            "public_distribution_channel_delivery_denied",
            serde_json::json!({
                "public_distribution_requested": true,
                "telegram_delivery_requested": true,
                "channel_delivery_requested": true,
                "external_delivery_requested": true
            }),
        ),
        release_artifact_publication_fixture(
            "scoped-production-durable-memory-write-dry-run-result-receipt-public-version-tag-release-request",
            "blocked_release_noop",
            false,
            "public_version_tag_release_promotion_denied",
            serde_json::json!({
                "public_version_tag_requested": true,
                "public_release_publish_requested": true,
                "public_ga_claim_requested": true
            }),
        ),
        release_artifact_publication_fixture(
            "scoped-production-durable-memory-write-dry-run-result-receipt-release-notes-changelog-terminal-approval-request",
            "blocked_release_noop",
            false,
            "release_notes_changelog_and_terminal_release_approval_denied",
            serde_json::json!({
                "release_notes_materialization_requested": true,
                "changelog_materialization_requested": true,
                "terminal_operator_decision_release_approval_requested": true,
                "release_artifact_publication_requested": true
            }),
        ),
        release_artifact_publication_fixture(
            "scoped-production-durable-memory-write-dry-run-result-receipt-release-publication-activation-memory-provider-install",
            "blocked_authority_noop",
            false,
            "activation_memory_provider_install_restart_active_binary_publication_denied",
            serde_json::json!({
                "activation_from_release_publication_requested": true,
                "dry_run_execution_publication_requested": true,
                "production_memory_write_publication_requested": true,
                "memory_write_publication_requested": true,
                "kg_write_publication_requested": true,
                "provider_prompt_publication_requested": true,
                "credential_publication_requested": true,
                "install_publication_requested": true,
                "service_restart_publication_requested": true,
                "active_binary_publication_requested": true
            }),
        ),
    ]);

    let source_report_sha256 = sha256_json_value(&source);
    let fixtures_hash_sha256 = sha256_json_value(&fixtures);
    let release_artifact_publication_denial_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-result-receipt-release-artifact-publication-denial:v1:source={}:publication=false:artifact=false:public=false",
        source_str(
            "dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_result_hash_sha256"
        )
    ));
    let release_artifact_write_denial_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-result-receipt-release-artifact-write-denial:v1:publication={release_artifact_publication_denial_hash_sha256}:release-artifact=false:public-artifact=false:signature=false:notarization=false"
    ));
    let release_artifact_publication_matrix_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-result-receipt-release-artifact-publication-matrix:v1:publication={release_artifact_publication_denial_hash_sha256}:artifact={release_artifact_write_denial_hash_sha256}:fixtures={fixtures_hash_sha256}"
    ));
    let release_artifact_publication_handoff_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-result-receipt-release-artifact-publication-handoff:v1:matrix={release_artifact_publication_matrix_hash_sha256}:next=release-artifact-publication-result-receipt-no-persistence-boundary"
    ));
    let release_artifact_publication_result_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-result-receipt-release-artifact-publication-result:v1:publication={release_artifact_publication_denial_hash_sha256}:artifact={release_artifact_write_denial_hash_sha256}:handoff={release_artifact_publication_handoff_hash_sha256}:accepted=true:release=false:public=false:activation=false:execution=false:production-write=false"
    ));
    let release_artifact_publication_boundary_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-result-receipt-release-artifact-publication-denial-boundary:v1:source={source_report_sha256}:result={release_artifact_publication_result_hash_sha256}:accepted=1:blocked=9:publication=false:artifact=false:authority=false:execution=false:production-write=false"
    ));
    let release_artifact_publication_policy_hash_sha256 = sha256_text_value(
        "scoped-production-durable-memory-write-dry-run-result-receipt-release-artifact-publication-denial-policy:v1:no-release-artifact-no-public-artifact-no-signature-no-notarization-no-publication-queue-no-manifest-no-distribution-no-public-release-no-terminal-release-approval-no-activation-no-execution-no-production-write-no-kg-no-provider-no-install",
    );

    let mut denials = source
        .get("denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_boundary")
        .and_then(serde_json::Value::as_array)
        .cloned()
        .unwrap_or_default();
    for &denial in RELEASE_ARTIFACT_PUBLICATION_DENIALS {
        denials.push(serde_json::json!(denial));
    }
    let denial_count = denials.len();

    let report_ready = source_ready
        && route_count_source_command_accepted
        && RELEASE_ARTIFACT_PUBLICATION_SURFACES.len() == 14
        && RELEASE_ARTIFACT_PUBLICATION_DENIALS.len() == 20
        && fixtures.as_array().map(Vec::len) == Some(10)
        && denial_count >= 100;

    let mut side_effects = serde_json::Map::new();
    side_effects.insert(
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_boundary_performed".to_string(),
        serde_json::json!(true),
    );
    side_effects.insert(
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_result_accepted".to_string(),
        serde_json::json!(true),
    );
    for key in [
        "dry_run_execution_result_receipt_release_artifact_publication_requested",
        "dry_run_execution_result_receipt_release_artifact_publication_recorded",
        "dry_run_execution_result_receipt_release_artifact_publication_accepted",
        "dry_run_execution_result_receipt_release_artifact_publication_persisted",
        "dry_run_execution_result_receipt_release_artifact_publication_materialized",
        "dry_run_execution_result_receipt_release_artifact_written",
        "dry_run_execution_result_receipt_public_artifact_written",
        "dry_run_execution_result_receipt_artifact_signature_accepted",
        "dry_run_execution_result_receipt_artifact_notarization_accepted",
        "dry_run_execution_result_receipt_publication_queue_enqueued",
        "dry_run_execution_result_receipt_publication_manifest_written",
        "dry_run_execution_result_receipt_public_distribution_performed",
        "dry_run_execution_result_receipt_public_version_tag_created",
        "dry_run_execution_result_receipt_release_notes_materialized",
        "dry_run_execution_result_receipt_changelog_materialized",
        "dry_run_execution_result_receipt_public_claim_promoted",
        "dry_run_execution_result_receipt_public_ga_claimed",
        "dry_run_execution_result_receipt_public_release_published",
        "dry_run_execution_result_receipt_terminal_operator_decision_promoted_to_release_approval",
        "dry_run_execution_result_receipt_authority_promoted_from_release_artifact_publication",
        "activation_allowed_by_release_artifact_publication",
        "activation_allowed_by_terminal_operator_decision",
        "activation_allowed_by_result_receipt",
        "activation_allowed",
        "activation_performed",
        "dry_run_execution_executed",
        "production_durable_memory_write_executed",
        "production_durable_memory_store_write_performed",
        "actual_production_durable_memory_write_performed",
        "durable_memory_store_write_performed",
        "memory_write_execution_performed",
        "memory_store_write_performed",
        "memory_store_mutated",
        "wal_write_performed",
        "receipt_persisted",
        "rollback_executed",
        "tombstone_cleanup_executed",
        "live_kg_write_performed",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "telegram_send_performed",
        "channel_send_performed",
        "external_send_performed",
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
        "install_executed",
        "launchd_mutated",
        "service_restart_performed",
        "service_restarted",
        "active_binary_mutated",
        "filesystem_written",
    ] {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "endpoint": HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_SCOPED_PRODUCTION_DURABLE_MEMORY_WRITE_DRY_RUN_EXECUTION_RESULT_RECEIPT_RELEASE_ARTIFACT_PUBLICATION_DENIAL_BOUNDARY_ENDPOINT,
        "source_command": "/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-release-artifact-publication-denial-boundary --json",
        "native_route": true,
        "side_effect_free": true,
        "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        "route_count": route_matrix.route_count,
        "implemented_route_count": route_matrix.implemented_route_count,
        "missing_route_count": route_matrix.missing_route_count,
        "route_count_source_command_accepted": route_count_source_command_accepted,
        "memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_boundary_ready": report_ready,
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_ready": report_ready,
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_accepted": report_ready,
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_mode": "dry_run_execution_result_receipt_release_artifact_publication_denial_boundary_no_release_artifact_no_publication_no_authority_no_execution_no_production_durable_memory_mutation",
        "source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_boundary_ready": source_bool("memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_boundary_ready"),
        "source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_boundary_report_sha256": source_report_sha256,
        "source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_boundary_hash_sha256": source_str("scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_boundary_hash_sha256"),
        "source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_policy_hash_sha256": source_str("scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_policy_hash_sha256"),
        "source_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_result_hash_sha256": source_str("dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_result_hash_sha256"),
        "source_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_handoff_hash_sha256": source_str("dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_handoff_hash_sha256"),
        "source_accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_fixture_count": source_u64("accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_fixture_count"),
        "source_blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_fixture_count": source_u64("blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_fixture_count"),
        "source_denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_boundary_count": source_u64("denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_boundary_count"),
    });
    extend_json_object(
        &mut report,
        serde_json::json!({
        "dry_run_execution_result_receipt_release_artifact_publication_denial_hash_sha256": release_artifact_publication_denial_hash_sha256,
        "dry_run_execution_result_receipt_release_artifact_write_denial_hash_sha256": release_artifact_write_denial_hash_sha256,
        "dry_run_execution_result_receipt_release_artifact_publication_matrix_hash_sha256": release_artifact_publication_matrix_hash_sha256,
        "dry_run_execution_result_receipt_release_artifact_publication_handoff_hash_sha256": release_artifact_publication_handoff_hash_sha256,
        "dry_run_execution_result_receipt_release_artifact_publication_result_hash_sha256": release_artifact_publication_result_hash_sha256,
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_boundary_hash_sha256": release_artifact_publication_boundary_hash_sha256,
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_policy_hash_sha256": release_artifact_publication_policy_hash_sha256,
        "required_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_surface_count": 14,
        "ready_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_surface_count": 14,
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_fixture_count": 10,
        "accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_fixture_count": 1,
        "blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_fixture_count": 9,
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_fixtures": fixtures,
        "denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_boundary": denials,
        "denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_boundary_count": denial_count,
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_boundary_performed_count": 1,
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_result_accepted_count": 1,
        "source_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_boundary_bound_count": 1,
        "dry_run_execution_result_receipt_release_artifact_publication_denied_count": 1,
        "dry_run_execution_result_receipt_release_artifact_write_denied_count": 1,
        "dry_run_execution_result_receipt_public_artifact_write_denied_count": 1,
        "dry_run_execution_result_receipt_artifact_signature_notarization_denied_count": 1,
        "dry_run_execution_result_receipt_publication_queue_manifest_denied_count": 1,
        "dry_run_execution_result_receipt_public_distribution_denied_count": 1,
        "dry_run_execution_result_receipt_public_release_publication_denied_count": 1,
        "dry_run_execution_result_receipt_release_artifact_publication_authority_denied_count": 1,
        "dry_run_execution_result_receipt_release_artifact_publication_handoff_bound_count": 1,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
        "dry_run_execution_result_receipt_release_artifact_publication_recorded_count": 0,
        "dry_run_execution_result_receipt_release_artifact_publication_accepted_count": 0,
        "dry_run_execution_result_receipt_release_artifact_publication_persisted_count": 0,
        "dry_run_execution_result_receipt_release_artifact_written_count": 0,
        "dry_run_execution_result_receipt_public_artifact_written_count": 0,
        "dry_run_execution_result_receipt_artifact_signature_accepted_count": 0,
        "dry_run_execution_result_receipt_artifact_notarization_accepted_count": 0,
        "dry_run_execution_result_receipt_publication_queue_enqueued_count": 0,
        "dry_run_execution_result_receipt_publication_manifest_written_count": 0,
        "dry_run_execution_result_receipt_public_distribution_performed_count": 0,
        "dry_run_execution_result_receipt_public_release_published_count": 0,
        "dry_run_execution_result_receipt_public_ga_claimed_count": 0,
        "dry_run_execution_result_receipt_terminal_operator_decision_release_approval_count": 0,
        "dry_run_execution_result_receipt_authority_promoted_from_release_artifact_publication_count": 0,
        "activation_performed_count": 0,
        "dry_run_execution_executed_count": 0,
        "production_durable_memory_write_executed_count": 0,
        "production_durable_memory_store_write_performed_count": 0,
        "memory_store_write_performed_count": 0,
        "wal_write_performed_count": 0,
        "receipt_persisted_count": 0,
        "live_kg_write_performed_count": 0,
        "provider_invoked_count": 0,
        "model_invoked_count": 0,
        "credential_read_count": 0,
        "channel_send_performed_count": 0,
        "external_send_performed_count": 0,
        "release_artifact_written_count": 0,
        "public_artifact_written_count": 0,
        "install_executed_count": 0,
        "service_restarted_count": 0,
        "active_binary_mutated_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
        "dry_run_execution_result_receipt_release_artifact_publication_requested": false,
        "dry_run_execution_result_receipt_release_artifact_publication_recorded": false,
        "dry_run_execution_result_receipt_release_artifact_publication_accepted": false,
        "dry_run_execution_result_receipt_release_artifact_publication_persisted": false,
        "dry_run_execution_result_receipt_release_artifact_publication_materialized": false,
        "dry_run_execution_result_receipt_release_artifact_written": false,
        "dry_run_execution_result_receipt_public_artifact_written": false,
        "dry_run_execution_result_receipt_artifact_signature_accepted": false,
        "dry_run_execution_result_receipt_artifact_notarization_accepted": false,
        "dry_run_execution_result_receipt_publication_queue_enqueued": false,
        "dry_run_execution_result_receipt_publication_manifest_written": false,
        "dry_run_execution_result_receipt_public_distribution_performed": false,
        "dry_run_execution_result_receipt_public_version_tag_created": false,
        "dry_run_execution_result_receipt_release_notes_materialized": false,
        "dry_run_execution_result_receipt_changelog_materialized": false,
        "dry_run_execution_result_receipt_public_claim_promoted": false,
        "dry_run_execution_result_receipt_public_ga_claimed": false,
        "dry_run_execution_result_receipt_public_release_published": false,
        "dry_run_execution_result_receipt_terminal_operator_decision_promoted_to_release_approval": false,
        "dry_run_execution_result_receipt_authority_promoted_from_release_artifact_publication": false,
        "activation_allowed_by_release_artifact_publication": false,
        "activation_allowed_by_terminal_operator_decision": false,
        "activation_allowed_by_result_receipt": false,
        "activation_allowed": false,
        "activation_performed": false,
        "dry_run_execution_executed": false,
        "production_durable_memory_write_executed": false,
        "production_durable_memory_store_write_performed": false,
        "actual_production_durable_memory_write_performed": false,
        "durable_memory_store_write_performed": false,
        "memory_write_execution_performed": false,
        "memory_store_write_performed": false,
        "memory_store_mutated": false,
        "wal_write_performed": false,
        "receipt_persisted": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
        "rollback_executed": false,
        "tombstone_cleanup_executed": false,
        "live_kg_write_performed": false,
        "provider_invoked": false,
        "model_invoked": false,
        "credential_read": false,
        "secret_file_read": false,
        "telegram_send_performed": false,
        "channel_send_performed": false,
        "external_send_performed": false,
        "release_artifact_publication_allowed": false,
        "release_artifact_publication_requested": false,
        "release_artifact_publication_accepted": false,
        "release_artifact_publication_recorded": false,
        "release_artifact_publication_persisted": false,
        "release_artifact_publication_materialized": false,
        "release_artifact_filesystem_written": false,
        "release_artifact_written": false,
        "public_artifact_written": false,
        "artifact_signature_accepted": false,
        "artifact_notarization_accepted": false,
        "publication_queue_enqueued": false,
        "publication_manifest_written": false,
        "public_distribution_performed": false,
        "public_release_published": false,
        "public_ga_claimed": false,
        "public_claim_promoted": false,
        "public_version_tag_created": false,
        "release_notes_materialized": false,
        "changelog_materialized": false,
        "terminal_operator_decision_promoted_to_release_approval": false,
        "install_executed": false,
        "launchd_mutated": false,
        "service_restart_performed": false,
        "service_restarted": false,
        "active_binary_mutated": false,
        "filesystem_written": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
        "allowed_next_actions": [
            {
                "action": "run_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_boundary_require_live_gate",
                "status": "allowed_verification_only",
                "publishes_release_artifact": false,
                "claims_public_release": false,
                "writes_release_artifact": false,
                "writes_public_artifact": false,
                "signs_or_notarizes_artifact": false,
                "enqueues_publication": false,
                "writes_publication_manifest": false,
                "performs_public_distribution": false,
                "promotes_terminal_decision_to_release_approval": false,
                "promotes_activation_authority": false,
                "executes_dry_run": false,
                "writes_production_durable_memory": false,
                "writes_memory_or_kg": false,
                "invokes_provider": false,
                "sends_externally": false,
                "installs_or_restarts": false,
                "mutates_active_binary": false
            },
            {
                "action": "prepare_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_result_receipt_no_persistence_boundary",
                "status": "allowed_report_only_next_slice",
                "persists_publication_result_receipt": false,
                "publishes_release_artifact": false,
                "writes_release_artifact": false,
                "claims_public_release": false,
                "promotes_activation_authority": false,
                "executes_dry_run": false,
                "writes_production_durable_memory": false,
                "writes_memory_or_kg": false,
                "invokes_model": false,
                "sends_externally": false,
                "installs_or_restarts": false,
                "mutates_active_binary": false
            }
        ],
        "side_effects": side_effects
        }),
    );
    report
}

fn hepta_memory_live_mutation_operator_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_result_receipt_no_persistence_boundary_report()
-> serde_json::Value {
    const RELEASE_ARTIFACT_PUBLICATION_RESULT_RECEIPT_SURFACES: &[&str] = &[
        "source_release_artifact_publication_denial_report_required",
        "publication_result_receipt_recording_denied",
        "publication_result_receipt_persistence_denied",
        "publication_result_receipt_materialization_denied",
        "publication_result_receipt_filesystem_write_denied",
        "publication_result_receipt_ledger_index_denied",
        "publication_result_receipt_queue_delivery_denied",
        "publication_result_receipt_export_query_denied",
        "publication_result_receipt_observability_denied",
        "publication_result_receipt_signature_timestamp_status_denied",
        "publication_completion_ack_denied",
        "publication_result_receipt_authority_promotion_denied",
        "release_artifact_publication_still_denied",
        "execution_memory_kg_provider_channel_install_still_denied",
    ];
    const RELEASE_ARTIFACT_PUBLICATION_RESULT_RECEIPT_DENIALS: &[&str] = &[
        "source_release_artifact_publication_denial_report_required",
        "publication_result_receipt_recording_denied",
        "publication_result_receipt_acceptance_denied",
        "publication_result_receipt_persistence_denied",
        "publication_result_receipt_materialization_denied",
        "publication_result_receipt_filesystem_write_denied",
        "publication_result_receipt_ledger_write_denied",
        "publication_result_receipt_index_denied",
        "publication_result_receipt_queue_denied",
        "publication_result_receipt_delivery_denied",
        "publication_result_receipt_export_denied",
        "publication_result_receipt_query_registration_denied",
        "publication_result_receipt_observability_denied",
        "publication_result_receipt_signature_timestamp_status_denied",
        "publication_completion_ack_recording_denied",
        "publication_completion_ack_persistence_denied",
        "publication_result_receipt_authority_promotion_denied",
        "release_artifact_publication_result_receipt_no_persistence_only",
        "release_artifact_public_artifact_publication_remain_denied",
        "execution_memory_kg_provider_channel_install_active_binary_remain_denied",
    ];

    fn release_artifact_publication_result_receipt_fixture(
        id: &str,
        status: &str,
        accepted: bool,
        reason: &str,
        extra: serde_json::Value,
    ) -> serde_json::Value {
        let mut fixture = serde_json::Map::new();
        macro_rules! insert_fixture_json {
            ($key:literal, $value:expr) => {
                fixture.insert($key.to_string(), serde_json::json!($value));
            };
        }
        insert_fixture_json!("id", id);
        insert_fixture_json!("fixture_id", id);
        insert_fixture_json!(
            "scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_result_receipt_no_persistence_status",
            status
        );
        insert_fixture_json!(
            "scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_result_receipt_no_persistence_accepted",
            accepted
        );
        insert_fixture_json!("source_release_artifact_publication_denial_present", true);
        insert_fixture_json!("source_release_artifact_publication_denial_ready", true);
        insert_fixture_json!(
            "publication_result_receipt_no_persistence_noop_confirmed",
            true
        );
        insert_fixture_json!("reason", reason);
        for key in [
            "publication_result_receipt_requested",
            "publication_result_receipt_recording_requested",
            "publication_result_receipt_persistence_requested",
            "publication_result_receipt_materialization_requested",
            "publication_result_receipt_filesystem_write_requested",
            "publication_result_receipt_ledger_write_requested",
            "publication_result_receipt_index_requested",
            "publication_result_receipt_queue_requested",
            "publication_result_receipt_delivery_requested",
            "publication_result_receipt_export_requested",
            "publication_result_receipt_query_requested",
            "publication_result_receipt_observability_requested",
            "publication_result_receipt_signature_requested",
            "publication_result_receipt_timestamp_requested",
            "publication_result_receipt_status_requested",
            "publication_completion_ack_requested",
            "publication_result_receipt_authority_promotion_requested",
            "release_artifact_publication_retry_requested",
            "release_artifact_write_requested",
            "public_artifact_write_requested",
            "public_release_publish_requested",
            "public_ga_claim_requested",
            "dry_run_execution_publication_result_receipt_requested",
            "production_memory_write_publication_result_receipt_requested",
            "memory_write_publication_result_receipt_requested",
            "kg_write_publication_result_receipt_requested",
            "provider_prompt_publication_result_receipt_requested",
            "credential_publication_result_receipt_requested",
            "channel_delivery_publication_result_receipt_requested",
            "install_publication_result_receipt_requested",
            "service_restart_publication_result_receipt_requested",
            "active_binary_publication_result_receipt_requested",
        ] {
            fixture.insert(key.to_string(), serde_json::json!(false));
        }
        for key in [
            "publication_result_receipt_recorded",
            "publication_result_receipt_accepted",
            "publication_result_receipt_persisted",
            "publication_result_receipt_materialized",
            "publication_result_receipt_filesystem_written",
            "publication_result_receipt_ledger_written",
            "publication_result_receipt_indexed",
            "publication_result_receipt_queued",
            "publication_result_receipt_enqueued",
            "publication_result_receipt_delivered",
            "publication_result_receipt_exported",
            "publication_result_receipt_query_registered",
            "publication_result_receipt_observability_recorded",
            "publication_result_receipt_signature_accepted",
            "publication_result_receipt_timestamp_accepted",
            "publication_result_receipt_status_accepted",
            "publication_completion_ack_recorded",
            "publication_completion_ack_persisted",
            "publication_completion_ack_accepted",
            "publication_result_receipt_authority_promoted",
            "release_artifact_publication_recorded",
            "release_artifact_publication_persisted",
            "release_artifact_publication_materialized",
            "release_artifact_written",
            "public_artifact_written",
            "public_release_published",
            "public_ga_claimed",
            "activation_allowed",
            "activation_performed",
            "dry_run_execution_executed",
            "production_durable_memory_write_executed",
            "production_durable_memory_store_write_performed",
            "memory_store_write_performed",
            "memory_store_mutated",
            "wal_write_performed",
            "receipt_persisted",
            "rollback_executed",
            "live_kg_write_performed",
            "provider_invoked",
            "model_invoked",
            "credential_read",
            "secret_file_read",
            "telegram_send_performed",
            "channel_send_performed",
            "external_send_performed",
            "install_executed",
            "launchd_mutated",
            "service_restarted",
            "service_restart_performed",
            "active_binary_mutated",
        ] {
            fixture.insert(key.to_string(), serde_json::json!(false));
        }
        if let Some(extra) = extra.as_object() {
            for (key, value) in extra {
                fixture.insert(key.clone(), value.clone());
            }
        }
        serde_json::Value::Object(fixture)
    }

    let route_matrix = control_ui_route_parity_report();
    let source = std::thread::Builder::new()
        .name("hepta-scoped-production-memory-release-publication-source-report".to_string())
        .stack_size(8 * 1024 * 1024)
        .spawn(hepta_memory_live_mutation_operator_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_boundary_report)
        .ok()
        .and_then(|handle| handle.join().ok())
        .unwrap_or_else(|| {
            serde_json::json!({
                "status": "blocked",
                "memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_boundary_ready": false,
                "source_release_artifact_publication_denial_report_thread_failed": true
            })
        });

    let source_bool = |key: &str| {
        source
            .get(key)
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
    };
    let source_u64 = |key: &str| {
        source
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let source_str = |key: &str| {
        source
            .get(key)
            .and_then(serde_json::Value::as_str)
            .unwrap_or("")
            .to_string()
    };

    let route_count_source_command_accepted = route_matrix.ready
        && route_matrix.route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let source_next_action_result_receipt = source
        .get("allowed_next_actions")
        .and_then(serde_json::Value::as_array)
        .and_then(|items| items.get(1))
        .map(|item| {
            item.get("action").and_then(serde_json::Value::as_str)
                == Some(
                    "prepare_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_result_receipt_no_persistence_boundary",
                )
                && item
                    .get("persists_publication_result_receipt")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && item
                    .get("publishes_release_artifact")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
        })
        .unwrap_or(false);
    let source_ready = source_str("status") == "ready"
        && source_bool(
            "memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_boundary_ready",
        )
        && source_bool(
            "scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_accepted",
        )
        && source_u64(
            "accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_fixture_count",
        ) == 1
        && source_u64(
            "blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_fixture_count",
        ) == 9
        && source_u64(
            "denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_boundary_count",
        ) >= 100
        && !source_bool("dry_run_execution_result_receipt_release_artifact_publication_recorded")
        && !source_bool("dry_run_execution_result_receipt_release_artifact_publication_accepted")
        && !source_bool("dry_run_execution_result_receipt_release_artifact_written")
        && !source_bool("dry_run_execution_result_receipt_public_artifact_written")
        && !source_bool("dry_run_execution_result_receipt_publication_queue_enqueued")
        && !source_bool("dry_run_execution_result_receipt_publication_manifest_written")
        && !source_bool("dry_run_execution_result_receipt_public_distribution_performed")
        && !source_bool("dry_run_execution_result_receipt_public_release_published")
        && !source_bool(
            "dry_run_execution_result_receipt_authority_promoted_from_release_artifact_publication",
        )
        && !source_bool("activation_performed")
        && !source_bool("dry_run_execution_executed")
        && !source_bool("production_durable_memory_store_write_performed")
        && !source_bool("memory_store_write_performed")
        && !source_bool("wal_write_performed")
        && !source_bool("receipt_persisted")
        && !source_bool("live_kg_write_performed")
        && !source_bool("provider_invoked")
        && !source_bool("model_invoked")
        && !source_bool("credential_read")
        && !source_bool("channel_send_performed")
        && !source_bool("external_send_performed")
        && !source_bool("install_executed")
        && !source_bool("service_restarted")
        && !source_bool("active_binary_mutated")
        && source_next_action_result_receipt;

    let fixtures = serde_json::Value::Array(vec![
        release_artifact_publication_result_receipt_fixture(
            "scoped-production-durable-memory-write-dry-run-result-receipt-release-artifact-publication-result-receipt-report-only-binding",
            "accepted_report_only",
            true,
            "publication_result_receipt_no_persistence_matrix_bound_without_receipt_recording_or_publication",
            serde_json::json!({}),
        ),
        release_artifact_publication_result_receipt_fixture(
            "scoped-production-durable-memory-write-dry-run-result-receipt-release-artifact-publication-result-receipt-missing-source",
            "blocked_noop",
            false,
            "source_release_artifact_publication_denial_report_required",
            serde_json::json!({
                "source_release_artifact_publication_denial_present": false,
                "source_release_artifact_publication_denial_ready": false,
                "publication_result_receipt_requested": true
            }),
        ),
        release_artifact_publication_result_receipt_fixture(
            "scoped-production-durable-memory-write-dry-run-result-receipt-release-artifact-publication-result-receipt-recording-request",
            "blocked_receipt_noop",
            false,
            "publication_result_receipt_recording_denied",
            serde_json::json!({
                "publication_result_receipt_requested": true,
                "publication_result_receipt_recording_requested": true
            }),
        ),
        release_artifact_publication_result_receipt_fixture(
            "scoped-production-durable-memory-write-dry-run-result-receipt-release-artifact-publication-result-receipt-persistence-request",
            "blocked_receipt_noop",
            false,
            "publication_result_receipt_persistence_materialization_filesystem_denied",
            serde_json::json!({
                "publication_result_receipt_requested": true,
                "publication_result_receipt_persistence_requested": true,
                "publication_result_receipt_materialization_requested": true,
                "publication_result_receipt_filesystem_write_requested": true
            }),
        ),
        release_artifact_publication_result_receipt_fixture(
            "scoped-production-durable-memory-write-dry-run-result-receipt-release-artifact-publication-result-receipt-ledger-index-request",
            "blocked_receipt_noop",
            false,
            "publication_result_receipt_ledger_index_denied",
            serde_json::json!({
                "publication_result_receipt_requested": true,
                "publication_result_receipt_ledger_write_requested": true,
                "publication_result_receipt_index_requested": true
            }),
        ),
        release_artifact_publication_result_receipt_fixture(
            "scoped-production-durable-memory-write-dry-run-result-receipt-release-artifact-publication-result-receipt-queue-delivery-request",
            "blocked_delivery_noop",
            false,
            "publication_result_receipt_queue_delivery_denied",
            serde_json::json!({
                "publication_result_receipt_requested": true,
                "publication_result_receipt_queue_requested": true,
                "publication_result_receipt_delivery_requested": true,
                "channel_delivery_publication_result_receipt_requested": true
            }),
        ),
        release_artifact_publication_result_receipt_fixture(
            "scoped-production-durable-memory-write-dry-run-result-receipt-release-artifact-publication-result-receipt-export-query-observability-request",
            "blocked_reporting_noop",
            false,
            "publication_result_receipt_export_query_observability_denied",
            serde_json::json!({
                "publication_result_receipt_requested": true,
                "publication_result_receipt_export_requested": true,
                "publication_result_receipt_query_requested": true,
                "publication_result_receipt_observability_requested": true
            }),
        ),
        release_artifact_publication_result_receipt_fixture(
            "scoped-production-durable-memory-write-dry-run-result-receipt-release-artifact-publication-result-receipt-signature-status-completion-ack-request",
            "blocked_ack_noop",
            false,
            "publication_result_receipt_signature_timestamp_status_and_completion_ack_denied",
            serde_json::json!({
                "publication_result_receipt_requested": true,
                "publication_result_receipt_signature_requested": true,
                "publication_result_receipt_timestamp_requested": true,
                "publication_result_receipt_status_requested": true,
                "publication_completion_ack_requested": true
            }),
        ),
        release_artifact_publication_result_receipt_fixture(
            "scoped-production-durable-memory-write-dry-run-result-receipt-release-artifact-publication-result-receipt-authority-publication-retry-request",
            "blocked_authority_noop",
            false,
            "publication_result_receipt_authority_and_publication_retry_denied",
            serde_json::json!({
                "publication_result_receipt_requested": true,
                "publication_result_receipt_authority_promotion_requested": true,
                "release_artifact_publication_retry_requested": true,
                "release_artifact_write_requested": true,
                "public_artifact_write_requested": true,
                "public_release_publish_requested": true,
                "public_ga_claim_requested": true
            }),
        ),
        release_artifact_publication_result_receipt_fixture(
            "scoped-production-durable-memory-write-dry-run-result-receipt-release-artifact-publication-result-receipt-execution-memory-provider-install-request",
            "blocked_execution_noop",
            false,
            "execution_memory_provider_channel_install_active_binary_result_receipt_denied",
            serde_json::json!({
                "publication_result_receipt_requested": true,
                "dry_run_execution_publication_result_receipt_requested": true,
                "production_memory_write_publication_result_receipt_requested": true,
                "memory_write_publication_result_receipt_requested": true,
                "kg_write_publication_result_receipt_requested": true,
                "provider_prompt_publication_result_receipt_requested": true,
                "credential_publication_result_receipt_requested": true,
                "channel_delivery_publication_result_receipt_requested": true,
                "install_publication_result_receipt_requested": true,
                "service_restart_publication_result_receipt_requested": true,
                "active_binary_publication_result_receipt_requested": true
            }),
        ),
    ]);

    let source_report_sha256 = sha256_json_value(&source);
    let fixtures_hash_sha256 = sha256_json_value(&fixtures);
    let publication_result_receipt_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-result-receipt-release-artifact-publication-result-receipt-no-persistence:v1:source={}:record=false:persist=false:materialize=false:deliver=false",
        source_str(
            "dry_run_execution_result_receipt_release_artifact_publication_result_hash_sha256"
        )
    ));
    let publication_result_receipt_matrix_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-result-receipt-release-artifact-publication-result-receipt-no-persistence-matrix:v1:receipt={publication_result_receipt_hash_sha256}:fixtures={fixtures_hash_sha256}"
    ));
    let publication_result_receipt_handoff_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-result-receipt-release-artifact-publication-result-receipt-no-persistence-handoff:v1:matrix={publication_result_receipt_matrix_hash_sha256}:next=release-artifact-publication-result-receipt-replay-idempotency-denial-boundary"
    ));
    let publication_result_receipt_result_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-result-receipt-release-artifact-publication-result-receipt-no-persistence-result:v1:receipt={publication_result_receipt_hash_sha256}:handoff={publication_result_receipt_handoff_hash_sha256}:accepted=true:record=false:persist=false:publication=false:authority=false:execution=false:production-write=false"
    ));
    let publication_result_receipt_boundary_hash_sha256 = sha256_text_value(&format!(
        "scoped-production-durable-memory-write-dry-run-result-receipt-release-artifact-publication-result-receipt-no-persistence-boundary:v1:source={source_report_sha256}:result={publication_result_receipt_result_hash_sha256}:accepted=1:blocked=9:receipt-persist=false:publication=false:authority=false:execution=false:production-write=false"
    ));
    let publication_result_receipt_policy_hash_sha256 = sha256_text_value(
        "scoped-production-durable-memory-write-dry-run-result-receipt-release-artifact-publication-result-receipt-no-persistence-policy:v1:no-receipt-recording-no-receipt-persistence-no-ledger-no-index-no-queue-no-delivery-no-export-no-query-no-observability-no-completion-ack-no-release-artifact-no-publication-no-authority-no-execution-no-production-write",
    );

    let mut denials = source
        .get("denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_boundary")
        .and_then(serde_json::Value::as_array)
        .cloned()
        .unwrap_or_default();
    for &denial in RELEASE_ARTIFACT_PUBLICATION_RESULT_RECEIPT_DENIALS {
        denials.push(serde_json::json!(denial));
    }
    let denial_count = denials.len();

    let report_ready = source_ready
        && route_count_source_command_accepted
        && fixtures.as_array().map(Vec::len) == Some(10)
        && RELEASE_ARTIFACT_PUBLICATION_RESULT_RECEIPT_SURFACES.len() == 14
        && RELEASE_ARTIFACT_PUBLICATION_RESULT_RECEIPT_DENIALS.len() == 20
        && denial_count >= 115;

    let mut side_effects = serde_json::Map::new();
    side_effects.insert(
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_result_receipt_no_persistence_boundary_performed".to_string(),
        serde_json::json!(true),
    );
    side_effects.insert(
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_result_receipt_no_persistence_result_accepted".to_string(),
        serde_json::json!(true),
    );
    for key in [
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_requested",
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_recorded",
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_accepted",
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_persisted",
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_materialized",
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_filesystem_written",
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_ledger_written",
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_indexed",
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_queued",
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_delivered",
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_exported",
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_query_registered",
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_observability_recorded",
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_signature_accepted",
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_timestamp_accepted",
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_status_accepted",
        "dry_run_execution_result_receipt_release_artifact_publication_completion_ack_recorded",
        "dry_run_execution_result_receipt_release_artifact_publication_completion_ack_persisted",
        "dry_run_execution_result_receipt_release_artifact_publication_completion_ack_accepted",
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_authority_promoted",
        "dry_run_execution_result_receipt_release_artifact_publication_recorded",
        "dry_run_execution_result_receipt_release_artifact_publication_persisted",
        "dry_run_execution_result_receipt_release_artifact_publication_materialized",
        "dry_run_execution_result_receipt_release_artifact_written",
        "dry_run_execution_result_receipt_public_artifact_written",
        "dry_run_execution_result_receipt_publication_queue_enqueued",
        "dry_run_execution_result_receipt_publication_manifest_written",
        "dry_run_execution_result_receipt_public_distribution_performed",
        "dry_run_execution_result_receipt_public_release_published",
        "dry_run_execution_result_receipt_public_ga_claimed",
        "dry_run_execution_result_receipt_authority_promoted_from_release_artifact_publication",
        "activation_performed",
        "dry_run_execution_executed",
        "production_durable_memory_write_executed",
        "production_durable_memory_store_write_performed",
        "actual_production_durable_memory_write_performed",
        "durable_memory_store_write_performed",
        "memory_write_execution_performed",
        "memory_store_write_performed",
        "memory_store_mutated",
        "wal_write_performed",
        "receipt_persisted",
        "rollback_executed",
        "tombstone_cleanup_executed",
        "live_kg_write_performed",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "telegram_send_performed",
        "channel_send_performed",
        "external_send_performed",
        "release_artifact_written",
        "public_artifact_written",
        "public_release_published",
        "public_ga_claimed",
        "install_executed",
        "launchd_mutated",
        "service_restart_performed",
        "service_restarted",
        "active_binary_mutated",
        "filesystem_written",
    ] {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "endpoint": HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_SCOPED_PRODUCTION_DURABLE_MEMORY_WRITE_DRY_RUN_EXECUTION_RESULT_RECEIPT_RELEASE_ARTIFACT_PUBLICATION_RESULT_RECEIPT_NO_PERSISTENCE_BOUNDARY_ENDPOINT,
        "source_command": "/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-release-artifact-publication-result-receipt-no-persistence-boundary --json",
        "native_route": true,
        "side_effect_free": true,
        "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        "route_count": route_matrix.route_count,
        "implemented_route_count": route_matrix.implemented_route_count,
        "missing_route_count": route_matrix.missing_route_count,
        "route_count_source_command_accepted": route_count_source_command_accepted,
        "memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_result_receipt_no_persistence_boundary_ready": report_ready,
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_result_receipt_no_persistence_ready": report_ready,
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_result_receipt_no_persistence_accepted": report_ready,
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_result_receipt_no_persistence_mode": "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_no_persistence_boundary_no_receipt_persistence_no_publication_no_authority_no_execution_no_production_durable_memory_mutation",
        "source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_boundary_ready": source_bool("memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_boundary_ready"),
        "source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_boundary_report_sha256": source_report_sha256,
        "source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_boundary_hash_sha256": source_str("scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_boundary_hash_sha256"),
        "source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_policy_hash_sha256": source_str("scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_policy_hash_sha256"),
        "source_dry_run_execution_result_receipt_release_artifact_publication_result_hash_sha256": source_str("dry_run_execution_result_receipt_release_artifact_publication_result_hash_sha256"),
        "source_dry_run_execution_result_receipt_release_artifact_publication_handoff_hash_sha256": source_str("dry_run_execution_result_receipt_release_artifact_publication_handoff_hash_sha256"),
        "source_accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_fixture_count": source_u64("accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_fixture_count"),
        "source_blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_fixture_count": source_u64("blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_fixture_count"),
        "source_denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_boundary_count": source_u64("denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_boundary_count"),
    });
    extend_json_object(
        &mut report,
        serde_json::json!({
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_hash_sha256": publication_result_receipt_hash_sha256,
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_matrix_hash_sha256": publication_result_receipt_matrix_hash_sha256,
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_handoff_hash_sha256": publication_result_receipt_handoff_hash_sha256,
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_result_hash_sha256": publication_result_receipt_result_hash_sha256,
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_result_receipt_no_persistence_boundary_hash_sha256": publication_result_receipt_boundary_hash_sha256,
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_result_receipt_no_persistence_policy_hash_sha256": publication_result_receipt_policy_hash_sha256,
        "required_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_result_receipt_no_persistence_surface_count": 14,
        "ready_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_result_receipt_no_persistence_surface_count": 14,
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_result_receipt_no_persistence_fixture_count": 10,
        "accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_result_receipt_no_persistence_fixture_count": 1,
        "blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_result_receipt_no_persistence_fixture_count": 9,
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_result_receipt_no_persistence_fixtures": fixtures,
        "denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_result_receipt_no_persistence_boundary": denials,
        "denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_result_receipt_no_persistence_boundary_count": denial_count,
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_result_receipt_no_persistence_boundary_performed_count": 1,
        "scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_result_receipt_no_persistence_result_accepted_count": 1,
        "source_dry_run_execution_result_receipt_release_artifact_publication_denial_boundary_bound_count": 1,
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_rendered_count": 1,
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_recording_denied_count": 1,
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_persistence_denied_count": 1,
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_ledger_index_denied_count": 1,
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_queue_delivery_denied_count": 1,
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_export_query_observability_denied_count": 1,
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_signature_timestamp_status_denied_count": 1,
        "dry_run_execution_result_receipt_release_artifact_publication_completion_ack_denied_count": 1,
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_authority_denied_count": 1,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_recorded_count": 0,
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_accepted_count": 0,
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_persisted_count": 0,
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_materialized_count": 0,
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_filesystem_written_count": 0,
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_ledger_written_count": 0,
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_indexed_count": 0,
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_queued_count": 0,
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_delivered_count": 0,
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_exported_count": 0,
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_query_registered_count": 0,
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_observability_recorded_count": 0,
        "dry_run_execution_result_receipt_release_artifact_publication_completion_ack_recorded_count": 0,
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_authority_promoted_count": 0,
        "dry_run_execution_result_receipt_release_artifact_publication_recorded_count": 0,
        "dry_run_execution_result_receipt_release_artifact_written_count": 0,
        "dry_run_execution_result_receipt_public_artifact_written_count": 0,
        "dry_run_execution_result_receipt_public_distribution_performed_count": 0,
        "dry_run_execution_result_receipt_public_release_published_count": 0,
        "dry_run_execution_result_receipt_public_ga_claimed_count": 0,
        "activation_performed_count": 0,
        "dry_run_execution_executed_count": 0,
        "production_durable_memory_write_executed_count": 0,
        "production_durable_memory_store_write_performed_count": 0,
        "memory_store_write_performed_count": 0,
        "wal_write_performed_count": 0,
        "receipt_persisted_count": 0,
        "live_kg_write_performed_count": 0,
        "provider_invoked_count": 0,
        "model_invoked_count": 0,
        "credential_read_count": 0,
        "channel_send_performed_count": 0,
        "external_send_performed_count": 0,
        "release_artifact_written_count": 0,
        "public_artifact_written_count": 0,
        "install_executed_count": 0,
        "service_restarted_count": 0,
        "active_binary_mutated_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_requested": false,
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_recorded": false,
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_accepted": false,
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_persisted": false,
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_materialized": false,
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_filesystem_written": false,
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_ledger_written": false,
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_indexed": false,
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_queued": false,
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_delivered": false,
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_exported": false,
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_query_registered": false,
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_observability_recorded": false,
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_signature_accepted": false,
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_timestamp_accepted": false,
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_status_accepted": false,
        "dry_run_execution_result_receipt_release_artifact_publication_completion_ack_recorded": false,
        "dry_run_execution_result_receipt_release_artifact_publication_completion_ack_persisted": false,
        "dry_run_execution_result_receipt_release_artifact_publication_completion_ack_accepted": false,
        "dry_run_execution_result_receipt_release_artifact_publication_result_receipt_authority_promoted": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
        "dry_run_execution_result_receipt_release_artifact_publication_recorded": false,
        "dry_run_execution_result_receipt_release_artifact_publication_persisted": false,
        "dry_run_execution_result_receipt_release_artifact_publication_materialized": false,
        "dry_run_execution_result_receipt_release_artifact_written": false,
        "dry_run_execution_result_receipt_public_artifact_written": false,
        "dry_run_execution_result_receipt_publication_queue_enqueued": false,
        "dry_run_execution_result_receipt_publication_manifest_written": false,
        "dry_run_execution_result_receipt_public_distribution_performed": false,
        "dry_run_execution_result_receipt_public_release_published": false,
        "dry_run_execution_result_receipt_public_ga_claimed": false,
        "dry_run_execution_result_receipt_authority_promoted_from_release_artifact_publication": false,
        "activation_allowed": false,
        "activation_performed": false,
        "dry_run_execution_executed": false,
        "production_durable_memory_write_executed": false,
        "production_durable_memory_store_write_performed": false,
        "actual_production_durable_memory_write_performed": false,
        "durable_memory_store_write_performed": false,
        "memory_write_execution_performed": false,
        "memory_store_write_performed": false,
        "memory_store_mutated": false,
        "wal_write_performed": false,
        "receipt_persisted": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
        "rollback_executed": false,
        "tombstone_cleanup_executed": false,
        "live_kg_write_performed": false,
        "provider_invoked": false,
        "model_invoked": false,
        "credential_read": false,
        "secret_file_read": false,
        "telegram_send_performed": false,
        "channel_send_performed": false,
        "external_send_performed": false,
        "release_artifact_publication_allowed": false,
        "release_artifact_publication_requested": false,
        "release_artifact_publication_accepted": false,
        "release_artifact_publication_recorded": false,
        "release_artifact_publication_persisted": false,
        "release_artifact_publication_materialized": false,
        "release_artifact_filesystem_written": false,
        "release_artifact_written": false,
        "public_artifact_written": false,
        "publication_queue_enqueued": false,
        "publication_manifest_written": false,
        "public_distribution_performed": false,
        "public_release_published": false,
        "public_ga_claimed": false,
        "public_claim_promoted": false,
        "install_executed": false,
        "launchd_mutated": false,
        "service_restart_performed": false,
        "service_restarted": false,
        "active_binary_mutated": false,
        "filesystem_written": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
        "allowed_next_actions": [
            {
                "action": "run_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_result_receipt_no_persistence_boundary_require_live_gate",
                "status": "allowed_verification_only",
                "records_publication_result_receipt": false,
                "persists_publication_result_receipt": false,
                "writes_receipt_ledger": false,
                "indexes_receipt": false,
                "queues_or_delivers_receipt": false,
                "exports_or_queries_receipt": false,
                "records_observability": false,
                "records_completion_ack": false,
                "publishes_release_artifact": false,
                "claims_public_release": false,
                "writes_release_artifact": false,
                "promotes_activation_authority": false,
                "executes_dry_run": false,
                "writes_production_durable_memory": false,
                "writes_memory_or_kg": false,
                "invokes_provider": false,
                "sends_externally": false,
                "installs_or_restarts": false,
                "mutates_active_binary": false
            },
            {
                "action": "prepare_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_result_receipt_replay_idempotency_denial_boundary",
                "status": "allowed_report_only_next_slice",
                "accepts_replay": false,
                "records_publication_result_receipt": false,
                "persists_publication_result_receipt": false,
                "publishes_release_artifact": false,
                "claims_public_release": false,
                "promotes_activation_authority": false,
                "executes_dry_run": false,
                "writes_production_durable_memory": false,
                "writes_memory_or_kg": false,
                "invokes_model": false,
                "sends_externally": false,
                "installs_or_restarts": false,
                "mutates_active_binary": false
            }
        ],
        "side_effects": side_effects
        }),
    );
    report
}

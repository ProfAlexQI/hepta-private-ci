fn hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source_final_ack =
        hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_report();
    let source_bool = |key: &str| {
        source_final_ack
            .get(key)
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
    };
    let source_u64 = |key: &str| {
        source_final_ack
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let source_status = source_final_ack
        .get("operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_status")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("unknown")
        .to_string();
    let source_ready = source_status == "blocked"
        && source_bool(
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_ready",
        );
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT;
    let report_ready = source_ready
        && route_count_source_command_accepted
        && route_matrix.missing_route_count == 0;
    let source_report_sha256 = sha256_json_value(&source_final_ack);

    let terminal_decision_fixture =
        |id: &str, status: &str, reason: &str, extra: serde_json::Value| -> serde_json::Value {
            let mut fixture = serde_json::Map::new();
            fixture.insert("id".to_string(), serde_json::json!(id));
            fixture.insert("fixture_id".to_string(), serde_json::json!(id));
            fixture.insert(
                "terminal_operator_decision_requested".to_string(),
                serde_json::json!(false),
            );
            fixture.insert(
                "terminal_operator_decision_status".to_string(),
                serde_json::json!(status),
            );
            fixture.insert(
                "source_final_acknowledgement_present".to_string(),
                serde_json::json!(true),
            );
            fixture.insert(
                "source_final_acknowledgement_ready".to_string(),
                serde_json::json!(true),
            );
            for key in [
                "terminal_decision_allowed",
                "terminal_decision_request_accepted",
                "terminal_decision_accepted",
                "terminal_decision_recorded",
                "terminal_decision_persisted",
                "terminal_decision_materialized",
                "terminal_decision_filesystem_written",
                "terminal_decision_delivered",
                "terminal_decision_channel_delivery_performed",
                "terminal_decision_identity_accepted",
                "terminal_decision_signature_accepted",
                "terminal_decision_timestamp_accepted",
                "terminal_decision_final_state_promoted",
                "terminal_decision_completion_promoted",
                "public_claim_requested",
                "public_claim_accepted",
                "public_claim_recorded",
                "public_claim_persisted",
                "public_claim_materialized",
                "public_claim_promoted",
                "public_ga_claimed",
                "public_release_published",
                "public_distribution_performed",
                "public_artifact_written",
                "release_artifact_written",
                "telegram_send_performed",
                "channel_send_performed",
                "external_send_performed",
                "receipt_recorded",
                "receipt_persisted",
                "receipt_accepted",
                "receipt_materialized",
                "receipt_filesystem_written",
                "completion_ack_recorded",
                "completion_ack_persisted",
                "completion_ack_accepted",
                "completion_ack_delivered",
                "activation_allowed",
                "activation_performed",
                "live_mutation_execution_performed",
                "memory_write_execution_performed",
                "memory_store_write_performed",
                "memory_store_mutated",
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
                fixture.insert(key.to_string(), serde_json::json!(false));
            }
            fixture.insert(
                "terminal_operator_decision_noop_confirmed".to_string(),
                serde_json::json!(true),
            );
            fixture.insert("reason".to_string(), serde_json::json!(reason));
            if let Some(extra_object) = extra.as_object() {
                fixture.extend(extra_object.clone());
            }
            serde_json::Value::Object(fixture)
        };

    let terminal_decision_public_claim_fixtures = serde_json::json!([
        terminal_decision_fixture(
            "operator-canary-controlled-request-harness-activation-result-receipt-terminal-decision-missing-final-ack",
            "blocked_noop",
            "source_final_operator_acknowledgement_report_required",
            serde_json::json!({
                "source_final_acknowledgement_present": false,
                "source_final_acknowledgement_ready": false,
                "terminal_operator_decision_requested": true,
            }),
        ),
        terminal_decision_fixture(
            "operator-canary-controlled-request-harness-activation-result-receipt-terminal-decision-request",
            "blocked_decision_noop",
            "terminal_operator_decision_request_shape_denied",
            serde_json::json!({"terminal_operator_decision_requested": true}),
        ),
        terminal_decision_fixture(
            "operator-canary-controlled-request-harness-activation-result-receipt-terminal-decision-acceptance-request",
            "blocked_acceptance_noop",
            "terminal_operator_decision_acceptance_denied",
            serde_json::json!({
                "terminal_operator_decision_requested": true,
                "terminal_decision_acceptance_requested": true,
            }),
        ),
        terminal_decision_fixture(
            "operator-canary-controlled-request-harness-activation-result-receipt-terminal-decision-recording-request",
            "blocked_decision_noop",
            "terminal_operator_decision_recording_denied",
            serde_json::json!({
                "terminal_operator_decision_requested": true,
                "terminal_decision_recording_requested": true,
            }),
        ),
        terminal_decision_fixture(
            "operator-canary-controlled-request-harness-activation-result-receipt-terminal-decision-persistence-filesystem-write-request",
            "blocked_decision_noop",
            "terminal_operator_decision_persistence_filesystem_write_denied",
            serde_json::json!({
                "terminal_operator_decision_requested": true,
                "terminal_decision_persistence_requested": true,
                "terminal_decision_filesystem_write_requested": true,
            }),
        ),
        terminal_decision_fixture(
            "operator-canary-controlled-request-harness-activation-result-receipt-terminal-decision-identity-signature-request",
            "blocked_acceptance_noop",
            "operator_identity_signature_terminal_decision_acceptance_denied",
            serde_json::json!({
                "terminal_operator_decision_requested": true,
                "operator_identity_acceptance_requested": true,
                "operator_signature_acceptance_requested": true,
                "operator_timestamp_acceptance_requested": true,
            }),
        ),
        terminal_decision_fixture(
            "operator-canary-controlled-request-harness-activation-result-receipt-terminal-decision-public-claim-request",
            "blocked_public_claim_noop",
            "public_claim_request_non_promotion_denied",
            serde_json::json!({
                "terminal_operator_decision_requested": true,
                "public_claim_requested": true,
                "public_claim_promotion_requested": true,
            }),
        ),
        terminal_decision_fixture(
            "operator-canary-controlled-request-harness-activation-result-receipt-terminal-decision-public-ga-release-request",
            "blocked_promotion_noop",
            "public_ga_release_publication_promotion_denied",
            serde_json::json!({
                "terminal_operator_decision_requested": true,
                "public_ga_claim_requested": true,
                "public_release_publish_requested": true,
                "public_distribution_requested": true,
                "release_artifact_write_requested": true,
            }),
        ),
        terminal_decision_fixture(
            "operator-canary-controlled-request-harness-activation-result-receipt-terminal-decision-activation-memory-provider-request",
            "blocked_decision_noop",
            "activation_memory_rollback_secret_provider_terminal_decision_denied",
            serde_json::json!({
                "terminal_operator_decision_requested": true,
                "activation_from_terminal_decision_requested": true,
                "memory_write_terminal_decision_requested": true,
                "rollback_terminal_decision_requested": true,
                "secret_material_terminal_decision_requested": true,
                "provider_prompt_terminal_decision_requested": true,
            }),
        ),
        terminal_decision_fixture(
            "operator-canary-controlled-request-harness-activation-result-receipt-terminal-decision-external-public-install-request",
            "blocked_promotion_noop",
            "external_public_install_restart_active_binary_terminal_decision_denied",
            serde_json::json!({
                "terminal_operator_decision_requested": true,
                "external_send_decision_requested": true,
                "public_claim_decision_requested": true,
                "release_artifact_decision_requested": true,
                "install_decision_requested": true,
                "service_restart_decision_requested": true,
                "active_binary_decision_requested": true,
            }),
        ),
    ]);
    let terminal_decision_public_claim_fixture_count = terminal_decision_public_claim_fixtures
        .as_array()
        .map(std::vec::Vec::len)
        .unwrap_or(0);
    let fixtures_sha256 = sha256_json_value(&terminal_decision_public_claim_fixtures);
    let contract_hash_sha256 = sha256_text_value(&format!(
        "hepta-canary-operator-review-acknowledgement-activation-command-result-receipt-terminal-operator-decision-public-claim-non-promotion-denial:v1:source={source_report_sha256}:fixtures={fixtures_sha256}:decision=0:public_claim=0:publish=0:artifact=0:live=0"
    ));
    let policy_hash_sha256 = sha256_text_value(
        "memory-intelligence-kg-operator-canary-harness-operator-review-acknowledgement-activation-command-result-receipt-terminal-operator-decision-public-claim-non-promotion-denial:v1:no-terminal-decision-accept:no-public-claim:no-ga-release:no-artifact:no-live",
    );
    let side_effect_hash_sha256 = sha256_text_value(
        "terminal_decision=false;public_claim=false;public_release=false;artifact=false;activation=false;provider=false;model=false;memory=false;kg=false;secret=false;install=false;restart=false;active_binary=false",
    );
    let mut denials = source_final_ack
        .get("denied_by_activation_command_result_receipt_final_operator_acknowledgement")
        .and_then(serde_json::Value::as_array)
        .cloned()
        .unwrap_or_default();
    for denial in [
        "source_final_operator_acknowledgement_report_required",
        "terminal_operator_decision_request_acceptance_denied",
        "terminal_operator_decision_acceptance_denied",
        "terminal_operator_decision_recording_denied",
        "terminal_operator_decision_persistence_denied",
        "terminal_operator_decision_materialization_denied",
        "terminal_operator_decision_filesystem_write_denied",
        "operator_identity_signature_terminal_decision_acceptance_denied",
        "terminal_operator_decision_delivery_denied",
        "telegram_send_denied",
        "public_claim_non_promotion_denied",
        "public_ga_release_publication_promotion_denied",
        "activation_from_terminal_operator_decision_denied",
        "memory_write_terminal_decision_denied",
        "rollback_terminal_decision_denied",
        "secret_material_terminal_decision_denied",
        "provider_prompt_terminal_decision_denied",
        "external_public_install_restart_active_binary_terminal_decision_denied",
    ] {
        denials.push(serde_json::Value::String(denial.to_string()));
    }
    let denied_count = denials.len();

    let mut report = source_final_ack.clone();
    extend_json_object(
        &mut report,
        serde_json::json!({
            "gate": "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_route",
            "status": if report_ready { "ready" } else { "blocked" },
            "source_command": "/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-terminal-operator-decision-public-claim-non-promotion-denial --json",
            "native_route": true,
            "compatibility_mode": "native_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_status",
            "side_effect_free": true,
            "audit_date": "2026-06-14",
            "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_TERMINAL_OPERATOR_DECISION_PUBLIC_CLAIM_NON_PROMOTION_DENIAL_ENDPOINT,
            "source_operator_review_acknowledgement_activation_command_result_receipt_final_operator_acknowledgement_route_endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_FINAL_OPERATOR_ACKNOWLEDGEMENT_NON_ACCEPTANCE_DENIAL_ENDPOINT,
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_terminal_operator_decision_public_claim_route_doc": "docs/architecture/i3-6301d9d7cf662d71e7d154f6.md",
            "source_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_final_operator_acknowledgement_route_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-final-operator-acknowledgement-non-acceptance-denial-route-gate.sh",
            "source_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_terminal_operator_decision_public_claim_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-terminal-operator-decision-public-claim-non-promotion-denial-gate.sh",
            "source_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_terminal_operator_decision_public_claim_route_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-terminal-operator-decision-public-claim-non-promotion-denial-route-gate.sh",
            "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
            "route_count": route_matrix.route_count,
            "implemented_route_count": route_matrix.implemented_route_count,
            "missing_route_count": route_matrix.missing_route_count,
            "route_count_source_command_accepted": route_count_source_command_accepted,
            "source_route_wired": true,
            "source_operator_review_acknowledgement_activation_command_result_receipt_final_operator_acknowledgement_route_ready": source_ready,
            "source_operator_review_acknowledgement_activation_command_result_receipt_final_operator_acknowledgement_route_status": source_status,
            "source_operator_review_acknowledgement_activation_command_result_receipt_final_operator_acknowledgement_route_report_sha256": source_report_sha256,
            "operator_authorization_scope": "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_terminal_operator_decision_public_claim_no_decision_accept_no_record_no_persist_no_delivery_no_public_claim_no_ga_release_no_artifact_no_context_memory_kg_provider_model_credential_channel_install_restart_binary",
            "operator_authorization_received": true,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_route_enabled": true,
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_ready": true,
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_status": "blocked",
            "activation_command_result_receipt_terminal_operator_decision_public_claim_schema_version": "memory_intelligence_kg_operator_canary_harness_operator_review_acknowledgement_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_v1",
            "activation_command_result_receipt_terminal_operator_decision_public_claim_mode": "native_route_stdout_only_terminal_operator_decision_public_claim_non_promotion_denial_no_decision_accept_no_public_claim_no_release_no_artifact_no_authority_no_live",
            "activation_command_result_receipt_terminal_operator_decision_public_claim_decision": "blocked_noop_activation_command_result_receipt_cannot_be_promoted_into_terminal_operator_decision_or_public_claim_authority",
            "source_final_operator_acknowledgement_fixture_count": source_u64("activation_command_result_receipt_final_operator_acknowledgement_fixture_count"),
            "source_blocked_final_operator_acknowledgement_fixture_count": source_u64("blocked_activation_command_result_receipt_final_operator_acknowledgement_fixture_count"),
            "source_accepted_final_operator_acknowledgement_fixture_count": source_u64("accepted_activation_command_result_receipt_final_operator_acknowledgement_fixture_count"),
            "source_final_operator_acknowledgement_performed_count": source_u64("activation_command_result_receipt_final_operator_acknowledgement_performed_count"),
            "terminal_operator_decision_public_claim_fixtures_sha256": fixtures_sha256,
            "terminal_operator_decision_public_claim_contract_hash_sha256": contract_hash_sha256,
            "terminal_operator_decision_public_claim_policy_hash_sha256": policy_hash_sha256,
            "side_effect_hash_sha256": side_effect_hash_sha256,
            "required_activation_command_result_receipt_terminal_operator_decision_public_claim_surface_count": 12,
            "ready_activation_command_result_receipt_terminal_operator_decision_public_claim_surface_count": 12,
            "side_effect_free_activation_command_result_receipt_terminal_operator_decision_public_claim_surface_count": 12,
            "required_activation_command_result_receipt_terminal_operator_decision_public_claim_fixture_count": 10,
            "activation_command_result_receipt_terminal_operator_decision_public_claim_fixtures": terminal_decision_public_claim_fixtures,
            "activation_command_result_receipt_terminal_operator_decision_public_claim_fixture_count": terminal_decision_public_claim_fixture_count,
            "blocked_activation_command_result_receipt_terminal_operator_decision_public_claim_fixture_count": terminal_decision_public_claim_fixture_count,
            "noop_activation_command_result_receipt_terminal_operator_decision_public_claim_fixture_count": terminal_decision_public_claim_fixture_count,
            "allowed_activation_command_result_receipt_terminal_operator_decision_public_claim_fixture_count": 0,
            "accepted_activation_command_result_receipt_terminal_operator_decision_public_claim_fixture_count": 0,
            "activation_command_result_receipt_terminal_operator_decision_performed_count": 0,
            "activation_command_result_receipt_public_claim_promotion_performed_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "activation_command_result_receipt_terminal_operator_decision_allowed": false,
            "activation_command_result_receipt_terminal_operator_decision_request_accepted": false,
            "activation_command_result_receipt_terminal_operator_decision_accepted": false,
            "activation_command_result_receipt_terminal_operator_decision_recorded": false,
            "activation_command_result_receipt_terminal_operator_decision_persisted": false,
            "activation_command_result_receipt_terminal_operator_decision_materialized": false,
            "activation_command_result_receipt_terminal_operator_decision_filesystem_written": false,
            "activation_command_result_receipt_terminal_operator_decision_delivered": false,
            "activation_command_result_receipt_terminal_operator_decision_channel_delivery_performed": false,
            "activation_command_result_receipt_terminal_operator_decision_identity_accepted": false,
            "activation_command_result_receipt_terminal_operator_decision_signature_accepted": false,
            "activation_command_result_receipt_terminal_operator_decision_timestamp_accepted": false,
            "activation_command_result_receipt_terminal_operator_decision_final_state_promoted": false,
            "activation_command_result_receipt_terminal_operator_decision_completion_promoted": false,
            "activation_command_result_receipt_public_claim_requested": false,
            "activation_command_result_receipt_public_claim_accepted": false,
            "activation_command_result_receipt_public_claim_recorded": false,
            "activation_command_result_receipt_public_claim_persisted": false,
            "activation_command_result_receipt_public_claim_materialized": false,
            "activation_command_result_receipt_public_claim_promoted": false,
            "activation_command_result_receipt_public_ga_claimed": false,
            "activation_command_result_receipt_public_release_published": false,
            "activation_command_result_receipt_public_distribution_performed": false,
            "activation_command_result_receipt_public_artifact_written": false,
            "telegram_send_performed": false,
            "channel_send_performed": false,
            "external_send_performed": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "activation_allowed_by_result_receipt_terminal_operator_decision": false,
            "activation_allowed_by_result_receipt_final_operator_acknowledgement": false,
            "activation_allowed_by_result_receipt": false,
            "activation_allowed": false,
            "activation_performed": false,
            "live_mutation_execution_ready": false,
            "live_mutation_execution_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_write_execution_allowed": false,
            "memory_write_execution_ready": false,
            "memory_write_execution_performed": false,
            "memory_store_write_path_enabled": false,
            "memory_store_write_allowed": false,
            "memory_store_write_performed": false,
            "memory_store_write_performed_count": 0,
            "memory_store_mutation_allowed": false,
            "memory_store_mutated": false,
            "rollback_execution_allowed": false,
            "rollback_executed": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "secret_material_read": false,
            "provider_prompt_replay_enabled": false,
            "provider_invoked": false,
            "model_invoked": false,
            "public_release_published": false,
            "public_ga_claimed": false,
            "release_artifact_written": false,
            "public_artifact_written": false,
            "public_distribution_performed": false,
            "install_executed": false,
            "launchd_mutated": false,
            "service_restarted": false,
            "service_restart_performed": false,
            "active_binary_mutated": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "activation_command_result_receipt_terminal_operator_decision_public_claim_surfaces": [
                "source_final_operator_acknowledgement_report_required",
                "terminal_operator_decision_request_shape_denied",
                "terminal_operator_decision_acceptance_denied",
                "terminal_operator_decision_recording_denied",
                "terminal_operator_decision_persistence_denied",
                "terminal_operator_decision_materialization_denied",
                "operator_identity_signature_terminal_decision_acceptance_denied",
                "terminal_operator_decision_delivery_denied",
                "public_claim_request_non_promotion_denied",
                "public_ga_release_publication_promotion_denied",
                "activation_from_terminal_operator_decision_denied",
                "external_public_install_restart_active_binary_terminal_decision_denied"
            ],
            "denied_by_activation_command_result_receipt_terminal_operator_decision_public_claim": denials,
            "denied_by_activation_command_result_receipt_terminal_operator_decision_public_claim_count": denied_count,
            "current_live_enabled_lane_count": 27,
            "enablement_lane_count": 30,
            "ready_enablement_lane_count": 30,
            "allowed_next_actions": [
                {
                    "action": "review_operator_review_acknowledgement_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial",
                    "status": "allowed_report_only",
                    "accepts_terminal_decision": false,
                    "claims_public_release": false,
                    "writes_release_artifact": false,
                    "activates_runtime": false,
                    "invokes_model": false,
                    "writes_memory_or_kg": false
                },
                {
                    "action": "stage_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_denial",
                    "status": "allowed_report_only_next_slice",
                    "publishes_release_artifact": false,
                    "claims_public_release": false,
                    "writes_release_artifact": false,
                    "activates_runtime": false,
                    "invokes_model": false,
                    "writes_memory_or_kg": false
                }
            ],
        }),
    );
    if let Some(side_effects) = report
        .get_mut("side_effects")
        .and_then(serde_json::Value::as_object_mut)
    {
        for key in [
            "activation_command_result_receipt_terminal_operator_decision_recorded",
            "activation_command_result_receipt_terminal_operator_decision_persisted",
            "activation_command_result_receipt_terminal_operator_decision_materialized",
            "activation_command_result_receipt_terminal_operator_decision_filesystem_written",
            "activation_command_result_receipt_terminal_operator_decision_delivered",
            "activation_command_result_receipt_terminal_operator_decision_channel_delivery_performed",
            "activation_command_result_receipt_terminal_operator_decision_identity_accepted",
            "activation_command_result_receipt_terminal_operator_decision_signature_accepted",
            "activation_command_result_receipt_terminal_operator_decision_timestamp_accepted",
            "activation_command_result_receipt_terminal_operator_decision_final_state_promoted",
            "activation_command_result_receipt_terminal_operator_decision_completion_promoted",
            "activation_command_result_receipt_public_claim_recorded",
            "activation_command_result_receipt_public_claim_persisted",
            "activation_command_result_receipt_public_claim_materialized",
            "activation_command_result_receipt_public_claim_promoted",
            "activation_command_result_receipt_public_ga_claimed",
            "activation_command_result_receipt_public_release_published",
            "activation_command_result_receipt_public_distribution_performed",
            "activation_command_result_receipt_public_artifact_written",
            "telegram_send_performed",
            "channel_send_performed",
            "external_send_performed",
            "activation_command_result_receipt_final_operator_acknowledgement_recorded",
            "activation_command_result_receipt_final_operator_acknowledgement_persisted",
            "activation_command_result_receipt_final_operator_acknowledgement_materialized",
            "activation_command_result_receipt_final_operator_acknowledgement_filesystem_written",
            "activation_command_result_receipt_final_operator_acknowledgement_delivered",
            "activation_command_result_receipt_operator_final_acceptance_recorded",
            "activation_command_result_receipt_operator_final_acceptance_persisted",
            "activation_command_result_receipt_recorded",
            "activation_command_result_receipt_persisted",
            "activation_command_result_receipt_accepted",
            "activation_command_result_receipt_materialized",
            "activation_command_result_receipt_filesystem_written",
            "activation_command_completion_ack_recorded",
            "activation_command_completion_ack_persisted",
            "activation_command_completion_ack_accepted",
            "activation_command_completion_ack_delivered",
            "activation_command_enabled",
            "activation_command_invoked",
            "activation_command_dispatched",
            "activation_performed",
            "live_mutation_execution_performed",
            "memory_write_execution_performed",
            "memory_store_write_performed",
            "memory_store_mutated",
            "live_kg_write_performed",
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
            "provider_prompt_replayed",
            "runtime_router_mutated",
            "runtime_attachment_performed",
            "live_context_attached",
            "context_injection_performed",
            "adapter_invoked",
            "filesystem_written",
            "public_release_published",
            "public_release_claimed",
            "public_ga_claimed",
            "release_artifact_written",
            "public_artifact_written",
            "public_distribution_performed",
            "install_executed",
            "launchd_mutated",
            "service_restarted",
            "service_restart_performed",
            "active_binary_mutated",
        ] {
            side_effects.insert(key.to_string(), serde_json::json!(false));
        }
    }
    report
}
fn hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source_terminal_decision = hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_report();
    let source_ready = source_terminal_decision
        .get("operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_ready")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);
    let source_status = source_terminal_decision
        .get("status")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("blocked");
    let source_report_sha256 = sha256_json_value(&source_terminal_decision);
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let source_u64 = |key: &str| -> u64 {
        source_terminal_decision
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let report_ready = source_ready
        && source_status == "ready"
        && route_count_source_command_accepted
        && source_u64(
            "activation_command_result_receipt_terminal_operator_decision_public_claim_fixture_count",
        ) == 10
        && source_u64(
            "blocked_activation_command_result_receipt_terminal_operator_decision_public_claim_fixture_count",
        ) == 10
        && source_u64(
            "allowed_activation_command_result_receipt_terminal_operator_decision_public_claim_fixture_count",
        ) == 0
        && source_u64(
            "activation_command_result_receipt_terminal_operator_decision_performed_count",
        ) == 0
        && source_u64("activation_command_result_receipt_public_claim_promotion_performed_count")
            == 0;

    let publication_fixture =
        |id: &str, status: &str, reason: &str, extra: serde_json::Value| -> serde_json::Value {
            let mut fixture = serde_json::Map::new();
            fixture.insert("id".to_string(), serde_json::json!(id));
            fixture.insert(
                "release_artifact_publication_status".to_string(),
                serde_json::json!(status),
            );
            fixture.insert(
                "source_terminal_operator_decision_present".to_string(),
                serde_json::json!(true),
            );
            fixture.insert(
                "source_terminal_operator_decision_ready".to_string(),
                serde_json::json!(true),
            );
            for key in [
                "release_artifact_publication_requested",
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
                "live_mutation_execution_performed",
                "memory_write_execution_performed",
                "memory_store_write_performed",
                "memory_store_mutated",
                "rollback_executed",
                "secret_material_read",
                "credential_read",
                "secret_file_read",
                "provider_invoked",
                "model_invoked",
                "install_executed",
                "launchd_mutated",
                "service_restarted",
                "active_binary_mutated",
            ] {
                fixture.insert(key.to_string(), serde_json::json!(false));
            }
            fixture.insert(
                "release_artifact_publication_noop_confirmed".to_string(),
                serde_json::json!(true),
            );
            fixture.insert("reason".to_string(), serde_json::json!(reason));
            if let Some(extra_object) = extra.as_object() {
                fixture.extend(extra_object.clone());
            }
            serde_json::Value::Object(fixture)
        };

    let release_artifact_publication_fixtures = serde_json::json!([
        publication_fixture(
            "operator-canary-controlled-request-harness-activation-result-receipt-release-artifact-publication-missing-terminal-decision",
            "blocked_noop",
            "source_terminal_operator_decision_report_required",
            serde_json::json!({
                "source_terminal_operator_decision_present": false,
                "source_terminal_operator_decision_ready": false,
                "release_artifact_publication_requested": true,
            }),
        ),
        publication_fixture(
            "operator-canary-controlled-request-harness-activation-result-receipt-release-artifact-write-request",
            "blocked_artifact_noop",
            "release_artifact_write_denied",
            serde_json::json!({
                "release_artifact_write_requested": true,
                "release_artifact_publication_requested": true,
            }),
        ),
        publication_fixture(
            "operator-canary-controlled-request-harness-activation-result-receipt-public-artifact-write-request",
            "blocked_artifact_noop",
            "public_artifact_write_denied",
            serde_json::json!({
                "public_artifact_write_requested": true,
                "release_artifact_publication_requested": true,
            }),
        ),
        publication_fixture(
            "operator-canary-controlled-request-harness-activation-result-receipt-artifact-signature-notarization-request",
            "blocked_artifact_noop",
            "artifact_signature_notarization_acceptance_denied",
            serde_json::json!({
                "artifact_signature_requested": true,
                "artifact_notarization_requested": true,
                "release_artifact_publication_requested": true,
            }),
        ),
        publication_fixture(
            "operator-canary-controlled-request-harness-activation-result-receipt-publication-queue-request",
            "blocked_publication_noop",
            "publication_queue_enqueue_denied",
            serde_json::json!({
                "publication_queue_enqueue_requested": true,
                "publication_manifest_write_requested": true,
            }),
        ),
        publication_fixture(
            "operator-canary-controlled-request-harness-activation-result-receipt-distribution-channel-request",
            "blocked_distribution_noop",
            "public_distribution_channel_delivery_denied",
            serde_json::json!({
                "public_distribution_requested": true,
                "telegram_delivery_requested": true,
                "channel_delivery_requested": true,
                "external_delivery_requested": true,
            }),
        ),
        publication_fixture(
            "operator-canary-controlled-request-harness-activation-result-receipt-public-version-tag-request",
            "blocked_release_noop",
            "public_version_tag_release_promotion_denied",
            serde_json::json!({
                "public_version_tag_requested": true,
                "public_release_publish_requested": true,
                "public_ga_claim_requested": true,
            }),
        ),
        publication_fixture(
            "operator-canary-controlled-request-harness-activation-result-receipt-release-notes-changelog-request",
            "blocked_artifact_noop",
            "release_notes_changelog_materialization_denied",
            serde_json::json!({
                "release_notes_materialization_requested": true,
                "changelog_materialization_requested": true,
            }),
        ),
        publication_fixture(
            "operator-canary-controlled-request-harness-activation-result-receipt-terminal-decision-as-release-approval",
            "blocked_promotion_noop",
            "terminal_operator_decision_is_not_release_approval",
            serde_json::json!({
                "terminal_operator_decision_release_approval_requested": true,
                "release_artifact_publication_requested": true,
            }),
        ),
        publication_fixture(
            "operator-canary-controlled-request-harness-activation-result-receipt-release-publication-activation-memory-provider-install",
            "blocked_promotion_noop",
            "activation_memory_provider_install_restart_active_binary_publication_denied",
            serde_json::json!({
                "activation_from_release_publication_requested": true,
                "memory_write_publication_requested": true,
                "provider_prompt_publication_requested": true,
                "install_publication_requested": true,
                "service_restart_publication_requested": true,
                "active_binary_publication_requested": true,
            }),
        ),
    ]);
    let release_artifact_publication_fixture_count = release_artifact_publication_fixtures
        .as_array()
        .map(std::vec::Vec::len)
        .unwrap_or(0);
    let fixtures_sha256 = sha256_json_value(&release_artifact_publication_fixtures);
    let contract_hash_sha256 = sha256_text_value(&format!(
        "hepta-canary-operator-review-acknowledgement-activation-command-result-receipt-release-artifact-publication-denial:v1:source={source_report_sha256}:fixtures={fixtures_sha256}:publication=0:artifact=0:release=0:install=0"
    ));
    let policy_hash_sha256 = sha256_text_value(
        "memory-intelligence-kg-operator-canary-harness-operator-review-acknowledgement-activation-command-result-receipt-release-artifact-publication-denial:v1:no-release-artifact:no-public-artifact:no-publication:no-distribution:no-install:no-live",
    );
    let side_effect_hash_sha256 = sha256_text_value(
        "release_artifact_publication=false;release_artifact=false;public_artifact=false;signature=false;notarization=false;publication_queue=false;public_release=false;public_ga=false;distribution=false;activation=false;provider=false;model=false;memory=false;kg=false;secret=false;install=false;restart=false;active_binary=false",
    );

    let mut denials = source_terminal_decision
        .get("denied_by_activation_command_result_receipt_terminal_operator_decision_public_claim")
        .and_then(serde_json::Value::as_array)
        .cloned()
        .unwrap_or_default();
    for denial in [
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
    ] {
        denials.push(serde_json::Value::String(denial.to_string()));
    }
    let denied_count = denials.len();

    let mut report = source_terminal_decision.clone();
    extend_json_object(
        &mut report,
        serde_json::json!({
            "gate": "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_denial_route",
            "status": if report_ready { "ready" } else { "blocked" },
            "source_command": "/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-release-artifact-publication-denial --json",
            "native_route": true,
            "compatibility_mode": "native_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_denial_status",
            "side_effect_free": true,
            "audit_date": "2026-06-14",
            "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_RELEASE_ARTIFACT_PUBLICATION_DENIAL_ENDPOINT,
            "source_operator_review_acknowledgement_activation_command_result_receipt_terminal_operator_decision_public_claim_route_endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_TERMINAL_OPERATOR_DECISION_PUBLIC_CLAIM_NON_PROMOTION_DENIAL_ENDPOINT,
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_route_doc": "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_RELEASE_ARTIFACT_PUBLICATION_DENIAL_ROUTE_GATE.md",
            "source_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_terminal_operator_decision_public_claim_route_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-terminal-operator-decision-public-claim-non-promotion-denial-route-gate.sh",
            "source_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-release-artifact-publication-denial-gate.sh",
            "source_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_route_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-release-artifact-publication-denial-route-gate.sh",
            "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
            "route_count": route_matrix.route_count,
            "implemented_route_count": route_matrix.implemented_route_count,
            "missing_route_count": route_matrix.missing_route_count,
            "route_count_source_command_accepted": route_count_source_command_accepted,
            "source_route_wired": true,
            "source_operator_review_acknowledgement_activation_command_result_receipt_terminal_operator_decision_public_claim_route_ready": source_ready,
            "source_operator_review_acknowledgement_activation_command_result_receipt_terminal_operator_decision_public_claim_route_status": source_status,
            "source_operator_review_acknowledgement_activation_command_result_receipt_terminal_operator_decision_public_claim_route_report_sha256": source_report_sha256,
            "operator_authorization_scope": "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_no_artifact_no_publication_no_release_no_distribution_no_install_no_context_memory_kg_provider_model_credential_channel_restart_binary",
            "operator_authorization_received": true,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_denial_route_enabled": true,
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_denial_ready": true,
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_denial_status": "blocked",
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_ready": true,
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_ready": true,
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_ready": true,
            "activation_command_result_receipt_release_artifact_publication_schema_version": "memory_intelligence_kg_operator_canary_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_denial_v1",
            "activation_command_result_receipt_release_artifact_publication_mode": "native_route_stdout_only_release_artifact_publication_denial_no_artifact_no_publication_no_release_no_distribution_no_install_no_live",
            "activation_command_result_receipt_release_artifact_publication_decision": "blocked_noop_terminal_operator_decision_cannot_be_promoted_into_release_artifact_publication_authority",
            "source_terminal_operator_decision_public_claim_fixture_count": source_u64("activation_command_result_receipt_terminal_operator_decision_public_claim_fixture_count"),
            "source_blocked_terminal_operator_decision_public_claim_fixture_count": source_u64("blocked_activation_command_result_receipt_terminal_operator_decision_public_claim_fixture_count"),
            "source_accepted_terminal_operator_decision_public_claim_fixture_count": source_u64("accepted_activation_command_result_receipt_terminal_operator_decision_public_claim_fixture_count"),
            "source_terminal_operator_decision_performed_count": source_u64("activation_command_result_receipt_terminal_operator_decision_performed_count"),
            "source_public_claim_promotion_performed_count": source_u64("activation_command_result_receipt_public_claim_promotion_performed_count"),
            "release_artifact_publication_fixtures_sha256": fixtures_sha256,
            "release_artifact_publication_contract_hash_sha256": contract_hash_sha256,
            "release_artifact_publication_policy_hash_sha256": policy_hash_sha256,
            "side_effect_hash_sha256": side_effect_hash_sha256,
            "required_activation_command_result_receipt_release_artifact_publication_surface_count": 12,
            "ready_activation_command_result_receipt_release_artifact_publication_surface_count": 12,
            "side_effect_free_activation_command_result_receipt_release_artifact_publication_surface_count": 12,
            "required_activation_command_result_receipt_release_artifact_publication_fixture_count": 10,
            "activation_command_result_receipt_release_artifact_publication_fixtures": release_artifact_publication_fixtures,
            "activation_command_result_receipt_release_artifact_publication_fixture_count": release_artifact_publication_fixture_count,
            "blocked_activation_command_result_receipt_release_artifact_publication_fixture_count": release_artifact_publication_fixture_count,
            "noop_activation_command_result_receipt_release_artifact_publication_fixture_count": release_artifact_publication_fixture_count,
            "allowed_activation_command_result_receipt_release_artifact_publication_fixture_count": 0,
            "accepted_activation_command_result_receipt_release_artifact_publication_fixture_count": 0,
            "activation_command_result_receipt_release_artifact_publication_performed_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
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
            "telegram_send_performed": false,
            "channel_send_performed": false,
            "external_send_performed": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "activation_command_result_receipt_recorded": false,
            "activation_command_result_receipt_persisted": false,
            "activation_command_result_receipt_accepted": false,
            "activation_command_result_receipt_materialized": false,
            "activation_command_completion_ack_recorded": false,
            "activation_allowed_by_release_artifact_publication": false,
            "activation_allowed_by_terminal_operator_decision": false,
            "activation_allowed_by_result_receipt": false,
            "activation_allowed": false,
            "activation_performed": false,
            "live_mutation_execution_ready": false,
            "live_mutation_execution_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_write_execution_allowed": false,
            "memory_write_execution_ready": false,
            "memory_write_execution_performed": false,
            "memory_store_write_path_enabled": false,
            "memory_store_write_allowed": false,
            "memory_store_write_performed": false,
            "memory_store_write_performed_count": 0,
            "memory_store_mutation_allowed": false,
            "memory_store_mutated": false,
            "rollback_execution_allowed": false,
            "rollback_executed": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "secret_material_read": false,
            "credential_read": false,
            "secret_file_read": false,
            "provider_prompt_replay_enabled": false,
            "provider_invoked": false,
            "model_invoked": false,
            "install_executed": false,
            "launchd_mutated": false,
            "service_restarted": false,
            "service_restart_performed": false,
            "active_binary_mutated": false,
            "activation_command_result_receipt_release_artifact_publication_surfaces": [
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
                "external_public_install_restart_active_binary_publication_denied"
            ],
            "denied_by_activation_command_result_receipt_release_artifact_publication": denials,
            "denied_by_activation_command_result_receipt_release_artifact_publication_count": denied_count,
            "current_live_enabled_lane_count": 28,
            "enablement_lane_count": 31,
            "ready_enablement_lane_count": 31,
            "allowed_next_actions": [
                {
                    "action": "review_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_denial",
                    "status": "allowed_report_only",
                    "publishes_release_artifact": false,
                    "claims_public_release": false,
                    "writes_release_artifact": false,
                    "installs_or_restarts": false,
                    "activates_runtime": false,
                    "invokes_model": false,
                    "writes_memory_or_kg": false
                },
                {
                    "action": "stage_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_result_receipt_no_persistence",
                    "status": "allowed_report_only_next_slice",
                    "records_publication_receipt": false,
                    "persists_publication_receipt": false,
                    "publishes_release_artifact": false,
                    "claims_public_release": false,
                    "activates_runtime": false,
                    "invokes_model": false,
                    "writes_memory_or_kg": false
                }
            ],
        }),
    );
    if let Some(side_effects) = report
        .get_mut("side_effects")
        .and_then(serde_json::Value::as_object_mut)
    {
        for key in [
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
            "public_release_claimed",
            "public_ga_claimed",
            "public_claim_promoted",
            "public_version_tag_created",
            "release_notes_materialized",
            "changelog_materialized",
            "terminal_operator_decision_promoted_to_release_approval",
            "telegram_send_performed",
            "channel_send_performed",
            "external_send_performed",
            "activation_command_result_receipt_terminal_operator_decision_recorded",
            "activation_command_result_receipt_terminal_operator_decision_persisted",
            "activation_command_result_receipt_public_claim_recorded",
            "activation_command_result_receipt_public_claim_promoted",
            "activation_command_result_receipt_recorded",
            "activation_command_result_receipt_persisted",
            "activation_command_result_receipt_accepted",
            "activation_command_result_receipt_materialized",
            "activation_command_completion_ack_recorded",
            "activation_command_enabled",
            "activation_command_invoked",
            "activation_command_dispatched",
            "activation_performed",
            "live_mutation_execution_performed",
            "memory_write_execution_performed",
            "memory_store_write_performed",
            "memory_store_mutated",
            "live_kg_write_performed",
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
            "provider_prompt_replayed",
            "runtime_router_mutated",
            "runtime_attachment_performed",
            "live_context_attached",
            "context_injection_performed",
            "adapter_invoked",
            "filesystem_written",
            "install_executed",
            "launchd_mutated",
            "service_restarted",
            "service_restart_performed",
            "active_binary_mutated",
        ] {
            side_effects.insert(key.to_string(), serde_json::json!(false));
        }
    }
    report
}

fn hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_result_receipt_no_persistence_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source_publication = hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_denial_report();
    let source_ready = source_publication
        .get("operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_denial_ready")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);
    let source_status = source_publication
        .get("status")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("blocked");
    let source_report_sha256 = sha256_json_value(&source_publication);
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let source_u64 = |key: &str| -> u64 {
        source_publication
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let report_ready = source_ready
        && source_status == "ready"
        && route_count_source_command_accepted
        && source_u64(
            "activation_command_result_receipt_release_artifact_publication_fixture_count",
        ) == 10
        && source_u64(
            "blocked_activation_command_result_receipt_release_artifact_publication_fixture_count",
        ) == 10
        && source_u64(
            "allowed_activation_command_result_receipt_release_artifact_publication_fixture_count",
        ) == 0
        && source_u64(
            "activation_command_result_receipt_release_artifact_publication_performed_count",
        ) == 0;

    let receipt_fixture =
        |id: &str, status: &str, reason: &str, extra: serde_json::Value| -> serde_json::Value {
            let mut fixture = serde_json::Map::new();
            fixture.insert("id".to_string(), serde_json::json!(id));
            fixture.insert(
                "publication_result_receipt_status".to_string(),
                serde_json::json!(status),
            );
            fixture.insert(
                "source_release_artifact_publication_present".to_string(),
                serde_json::json!(true),
            );
            fixture.insert(
                "source_release_artifact_publication_ready".to_string(),
                serde_json::json!(true),
            );
            for key in [
                "publication_result_receipt_requested",
                "publication_result_receipt_allowed",
                "publication_result_receipt_accepted",
                "publication_result_receipt_recorded",
                "publication_result_receipt_persisted",
                "publication_result_receipt_materialized",
                "publication_result_receipt_filesystem_written",
                "publication_result_receipt_ledger_written",
                "publication_result_receipt_indexed",
                "publication_result_receipt_enqueued",
                "publication_result_receipt_delivered",
                "publication_result_receipt_exported",
                "publication_result_receipt_query_registered",
                "publication_result_receipt_observability_recorded",
                "publication_result_receipt_hash_bound",
                "publication_result_receipt_signature_accepted",
                "publication_result_receipt_timestamp_accepted",
                "publication_result_receipt_status_accepted",
                "publication_completion_ack_recorded",
                "publication_completion_ack_persisted",
                "publication_completion_ack_accepted",
                "release_artifact_publication_recorded",
                "release_artifact_publication_persisted",
                "release_artifact_publication_materialized",
                "release_artifact_filesystem_written",
                "release_artifact_written",
                "public_artifact_written",
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
                "activation_allowed",
                "activation_performed",
                "live_mutation_execution_performed",
                "memory_write_execution_performed",
                "memory_store_write_performed",
                "memory_store_mutated",
                "rollback_executed",
                "secret_material_read",
                "credential_read",
                "secret_file_read",
                "provider_invoked",
                "model_invoked",
                "install_executed",
                "launchd_mutated",
                "service_restarted",
                "active_binary_mutated",
            ] {
                fixture.insert(key.to_string(), serde_json::json!(false));
            }
            fixture.insert(
                "publication_result_receipt_noop_confirmed".to_string(),
                serde_json::json!(true),
            );
            fixture.insert("reason".to_string(), serde_json::json!(reason));
            if let Some(extra_object) = extra.as_object() {
                fixture.extend(extra_object.clone());
            }
            serde_json::Value::Object(fixture)
        };

    let publication_result_receipt_fixtures = serde_json::json!([
        receipt_fixture(
            "operator-canary-release-publication-result-receipt-missing-publication-report",
            "blocked_noop",
            "source_release_artifact_publication_report_required",
            serde_json::json!({
                "source_release_artifact_publication_present": false,
                "source_release_artifact_publication_ready": false,
            }),
        ),
        receipt_fixture(
            "operator-canary-release-publication-result-receipt-record-request",
            "blocked_record_noop",
            "publication_result_receipt_recording_denied",
            serde_json::json!({"publication_result_receipt_record_requested": true}),
        ),
        receipt_fixture(
            "operator-canary-release-publication-result-receipt-persist-request",
            "blocked_persist_noop",
            "publication_result_receipt_persistence_denied",
            serde_json::json!({"publication_result_receipt_persist_requested": true}),
        ),
        receipt_fixture(
            "operator-canary-release-publication-result-receipt-materialize-filesystem-request",
            "blocked_materialize_noop",
            "publication_result_receipt_materialization_filesystem_write_denied",
            serde_json::json!({
                "publication_result_receipt_materialize_requested": true,
                "publication_result_receipt_filesystem_write_requested": true,
            }),
        ),
        receipt_fixture(
            "operator-canary-release-publication-result-receipt-ledger-index-queue-request",
            "blocked_ledger_index_queue_noop",
            "publication_result_receipt_ledger_index_queue_denied",
            serde_json::json!({
                "publication_result_receipt_ledger_write_requested": true,
                "publication_result_receipt_index_requested": true,
                "publication_result_receipt_enqueue_requested": true,
            }),
        ),
        receipt_fixture(
            "operator-canary-release-publication-result-receipt-export-query-observability-request",
            "blocked_export_query_observability_noop",
            "publication_result_receipt_export_query_observability_denied",
            serde_json::json!({
                "publication_result_receipt_export_requested": true,
                "publication_result_receipt_query_requested": true,
                "publication_result_receipt_observability_requested": true,
            }),
        ),
        receipt_fixture(
            "operator-canary-release-publication-result-receipt-delivery-request",
            "blocked_delivery_noop",
            "publication_result_receipt_delivery_denied",
            serde_json::json!({
                "publication_result_receipt_delivery_requested": true,
                "telegram_delivery_requested": true,
                "channel_delivery_requested": true,
                "external_delivery_requested": true,
            }),
        ),
        receipt_fixture(
            "operator-canary-release-publication-result-receipt-status-signature-request",
            "blocked_acceptance_noop",
            "publication_result_receipt_status_signature_acceptance_denied",
            serde_json::json!({
                "publication_result_receipt_status_acceptance_requested": true,
                "publication_result_receipt_signature_acceptance_requested": true,
                "publication_result_receipt_timestamp_acceptance_requested": true,
            }),
        ),
        receipt_fixture(
            "operator-canary-release-publication-result-receipt-completion-ack-request",
            "blocked_ack_noop",
            "publication_completion_ack_denied",
            serde_json::json!({"publication_completion_ack_requested": true}),
        ),
        receipt_fixture(
            "operator-canary-release-publication-result-receipt-authority-request",
            "blocked_authority_noop",
            "publication_result_receipt_cannot_authorize_publication_activation_or_install",
            serde_json::json!({
                "publication_authority_requested": true,
                "public_release_publish_requested": true,
                "public_distribution_requested": true,
                "release_artifact_write_requested": true,
                "activation_from_publication_receipt_requested": true,
                "memory_write_publication_receipt_requested": true,
                "provider_prompt_publication_receipt_requested": true,
                "install_publication_receipt_requested": true,
                "service_restart_publication_receipt_requested": true,
                "active_binary_publication_receipt_requested": true,
            }),
        ),
    ]);
    let publication_result_receipt_fixture_count = publication_result_receipt_fixtures
        .as_array()
        .map(std::vec::Vec::len)
        .unwrap_or(0);
    let fixtures_sha256 = sha256_json_value(&publication_result_receipt_fixtures);
    let contract_hash_sha256 = sha256_text_value(&format!(
        "hepta-canary-release-artifact-publication-result-receipt-no-persistence:v1:source={source_report_sha256}:fixtures={fixtures_sha256}:record=0:persist=0:deliver=0:authority=0"
    ));
    let policy_hash_sha256 = sha256_text_value(
        "memory-intelligence-kg-operator-canary-harness-release-artifact-publication-result-receipt-no-persistence:v1:no-record:no-persist:no-deliver:no-authority:no-install:no-live",
    );
    let side_effect_hash_sha256 = sha256_text_value(
        "publication_result_receipt=false;completion_ack=false;release_artifact=false;public_release=false;delivery=false;activation=false;provider=false;model=false;memory=false;kg=false;secret=false;install=false;restart=false;active_binary=false",
    );

    let mut denials = source_publication
        .get("denied_by_activation_command_result_receipt_release_artifact_publication")
        .and_then(serde_json::Value::as_array)
        .cloned()
        .unwrap_or_default();
    for denial in [
        "source_release_artifact_publication_report_required",
        "publication_result_receipt_recording_denied",
        "publication_result_receipt_persistence_denied",
        "publication_result_receipt_materialization_denied",
        "publication_result_receipt_filesystem_write_denied",
        "publication_result_receipt_ledger_index_queue_denied",
        "publication_result_receipt_export_query_observability_denied",
        "publication_result_receipt_delivery_denied",
        "publication_result_receipt_status_signature_acceptance_denied",
        "publication_completion_ack_denied",
        "publication_result_receipt_publication_authority_denied",
        "publication_result_receipt_activation_authority_denied",
        "publication_result_receipt_memory_provider_install_restart_active_binary_denied",
    ] {
        denials.push(serde_json::Value::String(denial.to_string()));
    }
    let denied_count = denials.len();

    let mut report = source_publication.clone();
    extend_json_object(
        &mut report,
        serde_json::json!({
            "gate": "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_result_receipt_no_persistence_route",
            "status": if report_ready { "ready" } else { "blocked" },
            "source_command": "/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-release-artifact-publication-result-receipt-no-persistence --json",
            "native_route": true,
            "compatibility_mode": "native_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_result_receipt_no_persistence_status",
            "side_effect_free": true,
            "audit_date": "2026-06-14",
            "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_RELEASE_ARTIFACT_PUBLICATION_RESULT_RECEIPT_NO_PERSISTENCE_ENDPOINT,
            "source_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_route_endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_CANARY_CONTROLLED_REQUEST_HARNESS_OPERATOR_REVIEW_ACKNOWLEDGEMENT_ACTIVATION_COMMAND_RESULT_RECEIPT_RELEASE_ARTIFACT_PUBLICATION_DENIAL_ENDPOINT,
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_result_receipt_route_doc": "docs/architecture/i3-cb5f2426a8596a77a7d84915.md",
            "source_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-release-artifact-publication-denial-gate.sh",
            "source_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_route_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-release-artifact-publication-denial-route-gate.sh",
            "source_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_result_receipt_gate": "scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-release-artifact-publication-result-receipt-no-persistence-gate.sh",
            "source_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_route_ready": source_ready,
            "source_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_route_status": source_status,
            "source_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_report_sha256": source_report_sha256,
            "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
            "route_count": route_matrix.route_count,
            "implemented_route_count": route_matrix.implemented_route_count,
            "missing_route_count": route_matrix.missing_route_count,
            "route_count_source_command_accepted": route_count_source_command_accepted,
            "source_route_wired": true,
            "operator_authorization_scope": "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_result_receipt_no_record_no_persist_no_materialize_no_deliver_no_authority_no_install_no_context_memory_kg_provider_model_credential_channel_restart_binary",
            "operator_authorization_received": true,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_result_receipt_no_persistence_route_enabled": true,
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_result_receipt_no_persistence_ready": true,
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_result_receipt_no_persistence_status": "blocked",
            "operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_denial_ready": true,
            "activation_command_result_receipt_release_artifact_publication_result_receipt_schema_version": "memory_intelligence_kg_operator_canary_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_result_receipt_no_persistence_v1",
            "activation_command_result_receipt_release_artifact_publication_result_receipt_mode": "native_route_stdout_only_publication_result_receipt_no_persistence_no_delivery_no_authority_no_install_no_live",
            "activation_command_result_receipt_release_artifact_publication_result_receipt_decision": "blocked_noop_release_artifact_publication_result_receipts_cannot_be_persisted_or_promoted_into_authority",
            "source_release_artifact_publication_fixture_count": source_u64("activation_command_result_receipt_release_artifact_publication_fixture_count"),
            "source_blocked_release_artifact_publication_fixture_count": source_u64("blocked_activation_command_result_receipt_release_artifact_publication_fixture_count"),
            "source_accepted_release_artifact_publication_fixture_count": source_u64("accepted_activation_command_result_receipt_release_artifact_publication_fixture_count"),
            "source_release_artifact_publication_performed_count": source_u64("activation_command_result_receipt_release_artifact_publication_performed_count"),
            "publication_result_receipt_fixtures_sha256": fixtures_sha256,
            "publication_result_receipt_contract_hash_sha256": contract_hash_sha256,
            "publication_result_receipt_policy_hash_sha256": policy_hash_sha256,
            "publication_result_receipt_side_effect_hash_sha256": side_effect_hash_sha256,
            "required_activation_command_result_receipt_release_artifact_publication_result_receipt_surface_count": 12,
            "ready_activation_command_result_receipt_release_artifact_publication_result_receipt_surface_count": 12,
            "side_effect_free_activation_command_result_receipt_release_artifact_publication_result_receipt_surface_count": 12,
            "required_activation_command_result_receipt_release_artifact_publication_result_receipt_fixture_count": 10,
            "activation_command_result_receipt_release_artifact_publication_result_receipt_fixtures": publication_result_receipt_fixtures,
            "activation_command_result_receipt_release_artifact_publication_result_receipt_fixture_count": publication_result_receipt_fixture_count,
            "blocked_activation_command_result_receipt_release_artifact_publication_result_receipt_fixture_count": publication_result_receipt_fixture_count,
            "noop_activation_command_result_receipt_release_artifact_publication_result_receipt_fixture_count": publication_result_receipt_fixture_count,
            "allowed_activation_command_result_receipt_release_artifact_publication_result_receipt_fixture_count": 0,
            "accepted_activation_command_result_receipt_release_artifact_publication_result_receipt_fixture_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "publication_result_receipt_allowed": false,
            "publication_result_receipt_accepted": false,
            "publication_result_receipt_recorded": false,
            "publication_result_receipt_persisted": false,
            "publication_result_receipt_materialized": false,
            "publication_result_receipt_filesystem_written": false,
            "publication_result_receipt_ledger_written": false,
            "publication_result_receipt_indexed": false,
            "publication_result_receipt_enqueued": false,
            "publication_result_receipt_delivered": false,
            "publication_result_receipt_exported": false,
            "publication_result_receipt_query_registered": false,
            "publication_result_receipt_observability_recorded": false,
            "publication_result_receipt_hash_bound": false,
            "publication_result_receipt_signature_accepted": false,
            "publication_result_receipt_timestamp_accepted": false,
            "publication_result_receipt_status_accepted": false,
            "publication_completion_ack_recorded": false,
            "publication_completion_ack_persisted": false,
            "publication_completion_ack_accepted": false,
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
            "public_version_tag_created": false,
            "release_notes_materialized": false,
            "changelog_materialized": false,
            "terminal_operator_decision_promoted_to_release_approval": false,
            "telegram_send_performed": false,
            "channel_send_performed": false,
            "external_send_performed": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "activation_allowed_by_publication_result_receipt": false,
            "activation_allowed_by_release_artifact_publication": false,
            "activation_allowed_by_terminal_operator_decision": false,
            "activation_allowed_by_result_receipt": false,
            "activation_allowed": false,
            "activation_performed": false,
            "live_mutation_execution_ready": false,
            "live_mutation_execution_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_write_execution_allowed": false,
            "memory_write_execution_ready": false,
            "memory_write_execution_performed": false,
            "memory_store_write_path_enabled": false,
            "memory_store_write_allowed": false,
            "memory_store_write_performed": false,
            "memory_store_write_performed_count": 0,
            "memory_store_mutation_allowed": false,
            "memory_store_mutated": false,
            "rollback_execution_allowed": false,
            "rollback_executed": false,
            "secret_material_read": false,
            "credential_read": false,
            "secret_file_read": false,
            "provider_prompt_replay_enabled": false,
            "provider_invoked": false,
            "model_invoked": false,
            "install_executed": false,
            "launchd_mutated": false,
            "service_restarted": false,
            "service_restart_performed": false,
            "active_binary_mutated": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "activation_command_result_receipt_release_artifact_publication_result_receipt_surfaces": [
                "source_release_artifact_publication_report_required",
                "publication_result_receipt_recording_denied",
                "publication_result_receipt_persistence_denied",
                "publication_result_receipt_materialization_denied",
                "publication_result_receipt_filesystem_write_denied",
                "publication_result_receipt_ledger_index_queue_denied",
                "publication_result_receipt_export_query_observability_denied",
                "publication_result_receipt_delivery_denied",
                "publication_result_receipt_status_signature_acceptance_denied",
                "publication_completion_ack_denied",
                "publication_result_receipt_authority_denied",
                "publication_result_receipt_external_install_restart_active_binary_denied"
            ],
            "denied_by_activation_command_result_receipt_release_artifact_publication_result_receipt_no_persistence": denials,
            "denied_by_activation_command_result_receipt_release_artifact_publication_result_receipt_no_persistence_count": denied_count,
            "current_live_enabled_lane_count": 29,
            "enablement_lane_count": 32,
            "ready_enablement_lane_count": 32,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "allowed_next_actions": [
                {
                    "action": "review_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_result_receipt_no_persistence",
                    "status": "allowed_report_only",
                    "records_publication_receipt": false,
                    "persists_publication_receipt": false,
                    "delivers_publication_receipt": false,
                    "derives_activation_authority": false,
                    "publishes_release_artifact": false,
                    "claims_public_release": false,
                    "installs_or_restarts": false,
                    "activates_runtime": false,
                    "invokes_model": false,
                    "writes_memory_or_kg": false
                }
            ],
        }),
    );
    if let Some(side_effects) = report
        .get_mut("side_effects")
        .and_then(serde_json::Value::as_object_mut)
    {
        for key in [
            "publication_result_receipt_recorded",
            "publication_result_receipt_persisted",
            "publication_result_receipt_materialized",
            "publication_result_receipt_filesystem_written",
            "publication_result_receipt_ledger_written",
            "publication_result_receipt_indexed",
            "publication_result_receipt_enqueued",
            "publication_result_receipt_delivered",
            "publication_result_receipt_exported",
            "publication_result_receipt_query_registered",
            "publication_result_receipt_observability_recorded",
            "publication_result_receipt_hash_bound",
            "publication_result_receipt_signature_accepted",
            "publication_result_receipt_timestamp_accepted",
            "publication_result_receipt_status_accepted",
            "publication_completion_ack_recorded",
            "publication_completion_ack_persisted",
            "publication_completion_ack_accepted",
            "release_artifact_publication_recorded",
            "release_artifact_publication_persisted",
            "release_artifact_publication_materialized",
            "release_artifact_filesystem_written",
            "release_artifact_written",
            "public_artifact_written",
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
            "install_executed",
            "launchd_mutated",
            "service_restarted",
            "active_binary_mutated",
            "filesystem_written",
        ] {
            side_effects.insert(key.to_string(), serde_json::json!(false));
        }
    }
    report
}

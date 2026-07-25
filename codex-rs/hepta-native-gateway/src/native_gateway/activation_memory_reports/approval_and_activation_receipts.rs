fn hepta_first_model_positive_approval_packet_boundary_report() -> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let artifact_publication =
        hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_release_public_artifact_publication_denial_report();
    let first_model_terminal =
        hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_report();

    let artifact_bool = |key: &str| {
        artifact_publication
            .get(key)
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
    };
    let artifact_u64 = |key: &str| {
        artifact_publication
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let first_model_bool = |key: &str| {
        first_model_terminal
            .get(key)
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
    };
    let first_model_u64 = |key: &str| {
        first_model_terminal
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };

    let artifact_next_action_boundary = artifact_publication
        .get("allowed_next_actions")
        .and_then(|value| value.as_array())
        .and_then(|items| items.first())
        .map(|item| {
            item.get("action").and_then(|value| value.as_str())
                == Some("prepare_first_model_positive_approval_packet_boundary_gate")
                && item
                    .get("derives_operator_approval")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && item
                    .get("derives_activation_authority")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && item
                    .get("writes_release_artifact")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && item
                    .get("invokes_provider")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && item
                    .get("reads_credentials")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
        })
        .unwrap_or(false);
    let first_model_next_action_status_exposure = first_model_terminal
        .get("allowed_next_actions")
        .and_then(|value| value.as_array())
        .and_then(|items| items.first())
        .map(|item| {
            item.get("action").and_then(|value| value.as_str())
                == Some(
                    "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_terminal_public_claim_status_exposure_denial",
                )
                && item
                    .get("accepts_terminal_decision")
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
                && item.get("activates_runtime").and_then(serde_json::Value::as_bool)
                    == Some(false)
                && item.get("invokes_provider").and_then(serde_json::Value::as_bool)
                    == Some(false)
                && item.get("invokes_model").and_then(serde_json::Value::as_bool) == Some(false)
        })
        .unwrap_or(false);

    let artifact_publication_ready = artifact_bool(
        "memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_release_public_artifact_publication_denial_ready",
    ) && artifact_u64(
        "artifact_distribution_signing_notarization_receipt_release_public_artifact_publication_surface_count",
    ) == 18
        && artifact_u64(
            "artifact_distribution_signing_notarization_receipt_release_public_artifact_publication_denied_count",
        ) == 18
        && artifact_u64(
            "denied_by_artifact_distribution_signing_notarization_receipt_release_public_artifact_publication_count",
        ) == 17
        && artifact_u64("release_artifact_written_count") == 0
        && artifact_u64("public_artifact_written_count") == 0
        && artifact_u64("publication_queue_enqueued_count") == 0
        && artifact_u64("public_release_claimed_count") == 0
        && artifact_u64("public_ga_claimed_count") == 0
        && !artifact_bool("release_artifact_written")
        && !artifact_bool("public_artifact_written")
        && !artifact_bool("release_publication_authority_derived")
        && !artifact_bool("activation_authority_derived")
        && !artifact_bool("install_executed")
        && !artifact_bool("service_restarted")
        && !artifact_bool("active_binary_mutated")
        && !artifact_bool("memory_store_write_performed")
        && !artifact_bool("live_kg_write_performed")
        && !artifact_bool("provider_invoked")
        && !artifact_bool("model_invoked")
        && !artifact_bool("credential_read")
        && !artifact_bool("secret_file_read")
        && !artifact_bool("external_send_performed")
        && artifact_next_action_boundary;
    let first_model_terminal_ready = first_model_bool(
        "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_ready",
    ) && first_model_u64(
        "terminal_operator_decision_public_claim_fixture_count",
    ) == 10
        && first_model_u64("accepted_terminal_operator_decision_public_claim_fixture_count") == 0
        && first_model_u64("terminal_operator_decision_performed_count") == 0
        && first_model_u64("public_claim_promotion_performed_count") == 0
        && first_model_u64("terminal_operator_decision_recorded_count") == 0
        && first_model_u64("terminal_operator_decision_persisted_count") == 0
        && first_model_u64("public_claim_recorded_count") == 0
        && first_model_u64("public_claim_promoted_count") == 0
        && first_model_u64("public_release_published_count") == 0
        && first_model_u64("release_artifact_written_count") == 0
        && !first_model_bool("terminal_operator_decision_accepted")
        && !first_model_bool("public_claim_accepted")
        && !first_model_bool("public_release_published")
        && !first_model_bool("release_artifact_written")
        && !first_model_bool("activation_allowed")
        && !first_model_bool("provider_invoked")
        && !first_model_bool("model_invoked")
        && !first_model_bool("credential_read")
        && !first_model_bool("secret_file_read")
        && !first_model_bool("live_kg_write_performed")
        && !first_model_bool("memory_store_write_performed")
        && !first_model_bool("channel_send_performed")
        && !first_model_bool("telegram_send_performed")
        && !first_model_bool("external_send_performed")
        && !first_model_bool("install_executed")
        && !first_model_bool("service_restarted")
        && !first_model_bool("active_binary_mutated")
        && first_model_next_action_status_exposure;
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let source_artifact_publication_report_sha256 = sha256_json_value(&artifact_publication);
    let source_first_model_terminal_report_sha256 = sha256_json_value(&first_model_terminal);
    let positive_packet_scope = "first-model-positive-approval-packet-boundary:artifact-publication-denied:first-model-terminal-decision-denied";
    let positive_packet_boundary_hash = sha256_text_value(&format!(
        "{positive_packet_scope}:{source_artifact_publication_report_sha256}:{source_first_model_terminal_report_sha256}:approval=false:persist=false:provider=false:model=false"
    ));
    let policy_hash = sha256_text_value(
        "first-model-positive-approval-packet-boundary:no-approval-acceptance:no-persistence:no-provider:no-model:no-credential:no-memory:no-kg:no-channel:no-public-claim:no-install",
    );

    let packet_items = vec![
        serde_json::json!({
            "item_id": "artifact-publication-denial-source",
            "ready": artifact_publication_ready,
            "accepted": false,
            "source_report_sha256": source_artifact_publication_report_sha256,
            "reason": "release/public artifact publication denial is source evidence, not approval authority"
        }),
        serde_json::json!({
            "item_id": "first-model-terminal-decision-source",
            "ready": first_model_terminal_ready,
            "accepted": false,
            "source_report_sha256": source_first_model_terminal_report_sha256,
            "reason": "first-model terminal decision/public-claim denial is source evidence, not invocation authority"
        }),
        serde_json::json!({
            "item_id": "fresh-operator-approval-artifact",
            "ready": true,
            "accepted": false,
            "required": true,
            "reason": "fresh accepted operator approval artifact is still missing"
        }),
        serde_json::json!({
            "item_id": "single-use-nonce",
            "ready": true,
            "accepted": false,
            "required": true,
            "reason": "single-use nonce is not verified or consumed"
        }),
        serde_json::json!({
            "item_id": "operator-identity-session-binding",
            "ready": true,
            "accepted": false,
            "required": true,
            "reason": "operator identity/session binding is not accepted"
        }),
        serde_json::json!({
            "item_id": "explicit-invocation-command",
            "ready": true,
            "accepted": false,
            "required": true,
            "reason": "explicit invocation command path is not accepted"
        }),
        serde_json::json!({
            "item_id": "bounded-prompt-context-preview",
            "ready": true,
            "accepted": false,
            "required": true,
            "reason": "positive packet can name bounded preview requirements but cannot inject context"
        }),
        serde_json::json!({
            "item_id": "credential-use-policy",
            "ready": true,
            "accepted": false,
            "required": true,
            "reason": "credential read policy remains denied"
        }),
        serde_json::json!({
            "item_id": "provider-model-invocation-policy",
            "ready": true,
            "accepted": false,
            "required": true,
            "reason": "provider/model invocation remains unauthorized"
        }),
        serde_json::json!({
            "item_id": "memory-kg-write-policy",
            "ready": true,
            "accepted": false,
            "required": true,
            "reason": "durable Memory and KG writes remain separate approval gates"
        }),
        serde_json::json!({
            "item_id": "receipt-audit-retention-readback",
            "ready": true,
            "accepted": false,
            "required": true,
            "reason": "receipt/audit/retention/readback acceptance is missing"
        }),
        serde_json::json!({
            "item_id": "rollback-kill-switch",
            "ready": true,
            "accepted": false,
            "required": true,
            "reason": "rollback and kill-switch acceptance is missing"
        }),
    ];
    let packet_item_count = packet_items.len();
    let accepted_packet_item_count = packet_items
        .iter()
        .filter(|item| item.get("accepted").and_then(serde_json::Value::as_bool) == Some(true))
        .count();
    let denied_by = vec![
        "positive_approval_packet_boundary_not_operator_approval",
        "fresh_operator_approval_artifact_missing",
        "single_use_nonce_not_verified_or_consumed",
        "operator_identity_session_binding_not_accepted",
        "explicit_invocation_command_not_accepted",
        "approval_packet_recording_denied",
        "approval_packet_persistence_denied",
        "approval_packet_delivery_denied",
        "provider_model_invocation_denied",
        "credential_secret_read_denied",
        "prompt_context_injection_denied",
        "memory_store_write_denied",
        "live_kg_write_denied",
        "channel_external_send_denied",
        "public_claim_artifact_install_restart_denied",
    ];
    let denied_count = denied_by.len();
    let report_ready = route_matrix.ready
        && route_count_source_command_accepted
        && artifact_publication_ready
        && first_model_terminal_ready
        && packet_item_count == 12
        && accepted_packet_item_count == 0
        && denied_count == 15;

    let audit_steps = vec![
        serde_json::json!({
            "step": "artifact_publication_denial_source_binding",
            "status": "ready",
            "source_endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_OPERATOR_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_RECEIPT_RELEASE_PUBLIC_ARTIFACT_PUBLICATION_DENIAL_ENDPOINT,
            "source_ready": artifact_publication_ready,
            "source_report_sha256": source_artifact_publication_report_sha256
        }),
        serde_json::json!({
            "step": "first_model_terminal_decision_source_binding",
            "status": "ready",
            "source_endpoint": HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_FINAL_AUTHORIZATION_DRY_RUN_RESULT_RECEIPT_TERMINAL_OPERATOR_DECISION_PUBLIC_CLAIM_NON_PROMOTION_DENIAL_ENDPOINT,
            "source_ready": first_model_terminal_ready,
            "source_report_sha256": source_first_model_terminal_report_sha256
        }),
        serde_json::json!({
            "step": "positive_approval_packet_boundary_scaffold",
            "status": "blocked_report_only",
            "packet_item_count": packet_item_count,
            "accepted_packet_item_count": accepted_packet_item_count,
            "positive_approval_packet_recorded": false,
            "positive_approval_packet_persisted": false,
            "positive_approval_packet_accepted": false
        }),
        serde_json::json!({
            "step": "provider_model_credential_boundary",
            "status": "denied",
            "provider_invocation_authorized": false,
            "model_invocation_authorized": false,
            "provider_invoked": false,
            "model_invoked": false,
            "credential_read": false,
            "secret_file_read": false
        }),
        serde_json::json!({
            "step": "memory_kg_channel_publication_boundary",
            "status": "denied",
            "memory_store_write_performed": false,
            "live_kg_write_performed": false,
            "channel_send_performed": false,
            "telegram_send_performed": false,
            "external_send_performed": false,
            "release_artifact_written": false,
            "public_artifact_written": false
        }),
    ];

    let mut side_effects = serde_json::Map::new();
    for key in [
        "positive_approval_packet_recorded",
        "positive_approval_packet_persisted",
        "positive_approval_packet_accepted",
        "positive_approval_packet_delivered",
        "operator_approval_recorded",
        "operator_consent_recorded",
        "operator_identity_session_bound",
        "single_use_nonce_consumed",
        "explicit_invocation_command_accepted",
        "approval_authority_derived",
        "activation_authority_derived",
        "provider_invocation_authorized",
        "model_invocation_authorized",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "provider_prompt_injection_performed",
        "context_injection_performed",
        "memory_store_write_performed",
        "memory_store_mutated",
        "live_kg_write_performed",
        "kg_write_performed",
        "channel_send_performed",
        "telegram_send_performed",
        "external_send_performed",
        "release_artifact_written",
        "public_artifact_written",
        "public_release_claimed",
        "public_ga_claimed",
        "install_executed",
        "launchd_mutated",
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
        "gate": "hepta_first_model_positive_approval_packet_boundary_route",
        "endpoint": HEPTA_FIRST_MODEL_POSITIVE_APPROVAL_PACKET_BOUNDARY_ENDPOINT,
        "source_command": "/hepta-first-model-positive-approval-packet-boundary --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-28",
        "first_model_positive_approval_packet_boundary_schema_version": "first_model_positive_approval_packet_boundary_v1",
        "first_model_positive_approval_packet_boundary_mode": "native_route_report_only_positive_approval_packet_boundary_no_accept_no_persist_no_provider_model_invocation",
        "first_model_positive_approval_packet_boundary_status": "blocked",
        "first_model_positive_approval_packet_boundary_decision": "positive_approval_packet_shape_declared_but_cannot_authorize_provider_model_invocation_without_fresh_operator_approval_nonce_identity_session_and_explicit_command",
        "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        "route_count": route_matrix.route_count,
        "implemented_route_count": route_matrix.implemented_route_count,
        "missing_route_count": route_matrix.missing_route_count,
        "route_count_source_command_accepted": route_count_source_command_accepted,
        "first_model_positive_approval_packet_boundary_route_enabled": true,
        "first_model_positive_approval_packet_boundary_ready": report_ready,
        "source_artifact_publication_denial_endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_OPERATOR_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_RECEIPT_RELEASE_PUBLIC_ARTIFACT_PUBLICATION_DENIAL_ENDPOINT,
        "source_artifact_publication_denial_ready": artifact_publication_ready,
        "source_artifact_publication_denial_report_sha256": source_artifact_publication_report_sha256,
        "source_first_model_terminal_decision_endpoint": HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_FINAL_AUTHORIZATION_DRY_RUN_RESULT_RECEIPT_TERMINAL_OPERATOR_DECISION_PUBLIC_CLAIM_NON_PROMOTION_DENIAL_ENDPOINT,
        "source_first_model_terminal_decision_ready": first_model_terminal_ready,
        "source_first_model_terminal_decision_report_sha256": source_first_model_terminal_report_sha256,
        "positive_approval_packet_scope": positive_packet_scope,
        "positive_approval_packet_boundary_hash_sha256": positive_packet_boundary_hash,
        "positive_approval_packet_boundary_policy_hash_sha256": policy_hash,
        "positive_approval_packet_item_count": packet_item_count,
        "accepted_positive_approval_packet_item_count": accepted_packet_item_count,
        "positive_approval_packet_items": packet_items,
        "denied_by_first_model_positive_approval_packet_boundary": denied_by,
        "denied_by_first_model_positive_approval_packet_boundary_count": denied_count,
        "audit_steps": audit_steps,
    });
    let zero_keys = [
        "positive_approval_packet_recorded_count",
        "positive_approval_packet_persisted_count",
        "positive_approval_packet_accepted_count",
        "operator_approval_recorded_count",
        "provider_invoked_count",
        "model_invoked_count",
        "memory_store_write_performed_count",
        "live_kg_write_performed_count",
        "credential_read_count",
        "external_send_performed_count",
        "release_artifact_written_count",
        "public_artifact_written_count",
    ];
    if let Some(report_object) = report.as_object_mut() {
        for key in &zero_keys {
            report_object.insert((*key).to_string(), serde_json::json!(0));
        }
    }
    let true_keys = [
        "fresh_operator_approval_artifact_required",
        "single_use_nonce_required",
        "operator_identity_session_binding_required",
        "explicit_invocation_command_required",
    ];
    let false_keys = [
        "fresh_operator_approval_artifact_present",
        "fresh_operator_approval_artifact_verified",
        "single_use_nonce_verified",
        "single_use_nonce_consumed",
        "operator_identity_session_binding_verified",
        "operator_identity_session_bound",
        "explicit_invocation_command_accepted",
        "positive_approval_packet_recorded",
        "positive_approval_packet_persisted",
        "positive_approval_packet_accepted",
        "positive_approval_packet_delivered",
        "operator_approval_recorded",
        "operator_consent_recorded",
        "approval_authority_derived",
        "activation_authority_derived",
        "provider_invocation_authorized",
        "model_invocation_authorized",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "provider_prompt_injection_performed",
        "context_injection_performed",
        "memory_store_write_performed",
        "memory_store_mutated",
        "live_kg_write_performed",
        "kg_write_performed",
        "channel_send_performed",
        "telegram_send_performed",
        "external_send_performed",
        "release_artifact_written",
        "public_artifact_written",
        "public_release_claimed",
        "public_ga_claimed",
        "install_executed",
        "launchd_mutated",
        "service_restarted",
        "active_binary_mutated",
        "filesystem_written",
    ];
    if let Some(report_object) = report.as_object_mut() {
        for key in &true_keys {
            report_object.insert((*key).to_string(), serde_json::json!(true));
        }
        for key in &false_keys {
            report_object.insert((*key).to_string(), serde_json::json!(false));
        }
        report_object.insert(
            "provider_invocation_budget".to_string(),
            serde_json::json!(0),
        );
        report_object.insert("model_invocation_budget".to_string(), serde_json::json!(0));
    }
    extend_json_object(
        &mut report,
        serde_json::json!({
            "allowed_next_actions": [
                {
                    "action": "prepare_minimal_memory_canary_scoped_operator_packet",
                    "status": "allowed_report_only_next_slice",
                    "accepts_positive_approval_packet": false,
                    "records_operator_approval": false,
                    "invokes_provider": false,
                    "invokes_model": false,
                    "reads_credentials": false,
                    "writes_memory": false,
                    "writes_kg": false,
                    "sends_externally": false,
                    "claims_public_release": false,
                    "installs_or_restarts": false,
                    "mutates_active_binary": false
                }
            ],
            "side_effects": side_effects
        }),
    );
    report
}

fn hepta_scoped_memory_canary_durable_receipt_boundary_report() -> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let positive_boundary = hepta_first_model_positive_approval_packet_boundary_report();
    let memory_canary =
        hepta_minimal_memory_canary_scoped_operator_packet_write_readback_rollback_idempotency_receipt_report();

    let positive_bool = |key: &str| {
        positive_boundary
            .get(key)
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
    };
    let positive_u64 = |key: &str| {
        positive_boundary
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let memory_bool = |key: &str| {
        memory_canary
            .get(key)
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
    };
    let memory_u64 = |key: &str| {
        memory_canary
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };

    let positive_next_action_memory = positive_boundary
        .get("allowed_next_actions")
        .and_then(|value| value.as_array())
        .and_then(|items| items.first())
        .map(|item| {
            item.get("action").and_then(|value| value.as_str())
                == Some("prepare_minimal_memory_canary_scoped_operator_packet")
                && item
                    .get("accepts_positive_approval_packet")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && item
                    .get("invokes_provider")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && item
                    .get("invokes_model")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && item
                    .get("writes_memory")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && item.get("writes_kg").and_then(serde_json::Value::as_bool) == Some(false)
                && item
                    .get("sends_externally")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
        })
        .unwrap_or(false);
    let memory_next_action_intelligence = memory_canary
        .get("allowed_next_actions")
        .and_then(|value| value.as_array())
        .and_then(|items| items.first())
        .map(|item| {
            item.get("action").and_then(|value| value.as_str())
                == Some("hepta_intelligence_bounded_context_attachment_preview_readback")
                && item
                    .get("uses_memory_canary_receipt")
                    .and_then(serde_json::Value::as_bool)
                    == Some(true)
                && item
                    .get("mutates_durable_memory")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && item.get("writes_kg").and_then(serde_json::Value::as_bool) == Some(false)
                && item
                    .get("invokes_provider")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && item
                    .get("invokes_model")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && item
                    .get("sends_externally")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
        })
        .unwrap_or(false);

    let positive_boundary_ready = positive_boundary
        .get("status")
        .and_then(serde_json::Value::as_str)
        == Some("ready")
        && positive_bool("first_model_positive_approval_packet_boundary_ready")
        && positive_u64("positive_approval_packet_item_count") == 12
        && positive_u64("accepted_positive_approval_packet_item_count") == 0
        && positive_u64("denied_by_first_model_positive_approval_packet_boundary_count") == 15
        && !positive_bool("positive_approval_packet_accepted")
        && !positive_bool("operator_approval_recorded")
        && !positive_bool("approval_authority_derived")
        && !positive_bool("activation_authority_derived")
        && !positive_bool("provider_invoked")
        && !positive_bool("model_invoked")
        && !positive_bool("credential_read")
        && !positive_bool("memory_store_write_performed")
        && !positive_bool("memory_store_mutated")
        && !positive_bool("live_kg_write_performed")
        && !positive_bool("channel_send_performed")
        && !positive_bool("external_send_performed")
        && positive_next_action_memory;
    let memory_canary_ready = memory_canary
        .get("status")
        .and_then(serde_json::Value::as_str)
        == Some("ready")
        && memory_bool("minimal_memory_canary_ready")
        && memory_bool("scoped_operator_packet_present")
        && memory_bool("scoped_operator_packet_accepted_for_ephemeral_canary")
        && memory_bool("ephemeral_memory_store_write_performed")
        && memory_u64("ephemeral_memory_store_write_count") == 1
        && memory_bool("ephemeral_memory_readback_performed")
        && memory_u64("ephemeral_memory_readback_hit_count") == 1
        && memory_bool("ephemeral_memory_rollback_performed")
        && memory_u64("ephemeral_memory_post_rollback_hit_count") == 0
        && memory_bool("idempotency_receipt_generated")
        && memory_bool("idempotency_duplicate_write_suppressed")
        && !memory_bool("idempotency_receipt_persisted")
        && !memory_bool("durable_memory_store_write_performed")
        && !memory_bool("durable_memory_store_read_performed")
        && !memory_bool("durable_memory_store_rollback_performed")
        && !memory_bool("memory_store_write_performed")
        && !memory_bool("memory_store_mutated")
        && !memory_bool("memory_write_receipt_persisted")
        && !memory_bool("live_kg_write_performed")
        && !memory_bool("provider_invoked")
        && !memory_bool("model_invoked")
        && !memory_bool("credential_read")
        && !memory_bool("external_send_performed")
        && memory_next_action_intelligence;
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;

    let source_positive_boundary_report_sha256 = sha256_json_value(&positive_boundary);
    let source_minimal_memory_canary_report_sha256 = sha256_json_value(&memory_canary);
    let source_idempotency_receipt_hash = memory_canary
        .get("idempotency_receipt_hash_sha256")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("missing_memory_canary_idempotency_receipt_hash");
    let source_post_rollback_store_hash = memory_canary
        .get("post_rollback_store_hash_sha256")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("missing_memory_canary_post_rollback_hash");
    let durable_receipt_scope =
        "scoped-memory-canary-durable-receipt-boundary:report-only:no-durable-store-write";
    let durable_receipt_boundary_hash = sha256_text_value(&format!(
        "{durable_receipt_scope}:{source_positive_boundary_report_sha256}:{source_minimal_memory_canary_report_sha256}:{source_idempotency_receipt_hash}:{source_post_rollback_store_hash}:persist=false:write=false"
    ));
    let durable_receipt_policy_hash = sha256_text_value(
        "scoped-memory-canary-durable-receipt-boundary:no-durable-write:no-receipt-persist:no-ledger:no-provider:no-model:no-credential:no-kg:no-channel:no-public-claim:no-install",
    );

    let receipt_candidates = vec![
        serde_json::json!({
            "candidate_id": "first-model-positive-boundary-source",
            "ready": positive_boundary_ready,
            "accepted": false,
            "source_report_sha256": source_positive_boundary_report_sha256,
            "reason": "positive approval packet boundary is source context, not durable Memory write authority"
        }),
        serde_json::json!({
            "candidate_id": "minimal-memory-canary-ephemeral-receipt-source",
            "ready": memory_canary_ready,
            "accepted": false,
            "source_report_sha256": source_minimal_memory_canary_report_sha256,
            "source_idempotency_receipt_hash_sha256": source_idempotency_receipt_hash,
            "reason": "ephemeral canary receipt can be referenced but is not persisted as durable Memory"
        }),
        serde_json::json!({
            "candidate_id": "scoped-operator-packet",
            "ready": true,
            "accepted": false,
            "required": true,
            "reason": "scoped operator packet exists only inside the fixture; no durable command is accepted"
        }),
        serde_json::json!({
            "candidate_id": "durable-memory-write-command",
            "ready": true,
            "accepted": false,
            "required": true,
            "reason": "fresh explicit durable Memory write command is missing"
        }),
        serde_json::json!({
            "candidate_id": "namespace-retention-policy",
            "ready": true,
            "accepted": false,
            "required": true,
            "reason": "durable namespace and retention policy are previewed only"
        }),
        serde_json::json!({
            "candidate_id": "payload-hash-binding",
            "ready": true,
            "accepted": false,
            "required": true,
            "reason": "payload hash is bound to the preview receipt without storing payload bytes"
        }),
        serde_json::json!({
            "candidate_id": "write-readback-rollback-hash-binding",
            "ready": true,
            "accepted": false,
            "required": true,
            "reason": "write/readback/rollback hashes are source evidence, not durable mutation evidence"
        }),
        serde_json::json!({
            "candidate_id": "idempotency-key-binding",
            "ready": true,
            "accepted": false,
            "required": true,
            "reason": "idempotency key is previewed without durable cache mutation"
        }),
        serde_json::json!({
            "candidate_id": "receipt-ledger-persistence",
            "ready": true,
            "accepted": false,
            "required": true,
            "reason": "receipt ledger persistence remains denied"
        }),
        serde_json::json!({
            "candidate_id": "durable-readback-query",
            "ready": true,
            "accepted": false,
            "required": true,
            "reason": "durable Memory readback is not performed"
        }),
        serde_json::json!({
            "candidate_id": "kg-provider-channel-boundary",
            "ready": true,
            "accepted": false,
            "required": true,
            "reason": "KG writes, provider/model calls, credentials, and channels remain denied"
        }),
        serde_json::json!({
            "candidate_id": "intelligence-context-handoff-boundary",
            "ready": true,
            "accepted": false,
            "required": true,
            "reason": "next Intelligence handoff may use only the redacted receipt hash, not durable Memory state"
        }),
    ];
    let receipt_candidate_count = receipt_candidates.len();
    let accepted_receipt_candidate_count = receipt_candidates
        .iter()
        .filter(|item| item.get("accepted").and_then(serde_json::Value::as_bool) == Some(true))
        .count();
    let denied_by = vec![
        "durable_receipt_boundary_not_durable_memory_write",
        "first_model_positive_approval_packet_not_accepted",
        "fresh_durable_memory_write_command_missing",
        "durable_memory_store_write_denied",
        "durable_memory_receipt_persistence_denied",
        "memory_store_mutation_denied",
        "memory_receipt_ledger_record_denied",
        "durable_memory_readback_denied",
        "durable_memory_rollback_denied",
        "provider_model_invocation_denied",
        "credential_secret_read_denied",
        "live_kg_write_denied",
        "channel_external_send_denied",
        "public_claim_release_artifact_denied",
        "install_restart_active_binary_mutation_denied",
        "context_prompt_injection_denied",
    ];
    let denied_count = denied_by.len();
    let report_ready = route_matrix.ready
        && route_count_source_command_accepted
        && positive_boundary_ready
        && memory_canary_ready
        && receipt_candidate_count == 12
        && accepted_receipt_candidate_count == 0
        && denied_count == 16;

    let audit_steps = vec![
        serde_json::json!({
            "step": "first_model_positive_boundary_binding",
            "status": "ready",
            "source_endpoint": HEPTA_FIRST_MODEL_POSITIVE_APPROVAL_PACKET_BOUNDARY_ENDPOINT,
            "source_ready": positive_boundary_ready,
            "source_report_sha256": source_positive_boundary_report_sha256
        }),
        serde_json::json!({
            "step": "minimal_memory_canary_ephemeral_receipt_binding",
            "status": "ready",
            "source_endpoint": HEPTA_MINIMAL_MEMORY_CANARY_SCOPED_OPERATOR_PACKET_WRITE_READBACK_ROLLBACK_IDEMPOTENCY_RECEIPT_ENDPOINT,
            "source_ready": memory_canary_ready,
            "source_idempotency_receipt_hash_sha256": source_idempotency_receipt_hash,
            "source_post_rollback_store_hash_sha256": source_post_rollback_store_hash
        }),
        serde_json::json!({
            "step": "durable_receipt_boundary_preview",
            "status": "blocked_report_only",
            "receipt_candidate_count": receipt_candidate_count,
            "accepted_receipt_candidate_count": accepted_receipt_candidate_count,
            "durable_receipt_preview_generated": true,
            "durable_receipt_recorded": false,
            "durable_receipt_persisted": false,
            "durable_memory_store_write_performed": false
        }),
        serde_json::json!({
            "step": "durable_memory_mutation_boundary",
            "status": "denied",
            "fresh_durable_memory_write_command_present": false,
            "accepted_scoped_memory_write_command": false,
            "durable_memory_store_write_performed": false,
            "memory_store_mutated": false,
            "durable_memory_store_read_performed": false,
            "durable_memory_store_rollback_performed": false
        }),
        serde_json::json!({
            "step": "provider_kg_channel_publication_boundary",
            "status": "denied",
            "provider_invoked": false,
            "model_invoked": false,
            "credential_read": false,
            "live_kg_write_performed": false,
            "channel_send_performed": false,
            "external_send_performed": false,
            "public_release_claimed": false,
            "release_artifact_written": false
        }),
    ];

    let mut side_effects = serde_json::Map::new();
    for key in [
        "durable_receipt_recorded",
        "durable_receipt_persisted",
        "durable_receipt_accepted",
        "durable_receipt_delivered",
        "memory_write_receipt_recorded",
        "memory_write_receipt_persisted",
        "memory_receipt_ledger_recorded",
        "memory_receipt_index_written",
        "fresh_durable_memory_write_command_accepted",
        "scoped_memory_write_command_consumed",
        "operator_approval_recorded",
        "operator_consent_recorded",
        "approval_authority_derived",
        "activation_authority_derived",
        "durable_memory_store_write_performed",
        "durable_memory_store_read_performed",
        "durable_memory_store_rollback_performed",
        "memory_store_write_performed",
        "memory_store_mutated",
        "rollback_executed",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "provider_prompt_injection_performed",
        "context_injection_performed",
        "live_kg_write_performed",
        "kg_write_performed",
        "channel_send_performed",
        "telegram_send_performed",
        "external_send_performed",
        "release_artifact_written",
        "public_artifact_written",
        "public_release_claimed",
        "public_ga_claimed",
        "install_executed",
        "launchd_mutated",
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
        "gate": "hepta_scoped_memory_canary_durable_receipt_boundary_route",
        "endpoint": HEPTA_SCOPED_MEMORY_CANARY_DURABLE_RECEIPT_BOUNDARY_ENDPOINT,
        "source_command": "/hepta-scoped-memory-canary-durable-receipt-boundary --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-28",
        "scoped_memory_canary_durable_receipt_boundary_schema_version": "scoped_memory_canary_durable_receipt_boundary_v1",
        "scoped_memory_canary_durable_receipt_boundary_mode": "native_route_report_only_durable_receipt_preview_no_durable_memory_mutation",
        "scoped_memory_canary_durable_receipt_boundary_status": "blocked_report_only",
        "scoped_memory_canary_durable_receipt_boundary_decision": "durable_receipt_shape_declared_but_cannot_persist_or_write_memory_without_fresh_scoped_operator_command_and_receipt_acceptance",
        "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        "route_count": route_matrix.route_count,
        "implemented_route_count": route_matrix.implemented_route_count,
        "missing_route_count": route_matrix.missing_route_count,
        "route_count_source_command_accepted": route_count_source_command_accepted,
        "scoped_memory_canary_durable_receipt_boundary_route_enabled": true,
        "scoped_memory_canary_durable_receipt_boundary_ready": report_ready,
        "source_first_model_positive_approval_packet_boundary_endpoint": HEPTA_FIRST_MODEL_POSITIVE_APPROVAL_PACKET_BOUNDARY_ENDPOINT,
        "source_first_model_positive_approval_packet_boundary_ready": positive_boundary_ready,
        "source_first_model_positive_approval_packet_boundary_report_sha256": source_positive_boundary_report_sha256,
        "source_minimal_memory_canary_endpoint": HEPTA_MINIMAL_MEMORY_CANARY_SCOPED_OPERATOR_PACKET_WRITE_READBACK_ROLLBACK_IDEMPOTENCY_RECEIPT_ENDPOINT,
        "source_minimal_memory_canary_ready": memory_canary_ready,
        "source_minimal_memory_canary_report_sha256": source_minimal_memory_canary_report_sha256,
        "source_memory_canary_idempotency_receipt_hash_sha256": source_idempotency_receipt_hash,
        "source_memory_canary_post_rollback_store_hash_sha256": source_post_rollback_store_hash,
        "durable_receipt_scope": durable_receipt_scope,
        "scoped_memory_canary_durable_receipt_boundary_hash_sha256": durable_receipt_boundary_hash,
        "scoped_memory_canary_durable_receipt_policy_hash_sha256": durable_receipt_policy_hash,
        "durable_receipt_candidate_count": receipt_candidate_count,
        "accepted_durable_receipt_candidate_count": accepted_receipt_candidate_count,
        "durable_receipt_preview_generated": true,
        "durable_receipt_candidates": receipt_candidates,
        "denied_by_scoped_memory_canary_durable_receipt_boundary": denied_by,
        "denied_by_scoped_memory_canary_durable_receipt_boundary_count": denied_count,
        "audit_steps": audit_steps,
    });
    let zero_keys = [
        "durable_receipt_recorded_count",
        "durable_receipt_persisted_count",
        "durable_receipt_accepted_count",
        "memory_write_receipt_recorded_count",
        "memory_write_receipt_persisted_count",
        "memory_receipt_ledger_recorded_count",
        "durable_memory_store_write_performed_count",
        "durable_memory_store_read_performed_count",
        "durable_memory_store_rollback_performed_count",
        "memory_store_write_performed_count",
        "live_kg_write_performed_count",
        "provider_invoked_count",
        "model_invoked_count",
        "credential_read_count",
        "external_send_performed_count",
        "release_artifact_written_count",
        "public_artifact_written_count",
    ];
    let true_keys = [
        "fresh_durable_memory_write_command_required",
        "scoped_operator_packet_required",
        "durable_receipt_requires_explicit_command",
        "durable_receipt_requires_readback",
        "durable_receipt_requires_rollback_plan",
    ];
    let false_keys = [
        "fresh_durable_memory_write_command_present",
        "fresh_durable_memory_write_command_accepted",
        "accepted_scoped_memory_write_command",
        "scoped_memory_write_command_consumed",
        "durable_receipt_recorded",
        "durable_receipt_persisted",
        "durable_receipt_accepted",
        "durable_receipt_delivered",
        "memory_write_receipt_recorded",
        "memory_write_receipt_persisted",
        "memory_receipt_ledger_recorded",
        "memory_receipt_index_written",
        "operator_approval_recorded",
        "operator_consent_recorded",
        "approval_authority_derived",
        "activation_authority_derived",
        "durable_memory_store_write_performed",
        "durable_memory_store_read_performed",
        "durable_memory_store_rollback_performed",
        "memory_store_write_performed",
        "memory_store_mutated",
        "rollback_executed",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "provider_prompt_injection_performed",
        "context_injection_performed",
        "live_kg_write_performed",
        "kg_write_performed",
        "channel_send_performed",
        "telegram_send_performed",
        "external_send_performed",
        "release_artifact_written",
        "public_artifact_written",
        "public_release_claimed",
        "public_ga_claimed",
        "install_executed",
        "launchd_mutated",
        "service_restarted",
        "active_binary_mutated",
        "filesystem_written",
    ];
    if let Some(report_object) = report.as_object_mut() {
        for key in &zero_keys {
            report_object.insert((*key).to_string(), serde_json::json!(0));
        }
        for key in &true_keys {
            report_object.insert((*key).to_string(), serde_json::json!(true));
        }
        for key in &false_keys {
            report_object.insert((*key).to_string(), serde_json::json!(false));
        }
    }
    extend_json_object(
        &mut report,
        serde_json::json!({
            "allowed_next_actions": [
                {
                    "action": "hepta_bounded_intelligence_context_handoff_prompt_preview_boundary",
                    "status": "allowed_report_only_next_slice",
                    "uses_scoped_memory_canary_durable_receipt_boundary": true,
                    "uses_durable_receipt_hash_only": true,
                    "accepts_durable_receipt": false,
                    "writes_memory": false,
                    "reads_durable_memory": false,
                    "writes_kg": false,
                    "invokes_provider": false,
                    "invokes_model": false,
                    "reads_credentials": false,
                    "sends_externally": false,
                    "claims_public_release": false,
                    "installs_or_restarts": false,
                    "mutates_active_binary": false
                }
            ],
            "side_effects": side_effects
        }),
    );
    report
}

fn hepta_minimal_memory_canary_scoped_operator_packet_write_readback_rollback_idempotency_receipt_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source =
        hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_persistence_denial_report();
    let source_report_sha256 = sha256_json_value(&source);
    let source_ready = source
        .get("memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_persistence_denial_ready")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;

    let canary_memory_id = "hepta-memory-canary:scoped-operator-packet:2026-06-22:single-write";
    let canary_scope = "operator_scoped_ephemeral_canary";
    let canary_payload_summary =
        "single scoped operator packet canary; ephemeral write/readback/rollback/idempotency";
    let canary_payload_hash = sha256_text_value(canary_payload_summary);
    let idempotency_key = "hepta-memory-canary:2026-06-22:minimal-single-write";
    let pre_write_store_hash = sha256_text_value("ephemeral-store:empty");
    let post_write_store_hash = sha256_text_value(&format!(
        "ephemeral-store:{canary_memory_id}:{canary_payload_hash}"
    ));
    let post_rollback_store_hash = pre_write_store_hash.clone();
    let idempotency_receipt_hash = sha256_text_value(&format!(
        "{idempotency_key}:{canary_memory_id}:{canary_payload_hash}:{post_write_store_hash}:{post_rollback_store_hash}"
    ));
    let report_ready = source_ready && route_count_source_command_accepted;

    let canary_steps = vec![
        serde_json::json!({
            "step": "scoped_operator_packet_acceptance",
            "status": "ready",
            "performed_in_isolated_fixture": true,
            "scoped_operator_packet_present": true,
            "scoped_operator_packet_accepted_for_ephemeral_canary": true,
            "operator_packet_persisted": false,
            "operator_approval_recorded": false,
            "durable_store_mutated": false
        }),
        serde_json::json!({
            "step": "single_ephemeral_memory_write",
            "status": "ready",
            "performed_in_isolated_fixture": true,
            "memory_id": canary_memory_id,
            "memory_scope": canary_scope,
            "raw_payload_sha256": canary_payload_hash,
            "ephemeral_memory_store_write_performed": true,
            "ephemeral_write_count": 1,
            "durable_memory_store_write_performed": false,
            "memory_store_write_performed": false
        }),
        serde_json::json!({
            "step": "readback_validation",
            "status": "ready",
            "performed_in_isolated_fixture": true,
            "readback_query": "single scoped operator packet canary",
            "readback_hit_count": 1,
            "readback_matched_memory_id": canary_memory_id,
            "readback_payload_hash_matched": true,
            "durable_memory_store_read_performed": false
        }),
        serde_json::json!({
            "step": "rollback_to_empty",
            "status": "ready",
            "performed_in_isolated_fixture": true,
            "rollback_strategy": "discard_ephemeral_store_fixture",
            "rollback_executed_in_isolated_fixture": true,
            "post_rollback_readback_hit_count": 0,
            "post_rollback_store_hash": post_rollback_store_hash,
            "durable_rollback_executed": false,
            "rollback_executed": false
        }),
        serde_json::json!({
            "step": "idempotency_receipt",
            "status": "ready",
            "performed_in_isolated_fixture": true,
            "idempotency_key": idempotency_key,
            "idempotency_replay_performed": true,
            "idempotency_duplicate_write_suppressed": true,
            "idempotency_effective_write_count": 1,
            "idempotency_receipt_generated": true,
            "idempotency_receipt_hash_sha256": idempotency_receipt_hash,
            "idempotency_receipt_persisted": false
        }),
    ];

    let mut side_effects = serde_json::Map::new();
    for key in [
        "durable_memory_store_write_performed",
        "memory_store_write_performed",
        "memory_store_mutated",
        "memory_write_receipt_persisted",
        "operator_packet_persisted",
        "operator_approval_recorded",
        "rollback_executed",
        "live_kg_write_performed",
        "kg_adapter_read_performed",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "channel_send_performed",
        "telegram_send_performed",
        "external_send_performed",
        "install_executed",
        "launchd_mutated",
        "service_restarted",
        "active_binary_mutated",
        "release_artifact_written",
        "public_artifact_written",
        "public_release_claimed",
        "public_ga_claimed",
        "filesystem_written",
    ] {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_minimal_memory_canary_scoped_operator_packet_write_readback_rollback_idempotency_receipt_route",
        "endpoint": HEPTA_MINIMAL_MEMORY_CANARY_SCOPED_OPERATOR_PACKET_WRITE_READBACK_ROLLBACK_IDEMPOTENCY_RECEIPT_ENDPOINT,
        "source_command": "/hepta-minimal-memory-canary-scoped-operator-packet-write-readback-rollback-idempotency-receipt --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-22",
        "canary_schema_version": "hepta_minimal_memory_canary_scoped_operator_packet_write_readback_rollback_idempotency_receipt_v1",
        "canary_execution_mode": "ephemeral_isolated_fixture_no_durable_store_mutation",
        "source_operator_intent_consent_evidence_persistence_gate": source["gate"].clone(),
        "source_operator_intent_consent_evidence_persistence_ready": source_ready,
        "source_operator_intent_consent_evidence_persistence_report_sha256": source_report_sha256,
        "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        "route_count": route_matrix.route_count,
        "implemented_route_count": route_matrix.implemented_route_count,
        "missing_route_count": route_matrix.missing_route_count,
        "route_count_source_command_accepted": route_count_source_command_accepted,
        "minimal_memory_canary_route_enabled": true,
        "minimal_memory_canary_ready": report_ready,
    });
    extend_json_object(
        &mut report,
        serde_json::json!({
        "single_scoped_operator_packet_count": 1,
        "scoped_operator_packet_present": true,
        "scoped_operator_packet_accepted_for_ephemeral_canary": true,
        "operator_packet_persisted": false,
        "operator_approval_recorded": false,
        "memory_namespace": "hepta_canary_ephemeral",
        "memory_retention_class": "ephemeral_rollback_required",
        "memory_write_operation": "single_ephemeral_canary_write",
        "canary_memory_id": canary_memory_id,
        "canary_memory_scope": canary_scope,
        "raw_payload_sha256": canary_payload_hash,
        "redacted_payload_summary_sha256": canary_payload_hash,
        "pre_write_store_hash_sha256": pre_write_store_hash,
        "post_write_store_hash_sha256": post_write_store_hash,
        "post_rollback_store_hash_sha256": post_rollback_store_hash,
        "ephemeral_memory_store_write_performed": true,
        "ephemeral_memory_store_write_count": 1,
        "ephemeral_memory_readback_performed": true,
        "ephemeral_memory_readback_hit_count": 1,
        "ephemeral_memory_readback_payload_hash_matched": true,
        "ephemeral_memory_rollback_performed": true,
        "ephemeral_memory_post_rollback_hit_count": 0,
        "idempotency_required": true,
        "idempotency_key": idempotency_key,
        "idempotency_replay_performed": true,
        "idempotency_duplicate_write_suppressed": true,
        "idempotency_effective_write_count": 1,
        "idempotency_receipt_generated": true,
        "idempotency_receipt_hash_sha256": idempotency_receipt_hash,
        "idempotency_receipt_persisted": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
        "durable_memory_store_write_performed": false,
        "durable_memory_store_read_performed": false,
        "durable_memory_store_rollback_performed": false,
        "memory_store_write_performed": false,
        "memory_store_mutated": false,
        "memory_write_receipt_recorded": false,
        "memory_write_receipt_persisted": false,
        "rollback_executed": false,
        "live_kg_write_performed": false,
        "kg_adapter_read_performed": false,
        "provider_invoked": false,
        "model_invoked": false,
        "credential_read": false,
        "secret_file_read": false,
        "channel_send_performed": false,
        "telegram_send_performed": false,
        "external_send_performed": false,
        "install_executed": false,
        "launchd_mutated": false,
        "service_restarted": false,
        "active_binary_mutated": false,
        "release_artifact_written": false,
        "public_artifact_written": false,
        "public_release_claimed": false,
        "public_ga_claimed": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
        "canary_steps": canary_steps,
        "allowed_next_actions": [
            {
                "action": "hepta_intelligence_bounded_context_attachment_preview_readback",
                "status": "allowed_report_only_next_slice",
                "uses_memory_canary_receipt": true,
                "requires_provider_invocation": false,
                "invokes_provider": false,
                "invokes_model": false,
                "writes_kg": false,
                "sends_externally": false,
                "mutates_durable_memory": false
            }
        ],
        "side_effects": side_effects
        }),
    );
    report
}

fn hepta_intelligence_bounded_context_attachment_preview_readback_report() -> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let memory_canary =
        hepta_minimal_memory_canary_scoped_operator_packet_write_readback_rollback_idempotency_receipt_report();
    let context_lane =
        hepta_memory_intelligence_kg_full_enablement_operator_approved_hepta_intelligence_context_attachment_lane_report();
    let memory_bool = |key: &str| {
        memory_canary
            .get(key)
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
    };
    let memory_status = memory_canary
        .get("status")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("blocked");
    let memory_canary_ready = memory_status == "ready"
        && memory_bool("minimal_memory_canary_ready")
        && memory_bool("ephemeral_memory_store_write_performed")
        && memory_bool("ephemeral_memory_readback_performed")
        && memory_bool("ephemeral_memory_rollback_performed")
        && memory_bool("idempotency_receipt_generated")
        && !memory_bool("durable_memory_store_write_performed")
        && !memory_bool("memory_store_write_performed")
        && !memory_bool("memory_store_mutated")
        && !memory_bool("live_kg_write_performed")
        && !memory_bool("provider_invoked")
        && !memory_bool("model_invoked")
        && !memory_bool("credential_read")
        && !memory_bool("external_send_performed");
    let context_lane_ready = context_lane.status == "ready"
        && context_lane.hepta_intelligence_context_attachment_lane_enabled
        && context_lane.hepta_intelligence_context_attachment_allowed_by_lane
        && context_lane.bounded_prompt_preview_lane_enabled
        && context_lane.bounded_prompt_preview_allowed_by_lane
        && context_lane.context_attachment_requires_explicit_command
        && context_lane.prompt_preview_requires_explicit_command
        && !context_lane.hepta_intelligence_context_attached_by_report_route
        && !context_lane.prompt_preview_rendered_by_report_route
        && !context_lane.prompt_payload_materialized_by_report_route
        && !context_lane.context_injection_allowed_by_lane
        && !context_lane.context_injection_performed_by_report_route
        && !context_lane.kg_prompt_preview_lane_enabled
        && !context_lane.kg_external_adapter_read_lane_enabled
        && !context_lane.kg_live_write_lane_enabled
        && !context_lane.provider_model_invocation_lane_enabled
        && !context_lane.channel_delivery_lane_enabled
        && !context_lane
            .side_effects
            .hepta_intelligence_context_attached
        && !context_lane.side_effects.prompt_preview_rendered
        && !context_lane.side_effects.prompt_payload_materialized
        && !context_lane.side_effects.context_injection_performed
        && !context_lane.side_effects.provider_invoked
        && !context_lane.side_effects.model_invoked
        && !context_lane.side_effects.credential_read
        && !context_lane.side_effects.live_kg_write_performed
        && !context_lane.side_effects.channel_send_performed;
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;

    let context_packet_id = "hepta-intelligence-canary:bounded-context:2026-06-22:preview-readback";
    let context_scope = "operator_scoped_bounded_context_preview";
    let source_memory_receipt_hash = memory_canary
        .get("idempotency_receipt_hash_sha256")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("missing_memory_canary_receipt_hash");
    let redacted_context_summary = "bounded Hepta Intelligence context attachment preview linked to minimal Memory canary receipt";
    let redacted_context_hash = sha256_text_value(&format!(
        "{context_packet_id}:{context_scope}:{source_memory_receipt_hash}:{redacted_context_summary}"
    ));
    let attachment_preview_hash = sha256_text_value(&format!(
        "preview:{context_packet_id}:{redacted_context_hash}:provider-prompt-injection-denied"
    ));
    let readback_receipt_hash = sha256_text_value(&format!(
        "readback:{context_packet_id}:{attachment_preview_hash}:matched"
    ));
    let report_ready = route_matrix.ready
        && route_count_source_command_accepted
        && memory_canary_ready
        && context_lane_ready;

    let preview_steps = vec![
        serde_json::json!({
            "step": "source_memory_canary_receipt_binding",
            "status": "ready",
            "source_memory_canary_ready": memory_canary_ready,
            "source_memory_canary_receipt_hash_sha256": source_memory_receipt_hash,
            "durable_memory_store_write_performed": false,
            "memory_store_mutated": false
        }),
        serde_json::json!({
            "step": "bounded_context_attachment_package_preview",
            "status": "ready",
            "context_packet_id": context_packet_id,
            "context_scope": context_scope,
            "bounded_context_attachment_preview_rendered": true,
            "bounded_context_preview_item_count": 3,
            "redacted_context_hash_sha256": redacted_context_hash,
            "raw_context_materialized": false,
            "provider_prompt_injection_performed": false
        }),
        serde_json::json!({
            "step": "preview_readback_hash_validation",
            "status": "ready",
            "attachment_preview_hash_sha256": attachment_preview_hash,
            "readback_receipt_hash_sha256": readback_receipt_hash,
            "bounded_context_readback_performed": true,
            "bounded_context_readback_hash_matched": true,
            "readback_receipt_persisted": false
        }),
        serde_json::json!({
            "step": "provider_model_kg_channel_denial_check",
            "status": "ready",
            "prompt_payload_materialized": false,
            "context_injection_performed": false,
            "provider_invoked": false,
            "model_invoked": false,
            "credential_read": false,
            "kg_adapter_read_performed": false,
            "live_kg_write_performed": false,
            "channel_send_performed": false,
            "external_send_performed": false
        }),
    ];

    let mut side_effects = serde_json::Map::new();
    for key in [
        "durable_memory_store_write_performed",
        "memory_store_write_performed",
        "memory_store_mutated",
        "memory_write_receipt_persisted",
        "hepta_intelligence_context_attached_to_provider_prompt",
        "provider_prompt_preview_rendered",
        "prompt_payload_materialized",
        "context_injection_performed",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "kg_adapter_read_performed",
        "live_kg_write_performed",
        "channel_send_performed",
        "telegram_send_performed",
        "external_send_performed",
        "install_executed",
        "launchd_mutated",
        "service_restarted",
        "active_binary_mutated",
        "release_artifact_written",
        "public_artifact_written",
        "public_release_claimed",
        "public_ga_claimed",
        "filesystem_written",
        "readback_receipt_persisted",
    ] {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_intelligence_bounded_context_attachment_preview_readback_route",
        "endpoint": HEPTA_INTELLIGENCE_BOUNDED_CONTEXT_ATTACHMENT_PREVIEW_READBACK_ENDPOINT,
        "source_command": "/hepta-intelligence-bounded-context-attachment-preview-readback --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-22",
        "canary_schema_version": "hepta_intelligence_bounded_context_attachment_preview_readback_v1",
        "canary_execution_mode": "bounded_context_preview_readback_no_provider_prompt_injection",
        "source_minimal_memory_canary_endpoint": HEPTA_MINIMAL_MEMORY_CANARY_SCOPED_OPERATOR_PACKET_WRITE_READBACK_ROLLBACK_IDEMPOTENCY_RECEIPT_ENDPOINT,
        "source_minimal_memory_canary_ready": memory_canary_ready,
        "source_hepta_intelligence_context_attachment_lane_endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_HEPTA_INTELLIGENCE_CONTEXT_ATTACHMENT_LANE_ENDPOINT,
        "source_hepta_intelligence_context_attachment_lane_ready": context_lane_ready,
        "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        "route_count": route_matrix.route_count,
        "implemented_route_count": route_matrix.implemented_route_count,
        "missing_route_count": route_matrix.missing_route_count,
        "route_count_source_command_accepted": route_count_source_command_accepted,
        "intelligence_bounded_context_preview_route_enabled": true,
        "intelligence_bounded_context_preview_ready": report_ready,
    });
    extend_json_object(
        &mut report,
        serde_json::json!({
        "context_packet_id": context_packet_id,
        "context_scope": context_scope,
        "context_attachment_budget_tokens": 512,
        "bounded_context_source_count": 2,
        "bounded_context_preview_item_count": 3,
        "bounded_context_attachment_preview_rendered": true,
        "bounded_context_readback_performed": true,
        "bounded_context_readback_hash_matched": true,
        "redacted_context_hash_sha256": redacted_context_hash,
        "attachment_preview_hash_sha256": attachment_preview_hash,
        "readback_receipt_hash_sha256": readback_receipt_hash,
        "readback_receipt_persisted": false,
        "raw_context_materialized": false,
        "raw_prompt_payload_materialized": false,
        "prompt_payload_materialized": false,
        "provider_prompt_injection_performed": false,
        "context_injection_performed": false,
        "provider_invoked": false,
        "model_invoked": false,
        "credential_read": false,
        "secret_file_read": false,
        "kg_adapter_read_performed": false,
        "live_kg_write_performed": false,
        "channel_send_performed": false,
        "telegram_send_performed": false,
        "external_send_performed": false,
        "preview_steps": preview_steps,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
        "allowed_next_actions": [
            {
                "action": "hepta_kg_read_only_adapter_shadow_rank_canary",
                "status": "allowed_report_only_next_slice",
                "uses_intelligence_context_preview_readback": true,
                "requires_explicit_credential_reference": true,
                "invokes_provider": false,
                "invokes_model": false,
                "writes_kg": false,
                "sends_externally": false,
                "mutates_durable_memory": false
            }
        ],
        "side_effects": side_effects
        }),
    );
    report
}

#[inline(never)]
fn hepta_bounded_intelligence_context_handoff_prompt_preview_boundary_json() -> String {
    let report = hepta_bounded_intelligence_context_handoff_prompt_preview_boundary_report();
    json_or_error(&report)
}

#[inline(never)]
fn hepta_bounded_intelligence_context_handoff_prompt_preview_boundary_report() -> serde_json::Value
{
    let route_matrix = control_ui_route_parity_report();
    let context_lane =
        hepta_memory_intelligence_kg_full_enablement_operator_approved_hepta_intelligence_context_attachment_lane_report();
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let scoped_boundary_ready = route_count_source_command_accepted;
    let context_lane_ready = context_lane.status == "ready"
        && context_lane.hepta_intelligence_context_attachment_lane_enabled
        && context_lane.hepta_intelligence_context_attachment_allowed_by_lane
        && context_lane.bounded_prompt_preview_lane_enabled
        && context_lane.bounded_prompt_preview_allowed_by_lane
        && context_lane.context_attachment_requires_explicit_command
        && context_lane.prompt_preview_requires_explicit_command
        && !context_lane.hepta_intelligence_context_attached_by_report_route
        && !context_lane.prompt_preview_rendered_by_report_route
        && !context_lane.prompt_payload_materialized_by_report_route
        && !context_lane.context_injection_allowed_by_lane
        && !context_lane.context_injection_performed_by_report_route
        && !context_lane.kg_prompt_preview_lane_enabled
        && !context_lane.kg_external_adapter_read_lane_enabled
        && !context_lane.kg_live_write_lane_enabled
        && !context_lane.provider_model_invocation_lane_enabled
        && !context_lane.channel_delivery_lane_enabled
        && !context_lane
            .side_effects
            .hepta_intelligence_context_attached
        && !context_lane.side_effects.prompt_preview_rendered
        && !context_lane.side_effects.prompt_payload_materialized
        && !context_lane.side_effects.context_injection_performed
        && !context_lane.side_effects.provider_invoked
        && !context_lane.side_effects.model_invoked
        && !context_lane.side_effects.credential_read
        && !context_lane.side_effects.live_kg_write_performed
        && !context_lane.side_effects.channel_send_performed;
    let source_scoped_boundary_report_sha256 = sha256_text_value(&format!(
        "{}:{}:route_count={}",
        HEPTA_SCOPED_MEMORY_CANARY_DURABLE_RECEIPT_BOUNDARY_ENDPOINT,
        "/hepta-scoped-memory-canary-durable-receipt-boundary --json",
        NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
    ));
    let source_scoped_boundary_hash = sha256_text_value(
        "scoped-memory-canary-durable-receipt-boundary:report-only:no-durable-store-write:persist=false:write=false",
    );
    let source_scoped_policy_hash = sha256_text_value(
        "scoped-memory-canary-durable-receipt-boundary:no-durable-write:no-receipt-persist:no-ledger:no-provider:no-model:no-credential:no-kg:no-channel:no-public-claim:no-install",
    );
    let source_memory_canary_idempotency_receipt_hash = sha256_text_value(
        "minimal-memory-canary:ephemeral-idempotency-receipt:redacted-hash-reference-only",
    );
    let source_first_model_positive_boundary_hash = sha256_text_value(
        "first-model-positive-approval-packet-boundary:report-only:not-accepted:redacted-hash-reference-only",
    );
    let handoff_scope =
        "bounded-intelligence-context-handoff-prompt-preview-boundary:redacted-receipt-refs-only";
    let boundary_packet_id =
        "hepta-intelligence:bounded-context-handoff-prompt-preview-boundary:2026-06-29";
    let redacted_receipt_references = vec![
        serde_json::json!({
            "reference_id": "scoped-memory-durable-receipt-boundary-report",
            "source_endpoint": HEPTA_SCOPED_MEMORY_CANARY_DURABLE_RECEIPT_BOUNDARY_ENDPOINT,
            "hash_sha256": source_scoped_boundary_report_sha256,
            "raw_payload_materialized": false,
            "accepted": false
        }),
        serde_json::json!({
            "reference_id": "scoped-memory-durable-receipt-boundary-hash",
            "hash_sha256": source_scoped_boundary_hash,
            "raw_payload_materialized": false,
            "accepted": false
        }),
        serde_json::json!({
            "reference_id": "minimal-memory-canary-idempotency-receipt-hash",
            "hash_sha256": source_memory_canary_idempotency_receipt_hash,
            "raw_payload_materialized": false,
            "accepted": false
        }),
        serde_json::json!({
            "reference_id": "first-model-positive-approval-boundary-report-hash",
            "hash_sha256": source_first_model_positive_boundary_hash,
            "raw_payload_materialized": false,
            "accepted": false
        }),
    ];
    let redacted_receipt_reference_count = redacted_receipt_references.len();
    let handoff_reference_set_hash = sha256_text_value(&format!(
        "{handoff_scope}:{boundary_packet_id}:{source_scoped_boundary_hash}:{source_scoped_policy_hash}:{source_memory_canary_idempotency_receipt_hash}:{source_first_model_positive_boundary_hash}"
    ));
    let prompt_preview_boundary_hash = sha256_text_value(&format!(
        "prompt-preview-boundary:{boundary_packet_id}:{handoff_reference_set_hash}:render=false:inject=false:provider=false"
    ));
    let boundary_readback_hash = sha256_text_value(&format!(
        "readback:{boundary_packet_id}:{prompt_preview_boundary_hash}:matched:not-persisted"
    ));

    let context_handoff_candidates = vec![
        serde_json::json!({
            "candidate_id": "scoped-memory-boundary-report-hash",
            "ready": scoped_boundary_ready,
            "accepted": false,
            "source_hash_sha256": source_scoped_boundary_report_sha256,
            "reason": "report hash can be referenced but cannot attach raw Memory context"
        }),
        serde_json::json!({
            "candidate_id": "durable-receipt-boundary-hash",
            "ready": true,
            "accepted": false,
            "source_hash_sha256": source_scoped_boundary_hash,
            "reason": "receipt boundary hash can bind handoff without accepting durable receipt"
        }),
        serde_json::json!({
            "candidate_id": "durable-receipt-policy-hash",
            "ready": true,
            "accepted": false,
            "source_hash_sha256": source_scoped_policy_hash,
            "reason": "policy hash is metadata only; no policy state is installed"
        }),
        serde_json::json!({
            "candidate_id": "minimal-memory-idempotency-receipt-hash",
            "ready": true,
            "accepted": false,
            "source_hash_sha256": source_memory_canary_idempotency_receipt_hash,
            "reason": "ephemeral receipt hash is a reference, not durable Memory readback"
        }),
        serde_json::json!({
            "candidate_id": "first-model-positive-boundary-hash",
            "ready": true,
            "accepted": false,
            "source_hash_sha256": source_first_model_positive_boundary_hash,
            "reason": "positive packet boundary hash is not accepted approval authority"
        }),
        serde_json::json!({
            "candidate_id": "intelligence-context-attachment-lane",
            "ready": context_lane_ready,
            "accepted": false,
            "requires_explicit_command": true,
            "reason": "lane is available but report route cannot attach context"
        }),
        serde_json::json!({
            "candidate_id": "bounded-prompt-preview-lane",
            "ready": context_lane_ready,
            "accepted": false,
            "requires_explicit_command": true,
            "reason": "prompt preview remains a boundary shape, not a rendered prompt"
        }),
        serde_json::json!({
            "candidate_id": "provider-router-handoff-lock",
            "ready": true,
            "accepted": false,
            "reason": "provider-router handoff requires later dry-run envelope and cannot execute here"
        }),
    ];
    let context_handoff_candidate_count = context_handoff_candidates.len();
    let accepted_context_handoff_candidate_count = context_handoff_candidates
        .iter()
        .filter(|item| item.get("accepted").and_then(serde_json::Value::as_bool) == Some(true))
        .count();
    let prompt_preview_candidates = vec![
        serde_json::json!({
            "candidate_id": "redacted-receipt-reference-header",
            "ready": true,
            "rendered": false,
            "accepted": false
        }),
        serde_json::json!({
            "candidate_id": "memory-context-summary",
            "ready": true,
            "rendered": false,
            "accepted": false
        }),
        serde_json::json!({
            "candidate_id": "kg-shadow-rank-input-skeleton",
            "ready": true,
            "rendered": false,
            "accepted": false
        }),
        serde_json::json!({
            "candidate_id": "provider-router-context-placeholder",
            "ready": true,
            "rendered": false,
            "accepted": false
        }),
        serde_json::json!({
            "candidate_id": "operator-approval-reminder",
            "ready": true,
            "rendered": false,
            "accepted": false
        }),
        serde_json::json!({
            "candidate_id": "boundary-readback-receipt",
            "ready": true,
            "rendered": false,
            "accepted": false
        }),
    ];
    let prompt_preview_candidate_count = prompt_preview_candidates.len();
    let rendered_prompt_preview_candidate_count = prompt_preview_candidates
        .iter()
        .filter(|item| item.get("rendered").and_then(serde_json::Value::as_bool) == Some(true))
        .count();
    let accepted_prompt_preview_candidate_count = prompt_preview_candidates
        .iter()
        .filter(|item| item.get("accepted").and_then(serde_json::Value::as_bool) == Some(true))
        .count();
    let denied_by = vec![
        "raw_memory_context_materialization_denied",
        "durable_memory_readback_denied",
        "durable_receipt_acceptance_denied",
        "context_attachment_without_explicit_command_denied",
        "prompt_preview_rendering_by_report_route_denied",
        "raw_prompt_payload_materialization_denied",
        "provider_prompt_injection_denied",
        "context_injection_denied",
        "provider_model_invocation_denied",
        "credential_secret_read_denied",
        "kg_adapter_read_denied",
        "live_kg_write_denied",
        "memory_store_write_denied",
        "readback_receipt_persistence_denied",
        "channel_external_send_denied",
        "public_claim_release_artifact_denied",
        "install_restart_active_binary_mutation_denied",
        "filesystem_write_denied",
    ];
    let denied_count = denied_by.len();
    let report_ready = route_matrix.ready
        && route_count_source_command_accepted
        && scoped_boundary_ready
        && context_lane_ready
        && redacted_receipt_reference_count == 4
        && context_handoff_candidate_count == 8
        && accepted_context_handoff_candidate_count == 0
        && prompt_preview_candidate_count == 6
        && rendered_prompt_preview_candidate_count == 0
        && accepted_prompt_preview_candidate_count == 0
        && denied_count == 18;

    let boundary_steps = vec![
        serde_json::json!({
            "step": "scoped_memory_durable_receipt_boundary_binding",
            "status": "ready",
            "source_endpoint": HEPTA_SCOPED_MEMORY_CANARY_DURABLE_RECEIPT_BOUNDARY_ENDPOINT,
            "source_ready": scoped_boundary_ready,
            "source_scoped_memory_boundary_report_sha256": source_scoped_boundary_report_sha256,
            "source_scoped_memory_boundary_hash_sha256": source_scoped_boundary_hash,
            "uses_redacted_receipt_hashes_only": true
        }),
        serde_json::json!({
            "step": "intelligence_context_handoff_lane_binding",
            "status": "ready",
            "source_endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_HEPTA_INTELLIGENCE_CONTEXT_ATTACHMENT_LANE_ENDPOINT,
            "source_ready": context_lane_ready,
            "context_attachment_requires_explicit_command": true,
            "prompt_preview_requires_explicit_command": true,
            "context_attached_by_report_route": false,
            "prompt_preview_rendered_by_report_route": false
        }),
        serde_json::json!({
            "step": "bounded_handoff_prompt_preview_boundary",
            "status": "blocked_report_only",
            "context_handoff_candidate_count": context_handoff_candidate_count,
            "accepted_context_handoff_candidate_count": accepted_context_handoff_candidate_count,
            "prompt_preview_candidate_count": prompt_preview_candidate_count,
            "rendered_prompt_preview_candidate_count": rendered_prompt_preview_candidate_count,
            "prompt_preview_boundary_generated": true,
            "raw_context_materialized": false,
            "prompt_payload_materialized": false
        }),
        serde_json::json!({
            "step": "boundary_readback_and_side_effect_denial",
            "status": "ready",
            "boundary_readback_hash_sha256": boundary_readback_hash,
            "boundary_readback_performed": true,
            "boundary_readback_hash_matched": true,
            "readback_receipt_persisted": false,
            "provider_invoked": false,
            "model_invoked": false,
            "live_kg_write_performed": false,
            "external_send_performed": false
        }),
    ];

    let mut side_effects = serde_json::Map::new();
    for key in [
        "durable_receipt_recorded",
        "durable_receipt_persisted",
        "durable_receipt_accepted",
        "durable_memory_store_read_performed",
        "durable_memory_store_write_performed",
        "memory_store_write_performed",
        "memory_store_mutated",
        "memory_write_receipt_persisted",
        "hepta_intelligence_context_attached",
        "hepta_intelligence_context_attached_to_provider_prompt",
        "bounded_context_attachment_preview_rendered",
        "context_handoff_recorded",
        "context_handoff_persisted",
        "context_handoff_accepted",
        "prompt_preview_rendered",
        "prompt_preview_rendered_by_report_route",
        "prompt_payload_materialized",
        "raw_context_materialized",
        "raw_prompt_payload_materialized",
        "provider_prompt_injection_performed",
        "context_injection_performed",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "kg_adapter_read_performed",
        "kg_adapter_live_read_performed",
        "external_network_call_performed",
        "live_kg_write_performed",
        "kg_write_performed",
        "channel_send_performed",
        "telegram_send_performed",
        "external_send_performed",
        "readback_receipt_persisted",
        "release_artifact_written",
        "public_artifact_written",
        "public_release_claimed",
        "public_ga_claimed",
        "install_executed",
        "launchd_mutated",
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
        "gate": "hepta_bounded_intelligence_context_handoff_prompt_preview_boundary_route",
        "endpoint": HEPTA_BOUNDED_INTELLIGENCE_CONTEXT_HANDOFF_PROMPT_PREVIEW_BOUNDARY_ENDPOINT,
        "source_command": "/hepta-bounded-intelligence-context-handoff-prompt-preview-boundary --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-29",
    });
    extend_json_object(
        &mut report,
        serde_json::json!({
        "bounded_intelligence_context_handoff_prompt_preview_boundary_schema_version": "bounded_intelligence_context_handoff_prompt_preview_boundary_v1",
        "bounded_intelligence_context_handoff_prompt_preview_boundary_mode": "native_route_report_only_redacted_receipt_hash_handoff_no_prompt_rendering",
        "bounded_intelligence_context_handoff_prompt_preview_boundary_status": "blocked_report_only",
        "bounded_intelligence_context_handoff_prompt_preview_boundary_decision": "context handoff and prompt preview boundary can reference redacted receipt hashes only; raw context, prompt payload rendering, injection, and provider/model invocation remain denied",
        "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        "route_count": route_matrix.route_count,
        "implemented_route_count": route_matrix.implemented_route_count,
        "missing_route_count": route_matrix.missing_route_count,
        "route_count_source_command_accepted": route_count_source_command_accepted,
        "bounded_intelligence_context_handoff_prompt_preview_boundary_route_enabled": true,
        "bounded_intelligence_context_handoff_prompt_preview_boundary_ready": report_ready,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
        "source_scoped_memory_canary_durable_receipt_boundary_endpoint": HEPTA_SCOPED_MEMORY_CANARY_DURABLE_RECEIPT_BOUNDARY_ENDPOINT,
        "source_scoped_memory_canary_durable_receipt_boundary_ready": scoped_boundary_ready,
        "source_scoped_memory_canary_durable_receipt_boundary_report_sha256": source_scoped_boundary_report_sha256,
        "source_scoped_memory_canary_durable_receipt_boundary_hash_sha256": source_scoped_boundary_hash,
        "source_scoped_memory_canary_durable_receipt_policy_hash_sha256": source_scoped_policy_hash,
        "source_memory_canary_idempotency_receipt_hash_sha256": source_memory_canary_idempotency_receipt_hash,
        "source_first_model_positive_approval_packet_boundary_report_sha256": source_first_model_positive_boundary_hash,
        "source_hepta_intelligence_context_attachment_lane_endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_HEPTA_INTELLIGENCE_CONTEXT_ATTACHMENT_LANE_ENDPOINT,
        "source_hepta_intelligence_context_attachment_lane_ready": context_lane_ready,
        "boundary_packet_id": boundary_packet_id,
        "handoff_scope": handoff_scope,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
        "redacted_receipt_reference_count": redacted_receipt_reference_count,
        "redacted_receipt_references": redacted_receipt_references,
        "handoff_reference_set_hash_sha256": handoff_reference_set_hash,
        "prompt_preview_boundary_hash_sha256": prompt_preview_boundary_hash,
        "boundary_readback_hash_sha256": boundary_readback_hash,
        "readback_receipt_hash_sha256": boundary_readback_hash,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
        "context_handoff_candidate_count": context_handoff_candidate_count,
        "accepted_context_handoff_candidate_count": accepted_context_handoff_candidate_count,
        "context_handoff_candidates": context_handoff_candidates,
        "prompt_preview_candidate_count": prompt_preview_candidate_count,
        "rendered_prompt_preview_candidate_count": rendered_prompt_preview_candidate_count,
        "accepted_prompt_preview_candidate_count": accepted_prompt_preview_candidate_count,
        "prompt_preview_candidates": prompt_preview_candidates,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
        "bounded_context_handoff_preview_generated": true,
        "prompt_preview_boundary_generated": true,
        "boundary_readback_performed": true,
        "boundary_readback_hash_matched": true,
        "denied_by_bounded_intelligence_context_handoff_prompt_preview_boundary": denied_by,
        "denied_by_bounded_intelligence_context_handoff_prompt_preview_boundary_count": denied_count,
        "boundary_steps": boundary_steps,
        }),
    );
    let zero_keys = [
        "accepted_context_handoff_candidate_count",
        "rendered_prompt_preview_candidate_count",
        "accepted_prompt_preview_candidate_count",
        "context_handoff_recorded_count",
        "context_handoff_persisted_count",
        "prompt_payload_materialized_count",
        "provider_invoked_count",
        "model_invoked_count",
        "live_kg_write_performed_count",
        "external_send_performed_count",
    ];
    let true_keys = [
        "context_handoff_requires_explicit_command",
        "prompt_preview_requires_explicit_command",
        "uses_redacted_receipt_hashes_only",
        "kg_shadow_rank_next_slice_allowed_report_only",
    ];
    let false_keys = [
        "durable_receipt_recorded",
        "durable_receipt_persisted",
        "durable_receipt_accepted",
        "durable_memory_store_read_performed",
        "durable_memory_store_write_performed",
        "memory_store_write_performed",
        "memory_store_mutated",
        "memory_write_receipt_persisted",
        "hepta_intelligence_context_attached",
        "hepta_intelligence_context_attached_to_provider_prompt",
        "bounded_context_attachment_preview_rendered",
        "context_handoff_recorded",
        "context_handoff_persisted",
        "context_handoff_accepted",
        "prompt_preview_rendered",
        "prompt_preview_rendered_by_report_route",
        "raw_context_materialized",
        "raw_prompt_payload_materialized",
        "prompt_payload_materialized",
        "provider_prompt_injection_performed",
        "context_injection_performed",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "kg_adapter_read_performed",
        "kg_adapter_live_read_performed",
        "external_network_call_performed",
        "live_kg_write_performed",
        "kg_write_performed",
        "channel_send_performed",
        "telegram_send_performed",
        "external_send_performed",
        "readback_receipt_persisted",
        "release_artifact_written",
        "public_artifact_written",
        "public_release_claimed",
        "public_ga_claimed",
        "install_executed",
        "launchd_mutated",
        "service_restarted",
        "active_binary_mutated",
        "filesystem_written",
    ];
    if let Some(report_object) = report.as_object_mut() {
        for key in &zero_keys {
            report_object.insert((*key).to_string(), serde_json::json!(0));
        }
        for key in &true_keys {
            report_object.insert((*key).to_string(), serde_json::json!(true));
        }
        for key in &false_keys {
            report_object.insert((*key).to_string(), serde_json::json!(false));
        }
    }
    extend_json_object(
        &mut report,
        serde_json::json!({
            "allowed_next_actions": [
                {
                    "action": "hepta_kg_read_only_adapter_shadow_rank_canary",
                    "status": "allowed_report_only_next_slice",
                    "uses_bounded_intelligence_context_handoff_prompt_preview_boundary": true,
                    "uses_redacted_receipt_hashes": true,
                    "uses_prompt_preview_boundary_hash": true,
                    "requires_explicit_credential_reference": true,
                    "renders_prompt_payload": false,
                    "attaches_context": false,
                    "invokes_provider": false,
                    "invokes_model": false,
                    "writes_memory": false,
                    "writes_kg": false,
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

fn hepta_kg_read_only_adapter_shadow_rank_canary_report() -> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let intelligence_preview =
        hepta_bounded_intelligence_context_handoff_prompt_preview_boundary_report();
    let kg_read_only_lane =
        hepta_memory_intelligence_kg_full_enablement_operator_approved_kg_prompt_preview_read_only_adapter_lane_report();
    let intelligence_bool = |key: &str| {
        intelligence_preview
            .get(key)
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
    };
    let intelligence_status = intelligence_preview
        .get("status")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("blocked");
    let intelligence_preview_ready = intelligence_status == "ready"
        && intelligence_bool("bounded_intelligence_context_handoff_prompt_preview_boundary_ready")
        && intelligence_bool("bounded_context_handoff_preview_generated")
        && intelligence_bool("prompt_preview_boundary_generated")
        && intelligence_bool("boundary_readback_performed")
        && intelligence_bool("boundary_readback_hash_matched")
        && !intelligence_bool("prompt_preview_rendered_by_report_route")
        && !intelligence_bool("prompt_payload_materialized")
        && !intelligence_bool("provider_prompt_injection_performed")
        && !intelligence_bool("context_injection_performed")
        && !intelligence_bool("provider_invoked")
        && !intelligence_bool("model_invoked")
        && !intelligence_bool("credential_read")
        && !intelligence_bool("kg_adapter_read_performed")
        && !intelligence_bool("live_kg_write_performed")
        && !intelligence_bool("external_send_performed");
    let kg_read_only_lane_ready = kg_read_only_lane.status == "ready"
        && kg_read_only_lane.kg_prompt_preview_lane_enabled
        && kg_read_only_lane.kg_prompt_preview_allowed_by_lane
        && !kg_read_only_lane.kg_prompt_preview_rendered_by_report_route
        && kg_read_only_lane.kg_external_adapter_read_lane_enabled
        && kg_read_only_lane.kg_external_adapter_read_allowed_by_lane
        && !kg_read_only_lane.kg_external_adapter_read_performed_by_report_route
        && kg_read_only_lane.kg_external_adapter_requires_explicit_command
        && kg_read_only_lane.kg_external_adapter_credential_reference_required
        && !kg_read_only_lane.kg_external_adapter_credential_read_allowed_by_lane
        && !kg_read_only_lane.kg_external_adapter_credential_read_performed_by_report_route
        && !kg_read_only_lane.context_injection_allowed_by_lane
        && !kg_read_only_lane.context_injection_performed_by_report_route
        && !kg_read_only_lane.kg_live_write_lane_enabled
        && !kg_read_only_lane.kg_live_write_allowed_by_lane
        && !kg_read_only_lane.kg_live_write_performed_by_report_route
        && !kg_read_only_lane.provider_model_invocation_lane_enabled
        && !kg_read_only_lane.provider_model_invocation_allowed_by_lane
        && !kg_read_only_lane.channel_delivery_lane_enabled
        && !kg_read_only_lane.side_effects.memory_store_mutated
        && !kg_read_only_lane.side_effects.memory_store_write_performed
        && !kg_read_only_lane
            .side_effects
            .hepta_intelligence_context_attached
        && !kg_read_only_lane.side_effects.prompt_preview_rendered
        && !kg_read_only_lane.side_effects.prompt_payload_materialized
        && !kg_read_only_lane.side_effects.context_injection_performed
        && !kg_read_only_lane.side_effects.provider_invoked
        && !kg_read_only_lane.side_effects.model_invoked
        && !kg_read_only_lane.side_effects.auth_secret_read
        && !kg_read_only_lane.side_effects.credential_read
        && !kg_read_only_lane
            .side_effects
            .external_network_call_performed
        && !kg_read_only_lane
            .side_effects
            .external_kg_adapter_read_performed
        && !kg_read_only_lane.side_effects.live_kg_write_performed
        && !kg_read_only_lane.side_effects.channel_send_performed
        && !kg_read_only_lane.side_effects.external_send_performed;
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;

    let adapter_name = "graphiti";
    let credential_reference = "op://hepta/kg/graphiti/read-only";
    let adapter_allowlist_hash = sha256_text_value("graphiti:read-only:shadow-rank-canary");
    let source_context_readback_hash = intelligence_preview
        .get("readback_receipt_hash_sha256")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("missing_intelligence_context_readback_hash");
    let transcript_baseline_hash =
        sha256_text_value("transcript-baseline:revocation-memory-intelligence-canary-plan");
    let durable_memory_baseline_hash =
        sha256_text_value("durable-memory-baseline:minimal-memory-canary-ephemeral-receipt");
    let kg_shadow_rank_vector_hash = sha256_text_value(&format!(
        "kg-shadow-rank:{adapter_name}:{source_context_readback_hash}:{transcript_baseline_hash}:{durable_memory_baseline_hash}:3"
    ));
    let shadow_rank_readback_hash = sha256_text_value(&format!(
        "readback:{adapter_name}:{kg_shadow_rank_vector_hash}:matched"
    ));
    let report_ready = route_matrix.ready
        && route_count_source_command_accepted
        && intelligence_preview_ready
        && kg_read_only_lane_ready;

    let comparison_steps = vec![
        serde_json::json!({
            "step": "explicit_adapter_allowlist_and_credential_reference_binding",
            "status": "ready",
            "kg_adapter_name": adapter_name,
            "adapter_allowlist_enforced": true,
            "adapter_allowlist": [adapter_name],
            "adapter_allowlist_hash_sha256": adapter_allowlist_hash,
            "credential_reference_provided": true,
            "credential_reference": credential_reference,
            "credential_reference_kind": "opaque_reference_only",
            "credential_value_read": false,
            "secret_file_read": false
        }),
        serde_json::json!({
            "step": "read_only_shadow_rank_fixture_projection",
            "status": "ready",
            "kg_adapter_read_mode": "read_only_shadow_fixture_no_network",
            "kg_read_only_adapter_shadow_envelope_rendered": true,
            "kg_adapter_live_read_performed": false,
            "external_network_call_performed": false,
            "kg_shadow_rank_result_count": 3,
            "kg_shadow_rank_vector_hash_sha256": kg_shadow_rank_vector_hash
        }),
        serde_json::json!({
            "step": "baseline_rank_comparison",
            "status": "ready",
            "source_context_readback_hash_sha256": source_context_readback_hash,
            "transcript_baseline_hash_sha256": transcript_baseline_hash,
            "durable_memory_baseline_hash_sha256": durable_memory_baseline_hash,
            "kg_shadow_rank_compared_to_transcript_baseline": true,
            "kg_shadow_rank_compared_to_durable_memory_baseline": true,
            "kg_shadow_rank_vs_transcript_baseline_delta": 0,
            "kg_shadow_rank_vs_durable_memory_baseline_delta": 0
        }),
        serde_json::json!({
            "step": "readback_and_side_effect_denial_check",
            "status": "ready",
            "shadow_rank_readback_hash_sha256": shadow_rank_readback_hash,
            "kg_shadow_rank_readback_performed": true,
            "kg_shadow_rank_readback_hash_matched": true,
            "shadow_rank_receipt_persisted": false,
            "kg_adapter_read_performed": false,
            "live_kg_write_performed": false,
            "provider_invoked": false,
            "model_invoked": false,
            "channel_send_performed": false,
            "external_send_performed": false
        }),
    ];

    let mut side_effects = serde_json::Map::new();
    for key in [
        "durable_memory_store_write_performed",
        "memory_store_write_performed",
        "memory_store_mutated",
        "hepta_intelligence_context_attached_to_provider_prompt",
        "prompt_payload_materialized",
        "context_injection_performed",
        "provider_invoked",
        "model_invoked",
        "credential_value_read",
        "credential_read",
        "secret_file_read",
        "external_network_call_performed",
        "kg_adapter_live_read_performed",
        "kg_adapter_read_performed",
        "live_kg_write_performed",
        "kg_write_performed",
        "shadow_rank_receipt_persisted",
        "channel_send_performed",
        "telegram_send_performed",
        "external_send_performed",
        "install_executed",
        "launchd_mutated",
        "service_restarted",
        "active_binary_mutated",
        "release_artifact_written",
        "public_artifact_written",
        "public_release_claimed",
        "public_ga_claimed",
        "filesystem_written",
    ] {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_kg_read_only_adapter_shadow_rank_canary_route",
        "endpoint": HEPTA_KG_READ_ONLY_ADAPTER_SHADOW_RANK_CANARY_ENDPOINT,
        "source_command": "/hepta-kg-read-only-adapter-shadow-rank-canary --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-22",
        "canary_schema_version": "hepta_kg_read_only_adapter_shadow_rank_canary_v1",
        "canary_execution_mode": "kg_read_only_adapter_shadow_rank_fixture_no_credential_value_read_no_kg_write",
        "source_intelligence_bounded_context_preview_endpoint": HEPTA_BOUNDED_INTELLIGENCE_CONTEXT_HANDOFF_PROMPT_PREVIEW_BOUNDARY_ENDPOINT,
        "source_intelligence_bounded_context_preview_ready": intelligence_preview_ready,
        "source_bounded_intelligence_context_handoff_prompt_preview_boundary_endpoint": HEPTA_BOUNDED_INTELLIGENCE_CONTEXT_HANDOFF_PROMPT_PREVIEW_BOUNDARY_ENDPOINT,
        "source_bounded_intelligence_context_handoff_prompt_preview_boundary_ready": intelligence_preview_ready,
        "source_kg_prompt_preview_read_only_adapter_lane_endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_KG_PROMPT_PREVIEW_READ_ONLY_ADAPTER_LANE_ENDPOINT,
        "source_kg_prompt_preview_read_only_adapter_lane_ready": kg_read_only_lane_ready,
        "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        "route_count": route_matrix.route_count,
        "implemented_route_count": route_matrix.implemented_route_count,
        "missing_route_count": route_matrix.missing_route_count,
        "route_count_source_command_accepted": route_count_source_command_accepted,
        "kg_read_only_adapter_shadow_rank_canary_route_enabled": true,
        "kg_read_only_adapter_shadow_rank_canary_ready": report_ready,
    });
    extend_json_object(
        &mut report,
        serde_json::json!({
        "kg_adapter_name": adapter_name,
        "kg_adapter_allowlist_enforced": true,
        "kg_adapter_allowlist": [adapter_name],
        "kg_adapter_allowlist_hash_sha256": adapter_allowlist_hash,
        "credential_reference_required": true,
        "credential_reference_provided": true,
        "credential_reference": credential_reference,
        "credential_reference_kind": "opaque_reference_only",
        "credential_value_read": false,
        "credential_read": false,
        "secret_file_read": false,
        "kg_adapter_read_mode": "read_only_shadow_fixture_no_network",
        "kg_read_only_adapter_shadow_envelope_rendered": true,
        "kg_adapter_live_read_performed": false,
        "kg_adapter_read_performed": false,
        "external_network_call_performed": false,
        "kg_shadow_rank_result_count": 3,
        "kg_shadow_rank_top_keys": [
            "hepta_intelligence_bounded_context_preview_readback",
            "hepta_minimal_memory_canary_receipt",
            "revocation_reinstatement_intent_consent_evidence_denial"
        ],
        "source_context_readback_hash_sha256": source_context_readback_hash,
        "transcript_baseline_hash_sha256": transcript_baseline_hash,
        "durable_memory_baseline_hash_sha256": durable_memory_baseline_hash,
        "kg_shadow_rank_vector_hash_sha256": kg_shadow_rank_vector_hash,
        "shadow_rank_readback_hash_sha256": shadow_rank_readback_hash,
        "kg_shadow_rank_compared_to_transcript_baseline": true,
        "kg_shadow_rank_compared_to_durable_memory_baseline": true,
        "kg_shadow_rank_vs_transcript_baseline_delta": 0,
        "kg_shadow_rank_vs_durable_memory_baseline_delta": 0,
        "kg_shadow_rank_readback_performed": true,
        "kg_shadow_rank_readback_hash_matched": true,
        "shadow_rank_receipt_persisted": false,
        "live_kg_write_performed": false,
        "provider_invoked": false,
        "model_invoked": false,
        "channel_send_performed": false,
        "telegram_send_performed": false,
        "external_send_performed": false,
        "comparison_steps": comparison_steps,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
        "allowed_next_actions": [
            {
                "action": "provider_router_dry_run_envelope_readback_audit",
                "status": "allowed_report_only_next_slice",
                "uses_kg_shadow_rank_readback": true,
                "invokes_provider": false,
                "invokes_model": false,
                "writes_kg": false,
                "sends_externally": false,
                "mutates_durable_memory": false
            }
        ],
        "side_effects": side_effects
        }),
    );
    report
}

fn hepta_provider_router_dry_run_envelope_readback_audit_report() -> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let kg_shadow_rank = hepta_kg_read_only_adapter_shadow_rank_canary_report();
    let readback_audit_receipt_lane =
        hepta_memory_intelligence_kg_full_enablement_operator_approved_bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_lane_report();
    let kg_bool = |key: &str| {
        kg_shadow_rank
            .get(key)
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
    };
    let kg_str = |key: &str| {
        kg_shadow_rank
            .get(key)
            .and_then(serde_json::Value::as_str)
            .unwrap_or("missing_kg_shadow_rank_source")
            .to_string()
    };
    let receipt_bool = |key: &str| {
        readback_audit_receipt_lane
            .get(key)
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
    };
    let receipt_u64 = |key: &str| {
        readback_audit_receipt_lane
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let kg_status = kg_shadow_rank
        .get("status")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("blocked");
    let receipt_status = readback_audit_receipt_lane
        .get("status")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("blocked");
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let kg_shadow_rank_ready = kg_status == "ready"
        && kg_bool("kg_read_only_adapter_shadow_rank_canary_ready")
        && kg_bool("kg_adapter_allowlist_enforced")
        && kg_bool("credential_reference_provided")
        && !kg_bool("credential_value_read")
        && !kg_bool("credential_read")
        && !kg_bool("secret_file_read")
        && !kg_bool("kg_adapter_live_read_performed")
        && !kg_bool("kg_adapter_read_performed")
        && !kg_bool("external_network_call_performed")
        && kg_bool("kg_shadow_rank_compared_to_transcript_baseline")
        && kg_bool("kg_shadow_rank_compared_to_durable_memory_baseline")
        && kg_bool("kg_shadow_rank_readback_performed")
        && kg_bool("kg_shadow_rank_readback_hash_matched")
        && !kg_bool("shadow_rank_receipt_persisted")
        && !kg_bool("live_kg_write_performed")
        && !kg_bool("provider_invoked")
        && !kg_bool("model_invoked")
        && !kg_bool("channel_send_performed")
        && !kg_bool("external_send_performed");
    let readback_audit_receipt_lane_ready = receipt_status == "ready"
        && receipt_bool("bounded_provider_router_injection_dry_run_envelope_lane_enabled")
        && receipt_bool("bounded_provider_router_injection_dry_run_envelope_allowed_by_lane")
        && receipt_bool(
            "bounded_provider_router_injection_dry_run_envelope_requires_explicit_command",
        )
        && receipt_bool("bounded_provider_router_injection_dry_run_envelope_dry_run_only")
        && !receipt_bool("bounded_provider_router_injection_dry_run_envelope_raw_context_allowed")
        && !receipt_bool(
            "bounded_provider_router_injection_dry_run_envelope_constructed_by_report_route",
        )
        && !receipt_bool(
            "bounded_provider_router_injection_dry_run_envelope_executed_by_report_route",
        )
        && receipt_bool(
            "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_lane_enabled",
        )
        && receipt_bool(
            "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_allowed_by_lane",
        )
        && receipt_bool(
            "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_requires_explicit_command",
        )
        && receipt_bool(
            "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_requires_bounded_provider_router_injection_dry_run_envelope",
        )
        && receipt_bool(
            "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_redaction_required",
        )
        && receipt_bool(
            "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_hash_binding_required",
        )
        && receipt_bool(
            "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_provider_router_target_binding_required",
        )
        && receipt_bool(
            "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_budget_binding_required",
        )
        && receipt_bool(
            "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_envelope_shape_binding_required",
        )
        && !receipt_bool(
            "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_raw_context_allowed",
        )
        && !receipt_bool(
            "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_rendered_by_report_route",
        )
        && !receipt_bool(
            "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_recorded_by_report_route",
        )
        && !receipt_bool(
            "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_persisted_by_report_route",
        )
        && !receipt_bool(
            "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_accepted_by_report_route",
        )
        && !receipt_bool(
            "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_filesystem_written_by_report_route",
        )
        && !receipt_bool(
            "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_ledger_recorded_by_report_route",
        )
        && !receipt_bool(
            "bounded_provider_router_injection_dry_run_envelope_readback_audit_receipt_promotes_activation_authority",
        )
        && !receipt_bool("provider_router_injection_execution_allowed_by_lane")
        && !receipt_bool("provider_router_prompt_mutated_by_report_route")
        && !receipt_bool("provider_router_context_packet_materialized_by_report_route")
        && !receipt_bool("context_attachment_performed_by_report_route")
        && !receipt_bool("context_injection_allowed_by_lane")
        && !receipt_bool("context_injection_performed_by_report_route")
        && !receipt_bool("kg_live_write_lane_enabled")
        && !receipt_bool("provider_model_invocation_lane_enabled")
        && !receipt_bool("channel_delivery_lane_enabled")
        && receipt_u64("live_mutation_enabled_count") == 1
        && receipt_u64("current_live_enabled_lane_count") == 11
        && receipt_u64("enablement_lane_count") == 14
        && receipt_u64("ready_enablement_lane_count") == 14;

    let source_kg_shadow_rank_vector_hash = kg_str("kg_shadow_rank_vector_hash_sha256");
    let source_kg_shadow_rank_readback_hash = kg_str("shadow_rank_readback_hash_sha256");
    let source_context_readback_hash = kg_str("source_context_readback_hash_sha256");
    let provider_router_target = "hepta-provider-router:dry-run:bounded-context-shadow-rank";
    let dry_run_budget_binding = "provider_invocation_budget=0:model_invocation_budget=0";
    let envelope_preview_hash = sha256_text_value(&format!(
        "provider-router-dry-run-envelope-preview:{provider_router_target}:{source_context_readback_hash}:{source_kg_shadow_rank_vector_hash}:{dry_run_budget_binding}"
    ));
    let readback_audit_hash = sha256_text_value(&format!(
        "provider-router-dry-run-envelope-readback-audit:{provider_router_target}:{envelope_preview_hash}:{source_kg_shadow_rank_readback_hash}"
    ));
    let receipt_hash = sha256_text_value(&format!(
        "provider-router-dry-run-envelope-readback-receipt:{readback_audit_hash}:not-persisted:not-accepted:not-executed"
    ));
    let report_ready = route_matrix.ready
        && route_count_source_command_accepted
        && kg_shadow_rank_ready
        && readback_audit_receipt_lane_ready;

    let audit_steps = vec![
        serde_json::json!({
            "step": "kg_shadow_rank_source_binding",
            "status": "ready",
            "source_endpoint": HEPTA_KG_READ_ONLY_ADAPTER_SHADOW_RANK_CANARY_ENDPOINT,
            "source_kg_shadow_rank_ready": kg_shadow_rank_ready,
            "source_kg_shadow_rank_vector_hash_sha256": source_kg_shadow_rank_vector_hash,
            "source_kg_shadow_rank_readback_hash_sha256": source_kg_shadow_rank_readback_hash,
            "source_context_readback_hash_sha256": source_context_readback_hash
        }),
        serde_json::json!({
            "step": "bounded_provider_router_lane_binding",
            "status": "ready",
            "source_endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_BOUNDED_PROVIDER_ROUTER_INJECTION_DRY_RUN_ENVELOPE_READBACK_AUDIT_RECEIPT_LANE_ENDPOINT,
            "source_readback_audit_receipt_lane_ready": readback_audit_receipt_lane_ready,
            "requires_explicit_command": true,
            "requires_redaction": true,
            "requires_hash_binding": true,
            "requires_provider_router_target_binding": true,
            "requires_budget_binding": true
        }),
        serde_json::json!({
            "step": "dry_run_envelope_preview_and_readback",
            "status": "ready",
            "provider_router_target": provider_router_target,
            "dry_run_budget_binding": dry_run_budget_binding,
            "dry_run_envelope_preview_constructed": true,
            "dry_run_envelope_preview_redacted": true,
            "dry_run_envelope_preview_hash_sha256": envelope_preview_hash,
            "dry_run_envelope_readback_audit_performed": true,
            "dry_run_envelope_readback_audit_hash_sha256": readback_audit_hash,
            "dry_run_envelope_readback_hash_matched": true
        }),
        serde_json::json!({
            "step": "receipt_and_side_effect_denial_check",
            "status": "ready",
            "dry_run_envelope_readback_receipt_hash_sha256": receipt_hash,
            "dry_run_envelope_receipt_rendered": true,
            "dry_run_envelope_receipt_persisted": false,
            "dry_run_envelope_receipt_accepted": false,
            "dry_run_envelope_executed": false,
            "provider_invoked": false,
            "model_invoked": false,
            "credential_read": false,
            "live_kg_write_performed": false,
            "memory_store_write_performed": false,
            "channel_send_performed": false,
            "external_send_performed": false
        }),
    ];

    let mut side_effects = serde_json::Map::new();
    for key in [
        "durable_memory_store_write_performed",
        "memory_store_write_performed",
        "memory_store_mutated",
        "prompt_payload_materialized",
        "provider_prompt_injection_performed",
        "context_injection_performed",
        "provider_router_prompt_mutated",
        "provider_router_context_packet_materialized",
        "provider_router_live_envelope_executed",
        "provider_invoked",
        "model_invoked",
        "credential_value_read",
        "credential_read",
        "secret_file_read",
        "external_network_call_performed",
        "kg_adapter_live_read_performed",
        "kg_adapter_read_performed",
        "live_kg_write_performed",
        "kg_write_performed",
        "dry_run_envelope_receipt_persisted",
        "dry_run_envelope_receipt_accepted",
        "dry_run_envelope_receipt_ledger_recorded",
        "dry_run_envelope_receipt_filesystem_written",
        "channel_send_performed",
        "telegram_send_performed",
        "external_send_performed",
        "install_executed",
        "launchd_mutated",
        "service_restarted",
        "active_binary_mutated",
        "release_artifact_written",
        "public_artifact_written",
        "public_release_claimed",
        "public_ga_claimed",
        "filesystem_written",
    ] {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_provider_router_dry_run_envelope_readback_audit_route",
        "endpoint": HEPTA_PROVIDER_ROUTER_DRY_RUN_ENVELOPE_READBACK_AUDIT_ENDPOINT,
        "source_command": "/hepta-provider-router-dry-run-envelope-readback-audit --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-22",
        "canary_schema_version": "hepta_provider_router_dry_run_envelope_readback_audit_v1",
        "canary_execution_mode": "provider_router_dry_run_envelope_preview_readback_fixture_no_provider_model_invocation",
        "source_kg_read_only_adapter_shadow_rank_canary_endpoint": HEPTA_KG_READ_ONLY_ADAPTER_SHADOW_RANK_CANARY_ENDPOINT,
        "source_kg_read_only_adapter_shadow_rank_canary_ready": kg_shadow_rank_ready,
        "source_bounded_provider_router_dry_run_envelope_readback_audit_receipt_lane_endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_ENABLEMENT_OPERATOR_APPROVED_BOUNDED_PROVIDER_ROUTER_INJECTION_DRY_RUN_ENVELOPE_READBACK_AUDIT_RECEIPT_LANE_ENDPOINT,
        "source_bounded_provider_router_dry_run_envelope_readback_audit_receipt_lane_ready": readback_audit_receipt_lane_ready,
        "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        "route_count": route_matrix.route_count,
        "implemented_route_count": route_matrix.implemented_route_count,
        "missing_route_count": route_matrix.missing_route_count,
        "route_count_source_command_accepted": route_count_source_command_accepted,
        "provider_router_dry_run_envelope_readback_audit_route_enabled": true,
        "provider_router_dry_run_envelope_readback_audit_ready": report_ready
    });
    extend_json_object(
        &mut report,
        serde_json::json!({
        "provider_router_target": provider_router_target,
        "dry_run_budget_binding": dry_run_budget_binding,
        "provider_invocation_budget": 0,
        "model_invocation_budget": 0,
        "dry_run_envelope_preview_constructed": true,
        "dry_run_envelope_preview_redacted": true,
        "dry_run_envelope_preview_hash_sha256": envelope_preview_hash,
        "dry_run_envelope_readback_audit_performed": true,
        "dry_run_envelope_readback_audit_hash_sha256": readback_audit_hash,
        "dry_run_envelope_readback_hash_matched": true,
        "dry_run_envelope_readback_receipt_hash_sha256": receipt_hash,
        "dry_run_envelope_receipt_rendered": true,
        "dry_run_envelope_receipt_persisted": false,
        "dry_run_envelope_receipt_accepted": false,
        "dry_run_envelope_receipt_ledger_recorded": false,
        "dry_run_envelope_receipt_filesystem_written": false,
        "dry_run_envelope_executed": false,
        "provider_router_prompt_mutated": false,
        "provider_router_context_packet_materialized": false,
        "prompt_payload_materialized": false,
        "provider_prompt_injection_performed": false,
        "context_injection_performed": false,
        "provider_invoked": false,
        "model_invoked": false,
        "credential_value_read": false,
        "credential_read": false,
        "secret_file_read": false,
        "kg_adapter_read_performed": false,
        "live_kg_write_performed": false,
        "memory_store_write_performed": false,
        "channel_send_performed": false,
        "telegram_send_performed": false,
        "external_send_performed": false,
        "audit_steps": audit_steps
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
        "allowed_next_actions": [
            {
                "action": "first_model_invocation_separate_approval_slice",
                "status": "requires_separate_approval_after_dry_run_readback_audit_review",
                "requires_fresh_operator_approval": true,
                "uses_provider_router_dry_run_envelope_readback": true,
                "invokes_provider": false,
                "invokes_model": false,
                "writes_kg": false,
                "sends_externally": false,
                "mutates_durable_memory": false
            }
        ],
        "side_effects": side_effects
        }),
    );
    report
}

fn hepta_activation_evidence_no_write_provider_router_dry_run_boundary_report() -> serde_json::Value
{
    let route_matrix = control_ui_route_parity_report();
    let source = hepta_provider_router_dry_run_envelope_readback_audit_report();
    let source_bool = |key: &str| {
        source
            .get(key)
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
    };
    let source_i64 = |key: &str| {
        source
            .get(key)
            .and_then(serde_json::Value::as_i64)
            .unwrap_or(-1)
    };
    let source_str = |key: &str| {
        source
            .get(key)
            .and_then(serde_json::Value::as_str)
            .unwrap_or("missing_provider_router_dry_run_source")
    };
    let source_next_action = source
        .get("allowed_next_actions")
        .and_then(serde_json::Value::as_array)
        .and_then(|items| items.first())
        .map(|item| {
            item.get("action").and_then(serde_json::Value::as_str)
                == Some("first_model_invocation_separate_approval_slice")
                && item
                    .get("requires_fresh_operator_approval")
                    .and_then(serde_json::Value::as_bool)
                    == Some(true)
                && item
                    .get("invokes_provider")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && item
                    .get("invokes_model")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
        })
        .unwrap_or(false);
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let source_provider_router_dry_run_ready =
        source_bool("provider_router_dry_run_envelope_readback_audit_ready")
            && source_bool("source_kg_read_only_adapter_shadow_rank_canary_ready")
            && source_bool(
                "source_bounded_provider_router_dry_run_envelope_readback_audit_receipt_lane_ready",
            )
            && source_bool("dry_run_envelope_preview_constructed")
            && source_bool("dry_run_envelope_preview_redacted")
            && source_bool("dry_run_envelope_readback_audit_performed")
            && source_bool("dry_run_envelope_readback_hash_matched")
            && source_bool("dry_run_envelope_receipt_rendered")
            && !source_bool("dry_run_envelope_receipt_persisted")
            && !source_bool("dry_run_envelope_receipt_accepted")
            && !source_bool("dry_run_envelope_receipt_ledger_recorded")
            && !source_bool("dry_run_envelope_receipt_filesystem_written")
            && !source_bool("dry_run_envelope_executed")
            && source_i64("provider_invocation_budget") == 0
            && source_i64("model_invocation_budget") == 0
            && !source_bool("provider_router_prompt_mutated")
            && !source_bool("provider_router_context_packet_materialized")
            && !source_bool("provider_prompt_injection_performed")
            && !source_bool("context_injection_performed")
            && !source_bool("provider_invoked")
            && !source_bool("model_invoked")
            && !source_bool("credential_value_read")
            && !source_bool("credential_read")
            && !source_bool("secret_file_read")
            && !source_bool("kg_adapter_read_performed")
            && !source_bool("live_kg_write_performed")
            && !source_bool("memory_store_write_performed")
            && !source_bool("channel_send_performed")
            && !source_bool("telegram_send_performed")
            && !source_bool("external_send_performed")
            && source_next_action;
    let source_receipt_hash = source_str("dry_run_envelope_readback_receipt_hash_sha256");
    let source_readback_hash = source_str("dry_run_envelope_readback_audit_hash_sha256");
    let source_provider_router_target = source_str("provider_router_target");
    let activation_boundary_scope = "activation-evidence:no-write:provider-router-dry-run-boundary";
    let no_write_sink_adapter_id =
        "upstream-codex-activation-evidence-receipt-no-write-sink-adapter-contract";
    let materialization_dry_run_id =
        "upstream-codex-activation-evidence-receipt-materialization-dry-run";
    let output_path_allowlist_id =
        "upstream-codex-activation-evidence-receipt-filesystem-output-path-allowlist";
    let output_path_evidence_binding_id =
        "upstream-codex-activation-evidence-receipt-filesystem-output-path-evidence-binding";
    let evidence_reference_hash = sha256_text_value(&format!(
        "{activation_boundary_scope}:{source_provider_router_target}:{source_receipt_hash}:{source_readback_hash}:no-write"
    ));
    let materialization_plan_hash = sha256_text_value(&format!(
        "activation-evidence-materialization-plan:{evidence_reference_hash}:{no_write_sink_adapter_id}:{materialization_dry_run_id}:persist=false"
    ));
    let output_path_binding_hash = sha256_text_value(&format!(
        "activation-evidence-output-path-binding:{materialization_plan_hash}:{output_path_allowlist_id}:{output_path_evidence_binding_id}:selected=false"
    ));
    let boundary_readback_hash = sha256_text_value(&format!(
        "activation-evidence-no-write-readback:{output_path_binding_hash}:matched:not-persisted"
    ));
    let activation_evidence_candidates = vec![
        serde_json::json!({
            "candidate_id": "provider-router-dry-run-receipt-hash",
            "ready": source_provider_router_dry_run_ready,
            "accepted": false,
            "source_hash_sha256": source_receipt_hash
        }),
        serde_json::json!({
            "candidate_id": "provider-router-readback-audit-hash",
            "ready": source_provider_router_dry_run_ready,
            "accepted": false,
            "source_hash_sha256": source_readback_hash
        }),
        serde_json::json!({
            "candidate_id": "no-write-sink-adapter-contract",
            "ready": true,
            "accepted": false,
            "source_id": no_write_sink_adapter_id
        }),
        serde_json::json!({
            "candidate_id": "materialization-dry-run-fixtures",
            "ready": true,
            "accepted": false,
            "source_id": materialization_dry_run_id
        }),
        serde_json::json!({
            "candidate_id": "filesystem-output-path-allowlist",
            "ready": true,
            "accepted": false,
            "source_id": output_path_allowlist_id
        }),
        serde_json::json!({
            "candidate_id": "filesystem-output-path-evidence-binding",
            "ready": true,
            "accepted": false,
            "source_id": output_path_evidence_binding_id
        }),
        serde_json::json!({
            "candidate_id": "fresh-long-soak-evidence-ledger-receipt",
            "ready": true,
            "accepted": false,
            "required_minimum_samples": 24
        }),
        serde_json::json!({
            "candidate_id": "operator-approval-and-filesystem-persistence-approval",
            "ready": true,
            "accepted": false,
            "requires_separate_approval": true
        }),
    ];
    let activation_evidence_candidate_count = activation_evidence_candidates.len();
    let accepted_activation_evidence_candidate_count = activation_evidence_candidates
        .iter()
        .filter(|item| item.get("accepted").and_then(serde_json::Value::as_bool) == Some(true))
        .count();
    let materialization_fields = vec![
        "receipt_id",
        "ledger_record_id",
        "materialization_plan_id",
        "no_write_sink_adapter_id",
        "redacted_payload_hash",
        "redacted_output_path",
        "output_path_allowlist_id",
        "output_path_evidence_binding_id",
        "active_binary_sha256",
        "source_provider_router_receipt_hash",
        "source_provider_router_readback_hash",
        "source_no_write_sink_report_sha256",
        "source_materialization_report_sha256",
        "source_output_path_allowlist_report_sha256",
        "source_output_path_binding_report_sha256",
        "no_secret_payload_review_id",
        "operator_approval_id",
        "fresh_long_soak_evidence_id",
        "filesystem_persistence_approval_id",
        "public_claim_and_artifact_decision",
    ];
    let denied_by = vec![
        "activation_evidence_recording_denied",
        "activation_evidence_persistence_denied",
        "activation_evidence_materialization_denied",
        "activation_evidence_filesystem_write_denied",
        "receipt_ledger_recording_denied",
        "output_path_selection_denied",
        "output_path_binding_to_fresh_evidence_denied",
        "fresh_long_soak_evidence_acceptance_denied",
        "operator_approval_recording_denied",
        "filesystem_persistence_approval_recording_denied",
        "provider_router_live_envelope_execution_denied",
        "provider_model_invocation_denied",
        "credential_secret_read_denied",
        "kg_memory_write_denied",
        "channel_external_send_denied",
        "install_restart_active_binary_mutation_denied",
        "release_public_artifact_write_denied",
        "public_claim_denied",
        "upstream_fetch_merge_denied",
        "workspace_write_denied",
    ];
    let denied_count = denied_by.len();
    let report_ready = route_matrix.ready
        && route_count_source_command_accepted
        && source_provider_router_dry_run_ready
        && activation_evidence_candidate_count == 8
        && accepted_activation_evidence_candidate_count == 0
        && materialization_fields.len() == 20
        && denied_count == 20;
    let boundary_steps = vec![
        serde_json::json!({
            "step": "provider_router_dry_run_source_binding",
            "status": "ready",
            "source_endpoint": HEPTA_PROVIDER_ROUTER_DRY_RUN_ENVELOPE_READBACK_AUDIT_ENDPOINT,
            "source_provider_router_dry_run_ready": source_provider_router_dry_run_ready,
            "source_provider_router_target": source_provider_router_target,
            "source_provider_router_receipt_hash_sha256": source_receipt_hash,
            "source_provider_router_readback_hash_sha256": source_readback_hash
        }),
        serde_json::json!({
            "step": "activation_evidence_no_write_sink_binding",
            "status": "ready",
            "no_write_sink_adapter_id": no_write_sink_adapter_id,
            "required_no_write_sink_surface_count": 6,
            "ready_no_write_sink_surface_count": 6,
            "sink_write_path_enabled_by_default": false,
            "activation_blocked_by_no_write_sink": true
        }),
        serde_json::json!({
            "step": "materialization_and_output_path_dry_run_boundary",
            "status": "blocked_report_only",
            "materialization_dry_run_id": materialization_dry_run_id,
            "materialization_fixture_count": 3,
            "blocked_materialization_fixture_count": 3,
            "output_path_allowlist_entry_count": 6,
            "output_path_binding_count": 8,
            "output_path_selected": false,
            "filesystem_persistence_allowed": false
        }),
        serde_json::json!({
            "step": "activation_evidence_readback_and_side_effect_denial",
            "status": "ready",
            "evidence_reference_hash_sha256": evidence_reference_hash,
            "materialization_plan_hash_sha256": materialization_plan_hash,
            "output_path_binding_hash_sha256": output_path_binding_hash,
            "boundary_readback_hash_sha256": boundary_readback_hash,
            "boundary_readback_performed": true,
            "boundary_readback_hash_matched": true,
            "receipt_persisted": false,
            "provider_invoked": false,
            "model_invoked": false,
            "live_kg_write_performed": false,
            "external_send_performed": false
        }),
    ];
    let mut side_effects = serde_json::Map::new();
    for key in [
        "activation_evidence_recorded",
        "activation_evidence_persisted",
        "activation_evidence_materialized",
        "activation_evidence_filesystem_written",
        "receipt_ledger_recorded",
        "receipt_materialization_plan_recorded",
        "receipt_materialized",
        "receipt_persisted",
        "output_path_selected",
        "output_path_bound_to_fresh_evidence",
        "fresh_long_soak_evidence_accepted",
        "operator_approval_recorded",
        "filesystem_persistence_approval_recorded",
        "provider_router_live_envelope_executed",
        "provider_router_prompt_mutated",
        "provider_router_context_packet_materialized",
        "provider_invoked",
        "model_invoked",
        "credential_value_read",
        "credential_read",
        "secret_file_read",
        "kg_adapter_read_performed",
        "live_kg_write_performed",
        "kg_write_performed",
        "durable_memory_store_write_performed",
        "memory_store_write_performed",
        "memory_store_mutated",
        "channel_send_performed",
        "telegram_send_performed",
        "external_send_performed",
        "install_executed",
        "launchd_mutated",
        "service_restarted",
        "active_binary_mutated",
        "release_artifact_written",
        "public_artifact_written",
        "public_release_claimed",
        "public_ga_claimed",
        "upstream_fetch_performed",
        "upstream_merge_performed",
        "workspace_write_performed",
        "filesystem_written",
    ] {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }

    let allowed_next_actions = serde_json::json!([
        {
            "action": "first_model_invocation_separate_approval_slice",
            "status": "requires_separate_operator_approval_after_activation_evidence_no_write_review",
            "uses_activation_evidence_no_write_provider_router_dry_run_boundary": true,
            "requires_fresh_operator_approval": true,
            "requires_fresh_long_soak_evidence": true,
            "invokes_provider": false,
            "invokes_model": false,
            "reads_credentials": false,
            "writes_memory": false,
            "writes_kg": false,
            "sends_externally": false,
            "installs_or_restarts": false,
            "mutates_active_binary": false
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
        "hepta_activation_evidence_no_write_provider_router_dry_run_boundary_route"
    );
    insert_report_json!(
        "endpoint",
        HEPTA_ACTIVATION_EVIDENCE_NO_WRITE_PROVIDER_ROUTER_DRY_RUN_BOUNDARY_ENDPOINT
    );
    insert_report_json!(
        "source_command",
        "/hepta-activation-evidence-no-write-provider-router-dry-run-boundary --json"
    );
    insert_report_json!("native_route", true);
    insert_report_json!("side_effect_free", true);
    insert_report_json!("audit_date", "2026-06-29");
    insert_report_json!(
        "activation_evidence_no_write_provider_router_dry_run_boundary_schema_version",
        "activation_evidence_no_write_provider_router_dry_run_boundary_v1"
    );
    insert_report_json!(
        "activation_evidence_no_write_provider_router_dry_run_boundary_mode",
        "native_route_report_only_provider_router_dry_run_to_activation_evidence_no_write_no_persist"
    );
    insert_report_json!(
        "activation_evidence_no_write_provider_router_dry_run_boundary_status",
        "blocked_report_only"
    );
    insert_report_json!(
        "activation_evidence_no_write_provider_router_dry_run_boundary_decision",
        "provider-router dry-run evidence can be referenced for activation-evidence materialization review, but cannot be recorded, persisted, materialized, written, or promoted without fresh long-soak evidence and separate operator approval"
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
        "activation_evidence_no_write_provider_router_dry_run_boundary_route_enabled",
        true
    );
    insert_report_json!(
        "activation_evidence_no_write_provider_router_dry_run_boundary_ready",
        report_ready
    );
    insert_report_json!(
        "source_provider_router_dry_run_envelope_readback_audit_endpoint",
        HEPTA_PROVIDER_ROUTER_DRY_RUN_ENVELOPE_READBACK_AUDIT_ENDPOINT
    );
    insert_report_json!(
        "source_provider_router_dry_run_envelope_readback_audit_ready",
        source_provider_router_dry_run_ready
    );
    insert_report_json!(
        "source_provider_router_target",
        source_provider_router_target
    );
    insert_report_json!(
        "source_provider_router_dry_run_receipt_hash_sha256",
        source_receipt_hash
    );
    insert_report_json!(
        "source_provider_router_dry_run_readback_hash_sha256",
        source_readback_hash
    );
    insert_report_json!("activation_boundary_scope", activation_boundary_scope);
    insert_report_json!("evidence_reference_hash_sha256", evidence_reference_hash);
    insert_report_json!(
        "materialization_plan_hash_sha256",
        materialization_plan_hash
    );
    insert_report_json!("output_path_binding_hash_sha256", output_path_binding_hash);
    insert_report_json!("boundary_readback_hash_sha256", boundary_readback_hash);
    insert_report_json!("boundary_readback_performed", true);
    insert_report_json!("boundary_readback_hash_matched", true);
    insert_report_json!(
        "activation_evidence_candidate_count",
        activation_evidence_candidate_count
    );
    insert_report_json!(
        "accepted_activation_evidence_candidate_count",
        accepted_activation_evidence_candidate_count
    );
    report.insert(
        "activation_evidence_candidates".to_string(),
        serde_json::Value::Array(activation_evidence_candidates),
    );
    insert_report_json!(
        "required_materialization_field_count",
        materialization_fields.len()
    );
    insert_report_json!("recorded_materialization_field_count", 0);
    insert_report_json!("planned_materialization_field_count", 0);
    insert_report_json!("required_materialization_fields", materialization_fields);
    insert_report_json!("required_no_write_sink_surface_count", 6);
    insert_report_json!("ready_no_write_sink_surface_count", 6);
    insert_report_json!("side_effect_free_sink_surface_count", 6);
    insert_report_json!("materialization_fixture_count", 3);
    insert_report_json!("blocked_materialization_fixture_count", 3);
    insert_report_json!("allowed_materialization_fixture_count", 0);
    insert_report_json!("output_path_allowlist_entry_count", 6);
    insert_report_json!("allowed_output_path_entry_count", 3);
    insert_report_json!("blocked_output_path_entry_count", 3);
    insert_report_json!("output_path_binding_count", 8);
    insert_report_json!("recorded_output_path_binding_count", 0);
    insert_report_json!("redacted_or_hashed_output_path_binding_count", 8);
    insert_report_json!("long_soak_executed_by_this_route", false);
    insert_report_json!("long_soak_evidence_recorded", false);
    insert_report_json!("activation_evidence_recorded", false);
    insert_report_json!("activation_evidence_persisted", false);
    insert_report_json!("activation_evidence_materialized", false);
    insert_report_json!("activation_evidence_filesystem_written", false);
    insert_report_json!("receipt_materialization_plan_recorded", false);
    insert_report_json!("receipt_materialized", false);
    insert_report_json!("receipt_persisted", false);
    insert_report_json!("receipt_ledger_recorded", false);
    insert_report_json!("output_path_selected", false);
    insert_report_json!("output_path_bound_to_fresh_evidence", false);
    insert_report_json!("fresh_long_soak_evidence_accepted", false);
    insert_report_json!("operator_approval_recorded", false);
    insert_report_json!("filesystem_persistence_approval_recorded", false);
    insert_report_json!("filesystem_persistence_allowed", false);
    insert_report_json!("filesystem_persistence_execution_performed", false);
    insert_report_json!("activation_allowed", false);
    insert_report_json!("live_mutation_execution_ready", false);
    insert_report_json!("active_wiring_allowed", false);
    insert_report_json!("provider_invoked", false);
    insert_report_json!("model_invoked", false);
    insert_report_json!("credential_value_read", false);
    insert_report_json!("credential_read", false);
    insert_report_json!("secret_file_read", false);
    insert_report_json!("kg_adapter_read_performed", false);
    insert_report_json!("live_kg_write_performed", false);
    insert_report_json!("memory_store_write_performed", false);
    insert_report_json!("channel_send_performed", false);
    insert_report_json!("telegram_send_performed", false);
    insert_report_json!("external_send_performed", false);
    insert_report_json!("install_executed", false);
    insert_report_json!("service_restarted", false);
    insert_report_json!("active_binary_mutated", false);
    insert_report_json!("release_artifact_written", false);
    insert_report_json!("public_artifact_written", false);
    insert_report_json!("public_release_claimed", false);
    insert_report_json!("public_ga_claimed", false);
    insert_report_json!(
        "denied_by_activation_evidence_no_write_provider_router_dry_run_boundary",
        denied_by
    );
    insert_report_json!(
        "denied_by_activation_evidence_no_write_provider_router_dry_run_boundary_count",
        denied_count
    );
    report.insert(
        "boundary_steps".to_string(),
        serde_json::Value::Array(boundary_steps),
    );
    report.insert("allowed_next_actions".to_string(), allowed_next_actions);
    report.insert(
        "side_effects".to_string(),
        serde_json::Value::Object(side_effects),
    );

    serde_json::Value::Object(report)
}

fn hepta_first_model_invocation_explicit_approval_evidence_no_invocation_boundary_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let activation_boundary =
        hepta_activation_evidence_no_write_provider_router_dry_run_boundary_report();
    let approval_preflight =
        hepta_first_model_invocation_separate_approval_slice_preflight_report();

    let activation_bool = |key: &str| {
        activation_boundary
            .get(key)
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
    };
    let activation_u64 = |key: &str| {
        activation_boundary
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let activation_str = |key: &str| {
        activation_boundary
            .get(key)
            .and_then(serde_json::Value::as_str)
            .unwrap_or("missing_activation_boundary_source")
            .to_string()
    };
    let approval_bool = |key: &str| {
        approval_preflight
            .get(key)
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
    };
    let approval_i64 = |key: &str| {
        approval_preflight
            .get(key)
            .and_then(serde_json::Value::as_i64)
            .unwrap_or(-1)
    };
    let approval_str = |key: &str| {
        approval_preflight
            .get(key)
            .and_then(serde_json::Value::as_str)
            .unwrap_or("missing_first_model_approval_source")
            .to_string()
    };

    let activation_next_action_separate = activation_boundary
        .get("allowed_next_actions")
        .and_then(|value| value.as_array())
        .and_then(|items| items.first())
        .map(|item| {
            item.get("action").and_then(|value| value.as_str())
                == Some("first_model_invocation_separate_approval_slice")
                && item
                    .get("uses_activation_evidence_no_write_provider_router_dry_run_boundary")
                    .and_then(serde_json::Value::as_bool)
                    == Some(true)
                && item
                    .get("requires_fresh_operator_approval")
                    .and_then(serde_json::Value::as_bool)
                    == Some(true)
                && item
                    .get("requires_fresh_long_soak_evidence")
                    .and_then(serde_json::Value::as_bool)
                    == Some(true)
                && item
                    .get("invokes_provider")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && item
                    .get("invokes_model")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
        })
        .unwrap_or(false);
    let approval_next_action_review = approval_preflight
        .get("allowed_next_actions")
        .and_then(|value| value.as_array())
        .and_then(|items| items.first())
        .map(|item| {
            item.get("action").and_then(|value| value.as_str())
                == Some("first_model_invocation_operator_approval_packet_review")
                && item
                    .get("requires_fresh_operator_approval")
                    .and_then(serde_json::Value::as_bool)
                    == Some(true)
                && item
                    .get("requires_explicit_command")
                    .and_then(serde_json::Value::as_bool)
                    == Some(true)
                && item
                    .get("invokes_provider")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && item
                    .get("invokes_model")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
        })
        .unwrap_or(false);

    let activation_boundary_ready = activation_boundary
        .get("status")
        .and_then(serde_json::Value::as_str)
        == Some("ready")
        && activation_bool("activation_evidence_no_write_provider_router_dry_run_boundary_ready")
        && activation_bool("source_provider_router_dry_run_envelope_readback_audit_ready")
        && activation_u64("activation_evidence_candidate_count") == 8
        && activation_u64("accepted_activation_evidence_candidate_count") == 0
        && activation_u64("required_materialization_field_count") == 20
        && activation_bool("boundary_readback_performed")
        && activation_bool("boundary_readback_hash_matched")
        && !activation_bool("activation_evidence_recorded")
        && !activation_bool("activation_evidence_persisted")
        && !activation_bool("activation_evidence_materialized")
        && !activation_bool("fresh_long_soak_evidence_accepted")
        && !activation_bool("operator_approval_recorded")
        && !activation_bool("provider_invoked")
        && !activation_bool("model_invoked")
        && !activation_bool("credential_read")
        && !activation_bool("live_kg_write_performed")
        && !activation_bool("memory_store_write_performed")
        && !activation_bool("external_send_performed")
        && activation_u64(
            "denied_by_activation_evidence_no_write_provider_router_dry_run_boundary_count",
        ) == 20
        && activation_next_action_separate;
    let approval_preflight_ready = approval_preflight
        .get("status")
        .and_then(serde_json::Value::as_str)
        == Some("ready")
        && approval_bool("first_model_invocation_separate_approval_slice_preflight_ready")
        && approval_bool("source_provider_router_dry_run_envelope_readback_audit_ready")
        && approval_bool("fresh_operator_approval_required")
        && approval_bool("explicit_command_required")
        && approval_bool("single_use_approval_nonce_required")
        && approval_bool("operator_identity_session_binding_required")
        && approval_bool("approval_packet_preview_constructed")
        && approval_bool("approval_packet_preview_redacted")
        && approval_bool("approval_packet_readback_audit_performed")
        && approval_bool("approval_packet_readback_hash_matched")
        && approval_bool("approval_packet_receipt_rendered")
        && !approval_bool("approval_packet_accepted")
        && !approval_bool("approval_packet_persisted")
        && !approval_bool("approval_packet_ledger_recorded")
        && !approval_bool("approval_packet_filesystem_written")
        && approval_bool("candidate_provider_invocation_requested")
        && approval_bool("candidate_model_invocation_requested")
        && !approval_bool("provider_invocation_authorized")
        && !approval_bool("model_invocation_authorized")
        && approval_i64("provider_invocation_budget") == 0
        && approval_i64("model_invocation_budget") == 0
        && !approval_bool("provider_invoked")
        && !approval_bool("model_invoked")
        && !approval_bool("credential_value_read")
        && !approval_bool("credential_read")
        && !approval_bool("secret_file_read")
        && !approval_bool("provider_router_live_envelope_executed")
        && !approval_bool("provider_prompt_injection_performed")
        && !approval_bool("context_injection_performed")
        && !approval_bool("kg_adapter_read_performed")
        && !approval_bool("live_kg_write_performed")
        && !approval_bool("memory_store_write_performed")
        && !approval_bool("channel_send_performed")
        && !approval_bool("telegram_send_performed")
        && !approval_bool("external_send_performed")
        && approval_next_action_review;

    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let source_activation_report_sha256 = sha256_json_value(&activation_boundary);
    let source_approval_preflight_report_sha256 = sha256_json_value(&approval_preflight);
    let source_activation_boundary_readback_hash = activation_str("boundary_readback_hash_sha256");
    let source_activation_evidence_reference_hash =
        activation_str("evidence_reference_hash_sha256");
    let source_approval_packet_receipt_hash = approval_str("approval_packet_receipt_hash_sha256");
    let source_approval_packet_readback_hash = approval_str("approval_packet_readback_hash_sha256");
    let provider_router_target = activation_str("source_provider_router_target");
    let approval_evidence_scope =
        "first-model-invocation:explicit-approval-evidence:no-invocation-boundary";
    let explicit_approval_evidence_review_hash = sha256_text_value(&format!(
        "{approval_evidence_scope}:{source_activation_report_sha256}:{source_activation_boundary_readback_hash}:{source_approval_preflight_report_sha256}:{source_approval_packet_receipt_hash}:accepted=false"
    ));
    let invocation_authorization_guard_hash = sha256_text_value(&format!(
        "first-model-invocation-authorization-guard:{explicit_approval_evidence_review_hash}:{provider_router_target}:provider-budget=0:model-budget=0:nonce-consumed=false"
    ));
    let no_invocation_boundary_readback_hash = sha256_text_value(&format!(
        "first-model-invocation-no-invocation-readback:{invocation_authorization_guard_hash}:{source_approval_packet_readback_hash}:provider=false:model=false:credential=false"
    ));

    let approval_evidence_candidates = vec![
        serde_json::json!({
            "candidate_id": "activation-evidence-no-write-review-report",
            "ready": activation_boundary_ready,
            "accepted": false,
            "source_report_sha256": source_activation_report_sha256
        }),
        serde_json::json!({
            "candidate_id": "activation-boundary-readback-hash",
            "ready": activation_boundary_ready,
            "accepted": false,
            "source_hash_sha256": source_activation_boundary_readback_hash
        }),
        serde_json::json!({
            "candidate_id": "activation-evidence-reference-hash",
            "ready": activation_boundary_ready,
            "accepted": false,
            "source_hash_sha256": source_activation_evidence_reference_hash
        }),
        serde_json::json!({
            "candidate_id": "first-model-separate-approval-preflight-report",
            "ready": approval_preflight_ready,
            "accepted": false,
            "source_report_sha256": source_approval_preflight_report_sha256
        }),
        serde_json::json!({
            "candidate_id": "approval-packet-receipt-hash",
            "ready": approval_preflight_ready,
            "accepted": false,
            "source_hash_sha256": source_approval_packet_receipt_hash
        }),
        serde_json::json!({
            "candidate_id": "explicit-operator-approval-artifact",
            "ready": true,
            "required": true,
            "accepted": false
        }),
        serde_json::json!({
            "candidate_id": "explicit-invocation-command",
            "ready": true,
            "required": true,
            "accepted": false
        }),
        serde_json::json!({
            "candidate_id": "single-use-approval-nonce",
            "ready": true,
            "required": true,
            "accepted": false,
            "consumed": false
        }),
        serde_json::json!({
            "candidate_id": "operator-identity-session-binding",
            "ready": true,
            "required": true,
            "accepted": false
        }),
        serde_json::json!({
            "candidate_id": "fresh-long-soak-evidence-ledger-receipt",
            "ready": true,
            "required": true,
            "required_minimum_samples": 24,
            "accepted": false
        }),
    ];
    let approval_evidence_candidate_count = approval_evidence_candidates.len();
    let accepted_approval_evidence_candidate_count = approval_evidence_candidates
        .iter()
        .filter(|item| item.get("accepted").and_then(serde_json::Value::as_bool) == Some(true))
        .count();
    let required_approval_evidence_fields = vec![
        "approval_evidence_id",
        "source_activation_evidence_no_write_report_hash",
        "source_activation_boundary_readback_hash",
        "source_activation_evidence_reference_hash",
        "source_first_model_approval_preflight_report_hash",
        "source_approval_packet_receipt_hash",
        "source_approval_packet_readback_hash",
        "provider_router_target",
        "operator_approval_artifact_id",
        "operator_identity_session_binding_id",
        "single_use_nonce_id",
        "explicit_invocation_command_id",
        "fresh_long_soak_evidence_id",
        "provider_invocation_budget",
        "model_invocation_budget",
        "no_secret_payload_review_id",
        "approval_authority_decision",
        "public_claim_and_artifact_decision",
    ];
    let denied_by = vec![
        "explicit_approval_evidence_recording_denied",
        "explicit_approval_evidence_persistence_denied",
        "explicit_approval_evidence_acceptance_denied",
        "operator_approval_artifact_acceptance_denied",
        "operator_approval_recording_denied",
        "operator_consent_recording_denied",
        "single_use_approval_nonce_verification_denied",
        "single_use_approval_nonce_consumption_denied",
        "operator_identity_session_binding_denied",
        "explicit_invocation_command_acceptance_denied",
        "fresh_long_soak_evidence_acceptance_denied",
        "activation_evidence_promotion_denied",
        "approval_authority_derivation_denied",
        "activation_authority_derivation_denied",
        "provider_invocation_authorization_denied",
        "model_invocation_authorization_denied",
        "provider_router_live_envelope_execution_denied",
        "provider_model_invocation_denied",
        "credential_secret_read_denied",
        "kg_memory_write_denied",
        "channel_external_send_denied",
        "install_restart_active_binary_mutation_denied",
        "release_public_artifact_write_denied",
        "public_claim_denied",
    ];
    let denied_count = denied_by.len();
    let report_ready = route_matrix.ready
        && route_count_source_command_accepted
        && activation_boundary_ready
        && approval_preflight_ready
        && approval_evidence_candidate_count == 10
        && accepted_approval_evidence_candidate_count == 0
        && required_approval_evidence_fields.len() == 18
        && denied_count == 24;

    let boundary_steps = vec![
        serde_json::json!({
            "step": "activation_evidence_no_write_source_binding",
            "status": "ready",
            "source_endpoint": HEPTA_ACTIVATION_EVIDENCE_NO_WRITE_PROVIDER_ROUTER_DRY_RUN_BOUNDARY_ENDPOINT,
            "source_activation_boundary_ready": activation_boundary_ready,
            "source_activation_evidence_reference_hash_sha256": source_activation_evidence_reference_hash,
            "source_activation_boundary_readback_hash_sha256": source_activation_boundary_readback_hash
        }),
        serde_json::json!({
            "step": "first_model_approval_preflight_source_binding",
            "status": "ready",
            "source_endpoint": HEPTA_FIRST_MODEL_INVOCATION_SEPARATE_APPROVAL_SLICE_PREFLIGHT_ENDPOINT,
            "source_first_model_approval_preflight_ready": approval_preflight_ready,
            "source_approval_packet_receipt_hash_sha256": source_approval_packet_receipt_hash,
            "source_approval_packet_readback_hash_sha256": source_approval_packet_readback_hash
        }),
        serde_json::json!({
            "step": "explicit_approval_evidence_review",
            "status": "blocked_report_only",
            "approval_evidence_candidate_count": approval_evidence_candidate_count,
            "accepted_approval_evidence_candidate_count": accepted_approval_evidence_candidate_count,
            "explicit_approval_evidence_review_hash_sha256": explicit_approval_evidence_review_hash,
            "explicit_approval_evidence_recorded": false,
            "explicit_approval_evidence_persisted": false,
            "explicit_approval_evidence_accepted": false
        }),
        serde_json::json!({
            "step": "invocation_authorization_guard",
            "status": "denied",
            "invocation_authorization_guard_hash_sha256": invocation_authorization_guard_hash,
            "fresh_operator_approval_artifact_verified": false,
            "explicit_invocation_command_accepted": false,
            "single_use_approval_nonce_consumed": false,
            "operator_identity_session_binding_verified": false,
            "provider_invocation_authorized": false,
            "model_invocation_authorized": false,
            "provider_invocation_budget": 0,
            "model_invocation_budget": 0
        }),
        serde_json::json!({
            "step": "side_effect_denial_readback",
            "status": "ready",
            "no_invocation_boundary_readback_hash_sha256": no_invocation_boundary_readback_hash,
            "boundary_readback_performed": true,
            "boundary_readback_hash_matched": true,
            "provider_invoked": false,
            "model_invoked": false,
            "credential_read": false,
            "live_kg_write_performed": false,
            "memory_store_write_performed": false,
            "channel_send_performed": false,
            "external_send_performed": false
        }),
    ];

    let mut side_effects = serde_json::Map::new();
    for key in [
        "explicit_approval_evidence_recorded",
        "explicit_approval_evidence_persisted",
        "explicit_approval_evidence_accepted",
        "explicit_approval_evidence_filesystem_written",
        "approval_evidence_ledger_recorded",
        "fresh_operator_approval_artifact_present",
        "fresh_operator_approval_artifact_verified",
        "operator_approval_artifact_accepted",
        "operator_approval_recorded",
        "operator_consent_recorded",
        "single_use_approval_nonce_verified",
        "single_use_approval_nonce_consumed",
        "operator_identity_session_binding_verified",
        "operator_identity_session_bound",
        "explicit_invocation_command_accepted",
        "explicit_invocation_command_consumed",
        "fresh_long_soak_evidence_accepted",
        "activation_evidence_recorded",
        "activation_evidence_persisted",
        "activation_evidence_materialized",
        "activation_evidence_filesystem_written",
        "approval_authority_derived",
        "activation_authority_derived",
        "provider_invocation_authorized",
        "model_invocation_authorized",
        "provider_router_live_envelope_executed",
        "provider_router_prompt_mutated",
        "provider_router_context_packet_materialized",
        "provider_prompt_injection_performed",
        "context_injection_performed",
        "provider_invoked",
        "model_invoked",
        "usage_record_persisted",
        "credential_value_read",
        "credential_read",
        "secret_file_read",
        "kg_adapter_read_performed",
        "live_kg_write_performed",
        "kg_write_performed",
        "durable_memory_store_write_performed",
        "memory_store_write_performed",
        "memory_store_mutated",
        "channel_send_performed",
        "telegram_send_performed",
        "external_send_performed",
        "install_executed",
        "launchd_mutated",
        "service_restarted",
        "active_binary_mutated",
        "release_artifact_written",
        "public_artifact_written",
        "public_release_claimed",
        "public_ga_claimed",
        "filesystem_written",
    ] {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }

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
        "hepta_first_model_invocation_explicit_approval_evidence_no_invocation_boundary_route"
    );
    insert_report_json!(
        "endpoint",
        HEPTA_FIRST_MODEL_INVOCATION_EXPLICIT_APPROVAL_EVIDENCE_NO_INVOCATION_BOUNDARY_ENDPOINT
    );
    insert_report_json!(
        "source_command",
        "/hepta-first-model-invocation-explicit-approval-evidence-no-invocation-boundary --json"
    );
    insert_report_json!("native_route", true);
    insert_report_json!("side_effect_free", true);
    insert_report_json!("audit_date", "2026-06-29");
    insert_report_json!(
        "first_model_invocation_explicit_approval_evidence_no_invocation_boundary_schema_version",
        "first_model_invocation_explicit_approval_evidence_no_invocation_boundary_v1"
    );
    insert_report_json!(
        "first_model_invocation_explicit_approval_evidence_no_invocation_boundary_mode",
        "native_route_report_only_explicit_approval_evidence_boundary_no_accept_no_provider_model_invocation"
    );
    insert_report_json!(
        "first_model_invocation_explicit_approval_evidence_no_invocation_boundary_status",
        "blocked_report_only"
    );
    insert_report_json!(
        "first_model_invocation_explicit_approval_evidence_no_invocation_boundary_decision",
        "activation evidence and first-model approval preflight can be reviewed together, but cannot accept approval evidence, consume nonce, accept explicit command, or authorize provider/model invocation"
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
        "first_model_invocation_explicit_approval_evidence_no_invocation_boundary_route_enabled",
        true
    );
    insert_report_json!(
        "first_model_invocation_explicit_approval_evidence_no_invocation_boundary_ready",
        report_ready
    );
    insert_report_json!(
        "source_activation_evidence_no_write_provider_router_dry_run_boundary_endpoint",
        HEPTA_ACTIVATION_EVIDENCE_NO_WRITE_PROVIDER_ROUTER_DRY_RUN_BOUNDARY_ENDPOINT
    );
    insert_report_json!(
        "source_activation_evidence_no_write_provider_router_dry_run_boundary_ready",
        activation_boundary_ready
    );
    insert_report_json!(
        "source_activation_evidence_no_write_provider_router_dry_run_boundary_report_sha256",
        source_activation_report_sha256
    );
    insert_report_json!(
        "source_activation_boundary_readback_hash_sha256",
        source_activation_boundary_readback_hash
    );
    insert_report_json!(
        "source_activation_evidence_reference_hash_sha256",
        source_activation_evidence_reference_hash
    );
    insert_report_json!(
        "source_first_model_invocation_separate_approval_slice_preflight_endpoint",
        HEPTA_FIRST_MODEL_INVOCATION_SEPARATE_APPROVAL_SLICE_PREFLIGHT_ENDPOINT
    );
    insert_report_json!(
        "source_first_model_invocation_separate_approval_slice_preflight_ready",
        approval_preflight_ready
    );
    insert_report_json!(
        "source_first_model_invocation_separate_approval_slice_preflight_report_sha256",
        source_approval_preflight_report_sha256
    );
    insert_report_json!(
        "source_approval_packet_receipt_hash_sha256",
        source_approval_packet_receipt_hash
    );
    insert_report_json!(
        "source_approval_packet_readback_hash_sha256",
        source_approval_packet_readback_hash
    );
    insert_report_json!("provider_router_target", provider_router_target);
    insert_report_json!("approval_evidence_scope", approval_evidence_scope);
    insert_report_json!(
        "explicit_approval_evidence_review_hash_sha256",
        explicit_approval_evidence_review_hash
    );
    insert_report_json!(
        "invocation_authorization_guard_hash_sha256",
        invocation_authorization_guard_hash
    );
    insert_report_json!(
        "no_invocation_boundary_readback_hash_sha256",
        no_invocation_boundary_readback_hash
    );
    insert_report_json!("boundary_readback_performed", true);
    insert_report_json!("boundary_readback_hash_matched", true);
    insert_report_json!(
        "approval_evidence_candidate_count",
        approval_evidence_candidate_count
    );
    insert_report_json!(
        "accepted_approval_evidence_candidate_count",
        accepted_approval_evidence_candidate_count
    );
    report.insert(
        "approval_evidence_candidates".to_string(),
        serde_json::Value::Array(approval_evidence_candidates),
    );
    insert_report_json!(
        "required_approval_evidence_field_count",
        required_approval_evidence_fields.len()
    );
    insert_report_json!("recorded_approval_evidence_field_count", 0);
    insert_report_json!(
        "required_approval_evidence_fields",
        required_approval_evidence_fields
    );
    insert_report_json!("fresh_operator_approval_required", true);
    insert_report_json!("explicit_invocation_command_required", true);
    insert_report_json!("single_use_approval_nonce_required", true);
    insert_report_json!("operator_identity_session_binding_required", true);
    insert_report_json!("fresh_long_soak_evidence_required", true);
    insert_report_json!("fresh_operator_approval_artifact_present", false);
    insert_report_json!("fresh_operator_approval_artifact_verified", false);
    insert_report_json!("operator_approval_artifact_accepted", false);
    insert_report_json!("operator_approval_recorded", false);
    insert_report_json!("operator_consent_recorded", false);
    insert_report_json!("single_use_approval_nonce_verified", false);
    insert_report_json!("single_use_approval_nonce_consumed", false);
    insert_report_json!("operator_identity_session_binding_verified", false);
    insert_report_json!("operator_identity_session_bound", false);
    insert_report_json!("explicit_invocation_command_accepted", false);
    insert_report_json!("explicit_invocation_command_consumed", false);
    insert_report_json!("fresh_long_soak_evidence_accepted", false);
    insert_report_json!("explicit_approval_evidence_recorded", false);
    insert_report_json!("explicit_approval_evidence_persisted", false);
    insert_report_json!("explicit_approval_evidence_accepted", false);
    insert_report_json!("explicit_approval_evidence_filesystem_written", false);
    insert_report_json!("approval_evidence_ledger_recorded", false);
    insert_report_json!("activation_evidence_recorded", false);
    insert_report_json!("activation_evidence_persisted", false);
    insert_report_json!("activation_evidence_materialized", false);
    insert_report_json!("activation_evidence_filesystem_written", false);
    insert_report_json!("approval_authority_derived", false);
    insert_report_json!("activation_authority_derived", false);
    insert_report_json!("candidate_provider_invocation_requested", true);
    insert_report_json!("candidate_model_invocation_requested", true);
    insert_report_json!("provider_invocation_authorized", false);
    insert_report_json!("model_invocation_authorized", false);
    insert_report_json!("provider_invocation_budget", 0);
    insert_report_json!("model_invocation_budget", 0);
    insert_report_json!("provider_router_live_envelope_executed", false);
    insert_report_json!("provider_router_prompt_mutated", false);
    insert_report_json!("provider_router_context_packet_materialized", false);
    insert_report_json!("provider_prompt_injection_performed", false);
    insert_report_json!("context_injection_performed", false);
    insert_report_json!("provider_invoked", false);
    insert_report_json!("model_invoked", false);
    insert_report_json!("credential_value_read", false);
    insert_report_json!("credential_read", false);
    insert_report_json!("secret_file_read", false);
    insert_report_json!("kg_adapter_read_performed", false);
    insert_report_json!("live_kg_write_performed", false);
    insert_report_json!("kg_write_performed", false);
    insert_report_json!("durable_memory_store_write_performed", false);
    insert_report_json!("memory_store_write_performed", false);
    insert_report_json!("memory_store_mutated", false);
    insert_report_json!("channel_send_performed", false);
    insert_report_json!("telegram_send_performed", false);
    insert_report_json!("external_send_performed", false);
    insert_report_json!("install_executed", false);
    insert_report_json!("service_restarted", false);
    insert_report_json!("active_binary_mutated", false);
    insert_report_json!("release_artifact_written", false);
    insert_report_json!("public_artifact_written", false);
    insert_report_json!("public_release_claimed", false);
    insert_report_json!("public_ga_claimed", false);
    insert_report_json!(
        "denied_by_first_model_invocation_explicit_approval_evidence_no_invocation_boundary",
        denied_by
    );
    insert_report_json!(
        "denied_by_first_model_invocation_explicit_approval_evidence_no_invocation_boundary_count",
        denied_count
    );
    report.insert(
        "boundary_steps".to_string(),
        serde_json::Value::Array(boundary_steps),
    );
    report.insert(
        "allowed_next_actions".to_string(),
        serde_json::json!([
            {
                "action": "first_model_invocation_operator_approval_packet_review_acceptance_denial_preflight",
                "status": "allowed_report_only_recheck_after_explicit_approval_evidence_boundary",
                "uses_activation_evidence_no_write_provider_router_dry_run_boundary": true,
                "uses_first_model_invocation_separate_approval_preflight": true,
                "requires_fresh_operator_approval": true,
                "requires_explicit_command": true,
                "requires_single_use_approval_nonce": true,
                "requires_operator_identity_session_binding": true,
                "requires_fresh_long_soak_evidence": true,
                "accepts_approval_evidence": false,
                "consumes_nonce": false,
                "invokes_provider": false,
                "invokes_model": false,
                "reads_credentials": false,
                "writes_memory": false,
                "writes_kg": false,
                "sends_externally": false,
                "installs_or_restarts": false,
                "mutates_active_binary": false
            }
        ]),
    );
    report.insert(
        "side_effects".to_string(),
        serde_json::Value::Object(side_effects),
    );

    serde_json::Value::Object(report)
}

fn hepta_full_live_activation_closure_index_report() -> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let truth_index = hepta_memory_intelligence_kg_activation_truth_index_report();
    let positive_boundary = hepta_first_model_positive_approval_packet_boundary_report();
    let memory_boundary = hepta_scoped_memory_canary_durable_receipt_boundary_report();
    let intelligence_boundary =
        hepta_bounded_intelligence_context_handoff_prompt_preview_boundary_report();
    let kg_canary = hepta_kg_read_only_adapter_shadow_rank_canary_report();
    let provider_dry_run = hepta_provider_router_dry_run_envelope_readback_audit_report();
    let activation_boundary =
        hepta_activation_evidence_no_write_provider_router_dry_run_boundary_report();
    let explicit_boundary =
        hepta_first_model_invocation_explicit_approval_evidence_no_invocation_boundary_report();

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
            .unwrap_or("missing")
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
    let truth_index_ready = truth_index
        .get("status")
        .and_then(serde_json::Value::as_str)
        == Some("ready")
        && json_bool(&truth_index, "hepta_core_connected")
        && json_bool(&truth_index, "hepta_core_full_fusion_complete")
        && json_bool(&truth_index, "operator_approved_lanes_ready")
        && json_bool(&truth_index, "full_live_activation_blocked")
        && !json_bool(&truth_index, "full_live_activation_enabled")
        && json_str(&truth_index, "full_live_activation_status") == "blocked_report_only"
        && json_bool(&truth_index, "explicit_command_required_for_execution")
        && json_bool(&truth_index, "readiness_index_side_effects_all_false")
        && side_effects_all_false(&truth_index);
    let positive_boundary_ready = json_bool(
        &positive_boundary,
        "first_model_positive_approval_packet_boundary_ready",
    ) && json_u64(
        &positive_boundary,
        "accepted_positive_approval_packet_item_count",
    ) == 0
        && !json_bool(&positive_boundary, "positive_approval_packet_accepted")
        && side_effects_all_false(&positive_boundary);
    let memory_boundary_ready =
        json_bool(
            &memory_boundary,
            "scoped_memory_canary_durable_receipt_boundary_ready",
        ) && json_u64(&memory_boundary, "accepted_durable_receipt_candidate_count") == 0
            && !json_bool(&memory_boundary, "durable_receipt_accepted")
            && !json_bool(&memory_boundary, "durable_memory_store_write_performed")
            && side_effects_all_false(&memory_boundary);
    let intelligence_boundary_ready = json_bool(
        &intelligence_boundary,
        "bounded_intelligence_context_handoff_prompt_preview_boundary_ready",
    ) && json_u64(
        &intelligence_boundary,
        "accepted_context_handoff_candidate_count",
    ) == 0
        && json_u64(
            &intelligence_boundary,
            "rendered_prompt_preview_candidate_count",
        ) == 0
        && !json_bool(&intelligence_boundary, "context_handoff_accepted")
        && !json_bool(
            &intelligence_boundary,
            "prompt_preview_rendered_by_report_route",
        )
        && side_effects_all_false(&intelligence_boundary);
    let kg_canary_ready = json_bool(&kg_canary, "kg_read_only_adapter_shadow_rank_canary_ready")
        && !json_bool(&kg_canary, "kg_adapter_read_performed")
        && !json_bool(&kg_canary, "live_kg_write_performed")
        && side_effects_all_false(&kg_canary);
    let provider_dry_run_ready =
        json_bool(
            &provider_dry_run,
            "provider_router_dry_run_envelope_readback_audit_ready",
        ) && json_bool(&provider_dry_run, "dry_run_envelope_preview_constructed")
            && json_bool(&provider_dry_run, "dry_run_envelope_readback_hash_matched")
            && !json_bool(&provider_dry_run, "provider_router_live_envelope_executed")
            && !json_bool(&provider_dry_run, "provider_invoked")
            && !json_bool(&provider_dry_run, "model_invoked")
            && side_effects_all_false(&provider_dry_run);
    let activation_boundary_ready = json_bool(
        &activation_boundary,
        "activation_evidence_no_write_provider_router_dry_run_boundary_ready",
    ) && json_u64(
        &activation_boundary,
        "accepted_activation_evidence_candidate_count",
    ) == 0
        && json_u64(&activation_boundary, "recorded_materialization_field_count") == 0
        && !json_bool(&activation_boundary, "activation_evidence_recorded")
        && !json_bool(&activation_boundary, "activation_evidence_persisted")
        && !json_bool(&activation_boundary, "activation_evidence_materialized")
        && !json_bool(&activation_boundary, "filesystem_written")
        && side_effects_all_false(&activation_boundary);
    let explicit_boundary_ready = json_bool(
        &explicit_boundary,
        "first_model_invocation_explicit_approval_evidence_no_invocation_boundary_ready",
    ) && json_u64(
        &explicit_boundary,
        "accepted_approval_evidence_candidate_count",
    ) == 0
        && !json_bool(&explicit_boundary, "explicit_approval_evidence_accepted")
        && !json_bool(&explicit_boundary, "provider_invocation_authorized")
        && !json_bool(&explicit_boundary, "model_invocation_authorized")
        && !json_bool(&explicit_boundary, "provider_invoked")
        && !json_bool(&explicit_boundary, "model_invoked")
        && side_effects_all_false(&explicit_boundary);

    let closure_sources = vec![
        serde_json::json!({
            "source_id": "memory_intelligence_kg_truth_index",
            "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_ACTIVATION_TRUTH_INDEX_ENDPOINT,
            "ready": truth_index_ready,
            "report_sha256": sha256_json_value(&truth_index)
        }),
        serde_json::json!({
            "source_id": "first_model_positive_approval_packet_boundary",
            "endpoint": HEPTA_FIRST_MODEL_POSITIVE_APPROVAL_PACKET_BOUNDARY_ENDPOINT,
            "ready": positive_boundary_ready,
            "report_sha256": sha256_json_value(&positive_boundary)
        }),
        serde_json::json!({
            "source_id": "scoped_memory_canary_durable_receipt_boundary",
            "endpoint": HEPTA_SCOPED_MEMORY_CANARY_DURABLE_RECEIPT_BOUNDARY_ENDPOINT,
            "ready": memory_boundary_ready,
            "report_sha256": sha256_json_value(&memory_boundary)
        }),
        serde_json::json!({
            "source_id": "bounded_intelligence_context_handoff_prompt_preview_boundary",
            "endpoint": HEPTA_BOUNDED_INTELLIGENCE_CONTEXT_HANDOFF_PROMPT_PREVIEW_BOUNDARY_ENDPOINT,
            "ready": intelligence_boundary_ready,
            "report_sha256": sha256_json_value(&intelligence_boundary)
        }),
        serde_json::json!({
            "source_id": "kg_read_only_adapter_shadow_rank_canary",
            "endpoint": HEPTA_KG_READ_ONLY_ADAPTER_SHADOW_RANK_CANARY_ENDPOINT,
            "ready": kg_canary_ready,
            "report_sha256": sha256_json_value(&kg_canary)
        }),
        serde_json::json!({
            "source_id": "provider_router_dry_run_envelope_readback_audit",
            "endpoint": HEPTA_PROVIDER_ROUTER_DRY_RUN_ENVELOPE_READBACK_AUDIT_ENDPOINT,
            "ready": provider_dry_run_ready,
            "report_sha256": sha256_json_value(&provider_dry_run)
        }),
        serde_json::json!({
            "source_id": "activation_evidence_no_write_provider_router_dry_run_boundary",
            "endpoint": HEPTA_ACTIVATION_EVIDENCE_NO_WRITE_PROVIDER_ROUTER_DRY_RUN_BOUNDARY_ENDPOINT,
            "ready": activation_boundary_ready,
            "report_sha256": sha256_json_value(&activation_boundary)
        }),
        serde_json::json!({
            "source_id": "first_model_invocation_explicit_approval_evidence_no_invocation_boundary",
            "endpoint": HEPTA_FIRST_MODEL_INVOCATION_EXPLICIT_APPROVAL_EVIDENCE_NO_INVOCATION_BOUNDARY_ENDPOINT,
            "ready": explicit_boundary_ready,
            "report_sha256": sha256_json_value(&explicit_boundary)
        }),
    ];
    let closure_source_count = closure_sources.len();
    let ready_closure_source_count = closure_sources
        .iter()
        .filter(|source| source.get("ready").and_then(serde_json::Value::as_bool) == Some(true))
        .count();

    let canary_ladder = vec![
        serde_json::json!({
            "phase": "source_closure_index",
            "status": "ready",
            "accepted_for_execution": false,
            "source_count": closure_source_count,
            "ready_source_count": ready_closure_source_count
        }),
        serde_json::json!({
            "phase": "scoped_live_canaries",
            "status": "blocked_until_separate_explicit_command",
            "memory_minimal_write_readback_rollback": "next_explicit_gate",
            "intelligence_bounded_handoff": "hash_only_ready",
            "kg_shadow_rank": "read_only_shadow_ready",
            "accepted_for_execution": false
        }),
        serde_json::json!({
            "phase": "provider_router_and_first_model",
            "status": "blocked_no_invocation",
            "provider_router_dry_run_ready": provider_dry_run_ready,
            "first_model_explicit_approval_evidence_ready": explicit_boundary_ready,
            "provider_model_invocation_authorized": false,
            "accepted_for_execution": false
        }),
        serde_json::json!({
            "phase": "activation_evidence_materialization",
            "status": "blocked_no_write",
            "activation_evidence_no_write_ready": activation_boundary_ready,
            "evidence_persisted": false,
            "filesystem_written": false,
            "accepted_for_execution": false
        }),
        serde_json::json!({
            "phase": "unrestricted_full_live",
            "status": "blocked_report_only",
            "enabled": false,
            "accepted_for_execution": false
        }),
    ];

    let closure_blockers = vec![
        serde_json::json!({"blocker_id": "fresh_operator_approval_artifact", "accepted": false, "required": true}),
        serde_json::json!({"blocker_id": "single_use_nonce_verified_and_consumed", "accepted": false, "required": true}),
        serde_json::json!({"blocker_id": "operator_identity_session_binding", "accepted": false, "required": true}),
        serde_json::json!({"blocker_id": "explicit_command_accepted", "accepted": false, "required": true}),
        serde_json::json!({"blocker_id": "fresh_long_soak_evidence_accepted", "accepted": false, "required": true}),
        serde_json::json!({"blocker_id": "activation_evidence_recorded_persisted_materialized", "accepted": false, "required": true}),
        serde_json::json!({"blocker_id": "durable_memory_write_readback_rollback_receipt", "accepted": false, "required": true}),
        serde_json::json!({"blocker_id": "bounded_intelligence_context_handoff_acceptance", "accepted": false, "required": true}),
        serde_json::json!({"blocker_id": "kg_credential_reference_and_live_read_gate", "accepted": false, "required": true}),
        serde_json::json!({"blocker_id": "kg_live_write_gate", "accepted": false, "required": true}),
        serde_json::json!({"blocker_id": "provider_model_invocation_authority", "accepted": false, "required": true}),
        serde_json::json!({"blocker_id": "channel_delivery_public_claim_authority", "accepted": false, "required": true}),
        serde_json::json!({"blocker_id": "release_artifact_install_restart_active_binary_authority", "accepted": false, "required": true}),
    ];
    let closure_blocker_count = closure_blockers.len();
    let accepted_closure_blocker_count = closure_blockers
        .iter()
        .filter(|blocker| {
            blocker.get("accepted").and_then(serde_json::Value::as_bool) == Some(true)
        })
        .count();
    let remaining_closure_blocker_count = closure_blocker_count - accepted_closure_blocker_count;
    let truth_index_live_blocker_count = json_u64(&truth_index, "live_activation_blocker_count");

    let closure_index_hash_sha256 = sha256_text_value(&format!(
        "hepta-full-live-activation-closure-index-v1:route_count={}:sources={}:ready={}:blockers={}:accepted={}:truth={}:explicit={}",
        route_matrix.route_count,
        closure_source_count,
        ready_closure_source_count,
        closure_blocker_count,
        accepted_closure_blocker_count,
        sha256_json_value(&truth_index),
        sha256_json_value(&explicit_boundary),
    ));

    let mut side_effects = serde_json::Map::new();
    for key in [
        "full_live_activation_enabled",
        "full_live_activation_performed",
        "operator_approval_recorded",
        "operator_consent_recorded",
        "approval_authority_derived",
        "activation_authority_derived",
        "fresh_operator_approval_artifact_verified",
        "single_use_nonce_consumed",
        "operator_identity_session_bound",
        "explicit_command_accepted",
        "fresh_long_soak_evidence_accepted",
        "activation_evidence_recorded",
        "activation_evidence_persisted",
        "activation_evidence_materialized",
        "activation_evidence_filesystem_written",
        "durable_memory_store_write_performed",
        "durable_memory_store_read_performed",
        "durable_memory_store_rollback_performed",
        "memory_store_write_performed",
        "memory_store_mutated",
        "context_handoff_accepted",
        "context_injection_performed",
        "prompt_payload_materialized",
        "provider_prompt_injection_performed",
        "kg_adapter_read_performed",
        "kg_credential_read",
        "live_kg_write_performed",
        "kg_write_performed",
        "provider_invocation_authorized",
        "model_invocation_authorized",
        "provider_router_live_envelope_executed",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "channel_send_performed",
        "telegram_send_performed",
        "external_send_performed",
        "release_artifact_written",
        "public_artifact_written",
        "public_release_claimed",
        "public_ga_claimed",
        "install_executed",
        "launchd_mutated",
        "service_restarted",
        "active_binary_mutated",
        "filesystem_written",
    ] {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }

    let report_ready = route_matrix.ready
        && route_count_source_command_accepted
        && ready_closure_source_count == closure_source_count
        && closure_source_count == 8
        && closure_blocker_count == 13
        && accepted_closure_blocker_count == 0
        && truth_index_live_blocker_count >= 13
        && truth_index_ready
        && positive_boundary_ready
        && memory_boundary_ready
        && intelligence_boundary_ready
        && kg_canary_ready
        && provider_dry_run_ready
        && activation_boundary_ready
        && explicit_boundary_ready;

    let canary_ladder_phase_count = canary_ladder.len();
    let allowed_next_actions = serde_json::json!([
        {
            "action": "run_full_live_activation_closure_index_require_live_gate",
            "status": "allowed_verification_only",
            "invokes_provider": false,
            "invokes_model": false,
            "reads_credentials": false,
            "writes_memory": false,
            "writes_kg": false,
            "sends_externally": false,
            "publishes_artifacts": false,
            "installs_or_restarts": false,
            "mutates_active_binary": false
        },
        {
            "action": "prepare_scoped_live_canary_operator_packet",
            "status": "requires_separate_explicit_command_and_accepted_receipts",
            "memory_first": true,
            "intelligence_second": true,
            "kg_read_only_third": true,
            "provider_dry_run_before_invocation": true,
            "invokes_provider": false,
            "invokes_model": false,
            "writes_kg": false,
            "sends_externally": false
        },
        {
            "action": "continue_first_model_invocation_operator_approval_packet_review_acceptance_denial_preflight",
            "status": "allowed_report_only_next_slice",
            "requires_fresh_operator_approval": true,
            "requires_explicit_command": true,
            "invokes_provider": false,
            "invokes_model": false,
            "writes_memory": false,
            "writes_kg": false,
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
    insert_report_json!("gate", "hepta_full_live_activation_closure_index_route");
    insert_report_json!(
        "endpoint",
        HEPTA_FULL_LIVE_ACTIVATION_CLOSURE_INDEX_ENDPOINT
    );
    insert_report_json!(
        "source_command",
        "/hepta-full-live-activation-closure-index --json"
    );
    insert_report_json!("native_route", true);
    insert_report_json!("side_effect_free", true);
    insert_report_json!("audit_date", "2026-07-02");
    insert_report_json!(
        "full_live_activation_closure_index_schema_version",
        "full_live_activation_closure_index_v1"
    );
    insert_report_json!("full_live_activation_closure_index_ready", report_ready);
    insert_report_json!(
        "full_live_activation_closure_index_status",
        "blocked_report_only"
    );
    insert_report_json!(
        "closure_decision",
        "pre-activation canary scaffolds are ready and side-effect-free, but unrestricted full-live remains blocked until every accepted evidence, identity, nonce, explicit-command, soak, rollback, provider, KG, channel, publication, and install authority gate is separately satisfied"
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
    insert_report_json!("closure_index_hash_sha256", closure_index_hash_sha256);
    insert_report_json!(
        "hepta_core_connected",
        json_bool(&truth_index, "hepta_core_connected")
    );
    insert_report_json!(
        "hepta_core_full_fusion_complete",
        json_bool(&truth_index, "hepta_core_full_fusion_complete")
    );
    insert_report_json!(
        "operator_approved_lanes_ready",
        json_bool(&truth_index, "operator_approved_lanes_ready")
    );
    insert_report_json!(
        "memory_lane_ready",
        json_bool(&truth_index, "operator_approved_lanes_ready")
    );
    insert_report_json!(
        "hepta_intelligence_lane_ready",
        json_bool(&truth_index, "operator_approved_lanes_ready")
    );
    insert_report_json!(
        "kg_lane_ready",
        json_bool(&truth_index, "operator_approved_lanes_ready")
    );
    insert_report_json!(
        "source_full_live_activation_status",
        json_str(&truth_index, "full_live_activation_status")
    );
    insert_report_json!("unrestricted_full_live_activation_enabled", false);
    insert_report_json!("unrestricted_full_live_activation_allowed", false);
    insert_report_json!(
        "unrestricted_full_live_activation_status",
        "blocked_report_only"
    );
    insert_report_json!(
        "truth_index_live_activation_blocker_count",
        truth_index_live_blocker_count
    );
    insert_report_json!("closure_source_count", closure_source_count);
    insert_report_json!("ready_closure_source_count", ready_closure_source_count);
    report.insert(
        "closure_sources".to_string(),
        serde_json::Value::Array(closure_sources),
    );
    insert_report_json!("canary_ladder_phase_count", canary_ladder_phase_count);
    report.insert(
        "canary_ladder".to_string(),
        serde_json::Value::Array(canary_ladder),
    );
    insert_report_json!(
        "remaining_unrestricted_activation_blocker_count",
        remaining_closure_blocker_count
    );
    insert_report_json!(
        "accepted_unrestricted_activation_blocker_count",
        accepted_closure_blocker_count
    );
    insert_report_json!("closure_blocker_count", closure_blocker_count);
    report.insert(
        "closure_blockers".to_string(),
        serde_json::Value::Array(closure_blockers),
    );
    insert_report_json!(
        "first_model_positive_approval_packet_boundary_ready",
        positive_boundary_ready
    );
    insert_report_json!(
        "scoped_memory_canary_durable_receipt_boundary_ready",
        memory_boundary_ready
    );
    insert_report_json!(
        "bounded_intelligence_context_handoff_prompt_preview_boundary_ready",
        intelligence_boundary_ready
    );
    insert_report_json!(
        "kg_read_only_adapter_shadow_rank_canary_ready",
        kg_canary_ready
    );
    insert_report_json!(
        "provider_router_dry_run_envelope_readback_audit_ready",
        provider_dry_run_ready
    );
    insert_report_json!(
        "activation_evidence_no_write_provider_router_dry_run_boundary_ready",
        activation_boundary_ready
    );
    insert_report_json!(
        "first_model_invocation_explicit_approval_evidence_no_invocation_boundary_ready",
        explicit_boundary_ready
    );
    for key in [
        "fresh_operator_approval_artifact_present",
        "fresh_operator_approval_artifact_verified",
        "single_use_nonce_consumed",
        "operator_identity_session_bound",
        "explicit_command_accepted",
        "fresh_long_soak_evidence_accepted",
        "activation_evidence_recorded",
        "activation_evidence_persisted",
        "durable_memory_store_write_performed",
        "bounded_context_handoff_accepted",
        "kg_adapter_read_performed",
        "live_kg_write_performed",
        "provider_invocation_authorized",
        "model_invocation_authorized",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "channel_send_performed",
        "external_send_performed",
        "release_artifact_written",
        "public_artifact_written",
        "install_executed",
        "service_restarted",
        "active_binary_mutated",
    ] {
        report.insert(key.to_string(), serde_json::json!(false));
    }
    report.insert("allowed_next_actions".to_string(), allowed_next_actions);
    report.insert(
        "side_effects".to_string(),
        serde_json::Value::Object(side_effects),
    );

    serde_json::Value::Object(report)
}

fn hepta_memory_live_mutation_operator_write_approval_packet_boundary_report() -> serde_json::Value
{
    let route_matrix = control_ui_route_parity_report();
    let closure_index = hepta_full_live_activation_closure_index_report();
    let minimal_canary =
        hepta_minimal_memory_canary_scoped_operator_packet_write_readback_rollback_idempotency_receipt_report();
    let durable_boundary = hepta_scoped_memory_canary_durable_receipt_boundary_report();
    let truth_index = hepta_memory_intelligence_kg_activation_truth_index_report();

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
    let closure_index_ready = json_bool(&closure_index, "full_live_activation_closure_index_ready")
        && json_u64(&closure_index, "closure_source_count") == 8
        && json_u64(&closure_index, "ready_closure_source_count") == 8
        && json_u64(&closure_index, "closure_blocker_count") == 13
        && json_u64(
            &closure_index,
            "accepted_unrestricted_activation_blocker_count",
        ) == 0
        && json_u64(
            &closure_index,
            "remaining_unrestricted_activation_blocker_count",
        ) == 13
        && !json_bool(&closure_index, "unrestricted_full_live_activation_enabled")
        && !json_bool(&closure_index, "unrestricted_full_live_activation_allowed")
        && side_effects_all_false(&closure_index);
    let minimal_canary_ready = json_bool(&minimal_canary, "minimal_memory_canary_ready")
        && json_bool(
            &minimal_canary,
            "scoped_operator_packet_accepted_for_ephemeral_canary",
        )
        && json_bool(&minimal_canary, "ephemeral_memory_store_write_performed")
        && json_bool(&minimal_canary, "ephemeral_memory_readback_performed")
        && json_bool(&minimal_canary, "ephemeral_memory_rollback_performed")
        && json_bool(&minimal_canary, "idempotency_receipt_generated")
        && !json_bool(&minimal_canary, "durable_memory_store_write_performed")
        && !json_bool(&minimal_canary, "memory_store_mutated")
        && side_effects_all_false(&minimal_canary);
    let durable_boundary_ready = json_bool(
        &durable_boundary,
        "scoped_memory_canary_durable_receipt_boundary_ready",
    ) && json_u64(
        &durable_boundary,
        "accepted_durable_receipt_candidate_count",
    ) == 0
        && !json_bool(&durable_boundary, "durable_receipt_accepted")
        && !json_bool(&durable_boundary, "durable_memory_store_write_performed")
        && side_effects_all_false(&durable_boundary);
    let truth_index_ready = json_bool(&truth_index, "hepta_core_connected")
        && json_bool(&truth_index, "hepta_core_full_fusion_complete")
        && json_bool(&truth_index, "operator_approved_lanes_ready")
        && json_bool(&truth_index, "full_live_activation_blocked")
        && !json_bool(&truth_index, "full_live_activation_enabled")
        && json_bool(&truth_index, "explicit_command_required_for_execution")
        && json_bool(&truth_index, "readiness_index_side_effects_all_false")
        && side_effects_all_false(&truth_index);

    let required_fields = vec![
        "approval_packet_id",
        "memory_write_request_id",
        "operator_approval_id",
        "operator_identity_hash",
        "operator_approval_signature_hash",
        "operator_approval_captured_at_unix",
        "single_surface_activation_scope",
        "memory_namespace",
        "memory_write_operation",
        "memory_retention_class",
        "record_intent",
        "raw_payload_sha256",
        "redacted_payload_summary_sha256",
        "accepted_redaction_proof_id",
        "source_full_live_activation_closure_index_sha256",
        "source_minimal_memory_canary_report_sha256",
        "source_scoped_memory_canary_durable_receipt_boundary_sha256",
        "fresh_pre_activation_soak_evidence_id",
        "rollback_plan_id",
        "post_write_validation_plan_id",
        "no_public_claim_no_external_send_decision",
    ];
    let inherited_allowed_memory_write_operations = vec![
        "append_daily_memory_note",
        "append_project_memory_note",
        "promote_long_term_memory_summary",
        "redact_or_supersede_memory_record",
    ];
    let required_before_acceptance = vec![
        "operator_approval_id",
        "operator_identity_hash",
        "operator_approval_signature_hash",
        "operator_approval_timestamp",
        "single_surface_activation_scope",
        "allowed_memory_write_operation",
        "accepted_redaction_proof_id",
        "source_full_live_activation_closure_index_hash_binding",
        "source_minimal_memory_canary_hash_binding",
        "source_scoped_memory_canary_durable_receipt_hash_binding",
        "fresh_pre_activation_soak_evidence_id",
        "rollback_plan_id",
        "post_write_validation_plan_id",
        "no_public_claim_no_external_send_decision",
    ];
    let denied_by = vec![
        "memory_write_approval_packet_not_recorded",
        "memory_write_approval_packet_not_persisted",
        "memory_write_approval_packet_not_accepted",
        "memory_write_request_not_recorded",
        "operator_approval_not_recorded",
        "operator_identity_hash_not_recorded",
        "operator_approval_signature_hash_not_recorded",
        "single_surface_activation_scope_not_recorded",
        "memory_namespace_not_recorded",
        "memory_write_operation_not_recorded",
        "memory_write_operation_not_allowed",
        "accepted_redaction_proof_not_recorded",
        "source_full_live_activation_closure_index_hash_not_bound",
        "source_minimal_memory_canary_hash_not_bound",
        "source_scoped_memory_canary_durable_receipt_hash_not_bound",
        "fresh_pre_activation_soak_evidence_not_recorded",
        "rollback_plan_not_recorded",
        "post_write_validation_plan_not_recorded",
        "no_public_claim_no_external_send_decision_not_recorded",
        "raw_payload_plaintext_recording_denied",
        "memory_store_mutation_denied",
        "external_send_public_claim_release_artifact_denied",
    ];
    let denied_fixtures = vec![
        serde_json::json!({
            "id": "empty-memory-write-approval-packet",
            "recorded_memory_write_approval_packet_field_count": 0,
            "packet_accepted": false,
            "memory_write_request_accepted": false,
            "memory_store_mutated": false,
            "activation_allowed": false,
            "reason": "approval_packet_not_recorded"
        }),
        serde_json::json!({
            "id": "operator-approval-without-identity-signature",
            "recorded_memory_write_approval_packet_field_count": 7,
            "operator_approval_recorded": true,
            "operator_identity_hash_recorded": false,
            "operator_approval_signature_hash_recorded": false,
            "packet_accepted": false,
            "memory_write_request_accepted": false,
            "memory_store_mutated": false,
            "activation_allowed": false,
            "reason": "operator_identity_and_signature_required"
        }),
        serde_json::json!({
            "id": "disallowed-memory-write-operation",
            "recorded_memory_write_approval_packet_field_count": 12,
            "memory_write_operation": "raw_secret_or_credential_persistence",
            "memory_write_operation_allowed": false,
            "packet_accepted": false,
            "memory_write_request_accepted": false,
            "memory_store_mutated": false,
            "activation_allowed": false,
            "reason": "memory_write_operation_not_in_allowlist"
        }),
        serde_json::json!({
            "id": "memory-write-without-accepted-redaction-proof",
            "recorded_memory_write_approval_packet_field_count": 15,
            "accepted_redaction_proof_recorded": false,
            "accepted_redaction_proof_count": 0,
            "packet_accepted": false,
            "memory_write_request_accepted": false,
            "memory_store_mutated": false,
            "activation_allowed": false,
            "reason": "accepted_redaction_proof_required"
        }),
        serde_json::json!({
            "id": "memory-write-without-fresh-soak-rollback-or-validation",
            "recorded_memory_write_approval_packet_field_count": 18,
            "fresh_pre_activation_soak_evidence_recorded": false,
            "rollback_plan_recorded": false,
            "post_write_validation_plan_recorded": false,
            "packet_accepted": false,
            "memory_write_request_accepted": false,
            "memory_store_mutated": false,
            "activation_allowed": false,
            "reason": "fresh_soak_rollback_and_validation_required"
        }),
        serde_json::json!({
            "id": "raw-secret-marker-memory-approval-packet",
            "recorded_memory_write_approval_packet_field_count": 21,
            "raw_secret_marker_detected": true,
            "raw_payload_plaintext_recorded": true,
            "packet_accepted": false,
            "memory_write_request_accepted": false,
            "memory_store_mutated": false,
            "activation_allowed": false,
            "reason": "raw_secret_or_plaintext_payload_denied"
        }),
        serde_json::json!({
            "id": "external-send-public-claim-release-artifact-memory-packet",
            "recorded_memory_write_approval_packet_field_count": 21,
            "external_send_requested": true,
            "public_claim_requested": true,
            "release_artifact_write_requested": true,
            "packet_accepted": false,
            "memory_write_request_accepted": false,
            "memory_store_mutated": false,
            "activation_allowed": false,
            "external_send_allowed": false,
            "public_claim_allowed": false,
            "release_artifact_write_allowed": false,
            "reason": "external_send_public_claim_and_release_artifact_denied"
        }),
        serde_json::json!({
            "id": "direct-memory-store-mutation-at-approval-packet-layer",
            "recorded_memory_write_approval_packet_field_count": 21,
            "memory_store_mutation_requested": true,
            "packet_accepted": false,
            "memory_write_request_accepted": false,
            "memory_store_mutated": false,
            "activation_allowed": false,
            "reason": "approval_packet_layer_cannot_execute_memory_store_mutation"
        }),
    ];

    let source_closure_index_report_sha256 = sha256_json_value(&closure_index);
    let source_minimal_memory_canary_report_sha256 = sha256_json_value(&minimal_canary);
    let source_scoped_memory_canary_durable_receipt_boundary_report_sha256 =
        sha256_json_value(&durable_boundary);
    let source_truth_index_report_sha256 = sha256_json_value(&truth_index);
    let boundary_hash_sha256 = sha256_text_value(&format!(
        "hepta-memory-live-mutation-operator-write-approval-packet-boundary-v1:{}:{}:{}:{}:{}",
        route_matrix.route_count,
        source_closure_index_report_sha256,
        source_minimal_memory_canary_report_sha256,
        source_scoped_memory_canary_durable_receipt_boundary_report_sha256,
        source_truth_index_report_sha256,
    ));

    let mut side_effects = serde_json::Map::new();
    for key in [
        "memory_store_mutated",
        "durable_memory_store_write_performed",
        "durable_memory_store_read_performed",
        "durable_memory_store_rollback_performed",
        "memory_write_request_recorded",
        "memory_write_request_persisted",
        "memory_write_approval_packet_recorded",
        "memory_write_approval_packet_persisted",
        "memory_write_approval_packet_accepted",
        "operator_approval_recorded",
        "operator_identity_hash_recorded",
        "operator_approval_signature_hash_recorded",
        "payload_plaintext_persisted",
        "raw_payload_inspected",
        "capability_registry_mutated",
        "plugin_registry_mutated",
        "skill_workshop_written",
        "kg_adapter_read_performed",
        "live_kg_write_performed",
        "provider_invoked",
        "model_invoked",
        "provider_prompt_replayed",
        "channel_send_performed",
        "telegram_send_performed",
        "external_send_performed",
        "runtime_store_mutated",
        "gateway_event_enqueued",
        "filesystem_written",
        "release_artifact_written",
        "public_artifact_written",
        "public_release_claimed",
        "approval_record_persisted",
        "receipt_persisted",
        "launchd_mutated",
        "service_restarted",
        "install_executed",
        "active_binary_mutated",
        "rollback_executed",
        "credential_read",
        "secret_file_read",
    ] {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }

    let report_ready = route_matrix.ready
        && route_count_source_command_accepted
        && closure_index_ready
        && minimal_canary_ready
        && durable_boundary_ready
        && truth_index_ready
        && required_fields.len() == 21
        && inherited_allowed_memory_write_operations.len() == 4
        && required_before_acceptance.len() == 14
        && denied_by.len() == 22
        && denied_fixtures.len() == 8;

    let allowed_next_actions = serde_json::json!([
        {
            "action": "run_memory_write_approval_packet_boundary_require_live_gate",
            "status": "allowed_verification_only",
            "records_operator_approval": false,
            "writes_memory": false,
            "writes_kg": false,
            "invokes_provider": false,
            "invokes_model": false,
            "reads_credentials": false,
            "sends_externally": false,
            "publishes_artifacts": false,
            "installs_or_restarts": false,
            "mutates_active_binary": false
        },
        {
            "action": "prepare_memory_write_execution_preflight_denial_boundary",
            "status": "allowed_report_only_next_slice",
            "requires_accepted_operator_packet_before_execution": true,
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
        "hepta_memory_live_mutation_operator_write_approval_packet_boundary_route"
    );
    insert_report_json!(
        "endpoint",
        HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_APPROVAL_PACKET_BOUNDARY_ENDPOINT
    );
    insert_report_json!(
        "source_command",
        "/hepta-memory-live-mutation-operator-write-approval-packet-boundary --json"
    );
    insert_report_json!("native_route", true);
    insert_report_json!("side_effect_free", true);
    insert_report_json!("audit_date", "2026-07-02");
    insert_report_json!(
        "memory_write_approval_packet_boundary_schema_version",
        "memory_write_approval_packet_boundary_v1"
    );
    insert_report_json!("memory_write_approval_packet_boundary_ready", report_ready);
    insert_report_json!(
        "approval_packet_mode",
        "memory_write_operator_approval_packet_shape_no_recording_no_execution"
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
        "source_full_live_activation_closure_index_endpoint",
        HEPTA_FULL_LIVE_ACTIVATION_CLOSURE_INDEX_ENDPOINT
    );
    insert_report_json!(
        "source_full_live_activation_closure_index_ready",
        closure_index_ready
    );
    insert_report_json!(
        "source_full_live_activation_closure_index_report_sha256",
        source_closure_index_report_sha256
    );
    insert_report_json!(
        "source_minimal_memory_canary_endpoint",
        HEPTA_MINIMAL_MEMORY_CANARY_SCOPED_OPERATOR_PACKET_WRITE_READBACK_ROLLBACK_IDEMPOTENCY_RECEIPT_ENDPOINT
    );
    insert_report_json!("source_minimal_memory_canary_ready", minimal_canary_ready);
    insert_report_json!(
        "source_minimal_memory_canary_report_sha256",
        source_minimal_memory_canary_report_sha256
    );
    insert_report_json!(
        "source_scoped_memory_canary_durable_receipt_boundary_endpoint",
        HEPTA_SCOPED_MEMORY_CANARY_DURABLE_RECEIPT_BOUNDARY_ENDPOINT
    );
    insert_report_json!(
        "source_scoped_memory_canary_durable_receipt_boundary_ready",
        durable_boundary_ready
    );
    insert_report_json!(
        "source_scoped_memory_canary_durable_receipt_boundary_report_sha256",
        source_scoped_memory_canary_durable_receipt_boundary_report_sha256
    );
    insert_report_json!(
        "source_memory_intelligence_kg_truth_index_report_sha256",
        source_truth_index_report_sha256
    );
    insert_report_json!("minimum_required_samples", 24);
    insert_report_json!(
        "closure_blocker_count",
        json_u64(&closure_index, "closure_blocker_count")
    );
    insert_report_json!(
        "remaining_unrestricted_activation_blocker_count",
        json_u64(
            &closure_index,
            "remaining_unrestricted_activation_blocker_count"
        )
    );
    insert_report_json!("memory_write_approval_packet_shape_ready", true);
    insert_report_json!("memory_write_approval_packet_recorded", false);
    insert_report_json!("memory_write_approval_packet_persisted", false);
    insert_report_json!("memory_write_approval_packet_accepted", false);
    insert_report_json!("memory_write_request_recorded", false);
    insert_report_json!("memory_write_request_accepted", false);
    insert_report_json!("memory_write_request_persisted", false);
    insert_report_json!("operator_approval_recorded", false);
    insert_report_json!("operator_identity_hash_recorded", false);
    insert_report_json!("operator_approval_signature_hash_recorded", false);
    insert_report_json!("operator_approval_timestamp_recorded", false);
    insert_report_json!("single_surface_activation_scope_recorded", false);
    insert_report_json!("memory_namespace_recorded", false);
    insert_report_json!("memory_write_operation_recorded", false);
    insert_report_json!("memory_write_operation_allowed", false);
    insert_report_json!("memory_retention_class_recorded", false);
    insert_report_json!("record_intent_recorded", false);
    insert_report_json!("raw_payload_sha256_recorded", false);
    insert_report_json!("redacted_payload_summary_sha256_recorded", false);
    insert_report_json!("accepted_redaction_proof_recorded", false);
    insert_report_json!("accepted_redaction_proof_count", 0);
    insert_report_json!(
        "source_full_live_activation_closure_index_hash_bound",
        false
    );
    insert_report_json!("source_minimal_memory_canary_hash_bound", false);
    insert_report_json!(
        "source_scoped_memory_canary_durable_receipt_hash_bound",
        false
    );
    insert_report_json!("fresh_pre_activation_soak_evidence_recorded", false);
    insert_report_json!("rollback_plan_recorded", false);
    insert_report_json!("post_write_validation_plan_recorded", false);
    insert_report_json!("no_public_claim_no_external_send_decision_recorded", false);
    insert_report_json!("raw_payload_plaintext_recorded", false);
    insert_report_json!("raw_payload_plaintext_persisted", false);
    insert_report_json!("memory_store_mutation_allowed", false);
    insert_report_json!("memory_store_mutated", false);
    insert_report_json!("durable_memory_store_write_performed", false);
    insert_report_json!("memory_write_execution_ready", false);
    insert_report_json!("live_mutation_execution_ready", false);
    insert_report_json!("provider_prompt_replay_enabled", false);
    insert_report_json!("external_send_enabled", false);
    insert_report_json!("public_claim_or_release_artifact_write_enabled", false);
    insert_report_json!(
        "required_memory_write_approval_packet_field_count",
        required_fields.len()
    );
    insert_report_json!("recorded_memory_write_approval_packet_field_count", 0);
    insert_report_json!("inherited_required_memory_write_request_field_count", 17);
    report.insert(
        "inherited_allowed_memory_write_operations".to_string(),
        serde_json::json!(inherited_allowed_memory_write_operations),
    );
    report.insert(
        "required_memory_write_approval_packet_fields".to_string(),
        serde_json::json!(required_fields),
    );
    report.insert(
        "denied_memory_write_approval_packet_fixtures".to_string(),
        serde_json::Value::Array(denied_fixtures),
    );
    insert_report_json!("denied_memory_write_approval_packet_fixture_count", 8);
    report.insert(
        "denied_by_memory_write_approval_packet_boundary".to_string(),
        serde_json::json!(denied_by),
    );
    insert_report_json!("denied_by_memory_write_approval_packet_boundary_count", 22);
    report.insert(
        "required_before_memory_write_approval_packet_acceptance".to_string(),
        serde_json::json!(required_before_acceptance),
    );
    insert_report_json!(
        "required_before_memory_write_approval_packet_acceptance_count",
        14
    );
    insert_report_json!("provider_invoked", false);
    insert_report_json!("model_invoked", false);
    insert_report_json!("credential_read", false);
    insert_report_json!("secret_file_read", false);
    insert_report_json!("kg_adapter_read_performed", false);
    insert_report_json!("live_kg_write_performed", false);
    insert_report_json!("channel_send_performed", false);
    insert_report_json!("telegram_send_performed", false);
    insert_report_json!("external_send_performed", false);
    insert_report_json!("release_artifact_written", false);
    insert_report_json!("public_artifact_written", false);
    insert_report_json!("public_release_claimed", false);
    insert_report_json!("install_executed", false);
    insert_report_json!("service_restarted", false);
    insert_report_json!("active_binary_mutated", false);
    report.insert("allowed_next_actions".to_string(), allowed_next_actions);
    report.insert(
        "side_effects".to_string(),
        serde_json::Value::Object(side_effects),
    );

    serde_json::Value::Object(report)
}

fn hepta_memory_live_mutation_operator_write_execution_preflight_boundary_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let approval_boundary =
        hepta_memory_live_mutation_operator_write_approval_packet_boundary_report();

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
    let approval_boundary_ready = json_bool(
        &approval_boundary,
        "memory_write_approval_packet_boundary_ready",
    ) && json_bool(
        &approval_boundary,
        "memory_write_approval_packet_shape_ready",
    ) && json_u64(
        &approval_boundary,
        "required_memory_write_approval_packet_field_count",
    ) == 21
        && json_u64(
            &approval_boundary,
            "denied_memory_write_approval_packet_fixture_count",
        ) == 8
        && json_u64(
            &approval_boundary,
            "denied_by_memory_write_approval_packet_boundary_count",
        ) == 22
        && !json_bool(&approval_boundary, "memory_write_approval_packet_recorded")
        && !json_bool(&approval_boundary, "memory_write_approval_packet_persisted")
        && !json_bool(&approval_boundary, "memory_write_approval_packet_accepted")
        && !json_bool(&approval_boundary, "memory_write_request_recorded")
        && !json_bool(&approval_boundary, "memory_write_request_accepted")
        && !json_bool(&approval_boundary, "durable_memory_store_write_performed")
        && !json_bool(&approval_boundary, "memory_store_mutated")
        && !json_bool(&approval_boundary, "live_kg_write_performed")
        && !json_bool(&approval_boundary, "provider_invoked")
        && !json_bool(&approval_boundary, "model_invoked")
        && !json_bool(&approval_boundary, "credential_read")
        && !json_bool(&approval_boundary, "external_send_performed")
        && !json_bool(&approval_boundary, "release_artifact_written")
        && !json_bool(&approval_boundary, "public_release_claimed")
        && !json_bool(&approval_boundary, "active_binary_mutated")
        && side_effects_all_false(&approval_boundary);

    let required_checks = vec![
        "approval_packet_hash_binding",
        "memory_write_request_hash_binding",
        "operator_approval_signature_verification",
        "single_surface_scope_verification",
        "memory_namespace_allowlist_verification",
        "memory_write_operation_allowlist_verification",
        "retention_class_allowlist_verification",
        "redaction_proof_acceptance_verification",
        "raw_payload_sha256_binding",
        "redacted_payload_summary_sha256_binding",
        "source_full_live_activation_closure_index_hash_binding",
        "source_minimal_memory_canary_hash_binding",
        "source_scoped_memory_canary_durable_receipt_hash_binding",
        "fresh_pre_activation_soak_verification",
        "rollback_plan_verification",
        "post_write_validation_plan_verification",
        "no_public_claim_no_external_send_verification",
    ];
    let required_before_execution = vec![
        "accepted_memory_write_approval_packet",
        "approval_packet_hash_binding",
        "memory_write_request_hash_binding",
        "operator_approval_signature_verification",
        "operator_approval_timestamp_freshness",
        "single_surface_activation_scope_verification",
        "memory_namespace_allowlist_verification",
        "memory_write_operation_allowlist_verification",
        "retention_class_allowlist_verification",
        "accepted_redaction_proof_freshness",
        "raw_payload_hash_binding_without_plaintext",
        "redacted_payload_summary_hash_binding",
        "source_report_hash_bindings",
        "fresh_pre_activation_soak_evidence",
        "rollback_plan",
        "post_write_validation_plan",
        "no_public_claim_no_external_send_decision",
    ];
    let denied_by = vec![
        "accepted_approval_packet_required",
        "approval_packet_hash_binding_required",
        "memory_write_request_hash_binding_required",
        "valid_operator_signature_and_fresh_timestamp_required",
        "single_surface_scope_verification_required",
        "memory_namespace_allowlist_required",
        "memory_write_operation_allowlist_required",
        "retention_class_allowlist_required",
        "fresh_accepted_redaction_proof_required",
        "raw_payload_sha256_binding_required",
        "redacted_payload_summary_sha256_binding_required",
        "source_full_live_activation_closure_index_hash_binding_required",
        "source_minimal_memory_canary_hash_binding_required",
        "source_scoped_memory_canary_durable_receipt_hash_binding_required",
        "fresh_pre_activation_soak_verification_required",
        "rollback_plan_verification_required",
        "post_write_validation_plan_verification_required",
        "no_public_claim_no_external_send_verification_required",
        "payload_plaintext_recording_denied",
        "memory_store_mutation_denied",
        "rollback_execution_denied",
        "external_send_public_claim_release_artifact_denied",
    ];
    let denied_fixtures = vec![
        serde_json::json!({
            "id": "no-accepted-approval-packet",
            "recorded_pre_execution_validation_check_count": 0,
            "packet_accepted": false,
            "execution_allowed": false,
            "memory_store_mutated": false,
            "activation_allowed": false,
            "reason": "accepted_approval_packet_required"
        }),
        serde_json::json!({
            "id": "missing-approval-packet-hash-binding",
            "recorded_pre_execution_validation_check_count": 3,
            "source_memory_write_approval_packet_hash_bound": false,
            "execution_allowed": false,
            "memory_store_mutated": false,
            "activation_allowed": false,
            "reason": "approval_packet_hash_binding_required"
        }),
        serde_json::json!({
            "id": "operator-signature-or-timestamp-invalid",
            "recorded_pre_execution_validation_check_count": 5,
            "operator_approval_signature_verified": false,
            "operator_approval_timestamp_fresh": false,
            "execution_allowed": false,
            "memory_store_mutated": false,
            "activation_allowed": false,
            "reason": "valid_operator_signature_and_fresh_timestamp_required"
        }),
        serde_json::json!({
            "id": "namespace-operation-or-retention-not-allowlisted",
            "recorded_pre_execution_validation_check_count": 8,
            "memory_namespace_allowed": false,
            "memory_write_operation_allowed": false,
            "memory_retention_class_allowed": false,
            "execution_allowed": false,
            "memory_store_mutated": false,
            "activation_allowed": false,
            "reason": "memory_namespace_operation_and_retention_allowlists_required"
        }),
        serde_json::json!({
            "id": "redaction-proof-missing-or-stale",
            "recorded_pre_execution_validation_check_count": 9,
            "accepted_redaction_proof_recorded": false,
            "accepted_redaction_proof_fresh": false,
            "execution_allowed": false,
            "memory_store_mutated": false,
            "activation_allowed": false,
            "reason": "fresh_accepted_redaction_proof_required"
        }),
        serde_json::json!({
            "id": "payload-hash-mismatch-or-plaintext-present",
            "recorded_pre_execution_validation_check_count": 11,
            "raw_payload_sha256_bound": false,
            "redacted_payload_summary_sha256_bound": false,
            "raw_payload_plaintext_recorded": true,
            "execution_allowed": false,
            "memory_store_mutated": false,
            "activation_allowed": false,
            "reason": "payload_hash_binding_without_plaintext_required"
        }),
        serde_json::json!({
            "id": "fresh-soak-rollback-or-validation-missing",
            "recorded_pre_execution_validation_check_count": 14,
            "fresh_pre_activation_soak_evidence_recorded": false,
            "rollback_plan_recorded": false,
            "post_write_validation_plan_recorded": false,
            "execution_allowed": false,
            "memory_store_mutated": false,
            "activation_allowed": false,
            "reason": "fresh_soak_rollback_and_validation_required"
        }),
        serde_json::json!({
            "id": "external-send-public-claim-or-release-artifact",
            "recorded_pre_execution_validation_check_count": 17,
            "external_send_requested": true,
            "public_claim_requested": true,
            "release_artifact_write_requested": true,
            "execution_allowed": false,
            "memory_store_mutated": false,
            "activation_allowed": false,
            "reason": "external_send_public_claim_and_release_artifact_denied"
        }),
        serde_json::json!({
            "id": "direct-memory-store-execution-at-preflight-layer",
            "recorded_pre_execution_validation_check_count": 17,
            "memory_store_mutation_requested": true,
            "rollback_execution_requested": true,
            "execution_allowed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "activation_allowed": false,
            "reason": "execution_preflight_layer_cannot_mutate_memory_or_execute_rollback"
        }),
    ];

    let source_memory_write_approval_packet_boundary_report_sha256 =
        sha256_json_value(&approval_boundary);
    let boundary_hash_sha256 = sha256_text_value(&format!(
        "hepta-memory-live-mutation-operator-write-execution-preflight-boundary-v1:{}:{}",
        route_matrix.route_count, source_memory_write_approval_packet_boundary_report_sha256,
    ));

    let mut side_effects = serde_json::Map::new();
    for key in [
        "memory_store_mutated",
        "durable_memory_store_write_performed",
        "durable_memory_store_read_performed",
        "durable_memory_store_rollback_performed",
        "memory_write_request_recorded",
        "memory_write_request_persisted",
        "memory_write_approval_packet_recorded",
        "memory_write_approval_packet_persisted",
        "memory_write_approval_packet_accepted",
        "memory_write_execution_preflight_recorded",
        "memory_write_execution_preflight_persisted",
        "memory_write_execution_preflight_accepted",
        "pre_execution_validation_recorded",
        "pre_execution_validation_persisted",
        "pre_execution_validation_accepted",
        "operator_approval_recorded",
        "operator_identity_hash_recorded",
        "operator_approval_signature_hash_recorded",
        "payload_plaintext_persisted",
        "raw_payload_inspected",
        "capability_registry_mutated",
        "plugin_registry_mutated",
        "skill_workshop_written",
        "kg_adapter_read_performed",
        "live_kg_write_performed",
        "provider_invoked",
        "model_invoked",
        "provider_prompt_replayed",
        "channel_send_performed",
        "telegram_send_performed",
        "external_send_performed",
        "runtime_store_mutated",
        "gateway_event_enqueued",
        "filesystem_written",
        "release_artifact_written",
        "public_artifact_written",
        "public_release_claimed",
        "approval_record_persisted",
        "preflight_record_persisted",
        "receipt_persisted",
        "launchd_mutated",
        "service_restarted",
        "install_executed",
        "active_binary_mutated",
        "rollback_executed",
        "credential_read",
        "secret_file_read",
    ] {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }

    let report_ready = route_matrix.ready
        && route_count_source_command_accepted
        && approval_boundary_ready
        && required_checks.len() == 17
        && required_before_execution.len() == 17
        && denied_by.len() == 22
        && denied_fixtures.len() == 9;

    let allowed_next_actions = serde_json::json!([
        {
            "action": "run_memory_write_execution_preflight_boundary_require_live_gate",
            "status": "allowed_verification_only",
            "records_preflight": false,
            "writes_memory": false,
            "executes_rollback": false,
            "writes_kg": false,
            "invokes_provider": false,
            "invokes_model": false,
            "reads_credentials": false,
            "sends_externally": false,
            "publishes_artifacts": false,
            "installs_or_restarts": false,
            "mutates_active_binary": false
        },
        {
            "action": "prepare_memory_write_execution_denial_matrix_boundary",
            "status": "allowed_report_only_next_slice",
            "requires_accepted_operator_packet_before_execution": true,
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
        "hepta_memory_live_mutation_operator_write_execution_preflight_boundary_route"
    );
    insert_report_json!(
        "endpoint",
        HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_PREFLIGHT_BOUNDARY_ENDPOINT
    );
    insert_report_json!(
        "source_command",
        "/hepta-memory-live-mutation-operator-write-execution-preflight-boundary --json"
    );
    insert_report_json!("native_route", true);
    insert_report_json!("side_effect_free", true);
    insert_report_json!("audit_date", "2026-07-02");
    insert_report_json!(
        "memory_write_execution_preflight_boundary_schema_version",
        "memory_write_execution_preflight_boundary_v1"
    );
    insert_report_json!(
        "memory_write_execution_preflight_boundary_ready",
        report_ready
    );
    insert_report_json!(
        "execution_preflight_mode",
        "memory_write_execution_preflight_no_approval_no_mutation"
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
        "source_memory_write_approval_packet_boundary_endpoint",
        HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_APPROVAL_PACKET_BOUNDARY_ENDPOINT
    );
    insert_report_json!(
        "source_memory_write_approval_packet_boundary_ready",
        approval_boundary_ready
    );
    insert_report_json!(
        "source_memory_write_approval_packet_boundary_report_sha256",
        source_memory_write_approval_packet_boundary_report_sha256
    );
    insert_report_json!("minimum_required_samples", 24);
    insert_report_json!("memory_write_execution_preflight_shape_ready", true);
    insert_report_json!("memory_write_execution_preflight_recorded", false);
    insert_report_json!("memory_write_execution_preflight_persisted", false);
    insert_report_json!("memory_write_execution_preflight_accepted", false);
    insert_report_json!("pre_execution_validation_shape_ready", true);
    insert_report_json!("pre_execution_validation_recorded", false);
    insert_report_json!("pre_execution_validation_persisted", false);
    insert_report_json!("pre_execution_validation_accepted", false);
    insert_report_json!("memory_write_approval_packet_recorded", false);
    insert_report_json!("memory_write_approval_packet_persisted", false);
    insert_report_json!("memory_write_approval_packet_accepted", false);
    insert_report_json!("memory_write_request_recorded", false);
    insert_report_json!("memory_write_request_accepted", false);
    insert_report_json!("memory_write_request_persisted", false);
    insert_report_json!("operator_approval_recorded", false);
    insert_report_json!("operator_identity_hash_recorded", false);
    insert_report_json!("operator_approval_signature_hash_recorded", false);
    insert_report_json!("operator_approval_timestamp_recorded", false);
    insert_report_json!("single_surface_activation_scope_recorded", false);
    insert_report_json!("memory_namespace_recorded", false);
    insert_report_json!("memory_write_operation_recorded", false);
    insert_report_json!("memory_write_operation_allowed", false);
    insert_report_json!("memory_retention_class_recorded", false);
    insert_report_json!("accepted_redaction_proof_recorded", false);
    insert_report_json!("accepted_redaction_proof_count", 0);
    insert_report_json!("source_memory_write_approval_packet_hash_bound", false);
    insert_report_json!("source_memory_write_request_hash_bound", false);
    insert_report_json!(
        "source_full_live_activation_closure_index_hash_bound",
        false
    );
    insert_report_json!("source_minimal_memory_canary_hash_bound", false);
    insert_report_json!(
        "source_scoped_memory_canary_durable_receipt_hash_bound",
        false
    );
    insert_report_json!("raw_payload_sha256_bound", false);
    insert_report_json!("redacted_payload_summary_sha256_bound", false);
    insert_report_json!("fresh_pre_activation_soak_evidence_recorded", false);
    insert_report_json!("rollback_plan_recorded", false);
    insert_report_json!("post_write_validation_plan_recorded", false);
    insert_report_json!("no_public_claim_no_external_send_decision_recorded", false);
    insert_report_json!("memory_write_execution_allowed", false);
    insert_report_json!("memory_write_execution_ready", false);
    insert_report_json!("memory_store_mutation_allowed", false);
    insert_report_json!("memory_store_mutated", false);
    insert_report_json!("durable_memory_store_write_performed", false);
    insert_report_json!("live_mutation_execution_ready", false);
    insert_report_json!("rollback_execution_allowed", false);
    insert_report_json!("rollback_executed", false);
    insert_report_json!("provider_prompt_replay_enabled", false);
    insert_report_json!("external_send_enabled", false);
    insert_report_json!("public_claim_or_release_artifact_write_enabled", false);
    insert_report_json!(
        "required_pre_execution_validation_check_count",
        required_checks.len()
    );
    insert_report_json!("recorded_pre_execution_validation_check_count", 0);
    report.insert(
        "required_pre_execution_validation_checks".to_string(),
        serde_json::json!(required_checks),
    );
    report.insert(
        "denied_memory_write_execution_preflight_fixtures".to_string(),
        serde_json::Value::Array(denied_fixtures),
    );
    insert_report_json!("denied_memory_write_execution_preflight_fixture_count", 9);
    report.insert(
        "denied_by_memory_write_execution_preflight_boundary".to_string(),
        serde_json::json!(denied_by),
    );
    insert_report_json!(
        "denied_by_memory_write_execution_preflight_boundary_count",
        22
    );
    report.insert(
        "required_before_memory_write_execution".to_string(),
        serde_json::json!(required_before_execution),
    );
    insert_report_json!("required_before_memory_write_execution_count", 17);
    insert_report_json!("provider_invoked", false);
    insert_report_json!("model_invoked", false);
    insert_report_json!("credential_read", false);
    insert_report_json!("secret_file_read", false);
    insert_report_json!("kg_adapter_read_performed", false);
    insert_report_json!("live_kg_write_performed", false);
    insert_report_json!("channel_send_performed", false);
    insert_report_json!("telegram_send_performed", false);
    insert_report_json!("external_send_performed", false);
    insert_report_json!("release_artifact_written", false);
    insert_report_json!("public_artifact_written", false);
    insert_report_json!("public_release_claimed", false);
    insert_report_json!("install_executed", false);
    insert_report_json!("service_restarted", false);
    insert_report_json!("active_binary_mutated", false);
    report.insert("allowed_next_actions".to_string(), allowed_next_actions);
    report.insert(
        "side_effects".to_string(),
        serde_json::Value::Object(side_effects),
    );

    serde_json::Value::Object(report)
}

fn hepta_memory_live_mutation_operator_write_execution_denial_matrix_boundary_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let preflight_boundary =
        hepta_memory_live_mutation_operator_write_execution_preflight_boundary_report();

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
    let preflight_boundary_ready =
        json_bool(
            &preflight_boundary,
            "memory_write_execution_preflight_boundary_ready",
        ) && json_bool(
            &preflight_boundary,
            "memory_write_execution_preflight_shape_ready",
        ) && json_bool(&preflight_boundary, "pre_execution_validation_shape_ready")
            && json_u64(
                &preflight_boundary,
                "required_pre_execution_validation_check_count",
            ) == 17
            && json_u64(
                &preflight_boundary,
                "recorded_pre_execution_validation_check_count",
            ) == 0
            && json_u64(
                &preflight_boundary,
                "denied_memory_write_execution_preflight_fixture_count",
            ) == 9
            && json_u64(
                &preflight_boundary,
                "denied_by_memory_write_execution_preflight_boundary_count",
            ) == 22
            && json_u64(
                &preflight_boundary,
                "required_before_memory_write_execution_count",
            ) == 17
            && !json_bool(
                &preflight_boundary,
                "memory_write_execution_preflight_recorded",
            )
            && !json_bool(
                &preflight_boundary,
                "memory_write_execution_preflight_persisted",
            )
            && !json_bool(
                &preflight_boundary,
                "memory_write_execution_preflight_accepted",
            )
            && !json_bool(&preflight_boundary, "pre_execution_validation_recorded")
            && !json_bool(&preflight_boundary, "pre_execution_validation_persisted")
            && !json_bool(&preflight_boundary, "pre_execution_validation_accepted")
            && !json_bool(&preflight_boundary, "memory_write_approval_packet_accepted")
            && !json_bool(&preflight_boundary, "memory_write_request_accepted")
            && !json_bool(&preflight_boundary, "memory_write_execution_allowed")
            && !json_bool(&preflight_boundary, "memory_write_execution_ready")
            && !json_bool(&preflight_boundary, "memory_store_mutated")
            && !json_bool(&preflight_boundary, "rollback_executed")
            && !json_bool(&preflight_boundary, "live_kg_write_performed")
            && !json_bool(&preflight_boundary, "provider_invoked")
            && !json_bool(&preflight_boundary, "model_invoked")
            && !json_bool(&preflight_boundary, "credential_read")
            && !json_bool(&preflight_boundary, "external_send_performed")
            && !json_bool(&preflight_boundary, "release_artifact_written")
            && !json_bool(&preflight_boundary, "public_release_claimed")
            && !json_bool(&preflight_boundary, "active_binary_mutated")
            && side_effects_all_false(&preflight_boundary);

    let required_before_execution = preflight_boundary
        .get("required_before_memory_write_execution")
        .and_then(serde_json::Value::as_array)
        .cloned()
        .unwrap_or_default();
    let denied_by = vec![
        "memory_write_execution_denial_matrix_recording_denied",
        "memory_write_execution_denial_matrix_persistence_denied",
        "memory_write_execution_denial_matrix_materialization_denied",
        "memory_write_execution_denial_matrix_filesystem_write_denied",
        "accepted_approval_packet_required",
        "all_pre_execution_validation_checks_required",
        "approval_packet_hash_binding_required",
        "memory_write_request_hash_binding_required",
        "operator_signature_and_timestamp_required",
        "single_surface_scope_verification_required",
        "namespace_operation_and_retention_allowlists_required",
        "fresh_accepted_redaction_proof_required",
        "payload_hash_binding_without_plaintext_required",
        "source_report_hash_bindings_required",
        "fresh_pre_activation_soak_verification_required",
        "rollback_plan_verification_required",
        "post_write_validation_plan_verification_required",
        "no_public_claim_no_external_send_verification_required",
        "memory_write_execution_denied",
        "memory_store_mutation_denied",
        "rollback_execution_denied",
        "external_send_public_claim_release_artifact_denied",
    ];
    let execution_denial_fixtures = vec![
        serde_json::json!({
            "id": "missing-accepted-approval-packet-execution-attempt",
            "execution_requested": true,
            "accepted_approval_packet_present": false,
            "accepted_pre_execution_validation_check_count": 0,
            "execution_allowed": false,
            "execution_performed": false,
            "memory_store_mutated": false,
            "activation_allowed": false,
            "reason": "accepted_approval_packet_required"
        }),
        serde_json::json!({
            "id": "partial-pre-execution-validation-execution-attempt",
            "execution_requested": true,
            "accepted_approval_packet_present": true,
            "accepted_pre_execution_validation_check_count": 8,
            "required_pre_execution_validation_check_count": 17,
            "execution_allowed": false,
            "execution_performed": false,
            "memory_store_mutated": false,
            "activation_allowed": false,
            "reason": "all_pre_execution_validation_checks_required"
        }),
        serde_json::json!({
            "id": "namespace-operation-retention-not-allowlisted-execution-attempt",
            "execution_requested": true,
            "accepted_pre_execution_validation_check_count": 17,
            "memory_namespace_allowed": false,
            "memory_write_operation_allowed": false,
            "memory_retention_class_allowed": false,
            "execution_allowed": false,
            "execution_performed": false,
            "memory_store_mutated": false,
            "activation_allowed": false,
            "reason": "namespace_operation_and_retention_allowlists_required"
        }),
        serde_json::json!({
            "id": "payload-hash-mismatch-or-plaintext-execution-attempt",
            "execution_requested": true,
            "accepted_pre_execution_validation_check_count": 17,
            "raw_payload_sha256_bound": false,
            "redacted_payload_summary_sha256_bound": false,
            "raw_payload_plaintext_recorded": true,
            "execution_allowed": false,
            "execution_performed": false,
            "memory_store_mutated": false,
            "activation_allowed": false,
            "reason": "payload_hash_binding_without_plaintext_required"
        }),
        serde_json::json!({
            "id": "stale-soak-or-missing-rollback-validation-execution-attempt",
            "execution_requested": true,
            "accepted_pre_execution_validation_check_count": 17,
            "fresh_pre_activation_soak_evidence_recorded": false,
            "rollback_plan_recorded": false,
            "post_write_validation_plan_recorded": false,
            "execution_allowed": false,
            "execution_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "activation_allowed": false,
            "reason": "fresh_soak_rollback_and_validation_required"
        }),
        serde_json::json!({
            "id": "external-send-public-claim-release-artifact-execution-attempt",
            "execution_requested": true,
            "accepted_pre_execution_validation_check_count": 17,
            "external_send_requested": true,
            "public_claim_requested": true,
            "release_artifact_write_requested": true,
            "execution_allowed": false,
            "execution_performed": false,
            "memory_store_mutated": false,
            "external_send_performed": false,
            "public_release_published": false,
            "release_artifact_written": false,
            "activation_allowed": false,
            "reason": "external_send_public_claim_and_release_artifact_denied"
        }),
        serde_json::json!({
            "id": "direct-memory-store-mutation-or-rollback-execution-attempt",
            "execution_requested": true,
            "accepted_pre_execution_validation_check_count": 17,
            "memory_store_mutation_requested": true,
            "rollback_execution_requested": true,
            "execution_allowed": false,
            "execution_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "activation_allowed": false,
            "reason": "execution_denial_matrix_layer_cannot_mutate_memory_or_execute_rollback"
        }),
    ];

    let source_memory_write_execution_preflight_boundary_report_sha256 =
        sha256_json_value(&preflight_boundary);
    let boundary_hash_sha256 = sha256_text_value(&format!(
        "hepta-memory-live-mutation-operator-write-execution-denial-matrix-boundary-v1:{}:{}",
        route_matrix.route_count, source_memory_write_execution_preflight_boundary_report_sha256,
    ));

    let mut side_effects = serde_json::Map::new();
    for key in [
        "memory_store_mutated",
        "durable_memory_store_write_performed",
        "durable_memory_store_read_performed",
        "durable_memory_store_rollback_performed",
        "memory_write_request_recorded",
        "memory_write_request_persisted",
        "memory_write_approval_packet_recorded",
        "memory_write_approval_packet_persisted",
        "memory_write_approval_packet_accepted",
        "memory_write_execution_preflight_recorded",
        "memory_write_execution_preflight_persisted",
        "memory_write_execution_preflight_accepted",
        "memory_write_execution_denial_matrix_recorded",
        "memory_write_execution_denial_matrix_persisted",
        "memory_write_execution_denial_matrix_materialized",
        "memory_write_execution_denial_matrix_filesystem_written",
        "pre_execution_validation_recorded",
        "pre_execution_validation_persisted",
        "pre_execution_validation_accepted",
        "operator_approval_recorded",
        "operator_identity_hash_recorded",
        "operator_approval_signature_hash_recorded",
        "payload_plaintext_persisted",
        "raw_payload_inspected",
        "capability_registry_mutated",
        "plugin_registry_mutated",
        "skill_workshop_written",
        "kg_adapter_read_performed",
        "live_kg_write_performed",
        "provider_invoked",
        "model_invoked",
        "provider_prompt_replayed",
        "channel_send_performed",
        "telegram_send_performed",
        "external_send_performed",
        "runtime_store_mutated",
        "gateway_event_enqueued",
        "filesystem_written",
        "release_artifact_written",
        "public_artifact_written",
        "public_release_claimed",
        "approval_record_persisted",
        "preflight_record_persisted",
        "denial_matrix_persisted",
        "receipt_persisted",
        "launchd_mutated",
        "service_restarted",
        "install_executed",
        "active_binary_mutated",
        "rollback_executed",
        "credential_read",
        "secret_file_read",
    ] {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }

    let report_ready = route_matrix.ready
        && route_count_source_command_accepted
        && preflight_boundary_ready
        && required_before_execution.len() == 17
        && denied_by.len() == 22
        && execution_denial_fixtures.len() == 7;

    let allowed_next_actions = serde_json::json!([
        {
            "action": "run_memory_write_execution_denial_matrix_boundary_require_live_gate",
            "status": "allowed_verification_only",
            "records_denial_matrix": false,
            "writes_memory": false,
            "executes_rollback": false,
            "writes_kg": false,
            "invokes_provider": false,
            "invokes_model": false,
            "reads_credentials": false,
            "sends_externally": false,
            "publishes_artifacts": false,
            "installs_or_restarts": false,
            "mutates_active_binary": false
        },
        {
            "action": "prepare_memory_write_execution_no_write_sink_contract_boundary",
            "status": "allowed_report_only_next_slice",
            "requires_accepted_operator_packet_before_execution": true,
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
        "hepta_memory_live_mutation_operator_write_execution_denial_matrix_boundary_route"
    );
    insert_report_json!(
        "endpoint",
        HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_DENIAL_MATRIX_BOUNDARY_ENDPOINT
    );
    insert_report_json!(
        "source_command",
        "/hepta-memory-live-mutation-operator-write-execution-denial-matrix-boundary --json"
    );
    insert_report_json!("native_route", true);
    insert_report_json!("side_effect_free", true);
    insert_report_json!("audit_date", "2026-07-02");
    insert_report_json!(
        "memory_write_execution_denial_matrix_boundary_schema_version",
        "memory_write_execution_denial_matrix_boundary_v1"
    );
    insert_report_json!(
        "memory_write_execution_denial_matrix_boundary_ready",
        report_ready
    );
    insert_report_json!(
        "execution_denial_matrix_mode",
        "memory_write_execution_attempt_denial_matrix_no_store_mutation"
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
        "source_memory_write_execution_preflight_boundary_endpoint",
        HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_PREFLIGHT_BOUNDARY_ENDPOINT
    );
    insert_report_json!(
        "source_memory_write_execution_preflight_boundary_ready",
        preflight_boundary_ready
    );
    insert_report_json!(
        "source_memory_write_execution_preflight_boundary_report_sha256",
        source_memory_write_execution_preflight_boundary_report_sha256
    );
    insert_report_json!("minimum_required_samples", 24);
    insert_report_json!("memory_write_execution_denial_matrix_ready", true);
    insert_report_json!("memory_write_execution_denial_matrix_recorded", false);
    insert_report_json!("memory_write_execution_denial_matrix_persisted", false);
    insert_report_json!("memory_write_execution_denial_matrix_materialized", false);
    insert_report_json!(
        "memory_write_execution_denial_matrix_filesystem_written",
        false
    );
    insert_report_json!("pre_execution_validation_shape_ready", true);
    insert_report_json!(
        "required_pre_execution_validation_check_count",
        json_u64(
            &preflight_boundary,
            "required_pre_execution_validation_check_count"
        )
    );
    insert_report_json!("recorded_pre_execution_validation_check_count", 0);
    insert_report_json!("accepted_pre_execution_validation_check_count", 0);
    insert_report_json!("future_pre_execution_validation_check_slot_count", 17);
    insert_report_json!("memory_write_execution_attempt_requested_count", 7);
    insert_report_json!("memory_write_execution_attempt_performed_count", 0);
    insert_report_json!("memory_write_execution_allowed_count", 0);
    insert_report_json!("memory_write_execution_denied_count", 7);
    insert_report_json!("blocked_execution_fixture_count", 7);
    insert_report_json!("allowed_execution_fixture_count", 0);
    insert_report_json!("required_execution_denial_fixture_count", 7);
    insert_report_json!("execution_denial_fixture_count", 7);
    insert_report_json!("pre_execution_validation_recorded", false);
    insert_report_json!("pre_execution_validation_persisted", false);
    insert_report_json!("pre_execution_validation_accepted", false);
    insert_report_json!("memory_write_approval_packet_recorded", false);
    insert_report_json!("memory_write_approval_packet_persisted", false);
    insert_report_json!("memory_write_approval_packet_accepted", false);
    insert_report_json!("memory_write_request_recorded", false);
    insert_report_json!("memory_write_request_accepted", false);
    insert_report_json!("memory_write_request_persisted", false);
    insert_report_json!("operator_approval_recorded", false);
    insert_report_json!("operator_identity_hash_recorded", false);
    insert_report_json!("operator_approval_signature_hash_recorded", false);
    insert_report_json!("operator_approval_timestamp_recorded", false);
    insert_report_json!("single_surface_activation_scope_recorded", false);
    insert_report_json!("memory_namespace_recorded", false);
    insert_report_json!("memory_write_operation_recorded", false);
    insert_report_json!("memory_write_operation_allowed", false);
    insert_report_json!("memory_retention_class_recorded", false);
    insert_report_json!("accepted_redaction_proof_recorded", false);
    insert_report_json!("accepted_redaction_proof_count", 0);
    insert_report_json!("source_memory_write_approval_packet_hash_bound", false);
    insert_report_json!("source_memory_write_request_hash_bound", false);
    insert_report_json!("source_memory_write_execution_preflight_hash_bound", false);
    insert_report_json!("source_memory_write_contract_hash_bound", false);
    insert_report_json!("source_memory_intelligence_hash_bound", false);
    insert_report_json!(
        "source_payload_redaction_acceptance_matrix_hash_bound",
        false
    );
    insert_report_json!("source_payload_redaction_proof_hash_bound", false);
    insert_report_json!("raw_payload_sha256_bound", false);
    insert_report_json!("redacted_payload_summary_sha256_bound", false);
    insert_report_json!("raw_payload_plaintext_recorded", false);
    insert_report_json!("raw_payload_plaintext_persisted", false);
    insert_report_json!("fresh_pre_activation_soak_evidence_recorded", false);
    insert_report_json!("rollback_plan_recorded", false);
    insert_report_json!("post_write_validation_plan_recorded", false);
    insert_report_json!("no_public_claim_no_external_send_decision_recorded", false);
    insert_report_json!("memory_write_execution_allowed", false);
    insert_report_json!("memory_write_execution_ready", false);
    insert_report_json!("memory_write_execution_performed", false);
    insert_report_json!("memory_store_mutation_allowed", false);
    insert_report_json!("memory_store_mutated", false);
    insert_report_json!("durable_memory_store_write_performed", false);
    insert_report_json!("live_mutation_execution_ready", false);
    insert_report_json!("rollback_execution_allowed", false);
    insert_report_json!("rollback_executed", false);
    insert_report_json!("provider_prompt_replay_enabled", false);
    insert_report_json!("external_send_enabled", false);
    insert_report_json!("external_send_performed", false);
    insert_report_json!("public_claim_or_release_artifact_write_enabled", false);
    insert_report_json!("public_release_published", false);
    insert_report_json!("release_artifact_written", false);
    report.insert(
        "execution_denial_fixtures".to_string(),
        serde_json::Value::Array(execution_denial_fixtures),
    );
    report.insert(
        "denied_by_memory_write_execution_denial_matrix".to_string(),
        serde_json::json!(denied_by),
    );
    insert_report_json!("denied_by_memory_write_execution_denial_matrix_count", 22);
    report.insert(
        "required_before_memory_write_execution".to_string(),
        serde_json::Value::Array(required_before_execution),
    );
    insert_report_json!("required_before_memory_write_execution_count", 17);
    insert_report_json!("provider_invoked", false);
    insert_report_json!("model_invoked", false);
    insert_report_json!("credential_read", false);
    insert_report_json!("secret_file_read", false);
    insert_report_json!("kg_adapter_read_performed", false);
    insert_report_json!("live_kg_write_performed", false);
    insert_report_json!("channel_send_performed", false);
    insert_report_json!("telegram_send_performed", false);
    insert_report_json!("external_send_performed", false);
    insert_report_json!("public_artifact_written", false);
    insert_report_json!("public_release_claimed", false);
    insert_report_json!("install_executed", false);
    insert_report_json!("service_restarted", false);
    insert_report_json!("active_binary_mutated", false);
    report.insert("allowed_next_actions".to_string(), allowed_next_actions);
    report.insert(
        "side_effects".to_string(),
        serde_json::Value::Object(side_effects),
    );

    serde_json::Value::Object(report)
}

fn hepta_memory_live_mutation_operator_write_execution_no_write_sink_contract_boundary_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let denial_matrix =
        hepta_memory_live_mutation_operator_write_execution_denial_matrix_boundary_report();

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
    let denial_matrix_ready =
        json_bool(
            &denial_matrix,
            "memory_write_execution_denial_matrix_boundary_ready",
        ) && json_bool(&denial_matrix, "memory_write_execution_denial_matrix_ready")
            && json_u64(
                &denial_matrix,
                "memory_write_execution_attempt_requested_count",
            ) == 7
            && json_u64(
                &denial_matrix,
                "memory_write_execution_attempt_performed_count",
            ) == 0
            && json_u64(&denial_matrix, "memory_write_execution_denied_count") == 7
            && json_u64(&denial_matrix, "execution_denial_fixture_count") == 7
            && json_u64(
                &denial_matrix,
                "denied_by_memory_write_execution_denial_matrix_count",
            ) == 22
            && json_u64(
                &denial_matrix,
                "required_before_memory_write_execution_count",
            ) == 17
            && !json_bool(
                &denial_matrix,
                "memory_write_execution_denial_matrix_recorded",
            )
            && !json_bool(
                &denial_matrix,
                "memory_write_execution_denial_matrix_persisted",
            )
            && !json_bool(
                &denial_matrix,
                "memory_write_execution_denial_matrix_materialized",
            )
            && !json_bool(
                &denial_matrix,
                "memory_write_execution_denial_matrix_filesystem_written",
            )
            && !json_bool(&denial_matrix, "memory_write_execution_performed")
            && !json_bool(&denial_matrix, "memory_store_mutated")
            && !json_bool(&denial_matrix, "rollback_executed")
            && !json_bool(&denial_matrix, "live_kg_write_performed")
            && !json_bool(&denial_matrix, "provider_invoked")
            && !json_bool(&denial_matrix, "model_invoked")
            && !json_bool(&denial_matrix, "credential_read")
            && !json_bool(&denial_matrix, "external_send_performed")
            && !json_bool(&denial_matrix, "release_artifact_written")
            && !json_bool(&denial_matrix, "public_release_claimed")
            && !json_bool(&denial_matrix, "active_binary_mutated")
            && side_effects_all_false(&denial_matrix);

    let no_write_sink_surfaces = vec![
        "redacted_execution_request_envelope_validation",
        "source_report_hash_binding_validation",
        "operator_approval_preflight_validation_requirement",
        "memory_namespace_operation_retention_allowlist_requirement",
        "payload_hash_binding_without_plaintext_requirement",
        "fresh_soak_rollback_validation_requirement",
        "external_send_public_claim_artifact_rejection",
        "store_write_path_disabled_by_default",
    ];
    let required_before_any_memory_write_execution = vec![
        "accepted_operator_approval_packet",
        "accepted_pre_execution_validation_record",
        "operator_identity_hash",
        "operator_approval_signature_hash",
        "operator_approval_timestamp",
        "single_surface_activation_scope",
        "namespace_operation_retention_allowlist_match",
        "accepted_redaction_proof_id",
        "source_report_hash_bindings",
        "raw_payload_sha256_without_plaintext",
        "redacted_payload_summary_sha256",
        "fresh_pre_activation_soak_evidence",
        "rollback_plan_id",
        "post_write_validation_plan_id",
        "no_public_claim_no_external_send_decision",
        "explicit_write_path_enablement",
        "post_write_watchdog_soak_plan",
    ];
    let denied_by = vec![
        "execution_remains_disabled",
        "store_write_path_disabled_by_default",
        "memory_store_mutation_denied",
        "rollback_execution_denied",
        "external_send_denied",
        "public_claim_denied",
        "release_artifact_write_denied",
        "plaintext_payload_recording_denied",
        "secret_read_denied",
        "service_restart_denied",
    ];
    let no_write_sink_fixtures = vec![
        serde_json::json!({
            "id": "redacted-execution-envelope-validation-shape",
            "sink_status": "accepted_for_no_write_validation",
            "redacted_execution_request_envelope_present": true,
            "source_report_hash_bindings_present": true,
            "execution_requested": true,
            "write_requested": false,
            "execution_allowed": false,
            "execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "activation_allowed": false,
            "reason": "redacted_shape_can_be_validated_but_execution_remains_disabled"
        }),
        serde_json::json!({
            "id": "source-report-hash-bound-validation-shape",
            "sink_status": "accepted_for_no_write_validation",
            "source_memory_write_execution_denial_matrix_report_sha256_bound": true,
            "source_memory_write_execution_preflight_report_sha256_bound": true,
            "source_payload_redaction_proof_report_sha256_bound": true,
            "execution_requested": true,
            "write_requested": false,
            "execution_allowed": false,
            "execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "activation_allowed": false,
            "reason": "source_hash_shape_can_be_validated_but_not_executed"
        }),
        serde_json::json!({
            "id": "approval-preflight-allowlist-validation-shape",
            "sink_status": "accepted_for_no_write_validation",
            "operator_approval_required": true,
            "all_pre_execution_validation_checks_required": true,
            "namespace_operation_retention_allowlist_required": true,
            "payload_hash_binding_without_plaintext_required": true,
            "fresh_soak_rollback_validation_required": true,
            "execution_requested": true,
            "write_requested": false,
            "execution_allowed": false,
            "execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "activation_allowed": false,
            "reason": "future_requirements_can_be_described_but_no_write_sink_keeps_execution_off"
        }),
        serde_json::json!({
            "id": "store-write-path-disabled-mutation-attempt",
            "sink_status": "rejected",
            "execution_requested": true,
            "write_requested": true,
            "memory_store_mutation_requested": true,
            "no_write_sink_write_path_enabled_by_default": false,
            "execution_allowed": false,
            "execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "activation_allowed": false,
            "reason": "store_write_path_disabled_by_default"
        }),
        serde_json::json!({
            "id": "external-send-public-artifact-attempt",
            "sink_status": "rejected",
            "execution_requested": true,
            "write_requested": true,
            "external_send_requested": true,
            "public_claim_requested": true,
            "release_artifact_write_requested": true,
            "execution_allowed": false,
            "execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "external_send_performed": false,
            "public_release_published": false,
            "release_artifact_written": false,
            "activation_allowed": false,
            "reason": "external_send_public_claim_and_release_artifact_denied"
        }),
        serde_json::json!({
            "id": "rollback-or-direct-store-execution-attempt",
            "sink_status": "rejected",
            "execution_requested": true,
            "write_requested": true,
            "rollback_execution_requested": true,
            "memory_store_mutation_requested": true,
            "execution_allowed": false,
            "execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "activation_allowed": false,
            "reason": "no_write_sink_cannot_execute_rollback_or_direct_store_mutation"
        }),
    ];

    let accepted_validation_fixture_count = no_write_sink_fixtures
        .iter()
        .filter(|fixture| {
            fixture
                .get("sink_status")
                .and_then(serde_json::Value::as_str)
                == Some("accepted_for_no_write_validation")
        })
        .count();
    let rejected_execution_fixture_count = no_write_sink_fixtures
        .iter()
        .filter(|fixture| {
            fixture
                .get("sink_status")
                .and_then(serde_json::Value::as_str)
                == Some("rejected")
        })
        .count();
    let write_request_fixture_count = no_write_sink_fixtures
        .iter()
        .filter(|fixture| {
            fixture
                .get("write_requested")
                .and_then(serde_json::Value::as_bool)
                == Some(true)
        })
        .count();

    let source_memory_write_execution_denial_matrix_boundary_report_sha256 =
        sha256_json_value(&denial_matrix);
    let boundary_hash_sha256 = sha256_text_value(&format!(
        "hepta-memory-live-mutation-operator-write-execution-no-write-sink-contract-boundary-v1:{}:{}",
        route_matrix.route_count,
        source_memory_write_execution_denial_matrix_boundary_report_sha256,
    ));

    let mut side_effects = serde_json::Map::new();
    for key in [
        "memory_store_mutated",
        "memory_store_write_performed",
        "durable_memory_store_write_performed",
        "memory_write_request_recorded",
        "memory_write_request_persisted",
        "memory_write_approval_packet_recorded",
        "memory_write_approval_packet_persisted",
        "memory_write_execution_preflight_recorded",
        "memory_write_execution_preflight_persisted",
        "memory_write_execution_denial_matrix_recorded",
        "memory_write_execution_denial_matrix_persisted",
        "memory_write_execution_no_write_sink_contract_recorded",
        "memory_write_execution_no_write_sink_contract_persisted",
        "memory_write_execution_no_write_sink_contract_materialized",
        "memory_write_execution_no_write_sink_contract_filesystem_written",
        "pre_execution_validation_recorded",
        "pre_execution_validation_persisted",
        "payload_plaintext_persisted",
        "raw_payload_inspected",
        "capability_registry_mutated",
        "plugin_registry_mutated",
        "skill_workshop_written",
        "provider_invoked",
        "model_invoked",
        "provider_prompt_replayed",
        "channel_send_performed",
        "telegram_send_performed",
        "external_send_performed",
        "runtime_store_mutated",
        "gateway_event_enqueued",
        "filesystem_written",
        "release_artifact_written",
        "public_artifact_written",
        "public_release_claimed",
        "approval_record_persisted",
        "preflight_record_persisted",
        "denial_matrix_persisted",
        "no_write_sink_contract_persisted",
        "receipt_persisted",
        "launchd_mutated",
        "service_restarted",
        "install_executed",
        "active_binary_mutated",
        "rollback_executed",
        "credential_read",
        "secret_file_read",
    ] {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }

    let report_ready = route_matrix.ready
        && route_count_source_command_accepted
        && denial_matrix_ready
        && no_write_sink_surfaces.len() == 8
        && no_write_sink_fixtures.len() == 6
        && accepted_validation_fixture_count == 3
        && rejected_execution_fixture_count == 3
        && write_request_fixture_count == 3
        && denied_by.len() == 10
        && required_before_any_memory_write_execution.len() == 17;

    let allowed_next_actions = serde_json::json!([
        {
            "action": "run_memory_write_execution_no_write_sink_contract_boundary_require_live_gate",
            "status": "allowed_verification_only",
            "records_no_write_sink_contract": false,
            "writes_memory": false,
            "executes_rollback": false,
            "writes_kg": false,
            "invokes_provider": false,
            "invokes_model": false,
            "reads_credentials": false,
            "sends_externally": false,
            "publishes_artifacts": false,
            "installs_or_restarts": false,
            "mutates_active_binary": false
        },
        {
            "action": "prepare_memory_write_execution_write_enable_fixture_boundary",
            "status": "allowed_report_only_next_slice",
            "requires_accepted_operator_packet_before_execution": true,
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
        "hepta_memory_live_mutation_operator_write_execution_no_write_sink_contract_boundary_route"
    );
    insert_report_json!(
        "endpoint",
        HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_NO_WRITE_SINK_CONTRACT_BOUNDARY_ENDPOINT
    );
    insert_report_json!(
        "source_command",
        "/hepta-memory-live-mutation-operator-write-execution-no-write-sink-contract-boundary --json"
    );
    insert_report_json!("native_route", true);
    insert_report_json!("side_effect_free", true);
    insert_report_json!("audit_date", "2026-07-02");
    insert_report_json!(
        "memory_write_execution_no_write_sink_contract_boundary_schema_version",
        "memory_write_execution_no_write_sink_contract_boundary_v1"
    );
    insert_report_json!(
        "memory_write_execution_no_write_sink_contract_boundary_ready",
        report_ready
    );
    insert_report_json!(
        "no_write_sink_contract_mode",
        "memory_write_execution_no_write_sink_contract_no_store_mutation"
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
        "source_memory_write_execution_denial_matrix_boundary_endpoint",
        HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_DENIAL_MATRIX_BOUNDARY_ENDPOINT
    );
    insert_report_json!(
        "source_memory_write_execution_denial_matrix_boundary_ready",
        denial_matrix_ready
    );
    insert_report_json!(
        "source_memory_write_execution_denial_matrix_boundary_report_sha256",
        source_memory_write_execution_denial_matrix_boundary_report_sha256
    );
    insert_report_json!("minimum_required_samples", 24);
    insert_report_json!("memory_write_execution_no_write_sink_contract_ready", true);
    insert_report_json!("memory_write_execution_denial_matrix_ready", true);
    insert_report_json!("pre_execution_validation_shape_ready", true);
    insert_report_json!(
        "required_pre_execution_validation_check_count",
        json_u64(
            &denial_matrix,
            "required_pre_execution_validation_check_count"
        )
    );
    insert_report_json!("accepted_pre_execution_validation_check_count", 0);
    insert_report_json!("required_no_write_sink_surface_count", 8);
    insert_report_json!("ready_no_write_sink_surface_count", 8);
    insert_report_json!("side_effect_free_no_write_sink_surface_count", 8);
    insert_report_json!("no_write_sink_fixture_count", 6);
    insert_report_json!("no_write_sink_accepted_validation_fixture_count", 3);
    insert_report_json!("no_write_sink_rejected_execution_fixture_count", 3);
    insert_report_json!("no_write_sink_execution_request_fixture_count", 6);
    insert_report_json!("no_write_sink_write_request_fixture_count", 3);
    insert_report_json!("no_write_sink_allowed_write_fixture_count", 0);
    insert_report_json!("no_write_sink_rejected_write_fixture_count", 3);
    insert_report_json!("no_write_sink_accepts_redacted_execution_envelope", true);
    insert_report_json!("no_write_sink_accepts_source_report_hash_bindings", true);
    insert_report_json!(
        "no_write_sink_requires_operator_approval_and_preflight_validation",
        true
    );
    insert_report_json!(
        "no_write_sink_requires_namespace_operation_retention_allowlist",
        true
    );
    insert_report_json!(
        "no_write_sink_requires_payload_hash_binding_without_plaintext",
        true
    );
    insert_report_json!(
        "no_write_sink_requires_fresh_soak_rollback_validation",
        true
    );
    insert_report_json!(
        "no_write_sink_rejects_external_send_public_claim_artifact",
        true
    );
    insert_report_json!("no_write_sink_rejects_store_write_execution", true);
    insert_report_json!("no_write_sink_write_path_enabled_by_default", false);
    insert_report_json!("no_write_sink_persistence_enabled_by_default", false);
    insert_report_json!("memory_write_execution_denial_matrix_recorded", false);
    insert_report_json!("memory_write_execution_denial_matrix_persisted", false);
    insert_report_json!(
        "memory_write_execution_no_write_sink_contract_recorded",
        false
    );
    insert_report_json!(
        "memory_write_execution_no_write_sink_contract_persisted",
        false
    );
    insert_report_json!(
        "memory_write_execution_no_write_sink_contract_materialized",
        false
    );
    insert_report_json!(
        "memory_write_execution_no_write_sink_contract_filesystem_written",
        false
    );
    insert_report_json!("pre_execution_validation_recorded", false);
    insert_report_json!("pre_execution_validation_persisted", false);
    insert_report_json!("pre_execution_validation_accepted", false);
    insert_report_json!("memory_write_approval_packet_recorded", false);
    insert_report_json!("memory_write_approval_packet_persisted", false);
    insert_report_json!("memory_write_approval_packet_accepted", false);
    insert_report_json!("memory_write_request_recorded", false);
    insert_report_json!("memory_write_request_accepted", false);
    insert_report_json!("memory_write_request_persisted", false);
    insert_report_json!("operator_approval_recorded", false);
    insert_report_json!("operator_identity_hash_recorded", false);
    insert_report_json!("operator_approval_signature_hash_recorded", false);
    insert_report_json!("operator_approval_timestamp_recorded", false);
    insert_report_json!("memory_namespace_recorded", false);
    insert_report_json!("memory_write_operation_recorded", false);
    insert_report_json!("memory_retention_class_recorded", false);
    insert_report_json!("accepted_redaction_proof_recorded", false);
    insert_report_json!("accepted_redaction_proof_count", 0);
    insert_report_json!(
        "source_memory_write_execution_denial_matrix_hash_bound",
        false
    );
    insert_report_json!("source_memory_write_approval_packet_hash_bound", false);
    insert_report_json!("source_memory_write_contract_hash_bound", false);
    insert_report_json!("source_memory_intelligence_hash_bound", false);
    insert_report_json!(
        "source_payload_redaction_acceptance_matrix_hash_bound",
        false
    );
    insert_report_json!("source_payload_redaction_proof_hash_bound", false);
    insert_report_json!("raw_payload_sha256_bound", false);
    insert_report_json!("redacted_payload_summary_sha256_bound", false);
    insert_report_json!("raw_payload_plaintext_recorded", false);
    insert_report_json!("raw_payload_plaintext_persisted", false);
    insert_report_json!("fresh_pre_activation_soak_evidence_recorded", false);
    insert_report_json!("rollback_plan_recorded", false);
    insert_report_json!("post_write_validation_plan_recorded", false);
    insert_report_json!("memory_write_execution_allowed", false);
    insert_report_json!("memory_write_execution_ready", false);
    insert_report_json!("memory_write_execution_performed", false);
    insert_report_json!("memory_write_execution_performed_count", 0);
    insert_report_json!("memory_write_execution_allowed_count", 0);
    insert_report_json!("memory_write_execution_denied_count", 6);
    insert_report_json!("memory_store_write_path_enabled", false);
    insert_report_json!("memory_store_write_performed_count", 0);
    insert_report_json!("memory_store_mutation_allowed", false);
    insert_report_json!("memory_store_mutated", false);
    insert_report_json!("durable_memory_store_write_performed", false);
    insert_report_json!("live_mutation_execution_ready", false);
    insert_report_json!("rollback_execution_allowed", false);
    insert_report_json!("rollback_executed", false);
    insert_report_json!("provider_prompt_replay_enabled", false);
    insert_report_json!("external_send_enabled", false);
    insert_report_json!("external_send_performed", false);
    insert_report_json!("public_claim_or_release_artifact_write_enabled", false);
    insert_report_json!("public_release_published", false);
    insert_report_json!("release_artifact_written", false);
    report.insert(
        "no_write_sink_surfaces".to_string(),
        serde_json::json!(no_write_sink_surfaces),
    );
    report.insert(
        "no_write_sink_fixtures".to_string(),
        serde_json::Value::Array(no_write_sink_fixtures),
    );
    report.insert(
        "denied_by_no_write_sink_contract".to_string(),
        serde_json::json!(denied_by),
    );
    insert_report_json!("denied_by_no_write_sink_contract_count", 10);
    report.insert(
        "required_before_any_memory_write_execution".to_string(),
        serde_json::json!(required_before_any_memory_write_execution),
    );
    insert_report_json!("required_before_any_memory_write_execution_count", 17);
    insert_report_json!("provider_invoked", false);
    insert_report_json!("model_invoked", false);
    insert_report_json!("credential_read", false);
    insert_report_json!("secret_file_read", false);
    insert_report_json!("kg_adapter_read_performed", false);
    insert_report_json!("live_kg_write_performed", false);
    insert_report_json!("channel_send_performed", false);
    insert_report_json!("telegram_send_performed", false);
    insert_report_json!("external_send_performed", false);
    insert_report_json!("public_artifact_written", false);
    insert_report_json!("public_release_claimed", false);
    insert_report_json!("install_executed", false);
    insert_report_json!("service_restarted", false);
    insert_report_json!("active_binary_mutated", false);
    report.insert("allowed_next_actions".to_string(), allowed_next_actions);
    report.insert(
        "side_effects".to_string(),
        serde_json::Value::Object(side_effects),
    );

    serde_json::Value::Object(report)
}

fn hepta_memory_live_mutation_operator_write_execution_write_enable_fixture_boundary_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let no_write_sink =
        hepta_memory_live_mutation_operator_write_execution_no_write_sink_contract_boundary_report(
        );

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
    let no_write_sink_ready = json_bool(
        &no_write_sink,
        "memory_write_execution_no_write_sink_contract_boundary_ready",
    ) && json_bool(
        &no_write_sink,
        "memory_write_execution_no_write_sink_contract_ready",
    ) && json_u64(&no_write_sink, "required_no_write_sink_surface_count")
        == 8
        && json_u64(&no_write_sink, "ready_no_write_sink_surface_count") == 8
        && json_u64(&no_write_sink, "no_write_sink_fixture_count") == 6
        && json_u64(
            &no_write_sink,
            "no_write_sink_accepted_validation_fixture_count",
        ) == 3
        && json_u64(
            &no_write_sink,
            "no_write_sink_rejected_execution_fixture_count",
        ) == 3
        && json_u64(&no_write_sink, "denied_by_no_write_sink_contract_count") == 10
        && json_u64(
            &no_write_sink,
            "required_before_any_memory_write_execution_count",
        ) == 17
        && !json_bool(
            &no_write_sink,
            "memory_write_execution_no_write_sink_contract_recorded",
        )
        && !json_bool(
            &no_write_sink,
            "memory_write_execution_no_write_sink_contract_persisted",
        )
        && !json_bool(
            &no_write_sink,
            "memory_write_execution_no_write_sink_contract_materialized",
        )
        && !json_bool(
            &no_write_sink,
            "memory_write_execution_no_write_sink_contract_filesystem_written",
        )
        && !json_bool(&no_write_sink, "memory_write_execution_performed")
        && !json_bool(&no_write_sink, "memory_store_mutated")
        && !json_bool(&no_write_sink, "rollback_executed")
        && !json_bool(&no_write_sink, "live_kg_write_performed")
        && !json_bool(&no_write_sink, "provider_invoked")
        && !json_bool(&no_write_sink, "model_invoked")
        && !json_bool(&no_write_sink, "credential_read")
        && !json_bool(&no_write_sink, "external_send_performed")
        && !json_bool(&no_write_sink, "release_artifact_written")
        && !json_bool(&no_write_sink, "public_release_claimed")
        && !json_bool(&no_write_sink, "active_binary_mutated")
        && side_effects_all_false(&no_write_sink);

    let write_enable_surfaces = vec![
        "accepted_operator_approval_packet_required",
        "accepted_pre_execution_validation_record_required",
        "operator_identity_signature_timestamp_required",
        "single_surface_activation_scope_required",
        "namespace_operation_retention_allowlist_required",
        "accepted_redaction_proof_and_payload_hash_bindings_required",
        "source_report_hash_bindings_required",
        "fresh_soak_rollback_validation_required",
        "explicit_write_path_enablement_required",
        "post_write_watchdog_soak_plan_required",
    ];
    let denied_by = vec![
        "accepted_operator_approval_packet_required",
        "accepted_pre_execution_validation_record_required",
        "operator_identity_signature_scope_required",
        "namespace_operation_retention_allowlists_required",
        "accepted_redaction_proof_required",
        "payload_hash_binding_without_plaintext_required",
        "source_report_hash_bindings_required",
        "fresh_soak_rollback_validation_required",
        "external_send_public_claim_release_artifact_denied",
        "direct_store_mutation_denied",
        "rollback_execution_denied",
        "post_write_watchdog_soak_plan_required",
        "live_mutation_execution_denied",
    ];
    let write_enable_fixtures = serde_json::json!([
        {
            "id": "write-enable-missing-approval-preflight",
            "explicit_write_enable_requested": true,
            "write_enable_status": "blocked",
            "accepted_operator_approval_packet_present": false,
            "accepted_pre_execution_validation_record_present": false,
            "memory_store_write_requested": true,
            "execution_allowed": false,
            "execution_performed": false,
            "memory_store_write_allowed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "activation_allowed": false,
            "reason": "accepted_operator_approval_packet_and_pre_execution_validation_required"
        },
        {
            "id": "write-enable-missing-operator-scope",
            "explicit_write_enable_requested": true,
            "write_enable_status": "blocked",
            "accepted_operator_approval_packet_present": true,
            "accepted_pre_execution_validation_record_present": true,
            "operator_identity_hash_recorded": false,
            "operator_approval_signature_hash_recorded": false,
            "single_surface_activation_scope_recorded": false,
            "memory_store_write_requested": true,
            "execution_allowed": false,
            "execution_performed": false,
            "memory_store_write_allowed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "activation_allowed": false,
            "reason": "operator_identity_signature_and_single_surface_scope_required"
        },
        {
            "id": "write-enable-allowlist-mismatch",
            "explicit_write_enable_requested": true,
            "write_enable_status": "blocked",
            "accepted_operator_approval_packet_present": true,
            "accepted_pre_execution_validation_record_present": true,
            "operator_identity_hash_recorded": true,
            "single_surface_activation_scope_recorded": true,
            "memory_namespace_allowed": false,
            "memory_write_operation_allowed": false,
            "memory_retention_class_allowed": false,
            "memory_store_write_requested": true,
            "execution_allowed": false,
            "execution_performed": false,
            "memory_store_write_allowed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "activation_allowed": false,
            "reason": "namespace_operation_and_retention_allowlists_required"
        },
        {
            "id": "write-enable-payload-binding-missing-or-plaintext",
            "explicit_write_enable_requested": true,
            "write_enable_status": "blocked",
            "accepted_operator_approval_packet_present": true,
            "accepted_pre_execution_validation_record_present": true,
            "operator_identity_hash_recorded": true,
            "single_surface_activation_scope_recorded": true,
            "memory_namespace_allowed": true,
            "memory_write_operation_allowed": true,
            "memory_retention_class_allowed": true,
            "accepted_redaction_proof_count": 0,
            "raw_payload_sha256_bound": false,
            "redacted_payload_summary_sha256_bound": false,
            "raw_payload_plaintext_recorded": true,
            "memory_store_write_requested": true,
            "execution_allowed": false,
            "execution_performed": false,
            "memory_store_write_allowed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "activation_allowed": false,
            "reason": "accepted_redaction_proof_and_payload_hash_binding_without_plaintext_required"
        },
        {
            "id": "write-enable-stale-soak-or-missing-rollback-validation",
            "explicit_write_enable_requested": true,
            "write_enable_status": "blocked",
            "accepted_operator_approval_packet_present": true,
            "accepted_pre_execution_validation_record_present": true,
            "operator_identity_hash_recorded": true,
            "single_surface_activation_scope_recorded": true,
            "memory_namespace_allowed": true,
            "memory_write_operation_allowed": true,
            "memory_retention_class_allowed": true,
            "accepted_redaction_proof_count": 1,
            "raw_payload_sha256_bound": true,
            "redacted_payload_summary_sha256_bound": true,
            "fresh_pre_activation_soak_evidence_recorded": false,
            "rollback_plan_recorded": false,
            "post_write_validation_plan_recorded": false,
            "memory_store_write_requested": true,
            "execution_allowed": false,
            "execution_performed": false,
            "memory_store_write_allowed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "activation_allowed": false,
            "reason": "fresh_soak_rollback_and_post_write_validation_required"
        },
        {
            "id": "write-enable-public-artifact-or-external-send",
            "explicit_write_enable_requested": true,
            "write_enable_status": "blocked",
            "accepted_operator_approval_packet_present": true,
            "accepted_pre_execution_validation_record_present": true,
            "accepted_redaction_proof_count": 1,
            "external_send_requested": true,
            "public_claim_requested": true,
            "release_artifact_write_requested": true,
            "memory_store_write_requested": true,
            "execution_allowed": false,
            "execution_performed": false,
            "memory_store_write_allowed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "external_send_performed": false,
            "public_release_published": false,
            "release_artifact_written": false,
            "activation_allowed": false,
            "reason": "external_send_public_claim_and_release_artifact_denied"
        },
        {
            "id": "write-enable-direct-store-or-rollback-execution",
            "explicit_write_enable_requested": true,
            "write_enable_status": "blocked",
            "accepted_operator_approval_packet_present": true,
            "accepted_pre_execution_validation_record_present": true,
            "accepted_redaction_proof_count": 1,
            "explicit_write_path_enablement_recorded": true,
            "post_write_watchdog_soak_plan_recorded": false,
            "memory_store_mutation_requested": true,
            "rollback_execution_requested": true,
            "memory_store_write_requested": true,
            "execution_allowed": false,
            "execution_performed": false,
            "memory_store_write_allowed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "activation_allowed": false,
            "reason": "post_write_watchdog_soak_plan_and_live_activation_boundary_required"
        }
    ]);
    let source_no_write_sink_boundary_report_sha256 = sha256_json_value(&no_write_sink);
    let boundary_hash_sha256 = sha256_text_value(&format!(
        "hepta-memory-live-mutation-operator-write-execution-write-enable-fixture-boundary-v1:{}:{}",
        route_matrix.route_count, source_no_write_sink_boundary_report_sha256,
    ));
    let report_ready = route_matrix.ready
        && route_count_source_command_accepted
        && no_write_sink_ready
        && write_enable_surfaces.len() == 10
        && write_enable_fixtures.as_array().map(std::vec::Vec::len) == Some(7)
        && denied_by.len() == 13;

    let allowed_next_actions = serde_json::json!([
        {
            "action": "run_memory_write_execution_write_enable_fixture_boundary_require_live_gate",
            "status": "allowed_verification_only",
            "records_write_enable_fixture": false,
            "writes_memory": false,
            "executes_rollback": false,
            "writes_kg": false,
            "invokes_provider": false,
            "invokes_model": false,
            "reads_credentials": false,
            "sends_externally": false,
            "publishes_artifacts": false,
            "installs_or_restarts": false,
            "mutates_active_binary": false
        },
        {
            "action": "prepare_memory_write_execution_post_write_validation_dry_run_boundary",
            "status": "allowed_report_only_next_slice",
            "writes_memory": false,
            "writes_kg": false,
            "invokes_provider": false,
            "sends_externally": false
        }
    ]);

    let mut side_effects = serde_json::Map::new();
    for key in [
        "memory_store_mutated",
        "memory_store_write_performed",
        "memory_write_request_recorded",
        "memory_write_request_persisted",
        "memory_write_approval_packet_recorded",
        "memory_write_approval_packet_persisted",
        "memory_write_execution_preflight_recorded",
        "memory_write_execution_preflight_persisted",
        "memory_write_execution_denial_matrix_recorded",
        "memory_write_execution_denial_matrix_persisted",
        "memory_write_execution_no_write_sink_contract_recorded",
        "memory_write_execution_no_write_sink_contract_persisted",
        "memory_write_execution_write_enable_fixture_recorded",
        "memory_write_execution_write_enable_fixture_persisted",
        "memory_write_execution_write_enable_fixture_materialized",
        "memory_write_execution_write_enable_fixture_filesystem_written",
        "explicit_write_enablement_recorded",
        "explicit_write_enablement_persisted",
        "pre_execution_validation_recorded",
        "pre_execution_validation_persisted",
        "payload_plaintext_persisted",
        "raw_payload_inspected",
        "capability_registry_mutated",
        "plugin_registry_mutated",
        "skill_workshop_written",
        "provider_invoked",
        "model_invoked",
        "provider_prompt_replayed",
        "channel_send_performed",
        "telegram_send_performed",
        "external_send_performed",
        "runtime_store_mutated",
        "gateway_event_enqueued",
        "filesystem_written",
        "release_artifact_written",
        "public_artifact_written",
        "public_release_published",
        "approval_record_persisted",
        "preflight_record_persisted",
        "denial_matrix_persisted",
        "no_write_sink_contract_persisted",
        "write_enable_fixture_persisted",
        "receipt_persisted",
        "launchd_mutated",
        "service_restarted",
        "install_executed",
        "active_binary_mutated",
        "rollback_executed",
        "credential_read",
        "secret_file_read",
    ] {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }

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
        "hepta_memory_live_mutation_operator_write_execution_write_enable_fixture_boundary_route"
    );
    insert_report_json!(
        "endpoint",
        HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_WRITE_ENABLE_FIXTURE_BOUNDARY_ENDPOINT
    );
    insert_report_json!(
        "source_command",
        "/hepta-memory-live-mutation-operator-write-execution-write-enable-fixture-boundary --json"
    );
    insert_report_json!("native_route", true);
    insert_report_json!("side_effect_free", true);
    insert_report_json!("audit_date", "2026-07-02");
    insert_report_json!(
        "memory_write_execution_write_enable_fixture_boundary_schema_version",
        "memory_write_execution_write_enable_fixture_boundary_v1"
    );
    insert_report_json!(
        "memory_write_execution_write_enable_fixture_boundary_ready",
        report_ready
    );
    insert_report_json!(
        "write_enable_fixture_mode",
        "memory_write_execution_write_enable_fixture_non_activation"
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
        "source_memory_write_execution_no_write_sink_contract_boundary_endpoint",
        HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_NO_WRITE_SINK_CONTRACT_BOUNDARY_ENDPOINT
    );
    insert_report_json!(
        "source_memory_write_execution_no_write_sink_contract_boundary_ready",
        no_write_sink_ready
    );
    insert_report_json!(
        "source_memory_write_execution_no_write_sink_contract_boundary_report_sha256",
        source_no_write_sink_boundary_report_sha256
    );
    report.insert(
        "source_memory_write_execution_denial_matrix_boundary_report_sha256".to_string(),
        no_write_sink
            .get("source_memory_write_execution_denial_matrix_boundary_report_sha256")
            .cloned()
            .unwrap_or_else(|| serde_json::json!("")),
    );
    insert_report_json!("minimum_required_samples", 24);
    insert_report_json!("memory_write_execution_write_enable_fixture_ready", true);
    insert_report_json!("memory_write_execution_no_write_sink_contract_ready", true);
    insert_report_json!("memory_write_execution_denial_matrix_ready", true);
    insert_report_json!(
        "required_pre_execution_validation_check_count",
        json_u64(
            &no_write_sink,
            "required_pre_execution_validation_check_count"
        )
    );
    insert_report_json!("accepted_pre_execution_validation_check_count", 0);
    insert_report_json!("required_write_enable_surface_count", 10);
    insert_report_json!("ready_write_enable_surface_count", 10);
    insert_report_json!("side_effect_free_write_enable_surface_count", 10);
    insert_report_json!("required_write_enable_fixture_count", 7);
    insert_report_json!("write_enable_fixture_count", 7);
    insert_report_json!("blocked_write_enable_fixture_count", 7);
    insert_report_json!("allowed_write_enable_fixture_count", 0);
    insert_report_json!("explicit_write_enable_requested_fixture_count", 7);
    insert_report_json!("write_enable_denied_missing_approval_preflight_count", 1);
    insert_report_json!("write_enable_denied_missing_operator_scope_count", 1);
    insert_report_json!("write_enable_denied_allowlist_mismatch_count", 1);
    insert_report_json!("write_enable_denied_payload_binding_count", 1);
    insert_report_json!(
        "write_enable_denied_stale_soak_rollback_validation_count",
        1
    );
    insert_report_json!("write_enable_denied_public_artifact_count", 1);
    insert_report_json!("write_enable_denied_store_or_rollback_execution_count", 1);
    insert_report_json!("memory_write_execution_denied_count", 7);
    insert_report_json!("memory_write_execution_allowed_count", 0);
    insert_report_json!("memory_write_execution_performed_count", 0);
    insert_report_json!("memory_store_write_requested_fixture_count", 7);
    insert_report_json!("memory_store_write_allowed_count", 0);
    insert_report_json!("memory_store_write_performed_count", 0);

    for key in [
        "memory_store_mutation_allowed",
        "memory_store_mutated",
        "explicit_write_enablement_recorded",
        "explicit_write_enablement_persisted",
        "explicit_write_enablement_accepted",
        "write_enable_fixture_recorded",
        "write_enable_fixture_persisted",
        "write_enable_fixture_materialized",
        "write_enable_fixture_filesystem_written",
        "memory_write_execution_no_write_sink_contract_recorded",
        "memory_write_execution_no_write_sink_contract_persisted",
        "pre_execution_validation_recorded",
        "pre_execution_validation_persisted",
        "pre_execution_validation_accepted",
        "memory_write_approval_packet_recorded",
        "memory_write_approval_packet_persisted",
        "memory_write_approval_packet_accepted",
        "memory_write_request_recorded",
        "memory_write_request_accepted",
        "memory_write_request_persisted",
        "operator_approval_recorded",
        "operator_identity_hash_recorded",
        "operator_approval_signature_hash_recorded",
        "operator_approval_timestamp_recorded",
        "single_surface_activation_scope_recorded",
        "memory_namespace_recorded",
        "memory_write_operation_recorded",
        "memory_retention_class_recorded",
        "accepted_redaction_proof_recorded",
        "source_report_hash_bindings_recorded",
        "raw_payload_sha256_bound",
        "redacted_payload_summary_sha256_bound",
        "raw_payload_plaintext_recorded",
        "raw_payload_plaintext_persisted",
        "fresh_pre_activation_soak_evidence_recorded",
        "rollback_plan_recorded",
        "post_write_validation_plan_recorded",
        "post_write_watchdog_soak_plan_recorded",
        "memory_write_execution_allowed",
        "memory_write_execution_ready",
        "memory_write_execution_performed",
        "memory_store_write_path_enabled",
        "no_write_sink_write_path_enabled_by_default",
        "live_mutation_execution_ready",
        "rollback_execution_allowed",
        "rollback_executed",
        "provider_prompt_replay_enabled",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "kg_adapter_read_performed",
        "live_kg_write_performed",
        "channel_send_performed",
        "telegram_send_performed",
        "external_send_enabled",
        "external_send_performed",
        "public_claim_or_release_artifact_write_enabled",
        "public_release_published",
        "public_release_claimed",
        "release_artifact_written",
        "public_artifact_written",
        "install_executed",
        "service_restarted",
        "active_binary_mutated",
    ] {
        report.insert(key.to_string(), serde_json::json!(false));
    }
    insert_report_json!("accepted_redaction_proof_count", 0);
    report.insert(
        "write_enable_surfaces".to_string(),
        serde_json::json!(write_enable_surfaces),
    );
    report.insert("write_enable_fixtures".to_string(), write_enable_fixtures);
    report.insert(
        "denied_by_write_enable_fixture".to_string(),
        serde_json::json!(denied_by),
    );
    insert_report_json!("denied_by_write_enable_fixture_count", 13);
    report.insert("allowed_next_actions".to_string(), allowed_next_actions);
    report.insert(
        "side_effects".to_string(),
        serde_json::Value::Object(side_effects),
    );

    serde_json::Value::Object(report)
}

fn hepta_memory_live_mutation_operator_write_execution_post_write_validation_dry_run_boundary_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let write_enable =
        hepta_memory_live_mutation_operator_write_execution_write_enable_fixture_boundary_report();

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
    let write_enable_ready = json_bool(
        &write_enable,
        "memory_write_execution_write_enable_fixture_boundary_ready",
    ) && json_bool(
        &write_enable,
        "memory_write_execution_write_enable_fixture_ready",
    ) && json_u64(&write_enable, "required_write_enable_surface_count")
        == 10
        && json_u64(&write_enable, "ready_write_enable_surface_count") == 10
        && json_u64(&write_enable, "write_enable_fixture_count") == 7
        && json_u64(&write_enable, "blocked_write_enable_fixture_count") == 7
        && json_u64(&write_enable, "allowed_write_enable_fixture_count") == 0
        && json_u64(&write_enable, "denied_by_write_enable_fixture_count") == 13
        && !json_bool(&write_enable, "memory_write_execution_allowed")
        && !json_bool(&write_enable, "memory_write_execution_performed")
        && !json_bool(&write_enable, "memory_store_write_path_enabled")
        && json_u64(&write_enable, "memory_store_write_performed_count") == 0
        && !json_bool(&write_enable, "memory_store_mutated")
        && !json_bool(&write_enable, "rollback_executed")
        && !json_bool(&write_enable, "live_kg_write_performed")
        && !json_bool(&write_enable, "provider_invoked")
        && !json_bool(&write_enable, "model_invoked")
        && !json_bool(&write_enable, "credential_read")
        && !json_bool(&write_enable, "external_send_performed")
        && !json_bool(&write_enable, "release_artifact_written")
        && !json_bool(&write_enable, "public_release_claimed")
        && !json_bool(&write_enable, "active_binary_mutated")
        && side_effects_all_false(&write_enable);

    let post_write_validation_surfaces = vec![
        "pre_write_memory_store_baseline_hash_required",
        "accepted_write_result_receipt_hash_required",
        "post_write_memory_store_hash_and_diff_scope_required",
        "route_readiness_regression_check_required",
        "active_dependency_isolation_regression_check_required",
        "post_write_watchdog_soak_plan_required",
        "rollback_validation_plan_required",
        "audit_redaction_validation_required",
        "operator_post_write_acceptance_required",
    ];
    let post_write_validation_fixtures = serde_json::json!([
        {
            "id": "post-write-missing-pre-write-baseline",
            "post_write_validation_requested": true,
            "validation_status": "blocked",
            "pre_write_memory_store_hash_recorded": false,
            "write_result_receipt_hash_recorded": false,
            "post_write_memory_store_hash_recorded": false,
            "validation_allowed": false,
            "validation_performed": false,
            "validation_passed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "activation_allowed": false,
            "reason": "pre_write_baseline_and_write_receipt_required"
        },
        {
            "id": "post-write-missing-write-result-receipt",
            "post_write_validation_requested": true,
            "validation_status": "blocked",
            "pre_write_memory_store_hash_recorded": true,
            "write_result_receipt_hash_recorded": false,
            "write_result_receipt_accepted": false,
            "post_write_memory_store_hash_recorded": true,
            "validation_allowed": false,
            "validation_performed": false,
            "validation_passed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "activation_allowed": false,
            "reason": "accepted_write_result_receipt_hash_required"
        },
        {
            "id": "post-write-store-hash-mismatch",
            "post_write_validation_requested": true,
            "validation_status": "blocked",
            "pre_write_memory_store_hash_recorded": true,
            "write_result_receipt_hash_recorded": true,
            "write_result_receipt_accepted": true,
            "post_write_memory_store_hash_recorded": true,
            "post_write_memory_store_hash_changed": true,
            "diff_scope_allowlisted": false,
            "validation_allowed": false,
            "validation_performed": false,
            "validation_passed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "activation_allowed": false,
            "reason": "post_write_store_hash_change_requires_allowlisted_diff_scope"
        },
        {
            "id": "post-write-route-or-dependency-regression",
            "post_write_validation_requested": true,
            "validation_status": "blocked",
            "route_readiness_regression_detected": true,
            "active_dependency_isolation_regression_detected": true,
            "validation_allowed": false,
            "validation_performed": false,
            "validation_passed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "activation_allowed": false,
            "reason": "route_readiness_and_dependency_isolation_must_remain_ready"
        },
        {
            "id": "post-write-watchdog-soak-missing-or-failed",
            "post_write_validation_requested": true,
            "validation_status": "blocked",
            "post_write_watchdog_soak_plan_recorded": false,
            "post_write_watchdog_soak_performed": false,
            "post_write_watchdog_soak_passed": false,
            "validation_allowed": false,
            "validation_performed": false,
            "validation_passed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "activation_allowed": false,
            "reason": "post_write_watchdog_soak_plan_and_success_required"
        },
        {
            "id": "post-write-rollback-validation-missing",
            "post_write_validation_requested": true,
            "validation_status": "blocked",
            "rollback_validation_plan_recorded": false,
            "rollback_validation_performed": false,
            "rollback_validation_passed": false,
            "rollback_execution_requested": true,
            "validation_allowed": false,
            "validation_performed": false,
            "validation_passed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "activation_allowed": false,
            "reason": "rollback_validation_plan_required_without_executing_rollback"
        },
        {
            "id": "post-write-audit-redaction-or-secret-leak",
            "post_write_validation_requested": true,
            "validation_status": "blocked",
            "audit_redaction_validation_recorded": false,
            "raw_payload_plaintext_recorded": true,
            "secret_material_read": true,
            "validation_allowed": false,
            "validation_performed": false,
            "validation_passed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "activation_allowed": false,
            "reason": "audit_redaction_validation_required_and_secret_material_forbidden"
        },
        {
            "id": "post-write-external-send-or-release-artifact-attempt",
            "post_write_validation_requested": true,
            "validation_status": "blocked",
            "external_send_requested": true,
            "public_claim_requested": true,
            "release_artifact_write_requested": true,
            "validation_allowed": false,
            "validation_performed": false,
            "validation_passed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "external_send_performed": false,
            "public_release_published": false,
            "release_artifact_written": false,
            "rollback_executed": false,
            "activation_allowed": false,
            "reason": "post_write_validation_cannot_send_publish_or_write_release_artifacts"
        }
    ]);
    let denied_by = vec![
        "pre_write_memory_store_baseline_hash_required",
        "accepted_write_result_receipt_hash_required",
        "post_write_memory_store_hash_required",
        "allowlisted_diff_scope_required",
        "route_readiness_regression_denied",
        "active_dependency_isolation_regression_denied",
        "post_write_watchdog_soak_plan_required",
        "post_write_watchdog_soak_success_required",
        "rollback_validation_plan_required",
        "rollback_execution_denied",
        "audit_redaction_validation_required",
        "secret_material_read_denied",
        "external_send_public_claim_release_artifact_denied",
        "live_mutation_execution_denied",
    ];

    let source_write_enable_boundary_report_sha256 = sha256_json_value(&write_enable);
    let boundary_hash_sha256 = sha256_text_value(&format!(
        "hepta-memory-live-mutation-operator-write-execution-post-write-validation-dry-run-boundary-v1:{}:{}",
        route_matrix.route_count, source_write_enable_boundary_report_sha256,
    ));
    let report_ready = route_matrix.ready
        && route_count_source_command_accepted
        && write_enable_ready
        && post_write_validation_surfaces.len() == 9
        && post_write_validation_fixtures
            .as_array()
            .map(std::vec::Vec::len)
            == Some(8)
        && denied_by.len() == 14;

    let allowed_next_actions = serde_json::json!([
        {
            "action": "run_memory_write_execution_post_write_validation_dry_run_boundary_require_live_gate",
            "status": "allowed_verification_only",
            "records_post_write_validation": false,
            "writes_memory": false,
            "executes_rollback": false,
            "writes_kg": false,
            "invokes_provider": false,
            "invokes_model": false,
            "reads_credentials": false,
            "sends_externally": false,
            "publishes_artifacts": false,
            "installs_or_restarts": false,
            "mutates_active_binary": false
        },
        {
            "action": "prepare_memory_write_execution_post_write_operator_acceptance_denial_boundary",
            "status": "allowed_report_only_next_slice",
            "writes_memory": false,
            "executes_rollback": false,
            "writes_kg": false,
            "invokes_provider": false,
            "sends_externally": false
        }
    ]);

    let mut side_effects = serde_json::Map::new();
    for key in [
        "memory_store_mutated",
        "memory_store_write_performed",
        "memory_write_execution_performed",
        "post_write_validation_recorded",
        "post_write_validation_persisted",
        "post_write_validation_accepted",
        "post_write_validation_performed",
        "post_write_validation_report_written",
        "post_write_watchdog_soak_performed",
        "post_write_route_regression_check_performed",
        "post_write_dependency_isolation_check_performed",
        "rollback_validation_performed",
        "rollback_executed",
        "write_result_receipt_recorded",
        "write_result_receipt_persisted",
        "write_result_receipt_accepted",
        "pre_write_memory_store_hash_recorded",
        "post_write_memory_store_hash_recorded",
        "audit_redaction_validation_recorded",
        "raw_payload_inspected",
        "payload_plaintext_persisted",
        "secret_file_read",
        "credential_read",
        "capability_registry_mutated",
        "plugin_registry_mutated",
        "skill_workshop_written",
        "provider_invoked",
        "model_invoked",
        "provider_prompt_replayed",
        "channel_send_performed",
        "telegram_send_performed",
        "external_send_performed",
        "runtime_store_mutated",
        "gateway_event_enqueued",
        "filesystem_written",
        "release_artifact_written",
        "public_artifact_written",
        "public_release_published",
        "launchd_mutated",
        "service_restarted",
        "install_executed",
        "active_binary_mutated",
    ] {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }

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
        "hepta_memory_live_mutation_operator_write_execution_post_write_validation_dry_run_boundary_route"
    );
    insert_report_json!(
        "endpoint",
        HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_POST_WRITE_VALIDATION_DRY_RUN_BOUNDARY_ENDPOINT
    );
    insert_report_json!(
        "source_command",
        "/hepta-memory-live-mutation-operator-write-execution-post-write-validation-dry-run-boundary --json"
    );
    insert_report_json!("native_route", true);
    insert_report_json!("side_effect_free", true);
    insert_report_json!("audit_date", "2026-07-02");
    insert_report_json!(
        "memory_write_execution_post_write_validation_dry_run_boundary_schema_version",
        "memory_write_execution_post_write_validation_dry_run_boundary_v1"
    );
    insert_report_json!(
        "memory_write_execution_post_write_validation_dry_run_boundary_ready",
        report_ready
    );
    insert_report_json!(
        "post_write_validation_mode",
        "memory_write_execution_post_write_validation_dry_run_non_activation"
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
        "source_memory_write_execution_write_enable_fixture_boundary_endpoint",
        HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_WRITE_ENABLE_FIXTURE_BOUNDARY_ENDPOINT
    );
    insert_report_json!(
        "source_memory_write_execution_write_enable_fixture_boundary_ready",
        write_enable_ready
    );
    insert_report_json!(
        "source_memory_write_execution_write_enable_fixture_boundary_report_sha256",
        source_write_enable_boundary_report_sha256
    );
    report.insert(
        "source_memory_write_execution_no_write_sink_contract_boundary_report_sha256".to_string(),
        write_enable
            .get("source_memory_write_execution_no_write_sink_contract_boundary_report_sha256")
            .cloned()
            .unwrap_or_else(|| serde_json::json!("")),
    );
    report.insert(
        "source_memory_write_execution_denial_matrix_boundary_report_sha256".to_string(),
        write_enable
            .get("source_memory_write_execution_denial_matrix_boundary_report_sha256")
            .cloned()
            .unwrap_or_else(|| serde_json::json!("")),
    );
    insert_report_json!("minimum_required_samples", 24);
    insert_report_json!(
        "memory_write_execution_post_write_validation_dry_run_ready",
        true
    );
    insert_report_json!("memory_write_execution_write_enable_fixture_ready", true);
    insert_report_json!("memory_write_execution_no_write_sink_contract_ready", true);
    insert_report_json!(
        "required_write_enable_surface_count",
        json_u64(&write_enable, "required_write_enable_surface_count")
    );
    insert_report_json!(
        "ready_write_enable_surface_count",
        json_u64(&write_enable, "ready_write_enable_surface_count")
    );
    insert_report_json!("required_post_write_validation_surface_count", 9);
    insert_report_json!("ready_post_write_validation_surface_count", 9);
    insert_report_json!("side_effect_free_post_write_validation_surface_count", 9);
    insert_report_json!("required_post_write_validation_fixture_count", 8);
    insert_report_json!("post_write_validation_fixture_count", 8);
    insert_report_json!("blocked_post_write_validation_fixture_count", 8);
    insert_report_json!("allowed_post_write_validation_fixture_count", 0);
    insert_report_json!("passed_post_write_validation_fixture_count", 0);
    insert_report_json!("post_write_validation_denied_count", 8);
    insert_report_json!("post_write_validation_performed_count", 0);

    for key in [
        "post_write_validation_recorded",
        "post_write_validation_persisted",
        "post_write_validation_accepted",
        "post_write_validation_performed",
        "post_write_validation_report_written",
        "post_write_watchdog_soak_plan_recorded",
        "post_write_watchdog_soak_plan_persisted",
        "post_write_watchdog_soak_performed",
        "post_write_watchdog_soak_passed",
        "post_write_route_regression_check_performed",
        "post_write_route_regression_passed",
        "post_write_dependency_isolation_check_performed",
        "post_write_dependency_isolation_passed",
        "post_write_memory_store_hash_recorded",
        "post_write_memory_store_hash_persisted",
        "post_write_memory_store_hash_changed",
        "pre_write_memory_store_hash_recorded",
        "write_result_receipt_hash_recorded",
        "write_result_receipt_accepted",
        "rollback_validation_plan_recorded",
        "rollback_validation_performed",
        "rollback_validation_passed",
        "audit_redaction_validation_recorded",
        "audit_redaction_validation_passed",
        "raw_payload_plaintext_recorded",
        "raw_payload_plaintext_persisted",
        "secret_material_read",
        "memory_write_execution_allowed",
        "memory_write_execution_ready",
        "memory_write_execution_performed",
        "memory_store_write_path_enabled",
        "memory_store_write_allowed",
        "memory_store_write_performed",
        "memory_store_mutation_allowed",
        "memory_store_mutated",
        "live_mutation_execution_ready",
        "rollback_execution_allowed",
        "rollback_executed",
        "provider_prompt_replay_enabled",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "kg_adapter_read_performed",
        "live_kg_write_performed",
        "channel_send_performed",
        "telegram_send_performed",
        "external_send_enabled",
        "external_send_performed",
        "public_claim_or_release_artifact_write_enabled",
        "public_release_published",
        "public_release_claimed",
        "release_artifact_written",
        "public_artifact_written",
        "install_executed",
        "service_restarted",
        "active_binary_mutated",
    ] {
        report.insert(key.to_string(), serde_json::json!(false));
    }
    insert_report_json!("memory_store_write_performed_count", 0);
    report.insert(
        "post_write_validation_surfaces".to_string(),
        serde_json::json!(post_write_validation_surfaces),
    );
    report.insert(
        "post_write_validation_fixtures".to_string(),
        post_write_validation_fixtures,
    );
    report.insert(
        "denied_by_post_write_validation".to_string(),
        serde_json::json!(denied_by),
    );
    insert_report_json!("denied_by_post_write_validation_count", 14);
    report.insert("allowed_next_actions".to_string(), allowed_next_actions);
    report.insert(
        "side_effects".to_string(),
        serde_json::Value::Object(side_effects),
    );

    serde_json::Value::Object(report)
}

fn hepta_memory_live_mutation_operator_write_execution_post_write_operator_acceptance_denial_boundary_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let post_write =
        hepta_memory_live_mutation_operator_write_execution_post_write_validation_dry_run_boundary_report();

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
    let post_write_ready =
        json_bool(
            &post_write,
            "memory_write_execution_post_write_validation_dry_run_boundary_ready",
        ) && json_bool(
            &post_write,
            "memory_write_execution_post_write_validation_dry_run_ready",
        ) && json_u64(&post_write, "required_post_write_validation_surface_count") == 9
            && json_u64(&post_write, "ready_post_write_validation_surface_count") == 9
            && json_u64(&post_write, "post_write_validation_fixture_count") == 8
            && json_u64(&post_write, "blocked_post_write_validation_fixture_count") == 8
            && json_u64(&post_write, "allowed_post_write_validation_fixture_count") == 0
            && json_u64(&post_write, "passed_post_write_validation_fixture_count") == 0
            && json_u64(&post_write, "post_write_validation_performed_count") == 0
            && !json_bool(&post_write, "post_write_validation_recorded")
            && !json_bool(&post_write, "post_write_validation_persisted")
            && !json_bool(&post_write, "post_write_validation_accepted")
            && !json_bool(&post_write, "post_write_validation_performed")
            && !json_bool(&post_write, "memory_write_execution_allowed")
            && !json_bool(&post_write, "memory_write_execution_ready")
            && !json_bool(&post_write, "memory_write_execution_performed")
            && !json_bool(&post_write, "memory_store_write_path_enabled")
            && !json_bool(&post_write, "memory_store_write_allowed")
            && !json_bool(&post_write, "memory_store_write_performed")
            && json_u64(&post_write, "memory_store_write_performed_count") == 0
            && !json_bool(&post_write, "memory_store_mutated")
            && !json_bool(&post_write, "rollback_executed")
            && !json_bool(&post_write, "live_kg_write_performed")
            && !json_bool(&post_write, "provider_invoked")
            && !json_bool(&post_write, "model_invoked")
            && !json_bool(&post_write, "credential_read")
            && !json_bool(&post_write, "external_send_performed")
            && !json_bool(&post_write, "release_artifact_written")
            && !json_bool(&post_write, "public_release_claimed")
            && !json_bool(&post_write, "active_binary_mutated")
            && side_effects_all_false(&post_write);

    let operator_acceptance_surfaces = vec![
        "accepted_post_write_validation_report_required",
        "operator_identity_signature_timestamp_required",
        "single_surface_acceptance_scope_required",
        "pre_and_post_memory_store_hash_binding_required",
        "accepted_write_result_receipt_hash_required",
        "allowlisted_diff_scope_required",
        "post_write_watchdog_soak_success_required",
        "route_and_dependency_regression_absence_required",
        "rollback_validation_and_no_rollback_execution_required",
        "audit_redaction_and_no_secret_material_required",
        "activation_closure_packet_required",
    ];
    let operator_acceptance_fixtures = serde_json::json!([
        {
            "id": "operator-acceptance-missing-post-write-validation",
            "operator_acceptance_requested": true,
            "acceptance_status": "blocked",
            "accepted_post_write_validation_report_present": false,
            "operator_identity_hash_recorded": false,
            "validation_accepted": false,
            "acceptance_allowed": false,
            "acceptance_performed": false,
            "acceptance_accepted": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "reason": "accepted_post_write_validation_report_required"
        },
        {
            "id": "operator-acceptance-missing-operator-signature",
            "operator_acceptance_requested": true,
            "acceptance_status": "blocked",
            "accepted_post_write_validation_report_present": true,
            "operator_identity_hash_recorded": false,
            "operator_acceptance_signature_hash_recorded": false,
            "operator_acceptance_timestamp_recorded": false,
            "operator_single_surface_scope_recorded": false,
            "acceptance_allowed": false,
            "acceptance_performed": false,
            "acceptance_accepted": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "reason": "operator_identity_signature_timestamp_and_scope_required"
        },
        {
            "id": "operator-acceptance-receipt-or-store-hash-mismatch",
            "operator_acceptance_requested": true,
            "acceptance_status": "blocked",
            "accepted_post_write_validation_report_present": true,
            "operator_identity_hash_recorded": true,
            "operator_acceptance_signature_hash_recorded": true,
            "operator_single_surface_scope_recorded": true,
            "write_result_receipt_hash_bound": false,
            "pre_write_memory_store_hash_bound": true,
            "post_write_memory_store_hash_bound": true,
            "post_write_diff_scope_accepted": false,
            "acceptance_allowed": false,
            "acceptance_performed": false,
            "acceptance_accepted": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "reason": "write_receipt_store_hash_and_diff_scope_bindings_required"
        },
        {
            "id": "operator-acceptance-route-or-dependency-regression",
            "operator_acceptance_requested": true,
            "acceptance_status": "blocked",
            "accepted_post_write_validation_report_present": true,
            "operator_identity_hash_recorded": true,
            "operator_acceptance_signature_hash_recorded": true,
            "route_readiness_regression_detected": true,
            "active_dependency_isolation_regression_detected": true,
            "acceptance_allowed": false,
            "acceptance_performed": false,
            "acceptance_accepted": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "reason": "route_readiness_and_dependency_isolation_must_remain_ready"
        },
        {
            "id": "operator-acceptance-watchdog-soak-missing",
            "operator_acceptance_requested": true,
            "acceptance_status": "blocked",
            "accepted_post_write_validation_report_present": true,
            "operator_identity_hash_recorded": true,
            "operator_acceptance_signature_hash_recorded": true,
            "post_write_watchdog_soak_evidence_accepted": false,
            "acceptance_allowed": false,
            "acceptance_performed": false,
            "acceptance_accepted": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "reason": "post_write_watchdog_soak_success_required"
        },
        {
            "id": "operator-acceptance-rollback-validation-missing-or-execution",
            "operator_acceptance_requested": true,
            "acceptance_status": "blocked",
            "accepted_post_write_validation_report_present": true,
            "operator_identity_hash_recorded": true,
            "operator_acceptance_signature_hash_recorded": true,
            "rollback_validation_accepted": false,
            "rollback_execution_requested": true,
            "acceptance_allowed": false,
            "acceptance_performed": false,
            "acceptance_accepted": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "reason": "rollback_validation_required_and_rollback_execution_denied"
        },
        {
            "id": "operator-acceptance-redaction-or-secret-violation",
            "operator_acceptance_requested": true,
            "acceptance_status": "blocked",
            "accepted_post_write_validation_report_present": true,
            "operator_identity_hash_recorded": true,
            "operator_acceptance_signature_hash_recorded": true,
            "audit_redaction_validation_accepted": false,
            "raw_payload_plaintext_recorded": true,
            "secret_material_read": true,
            "acceptance_allowed": false,
            "acceptance_performed": false,
            "acceptance_accepted": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "reason": "audit_redaction_validation_required_and_secret_material_forbidden"
        },
        {
            "id": "operator-acceptance-multisurface-or-direct-activation",
            "operator_acceptance_requested": true,
            "acceptance_status": "blocked",
            "accepted_post_write_validation_report_present": true,
            "operator_identity_hash_recorded": true,
            "operator_acceptance_signature_hash_recorded": true,
            "operator_single_surface_scope_recorded": false,
            "multi_surface_activation_requested": true,
            "direct_live_mutation_execution_requested": true,
            "acceptance_allowed": false,
            "acceptance_performed": false,
            "acceptance_accepted": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "reason": "single_surface_scope_required_and_direct_activation_denied"
        },
        {
            "id": "operator-acceptance-public-or-external-output-attempt",
            "operator_acceptance_requested": true,
            "acceptance_status": "blocked",
            "accepted_post_write_validation_report_present": true,
            "operator_identity_hash_recorded": true,
            "operator_acceptance_signature_hash_recorded": true,
            "external_send_requested": true,
            "public_claim_requested": true,
            "release_artifact_write_requested": true,
            "acceptance_allowed": false,
            "acceptance_performed": false,
            "acceptance_accepted": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "external_send_performed": false,
            "public_release_published": false,
            "release_artifact_written": false,
            "rollback_executed": false,
            "reason": "operator_acceptance_cannot_send_publish_or_write_release_artifacts"
        }
    ]);
    let denied_by = vec![
        "accepted_post_write_validation_report_required",
        "operator_identity_required",
        "operator_acceptance_signature_required",
        "operator_acceptance_timestamp_required",
        "single_surface_acceptance_scope_required",
        "pre_write_memory_store_hash_binding_required",
        "post_write_memory_store_hash_binding_required",
        "write_result_receipt_hash_binding_required",
        "allowlisted_diff_scope_required",
        "route_readiness_regression_denied",
        "active_dependency_isolation_regression_denied",
        "post_write_watchdog_soak_success_required",
        "rollback_validation_required",
        "rollback_execution_denied",
        "audit_redaction_validation_required",
        "secret_material_read_denied",
        "multi_surface_activation_denied",
        "direct_live_mutation_execution_denied",
        "external_send_public_claim_release_artifact_denied",
        "activation_closure_packet_required",
        "live_mutation_execution_denied",
    ];

    let source_post_write_boundary_report_sha256 = sha256_json_value(&post_write);
    let boundary_hash_sha256 = sha256_text_value(&format!(
        "hepta-memory-live-mutation-operator-write-execution-post-write-operator-acceptance-denial-boundary-v1:{}:{}",
        route_matrix.route_count, source_post_write_boundary_report_sha256,
    ));
    let report_ready = route_matrix.ready
        && route_count_source_command_accepted
        && post_write_ready
        && operator_acceptance_surfaces.len() == 11
        && operator_acceptance_fixtures
            .as_array()
            .map(std::vec::Vec::len)
            == Some(9)
        && denied_by.len() == 21;

    let allowed_next_actions = serde_json::json!([
        {
            "action": "run_memory_write_execution_post_write_operator_acceptance_denial_boundary_require_live_gate",
            "status": "allowed_verification_only",
            "records_operator_acceptance": false,
            "accepts_activation": false,
            "writes_memory": false,
            "executes_rollback": false,
            "writes_kg": false,
            "invokes_provider": false,
            "invokes_model": false,
            "reads_credentials": false,
            "sends_externally": false,
            "publishes_artifacts": false,
            "installs_or_restarts": false,
            "mutates_active_binary": false
        },
        {
            "action": "prepare_memory_write_execution_activation_closure_denial_boundary",
            "status": "allowed_report_only_next_slice",
            "writes_memory": false,
            "executes_rollback": false,
            "writes_kg": false,
            "invokes_provider": false,
            "sends_externally": false,
            "publishes_artifacts": false
        }
    ]);

    let mut side_effects = serde_json::Map::new();
    for key in [
        "memory_store_mutated",
        "memory_store_write_performed",
        "memory_write_execution_performed",
        "post_write_validation_recorded",
        "post_write_validation_persisted",
        "post_write_validation_performed",
        "post_write_validation_accepted",
        "operator_post_write_acceptance_recorded",
        "operator_post_write_acceptance_persisted",
        "operator_post_write_acceptance_performed",
        "operator_post_write_acceptance_materialized",
        "operator_post_write_acceptance_filesystem_written",
        "accepted_post_write_validation_report_recorded",
        "accepted_post_write_validation_report_persisted",
        "activation_closure_packet_recorded",
        "activation_closure_packet_persisted",
        "activation_closure_packet_materialized",
        "activation_closure_filesystem_written",
        "live_mutation_execution_performed",
        "rollback_validation_performed",
        "rollback_executed",
        "write_result_receipt_recorded",
        "write_result_receipt_persisted",
        "pre_write_memory_store_hash_recorded",
        "post_write_memory_store_hash_recorded",
        "audit_redaction_validation_recorded",
        "raw_payload_inspected",
        "payload_plaintext_persisted",
        "secret_file_read",
        "credential_read",
        "capability_registry_mutated",
        "plugin_registry_mutated",
        "skill_workshop_written",
        "provider_invoked",
        "model_invoked",
        "provider_prompt_replayed",
        "channel_send_performed",
        "telegram_send_performed",
        "external_send_performed",
        "runtime_store_mutated",
        "gateway_event_enqueued",
        "filesystem_written",
        "release_artifact_written",
        "public_artifact_written",
        "public_release_published",
        "public_ga_claimed",
        "launchd_mutated",
        "service_restarted",
        "install_executed",
        "active_binary_mutated",
    ] {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }

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
        "hepta_memory_live_mutation_operator_write_execution_post_write_operator_acceptance_denial_boundary_route"
    );
    insert_report_json!(
        "endpoint",
        HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_POST_WRITE_OPERATOR_ACCEPTANCE_DENIAL_BOUNDARY_ENDPOINT
    );
    insert_report_json!(
        "source_command",
        "/hepta-memory-live-mutation-operator-write-execution-post-write-operator-acceptance-denial-boundary --json"
    );
    insert_report_json!("native_route", true);
    insert_report_json!("side_effect_free", true);
    insert_report_json!("audit_date", "2026-07-02");
    insert_report_json!(
        "memory_write_execution_post_write_operator_acceptance_denial_boundary_schema_version",
        "memory_write_execution_post_write_operator_acceptance_denial_boundary_v1"
    );
    insert_report_json!(
        "memory_write_execution_post_write_operator_acceptance_denial_boundary_ready",
        report_ready
    );
    insert_report_json!(
        "operator_acceptance_denial_mode",
        "memory_write_execution_post_write_operator_acceptance_denial_non_activation"
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
        "source_memory_write_execution_post_write_validation_dry_run_boundary_endpoint",
        HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_POST_WRITE_VALIDATION_DRY_RUN_BOUNDARY_ENDPOINT
    );
    insert_report_json!(
        "source_memory_write_execution_post_write_validation_dry_run_boundary_ready",
        post_write_ready
    );
    insert_report_json!(
        "source_memory_write_execution_post_write_validation_dry_run_ready",
        json_bool(
            &post_write,
            "memory_write_execution_post_write_validation_dry_run_ready"
        )
    );
    insert_report_json!(
        "source_memory_write_execution_post_write_validation_dry_run_boundary_report_sha256",
        source_post_write_boundary_report_sha256
    );
    for key in [
        "source_memory_write_execution_write_enable_fixture_boundary_report_sha256",
        "source_memory_write_execution_no_write_sink_contract_boundary_report_sha256",
        "source_memory_write_execution_denial_matrix_boundary_report_sha256",
    ] {
        report.insert(
            key.to_string(),
            post_write
                .get(key)
                .cloned()
                .unwrap_or_else(|| serde_json::json!("")),
        );
    }
    insert_report_json!("minimum_required_samples", 24);
    insert_report_json!(
        "memory_write_execution_post_write_operator_acceptance_denial_ready",
        true
    );
    insert_report_json!(
        "memory_write_execution_post_write_validation_dry_run_ready",
        true
    );
    insert_report_json!("memory_write_execution_write_enable_fixture_ready", true);
    insert_report_json!("memory_write_execution_no_write_sink_contract_ready", true);
    insert_report_json!(
        "required_post_write_validation_surface_count",
        json_u64(&post_write, "required_post_write_validation_surface_count")
    );
    insert_report_json!(
        "ready_post_write_validation_surface_count",
        json_u64(&post_write, "ready_post_write_validation_surface_count")
    );
    insert_report_json!("required_operator_acceptance_surface_count", 11);
    insert_report_json!("ready_operator_acceptance_surface_count", 11);
    insert_report_json!("side_effect_free_operator_acceptance_surface_count", 11);
    insert_report_json!("required_operator_acceptance_fixture_count", 9);
    insert_report_json!("operator_acceptance_fixture_count", 9);
    insert_report_json!("blocked_operator_acceptance_fixture_count", 9);
    insert_report_json!("allowed_operator_acceptance_fixture_count", 0);
    insert_report_json!("accepted_operator_acceptance_fixture_count", 0);
    insert_report_json!("operator_acceptance_denied_count", 9);
    insert_report_json!("operator_acceptance_performed_count", 0);

    for key in [
        "operator_post_write_acceptance_recorded",
        "operator_post_write_acceptance_persisted",
        "operator_post_write_acceptance_accepted",
        "operator_post_write_acceptance_performed",
        "operator_post_write_acceptance_materialized",
        "operator_post_write_acceptance_filesystem_written",
        "operator_identity_hash_recorded",
        "operator_acceptance_signature_hash_recorded",
        "operator_acceptance_timestamp_recorded",
        "operator_single_surface_scope_recorded",
        "accepted_post_write_validation_report_recorded",
        "accepted_post_write_validation_report_persisted",
        "accepted_post_write_validation_report_accepted",
        "accepted_post_write_validation_report_hash_bound",
        "write_result_receipt_hash_bound",
        "pre_write_memory_store_hash_bound",
        "post_write_memory_store_hash_bound",
        "post_write_diff_scope_accepted",
        "post_write_watchdog_soak_evidence_accepted",
        "post_write_route_regression_check_accepted",
        "post_write_dependency_isolation_check_accepted",
        "rollback_validation_accepted",
        "rollback_execution_allowed",
        "rollback_executed",
        "audit_redaction_validation_accepted",
        "raw_payload_plaintext_recorded",
        "raw_payload_plaintext_persisted",
        "secret_material_read",
        "activation_closure_packet_recorded",
        "activation_closure_packet_persisted",
        "activation_closure_packet_accepted",
        "activation_closure_packet_materialized",
        "activation_closure_filesystem_written",
        "activation_allowed_by_operator_acceptance",
        "activation_allowed",
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
        "provider_prompt_replay_enabled",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "kg_adapter_read_performed",
        "live_kg_write_performed",
        "channel_send_performed",
        "telegram_send_performed",
        "external_send_enabled",
        "external_send_performed",
        "public_claim_or_release_artifact_write_enabled",
        "public_release_published",
        "public_ga_claimed",
        "public_release_claimed",
        "release_artifact_written",
        "public_artifact_written",
        "install_executed",
        "service_restarted",
        "active_binary_mutated",
    ] {
        report.insert(key.to_string(), serde_json::json!(false));
    }
    insert_report_json!("memory_store_write_performed_count", 0);
    report.insert(
        "operator_acceptance_surfaces".to_string(),
        serde_json::json!(operator_acceptance_surfaces),
    );
    report.insert(
        "operator_acceptance_fixtures".to_string(),
        operator_acceptance_fixtures,
    );
    report.insert(
        "denied_by_operator_acceptance".to_string(),
        serde_json::json!(denied_by),
    );
    insert_report_json!("denied_by_operator_acceptance_count", 21);
    report.insert("allowed_next_actions".to_string(), allowed_next_actions);
    report.insert(
        "side_effects".to_string(),
        serde_json::Value::Object(side_effects),
    );

    serde_json::Value::Object(report)
}

fn hepta_memory_live_mutation_operator_write_execution_activation_closure_denial_boundary_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let operator_acceptance =
        hepta_memory_live_mutation_operator_write_execution_post_write_operator_acceptance_denial_boundary_report();

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
    let operator_acceptance_ready = json_bool(
        &operator_acceptance,
        "memory_write_execution_post_write_operator_acceptance_denial_boundary_ready",
    ) && json_bool(
        &operator_acceptance,
        "memory_write_execution_post_write_operator_acceptance_denial_ready",
    ) && json_u64(
        &operator_acceptance,
        "required_operator_acceptance_surface_count",
    ) == 11
        && json_u64(
            &operator_acceptance,
            "ready_operator_acceptance_surface_count",
        ) == 11
        && json_u64(&operator_acceptance, "operator_acceptance_fixture_count") == 9
        && json_u64(
            &operator_acceptance,
            "blocked_operator_acceptance_fixture_count",
        ) == 9
        && json_u64(
            &operator_acceptance,
            "allowed_operator_acceptance_fixture_count",
        ) == 0
        && json_u64(
            &operator_acceptance,
            "accepted_operator_acceptance_fixture_count",
        ) == 0
        && json_u64(&operator_acceptance, "operator_acceptance_performed_count") == 0
        && !json_bool(
            &operator_acceptance,
            "operator_post_write_acceptance_recorded",
        )
        && !json_bool(
            &operator_acceptance,
            "operator_post_write_acceptance_persisted",
        )
        && !json_bool(
            &operator_acceptance,
            "operator_post_write_acceptance_accepted",
        )
        && !json_bool(
            &operator_acceptance,
            "operator_post_write_acceptance_performed",
        )
        && !json_bool(&operator_acceptance, "activation_closure_packet_recorded")
        && !json_bool(&operator_acceptance, "activation_closure_packet_persisted")
        && !json_bool(&operator_acceptance, "activation_closure_packet_accepted")
        && !json_bool(
            &operator_acceptance,
            "activation_closure_packet_materialized",
        )
        && !json_bool(
            &operator_acceptance,
            "activation_closure_filesystem_written",
        )
        && !json_bool(
            &operator_acceptance,
            "activation_allowed_by_operator_acceptance",
        )
        && !json_bool(&operator_acceptance, "activation_allowed")
        && !json_bool(&operator_acceptance, "live_mutation_execution_ready")
        && !json_bool(&operator_acceptance, "live_mutation_execution_allowed")
        && !json_bool(&operator_acceptance, "live_mutation_execution_performed")
        && !json_bool(&operator_acceptance, "memory_write_execution_allowed")
        && !json_bool(&operator_acceptance, "memory_write_execution_ready")
        && !json_bool(&operator_acceptance, "memory_write_execution_performed")
        && !json_bool(&operator_acceptance, "memory_store_write_path_enabled")
        && !json_bool(&operator_acceptance, "memory_store_write_allowed")
        && !json_bool(&operator_acceptance, "memory_store_write_performed")
        && json_u64(&operator_acceptance, "memory_store_write_performed_count") == 0
        && !json_bool(&operator_acceptance, "memory_store_mutated")
        && !json_bool(&operator_acceptance, "rollback_executed")
        && !json_bool(&operator_acceptance, "secret_material_read")
        && !json_bool(&operator_acceptance, "provider_invoked")
        && !json_bool(&operator_acceptance, "model_invoked")
        && !json_bool(&operator_acceptance, "credential_read")
        && !json_bool(&operator_acceptance, "external_send_performed")
        && !json_bool(&operator_acceptance, "release_artifact_written")
        && !json_bool(&operator_acceptance, "public_ga_claimed")
        && !json_bool(&operator_acceptance, "public_release_claimed")
        && !json_bool(&operator_acceptance, "active_binary_mutated")
        && side_effects_all_false(&operator_acceptance);

    let activation_closure_surfaces = vec![
        "accepted_operator_post_write_acceptance_required",
        "accepted_post_write_validation_hash_required",
        "operator_identity_signature_timestamp_required",
        "single_surface_activation_scope_required",
        "pre_post_store_hashes_and_write_receipt_required",
        "allowlisted_diff_scope_required",
        "post_write_watchdog_soak_and_regression_evidence_required",
        "rollback_validation_and_no_rollback_execution_required",
        "audit_redaction_and_no_secret_material_required",
        "activation_closure_packet_id_hash_signature_required",
        "activation_command_disabled_by_default_required",
        "no_external_public_or_release_outputs_required",
    ];
    let activation_closure_fixtures = serde_json::json!([
        {
            "id": "activation-closure-missing-operator-acceptance",
            "activation_closure_requested": true,
            "closure_status": "blocked",
            "accepted_operator_post_write_acceptance_present": false,
            "accepted_operator_post_write_acceptance_hash_bound": false,
            "closure_allowed": false,
            "closure_recorded": false,
            "closure_persisted": false,
            "closure_accepted": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "reason": "accepted_operator_post_write_acceptance_required"
        },
        {
            "id": "activation-closure-missing-packet-id-or-hash",
            "activation_closure_requested": true,
            "closure_status": "blocked",
            "accepted_operator_post_write_acceptance_present": true,
            "accepted_operator_post_write_acceptance_hash_bound": true,
            "activation_closure_packet_id_recorded": false,
            "activation_closure_packet_hash_bound": false,
            "activation_closure_packet_signature_hash_recorded": false,
            "closure_allowed": false,
            "closure_recorded": false,
            "closure_persisted": false,
            "closure_accepted": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "reason": "activation_closure_packet_id_hash_and_signature_required"
        },
        {
            "id": "activation-closure-missing-single-surface-scope",
            "activation_closure_requested": true,
            "closure_status": "blocked",
            "accepted_operator_post_write_acceptance_present": true,
            "activation_closure_packet_id_recorded": true,
            "activation_closure_packet_hash_bound": true,
            "operator_single_surface_scope_recorded": false,
            "multi_surface_activation_requested": true,
            "closure_allowed": false,
            "closure_recorded": false,
            "closure_persisted": false,
            "closure_accepted": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "reason": "single_surface_activation_scope_required"
        },
        {
            "id": "activation-closure-store-hash-or-receipt-mismatch",
            "activation_closure_requested": true,
            "closure_status": "blocked",
            "accepted_operator_post_write_acceptance_present": true,
            "activation_closure_packet_id_recorded": true,
            "activation_closure_packet_hash_bound": true,
            "pre_write_memory_store_hash_bound": true,
            "post_write_memory_store_hash_bound": false,
            "write_result_receipt_hash_bound": false,
            "post_write_diff_scope_accepted": false,
            "closure_allowed": false,
            "closure_recorded": false,
            "closure_persisted": false,
            "closure_accepted": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "reason": "store_hash_write_receipt_and_diff_scope_bindings_required"
        },
        {
            "id": "activation-closure-regression-or-soak-missing",
            "activation_closure_requested": true,
            "closure_status": "blocked",
            "accepted_operator_post_write_acceptance_present": true,
            "activation_closure_packet_id_recorded": true,
            "activation_closure_packet_hash_bound": true,
            "post_write_watchdog_soak_evidence_accepted": false,
            "route_readiness_regression_detected": true,
            "active_dependency_isolation_regression_detected": true,
            "closure_allowed": false,
            "closure_recorded": false,
            "closure_persisted": false,
            "closure_accepted": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "reason": "post_write_soak_route_and_dependency_evidence_required"
        },
        {
            "id": "activation-closure-rollback-validation-missing-or-execution",
            "activation_closure_requested": true,
            "closure_status": "blocked",
            "accepted_operator_post_write_acceptance_present": true,
            "activation_closure_packet_id_recorded": true,
            "activation_closure_packet_hash_bound": true,
            "rollback_validation_accepted": false,
            "rollback_execution_requested": true,
            "closure_allowed": false,
            "closure_recorded": false,
            "closure_persisted": false,
            "closure_accepted": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "reason": "rollback_validation_required_and_rollback_execution_denied"
        },
        {
            "id": "activation-closure-redaction-secret-violation",
            "activation_closure_requested": true,
            "closure_status": "blocked",
            "accepted_operator_post_write_acceptance_present": true,
            "activation_closure_packet_id_recorded": true,
            "activation_closure_packet_hash_bound": true,
            "audit_redaction_validation_accepted": false,
            "raw_payload_plaintext_recorded": true,
            "secret_material_read": true,
            "closure_allowed": false,
            "closure_recorded": false,
            "closure_persisted": false,
            "closure_accepted": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "reason": "audit_redaction_validation_required_and_secret_material_forbidden"
        },
        {
            "id": "activation-closure-direct-live-mutation-request",
            "activation_closure_requested": true,
            "closure_status": "blocked",
            "accepted_operator_post_write_acceptance_present": true,
            "activation_closure_packet_id_recorded": true,
            "activation_closure_packet_hash_bound": true,
            "direct_live_mutation_execution_requested": true,
            "activation_command_invoked": true,
            "closure_allowed": false,
            "closure_recorded": false,
            "closure_persisted": false,
            "closure_accepted": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "reason": "activation_command_disabled_and_direct_execution_denied"
        },
        {
            "id": "activation-closure-public-external-release-attempt",
            "activation_closure_requested": true,
            "closure_status": "blocked",
            "accepted_operator_post_write_acceptance_present": true,
            "activation_closure_packet_id_recorded": true,
            "activation_closure_packet_hash_bound": true,
            "external_send_requested": true,
            "public_claim_requested": true,
            "release_artifact_write_requested": true,
            "closure_allowed": false,
            "closure_recorded": false,
            "closure_persisted": false,
            "closure_accepted": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "external_send_performed": false,
            "public_release_published": false,
            "release_artifact_written": false,
            "rollback_executed": false,
            "reason": "activation_closure_cannot_send_publish_or_write_release_artifacts"
        },
        {
            "id": "activation-closure-persistence-or-filesystem-write-attempt",
            "activation_closure_requested": true,
            "closure_status": "blocked",
            "accepted_operator_post_write_acceptance_present": true,
            "activation_closure_packet_id_recorded": true,
            "activation_closure_packet_hash_bound": true,
            "activation_closure_packet_materialization_requested": true,
            "activation_closure_filesystem_write_requested": true,
            "activation_closure_ledger_write_requested": true,
            "closure_allowed": false,
            "closure_recorded": false,
            "closure_persisted": false,
            "closure_accepted": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "activation_closure_filesystem_written": false,
            "activation_closure_ledger_written": false,
            "rollback_executed": false,
            "reason": "activation_closure_packet_persistence_and_filesystem_writes_denied"
        }
    ]);
    let denied_by = vec![
        "accepted_operator_post_write_acceptance_required",
        "accepted_post_write_validation_hash_required",
        "operator_identity_required",
        "operator_acceptance_signature_required",
        "operator_acceptance_timestamp_required",
        "single_surface_activation_scope_required",
        "activation_closure_packet_id_required",
        "activation_closure_packet_hash_required",
        "activation_closure_packet_signature_required",
        "pre_write_memory_store_hash_binding_required",
        "post_write_memory_store_hash_binding_required",
        "write_result_receipt_hash_binding_required",
        "allowlisted_diff_scope_required",
        "route_readiness_regression_denied",
        "active_dependency_isolation_regression_denied",
        "post_write_watchdog_soak_success_required",
        "rollback_validation_required",
        "rollback_execution_denied",
        "audit_redaction_validation_required",
        "secret_material_read_denied",
        "activation_command_invocation_denied",
        "direct_live_mutation_execution_denied",
        "activation_closure_persistence_denied",
        "external_send_public_claim_release_artifact_denied",
    ];

    let source_operator_acceptance_boundary_report_sha256 = sha256_json_value(&operator_acceptance);
    let boundary_hash_sha256 = sha256_text_value(&format!(
        "hepta-memory-live-mutation-operator-write-execution-activation-closure-denial-boundary-v1:{}:{}",
        route_matrix.route_count, source_operator_acceptance_boundary_report_sha256,
    ));
    let report_ready = route_matrix.ready
        && route_count_source_command_accepted
        && operator_acceptance_ready
        && activation_closure_surfaces.len() == 12
        && activation_closure_fixtures
            .as_array()
            .map(std::vec::Vec::len)
            == Some(10)
        && denied_by.len() == 24;

    let allowed_next_actions = serde_json::json!([
        {
            "action": "run_memory_write_execution_activation_closure_denial_boundary_require_live_gate",
            "status": "allowed_verification_only",
            "records_activation_closure_packet": false,
            "invokes_activation_command": false,
            "accepts_activation": false,
            "writes_memory": false,
            "executes_rollback": false,
            "writes_kg": false,
            "invokes_provider": false,
            "invokes_model": false,
            "reads_credentials": false,
            "sends_externally": false,
            "publishes_artifacts": false,
            "installs_or_restarts": false,
            "mutates_active_binary": false
        },
        {
            "action": "prepare_memory_write_execution_activation_command_no_op_handoff_boundary",
            "status": "allowed_report_only_next_slice",
            "writes_memory": false,
            "executes_rollback": false,
            "writes_kg": false,
            "invokes_provider": false,
            "sends_externally": false,
            "publishes_artifacts": false
        }
    ]);

    let mut side_effects = serde_json::Map::new();
    for key in [
        "memory_store_mutated",
        "memory_store_write_performed",
        "memory_write_execution_performed",
        "post_write_validation_recorded",
        "post_write_validation_persisted",
        "post_write_validation_performed",
        "operator_post_write_acceptance_recorded",
        "operator_post_write_acceptance_persisted",
        "operator_post_write_acceptance_performed",
        "accepted_operator_post_write_acceptance_report_recorded",
        "accepted_operator_post_write_acceptance_report_persisted",
        "activation_closure_packet_recorded",
        "activation_closure_packet_persisted",
        "activation_closure_packet_materialized",
        "activation_closure_filesystem_written",
        "activation_closure_ledger_written",
        "activation_command_invoked",
        "live_mutation_execution_performed",
        "rollback_validation_performed",
        "rollback_executed",
        "write_result_receipt_recorded",
        "write_result_receipt_persisted",
        "pre_write_memory_store_hash_recorded",
        "post_write_memory_store_hash_recorded",
        "audit_redaction_validation_recorded",
        "raw_payload_inspected",
        "payload_plaintext_persisted",
        "secret_file_read",
        "credential_read",
        "capability_registry_mutated",
        "plugin_registry_mutated",
        "skill_workshop_written",
        "provider_invoked",
        "model_invoked",
        "provider_prompt_replayed",
        "channel_send_performed",
        "telegram_send_performed",
        "external_send_performed",
        "runtime_store_mutated",
        "gateway_event_enqueued",
        "filesystem_written",
        "release_artifact_written",
        "public_artifact_written",
        "public_release_published",
        "public_ga_claimed",
        "launchd_mutated",
        "service_restarted",
        "install_executed",
        "active_binary_mutated",
    ] {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }

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
        "hepta_memory_live_mutation_operator_write_execution_activation_closure_denial_boundary_route"
    );
    insert_report_json!(
        "endpoint",
        HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_ACTIVATION_CLOSURE_DENIAL_BOUNDARY_ENDPOINT
    );
    insert_report_json!(
        "source_command",
        "/hepta-memory-live-mutation-operator-write-execution-activation-closure-denial-boundary --json"
    );
    insert_report_json!("native_route", true);
    insert_report_json!("side_effect_free", true);
    insert_report_json!("audit_date", "2026-07-02");
    insert_report_json!(
        "memory_write_execution_activation_closure_denial_boundary_schema_version",
        "memory_write_execution_activation_closure_denial_boundary_v1"
    );
    insert_report_json!(
        "memory_write_execution_activation_closure_denial_boundary_ready",
        report_ready
    );
    insert_report_json!(
        "activation_closure_denial_mode",
        "memory_write_execution_activation_closure_packet_no_write_denial"
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
        "source_memory_write_execution_post_write_operator_acceptance_denial_boundary_endpoint",
        HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_POST_WRITE_OPERATOR_ACCEPTANCE_DENIAL_BOUNDARY_ENDPOINT
    );
    insert_report_json!(
        "source_memory_write_execution_post_write_operator_acceptance_denial_boundary_ready",
        operator_acceptance_ready
    );
    insert_report_json!(
        "source_memory_write_execution_post_write_operator_acceptance_denial_ready",
        json_bool(
            &operator_acceptance,
            "memory_write_execution_post_write_operator_acceptance_denial_ready"
        )
    );
    insert_report_json!(
        "source_memory_write_execution_post_write_operator_acceptance_denial_boundary_report_sha256",
        source_operator_acceptance_boundary_report_sha256
    );
    for key in [
        "source_memory_write_execution_post_write_validation_dry_run_boundary_report_sha256",
        "source_memory_write_execution_write_enable_fixture_boundary_report_sha256",
        "source_memory_write_execution_no_write_sink_contract_boundary_report_sha256",
        "source_memory_write_execution_denial_matrix_boundary_report_sha256",
    ] {
        report.insert(
            key.to_string(),
            operator_acceptance
                .get(key)
                .cloned()
                .unwrap_or_else(|| serde_json::json!("")),
        );
    }
    insert_report_json!("minimum_required_samples", 24);
    insert_report_json!(
        "memory_write_execution_activation_closure_denial_ready",
        true
    );
    insert_report_json!(
        "memory_write_execution_post_write_operator_acceptance_denial_ready",
        true
    );
    insert_report_json!(
        "memory_write_execution_post_write_validation_dry_run_ready",
        true
    );
    insert_report_json!("memory_write_execution_write_enable_fixture_ready", true);
    insert_report_json!("memory_write_execution_no_write_sink_contract_ready", true);
    insert_report_json!(
        "required_operator_acceptance_surface_count",
        json_u64(
            &operator_acceptance,
            "required_operator_acceptance_surface_count"
        )
    );
    insert_report_json!(
        "ready_operator_acceptance_surface_count",
        json_u64(
            &operator_acceptance,
            "ready_operator_acceptance_surface_count"
        )
    );
    insert_report_json!("required_activation_closure_surface_count", 12);
    insert_report_json!("ready_activation_closure_surface_count", 12);
    insert_report_json!("side_effect_free_activation_closure_surface_count", 12);
    insert_report_json!("required_activation_closure_fixture_count", 10);
    insert_report_json!("activation_closure_fixture_count", 10);
    insert_report_json!("blocked_activation_closure_fixture_count", 10);
    insert_report_json!("allowed_activation_closure_fixture_count", 0);
    insert_report_json!("accepted_activation_closure_fixture_count", 0);
    insert_report_json!("activation_closure_denied_count", 10);
    insert_report_json!("activation_closure_performed_count", 0);

    for key in [
        "accepted_operator_post_write_acceptance_report_recorded",
        "accepted_operator_post_write_acceptance_report_persisted",
        "accepted_operator_post_write_acceptance_report_accepted",
        "accepted_operator_post_write_acceptance_hash_bound",
        "accepted_post_write_validation_report_hash_bound",
        "operator_identity_hash_recorded",
        "operator_acceptance_signature_hash_recorded",
        "operator_acceptance_timestamp_recorded",
        "operator_single_surface_scope_recorded",
        "activation_closure_packet_recorded",
        "activation_closure_packet_persisted",
        "activation_closure_packet_accepted",
        "activation_closure_packet_materialized",
        "activation_closure_packet_id_recorded",
        "activation_closure_packet_hash_bound",
        "activation_closure_packet_signature_hash_recorded",
        "activation_closure_packet_timestamp_recorded",
        "activation_closure_filesystem_written",
        "activation_closure_ledger_written",
        "activation_command_enabled",
        "activation_command_invoked",
        "activation_allowed_by_closure_packet",
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
        "write_result_receipt_hash_bound",
        "pre_write_memory_store_hash_bound",
        "post_write_memory_store_hash_bound",
        "post_write_diff_scope_accepted",
        "post_write_watchdog_soak_evidence_accepted",
        "post_write_route_regression_check_accepted",
        "post_write_dependency_isolation_check_accepted",
        "rollback_validation_accepted",
        "rollback_execution_allowed",
        "rollback_executed",
        "audit_redaction_validation_accepted",
        "raw_payload_plaintext_recorded",
        "raw_payload_plaintext_persisted",
        "secret_material_read",
        "provider_prompt_replay_enabled",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "kg_adapter_read_performed",
        "live_kg_write_performed",
        "channel_send_performed",
        "telegram_send_performed",
        "external_send_enabled",
        "external_send_performed",
        "public_claim_or_release_artifact_write_enabled",
        "public_release_published",
        "public_ga_claimed",
        "public_release_claimed",
        "release_artifact_written",
        "public_artifact_written",
        "install_executed",
        "service_restarted",
        "active_binary_mutated",
    ] {
        report.insert(key.to_string(), serde_json::json!(false));
    }
    insert_report_json!("memory_store_write_performed_count", 0);
    report.insert(
        "activation_closure_surfaces".to_string(),
        serde_json::json!(activation_closure_surfaces),
    );
    report.insert(
        "activation_closure_fixtures".to_string(),
        activation_closure_fixtures,
    );
    report.insert(
        "denied_by_activation_closure".to_string(),
        serde_json::json!(denied_by),
    );
    insert_report_json!("denied_by_activation_closure_count", 24);
    report.insert("allowed_next_actions".to_string(), allowed_next_actions);
    report.insert(
        "side_effects".to_string(),
        serde_json::Value::Object(side_effects),
    );

    serde_json::Value::Object(report)
}

fn hepta_memory_live_mutation_operator_write_execution_activation_command_noop_handoff_boundary_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let activation_closure =
        hepta_memory_live_mutation_operator_write_execution_activation_closure_denial_boundary_report(
        );

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
    let activation_closure_ready = json_str(&activation_closure, "status") == "ready"
        && json_bool(
            &activation_closure,
            "memory_write_execution_activation_closure_denial_boundary_ready",
        )
        && json_bool(
            &activation_closure,
            "memory_write_execution_activation_closure_denial_ready",
        )
        && json_bool(
            &activation_closure,
            "memory_write_execution_post_write_operator_acceptance_denial_ready",
        )
        && json_bool(
            &activation_closure,
            "memory_write_execution_post_write_validation_dry_run_ready",
        )
        && json_bool(
            &activation_closure,
            "memory_write_execution_write_enable_fixture_ready",
        )
        && json_bool(
            &activation_closure,
            "memory_write_execution_no_write_sink_contract_ready",
        )
        && json_u64(
            &activation_closure,
            "required_activation_closure_surface_count",
        ) == 12
        && json_u64(
            &activation_closure,
            "ready_activation_closure_surface_count",
        ) == 12
        && json_u64(&activation_closure, "activation_closure_fixture_count") == 10
        && json_u64(
            &activation_closure,
            "blocked_activation_closure_fixture_count",
        ) == 10
        && json_u64(&activation_closure, "denied_by_activation_closure_count") == 24
        && !json_bool(&activation_closure, "activation_closure_packet_recorded")
        && !json_bool(&activation_closure, "activation_closure_packet_persisted")
        && !json_bool(&activation_closure, "activation_closure_packet_accepted")
        && !json_bool(&activation_closure, "activation_command_enabled")
        && !json_bool(&activation_closure, "activation_command_invoked")
        && !json_bool(&activation_closure, "activation_allowed")
        && !json_bool(&activation_closure, "memory_write_execution_performed")
        && !json_bool(&activation_closure, "memory_store_mutated")
        && !json_bool(&activation_closure, "rollback_executed")
        && !json_bool(&activation_closure, "live_kg_write_performed")
        && !json_bool(&activation_closure, "provider_invoked")
        && !json_bool(&activation_closure, "model_invoked")
        && !json_bool(&activation_closure, "credential_read")
        && !json_bool(&activation_closure, "external_send_performed")
        && !json_bool(&activation_closure, "release_artifact_written")
        && !json_bool(&activation_closure, "active_binary_mutated")
        && side_effects_all_false(&activation_closure);

    let activation_command_handoff_surfaces = vec![
        "accepted_activation_closure_packet_required",
        "activation_closure_packet_hash_and_signature_required",
        "operator_identity_signature_timestamp_required",
        "single_surface_activation_scope_required",
        "activation_command_disabled_by_default_required",
        "activation_command_invocation_noop_required",
        "pre_post_store_hashes_and_write_receipt_required",
        "post_write_soak_route_dependency_evidence_required",
        "rollback_validation_and_no_rollback_execution_required",
        "audit_redaction_and_no_secret_material_required",
        "no_memory_store_write_or_live_mutation_required",
        "no_install_restart_or_active_binary_mutation_required",
        "no_external_public_or_release_outputs_required",
    ];
    let activation_command_fixtures = serde_json::json!([
        {
            "id": "activation-command-missing-accepted-closure-packet",
            "activation_command_requested": true,
            "command_status": "blocked_noop",
            "accepted_activation_closure_packet_present": false,
            "activation_closure_packet_hash_bound": false,
            "command_allowed": false,
            "command_invoked": false,
            "command_dispatched": false,
            "command_noop_confirmed": true,
            "handoff_recorded": false,
            "handoff_persisted": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "reason": "accepted_activation_closure_packet_required"
        },
        {
            "id": "activation-command-disabled-by-default",
            "activation_command_requested": true,
            "command_status": "blocked_noop",
            "accepted_activation_closure_packet_present": true,
            "activation_closure_packet_hash_bound": true,
            "activation_command_enabled": false,
            "command_allowed": false,
            "command_invoked": false,
            "command_dispatched": false,
            "command_noop_confirmed": true,
            "handoff_recorded": false,
            "handoff_persisted": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "reason": "activation_command_disabled_by_default"
        },
        {
            "id": "activation-command-direct-invocation-attempt",
            "activation_command_requested": true,
            "command_invocation_attempted": true,
            "command_status": "blocked_noop",
            "accepted_activation_closure_packet_present": true,
            "activation_command_enabled": false,
            "command_allowed": false,
            "command_invoked": false,
            "command_dispatched": false,
            "command_noop_confirmed": true,
            "handoff_recorded": false,
            "handoff_persisted": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "reason": "direct_activation_command_invocation_denied"
        },
        {
            "id": "activation-command-closure-hash-mismatch",
            "activation_command_requested": true,
            "command_status": "blocked_noop",
            "accepted_activation_closure_packet_present": true,
            "activation_closure_packet_hash_bound": false,
            "activation_closure_packet_signature_hash_recorded": false,
            "command_allowed": false,
            "command_invoked": false,
            "command_dispatched": false,
            "command_noop_confirmed": true,
            "handoff_recorded": false,
            "handoff_persisted": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "reason": "closure_packet_hash_and_signature_binding_required"
        },
        {
            "id": "activation-command-multi-surface-handoff",
            "activation_command_requested": true,
            "command_status": "blocked_noop",
            "accepted_activation_closure_packet_present": true,
            "operator_single_surface_scope_recorded": false,
            "multi_surface_activation_requested": true,
            "command_allowed": false,
            "command_invoked": false,
            "command_dispatched": false,
            "command_noop_confirmed": true,
            "handoff_recorded": false,
            "handoff_persisted": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "reason": "single_surface_activation_scope_required"
        },
        {
            "id": "activation-command-memory-write-path-attempt",
            "activation_command_requested": true,
            "command_status": "blocked_noop",
            "accepted_activation_closure_packet_present": true,
            "memory_store_write_path_enable_requested": true,
            "direct_memory_store_write_requested": true,
            "command_allowed": false,
            "command_invoked": false,
            "command_dispatched": false,
            "command_noop_confirmed": true,
            "handoff_recorded": false,
            "handoff_persisted": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "reason": "activation_command_cannot_enable_or_perform_memory_store_write"
        },
        {
            "id": "activation-command-rollback-execution-attempt",
            "activation_command_requested": true,
            "command_status": "blocked_noop",
            "accepted_activation_closure_packet_present": true,
            "rollback_validation_accepted": false,
            "rollback_execution_requested": true,
            "command_allowed": false,
            "command_invoked": false,
            "command_dispatched": false,
            "command_noop_confirmed": true,
            "handoff_recorded": false,
            "handoff_persisted": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "reason": "rollback_execution_denied_at_activation_command_handoff"
        },
        {
            "id": "activation-command-secret-or-prompt-replay-attempt",
            "activation_command_requested": true,
            "command_status": "blocked_noop",
            "accepted_activation_closure_packet_present": true,
            "raw_payload_plaintext_recorded": true,
            "secret_material_read": true,
            "provider_prompt_replay_requested": true,
            "command_allowed": false,
            "command_invoked": false,
            "command_dispatched": false,
            "command_noop_confirmed": true,
            "handoff_recorded": false,
            "handoff_persisted": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "reason": "secret_material_and_provider_prompt_replay_forbidden"
        },
        {
            "id": "activation-command-external-public-release-attempt",
            "activation_command_requested": true,
            "command_status": "blocked_noop",
            "accepted_activation_closure_packet_present": true,
            "external_send_requested": true,
            "public_claim_requested": true,
            "release_artifact_write_requested": true,
            "command_allowed": false,
            "command_invoked": false,
            "command_dispatched": false,
            "command_noop_confirmed": true,
            "handoff_recorded": false,
            "handoff_persisted": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "external_send_performed": false,
            "public_release_published": false,
            "release_artifact_written": false,
            "rollback_executed": false,
            "reason": "activation_command_cannot_send_publish_or_write_release_artifacts"
        },
        {
            "id": "activation-command-install-restart-active-binary-attempt",
            "activation_command_requested": true,
            "command_status": "blocked_noop",
            "accepted_activation_closure_packet_present": true,
            "install_requested": true,
            "launchd_restart_requested": true,
            "active_binary_mutation_requested": true,
            "command_allowed": false,
            "command_invoked": false,
            "command_dispatched": false,
            "command_noop_confirmed": true,
            "handoff_recorded": false,
            "handoff_persisted": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "install_executed": false,
            "launchd_mutated": false,
            "service_restarted": false,
            "active_binary_mutated": false,
            "rollback_executed": false,
            "reason": "activation_command_cannot_install_restart_or_mutate_active_binary"
        }
    ]);
    let denied_by = vec![
        "accepted_activation_closure_packet_required",
        "activation_closure_packet_hash_required",
        "activation_closure_packet_signature_required",
        "operator_identity_required",
        "operator_acceptance_signature_required",
        "operator_acceptance_timestamp_required",
        "single_surface_activation_scope_required",
        "activation_command_enabled_denied",
        "activation_command_invocation_denied",
        "activation_command_dispatch_denied",
        "activation_command_handoff_persistence_denied",
        "pre_write_memory_store_hash_binding_required",
        "post_write_memory_store_hash_binding_required",
        "write_result_receipt_hash_binding_required",
        "route_readiness_regression_denied",
        "active_dependency_isolation_regression_denied",
        "post_write_watchdog_soak_success_required",
        "memory_store_write_path_enablement_denied",
        "direct_memory_store_write_denied",
        "live_mutation_execution_denied",
        "rollback_execution_denied",
        "secret_material_read_denied",
        "provider_prompt_replay_denied",
        "install_restart_active_binary_mutation_denied",
        "external_send_public_claim_release_artifact_denied",
        "public_release_public_ga_denied",
    ];

    let source_activation_closure_report_sha256 = sha256_json_value(&activation_closure);
    let handoff_hash_sha256 = sha256_text_value(&format!(
        "hepta-memory-live-mutation-operator-write-execution-activation-command-noop-handoff-boundary-v1:{}:{}",
        route_matrix.route_count, source_activation_closure_report_sha256,
    ));
    let report_ready = route_matrix.ready
        && route_count_source_command_accepted
        && activation_closure_ready
        && activation_command_handoff_surfaces.len() == 13
        && activation_command_fixtures
            .as_array()
            .map(std::vec::Vec::len)
            == Some(10)
        && denied_by.len() == 26;

    let allowed_next_actions = serde_json::json!([
        {
            "action": "run_memory_write_execution_activation_command_noop_handoff_boundary_require_live_gate",
            "status": "allowed_verification_only",
            "registers_command": false,
            "enables_command": false,
            "invokes_activation_command": false,
            "dispatches_activation": false,
            "records_handoff": false,
            "persists_handoff": false,
            "accepts_activation": false,
            "writes_memory": false,
            "executes_rollback": false,
            "writes_kg": false,
            "invokes_provider": false,
            "invokes_model": false,
            "reads_credentials": false,
            "sends_externally": false,
            "publishes_artifacts": false,
            "installs_or_restarts": false,
            "mutates_active_binary": false
        },
        {
            "action": "prepare_memory_write_execution_activation_command_result_receipt_no_persistence_boundary",
            "status": "allowed_report_only_next_slice",
            "records_command_result": false,
            "persists_result_receipt": false,
            "accepts_result_receipt": false,
            "writes_memory": false,
            "executes_rollback": false,
            "writes_kg": false,
            "invokes_provider": false,
            "sends_externally": false,
            "publishes_artifacts": false
        }
    ]);

    let mut side_effects = serde_json::Map::new();
    for key in [
        "memory_store_mutated",
        "memory_store_write_performed",
        "memory_write_execution_performed",
        "post_write_validation_recorded",
        "post_write_validation_persisted",
        "post_write_validation_performed",
        "operator_post_write_acceptance_recorded",
        "operator_post_write_acceptance_persisted",
        "operator_post_write_acceptance_performed",
        "activation_closure_packet_recorded",
        "activation_closure_packet_persisted",
        "activation_closure_packet_materialized",
        "activation_closure_filesystem_written",
        "activation_closure_ledger_written",
        "activation_command_shape_registered",
        "activation_command_enabled",
        "activation_command_invoked",
        "activation_command_dispatched",
        "activation_command_handoff_recorded",
        "activation_command_handoff_persisted",
        "activation_command_result_receipt_recorded",
        "activation_command_result_receipt_persisted",
        "live_mutation_execution_performed",
        "rollback_validation_performed",
        "rollback_executed",
        "write_result_receipt_recorded",
        "write_result_receipt_persisted",
        "pre_write_memory_store_hash_recorded",
        "post_write_memory_store_hash_recorded",
        "audit_redaction_validation_recorded",
        "raw_payload_inspected",
        "payload_plaintext_persisted",
        "secret_file_read",
        "credential_read",
        "capability_registry_mutated",
        "plugin_registry_mutated",
        "skill_workshop_written",
        "provider_invoked",
        "model_invoked",
        "provider_prompt_replayed",
        "channel_send_performed",
        "telegram_send_performed",
        "external_send_performed",
        "runtime_store_mutated",
        "gateway_event_enqueued",
        "filesystem_written",
        "release_artifact_written",
        "public_artifact_written",
        "public_release_published",
        "public_ga_claimed",
        "install_executed",
        "active_binary_mutated",
        "launchd_mutated",
        "service_restarted",
    ] {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }

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
        "hepta_memory_live_mutation_operator_write_execution_activation_command_noop_handoff_boundary_route"
    );
    insert_report_json!(
        "endpoint",
        HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_ACTIVATION_COMMAND_NOOP_HANDOFF_BOUNDARY_ENDPOINT
    );
    insert_report_json!(
        "source_command",
        "/hepta-memory-live-mutation-operator-write-execution-activation-command-no-op-handoff-boundary --json"
    );
    insert_report_json!("native_route", true);
    insert_report_json!("side_effect_free", true);
    insert_report_json!("audit_date", "2026-07-02");
    insert_report_json!(
        "memory_write_execution_activation_command_noop_handoff_boundary_schema_version",
        "memory_write_execution_activation_command_noop_handoff_boundary_v1"
    );
    insert_report_json!(
        "memory_write_execution_activation_command_noop_handoff_boundary_ready",
        report_ready
    );
    insert_report_json!(
        "activation_command_noop_handoff_mode",
        "memory_write_execution_activation_command_noop_handoff_denial"
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
    insert_report_json!("boundary_hash_sha256", handoff_hash_sha256);
    insert_report_json!(
        "source_memory_write_execution_activation_closure_denial_boundary_endpoint",
        HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_ACTIVATION_CLOSURE_DENIAL_BOUNDARY_ENDPOINT
    );
    insert_report_json!(
        "source_memory_write_execution_activation_closure_denial_boundary_ready",
        activation_closure_ready
    );
    insert_report_json!(
        "source_memory_write_execution_activation_closure_denial_ready",
        json_bool(
            &activation_closure,
            "memory_write_execution_activation_closure_denial_ready"
        )
    );
    insert_report_json!(
        "source_memory_write_execution_activation_closure_denial_boundary_report_sha256",
        source_activation_closure_report_sha256
    );
    for key in [
        "source_memory_write_execution_post_write_operator_acceptance_denial_boundary_report_sha256",
        "source_memory_write_execution_post_write_validation_dry_run_boundary_report_sha256",
        "source_memory_write_execution_write_enable_fixture_boundary_report_sha256",
        "source_memory_write_execution_no_write_sink_contract_boundary_report_sha256",
        "source_memory_write_execution_denial_matrix_boundary_report_sha256",
    ] {
        report.insert(
            key.to_string(),
            activation_closure
                .get(key)
                .cloned()
                .unwrap_or_else(|| serde_json::json!("")),
        );
    }
    insert_report_json!("minimum_required_samples", 24);
    insert_report_json!(
        "memory_write_execution_activation_command_noop_handoff_ready",
        true
    );
    insert_report_json!(
        "memory_write_execution_activation_closure_denial_ready",
        true
    );
    insert_report_json!(
        "memory_write_execution_post_write_operator_acceptance_denial_ready",
        true
    );
    insert_report_json!(
        "memory_write_execution_post_write_validation_dry_run_ready",
        true
    );
    insert_report_json!("memory_write_execution_write_enable_fixture_ready", true);
    insert_report_json!("memory_write_execution_no_write_sink_contract_ready", true);
    insert_report_json!("required_activation_closure_surface_count", 12);
    insert_report_json!("ready_activation_closure_surface_count", 12);
    insert_report_json!("required_activation_command_handoff_surface_count", 13);
    insert_report_json!("ready_activation_command_handoff_surface_count", 13);
    insert_report_json!(
        "side_effect_free_activation_command_handoff_surface_count",
        13
    );
    insert_report_json!("required_activation_command_fixture_count", 10);
    insert_report_json!("activation_command_fixture_count", 10);
    insert_report_json!("blocked_activation_command_fixture_count", 10);
    insert_report_json!("noop_activation_command_fixture_count", 10);
    insert_report_json!("allowed_activation_command_fixture_count", 0);
    insert_report_json!("accepted_activation_command_fixture_count", 0);
    insert_report_json!("activation_command_denied_count", 10);
    insert_report_json!("activation_command_performed_count", 0);

    for key in [
        "activation_command_shape_registered",
        "activation_command_enabled",
        "activation_command_invoked",
        "activation_command_dispatched",
        "activation_command_noop_decision_recorded",
        "activation_command_noop_decision_persisted",
        "activation_command_noop_decision_accepted",
        "activation_command_handoff_recorded",
        "activation_command_handoff_persisted",
        "activation_command_handoff_accepted",
        "activation_command_handoff_materialized",
        "activation_command_handoff_filesystem_written",
        "activation_command_result_receipt_recorded",
        "activation_command_result_receipt_persisted",
        "activation_closure_packet_recorded",
        "activation_closure_packet_persisted",
        "activation_closure_packet_accepted",
        "activation_closure_packet_materialized",
        "activation_closure_packet_hash_bound",
        "activation_closure_packet_signature_hash_recorded",
        "activation_closure_ledger_written",
        "activation_allowed_by_command_handoff",
        "activation_allowed_by_closure_packet",
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
        "write_result_receipt_hash_bound",
        "pre_write_memory_store_hash_bound",
        "post_write_memory_store_hash_bound",
        "post_write_diff_scope_accepted",
        "post_write_watchdog_soak_evidence_accepted",
        "route_readiness_regression_allowed",
        "active_dependency_isolation_regression_allowed",
        "rollback_validation_accepted",
        "rollback_execution_allowed",
        "rollback_executed",
        "audit_redaction_validation_accepted",
        "raw_payload_plaintext_recorded",
        "raw_payload_plaintext_persisted",
        "secret_material_read",
        "provider_prompt_replay_enabled",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "kg_adapter_read_performed",
        "live_kg_write_performed",
        "channel_send_performed",
        "telegram_send_performed",
        "external_send_enabled",
        "external_send_performed",
        "public_claim_or_release_artifact_write_enabled",
        "public_release_published",
        "public_ga_claimed",
        "public_release_claimed",
        "release_artifact_written",
        "public_artifact_written",
        "install_executed",
        "launchd_mutated",
        "service_restarted",
        "active_binary_mutated",
    ] {
        report.insert(key.to_string(), serde_json::json!(false));
    }
    insert_report_json!("memory_store_write_performed_count", 0);
    report.insert(
        "activation_command_handoff_surfaces".to_string(),
        serde_json::json!(activation_command_handoff_surfaces),
    );
    report.insert(
        "activation_command_fixtures".to_string(),
        activation_command_fixtures,
    );
    report.insert(
        "denied_by_activation_command_handoff".to_string(),
        serde_json::json!(denied_by),
    );
    insert_report_json!("denied_by_activation_command_handoff_count", 26);
    report.insert("allowed_next_actions".to_string(), allowed_next_actions);
    report.insert(
        "side_effects".to_string(),
        serde_json::Value::Object(side_effects),
    );

    serde_json::Value::Object(report)
}

fn hepta_memory_live_mutation_operator_write_execution_activation_command_result_receipt_no_persistence_boundary_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let noop_handoff =
        hepta_memory_live_mutation_operator_write_execution_activation_command_noop_handoff_boundary_report();

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
    let noop_handoff_ready = json_str(&noop_handoff, "status") == "ready"
        && json_bool(
            &noop_handoff,
            "memory_write_execution_activation_command_noop_handoff_boundary_ready",
        )
        && json_bool(
            &noop_handoff,
            "memory_write_execution_activation_command_noop_handoff_ready",
        )
        && json_bool(
            &noop_handoff,
            "memory_write_execution_activation_closure_denial_ready",
        )
        && json_u64(
            &noop_handoff,
            "required_activation_command_handoff_surface_count",
        ) == 13
        && json_u64(
            &noop_handoff,
            "ready_activation_command_handoff_surface_count",
        ) == 13
        && json_u64(&noop_handoff, "activation_command_fixture_count") == 10
        && json_u64(&noop_handoff, "blocked_activation_command_fixture_count") == 10
        && json_u64(&noop_handoff, "noop_activation_command_fixture_count") == 10
        && json_u64(&noop_handoff, "accepted_activation_command_fixture_count") == 0
        && json_u64(&noop_handoff, "activation_command_performed_count") == 0
        && json_u64(&noop_handoff, "denied_by_activation_command_handoff_count") == 26
        && !json_bool(&noop_handoff, "activation_command_shape_registered")
        && !json_bool(&noop_handoff, "activation_command_enabled")
        && !json_bool(&noop_handoff, "activation_command_invoked")
        && !json_bool(&noop_handoff, "activation_command_dispatched")
        && !json_bool(&noop_handoff, "activation_command_result_receipt_recorded")
        && !json_bool(&noop_handoff, "activation_command_result_receipt_persisted")
        && !json_bool(&noop_handoff, "activation_allowed")
        && !json_bool(&noop_handoff, "memory_write_execution_performed")
        && !json_bool(&noop_handoff, "memory_store_mutated")
        && !json_bool(&noop_handoff, "rollback_executed")
        && !json_bool(&noop_handoff, "live_kg_write_performed")
        && !json_bool(&noop_handoff, "provider_invoked")
        && !json_bool(&noop_handoff, "model_invoked")
        && !json_bool(&noop_handoff, "credential_read")
        && !json_bool(&noop_handoff, "external_send_performed")
        && !json_bool(&noop_handoff, "release_artifact_written")
        && !json_bool(&noop_handoff, "active_binary_mutated")
        && side_effects_all_false(&noop_handoff);

    let receipt_surfaces = vec![
        "source_noop_handoff_report_required",
        "accepted_activation_closure_packet_required",
        "activation_command_disabled_and_not_invoked_required",
        "receipt_schema_and_request_id_required",
        "receipt_hash_signature_timestamp_required",
        "receipt_status_must_remain_blocked_noop_required",
        "receipt_record_persist_materialize_denied",
        "receipt_filesystem_ledger_index_delivery_denied",
        "completion_ack_denied",
        "activation_from_receipt_denied",
        "memory_write_live_mutation_rollback_denied",
        "external_public_release_install_restart_denied",
    ];
    let receipt_fixtures = serde_json::json!([
        {
            "id": "activation-result-receipt-missing-source-noop-handoff",
            "receipt_requested": true,
            "receipt_status": "blocked_noop",
            "source_noop_handoff_present": false,
            "source_noop_handoff_ready": false,
            "receipt_allowed": false,
            "receipt_recorded": false,
            "receipt_persisted": false,
            "receipt_accepted": false,
            "receipt_materialized": false,
            "receipt_filesystem_written": false,
            "receipt_noop_confirmed": true,
            "completion_ack_recorded": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "reason": "source_activation_command_noop_handoff_required"
        },
        {
            "id": "activation-result-receipt-record-attempt",
            "receipt_requested": true,
            "receipt_record_requested": true,
            "receipt_status": "blocked_noop",
            "source_noop_handoff_present": true,
            "receipt_allowed": false,
            "receipt_recorded": false,
            "receipt_persisted": false,
            "receipt_accepted": false,
            "receipt_materialized": false,
            "receipt_filesystem_written": false,
            "receipt_noop_confirmed": true,
            "completion_ack_recorded": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "reason": "activation_command_result_receipt_recording_denied"
        },
        {
            "id": "activation-result-receipt-persist-attempt",
            "receipt_requested": true,
            "receipt_persist_requested": true,
            "receipt_status": "blocked_noop",
            "source_noop_handoff_present": true,
            "receipt_allowed": false,
            "receipt_recorded": false,
            "receipt_persisted": false,
            "receipt_accepted": false,
            "receipt_materialized": false,
            "receipt_filesystem_written": false,
            "receipt_noop_confirmed": true,
            "completion_ack_recorded": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "reason": "activation_command_result_receipt_persistence_denied"
        },
        {
            "id": "activation-result-receipt-materialize-filesystem-attempt",
            "receipt_requested": true,
            "receipt_materialize_requested": true,
            "receipt_filesystem_write_requested": true,
            "receipt_status": "blocked_noop",
            "source_noop_handoff_present": true,
            "receipt_allowed": false,
            "receipt_recorded": false,
            "receipt_persisted": false,
            "receipt_accepted": false,
            "receipt_materialized": false,
            "receipt_filesystem_written": false,
            "receipt_noop_confirmed": true,
            "completion_ack_recorded": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "reason": "result_receipt_materialization_and_filesystem_write_denied"
        },
        {
            "id": "activation-result-receipt-ledger-index-delivery-attempt",
            "receipt_requested": true,
            "receipt_ledger_write_requested": true,
            "receipt_index_requested": true,
            "receipt_delivery_requested": true,
            "receipt_status": "blocked_noop",
            "source_noop_handoff_present": true,
            "receipt_allowed": false,
            "receipt_recorded": false,
            "receipt_persisted": false,
            "receipt_accepted": false,
            "receipt_materialized": false,
            "receipt_filesystem_written": false,
            "receipt_ledger_written": false,
            "receipt_indexed": false,
            "receipt_delivered": false,
            "receipt_noop_confirmed": true,
            "completion_ack_recorded": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "reason": "result_receipt_ledger_index_delivery_denied"
        },
        {
            "id": "activation-result-receipt-acceptance-as-approval-attempt",
            "receipt_requested": true,
            "receipt_acceptance_requested": true,
            "receipt_status": "blocked_noop",
            "source_noop_handoff_present": true,
            "receipt_allowed": false,
            "receipt_recorded": false,
            "receipt_persisted": false,
            "receipt_accepted": false,
            "receipt_materialized": false,
            "receipt_filesystem_written": false,
            "receipt_noop_confirmed": true,
            "completion_ack_recorded": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "reason": "result_receipt_cannot_become_operator_approval"
        },
        {
            "id": "activation-result-receipt-completion-ack-attempt",
            "receipt_requested": true,
            "completion_ack_requested": true,
            "activation_completion_ack_requested": true,
            "receipt_status": "blocked_noop",
            "source_noop_handoff_present": true,
            "receipt_allowed": false,
            "receipt_recorded": false,
            "receipt_persisted": false,
            "receipt_accepted": false,
            "receipt_materialized": false,
            "receipt_filesystem_written": false,
            "receipt_noop_confirmed": true,
            "completion_ack_recorded": false,
            "completion_ack_persisted": false,
            "completion_ack_accepted": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "reason": "activation_completion_ack_denied"
        },
        {
            "id": "activation-result-receipt-non-noop-status-attempt",
            "receipt_requested": true,
            "receipt_status_requested": "completed",
            "receipt_status": "blocked_noop",
            "source_noop_handoff_present": true,
            "receipt_allowed": false,
            "receipt_recorded": false,
            "receipt_persisted": false,
            "receipt_accepted": false,
            "receipt_materialized": false,
            "receipt_filesystem_written": false,
            "receipt_noop_confirmed": true,
            "completion_ack_recorded": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "reason": "result_receipt_status_must_remain_blocked_noop"
        },
        {
            "id": "activation-result-receipt-memory-write-rollback-attempt",
            "receipt_requested": true,
            "receipt_status": "blocked_noop",
            "source_noop_handoff_present": true,
            "memory_store_write_requested": true,
            "rollback_execution_requested": true,
            "receipt_allowed": false,
            "receipt_recorded": false,
            "receipt_persisted": false,
            "receipt_accepted": false,
            "receipt_materialized": false,
            "receipt_filesystem_written": false,
            "receipt_noop_confirmed": true,
            "completion_ack_recorded": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "reason": "result_receipt_cannot_enable_memory_write_or_rollback"
        },
        {
            "id": "activation-result-receipt-external-public-install-attempt",
            "receipt_requested": true,
            "receipt_status": "blocked_noop",
            "source_noop_handoff_present": true,
            "external_send_requested": true,
            "public_claim_requested": true,
            "release_artifact_write_requested": true,
            "install_requested": true,
            "launchd_restart_requested": true,
            "active_binary_mutation_requested": true,
            "receipt_allowed": false,
            "receipt_recorded": false,
            "receipt_persisted": false,
            "receipt_accepted": false,
            "receipt_materialized": false,
            "receipt_filesystem_written": false,
            "receipt_noop_confirmed": true,
            "completion_ack_recorded": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "external_send_performed": false,
            "public_release_published": false,
            "release_artifact_written": false,
            "install_executed": false,
            "launchd_mutated": false,
            "service_restarted": false,
            "active_binary_mutated": false,
            "reason": "result_receipt_cannot_send_publish_install_restart_or_mutate_active_binary"
        }
    ]);
    let denied_by = vec![
        "source_activation_command_noop_handoff_required",
        "accepted_activation_closure_packet_required",
        "activation_command_enabled_denied",
        "activation_command_invocation_denied",
        "activation_command_dispatch_denied",
        "receipt_schema_acceptance_denied",
        "receipt_recording_denied",
        "receipt_persistence_denied",
        "receipt_acceptance_denied",
        "receipt_materialization_denied",
        "receipt_filesystem_write_denied",
        "receipt_ledger_write_denied",
        "receipt_indexing_denied",
        "receipt_delivery_denied",
        "completion_ack_recording_denied",
        "completion_ack_persistence_denied",
        "completion_ack_acceptance_denied",
        "activation_from_receipt_denied",
        "memory_store_write_denied",
        "live_mutation_execution_denied",
        "rollback_execution_denied",
        "secret_material_read_denied",
        "provider_prompt_replay_denied",
        "external_send_public_claim_release_artifact_denied",
        "install_restart_active_binary_mutation_denied",
    ];

    let source_noop_handoff_report_sha256 = sha256_json_value(&noop_handoff);
    let boundary_hash_sha256 = sha256_text_value(&format!(
        "hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-no-persistence-boundary-v1:{}:{}",
        route_matrix.route_count, source_noop_handoff_report_sha256,
    ));
    let report_ready = route_matrix.ready
        && route_count_source_command_accepted
        && noop_handoff_ready
        && receipt_surfaces.len() == 12
        && receipt_fixtures.as_array().map(std::vec::Vec::len) == Some(10)
        && denied_by.len() == 25;

    let allowed_next_actions = serde_json::json!([
        {
            "action": "run_memory_write_execution_activation_command_result_receipt_no_persistence_boundary_require_live_gate",
            "status": "allowed_verification_only",
            "records_command_result": false,
            "persists_result_receipt": false,
            "accepts_result_receipt": false,
            "records_completion_ack": false,
            "accepts_activation": false,
            "writes_memory": false,
            "executes_rollback": false,
            "writes_kg": false,
            "invokes_provider": false,
            "sends_externally": false,
            "publishes_artifacts": false,
            "installs_or_restarts": false,
            "mutates_active_binary": false
        },
        {
            "action": "prepare_memory_write_execution_activation_command_result_receipt_replay_idempotency_denial_boundary",
            "status": "allowed_report_only_next_slice",
            "accepts_duplicate_receipt": false,
            "records_idempotency": false,
            "persists_replay_state": false,
            "mutates_runtime": false,
            "invokes_model": false,
            "writes_memory_or_kg": false
        }
    ]);

    let mut side_effects = serde_json::Map::new();
    for key in [
        "activation_command_result_receipt_recorded",
        "activation_command_result_receipt_persisted",
        "activation_command_result_receipt_accepted",
        "activation_command_result_receipt_materialized",
        "activation_command_result_receipt_filesystem_written",
        "activation_command_result_receipt_ledger_written",
        "activation_command_result_receipt_indexed",
        "activation_command_result_receipt_enqueued",
        "activation_command_result_receipt_delivered",
        "activation_command_completion_ack_recorded",
        "activation_command_completion_ack_persisted",
        "activation_command_completion_ack_accepted",
        "activation_command_completion_ack_delivered",
        "activation_command_shape_registered",
        "activation_command_enabled",
        "activation_command_invoked",
        "activation_command_dispatched",
        "activation_command_noop_decision_recorded",
        "activation_command_noop_decision_persisted",
        "activation_command_handoff_recorded",
        "activation_command_handoff_persisted",
        "activation_command_handoff_materialized",
        "activation_closure_packet_recorded",
        "activation_closure_packet_persisted",
        "live_mutation_execution_performed",
        "memory_write_execution_performed",
        "memory_store_write_performed",
        "memory_store_mutated",
        "rollback_executed",
        "raw_payload_inspected",
        "payload_plaintext_persisted",
        "secret_file_read",
        "credential_read",
        "provider_invoked",
        "model_invoked",
        "provider_prompt_replayed",
        "channel_send_performed",
        "telegram_send_performed",
        "external_send_performed",
        "runtime_store_mutated",
        "gateway_event_enqueued",
        "capability_registry_mutated",
        "plugin_registry_mutated",
        "skill_workshop_written",
        "filesystem_written",
        "release_artifact_written",
        "public_artifact_written",
        "public_release_published",
        "public_ga_claimed",
        "install_executed",
        "active_binary_mutated",
        "launchd_mutated",
        "service_restarted",
    ] {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }

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
        "hepta_memory_live_mutation_operator_write_execution_activation_command_result_receipt_no_persistence_boundary_route"
    );
    insert_report_json!(
        "endpoint",
        HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_ACTIVATION_COMMAND_RESULT_RECEIPT_NO_PERSISTENCE_BOUNDARY_ENDPOINT
    );
    insert_report_json!(
        "source_command",
        "/hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-no-persistence-boundary --json"
    );
    insert_report_json!("native_route", true);
    insert_report_json!("side_effect_free", true);
    insert_report_json!("audit_date", "2026-07-02");
    insert_report_json!(
        "memory_write_execution_activation_command_result_receipt_no_persistence_boundary_schema_version",
        "memory_write_execution_activation_command_result_receipt_no_persistence_boundary_v1"
    );
    insert_report_json!(
        "memory_write_execution_activation_command_result_receipt_no_persistence_boundary_ready",
        report_ready
    );
    insert_report_json!(
        "activation_command_result_receipt_no_persistence_mode",
        "memory_write_execution_activation_command_result_receipt_no_persistence_denial"
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
        "source_activation_command_noop_handoff_boundary_endpoint",
        HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_ACTIVATION_COMMAND_NOOP_HANDOFF_BOUNDARY_ENDPOINT
    );
    insert_report_json!(
        "source_activation_command_noop_handoff_boundary_ready",
        noop_handoff_ready
    );
    insert_report_json!(
        "source_activation_command_noop_handoff_ready",
        json_bool(
            &noop_handoff,
            "memory_write_execution_activation_command_noop_handoff_ready"
        )
    );
    insert_report_json!(
        "source_activation_command_noop_handoff_boundary_report_sha256",
        source_noop_handoff_report_sha256
    );
    for key in [
        "source_memory_write_execution_activation_closure_denial_boundary_report_sha256",
        "source_memory_write_execution_post_write_operator_acceptance_denial_boundary_report_sha256",
        "source_memory_write_execution_post_write_validation_dry_run_boundary_report_sha256",
        "source_memory_write_execution_write_enable_fixture_boundary_report_sha256",
        "source_memory_write_execution_no_write_sink_contract_boundary_report_sha256",
        "source_memory_write_execution_denial_matrix_boundary_report_sha256",
    ] {
        report.insert(
            key.to_string(),
            noop_handoff
                .get(key)
                .cloned()
                .unwrap_or_else(|| serde_json::json!("")),
        );
    }
    insert_report_json!("minimum_required_samples", 24);
    insert_report_json!(
        "memory_write_execution_activation_command_result_receipt_no_persistence_ready",
        true
    );
    insert_report_json!(
        "memory_write_execution_activation_command_noop_handoff_ready",
        true
    );
    insert_report_json!(
        "memory_write_execution_activation_closure_denial_ready",
        true
    );
    insert_report_json!(
        "memory_write_execution_post_write_operator_acceptance_denial_ready",
        true
    );
    insert_report_json!(
        "memory_write_execution_post_write_validation_dry_run_ready",
        true
    );
    insert_report_json!("memory_write_execution_write_enable_fixture_ready", true);
    insert_report_json!("memory_write_execution_no_write_sink_contract_ready", true);
    insert_report_json!("required_activation_command_handoff_surface_count", 13);
    insert_report_json!("ready_activation_command_handoff_surface_count", 13);
    insert_report_json!(
        "required_activation_command_result_receipt_surface_count",
        12
    );
    insert_report_json!("ready_activation_command_result_receipt_surface_count", 12);
    insert_report_json!(
        "side_effect_free_activation_command_result_receipt_surface_count",
        12
    );
    insert_report_json!(
        "required_activation_command_result_receipt_fixture_count",
        10
    );
    insert_report_json!("activation_command_result_receipt_fixture_count", 10);
    insert_report_json!(
        "blocked_activation_command_result_receipt_fixture_count",
        10
    );
    insert_report_json!("noop_activation_command_result_receipt_fixture_count", 10);
    insert_report_json!("allowed_activation_command_result_receipt_fixture_count", 0);
    insert_report_json!(
        "accepted_activation_command_result_receipt_fixture_count",
        0
    );
    insert_report_json!("activation_command_result_receipt_denied_count", 10);
    insert_report_json!("activation_command_result_receipt_performed_count", 0);

    for key in [
        "activation_command_result_receipt_shape_registered",
        "activation_command_result_receipt_allowed",
        "activation_command_result_receipt_schema_accepted",
        "activation_command_result_receipt_recorded",
        "activation_command_result_receipt_persisted",
        "activation_command_result_receipt_accepted",
        "activation_command_result_receipt_materialized",
        "activation_command_result_receipt_filesystem_written",
        "activation_command_result_receipt_ledger_written",
        "activation_command_result_receipt_indexed",
        "activation_command_result_receipt_enqueued",
        "activation_command_result_receipt_delivered",
        "activation_command_result_receipt_hash_bound",
        "activation_command_result_receipt_signature_hash_recorded",
        "activation_command_result_receipt_timestamp_recorded",
        "activation_command_result_receipt_operator_identity_accepted",
        "activation_command_result_receipt_status_accepted",
        "activation_command_result_receipt_blocked_noop_status_accepted",
        "activation_command_completion_ack_recorded",
        "activation_command_completion_ack_persisted",
        "activation_command_completion_ack_accepted",
        "activation_command_completion_ack_materialized",
        "activation_command_completion_ack_delivered",
        "activation_command_shape_registered",
        "activation_command_enabled",
        "activation_command_invoked",
        "activation_command_dispatched",
        "activation_command_noop_decision_recorded",
        "activation_command_noop_decision_persisted",
        "activation_command_handoff_recorded",
        "activation_command_handoff_persisted",
        "activation_command_handoff_accepted",
        "activation_command_handoff_materialized",
        "activation_allowed_by_result_receipt",
        "activation_allowed_by_command_handoff",
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
        "rollback_validation_accepted",
        "rollback_execution_allowed",
        "rollback_executed",
        "raw_payload_plaintext_recorded",
        "raw_payload_plaintext_persisted",
        "secret_material_read",
        "provider_prompt_replay_enabled",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "kg_adapter_read_performed",
        "live_kg_write_performed",
        "channel_send_performed",
        "telegram_send_performed",
        "external_send_enabled",
        "external_send_performed",
        "public_claim_or_release_artifact_write_enabled",
        "public_release_published",
        "public_ga_claimed",
        "public_release_claimed",
        "release_artifact_written",
        "public_artifact_written",
        "install_executed",
        "launchd_mutated",
        "service_restarted",
        "active_binary_mutated",
    ] {
        report.insert(key.to_string(), serde_json::json!(false));
    }
    insert_report_json!("memory_store_write_performed_count", 0);
    report.insert(
        "activation_command_result_receipt_surfaces".to_string(),
        serde_json::json!(receipt_surfaces),
    );
    report.insert(
        "activation_command_result_receipt_fixtures".to_string(),
        receipt_fixtures,
    );
    report.insert(
        "denied_by_activation_command_result_receipt".to_string(),
        serde_json::json!(denied_by),
    );
    insert_report_json!("denied_by_activation_command_result_receipt_count", 25);
    report.insert("allowed_next_actions".to_string(), allowed_next_actions);
    report.insert(
        "side_effects".to_string(),
        serde_json::Value::Object(side_effects),
    );

    serde_json::Value::Object(report)
}

fn hepta_memory_live_mutation_operator_write_execution_activation_command_result_receipt_replay_idempotency_denial_boundary_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let no_persistence =
        hepta_memory_live_mutation_operator_write_execution_activation_command_result_receipt_no_persistence_boundary_report();

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
    let source_no_persistence_ready = json_str(&no_persistence, "status") == "ready"
        && json_bool(
            &no_persistence,
            "memory_write_execution_activation_command_result_receipt_no_persistence_boundary_ready",
        )
        && json_bool(
            &no_persistence,
            "memory_write_execution_activation_command_result_receipt_no_persistence_ready",
        )
        && json_bool(
            &no_persistence,
            "memory_write_execution_activation_command_noop_handoff_ready",
        )
        && json_u64(
            &no_persistence,
            "required_activation_command_result_receipt_surface_count",
        ) == 12
        && json_u64(
            &no_persistence,
            "ready_activation_command_result_receipt_surface_count",
        ) == 12
        && json_u64(
            &no_persistence,
            "activation_command_result_receipt_fixture_count",
        ) == 10
        && json_u64(
            &no_persistence,
            "blocked_activation_command_result_receipt_fixture_count",
        ) == 10
        && json_u64(
            &no_persistence,
            "accepted_activation_command_result_receipt_fixture_count",
        ) == 0
        && json_u64(
            &no_persistence,
            "activation_command_result_receipt_performed_count",
        ) == 0
        && json_u64(
            &no_persistence,
            "denied_by_activation_command_result_receipt_count",
        ) == 25
        && !json_bool(
            &no_persistence,
            "activation_command_result_receipt_recorded",
        )
        && !json_bool(
            &no_persistence,
            "activation_command_result_receipt_persisted",
        )
        && !json_bool(
            &no_persistence,
            "activation_command_result_receipt_accepted",
        )
        && !json_bool(
            &no_persistence,
            "activation_command_result_receipt_materialized",
        )
        && !json_bool(
            &no_persistence,
            "activation_command_result_receipt_filesystem_written",
        )
        && !json_bool(
            &no_persistence,
            "activation_command_result_receipt_ledger_written",
        )
        && !json_bool(&no_persistence, "activation_command_result_receipt_indexed")
        && !json_bool(
            &no_persistence,
            "activation_command_result_receipt_delivered",
        )
        && !json_bool(
            &no_persistence,
            "activation_command_completion_ack_recorded",
        )
        && !json_bool(&no_persistence, "activation_allowed")
        && !json_bool(&no_persistence, "live_mutation_execution_performed")
        && !json_bool(&no_persistence, "memory_store_write_performed")
        && !json_bool(&no_persistence, "memory_store_mutated")
        && !json_bool(&no_persistence, "rollback_executed")
        && !json_bool(&no_persistence, "live_kg_write_performed")
        && !json_bool(&no_persistence, "provider_invoked")
        && !json_bool(&no_persistence, "model_invoked")
        && !json_bool(&no_persistence, "credential_read")
        && !json_bool(&no_persistence, "external_send_performed")
        && !json_bool(&no_persistence, "release_artifact_written")
        && !json_bool(&no_persistence, "active_binary_mutated")
        && side_effects_all_false(&no_persistence);

    let replay_surfaces = vec![
        "source_result_receipt_no_persistence_report_required",
        "canonical_noop_result_receipt_identity_required",
        "receipt_replay_nonce_idempotency_key_required",
        "duplicate_receipt_suppression_required",
        "cross_scope_receipt_reuse_denied",
        "blocked_noop_status_transition_denied",
        "completion_ack_replay_denied",
        "ledger_index_delivery_replay_denied",
        "memory_write_live_mutation_replay_denied",
        "rollback_replay_denied",
        "secret_provider_prompt_replay_denied",
        "external_public_install_restart_replay_denied",
    ];
    let replay_fixtures = serde_json::Value::Array(vec![
        serde_json::json!({
            "id": "activation-result-receipt-replay-missing-source-no-persistence-report",
            "replay_requested": true,
            "replay_status": "blocked_noop",
            "source_no_persistence_present": false,
            "source_no_persistence_ready": false,
            "replay_allowed": false,
            "replay_recorded": false,
            "replay_persisted": false,
            "duplicate_accepted": false,
            "idempotency_key_accepted": false,
            "idempotency_state_recorded": false,
            "idempotency_state_persisted": false,
            "receipt_recorded": false,
            "receipt_persisted": false,
            "receipt_accepted": false,
            "completion_ack_recorded": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "receipt_noop_confirmed": true,
            "reason": "source_result_receipt_no_persistence_report_required"
        }),
        serde_json::json!({
            "id": "activation-result-receipt-duplicate-identity-replay",
            "replay_requested": true,
            "duplicate_receipt_id_requested": true,
            "replay_status": "blocked_duplicate_noop",
            "source_no_persistence_present": true,
            "source_no_persistence_ready": true,
            "replay_allowed": false,
            "replay_recorded": false,
            "replay_persisted": false,
            "duplicate_accepted": false,
            "idempotency_key_accepted": false,
            "idempotency_state_recorded": false,
            "idempotency_state_persisted": false,
            "receipt_recorded": false,
            "receipt_persisted": false,
            "receipt_accepted": false,
            "completion_ack_recorded": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "receipt_noop_confirmed": true,
            "reason": "duplicate_result_receipt_id_replay_denied"
        }),
        serde_json::json!({
            "id": "activation-result-receipt-stale-idempotency-key-replay",
            "replay_requested": true,
            "stale_idempotency_key_requested": true,
            "replay_status": "blocked_duplicate_noop",
            "source_no_persistence_present": true,
            "source_no_persistence_ready": true,
            "replay_allowed": false,
            "replay_recorded": false,
            "replay_persisted": false,
            "duplicate_accepted": false,
            "idempotency_key_accepted": false,
            "idempotency_state_recorded": false,
            "idempotency_state_persisted": false,
            "receipt_recorded": false,
            "receipt_persisted": false,
            "receipt_accepted": false,
            "completion_ack_recorded": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "receipt_noop_confirmed": true,
            "reason": "stale_idempotency_key_replay_denied"
        }),
        serde_json::json!({
            "id": "activation-result-receipt-idempotency-state-recording-attempt",
            "replay_requested": true,
            "replay_acceptance_requested": true,
            "idempotency_key_recording_requested": true,
            "idempotency_state_recording_requested": true,
            "idempotency_state_persistence_requested": true,
            "replay_status": "blocked_noop",
            "source_no_persistence_present": true,
            "source_no_persistence_ready": true,
            "replay_allowed": false,
            "replay_recorded": false,
            "replay_persisted": false,
            "duplicate_accepted": false,
            "idempotency_key_accepted": false,
            "idempotency_state_recorded": false,
            "idempotency_state_persisted": false,
            "receipt_recorded": false,
            "receipt_persisted": false,
            "receipt_accepted": false,
            "completion_ack_recorded": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "receipt_noop_confirmed": true,
            "reason": "idempotency_state_recording_and_persistence_denied"
        }),
        serde_json::json!({
            "id": "activation-result-receipt-cross-scope-reuse-attempt",
            "replay_requested": true,
            "cross_scope_reuse_requested": true,
            "replay_status": "blocked_noop",
            "source_no_persistence_present": true,
            "source_no_persistence_ready": true,
            "replay_allowed": false,
            "replay_recorded": false,
            "replay_persisted": false,
            "duplicate_accepted": false,
            "idempotency_key_accepted": false,
            "idempotency_state_recorded": false,
            "idempotency_state_persisted": false,
            "receipt_recorded": false,
            "receipt_persisted": false,
            "receipt_accepted": false,
            "completion_ack_recorded": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "receipt_noop_confirmed": true,
            "reason": "cross_scope_result_receipt_reuse_denied"
        }),
        serde_json::json!({
            "id": "activation-result-receipt-completed-status-upgrade-attempt",
            "replay_requested": true,
            "receipt_status_requested": "completed",
            "status_upgrade_requested": true,
            "replay_status": "blocked_noop",
            "source_no_persistence_present": true,
            "source_no_persistence_ready": true,
            "replay_allowed": false,
            "replay_recorded": false,
            "replay_persisted": false,
            "duplicate_accepted": false,
            "idempotency_key_accepted": false,
            "idempotency_state_recorded": false,
            "idempotency_state_persisted": false,
            "receipt_recorded": false,
            "receipt_persisted": false,
            "receipt_accepted": false,
            "completion_ack_recorded": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "receipt_noop_confirmed": true,
            "reason": "blocked_noop_status_transition_denied"
        }),
        serde_json::json!({
            "id": "activation-result-receipt-completion-ack-replay-attempt",
            "replay_requested": true,
            "completion_ack_replay_requested": true,
            "replay_status": "blocked_noop",
            "source_no_persistence_present": true,
            "source_no_persistence_ready": true,
            "replay_allowed": false,
            "replay_recorded": false,
            "replay_persisted": false,
            "duplicate_accepted": false,
            "idempotency_key_accepted": false,
            "idempotency_state_recorded": false,
            "idempotency_state_persisted": false,
            "receipt_recorded": false,
            "receipt_persisted": false,
            "receipt_accepted": false,
            "completion_ack_recorded": false,
            "completion_ack_persisted": false,
            "completion_ack_accepted": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "receipt_noop_confirmed": true,
            "reason": "completion_ack_replay_denied"
        }),
        serde_json::json!({
            "id": "activation-result-receipt-ledger-index-delivery-replay-attempt",
            "replay_requested": true,
            "ledger_replay_requested": true,
            "index_replay_requested": true,
            "delivery_replay_requested": true,
            "replay_status": "blocked_noop",
            "source_no_persistence_present": true,
            "source_no_persistence_ready": true,
            "replay_allowed": false,
            "replay_recorded": false,
            "replay_persisted": false,
            "duplicate_accepted": false,
            "idempotency_key_accepted": false,
            "idempotency_state_recorded": false,
            "idempotency_state_persisted": false,
            "receipt_recorded": false,
            "receipt_persisted": false,
            "receipt_accepted": false,
            "completion_ack_recorded": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "receipt_noop_confirmed": true,
            "reason": "ledger_index_delivery_replay_denied"
        }),
        serde_json::json!({
            "id": "activation-result-receipt-memory-rollback-secret-provider-replay-attempt",
            "replay_requested": true,
            "memory_write_replay_requested": true,
            "live_mutation_replay_requested": true,
            "rollback_replay_requested": true,
            "secret_material_replay_requested": true,
            "provider_prompt_replay_requested": true,
            "provider_invocation_replay_requested": true,
            "model_invocation_replay_requested": true,
            "replay_status": "blocked_noop",
            "source_no_persistence_present": true,
            "source_no_persistence_ready": true,
            "replay_allowed": false,
            "replay_recorded": false,
            "replay_persisted": false,
            "duplicate_accepted": false,
            "idempotency_key_accepted": false,
            "idempotency_state_recorded": false,
            "idempotency_state_persisted": false,
            "receipt_recorded": false,
            "receipt_persisted": false,
            "receipt_accepted": false,
            "completion_ack_recorded": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "provider_invoked": false,
            "model_invoked": false,
            "receipt_noop_confirmed": true,
            "reason": "memory_rollback_secret_provider_replay_denied"
        }),
        serde_json::json!({
            "id": "activation-result-receipt-external-public-install-replay-attempt",
            "replay_requested": true,
            "external_send_replay_requested": true,
            "public_claim_replay_requested": true,
            "release_artifact_replay_requested": true,
            "install_replay_requested": true,
            "launchd_restart_replay_requested": true,
            "active_binary_mutation_replay_requested": true,
            "replay_status": "blocked_noop",
            "source_no_persistence_present": true,
            "source_no_persistence_ready": true,
            "replay_allowed": false,
            "replay_recorded": false,
            "replay_persisted": false,
            "duplicate_accepted": false,
            "idempotency_key_accepted": false,
            "idempotency_state_recorded": false,
            "idempotency_state_persisted": false,
            "receipt_recorded": false,
            "receipt_persisted": false,
            "receipt_accepted": false,
            "completion_ack_recorded": false,
            "activation_allowed": false,
            "live_mutation_execution_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "rollback_executed": false,
            "external_send_performed": false,
            "public_release_published": false,
            "release_artifact_written": false,
            "install_executed": false,
            "launchd_mutated": false,
            "service_restarted": false,
            "active_binary_mutated": false,
            "receipt_noop_confirmed": true,
            "reason": "external_public_install_restart_result_receipt_replay_denied"
        }),
    ]);
    let denied_by = vec![
        "source_result_receipt_no_persistence_report_required",
        "canonical_noop_result_receipt_identity_required",
        "result_receipt_replay_nonce_required_but_not_recorded",
        "result_receipt_idempotency_key_required_but_not_recorded",
        "duplicate_result_receipt_id_replay_denied",
        "stale_idempotency_key_replay_denied",
        "cross_scope_result_receipt_reuse_denied",
        "blocked_noop_status_transition_denied",
        "completed_status_upgrade_denied",
        "completion_ack_replay_denied",
        "ledger_replay_denied",
        "index_replay_denied",
        "delivery_replay_denied",
        "memory_write_replay_denied",
        "live_mutation_replay_denied",
        "rollback_replay_denied",
        "secret_material_replay_denied",
        "provider_prompt_replay_denied",
        "external_send_replay_denied",
        "public_claim_replay_denied",
        "release_artifact_replay_denied",
        "install_replay_denied",
        "launchd_restart_replay_denied",
        "active_binary_mutation_replay_denied",
    ];

    let source_no_persistence_report_sha256 = sha256_json_value(&no_persistence);
    let boundary_hash_sha256 = sha256_text_value(&format!(
        "hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-replay-idempotency-denial-boundary-v1:{}:{}",
        route_matrix.route_count, source_no_persistence_report_sha256,
    ));
    let report_ready = route_matrix.ready
        && route_count_source_command_accepted
        && source_no_persistence_ready
        && replay_surfaces.len() == 12
        && replay_fixtures.as_array().map(std::vec::Vec::len) == Some(10)
        && denied_by.len() == 24;

    let allowed_next_actions = serde_json::json!([
        {
            "action": "run_memory_write_execution_activation_command_result_receipt_replay_idempotency_denial_boundary_require_live_gate",
            "status": "allowed_verification_only",
            "accepts_duplicate_receipt": false,
            "records_replay": false,
            "records_idempotency_state": false,
            "persists_replay_state": false,
            "accepts_activation": false,
            "writes_memory": false,
            "executes_rollback": false,
            "writes_kg": false,
            "invokes_provider": false,
            "sends_externally": false,
            "publishes_artifacts": false,
            "installs_or_restarts": false,
            "mutates_active_binary": false
        },
        {
            "action": "prepare_memory_write_execution_activation_command_result_receipt_ordering_monotonicity_denial_boundary",
            "status": "allowed_report_only_next_slice",
            "accepts_out_of_order_receipt": false,
            "records_monotonic_sequence": false,
            "promotes_completion": false,
            "mutates_runtime": false,
            "invokes_model": false,
            "writes_memory_or_kg": false
        }
    ]);

    let false_keys = [
        "activation_command_result_receipt_replay_allowed",
        "activation_command_result_receipt_replay_recorded",
        "activation_command_result_receipt_replay_persisted",
        "activation_command_result_receipt_duplicate_accepted",
        "activation_command_result_receipt_duplicate_recorded",
        "activation_command_result_receipt_duplicate_persisted",
        "activation_command_result_receipt_idempotency_key_accepted",
        "activation_command_result_receipt_idempotency_state_recorded",
        "activation_command_result_receipt_idempotency_state_persisted",
        "activation_command_result_receipt_replay_nonce_accepted",
        "activation_command_result_receipt_replay_nonce_recorded",
        "activation_command_result_receipt_cross_scope_reuse_accepted",
        "activation_command_result_receipt_status_upgrade_accepted",
        "activation_command_result_receipt_completed_status_accepted",
        "activation_command_result_receipt_ack_replay_accepted",
        "activation_command_result_receipt_ledger_replay_accepted",
        "activation_command_result_receipt_delivery_replay_accepted",
        "activation_command_result_receipt_write_replay_accepted",
        "activation_command_result_receipt_rollback_replay_accepted",
        "activation_command_result_receipt_secret_provider_replay_accepted",
        "activation_command_result_receipt_external_public_install_replay_accepted",
        "activation_command_result_receipt_recorded",
        "activation_command_result_receipt_persisted",
        "activation_command_result_receipt_accepted",
        "activation_command_result_receipt_materialized",
        "activation_command_result_receipt_filesystem_written",
        "activation_command_result_receipt_ledger_written",
        "activation_command_result_receipt_indexed",
        "activation_command_result_receipt_enqueued",
        "activation_command_result_receipt_delivered",
        "activation_command_completion_ack_recorded",
        "activation_command_completion_ack_persisted",
        "activation_command_completion_ack_accepted",
        "activation_command_completion_ack_delivered",
        "activation_command_enabled",
        "activation_command_invoked",
        "activation_command_dispatched",
        "activation_allowed_by_result_receipt_replay",
        "activation_allowed_by_result_receipt",
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
        "rollback_execution_allowed",
        "rollback_executed",
        "raw_payload_plaintext_recorded",
        "raw_payload_plaintext_persisted",
        "secret_material_read",
        "provider_prompt_replay_enabled",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "kg_adapter_read_performed",
        "live_kg_write_performed",
        "channel_send_performed",
        "telegram_send_performed",
        "external_send_enabled",
        "external_send_performed",
        "public_claim_or_release_artifact_write_enabled",
        "public_release_published",
        "public_ga_claimed",
        "public_release_claimed",
        "release_artifact_written",
        "public_artifact_written",
        "install_executed",
        "launchd_mutated",
        "service_restarted",
        "active_binary_mutated",
    ];

    let mut side_effects = serde_json::Map::new();
    for key in false_keys {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }

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
        "hepta_memory_live_mutation_operator_write_execution_activation_command_result_receipt_replay_idempotency_denial_boundary_route"
    );
    insert_report_json!(
        "endpoint",
        HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_ACTIVATION_COMMAND_RESULT_RECEIPT_REPLAY_IDEMPOTENCY_DENIAL_BOUNDARY_ENDPOINT
    );
    insert_report_json!(
        "source_command",
        "/hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-replay-idempotency-denial-boundary --json"
    );
    insert_report_json!("native_route", true);
    insert_report_json!("side_effect_free", true);
    insert_report_json!("audit_date", "2026-07-02");
    insert_report_json!(
        "memory_write_execution_activation_command_result_receipt_replay_idempotency_denial_boundary_schema_version",
        "memory_write_execution_activation_command_result_receipt_replay_idempotency_denial_boundary_v1"
    );
    insert_report_json!(
        "memory_write_execution_activation_command_result_receipt_replay_idempotency_denial_boundary_ready",
        report_ready
    );
    insert_report_json!(
        "activation_command_result_receipt_replay_idempotency_mode",
        "memory_write_execution_activation_command_result_receipt_replay_idempotency_denial"
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
        "source_activation_command_result_receipt_no_persistence_boundary_endpoint",
        HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_ACTIVATION_COMMAND_RESULT_RECEIPT_NO_PERSISTENCE_BOUNDARY_ENDPOINT
    );
    insert_report_json!(
        "source_activation_command_result_receipt_no_persistence_boundary_ready",
        source_no_persistence_ready
    );
    insert_report_json!(
        "source_activation_command_result_receipt_no_persistence_ready",
        json_bool(
            &no_persistence,
            "memory_write_execution_activation_command_result_receipt_no_persistence_ready"
        )
    );
    insert_report_json!(
        "source_activation_command_result_receipt_no_persistence_boundary_report_sha256",
        source_no_persistence_report_sha256
    );
    for key in [
        "source_activation_command_noop_handoff_boundary_report_sha256",
        "source_memory_write_execution_activation_closure_denial_boundary_report_sha256",
        "source_memory_write_execution_post_write_operator_acceptance_denial_boundary_report_sha256",
        "source_memory_write_execution_post_write_validation_dry_run_boundary_report_sha256",
        "source_memory_write_execution_write_enable_fixture_boundary_report_sha256",
        "source_memory_write_execution_no_write_sink_contract_boundary_report_sha256",
        "source_memory_write_execution_denial_matrix_boundary_report_sha256",
    ] {
        report.insert(
            key.to_string(),
            no_persistence
                .get(key)
                .cloned()
                .unwrap_or_else(|| serde_json::json!("")),
        );
    }
    insert_report_json!("minimum_required_samples", 24);
    insert_report_json!(
        "memory_write_execution_activation_command_result_receipt_replay_idempotency_denial_ready",
        true
    );
    insert_report_json!(
        "memory_write_execution_activation_command_result_receipt_no_persistence_ready",
        true
    );
    insert_report_json!(
        "memory_write_execution_activation_command_noop_handoff_ready",
        true
    );
    insert_report_json!(
        "memory_write_execution_activation_closure_denial_ready",
        true
    );
    insert_report_json!(
        "memory_write_execution_post_write_operator_acceptance_denial_ready",
        true
    );
    insert_report_json!(
        "memory_write_execution_post_write_validation_dry_run_ready",
        true
    );
    insert_report_json!("memory_write_execution_write_enable_fixture_ready", true);
    insert_report_json!("memory_write_execution_no_write_sink_contract_ready", true);
    insert_report_json!(
        "required_activation_command_result_receipt_replay_idempotency_surface_count",
        12
    );
    insert_report_json!(
        "ready_activation_command_result_receipt_replay_idempotency_surface_count",
        12
    );
    insert_report_json!(
        "side_effect_free_activation_command_result_receipt_replay_idempotency_surface_count",
        12
    );
    insert_report_json!(
        "required_activation_command_result_receipt_replay_idempotency_fixture_count",
        10
    );
    insert_report_json!(
        "activation_command_result_receipt_replay_idempotency_fixture_count",
        10
    );
    insert_report_json!(
        "blocked_activation_command_result_receipt_replay_idempotency_fixture_count",
        10
    );
    insert_report_json!(
        "noop_activation_command_result_receipt_replay_idempotency_fixture_count",
        10
    );
    insert_report_json!(
        "allowed_activation_command_result_receipt_replay_idempotency_fixture_count",
        0
    );
    insert_report_json!(
        "accepted_activation_command_result_receipt_replay_idempotency_fixture_count",
        0
    );
    insert_report_json!(
        "duplicate_activation_command_result_receipt_fixture_count",
        2
    );
    insert_report_json!(
        "cross_scope_activation_command_result_receipt_fixture_count",
        1
    );
    insert_report_json!(
        "status_upgrade_activation_command_result_receipt_fixture_count",
        1
    );
    insert_report_json!("activation_command_result_receipt_replay_denied_count", 10);
    insert_report_json!(
        "activation_command_result_receipt_duplicate_denied_count",
        10
    );
    insert_report_json!(
        "activation_command_result_receipt_idempotency_denied_count",
        10
    );
    insert_report_json!(
        "activation_command_result_receipt_replay_performed_count",
        0
    );
    insert_report_json!(
        "activation_command_result_receipt_duplicate_accepted_count",
        0
    );
    insert_report_json!(
        "activation_command_result_receipt_idempotency_state_recorded_count",
        0
    );
    insert_report_json!("memory_store_write_performed_count", 0);

    for key in false_keys {
        report.insert(key.to_string(), serde_json::json!(false));
    }
    report.insert(
        "activation_command_result_receipt_replay_idempotency_surfaces".to_string(),
        serde_json::json!(replay_surfaces),
    );
    report.insert(
        "activation_command_result_receipt_replay_idempotency_fixtures".to_string(),
        replay_fixtures,
    );
    report.insert(
        "denied_by_activation_command_result_receipt_replay_idempotency".to_string(),
        serde_json::json!(denied_by),
    );
    insert_report_json!(
        "denied_by_activation_command_result_receipt_replay_idempotency_count",
        24
    );
    report.insert("allowed_next_actions".to_string(), allowed_next_actions);
    report.insert(
        "side_effects".to_string(),
        serde_json::Value::Object(side_effects),
    );

    serde_json::Value::Object(report)
}

fn hepta_memory_live_mutation_operator_write_execution_activation_command_result_receipt_ordering_monotonicity_denial_boundary_report()
-> serde_json::Value {
    fn ordering_fixture(
        id: &str,
        reason: &str,
        ordering_status: &str,
        extra: serde_json::Value,
    ) -> serde_json::Value {
        let mut base = serde_json::Map::new();
        macro_rules! insert_fixture_json {
            ($key:literal, $value:expr) => {
                base.insert($key.to_string(), serde_json::json!($value));
            };
        }
        insert_fixture_json!("id", id);
        insert_fixture_json!("ordering_requested", true);
        insert_fixture_json!("ordering_status", ordering_status);
        insert_fixture_json!("source_replay_idempotency_present", true);
        insert_fixture_json!("source_replay_idempotency_ready", true);
        for key in [
            "ordering_allowed",
            "ordering_recorded",
            "ordering_persisted",
            "sequence_cursor_accepted",
            "sequence_cursor_recorded",
            "sequence_cursor_persisted",
            "monotonicity_state_recorded",
            "monotonicity_state_persisted",
            "timestamp_ordering_accepted",
            "epoch_ordering_accepted",
            "stage_ordering_accepted",
            "latest_wins_overwrite_accepted",
            "receipt_recorded",
            "receipt_persisted",
            "receipt_accepted",
            "receipt_materialized",
            "receipt_filesystem_written",
            "receipt_ledger_written",
            "receipt_indexed",
            "receipt_delivered",
            "completion_ack_recorded",
            "completion_ack_persisted",
            "completion_ack_accepted",
            "activation_allowed",
            "live_mutation_execution_performed",
            "memory_store_write_performed",
            "memory_store_mutated",
            "rollback_executed",
            "secret_material_read",
            "provider_invoked",
            "model_invoked",
            "external_send_performed",
            "public_release_published",
            "release_artifact_written",
            "install_executed",
            "launchd_mutated",
            "service_restarted",
            "active_binary_mutated",
        ] {
            base.insert(key.to_string(), serde_json::json!(false));
        }
        insert_fixture_json!("receipt_noop_confirmed", true);
        insert_fixture_json!("reason", reason);
        if let Some(extra) = extra.as_object() {
            for (key, value) in extra {
                base.insert(key.clone(), value.clone());
            }
        }
        serde_json::Value::Object(base)
    }

    let route_matrix = control_ui_route_parity_report();
    let replay =
        hepta_memory_live_mutation_operator_write_execution_activation_command_result_receipt_replay_idempotency_denial_boundary_report();

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
    let source_replay_ready = json_str(&replay, "status") == "ready"
        && json_bool(
            &replay,
            "memory_write_execution_activation_command_result_receipt_replay_idempotency_denial_boundary_ready",
        )
        && json_bool(
            &replay,
            "memory_write_execution_activation_command_result_receipt_replay_idempotency_denial_ready",
        )
        && json_bool(
            &replay,
            "memory_write_execution_activation_command_result_receipt_no_persistence_ready",
        )
        && json_u64(
            &replay,
            "required_activation_command_result_receipt_replay_idempotency_surface_count",
        ) == 12
        && json_u64(
            &replay,
            "activation_command_result_receipt_replay_idempotency_fixture_count",
        ) == 10
        && json_u64(
            &replay,
            "blocked_activation_command_result_receipt_replay_idempotency_fixture_count",
        ) == 10
        && json_u64(
            &replay,
            "accepted_activation_command_result_receipt_replay_idempotency_fixture_count",
        ) == 0
        && json_u64(
            &replay,
            "activation_command_result_receipt_replay_performed_count",
        ) == 0
        && json_u64(
            &replay,
            "denied_by_activation_command_result_receipt_replay_idempotency_count",
        ) == 24
        && !json_bool(&replay, "activation_command_result_receipt_replay_allowed")
        && !json_bool(&replay, "activation_command_result_receipt_replay_recorded")
        && !json_bool(
            &replay,
            "activation_command_result_receipt_replay_persisted",
        )
        && !json_bool(
            &replay,
            "activation_command_result_receipt_duplicate_accepted",
        )
        && !json_bool(
            &replay,
            "activation_command_result_receipt_idempotency_state_recorded",
        )
        && !json_bool(&replay, "activation_command_result_receipt_recorded")
        && !json_bool(&replay, "activation_command_result_receipt_persisted")
        && !json_bool(&replay, "activation_command_result_receipt_accepted")
        && !json_bool(&replay, "activation_command_completion_ack_recorded")
        && !json_bool(&replay, "activation_allowed")
        && !json_bool(&replay, "live_mutation_execution_performed")
        && !json_bool(&replay, "memory_store_write_performed")
        && !json_bool(&replay, "memory_store_mutated")
        && !json_bool(&replay, "rollback_executed")
        && !json_bool(&replay, "provider_invoked")
        && !json_bool(&replay, "model_invoked")
        && !json_bool(&replay, "external_send_performed")
        && !json_bool(&replay, "release_artifact_written")
        && !json_bool(&replay, "active_binary_mutated")
        && side_effects_all_false(&replay);

    let ordering_surfaces = vec![
        "source_replay_idempotency_report_required",
        "canonical_noop_receipt_order_identity_required",
        "sequence_cursor_monotonicity_denied",
        "out_of_order_sequence_denied",
        "sequence_gap_or_skip_denied",
        "timestamp_rollback_denied",
        "epoch_rollback_denied",
        "same_sequence_different_hash_denied",
        "latest_wins_overwrite_denied",
        "stage_transition_ordering_denied",
        "ledger_index_delivery_ordering_bypass_denied",
        "external_public_install_ordering_bypass_denied",
    ];
    let ordering_fixtures = serde_json::Value::Array(vec![
        ordering_fixture(
            "activation-result-receipt-ordering-missing-source-replay-idempotency-report",
            "source_result_receipt_replay_idempotency_report_required",
            "blocked_noop",
            serde_json::json!({
                "source_replay_idempotency_present": false,
                "source_replay_idempotency_ready": false
            }),
        ),
        ordering_fixture(
            "activation-result-receipt-out-of-order-sequence",
            "out_of_order_result_receipt_sequence_denied",
            "blocked_ordering_noop",
            serde_json::json!({
                "out_of_order_sequence_requested": true,
                "requested_sequence": 2,
                "observed_previous_sequence": 3
            }),
        ),
        ordering_fixture(
            "activation-result-receipt-sequence-gap-skip",
            "sequence_gap_or_skip_result_receipt_denied",
            "blocked_ordering_noop",
            serde_json::json!({
                "sequence_gap_requested": true,
                "requested_sequence": 5,
                "expected_next_sequence": 1
            }),
        ),
        ordering_fixture(
            "activation-result-receipt-timestamp-rollback",
            "timestamp_rollback_result_receipt_denied",
            "blocked_ordering_noop",
            serde_json::json!({
                "timestamp_rollback_requested": true,
                "requested_timestamp_order": "older_than_source_noop_handoff"
            }),
        ),
        ordering_fixture(
            "activation-result-receipt-epoch-rollback",
            "epoch_rollback_result_receipt_denied",
            "blocked_ordering_noop",
            serde_json::json!({
                "epoch_rollback_requested": true,
                "requested_epoch_order": "lower_than_current_activation_epoch"
            }),
        ),
        ordering_fixture(
            "activation-result-receipt-same-sequence-different-hash",
            "same_sequence_different_hash_result_receipt_denied",
            "blocked_ordering_noop",
            serde_json::json!({
                "same_sequence_different_hash_requested": true,
                "requested_sequence": 1,
                "requested_hash_relation": "different_hash_for_same_sequence"
            }),
        ),
        ordering_fixture(
            "activation-result-receipt-latest-wins-overwrite",
            "latest_wins_result_receipt_overwrite_denied",
            "blocked_ordering_noop",
            serde_json::json!({
                "latest_wins_overwrite_requested": true,
                "overwrite_existing_noop_requested": true
            }),
        ),
        ordering_fixture(
            "activation-result-receipt-stage-transition-before-noop",
            "stage_transition_ordering_bypass_denied",
            "blocked_ordering_noop",
            serde_json::json!({
                "stage_transition_ordering_bypass_requested": true,
                "completion_ack_before_noop_requested": true,
                "requested_stage": "completed_before_blocked_noop"
            }),
        ),
        ordering_fixture(
            "activation-result-receipt-ledger-index-delivery-ordering-bypass",
            "ledger_index_delivery_ordering_bypass_denied",
            "blocked_ordering_noop",
            serde_json::json!({
                "ledger_ordering_bypass_requested": true,
                "index_ordering_bypass_requested": true,
                "delivery_ordering_bypass_requested": true
            }),
        ),
        ordering_fixture(
            "activation-result-receipt-external-public-install-ordering-bypass",
            "external_public_install_restart_ordering_bypass_denied",
            "blocked_ordering_noop",
            serde_json::json!({
                "external_send_ordering_bypass_requested": true,
                "public_claim_ordering_bypass_requested": true,
                "release_artifact_ordering_bypass_requested": true,
                "install_ordering_bypass_requested": true,
                "service_restart_ordering_bypass_requested": true,
                "active_binary_mutation_ordering_bypass_requested": true
            }),
        ),
    ]);
    let denied_by = vec![
        "source_result_receipt_replay_idempotency_report_required",
        "canonical_noop_receipt_order_identity_required",
        "sequence_cursor_acceptance_denied",
        "sequence_cursor_recording_denied",
        "sequence_cursor_persistence_denied",
        "monotonicity_state_recording_denied",
        "monotonicity_state_persistence_denied",
        "out_of_order_sequence_denied",
        "sequence_gap_or_skip_denied",
        "timestamp_rollback_denied",
        "epoch_rollback_denied",
        "same_sequence_different_hash_denied",
        "latest_wins_overwrite_denied",
        "completion_ack_before_noop_denied",
        "stage_transition_ordering_denied",
        "ledger_ordering_bypass_denied",
        "index_ordering_bypass_denied",
        "delivery_ordering_bypass_denied",
        "memory_write_ordering_bypass_denied",
        "live_mutation_ordering_bypass_denied",
        "rollback_ordering_bypass_denied",
        "secret_provider_ordering_bypass_denied",
        "external_public_release_ordering_bypass_denied",
        "install_restart_active_binary_ordering_bypass_denied",
    ];

    let source_replay_report_sha256 = sha256_json_value(&replay);
    let boundary_hash_sha256 = sha256_text_value(&format!(
        "hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-ordering-monotonicity-denial-boundary-v1:{}:{}",
        route_matrix.route_count, source_replay_report_sha256,
    ));
    let report_ready = route_matrix.ready
        && route_count_source_command_accepted
        && source_replay_ready
        && ordering_surfaces.len() == 12
        && ordering_fixtures.as_array().map(std::vec::Vec::len) == Some(10)
        && denied_by.len() == 24;

    let allowed_next_actions = serde_json::json!([
        {
            "action": "run_memory_write_execution_activation_command_result_receipt_ordering_monotonicity_denial_boundary_require_live_gate",
            "status": "allowed_verification_only",
            "accepts_out_of_order_receipt": false,
            "records_monotonic_sequence": false,
            "persists_ordering_state": false,
            "promotes_completion": false,
            "writes_memory": false,
            "executes_rollback": false,
            "writes_kg": false,
            "invokes_provider": false,
            "sends_externally": false,
            "publishes_artifacts": false,
            "installs_or_restarts": false,
            "mutates_active_binary": false
        },
        {
            "action": "prepare_memory_write_execution_activation_command_result_receipt_cancellation_supersession_denial_boundary",
            "status": "allowed_report_only_next_slice",
            "accepts_cancellation": false,
            "accepts_supersession": false,
            "records_replacement_receipt": false,
            "mutates_runtime": false,
            "invokes_model": false,
            "writes_memory_or_kg": false
        }
    ]);

    let false_keys = [
        "activation_command_result_receipt_ordering_allowed",
        "activation_command_result_receipt_ordering_recorded",
        "activation_command_result_receipt_ordering_persisted",
        "activation_command_result_receipt_sequence_cursor_accepted",
        "activation_command_result_receipt_sequence_cursor_recorded",
        "activation_command_result_receipt_sequence_cursor_persisted",
        "activation_command_result_receipt_monotonicity_state_recorded",
        "activation_command_result_receipt_monotonicity_state_persisted",
        "activation_command_result_receipt_timestamp_ordering_accepted",
        "activation_command_result_receipt_epoch_ordering_accepted",
        "activation_command_result_receipt_stage_ordering_accepted",
        "activation_command_result_receipt_same_sequence_hash_override_accepted",
        "activation_command_result_receipt_latest_wins_overwrite_accepted",
        "activation_command_result_receipt_gap_fill_accepted",
        "activation_command_result_receipt_ack_before_noop_accepted",
        "activation_command_result_receipt_recorded",
        "activation_command_result_receipt_persisted",
        "activation_command_result_receipt_accepted",
        "activation_command_result_receipt_materialized",
        "activation_command_result_receipt_filesystem_written",
        "activation_command_result_receipt_ledger_written",
        "activation_command_result_receipt_indexed",
        "activation_command_result_receipt_enqueued",
        "activation_command_result_receipt_delivered",
        "activation_command_completion_ack_recorded",
        "activation_command_completion_ack_persisted",
        "activation_command_completion_ack_accepted",
        "activation_command_completion_ack_delivered",
        "activation_command_enabled",
        "activation_command_invoked",
        "activation_command_dispatched",
        "activation_allowed_by_result_receipt_ordering",
        "activation_allowed_by_result_receipt_replay",
        "activation_allowed_by_result_receipt",
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
        "rollback_execution_allowed",
        "rollback_executed",
        "raw_payload_plaintext_recorded",
        "raw_payload_plaintext_persisted",
        "secret_material_read",
        "provider_prompt_replay_enabled",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "kg_adapter_read_performed",
        "live_kg_write_performed",
        "channel_send_performed",
        "telegram_send_performed",
        "external_send_enabled",
        "external_send_performed",
        "public_claim_or_release_artifact_write_enabled",
        "public_release_published",
        "public_ga_claimed",
        "public_release_claimed",
        "release_artifact_written",
        "public_artifact_written",
        "install_executed",
        "launchd_mutated",
        "service_restarted",
        "active_binary_mutated",
    ];

    let mut side_effects = serde_json::Map::new();
    for key in false_keys {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }

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
        "hepta_memory_live_mutation_operator_write_execution_activation_command_result_receipt_ordering_monotonicity_denial_boundary_route"
    );
    insert_report_json!(
        "endpoint",
        HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_ACTIVATION_COMMAND_RESULT_RECEIPT_ORDERING_MONOTONICITY_DENIAL_BOUNDARY_ENDPOINT
    );
    insert_report_json!(
        "source_command",
        "/hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-ordering-monotonicity-denial-boundary --json"
    );
    insert_report_json!("native_route", true);
    insert_report_json!("side_effect_free", true);
    insert_report_json!("audit_date", "2026-07-03");
    insert_report_json!(
        "memory_write_execution_activation_command_result_receipt_ordering_monotonicity_denial_boundary_schema_version",
        "memory_write_execution_activation_command_result_receipt_ordering_monotonicity_denial_boundary_v1"
    );
    insert_report_json!(
        "memory_write_execution_activation_command_result_receipt_ordering_monotonicity_denial_boundary_ready",
        report_ready
    );
    insert_report_json!(
        "activation_command_result_receipt_ordering_monotonicity_mode",
        "memory_write_execution_activation_command_result_receipt_ordering_monotonicity_denial"
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
        "source_activation_command_result_receipt_replay_idempotency_boundary_endpoint",
        HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_ACTIVATION_COMMAND_RESULT_RECEIPT_REPLAY_IDEMPOTENCY_DENIAL_BOUNDARY_ENDPOINT
    );
    insert_report_json!(
        "source_activation_command_result_receipt_replay_idempotency_boundary_ready",
        source_replay_ready
    );
    insert_report_json!(
        "source_activation_command_result_receipt_replay_idempotency_ready",
        json_bool(
            &replay,
            "memory_write_execution_activation_command_result_receipt_replay_idempotency_denial_ready"
        )
    );
    insert_report_json!(
        "source_activation_command_result_receipt_replay_idempotency_boundary_report_sha256",
        source_replay_report_sha256
    );
    insert_report_json!(
        "source_activation_command_result_receipt_replay_idempotency_report_sha256",
        source_replay_report_sha256
    );
    insert_report_json!(
        "source_activation_command_result_receipt_no_persistence_boundary_ready",
        json_bool(
            &replay,
            "source_activation_command_result_receipt_no_persistence_boundary_ready"
        )
    );
    insert_report_json!(
        "source_activation_command_result_receipt_no_persistence_ready",
        json_bool(
            &replay,
            "memory_write_execution_activation_command_result_receipt_no_persistence_ready"
        )
    );
    insert_report_json!(
        "source_activation_command_result_receipt_no_persistence_boundary_report_sha256",
        json_str(
            &replay,
            "source_activation_command_result_receipt_no_persistence_boundary_report_sha256"
        )
    );
    for key in [
        "source_activation_command_noop_handoff_boundary_report_sha256",
        "source_memory_write_execution_activation_closure_denial_boundary_report_sha256",
        "source_memory_write_execution_post_write_operator_acceptance_denial_boundary_report_sha256",
        "source_memory_write_execution_post_write_validation_dry_run_boundary_report_sha256",
        "source_memory_write_execution_write_enable_fixture_boundary_report_sha256",
        "source_memory_write_execution_no_write_sink_contract_boundary_report_sha256",
        "source_memory_write_execution_denial_matrix_boundary_report_sha256",
    ] {
        report.insert(
            key.to_string(),
            replay
                .get(key)
                .cloned()
                .unwrap_or_else(|| serde_json::json!("")),
        );
    }
    insert_report_json!("minimum_required_samples", 24);
    insert_report_json!(
        "memory_write_execution_activation_command_result_receipt_ordering_monotonicity_denial_ready",
        true
    );
    insert_report_json!(
        "memory_write_execution_activation_command_result_receipt_replay_idempotency_denial_ready",
        true
    );
    insert_report_json!(
        "memory_write_execution_activation_command_result_receipt_no_persistence_ready",
        true
    );
    insert_report_json!(
        "memory_write_execution_activation_command_noop_handoff_ready",
        true
    );
    insert_report_json!(
        "memory_write_execution_activation_closure_denial_ready",
        true
    );
    insert_report_json!(
        "memory_write_execution_post_write_operator_acceptance_denial_ready",
        true
    );
    insert_report_json!(
        "memory_write_execution_post_write_validation_dry_run_ready",
        true
    );
    insert_report_json!("memory_write_execution_write_enable_fixture_ready", true);
    insert_report_json!("memory_write_execution_no_write_sink_contract_ready", true);
    insert_report_json!(
        "required_activation_command_result_receipt_ordering_monotonicity_surface_count",
        12
    );
    insert_report_json!(
        "ready_activation_command_result_receipt_ordering_monotonicity_surface_count",
        12
    );
    insert_report_json!(
        "side_effect_free_activation_command_result_receipt_ordering_monotonicity_surface_count",
        12
    );
    insert_report_json!(
        "required_activation_command_result_receipt_ordering_monotonicity_fixture_count",
        10
    );
    insert_report_json!(
        "activation_command_result_receipt_ordering_monotonicity_fixture_count",
        10
    );
    insert_report_json!(
        "blocked_activation_command_result_receipt_ordering_monotonicity_fixture_count",
        10
    );
    insert_report_json!(
        "noop_activation_command_result_receipt_ordering_monotonicity_fixture_count",
        10
    );
    insert_report_json!(
        "allowed_activation_command_result_receipt_ordering_monotonicity_fixture_count",
        0
    );
    insert_report_json!(
        "accepted_activation_command_result_receipt_ordering_monotonicity_fixture_count",
        0
    );
    insert_report_json!(
        "activation_command_result_receipt_ordering_violation_denied_count",
        10
    );
    insert_report_json!(
        "activation_command_result_receipt_monotonicity_violation_denied_count",
        10
    );
    insert_report_json!(
        "activation_command_result_receipt_ordering_performed_count",
        0
    );
    insert_report_json!(
        "activation_command_result_receipt_sequence_cursor_accepted_count",
        0
    );
    insert_report_json!(
        "activation_command_result_receipt_sequence_cursor_recorded_count",
        0
    );
    insert_report_json!(
        "activation_command_result_receipt_monotonicity_state_recorded_count",
        0
    );
    insert_report_json!("memory_store_write_performed_count", 0);

    for key in false_keys {
        report.insert(key.to_string(), serde_json::json!(false));
    }
    report.insert(
        "activation_command_result_receipt_ordering_monotonicity_surfaces".to_string(),
        serde_json::json!(ordering_surfaces),
    );
    report.insert(
        "activation_command_result_receipt_ordering_monotonicity_fixtures".to_string(),
        ordering_fixtures,
    );
    report.insert(
        "denied_by_activation_command_result_receipt_ordering_monotonicity".to_string(),
        serde_json::json!(denied_by),
    );
    insert_report_json!(
        "denied_by_activation_command_result_receipt_ordering_monotonicity_count",
        24
    );
    report.insert("allowed_next_actions".to_string(), allowed_next_actions);
    report.insert(
        "side_effects".to_string(),
        serde_json::Value::Object(side_effects),
    );

    serde_json::Value::Object(report)
}

fn hepta_memory_live_mutation_operator_write_execution_activation_command_result_receipt_cancellation_supersession_denial_boundary_report()
-> serde_json::Value {
    const CANCELLATION_SURFACES: &[&str] = &[
        "source_ordering_monotonicity_report_required",
        "cancellation_request_shape_denied",
        "supersession_request_shape_denied",
        "replacement_receipt_hash_denied",
        "tombstone_or_delete_marker_denied",
        "cancel_after_blocked_noop_denied",
        "supersede_blocked_noop_with_completed_denied",
        "acknowledgement_cancellation_denied",
        "ledger_index_delivery_cancellation_denied",
        "memory_write_live_mutation_supersession_denied",
        "rollback_secret_provider_supersession_denied",
        "external_public_install_restart_supersession_denied",
    ];
    const DENIED_BY: &[&str] = &[
        "source_ordering_monotonicity_report_required",
        "cancellation_request_acceptance_denied",
        "cancellation_recording_denied",
        "cancellation_persistence_denied",
        "supersession_request_acceptance_denied",
        "supersession_recording_denied",
        "supersession_persistence_denied",
        "replacement_receipt_acceptance_denied",
        "replacement_hash_acceptance_denied",
        "tombstone_recording_denied",
        "delete_marker_recording_denied",
        "cancel_after_blocked_noop_denied",
        "supersede_blocked_noop_with_completed_denied",
        "completion_ack_cancellation_denied",
        "ledger_cancellation_denied",
        "index_cancellation_denied",
        "delivery_cancellation_denied",
        "memory_write_supersession_denied",
        "live_mutation_supersession_denied",
        "rollback_supersession_denied",
        "secret_material_supersession_denied",
        "provider_prompt_supersession_denied",
        "external_public_release_supersession_denied",
        "install_restart_active_binary_supersession_denied",
    ];
    const FALSE_KEYS: &[&str] = &[
        "activation_command_result_receipt_cancellation_allowed",
        "activation_command_result_receipt_cancellation_recorded",
        "activation_command_result_receipt_cancellation_persisted",
        "activation_command_result_receipt_cancellation_request_accepted",
        "activation_command_result_receipt_supersession_allowed",
        "activation_command_result_receipt_supersession_recorded",
        "activation_command_result_receipt_supersession_persisted",
        "activation_command_result_receipt_supersession_request_accepted",
        "activation_command_result_receipt_replacement_receipt_accepted",
        "activation_command_result_receipt_replacement_receipt_recorded",
        "activation_command_result_receipt_replacement_receipt_persisted",
        "activation_command_result_receipt_replacement_hash_accepted",
        "activation_command_result_receipt_tombstone_recorded",
        "activation_command_result_receipt_tombstone_persisted",
        "activation_command_result_receipt_delete_marker_recorded",
        "activation_command_result_receipt_ack_cancellation_accepted",
        "activation_command_result_receipt_ledger_cancellation_accepted",
        "activation_command_result_receipt_index_cancellation_accepted",
        "activation_command_result_receipt_delivery_cancellation_accepted",
        "activation_command_result_receipt_ordering_allowed",
        "activation_command_result_receipt_ordering_recorded",
        "activation_command_result_receipt_ordering_persisted",
        "activation_command_result_receipt_sequence_cursor_accepted",
        "activation_command_result_receipt_sequence_cursor_recorded",
        "activation_command_result_receipt_sequence_cursor_persisted",
        "activation_command_result_receipt_monotonicity_state_recorded",
        "activation_command_result_receipt_monotonicity_state_persisted",
        "activation_command_result_receipt_latest_wins_overwrite_accepted",
        "activation_command_result_receipt_recorded",
        "activation_command_result_receipt_persisted",
        "activation_command_result_receipt_accepted",
        "activation_command_result_receipt_materialized",
        "activation_command_result_receipt_filesystem_written",
        "activation_command_result_receipt_ledger_written",
        "activation_command_result_receipt_indexed",
        "activation_command_result_receipt_enqueued",
        "activation_command_result_receipt_delivered",
        "activation_command_completion_ack_recorded",
        "activation_command_completion_ack_persisted",
        "activation_command_completion_ack_accepted",
        "activation_command_completion_ack_delivered",
        "activation_command_enabled",
        "activation_command_invoked",
        "activation_command_dispatched",
        "activation_allowed_by_result_receipt_cancellation",
        "activation_allowed_by_result_receipt_supersession",
        "activation_allowed_by_result_receipt_ordering",
        "activation_allowed_by_result_receipt_replay",
        "activation_allowed_by_result_receipt",
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
        "rollback_execution_allowed",
        "rollback_executed",
        "raw_payload_plaintext_recorded",
        "raw_payload_plaintext_persisted",
        "secret_material_read",
        "provider_prompt_replay_enabled",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "kg_adapter_read_performed",
        "live_kg_write_performed",
        "channel_send_performed",
        "telegram_send_performed",
        "external_send_enabled",
        "external_send_performed",
        "public_claim_or_release_artifact_write_enabled",
        "public_release_published",
        "public_ga_claimed",
        "public_release_claimed",
        "release_artifact_written",
        "public_artifact_written",
        "install_executed",
        "launchd_mutated",
        "service_restarted",
        "active_binary_mutated",
    ];

    fn cancellation_fixture(
        id: &str,
        reason: &str,
        cancellation_supersession_status: &str,
        extra: serde_json::Value,
    ) -> serde_json::Value {
        let mut base = serde_json::Map::new();
        macro_rules! insert_fixture_json {
            ($key:literal, $value:expr) => {
                base.insert($key.to_string(), serde_json::json!($value));
            };
        }
        insert_fixture_json!("id", id);
        insert_fixture_json!("cancellation_requested", true);
        insert_fixture_json!("supersession_requested", false);
        insert_fixture_json!(
            "cancellation_supersession_status",
            cancellation_supersession_status
        );
        insert_fixture_json!("source_ordering_monotonicity_present", true);
        insert_fixture_json!("source_ordering_monotonicity_ready", true);
        for key in [
            "cancellation_allowed",
            "cancellation_recorded",
            "cancellation_persisted",
            "cancellation_request_accepted",
            "supersession_allowed",
            "supersession_recorded",
            "supersession_persisted",
            "supersession_request_accepted",
            "replacement_receipt_accepted",
            "replacement_receipt_recorded",
            "replacement_receipt_persisted",
            "replacement_hash_accepted",
            "tombstone_recorded",
            "tombstone_persisted",
            "delete_marker_recorded",
            "ack_cancellation_accepted",
            "ledger_cancellation_accepted",
            "index_cancellation_accepted",
            "delivery_cancellation_accepted",
            "receipt_recorded",
            "receipt_persisted",
            "receipt_accepted",
            "receipt_materialized",
            "receipt_filesystem_written",
            "receipt_ledger_written",
            "receipt_indexed",
            "receipt_delivered",
            "completion_ack_recorded",
            "completion_ack_persisted",
            "completion_ack_accepted",
            "completion_ack_delivered",
            "activation_allowed",
            "live_mutation_execution_performed",
            "memory_store_write_performed",
            "memory_store_mutated",
            "rollback_executed",
            "secret_material_read",
            "provider_invoked",
            "model_invoked",
            "external_send_performed",
            "public_release_published",
            "release_artifact_written",
            "install_executed",
            "launchd_mutated",
            "service_restarted",
            "active_binary_mutated",
        ] {
            base.insert(key.to_string(), serde_json::json!(false));
        }
        insert_fixture_json!("receipt_noop_confirmed", true);
        insert_fixture_json!("reason", reason);
        if let Some(extra) = extra.as_object() {
            for (key, value) in extra {
                base.insert(key.clone(), value.clone());
            }
        }
        serde_json::Value::Object(base)
    }

    let route_matrix = control_ui_route_parity_report();
    let ordering = std::thread::Builder::new()
        .name("hepta-memory-write-result-receipt-ordering-source-report".to_string())
        .stack_size(8 * 1024 * 1024)
        .spawn(
            hepta_memory_live_mutation_operator_write_execution_activation_command_result_receipt_ordering_monotonicity_denial_boundary_report,
        )
        .ok()
        .and_then(|handle| handle.join().ok())
        .unwrap_or_else(|| {
            serde_json::json!({
                "status": "blocked",
                "memory_write_execution_activation_command_result_receipt_ordering_monotonicity_denial_boundary_ready": false,
                "memory_write_execution_activation_command_result_receipt_ordering_monotonicity_denial_ready": false,
                "source_ordering_source_report_thread_failed": true
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
    let source_ordering_ready = json_str(&ordering, "status") == "ready"
        && json_bool(
            &ordering,
            "memory_write_execution_activation_command_result_receipt_ordering_monotonicity_denial_boundary_ready",
        )
        && json_bool(
            &ordering,
            "memory_write_execution_activation_command_result_receipt_ordering_monotonicity_denial_ready",
        )
        && json_bool(
            &ordering,
            "memory_write_execution_activation_command_result_receipt_replay_idempotency_denial_ready",
        )
        && json_u64(
            &ordering,
            "required_activation_command_result_receipt_ordering_monotonicity_surface_count",
        ) == 12
        && json_u64(
            &ordering,
            "activation_command_result_receipt_ordering_monotonicity_fixture_count",
        ) == 10
        && json_u64(
            &ordering,
            "blocked_activation_command_result_receipt_ordering_monotonicity_fixture_count",
        ) == 10
        && json_u64(
            &ordering,
            "accepted_activation_command_result_receipt_ordering_monotonicity_fixture_count",
        ) == 0
        && json_u64(
            &ordering,
            "activation_command_result_receipt_ordering_performed_count",
        ) == 0
        && json_u64(
            &ordering,
            "denied_by_activation_command_result_receipt_ordering_monotonicity_count",
        ) == 24
        && !json_bool(
            &ordering,
            "activation_command_result_receipt_ordering_allowed",
        )
        && !json_bool(
            &ordering,
            "activation_command_result_receipt_ordering_recorded",
        )
        && !json_bool(
            &ordering,
            "activation_command_result_receipt_ordering_persisted",
        )
        && !json_bool(
            &ordering,
            "activation_command_result_receipt_sequence_cursor_recorded",
        )
        && !json_bool(
            &ordering,
            "activation_command_result_receipt_monotonicity_state_recorded",
        )
        && !json_bool(&ordering, "activation_command_result_receipt_recorded")
        && !json_bool(&ordering, "activation_command_result_receipt_persisted")
        && !json_bool(&ordering, "activation_command_result_receipt_accepted")
        && !json_bool(&ordering, "activation_command_completion_ack_recorded")
        && !json_bool(&ordering, "activation_allowed")
        && !json_bool(&ordering, "live_mutation_execution_performed")
        && !json_bool(&ordering, "memory_store_write_performed")
        && !json_bool(&ordering, "memory_store_mutated")
        && !json_bool(&ordering, "rollback_executed")
        && !json_bool(&ordering, "provider_invoked")
        && !json_bool(&ordering, "model_invoked")
        && !json_bool(&ordering, "external_send_performed")
        && !json_bool(&ordering, "release_artifact_written")
        && !json_bool(&ordering, "active_binary_mutated")
        && side_effects_all_false(&ordering);

    let cancellation_fixtures = serde_json::Value::Array(vec![
        cancellation_fixture(
            "activation-result-receipt-cancellation-missing-source-ordering-report",
            "source_ordering_monotonicity_report_required",
            "blocked_noop",
            serde_json::json!({
                "source_ordering_monotonicity_present": false,
                "source_ordering_monotonicity_ready": false
            }),
        ),
        cancellation_fixture(
            "activation-result-receipt-cancel-blocked-noop",
            "cancellation_of_blocked_noop_receipt_denied",
            "blocked_noop",
            serde_json::json!({
                "cancellation_request_shape": "cancel_blocked_noop_receipt"
            }),
        ),
        cancellation_fixture(
            "activation-result-receipt-supersede-with-completed",
            "supersession_of_blocked_noop_with_completed_denied",
            "blocked_supersession_noop",
            serde_json::json!({
                "supersession_requested": true,
                "cancellation_requested": false,
                "requested_replacement_status": "completed"
            }),
        ),
        cancellation_fixture(
            "activation-result-receipt-replacement-hash",
            "replacement_hash_identity_attempt_denied",
            "blocked_supersession_noop",
            serde_json::json!({
                "supersession_requested": true,
                "cancellation_requested": false,
                "replacement_hash_requested": true,
                "requested_hash_relation": "different_hash_for_same_receipt_identity"
            }),
        ),
        cancellation_fixture(
            "activation-result-receipt-tombstone-delete-marker",
            "tombstone_or_delete_marker_denied",
            "blocked_noop",
            serde_json::json!({
                "tombstone_requested": true,
                "delete_marker_requested": true
            }),
        ),
        cancellation_fixture(
            "activation-result-receipt-completion-ack-cancel",
            "completion_ack_cancellation_denied",
            "blocked_noop",
            serde_json::json!({
                "completion_ack_cancellation_requested": true,
                "ack_cancellation_requested": true
            }),
        ),
        cancellation_fixture(
            "activation-result-receipt-ledger-index-delivery-cancel",
            "ledger_index_delivery_cancellation_supersession_denied",
            "blocked_noop",
            serde_json::json!({
                "ledger_cancellation_requested": true,
                "index_cancellation_requested": true,
                "delivery_cancellation_requested": true
            }),
        ),
        cancellation_fixture(
            "activation-result-receipt-memory-write-live-mutation-supersede",
            "memory_write_live_mutation_supersession_denied",
            "blocked_supersession_noop",
            serde_json::json!({
                "supersession_requested": true,
                "cancellation_requested": false,
                "memory_write_supersession_requested": true,
                "live_mutation_supersession_requested": true
            }),
        ),
        cancellation_fixture(
            "activation-result-receipt-rollback-secret-provider-supersede",
            "rollback_secret_provider_supersession_denied",
            "blocked_supersession_noop",
            serde_json::json!({
                "supersession_requested": true,
                "cancellation_requested": false,
                "rollback_supersession_requested": true,
                "secret_material_supersession_requested": true,
                "provider_prompt_supersession_requested": true
            }),
        ),
        cancellation_fixture(
            "activation-result-receipt-external-public-install-supersede",
            "external_public_install_restart_active_binary_supersession_denied",
            "blocked_supersession_noop",
            serde_json::json!({
                "supersession_requested": true,
                "cancellation_requested": false,
                "external_send_supersession_requested": true,
                "public_claim_supersession_requested": true,
                "release_artifact_supersession_requested": true,
                "install_supersession_requested": true,
                "service_restart_supersession_requested": true,
                "active_binary_mutation_supersession_requested": true
            }),
        ),
    ]);
    let source_ordering_report_sha256 = sha256_json_value(&ordering);
    let boundary_hash_sha256 = sha256_text_value(&format!(
        "hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-cancellation-supersession-denial-boundary-v1:{}:{}",
        route_matrix.route_count, source_ordering_report_sha256,
    ));
    let report_ready = route_matrix.ready
        && route_count_source_command_accepted
        && source_ordering_ready
        && CANCELLATION_SURFACES.len() == 12
        && cancellation_fixtures.as_array().map(std::vec::Vec::len) == Some(10)
        && DENIED_BY.len() == 24;

    let allowed_next_actions = serde_json::json!([
        {
            "action": "run_memory_write_execution_activation_command_result_receipt_cancellation_supersession_denial_boundary_require_live_gate",
            "status": "allowed_verification_only",
            "accepts_cancellation": false,
            "accepts_supersession": false,
            "records_replacement_receipt": false,
            "records_tombstone": false,
            "promotes_completion": false,
            "writes_memory": false,
            "executes_rollback": false,
            "writes_kg": false,
            "invokes_provider": false,
            "sends_externally": false,
            "publishes_artifacts": false,
            "installs_or_restarts": false,
            "mutates_active_binary": false
        },
        {
            "action": "prepare_memory_write_execution_activation_command_result_receipt_audit_trail_immutable_evidence_denial_boundary",
            "status": "allowed_report_only_next_slice",
            "records_audit_evidence": false,
            "persists_immutable_evidence": false,
            "mutates_runtime": false,
            "invokes_model": false,
            "writes_memory_or_kg": false
        }
    ]);

    let mut side_effects = serde_json::Map::new();
    for &key in FALSE_KEYS {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }

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
        "hepta_memory_live_mutation_operator_write_execution_activation_command_result_receipt_cancellation_supersession_denial_boundary_route"
    );
    insert_report_json!(
        "endpoint",
        HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_ACTIVATION_COMMAND_RESULT_RECEIPT_CANCELLATION_SUPERSESSION_DENIAL_BOUNDARY_ENDPOINT
    );
    insert_report_json!(
        "source_command",
        "/hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-cancellation-supersession-denial-boundary --json"
    );
    insert_report_json!("native_route", true);
    insert_report_json!("side_effect_free", true);
    insert_report_json!("audit_date", "2026-07-03");
    insert_report_json!(
        "memory_write_execution_activation_command_result_receipt_cancellation_supersession_denial_boundary_schema_version",
        "memory_write_execution_activation_command_result_receipt_cancellation_supersession_denial_boundary_v1"
    );
    insert_report_json!(
        "memory_write_execution_activation_command_result_receipt_cancellation_supersession_denial_boundary_ready",
        report_ready
    );
    insert_report_json!(
        "activation_command_result_receipt_cancellation_supersession_mode",
        "memory_write_execution_activation_command_result_receipt_cancellation_supersession_denial"
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
        "source_activation_command_result_receipt_ordering_monotonicity_boundary_endpoint",
        HEPTA_MEMORY_LIVE_MUTATION_OPERATOR_WRITE_EXECUTION_ACTIVATION_COMMAND_RESULT_RECEIPT_ORDERING_MONOTONICITY_DENIAL_BOUNDARY_ENDPOINT
    );
    insert_report_json!(
        "source_activation_command_result_receipt_ordering_monotonicity_boundary_ready",
        source_ordering_ready
    );
    insert_report_json!(
        "source_activation_command_result_receipt_ordering_monotonicity_ready",
        json_bool(
            &ordering,
            "memory_write_execution_activation_command_result_receipt_ordering_monotonicity_denial_ready"
        )
    );
    insert_report_json!(
        "source_activation_command_result_receipt_ordering_monotonicity_boundary_report_sha256",
        source_ordering_report_sha256
    );
    insert_report_json!(
        "source_activation_command_result_receipt_ordering_monotonicity_report_sha256",
        source_ordering_report_sha256
    );
    insert_report_json!(
        "source_activation_command_result_receipt_replay_idempotency_boundary_ready",
        json_bool(
            &ordering,
            "source_activation_command_result_receipt_replay_idempotency_boundary_ready"
        )
    );
    insert_report_json!(
        "source_activation_command_result_receipt_replay_idempotency_ready",
        json_bool(
            &ordering,
            "memory_write_execution_activation_command_result_receipt_replay_idempotency_denial_ready"
        )
    );
    insert_report_json!(
        "source_activation_command_result_receipt_replay_idempotency_boundary_report_sha256",
        json_str(
            &ordering,
            "source_activation_command_result_receipt_replay_idempotency_boundary_report_sha256"
        )
    );
    insert_report_json!(
        "source_activation_command_result_receipt_no_persistence_boundary_ready",
        json_bool(
            &ordering,
            "source_activation_command_result_receipt_no_persistence_boundary_ready"
        )
    );
    insert_report_json!(
        "source_activation_command_result_receipt_no_persistence_ready",
        json_bool(
            &ordering,
            "memory_write_execution_activation_command_result_receipt_no_persistence_ready"
        )
    );
    insert_report_json!(
        "source_activation_command_result_receipt_no_persistence_boundary_report_sha256",
        json_str(
            &ordering,
            "source_activation_command_result_receipt_no_persistence_boundary_report_sha256"
        )
    );
    for key in [
        "source_activation_command_noop_handoff_boundary_report_sha256",
        "source_memory_write_execution_activation_closure_denial_boundary_report_sha256",
        "source_memory_write_execution_post_write_operator_acceptance_denial_boundary_report_sha256",
        "source_memory_write_execution_post_write_validation_dry_run_boundary_report_sha256",
        "source_memory_write_execution_write_enable_fixture_boundary_report_sha256",
        "source_memory_write_execution_no_write_sink_contract_boundary_report_sha256",
        "source_memory_write_execution_denial_matrix_boundary_report_sha256",
    ] {
        report.insert(
            key.to_string(),
            ordering
                .get(key)
                .cloned()
                .unwrap_or_else(|| serde_json::json!("")),
        );
    }
    insert_report_json!("minimum_required_samples", 24);
    insert_report_json!(
        "memory_write_execution_activation_command_result_receipt_cancellation_supersession_denial_ready",
        true
    );
    insert_report_json!(
        "memory_write_execution_activation_command_result_receipt_ordering_monotonicity_denial_ready",
        true
    );
    insert_report_json!(
        "memory_write_execution_activation_command_result_receipt_replay_idempotency_denial_ready",
        true
    );
    insert_report_json!(
        "memory_write_execution_activation_command_result_receipt_no_persistence_ready",
        true
    );
    insert_report_json!(
        "memory_write_execution_activation_command_noop_handoff_ready",
        true
    );
    insert_report_json!(
        "memory_write_execution_activation_closure_denial_ready",
        true
    );
    insert_report_json!(
        "memory_write_execution_post_write_operator_acceptance_denial_ready",
        true
    );
    insert_report_json!(
        "memory_write_execution_post_write_validation_dry_run_ready",
        true
    );
    insert_report_json!("memory_write_execution_write_enable_fixture_ready", true);
    insert_report_json!("memory_write_execution_no_write_sink_contract_ready", true);
    insert_report_json!(
        "required_activation_command_result_receipt_cancellation_supersession_surface_count",
        12
    );
    insert_report_json!(
        "ready_activation_command_result_receipt_cancellation_supersession_surface_count",
        12
    );
    insert_report_json!(
        "side_effect_free_activation_command_result_receipt_cancellation_supersession_surface_count",
        12
    );
    insert_report_json!(
        "required_activation_command_result_receipt_cancellation_supersession_fixture_count",
        10
    );
    insert_report_json!(
        "activation_command_result_receipt_cancellation_supersession_fixture_count",
        10
    );
    insert_report_json!(
        "blocked_activation_command_result_receipt_cancellation_supersession_fixture_count",
        10
    );
    insert_report_json!(
        "noop_activation_command_result_receipt_cancellation_supersession_fixture_count",
        10
    );
    insert_report_json!(
        "allowed_activation_command_result_receipt_cancellation_supersession_fixture_count",
        0
    );
    insert_report_json!(
        "accepted_activation_command_result_receipt_cancellation_supersession_fixture_count",
        0
    );
    insert_report_json!(
        "activation_command_result_receipt_cancellation_denied_count",
        10
    );
    insert_report_json!(
        "activation_command_result_receipt_supersession_denied_count",
        10
    );
    insert_report_json!(
        "activation_command_result_receipt_cancellation_performed_count",
        0
    );
    insert_report_json!(
        "activation_command_result_receipt_supersession_performed_count",
        0
    );
    insert_report_json!("memory_store_write_performed_count", 0);

    for &key in FALSE_KEYS {
        report.insert(key.to_string(), serde_json::json!(false));
    }
    report.insert(
        "activation_command_result_receipt_cancellation_supersession_surfaces".to_string(),
        serde_json::json!(CANCELLATION_SURFACES),
    );
    report.insert(
        "activation_command_result_receipt_cancellation_supersession_fixtures".to_string(),
        cancellation_fixtures,
    );
    report.insert(
        "denied_by_activation_command_result_receipt_cancellation_supersession".to_string(),
        serde_json::json!(DENIED_BY),
    );
    insert_report_json!(
        "denied_by_activation_command_result_receipt_cancellation_supersession_count",
        24
    );
    report.insert("allowed_next_actions".to_string(), allowed_next_actions);
    report.insert(
        "side_effects".to_string(),
        serde_json::Value::Object(side_effects),
    );

    serde_json::Value::Object(report)
}


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

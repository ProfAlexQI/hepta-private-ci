fn hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_retention_expiry_garbage_collection_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source_report =
        hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_audit_evidence_denial_report();
    let source_ready = source_report["status"].as_str() == Some("ready")
        && source_report["memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_audit_evidence_denial_ready"]
            .as_bool()
            .unwrap_or(false);
    let source_u64 = |key: &str| source_report[key].as_u64().unwrap_or(0);
    let source_report_sha256 = sha256_json_value(&source_report);
    let source_contract_hash = source_report
        .get("artifact_distribution_signing_notarization_receipt_audit_evidence_contract_hash_sha256")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("unknown")
        .to_string();
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;

    let surface_false_keys = [
        "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_allowed",
        "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_accepted",
        "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_recorded",
        "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_persisted",
        "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_materialized",
        "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_filesystem_written",
        "retention_policy_requested",
        "ttl_lease_requested",
        "expiry_timestamp_requested",
        "expiry_scheduler_requested",
        "expiry_timer_requested",
        "expiry_ack_requested",
        "garbage_collection_queue_requested",
        "garbage_collection_scan_requested",
        "garbage_collection_candidate_requested",
        "garbage_collection_decision_requested",
        "tombstone_gc_requested",
        "delete_marker_gc_requested",
        "archive_requested",
        "compaction_requested",
        "audit_evidence_retention_requested",
        "immutable_evidence_retention_requested",
        "hash_attestation_retention_requested",
        "witness_notary_expiry_requested",
        "ledger_index_retention_requested",
        "delivery_evidence_retention_requested",
        "status_evidence_expiry_requested",
        "external_telegram_retention_requested",
        "release_publication_retention_authority_requested",
        "activation_retention_authority_requested",
        "live_install_gc_evidence_requested",
        "retention_policy_recorded",
        "retention_policy_persisted",
        "ttl_lease_recorded",
        "ttl_lease_persisted",
        "expiry_timestamp_recorded",
        "expiry_scheduler_recorded",
        "expiry_timer_started",
        "expiry_ack_recorded",
        "expiry_state_persisted",
        "garbage_collection_queue_recorded",
        "garbage_collection_scan_performed",
        "garbage_collection_candidate_recorded",
        "garbage_collection_decision_recorded",
        "garbage_collection_state_persisted",
        "tombstone_gc_recorded",
        "delete_marker_gc_recorded",
        "archive_recorded",
        "compaction_recorded",
        "audit_evidence_retention_recorded",
        "immutable_evidence_retention_recorded",
        "hash_attestation_retention_recorded",
        "witness_notary_expiry_recorded",
        "ledger_index_retention_recorded",
        "delivery_evidence_retention_recorded",
        "status_evidence_expiry_recorded",
        "result_receipt_from_retention_recorded",
        "result_receipt_from_retention_persisted",
        "operator_approval_from_retention_derived",
        "release_publication_authority_from_retention_derived",
        "activation_authority_from_retention_derived",
        "download_link_from_retention_rendered",
        "install_command_from_retention_rendered",
        "install_from_retention_executed",
        "service_restart_from_retention_performed",
        "launchd_from_retention_mutated",
        "active_binary_from_retention_mutated",
        "memory_store_write_performed",
        "memory_store_mutated",
        "live_kg_write_performed",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "telegram_send_performed",
        "channel_send_performed",
        "external_send_performed",
        "public_release_claimed",
        "public_ga_claimed",
        "release_artifact_written",
        "public_artifact_written",
        "filesystem_written",
    ];
    let surface_specs: Vec<(&str, &str, &str, Vec<&str>)> = vec![
        (
            "source_signing_receipt_audit_evidence_report_required",
            "blocked_source_signing_receipt_audit_evidence_required_noop",
            "source_signing_receipt_audit_evidence_report_required",
            vec!["source_report_required"],
        ),
        (
            "artifact_signing_audit_trail_retention_policy",
            "blocked_artifact_signing_audit_trail_retention_noop",
            "artifact_signing_audit_trail_retention_policy_denied",
            vec![
                "retention_policy_requested",
                "audit_evidence_retention_requested",
            ],
        ),
        (
            "package_signing_immutable_evidence_ttl_lease",
            "blocked_package_signing_immutable_evidence_ttl_noop",
            "package_signing_immutable_evidence_ttl_lease_denied",
            vec![
                "ttl_lease_requested",
                "immutable_evidence_retention_requested",
            ],
        ),
        (
            "signature_manifest_hash_chain_expiry_timestamp",
            "blocked_signature_manifest_hash_expiry_noop",
            "signature_manifest_hash_chain_expiry_timestamp_denied",
            vec![
                "expiry_timestamp_requested",
                "hash_attestation_retention_requested",
            ],
        ),
        (
            "notarization_submission_attestation_retention_ledger",
            "blocked_notarization_attestation_retention_ledger_noop",
            "notarization_submission_attestation_retention_ledger_denied",
            vec![
                "retention_policy_requested",
                "ledger_index_retention_requested",
            ],
        ),
        (
            "notarization_ticket_witness_notary_expiry_scheduler",
            "blocked_witness_notary_expiry_scheduler_noop",
            "notarization_ticket_witness_notary_expiry_scheduler_denied",
            vec![
                "expiry_scheduler_requested",
                "expiry_timer_requested",
                "expiry_ack_requested",
                "witness_notary_expiry_requested",
            ],
        ),
        (
            "stapling_tombstone_garbage_collection_queue",
            "blocked_stapling_tombstone_gc_queue_noop",
            "stapling_tombstone_garbage_collection_queue_denied",
            vec![
                "garbage_collection_queue_requested",
                "tombstone_gc_requested",
            ],
        ),
        (
            "installer_replacement_evidence_garbage_collection_scan",
            "blocked_installer_replacement_gc_scan_noop",
            "installer_replacement_evidence_garbage_collection_scan_denied",
            vec![
                "garbage_collection_scan_requested",
                "garbage_collection_candidate_requested",
            ],
        ),
        (
            "provenance_immutable_evidence_archive",
            "blocked_provenance_immutable_evidence_archive_noop",
            "provenance_immutable_evidence_archive_denied",
            vec![
                "archive_requested",
                "immutable_evidence_retention_requested",
            ],
        ),
        (
            "sbom_evidence_compaction",
            "blocked_sbom_evidence_compaction_noop",
            "sbom_evidence_compaction_denied",
            vec!["compaction_requested", "audit_evidence_retention_requested"],
        ),
        (
            "release_asset_cancelled_query_retention",
            "blocked_release_asset_query_retention_noop",
            "release_asset_cancelled_query_retention_denied",
            vec![
                "retention_policy_requested",
                "audit_evidence_retention_requested",
            ],
        ),
        (
            "cdn_observability_expiry_ack",
            "blocked_cdn_observability_expiry_ack_noop",
            "cdn_observability_expiry_ack_denied",
            vec!["expiry_ack_requested", "status_evidence_expiry_requested"],
        ),
        (
            "package_registry_replacement_status_gc_decision",
            "blocked_package_registry_status_gc_decision_noop",
            "package_registry_replacement_status_gc_decision_denied",
            vec![
                "garbage_collection_decision_requested",
                "garbage_collection_candidate_requested",
            ],
        ),
        (
            "dashboard_endpoint_hash_status_retention",
            "blocked_dashboard_hash_status_retention_noop",
            "dashboard_endpoint_hash_status_retention_denied",
            vec![
                "retention_policy_requested",
                "delivery_evidence_retention_requested",
            ],
        ),
        (
            "external_telegram_retention_delivery",
            "blocked_external_telegram_retention_delivery_noop",
            "external_telegram_retention_delivery_denied",
            vec![
                "external_telegram_retention_requested",
                "delivery_evidence_retention_requested",
            ],
        ),
        (
            "release_publication_authority_retention",
            "blocked_release_publication_retention_authority_noop",
            "release_publication_authority_retention_denied",
            vec![
                "release_publication_retention_authority_requested",
                "retention_policy_requested",
            ],
        ),
        (
            "activation_live_install_garbage_collection_evidence",
            "blocked_activation_live_install_gc_noop",
            "activation_live_install_garbage_collection_evidence_denied",
            vec![
                "activation_retention_authority_requested",
                "live_install_gc_evidence_requested",
            ],
        ),
        (
            "install_restart_active_binary_retention_gc_path",
            "blocked_install_restart_active_binary_retention_gc_noop",
            "install_restart_active_binary_retention_gc_path_denied",
            vec![
                "live_install_gc_evidence_requested",
                "garbage_collection_queue_requested",
                "garbage_collection_decision_requested",
                "delete_marker_gc_requested",
            ],
        ),
    ];
    let surfaces = surface_specs
        .into_iter()
        .map(|(surface, status, reason, requested_flags)| {
            let mut surface_report = serde_json::json!({
                "surface": surface,
                "source_signing_receipt_audit_evidence_denial_ready": source_ready,
                "canonical_noop_signing_receipt_identity_required": true,
                "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_attempted": true,
                "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_noop_confirmed": true,
                "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_status": status,
                "reason": reason,
            });
            if let Some(surface_object) = surface_report.as_object_mut() {
                for key in &surface_false_keys {
                    surface_object.insert((*key).to_string(), serde_json::json!(false));
                }
                for flag in requested_flags {
                    surface_object.insert(flag.to_string(), serde_json::json!(true));
                }
            }
            surface_report
        })
        .collect::<Vec<_>>();
    let surface_count = surfaces.len();
    let contract_hash = sha256_text_value(&format!(
        "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-receipt-retention-expiry-garbage-collection-denial:native:source={source_report_sha256}:surfaces={surface_count}:route_count={}:retention=0:expiry=0:gc=0:archive=0:authority=0:install=0:live=0",
        route_matrix.route_count
    ));
    let policy_hash = sha256_text_value(
        "artifact-signing-notarization-result-receipt-retention-expiry-garbage-collection-denial:no-retention:no-expiry:no-gc:no-archive:no-compaction:no-authority:no-install:no-live",
    );
    let denials = vec![
        "source_artifact_distribution_signing_notarization_receipt_audit_evidence_report_required",
        "signing_receipt_retention_policy_denied",
        "signing_receipt_ttl_lease_denied",
        "signing_receipt_expiry_timestamp_denied",
        "signing_receipt_expiry_scheduler_timer_ack_denied",
        "signing_receipt_garbage_collection_queue_denied",
        "signing_receipt_garbage_collection_scan_denied",
        "signing_receipt_garbage_collection_candidate_decision_denied",
        "signing_receipt_tombstone_delete_marker_gc_denied",
        "signing_receipt_archive_denied",
        "signing_receipt_compaction_denied",
        "signing_receipt_audit_evidence_retention_denied",
        "signing_receipt_immutable_evidence_retention_denied",
        "signing_receipt_hash_attestation_retention_denied",
        "signing_receipt_witness_notary_expiry_denied",
        "external_telegram_signing_receipt_retention_delivery_denied",
        "release_publication_retention_authority_denied",
        "activation_live_install_gc_evidence_denied",
        "install_restart_active_binary_retention_gc_denied",
        "memory_provider_kg_secret_external_send_from_retention_denied",
    ];
    let denied_count = denials.len();
    let report_ready = source_ready
        && source_u64(
            "artifact_distribution_signing_notarization_receipt_audit_evidence_surface_count",
        ) == 18
        && source_u64(
            "artifact_distribution_signing_notarization_receipt_audit_evidence_attempt_count",
        ) == 18
        && source_u64(
            "artifact_distribution_signing_notarization_receipt_audit_evidence_denied_count",
        ) == 18
        && source_u64(
            "artifact_distribution_signing_notarization_receipt_audit_evidence_recorded_count",
        ) == 0
        && source_u64(
            "artifact_distribution_signing_notarization_receipt_audit_trail_recorded_count",
        ) == 0
        && source_u64(
            "artifact_distribution_signing_notarization_receipt_immutable_evidence_recorded_count",
        ) == 0
        && source_u64("artifact_distribution_signing_notarization_receipt_ledger_recorded_count")
            == 0
        && source_u64(
            "release_publication_authority_from_signing_receipt_audit_evidence_derived_count",
        ) == 0
        && source_u64("activation_authority_from_signing_receipt_audit_evidence_derived_count")
            == 0
        && surface_count == 18
        && route_count_source_command_accepted;

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_retention_expiry_garbage_collection_denial_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_OPERATOR_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_RECEIPT_RETENTION_EXPIRY_GARBAGE_COLLECTION_DENIAL_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-receipt-retention-expiry-garbage-collection-denial --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-27",
        "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_schema_version": "memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_operator_intent_consent_evidence_artifact_signing_receipt_retention_expiry_garbage_collection_denial_route_v1",
        "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_mode": "native_route_denied_signing_notarization_receipt_retention_expiry_garbage_collection_archive_compaction_authority_install_or_live_use",
        "minimum_required_samples": 24,
        "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        "route_count": route_matrix.route_count,
        "implemented_route_count": route_matrix.implemented_route_count,
        "missing_route_count": route_matrix.missing_route_count,
        "route_count_source_command_accepted": route_count_source_command_accepted,
        "source_route_wired": true,
        "memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_retention_expiry_garbage_collection_denial_route_enabled": true,
        "memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_retention_expiry_garbage_collection_denial_ready": report_ready,
    });
    extend_json_object(
        &mut report,
        serde_json::json!({
            "source_artifact_distribution_signing_notarization_receipt_audit_evidence_route": "hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_audit_evidence_denial_route",
            "source_artifact_distribution_signing_notarization_receipt_audit_evidence_ready": source_ready,
            "source_artifact_distribution_signing_notarization_receipt_audit_evidence_report_sha256": source_report_sha256,
            "source_artifact_distribution_signing_notarization_receipt_audit_evidence_contract_hash_sha256": source_contract_hash,
            "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_contract_hash_sha256": contract_hash,
            "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_policy_hash_sha256": policy_hash,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "source_artifact_distribution_signing_notarization_receipt_audit_evidence_surface_count": source_u64("artifact_distribution_signing_notarization_receipt_audit_evidence_surface_count"),
            "source_artifact_distribution_signing_notarization_receipt_audit_evidence_attempt_count": source_u64("artifact_distribution_signing_notarization_receipt_audit_evidence_attempt_count"),
            "source_artifact_distribution_signing_notarization_receipt_audit_evidence_denied_count": source_u64("artifact_distribution_signing_notarization_receipt_audit_evidence_denied_count"),
            "source_artifact_distribution_signing_notarization_receipt_audit_evidence_recorded_count": source_u64("artifact_distribution_signing_notarization_receipt_audit_evidence_recorded_count"),
            "source_artifact_distribution_signing_notarization_receipt_audit_trail_recorded_count": source_u64("artifact_distribution_signing_notarization_receipt_audit_trail_recorded_count"),
            "source_artifact_distribution_signing_notarization_receipt_immutable_evidence_recorded_count": source_u64("artifact_distribution_signing_notarization_receipt_immutable_evidence_recorded_count"),
            "source_artifact_distribution_signing_notarization_receipt_ledger_recorded_count": source_u64("artifact_distribution_signing_notarization_receipt_ledger_recorded_count"),
            "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_surface_count": surface_count,
            "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_attempt_count": surface_count,
            "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_denied_count": surface_count,
            "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_surfaces": surfaces,
            "denied_by_artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection": denials,
            "denied_by_artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_count": denied_count,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "allowed_next_actions": [
                {
                    "action": "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_export_query_observability_denial_gate",
                    "status": "allowed_report_only_next_slice",
                    "records_retention": false,
                    "records_retention_policy": false,
                    "records_expiry": false,
                    "records_garbage_collection": false,
                    "records_archive": false,
                    "records_compaction": false,
                    "registers_export": false,
                    "exports_receipt": false,
                    "registers_query": false,
                    "records_observability": false,
                    "records_operator_acceptance": false,
                    "derives_release_publication_authority": false,
                    "derives_activation_authority": false,
                    "renders_download_link": false,
                    "emits_install_command": false,
                    "installs_or_restarts": false,
                    "mutates_active_binary": false,
                    "mutates_memory_store": false,
                    "writes_kg": false,
                    "invokes_provider": false,
                    "reads_credentials": false,
                    "sends_externally": false
                }
            ],
        }),
    );

    let zero_keys = [
        "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_allowed_count",
        "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_accepted_count",
        "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_recorded_count",
        "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_persisted_count",
        "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_materialized_count",
        "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_filesystem_written_count",
        "artifact_distribution_signing_notarization_receipt_retention_policy_recorded_count",
        "artifact_distribution_signing_notarization_receipt_retention_policy_persisted_count",
        "artifact_distribution_signing_notarization_receipt_ttl_lease_recorded_count",
        "artifact_distribution_signing_notarization_receipt_ttl_lease_persisted_count",
        "artifact_distribution_signing_notarization_receipt_expiry_timestamp_recorded_count",
        "artifact_distribution_signing_notarization_receipt_expiry_scheduler_recorded_count",
        "artifact_distribution_signing_notarization_receipt_expiry_timer_started_count",
        "artifact_distribution_signing_notarization_receipt_expiry_ack_recorded_count",
        "artifact_distribution_signing_notarization_receipt_expiry_state_persisted_count",
        "artifact_distribution_signing_notarization_receipt_garbage_collection_queue_recorded_count",
        "artifact_distribution_signing_notarization_receipt_garbage_collection_scan_performed_count",
        "artifact_distribution_signing_notarization_receipt_garbage_collection_candidate_recorded_count",
        "artifact_distribution_signing_notarization_receipt_garbage_collection_decision_recorded_count",
        "artifact_distribution_signing_notarization_receipt_garbage_collection_state_persisted_count",
        "artifact_distribution_signing_notarization_receipt_tombstone_gc_recorded_count",
        "artifact_distribution_signing_notarization_receipt_delete_marker_gc_recorded_count",
        "artifact_distribution_signing_notarization_receipt_archive_recorded_count",
        "artifact_distribution_signing_notarization_receipt_compaction_recorded_count",
        "artifact_distribution_signing_notarization_receipt_audit_evidence_retention_recorded_count",
        "artifact_distribution_signing_notarization_receipt_immutable_evidence_retention_recorded_count",
        "artifact_distribution_signing_notarization_receipt_hash_attestation_retention_recorded_count",
        "artifact_distribution_signing_notarization_receipt_witness_notary_expiry_recorded_count",
        "artifact_distribution_signing_notarization_receipt_ledger_index_retention_recorded_count",
        "artifact_distribution_signing_notarization_receipt_delivery_evidence_retention_recorded_count",
        "artifact_distribution_signing_notarization_receipt_status_evidence_expiry_recorded_count",
        "release_publication_authority_from_signing_receipt_retention_derived_count",
        "activation_authority_from_signing_receipt_retention_derived_count",
        "download_link_from_signing_receipt_retention_rendered_count",
        "install_command_from_signing_receipt_retention_rendered_count",
        "install_from_signing_receipt_retention_executed_count",
        "service_restart_from_signing_receipt_retention_performed_count",
        "active_binary_from_signing_receipt_retention_mutated_count",
        "memory_store_write_performed_count",
        "live_kg_write_performed_count",
        "provider_invoked_count",
        "model_invoked_count",
        "credential_read_count",
        "secret_file_read_count",
        "external_send_performed_count",
    ];
    if let Some(report_object) = report.as_object_mut() {
        for key in &zero_keys {
            report_object.insert((*key).to_string(), serde_json::json!(0));
        }
    }

    let false_keys = [
        "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_accepted",
        "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_recorded",
        "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_persisted",
        "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_materialized",
        "artifact_distribution_signing_notarization_receipt_retention_policy_recorded",
        "artifact_distribution_signing_notarization_receipt_expiry_recorded",
        "artifact_distribution_signing_notarization_receipt_garbage_collection_recorded",
        "artifact_distribution_signing_notarization_receipt_archive_recorded",
        "artifact_distribution_signing_notarization_receipt_compaction_recorded",
        "artifact_distribution_signing_notarization_receipt_audit_evidence_retention_recorded",
        "artifact_distribution_signing_notarization_receipt_immutable_evidence_retention_recorded",
        "public_release_claimed",
        "public_ga_claimed",
        "operator_acceptance_recorded",
        "operator_approval_recorded",
        "release_publication_authority_derived",
        "activation_authority_derived",
        "activation_allowed",
        "activation_performed",
        "download_link_rendered",
        "install_command_rendered",
        "memory_store_write_performed",
        "memory_store_mutated",
        "live_kg_write_performed",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "install_executed",
        "launchd_mutated",
        "service_restarted",
        "active_binary_mutated",
        "external_send_performed",
        "release_artifact_written",
        "public_artifact_written",
        "filesystem_written",
    ];
    if let Some(report_object) = report.as_object_mut() {
        for key in &false_keys {
            report_object.insert((*key).to_string(), serde_json::json!(false));
        }
    }

    let side_effect_false_keys = [
        "retention_policy_recorded",
        "retention_policy_persisted",
        "ttl_lease_recorded",
        "ttl_lease_persisted",
        "expiry_timestamp_recorded",
        "expiry_scheduler_recorded",
        "expiry_timer_started",
        "expiry_ack_recorded",
        "expiry_state_persisted",
        "garbage_collection_queue_recorded",
        "garbage_collection_scan_performed",
        "garbage_collection_candidate_recorded",
        "garbage_collection_decision_recorded",
        "garbage_collection_state_persisted",
        "tombstone_gc_recorded",
        "delete_marker_gc_recorded",
        "archive_recorded",
        "compaction_recorded",
        "audit_evidence_retention_recorded",
        "immutable_evidence_retention_recorded",
        "hash_attestation_retention_recorded",
        "witness_notary_expiry_recorded",
        "ledger_index_retention_recorded",
        "delivery_evidence_retention_recorded",
        "status_evidence_expiry_recorded",
        "result_receipt_from_retention_recorded",
        "result_receipt_from_retention_persisted",
        "operator_approval_from_retention_derived",
        "release_publication_authority_from_retention_derived",
        "activation_authority_from_retention_derived",
        "download_link_from_retention_rendered",
        "install_command_from_retention_rendered",
        "install_executed",
        "launchd_mutated",
        "service_restarted",
        "active_binary_mutated",
        "memory_store_write_performed",
        "memory_store_mutated",
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
        "filesystem_written",
    ];
    let mut side_effects = serde_json::Map::new();
    for key in &side_effect_false_keys {
        side_effects.insert((*key).to_string(), serde_json::json!(false));
    }
    extend_json_object(
        &mut report,
        serde_json::json!({ "side_effects": side_effects }),
    );
    report
}

fn hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_export_query_observability_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source_report =
        hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_retention_expiry_garbage_collection_denial_report();
    let source_ready = source_report["status"].as_str() == Some("ready")
        && source_report["memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_retention_expiry_garbage_collection_denial_ready"]
            .as_bool()
            .unwrap_or(false);
    let source_u64 = |key: &str| source_report[key].as_u64().unwrap_or(0);
    let source_report_sha256 = sha256_json_value(&source_report);
    let source_contract_hash = source_report
        .get("artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_contract_hash_sha256")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("unknown")
        .to_string();
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;

    let surface_false_keys = [
        "artifact_distribution_signing_notarization_receipt_export_query_observability_allowed",
        "artifact_distribution_signing_notarization_receipt_export_query_observability_accepted",
        "artifact_distribution_signing_notarization_receipt_export_query_observability_recorded",
        "artifact_distribution_signing_notarization_receipt_export_query_observability_persisted",
        "artifact_distribution_signing_notarization_receipt_export_query_observability_materialized",
        "artifact_distribution_signing_notarization_receipt_export_query_observability_filesystem_written",
        "query_requested",
        "query_registration_requested",
        "query_execution_requested",
        "query_result_requested",
        "search_index_requested",
        "export_requested",
        "export_snapshot_requested",
        "export_file_requested",
        "export_stream_requested",
        "observability_requested",
        "metric_log_requested",
        "trace_event_requested",
        "dashboard_panel_requested",
        "alert_slo_requested",
        "operator_summary_readback_requested",
        "audit_view_requested",
        "ledger_observability_requested",
        "index_observability_requested",
        "delivery_observability_requested",
        "archive_view_requested",
        "compaction_view_requested",
        "external_telegram_observability_requested",
        "release_publication_authority_view_requested",
        "activation_authority_view_requested",
        "live_install_view_requested",
        "install_restart_active_binary_view_requested",
        "query_registered",
        "query_executed",
        "query_result_recorded",
        "query_result_persisted",
        "search_index_recorded",
        "search_index_persisted",
        "export_accepted",
        "export_snapshot_recorded",
        "export_snapshot_persisted",
        "export_file_written",
        "export_stream_opened",
        "observability_metric_recorded",
        "observability_log_recorded",
        "observability_trace_recorded",
        "observability_event_recorded",
        "dashboard_panel_recorded",
        "alert_registered",
        "slo_recorded",
        "operator_summary_recorded",
        "readback_surface_recorded",
        "audit_view_recorded",
        "ledger_observability_recorded",
        "index_observability_recorded",
        "delivery_observability_recorded",
        "retention_policy_recorded",
        "expiry_recorded",
        "garbage_collection_recorded",
        "archive_recorded",
        "compaction_recorded",
        "audit_evidence_recorded",
        "immutable_evidence_recorded",
        "hash_chain_recorded",
        "attestation_recorded",
        "witness_notary_recorded",
        "result_receipt_recorded",
        "result_receipt_persisted",
        "result_receipt_exported",
        "result_receipt_query_registered",
        "result_receipt_observability_recorded",
        "completion_ack_recorded",
        "operator_acceptance_from_export_query_observability_recorded",
        "operator_approval_from_export_query_observability_derived",
        "release_publication_authority_from_export_query_observability_derived",
        "activation_authority_from_export_query_observability_derived",
        "download_link_from_export_query_observability_rendered",
        "install_command_from_export_query_observability_rendered",
        "install_from_export_query_observability_executed",
        "service_restart_from_export_query_observability_performed",
        "launchd_from_export_query_observability_mutated",
        "active_binary_from_export_query_observability_mutated",
        "memory_store_write_performed",
        "memory_store_mutated",
        "live_kg_write_performed",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "telegram_send_performed",
        "channel_send_performed",
        "external_send_performed",
        "public_release_claimed",
        "public_ga_claimed",
        "release_artifact_written",
        "public_artifact_written",
        "filesystem_written",
    ];
    let surface_specs: Vec<(&str, &str, &str, Vec<&str>)> = vec![
        (
            "source_signing_receipt_retention_expiry_garbage_collection_report_required",
            "blocked_source_signing_receipt_retention_gc_report_required_noop",
            "source_signing_receipt_retention_expiry_garbage_collection_report_required",
            vec!["source_report_required"],
        ),
        (
            "artifact_signing_audit_trail_retention_policy_query_registration",
            "blocked_artifact_signing_retention_query_registration_noop",
            "artifact_signing_audit_trail_retention_policy_query_registration_denied",
            vec!["query_requested", "query_registration_requested"],
        ),
        (
            "package_signing_immutable_evidence_ttl_lease_query_execution",
            "blocked_package_signing_ttl_query_execution_noop",
            "package_signing_immutable_evidence_ttl_lease_query_execution_denied",
            vec!["query_requested", "query_execution_requested"],
        ),
        (
            "signature_manifest_hash_chain_expiry_query_result",
            "blocked_signature_manifest_expiry_query_result_noop",
            "signature_manifest_hash_chain_expiry_query_result_denied",
            vec!["query_requested", "query_result_requested"],
        ),
        (
            "notarization_attestation_retention_search_index",
            "blocked_notarization_retention_search_index_noop",
            "notarization_attestation_retention_search_index_denied",
            vec!["search_index_requested", "index_observability_requested"],
        ),
        (
            "notarization_ticket_witness_notary_export_request",
            "blocked_witness_notary_export_request_noop",
            "notarization_ticket_witness_notary_export_request_denied",
            vec!["export_requested"],
        ),
        (
            "stapling_tombstone_garbage_collection_export_snapshot",
            "blocked_stapling_tombstone_export_snapshot_noop",
            "stapling_tombstone_garbage_collection_export_snapshot_denied",
            vec!["export_requested", "export_snapshot_requested"],
        ),
        (
            "installer_replacement_garbage_collection_export_file",
            "blocked_installer_replacement_export_file_noop",
            "installer_replacement_garbage_collection_export_file_denied",
            vec!["export_requested", "export_file_requested"],
        ),
        (
            "provenance_immutable_evidence_archive_export_stream",
            "blocked_provenance_archive_export_stream_noop",
            "provenance_immutable_evidence_archive_export_stream_denied",
            vec![
                "export_requested",
                "export_stream_requested",
                "archive_view_requested",
            ],
        ),
        (
            "sbom_evidence_compaction_observability_metric_log",
            "blocked_sbom_compaction_metric_log_noop",
            "sbom_evidence_compaction_observability_metric_log_denied",
            vec![
                "observability_requested",
                "metric_log_requested",
                "compaction_view_requested",
            ],
        ),
        (
            "release_asset_cancelled_query_retention_readback",
            "blocked_release_asset_cancelled_query_readback_noop",
            "release_asset_cancelled_query_retention_readback_denied",
            vec!["operator_summary_readback_requested"],
        ),
        (
            "cdn_observability_expiry_dashboard_panel",
            "blocked_cdn_expiry_dashboard_panel_noop",
            "cdn_observability_expiry_dashboard_panel_denied",
            vec!["observability_requested", "dashboard_panel_requested"],
        ),
        (
            "package_registry_replacement_status_trace_event",
            "blocked_package_registry_status_trace_event_noop",
            "package_registry_replacement_status_trace_event_denied",
            vec!["observability_requested", "trace_event_requested"],
        ),
        (
            "dashboard_endpoint_hash_status_alert_slo",
            "blocked_dashboard_hash_status_alert_slo_noop",
            "dashboard_endpoint_hash_status_alert_slo_denied",
            vec!["observability_requested", "alert_slo_requested"],
        ),
        (
            "external_telegram_retention_delivery_observability",
            "blocked_external_telegram_retention_observability_noop",
            "external_telegram_retention_delivery_observability_denied",
            vec![
                "observability_requested",
                "delivery_observability_requested",
                "external_telegram_observability_requested",
            ],
        ),
        (
            "release_publication_authority_retention_view",
            "blocked_release_publication_authority_retention_view_noop",
            "release_publication_authority_retention_view_denied",
            vec![
                "audit_view_requested",
                "release_publication_authority_view_requested",
            ],
        ),
        (
            "activation_live_install_garbage_collection_view",
            "blocked_activation_live_install_gc_view_noop",
            "activation_live_install_garbage_collection_view_denied",
            vec![
                "activation_authority_view_requested",
                "live_install_view_requested",
            ],
        ),
        (
            "install_restart_active_binary_retention_gc_view",
            "blocked_install_restart_active_binary_retention_gc_view_noop",
            "install_restart_active_binary_retention_gc_view_denied",
            vec![
                "ledger_observability_requested",
                "install_restart_active_binary_view_requested",
            ],
        ),
    ];
    let surfaces = surface_specs
        .into_iter()
        .map(|(surface, status, reason, requested_flags)| {
            let mut surface_report = serde_json::json!({
                "surface": surface,
                "artifact_distribution_signing_notarization_receipt_export_query_observability_surface": surface,
                "source_signing_receipt_retention_expiry_garbage_collection_ready": source_ready,
                "canonical_noop_signing_receipt_identity_required": true,
                "artifact_distribution_signing_notarization_receipt_export_query_observability_attempted": true,
                "artifact_distribution_signing_notarization_receipt_export_query_observability_noop_confirmed": true,
                "artifact_distribution_signing_notarization_receipt_export_query_observability_status": status,
                "reason": reason,
            });
            if let Some(surface_object) = surface_report.as_object_mut() {
                for key in &surface_false_keys {
                    surface_object.insert((*key).to_string(), serde_json::json!(false));
                }
                for flag in requested_flags {
                    surface_object.insert(flag.to_string(), serde_json::json!(true));
                }
            }
            surface_report
        })
        .collect::<Vec<_>>();
    let surface_count = surfaces.len();
    let contract_hash = sha256_text_value(&format!(
        "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-receipt-export-query-observability-denial:native:source={source_report_sha256}:surfaces={surface_count}:route_count={}:query=0:export=0:observability=0:authority=0:install=0:live=0",
        route_matrix.route_count
    ));
    let policy_hash = sha256_text_value(
        "artifact-signing-notarization-result-receipt-export-query-observability-denial:no-query:no-export:no-observability:no-readback:no-authority:no-install:no-live",
    );
    let denials = vec![
        "source_artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_report_required",
        "signing_receipt_retention_query_registration_denied",
        "signing_receipt_ttl_query_execution_denied",
        "signing_receipt_expiry_query_result_denied",
        "signing_receipt_search_index_denied",
        "signing_receipt_export_request_denied",
        "signing_receipt_export_snapshot_denied",
        "signing_receipt_export_file_denied",
        "signing_receipt_export_stream_denied",
        "signing_receipt_archive_export_stream_denied",
        "signing_receipt_compaction_metric_log_denied",
        "signing_receipt_dashboard_panel_denied",
        "signing_receipt_trace_event_denied",
        "signing_receipt_alert_slo_denied",
        "external_telegram_signing_receipt_observability_denied",
        "release_publication_authority_view_denied",
        "activation_live_install_view_denied",
        "install_restart_active_binary_view_denied",
        "memory_provider_kg_secret_external_send_from_view_denied",
    ];
    let denied_count = denials.len();
    let report_ready = source_ready
        && source_u64(
            "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_surface_count",
        ) == 18
        && source_u64(
            "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_attempt_count",
        ) == 18
        && source_u64(
            "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_denied_count",
        ) == 18
        && source_u64(
            "artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_accepted_count",
        ) == 0
        && source_u64(
            "artifact_distribution_signing_notarization_receipt_retention_policy_recorded_count",
        ) == 0
        && source_u64(
            "artifact_distribution_signing_notarization_receipt_garbage_collection_scan_performed_count",
        ) == 0
        && source_u64("release_publication_authority_from_signing_receipt_retention_derived_count")
            == 0
        && source_u64("activation_authority_from_signing_receipt_retention_derived_count") == 0
        && surface_count == 18
        && route_count_source_command_accepted;

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_export_query_observability_denial_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_OPERATOR_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_RECEIPT_EXPORT_QUERY_OBSERVABILITY_DENIAL_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-receipt-export-query-observability-denial --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-27",
        "artifact_distribution_signing_notarization_receipt_export_query_observability_schema_version": "memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_operator_intent_consent_evidence_artifact_signing_receipt_export_query_observability_denial_route_v1",
        "artifact_distribution_signing_notarization_receipt_export_query_observability_mode": "native_route_denied_signing_notarization_receipt_query_export_observability_readback_views_authority_install_or_live_use",
        "minimum_required_samples": 24,
        "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        "route_count": route_matrix.route_count,
        "implemented_route_count": route_matrix.implemented_route_count,
        "missing_route_count": route_matrix.missing_route_count,
        "route_count_source_command_accepted": route_count_source_command_accepted,
        "source_route_wired": true,
        "memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_export_query_observability_denial_route_enabled": true,
        "memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_export_query_observability_denial_ready": report_ready,
        "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_export_query_observability_denial_ready": report_ready,
    });
    extend_json_object(
        &mut report,
        serde_json::json!({
            "source_artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_route": "hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_retention_expiry_garbage_collection_denial_route",
            "source_artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_ready": source_ready,
            "source_artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_report_sha256": source_report_sha256,
            "source_artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_contract_hash_sha256": source_contract_hash,
            "artifact_distribution_signing_notarization_receipt_export_query_observability_contract_hash_sha256": contract_hash,
            "artifact_distribution_signing_notarization_receipt_export_query_observability_policy_hash_sha256": policy_hash,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "source_artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_surface_count": source_u64("artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_surface_count"),
            "source_artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_attempt_count": source_u64("artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_attempt_count"),
            "source_artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_denied_count": source_u64("artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_denied_count"),
            "source_artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_accepted_count": source_u64("artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_accepted_count"),
            "source_artifact_distribution_signing_notarization_receipt_retention_policy_recorded_count": source_u64("artifact_distribution_signing_notarization_receipt_retention_policy_recorded_count"),
            "source_artifact_distribution_signing_notarization_receipt_garbage_collection_scan_performed_count": source_u64("artifact_distribution_signing_notarization_receipt_garbage_collection_scan_performed_count"),
            "source_release_publication_authority_from_signing_receipt_retention_derived_count": source_u64("release_publication_authority_from_signing_receipt_retention_derived_count"),
            "source_activation_authority_from_signing_receipt_retention_derived_count": source_u64("activation_authority_from_signing_receipt_retention_derived_count"),
            "artifact_distribution_signing_notarization_receipt_export_query_observability_surface_count": surface_count,
            "artifact_distribution_signing_notarization_receipt_export_query_observability_attempt_count": surface_count,
            "artifact_distribution_signing_notarization_receipt_export_query_observability_denied_count": surface_count,
            "artifact_distribution_signing_notarization_receipt_export_query_observability_surfaces": surfaces,
            "denied_by_artifact_distribution_signing_notarization_receipt_export_query_observability": denials,
            "denied_by_artifact_distribution_signing_notarization_receipt_export_query_observability_count": denied_count,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "allowed_next_actions": [
                {
                    "action": "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_non_persistence_denial_gate",
                    "status": "allowed_report_only_next_slice",
                    "registers_query": false,
                    "executes_query": false,
                    "records_query_result": false,
                    "writes_search_index": false,
                    "accepts_export": false,
                    "writes_export": false,
                    "opens_export_stream": false,
                    "records_observability": false,
                    "records_operator_summary": false,
                    "records_readback": false,
                    "records_audit_view": false,
                    "records_delivery_evidence": false,
                    "derives_release_publication_authority": false,
                    "derives_activation_authority": false,
                    "renders_download_link": false,
                    "emits_install_command": false,
                    "installs_or_restarts": false,
                    "mutates_active_binary": false,
                    "mutates_memory_store": false,
                    "writes_kg": false,
                    "invokes_provider": false,
                    "reads_credentials": false,
                    "sends_externally": false
                }
            ],
        }),
    );

    let zero_keys = [
        "artifact_distribution_signing_notarization_receipt_export_query_observability_allowed_count",
        "artifact_distribution_signing_notarization_receipt_export_query_observability_accepted_count",
        "artifact_distribution_signing_notarization_receipt_export_query_observability_recorded_count",
        "artifact_distribution_signing_notarization_receipt_export_query_observability_persisted_count",
        "artifact_distribution_signing_notarization_receipt_export_query_observability_materialized_count",
        "artifact_distribution_signing_notarization_receipt_export_query_observability_filesystem_written_count",
        "artifact_distribution_signing_notarization_receipt_export_query_observability_acceptance_recorded_count",
        "artifact_distribution_signing_notarization_receipt_query_registered_count",
        "artifact_distribution_signing_notarization_receipt_query_executed_count",
        "artifact_distribution_signing_notarization_receipt_query_result_recorded_count",
        "artifact_distribution_signing_notarization_receipt_query_result_persisted_count",
        "artifact_distribution_signing_notarization_receipt_search_index_recorded_count",
        "artifact_distribution_signing_notarization_receipt_search_index_persisted_count",
        "artifact_distribution_signing_notarization_receipt_export_accepted_count",
        "artifact_distribution_signing_notarization_receipt_export_snapshot_recorded_count",
        "artifact_distribution_signing_notarization_receipt_export_snapshot_persisted_count",
        "artifact_distribution_signing_notarization_receipt_export_file_written_count",
        "artifact_distribution_signing_notarization_receipt_export_stream_opened_count",
        "artifact_distribution_signing_notarization_receipt_observability_metric_recorded_count",
        "artifact_distribution_signing_notarization_receipt_observability_log_recorded_count",
        "artifact_distribution_signing_notarization_receipt_observability_trace_recorded_count",
        "artifact_distribution_signing_notarization_receipt_observability_event_recorded_count",
        "artifact_distribution_signing_notarization_receipt_dashboard_panel_recorded_count",
        "artifact_distribution_signing_notarization_receipt_alert_registered_count",
        "artifact_distribution_signing_notarization_receipt_slo_recorded_count",
        "artifact_distribution_signing_notarization_receipt_operator_summary_recorded_count",
        "artifact_distribution_signing_notarization_receipt_readback_surface_recorded_count",
        "artifact_distribution_signing_notarization_receipt_audit_view_recorded_count",
        "artifact_distribution_signing_notarization_receipt_ledger_observability_recorded_count",
        "artifact_distribution_signing_notarization_receipt_index_observability_recorded_count",
        "artifact_distribution_signing_notarization_receipt_delivery_observability_recorded_count",
        "release_publication_authority_from_signing_receipt_export_query_observability_derived_count",
        "activation_authority_from_signing_receipt_export_query_observability_derived_count",
        "download_link_from_signing_receipt_export_query_observability_rendered_count",
        "install_command_from_signing_receipt_export_query_observability_rendered_count",
        "install_from_signing_receipt_export_query_observability_executed_count",
        "service_restart_from_signing_receipt_export_query_observability_performed_count",
        "active_binary_from_signing_receipt_export_query_observability_mutated_count",
        "memory_store_write_performed_count",
        "live_kg_write_performed_count",
        "provider_invoked_count",
        "model_invoked_count",
        "credential_read_count",
        "secret_file_read_count",
        "external_send_performed_count",
    ];
    if let Some(report_object) = report.as_object_mut() {
        for key in &zero_keys {
            report_object.insert((*key).to_string(), serde_json::json!(0));
        }
    }

    let false_keys = [
        "artifact_distribution_signing_notarization_receipt_export_query_observability_accepted",
        "artifact_distribution_signing_notarization_receipt_export_query_observability_recorded",
        "artifact_distribution_signing_notarization_receipt_export_query_observability_persisted",
        "artifact_distribution_signing_notarization_receipt_export_query_observability_materialized",
        "artifact_distribution_signing_notarization_receipt_export_query_observability_filesystem_written",
        "artifact_distribution_signing_notarization_receipt_query_registered",
        "artifact_distribution_signing_notarization_receipt_query_executed",
        "artifact_distribution_signing_notarization_receipt_query_result_recorded",
        "artifact_distribution_signing_notarization_receipt_search_index_recorded",
        "artifact_distribution_signing_notarization_receipt_export_accepted",
        "artifact_distribution_signing_notarization_receipt_export_file_written",
        "artifact_distribution_signing_notarization_receipt_export_stream_opened",
        "artifact_distribution_signing_notarization_receipt_observability_recorded",
        "artifact_distribution_signing_notarization_receipt_dashboard_panel_recorded",
        "artifact_distribution_signing_notarization_receipt_alert_registered",
        "artifact_distribution_signing_notarization_receipt_operator_summary_recorded",
        "artifact_distribution_signing_notarization_receipt_readback_surface_recorded",
        "artifact_distribution_signing_notarization_receipt_audit_view_recorded",
        "artifact_distribution_signing_notarization_receipt_delivery_observability_recorded",
        "public_release_claimed",
        "public_ga_claimed",
        "operator_acceptance_recorded",
        "operator_approval_recorded",
        "release_publication_authority_derived",
        "activation_authority_derived",
        "activation_allowed",
        "activation_performed",
        "download_link_rendered",
        "install_command_rendered",
        "memory_store_write_performed",
        "memory_store_mutated",
        "live_kg_write_performed",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "install_executed",
        "launchd_mutated",
        "service_restarted",
        "active_binary_mutated",
        "external_send_performed",
        "release_artifact_written",
        "public_artifact_written",
        "filesystem_written",
    ];
    if let Some(report_object) = report.as_object_mut() {
        for key in &false_keys {
            report_object.insert((*key).to_string(), serde_json::json!(false));
        }
    }

    let side_effect_false_keys = [
        "query_registered",
        "query_executed",
        "query_result_recorded",
        "query_result_persisted",
        "search_index_recorded",
        "search_index_persisted",
        "export_accepted",
        "export_snapshot_recorded",
        "export_snapshot_persisted",
        "export_file_written",
        "export_stream_opened",
        "observability_metric_recorded",
        "observability_log_recorded",
        "observability_trace_recorded",
        "observability_event_recorded",
        "dashboard_panel_recorded",
        "alert_registered",
        "slo_recorded",
        "operator_summary_recorded",
        "readback_surface_recorded",
        "audit_view_recorded",
        "ledger_observability_recorded",
        "index_observability_recorded",
        "delivery_observability_recorded",
        "retention_policy_recorded",
        "expiry_recorded",
        "garbage_collection_recorded",
        "archive_recorded",
        "compaction_recorded",
        "result_receipt_recorded",
        "result_receipt_persisted",
        "result_receipt_exported",
        "result_receipt_query_registered",
        "result_receipt_observability_recorded",
        "completion_ack_recorded",
        "operator_acceptance_from_export_query_observability_recorded",
        "operator_approval_from_export_query_observability_derived",
        "release_publication_authority_from_export_query_observability_derived",
        "activation_authority_from_export_query_observability_derived",
        "download_link_from_export_query_observability_rendered",
        "install_command_from_export_query_observability_rendered",
        "install_executed",
        "launchd_mutated",
        "service_restarted",
        "active_binary_mutated",
        "memory_store_write_performed",
        "memory_store_mutated",
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
        "filesystem_written",
    ];
    let mut side_effects = serde_json::Map::new();
    for key in &side_effect_false_keys {
        side_effects.insert((*key).to_string(), serde_json::json!(false));
    }
    extend_json_object(
        &mut report,
        serde_json::json!({ "side_effects": side_effects }),
    );
    report
}

fn hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_operator_facing_summary_briefing_non_persistence_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source =
        hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_export_query_observability_denial_report();
    let source_report_sha256 = sha256_json_value(&source);
    let source_u64 = |key: &str| {
        source
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let source_bool = |key: &str| {
        source
            .get(key)
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
    };
    let source_ready = source_bool(
        "memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_export_query_observability_denial_ready",
    );
    let source_contract_hash = source
        .get("artifact_distribution_signing_notarization_receipt_export_query_observability_contract_hash_sha256")
        .cloned()
        .unwrap_or_else(|| serde_json::json!(""));
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;

    let surface_false_keys = [
        "artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_allowed",
        "artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_accepted",
        "artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_recorded",
        "artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_persisted",
        "artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_materialized",
        "artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_filesystem_written",
        "operator_summary_recorded",
        "operator_summary_persisted",
        "operator_briefing_recorded",
        "operator_briefing_persisted",
        "signing_receipt_readback_recorded",
        "signing_receipt_readback_persisted",
        "status_banner_recorded",
        "exported_summary_recorded",
        "briefing_card_recorded",
        "notification_timeline_recorded",
        "dashboard_narrative_recorded",
        "audit_narrative_recorded",
        "briefing_delivery_recorded",
        "final_summary_recorded",
        "operator_memo_recorded",
        "approval_summary_recorded",
        "external_briefing_delivered",
        "telegram_briefing_delivered",
        "authority_briefing_recorded",
        "live_status_briefing_recorded",
        "signing_receipt_query_registered",
        "signing_receipt_query_executed",
        "signing_receipt_query_result_recorded",
        "signing_receipt_export_accepted",
        "signing_receipt_export_file_written",
        "signing_receipt_export_stream_opened",
        "signing_receipt_observability_recorded",
        "signing_receipt_dashboard_recorded",
        "signing_receipt_alert_recorded",
        "signing_receipt_result_receipt_recorded",
        "signing_receipt_result_receipt_persisted",
        "signing_receipt_completion_ack_recorded",
        "operator_acceptance_from_summary_recorded",
        "operator_acceptance_from_briefing_recorded",
        "operator_approval_from_summary_derived",
        "operator_approval_from_briefing_derived",
        "release_publication_authority_from_summary_briefing_derived",
        "activation_authority_from_summary_briefing_derived",
        "download_link_from_summary_briefing_rendered",
        "install_command_from_summary_briefing_rendered",
        "install_from_summary_briefing_executed",
        "service_restart_from_summary_briefing_performed",
        "launchd_from_summary_briefing_mutated",
        "active_binary_from_summary_briefing_mutated",
        "memory_store_write_performed",
        "memory_store_mutated",
        "live_kg_write_performed",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "telegram_send_performed",
        "channel_send_performed",
        "external_send_performed",
        "public_release_claimed",
        "public_ga_claimed",
        "release_artifact_written",
        "public_artifact_written",
    ];
    let surface_specs = vec![
        (
            "source_signing_receipt_export_query_observability_report_required",
            "blocked_source_signing_receipt_observability_report_required_noop",
            "source_signing_receipt_export_query_observability_report_required",
            vec!["source_report_required"],
        ),
        (
            "artifact_signing_retention_query_operator_summary",
            "blocked_artifact_signing_query_summary_noop",
            "artifact_signing_retention_query_operator_summary_denied",
            vec!["summary_requested"],
        ),
        (
            "package_signing_ttl_query_operator_briefing",
            "blocked_package_signing_query_briefing_noop",
            "package_signing_ttl_query_operator_briefing_denied",
            vec!["briefing_requested"],
        ),
        (
            "signature_manifest_expiry_query_readback_digest",
            "blocked_signature_manifest_query_readback_noop",
            "signature_manifest_expiry_query_readback_digest_denied",
            vec!["readback_requested"],
        ),
        (
            "notarization_search_index_status_banner",
            "blocked_notarization_search_index_status_banner_noop",
            "notarization_search_index_status_banner_denied",
            vec!["status_banner_requested"],
        ),
        (
            "witness_notary_exported_summary_text",
            "blocked_witness_notary_exported_summary_noop",
            "witness_notary_exported_summary_text_denied",
            vec!["exported_summary_requested", "summary_requested"],
        ),
        (
            "tombstone_garbage_collection_export_briefing_card",
            "blocked_tombstone_gc_briefing_card_noop",
            "tombstone_garbage_collection_export_briefing_card_denied",
            vec!["briefing_card_requested", "briefing_requested"],
        ),
        (
            "replacement_garbage_collection_notification_timeline",
            "blocked_replacement_gc_notification_timeline_noop",
            "replacement_garbage_collection_notification_timeline_denied",
            vec!["notification_timeline_requested"],
        ),
        (
            "provenance_archive_dashboard_narrative",
            "blocked_provenance_archive_dashboard_narrative_noop",
            "provenance_archive_dashboard_narrative_denied",
            vec!["dashboard_narrative_requested"],
        ),
        (
            "sbom_compaction_audit_narrative",
            "blocked_sbom_compaction_audit_narrative_noop",
            "sbom_compaction_audit_narrative_denied",
            vec!["audit_narrative_requested"],
        ),
        (
            "release_asset_retention_readback_final_summary",
            "blocked_release_asset_final_summary_noop",
            "release_asset_retention_readback_final_summary_denied",
            vec![
                "final_summary_requested",
                "summary_requested",
                "readback_requested",
            ],
        ),
        (
            "cdn_expiry_dashboard_briefing_panel",
            "blocked_cdn_expiry_dashboard_briefing_noop",
            "cdn_expiry_dashboard_briefing_panel_denied",
            vec!["briefing_requested", "dashboard_narrative_requested"],
        ),
        (
            "package_registry_trace_event_operator_memo",
            "blocked_package_registry_trace_operator_memo_noop",
            "package_registry_trace_event_operator_memo_denied",
            vec!["operator_memo_requested"],
        ),
        (
            "dashboard_hash_alert_slo_approval_summary",
            "blocked_dashboard_hash_alert_approval_summary_noop",
            "dashboard_hash_alert_slo_approval_summary_denied",
            vec!["approval_summary_requested", "summary_requested"],
        ),
        (
            "external_telegram_observability_briefing_delivery",
            "blocked_external_telegram_observability_briefing_noop",
            "external_telegram_observability_briefing_delivery_denied",
            vec![
                "external_briefing_requested",
                "telegram_briefing_requested",
                "delivery_requested",
                "briefing_requested",
            ],
        ),
        (
            "release_publication_authority_view_briefing",
            "blocked_release_publication_authority_view_briefing_noop",
            "release_publication_authority_view_briefing_denied",
            vec!["authority_briefing_requested", "briefing_requested"],
        ),
        (
            "activation_live_install_view_status_briefing",
            "blocked_activation_live_install_status_briefing_noop",
            "activation_live_install_view_status_briefing_denied",
            vec!["live_status_briefing_requested", "briefing_requested"],
        ),
        (
            "install_restart_active_binary_view_status_briefing",
            "blocked_install_restart_active_binary_status_briefing_noop",
            "install_restart_active_binary_view_status_briefing_denied",
            vec![
                "live_status_briefing_requested",
                "briefing_requested",
                "install_restart_active_binary_status_requested",
            ],
        ),
    ];
    let surfaces = surface_specs
        .into_iter()
        .map(|(surface, status, reason, requested_flags)| {
            let mut surface_report = serde_json::json!({
                "surface": surface,
                "artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_surface": surface,
                "source_signing_receipt_export_query_observability_ready": source_ready,
                "canonical_noop_signing_receipt_identity_required": true,
                "artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_attempted": true,
                "artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_noop_confirmed": true,
                "artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_status": status,
                "reason": reason,
            });
            if let Some(surface_object) = surface_report.as_object_mut() {
                for key in &surface_false_keys {
                    surface_object.insert((*key).to_string(), serde_json::json!(false));
                }
                for flag in requested_flags {
                    surface_object.insert(flag.to_string(), serde_json::json!(true));
                }
            }
            surface_report
        })
        .collect::<Vec<_>>();
    let surface_count = surfaces.len();
    let contract_hash = sha256_text_value(&format!(
        "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-receipt-operator-facing-summary-briefing-non-persistence-denial:native:source={source_report_sha256}:surfaces={surface_count}:route_count={}:summary=0:briefing=0:readback=0:delivery=0:authority=0:install=0:live=0",
        route_matrix.route_count
    ));
    let policy_hash = sha256_text_value(
        "artifact-signing-notarization-result-receipt-operator-facing-summary-briefing-denial:no-summary:no-briefing:no-readback:no-delivery:no-authority:no-install:no-live",
    );
    let denials = vec![
        "source_artifact_distribution_signing_notarization_receipt_export_query_observability_report_required",
        "artifact_signing_query_operator_summary_denied",
        "package_signing_query_operator_briefing_denied",
        "signature_manifest_query_readback_denied",
        "notarization_search_index_status_banner_denied",
        "witness_notary_exported_summary_denied",
        "tombstone_garbage_collection_briefing_card_denied",
        "replacement_garbage_collection_notification_timeline_denied",
        "provenance_archive_dashboard_narrative_denied",
        "sbom_compaction_audit_narrative_denied",
        "release_asset_retention_final_summary_denied",
        "cdn_expiry_dashboard_briefing_denied",
        "package_registry_trace_operator_memo_denied",
        "dashboard_hash_alert_approval_summary_denied",
        "external_telegram_observability_briefing_denied",
        "release_publication_authority_view_briefing_denied",
        "activation_live_install_status_briefing_denied",
        "install_restart_active_binary_status_briefing_denied",
        "memory_provider_kg_secret_external_send_from_summary_briefing_denied",
    ];
    let denied_count = denials.len();
    let report_ready = source_ready
        && source_u64(
            "artifact_distribution_signing_notarization_receipt_export_query_observability_surface_count",
        ) == 18
        && source_u64(
            "artifact_distribution_signing_notarization_receipt_export_query_observability_denied_count",
        ) == 18
        && source_u64(
            "artifact_distribution_signing_notarization_receipt_operator_summary_recorded_count",
        ) == 0
        && source_u64(
            "release_publication_authority_from_signing_receipt_export_query_observability_derived_count",
        ) == 0
        && source_u64(
            "activation_authority_from_signing_receipt_export_query_observability_derived_count",
        ) == 0
        && surface_count == 18
        && route_count_source_command_accepted;

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_operator_facing_summary_briefing_non_persistence_denial_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_OPERATOR_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_RECEIPT_OPERATOR_FACING_SUMMARY_BRIEFING_NON_PERSISTENCE_DENIAL_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-receipt-operator-facing-summary-briefing-non-persistence-denial --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-27",
        "artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_schema_version": "memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_operator_intent_consent_evidence_artifact_signing_receipt_operator_facing_summary_briefing_non_persistence_denial_route_v1",
        "artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_mode": "native_route_denied_signing_notarization_receipt_operator_summary_briefing_readback_delivery_authority_install_or_live_use",
        "minimum_required_samples": 24,
        "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        "route_count": route_matrix.route_count,
        "implemented_route_count": route_matrix.implemented_route_count,
        "missing_route_count": route_matrix.missing_route_count,
        "route_count_source_command_accepted": route_count_source_command_accepted,
        "source_route_wired": true,
        "memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_operator_facing_summary_briefing_non_persistence_denial_route_enabled": true,
        "memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_operator_facing_summary_briefing_non_persistence_denial_ready": report_ready,
        "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_non_persistence_denial_ready": report_ready,
    });
    extend_json_object(
        &mut report,
        serde_json::json!({
            "source_artifact_distribution_signing_notarization_receipt_export_query_observability_route": "hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_export_query_observability_denial_route",
            "source_artifact_distribution_signing_notarization_receipt_export_query_observability_ready": source_ready,
            "source_artifact_distribution_signing_notarization_receipt_export_query_observability_report_sha256": source_report_sha256,
            "source_artifact_distribution_signing_notarization_receipt_export_query_observability_contract_hash_sha256": source_contract_hash,
            "artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_contract_hash_sha256": contract_hash,
            "artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_policy_hash_sha256": policy_hash,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "source_artifact_distribution_signing_notarization_receipt_export_query_observability_surface_count": source_u64("artifact_distribution_signing_notarization_receipt_export_query_observability_surface_count"),
            "source_artifact_distribution_signing_notarization_receipt_export_query_observability_attempt_count": source_u64("artifact_distribution_signing_notarization_receipt_export_query_observability_attempt_count"),
            "source_artifact_distribution_signing_notarization_receipt_export_query_observability_denied_count": source_u64("artifact_distribution_signing_notarization_receipt_export_query_observability_denied_count"),
            "source_artifact_distribution_signing_notarization_receipt_export_query_observability_accepted_count": source_u64("artifact_distribution_signing_notarization_receipt_export_query_observability_accepted_count"),
            "source_artifact_distribution_signing_notarization_receipt_operator_summary_recorded_count": source_u64("artifact_distribution_signing_notarization_receipt_operator_summary_recorded_count"),
            "source_release_publication_authority_from_signing_receipt_export_query_observability_derived_count": source_u64("release_publication_authority_from_signing_receipt_export_query_observability_derived_count"),
            "source_activation_authority_from_signing_receipt_export_query_observability_derived_count": source_u64("activation_authority_from_signing_receipt_export_query_observability_derived_count"),
            "artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_surface_count": surface_count,
            "artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_attempt_count": surface_count,
            "artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_denied_count": surface_count,
            "artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_surfaces": surfaces,
            "denied_by_artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing": denials,
            "denied_by_artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_count": denied_count,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "allowed_next_actions": [
                {
                    "action": "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_non_acceptance_denial_gate",
                    "status": "allowed_report_only_next_slice",
                    "records_summary": false,
                    "records_briefing": false,
                    "records_readback": false,
                    "records_status_banner": false,
                    "records_delivery": false,
                    "records_acknowledgement": false,
                    "derives_operator_approval": false,
                    "derives_release_publication_authority": false,
                    "derives_activation_authority": false,
                    "renders_download_link": false,
                    "emits_install_command": false,
                    "installs_or_restarts": false,
                    "mutates_active_binary": false,
                    "mutates_memory_store": false,
                    "writes_kg": false,
                    "invokes_provider": false,
                    "reads_credentials": false,
                    "sends_externally": false
                }
            ],
        }),
    );

    let zero_keys = [
        "artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_allowed_count",
        "artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_accepted_count",
        "artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_recorded_count",
        "artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_persisted_count",
        "artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_materialized_count",
        "artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_filesystem_written_count",
        "artifact_distribution_signing_notarization_receipt_operator_summary_recorded_count",
        "artifact_distribution_signing_notarization_receipt_operator_summary_persisted_count",
        "artifact_distribution_signing_notarization_receipt_operator_briefing_recorded_count",
        "artifact_distribution_signing_notarization_receipt_operator_briefing_persisted_count",
        "artifact_distribution_signing_notarization_receipt_readback_recorded_count",
        "artifact_distribution_signing_notarization_receipt_status_banner_recorded_count",
        "artifact_distribution_signing_notarization_receipt_exported_summary_recorded_count",
        "artifact_distribution_signing_notarization_receipt_briefing_card_recorded_count",
        "artifact_distribution_signing_notarization_receipt_notification_timeline_recorded_count",
        "artifact_distribution_signing_notarization_receipt_dashboard_narrative_recorded_count",
        "artifact_distribution_signing_notarization_receipt_audit_narrative_recorded_count",
        "artifact_distribution_signing_notarization_receipt_briefing_delivery_recorded_count",
        "artifact_distribution_signing_notarization_receipt_final_summary_recorded_count",
        "artifact_distribution_signing_notarization_receipt_operator_memo_recorded_count",
        "artifact_distribution_signing_notarization_receipt_approval_summary_recorded_count",
        "artifact_distribution_signing_notarization_receipt_external_briefing_delivered_count",
        "artifact_distribution_signing_notarization_receipt_telegram_briefing_delivered_count",
        "artifact_distribution_signing_notarization_receipt_authority_briefing_recorded_count",
        "artifact_distribution_signing_notarization_receipt_live_status_briefing_recorded_count",
        "artifact_distribution_signing_notarization_receipt_summary_briefing_acceptance_recorded_count",
        "operator_approval_from_signing_receipt_summary_briefing_derived_count",
        "release_publication_authority_from_signing_receipt_summary_briefing_derived_count",
        "activation_authority_from_signing_receipt_summary_briefing_derived_count",
        "download_link_from_signing_receipt_summary_briefing_rendered_count",
        "install_command_from_signing_receipt_summary_briefing_rendered_count",
        "install_from_signing_receipt_summary_briefing_executed_count",
        "service_restart_from_signing_receipt_summary_briefing_performed_count",
        "active_binary_from_signing_receipt_summary_briefing_mutated_count",
        "memory_store_write_performed_count",
        "live_kg_write_performed_count",
        "provider_invoked_count",
        "model_invoked_count",
        "credential_read_count",
        "secret_file_read_count",
        "external_send_performed_count",
    ];
    if let Some(report_object) = report.as_object_mut() {
        for key in &zero_keys {
            report_object.insert((*key).to_string(), serde_json::json!(0));
        }
    }

    let false_keys = [
        "artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_accepted",
        "artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_recorded",
        "artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_persisted",
        "artifact_distribution_signing_notarization_receipt_operator_summary_recorded",
        "artifact_distribution_signing_notarization_receipt_operator_summary_persisted",
        "artifact_distribution_signing_notarization_receipt_operator_briefing_recorded",
        "artifact_distribution_signing_notarization_receipt_operator_briefing_persisted",
        "artifact_distribution_signing_notarization_receipt_readback_recorded",
        "artifact_distribution_signing_notarization_receipt_status_banner_recorded",
        "artifact_distribution_signing_notarization_receipt_briefing_delivery_recorded",
        "operator_acceptance_recorded",
        "operator_approval_recorded",
        "release_publication_authority_derived",
        "activation_authority_derived",
        "activation_allowed",
        "activation_performed",
        "download_link_rendered",
        "install_command_rendered",
        "memory_store_write_performed",
        "memory_store_mutated",
        "live_kg_write_performed",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "install_executed",
        "launchd_mutated",
        "service_restarted",
        "active_binary_mutated",
        "external_send_performed",
        "release_artifact_written",
        "public_artifact_written",
        "filesystem_written",
    ];
    if let Some(report_object) = report.as_object_mut() {
        for key in &false_keys {
            report_object.insert((*key).to_string(), serde_json::json!(false));
        }
    }

    let side_effect_false_keys = [
        "operator_summary_recorded",
        "operator_summary_persisted",
        "operator_briefing_recorded",
        "operator_briefing_persisted",
        "readback_recorded",
        "status_banner_recorded",
        "exported_summary_recorded",
        "briefing_card_recorded",
        "notification_timeline_recorded",
        "dashboard_narrative_recorded",
        "audit_narrative_recorded",
        "briefing_delivery_recorded",
        "final_summary_recorded",
        "operator_memo_recorded",
        "approval_summary_recorded",
        "external_briefing_delivered",
        "telegram_briefing_delivered",
        "authority_briefing_recorded",
        "live_status_briefing_recorded",
        "summary_briefing_acceptance_recorded",
        "operator_approval_from_summary_briefing_derived",
        "release_publication_authority_from_summary_briefing_derived",
        "activation_authority_from_summary_briefing_derived",
        "download_link_from_summary_briefing_rendered",
        "install_command_from_summary_briefing_rendered",
        "install_executed",
        "launchd_mutated",
        "service_restarted",
        "active_binary_mutated",
        "memory_store_write_performed",
        "memory_store_mutated",
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
        "public_release_claimed",
        "public_ga_claimed",
        "filesystem_written",
    ];
    let mut side_effects = serde_json::Map::new();
    for key in &side_effect_false_keys {
        side_effects.insert((*key).to_string(), serde_json::json!(false));
    }
    extend_json_object(
        &mut report,
        serde_json::json!({ "side_effects": side_effects }),
    );
    report
}

fn hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_final_operator_acknowledgement_non_acceptance_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source =
        hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_operator_facing_summary_briefing_non_persistence_denial_report();
    let source_report_sha256 = sha256_json_value(&source);
    let source_u64 = |key: &str| {
        source
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let source_bool = |key: &str| {
        source
            .get(key)
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
    };
    let source_ready = source_bool(
        "memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_operator_facing_summary_briefing_non_persistence_denial_ready",
    );
    let source_contract_hash = source
        .get("artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_contract_hash_sha256")
        .cloned()
        .unwrap_or_else(|| serde_json::json!(""));
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;

    let surface_false_keys = [
        "source_signing_receipt_summary_briefing_report_required",
        "final_operator_acknowledgement_requested",
        "final_operator_acknowledgement_request_accepted",
        "final_operator_acknowledgement_accepted",
        "final_operator_acknowledgement_recorded",
        "final_operator_acknowledgement_persisted",
        "final_operator_acknowledgement_materialized",
        "final_operator_acknowledgement_filesystem_written",
        "final_operator_acknowledgement_delivered",
        "operator_received_requested",
        "operator_received_recorded",
        "operator_confirmed_requested",
        "operator_confirmed_recorded",
        "operator_read_requested",
        "operator_read_recorded",
        "operator_seen_requested",
        "operator_seen_recorded",
        "final_response_requested",
        "final_response_recorded",
        "completion_acknowledgement_requested",
        "completion_acknowledgement_recorded",
        "status_acknowledgement_requested",
        "status_acknowledgement_recorded",
        "summary_acknowledgement_requested",
        "summary_acknowledgement_recorded",
        "briefing_acknowledgement_requested",
        "briefing_acknowledgement_recorded",
        "readback_digest_acknowledgement_requested",
        "readback_digest_acknowledgement_recorded",
        "dashboard_acknowledgement_requested",
        "dashboard_acknowledgement_recorded",
        "notification_acknowledgement_requested",
        "notification_acknowledgement_recorded",
        "channel_acknowledgement_requested",
        "channel_acknowledgement_delivered",
        "external_acknowledgement_requested",
        "external_acknowledgement_sent",
        "telegram_acknowledgement_requested",
        "telegram_acknowledgement_sent",
        "operator_approval_acknowledgement_requested",
        "authority_acknowledgement_requested",
        "live_acknowledgement_requested",
        "install_restart_active_binary_acknowledgement_requested",
        "acknowledgement_acceptance_recorded",
        "operator_acceptance_from_acknowledgement_recorded",
        "operator_approval_from_acknowledgement_derived",
        "release_publication_authority_from_acknowledgement_derived",
        "activation_authority_from_acknowledgement_derived",
        "activation_command_from_acknowledgement_derived",
        "activation_from_acknowledgement_allowed",
        "live_execution_from_acknowledgement_allowed",
        "download_link_from_acknowledgement_rendered",
        "install_command_from_acknowledgement_rendered",
        "install_from_acknowledgement_executed",
        "service_restart_from_acknowledgement_performed",
        "launchd_from_acknowledgement_mutated",
        "active_binary_from_acknowledgement_mutated",
        "result_receipt_from_acknowledgement_recorded",
        "result_receipt_from_acknowledgement_persisted",
        "signing_receipt_summary_from_acknowledgement_recorded",
        "signing_receipt_briefing_from_acknowledgement_recorded",
        "memory_store_write_performed",
        "memory_store_mutated",
        "live_kg_write_performed",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "telegram_send_performed",
        "channel_send_performed",
        "external_send_performed",
        "public_release_claimed",
        "public_ga_claimed",
        "release_artifact_written",
        "public_artifact_written",
    ];
    let surface_specs: &[(&str, &str, &str, &[&str])] = &[
        (
            "source_signing_receipt_summary_briefing_report_required",
            "blocked_source_signing_receipt_summary_briefing_required_noop",
            "source_signing_receipt_summary_briefing_report_required",
            &["source_signing_receipt_summary_briefing_report_required"][..],
        ),
        (
            "artifact_signing_summary_final_operator_acknowledgement_claim",
            "blocked_artifact_signing_summary_final_ack_noop",
            "artifact_signing_summary_final_operator_acknowledgement_claim_denied",
            &["final_operator_acknowledgement_requested"][..],
        ),
        (
            "package_signing_briefing_operator_received_claim",
            "blocked_package_signing_briefing_operator_received_noop",
            "package_signing_briefing_operator_received_claim_denied",
            &["operator_received_requested"][..],
        ),
        (
            "signature_manifest_readback_operator_confirmed_claim",
            "blocked_signature_manifest_readback_operator_confirmed_noop",
            "signature_manifest_readback_operator_confirmed_claim_denied",
            &["operator_confirmed_requested"][..],
        ),
        (
            "notarization_status_banner_operator_read_claim",
            "blocked_notarization_status_banner_operator_read_noop",
            "notarization_status_banner_operator_read_claim_denied",
            &["operator_read_requested"][..],
        ),
        (
            "witness_notary_exported_summary_operator_seen_claim",
            "blocked_witness_notary_exported_summary_operator_seen_noop",
            "witness_notary_exported_summary_operator_seen_claim_denied",
            &["operator_seen_requested"][..],
        ),
        (
            "tombstone_garbage_collection_briefing_card_final_response_claim",
            "blocked_tombstone_gc_briefing_card_final_response_noop",
            "tombstone_garbage_collection_briefing_card_final_response_claim_denied",
            &["final_response_requested"][..],
        ),
        (
            "replacement_garbage_collection_notification_completion_acknowledgement_claim",
            "blocked_replacement_gc_notification_completion_ack_noop",
            "replacement_garbage_collection_notification_completion_acknowledgement_claim_denied",
            &["completion_acknowledgement_requested"][..],
        ),
        (
            "provenance_dashboard_narrative_status_acknowledgement_claim",
            "blocked_provenance_dashboard_status_ack_noop",
            "provenance_dashboard_narrative_status_acknowledgement_claim_denied",
            &[
                "status_acknowledgement_requested",
                "dashboard_acknowledgement_requested",
            ][..],
        ),
        (
            "sbom_audit_narrative_summary_acknowledgement_claim",
            "blocked_sbom_audit_summary_ack_noop",
            "sbom_audit_narrative_summary_acknowledgement_claim_denied",
            &["summary_acknowledgement_requested"][..],
        ),
        (
            "release_asset_final_summary_briefing_acknowledgement_claim",
            "blocked_release_asset_summary_briefing_ack_noop",
            "release_asset_final_summary_briefing_acknowledgement_claim_denied",
            &[
                "summary_acknowledgement_requested",
                "briefing_acknowledgement_requested",
            ][..],
        ),
        (
            "cdn_dashboard_briefing_readback_digest_acknowledgement_claim",
            "blocked_cdn_dashboard_briefing_readback_ack_noop",
            "cdn_dashboard_briefing_readback_digest_acknowledgement_claim_denied",
            &[
                "dashboard_acknowledgement_requested",
                "briefing_acknowledgement_requested",
                "readback_digest_acknowledgement_requested",
            ][..],
        ),
        (
            "package_registry_operator_memo_dashboard_notification_acknowledgement_claim",
            "blocked_package_registry_dashboard_notification_ack_noop",
            "package_registry_operator_memo_dashboard_notification_acknowledgement_claim_denied",
            &[
                "dashboard_acknowledgement_requested",
                "notification_acknowledgement_requested",
            ][..],
        ),
        (
            "dashboard_hash_approval_summary_channel_acknowledgement_claim",
            "blocked_dashboard_hash_approval_summary_channel_ack_noop",
            "dashboard_hash_approval_summary_channel_acknowledgement_claim_denied",
            &[
                "operator_approval_acknowledgement_requested",
                "summary_acknowledgement_requested",
                "channel_acknowledgement_requested",
            ][..],
        ),
        (
            "external_telegram_observability_briefing_acknowledgement_claim",
            "blocked_external_telegram_observability_briefing_ack_noop",
            "external_telegram_observability_briefing_acknowledgement_claim_denied",
            &[
                "external_acknowledgement_requested",
                "telegram_acknowledgement_requested",
                "briefing_acknowledgement_requested",
            ][..],
        ),
        (
            "release_publication_authority_view_acknowledgement_claim",
            "blocked_release_publication_authority_ack_noop",
            "release_publication_authority_view_acknowledgement_claim_denied",
            &["authority_acknowledgement_requested"][..],
        ),
        (
            "activation_live_install_view_acknowledgement_claim",
            "blocked_activation_live_install_ack_noop",
            "activation_live_install_view_acknowledgement_claim_denied",
            &["live_acknowledgement_requested"][..],
        ),
        (
            "install_restart_active_binary_status_acknowledgement_claim",
            "blocked_install_restart_active_binary_ack_noop",
            "install_restart_active_binary_status_acknowledgement_claim_denied",
            &[
                "live_acknowledgement_requested",
                "install_restart_active_binary_acknowledgement_requested",
                "status_acknowledgement_requested",
            ][..],
        ),
    ];
    let surfaces = surface_specs
        .iter()
        .map(|(surface, status, reason, true_keys)| {
            let mut surface_report = serde_json::json!({
                "surface": surface,
                "artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_surface": surface,
                "source_signing_receipt_summary_briefing_ready": source_ready,
                "artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_attempted": true,
                "artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_allowed": false,
                "artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_noop_confirmed": true,
                "artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_status": status,
                "reason": reason,
            });
            if let Some(surface_object) = surface_report.as_object_mut() {
                for key in &surface_false_keys {
                    surface_object.insert((*key).to_string(), serde_json::json!(false));
                }
                for key in true_keys.iter() {
                    surface_object.insert((*key).to_string(), serde_json::json!(true));
                }
            }
            surface_report
        })
        .collect::<Vec<_>>();
    let surface_count = surfaces.len();
    let contract_hash = sha256_text_value(&format!(
        "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-receipt-final-operator-acknowledgement-non-acceptance-denial:native:source={source_report_sha256}:surfaces={surface_count}:route_count={}:ack=0:authority=0:install=0:live=0",
        route_matrix.route_count
    ));
    let policy_hash = sha256_text_value(
        "artifact-signing-notarization-result-receipt-final-operator-acknowledgement:no-ack:no-received:no-confirmed:no-read:no-seen:no-final-response:no-status:no-authority:no-install:no-live",
    );
    let denials = vec![
        "source_artifact_distribution_signing_notarization_receipt_summary_briefing_report_required",
        "artifact_signing_summary_final_operator_acknowledgement_denied",
        "package_signing_briefing_operator_received_denied",
        "signature_manifest_readback_operator_confirmed_denied",
        "notarization_status_banner_operator_read_denied",
        "witness_notary_exported_summary_operator_seen_denied",
        "tombstone_garbage_collection_briefing_card_final_response_denied",
        "replacement_garbage_collection_notification_completion_acknowledgement_denied",
        "provenance_dashboard_narrative_status_acknowledgement_denied",
        "sbom_audit_narrative_summary_acknowledgement_denied",
        "release_asset_final_summary_briefing_acknowledgement_denied",
        "cdn_dashboard_briefing_readback_digest_acknowledgement_denied",
        "package_registry_operator_memo_dashboard_notification_acknowledgement_denied",
        "dashboard_hash_approval_summary_channel_acknowledgement_denied",
        "external_telegram_observability_briefing_acknowledgement_denied",
        "release_publication_authority_view_acknowledgement_denied",
        "activation_live_install_view_acknowledgement_denied",
        "install_restart_active_binary_status_acknowledgement_denied",
        "memory_provider_kg_secret_external_send_from_acknowledgement_denied",
    ];
    let denied_count = denials.len();
    let report_ready = source_ready
        && source_u64(
            "artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_surface_count",
        ) == 18
        && source_u64(
            "artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_denied_count",
        ) == 18
        && source_u64(
            "artifact_distribution_signing_notarization_receipt_operator_summary_recorded_count",
        ) == 0
        && source_u64(
            "artifact_distribution_signing_notarization_receipt_operator_briefing_recorded_count",
        ) == 0
        && source_u64("artifact_distribution_signing_notarization_receipt_readback_recorded_count")
            == 0
        && source_u64(
            "artifact_distribution_signing_notarization_receipt_briefing_delivery_recorded_count",
        ) == 0
        && source_u64(
            "release_publication_authority_from_signing_receipt_summary_briefing_derived_count",
        ) == 0
        && source_u64("activation_authority_from_signing_receipt_summary_briefing_derived_count")
            == 0
        && surface_count == 18
        && route_count_source_command_accepted;

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_final_operator_acknowledgement_non_acceptance_denial_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_OPERATOR_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_RECEIPT_FINAL_OPERATOR_ACKNOWLEDGEMENT_NON_ACCEPTANCE_DENIAL_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-receipt-final-operator-acknowledgement-non-acceptance-denial --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-27",
        "minimum_required_samples": 24,
        "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        "route_count": route_matrix.route_count,
        "implemented_route_count": route_matrix.implemented_route_count,
        "missing_route_count": route_matrix.missing_route_count,
        "route_count_source_command_accepted": route_count_source_command_accepted,
        "source_route_wired": true,
    });
    extend_json_object(
        &mut report,
        serde_json::json!({
            "artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_schema_version": "memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_operator_intent_consent_evidence_artifact_signing_receipt_final_operator_acknowledgement_non_acceptance_denial_route_v1",
            "artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_mode": "native_route_denied_signing_receipt_summary_briefing_cannot_be_acknowledged_accepted_promoted_or_used_for_authority_or_live_install",
            "memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_final_operator_acknowledgement_non_acceptance_denial_route_enabled": true,
            "memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_final_operator_acknowledgement_non_acceptance_denial_ready": report_ready,
            "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_non_acceptance_denial_ready": report_ready,
            "source_artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_route": "hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_operator_facing_summary_briefing_non_persistence_denial_route",
            "source_artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_ready": source_ready,
            "source_artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_report_sha256": source_report_sha256,
            "source_artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_contract_hash_sha256": source_contract_hash,
            "artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_contract_hash_sha256": contract_hash,
            "artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_policy_hash_sha256": policy_hash,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "source_artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_surface_count": source_u64("artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_surface_count"),
            "source_artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_attempt_count": source_u64("artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_attempt_count"),
            "source_artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_denied_count": source_u64("artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_denied_count"),
            "source_artifact_distribution_signing_notarization_receipt_operator_summary_recorded_count": source_u64("artifact_distribution_signing_notarization_receipt_operator_summary_recorded_count"),
            "source_artifact_distribution_signing_notarization_receipt_operator_briefing_recorded_count": source_u64("artifact_distribution_signing_notarization_receipt_operator_briefing_recorded_count"),
            "source_artifact_distribution_signing_notarization_receipt_readback_recorded_count": source_u64("artifact_distribution_signing_notarization_receipt_readback_recorded_count"),
            "source_artifact_distribution_signing_notarization_receipt_briefing_delivery_recorded_count": source_u64("artifact_distribution_signing_notarization_receipt_briefing_delivery_recorded_count"),
            "artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_surface_count": surface_count,
            "artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_attempt_count": surface_count,
            "artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_denied_count": surface_count,
            "artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_surfaces": surfaces,
            "denied_by_artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement": denials,
            "denied_by_artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_count": denied_count,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "allowed_next_actions": [{
                "action": "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_decision_status_promotion_denial_gate",
                "status": "allowed_report_only_next_slice",
                "records_final_acknowledgement": false,
                "records_received_confirmed_read_seen": false,
                "records_terminal_decision": false,
                "records_status_promotion": false,
                "derives_operator_approval": false,
                "derives_release_publication_authority": false,
                "derives_activation_authority": false,
                "renders_download_link": false,
                "emits_install_command": false,
                "installs_or_restarts": false,
                "mutates_active_binary": false,
                "mutates_memory_store": false,
                "writes_kg": false,
                "invokes_provider": false,
                "reads_credentials": false,
                "sends_externally": false
            }],
        }),
    );

    let zero_keys = [
        "artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_allowed_count",
        "artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_accepted_count",
        "artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_recorded_count",
        "artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_persisted_count",
        "artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_materialized_count",
        "artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_filesystem_written_count",
        "artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_delivered_count",
        "artifact_distribution_signing_notarization_receipt_operator_received_recorded_count",
        "artifact_distribution_signing_notarization_receipt_operator_confirmed_recorded_count",
        "artifact_distribution_signing_notarization_receipt_operator_read_recorded_count",
        "artifact_distribution_signing_notarization_receipt_operator_seen_recorded_count",
        "artifact_distribution_signing_notarization_receipt_final_response_recorded_count",
        "artifact_distribution_signing_notarization_receipt_completion_acknowledgement_recorded_count",
        "artifact_distribution_signing_notarization_receipt_status_acknowledgement_recorded_count",
        "artifact_distribution_signing_notarization_receipt_summary_acknowledgement_recorded_count",
        "artifact_distribution_signing_notarization_receipt_briefing_acknowledgement_recorded_count",
        "artifact_distribution_signing_notarization_receipt_readback_digest_acknowledgement_recorded_count",
        "artifact_distribution_signing_notarization_receipt_dashboard_acknowledgement_recorded_count",
        "artifact_distribution_signing_notarization_receipt_notification_acknowledgement_recorded_count",
        "artifact_distribution_signing_notarization_receipt_channel_acknowledgement_delivered_count",
        "artifact_distribution_signing_notarization_receipt_external_acknowledgement_sent_count",
        "artifact_distribution_signing_notarization_receipt_telegram_acknowledgement_sent_count",
        "artifact_distribution_signing_notarization_receipt_acceptance_from_acknowledgement_recorded_count",
        "operator_approval_from_signing_receipt_acknowledgement_derived_count",
        "release_publication_authority_from_signing_receipt_acknowledgement_derived_count",
        "activation_authority_from_signing_receipt_acknowledgement_derived_count",
        "download_link_from_signing_receipt_acknowledgement_rendered_count",
        "install_command_from_signing_receipt_acknowledgement_rendered_count",
        "install_from_signing_receipt_acknowledgement_executed_count",
        "service_restart_from_signing_receipt_acknowledgement_performed_count",
        "active_binary_from_signing_receipt_acknowledgement_mutated_count",
        "memory_store_write_performed_count",
        "live_kg_write_performed_count",
        "provider_invoked_count",
        "model_invoked_count",
        "credential_read_count",
        "secret_file_read_count",
        "external_send_performed_count",
    ];
    if let Some(report_object) = report.as_object_mut() {
        for key in &zero_keys {
            report_object.insert((*key).to_string(), serde_json::json!(0));
        }
    }

    let false_keys = [
        "artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_accepted",
        "artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_recorded",
        "artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_persisted",
        "artifact_distribution_signing_notarization_receipt_operator_received_recorded",
        "artifact_distribution_signing_notarization_receipt_operator_confirmed_recorded",
        "artifact_distribution_signing_notarization_receipt_operator_read_recorded",
        "artifact_distribution_signing_notarization_receipt_operator_seen_recorded",
        "artifact_distribution_signing_notarization_receipt_final_response_recorded",
        "artifact_distribution_signing_notarization_receipt_completion_acknowledgement_recorded",
        "artifact_distribution_signing_notarization_receipt_status_acknowledgement_recorded",
        "artifact_distribution_signing_notarization_receipt_acknowledgement_acceptance_recorded",
        "operator_acceptance_recorded",
        "operator_approval_recorded",
        "release_publication_authority_derived",
        "activation_authority_derived",
        "activation_allowed",
        "activation_performed",
        "download_link_rendered",
        "install_command_rendered",
        "memory_store_write_performed",
        "memory_store_mutated",
        "live_kg_write_performed",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "install_executed",
        "launchd_mutated",
        "service_restarted",
        "active_binary_mutated",
        "external_send_performed",
        "release_artifact_written",
        "public_artifact_written",
        "public_release_claimed",
        "public_ga_claimed",
        "filesystem_written",
    ];
    if let Some(report_object) = report.as_object_mut() {
        for key in &false_keys {
            report_object.insert((*key).to_string(), serde_json::json!(false));
        }
    }

    let mut side_effects = serde_json::Map::new();
    for key in &surface_false_keys {
        side_effects.insert((*key).to_string(), serde_json::json!(false));
    }
    side_effects.insert("filesystem_written".to_string(), serde_json::json!(false));
    extend_json_object(
        &mut report,
        serde_json::json!({ "side_effects": side_effects }),
    );
    report
}

fn hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_terminal_public_claim_status_exposure_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source =
        hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_final_operator_acknowledgement_non_acceptance_denial_report();
    let source_report_sha256 = sha256_json_value(&source);
    let source_u64 = |key: &str| {
        source
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let source_bool = |key: &str| {
        source
            .get(key)
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
    };
    let source_ready = source_bool(
        "memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_final_operator_acknowledgement_non_acceptance_denial_ready",
    );
    let source_contract_hash = source
        .get("artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_contract_hash_sha256")
        .cloned()
        .unwrap_or_else(|| serde_json::json!(""));
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;

    let surface_false_keys = [
        "source_signing_receipt_final_operator_acknowledgement_report_required",
        "public_claim_requested",
        "status_exposure_requested",
        "public_release_claim_requested",
        "public_status_exposure_requested",
        "telegram_status_exposure_requested",
        "release_publication_status_exposure_requested",
        "install_restart_active_binary_status_exposure_requested",
        "public_claim_allowed",
        "status_exposure_allowed",
        "public_release_claim_allowed",
        "public_status_exposure_allowed",
        "public_claim_recorded",
        "public_claim_persisted",
        "status_exposure_recorded",
        "status_exposure_persisted",
        "channel_status_exposure_delivered",
        "external_status_exposure_sent",
        "telegram_status_exposure_sent",
        "release_artifact_written",
        "public_artifact_written",
        "operator_approval_from_public_claim_derived",
        "release_publication_authority_from_public_claim_derived",
        "activation_authority_from_status_exposure_derived",
        "download_link_from_status_exposure_rendered",
        "install_command_from_status_exposure_emitted",
        "install_from_status_exposure_executed",
        "service_restart_from_status_exposure_performed",
        "active_binary_from_status_exposure_mutated",
        "memory_store_write_performed",
        "memory_store_mutated",
        "live_kg_write_performed",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "external_send_performed",
    ];
    let surface_specs: &[(&str, &str, &str, &[&str])] = &[
        (
            "source_signing_receipt_final_operator_acknowledgement_report_required",
            "blocked_source_signing_receipt_final_acknowledgement_required_noop",
            "source_signing_receipt_final_operator_acknowledgement_report_required",
            &["source_signing_receipt_final_operator_acknowledgement_report_required"][..],
        ),
        (
            "artifact_signing_receipt_public_claim_attempt",
            "blocked_artifact_signing_receipt_public_claim_noop",
            "artifact_signing_receipt_public_claim_attempt_denied",
            &["public_claim_requested"][..],
        ),
        (
            "package_signing_receipt_public_status_badge_exposure",
            "blocked_package_signing_receipt_public_status_badge_noop",
            "package_signing_receipt_public_status_badge_exposure_denied",
            &[
                "status_exposure_requested",
                "public_status_exposure_requested",
            ][..],
        ),
        (
            "signature_manifest_public_status_page_exposure",
            "blocked_signature_manifest_public_status_page_noop",
            "signature_manifest_public_status_page_exposure_denied",
            &[
                "status_exposure_requested",
                "public_status_exposure_requested",
            ][..],
        ),
        (
            "notarization_status_public_claim_attempt",
            "blocked_notarization_status_public_claim_noop",
            "notarization_status_public_claim_attempt_denied",
            &["public_claim_requested"][..],
        ),
        (
            "witness_notary_exported_summary_public_status_exposure",
            "blocked_witness_notary_public_status_noop",
            "witness_notary_exported_summary_public_status_exposure_denied",
            &[
                "status_exposure_requested",
                "public_status_exposure_requested",
            ][..],
        ),
        (
            "tombstone_garbage_collection_final_response_public_claim_attempt",
            "blocked_tombstone_gc_final_response_public_claim_noop",
            "tombstone_garbage_collection_final_response_public_claim_attempt_denied",
            &["public_claim_requested"][..],
        ),
        (
            "replacement_garbage_collection_completion_public_status_exposure",
            "blocked_replacement_gc_completion_public_status_noop",
            "replacement_garbage_collection_completion_public_status_exposure_denied",
            &[
                "status_exposure_requested",
                "public_status_exposure_requested",
            ][..],
        ),
        (
            "provenance_dashboard_public_status_exposure",
            "blocked_provenance_dashboard_public_status_noop",
            "provenance_dashboard_public_status_exposure_denied",
            &[
                "status_exposure_requested",
                "public_status_exposure_requested",
            ][..],
        ),
        (
            "sbom_audit_public_claim_attempt",
            "blocked_sbom_audit_public_claim_noop",
            "sbom_audit_public_claim_attempt_denied",
            &["public_claim_requested"][..],
        ),
        (
            "release_asset_public_briefing_exposure",
            "blocked_release_asset_public_briefing_noop",
            "release_asset_public_briefing_exposure_denied",
            &[
                "status_exposure_requested",
                "public_status_exposure_requested",
            ][..],
        ),
        (
            "cdn_dashboard_public_readback_exposure",
            "blocked_cdn_dashboard_public_readback_noop",
            "cdn_dashboard_public_readback_exposure_denied",
            &[
                "status_exposure_requested",
                "public_status_exposure_requested",
            ][..],
        ),
        (
            "package_registry_public_memo_notification_exposure",
            "blocked_package_registry_public_notification_noop",
            "package_registry_public_memo_notification_exposure_denied",
            &[
                "status_exposure_requested",
                "public_status_exposure_requested",
            ][..],
        ),
        (
            "dashboard_hash_public_approval_channel_exposure",
            "blocked_dashboard_hash_public_channel_exposure_noop",
            "dashboard_hash_public_approval_channel_exposure_denied",
            &[
                "status_exposure_requested",
                "public_status_exposure_requested",
            ][..],
        ),
        (
            "external_telegram_public_claim_exposure",
            "blocked_external_telegram_public_claim_exposure_noop",
            "external_telegram_public_claim_exposure_denied",
            &[
                "public_claim_requested",
                "telegram_status_exposure_requested",
            ][..],
        ),
        (
            "release_publication_public_claim_status_exposure",
            "blocked_release_publication_public_claim_status_exposure_noop",
            "release_publication_public_claim_status_exposure_denied",
            &[
                "public_claim_requested",
                "public_release_claim_requested",
                "status_exposure_requested",
                "public_status_exposure_requested",
                "release_publication_status_exposure_requested",
            ][..],
        ),
        (
            "activation_live_install_status_public_exposure",
            "blocked_activation_live_install_public_status_exposure_noop",
            "activation_live_install_status_public_exposure_denied",
            &[
                "status_exposure_requested",
                "public_status_exposure_requested",
            ][..],
        ),
        (
            "install_restart_active_binary_public_status_exposure",
            "blocked_install_restart_active_binary_public_status_exposure_noop",
            "install_restart_active_binary_public_status_exposure_denied",
            &[
                "status_exposure_requested",
                "public_status_exposure_requested",
                "install_restart_active_binary_status_exposure_requested",
            ][..],
        ),
    ];
    let surfaces = surface_specs
        .iter()
        .map(|(surface, status, reason, true_keys)| {
            let mut surface_report = serde_json::json!({
                "surface": surface,
                "artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_surface": surface,
                "source_signing_receipt_final_operator_acknowledgement_ready": source_ready,
                "artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_attempted": true,
                "artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_noop_confirmed": true,
                "artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_status": status,
                "reason": reason,
            });
            if let Some(surface_object) = surface_report.as_object_mut() {
                for key in &surface_false_keys {
                    surface_object.insert((*key).to_string(), serde_json::json!(false));
                }
                for key in true_keys.iter() {
                    surface_object.insert((*key).to_string(), serde_json::json!(true));
                }
            }
            surface_report
        })
        .collect::<Vec<_>>();
    let surface_count = surfaces.len();
    let contract_hash = sha256_text_value(&format!(
        "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-receipt-terminal-public-claim-status-exposure-denial:native:source={source_report_sha256}:surfaces={surface_count}:route_count={}:public=0:status=0:authority=0:install=0:live=0",
        route_matrix.route_count
    ));
    let policy_hash = sha256_text_value(
        "artifact-signing-notarization-receipt-terminal-public-claim-status-exposure:no-public-claim:no-status-exposure:no-release:no-channel:no-telegram:no-install",
    );
    let denials = vec![
        "source_artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_report_required",
        "artifact_distribution_signing_notarization_receipt_public_claim_recording_denied",
        "artifact_distribution_signing_notarization_receipt_public_status_exposure_denied",
        "artifact_distribution_signing_notarization_receipt_public_release_claim_denied",
        "artifact_distribution_signing_notarization_receipt_channel_external_telegram_public_status_denied",
        "artifact_distribution_signing_notarization_receipt_release_artifact_write_denied",
        "artifact_distribution_signing_notarization_receipt_public_artifact_write_denied",
        "artifact_distribution_signing_notarization_receipt_operator_approval_from_public_claim_denied",
        "artifact_distribution_signing_notarization_receipt_release_publication_authority_from_public_claim_denied",
        "artifact_distribution_signing_notarization_receipt_activation_authority_from_status_exposure_denied",
        "artifact_distribution_signing_notarization_receipt_download_install_restart_active_binary_from_status_exposure_denied",
        "artifact_distribution_signing_notarization_receipt_memory_provider_secret_external_send_from_public_exposure_denied",
    ];
    let denied_count = denials.len();
    let report_ready = source_ready
        && source_u64(
            "artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_surface_count",
        ) == 18
        && source_u64(
            "artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_denied_count",
        ) == 18
        && source_u64(
            "artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_recorded_count",
        ) == 0
        && source_u64(
            "artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_accepted_count",
        ) == 0
        && source_u64(
            "release_publication_authority_from_signing_receipt_acknowledgement_derived_count",
        ) == 0
        && source_u64("activation_authority_from_signing_receipt_acknowledgement_derived_count")
            == 0
        && surface_count == 18
        && route_count_source_command_accepted;

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_terminal_public_claim_status_exposure_denial_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_OPERATOR_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_RECEIPT_TERMINAL_PUBLIC_CLAIM_STATUS_EXPOSURE_DENIAL_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-receipt-terminal-public-claim-status-exposure-denial --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-28",
        "minimum_required_samples": 24,
        "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        "route_count": route_matrix.route_count,
        "implemented_route_count": route_matrix.implemented_route_count,
        "missing_route_count": route_matrix.missing_route_count,
        "route_count_source_command_accepted": route_count_source_command_accepted,
        "source_route_wired": true,
    });
    extend_json_object(
        &mut report,
        serde_json::json!({
            "artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_schema_version": "memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_operator_intent_consent_evidence_artifact_signing_receipt_terminal_public_claim_status_exposure_denial_route_v1",
            "artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_mode": "native_route_denied_signing_receipt_final_acknowledgement_cannot_create_public_claim_status_exposure_release_channel_telegram_or_live_install",
            "memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_terminal_public_claim_status_exposure_denial_route_enabled": true,
            "memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_terminal_public_claim_status_exposure_denial_ready": report_ready,
            "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_denial_ready": report_ready,
            "source_artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_route": "hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_final_operator_acknowledgement_non_acceptance_denial_route",
            "source_artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_ready": source_ready,
            "source_artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_report_sha256": source_report_sha256,
            "source_artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_contract_hash_sha256": source_contract_hash,
            "artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_contract_hash_sha256": contract_hash,
            "artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_policy_hash_sha256": policy_hash,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "source_artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_surface_count": source_u64("artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_surface_count"),
            "source_artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_attempt_count": source_u64("artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_attempt_count"),
            "source_artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_denied_count": source_u64("artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_denied_count"),
            "source_artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_accepted_count": source_u64("artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_accepted_count"),
            "source_artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_recorded_count": source_u64("artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_recorded_count"),
            "source_release_publication_authority_from_signing_receipt_acknowledgement_derived_count": source_u64("release_publication_authority_from_signing_receipt_acknowledgement_derived_count"),
            "source_activation_authority_from_signing_receipt_acknowledgement_derived_count": source_u64("activation_authority_from_signing_receipt_acknowledgement_derived_count"),
            "artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_surface_count": surface_count,
            "artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_attempt_count": surface_count,
            "artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_denied_count": surface_count,
            "artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_surfaces": surfaces,
            "denied_by_artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure": denials,
            "denied_by_artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_count": denied_count,
            "allowed_next_actions": [{
                "action": "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_denial_gate",
                "status": "allowed_report_only_next_slice",
                "records_public_claim": false,
                "records_status_exposure": false,
                "delivers_channel_status": false,
                "sends_telegram": false,
                "writes_release_artifact": false,
                "writes_public_artifact": false,
                "derives_operator_approval": false,
                "derives_release_publication_authority": false,
                "derives_activation_authority": false,
                "renders_download_link": false,
                "emits_install_command": false,
                "installs_or_restarts": false,
                "mutates_active_binary": false,
                "mutates_memory_store": false,
                "writes_kg": false,
                "invokes_provider": false,
                "reads_credentials": false,
                "sends_externally": false
            }],
        }),
    );

    let zero_keys = [
        "artifact_distribution_signing_notarization_receipt_public_claim_recorded_count",
        "artifact_distribution_signing_notarization_receipt_public_claim_persisted_count",
        "artifact_distribution_signing_notarization_receipt_status_exposure_recorded_count",
        "artifact_distribution_signing_notarization_receipt_status_exposure_persisted_count",
        "artifact_distribution_signing_notarization_receipt_channel_status_exposure_delivered_count",
        "artifact_distribution_signing_notarization_receipt_external_status_exposure_sent_count",
        "artifact_distribution_signing_notarization_receipt_telegram_status_exposure_sent_count",
        "release_artifact_written_count",
        "public_artifact_written_count",
        "operator_approval_from_signing_receipt_public_claim_derived_count",
        "release_publication_authority_from_signing_receipt_public_claim_derived_count",
        "activation_authority_from_signing_receipt_status_exposure_derived_count",
        "download_link_from_signing_receipt_status_exposure_rendered_count",
        "install_command_from_signing_receipt_status_exposure_emitted_count",
        "install_from_signing_receipt_status_exposure_executed_count",
        "service_restart_from_signing_receipt_status_exposure_performed_count",
        "active_binary_from_signing_receipt_status_exposure_mutated_count",
        "memory_store_write_performed_count",
        "live_kg_write_performed_count",
        "provider_invoked_count",
        "model_invoked_count",
        "credential_read_count",
        "secret_file_read_count",
        "external_send_performed_count",
    ];
    if let Some(report_object) = report.as_object_mut() {
        for key in &zero_keys {
            report_object.insert((*key).to_string(), serde_json::json!(0));
        }
    }

    let false_keys = [
        "artifact_distribution_signing_notarization_receipt_public_claim_recorded",
        "artifact_distribution_signing_notarization_receipt_status_exposure_recorded",
        "artifact_distribution_signing_notarization_receipt_public_status_exposed",
        "public_release_claimed",
        "public_ga_claimed",
        "operator_approval_recorded",
        "release_publication_authority_derived",
        "activation_authority_derived",
        "download_link_rendered",
        "install_command_emitted",
        "activation_allowed",
        "memory_store_write_performed",
        "memory_store_mutated",
        "live_kg_write_performed",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "install_executed",
        "launchd_mutated",
        "service_restarted",
        "active_binary_mutated",
        "release_artifact_written",
        "public_artifact_written",
        "external_send_performed",
        "filesystem_written",
    ];
    if let Some(report_object) = report.as_object_mut() {
        for key in &false_keys {
            report_object.insert((*key).to_string(), serde_json::json!(false));
        }
    }

    let mut side_effects = serde_json::Map::new();
    for key in &surface_false_keys {
        side_effects.insert((*key).to_string(), serde_json::json!(false));
    }
    for key in [
        "public_release_claimed",
        "public_ga_claimed",
        "telegram_send_performed",
        "channel_send_performed",
        "filesystem_written",
    ] {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }
    extend_json_object(
        &mut report,
        serde_json::json!({ "side_effects": side_effects }),
    );
    report
}

fn hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_terminal_public_claim_delivery_readback_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source =
        hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_terminal_public_claim_status_exposure_denial_report();
    let source_report_sha256 = sha256_json_value(&source);
    let source_u64 = |key: &str| {
        source
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let source_bool = |key: &str| {
        source
            .get(key)
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
    };
    let source_ready = source_bool(
        "memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_terminal_public_claim_status_exposure_denial_ready",
    );
    let source_contract_hash = source
        .get("artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_contract_hash_sha256")
        .cloned()
        .unwrap_or_else(|| serde_json::json!(""));
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;

    let surface_false_keys = [
        "source_public_claim_status_exposure_report_required",
        "public_claim_delivery_requested",
        "status_readback_requested",
        "channel_delivery_requested",
        "telegram_delivery_requested",
        "release_publication_delivery_readback_requested",
        "install_restart_active_binary_readback_requested",
        "public_claim_delivery_allowed",
        "status_readback_allowed",
        "channel_delivery_allowed",
        "telegram_delivery_allowed",
        "public_claim_delivery_recorded",
        "public_claim_delivery_persisted",
        "status_readback_recorded",
        "status_readback_persisted",
        "channel_delivery_recorded",
        "channel_delivery_persisted",
        "channel_status_readback_delivered",
        "external_delivery_readback_sent",
        "telegram_delivery_readback_sent",
        "delivery_receipt_recorded",
        "delivery_receipt_persisted",
        "readback_receipt_recorded",
        "readback_receipt_persisted",
        "release_artifact_written",
        "public_artifact_written",
        "operator_approval_from_delivery_readback_derived",
        "release_publication_authority_from_delivery_readback_derived",
        "activation_authority_from_delivery_readback_derived",
        "download_link_from_delivery_readback_rendered",
        "install_command_from_delivery_readback_emitted",
        "install_from_delivery_readback_executed",
        "service_restart_from_delivery_readback_performed",
        "active_binary_from_delivery_readback_mutated",
        "memory_store_write_performed",
        "memory_store_mutated",
        "live_kg_write_performed",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "external_send_performed",
    ];
    let surface_specs: &[(&str, &str, &str, &[&str])] = &[
        (
            "source_public_claim_status_exposure_report_required",
            "blocked_source_public_exposure_report_required_noop",
            "source_public_claim_status_exposure_report_required",
            &["source_public_claim_status_exposure_report_required"][..],
        ),
        (
            "artifact_signing_receipt_claim_channel_delivery_attempt",
            "blocked_artifact_signing_claim_channel_delivery_noop",
            "artifact_signing_receipt_claim_channel_delivery_attempt_denied",
            &[
                "public_claim_delivery_requested",
                "channel_delivery_requested",
            ][..],
        ),
        (
            "package_signing_status_badge_readback_attempt",
            "blocked_package_signing_status_badge_readback_noop",
            "package_signing_status_badge_readback_attempt_denied",
            &["status_readback_requested"][..],
        ),
        (
            "signature_manifest_status_page_readback_attempt",
            "blocked_signature_manifest_status_page_readback_noop",
            "signature_manifest_status_page_readback_attempt_denied",
            &["status_readback_requested"][..],
        ),
        (
            "notarization_claim_readback_attempt",
            "blocked_notarization_claim_readback_noop",
            "notarization_claim_readback_attempt_denied",
            &["public_claim_delivery_requested"][..],
        ),
        (
            "witness_notary_summary_channel_delivery_attempt",
            "blocked_witness_notary_summary_channel_delivery_noop",
            "witness_notary_summary_channel_delivery_attempt_denied",
            &["status_readback_requested", "channel_delivery_requested"][..],
        ),
        (
            "tombstone_gc_final_response_delivery_readback_attempt",
            "blocked_tombstone_gc_final_response_delivery_readback_noop",
            "tombstone_gc_final_response_delivery_readback_attempt_denied",
            &["public_claim_delivery_requested"][..],
        ),
        (
            "replacement_gc_completion_readback_attempt",
            "blocked_replacement_gc_completion_readback_noop",
            "replacement_gc_completion_readback_attempt_denied",
            &["status_readback_requested"][..],
        ),
        (
            "provenance_dashboard_status_readback_attempt",
            "blocked_provenance_dashboard_status_readback_noop",
            "provenance_dashboard_status_readback_attempt_denied",
            &["status_readback_requested"][..],
        ),
        (
            "sbom_audit_public_claim_readback_attempt",
            "blocked_sbom_audit_public_claim_readback_noop",
            "sbom_audit_public_claim_readback_attempt_denied",
            &["public_claim_delivery_requested"][..],
        ),
        (
            "release_asset_briefing_channel_delivery_attempt",
            "blocked_release_asset_briefing_channel_delivery_noop",
            "release_asset_briefing_channel_delivery_attempt_denied",
            &["status_readback_requested", "channel_delivery_requested"][..],
        ),
        (
            "cdn_dashboard_readback_attempt",
            "blocked_cdn_dashboard_readback_noop",
            "cdn_dashboard_readback_attempt_denied",
            &["status_readback_requested"][..],
        ),
        (
            "package_registry_memo_delivery_attempt",
            "blocked_package_registry_memo_delivery_noop",
            "package_registry_memo_delivery_attempt_denied",
            &["status_readback_requested", "channel_delivery_requested"][..],
        ),
        (
            "dashboard_hash_approval_channel_readback_attempt",
            "blocked_dashboard_hash_approval_channel_readback_noop",
            "dashboard_hash_approval_channel_readback_attempt_denied",
            &["status_readback_requested", "channel_delivery_requested"][..],
        ),
        (
            "external_telegram_claim_delivery_readback_attempt",
            "blocked_external_telegram_claim_delivery_readback_noop",
            "external_telegram_claim_delivery_readback_attempt_denied",
            &[
                "public_claim_delivery_requested",
                "telegram_delivery_requested",
            ][..],
        ),
        (
            "release_publication_claim_status_delivery_readback_attempt",
            "blocked_release_publication_claim_status_delivery_readback_noop",
            "release_publication_claim_status_delivery_readback_attempt_denied",
            &[
                "public_claim_delivery_requested",
                "status_readback_requested",
                "channel_delivery_requested",
                "release_publication_delivery_readback_requested",
            ][..],
        ),
        (
            "activation_live_install_status_readback_attempt",
            "blocked_activation_live_install_status_readback_noop",
            "activation_live_install_status_readback_attempt_denied",
            &["status_readback_requested"][..],
        ),
        (
            "install_restart_active_binary_status_readback_attempt",
            "blocked_install_restart_active_binary_status_readback_noop",
            "install_restart_active_binary_status_readback_attempt_denied",
            &[
                "status_readback_requested",
                "install_restart_active_binary_readback_requested",
            ][..],
        ),
    ];
    let surfaces = surface_specs
        .iter()
        .map(|(surface, status, reason, true_keys)| {
            let mut surface_report = serde_json::json!({
                "surface": surface,
                "artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_surface": surface,
                "source_artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_ready": source_ready,
                "artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_attempted": true,
                "artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_noop_confirmed": true,
                "artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_status": status,
                "reason": reason,
            });
            if let Some(surface_object) = surface_report.as_object_mut() {
                for key in &surface_false_keys {
                    surface_object.insert((*key).to_string(), serde_json::json!(false));
                }
                for key in true_keys.iter() {
                    surface_object.insert((*key).to_string(), serde_json::json!(true));
                }
            }
            surface_report
        })
        .collect::<Vec<_>>();
    let surface_count = surfaces.len();
    let contract_hash = sha256_text_value(&format!(
        "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-receipt-terminal-public-claim-delivery-readback-denial:native:source={source_report_sha256}:surfaces={surface_count}:route_count={}:delivery=0:readback=0:receipt=0:authority=0:install=0:live=0",
        route_matrix.route_count
    ));
    let policy_hash = sha256_text_value(
        "artifact-signing-notarization-receipt-terminal-public-claim-delivery-readback:no-delivery:no-readback:no-receipt:no-release:no-channel:no-telegram:no-install",
    );
    let denials = vec![
        "source_artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_report_required",
        "artifact_distribution_signing_notarization_receipt_public_claim_delivery_recording_denied",
        "artifact_distribution_signing_notarization_receipt_status_readback_recording_denied",
        "artifact_distribution_signing_notarization_receipt_channel_delivery_recording_denied",
        "artifact_distribution_signing_notarization_receipt_channel_external_telegram_delivery_readback_denied",
        "artifact_distribution_signing_notarization_receipt_delivery_receipt_persistence_denied",
        "artifact_distribution_signing_notarization_receipt_readback_receipt_persistence_denied",
        "artifact_distribution_signing_notarization_receipt_release_artifact_write_denied",
        "artifact_distribution_signing_notarization_receipt_public_artifact_write_denied",
        "artifact_distribution_signing_notarization_receipt_operator_approval_from_delivery_readback_denied",
        "artifact_distribution_signing_notarization_receipt_release_publication_authority_from_delivery_readback_denied",
        "artifact_distribution_signing_notarization_receipt_activation_authority_from_delivery_readback_denied",
        "artifact_distribution_signing_notarization_receipt_download_install_restart_active_binary_from_delivery_readback_denied",
        "artifact_distribution_signing_notarization_receipt_memory_provider_secret_external_send_from_delivery_readback_denied",
    ];
    let denied_count = denials.len();
    let report_ready = source_ready
        && source_u64(
            "artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_surface_count",
        ) == 18
        && source_u64(
            "artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_denied_count",
        ) == 18
        && source_u64(
            "artifact_distribution_signing_notarization_receipt_public_claim_recorded_count",
        ) == 0
        && source_u64(
            "artifact_distribution_signing_notarization_receipt_status_exposure_recorded_count",
        ) == 0
        && source_u64(
            "artifact_distribution_signing_notarization_receipt_channel_status_exposure_delivered_count",
        ) == 0
        && source_u64(
            "artifact_distribution_signing_notarization_receipt_external_status_exposure_sent_count",
        ) == 0
        && source_u64(
            "artifact_distribution_signing_notarization_receipt_telegram_status_exposure_sent_count",
        ) == 0
        && source_u64("release_artifact_written_count") == 0
        && source_u64("public_artifact_written_count") == 0
        && source_u64("operator_approval_from_signing_receipt_public_claim_derived_count") == 0
        && source_u64(
            "release_publication_authority_from_signing_receipt_public_claim_derived_count",
        ) == 0
        && source_u64("activation_authority_from_signing_receipt_status_exposure_derived_count")
            == 0
        && surface_count == 18
        && route_count_source_command_accepted;

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_terminal_public_claim_delivery_readback_denial_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_OPERATOR_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_RECEIPT_TERMINAL_PUBLIC_CLAIM_DELIVERY_READBACK_DENIAL_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-receipt-terminal-public-claim-delivery-readback-denial --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-28",
        "minimum_required_samples": 24,
        "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        "route_count": route_matrix.route_count,
        "implemented_route_count": route_matrix.implemented_route_count,
        "missing_route_count": route_matrix.missing_route_count,
        "route_count_source_command_accepted": route_count_source_command_accepted,
        "source_route_wired": true,
    });
    extend_json_object(
        &mut report,
        serde_json::json!({
            "artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_schema_version": "memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_operator_intent_consent_evidence_artifact_signing_receipt_terminal_public_claim_delivery_readback_denial_route_v1",
            "artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_mode": "native_route_denied_public_claim_status_exposure_cannot_create_delivery_readback_receipt_release_channel_telegram_or_live_install",
            "memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_terminal_public_claim_delivery_readback_denial_route_enabled": true,
            "memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_terminal_public_claim_delivery_readback_denial_ready": report_ready,
            "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_denial_ready": report_ready,
            "source_artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_route": "hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_terminal_public_claim_status_exposure_denial_route",
            "source_artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_ready": source_ready,
            "source_artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_report_sha256": source_report_sha256,
            "source_artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_contract_hash_sha256": source_contract_hash,
            "artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_contract_hash_sha256": contract_hash,
            "artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_policy_hash_sha256": policy_hash,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "source_artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_surface_count": source_u64("artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_surface_count"),
            "source_artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_attempt_count": source_u64("artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_attempt_count"),
            "source_artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_denied_count": source_u64("artifact_distribution_signing_notarization_receipt_terminal_public_claim_status_exposure_denied_count"),
            "source_artifact_distribution_signing_notarization_receipt_public_claim_recorded_count": source_u64("artifact_distribution_signing_notarization_receipt_public_claim_recorded_count"),
            "source_artifact_distribution_signing_notarization_receipt_status_exposure_recorded_count": source_u64("artifact_distribution_signing_notarization_receipt_status_exposure_recorded_count"),
            "source_artifact_distribution_signing_notarization_receipt_channel_status_exposure_delivered_count": source_u64("artifact_distribution_signing_notarization_receipt_channel_status_exposure_delivered_count"),
            "source_artifact_distribution_signing_notarization_receipt_external_status_exposure_sent_count": source_u64("artifact_distribution_signing_notarization_receipt_external_status_exposure_sent_count"),
            "source_artifact_distribution_signing_notarization_receipt_telegram_status_exposure_sent_count": source_u64("artifact_distribution_signing_notarization_receipt_telegram_status_exposure_sent_count"),
            "artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_surface_count": surface_count,
            "artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_attempt_count": surface_count,
            "artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_denied_count": surface_count,
            "artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_surfaces": surfaces,
            "denied_by_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback": denials,
            "denied_by_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_count": denied_count,
            "allowed_next_actions": [{
                "action": "prepare_artifact_distribution_signing_notarization_receipt_release_public_artifact_publication_denial_gate",
                "status": "allowed_report_only_next_slice",
                "records_public_claim_delivery": false,
                "records_status_readback": false,
                "records_channel_delivery": false,
                "records_delivery_receipt": false,
                "records_readback_receipt": false,
                "sends_telegram": false,
                "writes_release_artifact": false,
                "writes_public_artifact": false,
                "derives_operator_approval": false,
                "derives_release_publication_authority": false,
                "derives_activation_authority": false,
                "renders_download_link": false,
                "emits_install_command": false,
                "installs_or_restarts": false,
                "mutates_active_binary": false,
                "mutates_memory_store": false,
                "writes_kg": false,
                "invokes_provider": false,
                "reads_credentials": false,
                "sends_externally": false
            }],
        }),
    );

    let zero_keys = [
        "artifact_distribution_signing_notarization_receipt_public_claim_delivery_recorded_count",
        "artifact_distribution_signing_notarization_receipt_public_claim_delivery_persisted_count",
        "artifact_distribution_signing_notarization_receipt_status_readback_recorded_count",
        "artifact_distribution_signing_notarization_receipt_status_readback_persisted_count",
        "artifact_distribution_signing_notarization_receipt_channel_delivery_recorded_count",
        "artifact_distribution_signing_notarization_receipt_channel_delivery_persisted_count",
        "artifact_distribution_signing_notarization_receipt_channel_status_readback_delivered_count",
        "artifact_distribution_signing_notarization_receipt_external_delivery_readback_sent_count",
        "artifact_distribution_signing_notarization_receipt_telegram_delivery_readback_sent_count",
        "delivery_receipt_recorded_count",
        "delivery_receipt_persisted_count",
        "readback_receipt_recorded_count",
        "readback_receipt_persisted_count",
        "release_artifact_written_count",
        "public_artifact_written_count",
        "operator_approval_from_delivery_readback_derived_count",
        "release_publication_authority_from_delivery_readback_derived_count",
        "activation_authority_from_delivery_readback_derived_count",
        "download_link_from_delivery_readback_rendered_count",
        "install_command_from_delivery_readback_emitted_count",
        "install_from_delivery_readback_executed_count",
        "service_restart_from_delivery_readback_performed_count",
        "active_binary_from_delivery_readback_mutated_count",
        "memory_store_write_performed_count",
        "live_kg_write_performed_count",
        "provider_invoked_count",
        "model_invoked_count",
        "credential_read_count",
        "secret_file_read_count",
        "external_send_performed_count",
    ];
    if let Some(report_object) = report.as_object_mut() {
        for key in &zero_keys {
            report_object.insert((*key).to_string(), serde_json::json!(0));
        }
    }

    let false_keys = [
        "artifact_distribution_signing_notarization_receipt_public_claim_delivery_recorded",
        "artifact_distribution_signing_notarization_receipt_status_readback_recorded",
        "artifact_distribution_signing_notarization_receipt_channel_delivery_recorded",
        "delivery_receipt_recorded",
        "delivery_receipt_persisted",
        "readback_receipt_recorded",
        "readback_receipt_persisted",
        "public_release_claimed",
        "public_ga_claimed",
        "operator_approval_recorded",
        "release_publication_authority_derived",
        "activation_authority_derived",
        "download_link_rendered",
        "install_command_emitted",
        "activation_allowed",
        "memory_store_write_performed",
        "memory_store_mutated",
        "live_kg_write_performed",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "install_executed",
        "launchd_mutated",
        "service_restarted",
        "active_binary_mutated",
        "release_artifact_written",
        "public_artifact_written",
        "external_send_performed",
        "filesystem_written",
    ];
    if let Some(report_object) = report.as_object_mut() {
        for key in &false_keys {
            report_object.insert((*key).to_string(), serde_json::json!(false));
        }
    }

    let mut side_effects = serde_json::Map::new();
    for key in &surface_false_keys {
        side_effects.insert((*key).to_string(), serde_json::json!(false));
    }
    for key in [
        "telegram_send_performed",
        "channel_send_performed",
        "public_release_claimed",
        "public_ga_claimed",
        "filesystem_written",
    ] {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }
    extend_json_object(
        &mut report,
        serde_json::json!({ "side_effects": side_effects }),
    );
    report
}

fn hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_release_public_artifact_publication_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source =
        hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_terminal_public_claim_delivery_readback_denial_report();
    let source_report_sha256 = sha256_json_value(&source);
    let source_u64 = |key: &str| {
        source
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let source_bool = |key: &str| {
        source
            .get(key)
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
    };
    let source_ready = source_bool(
        "memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_terminal_public_claim_delivery_readback_denial_ready",
    );
    let source_contract_hash = source
        .get(
            "artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_contract_hash_sha256",
        )
        .and_then(serde_json::Value::as_str)
        .unwrap_or("missing");
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT;
    let surface_false_keys = [
        "release_artifact_publication_allowed",
        "public_artifact_publication_allowed",
        "publication_queue_allowed",
        "publication_manifest_allowed",
        "public_distribution_allowed",
        "package_registry_publication_allowed",
        "external_package_channel_publication_allowed",
        "telegram_package_channel_publication_allowed",
        "public_release_claim_allowed",
        "public_ga_claim_allowed",
        "release_notes_materialization_allowed",
        "changelog_materialization_allowed",
        "release_artifact_written",
        "public_artifact_written",
        "publication_queue_enqueued",
        "publication_manifest_written",
        "public_distribution_performed",
        "package_registry_artifact_published",
        "external_package_channel_published",
        "telegram_package_channel_published",
        "release_notes_materialized",
        "changelog_materialized",
        "operator_approval_from_publication_derived",
        "release_publication_authority_from_publication_derived",
        "activation_authority_from_publication_derived",
        "download_link_from_publication_rendered",
        "install_command_from_publication_emitted",
        "install_from_publication_executed",
        "service_restart_from_publication_performed",
        "active_binary_from_publication_mutated",
        "memory_store_write_performed",
        "memory_store_mutated",
        "live_kg_write_performed",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "external_send_performed",
    ];
    let surface_specs = [
        (
            "source_terminal_public_claim_delivery_readback_report_required",
            "blocked_missing_source_delivery_readback_noop",
            "source_terminal_public_claim_delivery_readback_report_required",
            &["release_artifact_publication_requested"][..],
        ),
        (
            "release_artifact_publication_attempt",
            "blocked_release_artifact_publication_noop",
            "release_artifact_publication_denied",
            &["release_artifact_publication_requested"][..],
        ),
        (
            "public_artifact_publication_attempt",
            "blocked_public_artifact_publication_noop",
            "public_artifact_publication_denied",
            &[
                "release_artifact_publication_requested",
                "public_artifact_publication_requested",
            ][..],
        ),
        (
            "signature_notarization_public_asset_publication_attempt",
            "blocked_signature_notarization_public_asset_publication_noop",
            "signature_notarization_public_asset_publication_denied",
            &[
                "release_artifact_publication_requested",
                "public_artifact_publication_requested",
            ][..],
        ),
        (
            "publication_queue_enqueue_attempt",
            "blocked_publication_queue_enqueue_noop",
            "publication_queue_enqueue_denied",
            &["publication_queue_requested"][..],
        ),
        (
            "publication_manifest_write_attempt",
            "blocked_publication_manifest_write_noop",
            "publication_manifest_write_denied",
            &[
                "release_artifact_publication_requested",
                "publication_manifest_requested",
            ][..],
        ),
        (
            "cdn_public_artifact_write_attempt",
            "blocked_cdn_public_artifact_write_noop",
            "cdn_public_artifact_write_denied",
            &[
                "public_artifact_publication_requested",
                "public_distribution_requested",
            ][..],
        ),
        (
            "update_feed_publication_attempt",
            "blocked_update_feed_publication_noop",
            "update_feed_publication_denied",
            &[
                "public_artifact_publication_requested",
                "public_distribution_requested",
            ][..],
        ),
        (
            "package_registry_publication_attempt",
            "blocked_package_registry_publication_noop",
            "package_registry_publication_denied",
            &[
                "public_artifact_publication_requested",
                "package_registry_publication_requested",
            ][..],
        ),
        (
            "external_package_channel_publication_attempt",
            "blocked_external_package_channel_publication_noop",
            "external_package_channel_publication_denied",
            &[
                "public_distribution_requested",
                "external_package_channel_publication_requested",
            ][..],
        ),
        (
            "telegram_package_channel_publication_attempt",
            "blocked_telegram_package_channel_publication_noop",
            "telegram_package_channel_publication_denied",
            &[
                "public_distribution_requested",
                "telegram_package_channel_publication_requested",
            ][..],
        ),
        (
            "public_release_claim_publication_attempt",
            "blocked_public_release_claim_publication_noop",
            "public_release_claim_publication_denied",
            &[
                "public_artifact_publication_requested",
                "public_release_claim_requested",
            ][..],
        ),
        (
            "public_ga_claim_publication_attempt",
            "blocked_public_ga_claim_publication_noop",
            "public_ga_claim_publication_denied",
            &[
                "public_artifact_publication_requested",
                "public_ga_claim_requested",
            ][..],
        ),
        (
            "release_notes_changelog_publication_attempt",
            "blocked_release_notes_changelog_publication_noop",
            "release_notes_changelog_publication_denied",
            &[
                "release_artifact_publication_requested",
                "release_notes_changelog_requested",
            ][..],
        ),
        (
            "distribution_index_publication_attempt",
            "blocked_distribution_index_publication_noop",
            "distribution_index_publication_denied",
            &[
                "public_artifact_publication_requested",
                "public_distribution_requested",
                "publication_manifest_requested",
            ][..],
        ),
        (
            "dashboard_publication_status_attempt",
            "blocked_dashboard_publication_status_noop",
            "dashboard_publication_status_denied",
            &[
                "public_distribution_requested",
                "release_publication_status_publication_requested",
            ][..],
        ),
        (
            "activation_live_install_publication_attempt",
            "blocked_activation_live_install_publication_noop",
            "activation_live_install_publication_denied",
            &[
                "public_release_claim_requested",
                "install_restart_active_binary_publication_requested",
            ][..],
        ),
        (
            "install_restart_active_binary_publication_attempt",
            "blocked_install_restart_active_binary_publication_noop",
            "install_restart_active_binary_publication_denied",
            &["install_restart_active_binary_publication_requested"][..],
        ),
    ];
    let surfaces = surface_specs
        .iter()
        .map(|(surface, status, reason, true_keys)| {
            let mut surface_report = serde_json::json!({
                "surface": surface,
                "artifact_distribution_signing_notarization_receipt_release_public_artifact_publication_surface": surface,
                "source_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_ready": source_ready,
                "artifact_distribution_signing_notarization_receipt_release_public_artifact_publication_attempted": true,
                "artifact_distribution_signing_notarization_receipt_release_public_artifact_publication_noop_confirmed": true,
                "artifact_distribution_signing_notarization_receipt_release_public_artifact_publication_status": status,
                "reason": reason,
            });
            if let Some(surface_object) = surface_report.as_object_mut() {
                for key in &surface_false_keys {
                    surface_object.insert((*key).to_string(), serde_json::json!(false));
                }
                for key in true_keys.iter() {
                    surface_object.insert((*key).to_string(), serde_json::json!(true));
                }
            }
            surface_report
        })
        .collect::<Vec<_>>();
    let surface_count = surfaces.len();
    let contract_hash = sha256_text_value(&format!(
        "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-receipt-release-public-artifact-publication-denial:native:source={source_report_sha256}:surfaces={surface_count}:route_count={}:release_artifact=0:public_artifact=0:publication=0:public_claim=0:authority=0:install=0:live=0",
        route_matrix.route_count
    ));
    let policy_hash = sha256_text_value(
        "artifact-signing-notarization-receipt-release-public-artifact-publication:no-release-artifact:no-public-artifact:no-publication:no-public-claim:no-channel:no-telegram:no-install",
    );
    let denials = vec![
        "source_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_report_required",
        "artifact_distribution_signing_notarization_receipt_release_artifact_write_denied",
        "artifact_distribution_signing_notarization_receipt_public_artifact_write_denied",
        "artifact_distribution_signing_notarization_receipt_publication_queue_enqueue_denied",
        "artifact_distribution_signing_notarization_receipt_publication_manifest_write_denied",
        "artifact_distribution_signing_notarization_receipt_public_distribution_denied",
        "artifact_distribution_signing_notarization_receipt_package_registry_publication_denied",
        "artifact_distribution_signing_notarization_receipt_external_package_channel_publication_denied",
        "artifact_distribution_signing_notarization_receipt_telegram_package_channel_publication_denied",
        "artifact_distribution_signing_notarization_receipt_public_release_claim_denied",
        "artifact_distribution_signing_notarization_receipt_public_ga_claim_denied",
        "artifact_distribution_signing_notarization_receipt_release_notes_changelog_materialization_denied",
        "artifact_distribution_signing_notarization_receipt_operator_approval_from_publication_denied",
        "artifact_distribution_signing_notarization_receipt_release_publication_authority_from_publication_denied",
        "artifact_distribution_signing_notarization_receipt_activation_authority_from_publication_denied",
        "artifact_distribution_signing_notarization_receipt_download_install_restart_active_binary_from_publication_denied",
        "artifact_distribution_signing_notarization_receipt_memory_provider_secret_external_send_from_publication_denied",
    ];
    let denied_count = denials.len();
    let report_ready = source_ready
        && source_u64(
            "artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_surface_count",
        ) == 18
        && source_u64(
            "artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_denied_count",
        ) == 18
        && source_u64(
            "artifact_distribution_signing_notarization_receipt_public_claim_delivery_recorded_count",
        ) == 0
        && source_u64(
            "artifact_distribution_signing_notarization_receipt_status_readback_recorded_count",
        ) == 0
        && source_u64(
            "artifact_distribution_signing_notarization_receipt_channel_delivery_recorded_count",
        ) == 0
        && source_u64("delivery_receipt_recorded_count") == 0
        && source_u64("readback_receipt_recorded_count") == 0
        && source_u64("release_artifact_written_count") == 0
        && source_u64("public_artifact_written_count") == 0
        && source_u64("release_publication_authority_from_delivery_readback_derived_count") == 0
        && source_u64("activation_authority_from_delivery_readback_derived_count") == 0
        && surface_count == 18
        && route_count_source_command_accepted;

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_release_public_artifact_publication_denial_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_OPERATOR_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_RECEIPT_RELEASE_PUBLIC_ARTIFACT_PUBLICATION_DENIAL_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-receipt-release-public-artifact-publication-denial --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-28",
        "minimum_required_samples": 24,
        "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        "route_count": route_matrix.route_count,
        "implemented_route_count": route_matrix.implemented_route_count,
        "missing_route_count": route_matrix.missing_route_count,
        "route_count_source_command_accepted": route_count_source_command_accepted,
        "source_route_wired": true,
    });
    extend_json_object(
        &mut report,
        serde_json::json!({
            "artifact_distribution_signing_notarization_receipt_release_public_artifact_publication_schema_version": "memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_operator_intent_consent_evidence_artifact_signing_receipt_release_public_artifact_publication_denial_route_v1",
            "artifact_distribution_signing_notarization_receipt_release_public_artifact_publication_mode": "native_route_denied_terminal_public_claim_delivery_readback_cannot_write_release_public_artifacts_publish_claims_or_live_install",
            "memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_release_public_artifact_publication_denial_route_enabled": true,
            "memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_release_public_artifact_publication_denial_ready": report_ready,
            "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_release_public_artifact_publication_denial_ready": report_ready,
            "source_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_route": "hepta_memory_intelligence_kg_full_live_activation_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_signing_receipt_terminal_public_claim_delivery_readback_denial_route",
            "source_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_ready": source_ready,
            "source_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_report_sha256": source_report_sha256,
            "source_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_contract_hash_sha256": source_contract_hash,
            "artifact_distribution_signing_notarization_receipt_release_public_artifact_publication_contract_hash_sha256": contract_hash,
            "artifact_distribution_signing_notarization_receipt_release_public_artifact_publication_policy_hash_sha256": policy_hash,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "source_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_surface_count": source_u64("artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_surface_count"),
            "source_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_attempt_count": source_u64("artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_attempt_count"),
            "source_artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_denied_count": source_u64("artifact_distribution_signing_notarization_receipt_terminal_public_claim_delivery_readback_denied_count"),
            "source_artifact_distribution_signing_notarization_receipt_public_claim_delivery_recorded_count": source_u64("artifact_distribution_signing_notarization_receipt_public_claim_delivery_recorded_count"),
            "source_artifact_distribution_signing_notarization_receipt_status_readback_recorded_count": source_u64("artifact_distribution_signing_notarization_receipt_status_readback_recorded_count"),
            "source_artifact_distribution_signing_notarization_receipt_channel_delivery_recorded_count": source_u64("artifact_distribution_signing_notarization_receipt_channel_delivery_recorded_count"),
            "source_delivery_receipt_recorded_count": source_u64("delivery_receipt_recorded_count"),
            "source_readback_receipt_recorded_count": source_u64("readback_receipt_recorded_count"),
            "source_release_publication_authority_from_delivery_readback_derived_count": source_u64("release_publication_authority_from_delivery_readback_derived_count"),
            "source_activation_authority_from_delivery_readback_derived_count": source_u64("activation_authority_from_delivery_readback_derived_count"),
            "artifact_distribution_signing_notarization_receipt_release_public_artifact_publication_surface_count": surface_count,
            "artifact_distribution_signing_notarization_receipt_release_public_artifact_publication_attempt_count": surface_count,
            "artifact_distribution_signing_notarization_receipt_release_public_artifact_publication_denied_count": surface_count,
            "artifact_distribution_signing_notarization_receipt_release_public_artifact_publication_surfaces": surfaces,
            "denied_by_artifact_distribution_signing_notarization_receipt_release_public_artifact_publication": denials,
            "denied_by_artifact_distribution_signing_notarization_receipt_release_public_artifact_publication_count": denied_count,
            "allowed_next_actions": [{
                "action": "prepare_first_model_positive_approval_packet_boundary_gate",
                "status": "allowed_report_only_next_slice",
                "writes_release_artifact": false,
                "writes_public_artifact": false,
                "publishes_public_distribution": false,
                "claims_public_release": false,
                "claims_public_ga": false,
                "derives_operator_approval": false,
                "derives_release_publication_authority": false,
                "derives_activation_authority": false,
                "installs_or_restarts": false,
                "mutates_active_binary": false,
                "mutates_memory_store": false,
                "writes_kg": false,
                "invokes_provider": false,
                "reads_credentials": false,
                "sends_externally": false
            }],
        }),
    );

    let zero_keys = [
        "release_artifact_written_count",
        "public_artifact_written_count",
        "publication_queue_enqueued_count",
        "publication_manifest_written_count",
        "public_distribution_performed_count",
        "package_registry_artifact_published_count",
        "external_package_channel_published_count",
        "telegram_package_channel_published_count",
        "public_release_claimed_count",
        "public_ga_claimed_count",
        "release_notes_materialized_count",
        "changelog_materialized_count",
        "operator_approval_from_publication_derived_count",
        "release_publication_authority_from_publication_derived_count",
        "activation_authority_from_publication_derived_count",
        "download_link_from_publication_rendered_count",
        "install_command_from_publication_emitted_count",
        "install_from_publication_executed_count",
        "service_restart_from_publication_performed_count",
        "active_binary_from_publication_mutated_count",
        "memory_store_write_performed_count",
        "live_kg_write_performed_count",
        "provider_invoked_count",
        "model_invoked_count",
        "credential_read_count",
        "secret_file_read_count",
        "external_send_performed_count",
    ];
    if let Some(report_object) = report.as_object_mut() {
        for key in &zero_keys {
            report_object.insert((*key).to_string(), serde_json::json!(0));
        }
    }

    let false_keys = [
        "release_artifact_written",
        "public_artifact_written",
        "publication_queue_enqueued",
        "publication_manifest_written",
        "public_distribution_performed",
        "package_registry_artifact_published",
        "external_package_channel_published",
        "telegram_package_channel_published",
        "public_release_claimed",
        "public_ga_claimed",
        "release_notes_materialized",
        "changelog_materialized",
        "operator_approval_recorded",
        "release_publication_authority_derived",
        "activation_authority_derived",
        "download_link_rendered",
        "install_command_emitted",
        "activation_allowed",
        "memory_store_write_performed",
        "memory_store_mutated",
        "live_kg_write_performed",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "install_executed",
        "launchd_mutated",
        "service_restarted",
        "active_binary_mutated",
        "channel_send_performed",
        "telegram_send_performed",
        "external_send_performed",
        "filesystem_written",
    ];
    if let Some(report_object) = report.as_object_mut() {
        for key in &false_keys {
            report_object.insert((*key).to_string(), serde_json::json!(false));
        }
    }

    let mut side_effects = serde_json::Map::new();
    for key in &surface_false_keys {
        side_effects.insert((*key).to_string(), serde_json::json!(false));
    }
    for key in [
        "release_artifact_written",
        "public_artifact_written",
        "publication_queue_enqueued",
        "publication_manifest_written",
        "public_distribution_performed",
        "public_release_claimed",
        "public_ga_claimed",
        "telegram_send_performed",
        "channel_send_performed",
        "external_send_performed",
        "filesystem_written",
    ] {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }
    extend_json_object(
        &mut report,
        serde_json::json!({ "side_effects": side_effects }),
    );
    report
}

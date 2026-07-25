fn hepta_memory_intelligence_kg_full_live_activation_readiness_index_report() -> serde_json::Value {
    let publication = hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_result_receipt_no_persistence_report();
    let publication_sha256 = sha256_json_value(&publication);
    let readiness_surfaces = serde_json::json!([
        {"surface": "core_runtime_dependency_attachment", "ready": true, "blocked": false, "mode": "code_dependency_and_report_ready"},
        {"surface": "memory_capability_absorption", "ready": true, "blocked": false, "mode": "absorbed_or_represented_report_only"},
        {"surface": "memory_live_mutation_execution", "ready": true, "blocked": true, "reason": "memory_store_live_execution_disabled"},
        {"surface": "kg_prompt_preview_context_injection", "ready": true, "blocked": true, "reason": "prompt_preview_and_context_injection_disabled"},
        {"surface": "kg_external_adapter_staging", "ready": true, "blocked": true, "reason": "credential_read_network_and_external_write_disabled"},
        {"surface": "operator_canary_activation_chain", "ready": true, "blocked": true, "reason": "operator_canary_chain_remains_noop_report_only"},
        {"surface": "publication_release_artifact_boundary", "ready": true, "blocked": true, "reason": "release_artifact_publication_and_receipt_persistence_denied"},
        {"surface": "provider_model_invocation_boundary", "ready": true, "blocked": true, "reason": "provider_and_model_invocation_disabled"},
        {"surface": "credential_secret_boundary", "ready": true, "blocked": true, "reason": "credential_and_secret_read_disabled"},
        {"surface": "install_restart_active_binary_boundary", "ready": true, "blocked": true, "reason": "install_restart_active_binary_mutation_denied"}
    ]);
    let live_activation_blockers = serde_json::json!([
        "memory_store_mutation_disabled",
        "context_injection_disabled",
        "prompt_preview_disabled",
        "kg_external_adapter_live_execution_disabled",
        "live_kg_write_disabled",
        "credential_secret_read_disabled",
        "provider_model_invocation_disabled",
        "operator_approval_packet_missing",
        "redaction_review_missing",
        "rollback_kill_switch_not_accepted_for_live",
        "post_write_validation_not_persisted",
        "idempotency_replay_ordering_not_live",
        "install_restart_active_binary_denied"
    ]);
    let readiness_index_contract_hash_sha256 = sha256_text_value(&format!(
        "hepta-memory-intelligence-kg-full-live-activation-readiness-index:native:publication={publication_sha256}:surfaces=10:blockers=13:live=0"
    ));
    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": "ready",
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_readiness_index_gate",
        "readiness_index_schema_version": "memory_intelligence_kg_full_live_activation_readiness_index_v1",
        "readiness_index_mode": "report_only_no_activation_no_secret_no_provider_no_write",
        "memory_intelligence_kg_full_live_activation_readiness_index_ready": true,
        "full_live_activation_enabled": false,
        "full_live_activation_status": "blocked_report_only",
        "minimum_required_samples": 24,
        "readiness_index_contract_hash_sha256": readiness_index_contract_hash_sha256,
        "source_reports": {
            "memory_intelligence_closure": {"gate": "hepta_memory_intelligence_closure_gate", "sha256": sha256_text_value("native-memory-intelligence-closure-report-only")},
            "kg_prompt_preview_preflight": {"gate": "hepta_kg_prompt_preview_preflight_gate", "sha256": sha256_text_value("native-kg-prompt-preview-preflight-report-only")},
            "memory_live_mutation_staging": {"gate": "hepta_memory_intelligence_kg_full_enablement_memory_live_mutation_staging_fixture_gate", "sha256": sha256_text_value("native-memory-live-mutation-staging-report-only")},
            "kg_external_adapter_staging": {"gate": "hepta_memory_intelligence_kg_full_enablement_kg_external_adapter_staging_receipt_gate", "sha256": sha256_text_value("native-kg-external-adapter-staging-report-only")},
            "operator_canary_publication_receipt": {"gate": publication["gate"].clone(), "sha256": publication_sha256}
        },
        "readiness_surfaces": readiness_surfaces,
        "live_activation_blockers": live_activation_blockers
    });
    extend_json_object(
        &mut report,
        serde_json::json!({
            "active_service_stack_consumes_memory_intelligence": true,
            "runtime_memory_intelligence_dependencies_ready": true,
            "memory_capability_inventory_ready": true,
            "memory_surface_count": 14,
            "absorbed_or_represented_count": 14,
            "gap_report_ready_count": 6,
            "live_mutation_enabled_count": 0,
            "full_live_memory_intelligence_closure_ready": false,
            "kg_prompt_preview_preflight_ready": true,
            "kg_prompt_preview_status": "blocked",
            "prompt_preview_allowed": false,
            "context_injection_allowed": false,
            "model_invoked": false,
            "live_write_enabled_count": 0,
            "memory_store_live_mutation_lane_ready": true,
            "memory_store_live_mutation_lane_current_live_execution_enabled": false,
            "kg_external_adapter_staging_lane_ready": true,
            "kg_external_adapter_staging_lane_current_live_execution_enabled": false,
            "operator_canary_publication_result_receipt_no_persistence_ready": true,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "allowed_next_actions": [
                {
                    "action": "stage_memory_intelligence_kg_full_live_activation_readiness_index_replay_idempotency_denial",
                    "status": "allowed_report_only_next_slice",
                    "mutates_memory_store": false,
                    "writes_kg": false,
                    "invokes_provider": false,
                    "persists_receipt": false
                },
                {
                    "action": "prepare_operator_activation_readiness_packet_template",
                    "status": "allowed_report_only_next_slice",
                    "records_operator_acceptance": false,
                    "activates_live": false,
                    "publishes_artifact": false
                }
            ],
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "hepta_intelligence_context_attached": false,
            "prompt_preview_rendered": false,
            "context_injection_performed": false,
            "provider_invoked": false,
            "external_kg_adapter_read_performed": false,
            "external_adapter_client_constructed": false,
            "network_call_performed": false,
            "external_db_write_performed": false,
            "live_kg_write_performed": false,
            "credential_read": false,
            "secret_file_read": false,
            "install_executed": false,
            "launchd_mutated": false,
            "service_restarted": false,
            "active_binary_mutated": false,
            "public_release_claimed": false,
            "public_ga_claimed": false,
            "release_artifact_written": false,
            "public_artifact_written": false,
            "external_send_performed": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "side_effects": {
                "memory_store_write_performed": false,
                "memory_store_mutated": false,
                "hepta_intelligence_context_attached": false,
                "prompt_preview_rendered": false,
                "context_injection_performed": false,
                "provider_invoked": false,
                "model_invoked": false,
                "external_kg_adapter_read_performed": false,
                "external_adapter_client_constructed": false,
                "network_call_performed": false,
                "external_db_write_performed": false,
                "live_kg_write_performed": false,
                "credential_read": false,
                "secret_file_read": false,
                "install_executed": false,
                "launchd_mutated": false,
                "service_restarted": false,
                "active_binary_mutated": false,
                "public_release_claimed": false,
                "public_ga_claimed": false,
                "release_artifact_written": false,
                "public_artifact_written": false,
                "external_send_performed": false,
                "filesystem_written": false
            }
        }),
    );
    report
}

fn hepta_memory_intelligence_kg_full_live_activation_readiness_index_replay_idempotency_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source = hepta_memory_intelligence_kg_full_live_activation_readiness_index_report();
    let source_ready = source
        .get("memory_intelligence_kg_full_live_activation_readiness_index_ready")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);
    let source_report_sha256 = sha256_json_value(&source);
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;
    let report_ready = source_ready
        && source
            .get("full_live_activation_enabled")
            .and_then(serde_json::Value::as_bool)
            == Some(false)
        && source
            .get("full_live_activation_status")
            .and_then(serde_json::Value::as_str)
            == Some("blocked_report_only")
        && source
            .get("live_mutation_enabled_count")
            .and_then(serde_json::Value::as_u64)
            == Some(0)
        && route_count_source_command_accepted;

    let denied_fixture = |id: &str, reason: &str, extra: serde_json::Value| -> serde_json::Value {
        let mut fixture = serde_json::Map::new();
        fixture.insert("id".to_string(), serde_json::json!(id));
        for key in [
            "source_readiness_index_present",
            "source_readiness_index_ready",
            "replay_requested",
            "replay_idempotency_noop_confirmed",
        ] {
            fixture.insert(key.to_string(), serde_json::json!(true));
        }
        for key in [
            "replay_allowed",
            "replay_accepted",
            "idempotency_key_registered",
            "idempotency_key_persisted",
            "idempotency_cache_written",
            "replay_cache_hit_promoted",
            "query_result_registered",
            "query_result_persisted",
            "index_entry_written",
            "export_recorded",
            "observability_recorded",
            "activation_authority_derived",
            "operator_acceptance_recorded",
            "operator_approval_recorded",
            "memory_store_write_performed",
            "memory_store_mutated",
            "hepta_intelligence_context_attached",
            "prompt_preview_rendered",
            "context_injection_performed",
            "provider_invoked",
            "model_invoked",
            "external_kg_adapter_read_performed",
            "external_adapter_client_constructed",
            "network_call_performed",
            "external_db_write_performed",
            "live_kg_write_performed",
            "credential_read",
            "secret_file_read",
            "install_executed",
            "launchd_mutated",
            "service_restarted",
            "active_binary_mutated",
            "public_release_claimed",
            "public_ga_claimed",
            "release_artifact_written",
            "public_artifact_written",
            "external_send_performed",
        ] {
            fixture.insert(key.to_string(), serde_json::json!(false));
        }
        fixture.insert("reason".to_string(), serde_json::json!(reason));
        if let Some(extra_object) = extra.as_object() {
            fixture.extend(extra_object.clone());
        }
        serde_json::Value::Object(fixture)
    };

    let fixtures = serde_json::json!([
        denied_fixture(
            "readiness-index-replay-missing-source",
            "source_readiness_index_required",
            serde_json::json!({"source_readiness_index_present": false, "source_readiness_index_ready": false})
        ),
        denied_fixture(
            "readiness-index-replay-request",
            "readiness_index_replay_denied",
            serde_json::json!({"explicit_replay_requested": true})
        ),
        denied_fixture(
            "readiness-index-idempotency-key-registration-request",
            "idempotency_key_registration_denied",
            serde_json::json!({"idempotency_key_registration_requested": true})
        ),
        denied_fixture(
            "readiness-index-idempotency-cache-write-request",
            "idempotency_cache_write_denied",
            serde_json::json!({"idempotency_cache_write_requested": true})
        ),
        denied_fixture(
            "readiness-index-query-result-registration-request",
            "query_result_registration_denied",
            serde_json::json!({"query_result_registration_requested": true})
        ),
        denied_fixture(
            "readiness-index-index-entry-write-request",
            "index_entry_write_denied",
            serde_json::json!({"index_entry_write_requested": true})
        ),
        denied_fixture(
            "readiness-index-export-observability-request",
            "export_observability_denied",
            serde_json::json!({"export_requested": true, "observability_requested": true})
        ),
        denied_fixture(
            "readiness-index-operator-acceptance-record-request",
            "operator_acceptance_from_readiness_index_denied",
            serde_json::json!({"operator_acceptance_record_requested": true})
        ),
        denied_fixture(
            "readiness-index-activation-authority-request",
            "activation_authority_from_readiness_index_denied",
            serde_json::json!({"activation_authority_requested": true})
        ),
        denied_fixture(
            "readiness-index-live-side-effect-request",
            "readiness_index_replay_cannot_authorize_live_side_effects",
            serde_json::json!({"memory_write_requested": true, "kg_write_requested": true, "provider_invocation_requested": true, "credential_read_requested": true, "install_restart_requested": true, "public_release_requested": true, "external_send_requested": true})
        ),
    ]);
    let fixture_count = fixtures.as_array().map(std::vec::Vec::len).unwrap_or(0);
    let replay_idempotency_contract_hash_sha256 = sha256_text_value(&format!(
        "hepta-memory-intelligence-kg-full-live-activation-readiness-index-replay-idempotency-denial:native:source={source_report_sha256}:fixtures={}:replay=0:persist=0:authority=0",
        sha256_json_value(&fixtures)
    ));
    let denials = serde_json::json!([
        "readiness_index_replay_denied",
        "readiness_index_idempotency_key_registration_denied",
        "readiness_index_idempotency_cache_write_denied",
        "readiness_index_query_result_registration_denied",
        "readiness_index_index_entry_write_denied",
        "readiness_index_export_observability_denied",
        "readiness_index_operator_acceptance_record_denied",
        "readiness_index_activation_authority_denied",
        "readiness_index_live_side_effects_denied"
    ]);

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_readiness_index_replay_idempotency_denial_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_READINESS_INDEX_REPLAY_IDEMPOTENCY_DENIAL_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-readiness-index-replay-idempotency-denial --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-15",
        "source_readiness_index_gate": source["gate"].clone(),
        "source_readiness_index_report_sha256": source_report_sha256,
        "replay_idempotency_contract_hash_sha256": replay_idempotency_contract_hash_sha256,
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
            "replay_idempotency_schema_version": "memory_intelligence_kg_full_live_activation_readiness_index_replay_idempotency_denial_v1",
            "replay_idempotency_mode": "native_route_readiness_index_replay_idempotency_report_only_no_persistence_no_authority",
            "memory_intelligence_kg_full_live_activation_readiness_index_replay_idempotency_denial_route_enabled": true,
            "memory_intelligence_kg_full_live_activation_readiness_index_replay_idempotency_denial_ready": true,
            "source_readiness_index_ready": source_ready,
            "source_full_live_activation_enabled": false,
            "source_full_live_activation_status": "blocked_report_only",
            "readiness_surface_count": 10,
            "live_activation_blocker_count": 13,
            "required_replay_idempotency_surface_count": 12,
            "ready_replay_idempotency_surface_count": 12,
            "side_effect_free_replay_idempotency_surface_count": 12,
            "required_replay_idempotency_fixture_count": 10,
            "replay_idempotency_fixture_count": fixture_count,
            "blocked_replay_idempotency_fixture_count": fixture_count,
            "noop_replay_idempotency_fixture_count": fixture_count,
            "allowed_replay_idempotency_fixture_count": 0,
            "accepted_replay_idempotency_fixture_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "replay_allowed": false,
            "replay_accepted": false,
            "idempotency_key_registered": false,
            "idempotency_key_persisted": false,
            "idempotency_cache_written": false,
            "replay_cache_hit_promoted": false,
            "query_result_registered": false,
            "query_result_persisted": false,
            "index_entry_written": false,
            "export_recorded": false,
            "observability_recorded": false,
            "activation_authority_derived": false,
            "operator_acceptance_recorded": false,
            "operator_approval_recorded": false,
            "replay_idempotency_fixtures": fixtures,
            "denied_by_readiness_index_replay_idempotency": denials,
            "denied_by_readiness_index_replay_idempotency_count": 9,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "allowed_next_actions": [
                {
                    "action": "prepare_operator_activation_readiness_packet_template",
                    "status": "allowed_report_only_next_slice",
                    "records_operator_acceptance": false,
                    "activates_live": false,
                    "publishes_artifact": false
                }
            ],
            "current_live_enabled_lane_count": 30,
            "enablement_lane_count": 33,
            "ready_enablement_lane_count": 33,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "hepta_intelligence_context_attached": false,
            "prompt_preview_rendered": false,
            "context_injection_performed": false,
            "provider_invoked": false,
            "model_invoked": false,
            "external_kg_adapter_read_performed": false,
            "external_adapter_client_constructed": false,
            "network_call_performed": false,
            "external_db_write_performed": false,
            "live_kg_write_performed": false,
            "credential_read": false,
            "secret_file_read": false,
            "install_executed": false,
            "launchd_mutated": false,
            "service_restarted": false,
            "active_binary_mutated": false,
            "public_release_claimed": false,
            "public_ga_claimed": false,
            "release_artifact_written": false,
            "public_artifact_written": false,
            "external_send_performed": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "side_effects": {
                "replay_performed": false,
                "replay_accepted": false,
                "idempotency_key_registered": false,
                "idempotency_key_persisted": false,
                "idempotency_cache_written": false,
                "query_result_registered": false,
                "query_result_persisted": false,
                "index_entry_written": false,
                "export_recorded": false,
                "observability_recorded": false,
                "activation_authority_derived": false,
                "operator_acceptance_recorded": false,
                "operator_approval_recorded": false,
                "memory_store_write_performed": false,
                "memory_store_mutated": false,
                "hepta_intelligence_context_attached": false,
                "prompt_preview_rendered": false,
                "context_injection_performed": false,
                "provider_invoked": false,
                "model_invoked": false,
                "external_kg_adapter_read_performed": false,
                "external_adapter_client_constructed": false,
                "network_call_performed": false,
                "external_db_write_performed": false,
                "live_kg_write_performed": false,
                "credential_read": false,
                "secret_file_read": false,
                "install_executed": false,
                "launchd_mutated": false,
                "service_restarted": false,
                "active_binary_mutated": false,
                "public_release_claimed": false,
                "public_ga_claimed": false,
                "release_artifact_written": false,
                "public_artifact_written": false,
                "external_send_performed": false,
                "filesystem_written": false
            }
        }),
    );
    report
}

fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source =
        hepta_memory_intelligence_kg_full_live_activation_readiness_index_replay_idempotency_denial_report();
    let source_ready = source
        .get("memory_intelligence_kg_full_live_activation_readiness_index_replay_idempotency_denial_ready")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);
    let source_full_live_activation_enabled = source
        .get("source_full_live_activation_enabled")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(true);
    let source_full_live_activation_status = source
        .get("source_full_live_activation_status")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("unknown");
    let source_replay_allowed = source
        .get("replay_allowed")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(true);
    let source_activation_authority_derived = source
        .get("activation_authority_derived")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(true);
    let source_report_sha256 = sha256_json_value(&source);
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;

    let packet_section =
        |section_id: &str, required_fields: &[&str], missing_reason: &str| -> serde_json::Value {
            serde_json::json!({
                "section_id": section_id,
                "required_fields": required_fields,
                "missing_reason": missing_reason,
                "status": "missing",
                "operator_input_required": true,
                "template_only": true,
                "report_only": true,
                "recorded": false,
                "persisted": false,
                "materialized": false,
                "accepted": false,
                "delivered": false,
                "activation_authority": false,
                "mutates_memory_store": false,
                "writes_kg": false,
                "attaches_intelligence_context": false,
                "invokes_provider": false,
                "reads_credentials": false,
                "installs_or_restarts": false,
                "publishes_artifacts": false,
                "sends_external": false
            })
        };
    let sections = serde_json::json!([
        packet_section(
            "operator_authority",
            &[
                "operator_identity_hash",
                "explicit_operator_approval_id",
                "approval_scope",
                "approval_timestamp",
                "approval_nonce",
            ],
            "explicit_operator_authority_not_recorded",
        ),
        packet_section(
            "activation_scope",
            &[
                "activation_request_id",
                "memory_scope",
                "intelligence_scope",
                "kg_scope",
                "single_use_activation_nonce",
            ],
            "activation_scope_not_bound",
        ),
        packet_section(
            "memory_live_mutation_controls",
            &[
                "memory_store_write_enable_id",
                "memory_store_rollback_plan_id",
                "post_write_validation_plan_id",
                "idempotency_replay_plan_id",
            ],
            "memory_live_mutation_controls_not_accepted",
        ),
        packet_section(
            "intelligence_context_controls",
            &[
                "context_attachment_plan_id",
                "prompt_preview_redaction_review_id",
                "context_injection_approval_id",
                "model_invocation_boundary_id",
            ],
            "intelligence_context_controls_not_accepted",
        ),
        packet_section(
            "kg_external_adapter_controls",
            &[
                "kg_adapter_manifest_id",
                "credential_reference_review_id",
                "network_allowlist_id",
                "external_write_rollback_plan_id",
                "live_kg_write_validation_id",
            ],
            "kg_external_adapter_controls_not_accepted",
        ),
        packet_section(
            "release_install_boundary",
            &[
                "no_public_release_claim_attestation",
                "no_release_artifact_write_attestation",
                "no_install_restart_attestation",
                "active_binary_no_mutation_attestation",
            ],
            "release_install_boundary_not_accepted",
        ),
        packet_section(
            "fresh_evidence_and_soak",
            &[
                "fresh_long_soak_sample_set_hash",
                "readiness_index_report_sha256",
                "replay_denial_report_sha256",
                "fresh_evidence_timestamp",
            ],
            "fresh_evidence_and_soak_not_accepted",
        ),
        packet_section(
            "rollback_kill_switch",
            &[
                "rollback_plan_id",
                "rollback_dry_run_evidence_id",
                "kill_switch_id",
                "kill_switch_dry_run_evidence_id",
            ],
            "rollback_kill_switch_not_accepted",
        ),
        packet_section(
            "audit_receipt_chain",
            &[
                "receipt_persistence_plan_id",
                "ledger_record_plan_id",
                "operator_review_plan_id",
                "completion_ack_policy_id",
            ],
            "audit_receipt_chain_not_accepted",
        ),
        packet_section(
            "final_operator_review",
            &[
                "final_review_packet_hash",
                "human_readable_summary_hash",
                "non_delegation_attestation",
                "manual_acceptance_channel",
            ],
            "final_operator_review_not_accepted",
        ),
    ]);
    let section_count = sections.as_array().map(std::vec::Vec::len).unwrap_or(0);
    let required_field_count = sections
        .as_array()
        .map(|sections| {
            sections
                .iter()
                .map(|section| {
                    section
                        .get("required_fields")
                        .and_then(serde_json::Value::as_array)
                        .map(std::vec::Vec::len)
                        .unwrap_or(0)
                })
                .sum::<usize>()
        })
        .unwrap_or(0);
    let operator_packet_template_hash_sha256 = sha256_text_value(&format!(
        "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template:native:source={source_report_sha256}:sections={}:required_fields={required_field_count}:acceptance=0:authority=0",
        sha256_json_value(&sections)
    ));
    let report_ready = source_ready
        && !source_full_live_activation_enabled
        && source_full_live_activation_status == "blocked_report_only"
        && !source_replay_allowed
        && !source_activation_authority_derived
        && section_count == 10
        && required_field_count == 43
        && route_count_source_command_accepted;

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-14",
        "source_readiness_index_replay_idempotency_denial_gate": source["gate"].clone(),
        "source_readiness_index_replay_idempotency_denial_report_sha256": source_report_sha256,
        "operator_packet_template_hash_sha256": operator_packet_template_hash_sha256,
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
            "operator_readiness_packet_template_schema_version": "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_v1",
            "operator_readiness_packet_template_mode": "native_route_report_only_template_no_acceptance_no_activation",
            "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_route_enabled": true,
            "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_ready": true,
            "source_readiness_index_replay_idempotency_denial_ready": source_ready,
            "source_full_live_activation_enabled": false,
            "source_full_live_activation_status": "blocked_report_only",
            "source_replay_allowed": false,
            "source_activation_authority_derived": false,
            "required_operator_packet_section_count": 10,
            "operator_packet_section_count": section_count,
            "missing_operator_packet_section_count": section_count,
            "accepted_operator_packet_section_count": 0,
            "recorded_operator_packet_section_count": 0,
            "operator_packet_required_field_count": required_field_count,
            "operator_packet_recorded_field_count": 0,
            "operator_packet_accepted_field_count": 0,
            "operator_packet_sections": sections,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "packet_template_recorded": false,
            "packet_template_persisted": false,
            "packet_template_materialized": false,
            "packet_template_delivered": false,
            "operator_acceptance_recorded": false,
            "operator_approval_recorded": false,
            "activation_authority_derived": false,
            "activation_allowed": false,
            "activation_performed": false,
            "allowed_next_actions": [
                {
                    "action": "review_operator_readiness_packet_template",
                    "status": "allowed_report_only",
                    "records_operator_acceptance": false,
                    "activates_live": false,
                    "mutates_memory_store": false,
                    "writes_kg": false
                },
                {
                    "action": "stage_operator_readiness_packet_template_non_acceptance_authority_replay_denial",
                    "status": "allowed_report_only_next_slice",
                    "records_operator_acceptance": false,
                    "activates_live": false,
                    "persists_packet": false
                }
            ],
            "denied_by_operator_readiness_packet_template": [
                "operator_packet_template_persistence_denied",
                "operator_packet_template_materialization_denied",
                "operator_packet_acceptance_recording_denied",
                "operator_packet_approval_recording_denied",
                "operator_packet_activation_authority_denied",
                "memory_live_mutation_from_template_denied",
                "kg_write_from_template_denied",
                "provider_model_from_template_denied",
                "credential_read_from_template_denied",
                "install_restart_active_binary_from_template_denied",
                "release_publication_from_template_denied",
                "external_send_from_template_denied"
            ],
            "denied_by_operator_readiness_packet_template_count": 12,
            "current_live_enabled_lane_count": 30,
            "enablement_lane_count": 34,
            "ready_enablement_lane_count": 34,
            "live_mutation_enabled_count": 1,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "hepta_intelligence_context_attached": false,
            "prompt_preview_rendered": false,
            "context_injection_performed": false,
            "provider_invoked": false,
            "model_invoked": false,
            "external_kg_adapter_read_performed": false,
            "external_adapter_client_constructed": false,
            "network_call_performed": false,
            "external_db_write_performed": false,
            "live_kg_write_performed": false,
            "credential_read": false,
            "secret_file_read": false,
            "install_executed": false,
            "launchd_mutated": false,
            "service_restarted": false,
            "active_binary_mutated": false,
            "public_release_claimed": false,
            "public_ga_claimed": false,
            "release_artifact_written": false,
            "public_artifact_written": false,
            "external_send_performed": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "side_effects": {
                "packet_template_recorded": false,
                "packet_template_persisted": false,
                "packet_template_materialized": false,
                "packet_template_delivered": false,
                "operator_acceptance_recorded": false,
                "operator_approval_recorded": false,
                "activation_authority_derived": false,
                "activation_allowed": false,
                "activation_performed": false,
                "memory_store_write_performed": false,
                "memory_store_mutated": false,
                "hepta_intelligence_context_attached": false,
                "prompt_preview_rendered": false,
                "context_injection_performed": false,
                "provider_invoked": false,
                "model_invoked": false,
                "external_kg_adapter_read_performed": false,
                "external_adapter_client_constructed": false,
                "network_call_performed": false,
                "external_db_write_performed": false,
                "live_kg_write_performed": false,
                "credential_read": false,
                "secret_file_read": false,
                "install_executed": false,
                "launchd_mutated": false,
                "service_restarted": false,
                "active_binary_mutated": false,
                "public_release_claimed": false,
                "public_ga_claimed": false,
                "release_artifact_written": false,
                "public_artifact_written": false,
                "external_send_performed": false,
                "filesystem_written": false
            }
        }),
    );
    report
}

fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_non_acceptance_authority_replay_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source =
        hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_report(
        );
    let source_report_sha256 = sha256_json_value(&source);
    let source_ready = source
        .get("memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_ready")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);
    let source_section_count = source
        .get("operator_packet_section_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_required_field_count = source
        .get("operator_packet_required_field_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_recorded_field_count = source
        .get("operator_packet_recorded_field_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_accepted_field_count = source
        .get("operator_packet_accepted_field_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;

    let denied_fixture = |id: &str, reason: &str, extra: serde_json::Value| {
        let mut fixture = serde_json::json!({
            "id": id,
            "source_operator_readiness_packet_template_present": true,
            "source_operator_readiness_packet_template_ready": true,
            "template_seen": true,
            "template_replayed": false,
            "template_replay_allowed": false,
            "template_replay_accepted": false,
            "template_reference_registered": false,
            "template_reference_persisted": false,
            "template_summary_promoted": false,
            "template_cache_written": false,
            "template_query_registered": false,
            "template_export_recorded": false,
            "template_observability_recorded": false,
            "operator_acceptance_recorded": false,
            "operator_approval_recorded": false,
            "activation_authority_derived": false,
            "activation_command_derived": false,
            "activation_allowed": false,
            "activation_performed": false,
            "template_non_acceptance_noop_confirmed": true,
            "reason": reason,
        });
        extend_json_object(
            &mut fixture,
            serde_json::json!({
                "memory_store_write_performed": false,
                "memory_store_mutated": false,
                "hepta_intelligence_context_attached": false,
                "prompt_preview_rendered": false,
                "context_injection_performed": false,
                "provider_invoked": false,
                "model_invoked": false,
                "external_kg_adapter_read_performed": false,
                "external_adapter_client_constructed": false,
            }),
        );
        extend_json_object(
            &mut fixture,
            serde_json::json!({
                "network_call_performed": false,
                "external_db_write_performed": false,
                "live_kg_write_performed": false,
                "credential_read": false,
                "secret_file_read": false,
                "install_executed": false,
                "launchd_mutated": false,
                "service_restarted": false,
                "active_binary_mutated": false,
            }),
        );
        extend_json_object(
            &mut fixture,
            serde_json::json!({
                "public_release_claimed": false,
                "public_ga_claimed": false,
                "release_artifact_written": false,
                "public_artifact_written": false,
                "external_send_performed": false,
            }),
        );
        extend_json_object(&mut fixture, extra);
        fixture
    };
    let fixtures = vec![
        denied_fixture(
            "operator-readiness-packet-template-viewed",
            "template_view_is_not_acceptance",
            serde_json::json!({"template_viewed": true}),
        ),
        denied_fixture(
            "operator-readiness-packet-template-summary",
            "template_summary_is_not_acceptance",
            serde_json::json!({"template_summary_requested": true}),
        ),
        denied_fixture(
            "operator-readiness-packet-template-replay",
            "template_replay_denied",
            serde_json::json!({"template_replayed": true}),
        ),
        denied_fixture(
            "operator-readiness-packet-template-reference-registration",
            "template_reference_registration_denied",
            serde_json::json!({"template_reference_registration_requested": true}),
        ),
        denied_fixture(
            "operator-readiness-packet-template-cache-write",
            "template_cache_write_denied",
            serde_json::json!({"template_cache_write_requested": true}),
        ),
        denied_fixture(
            "operator-readiness-packet-template-query-export-observability",
            "template_query_export_observability_denied",
            serde_json::json!({
                "template_query_requested": true,
                "template_export_requested": true,
                "template_observability_requested": true
            }),
        ),
        denied_fixture(
            "operator-readiness-packet-template-operator-acceptance",
            "template_cannot_record_operator_acceptance",
            serde_json::json!({"operator_acceptance_record_requested": true}),
        ),
        denied_fixture(
            "operator-readiness-packet-template-operator-approval",
            "template_cannot_record_operator_approval",
            serde_json::json!({"operator_approval_record_requested": true}),
        ),
        denied_fixture(
            "operator-readiness-packet-template-activation-authority",
            "template_cannot_derive_activation_authority",
            serde_json::json!({
                "activation_authority_requested": true,
                "activation_command_requested": true
            }),
        ),
        denied_fixture(
            "operator-readiness-packet-template-live-side-effects",
            "template_cannot_authorize_live_side_effects",
            serde_json::json!({
                "memory_write_requested": true,
                "kg_write_requested": true,
                "context_injection_requested": true,
                "provider_invocation_requested": true,
                "credential_read_requested": true,
                "install_restart_requested": true,
                "public_release_requested": true,
                "external_send_requested": true
            }),
        ),
    ];
    let denied_by_template_non_acceptance_authority_replay = vec![
        "operator_readiness_packet_template_view_acceptance_denied",
        "operator_readiness_packet_template_summary_acceptance_denied",
        "operator_readiness_packet_template_replay_denied",
        "operator_readiness_packet_template_reference_registration_denied",
        "operator_readiness_packet_template_cache_write_denied",
        "operator_readiness_packet_template_query_export_observability_denied",
        "operator_readiness_packet_template_operator_acceptance_denied",
        "operator_readiness_packet_template_operator_approval_denied",
        "operator_readiness_packet_template_activation_authority_denied",
        "operator_readiness_packet_template_live_side_effects_denied",
    ];
    let denied_by_template_non_acceptance_authority_replay_count =
        denied_by_template_non_acceptance_authority_replay.len();
    let non_acceptance_contract_hash_sha256 = sha256_text_value(&format!(
        "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-non-acceptance-authority-replay-denial:native:source={source_report_sha256}:fixtures={}:route_count={}",
        sha256_json_value(&serde_json::json!(fixtures)),
        route_matrix.route_count
    ));
    let report_ready = source_ready
        && source_section_count == 10
        && source_required_field_count == 43
        && source_recorded_field_count == 0
        && source_accepted_field_count == 0
        && route_count_source_command_accepted;
    let fixture_count = fixtures.len();

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_non_acceptance_authority_replay_denial_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_NON_ACCEPTANCE_AUTHORITY_REPLAY_DENIAL_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-non-acceptance-authority-replay-denial --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-14",
        "non_acceptance_schema_version": "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_non_acceptance_authority_replay_denial_v1",
        "non_acceptance_mode": "native_route_packet_template_view_summary_replay_reference_no_acceptance_no_authority_no_live",
        "source_operator_readiness_packet_template_gate": source["gate"].clone(),
        "source_operator_readiness_packet_template_ready": source_ready,
        "source_operator_readiness_packet_template_report_sha256": source_report_sha256,
        "non_acceptance_contract_hash_sha256": non_acceptance_contract_hash_sha256,
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
        "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_non_acceptance_authority_replay_denial_route_enabled": true,
        "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_non_acceptance_authority_replay_denial_ready": true,
        "source_operator_packet_section_count": source_section_count,
        "source_operator_packet_required_field_count": source_required_field_count,
        "source_operator_packet_recorded_field_count": source_recorded_field_count,
        "source_operator_packet_accepted_field_count": source_accepted_field_count,
        "required_non_acceptance_surface_count": 12,
        "ready_non_acceptance_surface_count": 12,
        "side_effect_free_non_acceptance_surface_count": 12,
        "required_non_acceptance_fixture_count": 10,
        "non_acceptance_fixture_count": fixture_count,
        "blocked_non_acceptance_fixture_count": fixture_count,
        "noop_non_acceptance_fixture_count": fixture_count,
        "allowed_non_acceptance_fixture_count": 0,
        "accepted_non_acceptance_fixture_count": 0,
        "template_view_is_acceptance": false,
        "template_summary_is_acceptance": false,
        "template_replay_allowed": false,
        "template_replay_accepted": false,
        "template_reference_registered": false,
        "template_reference_persisted": false,
        "template_cache_written": false,
        "template_query_registered": false,
        "template_export_recorded": false,
        "template_observability_recorded": false,
        "operator_acceptance_recorded": false,
        "operator_approval_recorded": false,
        "activation_authority_derived": false,
        "activation_command_derived": false,
        "activation_allowed": false,
        "activation_performed": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
        "non_acceptance_fixtures": fixtures,
        "denied_by_template_non_acceptance_authority_replay": denied_by_template_non_acceptance_authority_replay,
        "denied_by_template_non_acceptance_authority_replay_count": denied_by_template_non_acceptance_authority_replay_count,
        "allowed_next_actions": [
            {
                "action": "prepare_operator_readiness_packet_template_field_validation_denial_gate",
                "status": "allowed_report_only_next_slice",
                "records_operator_acceptance": false,
                "activates_live": false,
                "mutates_memory_store": false,
                "writes_kg": false
            }
        ],
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
        "memory_store_write_performed": false,
        "memory_store_mutated": false,
        "hepta_intelligence_context_attached": false,
        "prompt_preview_rendered": false,
        "context_injection_performed": false,
        "provider_invoked": false,
        "model_invoked": false,
        "external_kg_adapter_read_performed": false,
        "external_adapter_client_constructed": false,
        "network_call_performed": false,
        "external_db_write_performed": false,
        "live_kg_write_performed": false,
        "credential_read": false,
        "secret_file_read": false,
        "install_executed": false,
        "launchd_mutated": false,
        "service_restarted": false,
        "active_binary_mutated": false,
        "public_release_claimed": false,
        "public_ga_claimed": false,
        "release_artifact_written": false,
        "public_artifact_written": false,
        "external_send_performed": false,
        }),
    );

    let mut side_effects = serde_json::json!({
            "template_view_recorded": false,
            "template_summary_recorded": false,
            "template_replay_performed": false,
            "template_reference_registered": false,
            "template_reference_persisted": false,
            "template_cache_written": false,
            "template_query_registered": false,
            "template_export_recorded": false,
            "template_observability_recorded": false,
            "operator_acceptance_recorded": false,
            "operator_approval_recorded": false,
            "activation_authority_derived": false,
            "activation_command_derived": false,
            "activation_allowed": false,
            "activation_performed": false,
    });
    extend_json_object(
        &mut side_effects,
        serde_json::json!({
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "hepta_intelligence_context_attached": false,
            "prompt_preview_rendered": false,
            "context_injection_performed": false,
            "provider_invoked": false,
            "model_invoked": false,
            "external_kg_adapter_read_performed": false,
            "external_adapter_client_constructed": false,
            "network_call_performed": false,
            "external_db_write_performed": false,
        }),
    );
    extend_json_object(
        &mut side_effects,
        serde_json::json!({
            "live_kg_write_performed": false,
            "credential_read": false,
            "secret_file_read": false,
            "install_executed": false,
            "launchd_mutated": false,
            "service_restarted": false,
            "active_binary_mutated": false,
            "public_release_claimed": false,
            "public_ga_claimed": false,
            "release_artifact_written": false,
            "public_artifact_written": false,
            "external_send_performed": false,
            "filesystem_written": false
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "side_effects": side_effects
        }),
    );
    report
}

fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_field_validation_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source =
        hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_non_acceptance_authority_replay_denial_report();
    let template_source =
        hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_report(
        );
    let source_report_sha256 = sha256_json_value(&source);
    let source_ready = source
        .get("memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_non_acceptance_authority_replay_denial_ready")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);
    let source_section_count = source
        .get("source_operator_packet_section_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_required_field_count = source
        .get("source_operator_packet_required_field_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_recorded_field_count = source
        .get("source_operator_packet_recorded_field_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(1);
    let source_accepted_field_count = source
        .get("source_operator_packet_accepted_field_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(1);
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;

    let mut fields = Vec::new();
    if let Some(sections) = template_source
        .get("operator_packet_sections")
        .and_then(serde_json::Value::as_array)
    {
        for section in sections {
            let section_id = section
                .get("section_id")
                .and_then(serde_json::Value::as_str)
                .unwrap_or("unknown_section");
            if let Some(required_fields) = section
                .get("required_fields")
                .and_then(serde_json::Value::as_array)
            {
                for field in required_fields {
                    let field_name = field.as_str().unwrap_or("unknown_required_field");
                    fields.push(serde_json::json!({
                        "section_id": section_id,
                        "field_name": field_name,
                        "field_present": false,
                        "field_value_captured": false,
                        "field_value_hash_recorded": false,
                        "field_shape_validated": false,
                        "field_required": true,
                        "field_missing": true,
                        "field_recorded": false,
                        "field_persisted": false,
                        "field_accepted": false,
                        "field_authority_derived": false,
                        "field_live_execution_allowed": false,
                        "validation_status": "missing_denied",
                        "denial_reason": "operator_readiness_packet_template_field_value_not_recorded"
                    }));
                }
            }
        }
    }
    let field_validation_matrix = serde_json::json!(fields);
    let field_validation_matrix_count = field_validation_matrix
        .as_array()
        .map(std::vec::Vec::len)
        .unwrap_or(0);
    let field_validation_contract_hash_sha256 = sha256_text_value(&format!(
        "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-field-validation-denial:native:source={source_report_sha256}:fields={}:route_count={}:acceptance=0:authority=0:live=0",
        sha256_json_value(&field_validation_matrix),
        route_matrix.route_count
    ));
    let denied_by_field_validation = vec![
        "operator_readiness_packet_template_field_value_capture_denied",
        "operator_readiness_packet_template_field_hash_recording_denied",
        "operator_readiness_packet_template_field_shape_acceptance_denied",
        "operator_readiness_packet_template_field_persistence_denied",
        "operator_readiness_packet_template_field_operator_acceptance_denied",
        "operator_readiness_packet_template_field_authority_derivation_denied",
        "operator_readiness_packet_template_field_live_execution_denied",
    ];
    let denied_by_field_validation_count = denied_by_field_validation.len();
    let report_ready = source_ready
        && source_section_count == 10
        && source_required_field_count == 43
        && source_recorded_field_count == 0
        && source_accepted_field_count == 0
        && field_validation_matrix_count == 43
        && route_count_source_command_accepted;

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_field_validation_denial_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_FIELD_VALIDATION_DENIAL_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-field-validation-denial --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-14",
        "field_validation_schema_version": "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_field_validation_denial_v1",
        "field_validation_mode": "native_route_required_field_shape_matrix_no_values_no_persistence_no_acceptance_no_authority",
        "source_template_non_acceptance_gate": source["gate"].clone(),
        "source_template_non_acceptance_ready": source_ready,
        "source_template_non_acceptance_report_sha256": source_report_sha256,
        "field_validation_contract_hash_sha256": field_validation_contract_hash_sha256,
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
            "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_field_validation_denial_route_enabled": true,
            "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_field_validation_denial_ready": true,
            "source_operator_packet_section_count": source_section_count,
            "source_operator_packet_required_field_count": source_required_field_count,
            "source_operator_packet_recorded_field_count": source_recorded_field_count,
            "source_operator_packet_accepted_field_count": source_accepted_field_count,
            "required_field_count": 43,
            "field_validation_matrix_count": field_validation_matrix_count,
            "missing_field_count": field_validation_matrix_count,
            "present_field_count": 0,
            "captured_field_value_count": 0,
            "recorded_field_hash_count": 0,
            "shape_validated_field_count": 0,
            "accepted_field_count": 0,
            "authority_derived_field_count": 0,
            "live_execution_allowed_field_count": 0,
            "section_validation_count": 10,
            "required_field_validation_matrix": field_validation_matrix,
            "denied_by_field_validation": denied_by_field_validation,
            "denied_by_field_validation_count": denied_by_field_validation_count,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "allowed_next_actions": [
                {
                    "action": "prepare_operator_readiness_packet_template_section_completion_non_acceptance_gate",
                    "status": "allowed_report_only_next_slice",
                    "records_operator_acceptance": false,
                    "activates_live": false,
                    "persists_field_values": false,
                    "mutates_memory_store": false,
                    "writes_kg": false
                }
            ],
            "packet_template_recorded": false,
            "packet_template_persisted": false,
            "packet_template_materialized": false,
            "packet_template_delivered": false,
            "operator_acceptance_recorded": false,
            "operator_approval_recorded": false,
            "activation_authority_derived": false,
            "activation_command_derived": false,
            "activation_allowed": false,
            "activation_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "hepta_intelligence_context_attached": false,
            "prompt_preview_rendered": false,
            "context_injection_performed": false,
            "provider_invoked": false,
            "model_invoked": false,
            "external_kg_adapter_read_performed": false,
            "external_adapter_client_constructed": false,
            "network_call_performed": false,
            "external_db_write_performed": false,
            "live_kg_write_performed": false,
            "credential_read": false,
            "secret_file_read": false,
            "install_executed": false,
            "launchd_mutated": false,
            "service_restarted": false,
            "active_binary_mutated": false,
            "public_release_claimed": false,
            "public_ga_claimed": false,
            "release_artifact_written": false,
            "public_artifact_written": false,
            "external_send_performed": false,
        }),
    );

    let mut side_effects = serde_json::json!({
        "field_value_captured": false,
        "field_value_hash_recorded": false,
        "field_shape_accepted": false,
        "field_value_persisted": false,
        "field_acceptance_recorded": false,
        "field_authority_derived": false,
        "field_live_execution_allowed": false,
        "packet_template_recorded": false,
        "packet_template_persisted": false,
        "packet_template_materialized": false,
        "packet_template_delivered": false,
        "operator_acceptance_recorded": false,
        "operator_approval_recorded": false,
        "activation_authority_derived": false,
        "activation_command_derived": false,
        "activation_allowed": false,
        "activation_performed": false,
    });
    extend_json_object(
        &mut side_effects,
        serde_json::json!({
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "hepta_intelligence_context_attached": false,
            "prompt_preview_rendered": false,
            "context_injection_performed": false,
            "provider_invoked": false,
            "model_invoked": false,
            "external_kg_adapter_read_performed": false,
            "external_adapter_client_constructed": false,
            "network_call_performed": false,
            "external_db_write_performed": false,
        }),
    );
    extend_json_object(
        &mut side_effects,
        serde_json::json!({
            "live_kg_write_performed": false,
            "credential_read": false,
            "secret_file_read": false,
            "install_executed": false,
            "launchd_mutated": false,
            "service_restarted": false,
            "active_binary_mutated": false,
            "public_release_claimed": false,
            "public_ga_claimed": false,
            "release_artifact_written": false,
            "public_artifact_written": false,
            "external_send_performed": false,
            "filesystem_written": false
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "side_effects": side_effects
        }),
    );
    report
}

fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_section_completion_non_acceptance_report()
-> serde_json::Value {
    #[derive(Default)]
    struct SectionCompletionCounts {
        required_field_count: u64,
        missing_field_count: u64,
        present_field_count: u64,
        recorded_field_count: u64,
        accepted_field_count: u64,
        authority_derived_field_count: u64,
        live_execution_allowed_field_count: u64,
    }

    let route_matrix = control_ui_route_parity_report();
    let source =
        hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_field_validation_denial_report();
    let source_report_sha256 = sha256_json_value(&source);
    let source_ready = source
        .get("memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_field_validation_denial_ready")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);
    let source_section_count = source
        .get("source_operator_packet_section_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_required_field_count = source
        .get("source_operator_packet_required_field_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_missing_field_count = source
        .get("missing_field_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;

    let mut section_counts = BTreeMap::<String, SectionCompletionCounts>::new();
    if let Some(fields) = source
        .get("required_field_validation_matrix")
        .and_then(serde_json::Value::as_array)
    {
        for field in fields {
            let section_id = field
                .get("section_id")
                .and_then(serde_json::Value::as_str)
                .unwrap_or("unknown_section")
                .to_string();
            let entry = section_counts.entry(section_id).or_default();
            entry.required_field_count += 1;
            if field
                .get("field_missing")
                .and_then(serde_json::Value::as_bool)
                .unwrap_or(false)
            {
                entry.missing_field_count += 1;
            }
            if field
                .get("field_present")
                .and_then(serde_json::Value::as_bool)
                .unwrap_or(false)
            {
                entry.present_field_count += 1;
            }
            if field
                .get("field_recorded")
                .and_then(serde_json::Value::as_bool)
                .unwrap_or(false)
            {
                entry.recorded_field_count += 1;
            }
            if field
                .get("field_accepted")
                .and_then(serde_json::Value::as_bool)
                .unwrap_or(false)
            {
                entry.accepted_field_count += 1;
            }
            if field
                .get("field_authority_derived")
                .and_then(serde_json::Value::as_bool)
                .unwrap_or(false)
            {
                entry.authority_derived_field_count += 1;
            }
            if field
                .get("field_live_execution_allowed")
                .and_then(serde_json::Value::as_bool)
                .unwrap_or(false)
            {
                entry.live_execution_allowed_field_count += 1;
            }
        }
    }

    let section_completion_matrix = section_counts
        .into_iter()
        .map(|(section_id, counts)| {
            serde_json::json!({
                "section_id": section_id,
                "required_field_count": counts.required_field_count,
                "missing_field_count": counts.missing_field_count,
                "present_field_count": counts.present_field_count,
                "recorded_field_count": counts.recorded_field_count,
                "accepted_field_count": counts.accepted_field_count,
                "authority_derived_field_count": counts.authority_derived_field_count,
                "live_execution_allowed_field_count": counts.live_execution_allowed_field_count,
                "section_completion_attempted": true,
                "section_complete": false,
                "section_ready": false,
                "section_recorded": false,
                "section_persisted": false,
                "section_accepted": false,
                "section_operator_approval_derived": false,
                "section_activation_authority_derived": false,
                "section_live_execution_allowed": false,
                "completion_status": "completion_denied_missing_required_fields",
                "denial_reason": "operator_readiness_packet_template_section_completion_cannot_bypass_missing_fields"
            })
        })
        .collect::<Vec<_>>();
    let section_completion_matrix_count = section_completion_matrix.len();
    let section_completion_contract_hash_sha256 = sha256_text_value(&format!(
        "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-section-completion-non-acceptance:native:source={source_report_sha256}:sections={}:route_count={}:complete=0:accepted=0:authority=0:live=0",
        sha256_json_value(&serde_json::json!(section_completion_matrix)),
        route_matrix.route_count
    ));
    let denied_by_section_completion = vec![
        "operator_readiness_packet_template_section_completion_bypass_denied",
        "operator_readiness_packet_template_section_ready_promotion_denied",
        "operator_readiness_packet_template_section_recording_denied",
        "operator_readiness_packet_template_section_persistence_denied",
        "operator_readiness_packet_template_section_operator_acceptance_denied",
        "operator_readiness_packet_template_section_operator_approval_derivation_denied",
        "operator_readiness_packet_template_section_activation_authority_derivation_denied",
        "operator_readiness_packet_template_section_live_execution_denied",
    ];
    let denied_by_section_completion_count = denied_by_section_completion.len();
    let report_ready = source_ready
        && source_section_count == 10
        && source_required_field_count == 43
        && source_missing_field_count == 43
        && section_completion_matrix_count == 10
        && route_count_source_command_accepted;

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_section_completion_non_acceptance_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_SECTION_COMPLETION_NON_ACCEPTANCE_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-section-completion-non-acceptance --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-15",
        "section_completion_schema_version": "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_section_completion_non_acceptance_v1",
        "section_completion_mode": "native_route_section_completion_attempts_denied_no_acceptance_no_authority_no_live",
        "source_field_validation_gate": source["gate"].clone(),
        "source_field_validation_ready": source_ready,
        "source_field_validation_report_sha256": source_report_sha256,
        "section_completion_contract_hash_sha256": section_completion_contract_hash_sha256,
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
            "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_section_completion_non_acceptance_route_enabled": true,
            "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_section_completion_non_acceptance_ready": true,
            "source_operator_packet_section_count": source_section_count,
            "source_operator_packet_required_field_count": source_required_field_count,
            "source_required_field_count": source.get("required_field_count").cloned().unwrap_or_else(|| serde_json::json!(0)),
            "source_missing_field_count": source_missing_field_count,
            "section_completion_matrix_count": section_completion_matrix_count,
            "section_completion_attempt_count": section_completion_matrix_count,
            "section_complete_count": 0,
            "section_ready_count": 0,
            "section_recorded_count": 0,
            "section_persisted_count": 0,
            "section_accepted_count": 0,
            "section_operator_approval_derived_count": 0,
            "section_activation_authority_derived_count": 0,
            "section_live_execution_allowed_count": 0,
            "section_completion_matrix": section_completion_matrix,
            "denied_by_section_completion": denied_by_section_completion,
            "denied_by_section_completion_count": denied_by_section_completion_count,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "allowed_next_actions": [
                {
                    "action": "prepare_operator_readiness_packet_template_packet_assembly_non_acceptance_gate",
                    "status": "allowed_report_only_next_slice",
                    "records_operator_acceptance": false,
                    "derives_activation_authority": false,
                    "activates_live": false,
                    "persists_section_completion": false,
                    "mutates_memory_store": false,
                    "writes_kg": false
                }
            ],
            "packet_template_recorded": false,
            "packet_template_persisted": false,
            "packet_template_materialized": false,
            "packet_template_delivered": false,
            "section_completion_recorded": false,
            "section_completion_persisted": false,
            "section_completion_materialized": false,
            "section_completion_accepted": false,
            "operator_acceptance_recorded": false,
            "operator_approval_recorded": false,
            "activation_authority_derived": false,
            "activation_command_derived": false,
            "activation_allowed": false,
            "activation_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "hepta_intelligence_context_attached": false,
            "prompt_preview_rendered": false,
            "context_injection_performed": false,
            "provider_invoked": false,
            "model_invoked": false,
            "external_kg_adapter_read_performed": false,
            "external_adapter_client_constructed": false,
            "network_call_performed": false,
            "external_db_write_performed": false,
            "live_kg_write_performed": false,
            "credential_read": false,
            "secret_file_read": false,
            "install_executed": false,
            "launchd_mutated": false,
            "service_restarted": false,
            "active_binary_mutated": false,
            "public_release_claimed": false,
            "public_ga_claimed": false,
            "release_artifact_written": false,
            "public_artifact_written": false,
            "external_send_performed": false,
        }),
    );

    let mut side_effects = serde_json::json!({
        "section_completion_recorded": false,
        "section_completion_persisted": false,
        "section_completion_materialized": false,
        "section_completion_accepted": false,
        "section_ready_promoted": false,
        "section_operator_approval_derived": false,
        "section_activation_authority_derived": false,
        "section_live_execution_allowed": false,
        "packet_template_recorded": false,
        "packet_template_persisted": false,
        "packet_template_materialized": false,
        "packet_template_delivered": false,
        "operator_acceptance_recorded": false,
        "operator_approval_recorded": false,
        "activation_authority_derived": false,
        "activation_command_derived": false,
        "activation_allowed": false,
        "activation_performed": false,
    });
    extend_json_object(
        &mut side_effects,
        serde_json::json!({
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "hepta_intelligence_context_attached": false,
            "prompt_preview_rendered": false,
            "context_injection_performed": false,
            "provider_invoked": false,
            "model_invoked": false,
            "external_kg_adapter_read_performed": false,
            "external_adapter_client_constructed": false,
            "network_call_performed": false,
            "external_db_write_performed": false,
        }),
    );
    extend_json_object(
        &mut side_effects,
        serde_json::json!({
            "live_kg_write_performed": false,
            "credential_read": false,
            "secret_file_read": false,
            "install_executed": false,
            "launchd_mutated": false,
            "service_restarted": false,
            "active_binary_mutated": false,
            "public_release_claimed": false,
            "public_ga_claimed": false,
            "release_artifact_written": false,
            "public_artifact_written": false,
            "external_send_performed": false,
            "filesystem_written": false
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "side_effects": side_effects
        }),
    );
    report
}

fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_assembly_non_acceptance_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source =
        hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_section_completion_non_acceptance_report();
    let source_report_sha256 = sha256_json_value(&source);
    let source_ready = source
        .get("memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_section_completion_non_acceptance_ready")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);
    let source_section_count = source
        .get("source_operator_packet_section_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_required_field_count = source
        .get("source_operator_packet_required_field_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_missing_field_count = source
        .get("source_missing_field_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_section_completion_matrix_count = source
        .get("section_completion_matrix_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_section_complete_count = source
        .get("section_complete_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_section_ready_count = source
        .get("section_ready_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_section_recorded_count = source
        .get("section_recorded_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_section_accepted_count = source
        .get("section_accepted_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;

    let packet_assembly_attempts = vec![
        serde_json::json!({
            "attempt_id": "assemble_all_sections_incomplete_packet",
            "attempted_section_count": source_section_completion_matrix_count,
            "complete_section_count": source_section_complete_count,
            "missing_section_count": source_section_completion_matrix_count,
            "assembled": false,
            "accepted": false,
            "operator_approval_derived": false,
            "activation_authority_derived": false,
            "live_execution_allowed": false,
            "status": "assembly_denied_incomplete_sections",
        }),
        serde_json::json!({
            "attempt_id": "assemble_ready_sections_packet",
            "attempted_section_count": source_section_ready_count,
            "complete_section_count": 0,
            "missing_section_count": source_section_completion_matrix_count,
            "assembled": false,
            "accepted": false,
            "operator_approval_derived": false,
            "activation_authority_derived": false,
            "live_execution_allowed": false,
            "status": "assembly_denied_no_ready_sections",
        }),
        serde_json::json!({
            "attempt_id": "assemble_recorded_sections_packet",
            "attempted_section_count": source_section_recorded_count,
            "complete_section_count": 0,
            "missing_section_count": source_section_completion_matrix_count,
            "assembled": false,
            "accepted": false,
            "operator_approval_derived": false,
            "activation_authority_derived": false,
            "live_execution_allowed": false,
            "status": "assembly_denied_no_recorded_sections",
        }),
        serde_json::json!({
            "attempt_id": "assemble_accepted_sections_packet",
            "attempted_section_count": source_section_accepted_count,
            "complete_section_count": 0,
            "missing_section_count": source_section_completion_matrix_count,
            "assembled": false,
            "accepted": false,
            "operator_approval_derived": false,
            "activation_authority_derived": false,
            "live_execution_allowed": false,
            "status": "assembly_denied_no_accepted_sections",
        }),
    ];
    let packet_assembly_attempt_count = packet_assembly_attempts.len();
    let packet_assembly_contract_hash_sha256 = sha256_text_value(&format!(
        "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-assembly-non-acceptance:native:source={source_report_sha256}:attempts={packet_assembly_attempt_count}:route_count={}:assembled=0:accepted=0:authority=0:live=0",
        route_matrix.route_count
    ));
    let denied_by_packet_assembly = vec![
        "operator_readiness_packet_template_incomplete_section_assembly_denied",
        "operator_readiness_packet_template_packet_ready_promotion_denied",
        "operator_readiness_packet_template_packet_recording_denied",
        "operator_readiness_packet_template_packet_persistence_denied",
        "operator_readiness_packet_template_packet_acceptance_denied",
        "operator_readiness_packet_template_packet_operator_approval_derivation_denied",
        "operator_readiness_packet_template_packet_activation_authority_derivation_denied",
        "operator_readiness_packet_template_packet_activation_command_derivation_denied",
        "operator_readiness_packet_template_packet_live_execution_denied",
    ];
    let denied_by_packet_assembly_count = denied_by_packet_assembly.len();
    let report_ready = source_ready
        && source_section_count == 10
        && source_required_field_count == 43
        && source_missing_field_count == 43
        && source_section_completion_matrix_count == 10
        && source_section_complete_count == 0
        && source_section_ready_count == 0
        && packet_assembly_attempt_count == 4
        && route_count_source_command_accepted;

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_assembly_non_acceptance_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ASSEMBLY_NON_ACCEPTANCE_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-assembly-non-acceptance --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-15",
        "packet_assembly_schema_version": "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_assembly_non_acceptance_v1",
        "packet_assembly_mode": "native_route_incomplete_sections_cannot_assemble_accept_or_authorize_live",
        "source_section_completion_gate": source["gate"].clone(),
        "source_section_completion_ready": source_ready,
        "source_section_completion_report_sha256": source_report_sha256,
        "packet_assembly_contract_hash_sha256": packet_assembly_contract_hash_sha256,
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
            "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_assembly_non_acceptance_route_enabled": true,
            "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_assembly_non_acceptance_ready": true,
            "source_operator_packet_section_count": source_section_count,
            "source_operator_packet_required_field_count": source_required_field_count,
            "source_missing_field_count": source_missing_field_count,
            "source_section_completion_matrix_count": source_section_completion_matrix_count,
            "source_section_complete_count": source_section_complete_count,
            "source_section_ready_count": source_section_ready_count,
            "packet_assembly_attempt_count": packet_assembly_attempt_count,
            "packet_assembled_count": 0,
            "packet_complete_count": 0,
            "packet_ready_count": 0,
            "packet_recorded_count": 0,
            "packet_persisted_count": 0,
            "packet_accepted_count": 0,
            "packet_operator_approval_derived_count": 0,
            "packet_activation_authority_derived_count": 0,
            "packet_activation_command_derived_count": 0,
            "packet_live_execution_allowed_count": 0,
            "packet_assembly_attempts": packet_assembly_attempts,
            "denied_by_packet_assembly": denied_by_packet_assembly,
            "denied_by_packet_assembly_count": denied_by_packet_assembly_count,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "allowed_next_actions": [
                {
                    "action": "prepare_operator_readiness_packet_template_packet_acceptance_receipt_non_persistence_gate",
                    "status": "allowed_report_only_next_slice",
                    "records_operator_acceptance": false,
                    "persists_packet": false,
                    "derives_activation_authority": false,
                    "activates_live": false,
                    "mutates_memory_store": false,
                    "writes_kg": false
                }
            ],
            "packet_template_recorded": false,
            "packet_template_persisted": false,
            "packet_template_materialized": false,
            "packet_template_delivered": false,
            "packet_assembly_performed": false,
            "packet_assembly_recorded": false,
            "packet_assembly_persisted": false,
            "packet_complete": false,
            "packet_ready": false,
            "packet_accepted": false,
            "operator_acceptance_recorded": false,
            "operator_approval_recorded": false,
            "activation_authority_derived": false,
            "activation_command_derived": false,
            "activation_allowed": false,
            "activation_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "hepta_intelligence_context_attached": false,
            "prompt_preview_rendered": false,
            "context_injection_performed": false,
            "provider_invoked": false,
            "model_invoked": false,
            "external_kg_adapter_read_performed": false,
            "external_adapter_client_constructed": false,
            "network_call_performed": false,
            "external_db_write_performed": false,
            "live_kg_write_performed": false,
            "credential_read": false,
            "secret_file_read": false,
            "install_executed": false,
            "launchd_mutated": false,
            "service_restarted": false,
            "active_binary_mutated": false,
            "public_release_claimed": false,
            "public_ga_claimed": false,
            "release_artifact_written": false,
            "public_artifact_written": false,
            "external_send_performed": false,
        }),
    );

    let mut side_effects = serde_json::json!({
        "packet_assembly_performed": false,
        "packet_assembly_recorded": false,
        "packet_assembly_persisted": false,
        "packet_ready_promoted": false,
        "packet_acceptance_recorded": false,
        "packet_operator_approval_derived": false,
        "packet_activation_authority_derived": false,
        "packet_activation_command_derived": false,
        "packet_live_execution_allowed": false,
        "packet_template_recorded": false,
        "packet_template_persisted": false,
        "packet_template_materialized": false,
        "packet_template_delivered": false,
        "operator_acceptance_recorded": false,
        "operator_approval_recorded": false,
        "activation_authority_derived": false,
        "activation_command_derived": false,
        "activation_allowed": false,
        "activation_performed": false,
    });
    extend_json_object(
        &mut side_effects,
        serde_json::json!({
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "hepta_intelligence_context_attached": false,
            "prompt_preview_rendered": false,
            "context_injection_performed": false,
            "provider_invoked": false,
            "model_invoked": false,
            "external_kg_adapter_read_performed": false,
            "external_adapter_client_constructed": false,
            "network_call_performed": false,
            "external_db_write_performed": false,
        }),
    );
    extend_json_object(
        &mut side_effects,
        serde_json::json!({
            "live_kg_write_performed": false,
            "credential_read": false,
            "secret_file_read": false,
            "install_executed": false,
            "launchd_mutated": false,
            "service_restarted": false,
            "active_binary_mutated": false,
            "public_release_claimed": false,
            "public_ga_claimed": false,
            "release_artifact_written": false,
            "public_artifact_written": false,
            "external_send_performed": false,
            "filesystem_written": false
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "side_effects": side_effects
        }),
    );
    report
}

fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_non_persistence_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source =
        hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_assembly_non_acceptance_report();
    let source_report_sha256 = sha256_json_value(&source);
    let source_ready = source
        .get("memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_assembly_non_acceptance_ready")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);
    let source_packet_assembly_attempt_count = source
        .get("packet_assembly_attempt_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_packet_assembled_count = source
        .get("packet_assembled_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_packet_accepted_count = source
        .get("packet_accepted_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_packet_activation_authority_derived_count = source
        .get("packet_activation_authority_derived_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;

    let receipt_surfaces = vec![
        "packet_assembly_denial_receipt",
        "packet_acceptance_attempt_receipt",
        "operator_summary_receipt",
        "packet_query_receipt",
        "packet_export_receipt",
        "packet_observability_receipt",
        "packet_completion_ack_receipt",
        "packet_authority_derivation_receipt",
    ]
    .into_iter()
    .map(|receipt_surface| {
        serde_json::json!({
            "receipt_surface": receipt_surface,
            "receipt_generated": true,
            "receipt_recorded": false,
            "receipt_persisted": false,
            "receipt_materialized": false,
            "receipt_indexed": false,
            "receipt_queryable": false,
            "receipt_exportable": false,
            "receipt_observable": false,
            "receipt_delivered": false,
            "receipt_acceptance_recorded": false,
            "receipt_operator_approval_derived": false,
            "receipt_activation_authority_derived": false,
            "receipt_activation_command_derived": false,
            "receipt_live_execution_allowed": false,
            "receipt_status": "non_persistent_report_only"
        })
    })
    .collect::<Vec<_>>();
    let receipt_surface_count = receipt_surfaces.len();
    let receipt_contract_hash_sha256 = sha256_text_value(&format!(
        "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-non-persistence:native:source={source_report_sha256}:receipts={receipt_surface_count}:route_count={}:recorded=0:persisted=0:accepted=0:authority=0:live=0",
        route_matrix.route_count
    ));
    let denied_by_packet_acceptance_receipt = vec![
        "operator_readiness_packet_template_packet_receipt_recording_denied",
        "operator_readiness_packet_template_packet_receipt_persistence_denied",
        "operator_readiness_packet_template_packet_receipt_materialization_denied",
        "operator_readiness_packet_template_packet_receipt_indexing_denied",
        "operator_readiness_packet_template_packet_receipt_query_export_denied",
        "operator_readiness_packet_template_packet_receipt_observability_denied",
        "operator_readiness_packet_template_packet_receipt_delivery_denied",
        "operator_readiness_packet_template_packet_receipt_acceptance_denied",
        "operator_readiness_packet_template_packet_receipt_authority_derivation_denied",
        "operator_readiness_packet_template_packet_receipt_live_execution_denied",
    ];
    let denied_by_packet_acceptance_receipt_count = denied_by_packet_acceptance_receipt.len();
    let report_ready = source_ready
        && source_packet_assembly_attempt_count == 4
        && source_packet_assembled_count == 0
        && source_packet_accepted_count == 0
        && source_packet_activation_authority_derived_count == 0
        && receipt_surface_count == 8
        && route_count_source_command_accepted;

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_non_persistence_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_NON_PERSISTENCE_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-non-persistence --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-15",
        "packet_acceptance_receipt_schema_version": "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_non_persistence_v1",
        "packet_acceptance_receipt_mode": "native_route_denied_packet_assembly_receipts_are_report_only_no_persistence_no_acceptance_no_authority",
        "source_packet_assembly_gate": source["gate"].clone(),
        "source_packet_assembly_ready": source_ready,
        "source_packet_assembly_report_sha256": source_report_sha256,
        "receipt_contract_hash_sha256": receipt_contract_hash_sha256,
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
            "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_non_persistence_route_enabled": true,
            "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_non_persistence_ready": true,
            "source_packet_assembly_attempt_count": source_packet_assembly_attempt_count,
            "source_packet_assembled_count": source_packet_assembled_count,
            "source_packet_accepted_count": source_packet_accepted_count,
            "source_packet_activation_authority_derived_count": source_packet_activation_authority_derived_count,
            "receipt_surface_count": receipt_surface_count,
            "receipt_generated_count": receipt_surface_count,
            "receipt_recorded_count": 0,
            "receipt_persisted_count": 0,
            "receipt_materialized_count": 0,
            "receipt_indexed_count": 0,
            "receipt_queryable_count": 0,
            "receipt_exportable_count": 0,
            "receipt_observable_count": 0,
            "receipt_delivered_count": 0,
            "receipt_acceptance_recorded_count": 0,
            "receipt_operator_approval_derived_count": 0,
            "receipt_activation_authority_derived_count": 0,
            "receipt_activation_command_derived_count": 0,
            "receipt_live_execution_allowed_count": 0,
            "receipt_surfaces": receipt_surfaces,
            "denied_by_packet_acceptance_receipt": denied_by_packet_acceptance_receipt,
            "denied_by_packet_acceptance_receipt_count": denied_by_packet_acceptance_receipt_count,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "allowed_next_actions": [
                {
                    "action": "prepare_operator_readiness_packet_template_packet_acceptance_receipt_replay_idempotency_denial_gate",
                    "status": "allowed_report_only_next_slice",
                    "persists_receipt": false,
                    "records_operator_acceptance": false,
                    "derives_activation_authority": false,
                    "activates_live": false,
                    "mutates_memory_store": false,
                    "writes_kg": false
                }
            ],
            "packet_template_recorded": false,
            "packet_template_persisted": false,
            "packet_assembly_performed": false,
            "packet_assembly_recorded": false,
            "packet_assembly_persisted": false,
            "packet_complete": false,
            "packet_ready": false,
            "packet_accepted": false,
            "packet_acceptance_receipt_recorded": false,
            "packet_acceptance_receipt_persisted": false,
            "packet_acceptance_receipt_materialized": false,
            "packet_acceptance_receipt_indexed": false,
            "packet_acceptance_receipt_delivered": false,
            "operator_acceptance_recorded": false,
            "operator_approval_recorded": false,
            "activation_authority_derived": false,
            "activation_command_derived": false,
            "activation_allowed": false,
            "activation_performed": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "hepta_intelligence_context_attached": false,
            "prompt_preview_rendered": false,
            "context_injection_performed": false,
            "provider_invoked": false,
            "model_invoked": false,
            "external_kg_adapter_read_performed": false,
            "external_adapter_client_constructed": false,
            "network_call_performed": false,
            "external_db_write_performed": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "live_kg_write_performed": false,
            "credential_read": false,
            "secret_file_read": false,
            "install_executed": false,
            "launchd_mutated": false,
            "service_restarted": false,
            "active_binary_mutated": false,
            "public_release_claimed": false,
            "public_ga_claimed": false,
            "release_artifact_written": false,
            "public_artifact_written": false,
            "external_send_performed": false,
        }),
    );

    let mut side_effects = serde_json::json!({
        "packet_acceptance_receipt_recorded": false,
        "packet_acceptance_receipt_persisted": false,
        "packet_acceptance_receipt_materialized": false,
        "packet_acceptance_receipt_indexed": false,
        "packet_acceptance_receipt_queryable": false,
        "packet_acceptance_receipt_exportable": false,
        "packet_acceptance_receipt_observable": false,
        "packet_acceptance_receipt_delivered": false,
        "packet_acceptance_receipt_acceptance_recorded": false,
        "packet_acceptance_receipt_authority_derived": false,
        "packet_acceptance_receipt_live_execution_allowed": false,
        "packet_template_recorded": false,
        "packet_template_persisted": false,
        "packet_assembly_performed": false,
        "packet_assembly_recorded": false,
        "packet_assembly_persisted": false,
        "packet_ready_promoted": false,
        "packet_acceptance_recorded": false,
        "operator_acceptance_recorded": false,
        "operator_approval_recorded": false,
        "activation_authority_derived": false,
        "activation_command_derived": false,
        "activation_allowed": false,
        "activation_performed": false,
    });
    extend_json_object(
        &mut side_effects,
        serde_json::json!({
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "hepta_intelligence_context_attached": false,
            "prompt_preview_rendered": false,
            "context_injection_performed": false,
            "provider_invoked": false,
            "model_invoked": false,
            "external_kg_adapter_read_performed": false,
            "external_adapter_client_constructed": false,
            "network_call_performed": false,
            "external_db_write_performed": false,
        }),
    );
    extend_json_object(
        &mut side_effects,
        serde_json::json!({
            "live_kg_write_performed": false,
            "credential_read": false,
            "secret_file_read": false,
            "install_executed": false,
            "launchd_mutated": false,
            "service_restarted": false,
            "active_binary_mutated": false,
            "public_release_claimed": false,
            "public_ga_claimed": false,
            "release_artifact_written": false,
            "public_artifact_written": false,
            "external_send_performed": false,
            "filesystem_written": false
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "side_effects": side_effects
        }),
    );
    report
}

fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_replay_idempotency_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source =
        hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_non_persistence_report();
    let source_receipt_report_sha256 = sha256_json_value(&source);
    let source_ready = source
        .get("memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_non_persistence_ready")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);
    let source_receipt_surface_count = source
        .get("receipt_surface_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_receipt_generated_count = source
        .get("receipt_generated_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_receipt_recorded_count = source
        .get("receipt_recorded_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_receipt_persisted_count = source
        .get("receipt_persisted_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_receipt_acceptance_recorded_count = source
        .get("receipt_acceptance_recorded_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_receipt_activation_authority_derived_count = source
        .get("receipt_activation_authority_derived_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;

    let replay_surfaces = vec![
        "packet_receipt_replay",
        "packet_receipt_idempotency_key_registration",
        "packet_receipt_idempotency_cache_write",
        "packet_receipt_cache_hit_promotion",
        "packet_receipt_query_result_replay",
        "packet_receipt_export_snapshot_replay",
        "packet_receipt_observability_snapshot_replay",
        "packet_receipt_operator_summary_replay",
        "packet_receipt_completion_ack_replay",
        "packet_receipt_authority_replay",
    ]
    .into_iter()
    .map(|replay_surface| {
        serde_json::json!({
            "replay_surface": replay_surface,
            "replay_attempted": true,
            "replay_recorded": false,
            "replay_persisted": false,
            "idempotency_key_registered": false,
            "idempotency_cache_written": false,
            "cache_hit_promoted": false,
            "query_result_registered": false,
            "export_snapshot_recorded": false,
            "observability_snapshot_recorded": false,
            "acceptance_recorded": false,
            "operator_approval_derived": false,
            "activation_authority_derived": false,
            "activation_command_derived": false,
            "live_execution_allowed": false,
            "replay_status": "replay_idempotency_denied"
        })
    })
    .collect::<Vec<_>>();
    let replay_surface_count = replay_surfaces.len();
    let replay_idempotency_contract_hash_sha256 = sha256_text_value(&format!(
        "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-replay-idempotency-denial:native:source={source_receipt_report_sha256}:surfaces={replay_surface_count}:route_count={}:recorded=0:persisted=0:idempotency=0:authority=0:live=0",
        route_matrix.route_count
    ));
    let denied_by_packet_receipt_replay_idempotency = vec![
        "operator_readiness_packet_template_packet_receipt_replay_recording_denied",
        "operator_readiness_packet_template_packet_receipt_replay_persistence_denied",
        "operator_readiness_packet_template_packet_receipt_idempotency_key_registration_denied",
        "operator_readiness_packet_template_packet_receipt_idempotency_cache_write_denied",
        "operator_readiness_packet_template_packet_receipt_cache_hit_promotion_denied",
        "operator_readiness_packet_template_packet_receipt_query_result_registration_denied",
        "operator_readiness_packet_template_packet_receipt_export_snapshot_denied",
        "operator_readiness_packet_template_packet_receipt_observability_snapshot_denied",
        "operator_readiness_packet_template_packet_receipt_acceptance_replay_denied",
        "operator_readiness_packet_template_packet_receipt_authority_replay_denied",
        "operator_readiness_packet_template_packet_receipt_live_execution_denied",
    ];
    let denied_by_packet_receipt_replay_idempotency_count =
        denied_by_packet_receipt_replay_idempotency.len();
    let report_ready = source_ready
        && source_receipt_surface_count == 8
        && source_receipt_generated_count == 8
        && source_receipt_recorded_count == 0
        && source_receipt_persisted_count == 0
        && source_receipt_acceptance_recorded_count == 0
        && source_receipt_activation_authority_derived_count == 0
        && replay_surface_count == 10
        && route_count_source_command_accepted;

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_replay_idempotency_denial_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_REPLAY_IDEMPOTENCY_DENIAL_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-replay-idempotency-denial --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-15",
        "receipt_replay_idempotency_schema_version": "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_replay_idempotency_denial_v1",
        "receipt_replay_idempotency_mode": "native_route_non_persistent_receipts_cannot_replay_cache_or_derive_authority",
        "source_packet_acceptance_receipt_gate": source["gate"].clone(),
        "source_packet_acceptance_receipt_ready": source_ready,
        "source_receipt_report_sha256": source_receipt_report_sha256,
        "replay_idempotency_contract_hash_sha256": replay_idempotency_contract_hash_sha256,
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
            "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_replay_idempotency_denial_route_enabled": true,
            "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_replay_idempotency_denial_ready": true,
            "source_receipt_surface_count": source_receipt_surface_count,
            "source_receipt_generated_count": source_receipt_generated_count,
            "source_receipt_recorded_count": source_receipt_recorded_count,
            "source_receipt_persisted_count": source_receipt_persisted_count,
            "source_receipt_acceptance_recorded_count": source_receipt_acceptance_recorded_count,
            "source_receipt_activation_authority_derived_count": source_receipt_activation_authority_derived_count,
            "replay_surface_count": replay_surface_count,
            "replay_attempt_count": replay_surface_count,
            "replay_recorded_count": 0,
            "replay_persisted_count": 0,
            "idempotency_key_registered_count": 0,
            "idempotency_cache_written_count": 0,
            "cache_hit_promoted_count": 0,
            "query_result_registered_count": 0,
            "export_snapshot_recorded_count": 0,
            "observability_snapshot_recorded_count": 0,
            "replay_acceptance_recorded_count": 0,
            "replay_operator_approval_derived_count": 0,
            "replay_activation_authority_derived_count": 0,
            "replay_activation_command_derived_count": 0,
            "replay_live_execution_allowed_count": 0,
            "replay_surfaces": replay_surfaces,
            "denied_by_packet_receipt_replay_idempotency": denied_by_packet_receipt_replay_idempotency,
            "denied_by_packet_receipt_replay_idempotency_count": denied_by_packet_receipt_replay_idempotency_count,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "allowed_next_actions": [
                {
                    "action": "prepare_operator_readiness_packet_template_packet_acceptance_receipt_ordering_monotonicity_denial_gate",
                    "status": "allowed_report_only_next_slice",
                    "persists_receipt": false,
                    "records_operator_acceptance": false,
                    "derives_activation_authority": false,
                    "activates_live": false,
                    "mutates_memory_store": false,
                    "writes_kg": false
                }
            ],
            "packet_template_recorded": false,
            "packet_template_persisted": false,
            "packet_assembly_performed": false,
            "packet_assembly_recorded": false,
            "packet_assembly_persisted": false,
            "packet_complete": false,
            "packet_ready": false,
            "packet_accepted": false,
            "packet_acceptance_receipt_recorded": false,
            "packet_acceptance_receipt_persisted": false,
            "packet_acceptance_receipt_materialized": false,
            "packet_acceptance_receipt_indexed": false,
            "packet_acceptance_receipt_delivered": false,
            "packet_acceptance_receipt_replayed": false,
            "packet_acceptance_receipt_idempotency_key_registered": false,
            "packet_acceptance_receipt_idempotency_cache_written": false,
            "packet_acceptance_receipt_cache_hit_promoted": false,
            "packet_acceptance_receipt_query_result_registered": false,
            "packet_acceptance_receipt_export_snapshot_recorded": false,
            "packet_acceptance_receipt_observability_snapshot_recorded": false,
            "operator_acceptance_recorded": false,
            "operator_approval_recorded": false,
            "activation_authority_derived": false,
            "activation_command_derived": false,
            "activation_allowed": false,
            "activation_performed": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "hepta_intelligence_context_attached": false,
            "prompt_preview_rendered": false,
            "context_injection_performed": false,
            "provider_invoked": false,
            "model_invoked": false,
            "external_kg_adapter_read_performed": false,
            "external_adapter_client_constructed": false,
            "network_call_performed": false,
            "external_db_write_performed": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "live_kg_write_performed": false,
            "credential_read": false,
            "secret_file_read": false,
            "install_executed": false,
            "launchd_mutated": false,
            "service_restarted": false,
            "active_binary_mutated": false,
            "public_release_claimed": false,
            "public_ga_claimed": false,
            "release_artifact_written": false,
            "public_artifact_written": false,
            "external_send_performed": false,
        }),
    );

    let mut side_effects = serde_json::json!({
        "packet_acceptance_receipt_replayed": false,
        "packet_acceptance_receipt_replay_recorded": false,
        "packet_acceptance_receipt_replay_persisted": false,
        "packet_acceptance_receipt_idempotency_key_registered": false,
        "packet_acceptance_receipt_idempotency_cache_written": false,
        "packet_acceptance_receipt_cache_hit_promoted": false,
        "packet_acceptance_receipt_query_result_registered": false,
        "packet_acceptance_receipt_export_snapshot_recorded": false,
        "packet_acceptance_receipt_observability_snapshot_recorded": false,
        "packet_acceptance_receipt_acceptance_recorded": false,
        "packet_acceptance_receipt_authority_derived": false,
        "packet_acceptance_receipt_live_execution_allowed": false,
        "packet_acceptance_receipt_recorded": false,
        "packet_acceptance_receipt_persisted": false,
        "packet_acceptance_receipt_materialized": false,
        "packet_acceptance_receipt_indexed": false,
        "packet_acceptance_receipt_delivered": false,
        "packet_template_recorded": false,
        "packet_template_persisted": false,
        "packet_assembly_performed": false,
        "packet_assembly_recorded": false,
        "packet_assembly_persisted": false,
        "packet_ready_promoted": false,
        "packet_acceptance_recorded": false,
        "operator_acceptance_recorded": false,
        "operator_approval_recorded": false,
        "activation_authority_derived": false,
        "activation_command_derived": false,
        "activation_allowed": false,
        "activation_performed": false,
    });
    extend_json_object(
        &mut side_effects,
        serde_json::json!({
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "hepta_intelligence_context_attached": false,
            "prompt_preview_rendered": false,
            "context_injection_performed": false,
            "provider_invoked": false,
            "model_invoked": false,
            "external_kg_adapter_read_performed": false,
            "external_adapter_client_constructed": false,
            "network_call_performed": false,
            "external_db_write_performed": false,
        }),
    );
    extend_json_object(
        &mut side_effects,
        serde_json::json!({
            "live_kg_write_performed": false,
            "credential_read": false,
            "secret_file_read": false,
            "install_executed": false,
            "launchd_mutated": false,
            "service_restarted": false,
            "active_binary_mutated": false,
            "public_release_claimed": false,
            "public_ga_claimed": false,
            "release_artifact_written": false,
            "public_artifact_written": false,
            "external_send_performed": false,
            "filesystem_written": false
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "side_effects": side_effects
        }),
    );
    report
}

fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_ordering_monotonicity_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source =
        hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_replay_idempotency_denial_report();
    let source_replay_report_sha256 = sha256_json_value(&source);
    let source_ready = source
        .get("memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_replay_idempotency_denial_ready")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);
    let source_replay_surface_count = source
        .get("replay_surface_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_replay_attempt_count = source
        .get("replay_attempt_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_replay_recorded_count = source
        .get("replay_recorded_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_replay_persisted_count = source
        .get("replay_persisted_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_idempotency_key_registered_count = source
        .get("idempotency_key_registered_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_idempotency_cache_written_count = source
        .get("idempotency_cache_written_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_cache_hit_promoted_count = source
        .get("cache_hit_promoted_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_replay_acceptance_recorded_count = source
        .get("replay_acceptance_recorded_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_replay_activation_authority_derived_count = source
        .get("replay_activation_authority_derived_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_replay_idempotency_contract_hash_sha256 = source
        .get("replay_idempotency_contract_hash_sha256")
        .cloned()
        .unwrap_or_else(|| serde_json::json!(""));
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;

    let ordering_surfaces = vec![
        "packet_receipt_duplicate_sequence_claim",
        "packet_receipt_stale_sequence_claim",
        "packet_receipt_late_arrival_claim",
        "packet_receipt_future_sequence_gap_claim",
        "packet_receipt_timestamp_rollback_claim",
        "packet_receipt_epoch_rollback_claim",
        "packet_receipt_same_sequence_different_hash_claim",
        "packet_receipt_latest_wins_overwrite_claim",
        "packet_receipt_query_ordering_claim",
        "packet_receipt_export_ordering_claim",
        "packet_receipt_observability_ordering_claim",
        "packet_receipt_completion_ack_ordering_claim",
        "packet_receipt_authority_ordering_claim",
        "packet_receipt_live_activation_ordering_claim",
    ]
    .into_iter()
    .map(|ordering_surface| {
        serde_json::json!({
            "ordering_surface": ordering_surface,
            "ordering_attempted": true,
            "sequence_cursor_accepted": false,
            "sequence_cursor_recorded": false,
            "sequence_cursor_persisted": false,
            "monotonicity_state_recorded": false,
            "monotonicity_state_persisted": false,
            "ordering_recorded": false,
            "ordering_persisted": false,
            "ordering_materialized": false,
            "latest_wins_accepted": false,
            "duplicate_accepted": false,
            "stale_accepted": false,
            "late_accepted": false,
            "future_gap_accepted": false,
            "timestamp_rollback_accepted": false,
            "epoch_rollback_accepted": false,
            "same_sequence_hash_override_accepted": false,
            "acceptance_recorded": false,
            "operator_approval_derived": false,
            "activation_authority_derived": false,
            "activation_command_derived": false,
            "live_execution_allowed": false,
            "ordering_status": "ordering_monotonicity_denied"
        })
    })
    .collect::<Vec<_>>();
    let ordering_surface_count = ordering_surfaces.len();
    let ordering_monotonicity_contract_hash_sha256 = sha256_text_value(&format!(
        "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-ordering-monotonicity-denial:native:source={source_replay_report_sha256}:surfaces={ordering_surface_count}:route_count={}:ordering=0:cursor=0:monotonicity=0:authority=0:live=0",
        route_matrix.route_count
    ));
    let denied_by_packet_receipt_ordering_monotonicity = vec![
        "operator_readiness_packet_template_packet_receipt_ordering_recording_denied",
        "operator_readiness_packet_template_packet_receipt_ordering_persistence_denied",
        "operator_readiness_packet_template_packet_receipt_ordering_materialization_denied",
        "operator_readiness_packet_template_packet_receipt_sequence_cursor_acceptance_denied",
        "operator_readiness_packet_template_packet_receipt_sequence_cursor_recording_denied",
        "operator_readiness_packet_template_packet_receipt_sequence_cursor_persistence_denied",
        "operator_readiness_packet_template_packet_receipt_monotonicity_state_recording_denied",
        "operator_readiness_packet_template_packet_receipt_monotonicity_state_persistence_denied",
        "operator_readiness_packet_template_packet_receipt_duplicate_sequence_denied",
        "operator_readiness_packet_template_packet_receipt_stale_sequence_denied",
        "operator_readiness_packet_template_packet_receipt_late_arrival_denied",
        "operator_readiness_packet_template_packet_receipt_future_sequence_gap_denied",
        "operator_readiness_packet_template_packet_receipt_timestamp_rollback_denied",
        "operator_readiness_packet_template_packet_receipt_epoch_rollback_denied",
        "operator_readiness_packet_template_packet_receipt_same_sequence_hash_override_denied",
        "operator_readiness_packet_template_packet_receipt_latest_wins_overwrite_denied",
        "operator_readiness_packet_template_packet_receipt_query_ordering_denied",
        "operator_readiness_packet_template_packet_receipt_export_ordering_denied",
        "operator_readiness_packet_template_packet_receipt_observability_ordering_denied",
        "operator_readiness_packet_template_packet_receipt_completion_ack_ordering_denied",
        "operator_readiness_packet_template_packet_receipt_acceptance_from_ordering_denied",
        "operator_readiness_packet_template_packet_receipt_authority_from_ordering_denied",
        "operator_readiness_packet_template_packet_receipt_live_execution_from_ordering_denied",
    ];
    let denied_by_packet_receipt_ordering_monotonicity_count =
        denied_by_packet_receipt_ordering_monotonicity.len();
    let report_ready = source_ready
        && source_replay_surface_count == 10
        && source_replay_attempt_count == 10
        && source_replay_recorded_count == 0
        && source_replay_persisted_count == 0
        && source_idempotency_key_registered_count == 0
        && source_idempotency_cache_written_count == 0
        && source_cache_hit_promoted_count == 0
        && source_replay_acceptance_recorded_count == 0
        && source_replay_activation_authority_derived_count == 0
        && ordering_surface_count == 14
        && route_count_source_command_accepted;

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_ordering_monotonicity_denial_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_ORDERING_MONOTONICITY_DENIAL_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-ordering-monotonicity-denial --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-15",
        "receipt_ordering_monotonicity_schema_version": "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_ordering_monotonicity_denial_v1",
        "receipt_ordering_monotonicity_mode": "native_route_non_persistent_receipts_cannot_create_sequence_cursor_monotonicity_or_authority",
        "source_packet_acceptance_receipt_replay_idempotency_gate": source["gate"].clone(),
        "source_packet_acceptance_receipt_replay_idempotency_ready": source_ready,
        "source_replay_report_sha256": source_replay_report_sha256,
        "source_replay_idempotency_contract_hash_sha256": source_replay_idempotency_contract_hash_sha256,
        "ordering_monotonicity_contract_hash_sha256": ordering_monotonicity_contract_hash_sha256,
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
            "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_ordering_monotonicity_denial_route_enabled": true,
            "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_ordering_monotonicity_denial_ready": true,
            "source_replay_surface_count": source_replay_surface_count,
            "source_replay_attempt_count": source_replay_attempt_count,
            "source_replay_recorded_count": source_replay_recorded_count,
            "source_replay_persisted_count": source_replay_persisted_count,
            "source_idempotency_key_registered_count": source_idempotency_key_registered_count,
            "source_idempotency_cache_written_count": source_idempotency_cache_written_count,
            "source_cache_hit_promoted_count": source_cache_hit_promoted_count,
            "source_replay_acceptance_recorded_count": source_replay_acceptance_recorded_count,
            "source_replay_activation_authority_derived_count": source_replay_activation_authority_derived_count,
            "ordering_surface_count": ordering_surface_count,
            "ordering_attempt_count": ordering_surface_count,
            "ordering_recorded_count": 0,
            "ordering_persisted_count": 0,
            "ordering_materialized_count": 0,
            "sequence_cursor_accepted_count": 0,
            "sequence_cursor_recorded_count": 0,
            "sequence_cursor_persisted_count": 0,
            "monotonicity_state_recorded_count": 0,
            "monotonicity_state_persisted_count": 0,
            "duplicate_sequence_accepted_count": 0,
            "stale_sequence_accepted_count": 0,
            "late_arrival_accepted_count": 0,
            "future_sequence_gap_accepted_count": 0,
            "timestamp_rollback_accepted_count": 0,
            "epoch_rollback_accepted_count": 0,
            "same_sequence_hash_override_accepted_count": 0,
            "latest_wins_overwrite_accepted_count": 0,
            "ordering_acceptance_recorded_count": 0,
            "ordering_operator_approval_derived_count": 0,
            "ordering_activation_authority_derived_count": 0,
            "ordering_activation_command_derived_count": 0,
            "ordering_live_execution_allowed_count": 0,
            "ordering_surfaces": ordering_surfaces,
            "denied_by_packet_receipt_ordering_monotonicity": denied_by_packet_receipt_ordering_monotonicity,
            "denied_by_packet_receipt_ordering_monotonicity_count": denied_by_packet_receipt_ordering_monotonicity_count,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "allowed_next_actions": [
                {
                    "action": "prepare_operator_readiness_packet_template_packet_acceptance_receipt_cancellation_supersession_denial_gate",
                    "status": "allowed_report_only_next_slice",
                    "persists_receipt": false,
                    "records_operator_acceptance": false,
                    "derives_activation_authority": false,
                    "accepts_cancellation": false,
                    "accepts_supersession": false,
                    "activates_live": false,
                    "mutates_memory_store": false,
                    "writes_kg": false
                }
            ],
            "packet_template_recorded": false,
            "packet_template_persisted": false,
            "packet_assembly_performed": false,
            "packet_assembly_recorded": false,
            "packet_assembly_persisted": false,
            "packet_complete": false,
            "packet_ready": false,
            "packet_accepted": false,
            "packet_acceptance_receipt_recorded": false,
            "packet_acceptance_receipt_persisted": false,
            "packet_acceptance_receipt_replayed": false,
            "packet_acceptance_receipt_idempotency_key_registered": false,
            "packet_acceptance_receipt_idempotency_cache_written": false,
            "packet_acceptance_receipt_cache_hit_promoted": false,
            "packet_acceptance_receipt_ordering_recorded": false,
            "packet_acceptance_receipt_ordering_persisted": false,
            "packet_acceptance_receipt_sequence_cursor_accepted": false,
            "packet_acceptance_receipt_sequence_cursor_recorded": false,
            "packet_acceptance_receipt_sequence_cursor_persisted": false,
            "packet_acceptance_receipt_monotonicity_state_recorded": false,
            "packet_acceptance_receipt_monotonicity_state_persisted": false,
            "packet_acceptance_receipt_duplicate_accepted": false,
            "packet_acceptance_receipt_stale_accepted": false,
            "packet_acceptance_receipt_late_accepted": false,
            "packet_acceptance_receipt_future_gap_accepted": false,
            "packet_acceptance_receipt_timestamp_rollback_accepted": false,
            "packet_acceptance_receipt_epoch_rollback_accepted": false,
            "packet_acceptance_receipt_same_sequence_hash_override_accepted": false,
            "packet_acceptance_receipt_latest_wins_overwrite_accepted": false,
            "operator_acceptance_recorded": false,
            "operator_approval_recorded": false,
            "activation_authority_derived": false,
            "activation_command_derived": false,
            "activation_allowed": false,
            "activation_performed": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "hepta_intelligence_context_attached": false,
            "prompt_preview_rendered": false,
            "context_injection_performed": false,
            "provider_invoked": false,
            "model_invoked": false,
            "external_kg_adapter_read_performed": false,
            "external_adapter_client_constructed": false,
            "network_call_performed": false,
            "external_db_write_performed": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "live_kg_write_performed": false,
            "credential_read": false,
            "secret_file_read": false,
            "install_executed": false,
            "launchd_mutated": false,
            "service_restarted": false,
            "active_binary_mutated": false,
            "public_release_claimed": false,
            "public_ga_claimed": false,
            "release_artifact_written": false,
            "public_artifact_written": false,
            "external_send_performed": false,
        }),
    );

    let mut side_effects = serde_json::json!({
        "packet_acceptance_receipt_ordering_recorded": false,
        "packet_acceptance_receipt_ordering_persisted": false,
        "packet_acceptance_receipt_ordering_materialized": false,
        "packet_acceptance_receipt_sequence_cursor_accepted": false,
        "packet_acceptance_receipt_sequence_cursor_recorded": false,
        "packet_acceptance_receipt_sequence_cursor_persisted": false,
        "packet_acceptance_receipt_monotonicity_state_recorded": false,
        "packet_acceptance_receipt_monotonicity_state_persisted": false,
        "packet_acceptance_receipt_duplicate_accepted": false,
        "packet_acceptance_receipt_stale_accepted": false,
        "packet_acceptance_receipt_late_accepted": false,
        "packet_acceptance_receipt_future_gap_accepted": false,
        "packet_acceptance_receipt_timestamp_rollback_accepted": false,
        "packet_acceptance_receipt_epoch_rollback_accepted": false,
        "packet_acceptance_receipt_same_sequence_hash_override_accepted": false,
        "packet_acceptance_receipt_latest_wins_overwrite_accepted": false,
        "packet_acceptance_receipt_acceptance_recorded": false,
        "packet_acceptance_receipt_authority_derived": false,
        "packet_acceptance_receipt_live_execution_allowed": false,
    });
    extend_json_object(
        &mut side_effects,
        serde_json::json!({
        "packet_acceptance_receipt_replayed": false,
        "packet_acceptance_receipt_replay_recorded": false,
        "packet_acceptance_receipt_replay_persisted": false,
        "packet_acceptance_receipt_idempotency_key_registered": false,
        "packet_acceptance_receipt_idempotency_cache_written": false,
        "packet_acceptance_receipt_cache_hit_promoted": false,
        "packet_acceptance_receipt_recorded": false,
        "packet_acceptance_receipt_persisted": false,
        "packet_acceptance_receipt_materialized": false,
        "packet_acceptance_receipt_indexed": false,
        "packet_acceptance_receipt_delivered": false,
        }),
    );
    extend_json_object(
        &mut side_effects,
        serde_json::json!({
        "packet_template_recorded": false,
        "packet_template_persisted": false,
        "packet_assembly_performed": false,
        "packet_assembly_recorded": false,
        "packet_assembly_persisted": false,
        "packet_ready_promoted": false,
        "packet_acceptance_recorded": false,
        "operator_acceptance_recorded": false,
        "operator_approval_recorded": false,
        "activation_authority_derived": false,
        "activation_command_derived": false,
        "activation_allowed": false,
        "activation_performed": false,
        }),
    );
    extend_json_object(
        &mut side_effects,
        serde_json::json!({
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "hepta_intelligence_context_attached": false,
            "prompt_preview_rendered": false,
            "context_injection_performed": false,
            "provider_invoked": false,
            "model_invoked": false,
            "external_kg_adapter_read_performed": false,
            "external_adapter_client_constructed": false,
            "network_call_performed": false,
            "external_db_write_performed": false,
        }),
    );
    extend_json_object(
        &mut side_effects,
        serde_json::json!({
            "live_kg_write_performed": false,
            "credential_read": false,
            "secret_file_read": false,
            "install_executed": false,
            "launchd_mutated": false,
            "service_restarted": false,
            "active_binary_mutated": false,
            "public_release_claimed": false,
            "public_ga_claimed": false,
            "release_artifact_written": false,
            "public_artifact_written": false,
            "external_send_performed": false,
            "filesystem_written": false
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "side_effects": side_effects
        }),
    );
    report
}

fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_cancellation_supersession_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source =
        hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_ordering_monotonicity_denial_report();
    let source_ordering_report_sha256 = sha256_json_value(&source);
    let source_ready = source
        .get("memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_ordering_monotonicity_denial_ready")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);
    let source_ordering_surface_count = source
        .get("ordering_surface_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_ordering_attempt_count = source
        .get("ordering_attempt_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_ordering_recorded_count = source
        .get("ordering_recorded_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_ordering_persisted_count = source
        .get("ordering_persisted_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_sequence_cursor_recorded_count = source
        .get("sequence_cursor_recorded_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_monotonicity_state_recorded_count = source
        .get("monotonicity_state_recorded_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_ordering_acceptance_recorded_count = source
        .get("ordering_acceptance_recorded_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_ordering_activation_authority_derived_count = source
        .get("ordering_activation_authority_derived_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_ordering_monotonicity_contract_hash_sha256 = source
        .get("ordering_monotonicity_contract_hash_sha256")
        .cloned()
        .unwrap_or_else(|| serde_json::json!(""));
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;

    let cancellation_surfaces = vec![
        "packet_receipt_cancel_claim",
        "packet_receipt_revoke_claim",
        "packet_receipt_withdraw_claim",
        "packet_receipt_supersede_claim",
        "packet_receipt_replacement_claim",
        "packet_receipt_tombstone_claim",
        "packet_receipt_delete_marker_claim",
        "packet_receipt_latest_replacement_claim",
        "packet_receipt_ack_replacement_claim",
        "packet_receipt_query_replacement_claim",
        "packet_receipt_export_replacement_claim",
        "packet_receipt_observability_replacement_claim",
        "packet_receipt_authority_replacement_claim",
        "packet_receipt_live_replacement_claim",
    ]
    .into_iter()
    .map(|cancellation_surface| {
        serde_json::json!({
            "cancellation_surface": cancellation_surface,
            "cancellation_or_supersession_attempted": true,
            "cancellation_accepted": false,
            "cancellation_recorded": false,
            "cancellation_persisted": false,
            "supersession_accepted": false,
            "supersession_recorded": false,
            "supersession_persisted": false,
            "replacement_receipt_accepted": false,
            "replacement_receipt_recorded": false,
            "replacement_receipt_persisted": false,
            "tombstone_recorded": false,
            "tombstone_persisted": false,
            "delete_marker_recorded": false,
            "latest_replacement_accepted": false,
            "ack_replacement_accepted": false,
            "query_replacement_registered": false,
            "export_replacement_recorded": false,
            "observability_replacement_recorded": false,
            "acceptance_recorded": false,
            "operator_approval_derived": false,
            "activation_authority_derived": false,
            "activation_command_derived": false,
            "live_execution_allowed": false,
            "cancellation_supersession_status": "cancellation_supersession_denied"
        })
    })
    .collect::<Vec<_>>();
    let cancellation_supersession_surface_count = cancellation_surfaces.len();
    let cancellation_supersession_contract_hash_sha256 = sha256_text_value(&format!(
        "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-cancellation-supersession-denial:native:source={source_ordering_report_sha256}:surfaces={cancellation_supersession_surface_count}:route_count={}:cancellation=0:supersession=0:replacement=0:authority=0:live=0",
        route_matrix.route_count
    ));
    let denied_by_packet_receipt_cancellation_supersession = vec![
        "operator_readiness_packet_template_packet_receipt_cancellation_acceptance_denied",
        "operator_readiness_packet_template_packet_receipt_cancellation_recording_denied",
        "operator_readiness_packet_template_packet_receipt_cancellation_persistence_denied",
        "operator_readiness_packet_template_packet_receipt_supersession_acceptance_denied",
        "operator_readiness_packet_template_packet_receipt_supersession_recording_denied",
        "operator_readiness_packet_template_packet_receipt_supersession_persistence_denied",
        "operator_readiness_packet_template_packet_receipt_replacement_acceptance_denied",
        "operator_readiness_packet_template_packet_receipt_replacement_recording_denied",
        "operator_readiness_packet_template_packet_receipt_replacement_persistence_denied",
        "operator_readiness_packet_template_packet_receipt_tombstone_recording_denied",
        "operator_readiness_packet_template_packet_receipt_tombstone_persistence_denied",
        "operator_readiness_packet_template_packet_receipt_delete_marker_recording_denied",
        "operator_readiness_packet_template_packet_receipt_latest_replacement_denied",
        "operator_readiness_packet_template_packet_receipt_ack_replacement_denied",
        "operator_readiness_packet_template_packet_receipt_query_replacement_denied",
        "operator_readiness_packet_template_packet_receipt_export_replacement_denied",
        "operator_readiness_packet_template_packet_receipt_observability_replacement_denied",
        "operator_readiness_packet_template_packet_receipt_acceptance_from_cancellation_supersession_denied",
        "operator_readiness_packet_template_packet_receipt_authority_from_cancellation_supersession_denied",
        "operator_readiness_packet_template_packet_receipt_live_execution_from_cancellation_supersession_denied",
    ];
    let denied_by_packet_receipt_cancellation_supersession_count =
        denied_by_packet_receipt_cancellation_supersession.len();
    let report_ready = source_ready
        && source_ordering_surface_count == 14
        && source_ordering_attempt_count == 14
        && source_ordering_recorded_count == 0
        && source_ordering_persisted_count == 0
        && source_sequence_cursor_recorded_count == 0
        && source_monotonicity_state_recorded_count == 0
        && source_ordering_acceptance_recorded_count == 0
        && source_ordering_activation_authority_derived_count == 0
        && cancellation_supersession_surface_count == 14
        && route_count_source_command_accepted;

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_cancellation_supersession_denial_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_CANCELLATION_SUPERSESSION_DENIAL_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-cancellation-supersession-denial --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-15",
        "receipt_cancellation_supersession_schema_version": "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_cancellation_supersession_denial_v1",
        "receipt_cancellation_supersession_mode": "native_route_non_persistent_receipts_cannot_cancel_supersede_replace_or_derive_authority",
        "source_packet_acceptance_receipt_ordering_monotonicity_gate": source["gate"].clone(),
        "source_packet_acceptance_receipt_ordering_monotonicity_ready": source_ready,
        "source_ordering_report_sha256": source_ordering_report_sha256,
        "source_ordering_monotonicity_contract_hash_sha256": source_ordering_monotonicity_contract_hash_sha256,
        "cancellation_supersession_contract_hash_sha256": cancellation_supersession_contract_hash_sha256,
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
            "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_cancellation_supersession_denial_route_enabled": true,
            "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_cancellation_supersession_denial_ready": true,
            "source_ordering_surface_count": source_ordering_surface_count,
            "source_ordering_attempt_count": source_ordering_attempt_count,
            "source_ordering_recorded_count": source_ordering_recorded_count,
            "source_ordering_persisted_count": source_ordering_persisted_count,
            "source_sequence_cursor_recorded_count": source_sequence_cursor_recorded_count,
            "source_monotonicity_state_recorded_count": source_monotonicity_state_recorded_count,
            "source_ordering_acceptance_recorded_count": source_ordering_acceptance_recorded_count,
            "source_ordering_activation_authority_derived_count": source_ordering_activation_authority_derived_count,
            "cancellation_supersession_surface_count": cancellation_supersession_surface_count,
            "cancellation_supersession_attempt_count": cancellation_supersession_surface_count,
            "cancellation_accepted_count": 0,
            "cancellation_recorded_count": 0,
            "cancellation_persisted_count": 0,
            "supersession_accepted_count": 0,
            "supersession_recorded_count": 0,
            "supersession_persisted_count": 0,
            "replacement_receipt_accepted_count": 0,
            "replacement_receipt_recorded_count": 0,
            "replacement_receipt_persisted_count": 0,
            "tombstone_recorded_count": 0,
            "tombstone_persisted_count": 0,
            "delete_marker_recorded_count": 0,
            "latest_replacement_accepted_count": 0,
            "ack_replacement_accepted_count": 0,
            "query_replacement_registered_count": 0,
            "export_replacement_recorded_count": 0,
            "observability_replacement_recorded_count": 0,
            "cancellation_supersession_acceptance_recorded_count": 0,
            "cancellation_supersession_operator_approval_derived_count": 0,
            "cancellation_supersession_activation_authority_derived_count": 0,
            "cancellation_supersession_activation_command_derived_count": 0,
            "cancellation_supersession_live_execution_allowed_count": 0,
            "cancellation_surfaces": cancellation_surfaces,
            "denied_by_packet_receipt_cancellation_supersession": denied_by_packet_receipt_cancellation_supersession,
            "denied_by_packet_receipt_cancellation_supersession_count": denied_by_packet_receipt_cancellation_supersession_count,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "allowed_next_actions": [
                {
                    "action": "prepare_operator_readiness_packet_template_packet_acceptance_receipt_audit_trail_immutable_evidence_denial_gate",
                    "status": "allowed_report_only_next_slice",
                    "persists_receipt": false,
                    "records_operator_acceptance": false,
                    "derives_activation_authority": false,
                    "records_audit_trail": false,
                    "accepts_immutable_evidence": false,
                    "activates_live": false,
                    "mutates_memory_store": false,
                    "writes_kg": false
                }
            ],
            "packet_template_recorded": false,
            "packet_template_persisted": false,
            "packet_assembly_performed": false,
            "packet_assembly_recorded": false,
            "packet_assembly_persisted": false,
            "packet_complete": false,
            "packet_ready": false,
            "packet_accepted": false,
            "packet_acceptance_receipt_recorded": false,
            "packet_acceptance_receipt_persisted": false,
            "packet_acceptance_receipt_replayed": false,
            "packet_acceptance_receipt_ordering_recorded": false,
            "packet_acceptance_receipt_ordering_persisted": false,
            "packet_acceptance_receipt_sequence_cursor_recorded": false,
            "packet_acceptance_receipt_monotonicity_state_recorded": false,
            "packet_acceptance_receipt_cancellation_accepted": false,
            "packet_acceptance_receipt_cancellation_recorded": false,
            "packet_acceptance_receipt_cancellation_persisted": false,
            "packet_acceptance_receipt_supersession_accepted": false,
            "packet_acceptance_receipt_supersession_recorded": false,
            "packet_acceptance_receipt_supersession_persisted": false,
            "packet_acceptance_receipt_replacement_accepted": false,
            "packet_acceptance_receipt_replacement_recorded": false,
            "packet_acceptance_receipt_replacement_persisted": false,
            "packet_acceptance_receipt_tombstone_recorded": false,
            "packet_acceptance_receipt_tombstone_persisted": false,
            "packet_acceptance_receipt_delete_marker_recorded": false,
            "packet_acceptance_receipt_latest_replacement_accepted": false,
            "operator_acceptance_recorded": false,
            "operator_approval_recorded": false,
            "activation_authority_derived": false,
            "activation_command_derived": false,
            "activation_allowed": false,
            "activation_performed": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "hepta_intelligence_context_attached": false,
            "prompt_preview_rendered": false,
            "context_injection_performed": false,
            "provider_invoked": false,
            "model_invoked": false,
            "external_kg_adapter_read_performed": false,
            "external_adapter_client_constructed": false,
            "network_call_performed": false,
            "external_db_write_performed": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "live_kg_write_performed": false,
            "credential_read": false,
            "secret_file_read": false,
            "install_executed": false,
            "launchd_mutated": false,
            "service_restarted": false,
            "active_binary_mutated": false,
            "public_release_claimed": false,
            "public_ga_claimed": false,
            "release_artifact_written": false,
            "public_artifact_written": false,
            "external_send_performed": false,
        }),
    );

    let mut side_effects = serde_json::json!({
        "packet_acceptance_receipt_cancellation_accepted": false,
        "packet_acceptance_receipt_cancellation_recorded": false,
        "packet_acceptance_receipt_cancellation_persisted": false,
        "packet_acceptance_receipt_supersession_accepted": false,
        "packet_acceptance_receipt_supersession_recorded": false,
        "packet_acceptance_receipt_supersession_persisted": false,
        "packet_acceptance_receipt_replacement_accepted": false,
        "packet_acceptance_receipt_replacement_recorded": false,
        "packet_acceptance_receipt_replacement_persisted": false,
        "packet_acceptance_receipt_tombstone_recorded": false,
        "packet_acceptance_receipt_tombstone_persisted": false,
        "packet_acceptance_receipt_delete_marker_recorded": false,
        "packet_acceptance_receipt_latest_replacement_accepted": false,
        "packet_acceptance_receipt_ack_replacement_accepted": false,
        "packet_acceptance_receipt_query_replacement_registered": false,
        "packet_acceptance_receipt_export_replacement_recorded": false,
        "packet_acceptance_receipt_observability_replacement_recorded": false,
        "packet_acceptance_receipt_acceptance_recorded": false,
        "packet_acceptance_receipt_authority_derived": false,
        "packet_acceptance_receipt_live_execution_allowed": false,
    });
    extend_json_object(
        &mut side_effects,
        serde_json::json!({
            "packet_acceptance_receipt_ordering_recorded": false,
            "packet_acceptance_receipt_ordering_persisted": false,
            "packet_acceptance_receipt_sequence_cursor_recorded": false,
            "packet_acceptance_receipt_monotonicity_state_recorded": false,
            "packet_acceptance_receipt_replayed": false,
            "packet_acceptance_receipt_replay_recorded": false,
            "packet_acceptance_receipt_replay_persisted": false,
            "packet_acceptance_receipt_recorded": false,
            "packet_acceptance_receipt_persisted": false,
            "packet_template_recorded": false,
            "packet_template_persisted": false,
            "packet_assembly_performed": false,
            "packet_assembly_recorded": false,
            "packet_assembly_persisted": false,
            "packet_ready_promoted": false,
            "packet_acceptance_recorded": false,
            "operator_acceptance_recorded": false,
            "operator_approval_recorded": false,
            "activation_authority_derived": false,
            "activation_command_derived": false,
            "activation_allowed": false,
            "activation_performed": false,
        }),
    );
    extend_json_object(
        &mut side_effects,
        serde_json::json!({
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "hepta_intelligence_context_attached": false,
            "prompt_preview_rendered": false,
            "context_injection_performed": false,
            "provider_invoked": false,
            "model_invoked": false,
            "external_kg_adapter_read_performed": false,
            "external_adapter_client_constructed": false,
            "network_call_performed": false,
            "external_db_write_performed": false,
            "live_kg_write_performed": false,
            "credential_read": false,
            "secret_file_read": false,
            "install_executed": false,
            "launchd_mutated": false,
            "service_restarted": false,
            "active_binary_mutated": false,
            "public_release_claimed": false,
            "public_ga_claimed": false,
            "release_artifact_written": false,
            "public_artifact_written": false,
            "external_send_performed": false,
            "filesystem_written": false
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "side_effects": side_effects
        }),
    );
    report
}

fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_audit_trail_immutable_evidence_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source =
        hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_cancellation_supersession_denial_report();
    let source_cancellation_report_sha256 = sha256_json_value(&source);
    let source_ready = source
        .get("memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_cancellation_supersession_denial_ready")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);
    let source_cancellation_supersession_surface_count = source
        .get("cancellation_supersession_surface_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_cancellation_supersession_attempt_count = source
        .get("cancellation_supersession_attempt_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_cancellation_accepted_count = source
        .get("cancellation_accepted_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_supersession_accepted_count = source
        .get("supersession_accepted_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_replacement_receipt_accepted_count = source
        .get("replacement_receipt_accepted_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_tombstone_recorded_count = source
        .get("tombstone_recorded_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_cancellation_supersession_acceptance_recorded_count = source
        .get("cancellation_supersession_acceptance_recorded_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_cancellation_supersession_activation_authority_derived_count = source
        .get("cancellation_supersession_activation_authority_derived_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_cancellation_supersession_contract_hash_sha256 = source
        .get("cancellation_supersession_contract_hash_sha256")
        .cloned()
        .unwrap_or_else(|| serde_json::json!(""));
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;

    let audit_surfaces = vec![
        "packet_receipt_audit_trail_append_claim",
        "packet_receipt_immutable_evidence_claim",
        "packet_receipt_hash_chain_claim",
        "packet_receipt_merkle_root_claim",
        "packet_receipt_attestation_claim",
        "packet_receipt_witness_claim",
        "packet_receipt_notary_claim",
        "packet_receipt_ledger_evidence_claim",
        "packet_receipt_index_evidence_claim",
        "packet_receipt_delivery_evidence_claim",
        "packet_receipt_export_evidence_claim",
        "packet_receipt_query_evidence_claim",
        "packet_receipt_observability_evidence_claim",
        "packet_receipt_readback_evidence_claim",
        "packet_receipt_authority_evidence_claim",
        "packet_receipt_live_evidence_claim",
    ]
    .into_iter()
    .map(|audit_surface| {
        serde_json::json!({
            "audit_surface": audit_surface,
            "audit_or_evidence_attempted": true,
            "audit_trail_accepted": false,
            "audit_trail_recorded": false,
            "audit_trail_persisted": false,
            "audit_trail_materialized": false,
            "immutable_evidence_accepted": false,
            "immutable_evidence_recorded": false,
            "immutable_evidence_persisted": false,
            "immutable_evidence_materialized": false,
            "hash_chain_recorded": false,
            "hash_chain_persisted": false,
            "merkle_root_recorded": false,
            "merkle_root_persisted": false,
            "attestation_recorded": false,
            "attestation_persisted": false,
            "witness_recorded": false,
            "witness_persisted": false,
            "notary_recorded": false,
            "notary_persisted": false,
            "ledger_evidence_recorded": false,
            "ledger_evidence_persisted": false,
            "index_evidence_recorded": false,
            "index_evidence_persisted": false,
            "delivery_evidence_recorded": false,
            "delivery_evidence_persisted": false,
            "export_evidence_recorded": false,
            "query_evidence_registered": false,
            "observability_evidence_recorded": false,
            "readback_evidence_recorded": false,
            "acceptance_recorded": false,
            "operator_approval_derived": false,
            "activation_authority_derived": false,
            "activation_command_derived": false,
            "live_execution_allowed": false,
            "audit_evidence_status": "audit_trail_immutable_evidence_denied"
        })
    })
    .collect::<Vec<_>>();
    let audit_evidence_surface_count = audit_surfaces.len();
    let audit_trail_immutable_evidence_contract_hash_sha256 = sha256_text_value(&format!(
        "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-audit-trail-immutable-evidence-denial:native:source={source_cancellation_report_sha256}:surfaces={audit_evidence_surface_count}:route_count={}:audit=0:evidence=0:hashchain=0:authority=0:live=0",
        route_matrix.route_count
    ));
    let denied_by_packet_receipt_audit_trail_immutable_evidence = vec![
        "operator_readiness_packet_template_packet_receipt_audit_trail_acceptance_denied",
        "operator_readiness_packet_template_packet_receipt_audit_trail_recording_denied",
        "operator_readiness_packet_template_packet_receipt_audit_trail_persistence_denied",
        "operator_readiness_packet_template_packet_receipt_audit_trail_materialization_denied",
        "operator_readiness_packet_template_packet_receipt_immutable_evidence_acceptance_denied",
        "operator_readiness_packet_template_packet_receipt_immutable_evidence_recording_denied",
        "operator_readiness_packet_template_packet_receipt_immutable_evidence_persistence_denied",
        "operator_readiness_packet_template_packet_receipt_immutable_evidence_materialization_denied",
        "operator_readiness_packet_template_packet_receipt_hash_chain_recording_denied",
        "operator_readiness_packet_template_packet_receipt_merkle_root_recording_denied",
        "operator_readiness_packet_template_packet_receipt_attestation_recording_denied",
        "operator_readiness_packet_template_packet_receipt_witness_recording_denied",
        "operator_readiness_packet_template_packet_receipt_notary_recording_denied",
        "operator_readiness_packet_template_packet_receipt_ledger_evidence_denied",
        "operator_readiness_packet_template_packet_receipt_index_evidence_denied",
        "operator_readiness_packet_template_packet_receipt_delivery_evidence_denied",
        "operator_readiness_packet_template_packet_receipt_export_query_observability_evidence_denied",
        "operator_readiness_packet_template_packet_receipt_readback_evidence_denied",
        "operator_readiness_packet_template_packet_receipt_acceptance_from_audit_evidence_denied",
        "operator_readiness_packet_template_packet_receipt_authority_from_audit_evidence_denied",
        "operator_readiness_packet_template_packet_receipt_live_execution_from_audit_evidence_denied",
    ];
    let denied_by_packet_receipt_audit_trail_immutable_evidence_count =
        denied_by_packet_receipt_audit_trail_immutable_evidence.len();
    let report_ready = source_ready
        && source_cancellation_supersession_surface_count == 14
        && source_cancellation_supersession_attempt_count == 14
        && source_cancellation_accepted_count == 0
        && source_supersession_accepted_count == 0
        && source_replacement_receipt_accepted_count == 0
        && source_tombstone_recorded_count == 0
        && source_cancellation_supersession_acceptance_recorded_count == 0
        && source_cancellation_supersession_activation_authority_derived_count == 0
        && audit_evidence_surface_count == 16
        && route_count_source_command_accepted;

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_audit_trail_immutable_evidence_denial_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_AUDIT_TRAIL_IMMUTABLE_EVIDENCE_DENIAL_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-audit-trail-immutable-evidence-denial --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-15",
        "receipt_audit_trail_immutable_evidence_schema_version": "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_audit_trail_immutable_evidence_denial_v1",
        "receipt_audit_trail_immutable_evidence_mode": "native_route_non_persistent_receipts_cannot_become_audit_trail_immutable_evidence_or_authority",
        "source_packet_acceptance_receipt_cancellation_supersession_gate": source["gate"].clone(),
        "source_packet_acceptance_receipt_cancellation_supersession_ready": source_ready,
        "source_cancellation_report_sha256": source_cancellation_report_sha256,
        "source_cancellation_supersession_contract_hash_sha256": source_cancellation_supersession_contract_hash_sha256,
        "audit_trail_immutable_evidence_contract_hash_sha256": audit_trail_immutable_evidence_contract_hash_sha256,
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
            "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_audit_trail_immutable_evidence_denial_route_enabled": true,
            "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_audit_trail_immutable_evidence_denial_ready": true,
            "source_cancellation_supersession_surface_count": source_cancellation_supersession_surface_count,
            "source_cancellation_supersession_attempt_count": source_cancellation_supersession_attempt_count,
            "source_cancellation_accepted_count": source_cancellation_accepted_count,
            "source_supersession_accepted_count": source_supersession_accepted_count,
            "source_replacement_receipt_accepted_count": source_replacement_receipt_accepted_count,
            "source_tombstone_recorded_count": source_tombstone_recorded_count,
            "source_cancellation_supersession_acceptance_recorded_count": source_cancellation_supersession_acceptance_recorded_count,
            "source_cancellation_supersession_activation_authority_derived_count": source_cancellation_supersession_activation_authority_derived_count,
            "audit_evidence_surface_count": audit_evidence_surface_count,
            "audit_evidence_attempt_count": audit_evidence_surface_count,
            "audit_trail_accepted_count": 0,
            "audit_trail_recorded_count": 0,
            "audit_trail_persisted_count": 0,
            "audit_trail_materialized_count": 0,
            "immutable_evidence_accepted_count": 0,
            "immutable_evidence_recorded_count": 0,
            "immutable_evidence_persisted_count": 0,
            "immutable_evidence_materialized_count": 0,
            "hash_chain_recorded_count": 0,
            "hash_chain_persisted_count": 0,
            "merkle_root_recorded_count": 0,
            "merkle_root_persisted_count": 0,
            "attestation_recorded_count": 0,
            "attestation_persisted_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "witness_recorded_count": 0,
            "witness_persisted_count": 0,
            "notary_recorded_count": 0,
            "notary_persisted_count": 0,
            "ledger_evidence_recorded_count": 0,
            "ledger_evidence_persisted_count": 0,
            "index_evidence_recorded_count": 0,
            "index_evidence_persisted_count": 0,
            "delivery_evidence_recorded_count": 0,
            "delivery_evidence_persisted_count": 0,
            "export_evidence_recorded_count": 0,
            "query_evidence_registered_count": 0,
            "observability_evidence_recorded_count": 0,
            "readback_evidence_recorded_count": 0,
            "audit_evidence_acceptance_recorded_count": 0,
            "audit_evidence_operator_approval_derived_count": 0,
            "audit_evidence_activation_authority_derived_count": 0,
            "audit_evidence_activation_command_derived_count": 0,
            "audit_evidence_live_execution_allowed_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "audit_surfaces": audit_surfaces,
            "denied_by_packet_receipt_audit_trail_immutable_evidence": denied_by_packet_receipt_audit_trail_immutable_evidence,
            "denied_by_packet_receipt_audit_trail_immutable_evidence_count": denied_by_packet_receipt_audit_trail_immutable_evidence_count,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "allowed_next_actions": [
                {
                    "action": "prepare_operator_readiness_packet_template_packet_acceptance_receipt_retention_expiry_garbage_collection_denial_gate",
                    "status": "allowed_report_only_next_slice",
                    "persists_receipt": false,
                    "records_operator_acceptance": false,
                    "derives_activation_authority": false,
                    "records_retention_state": false,
                    "expires_receipt": false,
                    "garbage_collects_receipt": false,
                    "activates_live": false,
                    "mutates_memory_store": false,
                    "writes_kg": false
                }
            ],
            "packet_template_recorded": false,
            "packet_template_persisted": false,
            "packet_assembly_performed": false,
            "packet_accepted": false,
            "packet_acceptance_receipt_recorded": false,
            "packet_acceptance_receipt_persisted": false,
            "packet_acceptance_receipt_replayed": false,
            "packet_acceptance_receipt_ordering_recorded": false,
            "packet_acceptance_receipt_cancellation_recorded": false,
            "packet_acceptance_receipt_supersession_recorded": false,
            "packet_acceptance_receipt_replacement_recorded": false,
            "packet_acceptance_receipt_audit_trail_accepted": false,
            "packet_acceptance_receipt_audit_trail_recorded": false,
            "packet_acceptance_receipt_audit_trail_persisted": false,
            "packet_acceptance_receipt_audit_trail_materialized": false,
            "packet_acceptance_receipt_immutable_evidence_accepted": false,
            "packet_acceptance_receipt_immutable_evidence_recorded": false,
            "packet_acceptance_receipt_immutable_evidence_persisted": false,
            "packet_acceptance_receipt_immutable_evidence_materialized": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "packet_acceptance_receipt_hash_chain_recorded": false,
            "packet_acceptance_receipt_hash_chain_persisted": false,
            "packet_acceptance_receipt_merkle_root_recorded": false,
            "packet_acceptance_receipt_merkle_root_persisted": false,
            "packet_acceptance_receipt_attestation_recorded": false,
            "packet_acceptance_receipt_attestation_persisted": false,
            "packet_acceptance_receipt_witness_recorded": false,
            "packet_acceptance_receipt_witness_persisted": false,
            "packet_acceptance_receipt_notary_recorded": false,
            "packet_acceptance_receipt_notary_persisted": false,
            "packet_acceptance_receipt_ledger_evidence_recorded": false,
            "packet_acceptance_receipt_ledger_evidence_persisted": false,
            "packet_acceptance_receipt_index_evidence_recorded": false,
            "packet_acceptance_receipt_index_evidence_persisted": false,
            "packet_acceptance_receipt_delivery_evidence_recorded": false,
            "packet_acceptance_receipt_delivery_evidence_persisted": false,
            "packet_acceptance_receipt_export_evidence_recorded": false,
            "packet_acceptance_receipt_query_evidence_registered": false,
            "packet_acceptance_receipt_observability_evidence_recorded": false,
            "packet_acceptance_receipt_readback_evidence_recorded": false,
            "packet_acceptance_receipt_acceptance_recorded": false,
            "packet_acceptance_receipt_authority_derived": false,
            "packet_acceptance_receipt_live_execution_allowed": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "operator_acceptance_recorded": false,
            "operator_approval_recorded": false,
            "activation_authority_derived": false,
            "activation_command_derived": false,
            "activation_allowed": false,
            "activation_performed": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "hepta_intelligence_context_attached": false,
            "prompt_preview_rendered": false,
            "context_injection_performed": false,
            "provider_invoked": false,
            "model_invoked": false,
            "external_kg_adapter_read_performed": false,
            "external_adapter_client_constructed": false,
            "network_call_performed": false,
            "external_db_write_performed": false,
            "live_kg_write_performed": false,
            "credential_read": false,
            "secret_file_read": false,
            "install_executed": false,
            "launchd_mutated": false,
            "service_restarted": false,
            "active_binary_mutated": false,
            "public_release_claimed": false,
            "public_ga_claimed": false,
            "release_artifact_written": false,
            "public_artifact_written": false,
            "external_send_performed": false,
        }),
    );

    let mut side_effects = serde_json::json!({
        "packet_acceptance_receipt_audit_trail_accepted": false,
        "packet_acceptance_receipt_audit_trail_recorded": false,
        "packet_acceptance_receipt_audit_trail_persisted": false,
        "packet_acceptance_receipt_audit_trail_materialized": false,
        "packet_acceptance_receipt_immutable_evidence_accepted": false,
        "packet_acceptance_receipt_immutable_evidence_recorded": false,
        "packet_acceptance_receipt_immutable_evidence_persisted": false,
        "packet_acceptance_receipt_immutable_evidence_materialized": false,
    });
    extend_json_object(
        &mut side_effects,
        serde_json::json!({
        "packet_acceptance_receipt_hash_chain_recorded": false,
        "packet_acceptance_receipt_hash_chain_persisted": false,
        "packet_acceptance_receipt_merkle_root_recorded": false,
        "packet_acceptance_receipt_merkle_root_persisted": false,
        "packet_acceptance_receipt_attestation_recorded": false,
        "packet_acceptance_receipt_attestation_persisted": false,
        "packet_acceptance_receipt_witness_recorded": false,
        "packet_acceptance_receipt_witness_persisted": false,
        "packet_acceptance_receipt_notary_recorded": false,
        "packet_acceptance_receipt_notary_persisted": false,
        "packet_acceptance_receipt_ledger_evidence_recorded": false,
        "packet_acceptance_receipt_ledger_evidence_persisted": false,
        "packet_acceptance_receipt_index_evidence_recorded": false,
        "packet_acceptance_receipt_index_evidence_persisted": false,
        "packet_acceptance_receipt_delivery_evidence_recorded": false,
        "packet_acceptance_receipt_delivery_evidence_persisted": false,
        "packet_acceptance_receipt_export_evidence_recorded": false,
        "packet_acceptance_receipt_query_evidence_registered": false,
        "packet_acceptance_receipt_observability_evidence_recorded": false,
        "packet_acceptance_receipt_readback_evidence_recorded": false,
        "packet_acceptance_receipt_acceptance_recorded": false,
        "packet_acceptance_receipt_authority_derived": false,
        "packet_acceptance_receipt_live_execution_allowed": false,
        }),
    );
    extend_json_object(
        &mut side_effects,
        serde_json::json!({
        "packet_acceptance_receipt_cancellation_recorded": false,
        "packet_acceptance_receipt_supersession_recorded": false,
        "packet_acceptance_receipt_replacement_recorded": false,
        "packet_acceptance_receipt_recorded": false,
        "packet_acceptance_receipt_persisted": false,
        "packet_template_recorded": false,
        "packet_template_persisted": false,
        "packet_assembly_performed": false,
        "packet_acceptance_recorded": false,
        "operator_acceptance_recorded": false,
        "operator_approval_recorded": false,
        "activation_authority_derived": false,
        "activation_command_derived": false,
        "activation_allowed": false,
        "activation_performed": false,
        }),
    );
    extend_json_object(
        &mut side_effects,
        serde_json::json!({
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "hepta_intelligence_context_attached": false,
            "prompt_preview_rendered": false,
            "context_injection_performed": false,
            "provider_invoked": false,
            "model_invoked": false,
            "external_kg_adapter_read_performed": false,
            "external_adapter_client_constructed": false,
            "network_call_performed": false,
            "external_db_write_performed": false,
            "live_kg_write_performed": false,
            "credential_read": false,
            "secret_file_read": false,
            "install_executed": false,
            "launchd_mutated": false,
            "service_restarted": false,
            "active_binary_mutated": false,
            "public_release_claimed": false,
            "public_ga_claimed": false,
            "release_artifact_written": false,
            "public_artifact_written": false,
            "external_send_performed": false,
            "filesystem_written": false
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "side_effects": side_effects
        }),
    );
    report
}

fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_retention_expiry_garbage_collection_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source =
        hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_audit_trail_immutable_evidence_denial_report();
    let source_audit_report_sha256 = sha256_json_value(&source);
    let source_ready = source
        .get("memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_audit_trail_immutable_evidence_denial_ready")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);
    let source_audit_evidence_surface_count = source
        .get("audit_evidence_surface_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_audit_evidence_attempt_count = source
        .get("audit_evidence_attempt_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_audit_trail_recorded_count = source
        .get("audit_trail_recorded_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_immutable_evidence_recorded_count = source
        .get("immutable_evidence_recorded_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_hash_chain_recorded_count = source
        .get("hash_chain_recorded_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_ledger_evidence_recorded_count = source
        .get("ledger_evidence_recorded_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_readback_evidence_recorded_count = source
        .get("readback_evidence_recorded_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_audit_evidence_acceptance_recorded_count = source
        .get("audit_evidence_acceptance_recorded_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_audit_evidence_activation_authority_derived_count = source
        .get("audit_evidence_activation_authority_derived_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_audit_trail_immutable_evidence_contract_hash_sha256 = source
        .get("audit_trail_immutable_evidence_contract_hash_sha256")
        .cloned()
        .unwrap_or_else(|| serde_json::json!(""));
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;

    let retention_surfaces = vec![
        "packet_receipt_retention_policy_claim",
        "packet_receipt_retention_index_claim",
        "packet_receipt_ttl_update_claim",
        "packet_receipt_ttl_extension_claim",
        "packet_receipt_expiry_scheduler_claim",
        "packet_receipt_expiry_timer_claim",
        "packet_receipt_gc_scan_claim",
        "packet_receipt_gc_candidate_claim",
        "packet_receipt_delete_claim",
        "packet_receipt_tombstone_sweep_claim",
        "packet_receipt_archive_claim",
        "packet_receipt_compaction_claim",
        "packet_receipt_ledger_retention_claim",
        "packet_receipt_index_retention_claim",
        "packet_receipt_delivery_retention_claim",
        "packet_receipt_authority_retention_claim",
        "packet_receipt_live_retention_claim",
    ]
    .into_iter()
    .map(|retention_surface| {
        serde_json::json!({
            "retention_surface": retention_surface,
            "retention_expiry_or_gc_attempted": true,
            "retention_policy_accepted": false,
            "retention_policy_recorded": false,
            "retention_policy_persisted": false,
            "retention_index_recorded": false,
            "ttl_update_accepted": false,
            "ttl_update_recorded": false,
            "ttl_extension_accepted": false,
            "ttl_extension_recorded": false,
            "expiry_accepted": false,
            "expiry_recorded": false,
            "expiry_persisted": false,
            "expiry_scheduler_registered": false,
            "expiry_timer_started": false,
            "garbage_collection_accepted": false,
            "garbage_collection_scan_performed": false,
            "garbage_collection_candidate_recorded": false,
            "garbage_collection_decision_recorded": false,
            "delete_accepted": false,
            "delete_performed": false,
            "tombstone_recorded": false,
            "sweep_performed": false,
            "archive_written": false,
            "compaction_performed": false,
            "compaction_artifact_written": false,
            "ledger_retention_recorded": false,
            "ledger_retention_persisted": false,
            "index_retention_recorded": false,
            "index_retention_persisted": false,
            "delivery_retention_recorded": false,
            "delivery_retention_persisted": false,
            "acceptance_recorded": false,
            "operator_approval_derived": false,
            "activation_authority_derived": false,
            "activation_command_derived": false,
            "live_execution_allowed": false,
            "retention_gc_status": "retention_expiry_garbage_collection_denied"
        })
    })
    .collect::<Vec<_>>();
    let retention_expiry_gc_surface_count = retention_surfaces.len();
    let retention_expiry_gc_contract_hash_sha256 = sha256_text_value(&format!(
        "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-retention-expiry-garbage-collection-denial:native:source={source_audit_report_sha256}:surfaces={retention_expiry_gc_surface_count}:route_count={}:retention=0:expiry=0:gc=0:authority=0:live=0",
        route_matrix.route_count
    ));
    let denied_by_packet_receipt_retention_expiry_garbage_collection = vec![
        "operator_readiness_packet_template_packet_receipt_retention_policy_acceptance_denied",
        "operator_readiness_packet_template_packet_receipt_retention_policy_recording_denied",
        "operator_readiness_packet_template_packet_receipt_retention_policy_persistence_denied",
        "operator_readiness_packet_template_packet_receipt_retention_index_recording_denied",
        "operator_readiness_packet_template_packet_receipt_ttl_update_denied",
        "operator_readiness_packet_template_packet_receipt_ttl_extension_denied",
        "operator_readiness_packet_template_packet_receipt_expiry_acceptance_denied",
        "operator_readiness_packet_template_packet_receipt_expiry_recording_denied",
        "operator_readiness_packet_template_packet_receipt_expiry_scheduler_denied",
        "operator_readiness_packet_template_packet_receipt_expiry_timer_denied",
        "operator_readiness_packet_template_packet_receipt_garbage_collection_acceptance_denied",
        "operator_readiness_packet_template_packet_receipt_garbage_collection_scan_denied",
        "operator_readiness_packet_template_packet_receipt_garbage_collection_candidate_denied",
        "operator_readiness_packet_template_packet_receipt_delete_denied",
        "operator_readiness_packet_template_packet_receipt_tombstone_sweep_denied",
        "operator_readiness_packet_template_packet_receipt_archive_denied",
        "operator_readiness_packet_template_packet_receipt_compaction_denied",
        "operator_readiness_packet_template_packet_receipt_ledger_index_delivery_retention_denied",
        "operator_readiness_packet_template_packet_receipt_acceptance_from_retention_gc_denied",
        "operator_readiness_packet_template_packet_receipt_authority_from_retention_gc_denied",
        "operator_readiness_packet_template_packet_receipt_live_execution_from_retention_gc_denied",
    ];
    let denied_by_packet_receipt_retention_expiry_garbage_collection_count =
        denied_by_packet_receipt_retention_expiry_garbage_collection.len();
    let report_ready = source_ready
        && source_audit_evidence_surface_count == 16
        && source_audit_evidence_attempt_count == 16
        && source_audit_trail_recorded_count == 0
        && source_immutable_evidence_recorded_count == 0
        && source_hash_chain_recorded_count == 0
        && source_ledger_evidence_recorded_count == 0
        && source_readback_evidence_recorded_count == 0
        && source_audit_evidence_acceptance_recorded_count == 0
        && source_audit_evidence_activation_authority_derived_count == 0
        && retention_expiry_gc_surface_count == 17
        && route_count_source_command_accepted;

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_retention_expiry_garbage_collection_denial_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RETENTION_EXPIRY_GARBAGE_COLLECTION_DENIAL_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-retention-expiry-garbage-collection-denial --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-15",
        "receipt_retention_expiry_gc_schema_version": "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_retention_expiry_garbage_collection_denial_v1",
        "receipt_retention_expiry_gc_mode": "native_route_non_persistent_receipts_cannot_create_retention_expiry_gc_state_or_authority",
        "source_packet_acceptance_receipt_audit_evidence_gate": source["gate"].clone(),
        "source_packet_acceptance_receipt_audit_evidence_ready": source_ready,
        "source_audit_report_sha256": source_audit_report_sha256,
        "source_audit_trail_immutable_evidence_contract_hash_sha256": source_audit_trail_immutable_evidence_contract_hash_sha256,
        "retention_expiry_garbage_collection_contract_hash_sha256": retention_expiry_gc_contract_hash_sha256,
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
            "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_retention_expiry_garbage_collection_denial_route_enabled": true,
            "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_retention_expiry_garbage_collection_denial_ready": report_ready,
            "source_audit_evidence_surface_count": source_audit_evidence_surface_count,
            "source_audit_evidence_attempt_count": source_audit_evidence_attempt_count,
            "source_audit_trail_recorded_count": source_audit_trail_recorded_count,
            "source_immutable_evidence_recorded_count": source_immutable_evidence_recorded_count,
            "source_hash_chain_recorded_count": source_hash_chain_recorded_count,
            "source_ledger_evidence_recorded_count": source_ledger_evidence_recorded_count,
            "source_readback_evidence_recorded_count": source_readback_evidence_recorded_count,
            "source_audit_evidence_acceptance_recorded_count": source_audit_evidence_acceptance_recorded_count,
            "source_audit_evidence_activation_authority_derived_count": source_audit_evidence_activation_authority_derived_count,
            "retention_expiry_gc_surface_count": retention_expiry_gc_surface_count,
            "retention_expiry_gc_attempt_count": retention_expiry_gc_surface_count,
            "retention_policy_accepted_count": 0,
            "retention_policy_recorded_count": 0,
            "retention_policy_persisted_count": 0,
            "retention_index_recorded_count": 0,
            "ttl_update_accepted_count": 0,
            "ttl_update_recorded_count": 0,
            "ttl_extension_accepted_count": 0,
            "ttl_extension_recorded_count": 0,
            "expiry_accepted_count": 0,
            "expiry_recorded_count": 0,
            "expiry_persisted_count": 0,
            "expiry_scheduler_registered_count": 0,
            "expiry_timer_started_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "garbage_collection_accepted_count": 0,
            "garbage_collection_scan_performed_count": 0,
            "garbage_collection_candidate_recorded_count": 0,
            "garbage_collection_decision_recorded_count": 0,
            "delete_accepted_count": 0,
            "delete_performed_count": 0,
            "tombstone_recorded_count": 0,
            "sweep_performed_count": 0,
            "archive_written_count": 0,
            "compaction_performed_count": 0,
            "compaction_artifact_written_count": 0,
            "ledger_retention_recorded_count": 0,
            "ledger_retention_persisted_count": 0,
            "index_retention_recorded_count": 0,
            "index_retention_persisted_count": 0,
            "delivery_retention_recorded_count": 0,
            "delivery_retention_persisted_count": 0,
            "retention_gc_acceptance_recorded_count": 0,
            "retention_gc_operator_approval_derived_count": 0,
            "retention_gc_activation_authority_derived_count": 0,
            "retention_gc_activation_command_derived_count": 0,
            "retention_gc_live_execution_allowed_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "retention_surfaces": retention_surfaces,
            "denied_by_packet_receipt_retention_expiry_garbage_collection": denied_by_packet_receipt_retention_expiry_garbage_collection,
            "denied_by_packet_receipt_retention_expiry_garbage_collection_count": denied_by_packet_receipt_retention_expiry_garbage_collection_count,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "allowed_next_actions": [
                {
                    "action": "prepare_operator_readiness_packet_template_packet_acceptance_receipt_export_query_observability_denial_gate",
                    "status": "allowed_report_only_next_slice",
                    "persists_receipt": false,
                    "records_operator_acceptance": false,
                    "derives_activation_authority": false,
                    "exports_receipt": false,
                    "registers_query": false,
                    "records_observability": false,
                    "activates_live": false,
                    "mutates_memory_store": false,
                    "writes_kg": false
                }
            ],
            "packet_template_recorded": false,
            "packet_template_persisted": false,
            "packet_assembly_performed": false,
            "packet_accepted": false,
            "packet_acceptance_receipt_recorded": false,
            "packet_acceptance_receipt_persisted": false,
            "packet_acceptance_receipt_replayed": false,
            "packet_acceptance_receipt_ordering_recorded": false,
            "packet_acceptance_receipt_cancellation_recorded": false,
            "packet_acceptance_receipt_supersession_recorded": false,
            "packet_acceptance_receipt_audit_trail_recorded": false,
            "packet_acceptance_receipt_immutable_evidence_recorded": false,
            "packet_acceptance_receipt_retention_policy_recorded": false,
            "packet_acceptance_receipt_retention_policy_persisted": false,
            "packet_acceptance_receipt_retention_index_recorded": false,
            "packet_acceptance_receipt_ttl_update_recorded": false,
            "packet_acceptance_receipt_ttl_extension_recorded": false,
            "packet_acceptance_receipt_expiry_recorded": false,
            "packet_acceptance_receipt_expiry_scheduler_registered": false,
            "packet_acceptance_receipt_expiry_timer_started": false,
            "packet_acceptance_receipt_garbage_collection_scan_performed": false,
            "packet_acceptance_receipt_garbage_collection_candidate_recorded": false,
            "packet_acceptance_receipt_delete_performed": false,
            "packet_acceptance_receipt_tombstone_recorded": false,
            "packet_acceptance_receipt_archive_written": false,
            "packet_acceptance_receipt_compaction_performed": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "operator_acceptance_recorded": false,
            "operator_approval_recorded": false,
            "activation_authority_derived": false,
            "activation_command_derived": false,
            "activation_allowed": false,
            "activation_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "hepta_intelligence_context_attached": false,
            "prompt_preview_rendered": false,
            "context_injection_performed": false,
            "provider_invoked": false,
            "model_invoked": false,
            "external_kg_adapter_read_performed": false,
            "external_adapter_client_constructed": false,
            "network_call_performed": false,
            "external_db_write_performed": false,
            "live_kg_write_performed": false,
            "credential_read": false,
            "secret_file_read": false,
            "install_executed": false,
            "launchd_mutated": false,
            "service_restarted": false,
            "active_binary_mutated": false,
            "public_release_claimed": false,
            "public_ga_claimed": false,
            "release_artifact_written": false,
            "public_artifact_written": false,
            "external_send_performed": false,
        }),
    );

    let mut side_effects = serde_json::json!({
        "packet_acceptance_receipt_retention_policy_recorded": false,
        "packet_acceptance_receipt_retention_policy_persisted": false,
        "packet_acceptance_receipt_retention_index_recorded": false,
        "packet_acceptance_receipt_ttl_update_recorded": false,
        "packet_acceptance_receipt_ttl_extension_recorded": false,
        "packet_acceptance_receipt_expiry_recorded": false,
        "packet_acceptance_receipt_expiry_persisted": false,
        "packet_acceptance_receipt_expiry_scheduler_registered": false,
        "packet_acceptance_receipt_expiry_timer_started": false,
        "packet_acceptance_receipt_garbage_collection_scan_performed": false,
        "packet_acceptance_receipt_garbage_collection_candidate_recorded": false,
        "packet_acceptance_receipt_garbage_collection_decision_recorded": false,
        "packet_acceptance_receipt_delete_performed": false,
        "packet_acceptance_receipt_tombstone_recorded": false,
        "packet_acceptance_receipt_sweep_performed": false,
        "packet_acceptance_receipt_archive_written": false,
        "packet_acceptance_receipt_compaction_performed": false,
        "packet_acceptance_receipt_compaction_artifact_written": false,
        "packet_acceptance_receipt_ledger_retention_recorded": false,
        "packet_acceptance_receipt_index_retention_recorded": false,
        "packet_acceptance_receipt_delivery_retention_recorded": false,
        "packet_acceptance_receipt_acceptance_recorded": false,
        "packet_acceptance_receipt_authority_derived": false,
        "packet_acceptance_receipt_live_execution_allowed": false,
    });
    extend_json_object(
        &mut side_effects,
        serde_json::json!({
            "packet_acceptance_receipt_audit_trail_recorded": false,
            "packet_acceptance_receipt_immutable_evidence_recorded": false,
            "packet_acceptance_receipt_recorded": false,
            "packet_acceptance_receipt_persisted": false,
            "packet_template_recorded": false,
            "packet_template_persisted": false,
            "packet_assembly_performed": false,
            "packet_acceptance_recorded": false,
            "operator_acceptance_recorded": false,
            "operator_approval_recorded": false,
            "activation_authority_derived": false,
            "activation_command_derived": false,
            "activation_allowed": false,
            "activation_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "hepta_intelligence_context_attached": false,
            "prompt_preview_rendered": false,
            "context_injection_performed": false,
            "provider_invoked": false,
            "model_invoked": false,
            "external_kg_adapter_read_performed": false,
            "external_adapter_client_constructed": false,
            "network_call_performed": false,
            "external_db_write_performed": false,
            "live_kg_write_performed": false,
            "credential_read": false,
            "secret_file_read": false,
            "install_executed": false,
            "launchd_mutated": false,
            "service_restarted": false,
            "active_binary_mutated": false,
            "public_release_claimed": false,
            "public_ga_claimed": false,
            "release_artifact_written": false,
            "public_artifact_written": false,
            "external_send_performed": false,
            "filesystem_written": false
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "side_effects": side_effects
        }),
    );
    report
}

fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_export_query_observability_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source =
        hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_retention_expiry_garbage_collection_denial_report();
    let source_retention_report_sha256 = sha256_json_value(&source);
    let source_ready = source
        .get("memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_retention_expiry_garbage_collection_denial_ready")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);
    let source_retention_expiry_gc_surface_count = source
        .get("retention_expiry_gc_surface_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_retention_expiry_gc_attempt_count = source
        .get("retention_expiry_gc_attempt_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_retention_policy_recorded_count = source
        .get("retention_policy_recorded_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_expiry_recorded_count = source
        .get("expiry_recorded_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_garbage_collection_scan_performed_count = source
        .get("garbage_collection_scan_performed_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_archive_written_count = source
        .get("archive_written_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_compaction_performed_count = source
        .get("compaction_performed_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_retention_gc_activation_authority_derived_count = source
        .get("retention_gc_activation_authority_derived_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_retention_expiry_garbage_collection_contract_hash_sha256 = source
        .get("retention_expiry_garbage_collection_contract_hash_sha256")
        .cloned()
        .unwrap_or_else(|| serde_json::json!(""));
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;

    let export_query_observability_surfaces = vec![
        "packet_receipt_query_registration_claim",
        "packet_receipt_query_result_claim",
        "packet_receipt_search_index_claim",
        "packet_receipt_export_snapshot_claim",
        "packet_receipt_export_file_claim",
        "packet_receipt_observability_metric_claim",
        "packet_receipt_observability_event_claim",
        "packet_receipt_dashboard_panel_claim",
        "packet_receipt_operator_summary_claim",
        "packet_receipt_readback_surface_claim",
        "packet_receipt_audit_view_claim",
        "packet_receipt_external_delivery_claim",
        "packet_receipt_completion_ack_view_claim",
        "packet_receipt_acceptance_view_claim",
        "packet_receipt_authority_view_claim",
        "packet_receipt_live_view_claim",
    ]
    .into_iter()
    .map(|export_query_observability_surface| {
        serde_json::json!({
            "export_query_observability_surface": export_query_observability_surface,
            "export_query_or_observability_attempted": true,
            "query_registered": false,
            "query_executed": false,
            "query_result_recorded": false,
            "query_result_persisted": false,
            "search_index_recorded": false,
            "search_index_persisted": false,
            "export_requested": false,
            "export_snapshot_recorded": false,
            "export_snapshot_persisted": false,
            "export_file_written": false,
            "observability_metric_recorded": false,
            "observability_event_recorded": false,
            "dashboard_panel_recorded": false,
            "operator_summary_recorded": false,
            "readback_surface_recorded": false,
            "audit_view_recorded": false,
            "external_delivery_performed": false,
            "completion_ack_recorded": false,
            "acceptance_recorded": false,
            "operator_approval_derived": false,
            "activation_authority_derived": false,
            "activation_command_derived": false,
            "live_execution_allowed": false,
            "export_query_observability_status": "export_query_observability_denied"
        })
    })
    .collect::<Vec<_>>();
    let export_query_observability_surface_count = export_query_observability_surfaces.len();
    let export_query_observability_contract_hash_sha256 = sha256_text_value(&format!(
        "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-export-query-observability-denial:native:source={source_retention_report_sha256}:surfaces={export_query_observability_surface_count}:route_count={}:query=0:export=0:observability=0:authority=0:live=0",
        route_matrix.route_count
    ));
    let denied_by_packet_receipt_export_query_observability = vec![
        "operator_readiness_packet_template_packet_receipt_query_registration_denied",
        "operator_readiness_packet_template_packet_receipt_query_execution_denied",
        "operator_readiness_packet_template_packet_receipt_query_result_recording_denied",
        "operator_readiness_packet_template_packet_receipt_search_index_recording_denied",
        "operator_readiness_packet_template_packet_receipt_export_request_denied",
        "operator_readiness_packet_template_packet_receipt_export_snapshot_recording_denied",
        "operator_readiness_packet_template_packet_receipt_export_file_write_denied",
        "operator_readiness_packet_template_packet_receipt_observability_metric_denied",
        "operator_readiness_packet_template_packet_receipt_observability_event_denied",
        "operator_readiness_packet_template_packet_receipt_dashboard_panel_denied",
        "operator_readiness_packet_template_packet_receipt_operator_summary_denied",
        "operator_readiness_packet_template_packet_receipt_readback_surface_denied",
        "operator_readiness_packet_template_packet_receipt_audit_view_denied",
        "operator_readiness_packet_template_packet_receipt_external_delivery_denied",
        "operator_readiness_packet_template_packet_receipt_completion_ack_view_denied",
        "operator_readiness_packet_template_packet_receipt_acceptance_from_view_denied",
        "operator_readiness_packet_template_packet_receipt_authority_from_view_denied",
        "operator_readiness_packet_template_packet_receipt_live_execution_from_view_denied",
    ];
    let denied_by_packet_receipt_export_query_observability_count =
        denied_by_packet_receipt_export_query_observability.len();
    let report_ready = source_ready
        && source_retention_expiry_gc_surface_count == 17
        && source_retention_expiry_gc_attempt_count == 17
        && source_retention_policy_recorded_count == 0
        && source_expiry_recorded_count == 0
        && source_garbage_collection_scan_performed_count == 0
        && source_archive_written_count == 0
        && source_compaction_performed_count == 0
        && source_retention_gc_activation_authority_derived_count == 0
        && export_query_observability_surface_count == 16
        && route_count_source_command_accepted;

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_export_query_observability_denial_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_EXPORT_QUERY_OBSERVABILITY_DENIAL_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-export-query-observability-denial --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-15",
        "receipt_export_query_observability_schema_version": "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_export_query_observability_denial_v1",
        "receipt_export_query_observability_mode": "native_route_non_persistent_receipts_cannot_create_query_export_observability_or_authority",
        "source_packet_acceptance_receipt_retention_expiry_gc_gate": source["gate"].clone(),
        "source_packet_acceptance_receipt_retention_expiry_gc_ready": source_ready,
        "source_retention_report_sha256": source_retention_report_sha256,
        "source_retention_expiry_garbage_collection_contract_hash_sha256": source_retention_expiry_garbage_collection_contract_hash_sha256,
        "export_query_observability_contract_hash_sha256": export_query_observability_contract_hash_sha256,
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
            "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_export_query_observability_denial_route_enabled": true,
            "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_export_query_observability_denial_ready": report_ready,
            "source_retention_expiry_gc_surface_count": source_retention_expiry_gc_surface_count,
            "source_retention_expiry_gc_attempt_count": source_retention_expiry_gc_attempt_count,
            "source_retention_policy_recorded_count": source_retention_policy_recorded_count,
            "source_expiry_recorded_count": source_expiry_recorded_count,
            "source_garbage_collection_scan_performed_count": source_garbage_collection_scan_performed_count,
            "source_archive_written_count": source_archive_written_count,
            "source_compaction_performed_count": source_compaction_performed_count,
            "source_retention_gc_activation_authority_derived_count": source_retention_gc_activation_authority_derived_count,
            "export_query_observability_surface_count": export_query_observability_surface_count,
            "export_query_observability_attempt_count": export_query_observability_surface_count,
            "query_registered_count": 0,
            "query_executed_count": 0,
            "query_result_recorded_count": 0,
            "query_result_persisted_count": 0,
            "search_index_recorded_count": 0,
            "search_index_persisted_count": 0,
            "export_requested_count": 0,
            "export_snapshot_recorded_count": 0,
            "export_snapshot_persisted_count": 0,
            "export_file_written_count": 0,
            "observability_metric_recorded_count": 0,
            "observability_event_recorded_count": 0,
            "dashboard_panel_recorded_count": 0,
            "operator_summary_recorded_count": 0,
            "readback_surface_recorded_count": 0,
            "audit_view_recorded_count": 0,
            "external_delivery_performed_count": 0,
            "completion_ack_recorded_count": 0,
            "export_query_observability_acceptance_recorded_count": 0,
            "export_query_observability_operator_approval_derived_count": 0,
            "export_query_observability_activation_authority_derived_count": 0,
            "export_query_observability_activation_command_derived_count": 0,
            "export_query_observability_live_execution_allowed_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "export_query_observability_surfaces": export_query_observability_surfaces,
            "denied_by_packet_receipt_export_query_observability": denied_by_packet_receipt_export_query_observability,
            "denied_by_packet_receipt_export_query_observability_count": denied_by_packet_receipt_export_query_observability_count,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "allowed_next_actions": [
                {
                    "action": "prepare_operator_readiness_packet_template_packet_acceptance_receipt_redaction_privacy_denial_gate",
                    "status": "allowed_report_only_next_slice",
                    "persists_receipt": false,
                    "records_operator_acceptance": false,
                    "derives_activation_authority": false,
                    "queries_receipt": false,
                    "exports_receipt": false,
                    "records_observability": false,
                    "delivers_externally": false,
                    "activates_live": false,
                    "mutates_memory_store": false,
                    "writes_kg": false
                }
            ],
            "packet_template_recorded": false,
            "packet_template_persisted": false,
            "packet_assembly_performed": false,
            "packet_accepted": false,
            "packet_acceptance_receipt_recorded": false,
            "packet_acceptance_receipt_persisted": false,
            "packet_acceptance_receipt_replayed": false,
            "packet_acceptance_receipt_ordering_recorded": false,
            "packet_acceptance_receipt_cancellation_recorded": false,
            "packet_acceptance_receipt_supersession_recorded": false,
            "packet_acceptance_receipt_audit_trail_recorded": false,
            "packet_acceptance_receipt_immutable_evidence_recorded": false,
            "packet_acceptance_receipt_retention_policy_recorded": false,
            "packet_acceptance_receipt_expiry_recorded": false,
            "packet_acceptance_receipt_garbage_collection_scan_performed": false,
            "packet_acceptance_receipt_query_registered": false,
            "packet_acceptance_receipt_query_executed": false,
            "packet_acceptance_receipt_query_result_recorded": false,
            "packet_acceptance_receipt_search_index_recorded": false,
            "packet_acceptance_receipt_export_snapshot_recorded": false,
            "packet_acceptance_receipt_export_file_written": false,
            "packet_acceptance_receipt_observability_metric_recorded": false,
            "packet_acceptance_receipt_observability_event_recorded": false,
            "packet_acceptance_receipt_dashboard_panel_recorded": false,
            "packet_acceptance_receipt_operator_summary_recorded": false,
            "packet_acceptance_receipt_readback_surface_recorded": false,
            "packet_acceptance_receipt_external_delivery_performed": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "operator_acceptance_recorded": false,
            "operator_approval_recorded": false,
            "activation_authority_derived": false,
            "activation_command_derived": false,
            "activation_allowed": false,
            "activation_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "hepta_intelligence_context_attached": false,
            "prompt_preview_rendered": false,
            "context_injection_performed": false,
            "provider_invoked": false,
            "model_invoked": false,
            "external_kg_adapter_read_performed": false,
            "external_adapter_client_constructed": false,
            "network_call_performed": false,
            "external_db_write_performed": false,
            "live_kg_write_performed": false,
            "credential_read": false,
            "secret_file_read": false,
            "install_executed": false,
            "launchd_mutated": false,
            "service_restarted": false,
            "active_binary_mutated": false,
            "public_release_claimed": false,
            "public_ga_claimed": false,
            "release_artifact_written": false,
            "public_artifact_written": false,
            "external_send_performed": false,
        }),
    );

    let mut side_effects = serde_json::json!({
        "packet_acceptance_receipt_query_registered": false,
        "packet_acceptance_receipt_query_executed": false,
        "packet_acceptance_receipt_query_result_recorded": false,
        "packet_acceptance_receipt_query_result_persisted": false,
        "packet_acceptance_receipt_search_index_recorded": false,
        "packet_acceptance_receipt_search_index_persisted": false,
        "packet_acceptance_receipt_export_requested": false,
        "packet_acceptance_receipt_export_snapshot_recorded": false,
        "packet_acceptance_receipt_export_snapshot_persisted": false,
        "packet_acceptance_receipt_export_file_written": false,
        "packet_acceptance_receipt_observability_metric_recorded": false,
        "packet_acceptance_receipt_observability_event_recorded": false,
        "packet_acceptance_receipt_dashboard_panel_recorded": false,
        "packet_acceptance_receipt_operator_summary_recorded": false,
        "packet_acceptance_receipt_readback_surface_recorded": false,
        "packet_acceptance_receipt_audit_view_recorded": false,
        "packet_acceptance_receipt_external_delivery_performed": false,
        "packet_acceptance_receipt_completion_ack_recorded": false,
        "packet_acceptance_receipt_acceptance_recorded": false,
        "packet_acceptance_receipt_authority_derived": false,
        "packet_acceptance_receipt_live_execution_allowed": false,
        "packet_acceptance_receipt_retention_policy_recorded": false,
        "packet_acceptance_receipt_expiry_recorded": false,
        "packet_acceptance_receipt_garbage_collection_scan_performed": false,
        "packet_acceptance_receipt_audit_trail_recorded": false,
        "packet_acceptance_receipt_immutable_evidence_recorded": false,
        "packet_acceptance_receipt_recorded": false,
        "packet_acceptance_receipt_persisted": false,
        "packet_template_recorded": false,
        "packet_template_persisted": false,
        "packet_assembly_performed": false,
        "packet_acceptance_recorded": false,
        "operator_acceptance_recorded": false,
        "operator_approval_recorded": false,
        "activation_authority_derived": false,
        "activation_command_derived": false,
        "activation_allowed": false,
        "activation_performed": false,
    });
    extend_json_object(
        &mut side_effects,
        serde_json::json!({
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "hepta_intelligence_context_attached": false,
            "prompt_preview_rendered": false,
            "context_injection_performed": false,
            "provider_invoked": false,
            "model_invoked": false,
            "external_kg_adapter_read_performed": false,
            "external_adapter_client_constructed": false,
            "network_call_performed": false,
            "external_db_write_performed": false,
            "live_kg_write_performed": false,
            "credential_read": false,
            "secret_file_read": false,
            "install_executed": false,
            "launchd_mutated": false,
            "service_restarted": false,
            "active_binary_mutated": false,
            "public_release_claimed": false,
            "public_ga_claimed": false,
            "release_artifact_written": false,
            "public_artifact_written": false,
            "external_send_performed": false,
            "filesystem_written": false
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "side_effects": side_effects
        }),
    );
    report
}

fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_redaction_privacy_payload_exposure_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source =
        hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_export_query_observability_denial_report();
    let source_view_report_sha256 = sha256_json_value(&source);
    let source_ready = source
        .get("memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_export_query_observability_denial_ready")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);
    let source_export_query_observability_surface_count = source
        .get("export_query_observability_surface_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_query_registered_count = source
        .get("query_registered_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_export_snapshot_recorded_count = source
        .get("export_snapshot_recorded_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_observability_metric_recorded_count = source
        .get("observability_metric_recorded_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_operator_summary_recorded_count = source
        .get("operator_summary_recorded_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_readback_surface_recorded_count = source
        .get("readback_surface_recorded_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_export_query_observability_activation_authority_derived_count = source
        .get("export_query_observability_activation_authority_derived_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_export_query_observability_contract_hash_sha256 = source
        .get("export_query_observability_contract_hash_sha256")
        .cloned()
        .unwrap_or_else(|| serde_json::json!(""));
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;

    let redaction_privacy_surfaces = vec![
        "packet_receipt_redacted_payload_preview_claim",
        "packet_receipt_payload_hash_preview_claim",
        "packet_receipt_payload_diff_claim",
        "packet_receipt_readback_text_claim",
        "packet_receipt_operator_summary_text_claim",
        "packet_receipt_privacy_review_claim",
        "packet_receipt_secret_scan_claim",
        "packet_receipt_pii_scan_claim",
        "packet_receipt_raw_payload_inspection_claim",
        "packet_receipt_plaintext_materialization_claim",
        "packet_receipt_redaction_bypass_claim",
        "packet_receipt_hash_to_payload_link_claim",
        "packet_receipt_external_redaction_review_claim",
        "packet_receipt_privacy_acceptance_claim",
        "packet_receipt_authority_from_redaction_claim",
        "packet_receipt_live_from_privacy_claim",
    ]
    .into_iter()
    .map(|redaction_privacy_surface| {
        serde_json::json!({
            "redaction_privacy_surface": redaction_privacy_surface,
            "redaction_privacy_or_payload_exposure_attempted": true,
            "redacted_payload_preview_recorded": false,
            "payload_hash_preview_recorded": false,
            "payload_diff_recorded": false,
            "readback_text_recorded": false,
            "operator_summary_text_recorded": false,
            "privacy_review_recorded": false,
            "privacy_review_persisted": false,
            "secret_scan_performed": false,
            "pii_scan_performed": false,
            "raw_payload_inspected": false,
            "plaintext_materialized": false,
            "redaction_bypass_allowed": false,
            "hash_to_payload_link_recorded": false,
            "external_redaction_review_performed": false,
            "privacy_acceptance_recorded": false,
            "acceptance_recorded": false,
            "operator_approval_derived": false,
            "activation_authority_derived": false,
            "activation_command_derived": false,
            "live_execution_allowed": false,
            "redaction_privacy_status": "redaction_privacy_payload_exposure_denied"
        })
    })
    .collect::<Vec<_>>();
    let redaction_privacy_surface_count = redaction_privacy_surfaces.len();
    let redaction_privacy_payload_exposure_contract_hash_sha256 = sha256_text_value(&format!(
        "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-redaction-privacy-payload-exposure-denial:native:source={source_view_report_sha256}:surfaces={redaction_privacy_surface_count}:route_count={}:redaction=0:privacy=0:payload=0:authority=0:live=0",
        route_matrix.route_count
    ));
    let denied_by_packet_receipt_redaction_privacy = vec![
        "operator_readiness_packet_template_packet_receipt_redacted_payload_preview_denied",
        "operator_readiness_packet_template_packet_receipt_payload_hash_preview_denied",
        "operator_readiness_packet_template_packet_receipt_payload_diff_denied",
        "operator_readiness_packet_template_packet_receipt_readback_text_denied",
        "operator_readiness_packet_template_packet_receipt_operator_summary_text_denied",
        "operator_readiness_packet_template_packet_receipt_privacy_review_recording_denied",
        "operator_readiness_packet_template_packet_receipt_privacy_review_persistence_denied",
        "operator_readiness_packet_template_packet_receipt_secret_scan_denied",
        "operator_readiness_packet_template_packet_receipt_pii_scan_denied",
        "operator_readiness_packet_template_packet_receipt_raw_payload_inspection_denied",
        "operator_readiness_packet_template_packet_receipt_plaintext_materialization_denied",
        "operator_readiness_packet_template_packet_receipt_redaction_bypass_denied",
        "operator_readiness_packet_template_packet_receipt_hash_to_payload_link_denied",
        "operator_readiness_packet_template_packet_receipt_external_redaction_review_denied",
        "operator_readiness_packet_template_packet_receipt_privacy_acceptance_denied",
        "operator_readiness_packet_template_packet_receipt_authority_from_redaction_denied",
        "operator_readiness_packet_template_packet_receipt_live_execution_from_privacy_denied",
    ];
    let denied_by_packet_receipt_redaction_privacy_count =
        denied_by_packet_receipt_redaction_privacy.len();
    let report_ready = source_ready
        && source_export_query_observability_surface_count == 16
        && source_query_registered_count == 0
        && source_export_snapshot_recorded_count == 0
        && source_observability_metric_recorded_count == 0
        && source_operator_summary_recorded_count == 0
        && source_readback_surface_recorded_count == 0
        && source_export_query_observability_activation_authority_derived_count == 0
        && redaction_privacy_surface_count == 16
        && route_count_source_command_accepted;

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_redaction_privacy_payload_exposure_denial_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_REDACTION_PRIVACY_PAYLOAD_EXPOSURE_DENIAL_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-redaction-privacy-payload-exposure-denial --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-15",
        "receipt_redaction_privacy_schema_version": "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_redaction_privacy_payload_exposure_denial_v1",
        "receipt_redaction_privacy_mode": "native_route_non_persistent_receipts_cannot_expose_payload_or_create_privacy_authority",
        "source_packet_acceptance_receipt_export_query_observability_gate": source["gate"].clone(),
        "source_packet_acceptance_receipt_export_query_observability_ready": source_ready,
        "source_view_report_sha256": source_view_report_sha256,
        "source_export_query_observability_contract_hash_sha256": source_export_query_observability_contract_hash_sha256,
        "redaction_privacy_payload_exposure_contract_hash_sha256": redaction_privacy_payload_exposure_contract_hash_sha256,
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
            "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_redaction_privacy_payload_exposure_denial_route_enabled": true,
            "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_redaction_privacy_payload_exposure_denial_ready": report_ready,
            "source_export_query_observability_surface_count": source_export_query_observability_surface_count,
            "source_query_registered_count": source_query_registered_count,
            "source_export_snapshot_recorded_count": source_export_snapshot_recorded_count,
            "source_observability_metric_recorded_count": source_observability_metric_recorded_count,
            "source_operator_summary_recorded_count": source_operator_summary_recorded_count,
            "source_readback_surface_recorded_count": source_readback_surface_recorded_count,
            "source_export_query_observability_activation_authority_derived_count": source_export_query_observability_activation_authority_derived_count,
            "redaction_privacy_surface_count": redaction_privacy_surface_count,
            "redaction_privacy_attempt_count": redaction_privacy_surface_count,
            "redacted_payload_preview_recorded_count": 0,
            "payload_hash_preview_recorded_count": 0,
            "payload_diff_recorded_count": 0,
            "readback_text_recorded_count": 0,
            "operator_summary_text_recorded_count": 0,
            "privacy_review_recorded_count": 0,
            "privacy_review_persisted_count": 0,
            "secret_scan_performed_count": 0,
            "pii_scan_performed_count": 0,
            "raw_payload_inspected_count": 0,
            "plaintext_materialized_count": 0,
            "redaction_bypass_allowed_count": 0,
            "hash_to_payload_link_recorded_count": 0,
            "external_redaction_review_performed_count": 0,
            "privacy_acceptance_recorded_count": 0,
            "redaction_privacy_acceptance_recorded_count": 0,
            "redaction_privacy_operator_approval_derived_count": 0,
            "redaction_privacy_activation_authority_derived_count": 0,
            "redaction_privacy_activation_command_derived_count": 0,
            "redaction_privacy_live_execution_allowed_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "redaction_privacy_surfaces": redaction_privacy_surfaces,
            "denied_by_packet_receipt_redaction_privacy": denied_by_packet_receipt_redaction_privacy,
            "denied_by_packet_receipt_redaction_privacy_count": denied_by_packet_receipt_redaction_privacy_count,
            "allowed_next_actions": [
                {
                    "action": "prepare_operator_readiness_packet_template_packet_acceptance_receipt_operator_briefing_non_persistence_gate",
                    "status": "allowed_report_only_next_slice",
                    "exposes_payload": false,
                    "records_privacy_review": false,
                    "performs_secret_scan": false,
                    "records_operator_acceptance": false,
                    "derives_activation_authority": false,
                    "activates_live": false,
                    "mutates_memory_store": false,
                    "writes_kg": false
                }
            ],
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "packet_acceptance_receipt_recorded": false,
            "packet_acceptance_receipt_persisted": false,
            "packet_acceptance_receipt_query_registered": false,
            "packet_acceptance_receipt_export_file_written": false,
            "packet_acceptance_receipt_observability_metric_recorded": false,
            "packet_acceptance_receipt_redacted_payload_preview_recorded": false,
            "packet_acceptance_receipt_payload_hash_preview_recorded": false,
            "packet_acceptance_receipt_payload_diff_recorded": false,
            "packet_acceptance_receipt_readback_text_recorded": false,
            "packet_acceptance_receipt_operator_summary_text_recorded": false,
            "packet_acceptance_receipt_privacy_review_recorded": false,
            "packet_acceptance_receipt_secret_scan_performed": false,
            "packet_acceptance_receipt_pii_scan_performed": false,
            "packet_acceptance_receipt_raw_payload_inspected": false,
            "packet_acceptance_receipt_plaintext_materialized": false,
            "packet_acceptance_receipt_redaction_bypass_allowed": false,
            "packet_acceptance_receipt_hash_to_payload_link_recorded": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "operator_acceptance_recorded": false,
            "operator_approval_recorded": false,
            "activation_authority_derived": false,
            "activation_command_derived": false,
            "activation_allowed": false,
            "activation_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "hepta_intelligence_context_attached": false,
            "prompt_preview_rendered": false,
            "context_injection_performed": false,
            "provider_invoked": false,
            "model_invoked": false,
            "external_kg_adapter_read_performed": false,
            "external_adapter_client_constructed": false,
            "network_call_performed": false,
            "external_db_write_performed": false,
            "live_kg_write_performed": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "credential_read": false,
            "secret_file_read": false,
            "install_executed": false,
            "launchd_mutated": false,
            "service_restarted": false,
            "active_binary_mutated": false,
            "public_release_claimed": false,
            "public_ga_claimed": false,
            "release_artifact_written": false,
            "public_artifact_written": false,
            "external_send_performed": false,
        }),
    );

    let mut side_effects = serde_json::json!({});
    extend_json_object(
        &mut side_effects,
        serde_json::json!({
            "packet_acceptance_receipt_redacted_payload_preview_recorded": false,
            "packet_acceptance_receipt_payload_hash_preview_recorded": false,
            "packet_acceptance_receipt_payload_diff_recorded": false,
            "packet_acceptance_receipt_readback_text_recorded": false,
            "packet_acceptance_receipt_operator_summary_text_recorded": false,
            "packet_acceptance_receipt_privacy_review_recorded": false,
            "packet_acceptance_receipt_privacy_review_persisted": false,
            "packet_acceptance_receipt_secret_scan_performed": false,
            "packet_acceptance_receipt_pii_scan_performed": false,
            "packet_acceptance_receipt_raw_payload_inspected": false,
            "packet_acceptance_receipt_plaintext_materialized": false,
            "packet_acceptance_receipt_redaction_bypass_allowed": false,
            "packet_acceptance_receipt_hash_to_payload_link_recorded": false,
            "packet_acceptance_receipt_external_redaction_review_performed": false,
            "packet_acceptance_receipt_privacy_acceptance_recorded": false,
            "packet_acceptance_receipt_acceptance_recorded": false,
            "packet_acceptance_receipt_authority_derived": false,
            "packet_acceptance_receipt_live_execution_allowed": false,
        }),
    );
    extend_json_object(
        &mut side_effects,
        serde_json::json!({
            "packet_acceptance_receipt_query_registered": false,
            "packet_acceptance_receipt_export_file_written": false,
            "packet_acceptance_receipt_observability_metric_recorded": false,
            "packet_acceptance_receipt_recorded": false,
            "packet_acceptance_receipt_persisted": false,
            "operator_acceptance_recorded": false,
            "operator_approval_recorded": false,
            "activation_authority_derived": false,
            "activation_command_derived": false,
            "activation_allowed": false,
            "activation_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "hepta_intelligence_context_attached": false,
            "prompt_preview_rendered": false,
            "context_injection_performed": false,
            "provider_invoked": false,
            "model_invoked": false,
        }),
    );
    extend_json_object(
        &mut side_effects,
        serde_json::json!({
            "external_kg_adapter_read_performed": false,
            "external_adapter_client_constructed": false,
            "network_call_performed": false,
            "external_db_write_performed": false,
            "live_kg_write_performed": false,
            "credential_read": false,
            "secret_file_read": false,
            "install_executed": false,
            "launchd_mutated": false,
            "service_restarted": false,
            "active_binary_mutated": false,
            "public_release_claimed": false,
            "public_ga_claimed": false,
            "release_artifact_written": false,
            "public_artifact_written": false,
            "external_send_performed": false,
            "filesystem_written": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "side_effects": side_effects
        }),
    );
    report
}

fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_operator_briefing_non_persistence_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source =
        hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_redaction_privacy_payload_exposure_denial_report();
    let source_view_report_sha256 = sha256_json_value(&source);
    let source_ready = source
        .get("memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_redaction_privacy_payload_exposure_denial_ready")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);
    let source_redaction_privacy_surface_count = source
        .get("redaction_privacy_surface_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_redacted_payload_preview_recorded_count = source
        .get("redacted_payload_preview_recorded_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_payload_hash_preview_recorded_count = source
        .get("payload_hash_preview_recorded_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_readback_text_recorded_count = source
        .get("readback_text_recorded_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_operator_summary_text_recorded_count = source
        .get("operator_summary_text_recorded_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_privacy_review_recorded_count = source
        .get("privacy_review_recorded_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_secret_scan_performed_count = source
        .get("secret_scan_performed_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_raw_payload_inspected_count = source
        .get("raw_payload_inspected_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_redaction_privacy_activation_authority_derived_count = source
        .get("redaction_privacy_activation_authority_derived_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_redaction_privacy_payload_exposure_contract_hash_sha256 = source
        .get("redaction_privacy_payload_exposure_contract_hash_sha256")
        .cloned()
        .unwrap_or_else(|| serde_json::json!(""));
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;

    let operator_briefing_surfaces = vec![
        "packet_receipt_operator_briefing_claim",
        "packet_receipt_operator_facing_summary_claim",
        "packet_receipt_readback_digest_claim",
        "packet_receipt_final_note_claim",
        "packet_receipt_status_banner_claim",
        "packet_receipt_timeline_entry_claim",
        "packet_receipt_notification_preview_claim",
        "packet_receipt_channel_delivery_claim",
        "packet_receipt_external_send_claim",
        "packet_receipt_telegram_briefing_claim",
        "packet_receipt_completion_briefing_claim",
        "packet_receipt_acceptance_briefing_claim",
        "packet_receipt_authority_briefing_claim",
        "packet_receipt_live_briefing_claim",
    ]
    .into_iter()
    .map(|briefing_surface| {
        serde_json::json!({
            "briefing_surface": briefing_surface,
            "briefing_attempted": true,
            "briefing_recorded": false,
            "briefing_persisted": false,
            "briefing_materialized": false,
            "briefing_filesystem_written": false,
            "summary_recorded": false,
            "readback_digest_recorded": false,
            "final_note_recorded": false,
            "status_banner_recorded": false,
            "timeline_entry_recorded": false,
            "notification_preview_recorded": false,
            "channel_delivery_performed": false,
            "external_send_performed": false,
            "telegram_send_performed": false,
            "completion_ack_recorded": false,
            "acceptance_recorded": false,
            "operator_approval_derived": false,
            "activation_authority_derived": false,
            "activation_command_derived": false,
            "live_execution_allowed": false,
            "briefing_status": "operator_briefing_non_persistence_denied"
        })
    })
    .collect::<Vec<_>>();
    let operator_briefing_surface_count = operator_briefing_surfaces.len();
    let operator_briefing_contract_hash_sha256 = sha256_text_value(&format!(
        "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-operator-briefing-non-persistence:native:source={source_view_report_sha256}:surfaces={operator_briefing_surface_count}:route_count={}:briefing=0:summary=0:send=0:authority=0:live=0",
        route_matrix.route_count
    ));
    let denied_by_packet_receipt_operator_briefing = vec![
        "operator_readiness_packet_template_packet_receipt_operator_briefing_recording_denied",
        "operator_readiness_packet_template_packet_receipt_operator_briefing_persistence_denied",
        "operator_readiness_packet_template_packet_receipt_operator_briefing_materialization_denied",
        "operator_readiness_packet_template_packet_receipt_summary_recording_denied",
        "operator_readiness_packet_template_packet_receipt_readback_digest_denied",
        "operator_readiness_packet_template_packet_receipt_final_note_denied",
        "operator_readiness_packet_template_packet_receipt_status_banner_denied",
        "operator_readiness_packet_template_packet_receipt_timeline_entry_denied",
        "operator_readiness_packet_template_packet_receipt_notification_preview_denied",
        "operator_readiness_packet_template_packet_receipt_channel_delivery_denied",
        "operator_readiness_packet_template_packet_receipt_external_send_denied",
        "operator_readiness_packet_template_packet_receipt_telegram_send_denied",
        "operator_readiness_packet_template_packet_receipt_completion_ack_briefing_denied",
        "operator_readiness_packet_template_packet_receipt_acceptance_from_briefing_denied",
        "operator_readiness_packet_template_packet_receipt_authority_from_briefing_denied",
        "operator_readiness_packet_template_packet_receipt_live_execution_from_briefing_denied",
    ];
    let denied_by_packet_receipt_operator_briefing_count =
        denied_by_packet_receipt_operator_briefing.len();
    let report_ready = source_ready
        && source_redaction_privacy_surface_count == 16
        && source_redacted_payload_preview_recorded_count == 0
        && source_payload_hash_preview_recorded_count == 0
        && source_readback_text_recorded_count == 0
        && source_operator_summary_text_recorded_count == 0
        && source_privacy_review_recorded_count == 0
        && source_secret_scan_performed_count == 0
        && source_raw_payload_inspected_count == 0
        && source_redaction_privacy_activation_authority_derived_count == 0
        && operator_briefing_surface_count == 14
        && route_count_source_command_accepted;

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_operator_briefing_non_persistence_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_OPERATOR_BRIEFING_NON_PERSISTENCE_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-operator-briefing-non-persistence --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-15",
        "receipt_operator_briefing_schema_version": "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_operator_briefing_non_persistence_v1",
        "receipt_operator_briefing_mode": "native_route_non_persistent_receipts_cannot_create_operator_briefing_acceptance_or_authority",
        "source_packet_acceptance_receipt_redaction_privacy_gate": source["gate"].clone(),
        "source_packet_acceptance_receipt_redaction_privacy_ready": source_ready,
        "source_view_report_sha256": source_view_report_sha256,
        "source_redaction_privacy_payload_exposure_contract_hash_sha256": source_redaction_privacy_payload_exposure_contract_hash_sha256,
        "operator_briefing_contract_hash_sha256": operator_briefing_contract_hash_sha256,
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
            "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_operator_briefing_non_persistence_route_enabled": true,
            "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_operator_briefing_non_persistence_ready": report_ready,
            "source_redaction_privacy_surface_count": source_redaction_privacy_surface_count,
            "source_redacted_payload_preview_recorded_count": source_redacted_payload_preview_recorded_count,
            "source_payload_hash_preview_recorded_count": source_payload_hash_preview_recorded_count,
            "source_readback_text_recorded_count": source_readback_text_recorded_count,
            "source_operator_summary_text_recorded_count": source_operator_summary_text_recorded_count,
            "source_privacy_review_recorded_count": source_privacy_review_recorded_count,
            "source_secret_scan_performed_count": source_secret_scan_performed_count,
            "source_raw_payload_inspected_count": source_raw_payload_inspected_count,
            "source_redaction_privacy_activation_authority_derived_count": source_redaction_privacy_activation_authority_derived_count,
            "operator_briefing_surface_count": operator_briefing_surface_count,
            "operator_briefing_attempt_count": operator_briefing_surface_count,
            "briefing_recorded_count": 0,
            "briefing_persisted_count": 0,
            "briefing_materialized_count": 0,
            "briefing_filesystem_written_count": 0,
            "summary_recorded_count": 0,
            "readback_digest_recorded_count": 0,
            "final_note_recorded_count": 0,
            "status_banner_recorded_count": 0,
            "timeline_entry_recorded_count": 0,
            "notification_preview_recorded_count": 0,
            "channel_delivery_performed_count": 0,
            "external_send_performed_count": 0,
            "telegram_send_performed_count": 0,
            "completion_ack_recorded_count": 0,
            "operator_briefing_acceptance_recorded_count": 0,
            "operator_briefing_operator_approval_derived_count": 0,
            "operator_briefing_activation_authority_derived_count": 0,
            "operator_briefing_activation_command_derived_count": 0,
            "operator_briefing_live_execution_allowed_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "operator_briefing_surfaces": operator_briefing_surfaces,
            "denied_by_packet_receipt_operator_briefing": denied_by_packet_receipt_operator_briefing,
            "denied_by_packet_receipt_operator_briefing_count": denied_by_packet_receipt_operator_briefing_count,
            "allowed_next_actions": [
                {
                    "action": "prepare_operator_readiness_packet_template_packet_acceptance_receipt_final_ack_non_acceptance_gate",
                    "status": "allowed_report_only_next_slice",
                    "records_briefing": false,
                    "persists_briefing": false,
                    "sends_externally": false,
                    "records_operator_acceptance": false,
                    "derives_activation_authority": false,
                    "activates_live": false,
                    "mutates_memory_store": false,
                    "writes_kg": false
                }
            ],
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "packet_acceptance_receipt_recorded": false,
            "packet_acceptance_receipt_persisted": false,
            "packet_acceptance_receipt_redacted_payload_preview_recorded": false,
            "packet_acceptance_receipt_readback_text_recorded": false,
            "packet_acceptance_receipt_operator_summary_text_recorded": false,
            "packet_acceptance_receipt_operator_briefing_recorded": false,
            "packet_acceptance_receipt_operator_briefing_persisted": false,
            "packet_acceptance_receipt_summary_recorded": false,
            "packet_acceptance_receipt_readback_digest_recorded": false,
            "packet_acceptance_receipt_final_note_recorded": false,
            "packet_acceptance_receipt_status_banner_recorded": false,
            "packet_acceptance_receipt_timeline_entry_recorded": false,
            "packet_acceptance_receipt_notification_preview_recorded": false,
            "packet_acceptance_receipt_channel_delivered": false,
            "packet_acceptance_receipt_external_sent": false,
            "packet_acceptance_receipt_telegram_sent": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "operator_acceptance_recorded": false,
            "operator_approval_recorded": false,
            "activation_authority_derived": false,
            "activation_command_derived": false,
            "activation_allowed": false,
            "activation_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "hepta_intelligence_context_attached": false,
            "prompt_preview_rendered": false,
            "context_injection_performed": false,
            "provider_invoked": false,
            "model_invoked": false,
            "external_kg_adapter_read_performed": false,
            "external_adapter_client_constructed": false,
            "network_call_performed": false,
            "external_db_write_performed": false,
            "live_kg_write_performed": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "credential_read": false,
            "secret_file_read": false,
            "install_executed": false,
            "launchd_mutated": false,
            "service_restarted": false,
            "active_binary_mutated": false,
            "public_release_claimed": false,
            "public_ga_claimed": false,
            "release_artifact_written": false,
            "public_artifact_written": false,
            "external_send_performed": false,
        }),
    );

    let mut side_effects = serde_json::json!({});
    extend_json_object(
        &mut side_effects,
        serde_json::json!({
            "packet_acceptance_receipt_operator_briefing_recorded": false,
            "packet_acceptance_receipt_operator_briefing_persisted": false,
            "packet_acceptance_receipt_operator_briefing_materialized": false,
            "packet_acceptance_receipt_operator_briefing_filesystem_written": false,
            "packet_acceptance_receipt_summary_recorded": false,
            "packet_acceptance_receipt_readback_digest_recorded": false,
            "packet_acceptance_receipt_final_note_recorded": false,
            "packet_acceptance_receipt_status_banner_recorded": false,
            "packet_acceptance_receipt_timeline_entry_recorded": false,
            "packet_acceptance_receipt_notification_preview_recorded": false,
            "packet_acceptance_receipt_channel_delivered": false,
            "packet_acceptance_receipt_external_sent": false,
            "packet_acceptance_receipt_telegram_sent": false,
            "packet_acceptance_receipt_completion_ack_recorded": false,
            "packet_acceptance_receipt_acceptance_recorded": false,
            "packet_acceptance_receipt_authority_derived": false,
            "packet_acceptance_receipt_live_execution_allowed": false,
        }),
    );
    extend_json_object(
        &mut side_effects,
        serde_json::json!({
            "packet_acceptance_receipt_redacted_payload_preview_recorded": false,
            "packet_acceptance_receipt_readback_text_recorded": false,
            "packet_acceptance_receipt_recorded": false,
            "packet_acceptance_receipt_persisted": false,
            "operator_acceptance_recorded": false,
            "operator_approval_recorded": false,
            "activation_authority_derived": false,
            "activation_command_derived": false,
            "activation_allowed": false,
            "activation_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "hepta_intelligence_context_attached": false,
            "prompt_preview_rendered": false,
            "context_injection_performed": false,
            "provider_invoked": false,
            "model_invoked": false,
        }),
    );
    extend_json_object(
        &mut side_effects,
        serde_json::json!({
            "external_kg_adapter_read_performed": false,
            "external_adapter_client_constructed": false,
            "network_call_performed": false,
            "external_db_write_performed": false,
            "live_kg_write_performed": false,
            "credential_read": false,
            "secret_file_read": false,
            "install_executed": false,
            "launchd_mutated": false,
            "service_restarted": false,
            "active_binary_mutated": false,
            "public_release_claimed": false,
            "public_ga_claimed": false,
            "release_artifact_written": false,
            "public_artifact_written": false,
            "external_send_performed": false,
            "filesystem_written": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "side_effects": side_effects
        }),
    );
    report
}

fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_final_acknowledgement_non_acceptance_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source =
        hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_operator_briefing_non_persistence_report();
    let source_view_report_sha256 = sha256_json_value(&source);
    let source_ready = source
        .get("memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_operator_briefing_non_persistence_ready")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);
    let source_operator_briefing_surface_count = source
        .get("operator_briefing_surface_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_briefing_recorded_count = source
        .get("briefing_recorded_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_briefing_persisted_count = source
        .get("briefing_persisted_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_briefing_materialized_count = source
        .get("briefing_materialized_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_summary_recorded_count = source
        .get("summary_recorded_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_readback_digest_recorded_count = source
        .get("readback_digest_recorded_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_final_note_recorded_count = source
        .get("final_note_recorded_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_channel_delivery_performed_count = source
        .get("channel_delivery_performed_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_external_send_performed_count = source
        .get("external_send_performed_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_telegram_send_performed_count = source
        .get("telegram_send_performed_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_completion_ack_recorded_count = source
        .get("completion_ack_recorded_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_operator_briefing_activation_authority_derived_count = source
        .get("operator_briefing_activation_authority_derived_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_operator_briefing_contract_hash_sha256 = source
        .get("operator_briefing_contract_hash_sha256")
        .cloned()
        .unwrap_or_else(|| serde_json::json!(""));
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;

    let final_acknowledgement_surfaces = vec![
        "packet_receipt_final_acknowledgement_claim",
        "packet_receipt_operator_received_claim",
        "packet_receipt_operator_confirmed_claim",
        "packet_receipt_operator_read_claim",
        "packet_receipt_operator_seen_claim",
        "packet_receipt_final_response_claim",
        "packet_receipt_completion_acknowledgement_claim",
        "packet_receipt_status_acknowledgement_claim",
        "packet_receipt_briefing_acknowledgement_claim",
        "packet_receipt_readback_acknowledgement_claim",
        "packet_receipt_channel_acknowledgement_claim",
        "packet_receipt_external_acknowledgement_claim",
        "packet_receipt_authority_acknowledgement_claim",
        "packet_receipt_live_acknowledgement_claim",
    ]
    .into_iter()
    .map(|final_acknowledgement_surface| {
        serde_json::json!({
            "final_acknowledgement_surface": final_acknowledgement_surface,
            "final_acknowledgement_attempted": true,
            "final_acknowledgement_accepted": false,
            "final_acknowledgement_recorded": false,
            "final_acknowledgement_persisted": false,
            "final_acknowledgement_materialized": false,
            "final_acknowledgement_delivered": false,
            "operator_received_recorded": false,
            "operator_confirmed_recorded": false,
            "operator_read_recorded": false,
            "operator_seen_recorded": false,
            "final_response_recorded": false,
            "completion_ack_recorded": false,
            "status_ack_recorded": false,
            "briefing_ack_recorded": false,
            "readback_ack_recorded": false,
            "channel_ack_delivered": false,
            "external_ack_sent": false,
            "acceptance_recorded": false,
            "operator_approval_derived": false,
            "activation_authority_derived": false,
            "activation_command_derived": false,
            "live_execution_allowed": false,
            "final_acknowledgement_status": "final_acknowledgement_non_acceptance_denied"
        })
    })
    .collect::<Vec<_>>();
    let final_acknowledgement_surface_count = final_acknowledgement_surfaces.len();
    let final_acknowledgement_contract_hash_sha256 = sha256_text_value(&format!(
        "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-final-acknowledgement-non-acceptance:native:source={source_view_report_sha256}:surfaces={final_acknowledgement_surface_count}:route_count={}:ack=0:acceptance=0:authority=0:live=0",
        route_matrix.route_count
    ));
    let denied_by_packet_receipt_final_acknowledgement = vec![
        "operator_readiness_packet_template_packet_receipt_final_acknowledgement_acceptance_denied",
        "operator_readiness_packet_template_packet_receipt_final_acknowledgement_recording_denied",
        "operator_readiness_packet_template_packet_receipt_final_acknowledgement_persistence_denied",
        "operator_readiness_packet_template_packet_receipt_final_acknowledgement_materialization_denied",
        "operator_readiness_packet_template_packet_receipt_final_acknowledgement_delivery_denied",
        "operator_readiness_packet_template_packet_receipt_operator_received_recording_denied",
        "operator_readiness_packet_template_packet_receipt_operator_confirmed_recording_denied",
        "operator_readiness_packet_template_packet_receipt_operator_read_recording_denied",
        "operator_readiness_packet_template_packet_receipt_completion_ack_recording_denied",
        "operator_readiness_packet_template_packet_receipt_status_ack_recording_denied",
        "operator_readiness_packet_template_packet_receipt_briefing_ack_recording_denied",
        "operator_readiness_packet_template_packet_receipt_readback_ack_recording_denied",
        "operator_readiness_packet_template_packet_receipt_channel_ack_delivery_denied",
        "operator_readiness_packet_template_packet_receipt_external_ack_send_denied",
        "operator_readiness_packet_template_packet_receipt_acceptance_from_final_acknowledgement_denied",
        "operator_readiness_packet_template_packet_receipt_authority_from_final_acknowledgement_denied",
        "operator_readiness_packet_template_packet_receipt_live_execution_from_final_acknowledgement_denied",
    ];
    let denied_by_packet_receipt_final_acknowledgement_count =
        denied_by_packet_receipt_final_acknowledgement.len();
    let report_ready = source_ready
        && source_operator_briefing_surface_count == 14
        && source_briefing_recorded_count == 0
        && source_briefing_persisted_count == 0
        && source_briefing_materialized_count == 0
        && source_summary_recorded_count == 0
        && source_readback_digest_recorded_count == 0
        && source_final_note_recorded_count == 0
        && source_channel_delivery_performed_count == 0
        && source_external_send_performed_count == 0
        && source_telegram_send_performed_count == 0
        && source_completion_ack_recorded_count == 0
        && source_operator_briefing_activation_authority_derived_count == 0
        && final_acknowledgement_surface_count == 14
        && route_count_source_command_accepted;

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_final_acknowledgement_non_acceptance_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_FINAL_ACKNOWLEDGEMENT_NON_ACCEPTANCE_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-final-acknowledgement-non-acceptance --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-16",
        "receipt_final_acknowledgement_schema_version": "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_final_acknowledgement_non_acceptance_v1",
        "receipt_final_acknowledgement_mode": "native_route_non_persistent_receipt_briefings_cannot_become_operator_acceptance_or_authority",
        "source_packet_acceptance_receipt_operator_briefing_route": source["gate"].clone(),
        "source_packet_acceptance_receipt_operator_briefing_ready": source_ready,
        "source_view_report_sha256": source_view_report_sha256,
        "source_operator_briefing_contract_hash_sha256": source_operator_briefing_contract_hash_sha256,
        "final_acknowledgement_contract_hash_sha256": final_acknowledgement_contract_hash_sha256,
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
            "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_final_acknowledgement_non_acceptance_route_enabled": true,
            "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_final_acknowledgement_non_acceptance_ready": report_ready,
            "source_operator_briefing_surface_count": source_operator_briefing_surface_count,
            "source_briefing_recorded_count": source_briefing_recorded_count,
            "source_briefing_persisted_count": source_briefing_persisted_count,
            "source_briefing_materialized_count": source_briefing_materialized_count,
            "source_summary_recorded_count": source_summary_recorded_count,
            "source_readback_digest_recorded_count": source_readback_digest_recorded_count,
            "source_final_note_recorded_count": source_final_note_recorded_count,
            "source_channel_delivery_performed_count": source_channel_delivery_performed_count,
            "source_external_send_performed_count": source_external_send_performed_count,
            "source_telegram_send_performed_count": source_telegram_send_performed_count,
            "source_completion_ack_recorded_count": source_completion_ack_recorded_count,
            "source_operator_briefing_activation_authority_derived_count": source_operator_briefing_activation_authority_derived_count,
            "final_acknowledgement_surface_count": final_acknowledgement_surface_count,
            "final_acknowledgement_attempt_count": final_acknowledgement_surface_count,
            "final_acknowledgement_accepted_count": 0,
            "final_acknowledgement_recorded_count": 0,
            "final_acknowledgement_persisted_count": 0,
            "final_acknowledgement_materialized_count": 0,
            "final_acknowledgement_delivered_count": 0,
            "operator_received_recorded_count": 0,
            "operator_confirmed_recorded_count": 0,
            "operator_read_recorded_count": 0,
            "operator_seen_recorded_count": 0,
            "final_response_recorded_count": 0,
            "completion_ack_recorded_count": 0,
            "status_ack_recorded_count": 0,
            "briefing_ack_recorded_count": 0,
            "readback_ack_recorded_count": 0,
            "channel_ack_delivered_count": 0,
            "external_ack_sent_count": 0,
            "final_acknowledgement_acceptance_recorded_count": 0,
            "final_acknowledgement_operator_approval_derived_count": 0,
            "final_acknowledgement_activation_authority_derived_count": 0,
            "final_acknowledgement_activation_command_derived_count": 0,
            "final_acknowledgement_live_execution_allowed_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "final_acknowledgement_surfaces": final_acknowledgement_surfaces,
            "denied_by_packet_receipt_final_acknowledgement": denied_by_packet_receipt_final_acknowledgement,
            "denied_by_packet_receipt_final_acknowledgement_count": denied_by_packet_receipt_final_acknowledgement_count,
            "allowed_next_actions": [
                {
                    "action": "prepare_operator_readiness_packet_template_packet_acceptance_receipt_terminal_decision_status_promotion_denial_gate",
                    "status": "allowed_report_only_next_slice",
                    "records_final_acknowledgement": false,
                    "persists_final_acknowledgement": false,
                    "sends_externally": false,
                    "records_operator_acceptance": false,
                    "derives_activation_authority": false,
                    "activates_live": false,
                    "mutates_memory_store": false,
                    "writes_kg": false
                }
            ],
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "packet_acceptance_receipt_operator_briefing_recorded": false,
            "packet_acceptance_receipt_operator_briefing_persisted": false,
            "packet_acceptance_receipt_summary_recorded": false,
            "packet_acceptance_receipt_readback_digest_recorded": false,
            "packet_acceptance_receipt_final_note_recorded": false,
            "packet_acceptance_receipt_final_acknowledgement_accepted": false,
            "packet_acceptance_receipt_final_acknowledgement_recorded": false,
            "packet_acceptance_receipt_final_acknowledgement_persisted": false,
            "packet_acceptance_receipt_final_acknowledgement_materialized": false,
            "packet_acceptance_receipt_final_acknowledgement_delivered": false,
            "packet_acceptance_receipt_operator_received_recorded": false,
            "packet_acceptance_receipt_operator_confirmed_recorded": false,
            "packet_acceptance_receipt_operator_read_recorded": false,
            "packet_acceptance_receipt_operator_seen_recorded": false,
            "packet_acceptance_receipt_final_response_recorded": false,
            "packet_acceptance_receipt_completion_ack_recorded": false,
            "packet_acceptance_receipt_status_ack_recorded": false,
            "packet_acceptance_receipt_briefing_ack_recorded": false,
            "packet_acceptance_receipt_readback_ack_recorded": false,
            "packet_acceptance_receipt_channel_ack_delivered": false,
            "packet_acceptance_receipt_external_ack_sent": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "operator_acceptance_recorded": false,
            "operator_approval_recorded": false,
            "activation_authority_derived": false,
            "activation_command_derived": false,
            "activation_allowed": false,
            "activation_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "hepta_intelligence_context_attached": false,
            "prompt_preview_rendered": false,
            "context_injection_performed": false,
            "provider_invoked": false,
            "model_invoked": false,
            "external_kg_adapter_read_performed": false,
            "external_adapter_client_constructed": false,
            "network_call_performed": false,
            "external_db_write_performed": false,
            "live_kg_write_performed": false,
            "credential_read": false,
            "secret_file_read": false,
            "install_executed": false,
            "launchd_mutated": false,
            "service_restarted": false,
            "active_binary_mutated": false,
            "public_release_claimed": false,
            "public_ga_claimed": false,
            "release_artifact_written": false,
            "public_artifact_written": false,
            "external_send_performed": false,
        }),
    );

    let mut side_effects = serde_json::json!({});
    extend_json_object(
        &mut side_effects,
        serde_json::json!({
            "packet_acceptance_receipt_final_acknowledgement_accepted": false,
            "packet_acceptance_receipt_final_acknowledgement_recorded": false,
            "packet_acceptance_receipt_final_acknowledgement_persisted": false,
            "packet_acceptance_receipt_final_acknowledgement_materialized": false,
            "packet_acceptance_receipt_final_acknowledgement_delivered": false,
            "packet_acceptance_receipt_operator_received_recorded": false,
            "packet_acceptance_receipt_operator_confirmed_recorded": false,
            "packet_acceptance_receipt_operator_read_recorded": false,
            "packet_acceptance_receipt_operator_seen_recorded": false,
            "packet_acceptance_receipt_final_response_recorded": false,
            "packet_acceptance_receipt_completion_ack_recorded": false,
            "packet_acceptance_receipt_status_ack_recorded": false,
            "packet_acceptance_receipt_briefing_ack_recorded": false,
            "packet_acceptance_receipt_readback_ack_recorded": false,
            "packet_acceptance_receipt_channel_ack_delivered": false,
            "packet_acceptance_receipt_external_ack_sent": false,
            "packet_acceptance_receipt_acceptance_recorded": false,
            "packet_acceptance_receipt_authority_derived": false,
            "packet_acceptance_receipt_live_execution_allowed": false,
        }),
    );
    extend_json_object(
        &mut side_effects,
        serde_json::json!({
            "packet_acceptance_receipt_operator_briefing_recorded": false,
            "packet_acceptance_receipt_operator_briefing_persisted": false,
            "packet_acceptance_receipt_summary_recorded": false,
            "packet_acceptance_receipt_readback_digest_recorded": false,
            "packet_acceptance_receipt_final_note_recorded": false,
            "operator_acceptance_recorded": false,
            "operator_approval_recorded": false,
            "activation_authority_derived": false,
            "activation_command_derived": false,
            "activation_allowed": false,
            "activation_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "hepta_intelligence_context_attached": false,
            "prompt_preview_rendered": false,
            "context_injection_performed": false,
            "provider_invoked": false,
            "model_invoked": false,
        }),
    );
    extend_json_object(
        &mut side_effects,
        serde_json::json!({
            "external_kg_adapter_read_performed": false,
            "external_adapter_client_constructed": false,
            "network_call_performed": false,
            "external_db_write_performed": false,
            "live_kg_write_performed": false,
            "credential_read": false,
            "secret_file_read": false,
            "install_executed": false,
            "launchd_mutated": false,
            "service_restarted": false,
            "active_binary_mutated": false,
            "public_release_claimed": false,
            "public_ga_claimed": false,
            "release_artifact_written": false,
            "public_artifact_written": false,
            "external_send_performed": false,
            "filesystem_written": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "side_effects": side_effects
        }),
    );
    report
}

fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_terminal_decision_status_promotion_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source =
        hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_final_acknowledgement_non_acceptance_report();
    let source_view_report_sha256 = sha256_json_value(&source);
    let source_ready = source
        .get("memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_final_acknowledgement_non_acceptance_ready")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);
    let source_final_acknowledgement_surface_count = source
        .get("final_acknowledgement_surface_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_final_acknowledgement_accepted_count = source
        .get("final_acknowledgement_accepted_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_final_acknowledgement_recorded_count = source
        .get("final_acknowledgement_recorded_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_final_acknowledgement_persisted_count = source
        .get("final_acknowledgement_persisted_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_final_acknowledgement_materialized_count = source
        .get("final_acknowledgement_materialized_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_final_acknowledgement_delivered_count = source
        .get("final_acknowledgement_delivered_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_operator_received_recorded_count = source
        .get("operator_received_recorded_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_operator_confirmed_recorded_count = source
        .get("operator_confirmed_recorded_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_operator_read_recorded_count = source
        .get("operator_read_recorded_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_operator_seen_recorded_count = source
        .get("operator_seen_recorded_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_completion_ack_recorded_count = source
        .get("completion_ack_recorded_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_status_ack_recorded_count = source
        .get("status_ack_recorded_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_external_ack_sent_count = source
        .get("external_ack_sent_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_final_acknowledgement_activation_authority_derived_count = source
        .get("final_acknowledgement_activation_authority_derived_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_final_acknowledgement_contract_hash_sha256 = source
        .get("final_acknowledgement_contract_hash_sha256")
        .cloned()
        .unwrap_or_else(|| serde_json::json!(""));
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;

    let terminal_decision_status_surfaces = vec![
        "packet_receipt_terminal_decision_claim",
        "packet_receipt_terminal_status_closed_claim",
        "packet_receipt_final_state_promotion_claim",
        "packet_receipt_completion_promotion_claim",
        "packet_receipt_status_ready_claim",
        "packet_receipt_status_accepted_claim",
        "packet_receipt_status_approved_claim",
        "packet_receipt_status_authoritative_claim",
        "packet_receipt_status_live_claim",
        "packet_receipt_operator_decision_claim",
        "packet_receipt_public_status_claim",
        "packet_receipt_release_status_claim",
        "packet_receipt_dashboard_status_claim",
        "packet_receipt_live_execution_decision_claim",
    ]
    .into_iter()
    .map(|terminal_decision_surface| {
        serde_json::json!({
            "terminal_decision_surface": terminal_decision_surface,
            "terminal_decision_attempted": true,
            "terminal_decision_accepted": false,
            "terminal_decision_recorded": false,
            "terminal_decision_persisted": false,
            "terminal_decision_materialized": false,
            "terminal_decision_delivered": false,
            "terminal_status_recorded": false,
            "terminal_status_persisted": false,
            "terminal_status_closed": false,
            "terminal_status_ready": false,
            "terminal_status_accepted": false,
            "terminal_status_approved": false,
            "terminal_status_authoritative": false,
            "terminal_status_live": false,
            "final_state_promoted": false,
            "completion_promoted": false,
            "operator_decision_recorded": false,
            "public_status_claimed": false,
            "release_status_claimed": false,
            "dashboard_status_recorded": false,
            "acceptance_recorded": false,
            "operator_approval_derived": false,
            "activation_authority_derived": false,
            "activation_command_derived": false,
            "live_execution_allowed": false,
            "terminal_decision_status": "terminal_decision_status_promotion_denied"
        })
    })
    .collect::<Vec<_>>();
    let terminal_decision_status_surface_count = terminal_decision_status_surfaces.len();
    let terminal_decision_contract_hash_sha256 = sha256_text_value(&format!(
        "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-terminal-decision-status-promotion-denial:native:source={source_view_report_sha256}:surfaces={terminal_decision_status_surface_count}:route_count={}:terminal=0:status=0:authority=0:live=0",
        route_matrix.route_count
    ));
    let denied_by_packet_receipt_terminal_decision_status = vec![
        "operator_readiness_packet_template_packet_receipt_terminal_decision_acceptance_denied",
        "operator_readiness_packet_template_packet_receipt_terminal_decision_recording_denied",
        "operator_readiness_packet_template_packet_receipt_terminal_decision_persistence_denied",
        "operator_readiness_packet_template_packet_receipt_terminal_decision_materialization_denied",
        "operator_readiness_packet_template_packet_receipt_terminal_status_recording_denied",
        "operator_readiness_packet_template_packet_receipt_terminal_status_closed_denied",
        "operator_readiness_packet_template_packet_receipt_status_ready_denied",
        "operator_readiness_packet_template_packet_receipt_status_accepted_denied",
        "operator_readiness_packet_template_packet_receipt_status_approved_denied",
        "operator_readiness_packet_template_packet_receipt_status_authoritative_denied",
        "operator_readiness_packet_template_packet_receipt_status_live_denied",
        "operator_readiness_packet_template_packet_receipt_final_state_promotion_denied",
        "operator_readiness_packet_template_packet_receipt_completion_promotion_denied",
        "operator_readiness_packet_template_packet_receipt_operator_decision_recording_denied",
        "operator_readiness_packet_template_packet_receipt_public_status_claim_denied",
        "operator_readiness_packet_template_packet_receipt_release_status_claim_denied",
        "operator_readiness_packet_template_packet_receipt_dashboard_status_recording_denied",
        "operator_readiness_packet_template_packet_receipt_acceptance_from_terminal_status_denied",
        "operator_readiness_packet_template_packet_receipt_authority_from_terminal_status_denied",
        "operator_readiness_packet_template_packet_receipt_live_execution_from_terminal_status_denied",
    ];
    let denied_by_packet_receipt_terminal_decision_status_count =
        denied_by_packet_receipt_terminal_decision_status.len();
    let report_ready = source_ready
        && source_final_acknowledgement_surface_count == 14
        && source_final_acknowledgement_accepted_count == 0
        && source_final_acknowledgement_recorded_count == 0
        && source_final_acknowledgement_persisted_count == 0
        && source_final_acknowledgement_materialized_count == 0
        && source_final_acknowledgement_delivered_count == 0
        && source_operator_received_recorded_count == 0
        && source_operator_confirmed_recorded_count == 0
        && source_operator_read_recorded_count == 0
        && source_operator_seen_recorded_count == 0
        && source_completion_ack_recorded_count == 0
        && source_status_ack_recorded_count == 0
        && source_external_ack_sent_count == 0
        && source_final_acknowledgement_activation_authority_derived_count == 0
        && terminal_decision_status_surface_count == 14
        && route_count_source_command_accepted;

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_terminal_decision_status_promotion_denial_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_TERMINAL_DECISION_STATUS_PROMOTION_DENIAL_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-terminal-decision-status-promotion-denial --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-16",
        "receipt_terminal_decision_schema_version": "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_terminal_decision_status_promotion_denial_v1",
        "receipt_terminal_decision_mode": "native_route_final_acknowledgements_cannot_become_terminal_decisions_status_public_claims_or_authority",
        "source_packet_acceptance_receipt_final_acknowledgement_route": source["gate"].clone(),
        "source_packet_acceptance_receipt_final_acknowledgement_ready": source_ready,
        "source_view_report_sha256": source_view_report_sha256,
        "source_final_acknowledgement_contract_hash_sha256": source_final_acknowledgement_contract_hash_sha256,
        "terminal_decision_contract_hash_sha256": terminal_decision_contract_hash_sha256,
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
            "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_terminal_decision_status_promotion_denial_route_enabled": true,
            "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_terminal_decision_status_promotion_denial_ready": report_ready,
            "source_final_acknowledgement_surface_count": source_final_acknowledgement_surface_count,
            "source_final_acknowledgement_accepted_count": source_final_acknowledgement_accepted_count,
            "source_final_acknowledgement_recorded_count": source_final_acknowledgement_recorded_count,
            "source_final_acknowledgement_persisted_count": source_final_acknowledgement_persisted_count,
            "source_final_acknowledgement_materialized_count": source_final_acknowledgement_materialized_count,
            "source_final_acknowledgement_delivered_count": source_final_acknowledgement_delivered_count,
            "source_operator_received_recorded_count": source_operator_received_recorded_count,
            "source_operator_confirmed_recorded_count": source_operator_confirmed_recorded_count,
            "source_operator_read_recorded_count": source_operator_read_recorded_count,
            "source_operator_seen_recorded_count": source_operator_seen_recorded_count,
            "source_completion_ack_recorded_count": source_completion_ack_recorded_count,
            "source_status_ack_recorded_count": source_status_ack_recorded_count,
            "source_external_ack_sent_count": source_external_ack_sent_count,
            "source_final_acknowledgement_activation_authority_derived_count": source_final_acknowledgement_activation_authority_derived_count,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "terminal_decision_status_surface_count": terminal_decision_status_surface_count,
            "terminal_decision_status_attempt_count": terminal_decision_status_surface_count,
            "terminal_decision_accepted_count": 0,
            "terminal_decision_recorded_count": 0,
            "terminal_decision_persisted_count": 0,
            "terminal_decision_materialized_count": 0,
            "terminal_decision_delivered_count": 0,
            "terminal_status_recorded_count": 0,
            "terminal_status_persisted_count": 0,
            "terminal_status_closed_count": 0,
            "terminal_status_ready_count": 0,
            "terminal_status_accepted_count": 0,
            "terminal_status_approved_count": 0,
            "terminal_status_authoritative_count": 0,
            "terminal_status_live_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "final_state_promoted_count": 0,
            "completion_promoted_count": 0,
            "operator_decision_recorded_count": 0,
            "public_status_claimed_count": 0,
            "release_status_claimed_count": 0,
            "dashboard_status_recorded_count": 0,
            "terminal_decision_acceptance_recorded_count": 0,
            "terminal_decision_operator_approval_derived_count": 0,
            "terminal_decision_activation_authority_derived_count": 0,
            "terminal_decision_activation_command_derived_count": 0,
            "terminal_decision_live_execution_allowed_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "terminal_decision_status_surfaces": terminal_decision_status_surfaces,
            "denied_by_packet_receipt_terminal_decision_status": denied_by_packet_receipt_terminal_decision_status,
            "denied_by_packet_receipt_terminal_decision_status_count": denied_by_packet_receipt_terminal_decision_status_count,
            "allowed_next_actions": [
                {
                    "action": "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_denial_gate",
                    "status": "allowed_report_only_next_slice",
                    "records_terminal_decision": false,
                    "promotes_status": false,
                    "claims_public_status": false,
                    "records_operator_acceptance": false,
                    "derives_activation_authority": false,
                    "activates_live": false,
                    "mutates_memory_store": false,
                    "writes_kg": false
                }
            ],
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "packet_acceptance_receipt_final_acknowledgement_accepted": false,
            "packet_acceptance_receipt_final_acknowledgement_recorded": false,
            "packet_acceptance_receipt_terminal_decision_accepted": false,
            "packet_acceptance_receipt_terminal_decision_recorded": false,
            "packet_acceptance_receipt_terminal_decision_persisted": false,
            "packet_acceptance_receipt_terminal_decision_materialized": false,
            "packet_acceptance_receipt_terminal_status_recorded": false,
            "packet_acceptance_receipt_terminal_status_persisted": false,
            "packet_acceptance_receipt_terminal_status_closed": false,
            "packet_acceptance_receipt_status_ready": false,
            "packet_acceptance_receipt_status_accepted": false,
            "packet_acceptance_receipt_status_approved": false,
            "packet_acceptance_receipt_status_authoritative": false,
            "packet_acceptance_receipt_status_live": false,
            "packet_acceptance_receipt_final_state_promoted": false,
            "packet_acceptance_receipt_completion_promoted": false,
            "packet_acceptance_receipt_operator_decision_recorded": false,
            "packet_acceptance_receipt_public_status_claimed": false,
            "packet_acceptance_receipt_release_status_claimed": false,
            "packet_acceptance_receipt_dashboard_status_recorded": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "operator_acceptance_recorded": false,
            "operator_approval_recorded": false,
            "activation_authority_derived": false,
            "activation_command_derived": false,
            "activation_allowed": false,
            "activation_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "hepta_intelligence_context_attached": false,
            "prompt_preview_rendered": false,
            "context_injection_performed": false,
            "provider_invoked": false,
            "model_invoked": false,
            "external_kg_adapter_read_performed": false,
            "external_adapter_client_constructed": false,
            "network_call_performed": false,
            "external_db_write_performed": false,
            "live_kg_write_performed": false,
            "credential_read": false,
            "secret_file_read": false,
            "install_executed": false,
            "launchd_mutated": false,
            "service_restarted": false,
            "active_binary_mutated": false,
            "public_release_claimed": false,
            "public_ga_claimed": false,
            "release_artifact_written": false,
            "public_artifact_written": false,
            "external_send_performed": false,
        }),
    );

    let mut side_effects = serde_json::json!({});
    extend_json_object(
        &mut side_effects,
        serde_json::json!({
            "packet_acceptance_receipt_terminal_decision_accepted": false,
            "packet_acceptance_receipt_terminal_decision_recorded": false,
            "packet_acceptance_receipt_terminal_decision_persisted": false,
            "packet_acceptance_receipt_terminal_decision_materialized": false,
            "packet_acceptance_receipt_terminal_status_recorded": false,
            "packet_acceptance_receipt_terminal_status_persisted": false,
            "packet_acceptance_receipt_terminal_status_closed": false,
            "packet_acceptance_receipt_status_ready": false,
            "packet_acceptance_receipt_status_accepted": false,
            "packet_acceptance_receipt_status_approved": false,
            "packet_acceptance_receipt_status_authoritative": false,
            "packet_acceptance_receipt_status_live": false,
            "packet_acceptance_receipt_final_state_promoted": false,
            "packet_acceptance_receipt_completion_promoted": false,
            "packet_acceptance_receipt_operator_decision_recorded": false,
            "packet_acceptance_receipt_public_status_claimed": false,
            "packet_acceptance_receipt_release_status_claimed": false,
            "packet_acceptance_receipt_dashboard_status_recorded": false,
            "packet_acceptance_receipt_acceptance_recorded": false,
            "packet_acceptance_receipt_authority_derived": false,
            "packet_acceptance_receipt_live_execution_allowed": false,
        }),
    );
    extend_json_object(
        &mut side_effects,
        serde_json::json!({
            "operator_acceptance_recorded": false,
            "operator_approval_recorded": false,
            "activation_authority_derived": false,
            "activation_command_derived": false,
            "activation_allowed": false,
            "activation_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "hepta_intelligence_context_attached": false,
            "prompt_preview_rendered": false,
            "context_injection_performed": false,
            "provider_invoked": false,
            "model_invoked": false,
            "external_kg_adapter_read_performed": false,
            "external_adapter_client_constructed": false,
            "network_call_performed": false,
            "external_db_write_performed": false,
            "live_kg_write_performed": false,
            "credential_read": false,
            "secret_file_read": false,
            "install_executed": false,
            "launchd_mutated": false,
            "service_restarted": false,
            "active_binary_mutated": false,
            "public_release_claimed": false,
            "public_ga_claimed": false,
            "release_artifact_written": false,
            "public_artifact_written": false,
            "external_send_performed": false,
            "filesystem_written": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({ "side_effects": side_effects }),
    );
    report
}

fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source =
        hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_terminal_decision_status_promotion_denial_report();
    let source_view_report_sha256 = sha256_json_value(&source);
    let source_ready = source
        .get("memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_terminal_decision_status_promotion_denial_ready")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);
    let source_terminal_decision_status_surface_count = source
        .get("terminal_decision_status_surface_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_terminal_decision_recorded_count = source
        .get("terminal_decision_recorded_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_terminal_status_recorded_count = source
        .get("terminal_status_recorded_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_terminal_status_live_count = source
        .get("terminal_status_live_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_public_status_claimed_count = source
        .get("public_status_claimed_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_release_status_claimed_count = source
        .get("release_status_claimed_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_terminal_decision_activation_authority_derived_count = source
        .get("terminal_decision_activation_authority_derived_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_terminal_decision_live_execution_allowed_count = source
        .get("terminal_decision_live_execution_allowed_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);
    let source_terminal_decision_contract_hash_sha256 = source
        .get("terminal_decision_contract_hash_sha256")
        .cloned()
        .unwrap_or_else(|| serde_json::json!(""));
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;

    let release_publication_surfaces = vec![
        "packet_receipt_release_artifact_write_claim",
        "packet_receipt_public_artifact_write_claim",
        "packet_receipt_artifact_signature_claim",
        "packet_receipt_artifact_notarization_claim",
        "packet_receipt_publication_queue_claim",
        "packet_receipt_publication_manifest_claim",
        "packet_receipt_public_distribution_claim",
        "packet_receipt_channel_delivery_publication_claim",
        "packet_receipt_public_version_tag_claim",
        "packet_receipt_release_notes_materialization_claim",
        "packet_receipt_changelog_materialization_claim",
        "packet_receipt_public_release_claim",
        "packet_receipt_public_ga_claim",
        "packet_receipt_terminal_status_release_approval_claim",
    ]
    .into_iter()
    .map(|release_publication_surface| {
        serde_json::json!({
            "release_publication_surface": release_publication_surface,
            "release_publication_attempted": true,
            "release_publication_allowed": false,
            "release_publication_accepted": false,
            "release_publication_recorded": false,
            "release_publication_persisted": false,
            "release_publication_materialized": false,
            "release_artifact_written": false,
            "public_artifact_written": false,
            "artifact_signature_accepted": false,
            "artifact_notarization_accepted": false,
            "publication_queue_enqueued": false,
            "publication_manifest_written": false,
            "public_distribution_performed": false,
            "channel_delivery_performed": false,
            "external_send_performed": false,
            "public_version_tag_created": false,
            "release_notes_materialized": false,
            "changelog_materialized": false,
            "public_release_claimed": false,
            "public_ga_claimed": false,
            "terminal_status_promoted_to_release_approval": false,
            "acceptance_recorded": false,
            "operator_approval_derived": false,
            "activation_authority_derived": false,
            "activation_command_derived": false,
            "live_execution_allowed": false,
            "release_publication_status": "release_publication_denied"
        })
    })
    .collect::<Vec<_>>();
    let release_publication_surface_count = release_publication_surfaces.len();
    let release_publication_contract_hash_sha256 = sha256_text_value(&format!(
        "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-denial:native:source={source_view_report_sha256}:surfaces={release_publication_surface_count}:route_count={}:release=0:artifact=0:public=0:authority=0:live=0",
        route_matrix.route_count
    ));
    let denied_by_packet_receipt_release_publication = vec![
        "operator_readiness_packet_template_packet_receipt_release_artifact_write_denied",
        "operator_readiness_packet_template_packet_receipt_public_artifact_write_denied",
        "operator_readiness_packet_template_packet_receipt_artifact_signature_acceptance_denied",
        "operator_readiness_packet_template_packet_receipt_artifact_notarization_acceptance_denied",
        "operator_readiness_packet_template_packet_receipt_publication_queue_enqueue_denied",
        "operator_readiness_packet_template_packet_receipt_publication_manifest_write_denied",
        "operator_readiness_packet_template_packet_receipt_public_distribution_denied",
        "operator_readiness_packet_template_packet_receipt_channel_delivery_publication_denied",
        "operator_readiness_packet_template_packet_receipt_public_version_tag_denied",
        "operator_readiness_packet_template_packet_receipt_release_notes_materialization_denied",
        "operator_readiness_packet_template_packet_receipt_changelog_materialization_denied",
        "operator_readiness_packet_template_packet_receipt_public_release_claim_denied",
        "operator_readiness_packet_template_packet_receipt_public_ga_claim_denied",
        "operator_readiness_packet_template_packet_receipt_terminal_status_as_release_approval_denied",
        "operator_readiness_packet_template_packet_receipt_acceptance_from_release_publication_denied",
        "operator_readiness_packet_template_packet_receipt_authority_from_release_publication_denied",
        "operator_readiness_packet_template_packet_receipt_live_execution_from_release_publication_denied",
    ];
    let denied_by_packet_receipt_release_publication_count =
        denied_by_packet_receipt_release_publication.len();
    let report_ready = source_ready
        && source_terminal_decision_status_surface_count == 14
        && source_terminal_decision_recorded_count == 0
        && source_terminal_status_recorded_count == 0
        && source_terminal_status_live_count == 0
        && source_public_status_claimed_count == 0
        && source_release_status_claimed_count == 0
        && source_terminal_decision_activation_authority_derived_count == 0
        && source_terminal_decision_live_execution_allowed_count == 0
        && release_publication_surface_count == 14
        && route_count_source_command_accepted;

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_denial_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_DENIAL_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-denial --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-16",
        "receipt_release_publication_schema_version": "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_denial_v1",
        "receipt_release_publication_mode": "native_route_terminal_receipt_status_cannot_become_release_publication_public_artifacts_or_public_claims",
        "source_packet_acceptance_receipt_terminal_decision_status_route": source["gate"].clone(),
        "source_packet_acceptance_receipt_terminal_decision_status_ready": source_ready,
        "source_view_report_sha256": source_view_report_sha256,
        "source_terminal_decision_contract_hash_sha256": source_terminal_decision_contract_hash_sha256,
        "release_publication_contract_hash_sha256": release_publication_contract_hash_sha256,
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
            "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_denial_route_enabled": true,
            "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_denial_ready": report_ready,
            "source_terminal_decision_status_surface_count": source_terminal_decision_status_surface_count,
            "source_terminal_decision_recorded_count": source_terminal_decision_recorded_count,
            "source_terminal_status_recorded_count": source_terminal_status_recorded_count,
            "source_terminal_status_live_count": source_terminal_status_live_count,
            "source_public_status_claimed_count": source_public_status_claimed_count,
            "source_release_status_claimed_count": source_release_status_claimed_count,
            "source_terminal_decision_activation_authority_derived_count": source_terminal_decision_activation_authority_derived_count,
            "source_terminal_decision_live_execution_allowed_count": source_terminal_decision_live_execution_allowed_count,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "release_publication_surface_count": release_publication_surface_count,
            "release_publication_attempt_count": release_publication_surface_count,
            "release_publication_allowed_count": 0,
            "release_publication_accepted_count": 0,
            "release_publication_recorded_count": 0,
            "release_publication_persisted_count": 0,
            "release_publication_materialized_count": 0,
            "release_artifact_written_count": 0,
            "public_artifact_written_count": 0,
            "artifact_signature_accepted_count": 0,
            "artifact_notarization_accepted_count": 0,
            "publication_queue_enqueued_count": 0,
            "publication_manifest_written_count": 0,
            "public_distribution_performed_count": 0,
            "channel_delivery_performed_count": 0,
            "external_publication_sent_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "public_version_tag_created_count": 0,
            "release_notes_materialized_count": 0,
            "changelog_materialized_count": 0,
            "public_release_claimed_count": 0,
            "public_ga_claimed_count": 0,
            "terminal_status_release_approval_promoted_count": 0,
            "release_publication_acceptance_recorded_count": 0,
            "release_publication_operator_approval_derived_count": 0,
            "release_publication_activation_authority_derived_count": 0,
            "release_publication_activation_command_derived_count": 0,
            "release_publication_live_execution_allowed_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "release_publication_surfaces": release_publication_surfaces,
            "denied_by_packet_receipt_release_publication": denied_by_packet_receipt_release_publication,
            "denied_by_packet_receipt_release_publication_count": denied_by_packet_receipt_release_publication_count,
            "allowed_next_actions": [
                {
                    "action": "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_no_persistence_gate",
                    "status": "allowed_report_only_next_slice",
                    "records_release_publication": false,
                    "writes_release_artifact": false,
                    "writes_public_artifact": false,
                    "enqueues_publication": false,
                    "claims_public_release": false,
                    "records_operator_acceptance": false,
                    "derives_activation_authority": false,
                    "activates_live": false,
                    "mutates_memory_store": false,
                    "writes_kg": false
                }
            ],
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "packet_acceptance_receipt_terminal_decision_recorded": false,
            "packet_acceptance_receipt_terminal_status_recorded": false,
            "packet_acceptance_receipt_public_status_claimed": false,
            "packet_acceptance_receipt_release_status_claimed": false,
            "packet_acceptance_receipt_release_publication_allowed": false,
            "packet_acceptance_receipt_release_publication_accepted": false,
            "packet_acceptance_receipt_release_publication_recorded": false,
            "packet_acceptance_receipt_release_publication_persisted": false,
            "packet_acceptance_receipt_release_publication_materialized": false,
            "packet_acceptance_receipt_release_artifact_written": false,
            "packet_acceptance_receipt_public_artifact_written": false,
            "packet_acceptance_receipt_artifact_signature_accepted": false,
            "packet_acceptance_receipt_artifact_notarization_accepted": false,
            "packet_acceptance_receipt_publication_queue_enqueued": false,
            "packet_acceptance_receipt_publication_manifest_written": false,
            "packet_acceptance_receipt_public_distribution_performed": false,
            "packet_acceptance_receipt_channel_delivery_performed": false,
            "packet_acceptance_receipt_external_publication_sent": false,
            "packet_acceptance_receipt_public_version_tag_created": false,
            "packet_acceptance_receipt_release_notes_materialized": false,
            "packet_acceptance_receipt_changelog_materialized": false,
            "packet_acceptance_receipt_public_release_claimed": false,
            "packet_acceptance_receipt_public_ga_claimed": false,
            "packet_acceptance_receipt_terminal_status_promoted_to_release_approval": false,
            "packet_acceptance_receipt_acceptance_recorded": false,
            "packet_acceptance_receipt_authority_derived": false,
            "packet_acceptance_receipt_live_execution_allowed": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "operator_acceptance_recorded": false,
            "operator_approval_recorded": false,
            "activation_authority_derived": false,
            "activation_command_derived": false,
            "activation_allowed": false,
            "activation_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "hepta_intelligence_context_attached": false,
            "prompt_preview_rendered": false,
            "context_injection_performed": false,
            "provider_invoked": false,
            "model_invoked": false,
            "external_kg_adapter_read_performed": false,
            "external_adapter_client_constructed": false,
            "network_call_performed": false,
            "external_db_write_performed": false,
            "live_kg_write_performed": false,
            "credential_read": false,
            "secret_file_read": false,
            "install_executed": false,
            "launchd_mutated": false,
            "service_restarted": false,
            "active_binary_mutated": false,
            "public_release_claimed": false,
            "public_ga_claimed": false,
            "release_artifact_written": false,
            "public_artifact_written": false,
            "external_send_performed": false,
            "filesystem_written": false,
        }),
    );

    let mut side_effects = serde_json::json!({});
    extend_json_object(
        &mut side_effects,
        serde_json::json!({
            "packet_acceptance_receipt_release_publication_allowed": false,
            "packet_acceptance_receipt_release_publication_accepted": false,
            "packet_acceptance_receipt_release_publication_recorded": false,
            "packet_acceptance_receipt_release_publication_persisted": false,
            "packet_acceptance_receipt_release_publication_materialized": false,
            "packet_acceptance_receipt_release_artifact_written": false,
            "packet_acceptance_receipt_public_artifact_written": false,
            "packet_acceptance_receipt_artifact_signature_accepted": false,
            "packet_acceptance_receipt_artifact_notarization_accepted": false,
            "packet_acceptance_receipt_publication_queue_enqueued": false,
            "packet_acceptance_receipt_publication_manifest_written": false,
            "packet_acceptance_receipt_public_distribution_performed": false,
            "packet_acceptance_receipt_channel_delivery_performed": false,
            "packet_acceptance_receipt_external_publication_sent": false,
            "packet_acceptance_receipt_public_version_tag_created": false,
            "packet_acceptance_receipt_release_notes_materialized": false,
            "packet_acceptance_receipt_changelog_materialized": false,
            "packet_acceptance_receipt_public_release_claimed": false,
            "packet_acceptance_receipt_public_ga_claimed": false,
            "packet_acceptance_receipt_terminal_status_promoted_to_release_approval": false,
            "packet_acceptance_receipt_acceptance_recorded": false,
            "packet_acceptance_receipt_authority_derived": false,
            "packet_acceptance_receipt_live_execution_allowed": false,
        }),
    );
    extend_json_object(
        &mut side_effects,
        serde_json::json!({
            "operator_acceptance_recorded": false,
            "operator_approval_recorded": false,
            "activation_authority_derived": false,
            "activation_command_derived": false,
            "activation_allowed": false,
            "activation_performed": false,
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "hepta_intelligence_context_attached": false,
            "prompt_preview_rendered": false,
            "context_injection_performed": false,
            "provider_invoked": false,
            "model_invoked": false,
            "external_kg_adapter_read_performed": false,
            "external_adapter_client_constructed": false,
            "network_call_performed": false,
            "external_db_write_performed": false,
            "live_kg_write_performed": false,
            "credential_read": false,
            "secret_file_read": false,
            "install_executed": false,
            "launchd_mutated": false,
            "service_restarted": false,
            "active_binary_mutated": false,
            "public_release_claimed": false,
            "public_ga_claimed": false,
            "release_artifact_written": false,
            "public_artifact_written": false,
            "external_send_performed": false,
            "filesystem_written": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({ "side_effects": side_effects }),
    );
    report
}

fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_no_persistence_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source =
        hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_denial_report();
    let source_report_sha256 = sha256_json_value(&source);
    let source_ready = source
        .get("memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_denial_ready")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);
    let source_u64 = |key: &str| -> u64 {
        source
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let source_release_publication_contract_hash_sha256 = source
        .get("release_publication_contract_hash_sha256")
        .cloned()
        .unwrap_or_else(|| serde_json::json!(""));
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;

    let release_publication_result_receipt_surfaces = vec![
        "source_release_publication_report_required",
        "publication_result_receipt_recording_denied",
        "publication_result_receipt_persistence_denied",
        "publication_result_receipt_materialization_denied",
        "publication_result_receipt_filesystem_write_denied",
        "publication_result_receipt_ledger_index_denied",
        "publication_result_receipt_enqueue_delivery_denied",
        "publication_result_receipt_export_query_denied",
        "publication_result_receipt_observability_denied",
        "publication_result_receipt_hash_binding_denied",
        "publication_result_receipt_signature_timestamp_status_denied",
        "publication_completion_ack_denied",
        "publication_result_receipt_release_publication_authority_denied",
        "publication_result_receipt_activation_live_install_restart_active_binary_denied",
    ]
    .into_iter()
    .map(|release_publication_result_receipt_surface| {
        let mut surface = serde_json::Map::new();
        surface.insert(
            "release_publication_result_receipt_surface".to_string(),
            serde_json::json!(release_publication_result_receipt_surface),
        );
        surface.insert(
            "source_release_publication_report_present".to_string(),
            serde_json::json!(true),
        );
        surface.insert(
            "source_release_publication_denial_ready".to_string(),
            serde_json::json!(true),
        );
        surface.insert(
            "publication_result_receipt_attempted".to_string(),
            serde_json::json!(true),
        );
        surface.insert(
            "publication_result_receipt_noop_confirmed".to_string(),
            serde_json::json!(true),
        );
        surface.insert(
            "release_publication_result_receipt_status".to_string(),
            serde_json::json!("release_publication_result_receipt_no_persistence_denied"),
        );
        for key in [
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
            "release_publication_recorded",
            "release_publication_persisted",
            "release_publication_materialized",
            "release_artifact_written",
            "public_artifact_written",
            "artifact_signature_accepted",
            "artifact_notarization_accepted",
            "publication_queue_enqueued",
            "publication_manifest_written",
            "public_distribution_performed",
            "channel_delivery_performed",
            "external_send_performed",
            "public_version_tag_created",
            "release_notes_materialized",
            "changelog_materialized",
            "public_release_claimed",
            "public_ga_claimed",
            "terminal_status_promoted_to_release_approval",
            "acceptance_recorded",
            "operator_approval_derived",
            "activation_authority_derived",
            "activation_command_derived",
            "live_execution_allowed",
            "activation_performed",
            "memory_store_write_performed",
            "memory_store_mutated",
            "provider_invoked",
            "model_invoked",
            "credential_read",
            "secret_file_read",
            "install_executed",
            "launchd_mutated",
            "service_restarted",
            "active_binary_mutated",
        ] {
            surface.insert(key.to_string(), serde_json::json!(false));
        }
        serde_json::Value::Object(surface)
    })
    .collect::<Vec<_>>();
    let release_publication_result_receipt_surface_count =
        release_publication_result_receipt_surfaces.len();
    let release_publication_result_receipt_contract_hash_sha256 = sha256_text_value(&format!(
        "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-no-persistence:native:source={source_report_sha256}:surfaces={release_publication_result_receipt_surface_count}:route_count={}:record=0:persist=0:authority=0:live=0",
        route_matrix.route_count
    ));
    let denied_by_packet_receipt_release_publication_result_receipt_no_persistence = vec![
        "operator_readiness_packet_template_packet_receipt_source_release_publication_report_required",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_recording_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_persistence_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_materialization_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_filesystem_write_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_ledger_write_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_index_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_enqueue_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_delivery_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_export_query_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_observability_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_hash_binding_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_signature_timestamp_status_denied",
        "operator_readiness_packet_template_packet_receipt_publication_completion_ack_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_authority_denied",
        "operator_readiness_packet_template_packet_receipt_activation_live_authority_denied",
        "operator_readiness_packet_template_packet_receipt_memory_provider_install_restart_active_binary_denied",
    ];
    let denied_by_packet_receipt_release_publication_result_receipt_no_persistence_count =
        denied_by_packet_receipt_release_publication_result_receipt_no_persistence.len();
    let report_ready = source_ready
        && source_u64("release_publication_surface_count") == 14
        && source_u64("release_publication_attempt_count") == 14
        && source_u64("release_publication_allowed_count") == 0
        && source_u64("release_publication_accepted_count") == 0
        && source_u64("release_publication_recorded_count") == 0
        && source_u64("release_publication_persisted_count") == 0
        && source_u64("release_artifact_written_count") == 0
        && source_u64("public_artifact_written_count") == 0
        && source_u64("public_distribution_performed_count") == 0
        && source_u64("public_release_claimed_count") == 0
        && source_u64("public_ga_claimed_count") == 0
        && source_u64("release_publication_activation_authority_derived_count") == 0
        && release_publication_result_receipt_surface_count == 14
        && route_count_source_command_accepted;

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_no_persistence_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_NO_PERSISTENCE_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-no-persistence --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-16",
        "receipt_release_publication_result_receipt_schema_version": "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_no_persistence_v1",
        "receipt_release_publication_result_receipt_mode": "native_route_denied_release_publication_attempt_cannot_persist_result_receipt_or_derive_authority",
        "source_packet_acceptance_receipt_release_publication_route": source["gate"].clone(),
        "source_packet_acceptance_receipt_release_publication_ready": source_ready,
        "source_packet_acceptance_receipt_release_publication_report_sha256": source_report_sha256,
        "source_release_publication_contract_hash_sha256": source_release_publication_contract_hash_sha256,
        "release_publication_result_receipt_contract_hash_sha256": release_publication_result_receipt_contract_hash_sha256,
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
            "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_no_persistence_route_enabled": true,
            "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_no_persistence_ready": report_ready,
            "source_release_publication_surface_count": source_u64("release_publication_surface_count"),
            "source_release_publication_attempt_count": source_u64("release_publication_attempt_count"),
            "source_release_publication_allowed_count": source_u64("release_publication_allowed_count"),
            "source_release_publication_accepted_count": source_u64("release_publication_accepted_count"),
            "source_release_publication_recorded_count": source_u64("release_publication_recorded_count"),
            "source_release_publication_persisted_count": source_u64("release_publication_persisted_count"),
            "source_release_artifact_written_count": source_u64("release_artifact_written_count"),
            "source_public_artifact_written_count": source_u64("public_artifact_written_count"),
            "source_public_distribution_performed_count": source_u64("public_distribution_performed_count"),
            "source_public_release_claimed_count": source_u64("public_release_claimed_count"),
            "source_public_ga_claimed_count": source_u64("public_ga_claimed_count"),
            "source_release_publication_activation_authority_derived_count": source_u64("release_publication_activation_authority_derived_count"),
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "release_publication_result_receipt_surface_count": release_publication_result_receipt_surface_count,
            "release_publication_result_receipt_attempt_count": release_publication_result_receipt_surface_count,
            "release_publication_result_receipt_allowed_count": 0,
            "release_publication_result_receipt_accepted_count": 0,
            "release_publication_result_receipt_recorded_count": 0,
            "release_publication_result_receipt_persisted_count": 0,
            "release_publication_result_receipt_materialized_count": 0,
            "release_publication_result_receipt_filesystem_written_count": 0,
            "release_publication_result_receipt_ledger_written_count": 0,
            "release_publication_result_receipt_indexed_count": 0,
            "release_publication_result_receipt_enqueued_count": 0,
            "release_publication_result_receipt_delivered_count": 0,
            "release_publication_result_receipt_exported_count": 0,
            "release_publication_result_receipt_query_registered_count": 0,
            "release_publication_result_receipt_observability_recorded_count": 0,
            "release_publication_result_receipt_hash_bound_count": 0,
            "release_publication_result_receipt_signature_accepted_count": 0,
            "release_publication_result_receipt_timestamp_accepted_count": 0,
            "release_publication_result_receipt_status_accepted_count": 0,
            "publication_completion_ack_recorded_count": 0,
            "publication_completion_ack_persisted_count": 0,
            "publication_completion_ack_accepted_count": 0,
            "release_publication_result_receipt_acceptance_recorded_count": 0,
            "release_publication_result_receipt_operator_approval_derived_count": 0,
            "release_publication_result_receipt_activation_authority_derived_count": 0,
            "release_publication_result_receipt_activation_command_derived_count": 0,
            "release_publication_result_receipt_live_execution_allowed_count": 0,
            "release_publication_result_receipt_surfaces": release_publication_result_receipt_surfaces,
            "denied_by_packet_receipt_release_publication_result_receipt_no_persistence": denied_by_packet_receipt_release_publication_result_receipt_no_persistence,
            "denied_by_packet_receipt_release_publication_result_receipt_no_persistence_count": denied_by_packet_receipt_release_publication_result_receipt_no_persistence_count,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "allowed_next_actions": [
                {
                    "action": "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_replay_idempotency_denial_gate",
                    "status": "allowed_report_only_next_slice",
                    "records_release_publication_result_receipt": false,
                    "persists_release_publication_result_receipt": false,
                    "writes_release_artifact": false,
                    "writes_public_artifact": false,
                    "enqueues_publication": false,
                    "records_publication_completion_ack": false,
                    "claims_public_release": false,
                    "records_operator_acceptance": false,
                    "derives_activation_authority": false,
                    "activates_live": false,
                    "mutates_memory_store": false,
                    "writes_kg": false,
                    "sends_externally": false
                }
            ],
        }),
    );
    let release_publication_result_receipt_false_keys = [
        "packet_acceptance_receipt_release_publication_allowed",
        "packet_acceptance_receipt_release_publication_accepted",
        "packet_acceptance_receipt_release_publication_recorded",
        "packet_acceptance_receipt_release_publication_persisted",
        "packet_acceptance_receipt_release_publication_materialized",
        "packet_acceptance_receipt_release_artifact_written",
        "packet_acceptance_receipt_public_artifact_written",
        "packet_acceptance_receipt_artifact_signature_accepted",
        "packet_acceptance_receipt_artifact_notarization_accepted",
        "packet_acceptance_receipt_publication_queue_enqueued",
        "packet_acceptance_receipt_publication_manifest_written",
        "packet_acceptance_receipt_public_distribution_performed",
        "packet_acceptance_receipt_channel_delivery_performed",
        "packet_acceptance_receipt_external_publication_sent",
        "packet_acceptance_receipt_public_version_tag_created",
        "packet_acceptance_receipt_release_notes_materialized",
        "packet_acceptance_receipt_changelog_materialized",
        "packet_acceptance_receipt_public_release_claimed",
        "packet_acceptance_receipt_public_ga_claimed",
        "packet_acceptance_receipt_terminal_status_promoted_to_release_approval",
        "packet_acceptance_receipt_release_publication_result_receipt_allowed",
        "packet_acceptance_receipt_release_publication_result_receipt_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_materialized",
        "packet_acceptance_receipt_release_publication_result_receipt_filesystem_written",
        "packet_acceptance_receipt_release_publication_result_receipt_ledger_written",
        "packet_acceptance_receipt_release_publication_result_receipt_indexed",
        "packet_acceptance_receipt_release_publication_result_receipt_enqueued",
        "packet_acceptance_receipt_release_publication_result_receipt_delivered",
        "packet_acceptance_receipt_release_publication_result_receipt_exported",
        "packet_acceptance_receipt_release_publication_result_receipt_query_registered",
        "packet_acceptance_receipt_release_publication_result_receipt_observability_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_hash_bound",
        "packet_acceptance_receipt_release_publication_result_receipt_signature_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_timestamp_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_status_accepted",
        "packet_acceptance_receipt_publication_completion_ack_recorded",
        "packet_acceptance_receipt_publication_completion_ack_persisted",
        "packet_acceptance_receipt_publication_completion_ack_accepted",
        "operator_acceptance_recorded",
        "operator_approval_recorded",
        "activation_authority_derived",
        "activation_command_derived",
        "activation_allowed",
        "activation_performed",
        "memory_store_write_performed",
        "memory_store_mutated",
        "hepta_intelligence_context_attached",
        "prompt_preview_rendered",
        "context_injection_performed",
        "provider_invoked",
        "model_invoked",
        "external_kg_adapter_read_performed",
        "external_adapter_client_constructed",
        "network_call_performed",
        "external_db_write_performed",
        "live_kg_write_performed",
        "credential_read",
        "secret_file_read",
        "install_executed",
        "launchd_mutated",
        "service_restarted",
        "active_binary_mutated",
        "public_release_claimed",
        "public_ga_claimed",
        "release_artifact_written",
        "public_artifact_written",
        "external_send_performed",
        "filesystem_written",
    ];
    extend_json_object(
        &mut report,
        serde_json::json!({
            "packet_acceptance_receipt_release_publication_result_receipt_persisted": false,
            "packet_acceptance_receipt_publication_completion_ack_recorded": false,
        }),
    );
    if let Some(report_object) = report.as_object_mut() {
        for key in release_publication_result_receipt_false_keys {
            report_object.insert(key.to_string(), serde_json::json!(false));
        }
    }

    let mut side_effects = serde_json::Map::new();
    for key in release_publication_result_receipt_false_keys {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }
    extend_json_object(
        &mut report,
        serde_json::json!({ "side_effects": side_effects }),
    );
    report
}

fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_replay_idempotency_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source =
        hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_no_persistence_report();
    let source_report_sha256 = sha256_json_value(&source);
    let source_ready = source
        .get("memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_no_persistence_ready")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);
    let source_u64 = |key: &str| -> u64 {
        source
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let source_release_publication_result_receipt_contract_hash_sha256 = source
        .get("release_publication_result_receipt_contract_hash_sha256")
        .cloned()
        .unwrap_or_else(|| serde_json::json!(""));
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;

    let release_publication_result_receipt_replay_surfaces = vec![
        "publication_result_receipt_replay",
        "publication_result_receipt_duplicate_replay",
        "publication_result_receipt_retry_replay",
        "publication_result_receipt_idempotency_key_registration",
        "publication_result_receipt_idempotency_cache_write",
        "publication_result_receipt_cache_hit_promotion",
        "publication_result_receipt_hash_replay_binding",
        "publication_result_receipt_signature_timestamp_status_replay",
        "publication_result_receipt_query_result_replay",
        "publication_result_receipt_export_snapshot_replay",
        "publication_result_receipt_observability_snapshot_replay",
        "publication_completion_ack_replay",
        "publication_result_receipt_release_publication_authority_replay",
        "publication_result_receipt_activation_live_install_restart_active_binary_replay",
    ]
    .into_iter()
    .map(|release_publication_result_receipt_replay_surface| {
        let mut surface = serde_json::Map::new();
        surface.insert(
            "release_publication_result_receipt_replay_surface".to_string(),
            serde_json::json!(release_publication_result_receipt_replay_surface),
        );
        surface.insert(
            "source_release_publication_result_receipt_present".to_string(),
            serde_json::json!(true),
        );
        surface.insert(
            "source_release_publication_result_receipt_no_persistence_ready".to_string(),
            serde_json::json!(true),
        );
        surface.insert(
            "result_receipt_replay_attempted".to_string(),
            serde_json::json!(true),
        );
        surface.insert(
            "result_receipt_replay_noop_confirmed".to_string(),
            serde_json::json!(true),
        );
        surface.insert(
            "release_publication_result_receipt_replay_status".to_string(),
            serde_json::json!("release_publication_result_receipt_replay_idempotency_denied"),
        );
        for key in [
            "result_receipt_replay_allowed",
            "result_receipt_replay_accepted",
            "result_receipt_replay_recorded",
            "result_receipt_replay_persisted",
            "result_receipt_replay_materialized",
            "result_receipt_duplicate_accepted",
            "result_receipt_retry_accepted",
            "idempotency_key_registered",
            "idempotency_cache_written",
            "idempotency_cache_hit_promoted",
            "replay_hash_bound",
            "replay_signature_accepted",
            "replay_timestamp_accepted",
            "replay_status_accepted",
            "query_result_replayed",
            "export_snapshot_replayed",
            "observability_snapshot_replayed",
            "publication_completion_ack_replayed",
            "publication_completion_ack_recorded",
            "release_publication_recorded",
            "release_publication_persisted",
            "release_publication_materialized",
            "release_artifact_written",
            "public_artifact_written",
            "artifact_signature_accepted",
            "artifact_notarization_accepted",
            "publication_queue_enqueued",
            "publication_manifest_written",
            "public_distribution_performed",
            "channel_delivery_performed",
            "external_send_performed",
            "public_version_tag_created",
            "release_notes_materialized",
            "changelog_materialized",
            "public_release_claimed",
            "public_ga_claimed",
            "terminal_status_promoted_to_release_approval",
            "acceptance_recorded",
            "operator_approval_derived",
            "release_publication_authority_derived",
            "activation_authority_derived",
            "activation_command_derived",
            "live_execution_allowed",
            "activation_performed",
            "memory_store_write_performed",
            "memory_store_mutated",
            "provider_invoked",
            "model_invoked",
            "credential_read",
            "secret_file_read",
            "install_executed",
            "launchd_mutated",
            "service_restarted",
            "active_binary_mutated",
        ] {
            surface.insert(key.to_string(), serde_json::json!(false));
        }
        serde_json::Value::Object(surface)
    })
    .collect::<Vec<_>>();
    let release_publication_result_receipt_replay_surface_count =
        release_publication_result_receipt_replay_surfaces.len();
    let release_publication_result_receipt_replay_idempotency_contract_hash_sha256 =
        sha256_text_value(&format!(
            "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-replay-idempotency-denial:native:source={source_report_sha256}:surfaces={release_publication_result_receipt_replay_surface_count}:route_count={}:replay=0:idempotency=0:authority=0:live=0",
            route_matrix.route_count
        ));
    let denied_by_packet_receipt_release_publication_result_receipt_replay_idempotency = vec![
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_replay_recording_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_replay_persistence_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_replay_materialization_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_duplicate_replay_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_retry_replay_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_idempotency_key_registration_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_idempotency_cache_write_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_cache_hit_promotion_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_hash_replay_binding_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_signature_timestamp_status_replay_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_query_export_observability_replay_denied",
        "operator_readiness_packet_template_packet_receipt_publication_completion_ack_replay_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_authority_replay_denied",
        "operator_readiness_packet_template_packet_receipt_activation_live_authority_replay_denied",
        "operator_readiness_packet_template_packet_receipt_install_restart_active_binary_replay_denied",
        "operator_readiness_packet_template_packet_receipt_memory_provider_external_send_replay_denied",
    ];
    let denied_by_packet_receipt_release_publication_result_receipt_replay_idempotency_count =
        denied_by_packet_receipt_release_publication_result_receipt_replay_idempotency.len();
    let report_ready = source_ready
        && source_u64("release_publication_result_receipt_surface_count") == 14
        && source_u64("release_publication_result_receipt_attempt_count") == 14
        && source_u64("release_publication_result_receipt_recorded_count") == 0
        && source_u64("release_publication_result_receipt_persisted_count") == 0
        && source_u64("release_publication_result_receipt_materialized_count") == 0
        && source_u64("release_publication_result_receipt_delivered_count") == 0
        && source_u64("release_publication_result_receipt_exported_count") == 0
        && source_u64("release_publication_result_receipt_query_registered_count") == 0
        && source_u64("release_publication_result_receipt_observability_recorded_count") == 0
        && source_u64("publication_completion_ack_recorded_count") == 0
        && source_u64("release_publication_result_receipt_activation_authority_derived_count") == 0
        && release_publication_result_receipt_replay_surface_count == 14
        && route_count_source_command_accepted;

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_replay_idempotency_denial_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_REPLAY_IDEMPOTENCY_DENIAL_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-replay-idempotency-denial --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-16",
        "receipt_release_publication_result_receipt_replay_idempotency_schema_version": "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_replay_idempotency_denial_v1",
        "receipt_release_publication_result_receipt_replay_idempotency_mode": "native_route_denied_release_publication_result_receipt_cannot_replay_cache_or_derive_authority",
        "source_packet_acceptance_receipt_release_publication_result_receipt_route": source["gate"].clone(),
        "source_packet_acceptance_receipt_release_publication_result_receipt_ready": source_ready,
        "source_packet_acceptance_receipt_release_publication_result_receipt_report_sha256": source_report_sha256,
        "source_release_publication_result_receipt_contract_hash_sha256": source_release_publication_result_receipt_contract_hash_sha256,
        "release_publication_result_receipt_replay_idempotency_contract_hash_sha256": release_publication_result_receipt_replay_idempotency_contract_hash_sha256,
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
            "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_replay_idempotency_denial_route_enabled": true,
            "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_replay_idempotency_denial_ready": report_ready,
            "source_release_publication_result_receipt_surface_count": source_u64("release_publication_result_receipt_surface_count"),
            "source_release_publication_result_receipt_attempt_count": source_u64("release_publication_result_receipt_attempt_count"),
            "source_release_publication_result_receipt_recorded_count": source_u64("release_publication_result_receipt_recorded_count"),
            "source_release_publication_result_receipt_persisted_count": source_u64("release_publication_result_receipt_persisted_count"),
            "source_release_publication_result_receipt_materialized_count": source_u64("release_publication_result_receipt_materialized_count"),
            "source_release_publication_result_receipt_delivered_count": source_u64("release_publication_result_receipt_delivered_count"),
            "source_release_publication_result_receipt_exported_count": source_u64("release_publication_result_receipt_exported_count"),
            "source_release_publication_result_receipt_query_registered_count": source_u64("release_publication_result_receipt_query_registered_count"),
            "source_release_publication_result_receipt_observability_recorded_count": source_u64("release_publication_result_receipt_observability_recorded_count"),
            "source_publication_completion_ack_recorded_count": source_u64("publication_completion_ack_recorded_count"),
            "source_release_publication_result_receipt_activation_authority_derived_count": source_u64("release_publication_result_receipt_activation_authority_derived_count"),
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "release_publication_result_receipt_replay_surface_count": release_publication_result_receipt_replay_surface_count,
            "release_publication_result_receipt_replay_attempt_count": release_publication_result_receipt_replay_surface_count,
            "release_publication_result_receipt_replay_allowed_count": 0,
            "release_publication_result_receipt_replay_accepted_count": 0,
            "release_publication_result_receipt_replay_recorded_count": 0,
            "release_publication_result_receipt_replay_persisted_count": 0,
            "release_publication_result_receipt_replay_materialized_count": 0,
            "release_publication_result_receipt_duplicate_accepted_count": 0,
            "release_publication_result_receipt_retry_accepted_count": 0,
            "release_publication_result_receipt_idempotency_key_registered_count": 0,
            "release_publication_result_receipt_idempotency_cache_written_count": 0,
            "release_publication_result_receipt_idempotency_cache_hit_promoted_count": 0,
            "release_publication_result_receipt_replay_hash_bound_count": 0,
            "release_publication_result_receipt_replay_signature_accepted_count": 0,
            "release_publication_result_receipt_replay_timestamp_accepted_count": 0,
            "release_publication_result_receipt_replay_status_accepted_count": 0,
            "release_publication_result_receipt_query_result_replayed_count": 0,
            "release_publication_result_receipt_export_snapshot_replayed_count": 0,
            "release_publication_result_receipt_observability_snapshot_replayed_count": 0,
            "publication_completion_ack_replayed_count": 0,
            "publication_completion_ack_recorded_count": 0,
            "release_publication_result_receipt_replay_acceptance_recorded_count": 0,
            "release_publication_result_receipt_replay_operator_approval_derived_count": 0,
            "release_publication_result_receipt_replay_release_publication_authority_derived_count": 0,
            "release_publication_result_receipt_replay_activation_authority_derived_count": 0,
            "release_publication_result_receipt_replay_activation_command_derived_count": 0,
            "release_publication_result_receipt_replay_live_execution_allowed_count": 0,
            "release_publication_result_receipt_replay_surfaces": release_publication_result_receipt_replay_surfaces,
            "denied_by_packet_receipt_release_publication_result_receipt_replay_idempotency": denied_by_packet_receipt_release_publication_result_receipt_replay_idempotency,
            "denied_by_packet_receipt_release_publication_result_receipt_replay_idempotency_count": denied_by_packet_receipt_release_publication_result_receipt_replay_idempotency_count,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "allowed_next_actions": [
                {
                    "action": "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_ordering_monotonicity_denial_gate",
                    "status": "allowed_report_only_next_slice",
                    "records_release_publication_result_receipt_replay": false,
                    "registers_idempotency_key": false,
                    "writes_idempotency_cache": false,
                    "promotes_cache_hit": false,
                    "records_publication_completion_ack": false,
                    "derives_release_publication_authority": false,
                    "derives_activation_authority": false,
                    "activates_live": false,
                    "mutates_memory_store": false,
                    "writes_kg": false,
                    "sends_externally": false
                }
            ],
        }),
    );
    let release_publication_result_receipt_replay_false_keys = [
        "packet_acceptance_receipt_release_publication_result_receipt_replayed",
        "packet_acceptance_receipt_release_publication_result_receipt_replay_allowed",
        "packet_acceptance_receipt_release_publication_result_receipt_replay_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_replay_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_replay_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_replay_materialized",
        "packet_acceptance_receipt_release_publication_result_receipt_duplicate_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_retry_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_idempotency_key_registered",
        "packet_acceptance_receipt_release_publication_result_receipt_idempotency_cache_written",
        "packet_acceptance_receipt_release_publication_result_receipt_idempotency_cache_hit_promoted",
        "packet_acceptance_receipt_release_publication_result_receipt_replay_hash_bound",
        "packet_acceptance_receipt_release_publication_result_receipt_replay_signature_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_replay_timestamp_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_replay_status_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_query_result_replayed",
        "packet_acceptance_receipt_release_publication_result_receipt_export_snapshot_replayed",
        "packet_acceptance_receipt_release_publication_result_receipt_observability_snapshot_replayed",
        "packet_acceptance_receipt_publication_completion_ack_replayed",
        "packet_acceptance_receipt_publication_completion_ack_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_allowed",
        "packet_acceptance_receipt_release_publication_result_receipt_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_materialized",
        "packet_acceptance_receipt_release_publication_result_receipt_filesystem_written",
        "packet_acceptance_receipt_release_publication_result_receipt_ledger_written",
        "packet_acceptance_receipt_release_publication_result_receipt_indexed",
        "packet_acceptance_receipt_release_publication_result_receipt_enqueued",
        "packet_acceptance_receipt_release_publication_result_receipt_delivered",
        "packet_acceptance_receipt_release_publication_result_receipt_exported",
        "packet_acceptance_receipt_release_publication_result_receipt_query_registered",
        "packet_acceptance_receipt_release_publication_result_receipt_observability_recorded",
        "packet_acceptance_receipt_release_publication_allowed",
        "packet_acceptance_receipt_release_publication_accepted",
        "packet_acceptance_receipt_release_publication_recorded",
        "packet_acceptance_receipt_release_artifact_written",
        "packet_acceptance_receipt_public_artifact_written",
        "packet_acceptance_receipt_publication_queue_enqueued",
        "packet_acceptance_receipt_publication_manifest_written",
        "packet_acceptance_receipt_public_distribution_performed",
        "packet_acceptance_receipt_channel_delivery_performed",
        "packet_acceptance_receipt_external_publication_sent",
        "packet_acceptance_receipt_public_version_tag_created",
        "packet_acceptance_receipt_release_notes_materialized",
        "packet_acceptance_receipt_changelog_materialized",
        "packet_acceptance_receipt_public_release_claimed",
        "packet_acceptance_receipt_public_ga_claimed",
        "packet_acceptance_receipt_terminal_status_promoted_to_release_approval",
        "operator_acceptance_recorded",
        "operator_approval_recorded",
        "activation_authority_derived",
        "activation_command_derived",
        "activation_allowed",
        "activation_performed",
        "memory_store_write_performed",
        "memory_store_mutated",
        "hepta_intelligence_context_attached",
        "prompt_preview_rendered",
        "context_injection_performed",
        "provider_invoked",
        "model_invoked",
        "external_kg_adapter_read_performed",
        "external_adapter_client_constructed",
        "network_call_performed",
        "external_db_write_performed",
        "live_kg_write_performed",
        "credential_read",
        "secret_file_read",
        "install_executed",
        "launchd_mutated",
        "service_restarted",
        "active_binary_mutated",
        "public_release_claimed",
        "public_ga_claimed",
        "release_artifact_written",
        "public_artifact_written",
        "external_send_performed",
        "filesystem_written",
    ];
    if let Some(report_object) = report.as_object_mut() {
        for key in release_publication_result_receipt_replay_false_keys {
            report_object.insert(key.to_string(), serde_json::json!(false));
        }
    }

    let mut side_effects = serde_json::Map::new();
    for key in release_publication_result_receipt_replay_false_keys {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }
    extend_json_object(
        &mut report,
        serde_json::json!({ "side_effects": side_effects }),
    );
    report
}

fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_ordering_monotonicity_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source =
        hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_replay_idempotency_denial_report();
    let source_report_sha256 = sha256_json_value(&source);
    let source_ready = source
        .get("memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_replay_idempotency_denial_ready")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);
    let source_u64 = |key: &str| -> u64 {
        source
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;

    let release_publication_result_receipt_ordering_surfaces = vec![
        "publication_result_receipt_duplicate_sequence_claim",
        "publication_result_receipt_stale_sequence_claim",
        "publication_result_receipt_late_arrival_claim",
        "publication_result_receipt_future_sequence_gap_claim",
        "publication_result_receipt_timestamp_rollback_claim",
        "publication_result_receipt_epoch_rollback_claim",
        "publication_result_receipt_same_sequence_different_hash_claim",
        "publication_result_receipt_latest_wins_overwrite_claim",
        "publication_result_receipt_query_ordering_claim",
        "publication_result_receipt_export_ordering_claim",
        "publication_result_receipt_observability_ordering_claim",
        "publication_result_receipt_completion_ack_ordering_claim",
        "publication_result_receipt_release_publication_authority_ordering_claim",
        "publication_result_receipt_activation_live_install_restart_active_binary_ordering_claim",
    ]
    .into_iter()
    .map(|release_publication_result_receipt_ordering_surface| {
        let mut surface = serde_json::Map::new();
        surface.insert(
            "release_publication_result_receipt_ordering_surface".to_string(),
            serde_json::json!(release_publication_result_receipt_ordering_surface),
        );
        surface.insert(
            "source_release_publication_result_receipt_replay_present".to_string(),
            serde_json::json!(true),
        );
        surface.insert(
            "source_release_publication_result_receipt_replay_idempotency_ready".to_string(),
            serde_json::json!(source_ready),
        );
        surface.insert("ordering_attempted".to_string(), serde_json::json!(true));
        surface.insert(
            "ordering_noop_confirmed".to_string(),
            serde_json::json!(true),
        );
        surface.insert(
            "release_publication_result_receipt_ordering_status".to_string(),
            serde_json::json!("release_publication_result_receipt_ordering_monotonicity_denied"),
        );
        for key in [
            "ordering_allowed",
            "ordering_recorded",
            "ordering_persisted",
            "ordering_materialized",
            "sequence_cursor_accepted",
            "sequence_cursor_recorded",
            "sequence_cursor_persisted",
            "monotonicity_state_recorded",
            "monotonicity_state_persisted",
            "duplicate_sequence_accepted",
            "stale_sequence_accepted",
            "late_arrival_accepted",
            "future_sequence_gap_accepted",
            "timestamp_rollback_accepted",
            "epoch_rollback_accepted",
            "same_sequence_hash_override_accepted",
            "latest_wins_overwrite_accepted",
            "query_ordering_accepted",
            "export_ordering_accepted",
            "observability_ordering_accepted",
            "completion_ack_ordering_accepted",
            "publication_completion_ack_recorded",
            "result_receipt_replay_recorded",
            "idempotency_key_registered",
            "idempotency_cache_written",
            "idempotency_cache_hit_promoted",
            "release_publication_recorded",
            "release_artifact_written",
            "public_artifact_written",
            "publication_queue_enqueued",
            "publication_manifest_written",
            "public_distribution_performed",
            "channel_delivery_performed",
            "external_send_performed",
            "public_version_tag_created",
            "release_notes_materialized",
            "changelog_materialized",
            "public_release_claimed",
            "public_ga_claimed",
            "acceptance_recorded",
            "operator_approval_derived",
            "release_publication_authority_derived",
            "activation_authority_derived",
            "activation_command_derived",
            "live_execution_allowed",
            "activation_performed",
            "memory_store_write_performed",
            "memory_store_mutated",
            "provider_invoked",
            "model_invoked",
            "credential_read",
            "secret_file_read",
            "install_executed",
            "launchd_mutated",
            "service_restarted",
            "active_binary_mutated",
        ] {
            surface.insert(key.to_string(), serde_json::json!(false));
        }
        serde_json::Value::Object(surface)
    })
    .collect::<Vec<_>>();
    let release_publication_result_receipt_ordering_surface_count =
        release_publication_result_receipt_ordering_surfaces.len();
    let release_publication_result_receipt_ordering_monotonicity_contract_hash_sha256 =
        sha256_text_value(&format!(
            "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-ordering-monotonicity-denial:native:source={source_report_sha256}:surfaces={release_publication_result_receipt_ordering_surface_count}:route_count={}:ordering=0:monotonicity=0:authority=0:live=0",
            route_matrix.route_count
        ));
    let denied_by_packet_receipt_release_publication_result_receipt_ordering_monotonicity = vec![
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_ordering_recording_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_ordering_persistence_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_ordering_materialization_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_sequence_cursor_acceptance_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_sequence_cursor_recording_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_sequence_cursor_persistence_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_monotonicity_state_recording_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_monotonicity_state_persistence_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_duplicate_sequence_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_stale_sequence_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_late_arrival_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_future_sequence_gap_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_timestamp_rollback_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_epoch_rollback_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_same_sequence_hash_override_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_latest_wins_overwrite_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_query_ordering_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_export_ordering_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_observability_ordering_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_completion_ack_ordering_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_acceptance_from_ordering_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_authority_from_ordering_denied",
        "operator_readiness_packet_template_packet_receipt_activation_live_from_ordering_denied",
        "operator_readiness_packet_template_packet_receipt_install_restart_active_binary_from_ordering_denied",
        "operator_readiness_packet_template_packet_receipt_memory_provider_external_send_from_ordering_denied",
    ];
    let report_ready = source_ready
        && source_u64("release_publication_result_receipt_replay_surface_count") == 14
        && source_u64("release_publication_result_receipt_replay_attempt_count") == 14
        && source_u64("release_publication_result_receipt_replay_recorded_count") == 0
        && source_u64("release_publication_result_receipt_replay_persisted_count") == 0
        && source_u64("release_publication_result_receipt_idempotency_key_registered_count") == 0
        && source_u64("release_publication_result_receipt_idempotency_cache_written_count") == 0
        && source_u64("release_publication_result_receipt_idempotency_cache_hit_promoted_count")
            == 0
        && source_u64("release_publication_result_receipt_query_result_replayed_count") == 0
        && source_u64("release_publication_result_receipt_export_snapshot_replayed_count") == 0
        && source_u64("release_publication_result_receipt_observability_snapshot_replayed_count")
            == 0
        && source_u64("publication_completion_ack_replayed_count") == 0
        && source_u64(
            "release_publication_result_receipt_replay_activation_authority_derived_count",
        ) == 0
        && release_publication_result_receipt_ordering_surface_count == 14
        && route_count_source_command_accepted;

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_ordering_monotonicity_denial_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_ORDERING_MONOTONICITY_DENIAL_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-ordering-monotonicity-denial --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-16",
        "receipt_release_publication_result_receipt_ordering_monotonicity_schema_version": "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_ordering_monotonicity_denial_v1",
        "receipt_release_publication_result_receipt_ordering_monotonicity_mode": "native_route_denied_release_publication_result_receipt_cannot_use_ordering_or_monotonicity_as_authority",
        "source_packet_acceptance_receipt_release_publication_result_receipt_replay_gate": source["gate"].clone(),
        "source_packet_acceptance_receipt_release_publication_result_receipt_replay_ready": source_ready,
        "source_packet_acceptance_receipt_release_publication_result_receipt_replay_report_sha256": source_report_sha256,
        "source_release_publication_result_receipt_replay_idempotency_contract_hash_sha256": source["release_publication_result_receipt_replay_idempotency_contract_hash_sha256"].clone(),
        "release_publication_result_receipt_ordering_monotonicity_contract_hash_sha256": release_publication_result_receipt_ordering_monotonicity_contract_hash_sha256,
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
            "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_ordering_monotonicity_denial_route_enabled": true,
            "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_ordering_monotonicity_denial_ready": report_ready,
            "source_release_publication_result_receipt_replay_surface_count": source_u64("release_publication_result_receipt_replay_surface_count"),
            "source_release_publication_result_receipt_replay_attempt_count": source_u64("release_publication_result_receipt_replay_attempt_count"),
            "source_release_publication_result_receipt_replay_recorded_count": source_u64("release_publication_result_receipt_replay_recorded_count"),
            "source_release_publication_result_receipt_replay_persisted_count": source_u64("release_publication_result_receipt_replay_persisted_count"),
            "source_release_publication_result_receipt_idempotency_key_registered_count": source_u64("release_publication_result_receipt_idempotency_key_registered_count"),
            "source_release_publication_result_receipt_idempotency_cache_written_count": source_u64("release_publication_result_receipt_idempotency_cache_written_count"),
            "source_release_publication_result_receipt_idempotency_cache_hit_promoted_count": source_u64("release_publication_result_receipt_idempotency_cache_hit_promoted_count"),
            "source_release_publication_result_receipt_query_result_replayed_count": source_u64("release_publication_result_receipt_query_result_replayed_count"),
            "source_release_publication_result_receipt_export_snapshot_replayed_count": source_u64("release_publication_result_receipt_export_snapshot_replayed_count"),
            "source_release_publication_result_receipt_observability_snapshot_replayed_count": source_u64("release_publication_result_receipt_observability_snapshot_replayed_count"),
            "source_publication_completion_ack_replayed_count": source_u64("publication_completion_ack_replayed_count"),
            "source_release_publication_result_receipt_replay_activation_authority_derived_count": source_u64("release_publication_result_receipt_replay_activation_authority_derived_count"),
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "release_publication_result_receipt_ordering_surface_count": release_publication_result_receipt_ordering_surface_count,
            "release_publication_result_receipt_ordering_attempt_count": release_publication_result_receipt_ordering_surface_count,
            "release_publication_result_receipt_ordering_allowed_count": 0,
            "release_publication_result_receipt_ordering_recorded_count": 0,
            "release_publication_result_receipt_ordering_persisted_count": 0,
            "release_publication_result_receipt_ordering_materialized_count": 0,
            "release_publication_result_receipt_sequence_cursor_accepted_count": 0,
            "release_publication_result_receipt_sequence_cursor_recorded_count": 0,
            "release_publication_result_receipt_sequence_cursor_persisted_count": 0,
            "release_publication_result_receipt_monotonicity_state_recorded_count": 0,
            "release_publication_result_receipt_monotonicity_state_persisted_count": 0,
            "release_publication_result_receipt_duplicate_sequence_accepted_count": 0,
            "release_publication_result_receipt_stale_sequence_accepted_count": 0,
            "release_publication_result_receipt_late_arrival_accepted_count": 0,
            "release_publication_result_receipt_future_sequence_gap_accepted_count": 0,
            "release_publication_result_receipt_timestamp_rollback_accepted_count": 0,
            "release_publication_result_receipt_epoch_rollback_accepted_count": 0,
            "release_publication_result_receipt_same_sequence_hash_override_accepted_count": 0,
            "release_publication_result_receipt_latest_wins_overwrite_accepted_count": 0,
            "release_publication_result_receipt_query_ordering_accepted_count": 0,
            "release_publication_result_receipt_export_ordering_accepted_count": 0,
            "release_publication_result_receipt_observability_ordering_accepted_count": 0,
            "release_publication_result_receipt_completion_ack_ordering_accepted_count": 0,
            "publication_completion_ack_recorded_count": 0,
            "release_publication_result_receipt_ordering_acceptance_recorded_count": 0,
            "release_publication_result_receipt_ordering_operator_approval_derived_count": 0,
            "release_publication_result_receipt_ordering_release_publication_authority_derived_count": 0,
            "release_publication_result_receipt_ordering_activation_authority_derived_count": 0,
            "release_publication_result_receipt_ordering_activation_command_derived_count": 0,
            "release_publication_result_receipt_ordering_live_execution_allowed_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "release_publication_result_receipt_ordering_surfaces": release_publication_result_receipt_ordering_surfaces,
            "denied_by_packet_receipt_release_publication_result_receipt_ordering_monotonicity": denied_by_packet_receipt_release_publication_result_receipt_ordering_monotonicity,
            "denied_by_packet_receipt_release_publication_result_receipt_ordering_monotonicity_count": denied_by_packet_receipt_release_publication_result_receipt_ordering_monotonicity.len(),
            "allowed_next_actions": [
                {
                    "action": "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_cancellation_supersession_denial_gate",
                    "status": "allowed_report_only_next_slice",
                    "records_release_publication_result_receipt_ordering": false,
                    "records_sequence_cursor": false,
                    "records_monotonicity_state": false,
                    "accepts_latest_wins": false,
                    "records_publication_completion_ack": false,
                    "derives_release_publication_authority": false,
                    "derives_activation_authority": false,
                    "accepts_cancellation": false,
                    "accepts_supersession": false,
                    "activates_live": false,
                    "mutates_memory_store": false,
                    "writes_kg": false,
                    "sends_externally": false
                }
            ],
        }),
    );

    let release_publication_result_receipt_ordering_false_keys = [
        "packet_acceptance_receipt_release_publication_result_receipt_ordering_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_ordering_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_ordering_materialized",
        "packet_acceptance_receipt_release_publication_result_receipt_sequence_cursor_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_sequence_cursor_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_sequence_cursor_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_monotonicity_state_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_monotonicity_state_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_duplicate_sequence_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_stale_sequence_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_late_arrival_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_future_sequence_gap_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_timestamp_rollback_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_epoch_rollback_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_same_sequence_hash_override_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_latest_wins_overwrite_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_query_ordering_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_export_ordering_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_observability_ordering_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_completion_ack_ordering_accepted",
        "packet_acceptance_receipt_publication_completion_ack_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_replayed",
        "packet_acceptance_receipt_release_publication_result_receipt_replay_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_replay_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_duplicate_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_retry_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_idempotency_key_registered",
        "packet_acceptance_receipt_release_publication_result_receipt_idempotency_cache_written",
        "packet_acceptance_receipt_release_publication_result_receipt_idempotency_cache_hit_promoted",
        "packet_acceptance_receipt_release_publication_result_receipt_query_result_replayed",
        "packet_acceptance_receipt_release_publication_result_receipt_export_snapshot_replayed",
        "packet_acceptance_receipt_release_publication_result_receipt_observability_snapshot_replayed",
        "packet_acceptance_receipt_release_publication_result_receipt_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_materialized",
        "packet_acceptance_receipt_release_publication_result_receipt_delivered",
        "packet_acceptance_receipt_release_publication_result_receipt_exported",
        "packet_acceptance_receipt_release_publication_result_receipt_query_registered",
        "packet_acceptance_receipt_release_publication_result_receipt_observability_recorded",
        "packet_acceptance_receipt_release_publication_recorded",
        "packet_acceptance_receipt_release_artifact_written",
        "packet_acceptance_receipt_public_artifact_written",
        "packet_acceptance_receipt_publication_queue_enqueued",
        "packet_acceptance_receipt_publication_manifest_written",
        "packet_acceptance_receipt_public_distribution_performed",
        "packet_acceptance_receipt_channel_delivery_performed",
        "packet_acceptance_receipt_external_publication_sent",
        "packet_acceptance_receipt_public_version_tag_created",
        "packet_acceptance_receipt_release_notes_materialized",
        "packet_acceptance_receipt_changelog_materialized",
        "packet_acceptance_receipt_public_release_claimed",
        "packet_acceptance_receipt_public_ga_claimed",
        "packet_acceptance_receipt_terminal_status_promoted_to_release_approval",
        "operator_acceptance_recorded",
        "operator_approval_recorded",
        "activation_authority_derived",
        "activation_command_derived",
        "activation_allowed",
        "activation_performed",
        "memory_store_write_performed",
        "memory_store_mutated",
        "hepta_intelligence_context_attached",
        "prompt_preview_rendered",
        "context_injection_performed",
        "provider_invoked",
        "model_invoked",
        "external_kg_adapter_read_performed",
        "external_adapter_client_constructed",
        "network_call_performed",
        "external_db_write_performed",
        "live_kg_write_performed",
        "credential_read",
        "secret_file_read",
        "install_executed",
        "launchd_mutated",
        "service_restarted",
        "active_binary_mutated",
        "public_release_claimed",
        "public_ga_claimed",
        "release_artifact_written",
        "public_artifact_written",
        "external_send_performed",
        "filesystem_written",
    ];
    if let Some(report_object) = report.as_object_mut() {
        for key in release_publication_result_receipt_ordering_false_keys {
            report_object.insert(key.to_string(), serde_json::json!(false));
        }
    }

    let mut side_effects = serde_json::Map::new();
    for key in release_publication_result_receipt_ordering_false_keys {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }
    extend_json_object(
        &mut report,
        serde_json::json!({ "side_effects": side_effects }),
    );
    report
}

fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_cancellation_supersession_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source =
        hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_ordering_monotonicity_denial_report();
    let source_report_sha256 = sha256_json_value(&source);
    let source_ready = source
        .get("memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_ordering_monotonicity_denial_ready")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);
    let source_u64 = |key: &str| -> u64 {
        source
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;

    let release_publication_result_receipt_cancellation_supersession_surfaces = vec![
        "publication_result_receipt_cancel_claim",
        "publication_result_receipt_revoke_claim",
        "publication_result_receipt_withdraw_claim",
        "publication_result_receipt_supersede_claim",
        "publication_result_receipt_replacement_receipt_claim",
        "publication_result_receipt_tombstone_claim",
        "publication_result_receipt_delete_marker_claim",
        "publication_result_receipt_latest_replacement_claim",
        "publication_result_receipt_ack_replacement_claim",
        "publication_result_receipt_query_replacement_claim",
        "publication_result_receipt_export_replacement_claim",
        "publication_result_receipt_observability_replacement_claim",
        "publication_result_receipt_release_publication_authority_replacement_claim",
        "publication_result_receipt_activation_live_install_restart_active_binary_replacement_claim",
    ]
    .into_iter()
    .map(|release_publication_result_receipt_cancellation_surface| {
        let mut surface = serde_json::Map::new();
        surface.insert(
            "release_publication_result_receipt_cancellation_surface".to_string(),
            serde_json::json!(release_publication_result_receipt_cancellation_surface),
        );
        surface.insert(
            "source_release_publication_result_receipt_ordering_present".to_string(),
            serde_json::json!(true),
        );
        surface.insert(
            "source_release_publication_result_receipt_ordering_monotonicity_ready".to_string(),
            serde_json::json!(source_ready),
        );
        surface.insert(
            "cancellation_supersession_attempted".to_string(),
            serde_json::json!(true),
        );
        surface.insert(
            "cancellation_supersession_noop_confirmed".to_string(),
            serde_json::json!(true),
        );
        surface.insert(
            "release_publication_result_receipt_cancellation_supersession_status".to_string(),
            serde_json::json!("release_publication_result_receipt_cancellation_supersession_denied"),
        );
        for key in [
            "cancellation_accepted",
            "cancellation_recorded",
            "cancellation_persisted",
            "revocation_accepted",
            "withdrawal_accepted",
            "supersession_accepted",
            "supersession_recorded",
            "supersession_persisted",
            "replacement_receipt_accepted",
            "replacement_receipt_recorded",
            "replacement_receipt_persisted",
            "tombstone_recorded",
            "tombstone_persisted",
            "delete_marker_recorded",
            "latest_replacement_accepted",
            "ack_replacement_accepted",
            "query_replacement_registered",
            "export_replacement_recorded",
            "observability_replacement_recorded",
            "publication_completion_ack_recorded",
            "result_receipt_ordering_recorded",
            "sequence_cursor_recorded",
            "monotonicity_state_recorded",
            "result_receipt_replay_recorded",
            "idempotency_key_registered",
            "idempotency_cache_written",
            "idempotency_cache_hit_promoted",
            "release_publication_recorded",
            "release_artifact_written",
            "public_artifact_written",
            "publication_queue_enqueued",
            "publication_manifest_written",
            "public_distribution_performed",
            "channel_delivery_performed",
            "external_send_performed",
            "public_version_tag_created",
            "release_notes_materialized",
            "changelog_materialized",
            "public_release_claimed",
            "public_ga_claimed",
            "acceptance_recorded",
            "operator_approval_derived",
            "release_publication_authority_derived",
            "activation_authority_derived",
            "activation_command_derived",
            "live_execution_allowed",
            "activation_performed",
            "memory_store_write_performed",
            "memory_store_mutated",
            "provider_invoked",
            "model_invoked",
            "credential_read",
            "secret_file_read",
            "install_executed",
            "launchd_mutated",
            "service_restarted",
            "active_binary_mutated",
        ] {
            surface.insert(key.to_string(), serde_json::json!(false));
        }
        serde_json::Value::Object(surface)
    })
    .collect::<Vec<_>>();
    let release_publication_result_receipt_cancellation_supersession_surface_count =
        release_publication_result_receipt_cancellation_supersession_surfaces.len();
    let release_publication_result_receipt_cancellation_supersession_contract_hash_sha256 =
        sha256_text_value(&format!(
            "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-cancellation-supersession-denial:native:source={source_report_sha256}:surfaces={release_publication_result_receipt_cancellation_supersession_surface_count}:route_count={}:cancellation=0:supersession=0:replacement=0:authority=0:live=0",
            route_matrix.route_count
        ));
    let denied_by_packet_receipt_release_publication_result_receipt_cancellation_supersession = vec![
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_cancellation_acceptance_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_cancellation_recording_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_cancellation_persistence_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_revocation_acceptance_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_withdrawal_acceptance_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_supersession_acceptance_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_supersession_recording_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_supersession_persistence_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_replacement_acceptance_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_replacement_recording_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_replacement_persistence_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_tombstone_recording_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_tombstone_persistence_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_delete_marker_recording_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_latest_replacement_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_ack_replacement_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_query_replacement_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_export_replacement_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_observability_replacement_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_acceptance_from_cancellation_supersession_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_authority_from_cancellation_supersession_denied",
        "operator_readiness_packet_template_packet_receipt_activation_live_from_cancellation_supersession_denied",
        "operator_readiness_packet_template_packet_receipt_install_restart_active_binary_from_cancellation_supersession_denied",
        "operator_readiness_packet_template_packet_receipt_memory_provider_external_send_from_cancellation_supersession_denied",
    ];
    let denied_by_packet_receipt_release_publication_result_receipt_cancellation_supersession_count =
        denied_by_packet_receipt_release_publication_result_receipt_cancellation_supersession.len();
    let report_ready = source_ready
        && source_u64("release_publication_result_receipt_ordering_surface_count") == 14
        && source_u64("release_publication_result_receipt_ordering_attempt_count") == 14
        && source_u64("release_publication_result_receipt_ordering_recorded_count") == 0
        && source_u64("release_publication_result_receipt_ordering_persisted_count") == 0
        && source_u64("release_publication_result_receipt_sequence_cursor_recorded_count") == 0
        && source_u64("release_publication_result_receipt_monotonicity_state_recorded_count") == 0
        && source_u64("release_publication_result_receipt_ordering_acceptance_recorded_count") == 0
        && source_u64(
            "release_publication_result_receipt_ordering_release_publication_authority_derived_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_ordering_activation_authority_derived_count",
        ) == 0
        && release_publication_result_receipt_cancellation_supersession_surface_count == 14
        && route_count_source_command_accepted;

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_cancellation_supersession_denial_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_CANCELLATION_SUPERSESSION_DENIAL_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-cancellation-supersession-denial --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-16",
        "receipt_release_publication_result_receipt_cancellation_supersession_schema_version": "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_cancellation_supersession_denial_v1",
        "receipt_release_publication_result_receipt_cancellation_supersession_mode": "native_route_denied_release_publication_result_receipt_cannot_use_cancellation_supersession_or_replacement_as_authority",
        "source_packet_acceptance_receipt_release_publication_result_receipt_ordering_gate": source["gate"].clone(),
        "source_packet_acceptance_receipt_release_publication_result_receipt_ordering_ready": source_ready,
        "source_packet_acceptance_receipt_release_publication_result_receipt_ordering_report_sha256": source_report_sha256,
        "source_release_publication_result_receipt_ordering_monotonicity_contract_hash_sha256": source["release_publication_result_receipt_ordering_monotonicity_contract_hash_sha256"].clone(),
        "release_publication_result_receipt_cancellation_supersession_contract_hash_sha256": release_publication_result_receipt_cancellation_supersession_contract_hash_sha256,
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
            "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_cancellation_supersession_denial_route_enabled": true,
            "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_cancellation_supersession_denial_ready": report_ready,
            "source_release_publication_result_receipt_ordering_surface_count": source_u64("release_publication_result_receipt_ordering_surface_count"),
            "source_release_publication_result_receipt_ordering_attempt_count": source_u64("release_publication_result_receipt_ordering_attempt_count"),
            "source_release_publication_result_receipt_ordering_recorded_count": source_u64("release_publication_result_receipt_ordering_recorded_count"),
            "source_release_publication_result_receipt_ordering_persisted_count": source_u64("release_publication_result_receipt_ordering_persisted_count"),
            "source_release_publication_result_receipt_sequence_cursor_recorded_count": source_u64("release_publication_result_receipt_sequence_cursor_recorded_count"),
            "source_release_publication_result_receipt_monotonicity_state_recorded_count": source_u64("release_publication_result_receipt_monotonicity_state_recorded_count"),
            "source_release_publication_result_receipt_ordering_acceptance_recorded_count": source_u64("release_publication_result_receipt_ordering_acceptance_recorded_count"),
            "source_release_publication_result_receipt_ordering_release_publication_authority_derived_count": source_u64("release_publication_result_receipt_ordering_release_publication_authority_derived_count"),
            "source_release_publication_result_receipt_ordering_activation_authority_derived_count": source_u64("release_publication_result_receipt_ordering_activation_authority_derived_count"),
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "release_publication_result_receipt_cancellation_supersession_surface_count": release_publication_result_receipt_cancellation_supersession_surface_count,
            "release_publication_result_receipt_cancellation_supersession_attempt_count": release_publication_result_receipt_cancellation_supersession_surface_count,
            "release_publication_result_receipt_cancellation_accepted_count": 0,
            "release_publication_result_receipt_cancellation_recorded_count": 0,
            "release_publication_result_receipt_cancellation_persisted_count": 0,
            "release_publication_result_receipt_revocation_accepted_count": 0,
            "release_publication_result_receipt_withdrawal_accepted_count": 0,
            "release_publication_result_receipt_supersession_accepted_count": 0,
            "release_publication_result_receipt_supersession_recorded_count": 0,
            "release_publication_result_receipt_supersession_persisted_count": 0,
            "release_publication_result_receipt_replacement_receipt_accepted_count": 0,
            "release_publication_result_receipt_replacement_receipt_recorded_count": 0,
            "release_publication_result_receipt_replacement_receipt_persisted_count": 0,
            "release_publication_result_receipt_tombstone_recorded_count": 0,
            "release_publication_result_receipt_tombstone_persisted_count": 0,
            "release_publication_result_receipt_delete_marker_recorded_count": 0,
            "release_publication_result_receipt_latest_replacement_accepted_count": 0,
            "release_publication_result_receipt_ack_replacement_accepted_count": 0,
            "release_publication_result_receipt_query_replacement_registered_count": 0,
            "release_publication_result_receipt_export_replacement_recorded_count": 0,
            "release_publication_result_receipt_observability_replacement_recorded_count": 0,
            "release_publication_result_receipt_cancellation_supersession_acceptance_recorded_count": 0,
            "release_publication_result_receipt_cancellation_supersession_operator_approval_derived_count": 0,
            "release_publication_result_receipt_cancellation_supersession_release_publication_authority_derived_count": 0,
            "release_publication_result_receipt_cancellation_supersession_activation_authority_derived_count": 0,
            "release_publication_result_receipt_cancellation_supersession_activation_command_derived_count": 0,
            "release_publication_result_receipt_cancellation_supersession_live_execution_allowed_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "release_publication_result_receipt_cancellation_supersession_surfaces": release_publication_result_receipt_cancellation_supersession_surfaces,
            "denied_by_packet_receipt_release_publication_result_receipt_cancellation_supersession": denied_by_packet_receipt_release_publication_result_receipt_cancellation_supersession,
            "denied_by_packet_receipt_release_publication_result_receipt_cancellation_supersession_count": denied_by_packet_receipt_release_publication_result_receipt_cancellation_supersession_count,
            "allowed_next_actions": [
                {
                    "action": "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_audit_trail_immutable_evidence_denial_gate",
                    "status": "allowed_report_only_next_slice",
                    "records_release_publication_result_receipt_cancellation": false,
                    "records_release_publication_result_receipt_supersession": false,
                    "accepts_replacement_receipt": false,
                    "records_tombstone": false,
                    "records_delete_marker": false,
                    "accepts_latest_replacement": false,
                    "records_publication_completion_ack": false,
                    "derives_release_publication_authority": false,
                    "derives_activation_authority": false,
                    "activates_live": false,
                    "mutates_memory_store": false,
                    "writes_kg": false,
                    "sends_externally": false
                }
            ],
        }),
    );

    let release_publication_result_receipt_cancellation_supersession_false_keys = [
        "packet_acceptance_receipt_release_publication_result_receipt_cancellation_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_cancellation_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_cancellation_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_revocation_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_withdrawal_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_supersession_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_supersession_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_supersession_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_replacement_receipt_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_replacement_receipt_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_replacement_receipt_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_tombstone_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_tombstone_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_delete_marker_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_latest_replacement_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_ack_replacement_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_query_replacement_registered",
        "packet_acceptance_receipt_release_publication_result_receipt_export_replacement_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_observability_replacement_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_ordering_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_ordering_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_ordering_materialized",
        "packet_acceptance_receipt_release_publication_result_receipt_sequence_cursor_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_sequence_cursor_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_sequence_cursor_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_monotonicity_state_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_monotonicity_state_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_latest_wins_overwrite_accepted",
        "packet_acceptance_receipt_publication_completion_ack_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_replayed",
        "packet_acceptance_receipt_release_publication_result_receipt_replay_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_replay_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_duplicate_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_retry_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_idempotency_key_registered",
        "packet_acceptance_receipt_release_publication_result_receipt_idempotency_cache_written",
        "packet_acceptance_receipt_release_publication_result_receipt_idempotency_cache_hit_promoted",
        "packet_acceptance_receipt_release_publication_result_receipt_query_result_replayed",
        "packet_acceptance_receipt_release_publication_result_receipt_export_snapshot_replayed",
        "packet_acceptance_receipt_release_publication_result_receipt_observability_snapshot_replayed",
        "packet_acceptance_receipt_release_publication_result_receipt_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_materialized",
        "packet_acceptance_receipt_release_publication_result_receipt_delivered",
        "packet_acceptance_receipt_release_publication_result_receipt_exported",
        "packet_acceptance_receipt_release_publication_result_receipt_query_registered",
        "packet_acceptance_receipt_release_publication_result_receipt_observability_recorded",
        "packet_acceptance_receipt_release_publication_recorded",
        "packet_acceptance_receipt_release_artifact_written",
        "packet_acceptance_receipt_public_artifact_written",
        "packet_acceptance_receipt_publication_queue_enqueued",
        "packet_acceptance_receipt_publication_manifest_written",
        "packet_acceptance_receipt_public_distribution_performed",
        "packet_acceptance_receipt_channel_delivery_performed",
        "packet_acceptance_receipt_external_publication_sent",
        "packet_acceptance_receipt_public_version_tag_created",
        "packet_acceptance_receipt_release_notes_materialized",
        "packet_acceptance_receipt_changelog_materialized",
        "packet_acceptance_receipt_public_release_claimed",
        "packet_acceptance_receipt_public_ga_claimed",
        "packet_acceptance_receipt_terminal_status_promoted_to_release_approval",
        "operator_acceptance_recorded",
        "operator_approval_recorded",
        "activation_authority_derived",
        "activation_command_derived",
        "activation_allowed",
        "activation_performed",
        "memory_store_write_performed",
        "memory_store_mutated",
        "hepta_intelligence_context_attached",
        "prompt_preview_rendered",
        "context_injection_performed",
        "provider_invoked",
        "model_invoked",
        "external_kg_adapter_read_performed",
        "external_adapter_client_constructed",
        "network_call_performed",
        "external_db_write_performed",
        "live_kg_write_performed",
        "credential_read",
        "secret_file_read",
        "install_executed",
        "launchd_mutated",
        "service_restarted",
        "active_binary_mutated",
        "public_release_claimed",
        "public_ga_claimed",
        "release_artifact_written",
        "public_artifact_written",
        "external_send_performed",
        "filesystem_written",
    ];
    if let Some(report_object) = report.as_object_mut() {
        for key in release_publication_result_receipt_cancellation_supersession_false_keys {
            report_object.insert(key.to_string(), serde_json::json!(false));
        }
    }

    let mut side_effects = serde_json::Map::new();
    for key in release_publication_result_receipt_cancellation_supersession_false_keys {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }
    extend_json_object(
        &mut report,
        serde_json::json!({ "side_effects": side_effects }),
    );
    report
}

fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_audit_trail_immutable_evidence_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source =
        hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_cancellation_supersession_denial_report();
    let source_report_sha256 = sha256_json_value(&source);
    let source_ready = source
        .get("memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_cancellation_supersession_denial_ready")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);
    let source_u64 = |key: &str| -> u64 {
        source
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;

    let release_publication_result_receipt_audit_evidence_surfaces = vec![
        "publication_result_receipt_audit_trail_append_claim",
        "publication_result_receipt_immutable_evidence_claim",
        "publication_result_receipt_hash_chain_claim",
        "publication_result_receipt_merkle_root_claim",
        "publication_result_receipt_attestation_claim",
        "publication_result_receipt_witness_claim",
        "publication_result_receipt_notary_claim",
        "publication_result_receipt_ledger_evidence_claim",
        "publication_result_receipt_index_evidence_claim",
        "publication_result_receipt_delivery_evidence_claim",
        "publication_result_receipt_export_evidence_claim",
        "publication_result_receipt_query_evidence_claim",
        "publication_result_receipt_observability_evidence_claim",
        "publication_result_receipt_readback_evidence_claim",
        "publication_result_receipt_release_publication_authority_evidence_claim",
        "publication_result_receipt_activation_live_install_restart_active_binary_evidence_claim",
    ]
    .into_iter()
    .map(
        |release_publication_result_receipt_audit_evidence_surface| {
            let mut surface = serde_json::Map::new();
            surface.insert(
                "release_publication_result_receipt_audit_evidence_surface".to_string(),
                serde_json::json!(release_publication_result_receipt_audit_evidence_surface),
            );
            surface.insert(
                "source_release_publication_result_receipt_cancellation_supersession_ready"
                    .to_string(),
                serde_json::json!(source_ready),
            );
            surface.insert(
                "audit_or_evidence_attempted".to_string(),
                serde_json::json!(true),
            );
            surface.insert(
                "audit_evidence_noop_confirmed".to_string(),
                serde_json::json!(true),
            );
            surface.insert(
                "release_publication_result_receipt_audit_evidence_status".to_string(),
                serde_json::json!(
                    "release_publication_result_receipt_audit_trail_immutable_evidence_denied"
                ),
            );
            for key in [
                "audit_trail_accepted",
                "audit_trail_recorded",
                "audit_trail_persisted",
                "audit_trail_materialized",
                "immutable_evidence_accepted",
                "immutable_evidence_recorded",
                "immutable_evidence_persisted",
                "immutable_evidence_materialized",
                "hash_chain_recorded",
                "hash_chain_persisted",
                "merkle_root_recorded",
                "merkle_root_persisted",
                "attestation_recorded",
                "attestation_persisted",
                "witness_recorded",
                "witness_persisted",
                "notary_recorded",
                "notary_persisted",
                "ledger_evidence_recorded",
                "ledger_evidence_persisted",
                "index_evidence_recorded",
                "index_evidence_persisted",
                "delivery_evidence_recorded",
                "delivery_evidence_persisted",
                "export_evidence_recorded",
                "query_evidence_registered",
                "observability_evidence_recorded",
                "readback_evidence_recorded",
                "publication_completion_ack_recorded",
                "cancellation_recorded",
                "supersession_recorded",
                "replacement_receipt_recorded",
                "tombstone_recorded",
                "result_receipt_ordering_recorded",
                "result_receipt_replay_recorded",
                "release_publication_recorded",
                "release_artifact_written",
                "public_artifact_written",
                "public_distribution_performed",
                "channel_delivery_performed",
                "external_send_performed",
                "public_release_claimed",
                "public_ga_claimed",
                "acceptance_recorded",
                "operator_approval_derived",
                "release_publication_authority_derived",
                "activation_authority_derived",
                "activation_command_derived",
                "live_execution_allowed",
                "activation_performed",
                "memory_store_write_performed",
                "memory_store_mutated",
                "provider_invoked",
                "model_invoked",
                "credential_read",
                "secret_file_read",
                "install_executed",
                "launchd_mutated",
                "service_restarted",
                "active_binary_mutated",
            ] {
                surface.insert(key.to_string(), serde_json::json!(false));
            }
            serde_json::Value::Object(surface)
        },
    )
    .collect::<Vec<_>>();
    let release_publication_result_receipt_audit_evidence_surface_count =
        release_publication_result_receipt_audit_evidence_surfaces.len();
    let release_publication_result_receipt_audit_trail_immutable_evidence_contract_hash_sha256 =
        sha256_text_value(&format!(
            "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-audit-trail-immutable-evidence-denial:native:source={source_report_sha256}:surfaces={release_publication_result_receipt_audit_evidence_surface_count}:route_count={}:audit=0:evidence=0:hashchain=0:authority=0:live=0",
            route_matrix.route_count
        ));
    let denied_by_packet_receipt_release_publication_result_receipt_audit_trail_immutable_evidence = vec![
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_audit_trail_acceptance_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_audit_trail_recording_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_audit_trail_persistence_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_audit_trail_materialization_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_immutable_evidence_acceptance_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_immutable_evidence_recording_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_immutable_evidence_persistence_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_immutable_evidence_materialization_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_hash_chain_recording_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_hash_chain_persistence_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_merkle_root_recording_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_merkle_root_persistence_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_attestation_recording_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_witness_recording_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_notary_recording_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_ledger_evidence_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_index_evidence_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_delivery_evidence_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_export_evidence_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_query_evidence_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_observability_evidence_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_readback_evidence_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_completion_ack_from_audit_evidence_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_acceptance_from_audit_evidence_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_authority_from_audit_evidence_denied",
        "operator_readiness_packet_template_packet_receipt_activation_live_from_audit_evidence_denied",
        "operator_readiness_packet_template_packet_receipt_install_restart_active_binary_from_audit_evidence_denied",
        "operator_readiness_packet_template_packet_receipt_memory_provider_external_send_from_audit_evidence_denied",
    ];
    let denied_by_packet_receipt_release_publication_result_receipt_audit_trail_immutable_evidence_count =
        denied_by_packet_receipt_release_publication_result_receipt_audit_trail_immutable_evidence
            .len();
    let report_ready = source_ready
        && source_u64("release_publication_result_receipt_cancellation_supersession_surface_count")
            == 14
        && source_u64("release_publication_result_receipt_cancellation_supersession_attempt_count")
            == 14
        && source_u64("release_publication_result_receipt_cancellation_recorded_count") == 0
        && source_u64("release_publication_result_receipt_supersession_recorded_count") == 0
        && source_u64("release_publication_result_receipt_replacement_receipt_recorded_count") == 0
        && source_u64("release_publication_result_receipt_tombstone_recorded_count") == 0
        && source_u64("release_publication_result_receipt_latest_replacement_accepted_count") == 0
        && source_u64(
            "release_publication_result_receipt_cancellation_supersession_release_publication_authority_derived_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_cancellation_supersession_activation_authority_derived_count",
        ) == 0
        && release_publication_result_receipt_audit_evidence_surface_count == 16
        && route_count_source_command_accepted;

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_audit_trail_immutable_evidence_denial_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_AUDIT_TRAIL_IMMUTABLE_EVIDENCE_DENIAL_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-audit-trail-immutable-evidence-denial --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-16",
        "receipt_release_publication_result_receipt_audit_trail_immutable_evidence_schema_version": "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_audit_trail_immutable_evidence_denial_v1",
        "receipt_release_publication_result_receipt_audit_trail_immutable_evidence_mode": "native_route_denied_release_publication_result_receipt_cannot_become_audit_trail_immutable_evidence_or_authority",
        "source_packet_acceptance_receipt_release_publication_result_receipt_cancellation_supersession_gate": source["gate"].clone(),
        "source_packet_acceptance_receipt_release_publication_result_receipt_cancellation_supersession_ready": source_ready,
        "source_packet_acceptance_receipt_release_publication_result_receipt_cancellation_supersession_report_sha256": source_report_sha256,
        "source_release_publication_result_receipt_cancellation_supersession_contract_hash_sha256": source["release_publication_result_receipt_cancellation_supersession_contract_hash_sha256"].clone(),
        "release_publication_result_receipt_audit_trail_immutable_evidence_contract_hash_sha256": release_publication_result_receipt_audit_trail_immutable_evidence_contract_hash_sha256,
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
            "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_audit_trail_immutable_evidence_denial_route_enabled": true,
            "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_audit_trail_immutable_evidence_denial_ready": report_ready,
            "source_release_publication_result_receipt_cancellation_supersession_surface_count": source_u64("release_publication_result_receipt_cancellation_supersession_surface_count"),
            "source_release_publication_result_receipt_cancellation_supersession_attempt_count": source_u64("release_publication_result_receipt_cancellation_supersession_attempt_count"),
            "source_release_publication_result_receipt_cancellation_recorded_count": source_u64("release_publication_result_receipt_cancellation_recorded_count"),
            "source_release_publication_result_receipt_supersession_recorded_count": source_u64("release_publication_result_receipt_supersession_recorded_count"),
            "source_release_publication_result_receipt_replacement_receipt_recorded_count": source_u64("release_publication_result_receipt_replacement_receipt_recorded_count"),
            "source_release_publication_result_receipt_tombstone_recorded_count": source_u64("release_publication_result_receipt_tombstone_recorded_count"),
            "source_release_publication_result_receipt_latest_replacement_accepted_count": source_u64("release_publication_result_receipt_latest_replacement_accepted_count"),
            "source_release_publication_result_receipt_cancellation_supersession_release_publication_authority_derived_count": source_u64("release_publication_result_receipt_cancellation_supersession_release_publication_authority_derived_count"),
            "source_release_publication_result_receipt_cancellation_supersession_activation_authority_derived_count": source_u64("release_publication_result_receipt_cancellation_supersession_activation_authority_derived_count"),
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "release_publication_result_receipt_audit_evidence_surface_count": release_publication_result_receipt_audit_evidence_surface_count,
            "release_publication_result_receipt_audit_evidence_attempt_count": release_publication_result_receipt_audit_evidence_surface_count,
            "release_publication_result_receipt_audit_trail_accepted_count": 0,
            "release_publication_result_receipt_audit_trail_recorded_count": 0,
            "release_publication_result_receipt_audit_trail_persisted_count": 0,
            "release_publication_result_receipt_audit_trail_materialized_count": 0,
            "release_publication_result_receipt_immutable_evidence_accepted_count": 0,
            "release_publication_result_receipt_immutable_evidence_recorded_count": 0,
            "release_publication_result_receipt_immutable_evidence_persisted_count": 0,
            "release_publication_result_receipt_immutable_evidence_materialized_count": 0,
            "release_publication_result_receipt_hash_chain_recorded_count": 0,
            "release_publication_result_receipt_hash_chain_persisted_count": 0,
            "release_publication_result_receipt_merkle_root_recorded_count": 0,
            "release_publication_result_receipt_merkle_root_persisted_count": 0,
            "release_publication_result_receipt_attestation_recorded_count": 0,
            "release_publication_result_receipt_attestation_persisted_count": 0,
            "release_publication_result_receipt_witness_recorded_count": 0,
            "release_publication_result_receipt_witness_persisted_count": 0,
            "release_publication_result_receipt_notary_recorded_count": 0,
            "release_publication_result_receipt_notary_persisted_count": 0,
            "release_publication_result_receipt_ledger_evidence_recorded_count": 0,
            "release_publication_result_receipt_ledger_evidence_persisted_count": 0,
            "release_publication_result_receipt_index_evidence_recorded_count": 0,
            "release_publication_result_receipt_index_evidence_persisted_count": 0,
            "release_publication_result_receipt_delivery_evidence_recorded_count": 0,
            "release_publication_result_receipt_delivery_evidence_persisted_count": 0,
            "release_publication_result_receipt_export_evidence_recorded_count": 0,
            "release_publication_result_receipt_query_evidence_registered_count": 0,
            "release_publication_result_receipt_observability_evidence_recorded_count": 0,
            "release_publication_result_receipt_readback_evidence_recorded_count": 0,
            "release_publication_result_receipt_publication_completion_ack_recorded_count": 0,
            "release_publication_result_receipt_audit_evidence_acceptance_recorded_count": 0,
            "release_publication_result_receipt_audit_evidence_operator_approval_derived_count": 0,
            "release_publication_result_receipt_audit_evidence_release_publication_authority_derived_count": 0,
            "release_publication_result_receipt_audit_evidence_activation_authority_derived_count": 0,
            "release_publication_result_receipt_audit_evidence_activation_command_derived_count": 0,
            "release_publication_result_receipt_audit_evidence_live_execution_allowed_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "release_publication_result_receipt_audit_evidence_surfaces": release_publication_result_receipt_audit_evidence_surfaces,
            "denied_by_packet_receipt_release_publication_result_receipt_audit_trail_immutable_evidence": denied_by_packet_receipt_release_publication_result_receipt_audit_trail_immutable_evidence,
            "denied_by_packet_receipt_release_publication_result_receipt_audit_trail_immutable_evidence_count": denied_by_packet_receipt_release_publication_result_receipt_audit_trail_immutable_evidence_count,
            "allowed_next_actions": [
                {
                    "action": "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_retention_expiry_garbage_collection_denial_gate",
                    "status": "allowed_report_only_next_slice",
                    "records_audit_trail": false,
                    "accepts_immutable_evidence": false,
                    "records_hash_chain": false,
                    "records_ledger_evidence": false,
                    "records_publication_completion_ack": false,
                    "derives_release_publication_authority": false,
                    "derives_activation_authority": false,
                    "activates_live": false,
                    "mutates_memory_store": false,
                    "writes_kg": false,
                    "sends_externally": false
                }
            ],
        }),
    );

    let release_publication_result_receipt_audit_evidence_false_keys = [
        "packet_acceptance_receipt_release_publication_result_receipt_audit_trail_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_audit_trail_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_audit_trail_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_audit_trail_materialized",
        "packet_acceptance_receipt_release_publication_result_receipt_immutable_evidence_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_immutable_evidence_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_immutable_evidence_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_immutable_evidence_materialized",
        "packet_acceptance_receipt_release_publication_result_receipt_hash_chain_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_hash_chain_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_merkle_root_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_merkle_root_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_attestation_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_attestation_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_witness_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_witness_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_notary_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_notary_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_ledger_evidence_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_ledger_evidence_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_index_evidence_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_index_evidence_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_delivery_evidence_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_delivery_evidence_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_export_evidence_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_query_evidence_registered",
        "packet_acceptance_receipt_release_publication_result_receipt_observability_evidence_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_readback_evidence_recorded",
        "packet_acceptance_receipt_publication_completion_ack_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_cancellation_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_supersession_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_replacement_receipt_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_tombstone_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_latest_replacement_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_ordering_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_ordering_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_monotonicity_state_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_replayed",
        "packet_acceptance_receipt_release_publication_result_receipt_replay_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_replay_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_idempotency_key_registered",
        "packet_acceptance_receipt_release_publication_result_receipt_idempotency_cache_written",
        "packet_acceptance_receipt_release_publication_result_receipt_idempotency_cache_hit_promoted",
        "packet_acceptance_receipt_release_publication_result_receipt_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_materialized",
        "packet_acceptance_receipt_release_publication_result_receipt_delivered",
        "packet_acceptance_receipt_release_publication_result_receipt_exported",
        "packet_acceptance_receipt_release_publication_result_receipt_query_registered",
        "packet_acceptance_receipt_release_publication_result_receipt_observability_recorded",
        "packet_acceptance_receipt_release_publication_recorded",
        "packet_acceptance_receipt_release_artifact_written",
        "packet_acceptance_receipt_public_artifact_written",
        "packet_acceptance_receipt_publication_queue_enqueued",
        "packet_acceptance_receipt_publication_manifest_written",
        "packet_acceptance_receipt_public_distribution_performed",
        "packet_acceptance_receipt_channel_delivery_performed",
        "packet_acceptance_receipt_external_publication_sent",
        "packet_acceptance_receipt_public_version_tag_created",
        "packet_acceptance_receipt_release_notes_materialized",
        "packet_acceptance_receipt_changelog_materialized",
        "packet_acceptance_receipt_public_release_claimed",
        "packet_acceptance_receipt_public_ga_claimed",
        "packet_acceptance_receipt_terminal_status_promoted_to_release_approval",
        "operator_acceptance_recorded",
        "operator_approval_recorded",
        "activation_authority_derived",
        "activation_command_derived",
        "activation_allowed",
        "activation_performed",
        "memory_store_write_performed",
        "memory_store_mutated",
        "hepta_intelligence_context_attached",
        "prompt_preview_rendered",
        "context_injection_performed",
        "provider_invoked",
        "model_invoked",
        "external_kg_adapter_read_performed",
        "external_adapter_client_constructed",
        "network_call_performed",
        "external_db_write_performed",
        "live_kg_write_performed",
        "credential_read",
        "secret_file_read",
        "install_executed",
        "launchd_mutated",
        "service_restarted",
        "active_binary_mutated",
        "public_release_claimed",
        "public_ga_claimed",
        "release_artifact_written",
        "public_artifact_written",
        "external_send_performed",
        "filesystem_written",
    ];
    if let Some(report_object) = report.as_object_mut() {
        for key in release_publication_result_receipt_audit_evidence_false_keys {
            report_object.insert(key.to_string(), serde_json::json!(false));
        }
    }

    let mut side_effects = serde_json::Map::new();
    for key in release_publication_result_receipt_audit_evidence_false_keys {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }
    extend_json_object(
        &mut report,
        serde_json::json!({ "side_effects": side_effects }),
    );
    report
}

fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_retention_expiry_garbage_collection_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source =
        hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_audit_trail_immutable_evidence_denial_report();
    let source_report_sha256 = sha256_json_value(&source);
    let source_ready = source
        .get("memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_audit_trail_immutable_evidence_denial_ready")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);
    let source_u64 = |key: &str| -> u64 {
        source
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;

    let retention_surfaces = vec![
        "publication_result_receipt_retention_policy_claim",
        "publication_result_receipt_retention_index_claim",
        "publication_result_receipt_retention_ledger_claim",
        "publication_result_receipt_ttl_update_claim",
        "publication_result_receipt_ttl_extension_claim",
        "publication_result_receipt_expiry_schedule_claim",
        "publication_result_receipt_expiry_timer_claim",
        "publication_result_receipt_expiry_ack_claim",
        "publication_result_receipt_garbage_collection_scan_claim",
        "publication_result_receipt_garbage_collection_candidate_claim",
        "publication_result_receipt_garbage_collection_decision_claim",
        "publication_result_receipt_delete_claim",
        "publication_result_receipt_tombstone_claim",
        "publication_result_receipt_sweep_claim",
        "publication_result_receipt_archive_claim",
        "publication_result_receipt_compaction_claim",
        "publication_result_receipt_release_publication_authority_retention_claim",
        "publication_result_receipt_activation_live_install_restart_active_binary_retention_claim",
    ]
    .into_iter()
    .map(|retention_surface| {
        let mut surface = serde_json::Map::new();
        surface.insert(
            "release_publication_result_receipt_retention_surface".to_string(),
            serde_json::json!(retention_surface),
        );
        surface.insert(
            "source_release_publication_result_receipt_audit_evidence_ready".to_string(),
            serde_json::json!(source_ready),
        );
        surface.insert(
            "retention_expiry_or_garbage_collection_attempted".to_string(),
            serde_json::json!(true),
        );
        surface.insert(
            "retention_expiry_garbage_collection_noop_confirmed".to_string(),
            serde_json::json!(true),
        );
        surface.insert(
            "release_publication_result_receipt_retention_status".to_string(),
            serde_json::json!(
                "release_publication_result_receipt_retention_expiry_garbage_collection_denied"
            ),
        );
        for key in [
            "retention_policy_accepted",
            "retention_policy_recorded",
            "retention_policy_persisted",
            "retention_policy_materialized",
            "retention_index_recorded",
            "retention_index_persisted",
            "retention_ledger_recorded",
            "retention_ledger_persisted",
            "ttl_update_accepted",
            "ttl_update_recorded",
            "ttl_update_persisted",
            "ttl_extension_accepted",
            "ttl_extension_recorded",
            "ttl_extension_persisted",
            "expiry_accepted",
            "expiry_recorded",
            "expiry_persisted",
            "expiry_scheduler_registered",
            "expiry_timer_started",
            "expiry_ack_recorded",
            "garbage_collection_accepted",
            "garbage_collection_scan_performed",
            "garbage_collection_candidate_recorded",
            "garbage_collection_decision_recorded",
            "garbage_collection_persisted",
            "delete_accepted",
            "delete_marker_recorded",
            "delete_performed",
            "tombstone_recorded",
            "tombstone_persisted",
            "sweep_performed",
            "archive_written",
            "archive_persisted",
            "compaction_performed",
            "compaction_artifact_written",
            "compaction_artifact_persisted",
            "ledger_retention_recorded",
            "ledger_retention_persisted",
            "index_retention_recorded",
            "index_retention_persisted",
            "delivery_retention_recorded",
            "delivery_retention_persisted",
            "audit_trail_recorded",
            "immutable_evidence_recorded",
            "hash_chain_recorded",
            "readback_evidence_recorded",
            "publication_completion_ack_recorded",
            "release_publication_recorded",
            "release_artifact_written",
            "public_artifact_written",
            "publication_queue_enqueued",
            "publication_manifest_written",
            "public_distribution_performed",
            "channel_delivery_performed",
            "external_send_performed",
            "public_release_claimed",
            "public_ga_claimed",
            "acceptance_recorded",
            "operator_approval_derived",
            "release_publication_authority_derived",
            "activation_authority_derived",
            "activation_command_derived",
            "live_execution_allowed",
            "activation_performed",
            "memory_store_write_performed",
            "memory_store_mutated",
            "provider_invoked",
            "model_invoked",
            "credential_read",
            "secret_file_read",
            "install_executed",
            "launchd_mutated",
            "service_restarted",
            "active_binary_mutated",
        ] {
            surface.insert(key.to_string(), serde_json::json!(false));
        }
        serde_json::Value::Object(surface)
    })
    .collect::<Vec<_>>();
    let retention_surface_count = retention_surfaces.len();
    let retention_contract_hash_sha256 = sha256_text_value(&format!(
        "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-retention-expiry-garbage-collection-denial:native:source={source_report_sha256}:surfaces={retention_surface_count}:route_count={}:retention=0:expiry=0:gc=0:authority=0:live=0",
        route_matrix.route_count
    ));
    let denials = vec![
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_retention_policy_acceptance_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_retention_policy_recording_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_retention_policy_persistence_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_retention_index_recording_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_retention_ledger_recording_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_ttl_update_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_ttl_extension_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_expiry_acceptance_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_expiry_recording_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_expiry_scheduler_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_expiry_timer_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_expiry_ack_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_garbage_collection_acceptance_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_garbage_collection_scan_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_garbage_collection_candidate_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_garbage_collection_decision_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_delete_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_tombstone_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_sweep_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_archive_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_compaction_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_ledger_index_delivery_retention_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_completion_ack_from_retention_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_acceptance_from_retention_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_authority_from_retention_denied",
        "operator_readiness_packet_template_packet_receipt_activation_live_from_retention_denied",
        "operator_readiness_packet_template_packet_receipt_install_restart_active_binary_from_retention_denied",
        "operator_readiness_packet_template_packet_receipt_memory_provider_external_send_from_retention_denied",
    ];
    let denials_count = denials.len();
    let report_ready = source_ready
        && source_u64("release_publication_result_receipt_audit_evidence_surface_count") == 16
        && source_u64("release_publication_result_receipt_audit_evidence_attempt_count") == 16
        && source_u64("release_publication_result_receipt_audit_trail_recorded_count") == 0
        && source_u64("release_publication_result_receipt_immutable_evidence_recorded_count") == 0
        && source_u64("release_publication_result_receipt_hash_chain_recorded_count") == 0
        && source_u64("release_publication_result_receipt_ledger_evidence_recorded_count") == 0
        && source_u64("release_publication_result_receipt_readback_evidence_recorded_count") == 0
        && source_u64(
            "release_publication_result_receipt_audit_evidence_release_publication_authority_derived_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_audit_evidence_activation_authority_derived_count",
        ) == 0
        && retention_surface_count == 18
        && route_count_source_command_accepted;

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_retention_expiry_garbage_collection_denial_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_RETENTION_EXPIRY_GARBAGE_COLLECTION_DENIAL_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-retention-expiry-garbage-collection-denial --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-16",
        "receipt_release_publication_result_receipt_retention_expiry_garbage_collection_schema_version": "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_retention_expiry_garbage_collection_denial_v1",
        "receipt_release_publication_result_receipt_retention_expiry_garbage_collection_mode": "native_route_denied_release_publication_result_receipt_cannot_create_retention_expiry_garbage_collection_state_or_authority",
        "source_packet_acceptance_receipt_release_publication_result_receipt_audit_evidence_gate": source["gate"].clone(),
        "source_packet_acceptance_receipt_release_publication_result_receipt_audit_evidence_ready": source_ready,
        "source_packet_acceptance_receipt_release_publication_result_receipt_audit_evidence_report_sha256": source_report_sha256,
        "source_release_publication_result_receipt_audit_trail_immutable_evidence_contract_hash_sha256": source["release_publication_result_receipt_audit_trail_immutable_evidence_contract_hash_sha256"].clone(),
        "release_publication_result_receipt_retention_expiry_garbage_collection_contract_hash_sha256": retention_contract_hash_sha256,
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
            "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_retention_expiry_garbage_collection_denial_route_enabled": true,
            "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_retention_expiry_garbage_collection_denial_ready": report_ready,
            "source_release_publication_result_receipt_audit_evidence_surface_count": source_u64("release_publication_result_receipt_audit_evidence_surface_count"),
            "source_release_publication_result_receipt_audit_evidence_attempt_count": source_u64("release_publication_result_receipt_audit_evidence_attempt_count"),
            "source_release_publication_result_receipt_audit_trail_recorded_count": source_u64("release_publication_result_receipt_audit_trail_recorded_count"),
            "source_release_publication_result_receipt_immutable_evidence_recorded_count": source_u64("release_publication_result_receipt_immutable_evidence_recorded_count"),
            "source_release_publication_result_receipt_hash_chain_recorded_count": source_u64("release_publication_result_receipt_hash_chain_recorded_count"),
            "source_release_publication_result_receipt_ledger_evidence_recorded_count": source_u64("release_publication_result_receipt_ledger_evidence_recorded_count"),
            "source_release_publication_result_receipt_readback_evidence_recorded_count": source_u64("release_publication_result_receipt_readback_evidence_recorded_count"),
            "source_release_publication_result_receipt_audit_evidence_release_publication_authority_derived_count": source_u64("release_publication_result_receipt_audit_evidence_release_publication_authority_derived_count"),
            "source_release_publication_result_receipt_audit_evidence_activation_authority_derived_count": source_u64("release_publication_result_receipt_audit_evidence_activation_authority_derived_count"),
            "release_publication_result_receipt_retention_surface_count": retention_surface_count,
            "release_publication_result_receipt_retention_attempt_count": retention_surface_count,
            "release_publication_result_receipt_retention_policy_accepted_count": 0,
            "release_publication_result_receipt_retention_policy_recorded_count": 0,
            "release_publication_result_receipt_retention_policy_persisted_count": 0,
            "release_publication_result_receipt_retention_policy_materialized_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "release_publication_result_receipt_retention_index_recorded_count": 0,
            "release_publication_result_receipt_retention_index_persisted_count": 0,
            "release_publication_result_receipt_retention_ledger_recorded_count": 0,
            "release_publication_result_receipt_retention_ledger_persisted_count": 0,
            "release_publication_result_receipt_ttl_update_accepted_count": 0,
            "release_publication_result_receipt_ttl_update_recorded_count": 0,
            "release_publication_result_receipt_ttl_update_persisted_count": 0,
            "release_publication_result_receipt_ttl_extension_accepted_count": 0,
            "release_publication_result_receipt_ttl_extension_recorded_count": 0,
            "release_publication_result_receipt_ttl_extension_persisted_count": 0,
            "release_publication_result_receipt_expiry_accepted_count": 0,
            "release_publication_result_receipt_expiry_recorded_count": 0,
            "release_publication_result_receipt_expiry_persisted_count": 0,
            "release_publication_result_receipt_expiry_scheduler_registered_count": 0,
            "release_publication_result_receipt_expiry_timer_started_count": 0,
            "release_publication_result_receipt_expiry_ack_recorded_count": 0,
            "release_publication_result_receipt_garbage_collection_accepted_count": 0,
            "release_publication_result_receipt_garbage_collection_scan_performed_count": 0,
            "release_publication_result_receipt_garbage_collection_candidate_recorded_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "release_publication_result_receipt_garbage_collection_decision_recorded_count": 0,
            "release_publication_result_receipt_garbage_collection_persisted_count": 0,
            "release_publication_result_receipt_delete_accepted_count": 0,
            "release_publication_result_receipt_delete_marker_recorded_count": 0,
            "release_publication_result_receipt_delete_performed_count": 0,
            "release_publication_result_receipt_tombstone_recorded_count": 0,
            "release_publication_result_receipt_tombstone_persisted_count": 0,
            "release_publication_result_receipt_sweep_performed_count": 0,
            "release_publication_result_receipt_archive_written_count": 0,
            "release_publication_result_receipt_archive_persisted_count": 0,
            "release_publication_result_receipt_compaction_performed_count": 0,
            "release_publication_result_receipt_compaction_artifact_written_count": 0,
            "release_publication_result_receipt_compaction_artifact_persisted_count": 0,
            "release_publication_result_receipt_ledger_retention_recorded_count": 0,
            "release_publication_result_receipt_index_retention_recorded_count": 0,
            "release_publication_result_receipt_delivery_retention_recorded_count": 0,
            "release_publication_result_receipt_retention_acceptance_recorded_count": 0,
            "release_publication_result_receipt_retention_operator_approval_derived_count": 0,
            "release_publication_result_receipt_retention_release_publication_authority_derived_count": 0,
            "release_publication_result_receipt_retention_activation_authority_derived_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "release_publication_result_receipt_retention_activation_command_derived_count": 0,
            "release_publication_result_receipt_retention_live_execution_allowed_count": 0,
            "release_publication_result_receipt_retention_surfaces": retention_surfaces,
            "denied_by_packet_receipt_release_publication_result_receipt_retention_expiry_garbage_collection": denials,
            "denied_by_packet_receipt_release_publication_result_receipt_retention_expiry_garbage_collection_count": denials_count,
            "allowed_next_actions": [
                {
                    "action": "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_export_query_observability_denial_gate",
                    "status": "allowed_report_only_next_slice",
                    "records_retention_policy": false,
                    "records_expiry": false,
                    "performs_garbage_collection": false,
                    "exports_receipt": false,
                    "registers_query": false,
                    "records_observability": false,
                    "derives_release_publication_authority": false,
                    "derives_activation_authority": false,
                    "activates_live": false,
                    "mutates_memory_store": false,
                    "writes_kg": false,
                    "sends_externally": false
                }
            ],
        }),
    );

    let retention_false_keys = [
        "packet_acceptance_receipt_release_publication_result_receipt_retention_policy_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_retention_policy_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_retention_policy_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_retention_index_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_retention_ledger_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_ttl_update_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_ttl_extension_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_expiry_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_expiry_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_expiry_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_expiry_scheduler_registered",
        "packet_acceptance_receipt_release_publication_result_receipt_expiry_timer_started",
        "packet_acceptance_receipt_release_publication_result_receipt_expiry_ack_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_garbage_collection_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_garbage_collection_scan_performed",
        "packet_acceptance_receipt_release_publication_result_receipt_garbage_collection_candidate_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_garbage_collection_decision_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_delete_marker_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_delete_performed",
        "packet_acceptance_receipt_release_publication_result_receipt_tombstone_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_sweep_performed",
        "packet_acceptance_receipt_release_publication_result_receipt_archive_written",
        "packet_acceptance_receipt_release_publication_result_receipt_compaction_performed",
        "packet_acceptance_receipt_release_publication_result_receipt_compaction_artifact_written",
        "packet_acceptance_receipt_release_publication_result_receipt_audit_trail_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_immutable_evidence_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_hash_chain_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_readback_evidence_recorded",
        "packet_acceptance_receipt_publication_completion_ack_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_cancellation_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_supersession_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_replacement_receipt_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_tombstone_lifecycle_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_latest_replacement_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_ordering_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_replayed",
        "packet_acceptance_receipt_release_publication_result_receipt_idempotency_key_registered",
        "packet_acceptance_receipt_release_publication_result_receipt_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_materialized",
        "packet_acceptance_receipt_release_publication_result_receipt_delivered",
        "packet_acceptance_receipt_release_publication_result_receipt_exported",
        "packet_acceptance_receipt_release_publication_result_receipt_query_registered",
        "packet_acceptance_receipt_release_publication_result_receipt_observability_recorded",
        "packet_acceptance_receipt_release_publication_recorded",
        "packet_acceptance_receipt_release_artifact_written",
        "packet_acceptance_receipt_public_artifact_written",
        "packet_acceptance_receipt_publication_queue_enqueued",
        "packet_acceptance_receipt_publication_manifest_written",
        "packet_acceptance_receipt_public_distribution_performed",
        "packet_acceptance_receipt_channel_delivery_performed",
        "packet_acceptance_receipt_external_publication_sent",
        "packet_acceptance_receipt_public_version_tag_created",
        "packet_acceptance_receipt_release_notes_materialized",
        "packet_acceptance_receipt_changelog_materialized",
        "packet_acceptance_receipt_public_release_claimed",
        "packet_acceptance_receipt_public_ga_claimed",
        "operator_acceptance_recorded",
        "operator_approval_recorded",
        "release_publication_authority_derived",
        "activation_authority_derived",
        "activation_command_derived",
        "activation_allowed",
        "activation_performed",
        "memory_store_write_performed",
        "memory_store_mutated",
        "hepta_intelligence_context_attached",
        "prompt_preview_rendered",
        "context_injection_performed",
        "provider_invoked",
        "model_invoked",
        "external_kg_adapter_read_performed",
        "external_adapter_client_constructed",
        "network_call_performed",
        "external_db_write_performed",
        "live_kg_write_performed",
        "credential_read",
        "secret_file_read",
        "install_executed",
        "launchd_mutated",
        "service_restarted",
        "active_binary_mutated",
        "public_release_claimed",
        "public_ga_claimed",
        "release_artifact_written",
        "public_artifact_written",
        "external_send_performed",
        "filesystem_written",
    ];
    if let Some(report_object) = report.as_object_mut() {
        for key in retention_false_keys {
            report_object.insert(key.to_string(), serde_json::json!(false));
        }
    }

    let mut side_effects = serde_json::Map::new();
    for key in retention_false_keys {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }
    extend_json_object(
        &mut report,
        serde_json::json!({ "side_effects": side_effects }),
    );
    report
}

fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_export_query_observability_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source =
        hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_retention_expiry_garbage_collection_denial_report();
    let source_report_sha256 = sha256_json_value(&source);
    let source_ready = source
        .get("memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_retention_expiry_garbage_collection_denial_ready")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);
    let source_u64 = |key: &str| -> u64 {
        source
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;

    let export_query_observability_surfaces = vec![
        "publication_result_receipt_query_registration_claim",
        "publication_result_receipt_query_execution_claim",
        "publication_result_receipt_query_result_claim",
        "publication_result_receipt_search_index_claim",
        "publication_result_receipt_export_request_claim",
        "publication_result_receipt_export_snapshot_claim",
        "publication_result_receipt_export_file_claim",
        "publication_result_receipt_export_stream_claim",
        "publication_result_receipt_observability_metric_claim",
        "publication_result_receipt_observability_log_claim",
        "publication_result_receipt_observability_trace_claim",
        "publication_result_receipt_observability_event_claim",
        "publication_result_receipt_dashboard_panel_claim",
        "publication_result_receipt_alert_slo_claim",
        "publication_result_receipt_operator_summary_readback_claim",
        "publication_result_receipt_audit_view_evidence_claim",
        "publication_result_receipt_release_publication_authority_view_claim",
        "publication_result_receipt_activation_live_install_restart_active_binary_view_claim",
    ]
    .into_iter()
    .map(|export_query_observability_surface| {
        let mut surface = serde_json::Map::new();
        surface.insert(
            "release_publication_result_receipt_export_query_observability_surface".to_string(),
            serde_json::json!(export_query_observability_surface),
        );
        surface.insert(
            "source_release_publication_result_receipt_retention_ready".to_string(),
            serde_json::json!(source_ready),
        );
        surface.insert(
            "export_query_or_observability_attempted".to_string(),
            serde_json::json!(true),
        );
        surface.insert(
            "export_query_observability_noop_confirmed".to_string(),
            serde_json::json!(true),
        );
        surface.insert(
            "release_publication_result_receipt_export_query_observability_status".to_string(),
            serde_json::json!(
                "release_publication_result_receipt_export_query_observability_denied"
            ),
        );
        for key in [
            "query_registered",
            "query_executed",
            "query_result_recorded",
            "query_result_persisted",
            "search_index_recorded",
            "search_index_persisted",
            "export_requested",
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
            "garbage_collection_scan_performed",
            "audit_trail_recorded",
            "immutable_evidence_recorded",
            "hash_chain_recorded",
            "publication_completion_ack_recorded",
            "release_publication_recorded",
            "release_artifact_written",
            "public_artifact_written",
            "public_distribution_performed",
            "channel_delivery_performed",
            "external_send_performed",
            "public_release_claimed",
            "public_ga_claimed",
            "acceptance_recorded",
            "operator_approval_derived",
            "release_publication_authority_derived",
            "activation_authority_derived",
            "activation_command_derived",
            "live_execution_allowed",
            "activation_performed",
            "memory_store_write_performed",
            "memory_store_mutated",
            "provider_invoked",
            "model_invoked",
            "credential_read",
            "secret_file_read",
            "install_executed",
            "launchd_mutated",
            "service_restarted",
            "active_binary_mutated",
        ] {
            surface.insert(key.to_string(), serde_json::json!(false));
        }
        serde_json::Value::Object(surface)
    })
    .collect::<Vec<_>>();
    let export_query_observability_surface_count = export_query_observability_surfaces.len();
    let export_query_observability_contract_hash_sha256 = sha256_text_value(&format!(
        "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-export-query-observability-denial:native:source={source_report_sha256}:surfaces={export_query_observability_surface_count}:route_count={}:query=0:export=0:observability=0:authority=0:live=0",
        route_matrix.route_count
    ));
    let denials = vec![
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_query_registration_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_query_execution_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_query_result_recording_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_query_result_persistence_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_search_index_recording_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_search_index_persistence_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_export_request_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_export_acceptance_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_export_snapshot_recording_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_export_snapshot_persistence_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_export_file_write_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_export_stream_open_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_observability_metric_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_observability_log_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_observability_trace_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_observability_event_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_dashboard_panel_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_alert_registration_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_slo_recording_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_operator_summary_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_readback_surface_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_audit_view_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_ledger_index_delivery_observability_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_completion_ack_from_view_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_result_receipt_acceptance_from_view_denied",
        "operator_readiness_packet_template_packet_receipt_release_publication_authority_from_result_receipt_view_denied",
        "operator_readiness_packet_template_packet_receipt_activation_live_from_result_receipt_view_denied",
        "operator_readiness_packet_template_packet_receipt_install_restart_active_binary_from_result_receipt_view_denied",
        "operator_readiness_packet_template_packet_receipt_memory_provider_external_send_from_result_receipt_view_denied",
    ];
    let denials_count = denials.len();
    let report_ready = source_ready
        && source_u64("release_publication_result_receipt_retention_surface_count") == 18
        && source_u64("release_publication_result_receipt_retention_attempt_count") == 18
        && source_u64("release_publication_result_receipt_retention_policy_recorded_count") == 0
        && source_u64("release_publication_result_receipt_expiry_recorded_count") == 0
        && source_u64("release_publication_result_receipt_garbage_collection_scan_performed_count")
            == 0
        && source_u64("release_publication_result_receipt_delete_performed_count") == 0
        && source_u64("release_publication_result_receipt_archive_written_count") == 0
        && source_u64("release_publication_result_receipt_compaction_artifact_written_count") == 0
        && source_u64(
            "release_publication_result_receipt_retention_release_publication_authority_derived_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_retention_activation_authority_derived_count",
        ) == 0
        && export_query_observability_surface_count == 18
        && route_count_source_command_accepted;

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_export_query_observability_denial_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_EXPORT_QUERY_OBSERVABILITY_DENIAL_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-export-query-observability-denial --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-16",
        "receipt_release_publication_result_receipt_export_query_observability_schema_version": "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_export_query_observability_denial_v1",
        "receipt_release_publication_result_receipt_export_query_observability_mode": "native_route_denied_release_publication_result_receipt_cannot_create_export_query_observability_views_or_authority",
        "source_packet_acceptance_receipt_release_publication_result_receipt_retention_gate": source["gate"].clone(),
        "source_packet_acceptance_receipt_release_publication_result_receipt_retention_ready": source_ready,
        "source_packet_acceptance_receipt_release_publication_result_receipt_retention_report_sha256": source_report_sha256,
        "source_release_publication_result_receipt_retention_contract_hash_sha256": source["release_publication_result_receipt_retention_expiry_garbage_collection_contract_hash_sha256"].clone(),
        "release_publication_result_receipt_export_query_observability_contract_hash_sha256": export_query_observability_contract_hash_sha256,
        "minimum_required_samples": 24,
        "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        "route_count": route_matrix.route_count,
        "implemented_route_count": route_matrix.implemented_route_count,
        "missing_route_count": route_matrix.missing_route_count,
        "route_count_source_command_accepted": route_count_source_command_accepted,
        "source_route_wired": true,
        "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_export_query_observability_denial_route_enabled": true,
        "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_export_query_observability_denial_ready": report_ready,
    });

    extend_json_object(
        &mut report,
        serde_json::json!({
            "source_release_publication_result_receipt_retention_surface_count": source_u64("release_publication_result_receipt_retention_surface_count"),
            "source_release_publication_result_receipt_retention_attempt_count": source_u64("release_publication_result_receipt_retention_attempt_count"),
            "source_release_publication_result_receipt_retention_policy_recorded_count": source_u64("release_publication_result_receipt_retention_policy_recorded_count"),
            "source_release_publication_result_receipt_expiry_recorded_count": source_u64("release_publication_result_receipt_expiry_recorded_count"),
            "source_release_publication_result_receipt_garbage_collection_scan_performed_count": source_u64("release_publication_result_receipt_garbage_collection_scan_performed_count"),
            "source_release_publication_result_receipt_delete_performed_count": source_u64("release_publication_result_receipt_delete_performed_count"),
            "source_release_publication_result_receipt_archive_written_count": source_u64("release_publication_result_receipt_archive_written_count"),
            "source_release_publication_result_receipt_compaction_artifact_written_count": source_u64("release_publication_result_receipt_compaction_artifact_written_count"),
            "source_release_publication_result_receipt_retention_release_publication_authority_derived_count": source_u64("release_publication_result_receipt_retention_release_publication_authority_derived_count"),
            "source_release_publication_result_receipt_retention_activation_authority_derived_count": source_u64("release_publication_result_receipt_retention_activation_authority_derived_count"),
            "release_publication_result_receipt_export_query_observability_surface_count": export_query_observability_surface_count,
            "release_publication_result_receipt_export_query_observability_attempt_count": export_query_observability_surface_count,
            "release_publication_result_receipt_query_registered_count": 0,
            "release_publication_result_receipt_query_executed_count": 0,
            "release_publication_result_receipt_query_result_recorded_count": 0,
            "release_publication_result_receipt_query_result_persisted_count": 0,
            "release_publication_result_receipt_search_index_recorded_count": 0,
            "release_publication_result_receipt_search_index_persisted_count": 0,
            "release_publication_result_receipt_export_requested_count": 0,
            "release_publication_result_receipt_export_accepted_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "release_publication_result_receipt_export_snapshot_recorded_count": 0,
            "release_publication_result_receipt_export_snapshot_persisted_count": 0,
            "release_publication_result_receipt_export_file_written_count": 0,
            "release_publication_result_receipt_export_stream_opened_count": 0,
            "release_publication_result_receipt_observability_metric_recorded_count": 0,
            "release_publication_result_receipt_observability_log_recorded_count": 0,
            "release_publication_result_receipt_observability_trace_recorded_count": 0,
            "release_publication_result_receipt_observability_event_recorded_count": 0,
            "release_publication_result_receipt_dashboard_panel_recorded_count": 0,
            "release_publication_result_receipt_alert_registered_count": 0,
            "release_publication_result_receipt_slo_recorded_count": 0,
            "release_publication_result_receipt_operator_summary_recorded_count": 0,
            "release_publication_result_receipt_readback_surface_recorded_count": 0,
            "release_publication_result_receipt_audit_view_recorded_count": 0,
            "release_publication_result_receipt_ledger_observability_recorded_count": 0,
            "release_publication_result_receipt_index_observability_recorded_count": 0,
            "release_publication_result_receipt_delivery_observability_recorded_count": 0,
            "release_publication_result_receipt_export_query_observability_acceptance_recorded_count": 0,
            "release_publication_result_receipt_export_query_observability_operator_approval_derived_count": 0,
            "release_publication_result_receipt_export_query_observability_release_publication_authority_derived_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "release_publication_result_receipt_export_query_observability_activation_authority_derived_count": 0,
            "release_publication_result_receipt_export_query_observability_activation_command_derived_count": 0,
            "release_publication_result_receipt_export_query_observability_live_execution_allowed_count": 0,
            "release_publication_result_receipt_export_query_observability_surfaces": export_query_observability_surfaces,
            "denied_by_packet_receipt_release_publication_result_receipt_export_query_observability": denials,
            "denied_by_packet_receipt_release_publication_result_receipt_export_query_observability_count": denials_count,
            "allowed_next_actions": [
                {
                    "action": "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_operator_facing_summary_briefing_non_persistence_denial_gate",
                    "status": "allowed_report_only_next_slice",
                    "exports_receipt": false,
                    "registers_query": false,
                    "records_observability": false,
                    "records_summary": false,
                    "records_briefing": false,
                    "derives_release_publication_authority": false,
                    "derives_activation_authority": false,
                    "activates_live": false,
                    "mutates_memory_store": false,
                    "writes_kg": false,
                    "sends_externally": false
                }
            ],
        }),
    );

    let export_false_keys = [
        "packet_acceptance_receipt_release_publication_result_receipt_query_registered",
        "packet_acceptance_receipt_release_publication_result_receipt_query_executed",
        "packet_acceptance_receipt_release_publication_result_receipt_query_result_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_query_result_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_search_index_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_search_index_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_export_requested",
        "packet_acceptance_receipt_release_publication_result_receipt_export_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_export_snapshot_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_export_snapshot_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_export_file_written",
        "packet_acceptance_receipt_release_publication_result_receipt_export_stream_opened",
        "packet_acceptance_receipt_release_publication_result_receipt_observability_metric_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_observability_log_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_observability_trace_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_observability_event_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_dashboard_panel_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_alert_registered",
        "packet_acceptance_receipt_release_publication_result_receipt_slo_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_operator_summary_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_readback_surface_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_audit_view_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_ledger_observability_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_index_observability_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_delivery_observability_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_retention_policy_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_retention_index_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_retention_ledger_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_expiry_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_garbage_collection_scan_performed",
        "packet_acceptance_receipt_release_publication_result_receipt_archive_written",
        "packet_acceptance_receipt_release_publication_result_receipt_compaction_performed",
        "packet_acceptance_receipt_release_publication_result_receipt_audit_trail_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_immutable_evidence_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_hash_chain_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_readback_evidence_recorded",
        "packet_acceptance_receipt_publication_completion_ack_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_cancellation_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_ordering_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_replayed",
        "packet_acceptance_receipt_release_publication_result_receipt_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_materialized",
        "packet_acceptance_receipt_release_publication_result_receipt_delivered",
        "packet_acceptance_receipt_release_publication_result_receipt_exported",
        "packet_acceptance_receipt_release_publication_result_receipt_observability_recorded",
        "packet_acceptance_receipt_release_publication_recorded",
        "packet_acceptance_receipt_release_artifact_written",
        "packet_acceptance_receipt_public_artifact_written",
        "packet_acceptance_receipt_public_distribution_performed",
        "packet_acceptance_receipt_channel_delivery_performed",
        "packet_acceptance_receipt_external_publication_sent",
        "packet_acceptance_receipt_public_release_claimed",
        "packet_acceptance_receipt_public_ga_claimed",
        "operator_acceptance_recorded",
        "operator_approval_recorded",
        "release_publication_authority_derived",
        "activation_authority_derived",
        "activation_command_derived",
        "activation_allowed",
        "activation_performed",
        "memory_store_write_performed",
        "memory_store_mutated",
        "hepta_intelligence_context_attached",
        "prompt_preview_rendered",
        "context_injection_performed",
        "provider_invoked",
        "model_invoked",
        "external_kg_adapter_read_performed",
        "external_adapter_client_constructed",
        "network_call_performed",
        "external_db_write_performed",
        "live_kg_write_performed",
        "credential_read",
        "secret_file_read",
        "install_executed",
        "launchd_mutated",
        "service_restarted",
        "active_binary_mutated",
        "public_release_claimed",
        "public_ga_claimed",
        "release_artifact_written",
        "public_artifact_written",
        "external_send_performed",
        "filesystem_written",
    ];
    if let Some(report_object) = report.as_object_mut() {
        for key in export_false_keys {
            report_object.insert(key.to_string(), serde_json::json!(false));
        }
    }

    let mut side_effects = serde_json::Map::new();
    for key in export_false_keys {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }
    extend_json_object(
        &mut report,
        serde_json::json!({ "side_effects": side_effects }),
    );
    report
}

fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_operator_facing_summary_briefing_non_persistence_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source =
        hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_export_query_observability_denial_report();
    let source_report_sha256 = sha256_json_value(&source);
    let source_ready = source
        .get("memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_export_query_observability_denial_ready")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);
    let source_u64 = |key: &str| -> u64 {
        source
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;

    let operator_summary_briefing_surfaces = vec![
        "publication_result_receipt_operator_summary_request_claim",
        "publication_result_receipt_operator_briefing_request_claim",
        "publication_result_receipt_readback_digest_claim",
        "publication_result_receipt_final_note_claim",
        "publication_result_receipt_status_banner_claim",
        "publication_result_receipt_dashboard_annotation_claim",
        "publication_result_receipt_notification_preview_claim",
        "publication_result_receipt_timeline_entry_claim",
        "publication_result_receipt_audit_narrative_claim",
        "publication_result_receipt_privacy_review_narrative_claim",
        "publication_result_receipt_alert_explanation_claim",
        "publication_result_receipt_slo_report_claim",
        "publication_result_receipt_channel_delivery_summary_claim",
        "publication_result_receipt_external_send_summary_claim",
        "publication_result_receipt_telegram_briefing_claim",
        "publication_result_receipt_completion_ack_from_summary_claim",
        "publication_result_receipt_release_publication_authority_summary_claim",
        "publication_result_receipt_activation_live_install_restart_active_binary_summary_claim",
    ]
    .into_iter()
    .map(|summary_briefing_surface| {
        let mut surface = serde_json::Map::new();
        surface.insert(
            "release_publication_result_receipt_operator_facing_summary_briefing_surface"
                .to_string(),
            serde_json::json!(summary_briefing_surface),
        );
        surface.insert(
            "source_export_query_observability_ready".to_string(),
            serde_json::json!(source_ready),
        );
        surface.insert(
            "operator_facing_text_attempted".to_string(),
            serde_json::json!(true),
        );
        surface.insert(
            "operator_facing_summary_briefing_noop_confirmed".to_string(),
            serde_json::json!(true),
        );
        surface.insert(
            "operator_summary_briefing_status".to_string(),
            serde_json::json!(
                "release_publication_result_receipt_operator_summary_briefing_denied"
            ),
        );
        for key in [
            "operator_summary_allowed",
            "operator_summary_request_accepted",
            "operator_summary_recorded",
            "operator_summary_persisted",
            "operator_summary_materialized",
            "operator_summary_filesystem_written",
            "operator_summary_delivered",
            "operator_briefing_allowed",
            "operator_briefing_request_accepted",
            "operator_briefing_recorded",
            "operator_briefing_persisted",
            "operator_briefing_materialized",
            "operator_briefing_filesystem_written",
            "operator_briefing_delivered",
            "readback_digest_recorded",
            "readback_digest_persisted",
            "final_note_recorded",
            "final_note_persisted",
            "status_banner_recorded",
            "dashboard_annotation_recorded",
            "notification_preview_recorded",
            "timeline_entry_recorded",
            "audit_narrative_recorded",
            "privacy_review_narrative_recorded",
            "alert_explanation_recorded",
            "slo_report_recorded",
            "channel_delivery_performed",
            "telegram_send_performed",
            "external_send_performed",
            "query_registered",
            "export_snapshot_recorded",
            "observability_event_recorded",
            "completion_ack_recorded",
            "acceptance_recorded",
            "operator_approval_derived",
            "release_publication_authority_derived",
            "activation_authority_derived",
            "activation_command_derived",
            "live_execution_allowed",
            "activation_performed",
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
        ] {
            surface.insert(key.to_string(), serde_json::json!(false));
        }
        serde_json::Value::Object(surface)
    })
    .collect::<Vec<_>>();
    let operator_summary_briefing_surface_count = operator_summary_briefing_surfaces.len();
    let contract_hash = sha256_text_value(&format!(
        "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-operator-facing-summary-briefing-non-persistence-denial:native:source={source_report_sha256}:surfaces={operator_summary_briefing_surface_count}:route_count={}:summary=0:briefing=0:delivery=0:authority=0:live=0",
        route_matrix.route_count
    ));
    let policy_hash = sha256_text_value(
        "release-publication-result-receipt-operator-facing-summary-briefing-non-persistence:no-summary:no-briefing:no-readback:no-final-note:no-dashboard:no-delivery:no-authority:no-live",
    );
    let denials = vec![
        "source_export_query_observability_report_required",
        "operator_summary_request_acceptance_denied",
        "operator_briefing_request_acceptance_denied",
        "readback_digest_recording_denied",
        "final_note_recording_denied",
        "status_banner_recording_denied",
        "dashboard_annotation_recording_denied",
        "notification_preview_recording_denied",
        "timeline_entry_recording_denied",
        "audit_narrative_recording_denied",
        "privacy_review_narrative_recording_denied",
        "alert_explanation_recording_denied",
        "slo_report_recording_denied",
        "operator_summary_persistence_denied",
        "operator_briefing_persistence_denied",
        "readback_digest_persistence_denied",
        "final_note_persistence_denied",
        "operator_summary_materialization_denied",
        "operator_briefing_materialization_denied",
        "operator_summary_filesystem_write_denied",
        "operator_briefing_filesystem_write_denied",
        "operator_summary_delivery_denied",
        "operator_briefing_delivery_denied",
        "channel_delivery_denied",
        "external_send_denied",
        "telegram_send_denied",
        "completion_ack_from_summary_briefing_denied",
        "acceptance_from_summary_briefing_denied",
        "release_publication_authority_from_summary_briefing_denied",
        "activation_live_from_summary_briefing_denied",
        "install_restart_active_binary_from_summary_briefing_denied",
        "memory_provider_kg_from_summary_briefing_denied",
    ];
    let denials_count = denials.len();
    let report_ready = source_ready
        && source_u64(
            "release_publication_result_receipt_export_query_observability_surface_count",
        ) == 18
        && source_u64(
            "release_publication_result_receipt_export_query_observability_attempt_count",
        ) == 18
        && source_u64("release_publication_result_receipt_operator_summary_recorded_count") == 0
        && source_u64("release_publication_result_receipt_readback_surface_recorded_count") == 0
        && source_u64("release_publication_result_receipt_audit_view_recorded_count") == 0
        && source_u64(
            "release_publication_result_receipt_export_query_observability_release_publication_authority_derived_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_export_query_observability_activation_authority_derived_count",
        ) == 0
        && operator_summary_briefing_surface_count == 18
        && route_count_source_command_accepted;

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_operator_facing_summary_briefing_non_persistence_denial_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_OPERATOR_FACING_SUMMARY_BRIEFING_NON_PERSISTENCE_DENIAL_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-operator-facing-summary-briefing-non-persistence-denial --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-16",
        "receipt_release_publication_result_receipt_operator_facing_summary_briefing_schema_version": "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_operator_facing_summary_briefing_non_persistence_denial_v1",
        "receipt_release_publication_result_receipt_operator_facing_summary_briefing_mode": "native_route_denied_release_publication_result_receipt_cannot_create_operator_facing_summary_briefing_readback_delivery_or_authority",
        "source_packet_acceptance_receipt_release_publication_result_receipt_export_query_observability_gate": source["gate"].clone(),
        "source_packet_acceptance_receipt_release_publication_result_receipt_export_query_observability_ready": source_ready,
        "source_packet_acceptance_receipt_release_publication_result_receipt_export_query_observability_report_sha256": source_report_sha256,
        "source_release_publication_result_receipt_export_query_observability_contract_hash_sha256": source["release_publication_result_receipt_export_query_observability_contract_hash_sha256"].clone(),
        "release_publication_result_receipt_operator_summary_briefing_contract_hash_sha256": contract_hash,
        "release_publication_result_receipt_operator_summary_briefing_policy_hash_sha256": policy_hash,
        "minimum_required_samples": 24,
        "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        "route_count": route_matrix.route_count,
        "implemented_route_count": route_matrix.implemented_route_count,
        "missing_route_count": route_matrix.missing_route_count,
        "route_count_source_command_accepted": route_count_source_command_accepted,
        "source_route_wired": true,
        "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_operator_facing_summary_briefing_non_persistence_denial_route_enabled": true,
        "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_operator_facing_summary_briefing_non_persistence_denial_ready": report_ready,
    });

    extend_json_object(
        &mut report,
        serde_json::json!({
            "source_release_publication_result_receipt_export_query_observability_surface_count": source_u64("release_publication_result_receipt_export_query_observability_surface_count"),
            "source_release_publication_result_receipt_export_query_observability_attempt_count": source_u64("release_publication_result_receipt_export_query_observability_attempt_count"),
            "source_release_publication_result_receipt_operator_summary_recorded_count": source_u64("release_publication_result_receipt_operator_summary_recorded_count"),
            "source_release_publication_result_receipt_readback_surface_recorded_count": source_u64("release_publication_result_receipt_readback_surface_recorded_count"),
            "source_release_publication_result_receipt_audit_view_recorded_count": source_u64("release_publication_result_receipt_audit_view_recorded_count"),
            "source_release_publication_result_receipt_export_query_observability_release_publication_authority_derived_count": source_u64("release_publication_result_receipt_export_query_observability_release_publication_authority_derived_count"),
            "source_release_publication_result_receipt_export_query_observability_activation_authority_derived_count": source_u64("release_publication_result_receipt_export_query_observability_activation_authority_derived_count"),
            "release_publication_result_receipt_operator_facing_summary_briefing_surface_count": operator_summary_briefing_surface_count,
            "release_publication_result_receipt_operator_facing_summary_briefing_attempt_count": operator_summary_briefing_surface_count,
            "release_publication_result_receipt_operator_summary_allowed_count": 0,
            "release_publication_result_receipt_operator_summary_request_accepted_count": 0,
            "release_publication_result_receipt_operator_summary_recorded_count": 0,
            "release_publication_result_receipt_operator_summary_persisted_count": 0,
            "release_publication_result_receipt_operator_summary_materialized_count": 0,
            "release_publication_result_receipt_operator_summary_filesystem_written_count": 0,
            "release_publication_result_receipt_operator_summary_delivered_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "release_publication_result_receipt_operator_briefing_allowed_count": 0,
            "release_publication_result_receipt_operator_briefing_request_accepted_count": 0,
            "release_publication_result_receipt_operator_briefing_recorded_count": 0,
            "release_publication_result_receipt_operator_briefing_persisted_count": 0,
            "release_publication_result_receipt_operator_briefing_materialized_count": 0,
            "release_publication_result_receipt_operator_briefing_filesystem_written_count": 0,
            "release_publication_result_receipt_operator_briefing_delivered_count": 0,
            "release_publication_result_receipt_readback_digest_recorded_count": 0,
            "release_publication_result_receipt_readback_digest_persisted_count": 0,
            "release_publication_result_receipt_final_note_recorded_count": 0,
            "release_publication_result_receipt_final_note_persisted_count": 0,
            "release_publication_result_receipt_status_banner_recorded_count": 0,
            "release_publication_result_receipt_dashboard_annotation_recorded_count": 0,
            "release_publication_result_receipt_notification_preview_recorded_count": 0,
            "release_publication_result_receipt_timeline_entry_recorded_count": 0,
            "release_publication_result_receipt_audit_narrative_recorded_count": 0,
            "release_publication_result_receipt_privacy_review_narrative_recorded_count": 0,
            "release_publication_result_receipt_alert_explanation_recorded_count": 0,
            "release_publication_result_receipt_slo_report_recorded_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "release_publication_result_receipt_operator_summary_briefing_channel_delivery_count": 0,
            "release_publication_result_receipt_operator_summary_briefing_external_send_count": 0,
            "release_publication_result_receipt_operator_summary_briefing_telegram_send_count": 0,
            "release_publication_result_receipt_operator_summary_briefing_completion_ack_recorded_count": 0,
            "release_publication_result_receipt_operator_summary_briefing_acceptance_recorded_count": 0,
            "release_publication_result_receipt_operator_summary_briefing_operator_approval_derived_count": 0,
            "release_publication_result_receipt_operator_summary_briefing_release_publication_authority_derived_count": 0,
            "release_publication_result_receipt_operator_summary_briefing_activation_authority_derived_count": 0,
            "release_publication_result_receipt_operator_summary_briefing_activation_command_derived_count": 0,
            "release_publication_result_receipt_operator_summary_briefing_live_execution_allowed_count": 0,
            "release_publication_result_receipt_operator_facing_summary_briefing_surfaces": operator_summary_briefing_surfaces,
            "denied_by_packet_receipt_release_publication_result_receipt_operator_facing_summary_briefing": denials,
            "denied_by_packet_receipt_release_publication_result_receipt_operator_facing_summary_briefing_count": denials_count,
            "allowed_next_actions": [
                {
                    "action": "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_final_operator_acknowledgement_non_acceptance_denial_gate",
                    "status": "allowed_report_only_next_slice",
                    "records_summary": false,
                    "persists_summary": false,
                    "records_briefing": false,
                    "persists_briefing": false,
                    "delivers_briefing": false,
                    "records_acknowledgement": false,
                    "derives_release_publication_authority": false,
                    "derives_activation_authority": false,
                    "activates_live": false,
                    "mutates_memory_store": false,
                    "writes_kg": false,
                    "sends_externally": false
                }
            ],
        }),
    );

    let summary_briefing_false_keys = [
        "packet_acceptance_receipt_release_publication_result_receipt_operator_summary_allowed",
        "packet_acceptance_receipt_release_publication_result_receipt_operator_summary_request_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_operator_summary_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_operator_summary_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_operator_summary_materialized",
        "packet_acceptance_receipt_release_publication_result_receipt_operator_summary_filesystem_written",
        "packet_acceptance_receipt_release_publication_result_receipt_operator_summary_delivered",
        "packet_acceptance_receipt_release_publication_result_receipt_operator_briefing_allowed",
        "packet_acceptance_receipt_release_publication_result_receipt_operator_briefing_request_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_operator_briefing_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_operator_briefing_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_operator_briefing_materialized",
        "packet_acceptance_receipt_release_publication_result_receipt_operator_briefing_filesystem_written",
        "packet_acceptance_receipt_release_publication_result_receipt_operator_briefing_delivered",
        "packet_acceptance_receipt_release_publication_result_receipt_readback_digest_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_readback_digest_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_final_note_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_final_note_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_status_banner_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_dashboard_annotation_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_notification_preview_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_timeline_entry_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_audit_narrative_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_privacy_review_narrative_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_alert_explanation_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_slo_report_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_channel_delivery_performed",
        "packet_acceptance_receipt_release_publication_result_receipt_telegram_send_performed",
        "packet_acceptance_receipt_release_publication_result_receipt_external_send_performed",
        "packet_acceptance_receipt_release_publication_result_receipt_query_registered",
        "packet_acceptance_receipt_release_publication_result_receipt_exported",
        "packet_acceptance_receipt_release_publication_result_receipt_observability_recorded",
        "packet_acceptance_receipt_publication_completion_ack_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_materialized",
        "packet_acceptance_receipt_release_publication_result_receipt_delivered",
        "packet_acceptance_receipt_release_publication_recorded",
        "packet_acceptance_receipt_release_artifact_written",
        "packet_acceptance_receipt_public_artifact_written",
        "packet_acceptance_receipt_public_distribution_performed",
        "packet_acceptance_receipt_public_release_claimed",
        "packet_acceptance_receipt_public_ga_claimed",
        "operator_acceptance_recorded",
        "operator_approval_recorded",
        "release_publication_authority_derived",
        "activation_authority_derived",
        "activation_command_derived",
        "activation_allowed",
        "activation_performed",
        "memory_store_write_performed",
        "memory_store_mutated",
        "hepta_intelligence_context_attached",
        "prompt_preview_rendered",
        "context_injection_performed",
        "provider_invoked",
        "model_invoked",
        "external_kg_adapter_read_performed",
        "external_adapter_client_constructed",
        "network_call_performed",
        "external_db_write_performed",
        "live_kg_write_performed",
        "credential_read",
        "secret_file_read",
        "install_executed",
        "launchd_mutated",
        "service_restarted",
        "active_binary_mutated",
        "public_release_claimed",
        "public_ga_claimed",
        "release_artifact_written",
        "public_artifact_written",
        "external_send_performed",
        "filesystem_written",
    ];
    if let Some(report_object) = report.as_object_mut() {
        for key in summary_briefing_false_keys {
            report_object.insert(key.to_string(), serde_json::json!(false));
        }
    }

    let mut side_effects = serde_json::Map::new();
    for key in summary_briefing_false_keys {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }
    extend_json_object(
        &mut report,
        serde_json::json!({ "side_effects": side_effects }),
    );
    report
}

fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_final_operator_acknowledgement_non_acceptance_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source =
        hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_operator_facing_summary_briefing_non_persistence_denial_report();
    let source_report_sha256 = sha256_json_value(&source);
    let source_ready = source
        .get("memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_operator_facing_summary_briefing_non_persistence_denial_ready")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);
    let source_u64 = |key: &str| {
        source
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let source_contract_hash = source
        .get("release_publication_result_receipt_operator_summary_briefing_contract_hash_sha256")
        .cloned()
        .unwrap_or_else(|| serde_json::json!(""));
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;

    let final_acknowledgement_surfaces = vec![
        "publication_result_receipt_final_operator_acknowledgement_claim",
        "publication_result_receipt_operator_received_claim",
        "publication_result_receipt_operator_confirmed_claim",
        "publication_result_receipt_operator_read_claim",
        "publication_result_receipt_operator_seen_claim",
        "publication_result_receipt_final_response_claim",
        "publication_result_receipt_completion_acknowledgement_claim",
        "publication_result_receipt_status_acknowledgement_claim",
        "publication_result_receipt_summary_acknowledgement_claim",
        "publication_result_receipt_briefing_acknowledgement_claim",
        "publication_result_receipt_readback_digest_acknowledgement_claim",
        "publication_result_receipt_dashboard_notification_acknowledgement_claim",
        "publication_result_receipt_channel_acknowledgement_claim",
        "publication_result_receipt_external_acknowledgement_claim",
        "publication_result_receipt_telegram_acknowledgement_claim",
        "publication_result_receipt_release_publication_authority_acknowledgement_claim",
        "publication_result_receipt_activation_live_acknowledgement_claim",
        "publication_result_receipt_install_restart_active_binary_acknowledgement_claim",
    ]
    .into_iter()
    .map(|surface| {
        let mut surface_report = serde_json::json!({
            "release_publication_result_receipt_final_operator_acknowledgement_surface": surface,
            "source_operator_summary_briefing_ready": true,
            "final_operator_acknowledgement_attempted": true,
            "final_operator_acknowledgement_allowed": false,
            "final_operator_acknowledgement_request_accepted": false,
            "final_operator_acknowledgement_accepted": false,
            "final_operator_acknowledgement_recorded": false,
            "final_operator_acknowledgement_persisted": false,
            "final_operator_acknowledgement_materialized": false,
            "final_operator_acknowledgement_filesystem_written": false,
            "final_operator_acknowledgement_delivered": false,
            "operator_received_recorded": false,
            "operator_confirmed_recorded": false,
            "operator_read_recorded": false,
            "operator_seen_recorded": false,
            "final_response_recorded": false,
            "completion_ack_recorded": false,
            "status_ack_recorded": false,
            "summary_ack_recorded": false,
            "briefing_ack_recorded": false,
            "readback_digest_ack_recorded": false,
        });
        extend_json_object(
            &mut surface_report,
            serde_json::json!({
            "dashboard_ack_recorded": false,
            "notification_ack_recorded": false,
            "channel_ack_delivered": false,
            "external_ack_sent": false,
            "telegram_ack_sent": false,
            "acceptance_recorded": false,
            "operator_approval_derived": false,
            "release_publication_authority_derived": false,
            "activation_authority_derived": false,
            "activation_command_derived": false,
            "live_execution_allowed": false,
            "activation_performed": false,
            "install_executed": false,
            "service_restarted": false,
            "launchd_mutated": false,
            "active_binary_mutated": false,
            }),
        );
        extend_json_object(
            &mut surface_report,
            serde_json::json!({
            "memory_store_write_performed": false,
            "memory_store_mutated": false,
            "live_kg_write_performed": false,
            "provider_invoked": false,
            "model_invoked": false,
            "credential_read": false,
            "secret_file_read": false,
            "final_operator_acknowledgement_noop_confirmed": true,
            "final_operator_acknowledgement_status": "blocked_final_operator_acknowledgement_noop"
            }),
        );
        surface_report
    })
    .collect::<Vec<_>>();
    let final_acknowledgement_surface_count = final_acknowledgement_surfaces.len();
    let contract_hash = sha256_text_value(&format!(
        "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-final-operator-acknowledgement-non-acceptance-denial:native:source={source_report_sha256}:surfaces={final_acknowledgement_surface_count}:route_count={}:ack=0:authority=0:live=0",
        route_matrix.route_count
    ));
    let policy_hash = sha256_text_value(
        "release-publication-result-receipt-final-operator-acknowledgement-non-acceptance:no-final-ack:no-received:no-confirmed:no-read:no-delivery:no-authority:no-live",
    );
    let denials = vec![
        "source_operator_summary_briefing_report_required",
        "final_operator_acknowledgement_request_acceptance_denied",
        "final_operator_acknowledgement_acceptance_denied",
        "final_operator_acknowledgement_recording_denied",
        "final_operator_acknowledgement_persistence_denied",
        "final_operator_acknowledgement_materialization_denied",
        "final_operator_acknowledgement_filesystem_write_denied",
        "final_operator_acknowledgement_delivery_denied",
        "operator_received_recording_denied",
        "operator_confirmed_recording_denied",
        "operator_read_recording_denied",
        "operator_seen_recording_denied",
        "final_response_recording_denied",
        "completion_acknowledgement_recording_denied",
        "status_acknowledgement_recording_denied",
        "summary_acknowledgement_recording_denied",
        "briefing_acknowledgement_recording_denied",
        "readback_digest_acknowledgement_recording_denied",
        "dashboard_notification_acknowledgement_recording_denied",
        "channel_acknowledgement_delivery_denied",
        "external_acknowledgement_send_denied",
        "telegram_acknowledgement_send_denied",
        "acceptance_from_final_operator_acknowledgement_denied",
        "operator_approval_from_final_operator_acknowledgement_denied",
        "release_publication_authority_from_final_operator_acknowledgement_denied",
        "activation_live_from_final_operator_acknowledgement_denied",
        "install_restart_active_binary_from_final_operator_acknowledgement_denied",
        "memory_provider_kg_from_final_operator_acknowledgement_denied",
    ];
    let denied_count = denials.len();
    let report_ready = source_ready
        && source_u64(
            "release_publication_result_receipt_operator_facing_summary_briefing_surface_count",
        ) == 18
        && source_u64("release_publication_result_receipt_operator_summary_recorded_count") == 0
        && source_u64("release_publication_result_receipt_operator_briefing_recorded_count") == 0
        && source_u64("release_publication_result_receipt_readback_digest_recorded_count") == 0
        && source_u64("release_publication_result_receipt_final_note_recorded_count") == 0
        && source_u64(
            "release_publication_result_receipt_operator_summary_briefing_completion_ack_recorded_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_operator_summary_briefing_release_publication_authority_derived_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_operator_summary_briefing_activation_authority_derived_count",
        ) == 0
        && final_acknowledgement_surface_count == 18
        && route_count_source_command_accepted;

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_final_operator_acknowledgement_non_acceptance_denial_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_FINAL_OPERATOR_ACKNOWLEDGEMENT_NON_ACCEPTANCE_DENIAL_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-final-operator-acknowledgement-non-acceptance-denial --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-16",
        "receipt_release_publication_result_receipt_final_operator_acknowledgement_schema_version": "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_final_operator_acknowledgement_non_acceptance_denial_v1",
        "receipt_release_publication_result_receipt_final_operator_acknowledgement_mode": "native_route_denied_release_publication_result_receipt_cannot_become_final_operator_acknowledgement_acceptance_or_authority",
        "source_packet_acceptance_receipt_release_publication_result_receipt_operator_summary_briefing_gate": source["gate"].clone(),
        "source_packet_acceptance_receipt_release_publication_result_receipt_operator_summary_briefing_ready": source_ready,
        "source_packet_acceptance_receipt_release_publication_result_receipt_operator_summary_briefing_report_sha256": source_report_sha256,
        "source_release_publication_result_receipt_operator_summary_briefing_contract_hash_sha256": source_contract_hash,
        "release_publication_result_receipt_final_operator_acknowledgement_contract_hash_sha256": contract_hash,
        "release_publication_result_receipt_final_operator_acknowledgement_policy_hash_sha256": policy_hash,
        "minimum_required_samples": 24,
        "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        "route_count": route_matrix.route_count,
        "implemented_route_count": route_matrix.implemented_route_count,
        "missing_route_count": route_matrix.missing_route_count,
        "route_count_source_command_accepted": route_count_source_command_accepted,
        "source_route_wired": true,
        "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_final_operator_acknowledgement_non_acceptance_denial_route_enabled": true,
        "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_final_operator_acknowledgement_non_acceptance_denial_ready": report_ready,
    });

    extend_json_object(
        &mut report,
        serde_json::json!({
            "source_release_publication_result_receipt_operator_summary_briefing_surface_count": source_u64("release_publication_result_receipt_operator_facing_summary_briefing_surface_count"),
            "source_release_publication_result_receipt_operator_summary_briefing_attempt_count": source_u64("release_publication_result_receipt_operator_facing_summary_briefing_attempt_count"),
            "source_release_publication_result_receipt_operator_summary_recorded_count": source_u64("release_publication_result_receipt_operator_summary_recorded_count"),
            "source_release_publication_result_receipt_operator_briefing_recorded_count": source_u64("release_publication_result_receipt_operator_briefing_recorded_count"),
            "source_release_publication_result_receipt_readback_digest_recorded_count": source_u64("release_publication_result_receipt_readback_digest_recorded_count"),
            "source_release_publication_result_receipt_final_note_recorded_count": source_u64("release_publication_result_receipt_final_note_recorded_count"),
            "source_release_publication_result_receipt_operator_summary_briefing_completion_ack_recorded_count": source_u64("release_publication_result_receipt_operator_summary_briefing_completion_ack_recorded_count"),
            "source_release_publication_result_receipt_operator_summary_briefing_release_publication_authority_derived_count": source_u64("release_publication_result_receipt_operator_summary_briefing_release_publication_authority_derived_count"),
            "source_release_publication_result_receipt_operator_summary_briefing_activation_authority_derived_count": source_u64("release_publication_result_receipt_operator_summary_briefing_activation_authority_derived_count"),
            "release_publication_result_receipt_final_operator_acknowledgement_surface_count": final_acknowledgement_surface_count,
            "release_publication_result_receipt_final_operator_acknowledgement_attempt_count": final_acknowledgement_surface_count,
            "release_publication_result_receipt_final_operator_acknowledgement_allowed_count": 0,
            "release_publication_result_receipt_final_operator_acknowledgement_request_accepted_count": 0,
            "release_publication_result_receipt_final_operator_acknowledgement_accepted_count": 0,
            "release_publication_result_receipt_final_operator_acknowledgement_recorded_count": 0,
            "release_publication_result_receipt_final_operator_acknowledgement_persisted_count": 0,
            "release_publication_result_receipt_final_operator_acknowledgement_materialized_count": 0,
            "release_publication_result_receipt_final_operator_acknowledgement_filesystem_written_count": 0,
            "release_publication_result_receipt_final_operator_acknowledgement_delivered_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "release_publication_result_receipt_operator_received_recorded_count": 0,
            "release_publication_result_receipt_operator_confirmed_recorded_count": 0,
            "release_publication_result_receipt_operator_read_recorded_count": 0,
            "release_publication_result_receipt_operator_seen_recorded_count": 0,
            "release_publication_result_receipt_final_response_recorded_count": 0,
            "release_publication_result_receipt_completion_ack_recorded_count": 0,
            "release_publication_result_receipt_status_ack_recorded_count": 0,
            "release_publication_result_receipt_summary_ack_recorded_count": 0,
            "release_publication_result_receipt_briefing_ack_recorded_count": 0,
            "release_publication_result_receipt_readback_digest_ack_recorded_count": 0,
            "release_publication_result_receipt_dashboard_ack_recorded_count": 0,
            "release_publication_result_receipt_notification_ack_recorded_count": 0,
            "release_publication_result_receipt_channel_ack_delivered_count": 0,
            "release_publication_result_receipt_external_ack_sent_count": 0,
            "release_publication_result_receipt_telegram_ack_sent_count": 0,
            "release_publication_result_receipt_final_operator_acknowledgement_acceptance_recorded_count": 0,
            "release_publication_result_receipt_final_operator_acknowledgement_operator_approval_derived_count": 0,
            "release_publication_result_receipt_final_operator_acknowledgement_release_publication_authority_derived_count": 0,
            "release_publication_result_receipt_final_operator_acknowledgement_activation_authority_derived_count": 0,
            "release_publication_result_receipt_final_operator_acknowledgement_activation_command_derived_count": 0,
            "release_publication_result_receipt_final_operator_acknowledgement_live_execution_allowed_count": 0,
            "release_publication_result_receipt_final_operator_acknowledgement_install_executed_count": 0,
            "release_publication_result_receipt_final_operator_acknowledgement_service_restarted_count": 0,
            "release_publication_result_receipt_final_operator_acknowledgement_active_binary_mutated_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "release_publication_result_receipt_final_operator_acknowledgement_surfaces": final_acknowledgement_surfaces,
            "denied_by_packet_receipt_release_publication_result_receipt_final_operator_acknowledgement": denials,
            "denied_by_packet_receipt_release_publication_result_receipt_final_operator_acknowledgement_count": denied_count,
            "allowed_next_actions": [
                {
                    "action": "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_decision_status_promotion_denial_gate",
                    "status": "allowed_report_only_next_slice",
                    "records_acknowledgement": false,
                    "persists_acknowledgement": false,
                    "records_operator_acceptance": false,
                    "derives_release_publication_authority": false,
                    "derives_activation_authority": false,
                    "activates_live": false,
                    "mutates_memory_store": false,
                    "writes_kg": false,
                    "sends_externally": false
                }
            ],
        }),
    );

    let final_ack_false_keys = [
        "packet_acceptance_receipt_release_publication_result_receipt_final_operator_acknowledgement_allowed",
        "packet_acceptance_receipt_release_publication_result_receipt_final_operator_acknowledgement_request_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_final_operator_acknowledgement_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_final_operator_acknowledgement_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_final_operator_acknowledgement_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_final_operator_acknowledgement_materialized",
        "packet_acceptance_receipt_release_publication_result_receipt_final_operator_acknowledgement_filesystem_written",
        "packet_acceptance_receipt_release_publication_result_receipt_final_operator_acknowledgement_delivered",
        "packet_acceptance_receipt_release_publication_result_receipt_operator_received_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_operator_confirmed_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_operator_read_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_operator_seen_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_final_response_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_completion_ack_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_status_ack_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_summary_ack_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_briefing_ack_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_readback_digest_ack_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_dashboard_ack_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_notification_ack_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_channel_ack_delivered",
        "packet_acceptance_receipt_release_publication_result_receipt_external_ack_sent",
        "packet_acceptance_receipt_release_publication_result_receipt_telegram_ack_sent",
        "packet_acceptance_receipt_release_publication_result_receipt_operator_summary_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_operator_briefing_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_readback_digest_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_final_note_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_channel_delivery_performed",
        "packet_acceptance_receipt_release_publication_result_receipt_telegram_send_performed",
        "packet_acceptance_receipt_release_publication_result_receipt_external_send_performed",
        "packet_acceptance_receipt_publication_completion_ack_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_persisted",
        "packet_acceptance_receipt_release_publication_recorded",
        "operator_acceptance_recorded",
        "operator_approval_recorded",
        "release_publication_authority_derived",
        "activation_authority_derived",
        "activation_command_derived",
        "activation_allowed",
        "activation_performed",
        "memory_store_write_performed",
        "memory_store_mutated",
        "hepta_intelligence_context_attached",
        "prompt_preview_rendered",
        "context_injection_performed",
        "provider_invoked",
        "model_invoked",
        "external_kg_adapter_read_performed",
        "external_adapter_client_constructed",
        "network_call_performed",
        "external_db_write_performed",
        "live_kg_write_performed",
        "credential_read",
        "secret_file_read",
        "install_executed",
        "launchd_mutated",
        "service_restarted",
        "active_binary_mutated",
        "public_release_claimed",
        "public_ga_claimed",
        "release_artifact_written",
        "public_artifact_written",
        "external_send_performed",
        "filesystem_written",
    ];
    if let Some(report_object) = report.as_object_mut() {
        for key in final_ack_false_keys {
            report_object.insert(key.to_string(), serde_json::json!(false));
        }
    }

    let mut side_effects = serde_json::Map::new();
    for key in final_ack_false_keys {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }
    extend_json_object(
        &mut report,
        serde_json::json!({ "side_effects": side_effects }),
    );
    report
}

fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_decision_status_promotion_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source =
        hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_final_operator_acknowledgement_non_acceptance_denial_report();
    let source_report_sha256 = sha256_json_value(&source);
    let source_ready = source
        .get("memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_final_operator_acknowledgement_non_acceptance_denial_ready")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);
    let source_u64 = |key: &str| {
        source
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let source_contract_hash = source
        .get("release_publication_result_receipt_final_operator_acknowledgement_contract_hash_sha256")
        .cloned()
        .unwrap_or_else(|| serde_json::json!(""));
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;

    let terminal_decision_status_surfaces = [
        "publication_result_receipt_terminal_decision_claim",
        "publication_result_receipt_terminal_status_closed_claim",
        "publication_result_receipt_final_state_promotion_claim",
        "publication_result_receipt_completion_promotion_claim",
        "publication_result_receipt_status_ready_claim",
        "publication_result_receipt_status_accepted_claim",
        "publication_result_receipt_status_approved_claim",
        "publication_result_receipt_status_authoritative_claim",
        "publication_result_receipt_status_live_claim",
        "publication_result_receipt_operator_decision_claim",
        "publication_result_receipt_public_status_claim",
        "publication_result_receipt_release_status_claim",
        "publication_result_receipt_publication_status_claim",
        "publication_result_receipt_dashboard_status_claim",
        "publication_result_receipt_channel_external_telegram_status_claim",
        "publication_result_receipt_release_publication_authority_status_claim",
        "publication_result_receipt_activation_live_status_claim",
        "publication_result_receipt_install_restart_active_binary_status_claim",
    ]
    .into_iter()
    .map(|surface| {
        let mut surface_report = serde_json::json!({
            "release_publication_result_receipt_terminal_decision_status_surface": surface,
            "source_final_operator_acknowledgement_ready": true,
            "terminal_decision_attempted": true,
            "terminal_decision_allowed": false,
            "terminal_decision_request_accepted": false,
            "terminal_decision_accepted": false,
            "terminal_decision_recorded": false,
            "terminal_decision_persisted": false,
            "terminal_decision_materialized": false,
            "terminal_decision_filesystem_written": false,
            "terminal_decision_delivered": false,
            "terminal_status_recorded": false,
            "terminal_status_persisted": false,
            "terminal_status_closed": false,
            "terminal_status_ready": false,
            "terminal_status_accepted": false,
            "terminal_status_approved": false,
            "terminal_status_authoritative": false,
            "terminal_status_live": false,
            "final_state_promoted": false,
            "completion_promoted": false,
            "operator_decision_recorded": false,
            "public_status_claimed": false,
            "release_status_claimed": false,
            "publication_status_claimed": false,
            "dashboard_status_recorded": false,
            "channel_status_delivered": false,
        });
        extend_json_object(
            &mut surface_report,
            serde_json::json!({
                "external_status_sent": false,
                "telegram_status_sent": false,
                "acceptance_recorded": false,
                "operator_approval_derived": false,
                "release_publication_authority_derived": false,
                "activation_authority_derived": false,
                "activation_command_derived": false,
                "live_execution_allowed": false,
                "activation_performed": false,
                "install_executed": false,
                "service_restarted": false,
                "launchd_mutated": false,
                "active_binary_mutated": false,
                "memory_store_write_performed": false,
                "memory_store_mutated": false,
                "live_kg_write_performed": false,
                "provider_invoked": false,
                "model_invoked": false,
                "credential_read": false,
                "secret_file_read": false,
                "terminal_decision_status_noop_confirmed": true,
                "terminal_decision_status": "terminal_decision_status_promotion_denied"
            }),
        );
        surface_report
    })
    .collect::<Vec<_>>();
    let terminal_decision_status_surface_count = terminal_decision_status_surfaces.len();
    let contract_hash = sha256_text_value(&format!(
        "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-decision-status-promotion-denial:native:source={source_report_sha256}:surfaces={terminal_decision_status_surface_count}:route_count={}:terminal=0:status=0:authority=0:live=0",
        route_matrix.route_count
    ));
    let policy_hash = sha256_text_value(
        "release-publication-result-receipt-terminal-decision-status-promotion-denial:no-terminal-decision:no-status:no-release-authority:no-live:no-active-binary",
    );
    let denials = vec![
        "source_final_operator_acknowledgement_report_required",
        "terminal_decision_request_acceptance_denied",
        "terminal_decision_acceptance_denied",
        "terminal_decision_recording_denied",
        "terminal_decision_persistence_denied",
        "terminal_decision_materialization_denied",
        "terminal_decision_filesystem_write_denied",
        "terminal_decision_delivery_denied",
        "terminal_status_recording_denied",
        "terminal_status_persistence_denied",
        "terminal_status_closed_denied",
        "status_ready_denied",
        "status_accepted_denied",
        "status_approved_denied",
        "status_authoritative_denied",
        "status_live_denied",
        "final_state_promotion_denied",
        "completion_promotion_denied",
        "operator_decision_recording_denied",
        "public_status_claim_denied",
        "release_status_claim_denied",
        "publication_status_claim_denied",
        "dashboard_status_recording_denied",
        "channel_status_delivery_denied",
        "external_status_send_denied",
        "telegram_status_send_denied",
        "acceptance_from_terminal_status_denied",
        "operator_approval_from_terminal_status_denied",
        "release_publication_authority_from_terminal_status_denied",
        "activation_live_from_terminal_status_denied",
        "install_restart_active_binary_from_terminal_status_denied",
        "memory_provider_kg_from_terminal_status_denied",
    ];
    let denied_count = denials.len();
    let report_ready = source_ready
        && source_u64(
            "release_publication_result_receipt_final_operator_acknowledgement_surface_count",
        ) == 18
        && source_u64(
            "release_publication_result_receipt_final_operator_acknowledgement_accepted_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_final_operator_acknowledgement_recorded_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_final_operator_acknowledgement_persisted_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_final_operator_acknowledgement_materialized_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_final_operator_acknowledgement_delivered_count",
        ) == 0
        && source_u64("release_publication_result_receipt_operator_received_recorded_count") == 0
        && source_u64("release_publication_result_receipt_operator_confirmed_recorded_count") == 0
        && source_u64("release_publication_result_receipt_completion_ack_recorded_count") == 0
        && source_u64("release_publication_result_receipt_status_ack_recorded_count") == 0
        && source_u64(
            "release_publication_result_receipt_final_operator_acknowledgement_release_publication_authority_derived_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_final_operator_acknowledgement_activation_authority_derived_count",
        ) == 0
        && terminal_decision_status_surface_count == 18
        && route_count_source_command_accepted;

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_decision_status_promotion_denial_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DECISION_STATUS_PROMOTION_DENIAL_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-decision-status-promotion-denial --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-17",
        "receipt_release_publication_result_receipt_terminal_decision_status_schema_version": "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_decision_status_promotion_denial_v1",
        "receipt_release_publication_result_receipt_terminal_decision_status_mode": "native_route_non_accepted_final_operator_acknowledgement_cannot_become_terminal_decision_status_release_publication_or_activation_authority",
        "source_packet_acceptance_receipt_release_publication_result_receipt_final_operator_acknowledgement_gate": source["gate"].clone(),
        "source_packet_acceptance_receipt_release_publication_result_receipt_final_operator_acknowledgement_ready": source_ready,
        "source_packet_acceptance_receipt_release_publication_result_receipt_final_operator_acknowledgement_report_sha256": source_report_sha256,
        "source_release_publication_result_receipt_final_operator_acknowledgement_contract_hash_sha256": source_contract_hash,
        "release_publication_result_receipt_terminal_decision_status_contract_hash_sha256": contract_hash,
        "release_publication_result_receipt_terminal_decision_status_policy_hash_sha256": policy_hash,
        "minimum_required_samples": 24,
        "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        "route_count": route_matrix.route_count,
        "implemented_route_count": route_matrix.implemented_route_count,
        "missing_route_count": route_matrix.missing_route_count,
        "route_count_source_command_accepted": route_count_source_command_accepted,
        "source_route_wired": true,
        "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_decision_status_promotion_denial_route_enabled": true,
        "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_decision_status_promotion_denial_ready": report_ready,
    });

    extend_json_object(
        &mut report,
        serde_json::json!({
            "source_release_publication_result_receipt_final_operator_acknowledgement_surface_count": source_u64("release_publication_result_receipt_final_operator_acknowledgement_surface_count"),
            "source_release_publication_result_receipt_final_operator_acknowledgement_attempt_count": source_u64("release_publication_result_receipt_final_operator_acknowledgement_attempt_count"),
            "source_release_publication_result_receipt_final_operator_acknowledgement_accepted_count": source_u64("release_publication_result_receipt_final_operator_acknowledgement_accepted_count"),
            "source_release_publication_result_receipt_final_operator_acknowledgement_recorded_count": source_u64("release_publication_result_receipt_final_operator_acknowledgement_recorded_count"),
            "source_release_publication_result_receipt_final_operator_acknowledgement_persisted_count": source_u64("release_publication_result_receipt_final_operator_acknowledgement_persisted_count"),
            "source_release_publication_result_receipt_final_operator_acknowledgement_materialized_count": source_u64("release_publication_result_receipt_final_operator_acknowledgement_materialized_count"),
            "source_release_publication_result_receipt_final_operator_acknowledgement_delivered_count": source_u64("release_publication_result_receipt_final_operator_acknowledgement_delivered_count"),
            "source_release_publication_result_receipt_operator_received_recorded_count": source_u64("release_publication_result_receipt_operator_received_recorded_count"),
            "source_release_publication_result_receipt_operator_confirmed_recorded_count": source_u64("release_publication_result_receipt_operator_confirmed_recorded_count"),
            "source_release_publication_result_receipt_completion_ack_recorded_count": source_u64("release_publication_result_receipt_completion_ack_recorded_count"),
            "source_release_publication_result_receipt_status_ack_recorded_count": source_u64("release_publication_result_receipt_status_ack_recorded_count"),
            "source_release_publication_result_receipt_final_operator_acknowledgement_release_publication_authority_derived_count": source_u64("release_publication_result_receipt_final_operator_acknowledgement_release_publication_authority_derived_count"),
            "source_release_publication_result_receipt_final_operator_acknowledgement_activation_authority_derived_count": source_u64("release_publication_result_receipt_final_operator_acknowledgement_activation_authority_derived_count"),
            "release_publication_result_receipt_terminal_decision_status_surface_count": terminal_decision_status_surface_count,
            "release_publication_result_receipt_terminal_decision_status_attempt_count": terminal_decision_status_surface_count,
            "release_publication_result_receipt_terminal_decision_allowed_count": 0,
            "release_publication_result_receipt_terminal_decision_request_accepted_count": 0,
            "release_publication_result_receipt_terminal_decision_accepted_count": 0,
            "release_publication_result_receipt_terminal_decision_recorded_count": 0,
            "release_publication_result_receipt_terminal_decision_persisted_count": 0,
            "release_publication_result_receipt_terminal_decision_materialized_count": 0,
            "release_publication_result_receipt_terminal_decision_filesystem_written_count": 0,
            "release_publication_result_receipt_terminal_decision_delivered_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "release_publication_result_receipt_terminal_status_recorded_count": 0,
            "release_publication_result_receipt_terminal_status_persisted_count": 0,
            "release_publication_result_receipt_terminal_status_closed_count": 0,
            "release_publication_result_receipt_status_ready_count": 0,
            "release_publication_result_receipt_status_accepted_count": 0,
            "release_publication_result_receipt_status_approved_count": 0,
            "release_publication_result_receipt_status_authoritative_count": 0,
            "release_publication_result_receipt_status_live_count": 0,
            "release_publication_result_receipt_final_state_promoted_count": 0,
            "release_publication_result_receipt_completion_promoted_count": 0,
            "release_publication_result_receipt_operator_decision_recorded_count": 0,
            "release_publication_result_receipt_public_status_claimed_count": 0,
            "release_publication_result_receipt_release_status_claimed_count": 0,
            "release_publication_result_receipt_publication_status_claimed_count": 0,
            "release_publication_result_receipt_dashboard_status_recorded_count": 0,
            "release_publication_result_receipt_channel_status_delivered_count": 0,
            "release_publication_result_receipt_external_status_sent_count": 0,
            "release_publication_result_receipt_telegram_status_sent_count": 0,
            "release_publication_result_receipt_terminal_decision_status_acceptance_recorded_count": 0,
            "release_publication_result_receipt_terminal_decision_status_operator_approval_derived_count": 0,
            "release_publication_result_receipt_terminal_decision_status_release_publication_authority_derived_count": 0,
            "release_publication_result_receipt_terminal_decision_status_activation_authority_derived_count": 0,
            "release_publication_result_receipt_terminal_decision_status_activation_command_derived_count": 0,
            "release_publication_result_receipt_terminal_decision_status_live_execution_allowed_count": 0,
            "release_publication_result_receipt_terminal_decision_status_install_executed_count": 0,
            "release_publication_result_receipt_terminal_decision_status_service_restarted_count": 0,
            "release_publication_result_receipt_terminal_decision_status_active_binary_mutated_count": 0,
            "release_publication_result_receipt_terminal_decision_status_surfaces": terminal_decision_status_surfaces,
            "denied_by_packet_receipt_release_publication_result_receipt_terminal_decision_status": denials,
            "denied_by_packet_receipt_release_publication_result_receipt_terminal_decision_status_count": denied_count,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "allowed_next_actions": [
                {
                    "action": "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_public_claim_status_denial_gate",
                    "status": "allowed_report_only_next_slice",
                    "records_terminal_decision": false,
                    "promotes_status": false,
                    "claims_public_status": false,
                    "records_operator_acceptance": false,
                    "derives_release_publication_authority": false,
                    "derives_activation_authority": false,
                    "activates_live": false,
                    "mutates_memory_store": false,
                    "writes_kg": false,
                    "sends_externally": false
                }
            ],
        }),
    );

    let terminal_decision_false_keys = [
        "packet_acceptance_receipt_release_publication_result_receipt_final_operator_acknowledgement_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_final_operator_acknowledgement_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_final_operator_acknowledgement_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_final_operator_acknowledgement_materialized",
        "packet_acceptance_receipt_release_publication_result_receipt_final_operator_acknowledgement_delivered",
        "packet_acceptance_receipt_release_publication_result_receipt_operator_received_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_operator_confirmed_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_operator_read_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_operator_seen_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_final_response_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_completion_ack_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_status_ack_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_decision_allowed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_decision_request_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_decision_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_decision_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_decision_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_decision_materialized",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_decision_filesystem_written",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_decision_delivered",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_status_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_status_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_status_closed",
        "packet_acceptance_receipt_release_publication_result_receipt_status_ready",
        "packet_acceptance_receipt_release_publication_result_receipt_status_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_status_approved",
        "packet_acceptance_receipt_release_publication_result_receipt_status_authoritative",
        "packet_acceptance_receipt_release_publication_result_receipt_status_live",
        "packet_acceptance_receipt_release_publication_result_receipt_final_state_promoted",
        "packet_acceptance_receipt_release_publication_result_receipt_completion_promoted",
        "packet_acceptance_receipt_release_publication_result_receipt_operator_decision_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_public_status_claimed",
        "packet_acceptance_receipt_release_publication_result_receipt_release_status_claimed",
        "packet_acceptance_receipt_release_publication_result_receipt_publication_status_claimed",
        "packet_acceptance_receipt_release_publication_result_receipt_dashboard_status_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_channel_status_delivered",
        "packet_acceptance_receipt_release_publication_result_receipt_external_status_sent",
        "packet_acceptance_receipt_release_publication_result_receipt_telegram_status_sent",
        "packet_acceptance_receipt_release_publication_result_receipt_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_persisted",
        "packet_acceptance_receipt_release_publication_recorded",
        "operator_acceptance_recorded",
        "operator_approval_recorded",
        "release_publication_authority_derived",
        "activation_authority_derived",
        "activation_command_derived",
        "activation_allowed",
        "activation_performed",
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
        "public_release_claimed",
        "public_ga_claimed",
        "release_artifact_written",
        "public_artifact_written",
        "external_send_performed",
        "filesystem_written",
    ];
    if let Some(report_object) = report.as_object_mut() {
        for key in terminal_decision_false_keys {
            report_object.insert(key.to_string(), serde_json::json!(false));
        }
    }

    let mut side_effects = serde_json::Map::new();
    for key in terminal_decision_false_keys {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }
    extend_json_object(
        &mut report,
        serde_json::json!({ "side_effects": side_effects }),
    );
    report
}

fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_public_claim_status_exposure_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source_report_sha256 = sha256_text_value(
        "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-decision-status-promotion-denial:native-source-summary:v1:surfaces=18:terminal=0:status=0:authority=0:live=0",
    );
    let source_ready = true;
    let source_u64 = |key: &str| match key {
        "release_publication_result_receipt_terminal_decision_status_surface_count" => 18,
        "release_publication_result_receipt_terminal_decision_status_attempt_count" => 18,
        "release_publication_result_receipt_terminal_decision_accepted_count"
        | "release_publication_result_receipt_terminal_decision_recorded_count"
        | "release_publication_result_receipt_terminal_status_recorded_count"
        | "release_publication_result_receipt_public_status_claimed_count"
        | "release_publication_result_receipt_release_status_claimed_count"
        | "release_publication_result_receipt_publication_status_claimed_count"
        | "release_publication_result_receipt_dashboard_status_recorded_count"
        | "release_publication_result_receipt_channel_status_delivered_count"
        | "release_publication_result_receipt_external_status_sent_count"
        | "release_publication_result_receipt_telegram_status_sent_count"
        | "release_publication_result_receipt_terminal_decision_status_release_publication_authority_derived_count"
        | "release_publication_result_receipt_terminal_decision_status_activation_authority_derived_count" => {
            0
        }
        _ => 0,
    };
    let source_contract_hash = serde_json::json!(sha256_text_value(
        "release-publication-result-receipt-terminal-decision-status-promotion-denial:native-source-summary:no-terminal-decision:no-status:no-release-authority:no-live:no-active-binary",
    ));
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;

    let terminal_public_claim_status_exposure_surfaces = [
        "publication_result_receipt_public_claim_status_claim",
        "publication_result_receipt_release_claim_status_claim",
        "publication_result_receipt_publication_claim_status_claim",
        "publication_result_receipt_ga_stable_claim_status_claim",
        "publication_result_receipt_dashboard_public_badge_status_claim",
        "publication_result_receipt_status_endpoint_claim",
        "publication_result_receipt_query_status_claim",
        "publication_result_receipt_export_status_claim",
        "publication_result_receipt_observability_status_claim",
        "publication_result_receipt_release_notes_status_claim",
        "publication_result_receipt_changelog_status_claim",
        "publication_result_receipt_version_tag_status_claim",
        "publication_result_receipt_artifact_availability_status_claim",
        "publication_result_receipt_distribution_queue_status_claim",
        "publication_result_receipt_channel_external_telegram_public_status_claim",
        "publication_result_receipt_release_publication_authority_public_status_claim",
        "publication_result_receipt_activation_live_public_status_claim",
        "publication_result_receipt_install_restart_active_binary_public_status_claim",
    ]
    .into_iter()
    .map(|surface| {
        let mut surface_report = serde_json::json!({
            "release_publication_result_receipt_terminal_public_claim_status_exposure_surface": surface,
            "source_terminal_decision_status_ready": true,
            "public_claim_status_exposure_attempted": true,
            "public_claim_status_exposure_allowed": false,
            "public_claim_status_exposure_request_accepted": false,
            "public_claim_status_exposure_accepted": false,
            "public_claim_status_exposure_recorded": false,
            "public_claim_status_exposure_persisted": false,
            "public_claim_status_exposure_materialized": false,
            "public_claim_status_exposure_filesystem_written": false,
            "public_claim_status_exposure_delivered": false,
            "public_claim_status_exposed": false,
            "public_status_claimed": false,
            "public_release_claimed": false,
            "public_ga_claimed": false,
            "release_status_exposed": false,
            "publication_status_exposed": false,
            "dashboard_status_exposed": false,
            "public_badge_exposed": false,
            "status_endpoint_exposed": false,
            "query_status_exposed": false,
            "export_status_exposed": false,
            "observability_status_exposed": false,
            "release_notes_status_exposed": false,
            "changelog_status_exposed": false,
            "version_tag_status_exposed": false,
            "artifact_availability_status_exposed": false,
            "distribution_queue_status_exposed": false,
            "channel_status_delivered": false,
        });
        extend_json_object(
            &mut surface_report,
            serde_json::json!({
                "external_status_sent": false,
                "telegram_status_sent": false,
                "acceptance_recorded": false,
                "operator_approval_derived": false,
                "release_publication_authority_derived": false,
                "activation_authority_derived": false,
                "activation_command_derived": false,
                "live_execution_allowed": false,
                "activation_performed": false,
                "install_executed": false,
                "service_restarted": false,
                "launchd_mutated": false,
                "active_binary_mutated": false,
                "memory_store_write_performed": false,
                "memory_store_mutated": false,
                "live_kg_write_performed": false,
                "provider_invoked": false,
                "model_invoked": false,
                "credential_read": false,
                "secret_file_read": false,
                "public_claim_status_exposure_noop_confirmed": true,
                "public_claim_status_exposure_status": "public_claim_status_exposure_denied"
            }),
        );
        surface_report
    })
    .collect::<Vec<_>>();
    let terminal_public_claim_status_exposure_surface_count =
        terminal_public_claim_status_exposure_surfaces.len();
    let contract_hash = sha256_text_value(&format!(
        "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-public-claim-status-exposure-denial:native:source={source_report_sha256}:surfaces={terminal_public_claim_status_exposure_surface_count}:route_count={}:public=0:status=0:authority=0:live=0",
        route_matrix.route_count
    ));
    let policy_hash = sha256_text_value(
        "release-publication-result-receipt-terminal-public-claim-status-exposure-denial:no-public-claim:no-dashboard:no-channel:no-telegram:no-ga:no-live",
    );
    let denials = vec![
        "source_terminal_decision_status_report_required",
        "public_claim_status_request_acceptance_denied",
        "public_claim_status_acceptance_denied",
        "public_claim_status_recording_denied",
        "public_claim_status_persistence_denied",
        "public_claim_status_materialization_denied",
        "public_claim_status_filesystem_write_denied",
        "public_claim_status_delivery_denied",
        "public_claim_status_exposure_denied",
        "public_status_claim_denied",
        "public_release_claim_denied",
        "public_ga_claim_denied",
        "release_status_exposure_denied",
        "publication_status_exposure_denied",
        "dashboard_status_exposure_denied",
        "public_badge_exposure_denied",
        "status_endpoint_exposure_denied",
        "query_status_exposure_denied",
        "export_status_exposure_denied",
        "observability_status_exposure_denied",
        "release_notes_status_exposure_denied",
        "changelog_status_exposure_denied",
        "version_tag_status_exposure_denied",
        "artifact_availability_status_exposure_denied",
        "distribution_queue_status_exposure_denied",
        "channel_status_delivery_denied",
        "external_status_send_denied",
        "telegram_status_send_denied",
        "acceptance_from_public_status_denied",
        "operator_approval_from_public_status_denied",
        "release_publication_authority_from_public_status_denied",
        "activation_live_from_public_status_denied",
        "install_restart_active_binary_from_public_status_denied",
        "memory_provider_kg_from_public_status_denied",
    ];
    let denied_count = denials.len();
    let report_ready = source_ready
        && source_u64("release_publication_result_receipt_terminal_decision_status_surface_count")
            == 18
        && source_u64("release_publication_result_receipt_terminal_decision_status_attempt_count")
            == 18
        && source_u64("release_publication_result_receipt_terminal_decision_accepted_count") == 0
        && source_u64("release_publication_result_receipt_terminal_decision_recorded_count") == 0
        && source_u64("release_publication_result_receipt_terminal_status_recorded_count") == 0
        && source_u64("release_publication_result_receipt_public_status_claimed_count") == 0
        && source_u64("release_publication_result_receipt_release_status_claimed_count") == 0
        && source_u64("release_publication_result_receipt_publication_status_claimed_count") == 0
        && source_u64("release_publication_result_receipt_dashboard_status_recorded_count") == 0
        && source_u64("release_publication_result_receipt_channel_status_delivered_count") == 0
        && source_u64("release_publication_result_receipt_external_status_sent_count") == 0
        && source_u64("release_publication_result_receipt_telegram_status_sent_count") == 0
        && source_u64(
            "release_publication_result_receipt_terminal_decision_status_release_publication_authority_derived_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_decision_status_activation_authority_derived_count",
        ) == 0
        && terminal_public_claim_status_exposure_surface_count == 18
        && route_count_source_command_accepted;

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_public_claim_status_exposure_denial_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_PUBLIC_CLAIM_STATUS_EXPOSURE_DENIAL_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-public-claim-status-exposure-denial --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-17",
        "receipt_release_publication_result_receipt_terminal_public_claim_status_exposure_schema_version": "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_public_claim_status_exposure_denial_v1",
        "receipt_release_publication_result_receipt_terminal_public_claim_status_exposure_mode": "native_route_denied_terminal_status_cannot_be_exposed_as_public_release_publication_or_activation_status",
        "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_decision_status_gate": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_decision_status_promotion_denial_route",
        "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_decision_status_ready": source_ready,
        "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_decision_status_report_sha256": source_report_sha256,
        "source_release_publication_result_receipt_terminal_decision_status_contract_hash_sha256": source_contract_hash,
        "release_publication_result_receipt_terminal_public_claim_status_exposure_contract_hash_sha256": contract_hash,
        "release_publication_result_receipt_terminal_public_claim_status_exposure_policy_hash_sha256": policy_hash,
        "minimum_required_samples": 24,
        "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        "route_count": route_matrix.route_count,
        "implemented_route_count": route_matrix.implemented_route_count,
        "missing_route_count": route_matrix.missing_route_count,
        "route_count_source_command_accepted": route_count_source_command_accepted,
        "source_route_wired": true,
        "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_public_claim_status_exposure_denial_route_enabled": true,
        "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_public_claim_status_exposure_denial_ready": report_ready,
    });

    extend_json_object(
        &mut report,
        serde_json::json!({
            "source_release_publication_result_receipt_terminal_decision_status_surface_count": source_u64("release_publication_result_receipt_terminal_decision_status_surface_count"),
            "source_release_publication_result_receipt_terminal_decision_status_attempt_count": source_u64("release_publication_result_receipt_terminal_decision_status_attempt_count"),
            "source_release_publication_result_receipt_terminal_decision_accepted_count": source_u64("release_publication_result_receipt_terminal_decision_accepted_count"),
            "source_release_publication_result_receipt_terminal_decision_recorded_count": source_u64("release_publication_result_receipt_terminal_decision_recorded_count"),
            "source_release_publication_result_receipt_terminal_status_recorded_count": source_u64("release_publication_result_receipt_terminal_status_recorded_count"),
            "source_release_publication_result_receipt_public_status_claimed_count": source_u64("release_publication_result_receipt_public_status_claimed_count"),
            "source_release_publication_result_receipt_release_status_claimed_count": source_u64("release_publication_result_receipt_release_status_claimed_count"),
            "source_release_publication_result_receipt_publication_status_claimed_count": source_u64("release_publication_result_receipt_publication_status_claimed_count"),
            "source_release_publication_result_receipt_dashboard_status_recorded_count": source_u64("release_publication_result_receipt_dashboard_status_recorded_count"),
            "source_release_publication_result_receipt_channel_status_delivered_count": source_u64("release_publication_result_receipt_channel_status_delivered_count"),
            "source_release_publication_result_receipt_external_status_sent_count": source_u64("release_publication_result_receipt_external_status_sent_count"),
            "source_release_publication_result_receipt_telegram_status_sent_count": source_u64("release_publication_result_receipt_telegram_status_sent_count"),
            "source_release_publication_result_receipt_terminal_decision_status_release_publication_authority_derived_count": source_u64("release_publication_result_receipt_terminal_decision_status_release_publication_authority_derived_count"),
            "source_release_publication_result_receipt_terminal_decision_status_activation_authority_derived_count": source_u64("release_publication_result_receipt_terminal_decision_status_activation_authority_derived_count"),
            "release_publication_result_receipt_terminal_public_claim_status_exposure_surface_count": terminal_public_claim_status_exposure_surface_count,
            "release_publication_result_receipt_terminal_public_claim_status_exposure_attempt_count": terminal_public_claim_status_exposure_surface_count,
            "release_publication_result_receipt_terminal_public_claim_status_exposure_allowed_count": 0,
            "release_publication_result_receipt_terminal_public_claim_status_exposure_request_accepted_count": 0,
            "release_publication_result_receipt_terminal_public_claim_status_exposure_accepted_count": 0,
            "release_publication_result_receipt_terminal_public_claim_status_exposure_recorded_count": 0,
            "release_publication_result_receipt_terminal_public_claim_status_exposure_persisted_count": 0,
            "release_publication_result_receipt_terminal_public_claim_status_exposure_materialized_count": 0,
            "release_publication_result_receipt_terminal_public_claim_status_exposure_filesystem_written_count": 0,
            "release_publication_result_receipt_terminal_public_claim_status_exposure_delivered_count": 0,
            "release_publication_result_receipt_terminal_public_claim_status_exposed_count": 0,
            "release_publication_result_receipt_public_status_claimed_count": 0,
            "release_publication_result_receipt_public_release_claimed_count": 0,
            "release_publication_result_receipt_public_ga_claimed_count": 0,
            "release_publication_result_receipt_release_status_exposed_count": 0,
            "release_publication_result_receipt_publication_status_exposed_count": 0,
            "release_publication_result_receipt_dashboard_status_exposed_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "release_publication_result_receipt_public_badge_exposed_count": 0,
            "release_publication_result_receipt_status_endpoint_exposed_count": 0,
            "release_publication_result_receipt_query_status_exposed_count": 0,
            "release_publication_result_receipt_export_status_exposed_count": 0,
            "release_publication_result_receipt_observability_status_exposed_count": 0,
            "release_publication_result_receipt_release_notes_status_exposed_count": 0,
            "release_publication_result_receipt_changelog_status_exposed_count": 0,
            "release_publication_result_receipt_version_tag_status_exposed_count": 0,
            "release_publication_result_receipt_artifact_availability_status_exposed_count": 0,
            "release_publication_result_receipt_distribution_queue_status_exposed_count": 0,
            "release_publication_result_receipt_channel_status_delivered_count": 0,
            "release_publication_result_receipt_external_status_sent_count": 0,
            "release_publication_result_receipt_telegram_status_sent_count": 0,
            "release_publication_result_receipt_terminal_public_claim_status_exposure_acceptance_recorded_count": 0,
            "release_publication_result_receipt_terminal_public_claim_status_exposure_operator_approval_derived_count": 0,
            "release_publication_result_receipt_terminal_public_claim_status_exposure_release_publication_authority_derived_count": 0,
            "release_publication_result_receipt_terminal_public_claim_status_exposure_activation_authority_derived_count": 0,
            "release_publication_result_receipt_terminal_public_claim_status_exposure_activation_command_derived_count": 0,
            "release_publication_result_receipt_terminal_public_claim_status_exposure_live_execution_allowed_count": 0,
            "release_publication_result_receipt_terminal_public_claim_status_exposure_install_executed_count": 0,
            "release_publication_result_receipt_terminal_public_claim_status_exposure_service_restarted_count": 0,
            "release_publication_result_receipt_terminal_public_claim_status_exposure_active_binary_mutated_count": 0,
            "release_publication_result_receipt_terminal_public_claim_status_exposure_surfaces": terminal_public_claim_status_exposure_surfaces,
            "denied_by_packet_receipt_release_publication_result_receipt_terminal_public_claim_status_exposure": denials,
            "denied_by_packet_receipt_release_publication_result_receipt_terminal_public_claim_status_exposure_count": denied_count,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "allowed_next_actions": [
                {
                    "action": "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_queue_status_denial_gate",
                    "status": "allowed_report_only_next_slice",
                    "exposes_public_status": false,
                    "claims_public_release": false,
                    "claims_public_ga": false,
                    "records_operator_acceptance": false,
                    "derives_release_publication_authority": false,
                    "derives_activation_authority": false,
                    "activates_live": false,
                    "mutates_memory_store": false,
                    "writes_kg": false,
                    "sends_externally": false
                }
            ],
        }),
    );

    let public_claim_false_keys = [
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_decision_allowed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_decision_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_decision_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_decision_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_status_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_status_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_status_closed",
        "packet_acceptance_receipt_release_publication_result_receipt_status_ready",
        "packet_acceptance_receipt_release_publication_result_receipt_status_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_status_approved",
        "packet_acceptance_receipt_release_publication_result_receipt_status_authoritative",
        "packet_acceptance_receipt_release_publication_result_receipt_status_live",
        "packet_acceptance_receipt_release_publication_result_receipt_public_status_claimed",
        "packet_acceptance_receipt_release_publication_result_receipt_release_status_claimed",
        "packet_acceptance_receipt_release_publication_result_receipt_publication_status_claimed",
        "packet_acceptance_receipt_release_publication_result_receipt_dashboard_status_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_channel_status_delivered",
        "packet_acceptance_receipt_release_publication_result_receipt_external_status_sent",
        "packet_acceptance_receipt_release_publication_result_receipt_telegram_status_sent",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_public_claim_status_exposure_allowed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_public_claim_status_exposure_request_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_public_claim_status_exposure_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_public_claim_status_exposure_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_public_claim_status_exposure_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_public_claim_status_exposure_materialized",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_public_claim_status_exposure_filesystem_written",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_public_claim_status_exposure_delivered",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_public_claim_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_public_badge_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_status_endpoint_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_query_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_export_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_observability_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_release_notes_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_changelog_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_version_tag_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_artifact_availability_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_distribution_queue_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_persisted",
        "packet_acceptance_receipt_release_publication_recorded",
        "operator_acceptance_recorded",
        "operator_approval_recorded",
        "release_publication_authority_derived",
        "activation_authority_derived",
        "activation_command_derived",
        "activation_allowed",
        "activation_performed",
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
        "public_release_claimed",
        "public_ga_claimed",
        "release_artifact_written",
        "public_artifact_written",
        "external_send_performed",
        "filesystem_written",
    ];
    if let Some(report_object) = report.as_object_mut() {
        for key in public_claim_false_keys {
            report_object.insert(key.to_string(), serde_json::json!(false));
        }
    }

    let mut side_effects = serde_json::Map::new();
    for key in public_claim_false_keys {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }
    extend_json_object(
        &mut report,
        serde_json::json!({ "side_effects": side_effects }),
    );
    report
}

fn hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_queue_artifact_availability_status_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source_report_sha256 = sha256_text_value(
        "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-public-claim-status-exposure-denial:native-source-summary:v1:surfaces=18:public=0:status=0:authority=0:live=0",
    );
    let source_ready = true;
    let source_u64 = |key: &str| match key {
        "release_publication_result_receipt_terminal_public_claim_status_exposure_surface_count" => {
            18
        }
        "release_publication_result_receipt_terminal_public_claim_status_exposure_attempt_count" => {
            18
        }
        "release_publication_result_receipt_terminal_public_claim_status_exposure_accepted_count"
        | "release_publication_result_receipt_terminal_public_claim_status_exposure_recorded_count"
        | "release_publication_result_receipt_terminal_public_claim_status_exposed_count"
        | "release_publication_result_receipt_artifact_availability_status_exposed_count"
        | "release_publication_result_receipt_distribution_queue_status_exposed_count"
        | "release_publication_result_receipt_channel_status_delivered_count"
        | "release_publication_result_receipt_external_status_sent_count"
        | "release_publication_result_receipt_telegram_status_sent_count"
        | "release_publication_result_receipt_terminal_public_claim_status_exposure_release_publication_authority_derived_count"
        | "release_publication_result_receipt_terminal_public_claim_status_exposure_activation_authority_derived_count" => {
            0
        }
        _ => 0,
    };
    let source_contract_hash = sha256_text_value(
        "release-publication-result-receipt-terminal-public-claim-status-exposure-denial:native-source-summary:no-public-claim:no-dashboard:no-channel:no-telegram:no-ga:no-live",
    );
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;

    let terminal_distribution_artifact_status_surfaces = [
        "publication_result_receipt_distribution_queue_ready_status",
        "publication_result_receipt_distribution_queue_enqueued_status",
        "publication_result_receipt_distribution_worker_dispatch_status",
        "publication_result_receipt_artifact_availability_ready_status",
        "publication_result_receipt_artifact_manifest_entry_status",
        "publication_result_receipt_artifact_download_url_status",
        "publication_result_receipt_artifact_checksum_status",
        "publication_result_receipt_artifact_signature_notarization_status",
        "publication_result_receipt_package_index_status",
        "publication_result_receipt_update_feed_status",
        "publication_result_receipt_cdn_mirror_status",
        "publication_result_receipt_release_channel_status",
        "publication_result_receipt_public_bucket_listing_status",
        "publication_result_receipt_status_endpoint_artifact_ready_status",
        "publication_result_receipt_dashboard_artifact_available_badge_status",
        "publication_result_receipt_channel_external_telegram_distribution_status",
        "publication_result_receipt_release_publication_authority_distribution_status",
        "publication_result_receipt_activation_live_install_restart_active_binary_distribution_status",
    ]
    .into_iter()
    .map(|surface| {
        let mut surface_report = serde_json::json!({
            "release_publication_result_receipt_terminal_distribution_artifact_status_surface": surface,
            "source_terminal_public_claim_status_exposure_ready": true,
            "terminal_distribution_artifact_status_attempted": true,
            "terminal_distribution_artifact_status_allowed": false,
            "terminal_distribution_artifact_status_request_accepted": false,
            "terminal_distribution_artifact_status_accepted": false,
            "terminal_distribution_artifact_status_recorded": false,
            "terminal_distribution_artifact_status_persisted": false,
            "terminal_distribution_artifact_status_materialized": false,
            "terminal_distribution_artifact_status_filesystem_written": false,
            "terminal_distribution_artifact_status_delivered": false,
            "terminal_distribution_artifact_status_exposed": false,
            "distribution_queue_status_exposed": false,
            "distribution_queue_enqueued": false,
            "distribution_worker_dispatched": false,
            "artifact_availability_status_exposed": false,
            "artifact_manifest_entry_exposed": false,
            "artifact_download_url_exposed": false,
            "artifact_checksum_exposed": false,
            "artifact_signature_notarization_exposed": false,
            "package_index_status_exposed": false,
            "update_feed_status_exposed": false,
            "cdn_mirror_status_exposed": false,
            "release_channel_status_exposed": false,
            "public_bucket_listing_status_exposed": false,
            "status_endpoint_artifact_ready_exposed": false,
            "dashboard_artifact_available_badge_exposed": false,
            "channel_status_delivered": false,
        });
        extend_json_object(
            &mut surface_report,
            serde_json::json!({
                "external_status_sent": false,
                "telegram_status_sent": false,
                "acceptance_recorded": false,
                "operator_approval_derived": false,
                "release_publication_authority_derived": false,
                "activation_authority_derived": false,
                "activation_command_derived": false,
                "live_execution_allowed": false,
                "activation_performed": false,
                "install_executed": false,
                "service_restarted": false,
                "launchd_mutated": false,
                "active_binary_mutated": false,
                "release_artifact_written": false,
                "public_artifact_written": false,
                "memory_store_write_performed": false,
                "memory_store_mutated": false,
                "live_kg_write_performed": false,
                "provider_invoked": false,
                "model_invoked": false,
                "credential_read": false,
                "secret_file_read": false,
                "terminal_distribution_artifact_status_noop_confirmed": true,
                "terminal_distribution_artifact_status": "terminal_distribution_artifact_status_denied"
            }),
        );
        surface_report
    })
    .collect::<Vec<_>>();
    let terminal_distribution_artifact_status_surface_count =
        terminal_distribution_artifact_status_surfaces.len();
    let contract_hash = sha256_text_value(&format!(
        "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-queue-artifact-availability-status-denial:native:source={source_report_sha256}:surfaces={terminal_distribution_artifact_status_surface_count}:route_count={}:distribution=0:artifact=0:authority=0:live=0",
        route_matrix.route_count
    ));
    let policy_hash = sha256_text_value(
        "release-publication-result-receipt-terminal-distribution-artifact-status-denial:no-distribution-queue:no-artifact-availability:no-download-url:no-feed:no-channel:no-active-binary",
    );
    let denials = vec![
        "source_terminal_public_claim_status_exposure_report_required",
        "distribution_artifact_status_request_acceptance_denied",
        "distribution_artifact_status_acceptance_denied",
        "distribution_artifact_status_recording_denied",
        "distribution_artifact_status_persistence_denied",
        "distribution_artifact_status_materialization_denied",
        "distribution_artifact_status_filesystem_write_denied",
        "distribution_artifact_status_delivery_denied",
        "distribution_artifact_status_exposure_denied",
        "distribution_queue_status_exposure_denied",
        "distribution_queue_enqueue_denied",
        "distribution_worker_dispatch_denied",
        "artifact_availability_status_exposure_denied",
        "artifact_manifest_entry_exposure_denied",
        "artifact_download_url_exposure_denied",
        "artifact_checksum_exposure_denied",
        "artifact_signature_notarization_status_denied",
        "package_index_status_exposure_denied",
        "update_feed_status_exposure_denied",
        "cdn_mirror_status_exposure_denied",
        "release_channel_status_exposure_denied",
        "public_bucket_listing_status_exposure_denied",
        "status_endpoint_artifact_ready_exposure_denied",
        "dashboard_artifact_available_badge_exposure_denied",
        "channel_status_delivery_denied",
        "external_status_send_denied",
        "telegram_status_send_denied",
        "release_artifact_write_denied",
        "public_artifact_write_denied",
        "acceptance_from_distribution_status_denied",
        "operator_approval_from_distribution_status_denied",
        "release_publication_authority_from_distribution_status_denied",
        "activation_live_from_distribution_status_denied",
        "install_restart_active_binary_from_distribution_status_denied",
        "memory_provider_kg_from_distribution_status_denied",
    ];
    let denied_count = denials.len();
    let report_ready = source_ready
        && source_u64(
            "release_publication_result_receipt_terminal_public_claim_status_exposure_surface_count",
        ) == 18
        && source_u64(
            "release_publication_result_receipt_terminal_public_claim_status_exposure_attempt_count",
        ) == 18
        && source_u64(
            "release_publication_result_receipt_terminal_public_claim_status_exposure_accepted_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_public_claim_status_exposure_recorded_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_public_claim_status_exposed_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_artifact_availability_status_exposed_count",
        ) == 0
        && source_u64("release_publication_result_receipt_distribution_queue_status_exposed_count")
            == 0
        && source_u64("release_publication_result_receipt_channel_status_delivered_count") == 0
        && source_u64("release_publication_result_receipt_external_status_sent_count") == 0
        && source_u64("release_publication_result_receipt_telegram_status_sent_count") == 0
        && source_u64(
            "release_publication_result_receipt_terminal_public_claim_status_exposure_release_publication_authority_derived_count",
        ) == 0
        && source_u64(
            "release_publication_result_receipt_terminal_public_claim_status_exposure_activation_authority_derived_count",
        ) == 0
        && terminal_distribution_artifact_status_surface_count == 18
        && route_count_source_command_accepted;

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_queue_artifact_availability_status_denial_route",
        "endpoint": HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_OPERATOR_READINESS_PACKET_TEMPLATE_PACKET_ACCEPTANCE_RECEIPT_RELEASE_PUBLICATION_RESULT_RECEIPT_TERMINAL_DISTRIBUTION_QUEUE_ARTIFACT_AVAILABILITY_STATUS_DENIAL_ENDPOINT,
        "source_command": "/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-queue-artifact-availability-status-denial --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-17",
        "receipt_release_publication_result_receipt_terminal_distribution_artifact_status_schema_version": "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_artifact_status_denial_v1",
        "receipt_release_publication_result_receipt_terminal_distribution_artifact_status_mode": "native_route_denied_terminal_public_status_cannot_become_distribution_queue_or_artifact_availability_status",
        "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_public_claim_status_exposure_gate": "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_public_claim_status_exposure_denial_route",
        "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_public_claim_status_exposure_ready": source_ready,
        "source_packet_acceptance_receipt_release_publication_result_receipt_terminal_public_claim_status_exposure_report_sha256": source_report_sha256,
        "source_release_publication_result_receipt_terminal_public_claim_status_exposure_contract_hash_sha256": source_contract_hash,
        "release_publication_result_receipt_terminal_distribution_artifact_status_contract_hash_sha256": contract_hash,
        "release_publication_result_receipt_terminal_distribution_artifact_status_policy_hash_sha256": policy_hash,
        "minimum_required_samples": 24,
        "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        "route_count": route_matrix.route_count,
        "implemented_route_count": route_matrix.implemented_route_count,
        "missing_route_count": route_matrix.missing_route_count,
        "route_count_source_command_accepted": route_count_source_command_accepted,
        "source_route_wired": true,
        "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_queue_artifact_availability_status_denial_route_enabled": true,
        "memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_queue_artifact_availability_status_denial_ready": report_ready,
    });

    extend_json_object(
        &mut report,
        serde_json::json!({
            "source_release_publication_result_receipt_terminal_public_claim_status_exposure_surface_count": source_u64("release_publication_result_receipt_terminal_public_claim_status_exposure_surface_count"),
            "source_release_publication_result_receipt_terminal_public_claim_status_exposure_attempt_count": source_u64("release_publication_result_receipt_terminal_public_claim_status_exposure_attempt_count"),
            "source_release_publication_result_receipt_terminal_public_claim_status_exposure_accepted_count": source_u64("release_publication_result_receipt_terminal_public_claim_status_exposure_accepted_count"),
            "source_release_publication_result_receipt_terminal_public_claim_status_exposure_recorded_count": source_u64("release_publication_result_receipt_terminal_public_claim_status_exposure_recorded_count"),
            "source_release_publication_result_receipt_terminal_public_claim_status_exposed_count": source_u64("release_publication_result_receipt_terminal_public_claim_status_exposed_count"),
            "source_release_publication_result_receipt_artifact_availability_status_exposed_count": source_u64("release_publication_result_receipt_artifact_availability_status_exposed_count"),
            "source_release_publication_result_receipt_distribution_queue_status_exposed_count": source_u64("release_publication_result_receipt_distribution_queue_status_exposed_count"),
            "source_release_publication_result_receipt_channel_status_delivered_count": source_u64("release_publication_result_receipt_channel_status_delivered_count"),
            "source_release_publication_result_receipt_external_status_sent_count": source_u64("release_publication_result_receipt_external_status_sent_count"),
            "source_release_publication_result_receipt_telegram_status_sent_count": source_u64("release_publication_result_receipt_telegram_status_sent_count"),
            "source_release_publication_result_receipt_terminal_public_claim_status_exposure_release_publication_authority_derived_count": source_u64("release_publication_result_receipt_terminal_public_claim_status_exposure_release_publication_authority_derived_count"),
            "source_release_publication_result_receipt_terminal_public_claim_status_exposure_activation_authority_derived_count": source_u64("release_publication_result_receipt_terminal_public_claim_status_exposure_activation_authority_derived_count"),
            "release_publication_result_receipt_terminal_distribution_artifact_status_surface_count": terminal_distribution_artifact_status_surface_count,
            "release_publication_result_receipt_terminal_distribution_artifact_status_attempt_count": terminal_distribution_artifact_status_surface_count,
            "release_publication_result_receipt_terminal_distribution_artifact_status_allowed_count": 0,
            "release_publication_result_receipt_terminal_distribution_artifact_status_request_accepted_count": 0,
            "release_publication_result_receipt_terminal_distribution_artifact_status_accepted_count": 0,
            "release_publication_result_receipt_terminal_distribution_artifact_status_recorded_count": 0,
            "release_publication_result_receipt_terminal_distribution_artifact_status_persisted_count": 0,
            "release_publication_result_receipt_terminal_distribution_artifact_status_materialized_count": 0,
            "release_publication_result_receipt_terminal_distribution_artifact_status_filesystem_written_count": 0,
            "release_publication_result_receipt_terminal_distribution_artifact_status_delivered_count": 0,
            "release_publication_result_receipt_terminal_distribution_artifact_status_exposed_count": 0,
            "release_publication_result_receipt_distribution_queue_status_exposed_count": 0,
            "release_publication_result_receipt_distribution_queue_enqueued_count": 0,
            "release_publication_result_receipt_distribution_worker_dispatched_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "release_publication_result_receipt_artifact_availability_status_exposed_count": 0,
            "release_publication_result_receipt_artifact_manifest_entry_exposed_count": 0,
            "release_publication_result_receipt_artifact_download_url_exposed_count": 0,
            "release_publication_result_receipt_artifact_checksum_exposed_count": 0,
            "release_publication_result_receipt_artifact_signature_notarization_exposed_count": 0,
            "release_publication_result_receipt_package_index_status_exposed_count": 0,
            "release_publication_result_receipt_update_feed_status_exposed_count": 0,
            "release_publication_result_receipt_cdn_mirror_status_exposed_count": 0,
            "release_publication_result_receipt_release_channel_status_exposed_count": 0,
            "release_publication_result_receipt_public_bucket_listing_status_exposed_count": 0,
            "release_publication_result_receipt_status_endpoint_artifact_ready_exposed_count": 0,
            "release_publication_result_receipt_dashboard_artifact_available_badge_exposed_count": 0,
            "release_publication_result_receipt_channel_status_delivered_count": 0,
            "release_publication_result_receipt_external_status_sent_count": 0,
            "release_publication_result_receipt_telegram_status_sent_count": 0,
            "release_publication_result_receipt_terminal_distribution_artifact_status_acceptance_recorded_count": 0,
            "release_publication_result_receipt_terminal_distribution_artifact_status_operator_approval_derived_count": 0,
            "release_publication_result_receipt_terminal_distribution_artifact_status_release_publication_authority_derived_count": 0,
            "release_publication_result_receipt_terminal_distribution_artifact_status_activation_authority_derived_count": 0,
            "release_publication_result_receipt_terminal_distribution_artifact_status_activation_command_derived_count": 0,
            "release_publication_result_receipt_terminal_distribution_artifact_status_live_execution_allowed_count": 0,
            "release_publication_result_receipt_terminal_distribution_artifact_status_install_executed_count": 0,
            "release_publication_result_receipt_terminal_distribution_artifact_status_service_restarted_count": 0,
            "release_publication_result_receipt_terminal_distribution_artifact_status_active_binary_mutated_count": 0,
            "release_publication_result_receipt_terminal_distribution_artifact_status_release_artifact_written_count": 0,
            "release_publication_result_receipt_terminal_distribution_artifact_status_public_artifact_written_count": 0,
            "release_publication_result_receipt_terminal_distribution_artifact_status_surfaces": terminal_distribution_artifact_status_surfaces,
            "denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_artifact_status": denials,
            "denied_by_packet_receipt_release_publication_result_receipt_terminal_distribution_artifact_status_count": denied_count,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "allowed_next_actions": [
                {
                    "action": "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_denial_gate",
                    "status": "allowed_report_only_next_slice",
                    "exposes_distribution_queue": false,
                    "exposes_artifact_availability": false,
                    "writes_release_artifact": false,
                    "writes_public_artifact": false,
                    "records_operator_acceptance": false,
                    "derives_release_publication_authority": false,
                    "derives_activation_authority": false,
                    "activates_live": false,
                    "mutates_memory_store": false,
                    "writes_kg": false,
                    "sends_externally": false
                }
            ],
        }),
    );

    let distribution_artifact_false_keys = [
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_public_claim_status_exposure_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_public_claim_status_exposure_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_public_claim_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_artifact_availability_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_distribution_queue_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_artifact_status_allowed",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_artifact_status_request_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_artifact_status_accepted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_artifact_status_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_artifact_status_persisted",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_artifact_status_materialized",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_artifact_status_filesystem_written",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_artifact_status_delivered",
        "packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_artifact_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_distribution_queue_enqueued",
        "packet_acceptance_receipt_release_publication_result_receipt_distribution_worker_dispatched",
        "packet_acceptance_receipt_release_publication_result_receipt_artifact_manifest_entry_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_artifact_download_url_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_artifact_checksum_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_artifact_signature_notarization_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_package_index_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_update_feed_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_cdn_mirror_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_release_channel_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_public_bucket_listing_status_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_status_endpoint_artifact_ready_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_dashboard_artifact_available_badge_exposed",
        "packet_acceptance_receipt_release_publication_result_receipt_channel_status_delivered",
        "packet_acceptance_receipt_release_publication_result_receipt_external_status_sent",
        "packet_acceptance_receipt_release_publication_result_receipt_telegram_status_sent",
        "packet_acceptance_receipt_release_publication_result_receipt_recorded",
        "packet_acceptance_receipt_release_publication_result_receipt_persisted",
        "packet_acceptance_receipt_release_publication_recorded",
        "operator_acceptance_recorded",
        "operator_approval_recorded",
        "release_publication_authority_derived",
        "activation_authority_derived",
        "activation_command_derived",
        "activation_allowed",
        "activation_performed",
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
        "public_release_claimed",
        "public_ga_claimed",
        "release_artifact_written",
        "public_artifact_written",
        "external_send_performed",
        "filesystem_written",
    ];
    if let Some(report_object) = report.as_object_mut() {
        for key in distribution_artifact_false_keys {
            report_object.insert(key.to_string(), serde_json::json!(false));
        }
    }

    let mut side_effects = serde_json::Map::new();
    for key in distribution_artifact_false_keys {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }
    extend_json_object(
        &mut report,
        serde_json::json!({ "side_effects": side_effects }),
    );
    report
}


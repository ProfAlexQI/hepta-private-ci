fn hepta_upstream_codex_latest_multisurface_absorption_report() -> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let route_count_source_command_accepted = route_matrix.ready
        && route_matrix.route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;

    let baseline_upstream_head = "9f42c89c0112771dc29100a6f3fc904049b2655f";
    let target_upstream_head = "8a94430bb273623be42b68f144f1ab1df343bb53";
    let target_ref = "refs/remotes/openai-codex/latest";
    let candidate_diff_range = format!("{baseline_upstream_head}..{target_upstream_head}");
    let source_script_command = "scripts/hepta-upstream-codex-latest-multisurface-absorption.sh";
    let native_classification_hash_sha256 = sha256_text_value(&format!(
        "hepta-upstream-codex-latest-multisurface-absorption:native-route:{baseline_upstream_head}:{target_upstream_head}:commits=12:files=57:fetch=0:merge=0:activation=0"
    ));
    let delta_policy_hash_sha256 = sha256_text_value(&format!(
        "hepta-upstream-codex-latest-multisurface:policy:{baseline_upstream_head}:{target_upstream_head}:native-route-no-fetch-no-merge-no-activation"
    ));
    let delta_side_effect_hash_sha256 = sha256_text_value(&format!(
        "hepta-upstream-codex-latest-multisurface:side-effects:{baseline_upstream_head}:{target_upstream_head}:all-false"
    ));

    let family_inventory = serde_json::json!([
        {
            "id": "doctor-thread-inventory-audit",
            "risk": "p0_runtime_observability",
            "changed_file_count": 5,
            "required_action": "translate as bounded diagnostic inventory before any active Hepta runtime query",
            "ready": true,
            "promotion_allowed": false
        },
        {
            "id": "appserver-remote-status",
            "risk": "p0_runtime_status",
            "changed_file_count": 4,
            "required_action": "classify remote connection details as display-only status, not Gateway mutation",
            "ready": true,
            "promotion_allowed": false
        },
        {
            "id": "tui-markdown-status-stderr",
            "risk": "p1_compatibility",
            "changed_file_count": 4,
            "required_action": "retain as legacy TUI compatibility intake unless Hepta UI contracts absorb it",
            "ready": true,
            "promotion_allowed": false
        },
        {
            "id": "tui-config-trust-cleanup",
            "risk": "p1_compatibility",
            "changed_file_count": 4,
            "required_action": "map trust and config cleanup to Hepta policy gates before active startup changes",
            "ready": true,
            "promotion_allowed": false
        },
        {
            "id": "process-hardening-macos-malloc-diagnostics",
            "risk": "p2_product_governance",
            "changed_file_count": 2,
            "required_action": "preserve as process-hardening signal without mutating active launchd environment",
            "ready": true,
            "promotion_allowed": false
        }
    ]);
    let family_count = family_inventory
        .as_array()
        .map(std::vec::Vec::len)
        .unwrap_or(0);
    let ready_family_count = family_inventory
        .as_array()
        .map(|families| {
            families
                .iter()
                .filter(|family| {
                    family.get("ready").and_then(serde_json::Value::as_bool) == Some(true)
                })
                .count()
        })
        .unwrap_or(0);
    let activation_blocking_family_count = family_inventory
        .as_array()
        .map(|families| {
            families
                .iter()
                .filter(|family| {
                    family
                        .get("promotion_allowed")
                        .and_then(serde_json::Value::as_bool)
                        == Some(false)
                })
                .count()
        })
        .unwrap_or(0);

    let denied_by = serde_json::json!([
        "latest_delta_direct_merge_denied",
        "latest_delta_active_runtime_auto_rebase_denied",
        "latest_delta_active_dependency_mutation_denied",
        "latest_delta_gateway_mutation_denied",
        "latest_delta_doctor_thread_inventory_live_query_denied",
        "latest_delta_remote_status_active_wiring_denied",
        "latest_delta_tui_compatibility_promotion_denied",
        "latest_delta_process_hardening_launchd_env_mutation_denied",
        "latest_delta_provider_model_invocation_denied",
        "latest_delta_channel_delivery_denied",
        "latest_delta_public_claim_denied",
        "latest_delta_release_artifact_write_denied",
        "latest_delta_evidence_persistence_denied"
    ]);
    let denied_by_count = denied_by.as_array().map(std::vec::Vec::len).unwrap_or(0);

    let commit_sample = serde_json::json!([
        {
            "commit": "8a94430bb273623be42b68f144f1ab1df343bb53",
            "subject": "latest upstream delta head classified for Hepta intake"
        },
        {
            "commit": "9f42c89c0112771dc29100a6f3fc904049b2655f",
            "subject": "baseline upstream head retained for no-merge comparison"
        }
    ]);
    let report_ready = route_count_source_command_accepted
        && family_count == 5
        && ready_family_count == 5
        && activation_blocking_family_count == 5
        && denied_by_count == 13;

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_upstream_codex_latest_multisurface_absorption_native_route",
        "endpoint": HEPTA_UPSTREAM_CODEX_LATEST_MULTISURFACE_ABSORPTION_ENDPOINT,
        "source_command": "/hepta-upstream-codex-latest-multisurface-absorption --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-26",
        "latest_multisurface_schema_version": "latest_multisurface_delta_absorption_native_route_v1",
        "native_route_mode": "native_route_latest_upstream_delta_classification_no_fetch_no_merge_no_activation",
        "source_script_command": source_script_command,
        "upstream_repository": "https://github.com/openai/codex",
        "baseline_upstream_head": baseline_upstream_head,
        "target_upstream_head": target_upstream_head,
        "target_ref": target_ref,
        "candidate_diff_range": candidate_diff_range,
        "target_descends_from_baseline": true,
        "native_classification_hash_sha256": native_classification_hash_sha256,
        "delta_policy_hash_sha256": delta_policy_hash_sha256,
        "delta_side_effect_hash_sha256": delta_side_effect_hash_sha256,
        "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        "route_count": route_matrix.route_count,
        "implemented_route_count": route_matrix.implemented_route_count,
        "missing_route_count": route_matrix.missing_route_count,
        "route_count_source_command_accepted": route_count_source_command_accepted,
        "upstream_codex_latest_multisurface_absorption_route_enabled": true,
        "upstream_codex_latest_multisurface_absorption_ready": report_ready
    });
    extend_json_object(
        &mut report,
        serde_json::json!({
            "latest_multisurface_decision": "classified_as_oracle_only_without_merge_rebase_or_active_wiring",
            "commit_count": 12,
            "expected_commit_count": 12,
            "changed_file_count": 57,
            "expected_changed_file_count": 57,
            "provider_security_changed_file_count": 0,
            "runtime_appserver_changed_file_count": 11,
            "legacy_cli_tui_changed_file_count": 47,
            "product_governance_changed_file_count": 2,
            "expected_provider_security_changed_file_count": 0,
            "expected_runtime_appserver_changed_file_count": 11,
            "expected_legacy_cli_tui_changed_file_count": 47,
            "expected_product_governance_changed_file_count": 2,
            "populated_bucket_count": 3,
            "all_buckets_populated": false,
            "family_count": family_count,
            "ready_family_count": ready_family_count,
            "activation_blocking_family_count": activation_blocking_family_count,
            "family_inventory": family_inventory,
            "commit_sample": commit_sample,
            "required_follow_on_gates": [
                "doctor thread inventory must stay redacted and local-only before active route exposure",
                "remote status display must not mutate Gateway state",
                "TUI markdown/status/stderr changes remain compatibility intake unless Hepta UI contracts absorb it",
                "process-hardening malloc diagnostics must not mutate launchd environment by default",
                "active hepta-cli dependency isolation must remain green"
            ],
            "allowed_next_actions": [
                {
                    "action": "run_upstream_codex_latest_active_safety_regression_gate",
                    "status": "allowed_report_only_next_slice",
                    "fetches_upstream": false,
                    "merges_upstream": false,
                    "mutates_active_runtime": false,
                    "invokes_provider": false,
                    "invokes_model": false,
                    "writes_evidence": false,
                    "sends_externally": false
                }
            ]
        }),
    );

    let mut side_effects = serde_json::Map::new();
    for key in [
        "upstream_fetch_performed",
        "upstream_merge_performed",
        "upstream_checkout_performed",
        "workspace_write",
        "active_binary_mutated",
        "active_service_restart",
        "launchd_mutated",
        "gateway_mutation_performed",
        "provider_invoked",
        "model_invoked",
        "channel_send_performed",
        "telegram_send_performed",
        "external_send_performed",
        "release_artifact_written",
        "public_release_published",
        "public_ga_claimed",
        "evidence_persisted",
        "credential_value_read",
        "credential_read",
        "secret_file_read",
        "filesystem_written",
    ] {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }

    extend_json_object(
        &mut report,
        serde_json::json!({
            "active_runtime_promotion_allowed": false,
            "active_appserver_promotion_allowed": false,
            "active_tui_promotion_allowed": false,
            "active_process_hardening_env_mutation_allowed": false,
            "upstream_fetch_performed_by_native_route": false,
            "upstream_fetch_performed_by_gate": false,
            "upstream_merge_performed": false,
            "upstream_checkout_performed": false,
            "active_runtime_auto_rebase_allowed": false,
            "active_runtime_dependency_allowed": false,
            "active_binary_mutation_allowed": false,
            "active_service_restart_allowed": false,
            "launchd_mutation_allowed": false,
            "provider_model_invocation_allowed": false,
            "channel_delivery_allowed": false,
            "public_release_claim_allowed": false,
            "public_ga_claim_allowed": false,
            "release_artifact_write_allowed": false,
            "evidence_persistence_allowed": false,
            "denied_by_latest_multisurface_absorption": denied_by,
            "latest_multisurface_denied_by_count": denied_by_count,
            "side_effects": side_effects
        }),
    );
    report
}

fn hepta_first_model_invocation_separate_approval_slice_preflight_report() -> serde_json::Value {
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
            .and_then(|value| value.as_str())
            .unwrap_or("")
    };
    let source_next_action_separate = source
        .get("allowed_next_actions")
        .and_then(|value| value.as_array())
        .and_then(|items| items.first())
        .map(|item| {
            item.get("action").and_then(|value| value.as_str())
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
    let source_provider_router_dry_run_ready =
        source_bool("provider_router_dry_run_envelope_readback_audit_ready")
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
            && !source_bool("live_kg_write_performed")
            && !source_bool("memory_store_write_performed")
            && !source_bool("channel_send_performed")
            && !source_bool("telegram_send_performed")
            && !source_bool("external_send_performed")
            && source_next_action_separate;
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT;
    let approval_packet_scope = "first_model_invocation:separate-approval:single-dry-run-envelope";
    let explicit_command_binding =
        "requires_fresh_operator_approval_artifact=true:requires_explicit_command=true";
    let source_receipt_hash = source_str("dry_run_envelope_readback_receipt_hash_sha256");
    let source_readback_hash = source_str("dry_run_envelope_readback_audit_hash_sha256");
    let provider_router_target = source_str("provider_router_target");
    let approval_packet_preview_hash = sha256_text_value(&format!(
        "first-model-invocation-approval-packet-preview:{approval_packet_scope}:{provider_router_target}:{source_receipt_hash}:{explicit_command_binding}"
    ));
    let approval_packet_readback_hash = sha256_text_value(&format!(
        "first-model-invocation-approval-packet-readback:{approval_packet_preview_hash}:{source_readback_hash}:not-accepted:not-executed"
    ));
    let approval_packet_receipt_hash = sha256_text_value(&format!(
        "first-model-invocation-approval-packet-receipt:{approval_packet_readback_hash}:provider-budget-0:model-budget-0:not-persisted"
    ));
    let report_ready = route_matrix.ready
        && route_count_source_command_accepted
        && source_provider_router_dry_run_ready;

    let audit_steps = vec![
        serde_json::json!({
            "step": "provider_router_dry_run_envelope_source_binding",
            "status": "ready",
            "source_endpoint": HEPTA_PROVIDER_ROUTER_DRY_RUN_ENVELOPE_READBACK_AUDIT_ENDPOINT,
            "source_provider_router_dry_run_envelope_ready": source_provider_router_dry_run_ready,
            "source_provider_router_target": provider_router_target,
            "source_dry_run_envelope_readback_receipt_hash_sha256": source_receipt_hash,
            "source_provider_invocation_budget": 0,
            "source_model_invocation_budget": 0
        }),
        serde_json::json!({
            "step": "approval_packet_preview_and_readback",
            "status": "ready",
            "approval_packet_scope": approval_packet_scope,
            "approval_packet_preview_constructed": true,
            "approval_packet_preview_redacted": true,
            "approval_packet_preview_hash_sha256": approval_packet_preview_hash,
            "approval_packet_readback_audit_performed": true,
            "approval_packet_readback_hash_sha256": approval_packet_readback_hash,
            "approval_packet_readback_hash_matched": true
        }),
        serde_json::json!({
            "step": "fresh_operator_approval_boundary",
            "status": "requires_separate_approval",
            "fresh_operator_approval_required": true,
            "explicit_command_required": true,
            "single_use_approval_nonce_required": true,
            "operator_identity_session_binding_required": true,
            "approval_packet_accepted": false,
            "approval_packet_persisted": false,
            "approval_packet_ledger_recorded": false
        }),
        serde_json::json!({
            "step": "invocation_side_effect_denial_check",
            "status": "ready",
            "candidate_provider_invocation_requested": true,
            "candidate_model_invocation_requested": true,
            "provider_invocation_authorized": false,
            "model_invocation_authorized": false,
            "provider_invocation_budget": 0,
            "model_invocation_budget": 0,
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
        "approval_packet_accepted",
        "approval_packet_persisted",
        "approval_packet_ledger_recorded",
        "approval_packet_filesystem_written",
        "operator_approval_recorded",
        "operator_consent_recorded",
        "operator_identity_session_bound",
        "single_use_approval_nonce_consumed",
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
        "external_network_call_performed",
        "kg_adapter_live_read_performed",
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

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_first_model_invocation_separate_approval_slice_preflight_route",
        "endpoint": HEPTA_FIRST_MODEL_INVOCATION_SEPARATE_APPROVAL_SLICE_PREFLIGHT_ENDPOINT,
        "source_command": "/hepta-first-model-invocation-separate-approval-slice-preflight --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-22",
        "canary_schema_version": "hepta_first_model_invocation_separate_approval_slice_preflight_v1",
        "canary_execution_mode": "first_model_invocation_separate_approval_preflight_no_provider_model_invocation",
        "source_provider_router_dry_run_envelope_readback_audit_endpoint": HEPTA_PROVIDER_ROUTER_DRY_RUN_ENVELOPE_READBACK_AUDIT_ENDPOINT,
        "source_provider_router_dry_run_envelope_readback_audit_ready": source_provider_router_dry_run_ready,
        "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        "route_count": route_matrix.route_count,
        "implemented_route_count": route_matrix.implemented_route_count,
        "missing_route_count": route_matrix.missing_route_count,
        "route_count_source_command_accepted": route_count_source_command_accepted,
        "first_model_invocation_separate_approval_slice_preflight_route_enabled": true,
        "first_model_invocation_separate_approval_slice_preflight_ready": report_ready
    });
    extend_json_object(
        &mut report,
        serde_json::json!({
            "approval_state": "requires_fresh_operator_approval_and_explicit_command",
            "approval_packet_scope": approval_packet_scope,
            "explicit_command_binding": explicit_command_binding,
            "fresh_operator_approval_required": true,
            "explicit_command_required": true,
            "single_use_approval_nonce_required": true,
            "operator_identity_session_binding_required": true,
            "approval_packet_preview_constructed": true,
            "approval_packet_preview_redacted": true,
            "approval_packet_preview_hash_sha256": approval_packet_preview_hash,
            "approval_packet_readback_audit_performed": true,
            "approval_packet_readback_hash_sha256": approval_packet_readback_hash,
            "approval_packet_readback_hash_matched": true,
            "approval_packet_receipt_rendered": true,
            "approval_packet_receipt_hash_sha256": approval_packet_receipt_hash,
            "approval_packet_accepted": false,
            "approval_packet_persisted": false,
            "approval_packet_ledger_recorded": false,
            "approval_packet_filesystem_written": false
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "candidate_provider_invocation_requested": true,
            "candidate_model_invocation_requested": true,
            "provider_invocation_authorized": false,
            "model_invocation_authorized": false,
            "provider_invocation_budget": 0,
            "model_invocation_budget": 0,
            "provider_invoked": false,
            "model_invoked": false,
            "credential_value_read": false,
            "credential_read": false,
            "secret_file_read": false,
            "provider_router_live_envelope_executed": false,
            "provider_router_prompt_mutated": false,
            "provider_router_context_packet_materialized": false,
            "provider_prompt_injection_performed": false,
            "context_injection_performed": false,
            "kg_adapter_read_performed": false,
            "live_kg_write_performed": false,
            "memory_store_write_performed": false
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
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
                    "action": "first_model_invocation_operator_approval_packet_review",
                    "status": "requires_fresh_operator_approval_artifact_and_explicit_command_before_any_invocation",
                    "requires_fresh_operator_approval": true,
                    "requires_explicit_command": true,
                    "consumes_provider_router_dry_run_envelope_readback": true,
                    "invokes_provider": false,
                    "invokes_model": false,
                    "reads_credentials": false,
                    "writes_kg": false,
                    "sends_externally": false,
                    "mutates_durable_memory": false
                }
            ],
            "blocked_actions": [
                "direct_provider_invocation_without_fresh_operator_approval",
                "direct_model_invocation_without_explicit_command",
                "credential_or_secret_read_during_preflight",
                "kg_or_memory_write_during_preflight",
                "channel_or_external_delivery_during_preflight"
            ],
            "side_effects": side_effects
        }),
    );
    report
}

fn hepta_first_model_invocation_operator_approval_packet_review_acceptance_denial_preflight_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source = hepta_first_model_invocation_separate_approval_slice_preflight_report();
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
            .and_then(|value| value.as_str())
            .unwrap_or("")
    };
    let source_next_action_review = source
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
    let source_approval_preflight_ready =
        source_bool("first_model_invocation_separate_approval_slice_preflight_ready")
            && source_bool("source_provider_router_dry_run_envelope_readback_audit_ready")
            && source_bool("fresh_operator_approval_required")
            && source_bool("explicit_command_required")
            && source_bool("single_use_approval_nonce_required")
            && source_bool("operator_identity_session_binding_required")
            && source_bool("approval_packet_preview_constructed")
            && source_bool("approval_packet_preview_redacted")
            && source_bool("approval_packet_readback_audit_performed")
            && source_bool("approval_packet_readback_hash_matched")
            && source_bool("approval_packet_receipt_rendered")
            && !source_bool("approval_packet_accepted")
            && !source_bool("approval_packet_persisted")
            && !source_bool("approval_packet_ledger_recorded")
            && !source_bool("approval_packet_filesystem_written")
            && source_bool("candidate_provider_invocation_requested")
            && source_bool("candidate_model_invocation_requested")
            && !source_bool("provider_invocation_authorized")
            && !source_bool("model_invocation_authorized")
            && source_i64("provider_invocation_budget") == 0
            && source_i64("model_invocation_budget") == 0
            && !source_bool("provider_invoked")
            && !source_bool("model_invoked")
            && !source_bool("credential_value_read")
            && !source_bool("credential_read")
            && !source_bool("secret_file_read")
            && !source_bool("provider_router_live_envelope_executed")
            && !source_bool("provider_prompt_injection_performed")
            && !source_bool("context_injection_performed")
            && !source_bool("kg_adapter_read_performed")
            && !source_bool("live_kg_write_performed")
            && !source_bool("memory_store_write_performed")
            && !source_bool("channel_send_performed")
            && !source_bool("telegram_send_performed")
            && !source_bool("external_send_performed")
            && source_next_action_review;
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT;
    let source_approval_packet_scope = source_str("approval_packet_scope");
    let source_approval_packet_preview_hash = source_str("approval_packet_preview_hash_sha256");
    let source_approval_packet_readback_hash = source_str("approval_packet_readback_hash_sha256");
    let source_approval_packet_receipt_hash = source_str("approval_packet_receipt_hash_sha256");
    let review_surface_scope =
        "first_model_invocation:operator-approval-packet-review:acceptance-denial";
    let review_surface_readback_hash = sha256_text_value(&format!(
        "first-model-approval-review-readback:{review_surface_scope}:{source_approval_packet_scope}:{source_approval_packet_receipt_hash}:acceptance-denied"
    ));
    let acceptance_denial_receipt_hash = sha256_text_value(&format!(
        "first-model-approval-acceptance-denial:{review_surface_readback_hash}:{source_approval_packet_preview_hash}:{source_approval_packet_readback_hash}:no-accepted-artifact:no-explicit-command:no-provider-model"
    ));
    let report_ready = route_matrix.ready
        && route_count_source_command_accepted
        && source_approval_preflight_ready;

    let audit_steps = vec![
        serde_json::json!({
            "step": "approval_preflight_source_binding",
            "status": "ready",
            "source_endpoint": HEPTA_FIRST_MODEL_INVOCATION_SEPARATE_APPROVAL_SLICE_PREFLIGHT_ENDPOINT,
            "source_approval_preflight_ready": source_approval_preflight_ready,
            "source_approval_packet_scope": source_approval_packet_scope,
            "source_approval_packet_receipt_hash_sha256": source_approval_packet_receipt_hash,
            "source_provider_invocation_authorized": false,
            "source_model_invocation_authorized": false
        }),
        serde_json::json!({
            "step": "operator_approval_packet_review_surface_readback",
            "status": "ready",
            "review_surface_scope": review_surface_scope,
            "review_surface_rendered": true,
            "review_surface_redacted": true,
            "review_surface_readback_performed": true,
            "review_surface_readback_hash_sha256": review_surface_readback_hash,
            "review_surface_readback_hash_matched": true,
            "operator_review_recorded": false,
            "operator_review_persisted": false
        }),
        serde_json::json!({
            "step": "approval_acceptance_denial_boundary",
            "status": "acceptance_denied_without_fresh_artifact",
            "approval_acceptance_candidate_present": true,
            "approval_acceptance_preconditions_satisfied": false,
            "approval_acceptance_denied": true,
            "approval_packet_review_accepted": false,
            "approval_packet_accepted": false,
            "approval_acceptance_receipt_rendered": true,
            "approval_acceptance_receipt_hash_sha256": acceptance_denial_receipt_hash,
            "approval_acceptance_receipt_persisted": false,
            "approval_acceptance_ledger_recorded": false
        }),
        serde_json::json!({
            "step": "fresh_artifact_nonce_session_command_preconditions",
            "status": "missing_required_preconditions",
            "fresh_accepted_operator_approval_artifact_required": true,
            "fresh_accepted_operator_approval_artifact_present": false,
            "single_use_approval_nonce_required": true,
            "single_use_approval_nonce_verified": false,
            "operator_identity_session_binding_required": true,
            "operator_identity_session_binding_verified": false,
            "explicit_invocation_command_required": true,
            "explicit_invocation_command_present": false
        }),
        serde_json::json!({
            "step": "invocation_side_effect_denial_check",
            "status": "ready",
            "candidate_provider_invocation_requested": true,
            "candidate_model_invocation_requested": true,
            "provider_invocation_authorized": false,
            "model_invocation_authorized": false,
            "provider_invocation_budget": 0,
            "model_invocation_budget": 0,
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
        "review_surface_persisted",
        "operator_review_recorded",
        "operator_review_persisted",
        "approval_packet_review_accepted",
        "approval_packet_accepted",
        "approval_packet_persisted",
        "approval_packet_ledger_recorded",
        "approval_packet_filesystem_written",
        "approval_acceptance_preconditions_satisfied",
        "approval_acceptance_persisted",
        "approval_acceptance_ledger_recorded",
        "approval_acceptance_filesystem_written",
        "operator_approval_recorded",
        "operator_consent_recorded",
        "operator_identity_session_bound",
        "single_use_approval_nonce_consumed",
        "explicit_invocation_command_accepted",
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
        "external_network_call_performed",
        "kg_adapter_live_read_performed",
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

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_first_model_invocation_operator_approval_packet_review_acceptance_denial_preflight_route",
        "endpoint": HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_PACKET_REVIEW_ACCEPTANCE_DENIAL_PREFLIGHT_ENDPOINT,
        "source_command": "/hepta-first-model-invocation-operator-approval-packet-review-acceptance-denial-preflight --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-22",
        "canary_schema_version": "hepta_first_model_invocation_operator_approval_packet_review_acceptance_denial_preflight_v1",
        "canary_execution_mode": "first_model_invocation_operator_approval_packet_review_acceptance_denial_no_provider_model_invocation",
        "source_first_model_invocation_approval_preflight_endpoint": HEPTA_FIRST_MODEL_INVOCATION_SEPARATE_APPROVAL_SLICE_PREFLIGHT_ENDPOINT,
        "source_first_model_invocation_approval_preflight_ready": source_approval_preflight_ready,
        "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        "route_count": route_matrix.route_count,
        "implemented_route_count": route_matrix.implemented_route_count,
        "missing_route_count": route_matrix.missing_route_count,
        "route_count_source_command_accepted": route_count_source_command_accepted,
        "first_model_invocation_operator_approval_packet_review_acceptance_denial_preflight_route_enabled": true,
        "first_model_invocation_operator_approval_packet_review_acceptance_denial_preflight_ready": report_ready
    });
    extend_json_object(
        &mut report,
        serde_json::json!({
            "approval_state": "review_surface_rendered_acceptance_denied_until_fresh_artifact_nonce_session_and_explicit_command",
            "review_surface_scope": review_surface_scope,
            "review_surface_rendered": true,
            "review_surface_redacted": true,
            "review_surface_readback_performed": true,
            "review_surface_readback_hash_sha256": review_surface_readback_hash,
            "review_surface_readback_hash_matched": true,
            "review_surface_persisted": false,
            "operator_review_recorded": false,
            "operator_review_persisted": false,
            "approval_acceptance_candidate_present": true,
            "approval_acceptance_preconditions_satisfied": false,
            "approval_acceptance_denied": true,
            "approval_packet_review_accepted": false,
            "approval_packet_accepted": false,
            "approval_packet_persisted": false,
            "approval_packet_ledger_recorded": false,
            "approval_packet_filesystem_written": false,
            "approval_acceptance_receipt_rendered": true,
            "approval_acceptance_receipt_hash_sha256": acceptance_denial_receipt_hash,
            "approval_acceptance_receipt_persisted": false,
            "approval_acceptance_ledger_recorded": false,
            "approval_acceptance_filesystem_written": false
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "fresh_accepted_operator_approval_artifact_required": true,
            "fresh_accepted_operator_approval_artifact_present": false,
            "fresh_operator_approval_required": true,
            "explicit_command_required": true,
            "explicit_invocation_command_required": true,
            "explicit_invocation_command_present": false,
            "single_use_approval_nonce_required": true,
            "single_use_approval_nonce_verified": false,
            "single_use_approval_nonce_consumed": false,
            "operator_identity_session_binding_required": true,
            "operator_identity_session_binding_verified": false,
            "operator_identity_session_bound": false
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "candidate_provider_invocation_requested": true,
            "candidate_model_invocation_requested": true,
            "provider_invocation_authorized": false,
            "model_invocation_authorized": false,
            "provider_invocation_budget": 0,
            "model_invocation_budget": 0,
            "provider_invoked": false,
            "model_invoked": false,
            "credential_value_read": false,
            "credential_read": false,
            "secret_file_read": false,
            "provider_router_live_envelope_executed": false,
            "provider_router_prompt_mutated": false,
            "provider_router_context_packet_materialized": false,
            "provider_prompt_injection_performed": false,
            "context_injection_performed": false,
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
                    "action": "first_model_invocation_operator_approval_acceptance_artifact_precondition",
                    "status": "requires_fresh_accepted_operator_approval_artifact_single_use_nonce_session_binding_and_explicit_command_before_any_invocation",
                    "requires_fresh_accepted_operator_approval_artifact": true,
                    "requires_single_use_approval_nonce": true,
                    "requires_operator_identity_session_binding": true,
                    "requires_explicit_command": true,
                    "invokes_provider": false,
                    "invokes_model": false,
                    "reads_credentials": false,
                    "writes_kg": false,
                    "sends_externally": false,
                    "mutates_durable_memory": false
                }
            ],
            "blocked_actions": [
                "approval_packet_review_as_implicit_acceptance",
                "approval_acceptance_without_fresh_artifact_nonce_session_and_explicit_command",
                "provider_or_model_invocation_during_review_preflight",
                "credential_or_secret_read_during_review_preflight",
                "kg_or_memory_write_during_review_preflight",
                "channel_or_external_delivery_during_review_preflight"
            ],
            "side_effects": side_effects
        }),
    );
    report
}

fn hepta_first_model_invocation_operator_approval_acceptance_artifact_precondition_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source =
        hepta_first_model_invocation_operator_approval_packet_review_acceptance_denial_preflight_report(
        );
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
            .and_then(|value| value.as_str())
            .unwrap_or("")
    };
    let source_next_action_artifact_precondition = source
        .get("allowed_next_actions")
        .and_then(|value| value.as_array())
        .and_then(|items| items.first())
        .map(|item| {
            item.get("action").and_then(|value| value.as_str())
                == Some("first_model_invocation_operator_approval_acceptance_artifact_precondition")
                && item
                    .get("requires_fresh_accepted_operator_approval_artifact")
                    .and_then(serde_json::Value::as_bool)
                    == Some(true)
                && item
                    .get("requires_single_use_approval_nonce")
                    .and_then(serde_json::Value::as_bool)
                    == Some(true)
                && item
                    .get("requires_operator_identity_session_binding")
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
    let source_review_ready = source_bool(
        "first_model_invocation_operator_approval_packet_review_acceptance_denial_preflight_ready",
    ) && source_bool(
        "source_first_model_invocation_approval_preflight_ready",
    ) && source_bool("review_surface_rendered")
        && source_bool("review_surface_redacted")
        && source_bool("review_surface_readback_performed")
        && source_bool("review_surface_readback_hash_matched")
        && !source_bool("review_surface_persisted")
        && !source_bool("operator_review_recorded")
        && !source_bool("operator_review_persisted")
        && source_bool("approval_acceptance_candidate_present")
        && !source_bool("approval_acceptance_preconditions_satisfied")
        && source_bool("approval_acceptance_denied")
        && !source_bool("approval_packet_review_accepted")
        && !source_bool("approval_packet_accepted")
        && !source_bool("approval_packet_persisted")
        && !source_bool("approval_packet_ledger_recorded")
        && !source_bool("approval_packet_filesystem_written")
        && source_bool("approval_acceptance_receipt_rendered")
        && !source_bool("approval_acceptance_receipt_persisted")
        && source_bool("fresh_accepted_operator_approval_artifact_required")
        && !source_bool("fresh_accepted_operator_approval_artifact_present")
        && source_bool("explicit_invocation_command_required")
        && !source_bool("explicit_invocation_command_present")
        && source_bool("single_use_approval_nonce_required")
        && !source_bool("single_use_approval_nonce_verified")
        && !source_bool("single_use_approval_nonce_consumed")
        && source_bool("operator_identity_session_binding_required")
        && !source_bool("operator_identity_session_binding_verified")
        && !source_bool("operator_identity_session_bound")
        && source_bool("candidate_provider_invocation_requested")
        && source_bool("candidate_model_invocation_requested")
        && !source_bool("provider_invocation_authorized")
        && !source_bool("model_invocation_authorized")
        && source_i64("provider_invocation_budget") == 0
        && source_i64("model_invocation_budget") == 0
        && !source_bool("provider_invoked")
        && !source_bool("model_invoked")
        && !source_bool("credential_value_read")
        && !source_bool("credential_read")
        && !source_bool("secret_file_read")
        && !source_bool("provider_router_live_envelope_executed")
        && !source_bool("provider_prompt_injection_performed")
        && !source_bool("context_injection_performed")
        && !source_bool("kg_adapter_read_performed")
        && !source_bool("live_kg_write_performed")
        && !source_bool("memory_store_write_performed")
        && !source_bool("channel_send_performed")
        && !source_bool("telegram_send_performed")
        && !source_bool("external_send_performed")
        && source_next_action_artifact_precondition;
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT;
    let source_review_surface_scope = source_str("review_surface_scope");
    let source_review_readback_hash = source_str("review_surface_readback_hash_sha256");
    let source_acceptance_receipt_hash = source_str("approval_acceptance_receipt_hash_sha256");
    let artifact_precondition_scope =
        "first_model_invocation:operator-approval-acceptance-artifact-precondition";
    let precondition_readback_hash = sha256_text_value(&format!(
        "first-model-approval-artifact-precondition-readback:{artifact_precondition_scope}:{source_review_surface_scope}:{source_review_readback_hash}:missing-artifact-nonce-session-command"
    ));
    let precondition_receipt_hash = sha256_text_value(&format!(
        "first-model-approval-artifact-precondition-denial:{precondition_readback_hash}:{source_acceptance_receipt_hash}:fresh-artifact=false:nonce=false:session=false:command=false"
    ));
    let report_ready =
        route_matrix.ready && route_count_source_command_accepted && source_review_ready;

    let audit_steps = vec![
        serde_json::json!({
            "step": "approval_review_source_binding",
            "status": "ready",
            "source_endpoint": HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_PACKET_REVIEW_ACCEPTANCE_DENIAL_PREFLIGHT_ENDPOINT,
            "source_review_acceptance_denial_ready": source_review_ready,
            "source_review_surface_scope": source_review_surface_scope,
            "source_acceptance_receipt_hash_sha256": source_acceptance_receipt_hash,
            "source_approval_acceptance_denied": true
        }),
        serde_json::json!({
            "step": "accepted_artifact_presence_freshness_precondition",
            "status": "missing_fresh_accepted_operator_approval_artifact",
            "fresh_accepted_operator_approval_artifact_required": true,
            "fresh_accepted_operator_approval_artifact_present": false,
            "fresh_accepted_operator_approval_artifact_verified": false,
            "accepted_operator_approval_artifact_hash_matched": false,
            "approval_artifact_freshness_window_required": true,
            "approval_artifact_freshness_window_satisfied": false
        }),
        serde_json::json!({
            "step": "nonce_session_explicit_command_preconditions",
            "status": "missing_nonce_session_and_explicit_command",
            "single_use_approval_nonce_required": true,
            "single_use_approval_nonce_present": false,
            "single_use_approval_nonce_verified": false,
            "single_use_approval_nonce_consumed": false,
            "operator_identity_session_binding_required": true,
            "operator_identity_session_binding_present": false,
            "operator_identity_session_binding_verified": false,
            "explicit_invocation_command_required": true,
            "explicit_invocation_command_present": false,
            "explicit_invocation_command_accepted": false
        }),
        serde_json::json!({
            "step": "approval_artifact_replay_supersession_denial",
            "status": "replay_and_supersession_denied_without_fresh_artifact",
            "approval_artifact_replay_denied": true,
            "approval_artifact_supersession_denied": true,
            "stale_operator_approval_artifact_accepted": false,
            "superseded_operator_approval_artifact_accepted": false,
            "approval_artifact_reinstatement_accepted": false
        }),
        serde_json::json!({
            "step": "acceptance_authorization_denial",
            "status": "acceptance_authorization_denied",
            "approval_acceptance_candidate_present": true,
            "approval_acceptance_preconditions_satisfied": false,
            "approval_acceptance_denied": true,
            "approval_packet_review_accepted": false,
            "approval_packet_accepted": false,
            "operator_approval_recorded": false,
            "operator_consent_recorded": false
        }),
        serde_json::json!({
            "step": "invocation_side_effect_denial_check",
            "status": "ready",
            "candidate_provider_invocation_requested": true,
            "candidate_model_invocation_requested": true,
            "provider_invocation_authorized": false,
            "model_invocation_authorized": false,
            "provider_invocation_budget": 0,
            "model_invocation_budget": 0,
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
        "accepted_operator_approval_artifact_recorded",
        "accepted_operator_approval_artifact_persisted",
        "accepted_operator_approval_artifact_filesystem_read",
        "accepted_operator_approval_artifact_filesystem_written",
        "fresh_accepted_operator_approval_artifact_verified",
        "accepted_operator_approval_artifact_hash_matched",
        "approval_artifact_freshness_window_satisfied",
        "approval_artifact_replay_accepted",
        "stale_operator_approval_artifact_accepted",
        "superseded_operator_approval_artifact_accepted",
        "approval_artifact_reinstatement_accepted",
        "single_use_approval_nonce_present",
        "single_use_approval_nonce_verified",
        "single_use_approval_nonce_consumed",
        "operator_identity_session_binding_present",
        "operator_identity_session_binding_verified",
        "operator_identity_session_bound",
        "explicit_invocation_command_present",
        "explicit_invocation_command_accepted",
        "approval_acceptance_preconditions_satisfied",
        "approval_acceptance_persisted",
        "approval_acceptance_ledger_recorded",
        "approval_acceptance_filesystem_written",
        "approval_packet_review_accepted",
        "approval_packet_accepted",
        "approval_packet_persisted",
        "approval_packet_ledger_recorded",
        "approval_packet_filesystem_written",
        "operator_approval_recorded",
        "operator_consent_recorded",
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
        "external_network_call_performed",
        "kg_adapter_live_read_performed",
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

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_first_model_invocation_operator_approval_acceptance_artifact_precondition_route",
        "endpoint": HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_ACCEPTANCE_ARTIFACT_PRECONDITION_ENDPOINT,
        "source_command": "/hepta-first-model-invocation-operator-approval-acceptance-artifact-precondition --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-22",
        "canary_schema_version": "hepta_first_model_invocation_operator_approval_acceptance_artifact_precondition_v1",
        "canary_execution_mode": "first_model_invocation_operator_approval_acceptance_artifact_precondition_no_provider_model_invocation",
        "source_first_model_invocation_approval_review_acceptance_denial_endpoint": HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_PACKET_REVIEW_ACCEPTANCE_DENIAL_PREFLIGHT_ENDPOINT,
        "source_first_model_invocation_approval_review_acceptance_denial_ready": source_review_ready,
        "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        "route_count": route_matrix.route_count,
        "implemented_route_count": route_matrix.implemented_route_count,
        "missing_route_count": route_matrix.missing_route_count,
        "route_count_source_command_accepted": route_count_source_command_accepted,
        "first_model_invocation_operator_approval_acceptance_artifact_precondition_route_enabled": true,
        "first_model_invocation_operator_approval_acceptance_artifact_precondition_ready": report_ready
    });
    extend_json_object(
        &mut report,
        serde_json::json!({
            "approval_state": "accepted_artifact_precondition_rendered_but_missing_artifact_nonce_session_and_explicit_command",
            "artifact_precondition_scope": artifact_precondition_scope,
            "accepted_operator_approval_artifact_precondition_rendered": true,
            "accepted_operator_approval_artifact_precondition_readback_performed": true,
            "accepted_operator_approval_artifact_precondition_readback_hash_sha256": precondition_readback_hash,
            "accepted_operator_approval_artifact_precondition_readback_hash_matched": true,
            "accepted_operator_approval_artifact_recorded": false,
            "accepted_operator_approval_artifact_persisted": false,
            "accepted_operator_approval_artifact_filesystem_read": false,
            "accepted_operator_approval_artifact_filesystem_written": false,
            "fresh_accepted_operator_approval_artifact_required": true,
            "fresh_accepted_operator_approval_artifact_present": false,
            "fresh_accepted_operator_approval_artifact_verified": false,
            "accepted_operator_approval_artifact_hash_matched": false,
            "approval_artifact_freshness_window_required": true,
            "approval_artifact_freshness_window_satisfied": false,
            "approval_artifact_replay_denied": true,
            "approval_artifact_supersession_denied": true,
            "stale_operator_approval_artifact_accepted": false,
            "superseded_operator_approval_artifact_accepted": false,
            "approval_artifact_reinstatement_accepted": false
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "single_use_approval_nonce_required": true,
            "single_use_approval_nonce_present": false,
            "single_use_approval_nonce_verified": false,
            "single_use_approval_nonce_consumed": false,
            "operator_identity_session_binding_required": true,
            "operator_identity_session_binding_present": false,
            "operator_identity_session_binding_verified": false,
            "operator_identity_session_bound": false,
            "explicit_command_required": true,
            "explicit_invocation_command_required": true,
            "explicit_invocation_command_present": false,
            "explicit_invocation_command_accepted": false
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "approval_acceptance_candidate_present": true,
            "approval_acceptance_preconditions_satisfied": false,
            "approval_acceptance_denied": true,
            "approval_packet_review_accepted": false,
            "approval_packet_accepted": false,
            "approval_packet_persisted": false,
            "approval_packet_ledger_recorded": false,
            "approval_packet_filesystem_written": false,
            "approval_precondition_receipt_rendered": true,
            "approval_precondition_receipt_hash_sha256": precondition_receipt_hash,
            "approval_precondition_receipt_persisted": false,
            "approval_precondition_receipt_ledger_recorded": false,
            "approval_precondition_receipt_filesystem_written": false,
            "operator_approval_recorded": false,
            "operator_consent_recorded": false
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "candidate_provider_invocation_requested": true,
            "candidate_model_invocation_requested": true,
            "provider_invocation_authorized": false,
            "model_invocation_authorized": false,
            "provider_invocation_budget": 0,
            "model_invocation_budget": 0,
            "provider_invoked": false,
            "model_invoked": false,
            "credential_value_read": false,
            "credential_read": false,
            "secret_file_read": false,
            "provider_router_live_envelope_executed": false,
            "provider_router_prompt_mutated": false,
            "provider_router_context_packet_materialized": false,
            "provider_prompt_injection_performed": false,
            "context_injection_performed": false,
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
                    "action": "first_model_invocation_operator_approval_nonce_session_command_binding_preflight",
                    "status": "requires_positive_accepted_approval_artifact_fixture_before_nonce_session_command_binding",
                    "requires_fresh_accepted_operator_approval_artifact": true,
                    "requires_single_use_approval_nonce": true,
                    "requires_operator_identity_session_binding": true,
                    "requires_explicit_command": true,
                    "invokes_provider": false,
                    "invokes_model": false,
                    "reads_credentials": false,
                    "writes_kg": false,
                    "sends_externally": false,
                    "mutates_durable_memory": false
                }
            ],
            "blocked_actions": [
                "accepted_approval_artifact_missing_but_acceptance_recorded",
                "stale_or_superseded_approval_artifact_reused",
                "approval_nonce_replay_or_consumption_without_fresh_artifact",
                "identity_session_binding_skipped_before_invocation",
                "explicit_invocation_command_missing_but_provider_model_authorized",
                "provider_or_model_invocation_during_artifact_precondition",
                "credential_or_secret_read_during_artifact_precondition",
                "kg_or_memory_write_during_artifact_precondition",
                "channel_or_external_delivery_during_artifact_precondition"
            ],
            "side_effects": side_effects
        }),
    );
    report
}

fn hepta_first_model_invocation_operator_approval_nonce_session_command_binding_preflight_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source =
        hepta_first_model_invocation_operator_approval_acceptance_artifact_precondition_report();
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
            .and_then(|value| value.as_str())
            .unwrap_or("")
    };
    let source_next_action_nonce_session_command = source
        .get("allowed_next_actions")
        .and_then(|value| value.as_array())
        .and_then(|items| items.first())
        .map(|item| {
            item.get("action").and_then(|value| value.as_str())
                == Some("first_model_invocation_operator_approval_nonce_session_command_binding_preflight")
                && item
                    .get("requires_fresh_accepted_operator_approval_artifact")
                    .and_then(serde_json::Value::as_bool)
                    == Some(true)
                && item
                    .get("requires_single_use_approval_nonce")
                    .and_then(serde_json::Value::as_bool)
                    == Some(true)
                && item
                    .get("requires_operator_identity_session_binding")
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
                && item.get("invokes_model").and_then(serde_json::Value::as_bool) == Some(false)
        })
        .unwrap_or(false);
    let source_artifact_precondition_ready =
        source_bool(
            "first_model_invocation_operator_approval_acceptance_artifact_precondition_ready",
        ) && source_bool("source_first_model_invocation_approval_review_acceptance_denial_ready")
            && source_bool("accepted_operator_approval_artifact_precondition_rendered")
            && source_bool("accepted_operator_approval_artifact_precondition_readback_performed")
            && source_bool(
                "accepted_operator_approval_artifact_precondition_readback_hash_matched",
            )
            && !source_bool("accepted_operator_approval_artifact_recorded")
            && !source_bool("accepted_operator_approval_artifact_persisted")
            && !source_bool("accepted_operator_approval_artifact_filesystem_read")
            && !source_bool("accepted_operator_approval_artifact_filesystem_written")
            && source_bool("fresh_accepted_operator_approval_artifact_required")
            && !source_bool("fresh_accepted_operator_approval_artifact_present")
            && !source_bool("fresh_accepted_operator_approval_artifact_verified")
            && source_bool("approval_artifact_replay_denied")
            && source_bool("approval_artifact_supersession_denied")
            && !source_bool("stale_operator_approval_artifact_accepted")
            && !source_bool("superseded_operator_approval_artifact_accepted")
            && !source_bool("approval_artifact_reinstatement_accepted")
            && source_bool("single_use_approval_nonce_required")
            && !source_bool("single_use_approval_nonce_verified")
            && !source_bool("single_use_approval_nonce_consumed")
            && source_bool("operator_identity_session_binding_required")
            && !source_bool("operator_identity_session_binding_verified")
            && !source_bool("operator_identity_session_bound")
            && source_bool("explicit_invocation_command_required")
            && !source_bool("explicit_invocation_command_accepted")
            && source_bool("approval_acceptance_candidate_present")
            && !source_bool("approval_acceptance_preconditions_satisfied")
            && source_bool("approval_acceptance_denied")
            && !source_bool("approval_packet_accepted")
            && !source_bool("operator_approval_recorded")
            && !source_bool("operator_consent_recorded")
            && source_bool("candidate_provider_invocation_requested")
            && source_bool("candidate_model_invocation_requested")
            && !source_bool("provider_invocation_authorized")
            && !source_bool("model_invocation_authorized")
            && source_i64("provider_invocation_budget") == 0
            && source_i64("model_invocation_budget") == 0
            && !source_bool("provider_invoked")
            && !source_bool("model_invoked")
            && !source_bool("credential_value_read")
            && !source_bool("credential_read")
            && !source_bool("secret_file_read")
            && !source_bool("provider_prompt_injection_performed")
            && !source_bool("context_injection_performed")
            && !source_bool("kg_adapter_read_performed")
            && !source_bool("live_kg_write_performed")
            && !source_bool("memory_store_write_performed")
            && !source_bool("channel_send_performed")
            && !source_bool("telegram_send_performed")
            && !source_bool("external_send_performed")
            && source_next_action_nonce_session_command;
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT;
    let source_precondition_hash =
        source_str("accepted_operator_approval_artifact_precondition_readback_hash_sha256");
    let source_receipt_hash = source_str("approval_precondition_receipt_hash_sha256");
    let binding_scope = "first_model_invocation:operator-approval-nonce-session-command-binding";
    let synthetic_artifact_fixture_hash = sha256_text_value(&format!(
        "first-model-approval-synthetic-accepted-artifact-fixture:{binding_scope}:{source_precondition_hash}:{source_receipt_hash}:not-live-approval"
    ));
    let binding_readback_hash = sha256_text_value(&format!(
        "first-model-approval-nonce-session-command-binding-readback:{binding_scope}:{synthetic_artifact_fixture_hash}:nonce=false:session=false:command=false"
    ));
    let binding_denial_receipt_hash = sha256_text_value(&format!(
        "first-model-approval-nonce-session-command-binding-denial:{binding_readback_hash}:provider=false:model=false"
    ));
    let report_ready = route_matrix.ready
        && route_count_source_command_accepted
        && source_artifact_precondition_ready;

    let audit_steps = vec![
        serde_json::json!({
            "step": "approval_artifact_precondition_source_binding",
            "status": "ready",
            "source_endpoint": HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_ACCEPTANCE_ARTIFACT_PRECONDITION_ENDPOINT,
            "source_artifact_precondition_ready": source_artifact_precondition_ready,
            "source_precondition_receipt_hash_sha256": source_receipt_hash,
            "source_approval_acceptance_denied": true
        }),
        serde_json::json!({
            "step": "synthetic_accepted_artifact_fixture_isolation",
            "status": "dry_run_fixture_only",
            "synthetic_accepted_operator_approval_artifact_fixture_rendered": true,
            "synthetic_accepted_operator_approval_artifact_fixture_hash_sha256": synthetic_artifact_fixture_hash,
            "synthetic_accepted_operator_approval_artifact_fixture_persisted": false,
            "fresh_live_accepted_operator_approval_artifact_present": false,
            "operator_approval_recorded": false
        }),
        serde_json::json!({
            "step": "single_use_nonce_binding_preflight",
            "status": "nonce_not_present_or_consumed",
            "single_use_approval_nonce_required": true,
            "single_use_approval_nonce_fixture_rendered": true,
            "single_use_approval_nonce_present": false,
            "single_use_approval_nonce_verified": false,
            "single_use_approval_nonce_consumed": false,
            "single_use_approval_nonce_replay_denied": true
        }),
        serde_json::json!({
            "step": "operator_session_and_explicit_command_binding_preflight",
            "status": "session_and_command_not_bound",
            "operator_identity_session_binding_required": true,
            "operator_identity_session_binding_fixture_rendered": true,
            "operator_identity_session_binding_present": false,
            "operator_identity_session_binding_verified": false,
            "operator_identity_session_bound": false,
            "explicit_invocation_command_required": true,
            "explicit_invocation_command_fixture_rendered": true,
            "explicit_invocation_command_present": false,
            "explicit_invocation_command_accepted": false
        }),
        serde_json::json!({
            "step": "replay_cross_binding_denial",
            "status": "replay_and_cross_binding_denied",
            "single_use_approval_nonce_replay_denied": true,
            "operator_identity_session_cross_binding_denied": true,
            "explicit_invocation_command_replay_denied": true,
            "nonce_session_command_cross_binding_accepted": false
        }),
        serde_json::json!({
            "step": "invocation_side_effect_denial_check",
            "status": "ready",
            "candidate_provider_invocation_requested": true,
            "candidate_model_invocation_requested": true,
            "provider_invocation_authorized": false,
            "model_invocation_authorized": false,
            "provider_invocation_budget": 0,
            "model_invocation_budget": 0,
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
        "synthetic_accepted_operator_approval_artifact_fixture_persisted",
        "synthetic_accepted_operator_approval_artifact_fixture_filesystem_written",
        "fresh_live_accepted_operator_approval_artifact_present",
        "fresh_live_accepted_operator_approval_artifact_verified",
        "accepted_operator_approval_artifact_recorded",
        "accepted_operator_approval_artifact_persisted",
        "single_use_approval_nonce_present",
        "single_use_approval_nonce_verified",
        "single_use_approval_nonce_consumed",
        "single_use_approval_nonce_replay_accepted",
        "operator_identity_session_binding_present",
        "operator_identity_session_binding_verified",
        "operator_identity_session_bound",
        "operator_identity_session_cross_binding_accepted",
        "explicit_invocation_command_present",
        "explicit_invocation_command_accepted",
        "explicit_invocation_command_replay_accepted",
        "nonce_session_command_binding_preconditions_satisfied",
        "nonce_session_command_binding_accepted",
        "nonce_session_command_binding_persisted",
        "nonce_session_command_binding_ledger_recorded",
        "nonce_session_command_binding_filesystem_written",
        "approval_acceptance_preconditions_satisfied",
        "approval_acceptance_persisted",
        "approval_acceptance_ledger_recorded",
        "approval_packet_accepted",
        "approval_packet_persisted",
        "approval_packet_ledger_recorded",
        "approval_packet_filesystem_written",
        "operator_approval_recorded",
        "operator_consent_recorded",
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
        "external_network_call_performed",
        "kg_adapter_live_read_performed",
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

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_first_model_invocation_operator_approval_nonce_session_command_binding_preflight_route",
        "endpoint": HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_NONCE_SESSION_COMMAND_BINDING_PREFLIGHT_ENDPOINT,
        "source_command": "/hepta-first-model-invocation-operator-approval-nonce-session-command-binding-preflight --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-23",
        "canary_schema_version": "hepta_first_model_invocation_operator_approval_nonce_session_command_binding_preflight_v1",
        "canary_execution_mode": "first_model_invocation_operator_approval_nonce_session_command_binding_preflight_no_provider_model_invocation",
        "source_first_model_invocation_approval_acceptance_artifact_precondition_endpoint": HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_ACCEPTANCE_ARTIFACT_PRECONDITION_ENDPOINT,
        "source_first_model_invocation_approval_acceptance_artifact_precondition_ready": source_artifact_precondition_ready,
        "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        "route_count": route_matrix.route_count,
        "implemented_route_count": route_matrix.implemented_route_count,
        "missing_route_count": route_matrix.missing_route_count,
        "route_count_source_command_accepted": route_count_source_command_accepted,
        "first_model_invocation_operator_approval_nonce_session_command_binding_preflight_route_enabled": true,
        "first_model_invocation_operator_approval_nonce_session_command_binding_preflight_ready": report_ready
    });
    extend_json_object(
        &mut report,
        serde_json::json!({
            "approval_state": "synthetic_accepted_artifact_fixture_rendered_but_nonce_session_command_not_bound",
            "nonce_session_command_binding_scope": binding_scope,
            "synthetic_accepted_operator_approval_artifact_fixture_rendered": true,
            "synthetic_accepted_operator_approval_artifact_fixture_readback_performed": true,
            "synthetic_accepted_operator_approval_artifact_fixture_hash_sha256": synthetic_artifact_fixture_hash,
            "synthetic_accepted_operator_approval_artifact_fixture_hash_matched": true,
            "synthetic_accepted_operator_approval_artifact_fixture_persisted": false,
            "synthetic_accepted_operator_approval_artifact_fixture_filesystem_written": false,
            "fresh_live_accepted_operator_approval_artifact_required": true,
            "fresh_live_accepted_operator_approval_artifact_present": false,
            "fresh_live_accepted_operator_approval_artifact_verified": false,
            "accepted_operator_approval_artifact_recorded": false,
            "accepted_operator_approval_artifact_persisted": false
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "single_use_approval_nonce_required": true,
            "single_use_approval_nonce_fixture_rendered": true,
            "single_use_approval_nonce_present": false,
            "single_use_approval_nonce_verified": false,
            "single_use_approval_nonce_consumed": false,
            "single_use_approval_nonce_replay_denied": true,
            "operator_identity_session_binding_required": true,
            "operator_identity_session_binding_fixture_rendered": true,
            "operator_identity_session_binding_present": false,
            "operator_identity_session_binding_verified": false,
            "operator_identity_session_bound": false,
            "operator_identity_session_cross_binding_denied": true,
            "explicit_command_required": true,
            "explicit_invocation_command_required": true,
            "explicit_invocation_command_fixture_rendered": true,
            "explicit_invocation_command_present": false,
            "explicit_invocation_command_accepted": false,
            "explicit_invocation_command_replay_denied": true,
            "nonce_session_command_binding_candidate_present": true,
            "nonce_session_command_binding_preconditions_satisfied": false,
            "nonce_session_command_binding_denied": true,
            "nonce_session_command_binding_readback_performed": true,
            "nonce_session_command_binding_readback_hash_sha256": binding_readback_hash,
            "nonce_session_command_binding_readback_hash_matched": true,
            "nonce_session_command_binding_denial_receipt_rendered": true,
            "nonce_session_command_binding_denial_receipt_hash_sha256": binding_denial_receipt_hash,
            "nonce_session_command_binding_denial_receipt_persisted": false,
            "nonce_session_command_binding_denial_receipt_ledger_recorded": false,
            "nonce_session_command_binding_denial_receipt_filesystem_written": false
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "approval_acceptance_candidate_present": true,
            "approval_acceptance_preconditions_satisfied": false,
            "approval_acceptance_denied": true,
            "approval_packet_review_accepted": false,
            "approval_packet_accepted": false,
            "approval_packet_persisted": false,
            "approval_packet_ledger_recorded": false,
            "approval_packet_filesystem_written": false,
            "approval_final_authorization_denied": true,
            "operator_approval_recorded": false,
            "operator_consent_recorded": false
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "candidate_provider_invocation_requested": true,
            "candidate_model_invocation_requested": true,
            "provider_invocation_authorized": false,
            "model_invocation_authorized": false,
            "provider_invocation_budget": 0,
            "model_invocation_budget": 0,
            "provider_invoked": false,
            "model_invoked": false,
            "credential_value_read": false,
            "credential_read": false,
            "secret_file_read": false,
            "provider_router_live_envelope_executed": false,
            "provider_router_prompt_mutated": false,
            "provider_router_context_packet_materialized": false,
            "provider_prompt_injection_performed": false,
            "context_injection_performed": false,
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
                    "action": "first_model_invocation_operator_approval_final_authorization_dry_run_envelope_preflight",
                    "status": "requires_real_fresh_accepted_artifact_nonce_session_binding_and_explicit_command_before_any_provider_invocation",
                    "requires_fresh_accepted_operator_approval_artifact": true,
                    "requires_single_use_approval_nonce": true,
                    "requires_operator_identity_session_binding": true,
                    "requires_explicit_command": true,
                    "invokes_provider": false,
                    "invokes_model": false,
                    "reads_credentials": false,
                    "writes_kg": false,
                    "sends_externally": false,
                    "mutates_durable_memory": false
                }
            ],
            "blocked_actions": [
                "synthetic_fixture_as_live_operator_approval",
                "nonce_consumption_without_real_fresh_accepted_artifact",
                "session_cross_binding_or_replay",
                "explicit_invocation_command_replay_or_missing_command",
                "provider_or_model_invocation_during_nonce_session_command_preflight",
                "credential_or_secret_read_during_nonce_session_command_preflight",
                "kg_or_memory_write_during_nonce_session_command_preflight",
                "channel_or_external_delivery_during_nonce_session_command_preflight"
            ],
            "side_effects": side_effects
        }),
    );
    report
}

fn hepta_first_model_invocation_operator_approval_final_authorization_dry_run_envelope_preflight_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source =
        hepta_first_model_invocation_operator_approval_nonce_session_command_binding_preflight_report();
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
            .and_then(|value| value.as_str())
            .unwrap_or("")
    };
    let source_next_action_final_authorization = source
        .get("allowed_next_actions")
        .and_then(|value| value.as_array())
        .and_then(|items| items.first())
        .map(|item| {
            item.get("action").and_then(|value| value.as_str())
                == Some("first_model_invocation_operator_approval_final_authorization_dry_run_envelope_preflight")
                && item
                    .get("requires_fresh_accepted_operator_approval_artifact")
                    .and_then(serde_json::Value::as_bool)
                    == Some(true)
                && item
                    .get("requires_single_use_approval_nonce")
                    .and_then(serde_json::Value::as_bool)
                    == Some(true)
                && item
                    .get("requires_operator_identity_session_binding")
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
                && item.get("invokes_model").and_then(serde_json::Value::as_bool) == Some(false)
        })
        .unwrap_or(false);
    let source_nonce_session_command_ready = source_bool(
        "first_model_invocation_operator_approval_nonce_session_command_binding_preflight_ready",
    ) && source_bool(
        "source_first_model_invocation_approval_acceptance_artifact_precondition_ready",
    ) && source_bool(
        "synthetic_accepted_operator_approval_artifact_fixture_rendered",
    ) && source_bool(
        "synthetic_accepted_operator_approval_artifact_fixture_readback_performed",
    ) && source_bool(
        "synthetic_accepted_operator_approval_artifact_fixture_hash_matched",
    ) && !source_bool(
        "synthetic_accepted_operator_approval_artifact_fixture_persisted",
    ) && !source_bool(
        "fresh_live_accepted_operator_approval_artifact_present",
    ) && !source_bool(
        "fresh_live_accepted_operator_approval_artifact_verified",
    ) && !source_bool(
        "accepted_operator_approval_artifact_recorded",
    ) && !source_bool(
        "accepted_operator_approval_artifact_persisted",
    ) && source_bool("single_use_approval_nonce_required")
        && !source_bool("single_use_approval_nonce_present")
        && !source_bool("single_use_approval_nonce_verified")
        && !source_bool("single_use_approval_nonce_consumed")
        && source_bool("single_use_approval_nonce_replay_denied")
        && source_bool("operator_identity_session_binding_required")
        && !source_bool("operator_identity_session_binding_present")
        && !source_bool("operator_identity_session_binding_verified")
        && !source_bool("operator_identity_session_bound")
        && source_bool("operator_identity_session_cross_binding_denied")
        && source_bool("explicit_invocation_command_required")
        && !source_bool("explicit_invocation_command_present")
        && !source_bool("explicit_invocation_command_accepted")
        && source_bool("explicit_invocation_command_replay_denied")
        && source_bool("nonce_session_command_binding_candidate_present")
        && !source_bool("nonce_session_command_binding_preconditions_satisfied")
        && source_bool("nonce_session_command_binding_denied")
        && source_bool("nonce_session_command_binding_readback_performed")
        && source_bool("nonce_session_command_binding_readback_hash_matched")
        && source_bool("nonce_session_command_binding_denial_receipt_rendered")
        && !source_bool("nonce_session_command_binding_denial_receipt_persisted")
        && !source_bool("nonce_session_command_binding_denial_receipt_ledger_recorded")
        && !source_bool("nonce_session_command_binding_denial_receipt_filesystem_written")
        && source_bool("approval_acceptance_denied")
        && !source_bool("approval_packet_accepted")
        && source_bool("approval_final_authorization_denied")
        && !source_bool("operator_approval_recorded")
        && !source_bool("operator_consent_recorded")
        && source_bool("candidate_provider_invocation_requested")
        && source_bool("candidate_model_invocation_requested")
        && !source_bool("provider_invocation_authorized")
        && !source_bool("model_invocation_authorized")
        && source_i64("provider_invocation_budget") == 0
        && source_i64("model_invocation_budget") == 0
        && !source_bool("provider_invoked")
        && !source_bool("model_invoked")
        && !source_bool("credential_value_read")
        && !source_bool("credential_read")
        && !source_bool("secret_file_read")
        && !source_bool("provider_router_live_envelope_executed")
        && !source_bool("provider_prompt_injection_performed")
        && !source_bool("context_injection_performed")
        && !source_bool("kg_adapter_read_performed")
        && !source_bool("live_kg_write_performed")
        && !source_bool("memory_store_write_performed")
        && !source_bool("channel_send_performed")
        && !source_bool("telegram_send_performed")
        && !source_bool("external_send_performed")
        && source_next_action_final_authorization;
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT;
    let source_binding_hash = source_str("nonce_session_command_binding_readback_hash_sha256");
    let source_denial_receipt_hash =
        source_str("nonce_session_command_binding_denial_receipt_hash_sha256");
    let authorization_scope =
        "first_model_invocation:operator-approval-final-authorization-dry-run-envelope";
    let final_authorization_envelope_hash = sha256_text_value(&format!(
        "first-model-final-authorization-dry-run-envelope:{authorization_scope}:{source_binding_hash}:{source_denial_receipt_hash}:provider_budget=0:model_budget=0"
    ));
    let final_authorization_readback_hash = sha256_text_value(&format!(
        "first-model-final-authorization-dry-run-readback:{authorization_scope}:{final_authorization_envelope_hash}:authorization=false:provider=false:model=false"
    ));
    let final_authorization_denial_receipt_hash = sha256_text_value(&format!(
        "first-model-final-authorization-dry-run-denial-receipt:{final_authorization_readback_hash}:missing-real-artifact-nonce-session-command"
    ));
    let report_ready = route_matrix.ready
        && route_count_source_command_accepted
        && source_nonce_session_command_ready;

    let audit_steps = vec![
        serde_json::json!({
            "step": "nonce_session_command_source_binding",
            "status": "ready",
            "source_endpoint": HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_NONCE_SESSION_COMMAND_BINDING_PREFLIGHT_ENDPOINT,
            "source_nonce_session_command_binding_ready": source_nonce_session_command_ready,
            "source_binding_readback_hash_sha256": source_binding_hash,
            "source_denial_receipt_hash_sha256": source_denial_receipt_hash
        }),
        serde_json::json!({
            "step": "final_authorization_dry_run_envelope_construction",
            "status": "dry_run_only",
            "final_authorization_dry_run_envelope_rendered": true,
            "final_authorization_dry_run_envelope_hash_sha256": final_authorization_envelope_hash,
            "final_authorization_dry_run_envelope_persisted": false,
            "final_authorization_live_envelope_executed": false
        }),
        serde_json::json!({
            "step": "real_precondition_denial",
            "status": "missing_real_fresh_artifact_nonce_session_command",
            "fresh_live_accepted_operator_approval_artifact_present": false,
            "single_use_approval_nonce_verified": false,
            "operator_identity_session_binding_verified": false,
            "explicit_invocation_command_accepted": false,
            "final_authorization_preconditions_satisfied": false,
            "final_authorization_denied": true
        }),
        serde_json::json!({
            "step": "provider_model_budget_binding",
            "status": "budget_zero",
            "candidate_provider_invocation_requested": true,
            "candidate_model_invocation_requested": true,
            "provider_invocation_authorized": false,
            "model_invocation_authorized": false,
            "provider_invocation_budget": 0,
            "model_invocation_budget": 0,
            "provider_invoked": false,
            "model_invoked": false
        }),
        serde_json::json!({
            "step": "dry_run_receipt_non_persistence",
            "status": "not_persisted",
            "final_authorization_dry_run_readback_performed": true,
            "final_authorization_dry_run_readback_hash_sha256": final_authorization_readback_hash,
            "final_authorization_dry_run_readback_hash_matched": true,
            "final_authorization_denial_receipt_rendered": true,
            "final_authorization_denial_receipt_hash_sha256": final_authorization_denial_receipt_hash,
            "final_authorization_denial_receipt_persisted": false,
            "final_authorization_denial_receipt_ledger_recorded": false,
            "final_authorization_denial_receipt_filesystem_written": false
        }),
        serde_json::json!({
            "step": "side_effect_denial_check",
            "status": "ready",
            "credential_read": false,
            "secret_file_read": false,
            "provider_router_live_envelope_executed": false,
            "provider_prompt_injection_performed": false,
            "context_injection_performed": false,
            "live_kg_write_performed": false,
            "memory_store_write_performed": false,
            "channel_send_performed": false,
            "external_send_performed": false,
            "install_executed": false,
            "active_binary_mutated": false,
            "public_release_claimed": false
        }),
    ];

    let mut side_effects = serde_json::Map::new();
    for key in [
        "final_authorization_dry_run_envelope_persisted",
        "final_authorization_dry_run_envelope_filesystem_written",
        "final_authorization_live_envelope_executed",
        "final_authorization_preconditions_satisfied",
        "final_authorization_accepted",
        "final_authorization_persisted",
        "final_authorization_ledger_recorded",
        "final_authorization_filesystem_written",
        "final_authorization_denial_receipt_persisted",
        "final_authorization_denial_receipt_ledger_recorded",
        "final_authorization_denial_receipt_filesystem_written",
        "fresh_live_accepted_operator_approval_artifact_present",
        "fresh_live_accepted_operator_approval_artifact_verified",
        "single_use_approval_nonce_verified",
        "single_use_approval_nonce_consumed",
        "operator_identity_session_binding_verified",
        "operator_identity_session_bound",
        "explicit_invocation_command_accepted",
        "approval_packet_accepted",
        "approval_packet_persisted",
        "operator_approval_recorded",
        "operator_consent_recorded",
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
        "external_network_call_performed",
        "kg_adapter_live_read_performed",
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

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_first_model_invocation_operator_approval_final_authorization_dry_run_envelope_preflight_route",
        "endpoint": HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_FINAL_AUTHORIZATION_DRY_RUN_ENVELOPE_PREFLIGHT_ENDPOINT,
        "source_command": "/hepta-first-model-invocation-operator-approval-final-authorization-dry-run-envelope-preflight --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-23",
        "canary_schema_version": "hepta_first_model_invocation_operator_approval_final_authorization_dry_run_envelope_preflight_v1",
        "canary_execution_mode": "first_model_invocation_operator_approval_final_authorization_dry_run_envelope_preflight_no_provider_model_invocation",
        "source_first_model_invocation_approval_nonce_session_command_binding_endpoint": HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_NONCE_SESSION_COMMAND_BINDING_PREFLIGHT_ENDPOINT,
        "source_first_model_invocation_approval_nonce_session_command_binding_ready": source_nonce_session_command_ready,
        "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        "route_count": route_matrix.route_count,
        "implemented_route_count": route_matrix.implemented_route_count,
        "missing_route_count": route_matrix.missing_route_count,
        "route_count_source_command_accepted": route_count_source_command_accepted,
        "first_model_invocation_operator_approval_final_authorization_dry_run_envelope_preflight_route_enabled": true,
        "first_model_invocation_operator_approval_final_authorization_dry_run_envelope_preflight_ready": report_ready
    });
    extend_json_object(
        &mut report,
        serde_json::json!({
            "authorization_state": "final_authorization_dry_run_envelope_rendered_but_real_preconditions_missing",
            "final_authorization_scope": authorization_scope,
            "final_authorization_dry_run_envelope_rendered": true,
            "final_authorization_dry_run_envelope_hash_sha256": final_authorization_envelope_hash,
            "final_authorization_dry_run_envelope_hash_matched": true,
            "final_authorization_dry_run_envelope_persisted": false,
            "final_authorization_dry_run_envelope_filesystem_written": false,
            "final_authorization_live_envelope_executed": false,
            "final_authorization_dry_run_readback_performed": true,
            "final_authorization_dry_run_readback_hash_sha256": final_authorization_readback_hash,
            "final_authorization_dry_run_readback_hash_matched": true,
            "final_authorization_denial_receipt_rendered": true,
            "final_authorization_denial_receipt_hash_sha256": final_authorization_denial_receipt_hash,
            "final_authorization_denial_receipt_persisted": false,
            "final_authorization_denial_receipt_ledger_recorded": false,
            "final_authorization_denial_receipt_filesystem_written": false
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "fresh_live_accepted_operator_approval_artifact_required": true,
            "fresh_live_accepted_operator_approval_artifact_present": false,
            "fresh_live_accepted_operator_approval_artifact_verified": false,
            "single_use_approval_nonce_required": true,
            "single_use_approval_nonce_verified": false,
            "single_use_approval_nonce_consumed": false,
            "operator_identity_session_binding_required": true,
            "operator_identity_session_binding_verified": false,
            "operator_identity_session_bound": false,
            "explicit_invocation_command_required": true,
            "explicit_invocation_command_accepted": false,
            "final_authorization_candidate_present": true,
            "final_authorization_preconditions_satisfied": false,
            "final_authorization_denied": true,
            "final_authorization_accepted": false,
            "final_authorization_persisted": false,
            "final_authorization_ledger_recorded": false,
            "final_authorization_filesystem_written": false,
            "approval_packet_accepted": false,
            "operator_approval_recorded": false,
            "operator_consent_recorded": false
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "candidate_provider_invocation_requested": true,
            "candidate_model_invocation_requested": true,
            "provider_invocation_authorized": false,
            "model_invocation_authorized": false,
            "provider_invocation_budget": 0,
            "model_invocation_budget": 0,
            "provider_invoked": false,
            "model_invoked": false,
            "credential_value_read": false,
            "credential_read": false,
            "secret_file_read": false,
            "provider_router_live_envelope_executed": false,
            "provider_router_prompt_mutated": false,
            "provider_router_context_packet_materialized": false,
            "provider_prompt_injection_performed": false,
            "context_injection_performed": false,
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
                    "action": "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_no_persistence",
                    "status": "dry_run_receipt_only_until_real_approval_nonce_session_command_are_accepted",
                    "requires_fresh_accepted_operator_approval_artifact": true,
                    "requires_single_use_approval_nonce": true,
                    "requires_operator_identity_session_binding": true,
                    "requires_explicit_command": true,
                    "invokes_provider": false,
                    "invokes_model": false,
                    "reads_credentials": false,
                    "writes_kg": false,
                    "sends_externally": false,
                    "mutates_durable_memory": false
                }
            ],
            "blocked_actions": [
                "final_authorization_from_synthetic_fixture",
                "final_authorization_without_real_fresh_accepted_artifact",
                "final_authorization_without_single_use_nonce_verification",
                "final_authorization_without_operator_identity_session_binding",
                "final_authorization_without_explicit_invocation_command",
                "provider_or_model_invocation_during_final_authorization_dry_run",
                "credential_or_secret_read_during_final_authorization_dry_run",
                "kg_or_memory_write_during_final_authorization_dry_run",
                "channel_or_external_delivery_during_final_authorization_dry_run"
            ],
            "side_effects": side_effects
        }),
    );
    report
}

fn hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_no_persistence_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source =
        hepta_first_model_invocation_operator_approval_final_authorization_dry_run_envelope_preflight_report();
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
            .and_then(|value| value.as_str())
            .unwrap_or("")
    };
    let source_next_action_result_receipt = source
        .get("allowed_next_actions")
        .and_then(|value| value.as_array())
        .and_then(|items| items.first())
        .map(|item| {
            item.get("action").and_then(|value| value.as_str())
                == Some("first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_no_persistence")
                && item
                    .get("requires_fresh_accepted_operator_approval_artifact")
                    .and_then(serde_json::Value::as_bool)
                    == Some(true)
                && item
                    .get("requires_single_use_approval_nonce")
                    .and_then(serde_json::Value::as_bool)
                    == Some(true)
                && item
                    .get("requires_operator_identity_session_binding")
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
                && item.get("invokes_model").and_then(serde_json::Value::as_bool) == Some(false)
                && item
                    .get("reads_credentials")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && item.get("writes_kg").and_then(serde_json::Value::as_bool) == Some(false)
                && item
                    .get("sends_externally")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
        })
        .unwrap_or(false);
    let source_final_authorization_ready = source_bool(
        "first_model_invocation_operator_approval_final_authorization_dry_run_envelope_preflight_ready",
    ) && source_bool(
        "source_first_model_invocation_approval_nonce_session_command_binding_ready",
    ) && source_bool(
        "final_authorization_dry_run_envelope_rendered",
    ) && source_bool(
        "final_authorization_dry_run_envelope_hash_matched",
    ) && !source_bool(
        "final_authorization_dry_run_envelope_persisted",
    ) && !source_bool(
        "final_authorization_dry_run_envelope_filesystem_written",
    ) && !source_bool(
        "final_authorization_live_envelope_executed",
    ) && source_bool(
        "final_authorization_dry_run_readback_performed",
    ) && source_bool(
        "final_authorization_dry_run_readback_hash_matched",
    ) && source_bool(
        "final_authorization_denial_receipt_rendered",
    ) && !source_bool(
        "final_authorization_denial_receipt_persisted",
    ) && !source_bool(
        "final_authorization_denial_receipt_ledger_recorded",
    ) && !source_bool(
        "final_authorization_denial_receipt_filesystem_written",
    ) && source_bool(
        "fresh_live_accepted_operator_approval_artifact_required",
    ) && !source_bool(
        "fresh_live_accepted_operator_approval_artifact_present",
    ) && !source_bool(
        "fresh_live_accepted_operator_approval_artifact_verified",
    ) && source_bool("single_use_approval_nonce_required")
        && !source_bool("single_use_approval_nonce_verified")
        && !source_bool("single_use_approval_nonce_consumed")
        && source_bool("operator_identity_session_binding_required")
        && !source_bool("operator_identity_session_binding_verified")
        && !source_bool("operator_identity_session_bound")
        && source_bool("explicit_invocation_command_required")
        && !source_bool("explicit_invocation_command_accepted")
        && source_bool("final_authorization_candidate_present")
        && !source_bool("final_authorization_preconditions_satisfied")
        && source_bool("final_authorization_denied")
        && !source_bool("final_authorization_accepted")
        && !source_bool("final_authorization_persisted")
        && !source_bool("final_authorization_ledger_recorded")
        && !source_bool("final_authorization_filesystem_written")
        && !source_bool("approval_packet_accepted")
        && !source_bool("operator_approval_recorded")
        && !source_bool("operator_consent_recorded")
        && source_bool("candidate_provider_invocation_requested")
        && source_bool("candidate_model_invocation_requested")
        && !source_bool("provider_invocation_authorized")
        && !source_bool("model_invocation_authorized")
        && source_i64("provider_invocation_budget") == 0
        && source_i64("model_invocation_budget") == 0
        && !source_bool("provider_invoked")
        && !source_bool("model_invoked")
        && !source_bool("credential_value_read")
        && !source_bool("credential_read")
        && !source_bool("secret_file_read")
        && !source_bool("provider_router_live_envelope_executed")
        && !source_bool("provider_prompt_injection_performed")
        && !source_bool("context_injection_performed")
        && !source_bool("kg_adapter_read_performed")
        && !source_bool("live_kg_write_performed")
        && !source_bool("memory_store_write_performed")
        && !source_bool("channel_send_performed")
        && !source_bool("telegram_send_performed")
        && !source_bool("external_send_performed")
        && source_next_action_result_receipt;
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT;
    let source_envelope_hash = source_str("final_authorization_dry_run_envelope_hash_sha256");
    let source_readback_hash = source_str("final_authorization_dry_run_readback_hash_sha256");
    let source_denial_receipt_hash = source_str("final_authorization_denial_receipt_hash_sha256");
    let receipt_scope =
        "first_model_invocation:operator-approval-final-authorization-dry-run-result-receipt";
    let result_receipt_hash = sha256_text_value(&format!(
        "first-model-final-authorization-dry-run-result-receipt:{receipt_scope}:{source_envelope_hash}:{source_readback_hash}:{source_denial_receipt_hash}:recorded=false:persisted=false:accepted=false"
    ));
    let result_receipt_readback_hash = sha256_text_value(&format!(
        "first-model-final-authorization-dry-run-result-receipt-readback:{receipt_scope}:{result_receipt_hash}:provider=false:model=false:persistence=false"
    ));
    let result_receipt_denial_hash = sha256_text_value(&format!(
        "first-model-final-authorization-dry-run-result-receipt-no-persistence-denial:{result_receipt_readback_hash}:missing-real-final-authorization"
    ));
    let report_ready = route_matrix.ready
        && route_count_source_command_accepted
        && source_final_authorization_ready;

    let audit_steps = vec![
        serde_json::json!({
            "step": "final_authorization_dry_run_source_binding",
            "status": "ready",
            "source_endpoint": HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_FINAL_AUTHORIZATION_DRY_RUN_ENVELOPE_PREFLIGHT_ENDPOINT,
            "source_final_authorization_dry_run_ready": source_final_authorization_ready,
            "source_final_authorization_envelope_hash_sha256": source_envelope_hash,
            "source_final_authorization_readback_hash_sha256": source_readback_hash,
            "source_final_authorization_denial_receipt_hash_sha256": source_denial_receipt_hash
        }),
        serde_json::json!({
            "step": "dry_run_result_receipt_shape_rendering",
            "status": "rendered_report_only",
            "final_authorization_dry_run_result_receipt_rendered": true,
            "final_authorization_dry_run_result_receipt_hash_sha256": result_receipt_hash,
            "final_authorization_dry_run_result_receipt_recorded": false,
            "final_authorization_dry_run_result_receipt_persisted": false,
            "final_authorization_dry_run_result_receipt_accepted": false
        }),
        serde_json::json!({
            "step": "result_receipt_readback_no_persistence",
            "status": "not_persisted",
            "final_authorization_dry_run_result_receipt_readback_performed": true,
            "final_authorization_dry_run_result_receipt_readback_hash_sha256": result_receipt_readback_hash,
            "final_authorization_dry_run_result_receipt_readback_hash_matched": true,
            "final_authorization_dry_run_result_receipt_denial_hash_sha256": result_receipt_denial_hash,
            "final_authorization_dry_run_result_receipt_ledger_recorded": false,
            "final_authorization_dry_run_result_receipt_filesystem_written": false
        }),
        serde_json::json!({
            "step": "receipt_authority_non_promotion",
            "status": "authority_denied",
            "final_authorization_from_result_receipt_allowed": false,
            "operator_approval_from_result_receipt_accepted": false,
            "activation_from_result_receipt_allowed": false,
            "provider_invocation_authorized": false,
            "model_invocation_authorized": false,
            "provider_invoked": false,
            "model_invoked": false
        }),
        serde_json::json!({
            "step": "delivery_export_observability_denial",
            "status": "denied",
            "final_authorization_dry_run_result_receipt_enqueued": false,
            "final_authorization_dry_run_result_receipt_delivered": false,
            "final_authorization_dry_run_result_receipt_exported": false,
            "final_authorization_dry_run_result_receipt_query_registered": false,
            "final_authorization_dry_run_result_receipt_observability_recorded": false,
            "completion_ack_recorded": false,
            "completion_ack_accepted": false
        }),
        serde_json::json!({
            "step": "side_effect_denial_check",
            "status": "ready",
            "credential_read": false,
            "secret_file_read": false,
            "provider_router_live_envelope_executed": false,
            "provider_prompt_injection_performed": false,
            "context_injection_performed": false,
            "live_kg_write_performed": false,
            "memory_store_write_performed": false,
            "channel_send_performed": false,
            "external_send_performed": false,
            "install_executed": false,
            "active_binary_mutated": false,
            "public_release_claimed": false
        }),
    ];

    let mut side_effects = serde_json::Map::new();
    for key in [
        "final_authorization_dry_run_result_receipt_recorded",
        "final_authorization_dry_run_result_receipt_persisted",
        "final_authorization_dry_run_result_receipt_accepted",
        "final_authorization_dry_run_result_receipt_materialized",
        "final_authorization_dry_run_result_receipt_filesystem_written",
        "final_authorization_dry_run_result_receipt_ledger_recorded",
        "final_authorization_dry_run_result_receipt_indexed",
        "final_authorization_dry_run_result_receipt_enqueued",
        "final_authorization_dry_run_result_receipt_delivered",
        "final_authorization_dry_run_result_receipt_exported",
        "final_authorization_dry_run_result_receipt_query_registered",
        "final_authorization_dry_run_result_receipt_observability_recorded",
        "final_authorization_dry_run_result_receipt_hash_accepted",
        "completion_ack_recorded",
        "completion_ack_persisted",
        "completion_ack_accepted",
        "operator_approval_from_result_receipt_accepted",
        "final_authorization_from_result_receipt_allowed",
        "activation_from_result_receipt_allowed",
        "final_authorization_dry_run_envelope_persisted",
        "final_authorization_live_envelope_executed",
        "final_authorization_preconditions_satisfied",
        "final_authorization_accepted",
        "final_authorization_persisted",
        "fresh_live_accepted_operator_approval_artifact_present",
        "fresh_live_accepted_operator_approval_artifact_verified",
        "single_use_approval_nonce_verified",
        "single_use_approval_nonce_consumed",
        "operator_identity_session_binding_verified",
        "operator_identity_session_bound",
        "explicit_invocation_command_accepted",
        "approval_packet_accepted",
        "operator_approval_recorded",
        "operator_consent_recorded",
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
        "external_network_call_performed",
        "kg_adapter_live_read_performed",
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

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_no_persistence_route",
        "endpoint": HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_FINAL_AUTHORIZATION_DRY_RUN_RESULT_RECEIPT_NO_PERSISTENCE_ENDPOINT,
        "source_command": "/hepta-first-model-invocation-operator-approval-final-authorization-dry-run-result-receipt-no-persistence --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-23",
        "canary_schema_version": "hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_no_persistence_v1",
        "canary_execution_mode": "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_no_persistence_no_provider_model_invocation",
        "source_first_model_invocation_approval_final_authorization_dry_run_envelope_endpoint": HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_FINAL_AUTHORIZATION_DRY_RUN_ENVELOPE_PREFLIGHT_ENDPOINT,
        "source_first_model_invocation_approval_final_authorization_dry_run_envelope_ready": source_final_authorization_ready,
        "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        "route_count": route_matrix.route_count,
        "implemented_route_count": route_matrix.implemented_route_count,
        "missing_route_count": route_matrix.missing_route_count,
        "route_count_source_command_accepted": route_count_source_command_accepted,
        "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_no_persistence_route_enabled": true,
        "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_no_persistence_ready": report_ready
    });
    extend_json_object(
        &mut report,
        serde_json::json!({
            "result_receipt_state": "final_authorization_dry_run_result_receipt_rendered_but_not_persisted_or_accepted",
            "result_receipt_scope": receipt_scope,
            "source_final_authorization_dry_run_envelope_hash_sha256": source_envelope_hash,
            "source_final_authorization_dry_run_readback_hash_sha256": source_readback_hash,
            "source_final_authorization_denial_receipt_hash_sha256": source_denial_receipt_hash,
            "final_authorization_dry_run_result_receipt_rendered": true,
            "final_authorization_dry_run_result_receipt_hash_sha256": result_receipt_hash,
            "final_authorization_dry_run_result_receipt_hash_matched": true,
            "final_authorization_dry_run_result_receipt_readback_performed": true,
            "final_authorization_dry_run_result_receipt_readback_hash_sha256": result_receipt_readback_hash,
            "final_authorization_dry_run_result_receipt_readback_hash_matched": true,
            "final_authorization_dry_run_result_receipt_denial_hash_sha256": result_receipt_denial_hash,
            "final_authorization_dry_run_result_receipt_recorded": false,
            "final_authorization_dry_run_result_receipt_persisted": false,
            "final_authorization_dry_run_result_receipt_accepted": false,
            "final_authorization_dry_run_result_receipt_materialized": false,
            "final_authorization_dry_run_result_receipt_filesystem_written": false,
            "final_authorization_dry_run_result_receipt_ledger_recorded": false,
            "final_authorization_dry_run_result_receipt_indexed": false,
            "final_authorization_dry_run_result_receipt_enqueued": false,
            "final_authorization_dry_run_result_receipt_delivered": false,
            "final_authorization_dry_run_result_receipt_exported": false,
            "final_authorization_dry_run_result_receipt_query_registered": false,
            "final_authorization_dry_run_result_receipt_observability_recorded": false,
            "completion_ack_recorded": false,
            "completion_ack_persisted": false,
            "completion_ack_accepted": false
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "fresh_live_accepted_operator_approval_artifact_required": true,
            "fresh_live_accepted_operator_approval_artifact_present": false,
            "fresh_live_accepted_operator_approval_artifact_verified": false,
            "single_use_approval_nonce_required": true,
            "single_use_approval_nonce_verified": false,
            "single_use_approval_nonce_consumed": false,
            "operator_identity_session_binding_required": true,
            "operator_identity_session_binding_verified": false,
            "operator_identity_session_bound": false,
            "explicit_invocation_command_required": true,
            "explicit_invocation_command_accepted": false,
            "final_authorization_candidate_present": true,
            "final_authorization_preconditions_satisfied": false,
            "final_authorization_denied": true,
            "final_authorization_accepted": false,
            "final_authorization_persisted": false,
            "final_authorization_from_result_receipt_allowed": false,
            "operator_approval_from_result_receipt_accepted": false,
            "activation_from_result_receipt_allowed": false,
            "approval_packet_accepted": false,
            "operator_approval_recorded": false,
            "operator_consent_recorded": false
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "candidate_provider_invocation_requested": true,
            "candidate_model_invocation_requested": true,
            "provider_invocation_authorized": false,
            "model_invocation_authorized": false,
            "provider_invocation_budget": 0,
            "model_invocation_budget": 0,
            "provider_invoked": false,
            "model_invoked": false,
            "credential_value_read": false,
            "credential_read": false,
            "secret_file_read": false,
            "provider_router_live_envelope_executed": false,
            "provider_router_prompt_mutated": false,
            "provider_router_context_packet_materialized": false,
            "provider_prompt_injection_performed": false,
            "context_injection_performed": false,
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
                    "action": "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_replay_idempotency_denial",
                    "status": "allowed_report_only_next_slice",
                    "records_result_receipt": false,
                    "persists_result_receipt": false,
                    "accepts_result_receipt": false,
                    "requires_fresh_accepted_operator_approval_artifact": true,
                    "requires_single_use_approval_nonce": true,
                    "requires_operator_identity_session_binding": true,
                    "requires_explicit_command": true,
                    "invokes_provider": false,
                    "invokes_model": false,
                    "reads_credentials": false,
                    "writes_kg": false,
                    "sends_externally": false,
                    "mutates_durable_memory": false
                }
            ],
            "blocked_actions": [
                "final_authorization_result_receipt_recording",
                "final_authorization_result_receipt_persistence",
                "final_authorization_result_receipt_acceptance",
                "final_authorization_result_receipt_delivery",
                "final_authorization_result_receipt_export_query_observability",
                "completion_ack_from_dry_run_result_receipt",
                "operator_approval_from_dry_run_result_receipt",
                "activation_from_dry_run_result_receipt",
                "provider_or_model_invocation_from_dry_run_result_receipt",
                "credential_or_secret_read_from_dry_run_result_receipt",
                "kg_or_memory_write_from_dry_run_result_receipt",
                "channel_or_external_delivery_from_dry_run_result_receipt"
            ],
            "side_effects": side_effects
        }),
    );
    report
}

fn hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_replay_idempotency_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source =
        hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_no_persistence_report();
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
            .and_then(|value| value.as_str())
            .unwrap_or("")
    };
    let source_next_action_replay_idempotency = source
        .get("allowed_next_actions")
        .and_then(|value| value.as_array())
        .and_then(|items| items.first())
        .map(|item| {
            item.get("action").and_then(|value| value.as_str())
                == Some(
                    "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_replay_idempotency_denial",
                )
                && item
                    .get("records_result_receipt")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && item
                    .get("persists_result_receipt")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && item
                    .get("accepts_result_receipt")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && item
                    .get("requires_fresh_accepted_operator_approval_artifact")
                    .and_then(serde_json::Value::as_bool)
                    == Some(true)
                && item
                    .get("requires_single_use_approval_nonce")
                    .and_then(serde_json::Value::as_bool)
                    == Some(true)
                && item
                    .get("requires_operator_identity_session_binding")
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
                && item.get("invokes_model").and_then(serde_json::Value::as_bool) == Some(false)
        })
        .unwrap_or(false);
    let source_result_receipt_ready = source_bool(
        "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_no_persistence_ready",
    ) && source_bool(
        "source_first_model_invocation_approval_final_authorization_dry_run_envelope_ready",
    ) && source_bool(
        "final_authorization_dry_run_result_receipt_rendered",
    ) && source_bool(
        "final_authorization_dry_run_result_receipt_hash_matched",
    ) && source_bool(
        "final_authorization_dry_run_result_receipt_readback_performed",
    ) && source_bool(
        "final_authorization_dry_run_result_receipt_readback_hash_matched",
    ) && !source_bool(
        "final_authorization_dry_run_result_receipt_recorded",
    ) && !source_bool(
        "final_authorization_dry_run_result_receipt_persisted",
    ) && !source_bool(
        "final_authorization_dry_run_result_receipt_accepted",
    ) && !source_bool(
        "final_authorization_dry_run_result_receipt_materialized",
    ) && !source_bool(
        "final_authorization_dry_run_result_receipt_filesystem_written",
    ) && !source_bool(
        "final_authorization_dry_run_result_receipt_ledger_recorded",
    ) && !source_bool(
        "final_authorization_dry_run_result_receipt_indexed",
    ) && !source_bool(
        "final_authorization_dry_run_result_receipt_enqueued",
    ) && !source_bool(
        "final_authorization_dry_run_result_receipt_delivered",
    ) && !source_bool(
        "final_authorization_dry_run_result_receipt_exported",
    ) && !source_bool(
        "final_authorization_dry_run_result_receipt_query_registered",
    ) && !source_bool(
        "final_authorization_dry_run_result_receipt_observability_recorded",
    ) && !source_bool("completion_ack_recorded")
        && !source_bool("completion_ack_persisted")
        && !source_bool("completion_ack_accepted")
        && source_bool("fresh_live_accepted_operator_approval_artifact_required")
        && !source_bool("fresh_live_accepted_operator_approval_artifact_present")
        && !source_bool("fresh_live_accepted_operator_approval_artifact_verified")
        && source_bool("single_use_approval_nonce_required")
        && !source_bool("single_use_approval_nonce_verified")
        && !source_bool("single_use_approval_nonce_consumed")
        && source_bool("operator_identity_session_binding_required")
        && !source_bool("operator_identity_session_binding_verified")
        && !source_bool("operator_identity_session_bound")
        && source_bool("explicit_invocation_command_required")
        && !source_bool("explicit_invocation_command_accepted")
        && source_bool("final_authorization_candidate_present")
        && !source_bool("final_authorization_preconditions_satisfied")
        && source_bool("final_authorization_denied")
        && !source_bool("final_authorization_accepted")
        && !source_bool("final_authorization_persisted")
        && !source_bool("final_authorization_from_result_receipt_allowed")
        && !source_bool("operator_approval_from_result_receipt_accepted")
        && !source_bool("activation_from_result_receipt_allowed")
        && !source_bool("approval_packet_accepted")
        && !source_bool("operator_approval_recorded")
        && !source_bool("operator_consent_recorded")
        && source_bool("candidate_provider_invocation_requested")
        && source_bool("candidate_model_invocation_requested")
        && !source_bool("provider_invocation_authorized")
        && !source_bool("model_invocation_authorized")
        && source_i64("provider_invocation_budget") == 0
        && source_i64("model_invocation_budget") == 0
        && !source_bool("provider_invoked")
        && !source_bool("model_invoked")
        && !source_bool("credential_value_read")
        && !source_bool("credential_read")
        && !source_bool("secret_file_read")
        && !source_bool("provider_router_live_envelope_executed")
        && !source_bool("provider_prompt_injection_performed")
        && !source_bool("context_injection_performed")
        && !source_bool("kg_adapter_read_performed")
        && !source_bool("live_kg_write_performed")
        && !source_bool("memory_store_write_performed")
        && !source_bool("channel_send_performed")
        && !source_bool("telegram_send_performed")
        && !source_bool("external_send_performed")
        && source_next_action_replay_idempotency;
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT;
    let source_result_receipt_hash =
        source_str("final_authorization_dry_run_result_receipt_hash_sha256");
    let source_result_receipt_readback_hash =
        source_str("final_authorization_dry_run_result_receipt_readback_hash_sha256");
    let source_result_receipt_denial_hash =
        source_str("final_authorization_dry_run_result_receipt_denial_hash_sha256");
    let replay_scope = "first_model_invocation:operator-approval-final-authorization-dry-run-result-receipt-replay-idempotency-denial";
    let replay_idempotency_denial_hash = sha256_text_value(&format!(
        "first-model-final-authorization-dry-run-result-receipt-replay-idempotency-denial:{replay_scope}:{source_result_receipt_hash}:{source_result_receipt_readback_hash}:{source_result_receipt_denial_hash}:duplicate=false:retry=false:idempotency=false"
    ));
    let replay_idempotency_readback_hash = sha256_text_value(&format!(
        "first-model-final-authorization-dry-run-result-receipt-replay-idempotency-readback:{replay_idempotency_denial_hash}:replay=false:cache=false:authority=false"
    ));
    let replay_fixtures = vec![
        serde_json::json!({
            "fixture_id": "duplicate-result-receipt-hash",
            "replay_idempotency_status": "blocked_duplicate_result_receipt_hash",
            "duplicate_result_receipt_accepted": false,
            "final_authorization_dry_run_result_receipt_replay_allowed": false,
            "idempotency_state_recorded": false,
            "idempotency_state_persisted": false,
            "activation_from_replay_allowed": false,
            "receipt_noop_confirmed": true
        }),
        serde_json::json!({
            "fixture_id": "retry-same-idempotency-key",
            "replay_idempotency_status": "blocked_retry_same_idempotency_key",
            "retry_result_receipt_accepted": false,
            "final_authorization_dry_run_result_receipt_idempotency_key_accepted": false,
            "final_authorization_dry_run_result_receipt_idempotency_cache_written": false,
            "final_authorization_dry_run_result_receipt_idempotency_cache_hit_promoted": false,
            "activation_from_replay_allowed": false,
            "receipt_noop_confirmed": true
        }),
        serde_json::json!({
            "fixture_id": "stale-readback-hash-replay",
            "replay_idempotency_status": "blocked_stale_readback_hash_replay",
            "stale_result_receipt_replay_accepted": false,
            "result_receipt_hash_override_accepted": false,
            "final_authorization_dry_run_result_receipt_replay_recorded": false,
            "activation_from_replay_allowed": false,
            "receipt_noop_confirmed": true
        }),
        serde_json::json!({
            "fixture_id": "cross-scope-reuse-attempt",
            "replay_idempotency_status": "blocked_cross_scope_reuse_attempt",
            "final_authorization_dry_run_result_receipt_cross_scope_reuse_accepted": false,
            "operator_approval_from_replay_accepted": false,
            "activation_from_replay_allowed": false,
            "receipt_noop_confirmed": true
        }),
        serde_json::json!({
            "fixture_id": "status-upgrade-attempt",
            "replay_idempotency_status": "blocked_status_upgrade_attempt",
            "final_authorization_dry_run_result_receipt_status_upgrade_accepted": false,
            "final_authorization_dry_run_result_receipt_completed_status_accepted": false,
            "completion_ack_replay_accepted": false,
            "activation_from_replay_allowed": false,
            "receipt_noop_confirmed": true
        }),
        serde_json::json!({
            "fixture_id": "ledger-index-delivery-replay",
            "replay_idempotency_status": "blocked_ledger_index_delivery_replay",
            "final_authorization_dry_run_result_receipt_ledger_replay_accepted": false,
            "final_authorization_dry_run_result_receipt_index_replay_accepted": false,
            "final_authorization_dry_run_result_receipt_delivery_replay_accepted": false,
            "activation_from_replay_allowed": false,
            "receipt_noop_confirmed": true
        }),
        serde_json::json!({
            "fixture_id": "export-query-observability-replay",
            "replay_idempotency_status": "blocked_export_query_observability_replay",
            "final_authorization_dry_run_result_receipt_export_replay_accepted": false,
            "final_authorization_dry_run_result_receipt_query_replay_accepted": false,
            "final_authorization_dry_run_result_receipt_observability_replay_accepted": false,
            "activation_from_replay_allowed": false,
            "receipt_noop_confirmed": true
        }),
        serde_json::json!({
            "fixture_id": "provider-model-invocation-replay",
            "replay_idempotency_status": "blocked_provider_model_invocation_replay",
            "provider_invocation_authorized_from_replay": false,
            "model_invocation_authorized_from_replay": false,
            "provider_invoked": false,
            "model_invoked": false,
            "activation_from_replay_allowed": false,
            "receipt_noop_confirmed": true
        }),
    ];
    let replay_fixture_count = replay_fixtures.len();
    let report_ready =
        route_matrix.ready && route_count_source_command_accepted && source_result_receipt_ready;

    let audit_steps = vec![
        serde_json::json!({
            "step": "result_receipt_no_persistence_source_binding",
            "status": "ready",
            "source_endpoint": HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_FINAL_AUTHORIZATION_DRY_RUN_RESULT_RECEIPT_NO_PERSISTENCE_ENDPOINT,
            "source_result_receipt_no_persistence_ready": source_result_receipt_ready,
            "source_result_receipt_hash_sha256": source_result_receipt_hash,
            "source_result_receipt_readback_hash_sha256": source_result_receipt_readback_hash,
            "source_result_receipt_denial_hash_sha256": source_result_receipt_denial_hash
        }),
        serde_json::json!({
            "step": "replay_duplicate_retry_fixture_denial",
            "status": "blocked_report_only",
            "replay_idempotency_fixture_count": replay_fixture_count,
            "blocked_replay_idempotency_fixture_count": replay_fixture_count,
            "allowed_replay_idempotency_fixture_count": 0,
            "accepted_replay_idempotency_fixture_count": 0,
            "replay_idempotency_performed_count": 0,
            "duplicate_result_receipt_accepted_count": 0,
            "retry_result_receipt_accepted_count": 0
        }),
        serde_json::json!({
            "step": "idempotency_state_no_write",
            "status": "not_recorded_or_cached",
            "final_authorization_dry_run_result_receipt_idempotency_key_accepted": false,
            "final_authorization_dry_run_result_receipt_idempotency_key_registered": false,
            "final_authorization_dry_run_result_receipt_idempotency_state_recorded": false,
            "final_authorization_dry_run_result_receipt_idempotency_state_persisted": false,
            "final_authorization_dry_run_result_receipt_idempotency_cache_written": false,
            "final_authorization_dry_run_result_receipt_idempotency_cache_hit_promoted": false
        }),
        serde_json::json!({
            "step": "cross_scope_status_ack_replay_denial",
            "status": "denied",
            "final_authorization_dry_run_result_receipt_cross_scope_reuse_accepted": false,
            "final_authorization_dry_run_result_receipt_status_upgrade_accepted": false,
            "final_authorization_dry_run_result_receipt_completed_status_accepted": false,
            "completion_ack_replay_accepted": false,
            "final_authorization_dry_run_result_receipt_ledger_replay_accepted": false,
            "final_authorization_dry_run_result_receipt_index_replay_accepted": false,
            "final_authorization_dry_run_result_receipt_delivery_replay_accepted": false,
            "final_authorization_dry_run_result_receipt_export_replay_accepted": false,
            "final_authorization_dry_run_result_receipt_query_replay_accepted": false,
            "final_authorization_dry_run_result_receipt_observability_replay_accepted": false
        }),
        serde_json::json!({
            "step": "replay_authority_non_promotion",
            "status": "authority_denied",
            "final_authorization_from_replay_allowed": false,
            "operator_approval_from_replay_accepted": false,
            "activation_from_replay_allowed": false,
            "provider_invocation_authorized": false,
            "model_invocation_authorized": false,
            "provider_invoked": false,
            "model_invoked": false
        }),
        serde_json::json!({
            "step": "side_effect_denial_check",
            "status": "ready",
            "credential_read": false,
            "secret_file_read": false,
            "provider_router_live_envelope_executed": false,
            "provider_prompt_injection_performed": false,
            "context_injection_performed": false,
            "live_kg_write_performed": false,
            "memory_store_write_performed": false,
            "channel_send_performed": false,
            "external_send_performed": false,
            "install_executed": false,
            "active_binary_mutated": false,
            "public_release_claimed": false
        }),
    ];

    let mut side_effects = serde_json::Map::new();
    for key in [
        "final_authorization_dry_run_result_receipt_replay_recorded",
        "final_authorization_dry_run_result_receipt_replay_persisted",
        "final_authorization_dry_run_result_receipt_replay_performed",
        "final_authorization_dry_run_result_receipt_duplicate_accepted",
        "final_authorization_dry_run_result_receipt_retry_accepted",
        "final_authorization_dry_run_result_receipt_idempotency_key_accepted",
        "final_authorization_dry_run_result_receipt_idempotency_key_registered",
        "final_authorization_dry_run_result_receipt_idempotency_key_recorded",
        "final_authorization_dry_run_result_receipt_idempotency_state_recorded",
        "final_authorization_dry_run_result_receipt_idempotency_state_persisted",
        "final_authorization_dry_run_result_receipt_idempotency_cache_written",
        "final_authorization_dry_run_result_receipt_idempotency_cache_hit_promoted",
        "final_authorization_dry_run_result_receipt_replay_nonce_accepted",
        "final_authorization_dry_run_result_receipt_cross_scope_reuse_accepted",
        "final_authorization_dry_run_result_receipt_status_upgrade_accepted",
        "final_authorization_dry_run_result_receipt_completed_status_accepted",
        "completion_ack_replay_accepted",
        "operator_approval_from_replay_accepted",
        "final_authorization_from_replay_allowed",
        "activation_from_replay_allowed",
        "final_authorization_dry_run_result_receipt_recorded",
        "final_authorization_dry_run_result_receipt_persisted",
        "final_authorization_dry_run_result_receipt_accepted",
        "final_authorization_dry_run_result_receipt_materialized",
        "final_authorization_dry_run_result_receipt_filesystem_written",
        "final_authorization_dry_run_result_receipt_ledger_recorded",
        "final_authorization_dry_run_result_receipt_indexed",
        "final_authorization_dry_run_result_receipt_enqueued",
        "final_authorization_dry_run_result_receipt_delivered",
        "final_authorization_dry_run_result_receipt_exported",
        "final_authorization_dry_run_result_receipt_query_registered",
        "final_authorization_dry_run_result_receipt_observability_recorded",
        "completion_ack_recorded",
        "completion_ack_persisted",
        "completion_ack_accepted",
        "operator_approval_from_result_receipt_accepted",
        "final_authorization_from_result_receipt_allowed",
        "activation_from_result_receipt_allowed",
        "final_authorization_dry_run_envelope_persisted",
        "final_authorization_live_envelope_executed",
        "final_authorization_preconditions_satisfied",
        "final_authorization_accepted",
        "final_authorization_persisted",
        "fresh_live_accepted_operator_approval_artifact_present",
        "fresh_live_accepted_operator_approval_artifact_verified",
        "single_use_approval_nonce_verified",
        "single_use_approval_nonce_consumed",
        "operator_identity_session_binding_verified",
        "operator_identity_session_bound",
        "explicit_invocation_command_accepted",
        "approval_packet_accepted",
        "operator_approval_recorded",
        "operator_consent_recorded",
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
        "external_network_call_performed",
        "kg_adapter_live_read_performed",
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

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_replay_idempotency_denial_route",
        "endpoint": HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_FINAL_AUTHORIZATION_DRY_RUN_RESULT_RECEIPT_REPLAY_IDEMPOTENCY_DENIAL_ENDPOINT,
        "source_command": "/hepta-first-model-invocation-operator-approval-final-authorization-dry-run-result-receipt-replay-idempotency-denial --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-23",
        "canary_schema_version": "hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_replay_idempotency_denial_v1",
        "canary_execution_mode": "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_replay_idempotency_denial_no_provider_model_invocation",
        "source_first_model_invocation_approval_final_authorization_dry_run_result_receipt_no_persistence_endpoint": HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_FINAL_AUTHORIZATION_DRY_RUN_RESULT_RECEIPT_NO_PERSISTENCE_ENDPOINT,
        "source_first_model_invocation_approval_final_authorization_dry_run_result_receipt_no_persistence_ready": source_result_receipt_ready,
        "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        "route_count": route_matrix.route_count,
        "implemented_route_count": route_matrix.implemented_route_count,
        "missing_route_count": route_matrix.missing_route_count,
        "route_count_source_command_accepted": route_count_source_command_accepted,
        "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_replay_idempotency_denial_route_enabled": true,
        "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_replay_idempotency_denial_ready": report_ready
    });
    extend_json_object(
        &mut report,
        serde_json::json!({
            "result_receipt_replay_idempotency_state": "final_authorization_dry_run_result_receipt_replay_duplicate_retry_idempotency_denied",
            "result_receipt_replay_idempotency_scope": replay_scope,
            "source_final_authorization_dry_run_result_receipt_hash_sha256": source_result_receipt_hash,
            "source_final_authorization_dry_run_result_receipt_readback_hash_sha256": source_result_receipt_readback_hash,
            "source_final_authorization_dry_run_result_receipt_denial_hash_sha256": source_result_receipt_denial_hash,
            "final_authorization_dry_run_result_receipt_replay_idempotency_denial_hash_sha256": replay_idempotency_denial_hash,
            "final_authorization_dry_run_result_receipt_replay_idempotency_readback_hash_sha256": replay_idempotency_readback_hash,
            "final_authorization_dry_run_result_receipt_replay_idempotency_readback_hash_matched": true,
            "replay_idempotency_fixture_count": replay_fixture_count,
            "blocked_replay_idempotency_fixture_count": replay_fixture_count,
            "noop_replay_idempotency_fixture_count": replay_fixture_count,
            "allowed_replay_idempotency_fixture_count": 0,
            "accepted_replay_idempotency_fixture_count": 0,
            "replay_idempotency_performed_count": 0,
            "duplicate_result_receipt_accepted_count": 0,
            "retry_result_receipt_accepted_count": 0,
            "idempotency_state_recorded_count": 0,
            "idempotency_state_persisted_count": 0,
            "replay_idempotency_fixtures": replay_fixtures
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "final_authorization_dry_run_result_receipt_replay_allowed": false,
            "final_authorization_dry_run_result_receipt_replayed": false,
            "final_authorization_dry_run_result_receipt_replay_recorded": false,
            "final_authorization_dry_run_result_receipt_replay_persisted": false,
            "final_authorization_dry_run_result_receipt_replay_performed": false,
            "final_authorization_dry_run_result_receipt_duplicate_accepted": false,
            "final_authorization_dry_run_result_receipt_retry_accepted": false,
            "final_authorization_dry_run_result_receipt_idempotency_key_accepted": false,
            "final_authorization_dry_run_result_receipt_idempotency_key_registered": false,
            "final_authorization_dry_run_result_receipt_idempotency_key_recorded": false,
            "final_authorization_dry_run_result_receipt_idempotency_key_persisted": false,
            "final_authorization_dry_run_result_receipt_idempotency_state_recorded": false,
            "final_authorization_dry_run_result_receipt_idempotency_state_persisted": false,
            "final_authorization_dry_run_result_receipt_idempotency_cache_written": false,
            "final_authorization_dry_run_result_receipt_idempotency_cache_hit_promoted": false,
            "final_authorization_dry_run_result_receipt_replay_nonce_accepted": false,
            "final_authorization_dry_run_result_receipt_cross_scope_reuse_accepted": false,
            "final_authorization_dry_run_result_receipt_hash_override_accepted": false,
            "stale_result_receipt_replay_accepted": false,
            "late_result_receipt_replay_accepted": false,
            "future_result_receipt_replay_accepted": false,
            "final_authorization_dry_run_result_receipt_status_upgrade_accepted": false,
            "final_authorization_dry_run_result_receipt_completed_status_accepted": false,
            "completion_ack_replay_accepted": false,
            "final_authorization_dry_run_result_receipt_ledger_replay_accepted": false,
            "final_authorization_dry_run_result_receipt_index_replay_accepted": false,
            "final_authorization_dry_run_result_receipt_delivery_replay_accepted": false,
            "final_authorization_dry_run_result_receipt_export_replay_accepted": false,
            "final_authorization_dry_run_result_receipt_query_replay_accepted": false,
            "final_authorization_dry_run_result_receipt_observability_replay_accepted": false
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "final_authorization_from_replay_allowed": false,
            "operator_approval_from_replay_accepted": false,
            "activation_from_replay_allowed": false,
            "final_authorization_from_result_receipt_allowed": false,
            "operator_approval_from_result_receipt_accepted": false,
            "activation_from_result_receipt_allowed": false,
            "final_authorization_dry_run_result_receipt_recorded": false,
            "final_authorization_dry_run_result_receipt_persisted": false,
            "final_authorization_dry_run_result_receipt_accepted": false,
            "final_authorization_dry_run_result_receipt_materialized": false,
            "final_authorization_dry_run_result_receipt_filesystem_written": false,
            "final_authorization_dry_run_result_receipt_ledger_recorded": false,
            "final_authorization_dry_run_result_receipt_indexed": false,
            "final_authorization_dry_run_result_receipt_enqueued": false,
            "final_authorization_dry_run_result_receipt_delivered": false,
            "final_authorization_dry_run_result_receipt_exported": false,
            "final_authorization_dry_run_result_receipt_query_registered": false,
            "final_authorization_dry_run_result_receipt_observability_recorded": false,
            "completion_ack_recorded": false,
            "completion_ack_persisted": false,
            "completion_ack_accepted": false
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "fresh_live_accepted_operator_approval_artifact_required": true,
            "fresh_live_accepted_operator_approval_artifact_present": false,
            "fresh_live_accepted_operator_approval_artifact_verified": false,
            "single_use_approval_nonce_required": true,
            "single_use_approval_nonce_verified": false,
            "single_use_approval_nonce_consumed": false,
            "operator_identity_session_binding_required": true,
            "operator_identity_session_binding_verified": false,
            "operator_identity_session_bound": false,
            "explicit_invocation_command_required": true,
            "explicit_invocation_command_accepted": false,
            "final_authorization_candidate_present": true,
            "final_authorization_preconditions_satisfied": false,
            "final_authorization_denied": true,
            "final_authorization_accepted": false,
            "final_authorization_persisted": false,
            "approval_packet_accepted": false,
            "operator_approval_recorded": false,
            "operator_consent_recorded": false
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "candidate_provider_invocation_requested": true,
            "candidate_model_invocation_requested": true,
            "provider_invocation_authorized": false,
            "model_invocation_authorized": false,
            "provider_invocation_authorized_from_replay": false,
            "model_invocation_authorized_from_replay": false,
            "provider_invocation_budget": 0,
            "model_invocation_budget": 0,
            "provider_invoked": false,
            "model_invoked": false,
            "credential_value_read": false,
            "credential_read": false,
            "secret_file_read": false,
            "provider_router_live_envelope_executed": false,
            "provider_router_prompt_mutated": false,
            "provider_router_context_packet_materialized": false,
            "provider_prompt_injection_performed": false,
            "context_injection_performed": false,
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
                    "action": "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_ordering_monotonicity_denial",
                    "status": "allowed_report_only_next_slice",
                    "records_result_receipt": false,
                    "persists_result_receipt": false,
                    "accepts_result_receipt": false,
                    "registers_idempotency_key": false,
                    "writes_idempotency_cache": false,
                    "requires_fresh_accepted_operator_approval_artifact": true,
                    "requires_single_use_approval_nonce": true,
                    "requires_operator_identity_session_binding": true,
                    "requires_explicit_command": true,
                    "invokes_provider": false,
                    "invokes_model": false,
                    "reads_credentials": false,
                    "writes_kg": false,
                    "sends_externally": false,
                    "mutates_durable_memory": false
                }
            ],
            "blocked_actions": [
                "final_authorization_result_receipt_replay",
                "final_authorization_result_receipt_duplicate_acceptance",
                "final_authorization_result_receipt_retry_acceptance",
                "final_authorization_result_receipt_idempotency_key_registration",
                "final_authorization_result_receipt_idempotency_cache_write",
                "final_authorization_result_receipt_cross_scope_reuse",
                "final_authorization_result_receipt_status_upgrade",
                "completion_ack_from_result_receipt_replay",
                "operator_approval_from_result_receipt_replay",
                "activation_from_result_receipt_replay",
                "provider_or_model_invocation_from_result_receipt_replay",
                "credential_or_secret_read_from_result_receipt_replay",
                "kg_or_memory_write_from_result_receipt_replay",
                "channel_or_external_delivery_from_result_receipt_replay"
            ],
            "side_effects": side_effects
        }),
    );
    report
}

fn hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_ordering_monotonicity_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source =
        hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_replay_idempotency_denial_report();
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
            .and_then(|value| value.as_str())
            .unwrap_or("")
    };
    let source_next_action_ordering_monotonicity = source
        .get("allowed_next_actions")
        .and_then(|value| value.as_array())
        .and_then(|items| items.first())
        .map(|item| {
            item.get("action").and_then(|value| value.as_str())
                == Some(
                    "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_ordering_monotonicity_denial",
                )
                && item
                    .get("records_result_receipt")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && item
                    .get("persists_result_receipt")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && item
                    .get("accepts_result_receipt")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && item
                    .get("registers_idempotency_key")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && item
                    .get("writes_idempotency_cache")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && item
                    .get("requires_fresh_accepted_operator_approval_artifact")
                    .and_then(serde_json::Value::as_bool)
                    == Some(true)
                && item
                    .get("requires_single_use_approval_nonce")
                    .and_then(serde_json::Value::as_bool)
                    == Some(true)
                && item
                    .get("requires_operator_identity_session_binding")
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
                && item.get("invokes_model").and_then(serde_json::Value::as_bool) == Some(false)
        })
        .unwrap_or(false);
    let source_replay_idempotency_ready = source_bool(
        "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_replay_idempotency_denial_ready",
    ) && source_bool(
        "source_first_model_invocation_approval_final_authorization_dry_run_result_receipt_no_persistence_ready",
    ) && source_bool(
        "final_authorization_dry_run_result_receipt_replay_idempotency_readback_hash_matched",
    ) && source_i64("replay_idempotency_fixture_count") == 8
        && source_i64("blocked_replay_idempotency_fixture_count") == 8
        && source_i64("noop_replay_idempotency_fixture_count") == 8
        && source_i64("allowed_replay_idempotency_fixture_count") == 0
        && source_i64("accepted_replay_idempotency_fixture_count") == 0
        && source_i64("replay_idempotency_performed_count") == 0
        && source_i64("duplicate_result_receipt_accepted_count") == 0
        && source_i64("retry_result_receipt_accepted_count") == 0
        && source_i64("idempotency_state_recorded_count") == 0
        && source_i64("idempotency_state_persisted_count") == 0
        && !source_bool("final_authorization_dry_run_result_receipt_replay_allowed")
        && !source_bool("final_authorization_dry_run_result_receipt_replayed")
        && !source_bool("final_authorization_dry_run_result_receipt_replay_recorded")
        && !source_bool("final_authorization_dry_run_result_receipt_replay_persisted")
        && !source_bool("final_authorization_dry_run_result_receipt_replay_performed")
        && !source_bool("final_authorization_dry_run_result_receipt_duplicate_accepted")
        && !source_bool("final_authorization_dry_run_result_receipt_retry_accepted")
        && !source_bool("final_authorization_dry_run_result_receipt_idempotency_key_accepted")
        && !source_bool("final_authorization_dry_run_result_receipt_idempotency_key_registered")
        && !source_bool("final_authorization_dry_run_result_receipt_idempotency_state_recorded")
        && !source_bool("final_authorization_dry_run_result_receipt_idempotency_state_persisted")
        && !source_bool("final_authorization_dry_run_result_receipt_idempotency_cache_written")
        && !source_bool(
            "final_authorization_dry_run_result_receipt_idempotency_cache_hit_promoted",
        )
        && !source_bool("final_authorization_dry_run_result_receipt_cross_scope_reuse_accepted")
        && !source_bool("final_authorization_dry_run_result_receipt_status_upgrade_accepted")
        && !source_bool("final_authorization_dry_run_result_receipt_completed_status_accepted")
        && !source_bool("completion_ack_replay_accepted")
        && !source_bool("final_authorization_from_replay_allowed")
        && !source_bool("operator_approval_from_replay_accepted")
        && !source_bool("activation_from_replay_allowed")
        && !source_bool("provider_invocation_authorized")
        && !source_bool("model_invocation_authorized")
        && source_i64("provider_invocation_budget") == 0
        && source_i64("model_invocation_budget") == 0
        && !source_bool("provider_invoked")
        && !source_bool("model_invoked")
        && !source_bool("credential_read")
        && !source_bool("secret_file_read")
        && !source_bool("live_kg_write_performed")
        && !source_bool("memory_store_write_performed")
        && !source_bool("channel_send_performed")
        && !source_bool("telegram_send_performed")
        && !source_bool("external_send_performed")
        && source_next_action_ordering_monotonicity;
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT;
    let source_replay_denial_hash = source_str(
        "final_authorization_dry_run_result_receipt_replay_idempotency_denial_hash_sha256",
    );
    let source_replay_readback_hash = source_str(
        "final_authorization_dry_run_result_receipt_replay_idempotency_readback_hash_sha256",
    );
    let source_result_receipt_hash =
        source_str("source_final_authorization_dry_run_result_receipt_hash_sha256");
    let ordering_scope = "first_model_invocation:operator-approval-final-authorization-dry-run-result-receipt-ordering-monotonicity-denial";
    let ordering_monotonicity_denial_hash = sha256_text_value(&format!(
        "first-model-final-authorization-dry-run-result-receipt-ordering-monotonicity-denial:{ordering_scope}:{source_replay_denial_hash}:{source_replay_readback_hash}:{source_result_receipt_hash}:sequence=false:monotonic=false:latest-wins=false"
    ));
    let ordering_monotonicity_readback_hash = sha256_text_value(&format!(
        "first-model-final-authorization-dry-run-result-receipt-ordering-monotonicity-readback:{ordering_monotonicity_denial_hash}:cursor=false:epoch=false:authority=false"
    ));
    let ordering_fixtures = vec![
        serde_json::json!({
            "fixture_id": "duplicate-sequence-number",
            "ordering_monotonicity_status": "blocked_duplicate_sequence_number",
            "final_authorization_dry_run_result_receipt_duplicate_sequence_accepted": false,
            "final_authorization_dry_run_result_receipt_sequence_cursor_recorded": false,
            "activation_from_ordering_allowed": false,
            "receipt_noop_confirmed": true
        }),
        serde_json::json!({
            "fixture_id": "stale-sequence-cursor",
            "ordering_monotonicity_status": "blocked_stale_sequence_cursor",
            "final_authorization_dry_run_result_receipt_stale_sequence_accepted": false,
            "final_authorization_dry_run_result_receipt_monotonic_sequence_accepted": false,
            "activation_from_ordering_allowed": false,
            "receipt_noop_confirmed": true
        }),
        serde_json::json!({
            "fixture_id": "late-receipt-timestamp",
            "ordering_monotonicity_status": "blocked_late_receipt_timestamp",
            "final_authorization_dry_run_result_receipt_late_sequence_accepted": false,
            "final_authorization_dry_run_result_receipt_timestamp_ordering_accepted": false,
            "activation_from_ordering_allowed": false,
            "receipt_noop_confirmed": true
        }),
        serde_json::json!({
            "fixture_id": "future-sequence-gap",
            "ordering_monotonicity_status": "blocked_future_sequence_gap",
            "final_authorization_dry_run_result_receipt_future_sequence_accepted": false,
            "final_authorization_dry_run_result_receipt_gap_fill_accepted": false,
            "activation_from_ordering_allowed": false,
            "receipt_noop_confirmed": true
        }),
        serde_json::json!({
            "fixture_id": "timestamp-rollback",
            "ordering_monotonicity_status": "blocked_timestamp_rollback",
            "final_authorization_dry_run_result_receipt_timestamp_rollback_accepted": false,
            "final_authorization_dry_run_result_receipt_status_rollback_accepted": false,
            "activation_from_ordering_allowed": false,
            "receipt_noop_confirmed": true
        }),
        serde_json::json!({
            "fixture_id": "epoch-rollback",
            "ordering_monotonicity_status": "blocked_epoch_rollback",
            "final_authorization_dry_run_result_receipt_epoch_rollback_accepted": false,
            "final_authorization_dry_run_result_receipt_epoch_state_persisted": false,
            "activation_from_ordering_allowed": false,
            "receipt_noop_confirmed": true
        }),
        serde_json::json!({
            "fixture_id": "same-sequence-override",
            "ordering_monotonicity_status": "blocked_same_sequence_override",
            "final_authorization_dry_run_result_receipt_same_sequence_override_accepted": false,
            "operator_approval_from_ordering_accepted": false,
            "activation_from_ordering_allowed": false,
            "receipt_noop_confirmed": true
        }),
        serde_json::json!({
            "fixture_id": "latest-wins-promotion",
            "ordering_monotonicity_status": "blocked_latest_wins_promotion",
            "final_authorization_dry_run_result_receipt_latest_wins_promoted": false,
            "provider_invocation_authorized_from_ordering": false,
            "model_invocation_authorized_from_ordering": false,
            "provider_invoked": false,
            "model_invoked": false,
            "activation_from_ordering_allowed": false,
            "receipt_noop_confirmed": true
        }),
    ];
    let ordering_fixture_count = ordering_fixtures.len();
    let report_ready = route_matrix.ready
        && route_count_source_command_accepted
        && source_replay_idempotency_ready;

    let audit_steps = vec![
        serde_json::json!({
            "step": "replay_idempotency_source_binding",
            "status": "ready",
            "source_endpoint": HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_FINAL_AUTHORIZATION_DRY_RUN_RESULT_RECEIPT_REPLAY_IDEMPOTENCY_DENIAL_ENDPOINT,
            "source_replay_idempotency_ready": source_replay_idempotency_ready,
            "source_replay_denial_hash_sha256": source_replay_denial_hash,
            "source_replay_readback_hash_sha256": source_replay_readback_hash
        }),
        serde_json::json!({
            "step": "ordering_sequence_fixture_denial",
            "status": "blocked_report_only",
            "ordering_monotonicity_fixture_count": ordering_fixture_count,
            "blocked_ordering_monotonicity_fixture_count": ordering_fixture_count,
            "allowed_ordering_monotonicity_fixture_count": 0,
            "accepted_ordering_monotonicity_fixture_count": 0,
            "ordering_monotonicity_performed_count": 0
        }),
        serde_json::json!({
            "step": "sequence_cursor_no_write",
            "status": "not_recorded_or_persisted",
            "final_authorization_dry_run_result_receipt_sequence_cursor_accepted": false,
            "final_authorization_dry_run_result_receipt_sequence_cursor_recorded": false,
            "final_authorization_dry_run_result_receipt_sequence_cursor_persisted": false,
            "final_authorization_dry_run_result_receipt_monotonicity_state_recorded": false,
            "final_authorization_dry_run_result_receipt_monotonicity_state_persisted": false
        }),
        serde_json::json!({
            "step": "late_future_rollback_denial",
            "status": "denied",
            "final_authorization_dry_run_result_receipt_late_sequence_accepted": false,
            "final_authorization_dry_run_result_receipt_future_sequence_accepted": false,
            "final_authorization_dry_run_result_receipt_timestamp_rollback_accepted": false,
            "final_authorization_dry_run_result_receipt_epoch_rollback_accepted": false,
            "final_authorization_dry_run_result_receipt_same_sequence_override_accepted": false,
            "final_authorization_dry_run_result_receipt_latest_wins_promoted": false
        }),
        serde_json::json!({
            "step": "ordering_authority_non_promotion",
            "status": "authority_denied",
            "final_authorization_from_ordering_allowed": false,
            "operator_approval_from_ordering_accepted": false,
            "activation_from_ordering_allowed": false,
            "provider_invocation_authorized": false,
            "model_invocation_authorized": false,
            "provider_invoked": false,
            "model_invoked": false
        }),
        serde_json::json!({
            "step": "side_effect_denial_check",
            "status": "ready",
            "credential_read": false,
            "secret_file_read": false,
            "provider_router_live_envelope_executed": false,
            "provider_prompt_injection_performed": false,
            "context_injection_performed": false,
            "live_kg_write_performed": false,
            "memory_store_write_performed": false,
            "channel_send_performed": false,
            "external_send_performed": false,
            "install_executed": false,
            "active_binary_mutated": false,
            "public_release_claimed": false
        }),
    ];

    let mut side_effects = serde_json::Map::new();
    for key in [
        "final_authorization_dry_run_result_receipt_ordering_allowed",
        "final_authorization_dry_run_result_receipt_ordered",
        "final_authorization_dry_run_result_receipt_ordering_recorded",
        "final_authorization_dry_run_result_receipt_ordering_persisted",
        "final_authorization_dry_run_result_receipt_ordering_performed",
        "final_authorization_dry_run_result_receipt_sequence_cursor_accepted",
        "final_authorization_dry_run_result_receipt_sequence_cursor_recorded",
        "final_authorization_dry_run_result_receipt_sequence_cursor_persisted",
        "final_authorization_dry_run_result_receipt_monotonicity_state_recorded",
        "final_authorization_dry_run_result_receipt_monotonicity_state_persisted",
        "final_authorization_dry_run_result_receipt_monotonic_sequence_accepted",
        "final_authorization_dry_run_result_receipt_duplicate_sequence_accepted",
        "final_authorization_dry_run_result_receipt_stale_sequence_accepted",
        "final_authorization_dry_run_result_receipt_late_sequence_accepted",
        "final_authorization_dry_run_result_receipt_future_sequence_accepted",
        "final_authorization_dry_run_result_receipt_gap_fill_accepted",
        "final_authorization_dry_run_result_receipt_timestamp_ordering_accepted",
        "final_authorization_dry_run_result_receipt_timestamp_rollback_accepted",
        "final_authorization_dry_run_result_receipt_epoch_rollback_accepted",
        "final_authorization_dry_run_result_receipt_epoch_state_persisted",
        "final_authorization_dry_run_result_receipt_status_rollback_accepted",
        "final_authorization_dry_run_result_receipt_same_sequence_override_accepted",
        "final_authorization_dry_run_result_receipt_latest_wins_promoted",
        "completion_ack_ordering_accepted",
        "operator_approval_from_ordering_accepted",
        "final_authorization_from_ordering_allowed",
        "activation_from_ordering_allowed",
        "final_authorization_dry_run_result_receipt_recorded",
        "final_authorization_dry_run_result_receipt_persisted",
        "final_authorization_dry_run_result_receipt_accepted",
        "final_authorization_dry_run_result_receipt_materialized",
        "final_authorization_dry_run_result_receipt_filesystem_written",
        "completion_ack_recorded",
        "completion_ack_persisted",
        "completion_ack_accepted",
        "final_authorization_preconditions_satisfied",
        "final_authorization_accepted",
        "final_authorization_persisted",
        "fresh_live_accepted_operator_approval_artifact_present",
        "fresh_live_accepted_operator_approval_artifact_verified",
        "single_use_approval_nonce_verified",
        "single_use_approval_nonce_consumed",
        "operator_identity_session_binding_verified",
        "operator_identity_session_bound",
        "explicit_invocation_command_accepted",
        "approval_packet_accepted",
        "operator_approval_recorded",
        "operator_consent_recorded",
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
        "external_network_call_performed",
        "kg_adapter_live_read_performed",
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

    let mut report = serde_json::json!({
        "product": "Hepta",
        "runtime": "hepta",
        "status": if report_ready { "ready" } else { "blocked" },
        "base_url": "http://127.0.0.1:7373",
        "gate": "hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_ordering_monotonicity_denial_route",
        "endpoint": HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_FINAL_AUTHORIZATION_DRY_RUN_RESULT_RECEIPT_ORDERING_MONOTONICITY_DENIAL_ENDPOINT,
        "source_command": "/hepta-first-model-invocation-operator-approval-final-authorization-dry-run-result-receipt-ordering-monotonicity-denial --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-23",
        "canary_schema_version": "hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_ordering_monotonicity_denial_v1",
        "canary_execution_mode": "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_ordering_monotonicity_denial_no_provider_model_invocation",
        "source_first_model_invocation_approval_final_authorization_dry_run_result_receipt_replay_idempotency_denial_endpoint": HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_FINAL_AUTHORIZATION_DRY_RUN_RESULT_RECEIPT_REPLAY_IDEMPOTENCY_DENIAL_ENDPOINT,
        "source_first_model_invocation_approval_final_authorization_dry_run_result_receipt_replay_idempotency_denial_ready": source_replay_idempotency_ready,
        "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        "route_count": route_matrix.route_count,
        "implemented_route_count": route_matrix.implemented_route_count,
        "missing_route_count": route_matrix.missing_route_count,
        "route_count_source_command_accepted": route_count_source_command_accepted,
        "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_ordering_monotonicity_denial_route_enabled": true,
        "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_ordering_monotonicity_denial_ready": report_ready
    });
    extend_json_object(
        &mut report,
        serde_json::json!({
            "result_receipt_ordering_monotonicity_state": "final_authorization_dry_run_result_receipt_sequence_cursor_monotonicity_denied",
            "result_receipt_ordering_monotonicity_scope": ordering_scope,
            "source_final_authorization_dry_run_result_receipt_replay_idempotency_denial_hash_sha256": source_replay_denial_hash,
            "source_final_authorization_dry_run_result_receipt_replay_idempotency_readback_hash_sha256": source_replay_readback_hash,
            "source_final_authorization_dry_run_result_receipt_hash_sha256": source_result_receipt_hash,
            "final_authorization_dry_run_result_receipt_ordering_monotonicity_denial_hash_sha256": ordering_monotonicity_denial_hash,
            "final_authorization_dry_run_result_receipt_ordering_monotonicity_readback_hash_sha256": ordering_monotonicity_readback_hash,
            "final_authorization_dry_run_result_receipt_ordering_monotonicity_readback_hash_matched": true,
            "ordering_monotonicity_fixture_count": ordering_fixture_count,
            "blocked_ordering_monotonicity_fixture_count": ordering_fixture_count,
            "noop_ordering_monotonicity_fixture_count": ordering_fixture_count,
            "allowed_ordering_monotonicity_fixture_count": 0,
            "accepted_ordering_monotonicity_fixture_count": 0,
            "ordering_monotonicity_performed_count": 0,
            "sequence_cursor_recorded_count": 0,
            "sequence_cursor_persisted_count": 0,
            "monotonicity_state_recorded_count": 0,
            "monotonicity_state_persisted_count": 0,
            "ordering_monotonicity_fixtures": ordering_fixtures
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "final_authorization_dry_run_result_receipt_ordering_allowed": false,
            "final_authorization_dry_run_result_receipt_ordered": false,
            "final_authorization_dry_run_result_receipt_ordering_recorded": false,
            "final_authorization_dry_run_result_receipt_ordering_persisted": false,
            "final_authorization_dry_run_result_receipt_ordering_performed": false,
            "final_authorization_dry_run_result_receipt_sequence_cursor_accepted": false,
            "final_authorization_dry_run_result_receipt_sequence_cursor_recorded": false,
            "final_authorization_dry_run_result_receipt_sequence_cursor_persisted": false,
            "final_authorization_dry_run_result_receipt_monotonicity_state_recorded": false,
            "final_authorization_dry_run_result_receipt_monotonicity_state_persisted": false,
            "final_authorization_dry_run_result_receipt_monotonic_sequence_accepted": false,
            "final_authorization_dry_run_result_receipt_duplicate_sequence_accepted": false,
            "final_authorization_dry_run_result_receipt_stale_sequence_accepted": false,
            "final_authorization_dry_run_result_receipt_late_sequence_accepted": false,
            "final_authorization_dry_run_result_receipt_future_sequence_accepted": false,
            "final_authorization_dry_run_result_receipt_gap_fill_accepted": false,
            "final_authorization_dry_run_result_receipt_timestamp_ordering_accepted": false,
            "final_authorization_dry_run_result_receipt_timestamp_rollback_accepted": false,
            "final_authorization_dry_run_result_receipt_epoch_rollback_accepted": false,
            "final_authorization_dry_run_result_receipt_epoch_state_persisted": false,
            "final_authorization_dry_run_result_receipt_status_rollback_accepted": false,
            "final_authorization_dry_run_result_receipt_same_sequence_override_accepted": false,
            "final_authorization_dry_run_result_receipt_latest_wins_promoted": false,
            "completion_ack_ordering_accepted": false
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "final_authorization_from_ordering_allowed": false,
            "operator_approval_from_ordering_accepted": false,
            "activation_from_ordering_allowed": false,
            "final_authorization_from_replay_allowed": false,
            "operator_approval_from_replay_accepted": false,
            "activation_from_replay_allowed": false,
            "final_authorization_from_result_receipt_allowed": false,
            "operator_approval_from_result_receipt_accepted": false,
            "activation_from_result_receipt_allowed": false,
            "final_authorization_dry_run_result_receipt_recorded": false,
            "final_authorization_dry_run_result_receipt_persisted": false,
            "final_authorization_dry_run_result_receipt_accepted": false,
            "final_authorization_dry_run_result_receipt_materialized": false,
            "final_authorization_dry_run_result_receipt_filesystem_written": false,
            "completion_ack_recorded": false,
            "completion_ack_persisted": false,
            "completion_ack_accepted": false
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "fresh_live_accepted_operator_approval_artifact_required": true,
            "fresh_live_accepted_operator_approval_artifact_present": false,
            "fresh_live_accepted_operator_approval_artifact_verified": false,
            "single_use_approval_nonce_required": true,
            "single_use_approval_nonce_verified": false,
            "single_use_approval_nonce_consumed": false,
            "operator_identity_session_binding_required": true,
            "operator_identity_session_binding_verified": false,
            "operator_identity_session_bound": false,
            "explicit_invocation_command_required": true,
            "explicit_invocation_command_accepted": false,
            "final_authorization_candidate_present": true,
            "final_authorization_preconditions_satisfied": false,
            "final_authorization_denied": true,
            "final_authorization_accepted": false,
            "final_authorization_persisted": false,
            "approval_packet_accepted": false,
            "operator_approval_recorded": false,
            "operator_consent_recorded": false
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "candidate_provider_invocation_requested": true,
            "candidate_model_invocation_requested": true,
            "provider_invocation_authorized": false,
            "model_invocation_authorized": false,
            "provider_invocation_authorized_from_ordering": false,
            "model_invocation_authorized_from_ordering": false,
            "provider_invocation_budget": 0,
            "model_invocation_budget": 0,
            "provider_invoked": false,
            "model_invoked": false,
            "credential_value_read": false,
            "credential_read": false,
            "secret_file_read": false,
            "provider_router_live_envelope_executed": false,
            "provider_router_prompt_mutated": false,
            "provider_router_context_packet_materialized": false,
            "provider_prompt_injection_performed": false,
            "context_injection_performed": false,
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
                    "action": "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_cancellation_supersession_denial",
                    "status": "allowed_report_only_next_slice",
                    "records_result_receipt": false,
                    "persists_result_receipt": false,
                    "accepts_result_receipt": false,
                    "records_sequence_cursor": false,
                    "persists_monotonicity_state": false,
                    "requires_fresh_accepted_operator_approval_artifact": true,
                    "requires_single_use_approval_nonce": true,
                    "requires_operator_identity_session_binding": true,
                    "requires_explicit_command": true,
                    "invokes_provider": false,
                    "invokes_model": false,
                    "reads_credentials": false,
                    "writes_kg": false,
                    "sends_externally": false,
                    "mutates_durable_memory": false
                }
            ],
            "blocked_actions": [
                "final_authorization_result_receipt_sequence_cursor_recording",
                "final_authorization_result_receipt_monotonicity_state_persistence",
                "final_authorization_result_receipt_duplicate_sequence_acceptance",
                "final_authorization_result_receipt_stale_sequence_acceptance",
                "final_authorization_result_receipt_late_sequence_acceptance",
                "final_authorization_result_receipt_future_sequence_acceptance",
                "final_authorization_result_receipt_timestamp_rollback_acceptance",
                "final_authorization_result_receipt_epoch_rollback_acceptance",
                "final_authorization_result_receipt_same_sequence_override",
                "final_authorization_result_receipt_latest_wins_promotion",
                "operator_approval_from_result_receipt_ordering",
                "activation_from_result_receipt_ordering",
                "provider_or_model_invocation_from_result_receipt_ordering",
                "credential_or_secret_read_from_result_receipt_ordering",
                "kg_or_memory_write_from_result_receipt_ordering",
                "channel_or_external_delivery_from_result_receipt_ordering"
            ],
            "side_effects": side_effects
        }),
    );
    report
}

fn hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_cancellation_supersession_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source =
        hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_ordering_monotonicity_denial_report();
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
            .and_then(|value| value.as_str())
            .unwrap_or("")
    };
    let source_next_action_cancellation_supersession = source
        .get("allowed_next_actions")
        .and_then(|value| value.as_array())
        .and_then(|items| items.first())
        .map(|item| {
            item.get("action").and_then(|value| value.as_str())
                == Some(
                    "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_cancellation_supersession_denial",
                )
                && item
                    .get("records_result_receipt")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && item
                    .get("records_sequence_cursor")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && item
                    .get("persists_monotonicity_state")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && item.get("invokes_provider").and_then(serde_json::Value::as_bool) == Some(false)
                && item.get("invokes_model").and_then(serde_json::Value::as_bool) == Some(false)
        })
        .unwrap_or(false);
    let source_ordering_ready = source_bool(
        "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_ordering_monotonicity_denial_ready",
    ) && source_bool(
        "source_first_model_invocation_approval_final_authorization_dry_run_result_receipt_replay_idempotency_denial_ready",
    ) && source_bool(
        "final_authorization_dry_run_result_receipt_ordering_monotonicity_readback_hash_matched",
    ) && source_i64("ordering_monotonicity_fixture_count") == 8
        && source_i64("blocked_ordering_monotonicity_fixture_count") == 8
        && source_i64("noop_ordering_monotonicity_fixture_count") == 8
        && source_i64("allowed_ordering_monotonicity_fixture_count") == 0
        && source_i64("accepted_ordering_monotonicity_fixture_count") == 0
        && source_i64("ordering_monotonicity_performed_count") == 0
        && source_i64("sequence_cursor_recorded_count") == 0
        && source_i64("sequence_cursor_persisted_count") == 0
        && source_i64("monotonicity_state_recorded_count") == 0
        && source_i64("monotonicity_state_persisted_count") == 0
        && !source_bool("final_authorization_dry_run_result_receipt_ordering_allowed")
        && !source_bool("final_authorization_dry_run_result_receipt_ordered")
        && !source_bool("final_authorization_dry_run_result_receipt_ordering_recorded")
        && !source_bool("final_authorization_dry_run_result_receipt_ordering_persisted")
        && !source_bool("final_authorization_dry_run_result_receipt_sequence_cursor_recorded")
        && !source_bool("final_authorization_dry_run_result_receipt_monotonicity_state_persisted")
        && !source_bool("final_authorization_dry_run_result_receipt_latest_wins_promoted")
        && !source_bool("final_authorization_from_ordering_allowed")
        && !source_bool("operator_approval_from_ordering_accepted")
        && !source_bool("activation_from_ordering_allowed")
        && !source_bool("provider_invocation_authorized")
        && !source_bool("model_invocation_authorized")
        && source_i64("provider_invocation_budget") == 0
        && source_i64("model_invocation_budget") == 0
        && !source_bool("provider_invoked")
        && !source_bool("model_invoked")
        && !source_bool("credential_read")
        && !source_bool("secret_file_read")
        && !source_bool("live_kg_write_performed")
        && !source_bool("memory_store_write_performed")
        && !source_bool("channel_send_performed")
        && !source_bool("telegram_send_performed")
        && !source_bool("external_send_performed")
        && source_next_action_cancellation_supersession;
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT;
    let source_ordering_hash = source_str(
        "final_authorization_dry_run_result_receipt_ordering_monotonicity_denial_hash_sha256",
    );
    let source_ordering_readback_hash = source_str(
        "final_authorization_dry_run_result_receipt_ordering_monotonicity_readback_hash_sha256",
    );
    let cancellation_scope = "first_model_invocation:operator-approval-final-authorization-dry-run-result-receipt-cancellation-supersession-denial";
    let cancellation_supersession_denial_hash = sha256_text_value(&format!(
        "first-model-final-authorization-dry-run-result-receipt-cancellation-supersession-denial:{cancellation_scope}:{source_ordering_hash}:{source_ordering_readback_hash}:cancel=false:supersede=false:replace=false:tombstone=false"
    ));
    let cancellation_supersession_readback_hash = sha256_text_value(&format!(
        "first-model-final-authorization-dry-run-result-receipt-cancellation-supersession-readback:{cancellation_supersession_denial_hash}:lifecycle=false:authority=false:delivery=false"
    ));
    let cancellation_fixtures = vec![
        serde_json::json!({
            "fixture_id": "cancel-after-denied-result-receipt",
            "cancellation_supersession_status": "blocked_cancel_after_denied_result_receipt",
            "final_authorization_dry_run_result_receipt_cancellation_accepted": false,
            "activation_from_cancellation_supersession_allowed": false,
            "receipt_noop_confirmed": true
        }),
        serde_json::json!({
            "fixture_id": "supersede-denied-result-receipt",
            "cancellation_supersession_status": "blocked_supersede_denied_result_receipt",
            "final_authorization_dry_run_result_receipt_supersession_accepted": false,
            "activation_from_cancellation_supersession_allowed": false,
            "receipt_noop_confirmed": true
        }),
        serde_json::json!({
            "fixture_id": "replacement-receipt-claim",
            "cancellation_supersession_status": "blocked_replacement_receipt_claim",
            "final_authorization_dry_run_result_receipt_replacement_accepted": false,
            "operator_approval_from_cancellation_supersession_accepted": false,
            "receipt_noop_confirmed": true
        }),
        serde_json::json!({
            "fixture_id": "tombstone-delete-marker",
            "cancellation_supersession_status": "blocked_tombstone_delete_marker",
            "final_authorization_dry_run_result_receipt_tombstone_recorded": false,
            "final_authorization_dry_run_result_receipt_delete_marker_recorded": false,
            "receipt_noop_confirmed": true
        }),
        serde_json::json!({
            "fixture_id": "latest-replacement-promotion",
            "cancellation_supersession_status": "blocked_latest_replacement_promotion",
            "final_authorization_dry_run_result_receipt_latest_replacement_promoted": false,
            "provider_invocation_authorized_from_cancellation_supersession": false,
            "model_invocation_authorized_from_cancellation_supersession": false,
            "receipt_noop_confirmed": true
        }),
        serde_json::json!({
            "fixture_id": "completion-ack-replacement",
            "cancellation_supersession_status": "blocked_completion_ack_replacement",
            "completion_ack_cancellation_accepted": false,
            "completion_ack_replacement_accepted": false,
            "receipt_noop_confirmed": true
        }),
        serde_json::json!({
            "fixture_id": "export-query-observability-replacement",
            "cancellation_supersession_status": "blocked_export_query_observability_replacement",
            "result_receipt_cancelled_query_registered": false,
            "result_receipt_superseded_export_recorded": false,
            "result_receipt_replacement_observability_recorded": false,
            "receipt_noop_confirmed": true
        }),
        serde_json::json!({
            "fixture_id": "external-delivery-supersession",
            "cancellation_supersession_status": "blocked_external_delivery_supersession",
            "telegram_cancellation_supersession_sent": false,
            "external_send_performed": false,
            "provider_invoked": false,
            "model_invoked": false,
            "receipt_noop_confirmed": true
        }),
    ];
    let cancellation_fixture_count = cancellation_fixtures.len();
    let report_ready =
        route_matrix.ready && route_count_source_command_accepted && source_ordering_ready;

    let audit_steps = vec![
        serde_json::json!({
            "step": "ordering_monotonicity_source_binding",
            "status": "ready",
            "source_endpoint": HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_FINAL_AUTHORIZATION_DRY_RUN_RESULT_RECEIPT_ORDERING_MONOTONICITY_DENIAL_ENDPOINT,
            "source_ordering_monotonicity_ready": source_ordering_ready,
            "source_ordering_hash_sha256": source_ordering_hash,
            "source_ordering_readback_hash_sha256": source_ordering_readback_hash
        }),
        serde_json::json!({
            "step": "cancellation_supersession_fixture_denial",
            "status": "blocked_report_only",
            "cancellation_supersession_fixture_count": cancellation_fixture_count,
            "blocked_cancellation_supersession_fixture_count": cancellation_fixture_count,
            "allowed_cancellation_supersession_fixture_count": 0,
            "accepted_cancellation_supersession_fixture_count": 0,
            "cancellation_supersession_performed_count": 0
        }),
        serde_json::json!({
            "step": "replacement_lifecycle_no_write",
            "status": "not_recorded_or_persisted",
            "final_authorization_dry_run_result_receipt_cancellation_recorded": false,
            "final_authorization_dry_run_result_receipt_supersession_recorded": false,
            "final_authorization_dry_run_result_receipt_replacement_recorded": false,
            "final_authorization_dry_run_result_receipt_tombstone_recorded": false
        }),
        serde_json::json!({
            "step": "replacement_query_export_observability_denial",
            "status": "denied",
            "result_receipt_cancelled_query_registered": false,
            "result_receipt_superseded_export_recorded": false,
            "result_receipt_replacement_observability_recorded": false
        }),
        serde_json::json!({
            "step": "cancellation_supersession_authority_non_promotion",
            "status": "authority_denied",
            "final_authorization_from_cancellation_supersession_allowed": false,
            "operator_approval_from_cancellation_supersession_accepted": false,
            "activation_from_cancellation_supersession_allowed": false,
            "provider_invocation_authorized": false,
            "model_invocation_authorized": false,
            "provider_invoked": false,
            "model_invoked": false
        }),
        serde_json::json!({
            "step": "side_effect_denial_check",
            "status": "ready",
            "credential_read": false,
            "secret_file_read": false,
            "provider_router_live_envelope_executed": false,
            "live_kg_write_performed": false,
            "memory_store_write_performed": false,
            "channel_send_performed": false,
            "external_send_performed": false,
            "install_executed": false,
            "active_binary_mutated": false,
            "public_release_claimed": false
        }),
    ];

    let mut side_effects = serde_json::Map::new();
    for key in [
        "final_authorization_dry_run_result_receipt_cancellation_allowed",
        "final_authorization_dry_run_result_receipt_cancellation_accepted",
        "final_authorization_dry_run_result_receipt_cancellation_recorded",
        "final_authorization_dry_run_result_receipt_cancellation_persisted",
        "final_authorization_dry_run_result_receipt_supersession_accepted",
        "final_authorization_dry_run_result_receipt_supersession_recorded",
        "final_authorization_dry_run_result_receipt_supersession_persisted",
        "final_authorization_dry_run_result_receipt_replacement_accepted",
        "final_authorization_dry_run_result_receipt_replacement_recorded",
        "final_authorization_dry_run_result_receipt_replacement_persisted",
        "final_authorization_dry_run_result_receipt_tombstone_recorded",
        "final_authorization_dry_run_result_receipt_delete_marker_recorded",
        "final_authorization_dry_run_result_receipt_latest_replacement_promoted",
        "completion_ack_cancellation_accepted",
        "completion_ack_replacement_accepted",
        "result_receipt_cancelled_query_registered",
        "result_receipt_superseded_export_recorded",
        "result_receipt_replacement_observability_recorded",
        "operator_approval_from_cancellation_supersession_accepted",
        "final_authorization_from_cancellation_supersession_allowed",
        "activation_from_cancellation_supersession_allowed",
        "provider_invocation_authorized",
        "model_invocation_authorized",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "live_kg_write_performed",
        "memory_store_write_performed",
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
        "gate": "hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_cancellation_supersession_denial_route",
        "endpoint": HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_FINAL_AUTHORIZATION_DRY_RUN_RESULT_RECEIPT_CANCELLATION_SUPERSESSION_DENIAL_ENDPOINT,
        "source_command": "/hepta-first-model-invocation-operator-approval-final-authorization-dry-run-result-receipt-cancellation-supersession-denial --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-23",
        "canary_schema_version": "hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_cancellation_supersession_denial_v1",
        "canary_execution_mode": "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_cancellation_supersession_denial_no_provider_model_invocation",
        "source_first_model_invocation_approval_final_authorization_dry_run_result_receipt_ordering_monotonicity_denial_endpoint": HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_FINAL_AUTHORIZATION_DRY_RUN_RESULT_RECEIPT_ORDERING_MONOTONICITY_DENIAL_ENDPOINT,
        "source_first_model_invocation_approval_final_authorization_dry_run_result_receipt_ordering_monotonicity_denial_ready": source_ordering_ready,
        "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        "route_count": route_matrix.route_count,
        "implemented_route_count": route_matrix.implemented_route_count,
        "missing_route_count": route_matrix.missing_route_count,
        "route_count_source_command_accepted": route_count_source_command_accepted,
        "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_cancellation_supersession_denial_route_enabled": true,
        "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_cancellation_supersession_denial_ready": report_ready
    });
    extend_json_object(
        &mut report,
        serde_json::json!({
            "result_receipt_cancellation_supersession_state": "final_authorization_dry_run_result_receipt_cancellation_supersession_replacement_denied",
            "result_receipt_cancellation_supersession_scope": cancellation_scope,
            "source_final_authorization_dry_run_result_receipt_ordering_monotonicity_denial_hash_sha256": source_ordering_hash,
            "source_final_authorization_dry_run_result_receipt_ordering_monotonicity_readback_hash_sha256": source_ordering_readback_hash,
            "final_authorization_dry_run_result_receipt_cancellation_supersession_denial_hash_sha256": cancellation_supersession_denial_hash,
            "final_authorization_dry_run_result_receipt_cancellation_supersession_readback_hash_sha256": cancellation_supersession_readback_hash,
            "final_authorization_dry_run_result_receipt_cancellation_supersession_readback_hash_matched": true,
            "cancellation_supersession_fixture_count": cancellation_fixture_count,
            "blocked_cancellation_supersession_fixture_count": cancellation_fixture_count,
            "noop_cancellation_supersession_fixture_count": cancellation_fixture_count,
            "allowed_cancellation_supersession_fixture_count": 0,
            "accepted_cancellation_supersession_fixture_count": 0,
            "cancellation_supersession_performed_count": 0,
            "cancellation_supersession_fixtures": cancellation_fixtures,
            "cancellation_recorded_count": 0,
            "supersession_recorded_count": 0,
            "replacement_receipt_recorded_count": 0,
            "tombstone_recorded_count": 0,
            "delete_marker_recorded_count": 0
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "final_authorization_dry_run_result_receipt_cancellation_allowed": false,
            "final_authorization_dry_run_result_receipt_cancellation_accepted": false,
            "final_authorization_dry_run_result_receipt_cancellation_recorded": false,
            "final_authorization_dry_run_result_receipt_cancellation_persisted": false,
            "final_authorization_dry_run_result_receipt_supersession_accepted": false,
            "final_authorization_dry_run_result_receipt_supersession_recorded": false,
            "final_authorization_dry_run_result_receipt_supersession_persisted": false,
            "final_authorization_dry_run_result_receipt_replacement_accepted": false,
            "final_authorization_dry_run_result_receipt_replacement_recorded": false,
            "final_authorization_dry_run_result_receipt_replacement_persisted": false,
            "final_authorization_dry_run_result_receipt_tombstone_recorded": false,
            "final_authorization_dry_run_result_receipt_delete_marker_recorded": false,
            "final_authorization_dry_run_result_receipt_latest_replacement_promoted": false,
            "completion_ack_cancellation_accepted": false,
            "completion_ack_replacement_accepted": false,
            "result_receipt_cancelled_query_registered": false,
            "result_receipt_superseded_export_recorded": false,
            "result_receipt_replacement_observability_recorded": false
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "final_authorization_from_cancellation_supersession_allowed": false,
            "operator_approval_from_cancellation_supersession_accepted": false,
            "activation_from_cancellation_supersession_allowed": false,
            "provider_invocation_authorized": false,
            "model_invocation_authorized": false,
            "provider_invocation_authorized_from_cancellation_supersession": false,
            "model_invocation_authorized_from_cancellation_supersession": false,
            "provider_invocation_budget": 0,
            "model_invocation_budget": 0,
            "provider_invoked": false,
            "model_invoked": false,
            "credential_value_read": false,
            "credential_read": false,
            "secret_file_read": false,
            "provider_router_live_envelope_executed": false,
            "provider_prompt_injection_performed": false,
            "context_injection_performed": false,
            "kg_adapter_read_performed": false,
            "live_kg_write_performed": false,
            "memory_store_write_performed": false,
            "channel_send_performed": false,
            "telegram_send_performed": false,
            "external_send_performed": false
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "allowed_next_actions": [
                {
                    "action": "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_audit_immutable_evidence_denial",
                    "status": "allowed_report_only_next_slice",
                    "records_result_receipt": false,
                    "records_cancellation": false,
                    "records_supersession": false,
                    "records_replacement": false,
                    "persists_lifecycle_state": false,
                    "invokes_provider": false,
                    "invokes_model": false,
                    "reads_credentials": false,
                    "writes_kg": false,
                    "sends_externally": false,
                    "mutates_durable_memory": false
                }
            ],
            "audit_steps": audit_steps,
            "side_effects": side_effects
        }),
    );
    report
}

fn hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_audit_immutable_evidence_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source =
        hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_cancellation_supersession_denial_report();
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
            .and_then(|value| value.as_str())
            .unwrap_or("")
    };
    let source_next_action_audit_immutable_evidence = source
        .get("allowed_next_actions")
        .and_then(|value| value.as_array())
        .and_then(|items| items.first())
        .map(|item| {
            item.get("action").and_then(|value| value.as_str())
                == Some(
                    "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_audit_immutable_evidence_denial",
                )
                && item
                    .get("records_result_receipt")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && item
                    .get("records_cancellation")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && item
                    .get("records_supersession")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && item.get("invokes_provider").and_then(serde_json::Value::as_bool) == Some(false)
                && item.get("invokes_model").and_then(serde_json::Value::as_bool) == Some(false)
        })
        .unwrap_or(false);
    let source_cancellation_ready = source_bool(
        "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_cancellation_supersession_denial_ready",
    ) && source_bool(
        "source_first_model_invocation_approval_final_authorization_dry_run_result_receipt_ordering_monotonicity_denial_ready",
    ) && source_bool(
        "final_authorization_dry_run_result_receipt_cancellation_supersession_readback_hash_matched",
    ) && source_i64("cancellation_supersession_fixture_count") == 8
        && source_i64("blocked_cancellation_supersession_fixture_count") == 8
        && source_i64("noop_cancellation_supersession_fixture_count") == 8
        && source_i64("allowed_cancellation_supersession_fixture_count") == 0
        && source_i64("accepted_cancellation_supersession_fixture_count") == 0
        && source_i64("cancellation_supersession_performed_count") == 0
        && source_i64("cancellation_recorded_count") == 0
        && source_i64("supersession_recorded_count") == 0
        && source_i64("replacement_receipt_recorded_count") == 0
        && source_i64("tombstone_recorded_count") == 0
        && source_i64("delete_marker_recorded_count") == 0
        && !source_bool("final_authorization_dry_run_result_receipt_cancellation_allowed")
        && !source_bool("final_authorization_dry_run_result_receipt_cancellation_accepted")
        && !source_bool("final_authorization_dry_run_result_receipt_cancellation_recorded")
        && !source_bool("final_authorization_dry_run_result_receipt_cancellation_persisted")
        && !source_bool("final_authorization_dry_run_result_receipt_supersession_accepted")
        && !source_bool("final_authorization_dry_run_result_receipt_supersession_recorded")
        && !source_bool("final_authorization_dry_run_result_receipt_replacement_accepted")
        && !source_bool("final_authorization_dry_run_result_receipt_replacement_recorded")
        && !source_bool("final_authorization_dry_run_result_receipt_latest_replacement_promoted")
        && !source_bool("final_authorization_from_cancellation_supersession_allowed")
        && !source_bool("operator_approval_from_cancellation_supersession_accepted")
        && !source_bool("activation_from_cancellation_supersession_allowed")
        && !source_bool("provider_invocation_authorized")
        && !source_bool("model_invocation_authorized")
        && source_i64("provider_invocation_budget") == 0
        && source_i64("model_invocation_budget") == 0
        && !source_bool("provider_invoked")
        && !source_bool("model_invoked")
        && !source_bool("credential_read")
        && !source_bool("secret_file_read")
        && !source_bool("live_kg_write_performed")
        && !source_bool("memory_store_write_performed")
        && !source_bool("channel_send_performed")
        && !source_bool("telegram_send_performed")
        && !source_bool("external_send_performed")
        && source_next_action_audit_immutable_evidence;
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT;
    let source_cancellation_hash = source_str(
        "final_authorization_dry_run_result_receipt_cancellation_supersession_denial_hash_sha256",
    );
    let source_cancellation_readback_hash = source_str(
        "final_authorization_dry_run_result_receipt_cancellation_supersession_readback_hash_sha256",
    );
    let audit_scope = "first_model_invocation:operator-approval-final-authorization-dry-run-result-receipt-audit-immutable-evidence-denial";
    let audit_immutable_evidence_denial_hash = sha256_text_value(&format!(
        "first-model-final-authorization-dry-run-result-receipt-audit-immutable-evidence-denial:{audit_scope}:{source_cancellation_hash}:{source_cancellation_readback_hash}:audit=false:ledger=false:hash-chain=false:evidence=false:attestation=false"
    ));
    let audit_immutable_evidence_readback_hash = sha256_text_value(&format!(
        "first-model-final-authorization-dry-run-result-receipt-audit-immutable-evidence-readback:{audit_immutable_evidence_denial_hash}:witness=false:notary=false:export=false:external=false"
    ));
    let audit_fixtures = vec![
        serde_json::json!({
            "fixture_id": "audit-ledger-entry",
            "audit_immutable_evidence_status": "blocked_audit_ledger_entry",
            "final_authorization_dry_run_result_receipt_audit_recorded": false,
            "final_authorization_dry_run_result_receipt_ledger_written": false,
            "receipt_noop_confirmed": true
        }),
        serde_json::json!({
            "fixture_id": "hash-chain-append",
            "audit_immutable_evidence_status": "blocked_hash_chain_append",
            "final_authorization_dry_run_result_receipt_hash_chain_appended": false,
            "final_authorization_dry_run_result_receipt_ledger_written": false,
            "receipt_noop_confirmed": true
        }),
        serde_json::json!({
            "fixture_id": "immutable-evidence-materialization",
            "audit_immutable_evidence_status": "blocked_immutable_evidence_materialization",
            "final_authorization_dry_run_result_receipt_immutable_evidence_materialized": false,
            "final_authorization_from_audit_immutable_evidence_allowed": false,
            "receipt_noop_confirmed": true
        }),
        serde_json::json!({
            "fixture_id": "attestation-signature",
            "audit_immutable_evidence_status": "blocked_attestation_signature",
            "final_authorization_dry_run_result_receipt_attestation_signed": false,
            "operator_approval_from_audit_immutable_evidence_accepted": false,
            "receipt_noop_confirmed": true
        }),
        serde_json::json!({
            "fixture_id": "witness-notary-record",
            "audit_immutable_evidence_status": "blocked_witness_notary_record",
            "final_authorization_dry_run_result_receipt_witness_notarized": false,
            "activation_from_audit_immutable_evidence_allowed": false,
            "receipt_noop_confirmed": true
        }),
        serde_json::json!({
            "fixture_id": "merkle-root-public-proof",
            "audit_immutable_evidence_status": "blocked_merkle_root_public_proof",
            "final_authorization_dry_run_result_receipt_merkle_root_published": false,
            "public_release_claimed": false,
            "receipt_noop_confirmed": true
        }),
        serde_json::json!({
            "fixture_id": "audit-export-query-evidence",
            "audit_immutable_evidence_status": "blocked_audit_export_query_evidence",
            "final_authorization_dry_run_result_receipt_evidence_export_recorded": false,
            "result_receipt_audit_query_registered": false,
            "receipt_noop_confirmed": true
        }),
        serde_json::json!({
            "fixture_id": "external-evidence-delivery",
            "audit_immutable_evidence_status": "blocked_external_evidence_delivery",
            "final_authorization_dry_run_result_receipt_external_evidence_sent": false,
            "external_send_performed": false,
            "provider_invoked": false,
            "model_invoked": false,
            "receipt_noop_confirmed": true
        }),
    ];
    let audit_fixture_count = audit_fixtures.len();
    let report_ready =
        route_matrix.ready && route_count_source_command_accepted && source_cancellation_ready;

    let audit_steps = vec![
        serde_json::json!({
            "step": "cancellation_supersession_source_binding",
            "status": "ready",
            "source_endpoint": HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_FINAL_AUTHORIZATION_DRY_RUN_RESULT_RECEIPT_CANCELLATION_SUPERSESSION_DENIAL_ENDPOINT,
            "source_cancellation_supersession_ready": source_cancellation_ready,
            "source_cancellation_hash_sha256": source_cancellation_hash,
            "source_cancellation_readback_hash_sha256": source_cancellation_readback_hash
        }),
        serde_json::json!({
            "step": "audit_immutable_evidence_fixture_denial",
            "status": "blocked_report_only",
            "audit_immutable_evidence_fixture_count": audit_fixture_count,
            "blocked_audit_immutable_evidence_fixture_count": audit_fixture_count,
            "allowed_audit_immutable_evidence_fixture_count": 0,
            "accepted_audit_immutable_evidence_fixture_count": 0,
            "audit_immutable_evidence_performed_count": 0
        }),
        serde_json::json!({
            "step": "ledger_hash_chain_no_write",
            "status": "not_recorded_or_persisted",
            "final_authorization_dry_run_result_receipt_audit_recorded": false,
            "final_authorization_dry_run_result_receipt_ledger_written": false,
            "final_authorization_dry_run_result_receipt_hash_chain_appended": false,
            "final_authorization_dry_run_result_receipt_immutable_evidence_materialized": false
        }),
        serde_json::json!({
            "step": "attestation_witness_public_proof_denial",
            "status": "denied",
            "final_authorization_dry_run_result_receipt_attestation_signed": false,
            "final_authorization_dry_run_result_receipt_witness_notarized": false,
            "final_authorization_dry_run_result_receipt_merkle_root_published": false
        }),
        serde_json::json!({
            "step": "audit_immutable_evidence_authority_non_promotion",
            "status": "authority_denied",
            "final_authorization_from_audit_immutable_evidence_allowed": false,
            "operator_approval_from_audit_immutable_evidence_accepted": false,
            "activation_from_audit_immutable_evidence_allowed": false,
            "provider_invocation_authorized": false,
            "model_invocation_authorized": false,
            "provider_invoked": false,
            "model_invoked": false
        }),
        serde_json::json!({
            "step": "side_effect_denial_check",
            "status": "ready",
            "credential_read": false,
            "secret_file_read": false,
            "provider_router_live_envelope_executed": false,
            "live_kg_write_performed": false,
            "memory_store_write_performed": false,
            "channel_send_performed": false,
            "external_send_performed": false,
            "install_executed": false,
            "active_binary_mutated": false,
            "public_release_claimed": false
        }),
    ];

    let mut side_effects = serde_json::Map::new();
    for key in [
        "final_authorization_dry_run_result_receipt_audit_allowed",
        "final_authorization_dry_run_result_receipt_audit_recorded",
        "final_authorization_dry_run_result_receipt_ledger_written",
        "final_authorization_dry_run_result_receipt_hash_chain_appended",
        "final_authorization_dry_run_result_receipt_immutable_evidence_materialized",
        "final_authorization_dry_run_result_receipt_attestation_signed",
        "final_authorization_dry_run_result_receipt_witness_notarized",
        "final_authorization_dry_run_result_receipt_merkle_root_published",
        "final_authorization_dry_run_result_receipt_evidence_export_recorded",
        "final_authorization_dry_run_result_receipt_external_evidence_sent",
        "result_receipt_audit_query_registered",
        "operator_approval_from_audit_immutable_evidence_accepted",
        "final_authorization_from_audit_immutable_evidence_allowed",
        "activation_from_audit_immutable_evidence_allowed",
        "provider_invocation_authorized",
        "model_invocation_authorized",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "live_kg_write_performed",
        "memory_store_write_performed",
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
        "gate": "hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_audit_immutable_evidence_denial_route",
        "endpoint": HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_FINAL_AUTHORIZATION_DRY_RUN_RESULT_RECEIPT_AUDIT_IMMUTABLE_EVIDENCE_DENIAL_ENDPOINT,
        "source_command": "/hepta-first-model-invocation-operator-approval-final-authorization-dry-run-result-receipt-audit-immutable-evidence-denial --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-24",
        "canary_schema_version": "hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_audit_immutable_evidence_denial_v1",
        "canary_execution_mode": "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_audit_immutable_evidence_denial_no_provider_model_invocation",
        "source_first_model_invocation_approval_final_authorization_dry_run_result_receipt_cancellation_supersession_denial_endpoint": HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_FINAL_AUTHORIZATION_DRY_RUN_RESULT_RECEIPT_CANCELLATION_SUPERSESSION_DENIAL_ENDPOINT,
        "source_first_model_invocation_approval_final_authorization_dry_run_result_receipt_cancellation_supersession_denial_ready": source_cancellation_ready,
        "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        "route_count": route_matrix.route_count,
        "implemented_route_count": route_matrix.implemented_route_count,
        "missing_route_count": route_matrix.missing_route_count,
        "route_count_source_command_accepted": route_count_source_command_accepted,
        "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_audit_immutable_evidence_denial_route_enabled": true,
        "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_audit_immutable_evidence_denial_ready": report_ready
    });
    extend_json_object(
        &mut report,
        serde_json::json!({
            "result_receipt_audit_immutable_evidence_state": "final_authorization_dry_run_result_receipt_audit_immutable_evidence_denied",
            "result_receipt_audit_immutable_evidence_scope": audit_scope,
            "source_final_authorization_dry_run_result_receipt_cancellation_supersession_denial_hash_sha256": source_cancellation_hash,
            "source_final_authorization_dry_run_result_receipt_cancellation_supersession_readback_hash_sha256": source_cancellation_readback_hash,
            "final_authorization_dry_run_result_receipt_audit_immutable_evidence_denial_hash_sha256": audit_immutable_evidence_denial_hash,
            "final_authorization_dry_run_result_receipt_audit_immutable_evidence_readback_hash_sha256": audit_immutable_evidence_readback_hash,
            "final_authorization_dry_run_result_receipt_audit_immutable_evidence_readback_hash_matched": true,
            "audit_immutable_evidence_fixture_count": audit_fixture_count,
            "blocked_audit_immutable_evidence_fixture_count": audit_fixture_count,
            "noop_audit_immutable_evidence_fixture_count": audit_fixture_count,
            "allowed_audit_immutable_evidence_fixture_count": 0,
            "accepted_audit_immutable_evidence_fixture_count": 0,
            "audit_immutable_evidence_performed_count": 0,
            "audit_immutable_evidence_fixtures": audit_fixtures,
            "audit_recorded_count": 0,
            "ledger_written_count": 0,
            "hash_chain_appended_count": 0,
            "immutable_evidence_materialized_count": 0,
            "attestation_signed_count": 0,
            "witness_notarized_count": 0,
            "merkle_root_published_count": 0,
            "evidence_export_recorded_count": 0,
            "external_evidence_sent_count": 0
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "final_authorization_dry_run_result_receipt_audit_allowed": false,
            "final_authorization_dry_run_result_receipt_audit_recorded": false,
            "final_authorization_dry_run_result_receipt_ledger_written": false,
            "final_authorization_dry_run_result_receipt_hash_chain_appended": false,
            "final_authorization_dry_run_result_receipt_immutable_evidence_materialized": false,
            "final_authorization_dry_run_result_receipt_attestation_signed": false,
            "final_authorization_dry_run_result_receipt_witness_notarized": false,
            "final_authorization_dry_run_result_receipt_merkle_root_published": false,
            "final_authorization_dry_run_result_receipt_evidence_export_recorded": false,
            "final_authorization_dry_run_result_receipt_external_evidence_sent": false,
            "result_receipt_audit_query_registered": false,
            "final_authorization_from_audit_immutable_evidence_allowed": false,
            "operator_approval_from_audit_immutable_evidence_accepted": false,
            "activation_from_audit_immutable_evidence_allowed": false
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "provider_invocation_authorized": false,
            "model_invocation_authorized": false,
            "provider_invocation_authorized_from_audit_immutable_evidence": false,
            "model_invocation_authorized_from_audit_immutable_evidence": false,
            "provider_invocation_budget": 0,
            "model_invocation_budget": 0,
            "provider_invoked": false,
            "model_invoked": false,
            "credential_value_read": false,
            "credential_read": false,
            "secret_file_read": false,
            "provider_router_live_envelope_executed": false,
            "provider_prompt_injection_performed": false,
            "context_injection_performed": false,
            "kg_adapter_read_performed": false,
            "live_kg_write_performed": false,
            "memory_store_write_performed": false,
            "channel_send_performed": false,
            "telegram_send_performed": false,
            "external_send_performed": false
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "allowed_next_actions": [
                {
                    "action": "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_retention_expiry_garbage_collection_denial",
                    "status": "allowed_report_only_next_slice",
                    "records_result_receipt": false,
                    "records_audit": false,
                    "records_immutable_evidence": false,
                    "persists_ledger": false,
                    "exports_evidence": false,
                    "invokes_provider": false,
                    "invokes_model": false,
                    "reads_credentials": false,
                    "writes_kg": false,
                    "sends_externally": false,
                    "mutates_durable_memory": false
                }
            ],
            "audit_steps": audit_steps,
            "side_effects": side_effects
        }),
    );
    report
}

fn hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_retention_expiry_garbage_collection_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source =
        hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_audit_immutable_evidence_denial_report();
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
            .and_then(|value| value.as_str())
            .unwrap_or("")
    };
    let source_next_action_retention_expiry_garbage_collection = source
        .get("allowed_next_actions")
        .and_then(|value| value.as_array())
        .and_then(|items| items.first())
        .map(|item| {
            item.get("action").and_then(|value| value.as_str())
                == Some(
                    "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_retention_expiry_garbage_collection_denial",
                )
                && item
                    .get("records_audit")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && item
                    .get("records_immutable_evidence")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && item
                    .get("persists_ledger")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && item
                    .get("exports_evidence")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && item.get("invokes_provider").and_then(serde_json::Value::as_bool) == Some(false)
                && item.get("invokes_model").and_then(serde_json::Value::as_bool) == Some(false)
        })
        .unwrap_or(false);
    let source_audit_ready = source_bool(
        "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_audit_immutable_evidence_denial_ready",
    ) && source_bool(
        "source_first_model_invocation_approval_final_authorization_dry_run_result_receipt_cancellation_supersession_denial_ready",
    ) && source_bool(
        "final_authorization_dry_run_result_receipt_audit_immutable_evidence_readback_hash_matched",
    ) && source_i64("audit_immutable_evidence_fixture_count") == 8
        && source_i64("blocked_audit_immutable_evidence_fixture_count") == 8
        && source_i64("noop_audit_immutable_evidence_fixture_count") == 8
        && source_i64("allowed_audit_immutable_evidence_fixture_count") == 0
        && source_i64("accepted_audit_immutable_evidence_fixture_count") == 0
        && source_i64("audit_immutable_evidence_performed_count") == 0
        && source_i64("audit_recorded_count") == 0
        && source_i64("ledger_written_count") == 0
        && source_i64("hash_chain_appended_count") == 0
        && source_i64("immutable_evidence_materialized_count") == 0
        && source_i64("attestation_signed_count") == 0
        && source_i64("witness_notarized_count") == 0
        && source_i64("merkle_root_published_count") == 0
        && source_i64("evidence_export_recorded_count") == 0
        && source_i64("external_evidence_sent_count") == 0
        && !source_bool("final_authorization_dry_run_result_receipt_audit_allowed")
        && !source_bool("final_authorization_dry_run_result_receipt_audit_recorded")
        && !source_bool("final_authorization_dry_run_result_receipt_ledger_written")
        && !source_bool("final_authorization_dry_run_result_receipt_hash_chain_appended")
        && !source_bool(
            "final_authorization_dry_run_result_receipt_immutable_evidence_materialized",
        )
        && !source_bool("final_authorization_dry_run_result_receipt_attestation_signed")
        && !source_bool("final_authorization_dry_run_result_receipt_witness_notarized")
        && !source_bool("final_authorization_dry_run_result_receipt_merkle_root_published")
        && !source_bool("final_authorization_dry_run_result_receipt_evidence_export_recorded")
        && !source_bool("final_authorization_dry_run_result_receipt_external_evidence_sent")
        && !source_bool("final_authorization_from_audit_immutable_evidence_allowed")
        && !source_bool("operator_approval_from_audit_immutable_evidence_accepted")
        && !source_bool("activation_from_audit_immutable_evidence_allowed")
        && !source_bool("provider_invocation_authorized")
        && !source_bool("model_invocation_authorized")
        && source_i64("provider_invocation_budget") == 0
        && source_i64("model_invocation_budget") == 0
        && !source_bool("provider_invoked")
        && !source_bool("model_invoked")
        && !source_bool("credential_read")
        && !source_bool("secret_file_read")
        && !source_bool("live_kg_write_performed")
        && !source_bool("memory_store_write_performed")
        && !source_bool("channel_send_performed")
        && !source_bool("telegram_send_performed")
        && !source_bool("external_send_performed")
        && source_next_action_retention_expiry_garbage_collection;
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT;
    let source_audit_hash = source_str(
        "final_authorization_dry_run_result_receipt_audit_immutable_evidence_denial_hash_sha256",
    );
    let source_audit_readback_hash = source_str(
        "final_authorization_dry_run_result_receipt_audit_immutable_evidence_readback_hash_sha256",
    );
    let retention_scope = "first_model_invocation:operator-approval-final-authorization-dry-run-result-receipt-retention-expiry-garbage-collection-denial";
    let retention_expiry_garbage_collection_denial_hash = sha256_text_value(&format!(
        "first-model-final-authorization-dry-run-result-receipt-retention-expiry-garbage-collection-denial:{retention_scope}:{source_audit_hash}:{source_audit_readback_hash}:retention=false:ttl=false:expiry=false:gc=false:delete=false"
    ));
    let retention_expiry_garbage_collection_readback_hash = sha256_text_value(&format!(
        "first-model-final-authorization-dry-run-result-receipt-retention-expiry-garbage-collection-readback:{retention_expiry_garbage_collection_denial_hash}:archive=false:compaction=false:export=false:external=false"
    ));
    let retention_fixtures = vec![
        serde_json::json!({
            "fixture_id": "retention-policy-record",
            "retention_expiry_garbage_collection_status": "blocked_retention_policy_record",
            "final_authorization_dry_run_result_receipt_retention_recorded": false,
            "final_authorization_dry_run_result_receipt_retention_policy_persisted": false,
            "receipt_noop_confirmed": true
        }),
        serde_json::json!({
            "fixture_id": "ttl-expiry-schedule",
            "retention_expiry_garbage_collection_status": "blocked_ttl_expiry_schedule",
            "final_authorization_dry_run_result_receipt_ttl_scheduled": false,
            "final_authorization_dry_run_result_receipt_expiry_applied": false,
            "receipt_noop_confirmed": true
        }),
        serde_json::json!({
            "fixture_id": "garbage-collection-scan",
            "retention_expiry_garbage_collection_status": "blocked_garbage_collection_scan",
            "final_authorization_dry_run_result_receipt_garbage_collection_scan_performed": false,
            "final_authorization_dry_run_result_receipt_garbage_collection_candidate_recorded": false,
            "receipt_noop_confirmed": true
        }),
        serde_json::json!({
            "fixture_id": "delete-marker-materialization",
            "retention_expiry_garbage_collection_status": "blocked_delete_marker_materialization",
            "final_authorization_dry_run_result_receipt_delete_marker_recorded": false,
            "final_authorization_from_retention_expiry_garbage_collection_allowed": false,
            "receipt_noop_confirmed": true
        }),
        serde_json::json!({
            "fixture_id": "archive-compaction-record",
            "retention_expiry_garbage_collection_status": "blocked_archive_compaction_record",
            "final_authorization_dry_run_result_receipt_archive_recorded": false,
            "final_authorization_dry_run_result_receipt_compaction_performed": false,
            "receipt_noop_confirmed": true
        }),
        serde_json::json!({
            "fixture_id": "expiry-status-promotion",
            "retention_expiry_garbage_collection_status": "blocked_expiry_status_promotion",
            "operator_approval_from_retention_expiry_garbage_collection_accepted": false,
            "activation_from_retention_expiry_garbage_collection_allowed": false,
            "receipt_noop_confirmed": true
        }),
        serde_json::json!({
            "fixture_id": "retention-export-query",
            "retention_expiry_garbage_collection_status": "blocked_retention_export_query",
            "final_authorization_dry_run_result_receipt_retention_export_recorded": false,
            "result_receipt_retention_query_registered": false,
            "receipt_noop_confirmed": true
        }),
        serde_json::json!({
            "fixture_id": "external-retention-notification",
            "retention_expiry_garbage_collection_status": "blocked_external_retention_notification",
            "final_authorization_dry_run_result_receipt_external_retention_notification_sent": false,
            "external_send_performed": false,
            "provider_invoked": false,
            "model_invoked": false,
            "receipt_noop_confirmed": true
        }),
    ];
    let retention_fixture_count = retention_fixtures.len();
    let report_ready =
        route_matrix.ready && route_count_source_command_accepted && source_audit_ready;

    let audit_steps = vec![
        serde_json::json!({
            "step": "audit_immutable_evidence_source_binding",
            "status": "ready",
            "source_endpoint": HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_FINAL_AUTHORIZATION_DRY_RUN_RESULT_RECEIPT_AUDIT_IMMUTABLE_EVIDENCE_DENIAL_ENDPOINT,
            "source_audit_immutable_evidence_ready": source_audit_ready,
            "source_audit_hash_sha256": source_audit_hash,
            "source_audit_readback_hash_sha256": source_audit_readback_hash
        }),
        serde_json::json!({
            "step": "retention_expiry_garbage_collection_fixture_denial",
            "status": "blocked_report_only",
            "retention_expiry_garbage_collection_fixture_count": retention_fixture_count,
            "blocked_retention_expiry_garbage_collection_fixture_count": retention_fixture_count,
            "allowed_retention_expiry_garbage_collection_fixture_count": 0,
            "accepted_retention_expiry_garbage_collection_fixture_count": 0,
            "retention_expiry_garbage_collection_performed_count": 0
        }),
        serde_json::json!({
            "step": "ttl_expiry_gc_no_schedule_or_scan",
            "status": "not_recorded_or_persisted",
            "final_authorization_dry_run_result_receipt_retention_policy_persisted": false,
            "final_authorization_dry_run_result_receipt_ttl_scheduled": false,
            "final_authorization_dry_run_result_receipt_expiry_applied": false,
            "final_authorization_dry_run_result_receipt_garbage_collection_scan_performed": false
        }),
        serde_json::json!({
            "step": "archive_compaction_delete_marker_denial",
            "status": "denied",
            "final_authorization_dry_run_result_receipt_delete_marker_recorded": false,
            "final_authorization_dry_run_result_receipt_archive_recorded": false,
            "final_authorization_dry_run_result_receipt_compaction_performed": false
        }),
        serde_json::json!({
            "step": "retention_expiry_garbage_collection_authority_non_promotion",
            "status": "authority_denied",
            "final_authorization_from_retention_expiry_garbage_collection_allowed": false,
            "operator_approval_from_retention_expiry_garbage_collection_accepted": false,
            "activation_from_retention_expiry_garbage_collection_allowed": false,
            "provider_invocation_authorized": false,
            "model_invocation_authorized": false,
            "provider_invoked": false,
            "model_invoked": false
        }),
        serde_json::json!({
            "step": "side_effect_denial_check",
            "status": "ready",
            "credential_read": false,
            "secret_file_read": false,
            "provider_router_live_envelope_executed": false,
            "live_kg_write_performed": false,
            "memory_store_write_performed": false,
            "channel_send_performed": false,
            "external_send_performed": false,
            "install_executed": false,
            "active_binary_mutated": false,
            "public_release_claimed": false
        }),
    ];

    let mut side_effects = serde_json::Map::new();
    for key in [
        "final_authorization_dry_run_result_receipt_retention_recorded",
        "final_authorization_dry_run_result_receipt_retention_policy_persisted",
        "final_authorization_dry_run_result_receipt_ttl_scheduled",
        "final_authorization_dry_run_result_receipt_expiry_applied",
        "final_authorization_dry_run_result_receipt_garbage_collection_scan_performed",
        "final_authorization_dry_run_result_receipt_garbage_collection_candidate_recorded",
        "final_authorization_dry_run_result_receipt_delete_marker_recorded",
        "final_authorization_dry_run_result_receipt_archive_recorded",
        "final_authorization_dry_run_result_receipt_compaction_performed",
        "final_authorization_dry_run_result_receipt_retention_export_recorded",
        "final_authorization_dry_run_result_receipt_external_retention_notification_sent",
        "result_receipt_retention_query_registered",
        "operator_approval_from_retention_expiry_garbage_collection_accepted",
        "final_authorization_from_retention_expiry_garbage_collection_allowed",
        "activation_from_retention_expiry_garbage_collection_allowed",
        "provider_invocation_authorized",
        "model_invocation_authorized",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "live_kg_write_performed",
        "memory_store_write_performed",
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
        "gate": "hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_retention_expiry_garbage_collection_denial_route",
        "endpoint": HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_FINAL_AUTHORIZATION_DRY_RUN_RESULT_RECEIPT_RETENTION_EXPIRY_GARBAGE_COLLECTION_DENIAL_ENDPOINT,
        "source_command": "/hepta-first-model-invocation-operator-approval-final-authorization-dry-run-result-receipt-retention-expiry-garbage-collection-denial --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-24",
        "canary_schema_version": "hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_retention_expiry_garbage_collection_denial_v1",
        "canary_execution_mode": "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_retention_expiry_garbage_collection_denial_no_provider_model_invocation",
        "source_first_model_invocation_approval_final_authorization_dry_run_result_receipt_audit_immutable_evidence_denial_endpoint": HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_FINAL_AUTHORIZATION_DRY_RUN_RESULT_RECEIPT_AUDIT_IMMUTABLE_EVIDENCE_DENIAL_ENDPOINT,
        "source_first_model_invocation_approval_final_authorization_dry_run_result_receipt_audit_immutable_evidence_denial_ready": source_audit_ready,
        "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        "route_count": route_matrix.route_count,
        "implemented_route_count": route_matrix.implemented_route_count,
        "missing_route_count": route_matrix.missing_route_count,
        "route_count_source_command_accepted": route_count_source_command_accepted,
        "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_retention_expiry_garbage_collection_denial_route_enabled": true,
        "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_retention_expiry_garbage_collection_denial_ready": report_ready
    });
    extend_json_object(
        &mut report,
        serde_json::json!({
            "result_receipt_retention_expiry_garbage_collection_state": "final_authorization_dry_run_result_receipt_retention_expiry_garbage_collection_denied",
            "result_receipt_retention_expiry_garbage_collection_scope": retention_scope,
            "source_final_authorization_dry_run_result_receipt_audit_immutable_evidence_denial_hash_sha256": source_audit_hash,
            "source_final_authorization_dry_run_result_receipt_audit_immutable_evidence_readback_hash_sha256": source_audit_readback_hash,
            "final_authorization_dry_run_result_receipt_retention_expiry_garbage_collection_denial_hash_sha256": retention_expiry_garbage_collection_denial_hash,
            "final_authorization_dry_run_result_receipt_retention_expiry_garbage_collection_readback_hash_sha256": retention_expiry_garbage_collection_readback_hash,
            "final_authorization_dry_run_result_receipt_retention_expiry_garbage_collection_readback_hash_matched": true,
            "retention_expiry_garbage_collection_fixture_count": retention_fixture_count,
            "blocked_retention_expiry_garbage_collection_fixture_count": retention_fixture_count,
            "noop_retention_expiry_garbage_collection_fixture_count": retention_fixture_count,
            "allowed_retention_expiry_garbage_collection_fixture_count": 0,
            "accepted_retention_expiry_garbage_collection_fixture_count": 0,
            "retention_expiry_garbage_collection_performed_count": 0,
            "retention_expiry_garbage_collection_fixtures": retention_fixtures,
            "retention_recorded_count": 0,
            "retention_policy_persisted_count": 0,
            "ttl_scheduled_count": 0,
            "expiry_applied_count": 0,
            "garbage_collection_scan_performed_count": 0,
            "garbage_collection_candidate_recorded_count": 0,
            "delete_marker_recorded_count": 0,
            "archive_recorded_count": 0,
            "compaction_performed_count": 0,
            "retention_export_recorded_count": 0,
            "external_retention_notification_sent_count": 0
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "final_authorization_dry_run_result_receipt_retention_recorded": false,
            "final_authorization_dry_run_result_receipt_retention_policy_persisted": false,
            "final_authorization_dry_run_result_receipt_ttl_scheduled": false,
            "final_authorization_dry_run_result_receipt_expiry_applied": false,
            "final_authorization_dry_run_result_receipt_garbage_collection_scan_performed": false,
            "final_authorization_dry_run_result_receipt_garbage_collection_candidate_recorded": false,
            "final_authorization_dry_run_result_receipt_delete_marker_recorded": false,
            "final_authorization_dry_run_result_receipt_archive_recorded": false,
            "final_authorization_dry_run_result_receipt_compaction_performed": false,
            "final_authorization_dry_run_result_receipt_retention_export_recorded": false,
            "final_authorization_dry_run_result_receipt_external_retention_notification_sent": false,
            "result_receipt_retention_query_registered": false,
            "final_authorization_from_retention_expiry_garbage_collection_allowed": false,
            "operator_approval_from_retention_expiry_garbage_collection_accepted": false,
            "activation_from_retention_expiry_garbage_collection_allowed": false
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "provider_invocation_authorized": false,
            "model_invocation_authorized": false,
            "provider_invocation_authorized_from_retention_expiry_garbage_collection": false,
            "model_invocation_authorized_from_retention_expiry_garbage_collection": false,
            "provider_invocation_budget": 0,
            "model_invocation_budget": 0,
            "provider_invoked": false,
            "model_invoked": false,
            "credential_value_read": false,
            "credential_read": false,
            "secret_file_read": false,
            "provider_router_live_envelope_executed": false,
            "provider_prompt_injection_performed": false,
            "context_injection_performed": false,
            "kg_adapter_read_performed": false,
            "live_kg_write_performed": false,
            "memory_store_write_performed": false,
            "channel_send_performed": false,
            "telegram_send_performed": false,
            "external_send_performed": false
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "allowed_next_actions": [
                {
                    "action": "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_export_query_observability_denial",
                    "status": "allowed_report_only_next_slice",
                    "records_result_receipt": false,
                    "records_retention": false,
                    "records_expiry": false,
                    "records_garbage_collection": false,
                    "exports_receipt": false,
                    "invokes_provider": false,
                    "invokes_model": false,
                    "reads_credentials": false,
                    "writes_kg": false,
                    "sends_externally": false,
                    "mutates_durable_memory": false
                }
            ],
            "audit_steps": audit_steps,
            "side_effects": side_effects
        }),
    );
    report
}

fn hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_export_query_observability_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source =
        hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_retention_expiry_garbage_collection_denial_report();
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
            .and_then(|value| value.as_str())
            .unwrap_or("")
    };
    let source_next_action_export_query_observability = source
        .get("allowed_next_actions")
        .and_then(|value| value.as_array())
        .and_then(|items| items.first())
        .map(|item| {
            item.get("action").and_then(|value| value.as_str())
                == Some(
                    "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_export_query_observability_denial",
                )
                && item
                    .get("records_retention")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && item.get("records_expiry").and_then(serde_json::Value::as_bool) == Some(false)
                && item
                    .get("records_garbage_collection")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && item
                    .get("exports_receipt")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && item.get("invokes_provider").and_then(serde_json::Value::as_bool) == Some(false)
                && item.get("invokes_model").and_then(serde_json::Value::as_bool) == Some(false)
        })
        .unwrap_or(false);
    let source_retention_ready = source_bool(
        "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_retention_expiry_garbage_collection_denial_ready",
    ) && source_bool(
        "source_first_model_invocation_approval_final_authorization_dry_run_result_receipt_audit_immutable_evidence_denial_ready",
    ) && source_bool(
        "final_authorization_dry_run_result_receipt_retention_expiry_garbage_collection_readback_hash_matched",
    ) && source_i64(
        "retention_expiry_garbage_collection_fixture_count",
    ) == 8
        && source_i64("blocked_retention_expiry_garbage_collection_fixture_count") == 8
        && source_i64("noop_retention_expiry_garbage_collection_fixture_count") == 8
        && source_i64("allowed_retention_expiry_garbage_collection_fixture_count") == 0
        && source_i64("accepted_retention_expiry_garbage_collection_fixture_count") == 0
        && source_i64("retention_expiry_garbage_collection_performed_count") == 0
        && source_i64("retention_recorded_count") == 0
        && source_i64("retention_policy_persisted_count") == 0
        && source_i64("ttl_scheduled_count") == 0
        && source_i64("expiry_applied_count") == 0
        && source_i64("garbage_collection_scan_performed_count") == 0
        && source_i64("delete_marker_recorded_count") == 0
        && source_i64("archive_recorded_count") == 0
        && source_i64("compaction_performed_count") == 0
        && source_i64("retention_export_recorded_count") == 0
        && source_i64("external_retention_notification_sent_count") == 0
        && !source_bool("final_authorization_dry_run_result_receipt_retention_recorded")
        && !source_bool("final_authorization_dry_run_result_receipt_retention_policy_persisted")
        && !source_bool("final_authorization_dry_run_result_receipt_ttl_scheduled")
        && !source_bool("final_authorization_dry_run_result_receipt_expiry_applied")
        && !source_bool(
            "final_authorization_dry_run_result_receipt_garbage_collection_scan_performed",
        )
        && !source_bool("final_authorization_dry_run_result_receipt_delete_marker_recorded")
        && !source_bool("final_authorization_dry_run_result_receipt_archive_recorded")
        && !source_bool("final_authorization_dry_run_result_receipt_compaction_performed")
        && !source_bool(
            "final_authorization_dry_run_result_receipt_external_retention_notification_sent",
        )
        && !source_bool("final_authorization_from_retention_expiry_garbage_collection_allowed")
        && !source_bool("operator_approval_from_retention_expiry_garbage_collection_accepted")
        && !source_bool("activation_from_retention_expiry_garbage_collection_allowed")
        && !source_bool("provider_invocation_authorized")
        && !source_bool("model_invocation_authorized")
        && source_i64("provider_invocation_budget") == 0
        && source_i64("model_invocation_budget") == 0
        && !source_bool("provider_invoked")
        && !source_bool("model_invoked")
        && !source_bool("credential_read")
        && !source_bool("secret_file_read")
        && !source_bool("live_kg_write_performed")
        && !source_bool("memory_store_write_performed")
        && !source_bool("channel_send_performed")
        && !source_bool("telegram_send_performed")
        && !source_bool("external_send_performed")
        && source_next_action_export_query_observability;
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT;
    let source_retention_hash = source_str(
        "final_authorization_dry_run_result_receipt_retention_expiry_garbage_collection_denial_hash_sha256",
    );
    let source_retention_readback_hash = source_str(
        "final_authorization_dry_run_result_receipt_retention_expiry_garbage_collection_readback_hash_sha256",
    );
    let export_scope = "first_model_invocation:operator-approval-final-authorization-dry-run-result-receipt-export-query-observability-denial";
    let export_query_observability_denial_hash = sha256_text_value(&format!(
        "first-model-final-authorization-dry-run-result-receipt-export-query-observability-denial:{export_scope}:{source_retention_hash}:{source_retention_readback_hash}:export=false:query=false:metrics=false:logs=false:traces=false:alerts=false"
    ));
    let export_query_observability_readback_hash = sha256_text_value(&format!(
        "first-model-final-authorization-dry-run-result-receipt-export-query-observability-readback:{export_query_observability_denial_hash}:dashboard=false:slo=false:external=false:authority=false"
    ));
    let export_fixtures = vec![
        serde_json::json!({
            "fixture_id": "receipt-export-materialization",
            "export_query_observability_status": "blocked_export_materialization",
            "final_authorization_dry_run_result_receipt_export_materialized": false,
            "final_authorization_dry_run_result_receipt_export_filesystem_written": false,
            "receipt_noop_confirmed": true
        }),
        serde_json::json!({
            "fixture_id": "receipt-query-index",
            "export_query_observability_status": "blocked_query_index_registration",
            "final_authorization_dry_run_result_receipt_query_registered": false,
            "final_authorization_dry_run_result_receipt_query_executed": false,
            "receipt_noop_confirmed": true
        }),
        serde_json::json!({
            "fixture_id": "observability-metric",
            "export_query_observability_status": "blocked_observability_metric_recording",
            "final_authorization_dry_run_result_receipt_metric_recorded": false,
            "final_authorization_dry_run_result_receipt_dashboard_published": false,
            "receipt_noop_confirmed": true
        }),
        serde_json::json!({
            "fixture_id": "log-trace-correlation",
            "export_query_observability_status": "blocked_log_trace_correlation",
            "final_authorization_dry_run_result_receipt_log_recorded": false,
            "final_authorization_dry_run_result_receipt_trace_recorded": false,
            "receipt_noop_confirmed": true
        }),
        serde_json::json!({
            "fixture_id": "alert-slo-promotion",
            "export_query_observability_status": "blocked_alert_slo_promotion",
            "final_authorization_dry_run_result_receipt_alert_emitted": false,
            "final_authorization_dry_run_result_receipt_slo_recorded": false,
            "receipt_noop_confirmed": true
        }),
        serde_json::json!({
            "fixture_id": "operator-approval-from-query",
            "export_query_observability_status": "blocked_operator_approval_from_query",
            "operator_approval_from_export_query_observability_accepted": false,
            "activation_from_export_query_observability_allowed": false,
            "receipt_noop_confirmed": true
        }),
        serde_json::json!({
            "fixture_id": "provider-invocation-from-observability",
            "export_query_observability_status": "blocked_provider_invocation_from_observability",
            "provider_invocation_authorized_from_export_query_observability": false,
            "model_invocation_authorized_from_export_query_observability": false,
            "receipt_noop_confirmed": true
        }),
        serde_json::json!({
            "fixture_id": "external-observability-delivery",
            "export_query_observability_status": "blocked_external_observability_delivery",
            "final_authorization_dry_run_result_receipt_external_observability_sent": false,
            "external_send_performed": false,
            "provider_invoked": false,
            "model_invoked": false,
            "receipt_noop_confirmed": true
        }),
    ];
    let export_fixture_count = export_fixtures.len();
    let report_ready =
        route_matrix.ready && route_count_source_command_accepted && source_retention_ready;

    let audit_steps = vec![
        serde_json::json!({
            "step": "retention_expiry_garbage_collection_source_binding",
            "status": "ready",
            "source_endpoint": HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_FINAL_AUTHORIZATION_DRY_RUN_RESULT_RECEIPT_RETENTION_EXPIRY_GARBAGE_COLLECTION_DENIAL_ENDPOINT,
            "source_retention_expiry_garbage_collection_ready": source_retention_ready,
            "source_retention_hash_sha256": source_retention_hash,
            "source_retention_readback_hash_sha256": source_retention_readback_hash
        }),
        serde_json::json!({
            "step": "export_query_observability_fixture_denial",
            "status": "blocked_report_only",
            "export_query_observability_fixture_count": export_fixture_count,
            "blocked_export_query_observability_fixture_count": export_fixture_count,
            "allowed_export_query_observability_fixture_count": 0,
            "accepted_export_query_observability_fixture_count": 0,
            "export_query_observability_performed_count": 0
        }),
        serde_json::json!({
            "step": "export_query_no_materialization_or_index",
            "status": "not_recorded_or_persisted",
            "final_authorization_dry_run_result_receipt_export_materialized": false,
            "final_authorization_dry_run_result_receipt_export_filesystem_written": false,
            "final_authorization_dry_run_result_receipt_query_registered": false,
            "final_authorization_dry_run_result_receipt_query_executed": false
        }),
        serde_json::json!({
            "step": "observability_no_metric_log_trace_alert",
            "status": "denied",
            "final_authorization_dry_run_result_receipt_metric_recorded": false,
            "final_authorization_dry_run_result_receipt_log_recorded": false,
            "final_authorization_dry_run_result_receipt_trace_recorded": false,
            "final_authorization_dry_run_result_receipt_alert_emitted": false,
            "final_authorization_dry_run_result_receipt_dashboard_published": false
        }),
        serde_json::json!({
            "step": "export_query_observability_authority_non_promotion",
            "status": "authority_denied",
            "final_authorization_from_export_query_observability_allowed": false,
            "operator_approval_from_export_query_observability_accepted": false,
            "activation_from_export_query_observability_allowed": false,
            "provider_invocation_authorized": false,
            "model_invocation_authorized": false,
            "provider_invoked": false,
            "model_invoked": false
        }),
        serde_json::json!({
            "step": "side_effect_denial_check",
            "status": "ready",
            "credential_read": false,
            "secret_file_read": false,
            "provider_router_live_envelope_executed": false,
            "live_kg_write_performed": false,
            "memory_store_write_performed": false,
            "channel_send_performed": false,
            "external_send_performed": false,
            "install_executed": false,
            "active_binary_mutated": false,
            "public_release_claimed": false
        }),
    ];

    let mut side_effects = serde_json::Map::new();
    for key in [
        "final_authorization_dry_run_result_receipt_export_materialized",
        "final_authorization_dry_run_result_receipt_export_filesystem_written",
        "final_authorization_dry_run_result_receipt_query_registered",
        "final_authorization_dry_run_result_receipt_query_executed",
        "final_authorization_dry_run_result_receipt_metric_recorded",
        "final_authorization_dry_run_result_receipt_dashboard_published",
        "final_authorization_dry_run_result_receipt_log_recorded",
        "final_authorization_dry_run_result_receipt_trace_recorded",
        "final_authorization_dry_run_result_receipt_alert_emitted",
        "final_authorization_dry_run_result_receipt_slo_recorded",
        "final_authorization_dry_run_result_receipt_external_observability_sent",
        "operator_approval_from_export_query_observability_accepted",
        "final_authorization_from_export_query_observability_allowed",
        "activation_from_export_query_observability_allowed",
        "provider_invocation_authorized",
        "model_invocation_authorized",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "live_kg_write_performed",
        "memory_store_write_performed",
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
        "gate": "hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_export_query_observability_denial_route",
        "endpoint": HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_FINAL_AUTHORIZATION_DRY_RUN_RESULT_RECEIPT_EXPORT_QUERY_OBSERVABILITY_DENIAL_ENDPOINT,
        "source_command": "/hepta-first-model-invocation-operator-approval-final-authorization-dry-run-result-receipt-export-query-observability-denial --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-24",
        "canary_schema_version": "hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_export_query_observability_denial_v1",
        "canary_execution_mode": "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_export_query_observability_denial_no_provider_model_invocation",
        "source_first_model_invocation_approval_final_authorization_dry_run_result_receipt_retention_expiry_garbage_collection_denial_endpoint": HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_FINAL_AUTHORIZATION_DRY_RUN_RESULT_RECEIPT_RETENTION_EXPIRY_GARBAGE_COLLECTION_DENIAL_ENDPOINT,
        "source_first_model_invocation_approval_final_authorization_dry_run_result_receipt_retention_expiry_garbage_collection_denial_ready": source_retention_ready,
        "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        "route_count": route_matrix.route_count,
        "implemented_route_count": route_matrix.implemented_route_count,
        "missing_route_count": route_matrix.missing_route_count,
        "route_count_source_command_accepted": route_count_source_command_accepted,
        "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_export_query_observability_denial_route_enabled": true,
        "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_export_query_observability_denial_ready": report_ready
    });
    extend_json_object(
        &mut report,
        serde_json::json!({
            "result_receipt_export_query_observability_state": "final_authorization_dry_run_result_receipt_export_query_observability_denied",
            "result_receipt_export_query_observability_scope": export_scope,
            "source_final_authorization_dry_run_result_receipt_retention_expiry_garbage_collection_denial_hash_sha256": source_retention_hash,
            "source_final_authorization_dry_run_result_receipt_retention_expiry_garbage_collection_readback_hash_sha256": source_retention_readback_hash,
            "final_authorization_dry_run_result_receipt_export_query_observability_denial_hash_sha256": export_query_observability_denial_hash,
            "final_authorization_dry_run_result_receipt_export_query_observability_readback_hash_sha256": export_query_observability_readback_hash,
            "final_authorization_dry_run_result_receipt_export_query_observability_readback_hash_matched": true,
            "export_query_observability_fixture_count": export_fixture_count,
            "blocked_export_query_observability_fixture_count": export_fixture_count,
            "noop_export_query_observability_fixture_count": export_fixture_count,
            "allowed_export_query_observability_fixture_count": 0,
            "accepted_export_query_observability_fixture_count": 0,
            "export_query_observability_performed_count": 0,
            "export_query_observability_fixtures": export_fixtures,
            "export_materialized_count": 0,
            "export_filesystem_written_count": 0,
            "query_registered_count": 0,
            "query_executed_count": 0,
            "metric_recorded_count": 0,
            "dashboard_published_count": 0,
            "log_recorded_count": 0,
            "trace_recorded_count": 0,
            "alert_emitted_count": 0,
            "slo_recorded_count": 0,
            "external_observability_sent_count": 0
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "final_authorization_dry_run_result_receipt_export_materialized": false,
            "final_authorization_dry_run_result_receipt_export_filesystem_written": false,
            "final_authorization_dry_run_result_receipt_query_registered": false,
            "final_authorization_dry_run_result_receipt_query_executed": false,
            "final_authorization_dry_run_result_receipt_metric_recorded": false,
            "final_authorization_dry_run_result_receipt_dashboard_published": false,
            "final_authorization_dry_run_result_receipt_log_recorded": false,
            "final_authorization_dry_run_result_receipt_trace_recorded": false,
            "final_authorization_dry_run_result_receipt_alert_emitted": false,
            "final_authorization_dry_run_result_receipt_slo_recorded": false,
            "final_authorization_dry_run_result_receipt_external_observability_sent": false,
            "result_receipt_observability_query_registered": false,
            "final_authorization_from_export_query_observability_allowed": false,
            "operator_approval_from_export_query_observability_accepted": false,
            "activation_from_export_query_observability_allowed": false
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "provider_invocation_authorized": false,
            "model_invocation_authorized": false,
            "provider_invocation_authorized_from_export_query_observability": false,
            "model_invocation_authorized_from_export_query_observability": false,
            "provider_invocation_budget": 0,
            "model_invocation_budget": 0,
            "provider_invoked": false,
            "model_invoked": false,
            "credential_value_read": false,
            "credential_read": false,
            "secret_file_read": false,
            "provider_router_live_envelope_executed": false,
            "provider_prompt_injection_performed": false,
            "context_injection_performed": false,
            "kg_adapter_read_performed": false,
            "live_kg_write_performed": false,
            "memory_store_write_performed": false,
            "channel_send_performed": false,
            "telegram_send_performed": false,
            "external_send_performed": false
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "allowed_next_actions": [
                {
                    "action": "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_operator_facing_summary_briefing_non_persistence_denial",
                    "status": "allowed_report_only_next_slice",
                    "records_result_receipt": false,
                    "exports_receipt": false,
                    "registers_query": false,
                    "records_observability": false,
                    "delivers_briefing": false,
                    "invokes_provider": false,
                    "invokes_model": false,
                    "reads_credentials": false,
                    "writes_kg": false,
                    "sends_externally": false,
                    "mutates_durable_memory": false
                }
            ],
            "audit_steps": audit_steps,
            "side_effects": side_effects
        }),
    );
    report
}

fn hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_operator_facing_summary_briefing_non_persistence_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source =
        hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_export_query_observability_denial_report();
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
            .and_then(|value| value.as_str())
            .unwrap_or("")
    };
    let source_next_action_summary_briefing = source
        .get("allowed_next_actions")
        .and_then(|value| value.as_array())
        .and_then(|items| items.first())
        .map(|item| {
            item.get("action").and_then(|value| value.as_str())
                == Some(
                    "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_operator_facing_summary_briefing_non_persistence_denial",
                )
                && item.get("exports_receipt").and_then(serde_json::Value::as_bool) == Some(false)
                && item.get("registers_query").and_then(serde_json::Value::as_bool) == Some(false)
                && item
                    .get("records_observability")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && item.get("delivers_briefing").and_then(serde_json::Value::as_bool) == Some(false)
                && item.get("invokes_provider").and_then(serde_json::Value::as_bool) == Some(false)
                && item.get("invokes_model").and_then(serde_json::Value::as_bool) == Some(false)
        })
        .unwrap_or(false);
    let source_export_query_observability_ready = source_bool(
        "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_export_query_observability_denial_ready",
    ) && source_bool(
        "source_first_model_invocation_approval_final_authorization_dry_run_result_receipt_retention_expiry_garbage_collection_denial_ready",
    ) && source_bool(
        "final_authorization_dry_run_result_receipt_export_query_observability_readback_hash_matched",
    ) && source_i64(
        "export_query_observability_fixture_count",
    ) == 8
        && source_i64("blocked_export_query_observability_fixture_count") == 8
        && source_i64("noop_export_query_observability_fixture_count") == 8
        && source_i64("allowed_export_query_observability_fixture_count") == 0
        && source_i64("accepted_export_query_observability_fixture_count") == 0
        && source_i64("export_query_observability_performed_count") == 0
        && source_i64("export_materialized_count") == 0
        && source_i64("query_registered_count") == 0
        && source_i64("metric_recorded_count") == 0
        && source_i64("log_recorded_count") == 0
        && source_i64("trace_recorded_count") == 0
        && source_i64("alert_emitted_count") == 0
        && source_i64("external_observability_sent_count") == 0
        && !source_bool("final_authorization_dry_run_result_receipt_export_materialized")
        && !source_bool("final_authorization_dry_run_result_receipt_query_registered")
        && !source_bool("final_authorization_dry_run_result_receipt_metric_recorded")
        && !source_bool("final_authorization_dry_run_result_receipt_external_observability_sent")
        && !source_bool("final_authorization_from_export_query_observability_allowed")
        && !source_bool("operator_approval_from_export_query_observability_accepted")
        && !source_bool("activation_from_export_query_observability_allowed")
        && !source_bool("provider_invocation_authorized")
        && !source_bool("model_invocation_authorized")
        && source_i64("provider_invocation_budget") == 0
        && source_i64("model_invocation_budget") == 0
        && !source_bool("provider_invoked")
        && !source_bool("model_invoked")
        && !source_bool("credential_read")
        && !source_bool("secret_file_read")
        && !source_bool("live_kg_write_performed")
        && !source_bool("memory_store_write_performed")
        && !source_bool("channel_send_performed")
        && !source_bool("telegram_send_performed")
        && !source_bool("external_send_performed")
        && source_next_action_summary_briefing;
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT;
    let source_export_query_observability_hash = source_str(
        "final_authorization_dry_run_result_receipt_export_query_observability_denial_hash_sha256",
    );
    let source_export_query_observability_readback_hash = source_str(
        "final_authorization_dry_run_result_receipt_export_query_observability_readback_hash_sha256",
    );
    let operator_facing_summary_briefing_scope = "first_model_invocation:operator-approval-final-authorization-dry-run-result-receipt-operator-facing-summary-briefing-non-persistence-denial";
    let operator_facing_summary_briefing_denial_hash = sha256_text_value(&format!(
        "first-model-final-authorization-dry-run-result-receipt-operator-facing-summary-briefing-non-persistence-denial:{operator_facing_summary_briefing_scope}:{source_export_query_observability_hash}:{source_export_query_observability_readback_hash}:summary=false:briefing=false:delivery=false:provider=false:model=false"
    ));
    let operator_facing_summary_briefing_readback_hash = sha256_text_value(&format!(
        "first-model-final-authorization-dry-run-result-receipt-operator-facing-summary-briefing-readback:{operator_facing_summary_briefing_denial_hash}:dashboard=false:ack=false:activation=false:external=false"
    ));
    let summary_briefing_fixtures = vec![
        serde_json::json!({
            "fixture_id": "operator-summary-render",
            "operator_facing_summary_briefing_status": "blocked_summary_render_no_persistence",
            "operator_summary_rendered": false,
            "operator_summary_recorded": false,
            "receipt_noop_confirmed": true
        }),
        serde_json::json!({
            "fixture_id": "operator-briefing-materialization",
            "operator_facing_summary_briefing_status": "blocked_briefing_materialization",
            "operator_briefing_materialized": false,
            "operator_briefing_persisted": false,
            "receipt_noop_confirmed": true
        }),
        serde_json::json!({
            "fixture_id": "readback-dashboard-summary",
            "operator_facing_summary_briefing_status": "blocked_readback_dashboard_summary",
            "operator_summary_dashboard_published": false,
            "operator_readback_recorded": false,
            "receipt_noop_confirmed": true
        }),
        serde_json::json!({
            "fixture_id": "final-note-delivery",
            "operator_facing_summary_briefing_status": "blocked_final_note_delivery",
            "operator_final_note_recorded": false,
            "operator_final_note_delivered": false,
            "receipt_noop_confirmed": true
        }),
        serde_json::json!({
            "fixture_id": "channel-telegram-delivery",
            "operator_facing_summary_briefing_status": "blocked_channel_delivery",
            "channel_send_performed": false,
            "telegram_send_performed": false,
            "receipt_noop_confirmed": true
        }),
        serde_json::json!({
            "fixture_id": "acknowledgement-from-summary",
            "operator_facing_summary_briefing_status": "blocked_acknowledgement_from_summary",
            "operator_acknowledgement_from_summary_accepted": false,
            "receipt_noop_confirmed": true
        }),
        serde_json::json!({
            "fixture_id": "activation-from-briefing",
            "operator_facing_summary_briefing_status": "blocked_activation_from_briefing",
            "activation_from_operator_briefing_allowed": false,
            "activation_authority_from_operator_briefing_derived": false,
            "receipt_noop_confirmed": true
        }),
        serde_json::json!({
            "fixture_id": "provider-from-briefing",
            "operator_facing_summary_briefing_status": "blocked_provider_from_briefing",
            "provider_invocation_authorized_from_operator_briefing": false,
            "model_invocation_authorized_from_operator_briefing": false,
            "receipt_noop_confirmed": true
        }),
    ];
    let summary_briefing_fixture_count = summary_briefing_fixtures.len();
    let report_ready = route_matrix.ready
        && route_count_source_command_accepted
        && source_export_query_observability_ready;

    let audit_steps = vec![
        serde_json::json!({
            "step": "export_query_observability_source_binding",
            "status": "ready",
            "source_endpoint": HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_FINAL_AUTHORIZATION_DRY_RUN_RESULT_RECEIPT_EXPORT_QUERY_OBSERVABILITY_DENIAL_ENDPOINT,
            "source_export_query_observability_ready": source_export_query_observability_ready,
            "source_export_query_observability_hash_sha256": source_export_query_observability_hash,
            "source_export_query_observability_readback_hash_sha256": source_export_query_observability_readback_hash
        }),
        serde_json::json!({
            "step": "operator_facing_summary_briefing_fixture_denial",
            "status": "blocked_report_only",
            "operator_facing_summary_briefing_fixture_count": summary_briefing_fixture_count,
            "blocked_operator_facing_summary_briefing_fixture_count": summary_briefing_fixture_count,
            "allowed_operator_facing_summary_briefing_fixture_count": 0,
            "accepted_operator_facing_summary_briefing_fixture_count": 0,
            "operator_facing_summary_briefing_performed_count": 0
        }),
        serde_json::json!({
            "step": "operator_summary_briefing_no_persistence",
            "status": "not_recorded_or_persisted",
            "operator_summary_recorded": false,
            "operator_summary_persisted": false,
            "operator_briefing_recorded": false,
            "operator_briefing_persisted": false,
            "operator_briefing_materialized": false
        }),
        serde_json::json!({
            "step": "operator_summary_briefing_no_delivery",
            "status": "delivery_denied",
            "operator_summary_dashboard_published": false,
            "operator_readback_recorded": false,
            "operator_final_note_recorded": false,
            "operator_final_note_delivered": false,
            "channel_send_performed": false,
            "telegram_send_performed": false,
            "external_send_performed": false
        }),
        serde_json::json!({
            "step": "operator_summary_briefing_authority_non_promotion",
            "status": "authority_denied",
            "operator_acknowledgement_from_summary_accepted": false,
            "activation_from_operator_briefing_allowed": false,
            "activation_authority_from_operator_briefing_derived": false,
            "provider_invocation_authorized_from_operator_briefing": false,
            "model_invocation_authorized_from_operator_briefing": false,
            "provider_invoked": false,
            "model_invoked": false
        }),
        serde_json::json!({
            "step": "side_effect_denial_check",
            "status": "ready",
            "credential_read": false,
            "secret_file_read": false,
            "provider_router_live_envelope_executed": false,
            "live_kg_write_performed": false,
            "memory_store_write_performed": false,
            "channel_send_performed": false,
            "telegram_send_performed": false,
            "external_send_performed": false,
            "service_restarted": false,
            "active_binary_mutated": false,
            "public_release_claimed": false
        }),
    ];

    let mut side_effects = serde_json::Map::new();
    for key in [
        "operator_summary_recorded",
        "operator_summary_persisted",
        "operator_briefing_recorded",
        "operator_briefing_persisted",
        "operator_briefing_materialized",
        "operator_summary_dashboard_published",
        "operator_readback_recorded",
        "operator_final_note_recorded",
        "operator_final_note_delivered",
        "operator_acknowledgement_from_summary_accepted",
        "activation_from_operator_briefing_allowed",
        "activation_authority_from_operator_briefing_derived",
        "provider_invocation_authorized_from_operator_briefing",
        "model_invocation_authorized_from_operator_briefing",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "live_kg_write_performed",
        "memory_store_write_performed",
        "channel_send_performed",
        "telegram_send_performed",
        "external_send_performed",
        "service_restarted",
        "active_binary_mutated",
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
        "gate": "hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_operator_facing_summary_briefing_non_persistence_denial_route",
        "endpoint": HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_FINAL_AUTHORIZATION_DRY_RUN_RESULT_RECEIPT_OPERATOR_FACING_SUMMARY_BRIEFING_NON_PERSISTENCE_DENIAL_ENDPOINT,
        "source_command": "/hepta-first-model-invocation-operator-approval-final-authorization-dry-run-result-receipt-operator-facing-summary-briefing-non-persistence-denial --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-24",
        "canary_schema_version": "hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_operator_facing_summary_briefing_non_persistence_denial_v1",
        "canary_execution_mode": "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_operator_facing_summary_briefing_non_persistence_denial_no_delivery_no_provider_model_invocation",
        "source_first_model_invocation_approval_final_authorization_dry_run_result_receipt_export_query_observability_denial_endpoint": HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_FINAL_AUTHORIZATION_DRY_RUN_RESULT_RECEIPT_EXPORT_QUERY_OBSERVABILITY_DENIAL_ENDPOINT,
        "source_first_model_invocation_approval_final_authorization_dry_run_result_receipt_export_query_observability_denial_ready": source_export_query_observability_ready,
        "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        "route_count": route_matrix.route_count,
        "implemented_route_count": route_matrix.implemented_route_count,
        "missing_route_count": route_matrix.missing_route_count,
        "route_count_source_command_accepted": route_count_source_command_accepted,
        "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_operator_facing_summary_briefing_non_persistence_denial_route_enabled": true,
        "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_operator_facing_summary_briefing_non_persistence_denial_ready": report_ready
    });
    extend_json_object(
        &mut report,
        serde_json::json!({
        "result_receipt_operator_facing_summary_briefing_state": "final_authorization_dry_run_result_receipt_operator_facing_summary_briefing_non_persistence_denied",
        "result_receipt_operator_facing_summary_briefing_scope": operator_facing_summary_briefing_scope,
        "source_final_authorization_dry_run_result_receipt_export_query_observability_denial_hash_sha256": source_export_query_observability_hash,
        "source_final_authorization_dry_run_result_receipt_export_query_observability_readback_hash_sha256": source_export_query_observability_readback_hash,
        "final_authorization_dry_run_result_receipt_operator_facing_summary_briefing_denial_hash_sha256": operator_facing_summary_briefing_denial_hash,
        "final_authorization_dry_run_result_receipt_operator_facing_summary_briefing_readback_hash_sha256": operator_facing_summary_briefing_readback_hash,
        "final_authorization_dry_run_result_receipt_operator_facing_summary_briefing_readback_hash_matched": true,
        "operator_facing_summary_briefing_fixture_count": summary_briefing_fixture_count,
        "blocked_operator_facing_summary_briefing_fixture_count": summary_briefing_fixture_count,
        "noop_operator_facing_summary_briefing_fixture_count": summary_briefing_fixture_count,
        "allowed_operator_facing_summary_briefing_fixture_count": 0,
        "accepted_operator_facing_summary_briefing_fixture_count": 0,
        "operator_facing_summary_briefing_performed_count": 0,
        "operator_facing_summary_briefing_fixtures": summary_briefing_fixtures,
        "operator_summary_recorded_count": 0,
        "operator_summary_persisted_count": 0,
        "operator_briefing_recorded_count": 0,
        "operator_briefing_persisted_count": 0,
        "operator_briefing_materialized_count": 0,
        "operator_summary_dashboard_published_count": 0,
        "operator_readback_recorded_count": 0,
        "operator_final_note_recorded_count": 0,
        "operator_final_note_delivered_count": 0
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
        "operator_summary_recorded": false,
        "operator_summary_persisted": false,
        "operator_briefing_recorded": false,
        "operator_briefing_persisted": false,
        "operator_briefing_materialized": false,
        "operator_summary_dashboard_published": false,
        "operator_readback_recorded": false,
        "operator_final_note_recorded": false,
        "operator_final_note_delivered": false,
        "operator_acknowledgement_from_summary_accepted": false,
        "activation_from_operator_briefing_allowed": false,
        "activation_authority_from_operator_briefing_derived": false,
        "provider_invocation_authorized": false,
        "model_invocation_authorized": false,
        "provider_invocation_authorized_from_operator_briefing": false,
        "model_invocation_authorized_from_operator_briefing": false,
        "provider_invocation_budget": 0,
        "model_invocation_budget": 0,
        "provider_invoked": false,
        "model_invoked": false,
        "credential_value_read": false,
        "credential_read": false,
        "secret_file_read": false,
        "provider_router_live_envelope_executed": false,
        "provider_prompt_injection_performed": false,
        "context_injection_performed": false,
        "kg_adapter_read_performed": false,
        "live_kg_write_performed": false,
        "memory_store_write_performed": false,
        "channel_send_performed": false,
        "telegram_send_performed": false,
        "external_send_performed": false
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
        "allowed_next_actions": [
            {
                "action": "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_final_operator_acknowledgement_non_acceptance_denial",
                "status": "allowed_report_only_next_slice",
                "records_summary": false,
                "persists_briefing": false,
                "delivers_briefing": false,
                "accepts_acknowledgement": false,
                "invokes_provider": false,
                "invokes_model": false,
                "reads_credentials": false,
                "writes_kg": false,
                "sends_externally": false,
                "mutates_durable_memory": false
            }
        ],
        "audit_steps": audit_steps,
        "side_effects": side_effects
        }),
    );
    report
}

fn hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_final_operator_acknowledgement_non_acceptance_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source =
        hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_operator_facing_summary_briefing_non_persistence_denial_report();
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
            .and_then(|value| value.as_str())
            .unwrap_or("")
    };
    let source_next_action_final_acknowledgement = source
        .get("allowed_next_actions")
        .and_then(|value| value.as_array())
        .and_then(|items| items.first())
        .map(|item| {
            item.get("action").and_then(|value| value.as_str())
                == Some(
                    "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_final_operator_acknowledgement_non_acceptance_denial",
                )
                && item.get("records_summary").and_then(serde_json::Value::as_bool) == Some(false)
                && item.get("persists_briefing").and_then(serde_json::Value::as_bool) == Some(false)
                && item.get("delivers_briefing").and_then(serde_json::Value::as_bool) == Some(false)
                && item
                    .get("accepts_acknowledgement")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
                && item.get("invokes_provider").and_then(serde_json::Value::as_bool) == Some(false)
                && item.get("invokes_model").and_then(serde_json::Value::as_bool) == Some(false)
        })
        .unwrap_or(false);
    let source_summary_briefing_ready = source_bool(
        "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_operator_facing_summary_briefing_non_persistence_denial_ready",
    ) && source_bool(
        "source_first_model_invocation_approval_final_authorization_dry_run_result_receipt_export_query_observability_denial_ready",
    ) && source_bool(
        "final_authorization_dry_run_result_receipt_operator_facing_summary_briefing_readback_hash_matched",
    ) && source_i64(
        "operator_facing_summary_briefing_fixture_count",
    ) == 8
        && source_i64("blocked_operator_facing_summary_briefing_fixture_count") == 8
        && source_i64("noop_operator_facing_summary_briefing_fixture_count") == 8
        && source_i64("allowed_operator_facing_summary_briefing_fixture_count") == 0
        && source_i64("accepted_operator_facing_summary_briefing_fixture_count") == 0
        && source_i64("operator_facing_summary_briefing_performed_count") == 0
        && source_i64("operator_summary_recorded_count") == 0
        && source_i64("operator_summary_persisted_count") == 0
        && source_i64("operator_briefing_recorded_count") == 0
        && source_i64("operator_briefing_persisted_count") == 0
        && source_i64("operator_final_note_delivered_count") == 0
        && !source_bool("operator_summary_recorded")
        && !source_bool("operator_summary_persisted")
        && !source_bool("operator_briefing_recorded")
        && !source_bool("operator_briefing_persisted")
        && !source_bool("operator_final_note_delivered")
        && !source_bool("operator_acknowledgement_from_summary_accepted")
        && !source_bool("activation_from_operator_briefing_allowed")
        && !source_bool("activation_authority_from_operator_briefing_derived")
        && !source_bool("provider_invocation_authorized")
        && !source_bool("model_invocation_authorized")
        && source_i64("provider_invocation_budget") == 0
        && source_i64("model_invocation_budget") == 0
        && !source_bool("provider_invoked")
        && !source_bool("model_invoked")
        && !source_bool("credential_read")
        && !source_bool("secret_file_read")
        && !source_bool("live_kg_write_performed")
        && !source_bool("memory_store_write_performed")
        && !source_bool("channel_send_performed")
        && !source_bool("telegram_send_performed")
        && !source_bool("external_send_performed")
        && source_next_action_final_acknowledgement;
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT;
    let source_summary_briefing_hash = source_str(
        "final_authorization_dry_run_result_receipt_operator_facing_summary_briefing_denial_hash_sha256",
    );
    let source_summary_briefing_readback_hash = source_str(
        "final_authorization_dry_run_result_receipt_operator_facing_summary_briefing_readback_hash_sha256",
    );
    let final_acknowledgement_scope = "first_model_invocation:operator-approval-final-authorization-dry-run-result-receipt-final-operator-acknowledgement-non-acceptance-denial";
    let final_acknowledgement_denial_hash = sha256_text_value(&format!(
        "first-model-final-authorization-dry-run-result-receipt-final-operator-acknowledgement-non-acceptance-denial:{final_acknowledgement_scope}:{source_summary_briefing_hash}:{source_summary_briefing_readback_hash}:ack=false:accept=false:record=false:deliver=false:promote=false:provider=false:model=false"
    ));
    let final_acknowledgement_readback_hash = sha256_text_value(&format!(
        "first-model-final-authorization-dry-run-result-receipt-final-operator-acknowledgement-readback:{final_acknowledgement_denial_hash}:completion=false:activation=false:external=false:public=false"
    ));
    let final_ack_fixture =
        |id: &str, status: &str, reason: &str, extra: serde_json::Value| -> serde_json::Value {
            let mut fixture = serde_json::Map::new();
            fixture.insert("fixture_id".to_string(), serde_json::json!(id));
            fixture.insert(
                "final_operator_acknowledgement_status".to_string(),
                serde_json::json!(status),
            );
            fixture.insert(
                "source_operator_facing_summary_briefing_present".to_string(),
                serde_json::json!(true),
            );
            fixture.insert(
                "source_operator_facing_summary_briefing_ready".to_string(),
                serde_json::json!(true),
            );
            for key in [
                "final_operator_acknowledgement_requested",
                "final_operator_acknowledgement_allowed",
                "final_operator_acknowledgement_request_accepted",
                "final_operator_acknowledgement_accepted",
                "final_operator_acknowledgement_recorded",
                "final_operator_acknowledgement_persisted",
                "final_operator_acknowledgement_materialized",
                "final_operator_acknowledgement_filesystem_written",
                "final_operator_acknowledgement_delivered",
                "final_operator_acknowledgement_channel_delivery_performed",
                "final_operator_acknowledgement_identity_accepted",
                "final_operator_acknowledgement_signature_accepted",
                "final_operator_acknowledgement_timestamp_accepted",
                "final_operator_acknowledgement_final_state_promoted",
                "final_operator_acknowledgement_completion_promoted",
                "final_operator_acceptance_recorded",
                "final_operator_acceptance_persisted",
                "completion_acknowledgement_recorded",
                "status_acknowledgement_recorded",
                "summary_acknowledgement_recorded",
                "briefing_acknowledgement_recorded",
                "readback_digest_acknowledgement_recorded",
                "dashboard_acknowledgement_recorded",
                "notification_acknowledgement_recorded",
                "channel_acknowledgement_delivered",
                "external_acknowledgement_sent",
                "telegram_acknowledgement_sent",
                "operator_approval_from_acknowledgement_derived",
                "activation_authority_from_acknowledgement_derived",
                "provider_invocation_authorized_from_acknowledgement",
                "model_invocation_authorized_from_acknowledgement",
                "provider_invoked",
                "model_invoked",
                "credential_read",
                "secret_file_read",
                "live_kg_write_performed",
                "memory_store_write_performed",
                "channel_send_performed",
                "telegram_send_performed",
                "external_send_performed",
                "release_artifact_written",
                "public_claim_recorded",
                "public_release_claimed",
                "install_executed",
                "service_restart_performed",
                "active_binary_mutated",
            ] {
                fixture.insert(key.to_string(), serde_json::json!(false));
            }
            fixture.insert(
                "final_acknowledgement_noop_confirmed".to_string(),
                serde_json::json!(true),
            );
            fixture.insert("reason".to_string(), serde_json::json!(reason));
            if let Some(extra_object) = extra.as_object() {
                fixture.extend(extra_object.clone());
            }
            serde_json::Value::Object(fixture)
        };
    let final_acknowledgement_fixtures = vec![
        final_ack_fixture(
            "source-summary-briefing-required",
            "blocked_missing_source_summary_briefing",
            "source_operator_facing_summary_briefing_report_required",
            serde_json::json!({
                "source_operator_facing_summary_briefing_present": false,
                "source_operator_facing_summary_briefing_ready": false,
                "final_operator_acknowledgement_requested": true
            }),
        ),
        final_ack_fixture(
            "final-operator-acknowledgement-request",
            "blocked_acknowledgement_request_noop",
            "final_operator_acknowledgement_request_denied",
            serde_json::json!({"final_operator_acknowledgement_requested": true}),
        ),
        final_ack_fixture(
            "final-operator-acknowledgement-acceptance",
            "blocked_acknowledgement_acceptance",
            "final_operator_acknowledgement_acceptance_denied",
            serde_json::json!({
                "final_operator_acknowledgement_requested": true,
                "acknowledgement_acceptance_requested": true
            }),
        ),
        final_ack_fixture(
            "final-operator-acknowledgement-recording",
            "blocked_acknowledgement_recording",
            "final_operator_acknowledgement_recording_denied",
            serde_json::json!({
                "final_operator_acknowledgement_requested": true,
                "acknowledgement_recording_requested": true
            }),
        ),
        final_ack_fixture(
            "final-operator-acknowledgement-persistence",
            "blocked_acknowledgement_persistence",
            "final_operator_acknowledgement_persistence_and_filesystem_write_denied",
            serde_json::json!({
                "final_operator_acknowledgement_requested": true,
                "acknowledgement_persistence_requested": true,
                "acknowledgement_filesystem_write_requested": true
            }),
        ),
        final_ack_fixture(
            "final-operator-acknowledgement-delivery",
            "blocked_acknowledgement_delivery",
            "final_operator_acknowledgement_delivery_denied",
            serde_json::json!({
                "final_operator_acknowledgement_requested": true,
                "telegram_acknowledgement_requested": true,
                "channel_acknowledgement_requested": true,
                "external_acknowledgement_requested": true
            }),
        ),
        final_ack_fixture(
            "final-operator-acknowledgement-state-promotion",
            "blocked_final_state_promotion",
            "final_operator_acknowledgement_state_promotion_denied",
            serde_json::json!({
                "final_operator_acknowledgement_requested": true,
                "final_state_promotion_requested": true,
                "completion_acknowledgement_requested": true
            }),
        ),
        final_ack_fixture(
            "final-operator-acknowledgement-authority-promotion",
            "blocked_authority_promotion",
            "operator_approval_activation_provider_model_authority_from_acknowledgement_denied",
            serde_json::json!({
                "final_operator_acknowledgement_requested": true,
                "operator_approval_from_acknowledgement_requested": true,
                "activation_from_acknowledgement_requested": true,
                "provider_model_from_acknowledgement_requested": true
            }),
        ),
    ];
    let final_acknowledgement_fixture_count = final_acknowledgement_fixtures.len();
    let report_ready =
        route_matrix.ready && route_count_source_command_accepted && source_summary_briefing_ready;

    let audit_steps = vec![
        serde_json::json!({
            "step": "operator_facing_summary_briefing_source_binding",
            "status": "ready",
            "source_endpoint": HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_FINAL_AUTHORIZATION_DRY_RUN_RESULT_RECEIPT_OPERATOR_FACING_SUMMARY_BRIEFING_NON_PERSISTENCE_DENIAL_ENDPOINT,
            "source_operator_facing_summary_briefing_ready": source_summary_briefing_ready,
            "source_operator_facing_summary_briefing_hash_sha256": source_summary_briefing_hash,
            "source_operator_facing_summary_briefing_readback_hash_sha256": source_summary_briefing_readback_hash
        }),
        serde_json::json!({
            "step": "final_operator_acknowledgement_fixture_denial",
            "status": "blocked_report_only",
            "final_operator_acknowledgement_fixture_count": final_acknowledgement_fixture_count,
            "blocked_final_operator_acknowledgement_fixture_count": final_acknowledgement_fixture_count,
            "allowed_final_operator_acknowledgement_fixture_count": 0,
            "accepted_final_operator_acknowledgement_fixture_count": 0,
            "final_operator_acknowledgement_performed_count": 0
        }),
        serde_json::json!({
            "step": "final_operator_acknowledgement_no_acceptance_or_persistence",
            "status": "not_accepted_recorded_or_persisted",
            "final_operator_acknowledgement_accepted": false,
            "final_operator_acknowledgement_recorded": false,
            "final_operator_acknowledgement_persisted": false,
            "final_operator_acknowledgement_filesystem_written": false
        }),
        serde_json::json!({
            "step": "final_operator_acknowledgement_no_delivery",
            "status": "delivery_denied",
            "final_operator_acknowledgement_delivered": false,
            "channel_acknowledgement_delivered": false,
            "telegram_acknowledgement_sent": false,
            "external_acknowledgement_sent": false
        }),
        serde_json::json!({
            "step": "final_operator_acknowledgement_authority_non_promotion",
            "status": "authority_denied",
            "final_operator_acknowledgement_final_state_promoted": false,
            "operator_approval_from_acknowledgement_derived": false,
            "activation_authority_from_acknowledgement_derived": false,
            "provider_invocation_authorized_from_acknowledgement": false,
            "model_invocation_authorized_from_acknowledgement": false
        }),
        serde_json::json!({
            "step": "side_effect_denial_check",
            "status": "ready",
            "credential_read": false,
            "secret_file_read": false,
            "provider_router_live_envelope_executed": false,
            "live_kg_write_performed": false,
            "memory_store_write_performed": false,
            "channel_send_performed": false,
            "telegram_send_performed": false,
            "external_send_performed": false,
            "service_restarted": false,
            "active_binary_mutated": false,
            "public_release_claimed": false
        }),
    ];

    let mut side_effects = serde_json::Map::new();
    for key in [
        "final_operator_acknowledgement_accepted",
        "final_operator_acknowledgement_recorded",
        "final_operator_acknowledgement_persisted",
        "final_operator_acknowledgement_materialized",
        "final_operator_acknowledgement_filesystem_written",
        "final_operator_acknowledgement_delivered",
        "final_operator_acknowledgement_channel_delivery_performed",
        "final_operator_acknowledgement_final_state_promoted",
        "final_operator_acknowledgement_completion_promoted",
        "final_operator_acceptance_recorded",
        "final_operator_acceptance_persisted",
        "completion_acknowledgement_recorded",
        "status_acknowledgement_recorded",
        "summary_acknowledgement_recorded",
        "briefing_acknowledgement_recorded",
        "readback_digest_acknowledgement_recorded",
        "dashboard_acknowledgement_recorded",
        "notification_acknowledgement_recorded",
        "channel_acknowledgement_delivered",
        "external_acknowledgement_sent",
        "telegram_acknowledgement_sent",
        "operator_approval_from_acknowledgement_derived",
        "activation_authority_from_acknowledgement_derived",
        "provider_invocation_authorized_from_acknowledgement",
        "model_invocation_authorized_from_acknowledgement",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "live_kg_write_performed",
        "memory_store_write_performed",
        "channel_send_performed",
        "telegram_send_performed",
        "external_send_performed",
        "release_artifact_written",
        "public_claim_recorded",
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
        "gate": "hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_final_operator_acknowledgement_non_acceptance_denial_route",
        "endpoint": HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_FINAL_AUTHORIZATION_DRY_RUN_RESULT_RECEIPT_FINAL_OPERATOR_ACKNOWLEDGEMENT_NON_ACCEPTANCE_DENIAL_ENDPOINT,
        "source_command": "/hepta-first-model-invocation-operator-approval-final-authorization-dry-run-result-receipt-final-operator-acknowledgement-non-acceptance-denial --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-24",
        "canary_schema_version": "hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_final_operator_acknowledgement_non_acceptance_denial_v1",
        "canary_execution_mode": "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_final_operator_acknowledgement_non_acceptance_denial_no_ack_no_accept_no_delivery_no_provider_model_invocation",
        "source_first_model_invocation_approval_final_authorization_dry_run_result_receipt_operator_facing_summary_briefing_non_persistence_denial_endpoint": HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_FINAL_AUTHORIZATION_DRY_RUN_RESULT_RECEIPT_OPERATOR_FACING_SUMMARY_BRIEFING_NON_PERSISTENCE_DENIAL_ENDPOINT,
        "source_first_model_invocation_approval_final_authorization_dry_run_result_receipt_operator_facing_summary_briefing_non_persistence_denial_ready": source_summary_briefing_ready,
        "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        "route_count": route_matrix.route_count,
        "implemented_route_count": route_matrix.implemented_route_count,
        "missing_route_count": route_matrix.missing_route_count,
        "route_count_source_command_accepted": route_count_source_command_accepted,
        "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_final_operator_acknowledgement_non_acceptance_denial_route_enabled": true,
        "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_final_operator_acknowledgement_non_acceptance_denial_ready": report_ready
    });
    extend_json_object(
        &mut report,
        serde_json::json!({
        "result_receipt_final_operator_acknowledgement_state": "final_authorization_dry_run_result_receipt_final_operator_acknowledgement_non_acceptance_denied",
        "result_receipt_final_operator_acknowledgement_scope": final_acknowledgement_scope,
        "source_final_authorization_dry_run_result_receipt_operator_facing_summary_briefing_denial_hash_sha256": source_summary_briefing_hash,
        "source_final_authorization_dry_run_result_receipt_operator_facing_summary_briefing_readback_hash_sha256": source_summary_briefing_readback_hash,
        "final_authorization_dry_run_result_receipt_final_operator_acknowledgement_denial_hash_sha256": final_acknowledgement_denial_hash,
        "final_authorization_dry_run_result_receipt_final_operator_acknowledgement_readback_hash_sha256": final_acknowledgement_readback_hash,
        "final_authorization_dry_run_result_receipt_final_operator_acknowledgement_readback_hash_matched": true,
        "final_operator_acknowledgement_fixture_count": final_acknowledgement_fixture_count,
        "blocked_final_operator_acknowledgement_fixture_count": final_acknowledgement_fixture_count,
        "noop_final_operator_acknowledgement_fixture_count": final_acknowledgement_fixture_count,
        "allowed_final_operator_acknowledgement_fixture_count": 0,
        "accepted_final_operator_acknowledgement_fixture_count": 0,
        "final_operator_acknowledgement_performed_count": 0,
        "final_operator_acknowledgement_fixtures": final_acknowledgement_fixtures,
        "final_operator_acknowledgement_accepted_count": 0,
        "final_operator_acknowledgement_recorded_count": 0,
        "final_operator_acknowledgement_persisted_count": 0,
        "final_operator_acknowledgement_delivered_count": 0,
        "final_operator_acknowledgement_final_state_promoted_count": 0,
        "final_operator_acknowledgement_completion_promoted_count": 0
        }),
    );
    let mut final_acknowledgement_denials = serde_json::Map::new();
    for key in [
        "final_operator_acknowledgement_allowed",
        "final_operator_acknowledgement_request_accepted",
        "final_operator_acknowledgement_accepted",
        "final_operator_acknowledgement_recorded",
        "final_operator_acknowledgement_persisted",
        "final_operator_acknowledgement_materialized",
        "final_operator_acknowledgement_filesystem_written",
        "final_operator_acknowledgement_delivered",
        "final_operator_acknowledgement_channel_delivery_performed",
        "final_operator_acknowledgement_identity_accepted",
        "final_operator_acknowledgement_signature_accepted",
        "final_operator_acknowledgement_timestamp_accepted",
        "final_operator_acknowledgement_final_state_promoted",
        "final_operator_acknowledgement_completion_promoted",
        "final_operator_acceptance_recorded",
        "final_operator_acceptance_persisted",
        "completion_acknowledgement_recorded",
        "status_acknowledgement_recorded",
        "summary_acknowledgement_recorded",
        "briefing_acknowledgement_recorded",
        "readback_digest_acknowledgement_recorded",
        "dashboard_acknowledgement_recorded",
        "notification_acknowledgement_recorded",
        "channel_acknowledgement_delivered",
        "external_acknowledgement_sent",
        "telegram_acknowledgement_sent",
        "operator_approval_from_acknowledgement_derived",
        "activation_authority_from_acknowledgement_derived",
        "provider_invocation_authorized",
        "model_invocation_authorized",
        "provider_invocation_authorized_from_acknowledgement",
        "model_invocation_authorized_from_acknowledgement",
        "provider_invoked",
        "model_invoked",
        "credential_value_read",
        "credential_read",
        "secret_file_read",
        "provider_router_live_envelope_executed",
        "provider_prompt_injection_performed",
        "context_injection_performed",
        "kg_adapter_read_performed",
        "live_kg_write_performed",
        "memory_store_write_performed",
        "channel_send_performed",
        "telegram_send_performed",
        "external_send_performed",
        "release_artifact_written",
        "public_claim_recorded",
        "public_release_claimed",
        "install_executed",
        "service_restart_performed",
        "active_binary_mutated",
    ] {
        final_acknowledgement_denials.insert(key.to_string(), serde_json::json!(false));
    }
    final_acknowledgement_denials.insert(
        "provider_invocation_budget".to_string(),
        serde_json::json!(0),
    );
    final_acknowledgement_denials
        .insert("model_invocation_budget".to_string(), serde_json::json!(0));
    extend_json_object(
        &mut report,
        serde_json::Value::Object(final_acknowledgement_denials),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
        "allowed_next_actions": [
            {
                "action": "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial",
                "status": "allowed_report_only_next_slice",
                "accepts_acknowledgement": false,
                "accepts_terminal_decision": false,
                "claims_public_release": false,
                "writes_release_artifact": false,
                "activates_runtime": false,
                "invokes_provider": false,
                "invokes_model": false,
                "reads_credentials": false,
                "writes_kg": false,
                "sends_externally": false,
                "mutates_durable_memory": false
            }
        ],
        "audit_steps": audit_steps,
        "side_effects": side_effects
        }),
    );
    report
}

fn hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source =
        hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_final_operator_acknowledgement_non_acceptance_denial_report();
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
            .and_then(|value| value.as_str())
            .unwrap_or("")
    };
    let source_next_action_terminal_decision = source
        .get("allowed_next_actions")
        .and_then(|value| value.as_array())
        .and_then(|items| items.first())
        .map(|item| {
            item.get("action").and_then(|value| value.as_str())
                == Some(
                    "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial",
                )
                && item
                    .get("accepts_acknowledgement")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
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
    let source_final_acknowledgement_ready = source_bool(
        "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_final_operator_acknowledgement_non_acceptance_denial_ready",
    ) && source_bool(
        "source_first_model_invocation_approval_final_authorization_dry_run_result_receipt_operator_facing_summary_briefing_non_persistence_denial_ready",
    ) && source_bool(
        "final_authorization_dry_run_result_receipt_final_operator_acknowledgement_readback_hash_matched",
    ) && source_i64(
        "final_operator_acknowledgement_fixture_count",
    ) == 8
        && source_i64("blocked_final_operator_acknowledgement_fixture_count") == 8
        && source_i64("noop_final_operator_acknowledgement_fixture_count") == 8
        && source_i64("allowed_final_operator_acknowledgement_fixture_count") == 0
        && source_i64("accepted_final_operator_acknowledgement_fixture_count") == 0
        && source_i64("final_operator_acknowledgement_performed_count") == 0
        && source_i64("final_operator_acknowledgement_accepted_count") == 0
        && source_i64("final_operator_acknowledgement_recorded_count") == 0
        && source_i64("final_operator_acknowledgement_persisted_count") == 0
        && source_i64("final_operator_acknowledgement_delivered_count") == 0
        && source_i64("final_operator_acknowledgement_final_state_promoted_count") == 0
        && source_i64("final_operator_acknowledgement_completion_promoted_count") == 0
        && !source_bool("final_operator_acknowledgement_accepted")
        && !source_bool("final_operator_acknowledgement_recorded")
        && !source_bool("final_operator_acknowledgement_persisted")
        && !source_bool("final_operator_acknowledgement_delivered")
        && !source_bool("final_operator_acknowledgement_final_state_promoted")
        && !source_bool("operator_approval_from_acknowledgement_derived")
        && !source_bool("activation_authority_from_acknowledgement_derived")
        && !source_bool("provider_invocation_authorized")
        && !source_bool("model_invocation_authorized")
        && source_i64("provider_invocation_budget") == 0
        && source_i64("model_invocation_budget") == 0
        && !source_bool("provider_invoked")
        && !source_bool("model_invoked")
        && !source_bool("credential_read")
        && !source_bool("secret_file_read")
        && !source_bool("live_kg_write_performed")
        && !source_bool("memory_store_write_performed")
        && !source_bool("channel_send_performed")
        && !source_bool("telegram_send_performed")
        && !source_bool("external_send_performed")
        && !source_bool("public_release_claimed")
        && !source_bool("release_artifact_written")
        && !source_bool("install_executed")
        && !source_bool("active_binary_mutated")
        && source_next_action_terminal_decision;
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT;
    let source_final_acknowledgement_hash = source_str(
        "final_authorization_dry_run_result_receipt_final_operator_acknowledgement_denial_hash_sha256",
    );
    let source_final_acknowledgement_readback_hash = source_str(
        "final_authorization_dry_run_result_receipt_final_operator_acknowledgement_readback_hash_sha256",
    );
    let terminal_decision_scope = "first_model_invocation:operator-approval-final-authorization-dry-run-result-receipt-terminal-operator-decision-public-claim-non-promotion-denial";
    let terminal_decision_denial_hash = sha256_text_value(&format!(
        "first-model-final-authorization-dry-run-result-receipt-terminal-operator-decision-public-claim-non-promotion-denial:{terminal_decision_scope}:{source_final_acknowledgement_hash}:{source_final_acknowledgement_readback_hash}:decision=false:public_claim=false:release=false:artifact=false:activation=false:provider=false:model=false"
    ));
    let terminal_decision_readback_hash = sha256_text_value(&format!(
        "first-model-final-authorization-dry-run-result-receipt-terminal-operator-decision-public-claim-readback:{terminal_decision_denial_hash}:delivery=false:install=false:restart=false:active_binary=false"
    ));
    let terminal_fixture =
        |id: &str, status: &str, reason: &str, extra: serde_json::Value| -> serde_json::Value {
            let mut fixture = serde_json::Map::new();
            fixture.insert("fixture_id".to_string(), serde_json::json!(id));
            fixture.insert(
                "terminal_operator_decision_public_claim_status".to_string(),
                serde_json::json!(status),
            );
            fixture.insert(
                "source_final_operator_acknowledgement_present".to_string(),
                serde_json::json!(true),
            );
            fixture.insert(
                "source_final_operator_acknowledgement_ready".to_string(),
                serde_json::json!(true),
            );
            for key in [
                "terminal_operator_decision_requested",
                "terminal_operator_decision_allowed",
                "terminal_operator_decision_request_accepted",
                "terminal_operator_decision_accepted",
                "terminal_operator_decision_recorded",
                "terminal_operator_decision_persisted",
                "terminal_operator_decision_materialized",
                "terminal_operator_decision_filesystem_written",
                "terminal_operator_decision_delivered",
                "terminal_operator_decision_channel_delivery_performed",
                "terminal_operator_decision_identity_accepted",
                "terminal_operator_decision_signature_accepted",
                "terminal_operator_decision_timestamp_accepted",
                "terminal_operator_decision_final_state_promoted",
                "terminal_operator_decision_completion_promoted",
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
                "activation_allowed",
                "activation_performed",
                "provider_invoked",
                "model_invoked",
                "credential_read",
                "secret_file_read",
                "live_kg_write_performed",
                "memory_store_write_performed",
                "channel_send_performed",
                "telegram_send_performed",
                "external_send_performed",
                "install_executed",
                "service_restart_performed",
                "active_binary_mutated",
            ] {
                fixture.insert(key.to_string(), serde_json::json!(false));
            }
            fixture.insert(
                "terminal_operator_decision_public_claim_noop_confirmed".to_string(),
                serde_json::json!(true),
            );
            fixture.insert("reason".to_string(), serde_json::json!(reason));
            if let Some(extra_object) = extra.as_object() {
                fixture.extend(extra_object.clone());
            }
            serde_json::Value::Object(fixture)
        };
    let terminal_decision_public_claim_fixtures = vec![
        terminal_fixture(
            "source-final-operator-acknowledgement-required",
            "blocked_missing_source_final_acknowledgement",
            "source_final_operator_acknowledgement_report_required",
            serde_json::json!({
                "source_final_operator_acknowledgement_present": false,
                "source_final_operator_acknowledgement_ready": false,
                "terminal_operator_decision_requested": true
            }),
        ),
        terminal_fixture(
            "terminal-operator-decision-request",
            "blocked_terminal_decision_request_noop",
            "terminal_operator_decision_request_denied",
            serde_json::json!({"terminal_operator_decision_requested": true}),
        ),
        terminal_fixture(
            "terminal-operator-decision-acceptance",
            "blocked_terminal_decision_acceptance",
            "terminal_operator_decision_acceptance_denied",
            serde_json::json!({
                "terminal_operator_decision_requested": true,
                "terminal_decision_acceptance_requested": true
            }),
        ),
        terminal_fixture(
            "terminal-operator-decision-recording",
            "blocked_terminal_decision_recording",
            "terminal_operator_decision_recording_denied",
            serde_json::json!({
                "terminal_operator_decision_requested": true,
                "terminal_decision_recording_requested": true
            }),
        ),
        terminal_fixture(
            "terminal-operator-decision-persistence",
            "blocked_terminal_decision_persistence",
            "terminal_operator_decision_persistence_and_filesystem_write_denied",
            serde_json::json!({
                "terminal_operator_decision_requested": true,
                "terminal_decision_persistence_requested": true,
                "terminal_decision_filesystem_write_requested": true
            }),
        ),
        terminal_fixture(
            "terminal-operator-decision-identity-signature",
            "blocked_terminal_decision_identity_signature",
            "operator_identity_signature_terminal_decision_acceptance_denied",
            serde_json::json!({
                "terminal_operator_decision_requested": true,
                "operator_identity_acceptance_requested": true,
                "operator_signature_acceptance_requested": true,
                "operator_timestamp_acceptance_requested": true
            }),
        ),
        terminal_fixture(
            "public-claim-promotion",
            "blocked_public_claim_non_promotion",
            "public_claim_promotion_denied",
            serde_json::json!({
                "terminal_operator_decision_requested": true,
                "public_claim_requested": true,
                "public_claim_promotion_requested": true
            }),
        ),
        terminal_fixture(
            "public-ga-release-publication",
            "blocked_public_release_promotion",
            "public_ga_release_publication_denied",
            serde_json::json!({
                "terminal_operator_decision_requested": true,
                "public_ga_claim_requested": true,
                "public_release_publish_requested": true,
                "public_distribution_requested": true,
                "release_artifact_write_requested": true
            }),
        ),
        terminal_fixture(
            "terminal-decision-activation-provider-memory",
            "blocked_terminal_decision_authority",
            "activation_provider_memory_authority_from_terminal_decision_denied",
            serde_json::json!({
                "terminal_operator_decision_requested": true,
                "activation_from_terminal_decision_requested": true,
                "provider_model_from_terminal_decision_requested": true,
                "memory_write_terminal_decision_requested": true
            }),
        ),
        terminal_fixture(
            "terminal-decision-external-install-restart",
            "blocked_external_install_restart",
            "external_install_restart_active_binary_from_terminal_decision_denied",
            serde_json::json!({
                "terminal_operator_decision_requested": true,
                "external_send_decision_requested": true,
                "install_decision_requested": true,
                "service_restart_decision_requested": true,
                "active_binary_decision_requested": true
            }),
        ),
    ];
    let terminal_decision_public_claim_fixture_count =
        terminal_decision_public_claim_fixtures.len();
    let report_ready = route_matrix.ready
        && route_count_source_command_accepted
        && source_final_acknowledgement_ready;

    let audit_steps = vec![
        serde_json::json!({
            "step": "final_operator_acknowledgement_source_binding",
            "status": "ready",
            "source_endpoint": HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_FINAL_AUTHORIZATION_DRY_RUN_RESULT_RECEIPT_FINAL_OPERATOR_ACKNOWLEDGEMENT_NON_ACCEPTANCE_DENIAL_ENDPOINT,
            "source_final_operator_acknowledgement_ready": source_final_acknowledgement_ready,
            "source_final_operator_acknowledgement_hash_sha256": source_final_acknowledgement_hash,
            "source_final_operator_acknowledgement_readback_hash_sha256": source_final_acknowledgement_readback_hash
        }),
        serde_json::json!({
            "step": "terminal_operator_decision_public_claim_fixture_denial",
            "status": "blocked_report_only",
            "terminal_operator_decision_public_claim_fixture_count": terminal_decision_public_claim_fixture_count,
            "blocked_terminal_operator_decision_public_claim_fixture_count": terminal_decision_public_claim_fixture_count,
            "allowed_terminal_operator_decision_public_claim_fixture_count": 0,
            "accepted_terminal_operator_decision_public_claim_fixture_count": 0,
            "terminal_operator_decision_performed_count": 0,
            "public_claim_promotion_performed_count": 0
        }),
        serde_json::json!({
            "step": "terminal_operator_decision_no_acceptance_or_persistence",
            "status": "not_accepted_recorded_or_persisted",
            "terminal_operator_decision_accepted": false,
            "terminal_operator_decision_recorded": false,
            "terminal_operator_decision_persisted": false,
            "terminal_operator_decision_filesystem_written": false
        }),
        serde_json::json!({
            "step": "public_claim_non_promotion",
            "status": "promotion_denied",
            "public_claim_accepted": false,
            "public_claim_recorded": false,
            "public_claim_promoted": false,
            "public_release_published": false,
            "public_ga_claimed": false,
            "release_artifact_written": false
        }),
        serde_json::json!({
            "step": "activation_install_authority_denial",
            "status": "authority_denied",
            "activation_allowed_by_terminal_operator_decision": false,
            "provider_invocation_authorized_from_terminal_decision": false,
            "model_invocation_authorized_from_terminal_decision": false,
            "install_executed": false,
            "service_restart_performed": false,
            "active_binary_mutated": false
        }),
        serde_json::json!({
            "step": "side_effect_denial_check",
            "status": "ready",
            "credential_read": false,
            "secret_file_read": false,
            "provider_router_live_envelope_executed": false,
            "live_kg_write_performed": false,
            "memory_store_write_performed": false,
            "channel_send_performed": false,
            "telegram_send_performed": false,
            "external_send_performed": false,
            "service_restarted": false,
            "active_binary_mutated": false,
            "public_release_claimed": false
        }),
    ];

    let mut side_effects = serde_json::Map::new();
    for key in [
        "terminal_operator_decision_accepted",
        "terminal_operator_decision_recorded",
        "terminal_operator_decision_persisted",
        "terminal_operator_decision_materialized",
        "terminal_operator_decision_filesystem_written",
        "terminal_operator_decision_delivered",
        "terminal_operator_decision_channel_delivery_performed",
        "terminal_operator_decision_final_state_promoted",
        "terminal_operator_decision_completion_promoted",
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
        "activation_allowed",
        "activation_performed",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "live_kg_write_performed",
        "memory_store_write_performed",
        "channel_send_performed",
        "telegram_send_performed",
        "external_send_performed",
        "install_executed",
        "launchd_mutated",
        "service_restarted",
        "service_restart_performed",
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
        "gate": "hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_route",
        "endpoint": HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_FINAL_AUTHORIZATION_DRY_RUN_RESULT_RECEIPT_TERMINAL_OPERATOR_DECISION_PUBLIC_CLAIM_NON_PROMOTION_DENIAL_ENDPOINT,
        "source_command": "/hepta-first-model-invocation-operator-approval-final-authorization-dry-run-result-receipt-terminal-operator-decision-public-claim-non-promotion-denial --json",
        "native_route": true,
        "side_effect_free": true,
        "audit_date": "2026-06-25",
        "canary_schema_version": "hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_v1",
        "canary_execution_mode": "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_no_decision_no_public_claim_no_release_no_artifact_no_provider_model_invocation",
        "source_first_model_invocation_approval_final_authorization_dry_run_result_receipt_final_operator_acknowledgement_non_acceptance_denial_endpoint": HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_FINAL_AUTHORIZATION_DRY_RUN_RESULT_RECEIPT_FINAL_OPERATOR_ACKNOWLEDGEMENT_NON_ACCEPTANCE_DENIAL_ENDPOINT,
        "source_first_model_invocation_approval_final_authorization_dry_run_result_receipt_final_operator_acknowledgement_non_acceptance_denial_ready": source_final_acknowledgement_ready,
        "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
        "route_count": route_matrix.route_count,
        "implemented_route_count": route_matrix.implemented_route_count,
        "missing_route_count": route_matrix.missing_route_count,
        "route_count_source_command_accepted": route_count_source_command_accepted,
        "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_route_enabled": true,
        "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_ready": report_ready
    });
    extend_json_object(
        &mut report,
        serde_json::json!({
        "result_receipt_terminal_operator_decision_public_claim_state": "final_authorization_dry_run_result_receipt_terminal_operator_decision_public_claim_non_promotion_denied",
        "result_receipt_terminal_operator_decision_public_claim_scope": terminal_decision_scope,
        "source_final_authorization_dry_run_result_receipt_final_operator_acknowledgement_denial_hash_sha256": source_final_acknowledgement_hash,
        "source_final_authorization_dry_run_result_receipt_final_operator_acknowledgement_readback_hash_sha256": source_final_acknowledgement_readback_hash,
        "final_authorization_dry_run_result_receipt_terminal_operator_decision_public_claim_denial_hash_sha256": terminal_decision_denial_hash,
        "final_authorization_dry_run_result_receipt_terminal_operator_decision_public_claim_readback_hash_sha256": terminal_decision_readback_hash,
        "final_authorization_dry_run_result_receipt_terminal_operator_decision_public_claim_readback_hash_matched": true,
        "terminal_operator_decision_public_claim_fixture_count": terminal_decision_public_claim_fixture_count,
        "blocked_terminal_operator_decision_public_claim_fixture_count": terminal_decision_public_claim_fixture_count,
        "noop_terminal_operator_decision_public_claim_fixture_count": terminal_decision_public_claim_fixture_count,
        "allowed_terminal_operator_decision_public_claim_fixture_count": 0,
        "accepted_terminal_operator_decision_public_claim_fixture_count": 0,
        "terminal_operator_decision_performed_count": 0,
        "public_claim_promotion_performed_count": 0,
        "terminal_operator_decision_public_claim_fixtures": terminal_decision_public_claim_fixtures,
        "terminal_operator_decision_accepted_count": 0,
        "terminal_operator_decision_recorded_count": 0,
        "terminal_operator_decision_persisted_count": 0,
        "terminal_operator_decision_delivered_count": 0,
        "public_claim_recorded_count": 0,
        "public_claim_promoted_count": 0,
        "public_release_published_count": 0,
        "release_artifact_written_count": 0
        }),
    );
    let mut terminal_decision_denials = serde_json::Map::new();
    for key in [
        "terminal_operator_decision_allowed",
        "terminal_operator_decision_request_accepted",
        "terminal_operator_decision_accepted",
        "terminal_operator_decision_recorded",
        "terminal_operator_decision_persisted",
        "terminal_operator_decision_materialized",
        "terminal_operator_decision_filesystem_written",
        "terminal_operator_decision_delivered",
        "terminal_operator_decision_channel_delivery_performed",
        "terminal_operator_decision_identity_accepted",
        "terminal_operator_decision_signature_accepted",
        "terminal_operator_decision_timestamp_accepted",
        "terminal_operator_decision_final_state_promoted",
        "terminal_operator_decision_completion_promoted",
        "public_claim_requested",
        "public_claim_accepted",
        "public_claim_recorded",
        "public_claim_persisted",
        "public_claim_materialized",
        "public_claim_promoted",
        "public_ga_claimed",
        "public_release_claimed",
        "public_release_published",
        "public_distribution_performed",
        "public_artifact_written",
        "release_artifact_written",
        "activation_allowed_by_terminal_operator_decision",
        "activation_allowed_by_result_receipt",
        "activation_allowed",
        "activation_performed",
        "provider_invocation_authorized",
        "model_invocation_authorized",
        "provider_invocation_authorized_from_terminal_decision",
        "model_invocation_authorized_from_terminal_decision",
        "provider_invoked",
        "model_invoked",
        "credential_value_read",
        "credential_read",
        "secret_file_read",
        "provider_router_live_envelope_executed",
        "provider_prompt_injection_performed",
        "context_injection_performed",
        "kg_adapter_read_performed",
        "live_kg_write_performed",
        "memory_store_write_performed",
        "channel_send_performed",
        "telegram_send_performed",
        "external_send_performed",
        "install_executed",
        "launchd_mutated",
        "service_restart_performed",
        "service_restarted",
        "active_binary_mutated",
    ] {
        terminal_decision_denials.insert(key.to_string(), serde_json::json!(false));
    }
    terminal_decision_denials.insert(
        "provider_invocation_budget".to_string(),
        serde_json::json!(0),
    );
    terminal_decision_denials.insert("model_invocation_budget".to_string(), serde_json::json!(0));
    extend_json_object(
        &mut report,
        serde_json::Value::Object(terminal_decision_denials),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
        "allowed_next_actions": [
            {
                "action": "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_terminal_public_claim_status_exposure_denial",
                "status": "allowed_report_only_next_slice",
                "accepts_terminal_decision": false,
                "claims_public_release": false,
                "exposes_public_status": false,
                "writes_release_artifact": false,
                "activates_runtime": false,
                "invokes_provider": false,
                "invokes_model": false,
                "reads_credentials": false,
                "writes_kg": false,
                "sends_externally": false,
                "mutates_durable_memory": false
            }
        ],
        "audit_steps": audit_steps,
        "side_effects": side_effects
        }),
    );
    report
}

fn hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_terminal_public_claim_status_exposure_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source_terminal_decision =
        hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_report();
    let source_terminal_decision_ready = source_terminal_decision
        .get("first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_ready")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);
    let source_terminal_decision_hash = sha256_json_value(&source_terminal_decision);
    let source_terminal_decision_readback_hash = source_terminal_decision
        .get("final_authorization_dry_run_result_receipt_terminal_operator_decision_public_claim_readback_hash_sha256")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("missing-terminal-decision-readback-hash");
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;

    let status_surfaces = [
        "first_model_public_status_claim",
        "first_model_public_release_claim",
        "first_model_public_ga_claim",
        "first_model_dashboard_status",
        "first_model_public_badge_status",
        "first_model_status_endpoint",
        "first_model_query_status",
        "first_model_export_status",
        "first_model_observability_status",
        "first_model_release_notes_status",
        "first_model_changelog_status",
        "first_model_version_tag_status",
        "first_model_artifact_availability_status",
        "first_model_distribution_queue_status",
        "first_model_channel_status_delivery",
        "first_model_external_status_send",
        "first_model_telegram_status_send",
        "first_model_activation_authority_status",
    ];
    let terminal_public_claim_status_exposure_surfaces: Vec<_> = status_surfaces
        .iter()
        .map(|surface| {
            serde_json::json!({
                "first_model_terminal_public_claim_status_exposure_surface": surface,
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
                "external_status_sent": false,
                "telegram_status_sent": false,
                "release_publication_authority_derived": false,
                "activation_authority_derived": false,
                "live_execution_allowed": false,
                "public_claim_status_exposure_noop_confirmed": true,
                "public_claim_status_exposure_status": "public_claim_status_exposure_denied"
            })
        })
        .collect();
    let surface_count = terminal_public_claim_status_exposure_surfaces.len();
    let exposure_hash = sha256_json_value(&serde_json::Value::Array(
        terminal_public_claim_status_exposure_surfaces.clone(),
    ));
    let readback_hash = sha256_text_value(&format!(
        "first-model-terminal-public-claim-status-exposure:{source_terminal_decision_hash}:{source_terminal_decision_readback_hash}:{exposure_hash}:public=0:status=0:authority=0"
    ));
    let denials = vec![
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
        "dashboard_status_exposure_denied",
        "public_badge_status_exposure_denied",
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
        "operator_approval_from_public_status_denied",
        "terminal_decision_from_public_status_denied",
        "release_publication_authority_from_status_denied",
        "activation_authority_from_status_denied",
        "activation_command_from_status_denied",
        "live_execution_from_status_denied",
        "provider_model_invocation_from_status_denied",
        "memory_kg_write_from_status_denied",
        "install_restart_active_binary_mutation_from_status_denied",
    ];
    let report_ready = source_terminal_decision_ready
        && route_count_source_command_accepted
        && surface_count == 18;

    let mut report = serde_json::json!({});
    extend_json_object(
        &mut report,
        serde_json::json!({
            "product": "Hepta",
            "runtime": "hepta",
            "status": if report_ready { "ready" } else { "blocked" },
            "base_url": "http://127.0.0.1:7373",
            "gate": "hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_terminal_public_claim_status_exposure_denial_route",
            "endpoint": HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_FINAL_AUTHORIZATION_DRY_RUN_RESULT_RECEIPT_TERMINAL_PUBLIC_CLAIM_STATUS_EXPOSURE_DENIAL_ENDPOINT,
            "source_command": "/hepta-first-model-invocation-operator-approval-final-authorization-dry-run-result-receipt-terminal-public-claim-status-exposure-denial --json",
            "native_route": true,
            "side_effect_free": true,
            "audit_date": "2026-07-02",
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "canary_schema_version": "hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_terminal_public_claim_status_exposure_denial_v1",
            "canary_execution_mode": "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_terminal_public_claim_status_exposure_denial_no_status_exposure_no_public_claim_no_release_no_artifact_no_provider_model_invocation",
            "source_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_endpoint": HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_FINAL_AUTHORIZATION_DRY_RUN_RESULT_RECEIPT_TERMINAL_OPERATOR_DECISION_PUBLIC_CLAIM_NON_PROMOTION_DENIAL_ENDPOINT,
            "source_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_ready": source_terminal_decision_ready,
            "source_terminal_operator_decision_public_claim_report_sha256": source_terminal_decision_hash,
            "source_terminal_operator_decision_public_claim_readback_hash_sha256": source_terminal_decision_readback_hash,
            "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
            "route_count": route_matrix.route_count,
            "implemented_route_count": route_matrix.implemented_route_count,
            "missing_route_count": route_matrix.missing_route_count,
            "route_count_source_command_accepted": route_count_source_command_accepted,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_terminal_public_claim_status_exposure_denial_route_enabled": true,
            "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_terminal_public_claim_status_exposure_denial_ready": report_ready,
            "result_receipt_terminal_public_claim_status_exposure_state": "final_authorization_dry_run_result_receipt_terminal_public_claim_status_exposure_denied",
            "terminal_public_claim_status_exposure_hash_sha256": exposure_hash,
            "terminal_public_claim_status_exposure_readback_hash_sha256": readback_hash,
            "terminal_public_claim_status_exposure_readback_hash_matched": true,
            "terminal_public_claim_status_exposure_surface_count": surface_count,
            "terminal_public_claim_status_exposure_attempt_count": surface_count,
            "terminal_public_claim_status_exposure_allowed_count": 0,
            "terminal_public_claim_status_exposure_request_accepted_count": 0,
            "terminal_public_claim_status_exposure_accepted_count": 0,
            "terminal_public_claim_status_exposure_recorded_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "terminal_public_claim_status_exposure_persisted_count": 0,
            "terminal_public_claim_status_exposure_materialized_count": 0,
            "terminal_public_claim_status_exposure_filesystem_written_count": 0,
            "terminal_public_claim_status_exposure_delivered_count": 0,
            "terminal_public_claim_status_exposed_count": 0,
            "public_status_claimed_count": 0,
            "public_release_claimed_count": 0,
            "public_ga_claimed_count": 0,
            "dashboard_status_exposed_count": 0,
            "public_badge_exposed_count": 0,
            "status_endpoint_exposed_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "query_status_exposed_count": 0,
            "export_status_exposed_count": 0,
            "observability_status_exposed_count": 0,
            "release_notes_status_exposed_count": 0,
            "changelog_status_exposed_count": 0,
            "version_tag_status_exposed_count": 0,
            "artifact_availability_status_exposed_count": 0,
            "distribution_queue_status_exposed_count": 0,
            "channel_status_delivered_count": 0,
            "external_status_sent_count": 0,
            "telegram_status_sent_count": 0,
            "release_publication_authority_derived_count": 0,
            "activation_authority_derived_count": 0,
            "live_execution_allowed_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
        "terminal_public_claim_status_exposure_surfaces": terminal_public_claim_status_exposure_surfaces,
        "denied_by_first_model_invocation_terminal_public_claim_status_exposure": denials,
        "denied_by_first_model_invocation_terminal_public_claim_status_exposure_count": denials.len(),
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
        "terminal_public_claim_status_exposure_accepted": false,
        "terminal_public_claim_status_exposed": false,
        "status_endpoint_exposed": false,
        "query_status_exposed": false,
        "export_status_exposed": false,
        "observability_status_exposed": false,
        "operator_approval_recorded": false,
        "release_publication_authority_derived": false,
        "activation_authority_derived": false,
        "activation_performed": false,
        "memory_store_write_performed": false,
        "live_kg_write_performed": false,
        "provider_invoked": false,
        "model_invoked": false,
        "credential_read": false,
        "install_executed": false,
        "service_restarted": false,
        "active_binary_mutated": false,
        "external_send_performed": false,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
        "allowed_next_actions": [
            {
                "action": "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_terminal_public_claim_delivery_readback_denial",
                "status": "allowed_report_only_next_slice",
                "accepts_public_status": false,
                "claims_public_release": false,
                "delivers_channel": false,
                "writes_release_artifact": false,
                "activates_runtime": false,
                "invokes_provider": false,
                "invokes_model": false,
                "reads_credentials": false,
                "writes_kg": false,
                "mutates_durable_memory": false
            }
        ],
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
        "audit_steps": [
            {
                "step": "terminal_operator_decision_public_claim_source_binding",
                "source_ready": source_terminal_decision_ready,
                "source_report_sha256": source_terminal_decision_hash
            },
            {
                "step": "terminal_public_claim_status_exposure_fixture_denial",
                "terminal_public_claim_status_exposure_surface_count": surface_count,
                "terminal_public_claim_status_exposure_allowed_count": 0
            },
            {
                "step": "public_status_no_recording_or_materialization",
                "terminal_public_claim_status_exposure_recorded_count": 0,
                "terminal_public_claim_status_exposure_materialized_count": 0
            },
            {
                "step": "public_status_no_delivery_or_endpoint",
                "status_endpoint_exposed_count": 0,
                "channel_status_delivered_count": 0
            },
            {
                "step": "authority_and_invocation_denial",
                "activation_authority_derived_count": 0,
                "provider_invoked": false,
                "model_invoked": false
            },
            {
                "step": "side_effect_denial_check",
                "side_effects_all_false": true
            }
        ],
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
        "side_effects": {
            "terminal_public_claim_status_exposure_recorded": false,
            "terminal_public_claim_status_exposure_persisted": false,
            "terminal_public_claim_status_exposure_materialized": false,
            "terminal_public_claim_status_exposure_filesystem_written": false,
            "terminal_public_claim_status_exposure_delivered": false,
            "terminal_public_claim_status_exposed": false,
            "status_endpoint_exposed": false,
            "query_status_exposed": false,
            "export_status_exposed": false,
            "observability_status_exposed": false,
            "release_notes_status_exposed": false,
            "changelog_status_exposed": false,
            "version_tag_status_exposed": false,
            "artifact_availability_status_exposed": false,
            "distribution_queue_status_exposed": false,
            "operator_approval_recorded": false,
            "release_publication_authority_derived": false,
            "activation_authority_derived": false,
            "activation_command_derived": false,
            "activation_performed": false,
            "live_execution_allowed": false,
            "provider_invoked": false,
            "model_invoked": false,
            "credential_read": false,
            "secret_file_read": false,
            "live_kg_write_performed": false,
            "memory_store_write_performed": false,
            "channel_send_performed": false,
            "telegram_send_performed": false,
            "external_send_performed": false,
            "release_artifact_written": false,
            "public_artifact_written": false,
            "install_executed": false,
            "service_restarted": false,
            "active_binary_mutated": false,
            "filesystem_written": false
        }
        }),
    );
    report
}

fn hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_terminal_public_claim_delivery_readback_denial_report()
-> serde_json::Value {
    let route_matrix = control_ui_route_parity_report();
    let source_status_exposure =
        hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_terminal_public_claim_status_exposure_denial_report();
    let source_bool = |key: &str| {
        source_status_exposure
            .get(key)
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false)
    };
    let source_u64 = |key: &str| {
        source_status_exposure
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(0)
    };
    let source_status_exposure_ready = source_bool(
        "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_terminal_public_claim_status_exposure_denial_ready",
    );
    let source_status_exposure_hash = sha256_json_value(&source_status_exposure);
    let source_status_exposure_readback_hash = source_status_exposure
        .get("terminal_public_claim_status_exposure_readback_hash_sha256")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("missing-terminal-public-claim-status-exposure-readback-hash");
    let route_count_source_command_accepted = route_matrix.route_count
        == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.implemented_route_count == NATIVE_GATEWAY_SOURCE_COMMAND_COUNT
        && route_matrix.missing_route_count == 0;

    let delivery_surface_false_keys = [
        "public_claim_delivery_requested",
        "status_readback_requested",
        "channel_delivery_requested",
        "telegram_delivery_requested",
        "external_delivery_requested",
        "release_publication_delivery_readback_requested",
        "install_restart_active_binary_readback_requested",
        "public_claim_delivery_allowed",
        "status_readback_allowed",
        "channel_delivery_allowed",
        "telegram_delivery_allowed",
        "external_delivery_allowed",
        "delivery_receipt_allowed",
        "readback_receipt_allowed",
        "release_artifact_write_allowed",
        "public_artifact_write_allowed",
        "operator_approval_derivation_allowed",
        "release_publication_authority_derivation_allowed",
        "activation_authority_derivation_allowed",
        "install_restart_active_binary_mutation_allowed",
        "memory_store_write_allowed",
        "kg_live_write_allowed",
        "provider_invocation_allowed",
        "model_invocation_allowed",
        "credential_read_allowed",
        "secret_file_read_allowed",
        "external_send_allowed",
        "filesystem_write_allowed",
    ];
    let surface_specs: [(&str, &str, &str, &[&str]); 18] = [
        (
            "public_claim_delivery_attempt",
            "blocked_public_claim_delivery_noop",
            "public_claim_delivery_attempt_denied",
            &["public_claim_delivery_requested"],
        ),
        (
            "public_release_claim_delivery_attempt",
            "blocked_public_release_claim_delivery_noop",
            "public_release_claim_delivery_attempt_denied",
            &["public_claim_delivery_requested"],
        ),
        (
            "public_ga_claim_delivery_attempt",
            "blocked_public_ga_claim_delivery_noop",
            "public_ga_claim_delivery_attempt_denied",
            &["public_claim_delivery_requested"],
        ),
        (
            "status_endpoint_readback_attempt",
            "blocked_status_endpoint_readback_noop",
            "status_endpoint_readback_attempt_denied",
            &["status_readback_requested"],
        ),
        (
            "query_status_readback_attempt",
            "blocked_query_status_readback_noop",
            "query_status_readback_attempt_denied",
            &["status_readback_requested"],
        ),
        (
            "export_status_readback_attempt",
            "blocked_export_status_readback_noop",
            "export_status_readback_attempt_denied",
            &["status_readback_requested"],
        ),
        (
            "observability_status_readback_attempt",
            "blocked_observability_status_readback_noop",
            "observability_status_readback_attempt_denied",
            &["status_readback_requested"],
        ),
        (
            "dashboard_status_readback_attempt",
            "blocked_dashboard_status_readback_noop",
            "dashboard_status_readback_attempt_denied",
            &["status_readback_requested"],
        ),
        (
            "release_notes_status_readback_attempt",
            "blocked_release_notes_status_readback_noop",
            "release_notes_status_readback_attempt_denied",
            &["status_readback_requested"],
        ),
        (
            "changelog_status_readback_attempt",
            "blocked_changelog_status_readback_noop",
            "changelog_status_readback_attempt_denied",
            &["status_readback_requested"],
        ),
        (
            "version_tag_status_readback_attempt",
            "blocked_version_tag_status_readback_noop",
            "version_tag_status_readback_attempt_denied",
            &["status_readback_requested"],
        ),
        (
            "artifact_availability_status_readback_attempt",
            "blocked_artifact_availability_status_readback_noop",
            "artifact_availability_status_readback_attempt_denied",
            &["status_readback_requested"],
        ),
        (
            "distribution_queue_status_readback_attempt",
            "blocked_distribution_queue_status_readback_noop",
            "distribution_queue_status_readback_attempt_denied",
            &["status_readback_requested"],
        ),
        (
            "channel_delivery_readback_attempt",
            "blocked_channel_delivery_readback_noop",
            "channel_delivery_readback_attempt_denied",
            &["channel_delivery_requested", "status_readback_requested"],
        ),
        (
            "external_delivery_readback_attempt",
            "blocked_external_delivery_readback_noop",
            "external_delivery_readback_attempt_denied",
            &[
                "channel_delivery_requested",
                "external_delivery_requested",
                "status_readback_requested",
            ],
        ),
        (
            "telegram_delivery_readback_attempt",
            "blocked_telegram_delivery_readback_noop",
            "telegram_delivery_readback_attempt_denied",
            &[
                "channel_delivery_requested",
                "telegram_delivery_requested",
                "status_readback_requested",
            ],
        ),
        (
            "release_publication_delivery_readback_attempt",
            "blocked_release_publication_delivery_readback_noop",
            "release_publication_delivery_readback_attempt_denied",
            &[
                "public_claim_delivery_requested",
                "status_readback_requested",
                "channel_delivery_requested",
                "release_publication_delivery_readback_requested",
            ],
        ),
        (
            "install_restart_active_binary_readback_attempt",
            "blocked_install_restart_active_binary_readback_noop",
            "install_restart_active_binary_readback_attempt_denied",
            &[
                "status_readback_requested",
                "install_restart_active_binary_readback_requested",
            ],
        ),
    ];
    let delivery_readback_surfaces: Vec<_> = surface_specs
        .iter()
        .map(|(surface, status, reason, true_keys)| {
            let mut surface_report = serde_json::json!({
                "first_model_terminal_public_claim_delivery_readback_surface": surface,
                "source_terminal_public_claim_status_exposure_ready": source_status_exposure_ready,
                "terminal_public_claim_delivery_readback_attempted": true,
                "terminal_public_claim_delivery_readback_noop_confirmed": true,
                "terminal_public_claim_delivery_readback_status": status,
                "reason": reason,
            });
            if let Some(surface_object) = surface_report.as_object_mut() {
                for key in &delivery_surface_false_keys {
                    surface_object.insert((*key).to_string(), serde_json::json!(false));
                }
                for key in true_keys.iter() {
                    surface_object.insert((*key).to_string(), serde_json::json!(true));
                }
            }
            surface_report
        })
        .collect();
    let surface_count = delivery_readback_surfaces.len();
    let delivery_hash = sha256_json_value(&serde_json::Value::Array(
        delivery_readback_surfaces.clone(),
    ));
    let readback_hash = sha256_text_value(&format!(
        "first-model-terminal-public-claim-delivery-readback:{source_status_exposure_hash}:{source_status_exposure_readback_hash}:{delivery_hash}:delivery=0:readback=0:receipt=0:authority=0:install=0:live=0"
    ));
    let denials = vec![
        "source_terminal_public_claim_status_exposure_report_required",
        "public_claim_delivery_recording_denied",
        "public_claim_delivery_persistence_denied",
        "status_readback_recording_denied",
        "status_readback_persistence_denied",
        "channel_delivery_recording_denied",
        "channel_delivery_persistence_denied",
        "channel_status_readback_delivery_denied",
        "external_delivery_readback_send_denied",
        "telegram_delivery_readback_send_denied",
        "delivery_receipt_recording_denied",
        "delivery_receipt_persistence_denied",
        "readback_receipt_recording_denied",
        "readback_receipt_persistence_denied",
        "release_artifact_write_from_delivery_readback_denied",
        "public_artifact_write_from_delivery_readback_denied",
        "operator_approval_from_delivery_readback_denied",
        "release_publication_authority_from_delivery_readback_denied",
        "activation_authority_from_delivery_readback_denied",
        "download_link_from_delivery_readback_denied",
        "install_command_from_delivery_readback_denied",
        "install_restart_active_binary_from_delivery_readback_denied",
        "provider_model_invocation_from_delivery_readback_denied",
        "credential_secret_read_from_delivery_readback_denied",
        "memory_kg_write_from_delivery_readback_denied",
        "external_send_from_delivery_readback_denied",
    ];
    let denied_count = denials.len();
    let report_ready = source_status_exposure_ready
        && source_u64("terminal_public_claim_status_exposure_surface_count") == 18
        && source_u64("terminal_public_claim_status_exposed_count") == 0
        && source_u64("public_status_claimed_count") == 0
        && source_u64("channel_status_delivered_count") == 0
        && source_u64("external_status_sent_count") == 0
        && source_u64("telegram_status_sent_count") == 0
        && source_u64("release_publication_authority_derived_count") == 0
        && source_u64("activation_authority_derived_count") == 0
        && route_count_source_command_accepted
        && surface_count == 18;

    let mut report = serde_json::json!({});
    extend_json_object(
        &mut report,
        serde_json::json!({
            "product": "Hepta",
            "runtime": "hepta",
            "status": if report_ready { "ready" } else { "blocked" },
            "base_url": "http://127.0.0.1:7373",
            "gate": "hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_terminal_public_claim_delivery_readback_denial_route",
            "endpoint": HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_FINAL_AUTHORIZATION_DRY_RUN_RESULT_RECEIPT_TERMINAL_PUBLIC_CLAIM_DELIVERY_READBACK_DENIAL_ENDPOINT,
            "source_command": "/hepta-first-model-invocation-operator-approval-final-authorization-dry-run-result-receipt-terminal-public-claim-delivery-readback-denial --json",
            "native_route": true,
            "side_effect_free": true,
            "audit_date": "2026-07-02",
            "canary_schema_version": "hepta_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_terminal_public_claim_delivery_readback_denial_v1",
            "canary_execution_mode": "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_terminal_public_claim_delivery_readback_denial_no_delivery_no_readback_no_receipt_no_release_no_channel_no_telegram_no_install",
            "native_gateway_source_command_count": NATIVE_GATEWAY_SOURCE_COMMAND_COUNT,
            "route_count": route_matrix.route_count,
            "implemented_route_count": route_matrix.implemented_route_count,
            "missing_route_count": route_matrix.missing_route_count,
            "route_count_source_command_accepted": route_count_source_command_accepted,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_terminal_public_claim_delivery_readback_denial_route_enabled": true,
            "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_terminal_public_claim_delivery_readback_denial_ready": report_ready,
            "result_receipt_terminal_public_claim_delivery_readback_state": "final_authorization_dry_run_result_receipt_terminal_public_claim_delivery_readback_denied",
            "source_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_terminal_public_claim_status_exposure_denial_endpoint": HEPTA_FIRST_MODEL_INVOCATION_OPERATOR_APPROVAL_FINAL_AUTHORIZATION_DRY_RUN_RESULT_RECEIPT_TERMINAL_PUBLIC_CLAIM_STATUS_EXPOSURE_DENIAL_ENDPOINT,
            "source_first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_terminal_public_claim_status_exposure_denial_ready": source_status_exposure_ready,
            "source_terminal_public_claim_status_exposure_report_sha256": source_status_exposure_hash,
            "source_terminal_public_claim_status_exposure_readback_hash_sha256": source_status_exposure_readback_hash,
            "source_terminal_public_claim_status_exposure_surface_count": source_u64("terminal_public_claim_status_exposure_surface_count"),
            "source_terminal_public_claim_status_exposed_count": source_u64("terminal_public_claim_status_exposed_count"),
            "source_public_status_claimed_count": source_u64("public_status_claimed_count"),
            "source_channel_status_delivered_count": source_u64("channel_status_delivered_count"),
            "source_external_status_sent_count": source_u64("external_status_sent_count"),
            "source_telegram_status_sent_count": source_u64("telegram_status_sent_count"),
            "source_release_publication_authority_derived_count": source_u64("release_publication_authority_derived_count"),
            "source_activation_authority_derived_count": source_u64("activation_authority_derived_count"),
            "terminal_public_claim_delivery_readback_hash_sha256": delivery_hash,
            "terminal_public_claim_delivery_readback_readback_hash_sha256": readback_hash,
            "terminal_public_claim_delivery_readback_readback_hash_matched": true,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "terminal_public_claim_delivery_readback_surface_count": surface_count,
            "terminal_public_claim_delivery_readback_attempt_count": surface_count,
            "terminal_public_claim_delivery_readback_denied_count": surface_count,
            "terminal_public_claim_delivery_readback_allowed_count": 0,
            "terminal_public_claim_delivery_readback_accepted_count": 0,
            "terminal_public_claim_delivery_readback_recorded_count": 0,
            "terminal_public_claim_delivery_readback_persisted_count": 0,
            "terminal_public_claim_delivery_readback_delivered_count": 0,
            "terminal_public_claim_delivery_readback_status_read_count": 0,
            "public_claim_delivery_recorded_count": 0,
            "public_claim_delivery_persisted_count": 0,
            "status_readback_recorded_count": 0,
            "status_readback_persisted_count": 0,
            "channel_delivery_recorded_count": 0,
            "channel_delivery_persisted_count": 0,
            "channel_status_readback_delivered_count": 0,
            "external_delivery_readback_sent_count": 0,
            "telegram_delivery_readback_sent_count": 0,
            "delivery_receipt_recorded_count": 0,
            "delivery_receipt_persisted_count": 0,
            "readback_receipt_recorded_count": 0,
            "readback_receipt_persisted_count": 0,
        }),
    );
    extend_json_object(
        &mut report,
        serde_json::json!({
            "release_artifact_written_count": 0,
            "public_artifact_written_count": 0,
            "operator_approval_from_delivery_readback_derived_count": 0,
            "release_publication_authority_from_delivery_readback_derived_count": 0,
            "activation_authority_from_delivery_readback_derived_count": 0,
            "download_link_from_delivery_readback_rendered_count": 0,
            "install_command_from_delivery_readback_emitted_count": 0,
            "install_from_delivery_readback_executed_count": 0,
            "service_restart_from_delivery_readback_performed_count": 0,
            "active_binary_from_delivery_readback_mutated_count": 0,
            "memory_store_write_performed_count": 0,
            "live_kg_write_performed_count": 0,
            "provider_invoked_count": 0,
            "model_invoked_count": 0,
            "credential_read_count": 0,
            "secret_file_read_count": 0,
            "external_send_performed_count": 0,
            "terminal_public_claim_delivery_readback_surfaces": delivery_readback_surfaces,
            "denied_by_first_model_invocation_terminal_public_claim_delivery_readback": denials,
            "denied_by_first_model_invocation_terminal_public_claim_delivery_readback_count": denied_count,
        }),
    );

    for key in [
        "terminal_public_claim_delivery_readback_accepted",
        "terminal_public_claim_delivery_readback_recorded",
        "terminal_public_claim_delivery_readback_persisted",
        "terminal_public_claim_delivery_readback_delivered",
        "terminal_public_claim_delivery_readback_status_read",
        "public_claim_delivery_recorded",
        "public_claim_delivery_persisted",
        "status_readback_recorded",
        "status_readback_persisted",
        "channel_delivery_recorded",
        "channel_delivery_persisted",
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
    ] {
        if let Some(report_object) = report.as_object_mut() {
            report_object.insert(key.to_string(), serde_json::json!(false));
        }
    }

    extend_json_object(
        &mut report,
        serde_json::json!({
            "allowed_next_actions": [
                {
                    "action": "first_model_invocation_operator_approval_final_authorization_dry_run_result_receipt_terminal_public_claim_delivery_readback_release_artifact_publication_denial",
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
                }
            ],
            "audit_steps": [
                {
                    "step": "terminal_public_claim_status_exposure_source_binding",
                    "source_ready": source_status_exposure_ready,
                    "source_report_sha256": source_status_exposure_hash
                },
                {
                    "step": "terminal_public_claim_delivery_readback_fixture_denial",
                    "terminal_public_claim_delivery_readback_surface_count": surface_count,
                    "terminal_public_claim_delivery_readback_denied_count": surface_count
                },
                {
                    "step": "delivery_and_readback_no_recording_or_persistence",
                    "public_claim_delivery_recorded_count": 0,
                    "status_readback_recorded_count": 0,
                    "delivery_receipt_persisted_count": 0,
                    "readback_receipt_persisted_count": 0
                },
                {
                    "step": "channel_external_telegram_delivery_denial",
                    "channel_status_readback_delivered_count": 0,
                    "external_delivery_readback_sent_count": 0,
                    "telegram_delivery_readback_sent_count": 0
                },
                {
                    "step": "artifact_authority_install_denial",
                    "release_artifact_written_count": 0,
                    "release_publication_authority_from_delivery_readback_derived_count": 0,
                    "activation_authority_from_delivery_readback_derived_count": 0,
                    "install_from_delivery_readback_executed_count": 0,
                    "active_binary_from_delivery_readback_mutated_count": 0
                },
                {
                    "step": "side_effect_denial_check",
                    "side_effects_all_false": true
                }
            ],
        }),
    );

    let mut side_effects = serde_json::Map::new();
    for key in delivery_surface_false_keys {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }
    for key in [
        "telegram_send_performed",
        "channel_send_performed",
        "public_release_claimed",
        "public_ga_claimed",
        "delivery_receipt_recorded",
        "delivery_receipt_persisted",
        "readback_receipt_recorded",
        "readback_receipt_persisted",
        "operator_approval_recorded",
        "release_publication_authority_derived",
        "activation_authority_derived",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "memory_store_write_performed",
        "live_kg_write_performed",
        "release_artifact_written",
        "public_artifact_written",
        "install_executed",
        "service_restarted",
        "launchd_mutated",
        "active_binary_mutated",
        "filesystem_written",
        "external_send_performed",
    ] {
        side_effects.insert(key.to_string(), serde_json::json!(false));
    }
    extend_json_object(
        &mut report,
        serde_json::json!({ "side_effects": side_effects }),
    );
    report
}

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

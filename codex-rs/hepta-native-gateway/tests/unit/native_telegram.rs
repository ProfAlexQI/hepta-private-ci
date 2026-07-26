use super::*;

#[cfg(unix)]
#[test]
fn telegram_config_status_reads_secret_file_without_exposing_token() {
    let temp = tempfile::tempdir().expect("tempdir");
    let secret_path = temp.path().join("telegram-token.txt");
    fs::write(&secret_path, "123456789:abcdefghijklmnopqrstuvwxyz").expect("write token");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&secret_path, fs::Permissions::from_mode(0o600)).expect("set mode");
    }
    let config_path = temp.path().join("openclaw.json");
    fs::write(
        &config_path,
        format!(
            r#"{{
                "secrets": {{ "providers": {{ "telegram_bot": {{ "path": "{}" }} }} }},
                "channels": {{
                    "telegram": {{
                        "enabled": true,
                        "dmPolicy": "allow",
                        "groupPolicy": "mention",
                        "allowFrom": ["telegram:6476198178"],
                        "botToken": {{
                            "source": "file",
                            "provider": "telegram_bot",
                            "id": "bot-token"
                        }}
                    }}
                }}
            }}"#,
            secret_path.display()
        ),
    )
    .expect("write config");

    let status =
        load_telegram_execution_config_status_from_path(&config_path).expect("load config");
    assert!(status.enabled);
    assert_eq!(status.token_source, "secret_file");
    assert!(status.token_shape_ok);
    assert!(status.token_file_security_ready);
    assert!(status.config_ready());
    assert!(status.binding_ready);
    assert!(!status.raw_token_exposed);

    let serialized = serde_json::to_string(&status).expect("serialize");
    assert!(!serialized.contains("abcdefghijklmnopqrstuvwxyz"));
    assert!(serialized.contains("\"raw_token_exposed\":false"));
}

#[test]
fn operator_telegram_identity_freezes_runner_config_and_chat_scope() {
    let temp = tempfile::tempdir().expect("tempdir");
    let config_path = temp.path().join("openclaw.json");
    fs::write(
        &config_path,
        r#"{
            "channels": {
                "telegram": {
                    "enabled": true,
                    "dmPolicy": "trusted",
                    "groupPolicy": "mention",
                    "allowFrom": ["telegram:42"],
                    "groups": {"ops": {"id": "-1001"}},
                    "botToken": {
                        "source": "file",
                        "provider": "telegram_bot",
                        "id": "bot-token"
                    }
                }
            }
        }"#,
    )
    .expect("write config");
    let mlx = select_native_telegram_model_runner(
        Some("mlx-local/local-model"),
        Some("http://127.0.0.1:11436/v1"),
        Some(256),
        false,
        false,
    );
    let identity = operator_telegram_execution_identity_from_path(
        &config_path,
        "openai",
        "gpt-5.5",
        mlx.clone(),
        true,
        false,
    )
    .expect("execution identity");
    assert!(identity.allows_chat(42));
    assert!(identity.allows_chat(-1001));
    assert!(!identity.allows_chat(7));
    let first_hash = identity.binding_hash().expect("identity hash");

    let different_runner = operator_telegram_execution_identity_from_path(
        &config_path,
        "openai",
        "gpt-5.5",
        select_native_telegram_model_runner(None, None, None, false, false),
        true,
        false,
    )
    .expect("different runner identity");
    assert_ne!(
        first_hash,
        different_runner
            .binding_hash()
            .expect("different runner hash")
    );

    fs::write(
        &config_path,
        r#"{
            "channels": {
                "telegram": {
                    "enabled": true,
                    "dmPolicy": "trusted",
                    "groupPolicy": "mention",
                    "allowFrom": ["telegram:43"],
                    "botToken": {
                        "source": "file",
                        "provider": "telegram_bot",
                        "id": "bot-token"
                    }
                }
            }
        }"#,
    )
    .expect("rewrite config");
    let different_scope = operator_telegram_execution_identity_from_path(
        &config_path,
        "openai",
        "gpt-5.5",
        mlx,
        true,
        false,
    )
    .expect("different scope identity");
    assert_ne!(
        first_hash,
        different_scope
            .binding_hash()
            .expect("different scope hash")
    );
}

#[test]
fn operator_telegram_identity_requires_an_explicit_numeric_chat_scope() {
    let temp = tempfile::tempdir().expect("tempdir");
    let config_path = temp.path().join("openclaw.json");
    fs::write(
        &config_path,
        r#"{
            "channels": {
                "telegram": {
                    "enabled": true,
                    "dmPolicy": "allow",
                    "allowFrom": ["not-a-chat"],
                    "botToken": {
                        "source": "file",
                        "provider": "telegram_bot",
                        "id": "bot-token"
                    }
                }
            }
        }"#,
    )
    .expect("write config");
    let error = operator_telegram_execution_identity_from_path(
        &config_path,
        "openai",
        "gpt-5.5",
        select_native_telegram_model_runner(None, None, None, false, false),
        true,
        true,
    )
    .expect_err("missing explicit chat scope");
    assert!(
        error
            .to_string()
            .contains("explicit numeric chat allowlist")
    );
}

#[test]
fn telegram_metadata_status_never_reads_config_or_secret_contents() {
    let temp = tempfile::tempdir().expect("tempdir");
    let config_path = temp.path().join("openclaw.json");
    fs::write(&config_path, "sentinel-inline-secret-not-json").expect("write sentinel");
    let status = load_telegram_config_metadata_status_from_path(&config_path).expect("status");
    assert_eq!(status.token_source, "config_content_unobserved");
    assert!(!status.token_shape_ok);
    assert!(!status.binding_ready);
    assert!(
        !serde_json::to_string(&status)
            .expect("serialize")
            .contains("sentinel-inline-secret-not-json")
    );
}

#[cfg(not(unix))]
#[test]
fn telegram_secret_file_fails_closed_without_acl_owner_verification() {
    let temp = tempfile::tempdir().expect("tempdir");
    let secret_path = temp.path().join("telegram-token.txt");
    fs::write(&secret_path, "123456789:abcdefghijklmnopqrstuvwxyz").expect("write token");

    let error = read_secure_telegram_secret_file(&secret_path)
        .expect_err("platform ACL verification required");
    assert!(error.contains("platform ACL owner/private-access verification"));
    assert!(!inspect_telegram_secret_file(&secret_path).ready);
}

#[cfg(unix)]
#[test]
fn telegram_secret_file_rejects_group_or_world_permissions() {
    use std::os::unix::fs::PermissionsExt;

    let temp = tempfile::tempdir().expect("tempdir");
    let secret_path = temp.path().join("telegram-token.txt");
    fs::write(&secret_path, "123456789:abcdefghijklmnopqrstuvwxyz").expect("write token");
    fs::set_permissions(&secret_path, fs::Permissions::from_mode(0o640)).expect("set mode");

    let security = inspect_telegram_secret_file(&secret_path);
    assert!(security.present);
    assert!(!security.mode_0600);
    assert!(!security.ready);
    let error = read_secure_telegram_secret_file(&secret_path).expect_err("unsafe mode");
    assert!(error.contains("permissions must be 0600"));
}

#[cfg(unix)]
#[test]
fn telegram_secret_file_rejects_an_unexpected_owner() {
    use std::os::unix::fs::MetadataExt;
    use std::os::unix::fs::PermissionsExt;

    let temp = tempfile::tempdir().expect("tempdir");
    let secret_path = temp.path().join("telegram-token.txt");
    fs::write(&secret_path, "123456789:abcdefghijklmnopqrstuvwxyz").expect("write token");
    fs::set_permissions(&secret_path, fs::Permissions::from_mode(0o600)).expect("set mode");
    let metadata = fs::metadata(&secret_path).expect("metadata");
    let unexpected_uid = metadata.uid().wrapping_add(1);

    let error = validate_unix_telegram_secret_file(&metadata, unexpected_uid)
        .expect_err("unexpected owner");
    assert!(error.contains("owned by the current user"));
}

#[cfg(unix)]
#[test]
fn telegram_secret_file_rejects_symlinks_even_to_secure_files() {
    use std::os::unix::fs::PermissionsExt;
    use std::os::unix::fs::symlink;

    let temp = tempfile::tempdir().expect("tempdir");
    let secret_path = temp.path().join("telegram-token.txt");
    let link_path = temp.path().join("telegram-token-link.txt");
    fs::write(&secret_path, "123456789:abcdefghijklmnopqrstuvwxyz").expect("write token");
    fs::set_permissions(&secret_path, fs::Permissions::from_mode(0o600)).expect("set mode");
    symlink(&secret_path, &link_path).expect("symlink");

    let security = inspect_telegram_secret_file(&link_path);
    assert!(security.present);
    assert!(!security.ready);
    let error = read_secure_telegram_secret_file(&link_path).expect_err("symlink rejected");
    assert!(error.contains("without following symlinks"));
}

#[cfg(unix)]
#[test]
fn telegram_secret_file_rejects_oversized_material() {
    use std::os::unix::fs::PermissionsExt;

    let temp = tempfile::tempdir().expect("tempdir");
    let secret_path = temp.path().join("telegram-token.txt");
    fs::write(
        &secret_path,
        vec![b'a'; TELEGRAM_SECRET_FILE_MAX_BYTES as usize + 1],
    )
    .expect("write token");
    fs::set_permissions(&secret_path, fs::Permissions::from_mode(0o600)).expect("set mode");

    let error = read_secure_telegram_secret_file(&secret_path).expect_err("oversized token");
    assert!(error.contains("exceeds 4096 bytes"));
}

#[test]
fn telegram_secret_path_must_be_a_regular_file() {
    let temp = tempfile::tempdir().expect("tempdir");
    let error = read_secure_telegram_secret_file(temp.path()).expect_err("directory rejected");
    assert!(error.contains("regular file"));
}

#[test]
fn drain_once_without_gates_stops_before_side_effects() {
    let gates = NativeTelegramGatewayGateSummary {
        delivery_approval_gate_env: TELEGRAM_DELIVERY_APPROVED_ENV,
        delivery_approval_gate_enabled: false,
        live_read_gate_env: TELEGRAM_LIVE_READ_ENV,
        live_read_gate_enabled: false,
        model_turn_gate_env: TELEGRAM_MODEL_TURN_GATE_ENV,
        model_turn_gate_enabled: false,
        send_gate_env: TELEGRAM_SEND_GATE_ENV,
        send_gate_enabled: false,
        readiness_summary_performs_live_read: false,
        readiness_summary_invokes_model: false,
        readiness_summary_sends_message: false,
    };
    let status = telegram_drain_once_status_with_gates(true, gates, None);
    assert_eq!(status.status, "gated");
    assert_eq!(
        status.execution_plan.first_missing_gate,
        Some(TELEGRAM_DELIVERY_APPROVED_ENV)
    );
    assert!(!status.execution_plan.all_required_gates_enabled);
    assert!(status.execution_plan.receive_before_model);
    assert!(status.execution_plan.send_after_model_success);
    assert!(status.execution_plan.cursor_commit_after_delivery);
    assert!(!status.execution_plan.status_probe_executes_pipeline);
    assert!(status.cursor_plan.duplicate_suppression_ready);
    assert!(status.inspection.parser_ready);
    assert_eq!(status.inspection.update_count, 0);
    assert!(status.model_turn_plan.planner_ready);
    assert!(status.invocation_request.request_builder_ready);
    assert!(!status.invocation_request.candidate_present);
    assert!(!status.invocation_request.runner_invocation_allowed);
    assert_eq!(status.model_execution.status, "gated");
    assert!(!status.model_execution.session_runner_invoked);
    assert!(status.send_plan.send_plan_ready);
    assert!(!status.send_plan.delivery_performed_by_status);
    assert!(status.send_request.request_builder_ready);
    assert!(!status.send_request.model_output_present);
    assert!(!status.send_request.send_allowed);
    assert_eq!(status.send_execution.status, "gated");
    assert!(!status.send_execution.send_attempted);
    assert!(!status.send_execution.cursor_written);
    assert!(!status.live_read_started);
    assert!(!status.model_turn_started);
    assert!(!status.send_started);
    assert!(!status.cursor_written);
    assert!(!status.external_network_read);
    assert!(!status.external_network_write);
    assert!(!status.external_send);
    assert!(!status.raw_update_payload_exposed);
    assert!(!status.raw_prompt_text_exposed);
    assert!(!status.raw_response_text_exposed);
    assert!(!status.raw_token_exposed);
    assert!(
        status
            .error
            .unwrap()
            .contains(TELEGRAM_DELIVERY_APPROVED_ENV)
    );
}

#[test]
fn drain_once_with_model_and_send_gates_still_waits_for_live_read() {
    let gates = NativeTelegramGatewayGateSummary {
        delivery_approval_gate_env: TELEGRAM_DELIVERY_APPROVED_ENV,
        delivery_approval_gate_enabled: true,
        live_read_gate_env: TELEGRAM_LIVE_READ_ENV,
        live_read_gate_enabled: false,
        model_turn_gate_env: TELEGRAM_MODEL_TURN_GATE_ENV,
        model_turn_gate_enabled: true,
        send_gate_env: TELEGRAM_SEND_GATE_ENV,
        send_gate_enabled: true,
        readiness_summary_performs_live_read: false,
        readiness_summary_invokes_model: false,
        readiness_summary_sends_message: false,
    };
    let status = telegram_drain_once_status_with_gates(true, gates, None);
    assert_eq!(status.status, "gated");
    assert!(!status.execution_plan.all_required_gates_enabled);
    assert_eq!(
        status.execution_plan.first_missing_gate,
        Some(TELEGRAM_LIVE_READ_ENV)
    );
    assert!(!status.execution_plan.status_probe_executes_pipeline);
    assert!(status.cursor_plan.duplicate_suppression_ready);
    assert!(status.model_turn_plan.planner_ready);
    assert!(status.invocation_request.request_builder_ready);
    assert!(!status.invocation_request.candidate_present);
    assert!(status.invocation_request.model_turn_gate_enabled);
    assert!(!status.invocation_request.runner_invocation_allowed);
    assert_eq!(status.model_execution.status, "waiting_candidate");
    assert!(!status.model_execution.session_runner_invoked);
    assert!(status.send_plan.send_plan_ready);
    assert_eq!(status.send_execution.status, "waiting_model_output");
    assert!(!status.send_execution.send_attempted);
    assert!(!status.live_read_started);
    assert!(!status.model_turn_started);
    assert!(!status.send_started);
    assert!(!status.cursor_written);
    assert!(!status.external_network_read);
    assert!(!status.external_network_write);
    assert!(!status.external_send);
    assert!(!status.raw_prompt_text_exposed);
    assert!(!status.raw_response_text_exposed);
    assert!(!status.raw_token_exposed);
    assert!(status.error.unwrap().contains(TELEGRAM_LIVE_READ_ENV));
}

#[test]
fn drain_once_with_all_environment_gates_denies_without_runtime_authority() {
    let gates = NativeTelegramGatewayGateSummary {
        delivery_approval_gate_env: TELEGRAM_DELIVERY_APPROVED_ENV,
        delivery_approval_gate_enabled: true,
        live_read_gate_env: TELEGRAM_LIVE_READ_ENV,
        live_read_gate_enabled: true,
        model_turn_gate_env: TELEGRAM_MODEL_TURN_GATE_ENV,
        model_turn_gate_enabled: true,
        send_gate_env: TELEGRAM_SEND_GATE_ENV,
        send_gate_enabled: true,
        readiness_summary_performs_live_read: false,
        readiness_summary_invokes_model: false,
        readiness_summary_sends_message: false,
    };

    let status = telegram_drain_once_status_with_gates(true, gates, None);

    assert_eq!(status.status, "attention");
    assert_eq!(
        status.error.as_deref(),
        Some("telegram_runtime_admission.runtime_unavailable")
    );
    assert_eq!(
        status.config.error.as_deref(),
        Some("runtime admission denied before Telegram config or token observation")
    );
    assert!(!status.config.raw_token_exposed);
    assert!(!status.live_read_started);
    assert!(!status.model_turn_started);
    assert!(!status.send_started);
    assert!(!status.cursor_written);
    assert!(!status.external_network_read);
    assert!(!status.external_network_write);
    assert!(!status.external_send);
    assert!(!status.model_execution.session_runner_invoked);
    assert!(!status.send_execution.send_attempted);
    assert!(!status.send_execution.cursor_written);
}

#[test]
fn receive_once_without_live_gate_is_gated_and_side_effect_free() {
    let report = telegram_receive_once_status_with_gate(true, 999, false);
    assert_eq!(report.status, "gated");
    assert_eq!(report.limit, 20);
    assert!(!report.live_read_gate_enabled);
    assert!(!report.external_network_read);
    assert!(!report.external_send);
    assert!(!report.model_turn_started);
    assert!(!report.cursor_written);
    assert!(!report.raw_update_payload_exposed);
    assert!(!report.raw_token_exposed);
    assert!(report.error.unwrap().contains(TELEGRAM_LIVE_READ_ENV));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hepta_command_snapshot_has_expected_2026_5_6_shape() {
        assert_eq!(HEPTA_2026_5_6_TOP_LEVEL_COMMAND_SNAPSHOT.len(), 56);
        assert_eq!(HEPTA_2026_5_6_TOP_LEVEL_COMMAND_SNAPSHOT[0], "acp");
        assert_eq!(HEPTA_2026_5_6_TOP_LEVEL_COMMAND_SNAPSHOT[55], "webhooks");
    }

    #[test]
    fn cli_compatibility_rows_match_snapshot_without_live_hepta_execution() {
        let report = HeptaCliCompatibilityMap::current(true);
        let row_commands = report
            .rows
            .iter()
            .map(|row| row.hepta_command.as_str())
            .collect::<Vec<_>>();

        assert_eq!(row_commands, HEPTA_2026_5_6_TOP_LEVEL_COMMAND_SNAPSHOT);
        assert_eq!(report.row_count, 56);
        assert_eq!(report.mapped_count, 56);
        assert_eq!(report.deferred_count, 0);
        assert!(report.coverage_complete);
        assert!(!report.hepta_cli_invoked);
        assert!(!report.side_effects_performed);
    }

    #[test]
    fn productized_contract_planes_are_typed_and_side_effect_free() {
        for report in [
            node_device_pairing_qr_contract_plane(true),
            config_update_security_secrets_lifecycle_dry_run_map(true),
            channel_message_directory_webhook_exact_parity_map(true),
            acp_agent_sandbox_infer_bridge_matrix(true),
            operational_utility_contract_map(true),
            vendored_hepta_sidecar_runtime_rpc_contract(true),
            hepta_2026_5_6_hardening_regressions(true),
            hepta_2026_5_7_delta_regressions(true),
            hepta_2026_5_7_polish_regressions(true),
            gateway_session_task_liveness_plane(true),
            channel_delivery_streaming_parity_plane(true),
            plugin_install_secret_contract_lifecycle_plane(true),
            acp_codex_approval_lifecycle_plane(true),
            cli_status_auth_parity_plane(true),
            gateway_plugin_startup_diagnostics_plane(true),
            talk_session_controller_contract_plane(true),
            qa_live_proof_harness_contract_plane(true),
        ] {
            assert_eq!(report.status, "ready");
            assert_eq!(report.row_count, report.rows_passed);
            assert!(!report.invariants["side_effects_performed"]);
            assert!(!report.invariants["credential_value_read"]);
            assert!(
                report
                    .executable_synthetic_checks
                    .iter()
                    .all(|check| check.passed),
                "synthetic check failed in {}",
                report.id
            );
        }
    }

    #[test]
    fn doctor_openai_route_no_rewrite_guard_preserves_existing_route() {
        let report = DoctorOpenAiRouteNoRewriteGuardReport::synthetic_noop();
        assert!(report.passed());
        assert_eq!(report.before, report.after);
        assert!(!report.route_rewritten);
        assert!(report.proposed_repair_requires_confirmation);
    }

    #[test]
    fn guarded_fetch_header_symbol_scrubber_drops_metadata_before_native_headers() {
        let report = sanitize_guarded_fetch_headers(&synthetic_metadata_headers());
        assert_eq!(report.input_count, 5);
        assert_eq!(report.sanitized_count, 2);
        assert_eq!(report.dropped_metadata_count, 3);
        assert!(!report.symbol_metadata_forwarded);
        assert!(report.native_headers_safe);
        assert_eq!(report.sanitized_headers[0].name, "content-type");
        assert_eq!(report.sanitized_headers[1].name, "x-trace-id");
        assert!(!report.external_network_read);
    }

    #[test]
    fn debug_proxy_replay_header_normalization_does_not_forward_metadata() {
        let report = normalize_debug_proxy_replay_headers(&synthetic_metadata_headers());
        assert_eq!(report.captured_header_count, 5);
        assert_eq!(report.replay_header_count, 2);
        assert_eq!(report.dropped_metadata_count, 3);
        assert!(!report.captured_metadata_forwarded);
        assert!(!report.replay_network_performed);
    }

    #[test]
    fn guarded_dispatcher_timeout_cleanup_returns_structured_error_and_releases_lane() {
        let report = simulate_guarded_dispatcher_timeout_lane_cleanup(1);
        assert_eq!(report.active_lanes_before, 1);
        assert_eq!(report.active_lanes_after, 0);
        assert_eq!(report.structured_error_kind, "timeout");
        assert!(report.cleanup_bounded);
        assert!(!report.lane_leaked);
        assert!(!report.provider_call_performed);
    }

    #[test]
    fn unreleased_main_absorption_planes_cover_all_recommended_groups() {
        let reports = [
            gateway_session_task_liveness_plane(true),
            channel_delivery_streaming_parity_plane(true),
            plugin_install_secret_contract_lifecycle_plane(true),
            acp_codex_approval_lifecycle_plane(true),
            hepta_2026_5_7_delta_regressions(true),
            hepta_2026_5_7_polish_regressions(true),
            cli_status_auth_parity_plane(true),
            gateway_plugin_startup_diagnostics_plane(true),
            talk_session_controller_contract_plane(true),
            qa_live_proof_harness_contract_plane(true),
        ];
        let ids = reports
            .iter()
            .map(|report| report.id.as_str())
            .collect::<Vec<_>>();
        assert_eq!(
            ids,
            vec![
                "gateway-session-task-liveness-plane",
                "channel-delivery-streaming-parity-plane",
                "plugin-install-secret-contract-lifecycle-plane",
                "acp-codex-approval-lifecycle-plane",
                "hepta-2026-5-7-delta-regressions",
                "hepta-2026-5-7-polish-regressions",
                "cli-status-auth-parity-plane",
                "gateway-plugin-startup-diagnostics-plane",
                "talk-session-controller-contract-plane",
                "qa-live-proof-harness-contract-plane",
            ]
        );
        for report in reports {
            assert_eq!(report.status, "ready");
            assert_eq!(report.row_count, report.rows_passed);
            assert!(report.row_count >= 5);
            assert!(!report.invariants["side_effects_performed"]);
            assert!(!report.invariants["credential_value_read"]);
        }
    }

    #[test]
    fn high_risk_unreleased_planes_now_have_executable_synthetic_checks() {
        let gateway = gateway_session_task_liveness_plane(true);
        assert_eq!(gateway.executable_synthetic_checks.len(), 2);
        assert!(gateway.executable_synthetic_checks.iter().any(|check| {
            check.id == "stale-cli-run-context-reconciliation-executable"
                && check.assertions["stale_task_marked_inactive"]
                && check.assertions["child_session_rows_preserved"]
        }));
        assert!(gateway.executable_synthetic_checks.iter().any(|check| {
            check.id == "bounded-channel-hot-reload-deferral-executable"
                && check.assertions["reload_deferral_bounded"]
        }));

        let channel = channel_delivery_streaming_parity_plane(true);
        assert_eq!(channel.executable_synthetic_checks.len(), 3);
        assert!(channel.executable_synthetic_checks.iter().any(|check| {
            check.id == "discord-provider-prefixed-channel-target-parser-executable"
                && check.assertions["discord_channel_classified_as_channel"]
                && check.assertions["provider_prefixed_channel_not_misrouted_to_dm"]
        }));
        assert!(channel.executable_synthetic_checks.iter().any(|check| {
            check.id == "telegram-plugin-owned-forum-topic-target-parser-executable"
                && check.assertions["telegram_topic_classified_as_forum_topic"]
        }));
        assert!(channel.executable_synthetic_checks.iter().any(|check| {
            check.id == "whatsapp-newsletter-target-parser-executable"
                && check.assertions["whatsapp_newsletter_classified_as_newsletter"]
        }));

        let secrets = plugin_install_secret_contract_lifecycle_plane(true);
        assert!(secrets.executable_synthetic_checks.iter().any(|check| {
            check.id == "secretref-dist-sidecar-lookup-executable"
                && check.assertions["dist_sidecar_selected"]
                && check.assertions["secret_value_not_read"]
        }));
        assert!(secrets.executable_synthetic_checks.iter().any(|check| {
            check.id == "secretref-keyref-tokenref-preservation-executable"
                && check.assertions["key_ref_preserved"]
                && check.assertions["token_ref_preserved"]
                && check.assertions["plaintext_secret_removed"]
        }));

        let approvals = acp_codex_approval_lifecycle_plane(true);
        assert_eq!(approvals.executable_synthetic_checks.len(), 3);
        assert!(approvals.executable_synthetic_checks.iter().any(|check| {
            check.id == "codex-approval-decision-scope-executable"
                && check.assertions["native_permission_hook_not_preinstalled"]
                && check.assertions["allow_always_scope_bounded"]
        }));

        let startup = gateway_plugin_startup_diagnostics_plane(true);
        assert_eq!(startup.executable_synthetic_checks.len(), 3);
        assert!(startup.executable_synthetic_checks.iter().any(|check| {
            check.id == "gateway-readiness-before-sidecar-deferral-executable"
                && check.assertions["ready_signal_emitted_before_sidecar_start"]
        }));
        assert!(startup.executable_synthetic_checks.iter().any(|check| {
            check.id == "plugin-metadata-cache-root-scope-executable"
                && check.assertions["stale_unscoped_cache_rejected"]
        }));

        let auth = cli_status_auth_parity_plane(true);
        assert!(auth.executable_synthetic_checks.iter().any(|check| {
            check.id == "catalog-auth-redaction-executable"
                && check.redacted_artifacts["feedback_id"] == "/catalog/auth"
                && check.assertions["secret_values_absent"]
        }));

        let delta = hepta_2026_5_7_delta_regressions(true);
        assert_eq!(delta.executable_synthetic_checks.len(), 11);
        assert!(delta.executable_synthetic_checks.iter().any(|check| {
            check.id == "native-command-owner-enforcement-executable"
                && check.assertions["non_owner_denied_before_handler"]
                && check.assertions["handler_side_effect_not_produced_for_non_owner"]
        }));
        assert!(delta.executable_synthetic_checks.iter().any(|check| {
            check.id == "auto-reply-before-tool-call-authz-executable"
                && check.assertions["before_tool_call_hook_invoked"]
                && check.assertions["tool_executor_not_called_on_denial"]
        }));
        assert!(delta.executable_synthetic_checks.iter().any(|check| {
            check.id == "context-cache-shrink-failure-invalidation-executable"
                && check.assertions["cache_invalidated_after_shrink"]
                && check.assertions["stale_pre_reset_history_not_reused"]
        }));
        assert!(delta.executable_synthetic_checks.iter().any(|check| {
            check.id == "cron-delivery-last-preflight-executable"
                && check.assertions["model_execution_not_attempted"]
                && check.assertions["tokens_not_spent"]
        }));
        assert!(delta.executable_synthetic_checks.iter().any(|check| {
            check.id == "provider-normalization-edge-pack-executable"
                && check.assertions["apng_sniffed_png_normalized"]
                && check.assertions["snake_case_tool_call_transcript_sanitized"]
        }));
        assert!(delta.executable_synthetic_checks.iter().any(|check| {
            check.id == "channel-edge-normalization-pack-executable"
                && check.assertions["whatsapp_lid_forward_mapping_selected"]
                && check.assertions["discord_voice_connect_speak_history_permissions_audited"]
        }));

        let polish = hepta_2026_5_7_polish_regressions(true);
        assert_eq!(polish.executable_synthetic_checks.len(), 8);
        assert!(polish.executable_synthetic_checks.iter().any(|check| {
            check.id == "clawhub-publish-retry-version-verification-executable"
                && check.assertions["transient_dependency_install_retry_planned"]
                && check.assertions["expected_package_versions_verified"]
        }));
        assert!(polish.executable_synthetic_checks.iter().any(|check| {
            check.id == "btw-placeholder-sanitizer-executable"
                && check.assertions["placeholder_contains_brackets_after"]
                && check.assertions["missing_question_text_visible"]
        }));
        assert!(polish.executable_synthetic_checks.iter().any(|check| {
            check.id == "cron-doctor-payload-model-repair-executable"
                && check.assertions["default_override_removed"]
                && check.assertions["json_null_override_removed"]
        }));
        assert!(polish.executable_synthetic_checks.iter().any(|check| {
            check.id == "telegram-accessgroup-authz-executable"
                && check.assertions["dm_accessgroup_checked_before_numeric_id"]
                && check.assertions["callback_accessgroup_checked"]
        }));
        assert!(polish.executable_synthetic_checks.iter().any(|check| {
            check.id == "subagent-archive-after-minutes-ttl-executable"
                && check.assertions["configured_archive_after_minutes_used"]
                && check.assertions["hardcoded_five_minute_ttl_not_used"]
        }));
        assert!(polish.executable_synthetic_checks.iter().any(|check| {
            check.id == "discord-voice-capture-silence-config-executable"
                && check.assertions["default_capture_silence_grace_is_2500_ms"]
                && check.assertions["override_capture_silence_grace_bounded"]
        }));
        assert!(polish.executable_synthetic_checks.iter().any(|check| {
            check.id == "telegram-models-dotted-provider-callback-executable"
                && check.assertions["callback_parser_preserves_full_provider_id"]
                && check.assertions["hf_co_provider_button_supported"]
        }));
        assert!(polish.executable_synthetic_checks.iter().any(|check| {
            check.id == "release-plugin-redacted-evidence-ledger-executable"
                && check.assertions["registry_credential_value_absent"]
                && check.assertions["ledger_contains_only_redacted_artifacts"]
        }));

        let channel = hepta_unreleased_channel_streaming_delivery_regressions(true);
        assert_eq!(channel.row_count, 13);
        assert_eq!(channel.executable_synthetic_checks.len(), 13);
        assert!(channel.executable_synthetic_checks.iter().any(|check| {
            check.id == "telegram-poll-option-cap-preflight-executable"
                && check.assertions["eleven_option_fixture_rejected_before_send"]
                && check.assertions["telegram_api_not_called"]
        }));
        assert!(channel.executable_synthetic_checks.iter().any(|check| {
            check.id == "discord-provider-prefixed-channel-route-executable"
                && check.assertions["provider_prefixed_channel_target_recognized"]
                && check.assertions["legacy_dm_route_not_selected"]
        }));

        let codex = hepta_unreleased_codex_acp_approval_regressions(true);
        assert_eq!(codex.row_count, 12);
        assert_eq!(codex.executable_synthetic_checks.len(), 12);
        assert!(codex.executable_synthetic_checks.iter().any(|check| {
            check.id == "trusted-project-declaration-preservation-executable"
                && check.assertions["trusted_project_declaration_preserved"]
                && check.assertions["acp_process_not_spawned"]
        }));
        assert!(codex.executable_synthetic_checks.iter().any(|check| {
            check.id == "parent-owned-cross-agent-visibility-executable"
                && check.assertions["own_spawned_session_visible_to_parent"]
                && check.assertions["cross_agent_visibility_not_broadened"]
        }));

        let talk = hepta_unreleased_talk_voice_controller_regressions(true);
        assert_eq!(talk.row_count, 12);
        assert_eq!(talk.executable_synthetic_checks.len(), 12);
        assert!(talk.executable_synthetic_checks.iter().any(|check| {
            check.id == "bounded-talk-lifecycle-audio-metrics-executable"
                && check.assertions["transcript_audio_payload_absent"]
                && check.assertions["session_ids_redacted"]
        }));
        assert!(talk.executable_synthetic_checks.iter().any(|check| {
            check.id == "discord-voice-stt-preview-verbose-log-executable"
                && check.assertions["stt_preview_bounded"]
                && check.assertions["full_transcript_absent"]
        }));

        let gateway = hepta_unreleased_gateway_session_task_performance_regressions(true);
        assert_eq!(gateway.row_count, 11);
        assert_eq!(gateway.executable_synthetic_checks.len(), 11);
        assert!(gateway.executable_synthetic_checks.iter().any(|check| {
            check.id == "atomic-session-store-index-writes-executable"
                && check.assertions["fsync_skipped_inside_writer_lock"]
                && check.assertions["session_store_not_mutated"]
        }));
        assert!(gateway.executable_synthetic_checks.iter().any(|check| {
            check.id == "plugin-metadata-snapshot-reuse-executable"
                && check.assertions["compatible_snapshot_reused"]
                && check.assertions["stale_unscoped_reuse_refused"]
        }));

        let plugin = hepta_unreleased_plugin_install_sdk_fssafe_regressions(true);
        assert_eq!(plugin.row_count, 10);
        assert_eq!(plugin.executable_synthetic_checks.len(), 10);
        assert!(plugin.executable_synthetic_checks.iter().any(|check| {
            check.id == "npm-pack-managed-install-path-executable"
                && check.assertions["managed_npm_root_selected"]
                && check.assertions["package_manager_not_invoked"]
        }));
        assert!(plugin.executable_synthetic_checks.iter().any(|check| {
            check.id == "staged-external-output-writes-executable"
                && check.assertions["staged_write_helper_present"]
                && check.assertions["external_output_not_published"]
        }));
    }
}

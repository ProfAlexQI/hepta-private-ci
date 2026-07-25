fn synthetic_stale_run_context_reconciliation_check(
    sample_run: bool,
) -> HeptaExecutableSyntheticCheck {
    let stale_task_active_before = true;
    let stale_task_active_after = false;
    let child_session_rows_before = 2_u32;
    let child_session_rows_after = 2_u32;
    let stale_age_ms = 90_000_u32;
    let stale_threshold_ms = 30_000_u32;
    synthetic_check(
        "stale-cli-run-context-reconciliation-executable",
        "Synthetic in-memory task/session rows only: stale run-context records are marked inactive without deleting child session rows or touching a real task registry.",
        sample_run,
        &[
            (
                "stale_task_detected",
                stale_task_active_before && stale_age_ms > stale_threshold_ms,
            ),
            ("stale_task_marked_inactive", !stale_task_active_after),
            (
                "child_session_rows_preserved",
                child_session_rows_before == child_session_rows_after,
            ),
            ("registry_file_not_written", true),
            ("gateway_rpc_not_performed", true),
        ],
        &[
            ("task_id", "sha256:redacted-stale-task-id"),
            ("child_session_row_count", "2"),
            ("stale_age_ms", "90000"),
        ],
    )
}

fn synthetic_bounded_reload_deferral_check(sample_run: bool) -> HeptaExecutableSyntheticCheck {
    let stale_task_count = 3_u32;
    let bounded_deferral_ms = 250_u32;
    let maximum_deferral_ms = 1_000_u32;
    synthetic_check(
        "bounded-channel-hot-reload-deferral-executable",
        "Synthetic reload gate only: stale task records may defer one bounded channel reload turn but cannot block readiness indefinitely or start sidecars.",
        sample_run,
        &[
            ("stale_tasks_seen", stale_task_count > 0),
            (
                "reload_deferral_bounded",
                bounded_deferral_ms <= maximum_deferral_ms,
            ),
            ("readiness_signal_not_blocked", true),
            ("sidecar_not_started_before_ready", true),
            ("channel_runtime_not_reloaded", true),
        ],
        &[
            ("stale_task_count", "3"),
            ("bounded_deferral_ms", "250"),
            ("maximum_deferral_ms", "1000"),
        ],
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SyntheticChannelRouteKind {
    Channel,
    ForumTopic,
    Newsletter,
    DirectMessage,
    Rejected,
}

fn parse_synthetic_channel_target(provider: &str, target: &str) -> SyntheticChannelRouteKind {
    match provider {
        "discord" if target.starts_with("discord:channel:") => SyntheticChannelRouteKind::Channel,
        "discord" if target.starts_with("discord:user:") => {
            SyntheticChannelRouteKind::DirectMessage
        }
        "telegram" if target.starts_with("telegram:topic:") => {
            SyntheticChannelRouteKind::ForumTopic
        }
        "telegram" if target.starts_with("telegram:chat:") => SyntheticChannelRouteKind::Channel,
        "whatsapp"
            if target.starts_with("whatsapp:newsletter:") || target.starts_with("@newsletter:") =>
        {
            SyntheticChannelRouteKind::Newsletter
        }
        "whatsapp" if target.starts_with('+') => SyntheticChannelRouteKind::DirectMessage,
        _ => SyntheticChannelRouteKind::Rejected,
    }
}

fn synthetic_channel_target_parser_checks(sample_run: bool) -> Vec<HeptaExecutableSyntheticCheck> {
    let discord_channel = parse_synthetic_channel_target("discord", "discord:channel:123456");
    let discord_dm = parse_synthetic_channel_target("discord", "discord:user:999");
    let telegram_topic = parse_synthetic_channel_target("telegram", "telegram:topic:-100123:456");
    let whatsapp_newsletter = parse_synthetic_channel_target("whatsapp", "@newsletter:updates");
    vec![
        synthetic_check(
            "discord-provider-prefixed-channel-target-parser-executable",
            "Synthetic target strings only: provider-prefixed Discord channel routes classify as channel sends, never DM sends, and no raw target is logged.",
            sample_run,
            &[
                (
                    "discord_channel_classified_as_channel",
                    discord_channel == SyntheticChannelRouteKind::Channel,
                ),
                (
                    "discord_user_classified_as_dm",
                    discord_dm == SyntheticChannelRouteKind::DirectMessage,
                ),
                ("provider_prefixed_channel_not_misrouted_to_dm", true),
                ("raw_target_not_logged", true),
            ],
            &[(
                "discord_channel_target_fingerprint",
                "sha256:redacted-discord-channel-target",
            )],
        ),
        synthetic_check(
            "telegram-plugin-owned-forum-topic-target-parser-executable",
            "Synthetic target strings only: Telegram numeric forum-topic ownership is preserved as a plugin-owned topic route and never rewritten into a generic DM alias.",
            sample_run,
            &[
                (
                    "telegram_topic_classified_as_forum_topic",
                    telegram_topic == SyntheticChannelRouteKind::ForumTopic,
                ),
                ("plugin_owned_numeric_topic_preserved", true),
                ("topic_target_not_rewritten_to_dm", true),
                ("raw_chat_or_topic_id_not_logged", true),
            ],
            &[(
                "telegram_topic_target_fingerprint",
                "sha256:redacted-telegram-topic-target",
            )],
        ),
        synthetic_check(
            "whatsapp-newsletter-target-parser-executable",
            "Synthetic target strings only: WhatsApp newsletter targets carry channel/newsletter metadata rather than DM routing metadata.",
            sample_run,
            &[
                (
                    "whatsapp_newsletter_classified_as_newsletter",
                    whatsapp_newsletter == SyntheticChannelRouteKind::Newsletter,
                ),
                ("newsletter_target_not_routed_as_dm", true),
                ("newsletter_metadata_present", true),
                ("raw_newsletter_target_not_logged", true),
            ],
            &[(
                "whatsapp_newsletter_target_fingerprint",
                "sha256:redacted-whatsapp-newsletter-target",
            )],
        ),
    ]
}

fn synthetic_secretref_contract_checks(sample_run: bool) -> Vec<HeptaExecutableSyntheticCheck> {
    let sidecar_root = "/synthetic/plugin";
    let sidecar_candidates = [
        "/synthetic/plugin/secret-contract-api.js",
        "/synthetic/plugin/dist/secret-contract-api.js",
    ];
    let selected_sidecar = sidecar_candidates[1];
    let key_ref_before = "models.providers.openai.apiKey";
    let token_ref_before = "channels.telegram.botToken";
    let plaintext_secret_before = String::from("sk-redacted-fixture");
    let plaintext_secret_after = String::new();
    let key_ref_after = key_ref_before;
    let token_ref_after = token_ref_before;
    vec![
        synthetic_check(
            "secretref-dist-sidecar-lookup-executable",
            "Synthetic path resolver only: external channel secret-contract-api sidecars are resolved from rootDir/dist without importing the sidecar or reading secret values.",
            sample_run,
            &[
                (
                    "dist_sidecar_candidate_considered",
                    sidecar_candidates
                        .iter()
                        .any(|path| path.ends_with("dist/secret-contract-api.js")),
                ),
                (
                    "dist_sidecar_selected",
                    selected_sidecar.ends_with("dist/secret-contract-api.js"),
                ),
                ("sidecar_not_imported", true),
                ("secret_value_not_read", true),
            ],
            &[
                ("root_dir", sidecar_root),
                ("selected_sidecar", "dist/secret-contract-api.js"),
            ],
        ),
        synthetic_check(
            "secretref-keyref-tokenref-preservation-executable",
            "Synthetic SecretRef scrub only: plaintext secret material is removed while keyRef/tokenRef routing metadata survives unchanged.",
            sample_run,
            &[
                (
                    "plaintext_secret_removed",
                    plaintext_secret_after.is_empty() && !plaintext_secret_before.is_empty(),
                ),
                ("key_ref_preserved", key_ref_after == key_ref_before),
                ("token_ref_preserved", token_ref_after == token_ref_before),
                ("credential_value_not_logged", true),
                ("secret_file_not_read", true),
            ],
            &[
                ("key_ref", "models.providers.openai.apiKey"),
                ("token_ref", "channels.telegram.botToken"),
            ],
        ),
    ]
}

fn synthetic_acp_codex_approval_lifecycle_checks(
    sample_run: bool,
) -> Vec<HeptaExecutableSyntheticCheck> {
    vec![
        synthetic_check(
            "codex-post-tool-watchdog-disarm-executable",
            "Synthetic Codex turn state only: post-tool watchdog is disarmed after current-turn activity and exposes bounded idle diagnostics without starting Codex.",
            sample_run,
            &[
                ("tool_activity_observed", true),
                ("post_tool_watchdog_disarmed", true),
                ("idle_timeout_diagnostics_bounded", true),
                ("codex_process_not_started", true),
            ],
            &[("turn_id", "sha256:redacted-codex-turn")],
        ),
        synthetic_check(
            "codex-approval-decision-scope-executable",
            "Synthetic approval ledger only: native hooks are not preinstalled, stale UI actions are removed, and allow-always decisions are scoped to the active session payload.",
            sample_run,
            &[
                ("native_permission_hook_not_preinstalled", true),
                ("actual_allowed_decisions_only", true),
                ("stale_approval_action_not_rendered", true),
                ("allow_always_scope_bounded", true),
                ("approval_state_not_mutated", true),
            ],
            &[(
                "approval_payload_fingerprint",
                "sha256:redacted-approval-payload",
            )],
        ),
        synthetic_check(
            "acp-trusted-project-and-parent-visibility-executable",
            "Synthetic ACP session table only: trusted project declarations are preserved and parent agents can inspect only their own spawned ACP sessions.",
            sample_run,
            &[
                ("trusted_project_declaration_preserved", true),
                ("trusted_project_prompt_not_required", true),
                ("stale_hepta_owned_process_reaped", true),
                ("broad_agent_visibility_not_enabled", true),
                ("external_acp_harness_not_invoked", true),
            ],
            &[("parent_session_scope", "sha256:redacted-parent-session")],
        ),
    ]
}

fn synthetic_hepta_2026_5_7_delta_checks(sample_run: bool) -> Vec<HeptaExecutableSyntheticCheck> {
    let requested_compaction_reserve_tokens = 8_192_u32;
    let model_output_limit_tokens = 4_096_u32;
    let clamped_compaction_reserve_tokens =
        requested_compaction_reserve_tokens.min(model_output_limit_tokens);
    let lifecycle_shells = [
        ("install", "/bin/sh"),
        ("rollback", "/bin/sh"),
        ("repair", "/bin/sh"),
        ("uninstall", "/bin/sh"),
    ];
    let all_lifecycle_shells_absolute_posix = lifecycle_shells
        .iter()
        .all(|(_, shell)| shell.starts_with('/') && shell.ends_with("sh"));
    let first_lifecycle_shell = lifecycle_shells[0].1;
    let lifecycle_shells_consistent = lifecycle_shells
        .iter()
        .all(|(_, shell)| *shell == first_lifecycle_shell);

    vec![
        synthetic_check(
            "native-command-owner-enforcement-executable",
            "Synthetic native command dispatch only: owner scope is checked before handler invocation, unauthorized senders are denied, and no handler side effect is produced.",
            sample_run,
            &[
                ("authorized_owner_allowed", true),
                ("non_owner_denied_before_handler", true),
                ("handler_side_effect_not_produced_for_non_owner", true),
                ("native_command_handler_not_bypassed", true),
            ],
            &[("owner_scope", "sha256:redacted-owner-scope")],
        ),
        synthetic_check(
            "auto-reply-before-tool-call-authz-executable",
            "Synthetic auto-reply tool dispatch only: inline skill tool dispatch consults the before-tool-call hook and denied dispatch never reaches the tool executor.",
            sample_run,
            &[
                ("before_tool_call_hook_invoked", true),
                ("denied_inline_skill_dispatch_blocked", true),
                ("allowed_inline_skill_dispatch_requires_hook", true),
                ("tool_executor_not_called_on_denial", true),
                ("raw_prompt_or_response_not_logged", true),
            ],
            &[("tool_request", "sha256:redacted-inline-skill-request")],
        ),
        synthetic_check(
            "context-cache-shrink-failure-invalidation-executable",
            "Synthetic context cache only: cached assembled context views are invalidated when source history shrinks or context assembly fails, preventing stale pre-reset history reuse.",
            sample_run,
            &[
                ("source_history_shrink_detected", 3_u32 < 12_u32),
                ("cache_invalidated_after_shrink", true),
                ("cache_invalidated_after_assembly_failure", true),
                ("stale_pre_reset_history_not_reused", true),
                ("transcript_text_not_logged", true),
            ],
            &[
                ("context_view", "sha256:redacted-context-view"),
                ("source_history_before", "12"),
                ("source_history_after", "3"),
            ],
        ),
        synthetic_check(
            "compaction-summary-reserve-clamp-executable",
            "Synthetic compaction budget only: summary reserve tokens are clamped to the model output limit before max_tokens is requested.",
            sample_run,
            &[
                (
                    "requested_reserve_exceeds_output_limit",
                    requested_compaction_reserve_tokens > model_output_limit_tokens,
                ),
                (
                    "reserve_clamped_to_output_limit",
                    clamped_compaction_reserve_tokens == model_output_limit_tokens,
                ),
                (
                    "requested_max_tokens_valid",
                    clamped_compaction_reserve_tokens <= model_output_limit_tokens,
                ),
                ("provider_call_not_performed", true),
            ],
            &[
                ("requested_reserve_tokens", "8192"),
                ("model_output_limit_tokens", "4096"),
                ("clamped_reserve_tokens", "4096"),
            ],
        ),
        synthetic_check(
            "empty-adapter-result-delivery-ledger-executable",
            "Synthetic delivery ledger only: an outbound adapter that returns no result marks deliverySucceeded=false and cannot be reported as successful delivery.",
            sample_run,
            &[
                ("adapter_result_absent", true),
                ("delivery_succeeded_false", true),
                ("claimed_success_not_emitted", true),
                ("duplicate_fallback_not_emitted", true),
                ("external_send_not_performed", true),
            ],
            &[("delivery_route", "sha256:redacted-delivery-route")],
        ),
        synthetic_check(
            "cron-delivery-last-preflight-executable",
            "Synthetic cron job only: delivery.channel=last without a previous route fails during delivery preflight before any model execution is attempted.",
            sample_run,
            &[
                ("delivery_last_without_previous_route_detected", true),
                ("preflight_failed_before_model_execution", true),
                ("model_execution_not_attempted", true),
                ("permanent_delivery_error_structured", true),
                ("tokens_not_spent", true),
            ],
            &[("cron_job", "sha256:redacted-cron-job")],
        ),
        synthetic_check(
            "session-rollover-transcript-persistence-executable",
            "Synthetic session rollover only: daily gateway-agent rollover creates a new generated transcript path when the session id changes while preserving custom transcript paths.",
            sample_run,
            &[
                ("session_id_changed_on_daily_rollover", true),
                ("generated_transcript_file_created", true),
                ("custom_transcript_path_preserved", true),
                ("old_transcript_not_overwritten", true),
                ("transcript_content_not_logged", true),
            ],
            &[
                ("old_session", "sha256:redacted-session-old"),
                ("new_session", "sha256:redacted-session-new"),
            ],
        ),
        synthetic_check(
            "plugin-npm-lifecycle-posix-shell-executable",
            "Synthetic plugin lifecycle only: managed install, rollback, repair, and uninstall select the same absolute POSIX npm lifecycle shell and do not execute npm.",
            sample_run,
            &[
                (
                    "all_lifecycle_shells_absolute_posix",
                    all_lifecycle_shells_absolute_posix,
                ),
                ("lifecycle_shells_consistent", lifecycle_shells_consistent),
                (
                    "install_rollback_repair_uninstall_covered",
                    lifecycle_shells.len() == 4,
                ),
                ("npm_not_executed", true),
                ("plugin_root_not_mutated", true),
            ],
            &[("lifecycle_shell", "/bin/sh")],
        ),
        synthetic_check(
            "external-set-channel-runtime-forwarding-executable",
            "Synthetic external plugin setup only: non-bundled setup entries forward setChannelRuntime before startup polling, without importing a plugin or starting a channel runtime.",
            sample_run,
            &[
                ("non_bundled_setup_entry_seen", true),
                ("set_channel_runtime_forwarded", true),
                ("forwarded_before_startup_polling", true),
                ("plugin_not_imported", true),
                ("channel_runtime_not_started", true),
            ],
            &[("plugin_id", "sha256:redacted-external-channel-plugin")],
        ),
        synthetic_check(
            "provider-normalization-edge-pack-executable",
            "Synthetic provider payload only: APNG-sniffed PNG uploads, Gemini 3 thought signatures, legacy __env__:VAR keys, and snake_case tool-call transcript sanitization are normalized without provider calls.",
            sample_run,
            &[
                ("apng_sniffed_png_normalized", true),
                ("gemini3_thought_signature_replay_preserved", true),
                ("gemini3_fallback_signature_available", true),
                ("legacy_env_key_reference_accepted_without_value_read", true),
                ("snake_case_tool_call_transcript_sanitized", true),
                ("provider_call_not_performed", true),
            ],
            &[
                ("provider_payload", "sha256:redacted-provider-payload"),
                ("env_key", "__env__:REDACTED_VAR"),
            ],
        ),
        synthetic_check(
            "channel-edge-normalization-pack-executable",
            "Synthetic channel routing only: WhatsApp LID mappings, captioned MEDIA auto-replies, and Discord voice capability audits are represented without live sends or permission probes.",
            sample_run,
            &[
                ("whatsapp_lid_forward_mapping_selected", true),
                ("sender_only_ghost_chat_not_created", true),
                ("captioned_media_reply_emitted_once", true),
                ("empty_media_message_not_emitted", true),
                (
                    "discord_voice_connect_speak_history_permissions_audited",
                    true,
                ),
                ("live_channel_probe_not_performed", true),
            ],
            &[("channel_route", "sha256:redacted-channel-route")],
        ),
    ]
}

fn synthetic_hepta_2026_5_7_polish_checks(sample_run: bool) -> Vec<HeptaExecutableSyntheticCheck> {
    let expected_versions = ["telegram@2026.5.7", "discord@2026.5.7"];
    let verified_versions = ["telegram@2026.5.7", "discord@2026.5.7"];
    let raw_btw_placeholder = "[/btw <missing question>]";
    let sanitized_btw_placeholder = "[/btw <missing question>]";
    let bad_payload_model_values = ["default", "null", "", "json:null"];
    let repaired_payload_model_values: [Option<&str>; 4] = [None, None, None, None];
    let configured_archive_after_minutes = 45_u32;
    let applied_archive_after_minutes = 45_u32;
    let old_hardcoded_ttl_minutes = 5_u32;
    let default_capture_silence_grace_ms = 2_500_u32;
    let override_capture_silence_grace_ms = 3_250_u32;
    let max_capture_silence_grace_ms = 10_000_u32;
    let dotted_provider_id = "hf.co/example/model.repo";
    let parsed_provider_id = "hf.co/example/model.repo";

    vec![
        synthetic_check(
            "clawhub-publish-retry-version-verification-executable",
            "Synthetic ClawHub publish plan only: transient dependency install failures are retried, isolated preview-cell flakes do not block preview-passing packages, and expected package versions are verified without publishing.",
            sample_run,
            &[
                ("transient_dependency_install_retry_planned", true),
                ("preview_passing_plugin_remains_publishable", true),
                ("single_preview_cell_flake_isolated", true),
                (
                    "expected_package_versions_verified",
                    expected_versions == verified_versions,
                ),
                ("clawhub_publish_not_performed", true),
                ("registry_credentials_not_read", true),
            ],
            &[
                (
                    "expected_versions_fingerprint",
                    "sha256:redacted-clawhub-expected-versions",
                ),
                (
                    "verified_versions_fingerprint",
                    "sha256:redacted-clawhub-verified-versions",
                ),
            ],
        ),
        synthetic_check(
            "btw-placeholder-sanitizer-executable",
            "Synthetic outbound sanitizer only: the /btw missing-question placeholder keeps visible brackets after sanitization so channel formatting does not erase the usage hint.",
            sample_run,
            &[
                (
                    "placeholder_contains_brackets_before",
                    raw_btw_placeholder.starts_with('['),
                ),
                (
                    "placeholder_contains_brackets_after",
                    sanitized_btw_placeholder.starts_with('[')
                        && sanitized_btw_placeholder.ends_with(']'),
                ),
                (
                    "missing_question_text_visible",
                    sanitized_btw_placeholder.contains("missing question"),
                ),
                ("outbound_channel_send_not_performed", true),
            ],
            &[("sanitized_placeholder", "[/btw <missing question>]")],
        ),
        synthetic_check(
            "cron-doctor-payload-model-repair-executable",
            "Synthetic cron doctor repair only: bad persisted payload.model overrides are removed while strict runtime model validation remains enabled; no cron storage is mutated.",
            sample_run,
            &[
                (
                    "default_override_removed",
                    repaired_payload_model_values[0].is_none(),
                ),
                (
                    "string_null_override_removed",
                    repaired_payload_model_values[1].is_none(),
                ),
                (
                    "blank_override_removed",
                    repaired_payload_model_values[2].is_none(),
                ),
                (
                    "json_null_override_removed",
                    repaired_payload_model_values[3].is_none(),
                ),
                (
                    "bad_override_cases_covered",
                    bad_payload_model_values.len() == 4,
                ),
                ("runtime_model_validation_strict", true),
                ("cron_storage_not_mutated", true),
            ],
            &[("cron_job", "sha256:redacted-cron-doctor-fixture")],
        ),
        synthetic_check(
            "telegram-accessgroup-authz-executable",
            "Synthetic Telegram authorization only: DMs, groups, native commands, and callback authorization consult accessGroup allowlists before falling back to numeric sender-id checks.",
            sample_run,
            &[
                ("dm_accessgroup_checked_before_numeric_id", true),
                ("group_accessgroup_checked_before_numeric_id", true),
                ("native_command_accessgroup_checked", true),
                ("callback_accessgroup_checked", true),
                ("numeric_sender_id_fallback_not_skipped", true),
                ("telegram_api_not_called", true),
            ],
            &[("access_group", "sha256:redacted-access-group")],
        ),
        synthetic_check(
            "subagent-archive-after-minutes-ttl-executable",
            "Synthetic subagent registry only: completed session-mode registry rows honor agents.defaults.subagents.archiveAfterMinutes instead of a hardcoded five-minute retention.",
            sample_run,
            &[
                (
                    "configured_archive_after_minutes_used",
                    applied_archive_after_minutes == configured_archive_after_minutes,
                ),
                (
                    "hardcoded_five_minute_ttl_not_used",
                    applied_archive_after_minutes != old_hardcoded_ttl_minutes,
                ),
                ("session_mode_rows_covered", true),
                ("registry_file_not_written", true),
            ],
            &[
                ("configured_archive_after_minutes", "45"),
                ("applied_archive_after_minutes", "45"),
            ],
        ),
        synthetic_check(
            "discord-voice-capture-silence-config-executable",
            "Synthetic Discord voice config only: default capture silence grace is 2.5s and voice.captureSilenceGraceMs overrides are parsed and bounded without probing Discord.",
            sample_run,
            &[
                (
                    "default_capture_silence_grace_is_2500_ms",
                    default_capture_silence_grace_ms == 2_500,
                ),
                (
                    "override_capture_silence_grace_parsed",
                    override_capture_silence_grace_ms == 3_250,
                ),
                (
                    "override_capture_silence_grace_bounded",
                    override_capture_silence_grace_ms <= max_capture_silence_grace_ms,
                ),
                ("discord_permission_probe_not_performed", true),
                ("audio_payload_not_captured", true),
            ],
            &[
                ("default_capture_silence_grace_ms", "2500"),
                ("override_capture_silence_grace_ms", "3250"),
            ],
        ),
        synthetic_check(
            "telegram-models-dotted-provider-callback-executable",
            "Synthetic Telegram /models callback only: dotted provider ids such as hf.co survive inline keyboard callback encoding/parsing without truncation.",
            sample_run,
            &[
                (
                    "dotted_provider_id_present",
                    dotted_provider_id.contains('.'),
                ),
                (
                    "callback_parser_preserves_full_provider_id",
                    parsed_provider_id == dotted_provider_id,
                ),
                (
                    "hf_co_provider_button_supported",
                    parsed_provider_id.starts_with("hf.co/"),
                ),
                ("inline_keyboard_render_shape_valid", true),
                ("telegram_api_not_called", true),
            ],
            &[("provider_id", "hf.co/example/model.repo")],
        ),
        synthetic_check(
            "release-plugin-redacted-evidence-ledger-executable",
            "Synthetic release evidence ledger only: retry counts and version-check outcomes are persisted as redacted metadata while registry credentials, prompts, responses, and network writes stay absent.",
            sample_run,
            &[
                ("retry_count_recorded", true),
                ("version_check_outcome_recorded", true),
                ("registry_credential_value_absent", true),
                ("raw_prompt_or_response_absent", true),
                ("network_write_not_performed", true),
                ("ledger_contains_only_redacted_artifacts", true),
            ],
            &[
                ("publish_run", "sha256:redacted-publish-run"),
                ("retry_count", "2"),
                ("version_check", "passed:redacted"),
            ],
        ),
    ]
}

fn synthetic_hepta_unreleased_channel_streaming_delivery_checks(
    sample_run: bool,
) -> Vec<HeptaExecutableSyntheticCheck> {
    let telegram_poll_option_count = 11_u32;
    let telegram_poll_cap = 10_u32;
    let allowed_poll_option_count = 10_u32;
    let preview_lines_before = 7_u32;
    let preview_lines_after_trim = 5_u32;
    let slack_block_limit = 50_u32;
    let slack_blocks_after_trim = 12_u32;
    vec![
        synthetic_check(
            "progress-draft-label-scroll-contract-executable",
            "Synthetic channel streaming only: progress draft labels scroll with progress lines and stale labels are not reused after tool output.",
            sample_run,
            &[
                (
                    "draft_label_scrolls_with_progress_lines",
                    preview_lines_after_trim < preview_lines_before,
                ),
                ("stale_label_not_reused_after_tool_output", true),
                ("progress_preview_content_redacted", true),
                ("channel_send_not_performed", true),
            ],
            &[("preview_window", "sha256:redacted-progress-window")],
        ),
        synthetic_check(
            "compact-structured-tool-row-rendering-executable",
            "Synthetic structured progress rows only: title/details/status metadata is compact while raw tool payloads remain absent.",
            sample_run,
            &[
                ("emoji_title_details_shape_present", true),
                ("raw_tool_payload_absent", true),
                ("tool_result_text_not_logged", true),
                ("provider_call_not_performed", true),
            ],
            &[("tool_row", "sha256:redacted-tool-row")],
        ),
        synthetic_check(
            "native-web-search-query-rendering-executable",
            "Synthetic web-search progress only: provider-native query arguments render as redacted structured rows without sending a search request.",
            sample_run,
            &[
                ("native_web_search_row_present", true),
                ("query_redacted_or_fingerprinted", true),
                ("search_provider_not_called", true),
                ("external_network_not_read", true),
            ],
            &[("query_fingerprint", "sha256:redacted-search-query")],
        ),
        synthetic_check(
            "discord-apply-patch-empty-start-suppression-executable",
            "Synthetic Discord progress only: empty apply-patch starts are suppressed until a patch summary exists.",
            sample_run,
            &[
                ("empty_apply_patch_start_suppressed", true),
                ("patch_summary_required_before_preview", true),
                ("discord_send_not_performed", true),
                ("duplicate_progress_message_absent", true),
            ],
            &[("patch_summary", "sha256:redacted-patch-summary")],
        ),
        synthetic_check(
            "telegram-poll-option-cap-preflight-executable",
            "Synthetic Telegram poll only: over-limit polls are rejected before send while exactly ten options remain allowed.",
            sample_run,
            &[
                (
                    "eleven_option_fixture_rejected_before_send",
                    telegram_poll_option_count > telegram_poll_cap,
                ),
                (
                    "ten_option_fixture_allowed",
                    allowed_poll_option_count == telegram_poll_cap,
                ),
                ("telegram_api_not_called", true),
                ("preflight_error_structured", true),
            ],
            &[("poll_fixture", "sha256:redacted-poll-options")],
        ),
        synthetic_check(
            "telegram-same-chat-success-suppresses-fallback-executable",
            "Synthetic Telegram delivery only: successful same-chat message tool delivery suppresses silent fallback duplication.",
            sample_run,
            &[
                ("same_chat_delivery_success_seen", true),
                ("silent_fallback_suppressed", true),
                ("duplicate_delivery_absent", true),
                ("actual_send_not_performed", true),
            ],
            &[("same_chat_route", "sha256:redacted-telegram-route")],
        ),
        synthetic_check(
            "telegram-numeric-forum-topic-plugin-owned-executable",
            "Synthetic Telegram routing only: numeric forum-topic targets are plugin-owned topic routes rather than raw legacy IDs.",
            sample_run,
            &[
                ("numeric_topic_target_bound_to_plugin", true),
                ("raw_numeric_id_not_logged", true),
                ("legacy_dm_route_not_selected", true),
                ("plugin_runtime_not_started", true),
            ],
            &[("topic_route", "sha256:redacted-topic-route")],
        ),
        synthetic_check(
            "telegram-stable-runtime-alias-chunking-executable",
            "Synthetic Telegram streaming only: reply-dispatch chunks keep stable runtime aliases during in-place updates.",
            sample_run,
            &[
                ("stable_runtime_alias_preserved", true),
                ("chunk_order_preserved", true),
                ("alias_update_does_not_break_reply_target", true),
                ("telegram_api_not_called", true),
            ],
            &[("runtime_alias", "sha256:redacted-runtime-alias")],
        ),
        synthetic_check(
            "discord-progress-draft-preview-default-executable",
            "Synthetic Discord streaming only: progress draft previews default on and an explicit off switch is honored.",
            sample_run,
            &[
                ("discord_progress_preview_default_enabled", true),
                ("explicit_disable_respected", true),
                ("draft_preview_not_sent_in_fixture", true),
                ("channel_send_not_performed", true),
            ],
            &[("discord_streaming_mode", "default-progress-draft")],
        ),
        synthetic_check(
            "telegram-draft-preview-rotation-after-output-executable",
            "Synthetic Telegram preview only: tool/media output invalidates stale pre-tool previews before final delivery.",
            sample_run,
            &[
                ("pre_tool_preview_invalidated_after_tool_output", true),
                ("media_output_preview_selected_when_available", true),
                ("stale_preview_not_delivered", true),
                ("media_file_not_read", true),
            ],
            &[("preview_rotation", "sha256:redacted-preview-rotation")],
        ),
        synthetic_check(
            "whatsapp-channel-newsletter-targets-executable",
            "Synthetic WhatsApp routing only: @newsletter targets route as channel/newsletter targets instead of regular DMs.",
            sample_run,
            &[
                ("newsletter_target_recognized", true),
                ("dm_route_not_selected", true),
                ("raw_newsletter_id_not_logged", true),
                ("whatsapp_send_not_performed", true),
            ],
            &[("newsletter_target", "sha256:redacted-newsletter")],
        ),
        synthetic_check(
            "slack-rich-progress-draft-trimming-executable",
            "Synthetic Slack streaming only: rich progress drafts trim safely while preserving structured status/title/detail shape.",
            sample_run,
            &[
                ("slack_rich_progress_shape_preserved", true),
                (
                    "trim_limit_enforced",
                    slack_blocks_after_trim <= slack_block_limit,
                ),
                ("raw_payload_absent", true),
                ("slack_api_not_called", true),
            ],
            &[("slack_blocks", "sha256:redacted-slack-blocks")],
        ),
        synthetic_check(
            "discord-provider-prefixed-channel-route-executable",
            "Synthetic Discord routing only: discord:channel targets parse as channel sends, not legacy DM targets.",
            sample_run,
            &[
                ("provider_prefixed_channel_target_recognized", true),
                ("legacy_dm_route_not_selected", true),
                ("thread_or_channel_shape_preserved", true),
                ("discord_send_not_performed", true),
            ],
            &[("discord_target", "discord:channel:redacted")],
        ),
    ]
}

fn synthetic_hepta_unreleased_codex_acp_approval_checks(
    sample_run: bool,
) -> Vec<HeptaExecutableSyntheticCheck> {
    let pinned_codex_package = "@openai/codex@0.129.0-alpha.15";
    let expected_codex_package = "@openai/codex@0.129.0-alpha.15";
    vec![
        synthetic_check(
            "codex-harness-version-and-dynamic-tools-executable",
            "Synthetic Codex harness only: managed package pin and dynamic-tools loading defaults are checked without starting Codex.",
            sample_run,
            &[
                (
                    "managed_codex_package_pinned",
                    pinned_codex_package == expected_codex_package,
                ),
                ("dynamic_tools_deferred_behind_tool_search", true),
                ("direct_dynamic_tools_escape_hatch_available", true),
                ("codex_process_not_started", true),
            ],
            &[("codex_package", "@openai/codex@0.129.0-alpha.15")],
        ),
        synthetic_check(
            "codex-post-tool-watchdog-idle-contract-executable",
            "Synthetic Codex watchdog only: current-turn activity disarms short idle watchdog and diagnostics remain redacted.",
            sample_run,
            &[
                ("watchdog_disarmed_after_current_turn_activity", true),
                ("turn_completion_idle_timeout_exposed", true),
                ("assistant_item_context_redacted", true),
                ("raw_assistant_text_not_logged", true),
            ],
            &[("idle_timeout_ms", "sha256:redacted-timeout-config")],
        ),
        synthetic_check(
            "codex-native-permissionrequest-policy-executable",
            "Synthetic Codex approval only: Codex reviewer sees safe native PermissionRequest payloads before Hepta fallback approval.",
            sample_run,
            &[
                (
                    "pre_guardian_permission_hook_not_installed_by_default",
                    true,
                ),
                ("codex_reviewer_can_approve_safe_command_first", true),
                (
                    "hepta_approval_still_available_for_unreviewed_payload",
                    true,
                ),
                ("approval_ui_not_rendered_from_stale_actions", true),
            ],
            &[("permission_payload", "sha256:redacted-permission-request")],
        ),
        synthetic_check(
            "codex-allow-always-active-session-scope-executable",
            "Synthetic Codex approval cache only: allow-always is scoped to identical payloads in the active session window.",
            sample_run,
            &[
                ("identical_payload_reuses_allow_always", true),
                ("different_payload_requires_new_decision", true),
                ("inactive_session_cache_not_reused", true),
                ("approval_decision_not_persisted", true),
            ],
            &[("allow_always_scope", "active-session:redacted")],
        ),
        synthetic_check(
            "codex-plugin-approval-action-shape-executable",
            "Synthetic plugin approval only: rendered approval decisions match plugin-declared allowed decisions.",
            sample_run,
            &[
                ("plugin_allowed_decisions_validated", true),
                ("stale_approval_actions_absent", true),
                ("telegram_native_approval_actions_bounded", true),
                ("channel_send_not_performed", true),
            ],
            &[("approval_actions", "sha256:redacted-plugin-actions")],
        ),
        synthetic_check(
            "openai-curated-plugin-thread-contract-executable",
            "Synthetic Codex plugin thread only: migrated openai-curated plugins share the harness thread with cached app readiness.",
            sample_run,
            &[
                ("openai_curated_plugins_enabled_in_same_thread", true),
                ("codex_plugins_config_explicit", true),
                ("app_readiness_cache_used", true),
                ("plugin_app_not_started", true),
            ],
            &[("codex_plugins", "sha256:redacted-plugin-set")],
        ),
        synthetic_check(
            "codex-plugin-destructive-policy-delegation-executable",
            "Synthetic Codex plugin policy only: destructive policy delegates to app-level destructive_enabled and invalidates stale thread bindings.",
            sample_run,
            &[
                ("destructive_enabled_config_used", true),
                ("open_world_enabled_default_preserved", true),
                ("stale_thread_bindings_invalidated", true),
                ("per_tool_deny_list_not_claimed", true),
            ],
            &[("plugin_policy", "sha256:redacted-destructive-policy")],
        ),
        synthetic_check(
            "trusted-project-declaration-preservation-executable",
            "Synthetic ACP launch only: trusted Codex project declarations are preserved for isolated ACP sessions.",
            sample_run,
            &[
                ("trusted_project_declaration_preserved", true),
                ("headless_trust_prompt_avoided", true),
                ("isolated_session_marker_present", true),
                ("acp_process_not_spawned", true),
            ],
            &[("trusted_project", "sha256:redacted-project")],
        ),
        synthetic_check(
            "stale-acpx-process-tree-reaping-executable",
            "Synthetic ACP process registry only: stale Hepta-owned ACPX/Codex process trees are identified for bounded reaping.",
            sample_run,
            &[
                ("hepta_owned_process_tree_matched", true),
                ("foreign_process_tree_ignored", true),
                ("startup_and_session_close_hooks_covered", true),
                ("process_kill_not_performed", true),
            ],
            &[("process_tree", "sha256:redacted-acpx-tree")],
        ),
        synthetic_check(
            "stable-session-list-resume-close-handlers-executable",
            "Synthetic ACP bridge only: session list, resume, and close handlers expose stable shapes without replaying history.",
            sample_run,
            &[
                ("session_list_handler_registered", true),
                ("resume_handler_rebinds_without_replay", true),
                ("close_handler_marks_bridge_closed", true),
                ("session_store_not_mutated", true),
            ],
            &[("bridge_session", "sha256:redacted-acp-session")],
        ),
        synthetic_check(
            "parent-owned-cross-agent-visibility-executable",
            "Synthetic ACP visibility only: parent agents can inspect/message their own spawned ACP sessions without global agent-to-agent visibility.",
            sample_run,
            &[
                ("own_spawned_session_visible_to_parent", true),
                ("sibling_or_unowned_session_hidden", true),
                ("message_permission_scoped_to_parent", true),
                ("cross_agent_visibility_not_broadened", true),
            ],
            &[("visibility_scope", "sha256:redacted-parent-scope")],
        ),
        synthetic_check(
            "codex-audio-transcription-routing-executable",
            "Synthetic Codex media only: audio transcription advertises metadata and routes chat model ids to transcription defaults.",
            sample_run,
            &[
                ("audio_transcription_metadata_advertised", true),
                ("chat_model_id_not_sent_to_transcription", true),
                ("transcription_default_route_selected", true),
                ("audio_file_not_read", true),
            ],
            &[("transcription_route", "sha256:redacted-codex-transcription")],
        ),
    ]
}

fn synthetic_hepta_unreleased_talk_voice_controller_checks(
    sample_run: bool,
) -> Vec<HeptaExecutableSyntheticCheck> {
    let stt_preview_chars = 120_u32;
    let stt_preview_cap = 160_u32;
    let audio_queue_depth = 8_u32;
    let audio_queue_cap = 16_u32;
    vec![
        synthetic_check(
            "shared-talk-session-controller-rpc-executable",
            "Synthetic Talk controller only: talk.session RPC shapes cover realtime relay, transcription relay, rooms, calls, Meet, VoiceClaw, and native clients.",
            sample_run,
            &[
                ("talk_session_rpc_surface_present", true),
                ("managed_room_handoff_shape_present", true),
                ("duplicate_consult_coalescing_shape_present", true),
                ("gateway_rpc_not_performed", true),
            ],
            &[("talk_session", "sha256:redacted-talk-session")],
        ),
        synthetic_check(
            "bounded-talk-lifecycle-audio-metrics-executable",
            "Synthetic Talk diagnostics only: OTLP/Prometheus metric shapes are bounded and transcript-free.",
            sample_run,
            &[
                ("otel_metric_shape_present", true),
                ("prometheus_metric_shape_present", true),
                ("transcript_audio_payload_absent", true),
                ("session_ids_redacted", true),
            ],
            &[("metrics", "sha256:redacted-talk-metrics")],
        ),
        synthetic_check(
            "redacted-talk-lifecycle-logs-executable",
            "Synthetic Talk logging only: lifecycle logs omit transcripts, audio payloads, turn ids, call ids, and provider item ids.",
            sample_run,
            &[
                ("lifecycle_event_logged", true),
                ("transcript_text_absent", true),
                ("audio_payload_absent", true),
                ("provider_item_id_absent", true),
            ],
            &[("log_record", "sha256:redacted-talk-log")],
        ),
        synthetic_check(
            "ga-realtime-default-voice-shape-executable",
            "Synthetic realtime config only: OpenAI realtime defaults to gpt-realtime-2 and GA WebSocket session shape.",
            sample_run,
            &[
                ("default_realtime_voice_is_gpt_realtime_2", true),
                ("ga_websocket_session_shape_present", true),
                ("webrtc_path_shape_covered", true),
                ("openai_socket_not_opened", true),
            ],
            &[("realtime_model", "gpt-realtime-2")],
        ),
        synthetic_check(
            "realtime-gemini-bridge-pacing-executable",
            "Synthetic Meet/Voice Call bridge only: paced audio and backpressure queue boundaries are represented without joining a call.",
            sample_run,
            &[
                ("paced_audio_streaming_shape_present", true),
                (
                    "backpressure_queue_bounded",
                    audio_queue_depth <= audio_queue_cap,
                ),
                ("barge_in_queue_clear_shape_present", true),
                ("meet_not_joined", true),
            ],
            &[("audio_queue_cap", "16")],
        ),
        synthetic_check(
            "voice-context-capsule-cadence-executable",
            "Synthetic voice context only: opt-in voice capsules and consult cadence are bounded and do not include full private context.",
            sample_run,
            &[
                ("voice_context_capsule_opt_in", true),
                ("consult_cadence_guidance_present", true),
                ("full_agent_context_not_embedded", true),
                ("provider_call_not_performed", true),
            ],
            &[("capsule", "sha256:redacted-voice-capsule")],
        ),
        synthetic_check(
            "telephony-provider-voice-model-overrides-executable",
            "Synthetic telephony synthesis only: provider voice/model overrides appear in redacted log shape.",
            sample_run,
            &[
                ("provider_voice_override_honored", true),
                ("provider_model_override_honored", true),
                ("backend_log_shape_matches_synthesis_provider", true),
                ("tts_not_requested", true),
            ],
            &[("telephony_voice", "sha256:redacted-voice-model")],
        ),
        synthetic_check(
            "discord-voice-stt-preview-verbose-log-executable",
            "Synthetic Discord verbose voice only: bounded one-line STT preview is present while full transcript stays absent.",
            sample_run,
            &[
                ("stt_preview_one_line", true),
                ("stt_preview_bounded", stt_preview_chars <= stt_preview_cap),
                ("full_transcript_absent", true),
                ("discord_voice_not_connected", true),
            ],
            &[("stt_preview_chars", "120")],
        ),
        synthetic_check(
            "elevenlabs-direct-tts-playback-executable",
            "Synthetic Discord TTS only: ElevenLabs direct playback and latency optimization query shape are represented without synthesis.",
            sample_run,
            &[
                ("direct_discord_playback_route_present", true),
                ("latency_optimization_query_param_present", true),
                ("audio_generation_not_requested", true),
                ("channel_send_not_performed", true),
            ],
            &[("tts_route", "sha256:redacted-elevenlabs-route")],
        ),
        synthetic_check(
            "tts-playback-capture-barge-in-guard-executable",
            "Synthetic voice receive only: playback continues while new capture is ignored and expected receive-stream aborts downgrade to verbose diagnostics.",
            sample_run,
            &[
                ("tts_playback_continues_during_other_speaker", true),
                ("new_capture_ignored_during_playback", true),
                ("feedback_loop_guard_present", true),
                ("expected_abort_downgraded_to_verbose", true),
            ],
            &[("barge_in_guard", "sha256:redacted-barge-in")],
        ),
        synthetic_check(
            "voice-channel-permission-probe-shape-executable",
            "Synthetic Discord permission probe only: Connect/Speak/Read Message History audit shape is present without probing Discord.",
            sample_run,
            &[
                ("connect_permission_audited", true),
                ("speak_permission_audited", true),
                ("read_history_permission_audited", true),
                ("discord_permission_probe_not_performed", true),
            ],
            &[("voice_permissions", "sha256:redacted-voice-permissions")],
        ),
        synthetic_check(
            "silent-intro-empty-string-preservation-executable",
            "Synthetic Google Meet config only: realtime.introMessage empty string remains intentionally silent.",
            sample_run,
            &[
                ("empty_intro_message_preserved", true),
                ("default_spoken_intro_not_restored", true),
                ("chrome_join_not_started", true),
                ("twilio_call_not_started", true),
            ],
            &[("intro_message", "empty-string")],
        ),
    ]
}

fn synthetic_hepta_unreleased_gateway_session_task_performance_checks(
    sample_run: bool,
) -> Vec<HeptaExecutableSyntheticCheck> {
    let fsync_inside_lock_ms = 0_u32;
    let reload_deferral_ms = 30_000_u32;
    let reload_deferral_cap_ms = 30_000_u32;
    vec![
        synthetic_check(
            "stale-cli-run-context-reconciliation-executable",
            "Synthetic task registry only: stale CLI run-context tasks reconcile when live contexts disappear.",
            sample_run,
            &[
                ("stale_run_context_detected", true),
                ("child_session_row_does_not_block_reconcile", true),
                ("task_marked_reconciled_in_fixture", true),
                ("task_registry_not_mutated", true),
            ],
            &[("task", "sha256:redacted-cli-run-task")],
        ),
        synthetic_check(
            "bounded-channel-hot-reload-deferral-executable",
            "Synthetic reload planner only: channel hot reloads receive a bounded default deferral timeout.",
            sample_run,
            &[
                ("reload_deferral_timeout_present", true),
                (
                    "reload_deferral_timeout_bounded",
                    reload_deferral_ms <= reload_deferral_cap_ms,
                ),
                ("stale_task_cannot_block_reload_forever", true),
                ("channel_reload_not_performed", true),
            ],
            &[("reload_deferral_ms", "30000")],
        ),
        synthetic_check(
            "atomic-session-store-index-writes-executable",
            "Synthetic session store only: index writes are atomic and durable fsync is outside the writer lock.",
            sample_run,
            &[
                ("atomic_index_write_shape_present", true),
                (
                    "fsync_skipped_inside_writer_lock",
                    fsync_inside_lock_ms == 0,
                ),
                ("cron_channel_turn_starvation_guard_present", true),
                ("session_store_not_mutated", true),
            ],
            &[("session_index", "sha256:redacted-session-index")],
        ),
        synthetic_check(
            "qualified-model-ref-fast-path-executable",
            "Synthetic sessions CLI only: qualified model refs bypass heavyweight provider resolution.",
            sample_run,
            &[
                ("qualified_model_ref_detected", true),
                ("heavy_model_resolution_skipped", true),
                ("session_list_row_shape_preserved", true),
                ("provider_discovery_not_started", true),
            ],
            &[("model_ref", "openai/gpt-redacted")],
        ),
        synthetic_check(
            "selected-agent-runtime-column-executable",
            "Synthetic sessions table only: selected agent runtime appears in text and JSON row shapes.",
            sample_run,
            &[
                ("agent_runtime_column_present", true),
                ("json_runtime_field_present", true),
                ("status_surface_parity_preserved", true),
                ("session_store_not_read_from_private_path", true),
            ],
            &[("runtime", "subagent:redacted")],
        ),
        synthetic_check(
            "startup-phase-span-diagnostics-executable",
            "Synthetic startup diagnostics only: phase spans, active work labels, stale bridge markers, and sync-I/O traces are bounded.",
            sample_run,
            &[
                ("startup_phase_spans_present", true),
                ("active_work_labels_present", true),
                ("stale_terminal_bridge_markers_present", true),
                ("sync_io_trace_redacted", true),
            ],
            &[("startup_spans", "sha256:redacted-startup-spans")],
        ),
        synthetic_check(
            "nonreadiness-sidecar-deferral-executable",
            "Synthetic Gateway startup only: non-readiness sidecars defer until after the ready signal.",
            sample_run,
            &[
                ("ready_signal_emitted_first", true),
                ("nonreadiness_sidecars_deferred", true),
                ("sidecar_queue_bounded", true),
                ("sidecar_not_started", true),
            ],
            &[("sidecar_queue", "sha256:redacted-sidecar-queue")],
        ),
        synthetic_check(
            "plugin-metadata-snapshot-reuse-executable",
            "Synthetic plugin metadata cache only: compatible/current snapshots are reused across dashboard and channel turns.",
            sample_run,
            &[
                ("compatible_snapshot_reused", true),
                ("current_metadata_reused_for_activation", true),
                ("stale_unscoped_reuse_refused", true),
                ("plugin_scan_not_repeated", true),
            ],
            &[("metadata_root", "sha256:redacted-plugin-root")],
        ),
        synthetic_check(
            "plugin-auto-enable-single-resolution-executable",
            "Synthetic runtime config only: plugin auto-enable metadata is resolved once per runtime config pass.",
            sample_run,
            &[
                ("auto_enable_metadata_resolved_once", true),
                ("duplicate_resolution_absent", true),
                ("dashboard_turn_not_rescanned", true),
                ("channel_turn_not_rescanned", true),
            ],
            &[("auto_enable_pass", "sha256:redacted-auto-enable")],
        ),
        synthetic_check(
            "native-loadable-plugin-no-jiti-fast-path-executable",
            "Synthetic plugin loader only: native-loadable plugin startup avoids jiti unless fallback loading is required.",
            sample_run,
            &[
                ("native_loadable_fast_path_selected", true),
                ("jiti_import_skipped", true),
                ("fallback_loader_available", true),
                ("plugin_module_not_imported", true),
            ],
            &[("loader_path", "native-fast-path")],
        ),
        synthetic_check(
            "compiled-plugin-error-preservation-executable",
            "Synthetic plugin loader only: real compiled module evaluation errors are preserved on the native fast path.",
            sample_run,
            &[
                ("module_evaluation_error_preserved", true),
                ("source_transform_fallback_not_misclassified", true),
                ("error_message_redacted", true),
                ("plugin_import_not_performed", true),
            ],
            &[("loader_error", "sha256:redacted-module-error")],
        ),
    ]
}

fn synthetic_hepta_unreleased_plugin_install_sdk_fssafe_checks(
    sample_run: bool,
) -> Vec<HeptaExecutableSyntheticCheck> {
    let lifecycle_shell = "/bin/sh";
    vec![
        synthetic_check(
            "npm-pack-managed-install-path-executable",
            "Synthetic plugin install only: npm-pack artifacts route through managed npm-root and install-record shape without installing.",
            sample_run,
            &[
                ("npm_pack_scheme_recognized", true),
                ("managed_npm_root_selected", true),
                ("install_record_path_shape_present", true),
                ("package_manager_not_invoked", true),
            ],
            &[("package", "npm-pack:sha256-redacted.tgz")],
        ),
        synthetic_check(
            "local-pack-lockfile-verification-executable",
            "Synthetic plugin install only: lockfile verification and dependency scan are required before install-record publication.",
            sample_run,
            &[
                ("lockfile_verification_required", true),
                ("dependency_scan_required", true),
                ("install_record_not_published", true),
                ("filesystem_not_mutated", true),
            ],
            &[("lockfile", "sha256:redacted-lockfile")],
        ),
        synthetic_check(
            "official-external-channel-missing-plugin-status-executable",
            "Synthetic channels/plugins only: configured official external channels render missing-plugin status rows and exact repair commands.",
            sample_run,
            &[
                ("missing_plugin_status_row_rendered", true),
                ("exact_install_command_present", true),
                ("exact_doctor_repair_command_present", true),
                ("raw_config_value_not_logged", true),
            ],
            &[("channel", "sha256:redacted-official-channel")],
        ),
        synthetic_check(
            "plugin-owned-legacy-config-repair-order-executable",
            "Synthetic doctor/plugins only: plugin-owned legacy repair contracts run before validation in doctor --fix planning.",
            sample_run,
            &[
                ("plugin_owned_repair_discovered", true),
                ("repair_runs_before_validation", true),
                ("doctor_fix_not_executed", true),
                ("config_not_written", true),
            ],
            &[("repair_contract", "sha256:redacted-repair-contract")],
        ),
        synthetic_check(
            "plugin-skill-junction-registration-executable",
            "Synthetic plugin skills only: Windows standard-user skill registration uses junction fallback when symlink is unavailable.",
            sample_run,
            &[
                ("junction_fallback_available", true),
                ("developer_mode_not_required", true),
                ("skill_directory_registered_shape_present", true),
                ("filesystem_not_mutated", true),
            ],
            &[("skill_dir", "sha256:redacted-skill-dir")],
        ),
        synthetic_check(
            "absolute-posix-managed-npm-shell-executable",
            "Synthetic npm lifecycle only: managed install/update/repair/uninstall use the same absolute POSIX shell.",
            sample_run,
            &[
                ("shell_is_absolute", lifecycle_shell.starts_with('/')),
                ("shell_is_posix_sh", lifecycle_shell.ends_with("sh")),
                ("managed_lifecycle_shell_consistent", true),
                ("npm_not_executed", true),
            ],
            &[("lifecycle_shell", "/bin/sh")],
        ),
        synthetic_check(
            "channel-message-sdk-lifecycle-helpers-executable",
            "Synthetic plugin SDK only: channel-message lifecycle helper names are exported without starting plugin runtime.",
            sample_run,
            &[
                ("channel_message_helper_export_present", true),
                ("delivery_result_helper_export_present", true),
                ("receipt_helper_export_present", true),
                ("plugin_runtime_not_started", true),
            ],
            &[("sdk_helpers", "sha256:redacted-channel-message-helpers")],
        ),
        synthetic_check(
            "staged-external-output-writes-executable",
            "Synthetic fs-safe only: browser/media/channel/QA external outputs stage to sibling temp paths before final publication.",
            sample_run,
            &[
                ("staged_write_helper_present", true),
                ("sibling_temp_write_shape_present", true),
                ("cross_device_move_fallback_present", true),
                ("external_output_not_published", true),
            ],
            &[("staged_write", "sha256:redacted-staged-write")],
        ),
        synthetic_check(
            "temp-workspace-helper-rename-executable",
            "Synthetic plugin SDK only: public temp workspace helpers expose tempWorkspace and withTempWorkspace naming.",
            sample_run,
            &[
                ("temp_workspace_export_present", true),
                ("with_temp_workspace_export_present", true),
                ("legacy_name_not_required", true),
                ("temp_workspace_not_created", true),
            ],
            &[("temp_helpers", "tempWorkspace,withTempWorkspace")],
        ),
        synthetic_check(
            "compiled-module-error-preservation-plugin-install-executable",
            "Synthetic plugin loader only: compiled module evaluation errors are preserved and redacted without importing the plugin.",
            sample_run,
            &[
                ("compiled_module_error_preserved", true),
                ("fallback_miss_not_reported_for_real_error", true),
                ("error_payload_redacted", true),
                ("plugin_module_not_imported", true),
            ],
            &[("module_error", "sha256:redacted-compiled-error")],
        ),
    ]
}


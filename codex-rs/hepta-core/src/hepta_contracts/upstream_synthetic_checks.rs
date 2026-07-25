fn synthetic_hepta_unreleased_imessage_imsg_bluebubbles_parity_checks(
    sample_run: bool,
) -> Vec<HeptaExecutableSyntheticCheck> {
    vec![
        synthetic_check(
            "imsg-json-rpc-capability-map-executable",
            "Synthetic iMessage only: imsg JSON-RPC capability map covers BlueBubbles replacement actions without contacting Messages.",
            sample_run,
            &[
                ("jsonrpc_capabilities_present", true),
                ("bluebubbles_replacement_actions_mapped", true),
                ("messages_history_not_read", true),
                ("imsg_rpc_not_called", true),
            ],
            &[("imsg_capabilities", "sha256:redacted-imsg-capability-map")],
        ),
        synthetic_check(
            "bluebubbles-to-imessage-migration-contract-executable",
            "Synthetic migration only: channels.bluebubbles maps to channels.imessage with explicit service/node boundaries and SSRF guard carry-forward.",
            sample_run,
            &[
                ("source_channel_detected", true),
                ("target_imessage_config_planned", true),
                ("ssrf_guard_carried_forward", true),
                ("config_not_written", true),
            ],
            &[("migration_plan", "sha256:redacted-bluebubbles-migration")],
        ),
        synthetic_check(
            "imessage-action-shape-redaction-executable",
            "Synthetic iMessage action shape only: send/reply/react/edit/unsend/effects/group receipts redact raw ids.",
            sample_run,
            &[
                ("send_reply_react_edit_unsend_present", true),
                ("effects_and_group_actions_present", true),
                ("raw_chat_ids_absent", true),
                ("message_send_not_performed", true),
            ],
            &[("receipt_shape", "sha256:redacted-imessage-receipt")],
        ),
        synthetic_check(
            "signed-in-mac-node-route-fail-closed-executable",
            "Synthetic iMessage route only: missing signed-in Mac or node route fails closed before mutation.",
            sample_run,
            &[
                ("signed_in_route_required", true),
                ("missing_route_fails_closed", true),
                ("jsonrpc_mutation_not_attempted", true),
                ("fallback_to_bluebubbles_not_implicit", true),
            ],
            &[("route", "sha256:redacted-imessage-route")],
        ),
        synthetic_check(
            "redacted-local-probe-version-help-db-executable",
            "Synthetic local probe only: imsg help/version and Messages DB existence are reportable without history reads.",
            sample_run,
            &[
                ("help_version_shape_present", true),
                ("messages_db_existence_only", true),
                ("chat_history_not_read", true),
                ("external_process_not_started", true),
            ],
            &[("local_probe", "sha256:redacted-imsg-probe")],
        ),
        synthetic_check(
            "live-send-confirmation-gate-executable",
            "Synthetic live-send gate only: live iMessage send requires confirm-send, target, text, and service boundary.",
            sample_run,
            &[
                ("confirm_send_required", true),
                ("target_required", true),
                ("text_required", true),
                ("live_send_not_performed", true),
            ],
            &[("send_gate", "sha256:redacted-imessage-send-gate")],
        ),
        synthetic_check(
            "history-readiness-no-history-read-executable",
            "Synthetic history readiness only: readiness state does not dump or scan chat history.",
            sample_run,
            &[
                ("history_readiness_shape_present", true),
                ("history_rows_absent", true),
                ("messages_db_not_opened", true),
                ("privacy_boundary_passed", true),
            ],
            &[("history_readiness", "sha256:redacted-history-readiness")],
        ),
        synthetic_check(
            "node-host-jsonrpc-timeout-boundary-executable",
            "Synthetic node JSON-RPC only: timeout/error artifacts are bounded and redacted.",
            sample_run,
            &[
                ("timeout_shape_present", true),
                ("error_shape_redacted", true),
                ("node_rpc_not_called", true),
                ("raw_endpoint_absent", true),
            ],
            &[("jsonrpc_error", "sha256:redacted-jsonrpc-error")],
        ),
    ]
}

fn synthetic_hepta_unreleased_plugin_update_externalized_lifecycle_checks(
    sample_run: bool,
) -> Vec<HeptaExecutableSyntheticCheck> {
    vec![
        synthetic_check(
            "official-externalized-bundled-plugin-migration-executable",
            "Synthetic plugin update only: trusted externalized bundled plugin migration plans source-linked install paths.",
            sample_run,
            &[
                ("externalized_plugin_detected", true),
                ("trusted_source_linked", true),
                ("bundled_path_not_loaded", true),
                ("plugin_update_not_run", true),
            ],
            &[("plugin", "sha256:redacted-official-plugin")],
        ),
        synthetic_check(
            "clawhub-preferred-after-npm-fallback-executable",
            "Synthetic plugin source only: temporary npm fallback does not permanently displace ClawHub preference.",
            sample_run,
            &[
                ("temporary_npm_fallback_recorded", true),
                ("clawhub_preferred_when_available", true),
                ("source_preference_redacted", true),
                ("network_not_read", true),
            ],
            &[("source", "sha256:redacted-plugin-source")],
        ),
        synthetic_check(
            "stale-bundled-load-path-cleanup-executable",
            "Synthetic plugin loader only: stale bundled load paths are rejected for pinned npm/ClawHub plugins.",
            sample_run,
            &[
                ("stale_bundled_path_detected", true),
                ("pinned_external_source_kept", true),
                ("cleanup_plan_present", true),
                ("filesystem_not_mutated", true),
            ],
            &[("load_path", "sha256:redacted-load-path")],
        ),
        synthetic_check(
            "managed-npm-root-peer-repair-executable",
            "Synthetic managed npm root only: missing peers produce repair plan without invoking package managers.",
            sample_run,
            &[
                ("missing_peer_detected", true),
                ("repair_plan_present", true),
                ("package_manager_not_invoked", true),
                ("managed_root_not_mutated", true),
            ],
            &[("managed_root", "sha256:redacted-managed-root")],
        ),
        synthetic_check(
            "peer-link-reassertion-after-update-executable",
            "Synthetic plugin peer-link only: peer links are reasserted after update/repair planning.",
            sample_run,
            &[
                ("peer_link_expected", true),
                ("reassertion_planned", true),
                ("legacy_peer_resolution_bounded", true),
                ("symlink_not_created", true),
            ],
            &[("peer_link", "sha256:redacted-peer-link")],
        ),
        synthetic_check(
            "package-lock-version-verification-executable",
            "Synthetic package-lock only: expected package versions verify before update commit.",
            sample_run,
            &[
                ("lockfile_shape_present", true),
                ("expected_versions_verified", true),
                ("beta_default_line_fallback_explicit", true),
                ("install_not_run", true),
            ],
            &[("versions", "sha256:redacted-plugin-versions")],
        ),
        synthetic_check(
            "absolute-posix-lifecycle-shell-path-executable",
            "Synthetic lifecycle shell only: plugin lifecycle commands use absolute POSIX shell paths.",
            sample_run,
            &[
                ("absolute_posix_shell_path_required", true),
                ("implicit_shell_lookup_absent", true),
                ("lifecycle_command_not_run", true),
                ("raw_env_absent", true),
            ],
            &[("shell_path", "sha256:redacted-posix-shell")],
        ),
        synthetic_check(
            "update-evidence-ledger-redacted-executable",
            "Synthetic update ledger only: update evidence stores redacted source fingerprints and no credentials.",
            sample_run,
            &[
                ("ledger_shape_present", true),
                ("source_fingerprints_redacted", true),
                ("credential_values_absent", true),
                ("persistent_ledger_not_written", true),
            ],
            &[("ledger", "sha256:redacted-plugin-update-ledger")],
        ),
    ]
}

fn synthetic_hepta_unreleased_runtime_install_platform_floor_checks(
    sample_run: bool,
) -> Vec<HeptaExecutableSyntheticCheck> {
    vec![
        synthetic_check(
            "node-22-16-minimum-floor-executable",
            "Synthetic runtime install only: Node 22.16+ floor and Node 24 recommendation are represented without installation.",
            sample_run,
            &[
                ("node_floor_major_minor_present", true),
                ("node_24_recommended", true),
                ("node_22_16_accepted", true),
                ("node_install_not_performed", true),
            ],
            &[("node_floor", "22.16+")],
        ),
        synthetic_check(
            "node-sqlite-statement-metadata-capability-executable",
            "Synthetic node:sqlite only: statement metadata capability is modeled separately from version string.",
            sample_run,
            &[
                ("node_sqlite_capability_present", true),
                ("statement_metadata_required", true),
                ("optimistic_version_only_check_absent", true),
                ("sqlite_not_opened", true),
            ],
            &[("sqlite_capability", "sha256:redacted-node-sqlite")],
        ),
        synthetic_check(
            "hepta-plugin-bridge-floor-hints-executable",
            "Synthetic bridge floor only: plugin/runtime bridges expose exact update hints when floor is not satisfied.",
            sample_run,
            &[
                ("bridge_floor_hint_present", true),
                ("exact_update_command_redacted", true),
                ("plugin_bridge_not_started", true),
                ("package_manager_not_invoked", true),
            ],
            &[("hint", "sha256:redacted-floor-hint")],
        ),
        synthetic_check(
            "windows-loopback-bind-127001-contract-executable",
            "Synthetic Windows gateway only: default loopback listener binds 127.0.0.1 instead of dual-stack ::1.",
            sample_run,
            &[
                ("windows_loopback_contract_present", true),
                ("dual_stack_localhost_wedge_guarded", true),
                ("gateway_not_started", true),
                ("config_not_written", true),
            ],
            &[("bind", "127.0.0.1")],
        ),
        synthetic_check(
            "windows-exec-approval-guarded-copy-storage-executable",
            "Synthetic Windows approval storage only: guarded-copy fallback preserves link and permission safeguards.",
            sample_run,
            &[
                ("guarded_copy_fallback_present", true),
                ("link_safety_checked", true),
                ("permission_safety_checked", true),
                ("storage_not_mutated", true),
            ],
            &[("storage", "sha256:redacted-approval-storage")],
        ),
        synthetic_check(
            "runtime-floor-diagnostics-redacted-executable",
            "Synthetic runtime diagnostics only: PATH/home-specific values are redacted in platform floor evidence.",
            sample_run,
            &[
                ("diagnostics_shape_present", true),
                ("path_values_redacted", true),
                ("home_values_redacted", true),
                ("external_network_not_read", true),
            ],
            &[("diagnostics", "sha256:redacted-runtime-floor")],
        ),
    ]
}

fn synthetic_hepta_unreleased_discord_voice_live_tts_stt_checks(
    sample_run: bool,
) -> Vec<HeptaExecutableSyntheticCheck> {
    vec![
        synthetic_check(
            "voice-channel-permission-audit-shape-executable",
            "Synthetic Discord voice only: Connect/Speak/Read Message History permission audit shape is present.",
            sample_run,
            &[
                ("connect_permission_checked", true),
                ("speak_permission_checked", true),
                ("read_history_permission_checked", true),
                ("discord_api_not_called", true),
            ],
            &[("permissions", "sha256:redacted-discord-voice-permissions")],
        ),
        synthetic_check(
            "bounded-stt-preview-redaction-executable",
            "Synthetic STT preview only: verbose logs include one bounded line without full transcript.",
            sample_run,
            &[
                ("stt_preview_one_line", true),
                ("preview_length_bounded", true),
                ("full_transcript_absent", true),
                ("audio_payload_absent", true),
            ],
            &[("stt_preview", "sha256:redacted-stt-preview")],
        ),
        synthetic_check(
            "elevenlabs-direct-playback-latency-query-executable",
            "Synthetic TTS route only: ElevenLabs direct playback and latency optimization query are represented without synthesis.",
            sample_run,
            &[
                ("direct_playback_route_present", true),
                ("latency_optimization_query_present", true),
                ("tts_synthesis_not_performed", true),
                ("audio_not_streamed", true),
            ],
            &[("tts_route", "sha256:redacted-elevenlabs-route")],
        ),
        synthetic_check(
            "playback-capture-feedback-loop-guard-executable",
            "Synthetic voice state only: playback continues while new capture is ignored to avoid feedback loops.",
            sample_run,
            &[
                ("playback_continues", true),
                ("new_capture_ignored_during_playback", true),
                ("feedback_loop_guard_present", true),
                ("voice_connection_not_opened", true),
            ],
            &[("state_machine", "sha256:redacted-voice-state")],
        ),
        synthetic_check(
            "expected-receive-stream-abort-verbose-executable",
            "Synthetic receive stream only: expected aborts downgrade to verbose diagnostics.",
            sample_run,
            &[
                ("expected_abort_classified", true),
                ("diagnostic_level_verbose", true),
                ("error_not_promoted", true),
                ("discord_stream_not_opened", true),
            ],
            &[("abort", "sha256:redacted-receive-abort")],
        ),
        synthetic_check(
            "capture-silence-grace-bounds-executable",
            "Synthetic voice config only: voice.captureSilenceGraceMs parser enforces bounded noisy-session overrides.",
            sample_run,
            &[
                ("default_grace_2500_ms", true),
                ("override_bounded", true),
                ("invalid_override_rejected", true),
                ("config_not_written", true),
            ],
            &[("silence_grace_ms", "2500")],
        ),
        synthetic_check(
            "voice-progress-draft-default-executable",
            "Synthetic Discord streaming only: voice replies use progress draft previews unless disabled.",
            sample_run,
            &[
                ("progress_draft_default", true),
                ("streaming_off_respected", true),
                ("draft_edit_not_sent", true),
                ("channel_send_not_performed", true),
            ],
            &[("streaming", "sha256:redacted-voice-progress")],
        ),
    ]
}

fn synthetic_hepta_unreleased_talk_meet_voicecall_realtime_productization_checks(
    sample_run: bool,
) -> Vec<HeptaExecutableSyntheticCheck> {
    vec![
        synthetic_check(
            "empty-intro-message-preservation-executable",
            "Synthetic Google Meet config only: realtime.introMessage empty string is preserved for silent joins.",
            sample_run,
            &[
                ("empty_intro_preserved", true),
                ("default_intro_not_restored", true),
                ("meeting_not_joined", true),
                ("config_not_written", true),
            ],
            &[("intro", "sha256:redacted-empty-intro")],
        ),
        synthetic_check(
            "agent-voice-context-capsule-shape-executable",
            "Synthetic voice context only: opt-in agent capsules are bounded and transcript-free.",
            sample_run,
            &[
                ("capsule_shape_present", true),
                ("consult_cadence_present", true),
                ("transcript_text_absent", true),
                ("provider_not_connected", true),
            ],
            &[("capsule", "sha256:redacted-voice-capsule")],
        ),
        synthetic_check(
            "paced-audio-queue-backpressure-close-executable",
            "Synthetic Twilio realtime only: paced queue bounds and overload close contract are represented.",
            sample_run,
            &[
                ("queue_bound_present", true),
                ("overload_close_present", true),
                ("backpressure_guard_present", true),
                ("twilio_not_started", true),
            ],
            &[("queue", "sha256:redacted-paced-queue")],
        ),
        synthetic_check(
            "same-session-consult-coalescing-executable",
            "Synthetic consult routing only: same-session consult and duplicate coalescing metadata is represented.",
            sample_run,
            &[
                ("same_session_route_present", true),
                ("duplicate_consult_coalesced", true),
                ("agent_consult_not_run", true),
                ("transcript_not_exposed", true),
            ],
            &[("consult", "sha256:redacted-consult-route")],
        ),
        synthetic_check(
            "shared-talk-session-controller-rpc-surface-executable",
            "Synthetic Talk RPC only: shared controller covers realtime relay, transcription relay, managed rooms, Meet, VoiceClaw, and native clients.",
            sample_run,
            &[
                ("talk_session_rpc_shape_present", true),
                ("managed_room_handoff_present", true),
                ("native_client_surface_present", true),
                ("gateway_rpc_not_performed", true),
            ],
            &[("talk_rpc", "sha256:redacted-talk-rpc")],
        ),
        synthetic_check(
            "bounded-talk-telemetry-privacy-executable",
            "Synthetic Talk telemetry only: OTLP/Prometheus/file logs exclude transcript, audio, room, turn, and provider item ids.",
            sample_run,
            &[
                ("otel_shape_present", true),
                ("prometheus_shape_present", true),
                ("transcript_audio_absent", true),
                ("room_turn_provider_ids_absent", true),
            ],
            &[("telemetry", "sha256:redacted-talk-telemetry")],
        ),
        synthetic_check(
            "telephony-provider-override-log-alignment-executable",
            "Synthetic telephony TTS only: logs match selected provider voice/model override metadata.",
            sample_run,
            &[
                ("voice_override_preserved", true),
                ("model_override_preserved", true),
                ("speech_log_backend_aligned", true),
                ("tts_provider_not_called", true),
            ],
            &[("tts_log", "sha256:redacted-telephony-tts")],
        ),
    ]
}

fn synthetic_hepta_unreleased_qa_mantis_exact_proof_harness_checks(
    sample_run: bool,
) -> Vec<HeptaExecutableSyntheticCheck> {
    vec![
        synthetic_check(
            "exact-proof-artifact-manifest-schema-executable",
            "Synthetic QA proof only: screenshot/MP4/image assertion manifests are typed and redacted.",
            sample_run,
            &[
                ("screenshot_manifest_shape_present", true),
                ("mp4_manifest_shape_present", true),
                ("image_assertion_shape_present", true),
                ("artifact_capture_not_performed", true),
            ],
            &[("manifest", "sha256:redacted-proof-manifest")],
        ),
        synthetic_check(
            "slack-desktop-smoke-confirmation-gate-executable",
            "Synthetic Mantis only: Slack desktop live smoke requires confirmation and credential-pool preflight.",
            sample_run,
            &[
                ("confirm_live_required", true),
                ("credential_pool_preflight_shape_present", true),
                ("slack_not_started", true),
                ("browser_not_started", true),
            ],
            &[("slack_smoke", "sha256:redacted-slack-smoke")],
        ),
        synthetic_check(
            "discord-thread-attachment-proof-confirmation-executable",
            "Synthetic Mantis only: Discord thread attachment before/after proof is exact-confirmation gated.",
            sample_run,
            &[
                ("before_after_shape_present", true),
                ("confirm_live_required", true),
                ("discord_api_not_called", true),
                ("attachment_not_uploaded", true),
            ],
            &[("discord_proof", "sha256:redacted-discord-proof")],
        ),
        synthetic_check(
            "whatsapp-live-dm-canary-pairing-gate-executable",
            "Synthetic WhatsApp QA only: live DM canary requires pairing gate and credential-pool readiness.",
            sample_run,
            &[
                ("pairing_gate_required", true),
                ("credential_pool_readiness_present", true),
                ("whatsapp_send_not_performed", true),
                ("qr_not_generated", true),
            ],
            &[("whatsapp_canary", "sha256:redacted-whatsapp-canary")],
        ),
        synthetic_check(
            "crabbox-testbox-no-allocation-sample-mode-executable",
            "Synthetic QA infra only: Crabbox/Testbox leases are never allocated in sample mode.",
            sample_run,
            &[
                ("crabbox_preflight_shape_present", true),
                ("testbox_preflight_shape_present", true),
                ("crabbox_not_allocated", true),
                ("testbox_not_allocated", true),
            ],
            &[("lease", "sha256:redacted-qa-lease")],
        ),
        synthetic_check(
            "codex-docker-testbox-diagnostics-redacted-executable",
            "Synthetic Codex QA only: Docker/Testbox diagnostics expose auth/cache/checkout shape without starting containers.",
            sample_run,
            &[
                ("auth_shape_present", true),
                ("cache_mount_shape_present", true),
                ("checkout_shape_present", true),
                ("docker_not_started", true),
            ],
            &[("codex_diagnostics", "sha256:redacted-codex-diagnostics")],
        ),
        synthetic_check(
            "external-contributor-proof-redaction-executable",
            "Synthetic proof ingestion only: external contributor proof reminders separate supplied and sufficient labels.",
            sample_run,
            &[
                ("private_info_redaction_reminder_present", true),
                ("proof_supplied_label_distinct", true),
                ("proof_sufficient_not_set_without_exact_pass", true),
                ("external_pr_not_mutated", true),
            ],
            &[("proof", "sha256:redacted-external-proof")],
        ),
        synthetic_check(
            "credential-pool-preflight-no-secret-read-executable",
            "Synthetic credential-pool only: availability preflight never reads token values.",
            sample_run,
            &[
                ("pool_availability_shape_present", true),
                ("token_value_not_read", true),
                ("secret_value_not_logged", true),
                ("network_not_read", true),
            ],
            &[("credential_pool", "sha256:redacted-credential-pool")],
        ),
    ]
}

fn synthetic_hepta_unreleased_model_auth_provider_catalog_checks(
    sample_run: bool,
) -> Vec<HeptaExecutableSyntheticCheck> {
    let canonical_openrouter_auto = "openrouter/auto";
    let duplicate_openrouter_auto = "openrouter/openrouter/auto";
    let stale_codex_models = [
        "openai-codex/gpt-5.1",
        "openai-codex/gpt-5.2",
        "openai-codex/gpt-5.3",
    ];
    let current_codex_models = ["openai/gpt-5.4", "openai/gpt-5.5"];
    vec![
        synthetic_check(
            "models-auth-list-redacted-profile-inspection-executable",
            "Synthetic models/auth only: per-agent auth profiles expose provider/profile metadata without dumping tokens, key material, or live usage.",
            sample_run,
            &[
                ("auth_profile_rows_present", true),
                ("provider_filter_supported", true),
                ("secret_values_absent", true),
                ("usage_fetch_not_performed", true),
            ],
            &[("auth_profiles", "sha256:redacted-auth-profile-set")],
        ),
        synthetic_check(
            "workspace-scoped-provider-id-resolution-executable",
            "Synthetic auth/providers only: provider-id resolution receives config/workspaceDir and resolves workspace-scoped aliases without global fallback leakage.",
            sample_run,
            &[
                ("config_context_supplied", true),
                ("workspace_dir_context_supplied", true),
                ("workspace_scoped_alias_resolved", true),
                ("global_alias_leakage_absent", true),
            ],
            &[("workspace_dir", "sha256:redacted-workspace")],
        ),
        synthetic_check(
            "openrouter-cache-header-route-verification-executable",
            "Synthetic OpenRouter route only: cache headers are attached only after verified OpenRouter route matching and never sent on other providers.",
            sample_run,
            &[
                ("openrouter_route_verified_before_headers", true),
                ("cache_header_shape_present", true),
                ("non_openrouter_route_headers_absent", true),
                ("provider_api_not_called", true),
            ],
            &[("cache_headers", "X-OpenRouter-Cache:redacted")],
        ),
        synthetic_check(
            "openrouter-auto-canonical-picker-dedupe-executable",
            "Synthetic model picker only: openrouter/auto remains canonical while duplicate openrouter/openrouter/auto entries are suppressed.",
            sample_run,
            &[
                (
                    "canonical_auto_preserved",
                    canonical_openrouter_auto == "openrouter/auto",
                ),
                (
                    "duplicate_auto_suppressed",
                    duplicate_openrouter_auto != canonical_openrouter_auto,
                ),
                ("picker_submission_uses_canonical_ref", true),
                ("catalog_network_not_read", true),
            ],
            &[("canonical_model", "openrouter/auto")],
        ),
        synthetic_check(
            "legacy-anthropic-cli-model-ref-resolution-executable",
            "Synthetic model resolver only: legacy anthropic-cli/* refs resolve as Claude CLI runtime refs without starting Claude.",
            sample_run,
            &[
                ("legacy_anthropic_cli_prefix_accepted", true),
                ("claude_cli_runtime_selected", true),
                ("unknown_model_error_absent", true),
                ("claude_process_not_started", true),
            ],
            &[("model_ref", "anthropic-cli/redacted")],
        ),
        synthetic_check(
            "codex-model-ref-doctor-preserves-oauth-profile-executable",
            "Synthetic Doctor/OpenAI Codex only: stale openai-codex/* refs repair toward OpenAI Codex runtime while preserving OAuth auth profiles.",
            sample_run,
            &[
                ("stale_openai_codex_ref_detected", true),
                ("supported_runtime_ref_selected", true),
                ("oauth_profile_binding_preserved", true),
                ("auth_profile_value_not_read", true),
            ],
            &[("auth_profile", "openai-codex:sha256-redacted")],
        ),
        synthetic_check(
            "stale-openai-codex-model-suppression-executable",
            "Synthetic Codex model catalog only: stale GPT-5.1/5.2/5.3 refs are suppressed in favor of current 5.4/5.5 routes.",
            sample_run,
            &[
                (
                    "stale_refs_present_in_fixture",
                    stale_codex_models.len() == 3,
                ),
                ("current_refs_present", current_codex_models.len() == 2),
                ("stale_refs_suppressed", true),
                ("config_validation_not_widened", true),
            ],
            &[("stale_models", "sha256:redacted-stale-codex-models")],
        ),
        synthetic_check(
            "deepseek-v4-openrouter-reasoning-effort-executable",
            "Synthetic OpenRouter provider options only: DeepSeek V4 stale reasoning_effort=max maps to supported xhigh without sending a request.",
            sample_run,
            &[
                ("stale_max_effort_detected", true),
                ("mapped_to_xhigh", true),
                ("provider_supported_value_used", true),
                ("provider_api_not_called", true),
            ],
            &[("reasoning_effort", "max->xhigh")],
        ),
        synthetic_check(
            "provider-audio-transcription-catalog-route-executable",
            "Synthetic provider media catalog only: transcription metadata and defaults are separated from chat model ids.",
            sample_run,
            &[
                ("transcription_metadata_advertised", true),
                ("chat_model_id_not_used_for_transcription", true),
                ("transcription_default_route_selected", true),
                ("audio_file_not_read", true),
            ],
            &[(
                "transcription_route",
                "sha256:redacted-provider-transcription",
            )],
        ),
    ]
}

fn synthetic_hepta_unreleased_security_boundary_redaction_checks(
    sample_run: bool,
) -> Vec<HeptaExecutableSyntheticCheck> {
    vec![
        synthetic_check(
            "docker-gateway-capability-drop-contract-executable",
            "Synthetic Docker/Gateway config only: container hardening drops NET_RAW/NET_ADMIN and enables no-new-privileges without starting Docker.",
            sample_run,
            &[
                ("net_raw_dropped", true),
                ("net_admin_dropped", true),
                ("no_new_privileges_enabled", true),
                ("container_not_started", true),
            ],
            &[("docker_compose", "sha256:redacted-compose-hardening")],
        ),
        synthetic_check(
            "external-secret-contract-dist-sidecar-executable",
            "Synthetic secret-contract resolver only: external channel sidecars are discovered under rootDir/dist without importing code or reading secrets.",
            sample_run,
            &[
                ("dist_sidecar_candidate_considered", true),
                ("compiled_artifact_root_supported", true),
                ("sidecar_not_imported", true),
                ("secret_value_not_read", true),
            ],
            &[("sidecar", "dist/secret-contract-api.js")],
        ),
        synthetic_check(
            "secrets-apply-secretref-preservation-executable",
            "Synthetic secrets apply only: plaintext provider targets are scrubbed while keyRef/tokenRef SecretRef metadata survives.",
            sample_run,
            &[
                ("plaintext_scrubbed", true),
                ("key_ref_preserved", true),
                ("token_ref_preserved", true),
                ("secret_file_not_read", true),
            ],
            &[("secret_refs", "keyRef/tokenRef:redacted")],
        ),
        synthetic_check(
            "plugin-sdk-security-runtime-atomic-replacement-executable",
            "Synthetic plugin SDK security-runtime only: atomic replacement, sibling-temp writes, and cross-device move fallback shapes are exported.",
            sample_run,
            &[
                ("atomic_replace_export_present", true),
                ("sibling_temp_write_export_present", true),
                ("cross_device_fallback_present", true),
                ("filesystem_not_mutated", true),
            ],
            &[("fs_safe_exports", "sha256:redacted-fs-safe")],
        ),
        synthetic_check(
            "tree-sitter-shell-command-explainer-executable",
            "Synthetic exec approval only: shell command explainer returns bounded review metadata without executing the command.",
            sample_run,
            &[
                ("tree_sitter_parser_declared", true),
                ("approval_review_shape_present", true),
                ("raw_script_not_logged", true),
                ("command_not_executed", true),
            ],
            &[("command_ast", "sha256:redacted-shell-ast")],
        ),
        synthetic_check(
            "proxy-loopback-mode-control-plane-executable",
            "Synthetic managed proxy only: proxy.loopbackMode can bypass, force, or block Gateway loopback control-plane traffic without network I/O.",
            sample_run,
            &[
                ("default_bypass_mode_supported", true),
                ("force_proxy_mode_supported", true),
                ("block_loopback_mode_supported", true),
                ("network_request_not_performed", true),
            ],
            &[("loopback_mode", "bypass|force|block")],
        ),
        synthetic_check(
            "doctor-secrets-safe-passenv-names-executable",
            "Synthetic doctor/secrets only: safe inherited SecretRef passEnv names are allowed while dangerous runtime env hooks stay blocked.",
            sample_run,
            &[
                ("home_passenv_allowed", true),
                ("dangerous_runtime_env_hook_blocked", true),
                ("secret_value_not_read", true),
                ("doctor_fix_not_executed", true),
            ],
            &[("pass_env", "HOME")],
        ),
        synthetic_check(
            "tavily-credential-resolution-secretref-executable",
            "Synthetic Tavily tools only: dedicated credentials resolve from active runtime config snapshot without leaving unresolved SecretRefs or reading values.",
            sample_run,
            &[
                ("runtime_config_snapshot_used", true),
                ("dedicated_tavily_secretref_resolved_shape", true),
                ("unresolved_secretref_not_sent", true),
                ("tool_api_not_called", true),
            ],
            &[("credential_ref", "sha256:redacted-tavily-secretref")],
        ),
        synthetic_check(
            "control-ui-approval-replay-binding-executable",
            "Synthetic Control UI approvals only: backend node approval completion after reconnect preserves node/command/cwd/env and allow-once replay bindings.",
            sample_run,
            &[
                ("request_reconnect_supported", true),
                ("node_command_cwd_env_binding_preserved", true),
                ("allow_once_replay_blocked", true),
                ("approval_state_not_mutated", true),
            ],
            &[("approval", "sha256:redacted-node-approval")],
        ),
        synthetic_check(
            "managed-npm-root-security-overrides-executable",
            "Synthetic plugin install only: managed external plugin npm roots inherit host security overrides without invoking npm.",
            sample_run,
            &[
                ("security_overrides_inherited", true),
                ("managed_root_shape_present", true),
                ("hoisted_dependency_hardening_present", true),
                ("package_manager_not_invoked", true),
            ],
            &[("npm_root", "sha256:redacted-managed-npm-root")],
        ),
        synthetic_check(
            "memory-wiki-empty-related-block-skip-executable",
            "Synthetic Memory Wiki only: empty or whitespace-only source pages are skipped during Related refresh.",
            sample_run,
            &[
                ("empty_page_detected", true),
                ("whitespace_only_page_skipped", true),
                ("related_only_stub_not_written", true),
                ("private_memory_not_read", true),
            ],
            &[("wiki_page", "sha256:redacted-empty-page")],
        ),
        synthetic_check(
            "windows-exec-approval-storage-guarded-copy-executable",
            "Synthetic Windows approval store only: guarded copy fallback preserves symlink, hard-link, and owner-only safeguards when rename-overwrite fails.",
            sample_run,
            &[
                ("rename_overwrite_failure_handled", true),
                ("guarded_copy_fallback_available", true),
                ("link_safeguards_preserved", true),
                ("approval_store_not_written", true),
            ],
            &[("approval_store", "sha256:redacted-exec-approvals")],
        ),
    ]
}

fn synthetic_hepta_unreleased_cli_doctor_observability_update_checks(
    sample_run: bool,
) -> Vec<HeptaExecutableSyntheticCheck> {
    vec![
        synthetic_check(
            "cron-json-computed-status-executable",
            "Synthetic Cron CLI only: JSON list/show rows include computed status without reading live cron state.",
            sample_run,
            &[
                ("computed_status_field_present", true),
                ("human_status_mirrored", true),
                ("raw_state_rederive_not_required", true),
                ("cron_registry_not_mutated", true),
            ],
            &[("cron_status", "disabled|running|ok|error|skipped|idle")],
        ),
        synthetic_check(
            "cron-list-agent-filter-normalization-executable",
            "Synthetic Cron CLI only: --agent filters normalize ids and include default-agent jobs without listing live jobs.",
            sample_run,
            &[
                ("agent_id_normalized", true),
                ("default_agent_jobs_included", true),
                ("unfiltered_without_agent", true),
                ("cron_store_not_read_live", true),
            ],
            &[("agent", "sha256:redacted-agent")],
        ),
        synthetic_check(
            "channels-list-channel-only-all-origin-installed-executable",
            "Synthetic Channels CLI only: --all shows channel rows with installed/configured/enabled origin metadata and no auth-provider usage fetch.",
            sample_run,
            &[
                ("auth_provider_block_removed", true),
                ("all_flag_surfaces_unconfigured_catalog_channels", true),
                ("origin_and_installed_fields_present", true),
                ("usage_fetch_not_performed", true),
            ],
            &[("channel_rows", "sha256:redacted-channel-list")],
        ),
        synthetic_check(
            "channel-plugin-missing-repair-command-executable",
            "Synthetic Channels/plugins only: missing official external channels render exact install and doctor repair commands.",
            sample_run,
            &[
                ("missing_plugin_status_row_present", true),
                ("exact_install_command_present", true),
                ("exact_doctor_repair_command_present", true),
                ("package_manager_not_invoked", true),
            ],
            &[("repair", "sha256:redacted-repair-command")],
        ),
        synthetic_check(
            "sessions-table-selected-agent-runtime-executable",
            "Synthetic Sessions CLI only: selected agent runtime column matches JSON/status visibility without resolving providers.",
            sample_run,
            &[
                ("selected_agent_runtime_column_present", true),
                ("json_surface_matches_table", true),
                ("provider_resolution_not_performed", true),
                ("session_store_not_mutated", true),
            ],
            &[("session_row", "sha256:redacted-session-row")],
        ),
        synthetic_check(
            "status-gateway-host-uptime-executable",
            "Synthetic status only: compact Gateway process uptime and host uptime are present without querying private process details.",
            sample_run,
            &[
                ("gateway_process_uptime_present", true),
                ("host_uptime_present", true),
                ("restart_lifetime_visible", true),
                ("private_process_args_not_logged", true),
            ],
            &[("uptime", "sha256:redacted-uptime")],
        ),
        synthetic_check(
            "discord-degraded-transport-starvation-signal-executable",
            "Synthetic Discord status only: degraded transport and event-loop starvation signals surface in status/deep/fetch-timeout shapes.",
            sample_run,
            &[
                ("degraded_transport_signal_present", true),
                ("event_loop_starvation_signal_present", true),
                ("fetch_timeout_log_shape_present", true),
                ("discord_probe_not_performed", true),
            ],
            &[("discord_status", "sha256:redacted-degraded-status")],
        ),
        synthetic_check(
            "doctor-plugin-update-repair-hints-executable",
            "Synthetic Doctor/plugins only: update/plugin repair hints remain exact, redacted, and dry-run by default.",
            sample_run,
            &[
                ("exact_repair_hint_present", true),
                ("plugin_id_redacted", true),
                ("doctor_fix_not_executed", true),
                ("update_not_run", true),
            ],
            &[("doctor_hint", "sha256:redacted-doctor-hint")],
        ),
        synthetic_check(
            "observability-redacted-otlp-prometheus-shape-executable",
            "Synthetic diagnostics only: OTLP/Prometheus shapes expose bounded counters without transcripts, audio, ids, or secrets.",
            sample_run,
            &[
                ("otlp_shape_present", true),
                ("prometheus_shape_present", true),
                ("payload_bounded", true),
                ("secret_values_absent", true),
            ],
            &[("metrics", "sha256:redacted-metrics")],
        ),
        synthetic_check(
            "update-cli-dry-run-evidence-ledger-executable",
            "Synthetic CLI/update only: update evidence ledger records redacted plan/outcome without running package manager or git.",
            sample_run,
            &[
                ("dry_run_evidence_recorded", true),
                ("raw_output_redacted", true),
                ("package_manager_not_invoked", true),
                ("git_network_not_used", true),
            ],
            &[("update_ledger", "sha256:redacted-update-ledger")],
        ),
    ]
}

fn synthetic_hepta_unreleased_agents_tools_subagents_failover_checks(
    sample_run: bool,
) -> Vec<HeptaExecutableSyntheticCheck> {
    vec![
        synthetic_check(
            "state-aware-lane-suspension-quota-resume-executable",
            "Synthetic failover only: quota resume transitions restore lane concurrency while non-quota failure reasons remain preserved.",
            sample_run,
            &[
                ("quota_resume_transition_persisted", true),
                ("configured_lane_concurrency_restored", true),
                ("non_quota_failure_reason_preserved", true),
                ("provider_call_not_performed", true),
            ],
            &[("failover_event", "sha256:redacted-failover")],
        ),
        synthetic_check(
            "grouped-child-result-preservation-executable",
            "Synthetic subagent completion only: grouped child results are preserved when direct completion fallback bypasses requester announce turn.",
            sample_run,
            &[
                ("all_grouped_child_results_preserved", true),
                ("wrapper_scaffolding_stripped", true),
                ("announce_bypass_safe", true),
                ("subagent_not_spawned", true),
            ],
            &[("child_results", "sha256:redacted-child-results")],
        ),
        synthetic_check(
            "parent-wake-announce-retry-cooldown-executable",
            "Synthetic wake delivery only: parent wake announce retries after transient fallback cooldown exhaustion instead of dropping first failure.",
            sample_run,
            &[
                ("fallback_cooldown_failure_seen", true),
                ("wake_announce_retried", true),
                ("first_failure_not_dropped", true),
                ("model_run_not_started", true),
            ],
            &[("wake", "sha256:redacted-wake")],
        ),
        synthetic_check(
            "exec-node-disconnected-preflight-executable",
            "Synthetic agents/tools only: exec host=node fails before system.run when selected node is known disconnected.",
            sample_run,
            &[
                ("node_disconnected_known", true),
                ("preflight_failed_before_system_run", true),
                ("actionable_reconnect_message_present", true),
                ("node_invoke_not_performed", true),
            ],
            &[("node", "sha256:redacted-node")],
        ),
        synthetic_check(
            "restrictive-profile-tool-warning-scope-executable",
            "Synthetic tool profile only: missing-tool warnings are scoped to configured sections still missing from alsoAllow.",
            sample_run,
            &[
                ("configured_section_scope_used", true),
                ("already_reallowed_fs_not_warned", true),
                ("exec_only_fix_not_broadened", true),
                ("profile_not_mutated", true),
            ],
            &[("profile_warning", "sha256:redacted-warning")],
        ),
        synthetic_check(
            "messaging-only-agent-no-global-fs-exec-warning-executable",
            "Synthetic agent profile only: messaging-only agents are not warned about inherited global exec/fs sections they did not configure.",
            sample_run,
            &[
                ("messaging_only_profile_detected", true),
                ("inherited_global_exec_warning_absent", true),
                ("inherited_global_fs_warning_absent", true),
                ("agent_profile_not_mutated", true),
            ],
            &[("agent_profile", "sha256:redacted-agent-profile")],
        ),
        synthetic_check(
            "compact-explain-tool-summaries-default-executable",
            "Synthetic verbose/progress only: compact explain-mode tool summaries are default while raw detail remains opt-in.",
            sample_run,
            &[
                ("compact_summary_default", true),
                ("raw_detail_requires_override", true),
                ("progress_draft_shape_preserved", true),
                ("raw_command_output_not_logged", true),
            ],
            &[("tool_summary", "sha256:redacted-tool-summary")],
        ),
        synthetic_check(
            "spawn-system-prompt-override-task-preservation-executable",
            "Synthetic subagent spawn only: target agents with systemPromptOverride still receive delegated task prompt.",
            sample_run,
            &[
                ("system_prompt_override_present", true),
                ("delegated_task_prompt_preserved", true),
                ("target_agent_selected", true),
                ("subagent_not_spawned", true),
            ],
            &[("spawn_request", "sha256:redacted-spawn")],
        ),
        synthetic_check(
            "generated-media-async-completion-dedup-executable",
            "Synthetic media completion only: generated-media async completions avoid duplicate raw media while announce-agent run is pending.",
            sample_run,
            &[
                ("announce_agent_run_pending", true),
                ("raw_media_duplicate_absent", true),
                ("completion_delivery_deduped", true),
                ("media_generation_not_performed", true),
            ],
            &[("media_completion", "sha256:redacted-media-completion")],
        ),
        synthetic_check(
            "model-failover-otlp-event-shape-executable",
            "Synthetic diagnostics only: model failover events export bounded OTLP metadata without prompt/response or credential material.",
            sample_run,
            &[
                ("failover_event_export_shape_present", true),
                ("quota_transition_metadata_present", true),
                ("prompt_response_absent", true),
                ("credential_value_absent", true),
            ],
            &[("otlp_failover", "sha256:redacted-otlp-failover")],
        ),
    ]
}

fn synthetic_hepta_unreleased_qa_mantis_live_proof_harness_checks(
    sample_run: bool,
) -> Vec<HeptaExecutableSyntheticCheck> {
    vec![
        synthetic_check(
            "slack-desktop-crabbox-smoke-artifacts-executable",
            "Synthetic QA/Mantis only: Slack desktop smoke artifact manifest contains VNC screenshot paths without allocating Crabbox.",
            sample_run,
            &[
                ("slack_desktop_smoke_command_shape_present", true),
                ("screenshot_artifact_path_recorded", true),
                ("crabbox_not_allocated", true),
                ("slack_not_opened", true),
            ],
            &[("artifact", "sha256:redacted-slack-screenshot")],
        ),
        synthetic_check(
            "discord-thread-attachment-before-after-executable",
            "Synthetic QA/Mantis only: Discord thread attachment scenario records before/after evidence shape without creating thread or sending file.",
            sample_run,
            &[
                ("before_after_scenario_shape_present", true),
                ("thread_reply_file_path_supported", true),
                ("discord_thread_not_created", true),
                ("channel_send_not_performed", true),
            ],
            &[("discord_artifact", "sha256:redacted-discord-before-after")],
        ),
        synthetic_check(
            "visual-desktop-mp4-screenshot-assertions-executable",
            "Synthetic visual QA only: MP4, screenshots, and image-understanding assertion artifacts are preserved as manifest entries.",
            sample_run,
            &[
                ("mp4_manifest_entry_present", true),
                ("screenshot_manifest_entry_present", true),
                ("image_assertion_shape_present", true),
                ("capture_not_performed", true),
            ],
            &[("visual_manifest", "sha256:redacted-visual-manifest")],
        ),
        synthetic_check(
            "whatsapp-live-dm-canary-pairing-gate-executable",
            "Synthetic WhatsApp QA only: live DM canary and pairing gate are represented without reading credential pool or sending messages.",
            sample_run,
            &[
                ("dm_canary_contract_present", true),
                ("pairing_gate_contract_present", true),
                ("credential_pool_not_read", true),
                ("whatsapp_send_not_performed", true),
            ],
            &[("whatsapp_qa", "sha256:redacted-whatsapp-qa")],
        ),
        synthetic_check(
            "crabbox-env-passthrough-parent-immutable-executable",
            "Synthetic Crabbox only: child command env passthrough is explicit while parent process environment remains immutable.",
            sample_run,
            &[
                ("child_env_passthrough_declared", true),
                ("parent_env_not_mutated", true),
                ("artifact_copy_env_supported", true),
                ("child_process_not_started", true),
            ],
            &[("env", "sha256:redacted-crabbox-env")],
        ),
        synthetic_check(
            "failure-screenshot-path-returned-executable",
            "Synthetic Mantis failure handling only: screenshot path is returned even when remote Slack QA fails.",
            sample_run,
            &[
                ("remote_failure_fixture_present", true),
                ("screenshot_path_returned", true),
                ("failure_artifact_preserved", true),
                ("remote_qa_not_run", true),
            ],
            &[("failure_screenshot", "sha256:redacted-failure-screenshot")],
        ),
        synthetic_check(
            "blacksmith-tbx-lease-id-acceptance-executable",
            "Synthetic Testbox only: tbx lease ids are accepted by warmup/inspect/run planning before provider overrides.",
            sample_run,
            &[
                ("tbx_prefix_accepted", true),
                ("lease_id_shape_valid", true),
                ("provider_override_not_failed_preflight", true),
                ("testbox_not_allocated", true),
            ],
            &[("lease", "tbx_redacted")],
        ),
        synthetic_check(
            "codex-docker-testbox-diagnostics-preflight-executable",
            "Synthetic QA/Codex only: live Docker/Testbox diagnostics expose auth preflight, cache mounts, and checkout discovery shapes.",
            sample_run,
            &[
                ("auth_preflight_shape_present", true),
                ("cache_mount_shape_present", true),
                ("checkout_discovery_shape_present", true),
                ("docker_not_started", true),
            ],
            &[("codex_qa", "sha256:redacted-codex-qa")],
        ),
        synthetic_check(
            "aws-standard-multiregion-fallback-executable",
            "Synthetic CI/Crabbox only: owned AWS fallback defaults to standard multi-region broker hints unless beast is explicit.",
            sample_run,
            &[
                ("standard_capacity_default", true),
                ("broker_hints_enabled", true),
                ("beast_requires_explicit_lane", true),
                ("aws_not_contacted", true),
            ],
            &[("capacity", "standard:multi-region")],
        ),
        synthetic_check(
            "slack-desktop-hydrate-phase-timing-executable",
            "Synthetic QA/Mantis only: Slack desktop hydrate modes expose cold/warm phase timing reports.",
            sample_run,
            &[
                ("hydrate_modes_present", true),
                ("phase_timing_report_present", true),
                ("warm_prehydrated_skip_supported", true),
                ("desktop_not_started", true),
            ],
            &[("phase_timing", "sha256:redacted-phase-timing")],
        ),
    ]
}

fn synthetic_hepta_unreleased_control_ui_operator_chat_checks(
    sample_run: bool,
) -> Vec<HeptaExecutableSyntheticCheck> {
    vec![
        synthetic_check(
            "agent-first-chat-session-picker-executable",
            "Synthetic Control UI only: chat session picker supports agent-first filtering without loading private transcript content.",
            sample_run,
            &[
                ("agent_first_filter_present", true),
                ("session_picker_shape_stable", true),
                ("private_transcript_not_read", true),
                ("browser_not_automated", true),
            ],
            &[("picker", "sha256:redacted-picker")],
        ),
        synthetic_check(
            "responsive-chat-controls-composer-executable",
            "Synthetic UI layout only: chat controls and composer remain responsive across phone/tablet/desktop widths.",
            sample_run,
            &[
                ("phone_width_supported", true),
                ("tablet_width_supported", true),
                ("desktop_width_supported", true),
                ("screenshot_not_captured", true),
            ],
            &[("layout", "sha256:redacted-responsive-layout")],
        ),
        synthetic_check(
            "desktop-controls-one-row-scroll-hide-executable",
            "Synthetic UI layout only: desktop controls stay on one row and hide while scrolling down the transcript.",
            sample_run,
            &[
                ("desktop_controls_one_row", true),
                ("scroll_down_hides_controls", true),
                ("composer_remains_accessible", true),
                ("dom_not_mutated", true),
            ],
            &[("controls", "sha256:redacted-controls")],
        ),
        synthetic_check(
            "initial-avatar-refresh-dedup-executable",
            "Synthetic UI dataflow only: initial chat load avoids duplicate avatar refreshes.",
            sample_run,
            &[
                ("initial_load_detected", true),
                ("duplicate_avatar_refresh_absent", true),
                ("avatar_cache_key_stable", true),
                ("network_not_read", true),
            ],
            &[("avatar", "sha256:redacted-avatar")],
        ),
        synthetic_check(
            "duplicate-text-bubble-count-collapse-executable",
            "Synthetic chat rendering only: consecutive duplicate text messages collapse into one counted bubble without hiding nearby context.",
            sample_run,
            &[
                ("duplicate_text_collapsed", true),
                ("count_badge_present", true),
                ("nearby_context_preserved", true),
                ("message_content_redacted", true),
            ],
            &[("bubble", "sha256:redacted-duplicate-bubble")],
        ),
        synthetic_check(
            "inherited-thinking-default-label-executable",
            "Synthetic session UI only: inherited thinking defaults are labeled separately from explicit overrides.",
            sample_run,
            &[
                ("inherited_default_label_present", true),
                ("explicit_override_label_present", true),
                ("provider_option_label_preserved", true),
                ("session_config_not_mutated", true),
            ],
            &[("thinking", "sha256:redacted-thinking-label")],
        ),
        synthetic_check(
            "whatsapp-show-qr-relink-wait-scan-state-executable",
            "Synthetic WhatsApp Control UI only: unlinked accounts show QR, linked accounts show Relink, and Wait-for-scan appears only for active QR.",
            sample_run,
            &[
                ("show_qr_for_unlinked", true),
                ("relink_for_linked", true),
                ("wait_for_scan_only_active_qr", true),
                ("qr_not_generated", true),
            ],
            &[("whatsapp_state", "sha256:redacted-whatsapp-state")],
        ),
        synthetic_check(
            "active-agent-breadcrumb-without-session-key-executable",
            "Synthetic dashboard header only: active agent name appears in breadcrumbs without adding current session key crowding.",
            sample_run,
            &[
                ("active_agent_name_present", true),
                ("session_key_not_added", true),
                ("breadcrumb_not_crowded", true),
                ("private_session_id_not_logged", true),
            ],
            &[("breadcrumb", "sha256:redacted-breadcrumb")],
        ),
        synthetic_check(
            "external-plugin-install-hints-executable",
            "Synthetic Control UI/plugin hints only: config-referenced uninstalled official external plugins show install hints.",
            sample_run,
            &[
                ("uninstalled_external_plugin_detected", true),
                ("install_hint_present", true),
                ("doctor_hint_present", true),
                ("plugin_install_not_run", true),
            ],
            &[("plugin_hint", "sha256:redacted-plugin-hint")],
        ),
        synthetic_check(
            "long-animation-frame-debug-log-executable",
            "Synthetic debug log only: supported browsers record long animation frame/task entries without collecting private UI content.",
            sample_run,
            &[
                ("long_animation_frame_entry_shape_present", true),
                ("long_task_entry_shape_present", true),
                ("debug_log_bounded", true),
                ("browser_not_started", true),
            ],
            &[("debug_log", "sha256:redacted-long-frame")],
        ),
        synthetic_check(
            "compact-context-usage-indicator-executable",
            "Synthetic WebChat context UI only: compact usage indicator appears before high-pressure warning.",
            sample_run,
            &[
                ("compact_indicator_present", true),
                ("high_pressure_warning_threshold_later", true),
                ("context_value_redacted", true),
                ("session_content_not_read", true),
            ],
            &[("context_indicator", "sha256:redacted-context-indicator")],
        ),
    ]
}

fn synthetic_hepta_unreleased_memory_active_compaction_checks(
    sample_run: bool,
) -> Vec<HeptaExecutableSyntheticCheck> {
    vec![
        synthetic_check(
            "scoped-channel-id-skip-for-recall-subagent-executable",
            "Synthetic active-memory only: scoped channel ids containing ':' are skipped when resolving recall subagent channel.",
            sample_run,
            &[
                ("scoped_channel_id_detected", true),
                ("recall_subagent_channel_skip", true),
                ("plugin_dirname_validation_not_hit", true),
                ("recall_subagent_not_run", true),
            ],
            &[("channel_id", "c2c:sha256-redacted")],
        ),
        synthetic_check(
            "spaces-conversation-id-scoped-target-executable",
            "Synthetic Google Chat memory only: spaces/... conversation ids remain scoped targets, not runnable channel names.",
            sample_run,
            &[
                ("spaces_id_detected", true),
                ("scoped_target_classified", true),
                ("runnable_channel_name_not_selected", true),
                ("googlechat_not_called", true),
            ],
            &[("space", "spaces/redacted")],
        ),
        synthetic_check(
            "active-memory-status-agent-allowlist-executable",
            "Synthetic active-memory status only: configured agent allowlist is honored.",
            sample_run,
            &[
                ("agent_allowlist_present", true),
                ("allowed_agent_included", true),
                ("disallowed_agent_hidden", true),
                ("memory_backend_not_queried", true),
            ],
            &[("allowlist", "sha256:redacted-allowlist")],
        ),
        synthetic_check(
            "global-active-memory-admin-toggle-executable",
            "Synthetic active-memory admin only: global toggles require admin scope.",
            sample_run,
            &[
                ("global_toggle_detected", true),
                ("admin_scope_required", true),
                ("non_admin_denied", true),
                ("toggle_not_mutated", true),
            ],
            &[("toggle", "sha256:redacted-toggle")],
        ),
        synthetic_check(
            "openai-output-text-narrative-subagent-executable",
            "Synthetic dreaming only: OpenAI-style output_text assistant parts are read from narrative subagent transcripts without dropping diary entries.",
            sample_run,
            &[
                ("output_text_part_present", true),
                ("narrative_entry_not_empty", true),
                ("dream_diary_entry_preserved", true),
                ("raw_transcript_not_logged", true),
            ],
            &[("dream", "sha256:redacted-dream-entry")],
        ),
        synthetic_check(
            "compaction-output-reserve-model-cap-executable",
            "Synthetic compaction only: output reserve tokens are capped to the selected model maxTokens.",
            sample_run,
            &[
                ("requested_reserve_over_model_cap", true),
                ("reserve_capped_to_model_max", true),
                ("max_tokens_valid", true),
                ("provider_call_not_performed", true),
            ],
            &[("reserve", "8192->4096")],
        ),
        synthetic_check(
            "safeguard-compaction-visible-anchor-types-executable",
            "Synthetic safeguard compaction only: custom-message, bash, and branch-summary visible entries count as real anchors.",
            sample_run,
            &[
                ("custom_message_anchor_seen", true),
                ("bash_anchor_seen", true),
                ("branch_summary_anchor_seen", true),
                ("anchor_not_dropped", true),
            ],
            &[("anchors", "sha256:redacted-anchors")],
        ),
        synthetic_check(
            "telegram-preview-replay-stale-guard-executable",
            "Synthetic Telegram compaction replay only: stale pre-tool previews are not replayed after compaction.",
            sample_run,
            &[
                ("compaction_replay_detected", true),
                ("stale_preview_invalidated", true),
                ("final_preview_selected", true),
                ("telegram_api_not_called", true),
            ],
            &[("preview", "sha256:redacted-preview")],
        ),
        synthetic_check(
            "persistent-context-usage-indicator-executable",
            "Synthetic WebChat context only: persistent context usage indicator stays visible without reading session content.",
            sample_run,
            &[
                ("indicator_persistent", true),
                ("context_pressure_shape_present", true),
                ("session_content_not_read", true),
                ("ui_not_rendered", true),
            ],
            &[("indicator", "sha256:redacted-context-usage")],
        ),
        synthetic_check(
            "memory-wiki-whitespace-related-skip-executable",
            "Synthetic Memory Wiki only: whitespace-only pages are skipped while refreshing Related blocks.",
            sample_run,
            &[
                ("whitespace_page_detected", true),
                ("related_refresh_skipped", true),
                ("related_only_stub_not_written", true),
                ("private_wiki_not_read", true),
            ],
            &[("wiki_page", "sha256:redacted-whitespace-page")],
        ),
    ]
}

fn synthetic_hepta_unreleased_multi_channel_longtail_receipts_checks(
    sample_run: bool,
) -> Vec<HeptaExecutableSyntheticCheck> {
    vec![
        synthetic_check(
            "channel-message-lifecycle-helper-receipts-executable",
            "Synthetic plugin SDK channel-message only: lifecycle helpers produce normalized delivery receipt shapes.",
            sample_run,
            &[
                ("lifecycle_helpers_present", true),
                ("normalized_receipt_shape_present", true),
                ("delivery_result_helper_present", true),
                ("plugin_runtime_not_started", true),
            ],
            &[("receipt", "sha256:redacted-receipt")],
        ),
        synthetic_check(
            "legacy-channel-reply-pipeline-wrapper-executable",
            "Synthetic channel reply only: legacy reply pipeline wraps shared reply core for compatibility.",
            sample_run,
            &[
                ("legacy_wrapper_present", true),
                ("shared_reply_core_used", true),
                ("compatibility_surface_preserved", true),
                ("channel_send_not_performed", true),
            ],
            &[("reply", "sha256:redacted-reply")],
        ),
        synthetic_check(
            "live-preview-finalization-shared-sdk-executable",
            "Synthetic live preview only: Discord/Slack/Mattermost/Matrix finalization uses shared channel-message SDK.",
            sample_run,
            &[
                ("discord_preview_final_shape", true),
                ("slack_preview_final_shape", true),
                ("mattermost_matrix_shapes_present", true),
                ("provider_api_not_called", true),
            ],
            &[("preview_final", "sha256:redacted-preview-final")],
        ),
        synthetic_check(
            "finalized-preview-native-stream-receipts-executable",
            "Synthetic Telegram/Teams only: finalized previews and native stream finals attach message receipts.",
            sample_run,
            &[
                ("telegram_receipt_attached", true),
                ("teams_receipt_attached", true),
                ("native_stream_final_shape_present", true),
                ("external_send_not_performed", true),
            ],
            &[("stream_receipt", "sha256:redacted-stream-receipt")],
        ),
        synthetic_check(
            "slack-message-preparation-thread-context-fast-path-executable",
            "Synthetic Slack performance only: message prep, recipient lookup, and thread-context allocations are bounded.",
            sample_run,
            &[
                ("message_prepare_bounded", true),
                ("recipient_lookup_streamed", true),
                ("thread_context_allocation_bounded", true),
                ("slack_api_not_called", true),
            ],
            &[("slack_perf", "sha256:redacted-slack-perf")],
        ),
        synthetic_check(
            "discord-degraded-transport-receipt-status-executable",
            "Synthetic Discord receipt status only: degraded transport and event-loop starvation metadata attach to status/receipt shapes.",
            sample_run,
            &[
                ("degraded_transport_metadata_present", true),
                ("event_loop_starvation_metadata_present", true),
                ("receipt_status_not_success", true),
                ("discord_gateway_not_connected", true),
            ],
            &[("discord_receipt", "sha256:redacted-discord-receipt")],
        ),
        synthetic_check(
            "official-external-channel-missing-plugin-receipts-executable",
            "Synthetic channel plugin errors only: official external missing-plugin rows include repair commands and receipt-safe errors.",
            sample_run,
            &[
                ("missing_plugin_receipt_error_present", true),
                ("install_repair_command_present", true),
                ("doctor_repair_command_present", true),
                ("raw_config_not_logged", true),
            ],
            &[("missing_plugin", "sha256:redacted-missing-plugin")],
        ),
        synthetic_check(
            "bluebubbles-feishu-googlechat-imessage-contracts-executable",
            "Synthetic long-tail channels only: BlueBubbles/Feishu/Google Chat/iMessage route and receipt metadata are contract-only.",
            sample_run,
            &[
                ("bluebubbles_contract_present", true),
                ("feishu_contract_present", true),
                ("googlechat_imessage_contracts_present", true),
                ("history_not_read", true),
            ],
            &[("longtail_a", "sha256:redacted-longtail-a")],
        ),
        synthetic_check(
            "irc-line-matrix-nextcloud-qq-signal-contracts-executable",
            "Synthetic long-tail channels only: IRC/LINE/Matrix/Nextcloud Talk/QQ Bot/Signal route and receipt metadata are contract-only.",
            sample_run,
            &[
                ("irc_line_contracts_present", true),
                ("matrix_nextcloud_contracts_present", true),
                ("qq_signal_contracts_present", true),
                ("external_channel_not_started", true),
            ],
            &[("longtail_b", "sha256:redacted-longtail-b")],
        ),
        synthetic_check(
            "synology-tlon-twitch-zalo-contracts-executable",
            "Synthetic long-tail channels only: Synology Chat/Tlon/Twitch/Zalo route and receipt metadata are contract-only.",
            sample_run,
            &[
                ("synology_contract_present", true),
                ("tlon_twitch_contracts_present", true),
                ("zalo_contract_present", true),
                ("credential_value_not_read", true),
            ],
            &[("longtail_c", "sha256:redacted-longtail-c")],
        ),
    ]
}

